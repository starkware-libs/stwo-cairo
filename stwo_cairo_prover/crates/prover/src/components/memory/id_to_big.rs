use std::simd::Simd;

use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::super::{
    LookupFunc, Standard, StandardClaim, StandardComponent, StandardLookupData, StandardProver,
};
use super::addr_to_id::BIG_INITIAL_ID;
use super::{MemoryAndRangeElements, N_MEM_BIG_LIMBS};
use crate::components::{ContextFor, StandardInteractionClaim, StandardInteractionProver};
use crate::felt::split_f252_simd;
use crate::input::mem::Memory;

pub type IdToBigProver = StandardProver<IdToBig>;
pub type IdToBigInteractionProver = StandardInteractionProver<IdToBigLookupData>;
pub type IdToBigClaim = StandardClaim<IdToBig>;
pub type IdToBigInteractionClaim = StandardInteractionClaim;
pub type IdToBigComponent = StandardComponent<IdToBig>;

// Builder
pub struct IdToBigBuilder {
    pub initial_id: u32,
    // pub values: Vec<[u32; 8]>,
    pub mults: Vec<u32>,
}
impl IdToBigBuilder {
    // TODO: include u64 values.
    // Note: Possible optimization is to create a standalone id->inst value component, in case
    // program opcodes are not ran a lot of time.
    pub fn new(mem: &Memory) -> Self {
        let size = mem.f252_values.len();
        Self {
            initial_id: (2 << 28),
            mults: vec![0; size],
        }
    }
    pub fn add_inputs_simd(
        &mut self,
        mem: &Memory,
        inputs: Simd<i32, N_LANES>,
    ) -> [Simd<u32, N_LANES>; 8] {
        let res = inputs.to_array().map(|id| self.add_inputs(mem, id));
        std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| res[j][i])))
    }
    pub fn add_inputs(&mut self, mem: &Memory, id: i32) -> [u32; 8] {
        let id = (id - BIG_INITIAL_ID) as usize;
        self.mults[id] += 1;
        mem.f252_values[id]
    }
    pub fn build(self, values: Vec<[u32; 8]>) -> IdToBigProver {
        let inputs = values
            .into_iter()
            .zip(self.mults)
            .map(|(value, mult)| IdToBigInput { value, mult });
        StandardProver::new(self.initial_id, inputs).pop().unwrap()
    }
}

// Addr to id.
#[derive(Clone)]
pub struct IdToBigInput {
    pub value: [u32; 8],
    pub mult: u32,
}
pub struct PackedIdToBigInput {
    pub value: [Simd<u32, N_LANES>; 8],
    pub mult: Simd<u32, N_LANES>,
}
impl From<[IdToBigInput; N_LANES]> for PackedIdToBigInput {
    fn from(value: [IdToBigInput; N_LANES]) -> Self {
        PackedIdToBigInput {
            value: std::array::from_fn(|i| {
                Simd::from_array(std::array::from_fn(|j| value[j].value[i]))
            }),
            mult: Simd::from_array(std::array::from_fn(|j| value[j].mult)),
        }
    }
}
#[derive(Clone)]
pub struct IdToBig;
impl Standard for IdToBig {
    type LookupElements = MemoryAndRangeElements;
    type Input = IdToBigInput;
    type PackedInput = PackedIdToBigInput;
    type LookupData = IdToBigLookupData;
    type Params = u32;
    const N_REPETITIONS: usize = 1;

    fn pad(mut input: IdToBigInput) -> IdToBigInput {
        input.mult = 0;
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        MemoryAndRangeElements::dummy()
    }
    fn dummy_params() -> Self::Params {
        0
    }
    fn new_lookup_data(log_size: u32, &initial_id: &u32) -> Vec<Self::LookupData> {
        vec![IdToBigLookupData {
            log_size,
            initial_id,
            value: vec![[Simd::splat(0); 8]; 1 << (log_size - LOG_N_LANES)],
            mult: vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)],
        }]
    }

    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        els: &Self::LookupElements,
        &initial_id: &u32,
    ) {
        // TODO: This should be a constant column.
        let base_id = eval.next_trace_mask();
        let id = base_id + E::F::from(M31::from_u32_unchecked(initial_id));
        let mult = eval.next_trace_mask();

        let mut values = [E::F::zero(); 1 + N_MEM_BIG_LIMBS];
        values[0] = id;
        for i in 0..N_MEM_BIG_LIMBS {
            values[i + 1] = eval.next_trace_mask();
        }

        // Add to big value relation.
        logup.push_lookup(eval, (-mult).into(), &values, &els.mem.id_to_big);
        logup.push_lookup(eval, Zero::zero(), &values, &els.mem.id_to_big); // TODO: Remove.

        // Range check limbs.
        for i in 0..N_MEM_BIG_LIMBS {
            // TODO: Change to 1.
            logup.push_lookup(eval, E::EF::zero(), &[values[i + 1]], &els.range.rc18);
        }
    }
}
impl ContextFor<IdToBig> for () {
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedIdToBigInput,
        row_index: usize,
        lookup_data: &mut IdToBigLookupData,
    ) {
        // TODO: This should be a constant column.
        dst[0].data[row_index] = PackedM31::from_array(std::array::from_fn(|i| {
            M31::from_u32_unchecked((row_index * N_LANES + i) as u32)
        }));
        dst[1].data[row_index] = unsafe { PackedM31::from_simd_unchecked(input.mult) };

        let limbs = split_f252_simd(input.value);
        for i in 0..N_MEM_BIG_LIMBS {
            dst[2 + i].data[row_index] = limbs[i];
        }

        lookup_data.value[row_index] = input.value;
        lookup_data.mult[row_index] = input.mult;
    }
}

pub struct IdToBigLookupData {
    log_size: u32,
    initial_id: u32,
    pub value: Vec<[Simd<u32, N_LANES>; 8]>,
    pub mult: Vec<Simd<u32, N_LANES>>,
}
impl StandardLookupData for IdToBigLookupData {
    const N_LOOKUPS: usize = N_MEM_BIG_LIMBS + 1 + 1; // TODO: Remove.
    type Elements = MemoryAndRangeElements;
    fn lookups<'a>(&'a self, els: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        chain![
            // Add to big value relation.
            [
                Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(move |row| {
                    let offset = PackedM31::from_array(std::array::from_fn(|i| {
                        M31::from_u32_unchecked(i as u32)
                    }));

                    let limbs = split_f252_simd(self.value[row]);
                    let id = offset
                        + M31::from(
                            (row * N_LANES * IdToBig::N_REPETITIONS) as u32 + self.initial_id,
                        );
                    let mut lookup_values = [Zero::zero(); 1 + N_MEM_BIG_LIMBS];
                    lookup_values[0] = id;
                    #[allow(clippy::manual_memcpy)]
                    for i in 0..N_MEM_BIG_LIMBS {
                        lookup_values[i + 1] = limbs[i];
                    }

                    let mult = unsafe { PackedM31::from_simd_unchecked(self.mult[row]) };
                    let denom = els.mem.id_to_big.combine(&lookup_values);

                    Fraction::new(-mult, denom)
                })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>
            ],
            // Dummy. TODO: Remove.
            [
                Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(move |row| {
                    let offset = PackedM31::from_array(std::array::from_fn(|i| {
                        M31::from_u32_unchecked(i as u32)
                    }));

                    let limbs = split_f252_simd(self.value[row]);
                    let id = offset
                        + M31::from(
                            (row * N_LANES * IdToBig::N_REPETITIONS) as u32 + self.initial_id,
                        );
                    let mut lookup_values = [Zero::zero(); 1 + N_MEM_BIG_LIMBS];
                    lookup_values[0] = id;
                    #[allow(clippy::manual_memcpy)]
                    for i in 0..N_MEM_BIG_LIMBS {
                        lookup_values[i + 1] = limbs[i];
                    }

                    let _mult = unsafe { PackedM31::from_simd_unchecked(self.mult[row]) };
                    let denom = els.mem.id_to_big.combine(&lookup_values);

                    Fraction::new(PackedM31::zero(), denom)
                })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>
            ],
            // Range check limbs.
            (0..N_MEM_BIG_LIMBS).map(|i| {
                Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(move |row| {
                    // TODO: Consider optimizing.
                    let limbs = split_f252_simd(self.value[row]);
                    let denom = els.range.rc18.combine(&[limbs[i]]);

                    // Fraction::new(PackedM31::one(), denom)
                    Fraction::new(PackedM31::zero(), denom)
                })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>
            })
        ]
        .collect_vec()
    }
}

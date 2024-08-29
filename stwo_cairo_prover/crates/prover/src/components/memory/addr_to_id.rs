use std::simd::Simd;

use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::super::{LookupFunc, Standard, StandardLookupData};
use super::{m31_from_i32, MemoryElements};
use crate::components::{
    ContextFor, StandardClaimStack, StandardComponentStack, StandardInteractionClaimStack,
    StandardInteractionProverStack, StandardProverStack,
};
use crate::input::mem::{EncodedMemoryValueId, Memory, MemoryValueId};

pub type AddrToIdProver = StandardProverStack<AddrToId>;
pub type AddrToIdInteractionProver = StandardInteractionProverStack<AddrToIdLookupData>;
pub type AddrToIdClaim = StandardClaimStack<AddrToId>;
pub type AddrToIdInteractionClaim = StandardInteractionClaimStack;
pub type AddrToIdComponent = StandardComponentStack<AddrToId>;

pub const BIG_INITIAL_ID: i32 = 1 << 29;

// TODO: Add small ids and addresses to big value lookup relation.
// TODO: Drop u64. Add instruction cache instead.
// Builder
pub struct AddrToIdBuilder {
    pub mults: Vec<u32>,
}
impl AddrToIdBuilder {
    pub fn new(mem: &Memory) -> Self {
        let size = mem.address_to_id.len();
        Self {
            mults: vec![0; size],
        }
    }
    pub fn add_inputs_simd(
        &mut self,
        mem: &Memory,
        inputs: Simd<u32, N_LANES>,
    ) -> Simd<i32, N_LANES> {
        Simd::from_array(inputs.to_array().map(|addr| self.add_inputs(mem, addr)))
    }
    pub fn add_inputs(&mut self, mem: &Memory, addr: u32) -> i32 {
        let addr = addr as usize;
        self.mults[addr] += 1;
        id_from_encoded(mem.address_to_id[addr])
    }
    pub fn build(self, addr_to_id: Vec<EncodedMemoryValueId>) -> AddrToIdProver {
        let inputs = addr_to_id
            .into_iter()
            .zip(self.mults)
            .map(|(id, mult)| AddrToIdInput {
                id: id_from_encoded(id),
                mult,
            });
        AddrToIdProver::new((), inputs)
    }
}
pub fn id_from_encoded(id: EncodedMemoryValueId) -> i32 {
    match id.decode() {
        MemoryValueId::Small(x) => x,
        MemoryValueId::F252(x) => x as i32 + BIG_INITIAL_ID,
    }
}

// Addr to id.
#[derive(Clone)]
pub struct AddrToIdInput {
    pub id: i32,
    pub mult: u32,
}
pub struct PackedAddrToIdInput {
    pub id: Simd<i32, N_LANES>,
    pub mult: Simd<u32, N_LANES>,
}
impl From<[AddrToIdInput; N_LANES]> for PackedAddrToIdInput {
    fn from(value: [AddrToIdInput; N_LANES]) -> Self {
        PackedAddrToIdInput {
            id: Simd::from_array(value.each_ref().map(|x| x.id)),
            mult: Simd::from_array(value.each_ref().map(|x| x.mult)),
        }
    }
}
#[derive(Clone)]
pub struct AddrToId;
impl Standard for AddrToId {
    type LookupElements = MemoryElements;
    type Input = AddrToIdInput;
    type PackedInput = PackedAddrToIdInput;
    type LookupData = AddrToIdLookupData;
    type Params = ();
    const N_REPETITIONS: usize = 8;

    fn pad(mut input: Self::Input) -> Self::Input {
        input.mult = 0;
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        MemoryElements::dummy()
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &(), start_index: usize) -> Self::LookupData {
        AddrToIdLookupData {
            log_size,
            start_index,
            id: vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)],
            mult: vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)],
        }
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &Self::LookupElements,
        _params: &(),
        start_index: usize,
    ) {
        // TODO: This should be a constant column.
        let addr = eval.next_trace_mask() + E::F::from((start_index as u32).into());
        let id = eval.next_trace_mask();
        let mult = eval.next_trace_mask();
        logup.push_frac(eval, elements.addr_to_id.combine_frac(-mult, &[addr, id]));
    }
}
impl ContextFor<AddrToId> for () {
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedAddrToIdInput,
        row_index: usize,
        lookup_data: &mut AddrToIdLookupData,
    ) {
        // TODO: This should be a constant column.
        dst[0].data[row_index] = PackedM31::from_array(std::array::from_fn(|i| {
            M31::from_u32_unchecked((row_index * N_LANES + i) as u32)
        }));
        dst[1].data[row_index] = m31_from_i32(input.id);
        dst[2].data[row_index] = unsafe { PackedM31::from_simd_unchecked(input.mult) };
        lookup_data.id[row_index] = input.id;
        lookup_data.mult[row_index] = input.mult;
    }
}

pub struct AddrToIdLookupData {
    log_size: u32,
    start_index: usize,
    pub id: Vec<Simd<i32, N_LANES>>,
    pub mult: Vec<Simd<u32, N_LANES>>,
}
impl StandardLookupData for AddrToIdLookupData {
    const N_LOOKUPS: usize = 1;
    type Elements = LookupElements<2>;
    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        vec![
            // Address lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(move |row| {
                let offset = PackedM31::from_array(std::array::from_fn(|i| {
                    M31::from_u32_unchecked(i as u32)
                }));
                let addr = offset
                    + PackedM31::broadcast(M31::from_u32_unchecked(
                        (row * N_LANES + self.start_index) as u32,
                    ));
                let id = m31_from_i32(self.id[row]);
                let mult = unsafe { PackedM31::from_simd_unchecked(self.mult[row]) };
                elements.combine_frac(-mult, &[addr, id])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

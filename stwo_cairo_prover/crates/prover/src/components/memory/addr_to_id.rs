use std::simd::Simd;

use itertools::Itertools;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::super::{
    LookupFunc, Standard, StandardClaim, StandardComponent, StandardLookupData, StandardProver,
};
use super::{m31_from_i32, MemoryElements};
use crate::components::{ContextFor, StandardInteractionClaim, StandardInteractionProver};
use crate::input::mem::{EncodedMemoryValueId, Memory, MemoryValueId};

pub type AddrToIdProver = StandardProver<AddrToId>;
pub type AddrToIdInteractionProver = StandardInteractionProver<AddrToIdLookupData>;
pub type AddrToIdClaim = StandardClaim<AddrToId>;
pub type AddrToIdInteractionClaim = StandardInteractionClaim;
pub type AddrToIdComponent = StandardComponent<AddrToId>;

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
        let size = addr_to_id.len();
        let inputs = (0..size).step_by(N_IDS_PER_LINE).map(|i| {
            let mut id = [0; N_IDS_PER_LINE];
            let mut mult = [0; N_IDS_PER_LINE];
            for j in 0..N_IDS_PER_LINE {
                let index = i + j;
                if index < size {
                    id[j] = id_from_encoded(addr_to_id[index]);
                    mult[j] = self.mults[index];
                }
            }
            AddrToIdInput { id, mult }
        });
        StandardProver::new((), inputs)
    }
}
pub fn id_from_encoded(id: EncodedMemoryValueId) -> i32 {
    match id.decode() {
        MemoryValueId::Small(x) => x,
        MemoryValueId::F252(x) => x as i32 + BIG_INITIAL_ID,
    }
}

// Addr to id.
pub const N_IDS_PER_LINE: usize = 4;
#[derive(Clone)]
pub struct AddrToIdInput {
    pub id: [i32; N_IDS_PER_LINE],
    pub mult: [u32; N_IDS_PER_LINE],
}
pub struct PackedAddrToIdInput {
    pub id: [Simd<i32, N_LANES>; N_IDS_PER_LINE],
    pub mult: [Simd<u32, N_LANES>; N_IDS_PER_LINE],
}
impl From<[AddrToIdInput; N_LANES]> for PackedAddrToIdInput {
    fn from(value: [AddrToIdInput; N_LANES]) -> Self {
        PackedAddrToIdInput {
            id: std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| value[j].id[i]))),
            mult: std::array::from_fn(|i| {
                Simd::from_array(std::array::from_fn(|j| value[j].mult[i]))
            }),
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
    const N_REPETITIONS: usize = 1;

    fn pad(mut input: Self::Input) -> Self::Input {
        input.mult = [0; N_IDS_PER_LINE];
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        MemoryElements::dummy()
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &()) -> Vec<Self::LookupData> {
        let a = [(); N_IDS_PER_LINE];
        vec![AddrToIdLookupData {
            log_size,
            id: a.map(|_| vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)]),
            mult: a.map(|_| vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)]),
        }]
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &Self::LookupElements,
        _params: &(),
    ) {
        // TODO: This should be a constant column.
        let base_addr = eval.next_trace_mask();
        for i in 0..N_IDS_PER_LINE {
            let id = eval.next_trace_mask();
            let mult = eval.next_trace_mask();
            logup.push_lookup(
                eval,
                (-mult).into(),
                &[
                    base_addr * M31::from(N_IDS_PER_LINE)
                        + E::F::from(M31::from_u32_unchecked(i as u32)),
                    id,
                ],
                &elements.addr_to_id,
            );
        }
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
        for i in 0..N_IDS_PER_LINE {
            dst[1 + 2 * i].data[row_index] = m31_from_i32(input.id[i]);
            dst[1 + 2 * i + 1].data[row_index] =
                unsafe { PackedM31::from_simd_unchecked(input.mult[i]) };
            lookup_data.id[i][row_index] = input.id[i];
            lookup_data.mult[i][row_index] = input.mult[i];
        }
    }
}

pub struct AddrToIdLookupData {
    log_size: u32,
    pub id: [Vec<Simd<i32, N_LANES>>; N_IDS_PER_LINE],
    pub mult: [Vec<Simd<u32, N_LANES>>; N_IDS_PER_LINE],
}
impl StandardLookupData for AddrToIdLookupData {
    const N_LOOKUPS: usize = N_IDS_PER_LINE;
    type Elements = LookupElements<2>;
    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        (0..N_IDS_PER_LINE)
            .flat_map(|i| {
                [
                    // Address lookup.
                    Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(move |row| {
                        let offset = PackedM31::from_array(std::array::from_fn(|i| {
                            M31::from_u32_unchecked((i * N_IDS_PER_LINE) as u32)
                        }));
                        let index: usize = row * N_LANES * N_IDS_PER_LINE + i;
                        let addr =
                            PackedM31::broadcast(M31::from_u32_unchecked(index as u32)) + offset;
                        let id = m31_from_i32(self.id[i][row]);
                        let mult = unsafe { PackedM31::from_simd_unchecked(self.mult[i][row]) };
                        let denom = elements.combine(&[addr, id]);

                        Fraction::new(-mult, denom)
                    }))
                        as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
                ]
            })
            .collect_vec()
    }
}

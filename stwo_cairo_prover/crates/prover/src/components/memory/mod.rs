use std::simd::cmp::SimdOrd;
use std::simd::num::SimdInt;
use std::simd::Simd;

use itertools::Itertools;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{self, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::{
    LookupFunc, Standard, StandardClaim, StandardComponent, StandardLookupData, StandardProver,
};
use crate::input::mem::{EncodedMemoryValueId, MemoryValueId};

pub const N_MEM_BIG_LIMBS: usize = 14;
pub const N_MEM_BIG_LIMB_BITS: usize = 18;

#[derive(Clone)]
pub struct MemoryElements {
    pub addr_to_id: LookupElements<2>,
    pub id_to_big: LookupElements<{ 1 + N_MEM_BIG_LIMBS }>,
}
impl MemoryElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            addr_to_id: LookupElements::draw(channel),
            id_to_big: LookupElements::draw(channel),
        }
    }
}

pub type AddrToIdProver = StandardProver<AddrToId>;
pub type AddrToIdClaim = StandardClaim<AddrToId>;
pub type AddrToIdComponent = StandardComponent<AddrToId>;

pub const N_INSTR_LIMBS: usize = 4;
pub type InstructionElements = LookupElements<{ 1 + N_INSTR_LIMBS }>;

// Builder
pub struct AddrToIdProverBuilder {
    pub ids: Vec<i32>,
    pub mults: Vec<u32>,
}
impl AddrToIdProverBuilder {
    pub fn new(addr_to_id: Vec<EncodedMemoryValueId>) -> Self {
        let size = addr_to_id.len();
        Self {
            ids: addr_to_id
                .into_iter()
                .map(|id| match id.decode() {
                    MemoryValueId::Small(x) => x,
                    MemoryValueId::U64(x) => x as i32 + (1 << 28),
                    MemoryValueId::F252(x) => x as i32 + (2 << 28),
                })
                .collect_vec(),
            mults: vec![0; size],
        }
    }
    pub fn add_inputs_simd(&mut self, inputs: Simd<u32, N_LANES>) -> PackedM31 {
        let addresses = inputs.to_array();
        m31_from_i32(Simd::from_array(
            addresses.map(|addr| self.add_inputs(addr)),
        ))
    }
    pub fn add_inputs(&mut self, addr: u32) -> i32 {
        let addr = addr as usize;
        self.mults[addr] += 1;
        self.ids[addr]
    }
    pub fn build(self) -> AddrToIdProver {
        println!("ids: {:?}", self.ids);
        let size = self.ids.len();
        // Cut self.ids and self.mults into a vector of AddToIdInput s.
        let inputs = (0..size).step_by(N_IDS_PER_LINE).map(|i| {
            let mut id = [0; N_IDS_PER_LINE];
            let mut mult = [0; N_IDS_PER_LINE];
            for j in 0..N_IDS_PER_LINE {
                let index = i + j;
                if index < size {
                    id[j] = self.ids[index];
                    mult[j] = self.mults[index];
                }
            }
            AddToIdInput { id, mult }
        });
        StandardProver::new(inputs).pop().unwrap()
    }
}

// Addr to id.
pub const N_IDS_PER_LINE: usize = 4;
#[derive(Clone)]
pub struct AddToIdInput {
    pub id: [i32; N_IDS_PER_LINE],
    pub mult: [u32; N_IDS_PER_LINE],
}
pub struct PackedAddToIdInput {
    pub id: [Simd<i32, N_LANES>; N_IDS_PER_LINE],
    pub mult: [Simd<u32, N_LANES>; N_IDS_PER_LINE],
}
impl From<[AddToIdInput; N_LANES]> for PackedAddToIdInput {
    fn from(value: [AddToIdInput; N_LANES]) -> Self {
        PackedAddToIdInput {
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
    type Context<'a> = ();
    type PackedInput = PackedAddToIdInput;
    type LookupData = AddrToIdLookupData;
    const N_REPETITIONS: usize = 1;

    fn n_columns() -> usize {
        1 + N_IDS_PER_LINE * 2
    }

    fn new_lookup_data(log_size: u32) -> Self::LookupData {
        let a = [(); N_IDS_PER_LINE];
        AddrToIdLookupData {
            log_size,
            id: a.map(|_| vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)]),
            mult: a.map(|_| vec![Simd::splat(0); 1 << (log_size - LOG_N_LANES)]),
        }
    }

    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &Self::LookupElements,
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

    fn write_trace_row(
        dst: &mut [BaseColumn],
        input: &Self::PackedInput,
        row_index: usize,
        _ctx: &mut Self::Context<'_>,
        lookup_data: &mut Self::LookupData,
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
fn m31_from_i32(x: Simd<i32, N_LANES>) -> PackedM31 {
    // Cast to unsigned.
    let x: Simd<u32, N_LANES> = x.cast();
    let x = Simd::simd_min(x, x + m31::MODULUS);
    unsafe { PackedM31::from_simd_unchecked(x) }
}

pub struct AddrToIdLookupData {
    log_size: u32,
    pub id: [Vec<Simd<i32, N_LANES>>; N_IDS_PER_LINE],
    pub mult: [Vec<Simd<u32, N_LANES>>; N_IDS_PER_LINE],
}
impl StandardLookupData for AddrToIdLookupData {
    const N_LOOKUPS: usize = 2 * N_IDS_PER_LINE;
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

                        // TODO: Debug.
                        for i in 0..N_LANES {
                            if mult.to_array()[i].0 != 0 {
                                println!(
                                    "A mult: -{} addr: {} id: {}",
                                    mult.to_array()[i].0,
                                    addr.to_array()[i].0,
                                    id.to_array()[i].0
                                );
                            }
                        }
                        Fraction::new(-mult, denom)
                    }))
                        as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
                ]
            })
            .collect_vec()
    }
}

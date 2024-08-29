use std::collections::HashMap;
use std::simd::num::SimdInt;
use std::simd::Simd;

use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::super::{LookupFunc, Standard, StandardLookupData};
use super::addr_to_id::AddrToIdBuilder;
use super::id_to_big::IdToBigBuilder;
use super::MemoryAndRangeElements;
use crate::components::{
    ContextFor, StandardClaimStack, StandardComponentStack, StandardInteractionClaimStack,
    StandardInteractionProverStack, StandardProverStack,
};
use crate::input::mem::Memory;

pub type InstMemProver = StandardProverStack<InstMem>;
pub type InstMemInteractionProver = StandardInteractionProverStack<InstMemLookupData>;
pub type InstMemClaim = StandardClaimStack<InstMem>;
pub type InstMemInteractionClaim = StandardInteractionClaimStack;
pub type InstMemComponent = StandardComponentStack<InstMem>;

// Builder
#[derive(Default)]
pub struct InstMemBuilder {
    pub addr_to_mult: HashMap<u32, u32>,
}
impl InstMemBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_inputs_simd(&mut self, inputs: Simd<u32, N_LANES>) {
        inputs.to_array().map(|addr| self.add_inputs(addr));
    }
    pub fn add_inputs(&mut self, addr: u32) {
        *self.addr_to_mult.entry(addr).or_default() += 1;
    }
    pub fn build(self) -> InstMemProver {
        let inputs = self
            .addr_to_mult
            .into_iter()
            .map(|(addr, mult)| InstMemInput { addr, mult });
        InstMemProver::new((), inputs)
    }
}

#[derive(Clone)]
pub struct InstMem;

impl Standard for InstMem {
    type LookupElements = MemoryAndRangeElements;
    // TODO: Add big memory and range checks.
    type Input = InstMemInput;
    type PackedInput = PackedInstMemInput;
    type LookupData = InstMemLookupData;
    type Params = ();
    const N_REPETITIONS: usize = 2;

    fn dummy_elements() -> Self::LookupElements {
        MemoryAndRangeElements::dummy()
    }
    fn pad(mut input: Self::Input) -> Self::Input {
        input.mult = 0;
        input
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &(), _start_index: usize) -> Self::LookupData {
        InstMemLookupData {
            log_size,
            addr: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            id: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            mult: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            value: vec![[Simd::splat(0); 2]; 1 << (log_size - LOG_N_LANES)],
        }
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        els: &Self::LookupElements,
        _params: &Self::Params,
        _start_index: usize,
    ) {
        // 16 16 16 15 \cap 18 18 18 =
        // 16 2 14 4 12 6 9
        let mult = eval.next_trace_mask();
        let addr = eval.next_trace_mask();
        let id = eval.next_trace_mask();
        let parts = [0; 7].map(|_| eval.next_trace_mask());

        // Range checks.
        // TODO: Change to 1.
        logup.push_frac(
            eval,
            els.range.rc16.combine_frac(E::F::zero(), &parts[0..=0]),
        );
        logup.push_frac(
            eval,
            els.range.rc2_14.combine_frac(E::F::zero(), &parts[1..=2]),
        );
        logup.push_frac(
            eval,
            els.range.rc4_12.combine_frac(E::F::zero(), &parts[3..=4]),
        );
        logup.push_frac(
            eval,
            els.range.rc6_9.combine_frac(E::F::zero(), &parts[5..=6]),
        );

        // Instruction decoding.
        let offset0 = parts[0];
        let offset1 = parts[1] + parts[2] * M31::from(1 << 2);
        let offset2 = parts[3] + parts[4] * M31::from(1 << 4);
        let flags = parts[5] + parts[6] * M31::from(1 << 6);
        logup.push_frac(
            eval,
            els.mem
                .instructions
                .combine_frac(-mult, &[addr, offset0, offset1, offset2, flags]),
        );

        // Id lookup.
        logup.push_frac(
            eval,
            els.mem.addr_to_id.combine_frac(E::F::one(), &[addr, id]),
        );

        // Memory lookup.
        let limb0 = parts[0] + parts[1] * M31::from(1 << 16);
        let limb1 = parts[2] + parts[3] * M31::from(1 << 14);
        let limb2 = parts[4] + parts[5] * M31::from(1 << 12);
        let limb3 = parts[6];
        logup.push_frac(
            eval,
            els.mem
                .id_to_big
                .combine_frac(E::F::one(), &[id, limb0, limb1, limb2, limb3]),
        );
    }
}

pub struct InstMemCtx<'a> {
    pub addr_to_id: &'a mut AddrToIdBuilder,
    pub id_to_big: &'a mut IdToBigBuilder,
    pub mem: &'a Memory,
}
impl<'a> ContextFor<InstMem> for InstMemCtx<'a> {
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedInstMemInput,
        row_index: usize,
        lookup_data: &mut InstMemLookupData,
    ) {
        let mult = unsafe { PackedM31::from_simd_unchecked(input.mult) };
        let addr = unsafe { PackedM31::from_simd_unchecked(input.addr) };
        dst[0].data[row_index] = mult;
        dst[1].data[row_index] = addr;

        let id = self.addr_to_id.add_inputs_simd(self.mem, input.addr);
        let id_m31 = unsafe { PackedM31::from_simd_unchecked(id.cast()) };
        dst[2].data[row_index] = id_m31;

        let value = self.id_to_big.add_inputs_simd(self.mem, id);
        split_big(&value).iter().enumerate().for_each(|(i, part)| {
            dst[3 + i].data[row_index] = *part;
        });

        lookup_data.mult[row_index] = mult;
        lookup_data.addr[row_index] = addr;
        lookup_data.id[row_index] = id_m31;
        lookup_data.value[row_index] = [value[0], value[1]];
    }
}

pub fn split_big(value: &[Simd<u32, N_LANES>]) -> [PackedM31; 7] {
    let part0 = value[0] & Simd::splat((1 << 16) - 1);
    let part1 = (value[0] >> 16) & Simd::splat((1 << 2) - 1);
    let part2 = value[0] >> 18;
    let part3 = value[1] & Simd::splat((1 << 4) - 1);
    let part4 = (value[1] >> 4) & Simd::splat((1 << 12) - 1);
    let part5 = (value[1] >> 16) & Simd::splat((1 << 6) - 1);
    let part6 = value[1] >> 22;
    [part0, part1, part2, part3, part4, part5, part6]
        .map(|x| unsafe { PackedM31::from_simd_unchecked(x) })
}

#[derive(Clone)]
pub struct InstMemInput {
    pub addr: u32,
    pub mult: u32,
}
pub struct PackedInstMemInput {
    pub addr: Simd<u32, N_LANES>,
    pub mult: Simd<u32, N_LANES>,
}
impl From<[InstMemInput; N_LANES]> for PackedInstMemInput {
    fn from(value: [InstMemInput; N_LANES]) -> Self {
        PackedInstMemInput {
            addr: Simd::from_array(std::array::from_fn(|i| value[i].addr)),
            mult: Simd::from_array(std::array::from_fn(|i| value[i].mult)),
        }
    }
}

pub struct InstMemLookupData {
    log_size: u32,
    addr: Vec<PackedM31>,
    id: Vec<PackedM31>,
    mult: Vec<PackedM31>,
    value: Vec<[Simd<u32, N_LANES>; 2]>,
}

impl StandardLookupData for InstMemLookupData {
    const N_LOOKUPS: usize = 7;

    type Elements = MemoryAndRangeElements;

    // TODO: Ensure length.
    fn lookups<'a>(&'a self, els: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        vec![
            // TODO: Change mults to 1.
            // Range checks.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let parts = split_big(&self.value[row]);
                els.range
                    .rc16
                    .combine_frac(PackedM31::zero(), &parts[0..=0])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let parts = split_big(&self.value[row]);
                els.range
                    .rc2_14
                    .combine_frac(PackedM31::zero(), &parts[1..=2])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let parts = split_big(&self.value[row]);
                els.range
                    .rc4_12
                    .combine_frac(PackedM31::zero(), &parts[3..=4])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let parts = split_big(&self.value[row]);
                els.range
                    .rc6_9
                    .combine_frac(PackedM31::zero(), &parts[5..=6])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Instruction decoding.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let parts = split_big(&self.value[row]);
                let offset0 = parts[0];
                let offset1 = parts[1] + parts[2] * M31::from(1 << 2);
                let offset2 = parts[3] + parts[4] * M31::from(1 << 4);
                let flags = parts[5] + parts[6] * M31::from(1 << 6);
                els.mem.instructions.combine_frac(
                    -self.mult[row],
                    &[self.addr[row], offset0, offset1, offset2, flags],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Id lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                els.mem
                    .addr_to_id
                    .combine_frac(PackedM31::one(), &[self.addr[row], self.id[row]])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Memory lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let parts = split_big(&self.value[row]);
                let limb0 = parts[0] + parts[1] * M31::from(1 << 16);
                let limb1 = parts[2] + parts[3] * M31::from(1 << 14);
                let limb2 = parts[4] + parts[5] * M31::from(1 << 12);
                let limb3 = parts[6];
                els.mem.id_to_big.combine_frac(
                    PackedM31::one(),
                    &[self.id[row], limb0, limb1, limb2, limb3],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

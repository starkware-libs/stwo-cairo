use std::simd::Simd;

use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::lookups::utils::Fraction;

use super::super::{
    LookupFunc, Standard, StandardClaim, StandardComponent, StandardInteractionProver,
    StandardLookupData, StandardProver,
};
use super::PackedVmState;
use crate::components::opcode::{OpcodeElements, OpcodeGenContext};
use crate::components::ContextFor;
use crate::input::instructions::VmState;

pub type GenericOpcodeProver = StandardProver<GenericOpcode>;
pub type GenericOpcodeClaim = StandardClaim<GenericOpcode>;
pub type GenericOpcodeComponent = StandardComponent<GenericOpcode>;
pub type GenericOpcodeInteractionProver = StandardInteractionProver<GenericOpcodeLookupData>;

#[derive(Clone)]
pub struct GenericOpcode;

impl Standard for GenericOpcode {
    type LookupElements = OpcodeElements;
    type Input = GenericInput;
    type PackedInput = PackedGenericInput;
    type LookupData = GenericOpcodeLookupData;
    type Params = ();
    const N_REPETITIONS: usize = 2;

    fn pad(input: GenericInput) -> GenericInput {
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        OpcodeElements::dummy()
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &()) -> Vec<Self::LookupData> {
        (0..2)
            .map(|_| GenericOpcodeLookupData {
                log_size,
                pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                ap: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                new_pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                new_ap: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                new_fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            })
            .collect()
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &OpcodeElements,
        _params: &(),
    ) {
        let pc = eval.next_trace_mask();
        let ap = eval.next_trace_mask();
        let fp = eval.next_trace_mask();
        let new_pc = eval.next_trace_mask();
        let new_ap = eval.next_trace_mask();
        let new_fp = eval.next_trace_mask();

        // State lookups.
        logup.push_lookup(eval, -E::EF::one(), &[pc, ap, fp], &elements.state);
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[new_pc, new_ap, new_fp],
            &elements.state,
        );
    }
}
impl<'a> ContextFor<GenericOpcode> for OpcodeGenContext<'a> {
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedGenericInput,
        row_index: usize,
        lookup_data: &mut GenericOpcodeLookupData,
    ) {
        let pc = unsafe { PackedM31::from_simd_unchecked(input.0[0].pc) };
        let ap = unsafe { PackedM31::from_simd_unchecked(input.0[0].ap) };
        let fp = unsafe { PackedM31::from_simd_unchecked(input.0[0].fp) };
        let new_pc = unsafe { PackedM31::from_simd_unchecked(input.0[1].pc) };
        let new_ap = unsafe { PackedM31::from_simd_unchecked(input.0[1].ap) };
        let new_fp = unsafe { PackedM31::from_simd_unchecked(input.0[1].fp) };
        dst[0].data[row_index] = pc;
        dst[1].data[row_index] = ap;
        dst[2].data[row_index] = fp;
        dst[3].data[row_index] = new_pc;
        dst[4].data[row_index] = new_ap;
        dst[5].data[row_index] = new_fp;

        lookup_data.pc[row_index] = pc;
        lookup_data.ap[row_index] = ap;
        lookup_data.fp[row_index] = fp;
        lookup_data.new_pc[row_index] = new_pc;
        lookup_data.new_ap[row_index] = new_ap;
        lookup_data.new_fp[row_index] = new_fp;
    }
}

#[derive(Clone, Debug)]
pub struct GenericInput(pub [VmState; 2]);

pub struct PackedGenericInput(pub [PackedVmState; 2]);

impl From<[GenericInput; N_LANES]> for PackedGenericInput {
    fn from(value: [GenericInput; N_LANES]) -> Self {
        PackedGenericInput(std::array::from_fn(|i| PackedVmState {
            pc: Simd::from_array(std::array::from_fn(|j| value[j].0[i].pc)),
            ap: Simd::from_array(std::array::from_fn(|j| value[j].0[i].ap)),
            fp: Simd::from_array(std::array::from_fn(|j| value[j].0[i].fp)),
        }))
    }
}

pub struct GenericOpcodeLookupData {
    log_size: u32,
    pc: Vec<PackedM31>,
    ap: Vec<PackedM31>,
    fp: Vec<PackedM31>,
    new_pc: Vec<PackedM31>,
    new_ap: Vec<PackedM31>,
    new_fp: Vec<PackedM31>,
}

impl StandardLookupData for GenericOpcodeLookupData {
    const N_LOOKUPS: usize = 2;

    type Elements = OpcodeElements;

    // TODO: Ensure length.
    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        vec![
            // Input state lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom = elements
                    .state
                    .combine(&[self.pc[row], self.ap[row], self.fp[row]]);
                Fraction::new(-PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Output state lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom =
                    elements
                        .state
                        .combine(&[self.new_pc[row], self.new_ap[row], self.new_fp[row]]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

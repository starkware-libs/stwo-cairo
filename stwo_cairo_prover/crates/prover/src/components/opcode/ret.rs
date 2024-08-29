use std::simd::num::SimdInt;
use std::simd::Simd;

use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;

use super::super::{
    LookupFunc, Standard, StandardClaim, StandardComponent, StandardInteractionProver,
    StandardLookupData, StandardProver,
};
use super::{cairo_offset, PackedVmState};
use crate::components::opcode::{OpcodeElements, OpcodeGenContext};
use crate::components::ContextFor;
use crate::input::instructions::VmState;
use crate::input::mem::Memory;

pub type RetOpcodeProver = StandardProver<RetOpcode>;
pub type RetOpcodeClaim = StandardClaim<RetOpcode>;
pub type RetOpcodeComponent = StandardComponent<RetOpcode>;
pub type RetOpcodeInteractionProver = StandardInteractionProver<RetLookupData>;

const RET_FLAGS: u32 = 0b010000010001011;

#[derive(Clone)]
pub struct RetOpcode;
impl RetOpcodeProver {
    pub fn pad_transition(&self, input: VmState, mem: &Memory) -> (u32, [VmState; 2]) {
        let output_state = VmState {
            pc: mem.get(input.fp - 1).as_small() as u32,
            ap: input.ap,
            fp: mem.get(input.fp - 2).as_small() as u32,
        };
        (self.n_padding(), [input, output_state])
    }
}

impl Standard for RetOpcode {
    type LookupElements = OpcodeElements;
    type Input = VmState;
    type PackedInput = PackedVmState;
    type LookupData = RetLookupData;
    type Params = ();
    const N_REPETITIONS: usize = 2;

    fn pad(input: VmState) -> VmState {
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        OpcodeElements::dummy()
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &()) -> Vec<Self::LookupData> {
        (0..2)
            .map(|_| RetLookupData {
                log_size,
                pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                ap: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                new_pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
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

        // Instruction lookup.
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[
                pc,
                cairo_offset(-2).into(),
                cairo_offset(-1).into(),
                cairo_offset(-1).into(),
                M31::from(RET_FLAGS).into(),
            ],
            &elements.mem.instructions,
        );

        // fp-1 lookup
        let new_pc = eval.next_trace_mask();
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[fp - M31::from(1).into(), new_pc],
            &elements.mem.addr_to_id,
        );

        // fp-2 lookup.
        let new_fp = eval.next_trace_mask();
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[fp - M31::from(2).into(), new_fp],
            &elements.mem.addr_to_id,
        );

        // State lookups.
        logup.push_lookup(eval, -E::EF::one(), &[pc, ap, fp], &elements.state);
        logup.push_lookup(eval, E::EF::one(), &[new_pc, ap, new_fp], &elements.state);
    }
}
impl<'a> ContextFor<RetOpcode> for OpcodeGenContext<'a> {
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedVmState,
        row_index: usize,
        lookup_data: &mut RetLookupData,
    ) {
        let pc = unsafe { PackedM31::from_simd_unchecked(input.pc) };
        let ap = unsafe { PackedM31::from_simd_unchecked(input.ap) };
        let fp = unsafe { PackedM31::from_simd_unchecked(input.fp) };
        dst[0].data[row_index] = pc;
        dst[1].data[row_index] = ap;
        dst[2].data[row_index] = fp;
        lookup_data.pc[row_index] = pc;
        lookup_data.ap[row_index] = ap;
        lookup_data.fp[row_index] = fp;

        self.mem_builder.instruction_mem.add_inputs_simd(input.pc);

        let new_pc = self
            .mem_builder
            .addr_to_id
            .add_inputs_simd(self.mem, input.fp - Simd::splat(1));
        let new_pc = unsafe { PackedM31::from_simd_unchecked(new_pc.cast()) };
        dst[3].data[row_index] = new_pc;
        lookup_data.new_pc[row_index] = new_pc;

        let new_fp = self
            .mem_builder
            .addr_to_id
            .add_inputs_simd(self.mem, input.fp - Simd::splat(2));
        let new_fp = unsafe { PackedM31::from_simd_unchecked(new_fp.cast()) };
        dst[4].data[row_index] = new_fp;
        lookup_data.new_fp[row_index] = new_fp;
    }
}

pub struct RetLookupData {
    log_size: u32,
    pc: Vec<PackedM31>,
    ap: Vec<PackedM31>,
    fp: Vec<PackedM31>,
    new_pc: Vec<PackedM31>,
    new_fp: Vec<PackedM31>,
}

impl StandardLookupData for RetLookupData {
    const N_LOOKUPS: usize = 5;

    type Elements = OpcodeElements;

    // TODO: Ensure length.
    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        vec![
            // Instruction lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom = elements.mem.instructions.combine(&[
                    self.pc[row],
                    cairo_offset(-2).into(),
                    cairo_offset(-1).into(),
                    cairo_offset(-1).into(),
                    M31::from(RET_FLAGS).into(),
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // fp-1 lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom = elements.mem.addr_to_id.combine(&[
                    self.fp[row] - PackedM31::broadcast(M31::from(1)),
                    self.new_pc[row],
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // fp-2 lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom = elements.mem.addr_to_id.combine(&[
                    self.fp[row] - PackedM31::broadcast(M31::from(2)),
                    self.new_fp[row],
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
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
                        .combine(&[self.new_pc[row], self.ap[row], self.new_fp[row]]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

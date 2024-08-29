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

use super::super::{LookupFunc, Standard, StandardLookupData};
use super::{cairo_offset, PackedVmState};
use crate::components::opcode::{OpcodeElements, OpcodeGenContext};
use crate::components::{
    ContextFor, StandardClaimStack, StandardComponentStack, StandardInteractionProverStack,
    StandardProverStack,
};
use crate::input::instructions::VmState;
use crate::input::mem::Memory;

pub type RetOpcodeProver = StandardProverStack<RetOpcode>;
pub type RetOpcodeClaim = StandardClaimStack<RetOpcode>;
pub type RetOpcodeComponent = StandardComponentStack<RetOpcode>;
pub type RetOpcodeInteractionProver = StandardInteractionProverStack<RetLookupData>;

const RET_FLAGS: u32 = 0b010000010001011;

#[derive(Clone)]
pub struct RetOpcode;
impl RetOpcodeProver {
    pub fn pad_transition(&self, mem: &Memory) -> (u32, [VmState; 2]) {
        if self.0.is_empty() {
            return (0, [VmState::default(), VmState::default()]);
        }
        let input_state = self.0.last().unwrap().inputs[0].first();
        let output_state = VmState {
            pc: mem.get(input_state.fp - 1).as_small() as u32,
            ap: input_state.ap,
            fp: mem.get(input_state.fp - 2).as_small() as u32,
        };
        assert!(self.0[..self.0.len() - 1]
            .iter()
            .all(|x| x.n_padding() == 0));
        let n_padding = self.0.last().unwrap().n_padding();
        (n_padding, [input_state, output_state])
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
    fn new_lookup_data(log_size: u32, _params: &(), _start_index: usize) -> Self::LookupData {
        RetLookupData {
            log_size,
            pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            ap: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            new_pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            new_fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
        }
    }
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &OpcodeElements,
        _params: &(),
        _start_index: usize,
    ) {
        let pc = eval.next_trace_mask();
        let ap = eval.next_trace_mask();
        let fp = eval.next_trace_mask();

        // Instruction lookup.
        logup.push_frac(
            eval,
            elements.mem.instructions.combine_frac(
                E::F::one(),
                &[
                    pc,
                    cairo_offset(-2).into(),
                    cairo_offset(-1).into(),
                    cairo_offset(-1).into(),
                    M31::from(RET_FLAGS).into(),
                ],
            ),
        );

        // fp-1 lookup
        let new_pc = eval.next_trace_mask();
        logup.push_frac(
            eval,
            elements
                .mem
                .addr_to_id
                .combine_frac(E::F::one(), &[fp - M31::from(1).into(), new_pc]),
        );

        // fp-2 lookup.
        let new_fp = eval.next_trace_mask();
        logup.push_frac(
            eval,
            elements
                .mem
                .addr_to_id
                .combine_frac(E::F::one(), &[fp - M31::from(2).into(), new_fp]),
        );

        // State lookups.
        logup.push_frac(
            eval,
            elements.state.combine_frac(-E::F::one(), &[pc, ap, fp]),
        );
        logup.push_frac(
            eval,
            elements
                .state
                .combine_frac(E::F::one(), &[new_pc, ap, new_fp]),
        );
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
        assert!(new_pc.to_array().iter().all(|x| x < &(1 << 26)));
        let new_pc = unsafe { PackedM31::from_simd_unchecked(new_pc.cast()) };
        dst[3].data[row_index] = new_pc;
        lookup_data.new_pc[row_index] = new_pc;

        let new_fp = self
            .mem_builder
            .addr_to_id
            .add_inputs_simd(self.mem, input.fp - Simd::splat(2));
        assert!(new_fp.to_array().iter().all(|x| x < &(1 << 26)));
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
                elements.mem.instructions.combine_frac(
                    PackedM31::one(),
                    &[
                        self.pc[row],
                        cairo_offset(-2).into(),
                        cairo_offset(-1).into(),
                        cairo_offset(-1).into(),
                        M31::from(RET_FLAGS).into(),
                    ],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // fp-1 lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                elements.mem.addr_to_id.combine_frac(
                    PackedM31::one(),
                    &[
                        self.fp[row] - PackedM31::broadcast(M31::from(1)),
                        self.new_pc[row],
                    ],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // fp-2 lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                elements.mem.addr_to_id.combine_frac(
                    PackedM31::one(),
                    &[
                        self.fp[row] - PackedM31::broadcast(M31::from(2)),
                        self.new_fp[row],
                    ],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Input state lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                elements.state.combine_frac(
                    -PackedM31::one(),
                    &[self.pc[row], self.ap[row], self.fp[row]],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Output state lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                elements.state.combine_frac(
                    PackedM31::one(),
                    &[self.new_pc[row], self.ap[row], self.new_fp[row]],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

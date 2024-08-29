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
use crate::components::memory::m31_from_i32;
use crate::components::opcode::{OpcodeElements, OpcodeGenContext};
use crate::components::{
    ContextFor, StandardClaimStack, StandardComponentStack, StandardInteractionProverStack,
    StandardProverStack,
};
use crate::input::instructions::VmState;
use crate::input::mem::Memory;

pub type JmpRelOpcodeProver<const INC_AP: bool> = StandardProverStack<JmpRelOpcode<INC_AP>>;
pub type JmpRelOpcodeClaim<const INC_AP: bool> = StandardClaimStack<JmpRelOpcode<INC_AP>>;
pub type JmpRelOpcodeComponent<const INC_AP: bool> = StandardComponentStack<JmpRelOpcode<INC_AP>>;
pub type JmpRelOpcodeInteractionProver<const INC_AP: bool> =
    StandardInteractionProverStack<JmpRelOpcodeLookupData<INC_AP>>;

const JMP_ABS_FLAG: u32 = 0b000000100000111;
const FLAG_INC_AP_BIT: u32 = 11;

#[derive(Clone)]
pub struct JmpRelOpcode<const INC_AP: bool>;
impl<const INC_AP: bool> JmpRelOpcode<INC_AP> {
    const FLAGS: u32 = { JMP_ABS_FLAG + if INC_AP { 1 << FLAG_INC_AP_BIT } else { 0 } };
}

impl<const INC_AP: bool> JmpRelOpcodeProver<INC_AP> {
    pub fn pad_transition(&self, mem: &Memory) -> (u32, [VmState; 2]) {
        if self.0.is_empty() {
            return (0, [VmState::default(), VmState::default()]);
        }
        let input = self.0.last().unwrap().inputs[0].first();
        let imm = mem.get(input.pc + 1).as_small();
        let new_pc = (input.pc as i32 + imm) as u32;
        let output_state = VmState {
            pc: new_pc,
            ap: input.ap + if INC_AP { 1 } else { 0 },
            fp: input.fp,
        };
        assert!(self.0[..self.0.len() - 1]
            .iter()
            .all(|x| x.n_padding() == 0));
        let n_padding = self.0.last().unwrap().n_padding();
        (n_padding, [input, output_state])
    }
}

impl<const INC_AP: bool> Standard for JmpRelOpcode<INC_AP> {
    type LookupElements = OpcodeElements;
    type Input = VmState;
    type PackedInput = PackedVmState;
    type LookupData = JmpRelOpcodeLookupData<INC_AP>;
    type Params = ();
    const N_REPETITIONS: usize = 1;

    fn pad(input: VmState) -> VmState {
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        OpcodeElements::dummy()
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &(), _start_index: usize) -> Self::LookupData {
        JmpRelOpcodeLookupData {
            log_size,
            pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            ap: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
            imm: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
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
        let imm = eval.next_trace_mask();

        // Instruction lookup.
        logup.push_frac(
            eval,
            elements.mem.instructions.combine_frac(
                E::F::one(),
                &[
                    pc,
                    cairo_offset(-1).into(),
                    cairo_offset(-1).into(),
                    cairo_offset(1).into(),
                    M31::from(Self::FLAGS).into(),
                ],
            ),
        );

        // imm lookup
        logup.push_frac(
            eval,
            elements
                .mem
                .addr_to_id
                .combine_frac(E::F::one(), &[pc + E::F::one(), imm]),
        );
        let new_pc = pc + imm;

        // Soundness: To avoid checking that imm is small, we rely on the fact that pc and new_pc
        // are guaranteed to be addresses. Therefore, imm is a difference between two addresses.
        // We should make sure this implies that imm is small.

        let new_ap = if INC_AP { ap + E::F::one() } else { ap };

        // State lookups.
        logup.push_frac(
            eval,
            elements.state.combine_frac(-E::F::one(), &[pc, ap, fp]),
        );
        logup.push_frac(
            eval,
            elements
                .state
                .combine_frac(E::F::one(), &[new_pc, new_ap, fp]),
        );
    }
}
impl<'a, const INC_AP: bool> ContextFor<JmpRelOpcode<INC_AP>> for OpcodeGenContext<'a> {
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedVmState,
        row_index: usize,
        lookup_data: &mut JmpRelOpcodeLookupData<INC_AP>,
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

        let imm = self
            .mem_builder
            .addr_to_id
            .add_inputs_simd(self.mem, input.pc + Simd::splat(1));
        let imm = m31_from_i32(imm.cast());
        dst[3].data[row_index] = imm;
        lookup_data.imm[row_index] = imm;
    }
}

pub struct JmpRelOpcodeLookupData<const INC_AP: bool> {
    log_size: u32,
    pc: Vec<PackedM31>,
    ap: Vec<PackedM31>,
    fp: Vec<PackedM31>,
    imm: Vec<PackedM31>,
}

impl<const INC_AP: bool> StandardLookupData for JmpRelOpcodeLookupData<INC_AP> {
    const N_LOOKUPS: usize = 4;

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
                        cairo_offset(-1).into(),
                        cairo_offset(-1).into(),
                        cairo_offset(1).into(),
                        M31::from(JmpRelOpcode::<INC_AP>::FLAGS).into(),
                    ],
                )
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // imm lookup
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                elements.mem.addr_to_id.combine_frac(
                    PackedM31::one(),
                    &[self.pc[row] + PackedM31::one(), self.imm[row]],
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
                let new_ap = if INC_AP {
                    self.ap[row] + PackedM31::one()
                } else {
                    self.ap[row]
                };
                let new_pc = self.pc[row] + self.imm[row];
                elements
                    .state
                    .combine_frac(PackedM31::one(), &[new_pc, new_ap, self.fp[row]])
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

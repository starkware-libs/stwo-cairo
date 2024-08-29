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
use super::{cairo_offset, PackedVmState, CAIRO_OFFSET_SHIFT};
use crate::components::opcode::{OpcodeElements, OpcodeGenContext};
use crate::components::{
    ContextFor, StandardClaimStack, StandardComponentStack, StandardInteractionProverStack,
    StandardProverStack,
};
use crate::input::instructions::VmState;
use crate::input::mem::Memory;

pub type JmpAbsOpcodeProver<const OP1_FP: bool, const INC_AP: bool> =
    StandardProverStack<JmpAbsOpcode<OP1_FP, INC_AP>>;
pub type JmpAbsOpcodeClaim<const OP1_FP: bool, const INC_AP: bool> =
    StandardClaimStack<JmpAbsOpcode<OP1_FP, INC_AP>>;
pub type JmpAbsOpcodeComponent<const OP1_FP: bool, const INC_AP: bool> =
    StandardComponentStack<JmpAbsOpcode<OP1_FP, INC_AP>>;
pub type JmpAbsOpcodeInteractionProver<const OP1_FP: bool, const INC_AP: bool> =
    StandardInteractionProverStack<JmpAbsOpcodeLookupData<OP1_FP, INC_AP>>;

const JMP_ABS_FLAG: u32 = 0b000000010000011;
const FLAG_OP1_BASE_FP_BIT: u32 = 3;
const FLAG_OP1_BASE_AP_BIT: u32 = 4;
const FLAG_INC_AP_BIT: u32 = 11;

#[derive(Clone)]
pub struct JmpAbsOpcode<const OP1_FP: bool, const INC_AP: bool>;
impl<const OP1_FP: bool, const INC_AP: bool> JmpAbsOpcode<OP1_FP, INC_AP> {
    const FLAGS: u32 = {
        JMP_ABS_FLAG
            + if OP1_FP {
                1 << FLAG_OP1_BASE_FP_BIT
            } else {
                1 << FLAG_OP1_BASE_AP_BIT
            }
            + if INC_AP { 1 << FLAG_INC_AP_BIT } else { 0 }
    };
}

impl<const OP1_FP: bool, const INC_AP: bool> JmpAbsOpcodeProver<OP1_FP, INC_AP> {
    pub fn pad_transition(&self, mem: &Memory) -> (u32, [VmState; 2]) {
        if self.0.is_empty() {
            return (0, [VmState::default(), VmState::default()]);
        }
        let input = self.0.last().unwrap().inputs[0].first();
        let offset2 = ((mem.get_inst(input.pc).unwrap() >> 32) & 0xFF) as u32;
        let addr = if OP1_FP { input.fp } else { input.ap } + offset2 - CAIRO_OFFSET_SHIFT;
        let new_pc = mem.get(addr).as_small() as u32;
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

impl<const OP1_FP: bool, const INC_AP: bool> Standard for JmpAbsOpcode<OP1_FP, INC_AP> {
    type LookupElements = OpcodeElements;
    type Input = VmState;
    type PackedInput = PackedVmState;
    type LookupData = JmpAbsOpcodeLookupData<OP1_FP, INC_AP>;
    type Params = ();
    const N_REPETITIONS: usize = 1;

    fn pad(input: VmState) -> VmState {
        input
    }
    fn dummy_elements() -> Self::LookupElements {
        OpcodeElements::dummy()
    }
    fn dummy_params() -> Self::Params {}
    fn new_lookup_data(log_size: u32, _params: &()) -> Vec<Self::LookupData> {
        (0..2)
            .map(|_| JmpAbsOpcodeLookupData {
                log_size,
                pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                ap: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                fp: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                offset2: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
                new_pc: vec![PackedM31::zero(); 1 << (log_size - LOG_N_LANES)],
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
        let offset2 = eval.next_trace_mask();
        let new_pc = eval.next_trace_mask();

        // Instruction lookup.
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[
                pc,
                cairo_offset(-1).into(),
                cairo_offset(-1).into(),
                offset2,
                M31::from(Self::FLAGS).into(),
            ],
            &elements.mem.instructions,
        );

        // op1 lookup
        let addr = if OP1_FP { fp } else { ap } + offset2 - M31::from(CAIRO_OFFSET_SHIFT).into();
        logup.push_lookup(
            eval,
            E::EF::one(),
            &[addr, new_pc],
            &elements.mem.addr_to_id,
        );

        let new_ap = if INC_AP { ap + E::F::one() } else { ap };

        // State lookups.
        logup.push_lookup(eval, -E::EF::zero(), &[pc, ap, fp], &elements.state);
        logup.push_lookup(eval, E::EF::zero(), &[new_pc, new_ap, fp], &elements.state);
    }
}
impl<'a, const OP1_FP: bool, const INC_AP: bool> ContextFor<JmpAbsOpcode<OP1_FP, INC_AP>>
    for OpcodeGenContext<'a>
{
    fn write_trace_row(
        &mut self,
        dst: &mut [BaseColumn],
        input: &PackedVmState,
        row_index: usize,
        lookup_data: &mut JmpAbsOpcodeLookupData<OP1_FP, INC_AP>,
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
        let insts: [u64; N_LANES] =
            std::array::from_fn(|i| self.mem.get_inst(input.pc.to_array()[i]).unwrap());
        let offset2 = Simd::from_array(insts.map(|x| ((x >> 32) & 0xFF) as u32));
        let addr = if OP1_FP { input.fp } else { input.ap } + offset2;

        let offset2 = unsafe { PackedM31::from_simd_unchecked(offset2) };
        dst[3].data[row_index] = offset2;
        lookup_data.offset2[row_index] = offset2;

        let new_pc = self.mem_builder.addr_to_id.add_inputs_simd(self.mem, addr);
        let new_pc = unsafe { PackedM31::from_simd_unchecked(new_pc.cast()) };
        dst[4].data[row_index] = new_pc;
        lookup_data.new_pc[row_index] = new_pc;
    }
}

pub struct JmpAbsOpcodeLookupData<const OP1_FP: bool, const INC_AP: bool> {
    log_size: u32,
    pc: Vec<PackedM31>,
    ap: Vec<PackedM31>,
    fp: Vec<PackedM31>,
    offset2: Vec<PackedM31>,
    new_pc: Vec<PackedM31>,
}

impl<const OP1_FP: bool, const INC_AP: bool> StandardLookupData
    for JmpAbsOpcodeLookupData<OP1_FP, INC_AP>
{
    const N_LOOKUPS: usize = 4;

    type Elements = OpcodeElements;

    // TODO: Ensure length.
    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>> {
        vec![
            // Instruction lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom = elements.mem.instructions.combine(&[
                    self.pc[row],
                    cairo_offset(-1).into(),
                    cairo_offset(-1).into(),
                    self.offset2[row],
                    M31::from(JmpAbsOpcode::<OP1_FP, INC_AP>::FLAGS).into(),
                ]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // op1 lookup
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let addr = if OP1_FP { self.fp[row] } else { self.ap[row] } + self.offset2[row]
                    - M31::from(CAIRO_OFFSET_SHIFT).into();
                let denom = elements.mem.addr_to_id.combine(&[addr, self.new_pc[row]]);
                Fraction::new(PackedM31::one(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Input state lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let denom = elements
                    .state
                    .combine(&[self.pc[row], self.ap[row], self.fp[row]]);
                Fraction::new(-PackedM31::zero(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
            // Output state lookup.
            Box::new((0..(1 << (self.log_size - LOG_N_LANES))).map(|row| {
                let new_ap = if INC_AP {
                    self.ap[row] + PackedM31::one()
                } else {
                    self.ap[row]
                };
                let denom = elements
                    .state
                    .combine(&[self.new_pc[row], new_ap, self.fp[row]]);
                Fraction::new(PackedM31::zero(), denom)
            })) as Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>>>,
        ]
    }
}

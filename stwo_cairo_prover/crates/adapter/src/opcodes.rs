use std::fmt::Display;

use serde::{Deserialize, Serialize};
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_prover::core::fields::m31::M31;
use tracing::{span, Level};

use super::decode::{Instruction, OpcodeExtension};
use super::memory::{MemoryBuilder, MemoryValue};
use super::vm_import::RelocatedTraceEntry;

// Small add operands are 27 bits.
const SMALL_ADD_MAX_VALUE: i32 = 2_i32.pow(27) - 1;
const SMALL_ADD_MIN_VALUE: i32 = -(2_i32.pow(27));

// Small mul operands are 36 bits.
const SMALL_MUL_MAX_VALUE: u64 = 2_u64.pow(36) - 1;
const SMALL_MUL_MIN_VALUE: u64 = 0;

// TODO (Stav): Ensure it stays synced with that opcdode AIR's list.
/// This struct holds the components used to prove the opcodes in a Cairo program,
/// and should match the opcode's air used by `stwo-cairo-air`.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CasmStatesByOpcode {
    pub generic_opcode: Vec<CasmState>,
    pub add_ap_opcode: Vec<CasmState>,
    pub add_opcode: Vec<CasmState>,
    pub add_opcode_small: Vec<CasmState>,
    pub assert_eq_opcode: Vec<CasmState>,
    pub assert_eq_opcode_double_deref: Vec<CasmState>,
    pub assert_eq_opcode_imm: Vec<CasmState>,
    pub call_opcode: Vec<CasmState>,
    pub call_opcode_rel: Vec<CasmState>,
    pub call_opcode_op_1_base_fp: Vec<CasmState>,
    pub jnz_opcode: Vec<CasmState>,
    pub jnz_opcode_taken: Vec<CasmState>,
    pub jump_opcode_rel_imm: Vec<CasmState>,
    pub jump_opcode_rel: Vec<CasmState>,
    pub jump_opcode_double_deref: Vec<CasmState>,
    pub jump_opcode: Vec<CasmState>,
    pub mul_opcode_small: Vec<CasmState>,
    pub mul_opcode: Vec<CasmState>,
    pub ret_opcode: Vec<CasmState>,
    pub blake2s_opcode: Vec<CasmState>,
    pub qm31_add_mul_opcode: Vec<CasmState>,
}
impl CasmStatesByOpcode {
    fn from_iter(
        iter: impl DoubleEndedIterator<Item = RelocatedTraceEntry>,
        memory: &MemoryBuilder,
    ) -> Self {
        let mut res = CasmStatesByOpcode::default();
        for entry in iter {
            res.push_instr(memory, entry.into());
        }
        res
    }

    /// Pushes the state transition at pc into the appropriate opcode component.
    fn push_instr(&mut self, memory: &MemoryBuilder, state: CasmState) {
        let CasmState { ap, fp, pc } = state;
        let encoded_instruction = memory.get_inst(pc.0);
        let instruction = Instruction::decode(encoded_instruction);

        match instruction {
            // ret.
            Instruction {
                offset0: -2,
                offset1: -1,
                offset2: -1,
                dst_base_fp: true,
                op0_base_fp: true,
                op_1_imm: false,
                op_1_base_fp: true,
                op_1_base_ap: false,
                res_add: false,
                res_mul: false,
                pc_update_jump: true,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: false,
                opcode_call: false,
                opcode_ret: true,
                opcode_assert_eq: false,
                opcode_extension: OpcodeExtension::Stone,
            } => self.ret_opcode.push(state),

            // add ap.
            Instruction {
                offset0: -1,
                offset1: -1,
                offset2,
                dst_base_fp: true,
                op0_base_fp: true,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: true,
                ap_update_add_1: false,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                // ap += imm.
                // ap += [ap/fp + offset2].
                assert_eq!(
                    (op_1_imm as u8) + (op_1_base_fp as u8) + (op_1_base_ap as u8),
                    1,
                    "add_ap opcode requires exactly one of op_1_imm, op_1_base_fp, op_1_base_ap must be true"
                );
                assert!(
                    (!op_1_imm) || offset2 == 1,
                    "add_ap opcode requires that if op_1_imm is true, offset2 must be 1"
                );
                self.add_ap_opcode.push(state);
            }
            // jump.
            Instruction {
                offset0: -1,
                offset1,
                offset2,
                dst_base_fp: true,
                op0_base_fp,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump,
                pc_update_jump_rel,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                if op_1_imm {
                    // jump rel imm.
                    assert!(
                        pc_update_jump_rel
                            && !pc_update_jump
                            && !op_1_base_fp
                            && !op_1_base_ap
                            && op0_base_fp
                            && offset1 == -1
                            && offset2 == 1
                    );
                    self.jump_opcode_rel_imm.push(state);
                } else if pc_update_jump_rel {
                    // jump rel [ap/fp + offset2].
                    assert!(
                        !pc_update_jump
                            && (op_1_base_fp || op_1_base_ap)
                            && op0_base_fp
                            && offset1 == -1
                    );
                    self.jump_opcode_rel.push(state);
                } else if !op_1_base_fp && !op_1_base_ap {
                    // jump abs [[ap/fp + offset1] + offset2].
                    assert!(pc_update_jump);
                    self.jump_opcode_double_deref.push(state);
                } else {
                    // jump abs [ap/fp + offset2].
                    assert!(
                        (op_1_base_fp || op_1_base_ap)
                            && op0_base_fp
                            && pc_update_jump
                            && offset1 == -1
                    );
                    self.jump_opcode.push(state);
                }
            }

            // call.
            Instruction {
                offset0: 0,
                offset1: 1,
                offset2,
                dst_base_fp: false,
                op0_base_fp: false,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump,
                pc_update_jump_rel,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: false,
                opcode_call: true,
                opcode_ret: false,
                opcode_assert_eq: false,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                if pc_update_jump_rel {
                    // call rel imm.
                    assert!(
                        op_1_imm
                            && !op_1_base_fp
                            && !op_1_base_ap
                            && offset2 == 1
                            && !pc_update_jump
                    );
                    self.call_opcode_rel.push(state);
                } else if op_1_base_fp {
                    // call abs [fp + offset2].
                    assert!(!op_1_base_ap && !op_1_imm && pc_update_jump);
                    self.call_opcode_op_1_base_fp.push(state);
                } else {
                    // call abs [ap + offset2].
                    assert!(op_1_base_ap && !op_1_imm && pc_update_jump);
                    self.call_opcode.push(state);
                }
            }

            // jnz.
            Instruction {
                offset0,
                offset1: -1,
                offset2: 1,
                dst_base_fp,
                op0_base_fp: true,
                op_1_imm: true,
                op_1_base_fp: false,
                op_1_base_ap: false,
                res_add: false,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: true,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                // jump rel imm if [ap/fp + offset0] != 0.
                let dst_addr = if dst_base_fp { fp } else { ap };
                let dst = memory.get(dst_addr.0.checked_add_signed(offset0 as i32).unwrap());
                let taken = dst != MemoryValue::Small(0);
                if taken {
                    self.jnz_opcode_taken.push(state);
                } else {
                    self.jnz_opcode.push(state);
                }
            }

            // assert equal.
            Instruction {
                offset0: _,
                offset1,
                offset2,
                dst_base_fp: _,
                op0_base_fp,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: true,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                if op_1_imm {
                    // [ap/fp + offset0] = imm.
                    assert!(
                        !op_1_base_fp
                            && !op_1_base_ap
                            && offset2 == 1
                            && op0_base_fp
                            && offset1 == -1
                    );
                    self.assert_eq_opcode_imm.push(state);
                } else if !op_1_base_fp && !op_1_base_ap {
                    // [ap/fp + offset0] = [[ap/fp + offset1] + offset2].
                    self.assert_eq_opcode_double_deref.push(state);
                } else {
                    // [ap/fp + offset0] = [ap/fp + offset1].
                    assert!((op_1_base_fp || op_1_base_ap) && offset1 == -1 && op0_base_fp);
                    self.assert_eq_opcode.push(state);
                }
            }

            // mul.
            Instruction {
                offset0: _,
                offset1,
                offset2,
                dst_base_fp: _,
                op0_base_fp,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add: false,
                res_mul: true,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: true,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                let (op0_addr, op_1_addr) = (
                    if op0_base_fp { fp } else { ap },
                    if op_1_imm {
                        pc
                    } else if op_1_base_fp {
                        fp
                    } else {
                        ap
                    },
                );
                let (op0, op_1) = (
                    memory.get(op0_addr.0.checked_add_signed(offset1 as i32).unwrap()),
                    memory.get(op_1_addr.0.checked_add_signed(offset2 as i32).unwrap()),
                );

                // [ap/fp + offset0] = [ap/fp + offset1] * imm.
                // [ap/fp + offset0] = [ap/fp + offset1] * [ap/fp + offset2].
                assert_eq!(
                    (op_1_imm as u8) + (op_1_base_fp as u8) + (op_1_base_ap as u8),
                    1,
                    "mul opcode requires exactly one of op_1_imm, op_1_base_fp, op_1_base_ap must be true"
                );
                assert!(
                    (!op_1_imm) || offset2 == 1,
                    "mul opcode requires that if op_1_imm is true, offset2 must be 1"
                );
                if is_small_mul(op0, op_1) {
                    self.mul_opcode_small.push(state);
                } else {
                    self.mul_opcode.push(state);
                }
            }

            // add.
            Instruction {
                offset0,
                offset1,
                offset2,
                dst_base_fp,
                op0_base_fp,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add: true,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: true,
                opcode_extension: OpcodeExtension::Stone,
            } => {
                let (dst_addr, op0_addr, op_1_addr) = (
                    if dst_base_fp { fp } else { ap },
                    if op0_base_fp { fp } else { ap },
                    if op_1_imm {
                        pc
                    } else if op_1_base_fp {
                        fp
                    } else {
                        ap
                    },
                );
                let (dst, op0, op_1) = (
                    memory.get(dst_addr.0.checked_add_signed(offset0 as i32).unwrap()),
                    memory.get(op0_addr.0.checked_add_signed(offset1 as i32).unwrap()),
                    memory.get(op_1_addr.0.checked_add_signed(offset2 as i32).unwrap()),
                );

                // [ap/fp + offset0] = [ap/fp + offset1] + imm.
                // [ap/fp + offset0] = [ap/fp + offset1] + [ap/fp + offset2].
                assert_eq!(
                    (op_1_imm as u8) + (op_1_base_fp as u8) + (op_1_base_ap as u8),
                    1,
                    "add opcode requires exactly one of op_1_imm, op_1_base_fp, op_1_base_ap must be true"
                );
                assert!(
                    (!op_1_imm) || offset2 == 1,
                    "add opcode requires that if op_1_imm is true, offset2 must be 1"
                );
                if is_small_add(dst, op0, op_1) {
                    self.add_opcode_small.push(state);
                } else {
                    self.add_opcode.push(state);
                }
            }

            // Blake.
            Instruction {
                offset0: _,
                offset1: _,
                offset2: _,
                dst_base_fp: _,
                op0_base_fp: _,
                op_1_imm: false,
                op_1_base_fp,
                op_1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
                opcode_extension: OpcodeExtension::Blake | OpcodeExtension::BlakeFinalize,
            } => {
                assert!(
                    op_1_base_fp ^ op_1_base_ap,
                    "Blake opcode requires exactly one of op_1_base_fp and op_1_base_ap to be true"
                );
                self.blake2s_opcode.push(state);
            }

            // QM31 add mul.
            Instruction {
                offset0: _,
                offset1: _,
                offset2,
                dst_base_fp: _,
                op0_base_fp: _,
                op_1_imm,
                op_1_base_fp,
                op_1_base_ap,
                res_add,
                res_mul,
                pc_update_jump: false,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: _,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: true,
                opcode_extension: OpcodeExtension::QM31Operation,
            } => {
                // [ap/fp + offset0] = [ap/fp + offset1] +/* [ap/fp/pc + offset2]
                assert_eq!(
                    (op_1_imm as u8) + (op_1_base_fp as u8) + (op_1_base_ap as u8),
                    1,
                    "qm31_add_mul opcode requires exactly one of op_1_imm, op_1_base_fp, op_1_base_ap must be true"
                );
                assert!(
                    res_add ^ res_mul,
                    "qm31_add_mul opcode requires exactly one of res_add, res_mul must be true"
                );
                assert!(
                    (!op_1_imm) || offset2 == 1,
                    "qm31_add_mul opcode requires that if op_1_imm is true, offset2 must be 1"
                );
                self.qm31_add_mul_opcode.push(state);
            }

            // generic opcode.
            _ => {
                if !matches!(instruction.opcode_extension, OpcodeExtension::Stone) {
                    panic!("`generic_opcode` component supports `Stone` opcodes only.");
                }
                self.generic_opcode.push(state);
            }
        }
    }
    pub fn counts(&self) -> Vec<(String, usize)> {
        vec![
            ("generic_opcode".to_string(), self.generic_opcode.len()),
            ("add_ap_opcode".to_string(), self.add_ap_opcode.len()),
            ("add_opcode".to_string(), self.add_opcode.len()),
            ("add_opcode_small".to_string(), self.add_opcode_small.len()),
            ("assert_eq_opcode".to_string(), self.assert_eq_opcode.len()),
            (
                "assert_eq_opcode_double_deref".to_string(),
                self.assert_eq_opcode_double_deref.len(),
            ),
            (
                "assert_eq_opcode_imm".to_string(),
                self.assert_eq_opcode_imm.len(),
            ),
            ("call_opcode".to_string(), self.call_opcode.len()),
            ("call_opcode_rel".to_string(), self.call_opcode_rel.len()),
            (
                "call_opcode_op_1_base_fp".to_string(),
                self.call_opcode_op_1_base_fp.len(),
            ),
            ("jnz_opcode".to_string(), self.jnz_opcode.len()),
            ("jnz_opcode_taken".to_string(), self.jnz_opcode_taken.len()),
            (
                "jump_opcode_rel_imm".to_string(),
                self.jump_opcode_rel_imm.len(),
            ),
            ("jump_opcode_rel".to_string(), self.jump_opcode_rel.len()),
            (
                "jump_opcode_double_deref".to_string(),
                self.jump_opcode_double_deref.len(),
            ),
            ("jump_opcode".to_string(), self.jump_opcode.len()),
            ("mul_opcode_small".to_string(), self.mul_opcode_small.len()),
            ("mul_opcode".to_string(), self.mul_opcode.len()),
            ("ret_opcode".to_string(), self.ret_opcode.len()),
            ("blake2s_opcode".to_string(), self.blake2s_opcode.len()),
            (
                "qm31_add_mul_opcode".to_string(),
                self.qm31_add_mul_opcode.len(),
            ),
        ]
    }
}

impl Display for CasmStatesByOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let counts = self.counts();
        let total_steps = counts.iter().map(|(_, count)| count).sum::<usize>();
        writeln!(f, "Total steps: {total_steps}")?;
        for (name, count) in &counts {
            writeln!(f, "{name}: {count}")?;
        }
        Ok(())
    }
}

impl From<RelocatedTraceEntry> for CasmState {
    fn from(entry: RelocatedTraceEntry) -> Self {
        Self {
            pc: M31(entry.pc as u32),
            ap: M31(entry.ap as u32),
            fp: M31(entry.fp as u32),
        }
    }
}

/// Holds the state transitions of a Cairo program, split according to the components responsible
/// for proving each transition.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StateTransitions {
    pub initial_state: CasmState,
    pub final_state: CasmState,
    pub casm_states_by_opcode: CasmStatesByOpcode,
}

impl StateTransitions {
    /// Iterates over the casm states and splits them into the appropriate opcode components.
    ///
    /// # Returns
    ///
    /// - StateTransitions, used to feed the opcodes' air.
    /// - A map from pc to instruction that is used to feed
    ///   [`crate::cairo_air::components::verify_instruction::ClaimGenerator`].
    pub fn from_iter(
        iter: impl DoubleEndedIterator<Item = RelocatedTraceEntry>,
        memory: &MemoryBuilder,
    ) -> Self {
        let _span = span!(Level::INFO, "StateTransitions::from_iter").entered();
        let mut iter = iter.peekable();

        let initial_state = (*iter.peek().expect("Must have an initial state.")).into();

        // Assuming the last instruction is jrl0, no need to push it.
        let final_state = iter.next_back().unwrap().into();

        let states = CasmStatesByOpcode::from_iter(iter, memory);

        StateTransitions {
            initial_state,
            final_state,
            casm_states_by_opcode: states,
        }
    }
}

fn is_within_range(val: MemoryValue, min: i128, max: i128) -> bool {
    matches!(val, MemoryValue::Small(val) if (val as i128 >= min) && (val as i128 <= max))
}

// Returns 'true' if all the operands are within the range of [-2^27, 2^27 - 1].
fn is_small_add(dst: MemoryValue, op0: MemoryValue, op_1: MemoryValue) -> bool {
    [dst, op0, op_1].iter().all(|val| {
        is_within_range(
            *val,
            SMALL_ADD_MIN_VALUE as i128,
            SMALL_ADD_MAX_VALUE as i128,
        )
    })
}

// Returns 'true' the multiplication factors are in the range [0, 2^36-1].
fn is_small_mul(op0: MemoryValue, op_1: MemoryValue) -> bool {
    [op0, op_1].iter().all(|val| {
        is_within_range(
            *val,
            SMALL_MUL_MIN_VALUE as i128,
            SMALL_MUL_MAX_VALUE as i128,
        )
    })
}

/// Tests instructions mapping.
#[cfg(test)]
mod mappings_tests {

    use cairo_lang_casm::casm;
    use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
    use cairo_vm::types::layout_name::LayoutName;
    use cairo_vm::vm::runners::cairo_runner::CairoRunner;
    use stwo_cairo_common::prover_types::cpu::CasmState;
    use stwo_prover::core::fields::m31::M31;

    use crate::decode::{Instruction, OpcodeExtension};
    use crate::memory::*;
    use crate::opcodes::{CasmStatesByOpcode, StateTransitions};
    use crate::plain::{adapt_finished_runner, program_from_casm};
    use crate::relocator::relocator_tests::{create_test_relocator, get_test_relocatble_trace};
    use crate::vm_import::RelocatedTraceEntry;
    use crate::{casm_state, relocated_trace_entry, ProverInput};

    /// Translates a plain casm into a ProverInput by running the program and extracting the memory
    /// and the state transitions.
    fn input_from_plain_casm(casm: Vec<cairo_lang_casm::instructions::Instruction>) -> ProverInput {
        let (program, program_len) = program_from_casm(casm);

        let mut runner =
            CairoRunner::new(&program, LayoutName::all_cairo_stwo, None, true, true, true)
                .expect("Runner creation failed");
        runner.initialize(true).expect("Initialization failed");
        runner
            .run_until_pc(
                (runner.program_base.unwrap() + program_len).unwrap(),
                &mut BuiltinHintProcessor::new_empty(),
            )
            .expect("Run failed");
        runner.relocate(true).unwrap();
        adapt_finished_runner(runner).expect("Failed to adapt finished runner")
    }

    #[test]
    fn test_jmp_rel() {
        // Encoding for the instruction `jmp rel [fp]`.
        // Flags: pc_update_jump_rel, op_1_base_fp,  op0_base_fp, dst_base_fp
        // Offsets: offset2 = 0, offset1 = -1, offset0 = -1
        let encoded_instr = 0b000000100001011100000000000000001111111111111110111111111111111;
        let x = u128_to_4_limbs(encoded_instr);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        let trace_entry = relocated_trace_entry!(1, 1, 1);
        let states = CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);
        assert_eq!(states.jump_opcode_rel.len(), 1);
    }

    #[test]
    fn test_jmp_abs_double_deref() {
        // Encoding for the instruction `jmp abs [[ap + 0] + 0]`.
        // Flags: pc_update_jmp, dst_base_fp
        // Offsets: offset2 = 0, offset1 = 0, offset0 = -1
        let encoded_instr = 0b000000010000001100000000000000010000000000000000111111111111111;
        let x = u128_to_4_limbs(encoded_instr);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        let trace_entry = relocated_trace_entry!(1, 1, 1);
        let states = CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);
        assert_eq!(states.jump_opcode_double_deref.len(), 1);
    }

    // TODO(Stav): un-ignore when the opcode is in.
    #[ignore]
    #[test]
    fn test_jmp_abs() {
        let instructions = casm! {
            call rel 2;
            [ap] = [ap-1] + 3;
            jmp abs [ap];
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.jump_opcode.len(), 1);
        assert_eq!(casm_states_by_opcode.call_opcode_rel.len(), 1);
        assert_eq!(casm_states_by_opcode.add_opcode_small.len(), 1);
    }

    #[test]
    fn test_jmp_rel_imm() {
        let instructions = casm! {
            jmp rel 2;
            [ap] = [ap-1] + 3, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.jump_opcode_rel_imm.len(), 1);
    }

    #[test]
    fn test_add_ap() {
        let instructions = casm! {
            [ap] = 38, ap++;
            [ap] = 12, ap++;
            ap += [ap -2];
            ap += [fp + 1];
            ap += 1;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.add_ap_opcode.len(), 3);
    }

    #[test]
    fn test_call() {
        let instructions = casm! {
            call rel 2;
            call abs [fp - 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.call_opcode_op_1_base_fp.len(), 2);
        assert_eq!(casm_states_by_opcode.call_opcode_rel.len(), 1);
    }

    #[test]
    fn test_call2() {
        let instructions = casm! {
            call rel 2;
            call abs [ap - 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.call_opcode.len(), 2);
    }

    #[test]
    fn test_jnz_not_taken_ap() {
        let instructions = casm! {
            [ap] = 0, ap++;
            jmp rel 2 if [ap-1] != 0;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.jnz_opcode.len(), 1);
    }

    #[test]
    fn test_jnz_not_taken_fp() {
        let instructions = casm! {
            call rel 2;
            [ap] = 0, ap++;
            jmp rel 2 if [fp] != 0;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.jnz_opcode.len(), 1);
    }

    #[test]
    fn test_jnz_taken_fp() {
        let instructions = casm! {
            call rel 2;
            jmp rel 2 if [fp-1] != 0;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.jnz_opcode_taken.len(), 1);
    }

    #[test]
    fn test_jnz_taken_ap() {
        let instructions = casm! {
            [ap] = 5, ap++;
            jmp rel 2 if [ap-1] != 0;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.jnz_opcode_taken.len(), 1);
    }

    #[test]
    fn test_assert_equal() {
        let instructions = casm! {
            [ap] =  8, ap++;
            [ap] =  8, ap++;
            [ap+2] = [fp + 1];
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.assert_eq_opcode.len(), 1);
    }

    #[test]
    fn test_add_small() {
        let instructions = casm! {
            call rel 2;
            [ap] = 134217725, ap++;
            [ap] = 2, ap++;
            // 134217725 + 2= 2^27-1.
            [ap] = [fp] + [ap-1], ap++;
            // 134217724 + 3 = 2^27-1.
            [ap] = [fp-1] + 134217724, ap++;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.add_opcode_small.len(), 2);
        assert_eq!(casm_states_by_opcode.assert_eq_opcode_imm.len(), 2);
    }

    #[test]
    fn test_add_big() {
        let instructions = casm! {
            call rel 2;
            [ap] = 134217725, ap++;
            [ap] = 3, ap++;
            // 134217725 + 3 = is 2^27.
            [ap] = [fp] + [ap-1], ap++;
            [ap] = [ap-1] + 1, ap++;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.add_opcode.len(), 2);
    }

    #[test]
    fn test_mul_small() {
        let instructions = casm! {
            // 2^36-1 is the maximal factor value for a small mul.
            [ap] =  262145, ap++;
            [ap] =  [ap-1]*262143, ap++;
            // 2^36-1 is the maximal factor value for a small mul.
            [ap] = [ap-1], ap++;
            [ap] = [ap-1] * [ap-2], ap++;
            [ap] = [ap-2]*2147483647, ap++;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.mul_opcode_small.len(), 3);
    }

    #[test]
    fn test_mul_big() {
        let instructions = casm! {
            [ap] =  8, ap++;
            // 2^36 is the minimal factor value for a big mul.
            [ap] = 262144, ap++;
            [ap] = [ap-1] * 262144, ap++;
            [ap] = [ap-1] * [ap-3], ap++;
            [ap] = [ap-2]* 2, ap++;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.mul_opcode.len(), 2);
        assert_eq!(casm_states_by_opcode.mul_opcode_small.len(), 1);
    }

    #[test]
    fn test_generic() {
        let instructions = casm! {
        [ap]=1, ap++;
        [ap]=2, ap++;
        jmp rel [ap-2] if [ap-1] != 0;
        [ap]=1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.generic_opcode.len(), 1);
    }

    #[test]
    fn test_ret() {
        let instructions = casm! {
        [ap] = 10, ap++;
        call rel 4;
        jmp rel 11;

        jmp rel 4 if [fp-3] != 0;
        jmp rel 6;
        [ap] = [fp-3] + (-1), ap++;
        call rel (-6);
        ret;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.ret_opcode.len(), 11);
    }

    #[test]
    fn test_assert_eq_double_deref() {
        let instructions = casm! {
            call rel 2;
            [ap] = 100, ap++;
            [ap] = [[fp - 2] + 2], ap++;  // [fp - 2] is the old fp.
            [ap] = 5;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.assert_eq_opcode_double_deref.len(), 1);
    }

    #[test]
    fn test_blake_finalize() {
        let encoded_blake_finalize_inst =
            0b10000000000001011011111111111110101111111111111000111111111111011;
        let x = u128_to_4_limbs(encoded_blake_finalize_inst);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        let instruction = Instruction::decode(memory_builder.get_inst(1));
        let trace_entry = relocated_trace_entry!(1, 1, 1);

        let states = CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);

        matches!(instruction.opcode_extension, OpcodeExtension::BlakeFinalize);
        assert_eq!(states.blake2s_opcode.len(), 1);
    }

    #[test]
    fn test_qm31_add_mul_opcode() {
        let encoded_qm31_add_mul_inst =
            0b11100000001001010011111111111110101111111111111001000000000000000;
        let x = u128_to_4_limbs(encoded_qm31_add_mul_inst);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        let instruction = Instruction::decode(memory_builder.get_inst(1));
        let trace_entry = relocated_trace_entry!(1, 1, 1);
        let states = CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);

        matches!(instruction.opcode_extension, OpcodeExtension::QM31Operation);
        assert_eq!(states.qm31_add_mul_opcode.len(), 1);
    }

    #[test]
    fn test_casm_state_from_relocator() {
        let relocator = create_test_relocator();
        let encoded_qm31_add_mul_inst =
            0b11100000001001010011111111111110101111111111111001000000000000000;
        let x = u128_to_4_limbs(encoded_qm31_add_mul_inst);

        let memory_value = MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, memory_value);
        memory_builder.set(5, memory_value);
        memory_builder.set(85, memory_value);

        let state_transitions = StateTransitions::from_iter(
            relocator
                .relocate_trace(&get_test_relocatble_trace())
                .into_iter(),
            &memory_builder,
        );
        assert_eq!(
            state_transitions.casm_states_by_opcode.qm31_add_mul_opcode,
            vec![casm_state!(1, 5, 5), casm_state!(5, 6, 6)]
        );
        assert_eq!(state_transitions.final_state, casm_state!(85, 6, 6));
        assert_eq!(state_transitions.initial_state, casm_state!(1, 5, 5));
    }
}

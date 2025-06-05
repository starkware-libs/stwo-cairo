use std::fmt::Display;

// use rayon::iter::ParallelIterator;
// use rayon::slice::ParallelSlice;
use serde::{Deserialize, Serialize};
use stwo_cairo_common::prover_types::cpu::{CasmState, Relocatable};
// use stwo_prover::core::fields::m31::M31;
use tracing::{span, Level};

use super::decode::{Instruction, OpcodeExtension};
use super::memory::{MemoryBuilder, MemoryValue};
// use super::vm_import::RelocatedTraceEntry;
use cairo_vm::vm::trace::trace_entry::TraceEntry;

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
    pub blake_compress_opcode: Vec<CasmState>,
    pub qm_31_add_mul_opcode: Vec<CasmState>,
}
impl CasmStatesByOpcode {
    fn from_iter_relocatables<'a>(
        iter: impl DoubleEndedIterator<Item = &'a TraceEntry>,
        memory: &MemoryBuilder,
    ) -> Self {
        let mut res = CasmStatesByOpcode::default();
        for entry_ref in iter {
            res.push_instr(
                memory,
                CasmState {
                    pc: entry_ref.pc.offset.into(),
                    ap: entry_ref.ap.into(),
                    fp: entry_ref.fp.into(),
                },
            );
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
                let offset = dst_addr.0.checked_add_signed(offset0 as i32).unwrap();
                let dst = memory.get(Relocatable::execution(offset));
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
                    {
                        let new_offset = op0_addr.0.checked_add_signed(offset1 as i32).unwrap();
                        memory.get(Relocatable::execution(new_offset))
                    },
                    {
                        let new_offset = op_1_addr.0.checked_add_signed(offset2 as i32).unwrap();
                        memory.get(Relocatable {
                            segment_index: if op_1_imm { 0 } else { 1 },
                            offset: new_offset as u32,
                        })
                    },
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
                    {
                        let new_offset = dst_addr.0.checked_add_signed(offset0 as i32).unwrap();
                        memory.get(Relocatable::execution(new_offset))
                    },
                    {
                        let new_offset = op0_addr.0.checked_add_signed(offset1 as i32).unwrap();
                        memory.get(Relocatable::execution(new_offset))
                    },
                    {
                        let new_offset = op_1_addr.0.checked_add_signed(offset2 as i32).unwrap();
                        memory.get(Relocatable {
                            segment_index: if op_1_imm { 0 } else { 1 },
                            offset: new_offset as u32,
                        })
                    },
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
                self.blake_compress_opcode.push(state);
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
                self.qm_31_add_mul_opcode.push(state);
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

    pub fn merge(
        &mut self,
        CasmStatesByOpcode {
            generic_opcode,
            add_ap_opcode,
            add_opcode,
            add_opcode_small,
            assert_eq_opcode,
            assert_eq_opcode_double_deref,
            assert_eq_opcode_imm,
            call_opcode,
            call_opcode_rel,
            call_opcode_op_1_base_fp,
            jnz_opcode,
            jnz_opcode_taken,
            jump_opcode_rel_imm,
            jump_opcode_rel,
            jump_opcode_double_deref,
            jump_opcode,
            mul_opcode_small,
            mul_opcode,
            ret_opcode,
            blake_compress_opcode,
            qm_31_add_mul_opcode,
        }: &Self,
    ) {
        self.generic_opcode.extend(generic_opcode);
        self.add_ap_opcode.extend(add_ap_opcode);
        self.add_opcode.extend(add_opcode);
        self.add_opcode_small.extend(add_opcode_small);
        self.assert_eq_opcode.extend(assert_eq_opcode);
        self.assert_eq_opcode_double_deref
            .extend(assert_eq_opcode_double_deref);
        self.assert_eq_opcode_imm.extend(assert_eq_opcode_imm);
        self.call_opcode.extend(call_opcode);
        self.call_opcode_rel.extend(call_opcode_rel);
        self.call_opcode_op_1_base_fp
            .extend(call_opcode_op_1_base_fp);
        self.jnz_opcode.extend(jnz_opcode);
        self.jnz_opcode_taken.extend(jnz_opcode_taken);
        self.jump_opcode_rel_imm.extend(jump_opcode_rel_imm);
        self.jump_opcode_rel.extend(jump_opcode_rel);
        self.jump_opcode_double_deref
            .extend(jump_opcode_double_deref);
        self.jump_opcode.extend(jump_opcode);
        self.mul_opcode_small.extend(mul_opcode_small);
        self.mul_opcode.extend(mul_opcode);
        self.ret_opcode.extend(ret_opcode);
        self.blake_compress_opcode.extend(blake_compress_opcode);
        self.qm_31_add_mul_opcode.extend(qm_31_add_mul_opcode);
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
            (
                "blake_compress_opcode".to_string(),
                self.blake_compress_opcode.len(),
            ),
            (
                "qm_31_add_mul_opcode".to_string(),
                self.qm_31_add_mul_opcode.len(),
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

/// Holds the state transitions of a Cairo program, split according to the components responsible
/// for proving each transition.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct StateTransitions {
    pub initial_state: CasmState,
    pub final_state: CasmState,
    pub casm_states_by_opcode: CasmStatesByOpcode,
}

impl StateTransitions {
    pub fn from_relocatables(trace: &[TraceEntry], memory: &MemoryBuilder) -> Self {
        let _span = span!(Level::INFO, "StateTransitions::from_relocatables").entered();
        let initial_state_entry = trace.first().unwrap();
        let initial_state = CasmState {
            pc: initial_state_entry.pc.offset.into(),
            ap: initial_state_entry.ap.into(),
            fp: initial_state_entry.fp.into(),
        };
        let final_state_entry = trace.last().unwrap();
        let final_state = CasmState {
            pc: final_state_entry.pc.offset.into(),
            ap: final_state_entry.ap.into(),
            fp: final_state_entry.fp.into(),
        };
        let trace = &trace[..trace.len() - 1];

        let casm_states_by_opcode =
            CasmStatesByOpcode::from_iter_relocatables(trace.iter(), memory);

        StateTransitions {
            initial_state,
            final_state,
            casm_states_by_opcode,
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

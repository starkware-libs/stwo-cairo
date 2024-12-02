use prover_types::cpu::CasmState;
use stwo_prover::core::fields::m31::M31;

use super::decode::Instruction;
use super::mem::{MemoryBuilder, MemoryValue};
use super::vm_import::TraceEntry;

// TODO (Stav): Ensure it stays synced with that opcdode AIR's list.
/// This struct holds the components used to prove the opcodes in a Cairo program,
/// and should match the opcode's air used by `stwo-cairo-air`.
#[derive(Debug, Default, Clone)]
pub struct CasmStatesByOpcode {
    pub generic_opcode: Vec<CasmState>,
    pub add_ap_opcode_is_imm_f_op1_base_fp_f: Vec<CasmState>,
    pub add_ap_opcode_is_imm_t_op1_base_fp_f: Vec<CasmState>,
    pub add_ap_opcode_is_imm_f_op1_base_fp_t: Vec<CasmState>,
    pub add_opcode_is_small_t_is_imm_t: Vec<CasmState>,
    pub add_opcode_is_small_f_is_imm_f: Vec<CasmState>,
    pub add_opcode_is_small_t_is_imm_f: Vec<CasmState>,
    pub add_opcode_is_small_f_is_imm_t: Vec<CasmState>,
    pub assert_eq_opcode_is_double_deref_f_is_imm_f: Vec<CasmState>,
    pub assert_eq_opcode_is_double_deref_t_is_imm_f: Vec<CasmState>,
    pub assert_eq_opcode_is_double_deref_f_is_imm_t: Vec<CasmState>,
    pub call_opcode_is_rel_f_op1_base_fp_f: Vec<CasmState>,
    pub call_opcode_is_rel_t_op1_base_fp_f: Vec<CasmState>,
    pub call_opcode_is_rel_f_op1_base_fp_t: Vec<CasmState>,
    pub jnz_opcode_is_taken_t_dst_base_fp_t: Vec<CasmState>,
    pub jnz_opcode_is_taken_f_dst_base_fp_f: Vec<CasmState>,
    pub jnz_opcode_is_taken_t_dst_base_fp_f: Vec<CasmState>,
    pub jnz_opcode_is_taken_f_dst_base_fp_t: Vec<CasmState>,
    pub jump_opcode_is_rel_t_is_imm_t_is_double_deref_f: Vec<CasmState>,
    pub jump_opcode_is_rel_t_is_imm_f_is_double_deref_f: Vec<CasmState>,
    pub jump_opcode_is_rel_f_is_imm_f_is_double_deref_t: Vec<CasmState>,
    pub jump_opcode_is_rel_f_is_imm_f_is_double_deref_f: Vec<CasmState>,
    pub mul_opcode_is_small_t_is_imm_t: Vec<CasmState>,
    pub mul_opcode_is_small_t_is_imm_f: Vec<CasmState>,
    pub mul_opcode_is_small_f_is_imm_f: Vec<CasmState>,
    pub mul_opcode_is_small_f_is_imm_t: Vec<CasmState>,
    pub ret_opcode: Vec<CasmState>,
}

impl CasmStatesByOpcode {
    pub fn counts(&self) -> Vec<(&str, usize)> {
        vec![
            ("genric_opcode", self.generic_opcode.len()),
            (
                "add_ap_opcode_is_imm_f_op1_base_fp_f",
                self.add_ap_opcode_is_imm_f_op1_base_fp_f.len(),
            ),
            (
                "add_ap_opcode_is_imm_t_op1_base_fp_f",
                self.add_ap_opcode_is_imm_t_op1_base_fp_f.len(),
            ),
            (
                "add_ap_opcode_is_imm_f_op1_base_fp_t",
                self.add_ap_opcode_is_imm_f_op1_base_fp_t.len(),
            ),
            (
                "add_opcode_is_small_t_is_imm_t",
                self.add_opcode_is_small_t_is_imm_t.len(),
            ),
            (
                "add_opcode_is_small_f_is_imm_f",
                self.add_opcode_is_small_f_is_imm_f.len(),
            ),
            (
                "add_opcode_is_small_t_is_imm_f",
                self.add_opcode_is_small_t_is_imm_f.len(),
            ),
            (
                "add_opcode_is_small_f_is_imm_t",
                self.add_opcode_is_small_f_is_imm_t.len(),
            ),
            (
                "assert_eq_opcode_is_double_deref_f_is_imm_f",
                self.assert_eq_opcode_is_double_deref_f_is_imm_f.len(),
            ),
            (
                "assert_eq_opcode_is_double_deref_t_is_imm_f",
                self.assert_eq_opcode_is_double_deref_t_is_imm_f.len(),
            ),
            (
                "assert_eq_opcode_is_double_deref_f_is_imm_t",
                self.assert_eq_opcode_is_double_deref_f_is_imm_t.len(),
            ),
            (
                "call_opcode_is_rel_f_op1_base_fp_f",
                self.call_opcode_is_rel_f_op1_base_fp_f.len(),
            ),
            (
                "call_opcode_is_rel_t_op1_base_fp_f",
                self.call_opcode_is_rel_t_op1_base_fp_f.len(),
            ),
            (
                "call_opcode_is_rel_f_op1_base_fp_t",
                self.call_opcode_is_rel_f_op1_base_fp_t.len(),
            ),
            (
                "jnz_opcode_is_taken_t_dst_base_fp_t",
                self.jnz_opcode_is_taken_t_dst_base_fp_t.len(),
            ),
            (
                "jnz_opcode_is_taken_f_dst_base_fp_f",
                self.jnz_opcode_is_taken_f_dst_base_fp_f.len(),
            ),
            (
                "jnz_opcode_is_taken_t_dst_base_fp_f",
                self.jnz_opcode_is_taken_t_dst_base_fp_f.len(),
            ),
            (
                "jnz_opcode_is_taken_f_dst_base_fp_t",
                self.jnz_opcode_is_taken_f_dst_base_fp_t.len(),
            ),
            (
                "jump_opcode_is_rel_t_is_imm_t_is_double_deref_f",
                self.jump_opcode_is_rel_t_is_imm_t_is_double_deref_f.len(),
            ),
            (
                "jump_opcode_is_rel_t_is_imm_f_is_double_deref_f",
                self.jump_opcode_is_rel_t_is_imm_f_is_double_deref_f.len(),
            ),
            (
                "jump_opcode_is_rel_f_is_imm_f_is_double_deref_t",
                self.jump_opcode_is_rel_f_is_imm_f_is_double_deref_t.len(),
            ),
            (
                "jump_opcode_is_rel_f_is_imm_f_is_double_deref_f",
                self.jump_opcode_is_rel_f_is_imm_f_is_double_deref_f.len(),
            ),
            (
                "mul_opcode_is_small_t_is_imm_t",
                self.mul_opcode_is_small_t_is_imm_t.len(),
            ),
            (
                "mul_opcode_is_small_t_is_imm_f",
                self.mul_opcode_is_small_t_is_imm_f.len(),
            ),
            (
                "mul_opcode_is_small_f_is_imm_f",
                self.mul_opcode_is_small_f_is_imm_f.len(),
            ),
            (
                "mul_opcode_is_small_f_is_imm_t",
                self.mul_opcode_is_small_f_is_imm_t.len(),
            ),
            ("ret_opcode", self.ret_opcode.len()),
        ]
    }
}

impl From<TraceEntry> for CasmState {
    fn from(entry: TraceEntry) -> Self {
        Self {
            pc: M31(entry.pc as u32),
            ap: M31(entry.ap as u32),
            fp: M31(entry.fp as u32),
        }
    }
}

/// Holds the state transitions of a Cairo program, split according to the components responsible
/// for proving each transition.
#[derive(Debug, Default)]
pub struct StateTransitions {
    pub initial_state: CasmState,
    pub final_state: CasmState,
    pub casm_states_by_opcode: CasmStatesByOpcode,
}

impl StateTransitions {
    /// Iterates over the casm states and splits them into the appropriate opcode components.
    pub fn from_iter(
        iter: impl Iterator<Item = TraceEntry>,
        mem: &mut MemoryBuilder,
        dev_mode: bool,
    ) -> Self {
        let mut res = Self::default();
        let mut iter = iter.peekable();

        let Some(first) = iter.next() else {
            return res;
        };
        res.initial_state = first.into();
        res.push_instr(mem, first.into(), dev_mode);

        while let Some(entry) = iter.next() {
            // TODO(Ohad): Check if the adapter outputs the final state.
            let Some(_) = iter.peek() else {
                res.final_state = entry.into();
                break;
            };
            res.push_instr(mem, entry.into(), dev_mode);
        }
        res
    }

    // TODO(Ohad): remove dev_mode after adding the rest of the instructions.
    /// Pushes the state transition at pc into the appropriate opcode component.
    fn push_instr(&mut self, mem: &mut MemoryBuilder, state: CasmState, dev_mode: bool) {
        let CasmState { ap, fp, pc } = state;
        let instruction = mem.get_inst(pc.0);
        let instruction = Instruction::decode(instruction);

        match instruction {
            // ret.
            Instruction {
                offset0: -2,
                offset1: -1,
                offset2: -1,
                dst_base_fp: true,
                op0_base_fp: true,
                op1_imm: false,
                op1_base_fp: true,
                op1_base_ap: false,
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
            } => self.casm_states_by_opcode.ret_opcode.push(state),

            // add ap.
            Instruction {
                offset0: -1,
                offset1: -1,
                offset2,
                dst_base_fp: true,
                op0_base_fp: true,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
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
            } => {
                if op1_imm {
                    // ap += Imm.
                    assert!(!op1_base_fp && !op1_base_ap && offset2 == 1);
                    self.casm_states_by_opcode
                        .add_ap_opcode_is_imm_t_op1_base_fp_f
                        .push(state);
                } else if op1_base_fp {
                    // ap += [fp + offset2].
                    assert!(!op1_base_ap);
                    self.casm_states_by_opcode
                        .add_ap_opcode_is_imm_f_op1_base_fp_t
                        .push(state);
                } else {
                    // ap += [ap + offset2].
                    assert!(op1_base_ap);
                    self.casm_states_by_opcode
                        .add_ap_opcode_is_imm_f_op1_base_fp_f
                        .push(state);
                }
            }
            // jump.
            Instruction {
                offset0: -1,
                offset1,
                offset2,
                dst_base_fp: true,
                op0_base_fp,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
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
            } => {
                if op1_imm {
                    // jump rel imm.
                    assert!(
                        pc_update_jump_rel
                            && !pc_update_jump
                            && !op1_base_fp
                            && !op1_base_ap
                            && op0_base_fp
                            && offset1 == -1
                            && offset2 == 1
                    );
                    self.casm_states_by_opcode
                        .jump_opcode_is_rel_t_is_imm_t_is_double_deref_f
                        .push(state);
                } else if pc_update_jump_rel {
                    // jump rel [ap/fp + offset2].
                    assert!(
                        !pc_update_jump
                            && (op1_base_fp || op1_base_ap)
                            && op0_base_fp
                            && offset1 == -1
                    );
                    self.casm_states_by_opcode
                        .jump_opcode_is_rel_t_is_imm_f_is_double_deref_f
                        .push(state);
                } else if !op1_base_fp && !op1_base_ap {
                    // jump abs [[ap/fp + offset1] + offset2].
                    assert!(pc_update_jump);
                    self.casm_states_by_opcode
                        .jump_opcode_is_rel_f_is_imm_f_is_double_deref_t
                        .push(state);
                } else {
                    // jump abs [ap/fp + offset2].
                    assert!(
                        (op1_base_fp || op1_base_ap)
                            && op0_base_fp
                            && pc_update_jump
                            && offset1 == -1
                    );
                    self.casm_states_by_opcode
                        .jump_opcode_is_rel_f_is_imm_f_is_double_deref_f
                        .push(state);
                }
            }

            // call.
            Instruction {
                offset0: 0,
                offset1: 1,
                offset2,
                dst_base_fp: false,
                op0_base_fp: false,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
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
            } if !dev_mode => {
                if pc_update_jump_rel {
                    // call rel imm.
                    assert!(
                        op1_imm && !op1_base_fp && !op1_base_ap && offset2 == 1 && !pc_update_jump
                    );
                    self.casm_states_by_opcode
                        .call_opcode_is_rel_t_op1_base_fp_f
                        .push(state);
                } else if op1_base_fp {
                    // call abs [fp + offset2].
                    assert!(!op1_base_ap && !op1_imm && pc_update_jump);
                    self.casm_states_by_opcode
                        .call_opcode_is_rel_f_op1_base_fp_t
                        .push(state);
                } else {
                    // call abs [ap + offset2].
                    assert!(op1_base_ap && !op1_imm && pc_update_jump);
                    self.casm_states_by_opcode
                        .call_opcode_is_rel_f_op1_base_fp_f
                        .push(state);
                }
            }

            // jnz.
            Instruction {
                offset0,
                offset1: -1,
                offset2: 1,
                dst_base_fp,
                op0_base_fp: true,
                op1_imm: true,
                op1_base_fp: false,
                op1_base_ap: false,
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
            } => {
                let dst_addr = if dst_base_fp { fp } else { ap };
                let dst = mem.get(dst_addr.0.checked_add_signed(offset0 as i32).unwrap());
                let taken = dst != MemoryValue::Small(0);
                if taken {
                    if dst_base_fp {
                        // jump rel imm if [fp + offset0] != 0.
                        self.casm_states_by_opcode
                            .jnz_opcode_is_taken_t_dst_base_fp_t
                            .push(state);
                    } else {
                        // jump rel imm if [ap + offset0] != 0.
                        self.casm_states_by_opcode
                            .jnz_opcode_is_taken_t_dst_base_fp_f
                            .push(state);
                    };
                } else if dst_base_fp {
                    // jump rel imm if [fp + offset0] != 0.
                    self.casm_states_by_opcode
                        .jnz_opcode_is_taken_f_dst_base_fp_t
                        .push(state);
                } else {
                    // jump rel imm if [ap + offset] != 0.
                    self.casm_states_by_opcode
                        .jnz_opcode_is_taken_f_dst_base_fp_f
                        .push(state);
                };
            }

            // assert equal.
            Instruction {
                offset0: _,
                offset1,
                offset2,
                dst_base_fp: _,
                op0_base_fp,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
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
            } => {
                if op1_imm {
                    // [ap/fp + offset0] = imm.
                    assert!(
                        !op1_base_fp
                            && !op1_base_ap
                            && offset2 == 1
                            && op0_base_fp
                            && offset1 == -1
                    );
                    self.casm_states_by_opcode
                        .assert_eq_opcode_is_double_deref_f_is_imm_t
                        .push(state);
                } else if !op1_base_fp && !op1_base_ap {
                    // [ap/fp + offset0] = [[ap/fp + offset1] + offset2].
                    self.casm_states_by_opcode
                        .assert_eq_opcode_is_double_deref_t_is_imm_f
                        .push(state);
                } else {
                    // [ap/fp + offset0] = [ap/fp + offset1].
                    assert!((op1_base_fp || op1_base_ap) && offset1 == -1 && op0_base_fp);
                    self.casm_states_by_opcode
                        .assert_eq_opcode_is_double_deref_f_is_imm_f
                        .push(state);
                }
            }

            // mul.
            Instruction {
                offset0,
                offset1: _,
                offset2,
                dst_base_fp: _,
                op0_base_fp,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
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
            } => {
                let op1_addr = if op0_base_fp { fp } else { ap };
                let op1 = mem.get(op1_addr.0.checked_add_signed(offset0 as i32).unwrap());
                if op1_imm {
                    // [ap/fp + offset0] = [ap/fp + offset1] * Imm.
                    assert!(!op1_base_fp && !op1_base_ap && offset2 == 1);
                    if let MemoryValue::Small(_) = op1 {
                        self.casm_states_by_opcode
                            .mul_opcode_is_small_t_is_imm_t
                            .push(state);
                    } else {
                        self.casm_states_by_opcode
                            .mul_opcode_is_small_f_is_imm_t
                            .push(state);
                    };
                } else {
                    // [ap/fp + offset0] = [ap/fp + offset1] * [ap/fp + offset2].
                    assert!((op1_base_fp || op1_base_ap));
                    let op0_addr = if op0_base_fp { fp } else { ap };
                    let op0 = mem.get(op0_addr.0.checked_add_signed(offset0 as i32).unwrap());
                    if let MemoryValue::F252(_) = op1 {
                        self.casm_states_by_opcode
                            .mul_opcode_is_small_f_is_imm_f
                            .push(state);
                    } else if let MemoryValue::F252(_) = op0 {
                        self.casm_states_by_opcode
                            .mul_opcode_is_small_f_is_imm_f
                            .push(state);
                    } else {
                        self.casm_states_by_opcode
                            .mul_opcode_is_small_t_is_imm_f
                            .push(state);
                    }
                }
            }

            // add.
            Instruction {
                offset0,
                offset1: _,
                offset2,
                dst_base_fp: _,
                op0_base_fp,
                op1_imm,
                op1_base_fp,
                op1_base_ap,
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
            } => {
                let op1_addr = if op0_base_fp { fp } else { ap };
                let op1 = mem.get(op1_addr.0.checked_add_signed(offset0 as i32).unwrap());
                if op1_imm {
                    // [ap/fp + offset0] = [ap/fp + offset1] + Imm.
                    assert!(!op1_base_fp && !op1_base_ap && offset2 == 1);
                    let op1_addr = if op0_base_fp { fp } else { ap };
                    let op1 = mem.get(op1_addr.0.checked_add_signed(offset0 as i32).unwrap());
                    if let MemoryValue::Small(_) = op1 {
                        self.casm_states_by_opcode
                            .add_opcode_is_small_t_is_imm_t
                            .push(state);
                    } else {
                        self.casm_states_by_opcode
                            .add_opcode_is_small_f_is_imm_t
                            .push(state);
                    };
                } else {
                    // [ap/fp + offset0] = [ap/fp + offset1] + [ap/fp + offset2].
                    assert!((op1_base_fp || op1_base_ap));
                    let op0_addr = if op0_base_fp { fp } else { ap };
                    let op0 = mem.get(op0_addr.0.checked_add_signed(offset0 as i32).unwrap());
                    if let MemoryValue::F252(_) = op1 {
                        self.casm_states_by_opcode
                            .add_opcode_is_small_f_is_imm_f
                            .push(state);
                    } else if let MemoryValue::F252(_) = op0 {
                        self.casm_states_by_opcode
                            .add_opcode_is_small_t_is_imm_f
                            .push(state);
                    } else {
                        self.casm_states_by_opcode
                            .add_opcode_is_small_t_is_imm_f
                            .push(state);
                    }
                }
            }

            // generic opcode.
            _ => {
                self.casm_states_by_opcode.generic_opcode.push(state);
            }
        }
    }
}

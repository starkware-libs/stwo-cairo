use std::fmt::Display;

use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
use crypto_bigint::U256;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use serde::{Deserialize, Serialize};
use stwo::core::fields::m31::M31;
use stwo_cairo_common::memory::MEMORY_ADDRESS_BOUND;
use stwo_cairo_common::prover_types::cpu::CasmState;
use tracing::{span, Level};

use super::decode::{Instruction, OpcodeExtension};
use super::memory::{MemoryBuilder, MemoryValue};

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
    pub call_opcode_abs: Vec<CasmState>,
    pub call_opcode_rel_imm: Vec<CasmState>,
    pub jnz_opcode_non_taken: Vec<CasmState>,
    pub jnz_opcode_taken: Vec<CasmState>,
    pub jump_opcode_rel_imm: Vec<CasmState>,
    pub jump_opcode_rel: Vec<CasmState>,
    pub jump_opcode_double_deref: Vec<CasmState>,
    pub jump_opcode_abs: Vec<CasmState>,
    pub mul_opcode_small: Vec<CasmState>,
    pub mul_opcode: Vec<CasmState>,
    pub ret_opcode: Vec<CasmState>,
    pub blake_compress_opcode: Vec<CasmState>,
    pub qm_31_add_mul_opcode: Vec<CasmState>,
}
impl CasmStatesByOpcode {
    fn from_iter(
        iter: impl DoubleEndedIterator<Item = RelocatedTraceEntry>,
        memory: &MemoryBuilder,
    ) -> Self {
        let mut res = CasmStatesByOpcode::default();
        for entry in iter {
            res.push_instr(memory, into_casm_state(&entry));
        }
        res
    }

    /// Pushes the state transition at pc into the appropriate opcode component.
    fn push_instr(&mut self, memory: &MemoryBuilder, state: CasmState) {
        assert_state_in_address_space(state);
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
                    self.jump_opcode_abs.push(state);
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
                    self.call_opcode_rel_imm.push(state);
                } else {
                    // call abs [ap/fp + offset2].
                    assert!((op_1_base_ap ^ op_1_base_fp) && !op_1_imm && pc_update_jump);
                    self.call_opcode_abs.push(state);
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
                let taken = !dst.is_zero();
                if taken {
                    self.jnz_opcode_taken.push(state);
                } else {
                    self.jnz_opcode_non_taken.push(state);
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
            call_opcode_abs,
            call_opcode_rel_imm,
            jnz_opcode_non_taken,
            jnz_opcode_taken,
            jump_opcode_rel_imm,
            jump_opcode_rel,
            jump_opcode_double_deref,
            jump_opcode_abs,
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
        self.call_opcode_abs.extend(call_opcode_abs);
        self.call_opcode_rel_imm.extend(call_opcode_rel_imm);
        self.jnz_opcode_non_taken.extend(jnz_opcode_non_taken);
        self.jnz_opcode_taken.extend(jnz_opcode_taken);
        self.jump_opcode_rel_imm.extend(jump_opcode_rel_imm);
        self.jump_opcode_rel.extend(jump_opcode_rel);
        self.jump_opcode_double_deref
            .extend(jump_opcode_double_deref);
        self.jump_opcode_abs.extend(jump_opcode_abs);
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
            ("call_opcode_abs".to_string(), self.call_opcode_abs.len()),
            (
                "call_opcode_rel_imm".to_string(),
                self.call_opcode_rel_imm.len(),
            ),
            (
                "jnz_opcode_non_taken".to_string(),
                self.jnz_opcode_non_taken.len(),
            ),
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
            ("jump_opcode_abs".to_string(), self.jump_opcode_abs.len()),
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
        let log_total_steps = (total_steps as f64).log2();
        writeln!(f, "Total steps: {total_steps}, 2 ** {log_total_steps:.2?}")?;
        for (name, count) in &counts {
            let log_count = (*count as f64).log2();
            writeln!(f, "{name}: {count}, 2 ** {log_count:.2?}")?;
        }
        Ok(())
    }
}

fn into_casm_state(entry: &RelocatedTraceEntry) -> CasmState {
    CasmState {
        pc: M31(entry.pc as u32),
        ap: M31(entry.ap as u32),
        fp: M31(entry.fp as u32),
    }
}

fn assert_state_in_address_space(casm_state: CasmState) {
    assert!(
        (casm_state.ap.0 as usize) < MEMORY_ADDRESS_BOUND,
        "AP out of address range: {}",
        casm_state.ap.0
    );
    assert!(
        (casm_state.fp.0 as usize) < MEMORY_ADDRESS_BOUND,
        "FP out of address range: {}",
        casm_state.fp.0
    );
    assert!(
        (casm_state.pc.0 as usize) < MEMORY_ADDRESS_BOUND,
        "PC out of address range: {}",
        casm_state.pc.0
    );
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
    // TODO(Ohad): introduce `parallel` feature or delete this function.
    pub fn from_iter(
        iter: impl DoubleEndedIterator<Item = RelocatedTraceEntry>,
        memory: &MemoryBuilder,
    ) -> Self {
        let _span = span!(Level::INFO, "StateTransitions::from_iter").entered();
        let mut iter = iter.peekable();

        let initial_state = into_casm_state(iter.peek().expect("Must have an initial state."));
        assert_state_in_address_space(initial_state);

        // Assuming the last instruction is jrl0, no need to push it.
        let final_state = into_casm_state(&iter.next_back().expect("Must have a final state."));
        assert_state_in_address_space(final_state);

        let states = CasmStatesByOpcode::from_iter(iter, memory);

        StateTransitions {
            initial_state,
            final_state,
            casm_states_by_opcode: states,
        }
    }

    pub fn from_slice_parallel(trace: &[RelocatedTraceEntry], memory: &MemoryBuilder) -> Self {
        let _span = span!(Level::INFO, "StateTransitions::from_slice_parallel").entered();
        let initial_state = into_casm_state(trace.first().unwrap());
        assert_state_in_address_space(initial_state);

        // Assuming the last instruction is jrl0, no need to push it.
        let final_state = into_casm_state(trace.last().unwrap());
        assert_state_in_address_space(final_state);

        let trace = &trace[..trace.len() - 1];

        let n_workers = rayon::current_num_threads();
        let chunk_size = trace.len().div_ceil(n_workers);
        let casm_states_by_opcode = trace
            .par_chunks(chunk_size)
            .map(|chunk| CasmStatesByOpcode::from_iter(chunk.iter().cloned(), memory))
            .reduce(Default::default, |mut acc, chunk| {
                acc.merge(&chunk);
                acc
            });

        StateTransitions {
            initial_state,
            final_state,
            casm_states_by_opcode,
        }
    }
}

fn u256_from_le_array(arr: [u32; 8]) -> U256 {
    let mut buf = [0u8; 32];
    for (i, x) in arr.iter().enumerate() {
        buf[i * 4..(i + 1) * 4].copy_from_slice(&x.to_le_bytes());
    }
    U256::from_le_slice(&buf)
}

/// Small add ranges: [0 … 2^29 − 1] (positive) | [P − 2^29 - 1 … P − 1] (negative mod P).
// 2^29 - 1
const SMALL_ADD_POSITIVE_UPPER_BOUND: U256 = U256::from_u32(2_u32.pow(29) - 1);
// P - 2^29 - 1
const SMALL_ADD_NEGATIVE_LOWER_BOUND: U256 =
    U256::from_be_hex("0800000000000010FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE0000000");
// P - 1
const SMALL_ADD_NEGATIVE_UPPER_BOUND: U256 =
    U256::from_be_hex("0800000000000011000000000000000000000000000000000000000000000000");

// Returns 'true' if all the operands modulo P are within the range of [-2^29 - 1, 2^29 - 1].
fn is_small_add(dst: MemoryValue, op0: MemoryValue, op_1: MemoryValue) -> bool {
    [dst, op0, op_1].iter().all(|val| {
        let value = u256_from_le_array(val.as_u256());

        value <= SMALL_ADD_POSITIVE_UPPER_BOUND
            || (value >= SMALL_ADD_NEGATIVE_LOWER_BOUND && value <= SMALL_ADD_NEGATIVE_UPPER_BOUND)
    })
}

const SMALL_MUL_MAX_VALUE: u64 = 2u64.pow(36) - 1;
// Returns 'true' if the multiplication factors are in the range [0, 2^36-1].
fn is_small_mul(op0: MemoryValue, op_1: MemoryValue) -> bool {
    [op0, op_1].iter().all(|val| {
        let value = val.as_u256();
        value[2..8] == [0; 6]
            && (value[0] as u64 + ((value[1] as u64) << 32) <= SMALL_MUL_MAX_VALUE)
    })
}

/// Tests instructions mapping.
#[cfg(test)]
mod mappings_tests {

    use cairo_lang_casm::casm;
    use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
    use cairo_vm::relocatable;
    use cairo_vm::types::layout_name::LayoutName;
    use cairo_vm::types::relocatable::{MaybeRelocatable, Relocatable};
    use cairo_vm::vm::runners::cairo_runner::CairoRunner;
    use cairo_vm::vm::trace::trace_entry::RelocatedTraceEntry;
    use stwo::core::fields::m31::M31;
    use stwo_cairo_common::prover_types::cpu::CasmState;

    use crate::adapter::adapt;
    use crate::decode::{Instruction, OpcodeExtension};
    use crate::memory::*;
    use crate::opcodes::{is_small_add, CasmStatesByOpcode, StateTransitions};
    use crate::relocator::relocator_tests::get_test_relocatble_trace;
    use crate::relocator::Relocator;
    use crate::test_utils::program_from_casm;
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

        adapt(&runner).expect("Adapter failed")
    }

    #[test]
    fn test_small_add_positive_range() {
        // lower bound
        let mut dst = MemoryValue::Small(0);
        let mut op0 = MemoryValue::Small(0);
        let mut op1 = MemoryValue::Small(0);
        assert!(is_small_add(dst, op0, op1));

        // upper bound
        let positive_upper_bound = 2_u128.pow(29) - 1;
        dst = MemoryValue::Small(positive_upper_bound);
        op0 = MemoryValue::Small(positive_upper_bound);
        op1 = MemoryValue::Small(positive_upper_bound);
        assert!(is_small_add(dst, op0, op1));

        dst = MemoryValue::F252(dst.as_u256());
        op0 = MemoryValue::F252(op0.as_u256());
        op1 = MemoryValue::F252(op1.as_u256());
        assert!(is_small_add(dst, op0, op1));

        // value in the range
        let value_in_range = MemoryValue::Small(2_u128.pow(27) - 10);
        dst = value_in_range;
        op0 = value_in_range;
        op1 = value_in_range;
        assert!(is_small_add(dst, op0, op1));
    }

    #[test]
    fn test_small_add_negative_range() {
        // lower bound
        let p_min_2_to_29_min_1: [u32; 8] = [
            0xE000_0000,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0x0000_0010,
            0x0800_0000,
        ];
        let mut dst = MemoryValue::F252(p_min_2_to_29_min_1);
        let mut op0 = MemoryValue::F252(p_min_2_to_29_min_1);
        let mut op1 = MemoryValue::F252(p_min_2_to_29_min_1);
        assert!(is_small_add(dst, op0, op1));

        // upper bound
        dst = MemoryValue::F252(P_MIN_1);
        op0 = MemoryValue::F252(P_MIN_1);
        op1 = MemoryValue::F252(P_MIN_1);
        assert!(is_small_add(dst, op0, op1));

        // value in the range
        let p_min_2_to_10 = [
            0xfffffc01, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x00000010,
            0x08000000,
        ];
        op1 = MemoryValue::F252(p_min_2_to_10);
        op0 = MemoryValue::F252(p_min_2_to_10);
        assert!(is_small_add(dst, op0, op1));
    }

    #[test]
    fn test_not_small_add() {
        let value = 2_u128.pow(29);
        let mut dst = MemoryValue::Small(value);
        let mut op0 = MemoryValue::Small(value);
        let mut op1 = MemoryValue::Small(value);
        assert!(!is_small_add(dst, op0, op1));

        let p_min_2_to_29_min_2: [u32; 8] = [
            0xDFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0xFFFF_FFFF,
            0x0000_0010,
            0x0800_0000,
        ];
        dst = MemoryValue::F252(p_min_2_to_29_min_2);
        op0 = MemoryValue::F252(p_min_2_to_29_min_2);
        op1 = MemoryValue::F252(p_min_2_to_29_min_2);
        assert!(!is_small_add(dst, op0, op1));
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
        assert_eq!(casm_states_by_opcode.jump_opcode_abs.len(), 1);
        assert_eq!(casm_states_by_opcode.call_opcode_rel_imm.len(), 1);
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
    fn test_add_ap_upper_edge_case() {
        // Encoding for the instruction `ap += [fp]`.
        // Flags: dst_base_fp, op0_base_fp, op1_base_fp, ap_update_add
        // Offsets: offset2 = 0, offset1 = -1, offset0 = -1
        let [ap, fp, pc] = [7, 20, 1];
        let encoded_instr = 0b000010000001011100000000000000001111111111111110111111111111111;
        let x = u128_to_4_limbs(encoded_instr);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(pc, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));
        memory_builder.set(fp, MemoryValue::Small((((1 << 27) - 1) - ap) as u128));

        let trace_entry = relocated_trace_entry!(ap as usize, fp as usize, pc as usize);
        let casm_states_by_opcode =
            CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);
        assert_eq!(casm_states_by_opcode.add_ap_opcode.len(), 1);
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
        assert_eq!(casm_states_by_opcode.call_opcode_abs.len(), 2);
        assert_eq!(casm_states_by_opcode.call_opcode_rel_imm.len(), 1);
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
        assert_eq!(casm_states_by_opcode.call_opcode_abs.len(), 2);
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
        assert_eq!(casm_states_by_opcode.jnz_opcode_non_taken.len(), 1);
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
        assert_eq!(casm_states_by_opcode.jnz_opcode_non_taken.len(), 1);
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
            [ap] = 536870909, ap++;
            [ap] = 2, ap++;
            // 536870909 + 2= 2^29-1.
            [ap] = [fp] + [ap-1], ap++;
            // 536870908 + 3 = 2^29-1.
            [ap] = [fp-1] + 536870908, ap++;
            [ap] = 1, ap++;
        }
        .instructions;

        let input = input_from_plain_casm(instructions);
        let casm_states_by_opcode = input.state_transitions.casm_states_by_opcode;
        assert_eq!(casm_states_by_opcode.add_opcode_small.len(), 2);
        assert_eq!(casm_states_by_opcode.assert_eq_opcode_imm.len(), 2);
    }

    #[test]
    fn test_add_small_negative() {
        // Encoding for the instruction `[fp + 2] = [fp] + Imm`.
        // Flags: dst_base_fp, op0_base_fp, op1_imm
        // Offsets: offset2 = 1, offset1 = 0, offset0 = 0
        let [ap, fp, pc] = [7, 20, 1];
        let encoded_instr = 0b0100000000100111100000000000000110000000000000001000000000000010;
        let x = u128_to_4_limbs(encoded_instr);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(pc, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));
        memory_builder.set(pc + 1, MemoryValue::F252(P_MIN_1));
        memory_builder.set(fp, MemoryValue::F252(P_MIN_1));
        memory_builder.set(fp + 2, MemoryValue::F252(P_MIN_2));
        let trace_entry = relocated_trace_entry!(ap as usize, fp as usize, pc as usize);

        let casm_states_by_opcode =
            CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);

        assert_eq!(casm_states_by_opcode.add_opcode_small.len(), 1);
    }

    #[test]
    fn test_add_big_f252() {
        let [ap, fp, pc] = [7, 20, 1];
        let encoded_instr = 0b0100000000100111100000000000000110000000000000001000000000000010;
        let x = u128_to_4_limbs(encoded_instr);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(pc, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        let mut value = [0; 8];
        value[0..4].copy_from_slice(&u128_to_4_limbs((1 << 127) + 346));
        memory_builder.set(pc + 1, MemoryValue::F252(value));
        memory_builder.set(fp, MemoryValue::F252(value));
        memory_builder.set(fp + 2, MemoryValue::F252(value));
        let trace_entry = relocated_trace_entry!(ap as usize, fp as usize, pc as usize);

        let casm_states_by_opcode =
            CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);

        assert_eq!(casm_states_by_opcode.add_opcode.len(), 1);
    }

    #[test]
    fn test_add_big() {
        let instructions = casm! {
            call rel 2;
            [ap] = 536870909, ap++;
            [ap] = 3, ap++;
            // 536870909 + 3 = is 2^29.
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
        assert_eq!(states.blake_compress_opcode.len(), 1);
    }

    #[test]
    fn test_qm_31_add_mul_opcode() {
        let encoded_qm_31_add_mul_inst =
            0b11100000001001010011111111111110101111111111111001000000000000000;
        let x = u128_to_4_limbs(encoded_qm_31_add_mul_inst);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        let instruction = Instruction::decode(memory_builder.get_inst(1));
        let trace_entry = relocated_trace_entry!(1, 1, 1);
        let states = CasmStatesByOpcode::from_iter([trace_entry].into_iter(), &memory_builder);

        matches!(instruction.opcode_extension, OpcodeExtension::QM31Operation);
        assert_eq!(states.qm_31_add_mul_opcode.len(), 1);
    }

    #[test]
    fn test_casm_state_from_relocator() {
        let segment0 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(9.into())),
            Some(MaybeRelocatable::RelocatableValue(relocatable!(2, 1))),
        ];
        let builtin_segment1 =
            vec![Some(MaybeRelocatable::RelocatableValue(relocatable!(0, 1))); 80];
        let segment2 = vec![
            Some(MaybeRelocatable::Int(1.into())),
            Some(MaybeRelocatable::Int(2.into())),
            Some(MaybeRelocatable::Int(3.into())),
        ];
        let memory = vec![segment0, builtin_segment1, segment2];
        let relocator = Relocator::new(&memory);

        let encoded_qm_31_add_mul_inst =
            0b11100000001001010011111111111110101111111111111001000000000000000;
        let x = u128_to_4_limbs(encoded_qm_31_add_mul_inst);

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
            state_transitions.casm_states_by_opcode.qm_31_add_mul_opcode,
            vec![casm_state!(1, 5, 5), casm_state!(5, 6, 6)]
        );
        assert_eq!(state_transitions.final_state, casm_state!(85, 6, 6));
        assert_eq!(state_transitions.initial_state, casm_state!(1, 5, 5));
    }

    #[test]
    #[should_panic(expected = "AP out of address range: 536870912")]
    fn test_ap_out_of_range_from_iter() {
        let reloctated_trace = [relocated_trace_entry!(2usize.pow(29), 1, 1)];
        StateTransitions::from_iter(
            reloctated_trace.into_iter(),
            &MemoryBuilder::new(MemoryConfig::default()),
        );
    }

    #[test]
    #[should_panic(expected = "AP out of address range: 536870912")]
    fn test_ap_out_of_range_from_slice() {
        let mut reloctated_trace: [RelocatedTraceEntry; 80] =
            std::array::from_fn(|_| relocated_trace_entry!(2, 1, 1));
        reloctated_trace[68] = relocated_trace_entry!(2usize.pow(29), 1, 1);

        let encoded_blake_finalize_inst =
            0b10000000000001011011111111111110101111111111111000111111111111011;
        let x = u128_to_4_limbs(encoded_blake_finalize_inst);
        let mut memory_builder = MemoryBuilder::new(MemoryConfig::default());
        memory_builder.set(1, MemoryValue::F252([x[0], x[1], x[2], x[3], 0, 0, 0, 0]));

        StateTransitions::from_slice_parallel(&reloctated_trace, &memory_builder);
    }
}

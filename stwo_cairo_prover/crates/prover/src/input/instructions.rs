use prover_types::cpu::CasmState;
use serde::{Deserialize, Serialize};
use stwo_prover::core::fields::m31::M31;

use super::decode::Instruction;
use super::mem::{MemoryBuilder, MemoryValue};
use super::vm_import::TraceEntry;

impl From<TraceEntry> for CasmState {
    fn from(entry: TraceEntry) -> Self {
        Self {
            pc: M31(entry.pc as u32),
            ap: M31(entry.ap as u32),
            fp: M31(entry.fp as u32),
        }
    }
}

// TODO(yuval/alonT): consider making the indexing mechanism more explicit in the code).
/// The instructions usage in the input, split to Stwo opcodes.
///
/// For each opcode with flags, the array describes the different flag combinations. The index
/// refers to the flag combination in bit-reverse/little-endian. For example, jnz_imm at index 1
/// (100 in bit-reverse/little-endian) is for: fp (1=true), not taken (0=false), no ap++ (0=false).
/// Note: for the flag "fp/ap", true means fp-based and false means ap-based.
#[derive(Debug, Default)]
pub struct Instructions {
    pub initial_state: CasmState,
    pub final_state: CasmState,

    /// ret.
    pub ret: Vec<CasmState>,

    /// ap += imm.
    pub add_ap: Vec<CasmState>,

    /// jump rel imm.
    /// Flags: ap++?.
    pub jmp_rel_imm: [Vec<CasmState>; 2],

    /// jump abs [fp/ap + offset].
    /// Flags: fp/ap, ap++?.
    pub jmp_abs: [Vec<CasmState>; 4],

    /// call rel imm.
    pub call_rel_imm: Vec<CasmState>,

    /// call abs [fp/ap + offset].
    /// Flags: fp/ap.
    pub call_abs: [Vec<CasmState>; 2],

    /// jump rel imm if [fp/ap + offset] != 0.
    /// Flags: fp/ap, taken?, ap++?.
    pub jnz_imm: [Vec<CasmState>; 8],

    /// - [fp/ap + offset0] = [fp/ap + offset2]
    pub mov_mem: Vec<CasmState>,

    /// - [fp/ap + offset0] = [[fp/ap + offset1] + offset2]
    pub deref: Vec<CasmState>,

    /// - [fp/ap + offset0] = imm
    pub push_imm: Vec<CasmState>,

    pub generic: Vec<CasmState>,
}
impl Instructions {
    pub fn from_iter(mut iter: impl Iterator<Item = TraceEntry>, mem: &mut MemoryBuilder) -> Self {
        let mut res = Self::default();

        let Some(first) = iter.next() else {
            return res;
        };
        res.initial_state = first.into();
        res.push_instr(mem, first.into());

        for entry in iter {
            res.final_state = entry.into();
            res.push_instr(mem, entry.into());
        }
        res
    }

    fn push_instr(&mut self, mem: &mut MemoryBuilder, state: CasmState) {
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
            } => self.ret.push(state),
            // ap += imm.
            Instruction {
                offset0: -1,
                offset1: -1,
                offset2: 1,
                dst_base_fp: true,
                op0_base_fp: true,
                op1_imm: true,
                op1_base_fp: false,
                op1_base_ap: false,
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
            } => self.add_ap.push(state),
            // jump rel imm.
            Instruction {
                offset0: -1,
                offset1: -1,
                offset2: 1,
                dst_base_fp: true,
                op0_base_fp: true,
                op1_imm: true,
                op1_base_fp: false,
                op1_base_ap: false,
                res_add: false,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: true,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
            } => {
                self.jmp_rel_imm[ap_update_add_1 as usize].push(state);
            }
            // jump abs [ap/fp + offset].
            Instruction {
                offset0: -1,
                offset1: -1,
                offset2: _,
                dst_base_fp: true,
                op0_base_fp: true,
                op1_imm: false,
                op1_base_fp,
                op1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump: true,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
            } if op1_base_fp != op1_base_ap => {
                let index = op1_base_fp as usize | (ap_update_add_1 as usize) << 1;
                self.jmp_abs[index].push(state);
            }
            // call rel imm.
            Instruction {
                offset0: 0,
                offset1: 1,
                offset2: 1,
                dst_base_fp: false,
                op0_base_fp: false,
                op1_imm: true,
                op1_base_fp: false,
                op1_base_ap: false,
                res_add: false,
                res_mul: false,
                pc_update_jump: false,
                pc_update_jump_rel: true,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: false,
                opcode_call: true,
                opcode_ret: false,
                opcode_assert_eq: false,
            } => {
                self.call_rel_imm.push(state);
            }
            // call abs [ap/fp + offset].
            Instruction {
                offset0: 0,
                offset1: 1,
                offset2: _,
                dst_base_fp: false,
                op0_base_fp: false,
                op1_imm: false,
                op1_base_fp,
                op1_base_ap,
                res_add: false,
                res_mul: false,
                pc_update_jump: true,
                pc_update_jump_rel: false,
                pc_update_jnz: false,
                ap_update_add: false,
                ap_update_add_1: false,
                opcode_call: true,
                opcode_ret: false,
                opcode_assert_eq: false,
            } if op1_base_fp != op1_base_ap => {
                let index = op1_base_fp as usize;
                self.call_abs[index].push(state);
            }
            // jnz
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
                ap_update_add_1,
                opcode_call: false,
                opcode_ret: false,
                opcode_assert_eq: false,
            } => {
                let dst_addr = if dst_base_fp { fp } else { ap };
                let dst = mem.get(dst_addr.0.checked_add_signed(offset0 as i32).unwrap());
                let taken = dst != MemoryValue::Small(0);
                let index = (dst_base_fp as usize)
                    | (taken as usize) << 1
                    | (ap_update_add_1 as usize) << 2;
                self.jnz_imm[index].push(state);
            }
            // [ap/fp + offset0] = [ap/fp + offset2].
            Instruction {
                offset0: _,
                offset1: -1,
                offset2: _,
                dst_base_fp: _,
                op0_base_fp: true,
                op1_imm: false,
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
            } if op1_base_fp != op1_base_ap => {
                self.mov_mem.push(state);
            }
            // [ap/fp + offset0] = [[ap/fp + offset1] + offset2].
            Instruction {
                offset0: _,
                offset1: _,
                offset2: _,
                dst_base_fp: _,
                op0_base_fp: _,
                op1_imm: false,
                op1_base_fp: false,
                op1_base_ap: false,
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
                self.deref.push(state);
            }
            // [ap/fp + offset0] = imm.
            Instruction {
                offset0: _,
                offset1: -1,
                offset2: 1,
                dst_base_fp: _,
                op0_base_fp: true,
                op1_imm: true,
                op1_base_fp: false,
                op1_base_ap: false,
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
                self.push_imm.push(state);
            }
            _ => {
                self.generic.push(state);
            }
        }
    }

    pub fn counts(&self) -> InstructionCounts {
        InstructionCounts {
            ret: self.ret.len(),
            add_ap: self.add_ap.len(),
            jmp_rel_imm: self.jmp_rel_imm.each_ref().map(Vec::len),
            jmp_abs: self.jmp_abs.each_ref().map(Vec::len),
            call_rel_imm: self.call_rel_imm.len(),
            call_abs: self.call_abs.each_ref().map(Vec::len),
            jnz_imm: self.jnz_imm.each_ref().map(Vec::len),
            mov_mem: self.mov_mem.len(),
            deref: self.deref.len(),
            push_imm: self.push_imm.len(),
            generic: self.generic.len(),
        }
    }
}

/// The counts of the instructions usage in the input, split to Stwo opcodes.
///
/// See the documentation of `Instructions` for more details about the indexing mechanism.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstructionCounts {
    pub ret: usize,
    pub add_ap: usize,

    /// jump rel imm.
    /// Flags: ap++?.
    pub jmp_rel_imm: [usize; 2],

    // jump abs [fp/ap + offset].
    // Flags: fp/ap, ap++?.
    pub jmp_abs: [usize; 4],

    /// call rel imm.
    pub call_rel_imm: usize,

    // call abs [fp/ap + offset].
    // Flags: fp/ap.
    pub call_abs: [usize; 2],

    /// jump rel imm if [fp/ap + offset] != 0.
    /// Flags: fp/ap, taken?, ap++?.
    pub jnz_imm: [usize; 8],

    /// - [fp/ap + offset0] = [fp/ap + offset2]
    pub mov_mem: usize,

    /// - [fp/ap + offset0] = [[fp/ap + offset1] + offset2]
    pub deref: usize,

    /// - [fp/ap + offset0] = imm
    pub push_imm: usize,

    pub generic: usize,
}

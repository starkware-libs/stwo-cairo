use super::decode::Instruction;
use super::mem::{Memory, MemoryValue};
use super::vm_import::TraceEntry;

// TODO(spapini): Move this:
pub struct VmState {
    pub pc: u32,
    pub ap: u32,
    pub fp: u32,
}

#[derive(Default)]
pub struct Instructions {
    pub ret: Vec<VmState>,
    pub add_ap: Vec<VmState>,

    /// jump rel imm.
    /// Flags: ap++?.
    pub jmp_rel_imm: [Vec<VmState>; 2],

    // jump abs [ap/fp + offset].
    // Flags: ap/fp, ap++?.
    pub jmp_abs: [Vec<VmState>; 4],

    /// call rel imm.
    pub call_rel_imm: Vec<VmState>,

    // call abs [ap/fp + offset].
    // Flags: ap/fp.
    pub call_abs: [Vec<VmState>; 2],

    /// jump rel imm if [ap/fp + offset] != 0.
    /// Flags: ap/fp, taken?, ap++?.
    pub jnz_imm: [Vec<VmState>; 8],

    /// - [ap/fp + offset0] = [ap/fp + offset2]
    pub mov_mem: Vec<VmState>,

    /// - [ap/fp + offset0] = [[ap/fp + offset1] + offset2]
    pub deref: Vec<VmState>,

    /// - [ap/fp + offset0] = imm
    pub push_imm: Vec<VmState>,

    pub generic: Vec<VmState>,
}
impl Instructions {
    pub fn from_iter(iter: impl Iterator<Item = TraceEntry>, mem: &Memory) -> Self {
        let mut res = Self::default();
        for TraceEntry { ap, fp, pc } in iter {
            let ap = ap as u32;
            let fp = fp as u32;
            let pc = pc as u32;
            let state = VmState { pc, ap, fp };
            let instruction = mem.get(pc).as_u64();
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
                } => res.ret.push(state),
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
                } => res.add_ap.push(state),
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
                    res.jmp_rel_imm[ap_update_add_1 as usize].push(state);
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
                    res.jmp_abs[index].push(state);
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
                    res.call_rel_imm.push(state);
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
                    res.call_abs[index].push(state);
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
                    let dst = mem.get(dst_addr.checked_add_signed(offset0 as i32).unwrap());
                    let taken = dst != MemoryValue::Small(0);
                    let index = (dst_base_fp as usize)
                        | (taken as usize) << 1
                        | (ap_update_add_1 as usize) << 2;
                    res.jnz_imm[index].push(state);
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
                    res.mov_mem.push(state);
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
                    res.deref.push(state);
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
                    res.push_imm.push(state);
                }
                _ => {
                    res.generic.push(state);
                }
            }
        }
        res
    }

    pub fn usage(&self) -> InstructionUsage {
        InstructionUsage {
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct InstructionUsage {
    pub ret: usize,
    pub add_ap: usize,

    /// jump rel imm.
    /// Flags: ap++?.
    pub jmp_rel_imm: [usize; 2],

    // jump abs [ap/fp + offset].
    // Flags: ap/fp, ap++?.
    pub jmp_abs: [usize; 4],

    /// call rel imm.
    pub call_rel_imm: usize,

    // call abs [ap/fp + offset].
    // Flags: ap/fp.
    pub call_abs: [usize; 2],

    /// jump rel imm if [ap/fp + offset] != 0.
    /// Flags: ap/fp, taken?, ap++?.
    pub jnz_imm: [usize; 8],

    /// - [ap/fp + offset0] = [ap/fp + offset2]
    pub mov_mem: usize,

    /// - [ap/fp + offset0] = [[ap/fp + offset1] + offset2]
    pub deref: usize,

    /// - [ap/fp + offset0] = imm
    pub push_imm: usize,

    pub generic: usize,
}

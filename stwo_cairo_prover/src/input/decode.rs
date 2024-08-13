#[derive(Clone, Debug)]
pub struct Instruction {
    pub offset0: i16,
    pub offset1: i16,
    pub offset2: i16,
    pub dst_base_fp: bool,
    pub op0_base_fp: bool,
    pub op1_imm: bool,
    pub op1_base_fp: bool,
    pub op1_base_ap: bool,
    pub res_add: bool,
    pub res_mul: bool,
    pub pc_update_jump: bool,
    pub pc_update_jump_rel: bool,
    pub pc_update_jnz: bool,
    pub ap_update_add: bool,
    pub ap_update_add_1: bool,
    pub opcode_call: bool,
    pub opcode_ret: bool,
    pub opcode_assert_eq: bool,
}
impl Instruction {
    pub fn decode(mut encoded_instr: u64) -> Instruction {
        let mut next_offset = || {
            let offset = (encoded_instr & 0xffff) as u16;
            encoded_instr >>= 16;
            offset.overflowing_sub(0x8000u16).0 as i16
        };
        let offset0 = next_offset();
        let offset1 = next_offset();
        let offset2 = next_offset();

        let mut next_bit = || {
            let bit = encoded_instr & 1;
            encoded_instr >>= 1;
            bit != 0
        };

        Instruction {
            offset0,
            offset1,
            offset2,
            dst_base_fp: next_bit(),
            op0_base_fp: next_bit(),
            op1_imm: next_bit(),
            op1_base_fp: next_bit(),
            op1_base_ap: next_bit(),
            res_add: next_bit(),
            res_mul: next_bit(),
            pc_update_jump: next_bit(),
            pc_update_jump_rel: next_bit(),
            pc_update_jnz: next_bit(),
            ap_update_add: next_bit(),
            ap_update_add_1: next_bit(),
            opcode_call: next_bit(),
            opcode_ret: next_bit(),
            opcode_assert_eq: next_bit(),
        }
    }
}

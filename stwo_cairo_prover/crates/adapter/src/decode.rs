use stwo::core::fields::m31::M31;

#[derive(Clone, Debug)]
pub enum OpcodeExtension {
    Stone,
    Blake,
    BlakeFinalize,
    QM31Operation,
}
impl OpcodeExtension {
    /// Converts the trailing bits (encoded_instr after shifting right by 63) of an instruction to
    /// an OpcodeExtension and returns it.
    /// # Panics
    /// - if the trailing bits do not correspond to a valid OpcodeExtension.
    pub fn from_instruction_trailing_bits(value: u128) -> OpcodeExtension {
        match value {
            0 => OpcodeExtension::Stone,
            1 => OpcodeExtension::Blake,
            2 => OpcodeExtension::BlakeFinalize,
            3 => OpcodeExtension::QM31Operation,
            _ => panic!("Invalid opcode extension number: {value}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub offset0: i16,
    pub offset1: i16,
    pub offset2: i16,
    pub dst_base_fp: bool,
    pub op0_base_fp: bool,
    pub op_1_imm: bool,
    pub op_1_base_fp: bool,
    pub op_1_base_ap: bool,
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
    pub opcode_extension: OpcodeExtension,
}
impl Instruction {
    pub fn decode(mut encoded_instr: u128) -> Instruction {
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
            op_1_imm: next_bit(),
            op_1_base_fp: next_bit(),
            op_1_base_ap: next_bit(),
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
            // The remaining bits in encoded_instr are the opcode extension.
            opcode_extension: OpcodeExtension::from_instruction_trailing_bits(encoded_instr),
        }
    }
}

/// Constructs the input for the DecodeInstruction routine.
///
/// # Arguments
///
/// - `encoded_instr`: The encoded instruction.
///
/// # Returns
///
/// The Deconstructed instruction in the form of (offsets, flags, extension).
pub fn deconstruct_instruction(mut encoded_instr: u128) -> ([M31; 3], [M31; 2], M31) {
    let mut next_offset = || {
        let offset = (encoded_instr & 0xffff) as u16;
        encoded_instr >>= 16;
        offset
    };
    let offsets = std::array::from_fn(|_| M31(next_offset() as u32));

    let mut next_bit = || {
        let bit = (encoded_instr & 1) as u32;
        encoded_instr >>= 1;
        bit
    };

    let mut flags = [0; 2];
    for i in 0..15 {
        let bit = next_bit();
        if i < 6 {
            flags[0] |= bit << (i + 3);
        } else {
            flags[1] |= bit << (i - 6);
        }
    }
    let flags = flags.map(M31);

    // The remaining bits in encoded_instr are the opcode extension.
    let opcode_extension = M31(encoded_instr as u32);

    (offsets, flags, opcode_extension)
}

#[cfg(test)]
mod tests {
    use stwo::core::fields::m31::M31;

    use crate::decode::deconstruct_instruction;

    #[test]
    fn test_deconstruct_instruction() {
        let encoded_instr = 0b10100011010001010000000000000000100000000000000110000000000000111;
        let expected_opcode_extension = M31(2);
        let expected_flags = [0x50, 0x11a].map(M31);
        let expected_offsets = [7, 3, 1].map(M31);

        let (offsets, flags, opcode_extension) = deconstruct_instruction(encoded_instr);

        assert_eq!(offsets, expected_offsets);
        assert_eq!(flags, expected_flags);
        assert_eq!(opcode_extension, expected_opcode_extension);
    }
}

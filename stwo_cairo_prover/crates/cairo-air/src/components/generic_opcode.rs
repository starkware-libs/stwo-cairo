use crate::components::prelude::*;
use crate::components::subroutines::decode_generic_instruction::DecodeGenericInstruction;
use crate::components::subroutines::eval_operands::EvalOperands;
use crate::components::subroutines::handle_opcodes::HandleOpcodes;
use crate::components::subroutines::update_registers::UpdateRegisters;

pub const N_TRACE_COLUMNS: usize = 236;
pub const RELATION_USES_PER_ROW: [RelationUse; 6] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 3,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 3,
    },
    RelationUse {
        relation_id: "Opcodes",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_19",
        uses: 28,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 28,
    },
    RelationUse {
        relation_id: "VerifyInstruction",
        uses: 1,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub range_check_19_lookup_elements: relations::RangeCheck_19,
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 33];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.claim.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let offset2_col5 = eval.next_trace_mask();
        let dst_base_fp_col6 = eval.next_trace_mask();
        let op0_base_fp_col7 = eval.next_trace_mask();
        let op1_imm_col8 = eval.next_trace_mask();
        let op1_base_fp_col9 = eval.next_trace_mask();
        let op1_base_ap_col10 = eval.next_trace_mask();
        let res_add_col11 = eval.next_trace_mask();
        let res_mul_col12 = eval.next_trace_mask();
        let pc_update_jump_col13 = eval.next_trace_mask();
        let pc_update_jump_rel_col14 = eval.next_trace_mask();
        let pc_update_jnz_col15 = eval.next_trace_mask();
        let ap_update_add_col16 = eval.next_trace_mask();
        let ap_update_add_1_col17 = eval.next_trace_mask();
        let opcode_call_col18 = eval.next_trace_mask();
        let opcode_ret_col19 = eval.next_trace_mask();
        let opcode_assert_eq_col20 = eval.next_trace_mask();
        let dst_src_col21 = eval.next_trace_mask();
        let dst_id_col22 = eval.next_trace_mask();
        let dst_limb_0_col23 = eval.next_trace_mask();
        let dst_limb_1_col24 = eval.next_trace_mask();
        let dst_limb_2_col25 = eval.next_trace_mask();
        let dst_limb_3_col26 = eval.next_trace_mask();
        let dst_limb_4_col27 = eval.next_trace_mask();
        let dst_limb_5_col28 = eval.next_trace_mask();
        let dst_limb_6_col29 = eval.next_trace_mask();
        let dst_limb_7_col30 = eval.next_trace_mask();
        let dst_limb_8_col31 = eval.next_trace_mask();
        let dst_limb_9_col32 = eval.next_trace_mask();
        let dst_limb_10_col33 = eval.next_trace_mask();
        let dst_limb_11_col34 = eval.next_trace_mask();
        let dst_limb_12_col35 = eval.next_trace_mask();
        let dst_limb_13_col36 = eval.next_trace_mask();
        let dst_limb_14_col37 = eval.next_trace_mask();
        let dst_limb_15_col38 = eval.next_trace_mask();
        let dst_limb_16_col39 = eval.next_trace_mask();
        let dst_limb_17_col40 = eval.next_trace_mask();
        let dst_limb_18_col41 = eval.next_trace_mask();
        let dst_limb_19_col42 = eval.next_trace_mask();
        let dst_limb_20_col43 = eval.next_trace_mask();
        let dst_limb_21_col44 = eval.next_trace_mask();
        let dst_limb_22_col45 = eval.next_trace_mask();
        let dst_limb_23_col46 = eval.next_trace_mask();
        let dst_limb_24_col47 = eval.next_trace_mask();
        let dst_limb_25_col48 = eval.next_trace_mask();
        let dst_limb_26_col49 = eval.next_trace_mask();
        let dst_limb_27_col50 = eval.next_trace_mask();
        let op0_src_col51 = eval.next_trace_mask();
        let op0_id_col52 = eval.next_trace_mask();
        let op0_limb_0_col53 = eval.next_trace_mask();
        let op0_limb_1_col54 = eval.next_trace_mask();
        let op0_limb_2_col55 = eval.next_trace_mask();
        let op0_limb_3_col56 = eval.next_trace_mask();
        let op0_limb_4_col57 = eval.next_trace_mask();
        let op0_limb_5_col58 = eval.next_trace_mask();
        let op0_limb_6_col59 = eval.next_trace_mask();
        let op0_limb_7_col60 = eval.next_trace_mask();
        let op0_limb_8_col61 = eval.next_trace_mask();
        let op0_limb_9_col62 = eval.next_trace_mask();
        let op0_limb_10_col63 = eval.next_trace_mask();
        let op0_limb_11_col64 = eval.next_trace_mask();
        let op0_limb_12_col65 = eval.next_trace_mask();
        let op0_limb_13_col66 = eval.next_trace_mask();
        let op0_limb_14_col67 = eval.next_trace_mask();
        let op0_limb_15_col68 = eval.next_trace_mask();
        let op0_limb_16_col69 = eval.next_trace_mask();
        let op0_limb_17_col70 = eval.next_trace_mask();
        let op0_limb_18_col71 = eval.next_trace_mask();
        let op0_limb_19_col72 = eval.next_trace_mask();
        let op0_limb_20_col73 = eval.next_trace_mask();
        let op0_limb_21_col74 = eval.next_trace_mask();
        let op0_limb_22_col75 = eval.next_trace_mask();
        let op0_limb_23_col76 = eval.next_trace_mask();
        let op0_limb_24_col77 = eval.next_trace_mask();
        let op0_limb_25_col78 = eval.next_trace_mask();
        let op0_limb_26_col79 = eval.next_trace_mask();
        let op0_limb_27_col80 = eval.next_trace_mask();
        let op1_src_col81 = eval.next_trace_mask();
        let op1_id_col82 = eval.next_trace_mask();
        let op1_limb_0_col83 = eval.next_trace_mask();
        let op1_limb_1_col84 = eval.next_trace_mask();
        let op1_limb_2_col85 = eval.next_trace_mask();
        let op1_limb_3_col86 = eval.next_trace_mask();
        let op1_limb_4_col87 = eval.next_trace_mask();
        let op1_limb_5_col88 = eval.next_trace_mask();
        let op1_limb_6_col89 = eval.next_trace_mask();
        let op1_limb_7_col90 = eval.next_trace_mask();
        let op1_limb_8_col91 = eval.next_trace_mask();
        let op1_limb_9_col92 = eval.next_trace_mask();
        let op1_limb_10_col93 = eval.next_trace_mask();
        let op1_limb_11_col94 = eval.next_trace_mask();
        let op1_limb_12_col95 = eval.next_trace_mask();
        let op1_limb_13_col96 = eval.next_trace_mask();
        let op1_limb_14_col97 = eval.next_trace_mask();
        let op1_limb_15_col98 = eval.next_trace_mask();
        let op1_limb_16_col99 = eval.next_trace_mask();
        let op1_limb_17_col100 = eval.next_trace_mask();
        let op1_limb_18_col101 = eval.next_trace_mask();
        let op1_limb_19_col102 = eval.next_trace_mask();
        let op1_limb_20_col103 = eval.next_trace_mask();
        let op1_limb_21_col104 = eval.next_trace_mask();
        let op1_limb_22_col105 = eval.next_trace_mask();
        let op1_limb_23_col106 = eval.next_trace_mask();
        let op1_limb_24_col107 = eval.next_trace_mask();
        let op1_limb_25_col108 = eval.next_trace_mask();
        let op1_limb_26_col109 = eval.next_trace_mask();
        let op1_limb_27_col110 = eval.next_trace_mask();
        let add_res_limb_0_col111 = eval.next_trace_mask();
        let add_res_limb_1_col112 = eval.next_trace_mask();
        let add_res_limb_2_col113 = eval.next_trace_mask();
        let add_res_limb_3_col114 = eval.next_trace_mask();
        let add_res_limb_4_col115 = eval.next_trace_mask();
        let add_res_limb_5_col116 = eval.next_trace_mask();
        let add_res_limb_6_col117 = eval.next_trace_mask();
        let add_res_limb_7_col118 = eval.next_trace_mask();
        let add_res_limb_8_col119 = eval.next_trace_mask();
        let add_res_limb_9_col120 = eval.next_trace_mask();
        let add_res_limb_10_col121 = eval.next_trace_mask();
        let add_res_limb_11_col122 = eval.next_trace_mask();
        let add_res_limb_12_col123 = eval.next_trace_mask();
        let add_res_limb_13_col124 = eval.next_trace_mask();
        let add_res_limb_14_col125 = eval.next_trace_mask();
        let add_res_limb_15_col126 = eval.next_trace_mask();
        let add_res_limb_16_col127 = eval.next_trace_mask();
        let add_res_limb_17_col128 = eval.next_trace_mask();
        let add_res_limb_18_col129 = eval.next_trace_mask();
        let add_res_limb_19_col130 = eval.next_trace_mask();
        let add_res_limb_20_col131 = eval.next_trace_mask();
        let add_res_limb_21_col132 = eval.next_trace_mask();
        let add_res_limb_22_col133 = eval.next_trace_mask();
        let add_res_limb_23_col134 = eval.next_trace_mask();
        let add_res_limb_24_col135 = eval.next_trace_mask();
        let add_res_limb_25_col136 = eval.next_trace_mask();
        let add_res_limb_26_col137 = eval.next_trace_mask();
        let add_res_limb_27_col138 = eval.next_trace_mask();
        let sub_p_bit_col139 = eval.next_trace_mask();
        let mul_res_limb_0_col140 = eval.next_trace_mask();
        let mul_res_limb_1_col141 = eval.next_trace_mask();
        let mul_res_limb_2_col142 = eval.next_trace_mask();
        let mul_res_limb_3_col143 = eval.next_trace_mask();
        let mul_res_limb_4_col144 = eval.next_trace_mask();
        let mul_res_limb_5_col145 = eval.next_trace_mask();
        let mul_res_limb_6_col146 = eval.next_trace_mask();
        let mul_res_limb_7_col147 = eval.next_trace_mask();
        let mul_res_limb_8_col148 = eval.next_trace_mask();
        let mul_res_limb_9_col149 = eval.next_trace_mask();
        let mul_res_limb_10_col150 = eval.next_trace_mask();
        let mul_res_limb_11_col151 = eval.next_trace_mask();
        let mul_res_limb_12_col152 = eval.next_trace_mask();
        let mul_res_limb_13_col153 = eval.next_trace_mask();
        let mul_res_limb_14_col154 = eval.next_trace_mask();
        let mul_res_limb_15_col155 = eval.next_trace_mask();
        let mul_res_limb_16_col156 = eval.next_trace_mask();
        let mul_res_limb_17_col157 = eval.next_trace_mask();
        let mul_res_limb_18_col158 = eval.next_trace_mask();
        let mul_res_limb_19_col159 = eval.next_trace_mask();
        let mul_res_limb_20_col160 = eval.next_trace_mask();
        let mul_res_limb_21_col161 = eval.next_trace_mask();
        let mul_res_limb_22_col162 = eval.next_trace_mask();
        let mul_res_limb_23_col163 = eval.next_trace_mask();
        let mul_res_limb_24_col164 = eval.next_trace_mask();
        let mul_res_limb_25_col165 = eval.next_trace_mask();
        let mul_res_limb_26_col166 = eval.next_trace_mask();
        let mul_res_limb_27_col167 = eval.next_trace_mask();
        let k_col168 = eval.next_trace_mask();
        let carry_0_col169 = eval.next_trace_mask();
        let carry_1_col170 = eval.next_trace_mask();
        let carry_2_col171 = eval.next_trace_mask();
        let carry_3_col172 = eval.next_trace_mask();
        let carry_4_col173 = eval.next_trace_mask();
        let carry_5_col174 = eval.next_trace_mask();
        let carry_6_col175 = eval.next_trace_mask();
        let carry_7_col176 = eval.next_trace_mask();
        let carry_8_col177 = eval.next_trace_mask();
        let carry_9_col178 = eval.next_trace_mask();
        let carry_10_col179 = eval.next_trace_mask();
        let carry_11_col180 = eval.next_trace_mask();
        let carry_12_col181 = eval.next_trace_mask();
        let carry_13_col182 = eval.next_trace_mask();
        let carry_14_col183 = eval.next_trace_mask();
        let carry_15_col184 = eval.next_trace_mask();
        let carry_16_col185 = eval.next_trace_mask();
        let carry_17_col186 = eval.next_trace_mask();
        let carry_18_col187 = eval.next_trace_mask();
        let carry_19_col188 = eval.next_trace_mask();
        let carry_20_col189 = eval.next_trace_mask();
        let carry_21_col190 = eval.next_trace_mask();
        let carry_22_col191 = eval.next_trace_mask();
        let carry_23_col192 = eval.next_trace_mask();
        let carry_24_col193 = eval.next_trace_mask();
        let carry_25_col194 = eval.next_trace_mask();
        let carry_26_col195 = eval.next_trace_mask();
        let res_limb_0_col196 = eval.next_trace_mask();
        let res_limb_1_col197 = eval.next_trace_mask();
        let res_limb_2_col198 = eval.next_trace_mask();
        let res_limb_3_col199 = eval.next_trace_mask();
        let res_limb_4_col200 = eval.next_trace_mask();
        let res_limb_5_col201 = eval.next_trace_mask();
        let res_limb_6_col202 = eval.next_trace_mask();
        let res_limb_7_col203 = eval.next_trace_mask();
        let res_limb_8_col204 = eval.next_trace_mask();
        let res_limb_9_col205 = eval.next_trace_mask();
        let res_limb_10_col206 = eval.next_trace_mask();
        let res_limb_11_col207 = eval.next_trace_mask();
        let res_limb_12_col208 = eval.next_trace_mask();
        let res_limb_13_col209 = eval.next_trace_mask();
        let res_limb_14_col210 = eval.next_trace_mask();
        let res_limb_15_col211 = eval.next_trace_mask();
        let res_limb_16_col212 = eval.next_trace_mask();
        let res_limb_17_col213 = eval.next_trace_mask();
        let res_limb_18_col214 = eval.next_trace_mask();
        let res_limb_19_col215 = eval.next_trace_mask();
        let res_limb_20_col216 = eval.next_trace_mask();
        let res_limb_21_col217 = eval.next_trace_mask();
        let res_limb_22_col218 = eval.next_trace_mask();
        let res_limb_23_col219 = eval.next_trace_mask();
        let res_limb_24_col220 = eval.next_trace_mask();
        let res_limb_25_col221 = eval.next_trace_mask();
        let res_limb_26_col222 = eval.next_trace_mask();
        let res_limb_27_col223 = eval.next_trace_mask();
        let msb_col224 = eval.next_trace_mask();
        let mid_limbs_set_col225 = eval.next_trace_mask();
        let dst_sum_squares_inv_col226 = eval.next_trace_mask();
        let dst_sum_inv_col227 = eval.next_trace_mask();
        let op1_as_rel_imm_cond_col228 = eval.next_trace_mask();
        let msb_col229 = eval.next_trace_mask();
        let mid_limbs_set_col230 = eval.next_trace_mask();
        let next_pc_jnz_col231 = eval.next_trace_mask();
        let next_pc_col232 = eval.next_trace_mask();
        let next_ap_col233 = eval.next_trace_mask();
        let next_fp_col234 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_generic_instruction_output_tmp_57455_26_limb_0, decode_generic_instruction_output_tmp_57455_26_limb_1, decode_generic_instruction_output_tmp_57455_26_limb_2, decode_generic_instruction_output_tmp_57455_26_limb_3, decode_generic_instruction_output_tmp_57455_26_limb_4, decode_generic_instruction_output_tmp_57455_26_limb_5, decode_generic_instruction_output_tmp_57455_26_limb_6, decode_generic_instruction_output_tmp_57455_26_limb_7, decode_generic_instruction_output_tmp_57455_26_limb_8, decode_generic_instruction_output_tmp_57455_26_limb_9, decode_generic_instruction_output_tmp_57455_26_limb_10, decode_generic_instruction_output_tmp_57455_26_limb_11, decode_generic_instruction_output_tmp_57455_26_limb_12, decode_generic_instruction_output_tmp_57455_26_limb_13, decode_generic_instruction_output_tmp_57455_26_limb_14, decode_generic_instruction_output_tmp_57455_26_limb_15, decode_generic_instruction_output_tmp_57455_26_limb_16, decode_generic_instruction_output_tmp_57455_26_limb_17, decode_generic_instruction_output_tmp_57455_26_limb_18, decode_generic_instruction_output_tmp_57455_26_limb_19, decode_generic_instruction_output_tmp_57455_26_limb_20, decode_generic_instruction_output_tmp_57455_26_limb_21, decode_generic_instruction_output_tmp_57455_26_limb_22] =
            DecodeGenericInstruction::evaluate(
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_imm_col8.clone(),
                op1_base_fp_col9.clone(),
                op1_base_ap_col10.clone(),
                res_add_col11.clone(),
                res_mul_col12.clone(),
                pc_update_jump_col13.clone(),
                pc_update_jump_rel_col14.clone(),
                pc_update_jnz_col15.clone(),
                ap_update_add_col16.clone(),
                ap_update_add_1_col17.clone(),
                opcode_call_col18.clone(),
                opcode_ret_col19.clone(),
                opcode_assert_eq_col20.clone(),
                &self.verify_instruction_lookup_elements,
                &mut eval,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [eval_operands_output_tmp_57455_92_limb_0, eval_operands_output_tmp_57455_92_limb_1, eval_operands_output_tmp_57455_92_limb_2, eval_operands_output_tmp_57455_92_limb_3, eval_operands_output_tmp_57455_92_limb_4, eval_operands_output_tmp_57455_92_limb_5, eval_operands_output_tmp_57455_92_limb_6, eval_operands_output_tmp_57455_92_limb_7, eval_operands_output_tmp_57455_92_limb_8, eval_operands_output_tmp_57455_92_limb_9, eval_operands_output_tmp_57455_92_limb_10, eval_operands_output_tmp_57455_92_limb_11, eval_operands_output_tmp_57455_92_limb_12, eval_operands_output_tmp_57455_92_limb_13, eval_operands_output_tmp_57455_92_limb_14, eval_operands_output_tmp_57455_92_limb_15, eval_operands_output_tmp_57455_92_limb_16, eval_operands_output_tmp_57455_92_limb_17, eval_operands_output_tmp_57455_92_limb_18, eval_operands_output_tmp_57455_92_limb_19, eval_operands_output_tmp_57455_92_limb_20, eval_operands_output_tmp_57455_92_limb_21, eval_operands_output_tmp_57455_92_limb_22, eval_operands_output_tmp_57455_92_limb_23, eval_operands_output_tmp_57455_92_limb_24, eval_operands_output_tmp_57455_92_limb_25, eval_operands_output_tmp_57455_92_limb_26, eval_operands_output_tmp_57455_92_limb_27, eval_operands_output_tmp_57455_92_limb_28, eval_operands_output_tmp_57455_92_limb_29, eval_operands_output_tmp_57455_92_limb_30, eval_operands_output_tmp_57455_92_limb_31, eval_operands_output_tmp_57455_92_limb_32, eval_operands_output_tmp_57455_92_limb_33, eval_operands_output_tmp_57455_92_limb_34, eval_operands_output_tmp_57455_92_limb_35, eval_operands_output_tmp_57455_92_limb_36, eval_operands_output_tmp_57455_92_limb_37, eval_operands_output_tmp_57455_92_limb_38, eval_operands_output_tmp_57455_92_limb_39, eval_operands_output_tmp_57455_92_limb_40, eval_operands_output_tmp_57455_92_limb_41, eval_operands_output_tmp_57455_92_limb_42, eval_operands_output_tmp_57455_92_limb_43, eval_operands_output_tmp_57455_92_limb_44, eval_operands_output_tmp_57455_92_limb_45, eval_operands_output_tmp_57455_92_limb_46, eval_operands_output_tmp_57455_92_limb_47, eval_operands_output_tmp_57455_92_limb_48, eval_operands_output_tmp_57455_92_limb_49, eval_operands_output_tmp_57455_92_limb_50, eval_operands_output_tmp_57455_92_limb_51, eval_operands_output_tmp_57455_92_limb_52, eval_operands_output_tmp_57455_92_limb_53, eval_operands_output_tmp_57455_92_limb_54, eval_operands_output_tmp_57455_92_limb_55, eval_operands_output_tmp_57455_92_limb_56, eval_operands_output_tmp_57455_92_limb_57, eval_operands_output_tmp_57455_92_limb_58, eval_operands_output_tmp_57455_92_limb_59, eval_operands_output_tmp_57455_92_limb_60, eval_operands_output_tmp_57455_92_limb_61, eval_operands_output_tmp_57455_92_limb_62, eval_operands_output_tmp_57455_92_limb_63, eval_operands_output_tmp_57455_92_limb_64, eval_operands_output_tmp_57455_92_limb_65, eval_operands_output_tmp_57455_92_limb_66, eval_operands_output_tmp_57455_92_limb_67, eval_operands_output_tmp_57455_92_limb_68, eval_operands_output_tmp_57455_92_limb_69, eval_operands_output_tmp_57455_92_limb_70, eval_operands_output_tmp_57455_92_limb_71, eval_operands_output_tmp_57455_92_limb_72, eval_operands_output_tmp_57455_92_limb_73, eval_operands_output_tmp_57455_92_limb_74, eval_operands_output_tmp_57455_92_limb_75, eval_operands_output_tmp_57455_92_limb_76, eval_operands_output_tmp_57455_92_limb_77, eval_operands_output_tmp_57455_92_limb_78, eval_operands_output_tmp_57455_92_limb_79, eval_operands_output_tmp_57455_92_limb_80, eval_operands_output_tmp_57455_92_limb_81, eval_operands_output_tmp_57455_92_limb_82, eval_operands_output_tmp_57455_92_limb_83, eval_operands_output_tmp_57455_92_limb_84, eval_operands_output_tmp_57455_92_limb_85, eval_operands_output_tmp_57455_92_limb_86, eval_operands_output_tmp_57455_92_limb_87, eval_operands_output_tmp_57455_92_limb_88, eval_operands_output_tmp_57455_92_limb_89, eval_operands_output_tmp_57455_92_limb_90, eval_operands_output_tmp_57455_92_limb_91, eval_operands_output_tmp_57455_92_limb_92, eval_operands_output_tmp_57455_92_limb_93, eval_operands_output_tmp_57455_92_limb_94, eval_operands_output_tmp_57455_92_limb_95, eval_operands_output_tmp_57455_92_limb_96, eval_operands_output_tmp_57455_92_limb_97, eval_operands_output_tmp_57455_92_limb_98, eval_operands_output_tmp_57455_92_limb_99, eval_operands_output_tmp_57455_92_limb_100, eval_operands_output_tmp_57455_92_limb_101, eval_operands_output_tmp_57455_92_limb_102, eval_operands_output_tmp_57455_92_limb_103, eval_operands_output_tmp_57455_92_limb_104, eval_operands_output_tmp_57455_92_limb_105, eval_operands_output_tmp_57455_92_limb_106, eval_operands_output_tmp_57455_92_limb_107, eval_operands_output_tmp_57455_92_limb_108, eval_operands_output_tmp_57455_92_limb_109, eval_operands_output_tmp_57455_92_limb_110, eval_operands_output_tmp_57455_92_limb_111] =
            EvalOperands::evaluate(
                [
                    input_pc_col0.clone(),
                    input_ap_col1.clone(),
                    input_fp_col2.clone(),
                    dst_base_fp_col6.clone(),
                    op0_base_fp_col7.clone(),
                    op1_imm_col8.clone(),
                    op1_base_fp_col9.clone(),
                    op1_base_ap_col10.clone(),
                    res_add_col11.clone(),
                    res_mul_col12.clone(),
                    pc_update_jump_col13.clone(),
                    pc_update_jump_rel_col14.clone(),
                    pc_update_jnz_col15.clone(),
                    ap_update_add_col16.clone(),
                    ap_update_add_1_col17.clone(),
                    opcode_call_col18.clone(),
                    opcode_ret_col19.clone(),
                    opcode_assert_eq_col20.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_15.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_16.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_17.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_18.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_19.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_20.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_21.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_22.clone(),
                ],
                dst_src_col21.clone(),
                dst_id_col22.clone(),
                dst_limb_0_col23.clone(),
                dst_limb_1_col24.clone(),
                dst_limb_2_col25.clone(),
                dst_limb_3_col26.clone(),
                dst_limb_4_col27.clone(),
                dst_limb_5_col28.clone(),
                dst_limb_6_col29.clone(),
                dst_limb_7_col30.clone(),
                dst_limb_8_col31.clone(),
                dst_limb_9_col32.clone(),
                dst_limb_10_col33.clone(),
                dst_limb_11_col34.clone(),
                dst_limb_12_col35.clone(),
                dst_limb_13_col36.clone(),
                dst_limb_14_col37.clone(),
                dst_limb_15_col38.clone(),
                dst_limb_16_col39.clone(),
                dst_limb_17_col40.clone(),
                dst_limb_18_col41.clone(),
                dst_limb_19_col42.clone(),
                dst_limb_20_col43.clone(),
                dst_limb_21_col44.clone(),
                dst_limb_22_col45.clone(),
                dst_limb_23_col46.clone(),
                dst_limb_24_col47.clone(),
                dst_limb_25_col48.clone(),
                dst_limb_26_col49.clone(),
                dst_limb_27_col50.clone(),
                op0_src_col51.clone(),
                op0_id_col52.clone(),
                op0_limb_0_col53.clone(),
                op0_limb_1_col54.clone(),
                op0_limb_2_col55.clone(),
                op0_limb_3_col56.clone(),
                op0_limb_4_col57.clone(),
                op0_limb_5_col58.clone(),
                op0_limb_6_col59.clone(),
                op0_limb_7_col60.clone(),
                op0_limb_8_col61.clone(),
                op0_limb_9_col62.clone(),
                op0_limb_10_col63.clone(),
                op0_limb_11_col64.clone(),
                op0_limb_12_col65.clone(),
                op0_limb_13_col66.clone(),
                op0_limb_14_col67.clone(),
                op0_limb_15_col68.clone(),
                op0_limb_16_col69.clone(),
                op0_limb_17_col70.clone(),
                op0_limb_18_col71.clone(),
                op0_limb_19_col72.clone(),
                op0_limb_20_col73.clone(),
                op0_limb_21_col74.clone(),
                op0_limb_22_col75.clone(),
                op0_limb_23_col76.clone(),
                op0_limb_24_col77.clone(),
                op0_limb_25_col78.clone(),
                op0_limb_26_col79.clone(),
                op0_limb_27_col80.clone(),
                op1_src_col81.clone(),
                op1_id_col82.clone(),
                op1_limb_0_col83.clone(),
                op1_limb_1_col84.clone(),
                op1_limb_2_col85.clone(),
                op1_limb_3_col86.clone(),
                op1_limb_4_col87.clone(),
                op1_limb_5_col88.clone(),
                op1_limb_6_col89.clone(),
                op1_limb_7_col90.clone(),
                op1_limb_8_col91.clone(),
                op1_limb_9_col92.clone(),
                op1_limb_10_col93.clone(),
                op1_limb_11_col94.clone(),
                op1_limb_12_col95.clone(),
                op1_limb_13_col96.clone(),
                op1_limb_14_col97.clone(),
                op1_limb_15_col98.clone(),
                op1_limb_16_col99.clone(),
                op1_limb_17_col100.clone(),
                op1_limb_18_col101.clone(),
                op1_limb_19_col102.clone(),
                op1_limb_20_col103.clone(),
                op1_limb_21_col104.clone(),
                op1_limb_22_col105.clone(),
                op1_limb_23_col106.clone(),
                op1_limb_24_col107.clone(),
                op1_limb_25_col108.clone(),
                op1_limb_26_col109.clone(),
                op1_limb_27_col110.clone(),
                add_res_limb_0_col111.clone(),
                add_res_limb_1_col112.clone(),
                add_res_limb_2_col113.clone(),
                add_res_limb_3_col114.clone(),
                add_res_limb_4_col115.clone(),
                add_res_limb_5_col116.clone(),
                add_res_limb_6_col117.clone(),
                add_res_limb_7_col118.clone(),
                add_res_limb_8_col119.clone(),
                add_res_limb_9_col120.clone(),
                add_res_limb_10_col121.clone(),
                add_res_limb_11_col122.clone(),
                add_res_limb_12_col123.clone(),
                add_res_limb_13_col124.clone(),
                add_res_limb_14_col125.clone(),
                add_res_limb_15_col126.clone(),
                add_res_limb_16_col127.clone(),
                add_res_limb_17_col128.clone(),
                add_res_limb_18_col129.clone(),
                add_res_limb_19_col130.clone(),
                add_res_limb_20_col131.clone(),
                add_res_limb_21_col132.clone(),
                add_res_limb_22_col133.clone(),
                add_res_limb_23_col134.clone(),
                add_res_limb_24_col135.clone(),
                add_res_limb_25_col136.clone(),
                add_res_limb_26_col137.clone(),
                add_res_limb_27_col138.clone(),
                sub_p_bit_col139.clone(),
                mul_res_limb_0_col140.clone(),
                mul_res_limb_1_col141.clone(),
                mul_res_limb_2_col142.clone(),
                mul_res_limb_3_col143.clone(),
                mul_res_limb_4_col144.clone(),
                mul_res_limb_5_col145.clone(),
                mul_res_limb_6_col146.clone(),
                mul_res_limb_7_col147.clone(),
                mul_res_limb_8_col148.clone(),
                mul_res_limb_9_col149.clone(),
                mul_res_limb_10_col150.clone(),
                mul_res_limb_11_col151.clone(),
                mul_res_limb_12_col152.clone(),
                mul_res_limb_13_col153.clone(),
                mul_res_limb_14_col154.clone(),
                mul_res_limb_15_col155.clone(),
                mul_res_limb_16_col156.clone(),
                mul_res_limb_17_col157.clone(),
                mul_res_limb_18_col158.clone(),
                mul_res_limb_19_col159.clone(),
                mul_res_limb_20_col160.clone(),
                mul_res_limb_21_col161.clone(),
                mul_res_limb_22_col162.clone(),
                mul_res_limb_23_col163.clone(),
                mul_res_limb_24_col164.clone(),
                mul_res_limb_25_col165.clone(),
                mul_res_limb_26_col166.clone(),
                mul_res_limb_27_col167.clone(),
                k_col168.clone(),
                carry_0_col169.clone(),
                carry_1_col170.clone(),
                carry_2_col171.clone(),
                carry_3_col172.clone(),
                carry_4_col173.clone(),
                carry_5_col174.clone(),
                carry_6_col175.clone(),
                carry_7_col176.clone(),
                carry_8_col177.clone(),
                carry_9_col178.clone(),
                carry_10_col179.clone(),
                carry_11_col180.clone(),
                carry_12_col181.clone(),
                carry_13_col182.clone(),
                carry_14_col183.clone(),
                carry_15_col184.clone(),
                carry_16_col185.clone(),
                carry_17_col186.clone(),
                carry_18_col187.clone(),
                carry_19_col188.clone(),
                carry_20_col189.clone(),
                carry_21_col190.clone(),
                carry_22_col191.clone(),
                carry_23_col192.clone(),
                carry_24_col193.clone(),
                carry_25_col194.clone(),
                carry_26_col195.clone(),
                res_limb_0_col196.clone(),
                res_limb_1_col197.clone(),
                res_limb_2_col198.clone(),
                res_limb_3_col199.clone(),
                res_limb_4_col200.clone(),
                res_limb_5_col201.clone(),
                res_limb_6_col202.clone(),
                res_limb_7_col203.clone(),
                res_limb_8_col204.clone(),
                res_limb_9_col205.clone(),
                res_limb_10_col206.clone(),
                res_limb_11_col207.clone(),
                res_limb_12_col208.clone(),
                res_limb_13_col209.clone(),
                res_limb_14_col210.clone(),
                res_limb_15_col211.clone(),
                res_limb_16_col212.clone(),
                res_limb_17_col213.clone(),
                res_limb_18_col214.clone(),
                res_limb_19_col215.clone(),
                res_limb_20_col216.clone(),
                res_limb_21_col217.clone(),
                res_limb_22_col218.clone(),
                res_limb_23_col219.clone(),
                res_limb_24_col220.clone(),
                res_limb_25_col221.clone(),
                res_limb_26_col222.clone(),
                res_limb_27_col223.clone(),
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
                &self.range_check_9_9_lookup_elements,
                &self.range_check_19_lookup_elements,
                &mut eval,
            );
        HandleOpcodes::evaluate(
            [
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_imm_col8.clone(),
                op1_base_fp_col9.clone(),
                op1_base_ap_col10.clone(),
                res_add_col11.clone(),
                res_mul_col12.clone(),
                pc_update_jump_col13.clone(),
                pc_update_jump_rel_col14.clone(),
                pc_update_jnz_col15.clone(),
                ap_update_add_col16.clone(),
                ap_update_add_1_col17.clone(),
                opcode_call_col18.clone(),
                opcode_ret_col19.clone(),
                opcode_assert_eq_col20.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_15.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_16.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_17.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_18.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_19.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_20.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_21.clone(),
                decode_generic_instruction_output_tmp_57455_26_limb_22.clone(),
                dst_limb_0_col23.clone(),
                dst_limb_1_col24.clone(),
                dst_limb_2_col25.clone(),
                dst_limb_3_col26.clone(),
                dst_limb_4_col27.clone(),
                dst_limb_5_col28.clone(),
                dst_limb_6_col29.clone(),
                dst_limb_7_col30.clone(),
                dst_limb_8_col31.clone(),
                dst_limb_9_col32.clone(),
                dst_limb_10_col33.clone(),
                dst_limb_11_col34.clone(),
                dst_limb_12_col35.clone(),
                dst_limb_13_col36.clone(),
                dst_limb_14_col37.clone(),
                dst_limb_15_col38.clone(),
                dst_limb_16_col39.clone(),
                dst_limb_17_col40.clone(),
                dst_limb_18_col41.clone(),
                dst_limb_19_col42.clone(),
                dst_limb_20_col43.clone(),
                dst_limb_21_col44.clone(),
                dst_limb_22_col45.clone(),
                dst_limb_23_col46.clone(),
                dst_limb_24_col47.clone(),
                dst_limb_25_col48.clone(),
                dst_limb_26_col49.clone(),
                dst_limb_27_col50.clone(),
                op0_limb_0_col53.clone(),
                op0_limb_1_col54.clone(),
                op0_limb_2_col55.clone(),
                op0_limb_3_col56.clone(),
                op0_limb_4_col57.clone(),
                op0_limb_5_col58.clone(),
                op0_limb_6_col59.clone(),
                op0_limb_7_col60.clone(),
                op0_limb_8_col61.clone(),
                op0_limb_9_col62.clone(),
                op0_limb_10_col63.clone(),
                op0_limb_11_col64.clone(),
                op0_limb_12_col65.clone(),
                op0_limb_13_col66.clone(),
                op0_limb_14_col67.clone(),
                op0_limb_15_col68.clone(),
                op0_limb_16_col69.clone(),
                op0_limb_17_col70.clone(),
                op0_limb_18_col71.clone(),
                op0_limb_19_col72.clone(),
                op0_limb_20_col73.clone(),
                op0_limb_21_col74.clone(),
                op0_limb_22_col75.clone(),
                op0_limb_23_col76.clone(),
                op0_limb_24_col77.clone(),
                op0_limb_25_col78.clone(),
                op0_limb_26_col79.clone(),
                op0_limb_27_col80.clone(),
                res_limb_0_col196.clone(),
                res_limb_1_col197.clone(),
                res_limb_2_col198.clone(),
                res_limb_3_col199.clone(),
                res_limb_4_col200.clone(),
                res_limb_5_col201.clone(),
                res_limb_6_col202.clone(),
                res_limb_7_col203.clone(),
                res_limb_8_col204.clone(),
                res_limb_9_col205.clone(),
                res_limb_10_col206.clone(),
                res_limb_11_col207.clone(),
                res_limb_12_col208.clone(),
                res_limb_13_col209.clone(),
                res_limb_14_col210.clone(),
                res_limb_15_col211.clone(),
                res_limb_16_col212.clone(),
                res_limb_17_col213.clone(),
                res_limb_18_col214.clone(),
                res_limb_19_col215.clone(),
                res_limb_20_col216.clone(),
                res_limb_21_col217.clone(),
                res_limb_22_col218.clone(),
                res_limb_23_col219.clone(),
                res_limb_24_col220.clone(),
                res_limb_25_col221.clone(),
                res_limb_26_col222.clone(),
                res_limb_27_col223.clone(),
            ],
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [update_registers_output_tmp_57455_110_pc, update_registers_output_tmp_57455_110_ap, update_registers_output_tmp_57455_110_fp] =
            UpdateRegisters::evaluate(
                [
                    input_pc_col0.clone(),
                    input_ap_col1.clone(),
                    input_fp_col2.clone(),
                    dst_base_fp_col6.clone(),
                    op0_base_fp_col7.clone(),
                    op1_imm_col8.clone(),
                    op1_base_fp_col9.clone(),
                    op1_base_ap_col10.clone(),
                    res_add_col11.clone(),
                    res_mul_col12.clone(),
                    pc_update_jump_col13.clone(),
                    pc_update_jump_rel_col14.clone(),
                    pc_update_jnz_col15.clone(),
                    ap_update_add_col16.clone(),
                    ap_update_add_1_col17.clone(),
                    opcode_call_col18.clone(),
                    opcode_ret_col19.clone(),
                    opcode_assert_eq_col20.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_15.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_16.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_17.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_18.clone(),
                    decode_generic_instruction_output_tmp_57455_26_limb_19.clone(),
                    dst_limb_0_col23.clone(),
                    dst_limb_1_col24.clone(),
                    dst_limb_2_col25.clone(),
                    dst_limb_3_col26.clone(),
                    dst_limb_4_col27.clone(),
                    dst_limb_5_col28.clone(),
                    dst_limb_6_col29.clone(),
                    dst_limb_7_col30.clone(),
                    dst_limb_8_col31.clone(),
                    dst_limb_9_col32.clone(),
                    dst_limb_10_col33.clone(),
                    dst_limb_11_col34.clone(),
                    dst_limb_12_col35.clone(),
                    dst_limb_13_col36.clone(),
                    dst_limb_14_col37.clone(),
                    dst_limb_15_col38.clone(),
                    dst_limb_16_col39.clone(),
                    dst_limb_17_col40.clone(),
                    dst_limb_18_col41.clone(),
                    dst_limb_19_col42.clone(),
                    dst_limb_20_col43.clone(),
                    dst_limb_21_col44.clone(),
                    dst_limb_22_col45.clone(),
                    dst_limb_23_col46.clone(),
                    dst_limb_24_col47.clone(),
                    dst_limb_25_col48.clone(),
                    dst_limb_26_col49.clone(),
                    dst_limb_27_col50.clone(),
                    op1_limb_0_col83.clone(),
                    op1_limb_1_col84.clone(),
                    op1_limb_2_col85.clone(),
                    op1_limb_3_col86.clone(),
                    op1_limb_4_col87.clone(),
                    op1_limb_5_col88.clone(),
                    op1_limb_6_col89.clone(),
                    op1_limb_7_col90.clone(),
                    op1_limb_8_col91.clone(),
                    op1_limb_9_col92.clone(),
                    op1_limb_10_col93.clone(),
                    op1_limb_11_col94.clone(),
                    op1_limb_12_col95.clone(),
                    op1_limb_13_col96.clone(),
                    op1_limb_14_col97.clone(),
                    op1_limb_15_col98.clone(),
                    op1_limb_16_col99.clone(),
                    op1_limb_17_col100.clone(),
                    op1_limb_18_col101.clone(),
                    op1_limb_19_col102.clone(),
                    op1_limb_20_col103.clone(),
                    op1_limb_21_col104.clone(),
                    op1_limb_22_col105.clone(),
                    op1_limb_23_col106.clone(),
                    op1_limb_24_col107.clone(),
                    op1_limb_25_col108.clone(),
                    op1_limb_26_col109.clone(),
                    op1_limb_27_col110.clone(),
                    res_limb_0_col196.clone(),
                    res_limb_1_col197.clone(),
                    res_limb_2_col198.clone(),
                    res_limb_3_col199.clone(),
                    res_limb_4_col200.clone(),
                    res_limb_5_col201.clone(),
                    res_limb_6_col202.clone(),
                    res_limb_7_col203.clone(),
                    res_limb_8_col204.clone(),
                    res_limb_9_col205.clone(),
                    res_limb_10_col206.clone(),
                    res_limb_11_col207.clone(),
                    res_limb_12_col208.clone(),
                    res_limb_13_col209.clone(),
                    res_limb_14_col210.clone(),
                    res_limb_15_col211.clone(),
                    res_limb_16_col212.clone(),
                    res_limb_17_col213.clone(),
                    res_limb_18_col214.clone(),
                    res_limb_19_col215.clone(),
                    res_limb_20_col216.clone(),
                    res_limb_21_col217.clone(),
                    res_limb_22_col218.clone(),
                    res_limb_23_col219.clone(),
                    res_limb_24_col220.clone(),
                    res_limb_25_col221.clone(),
                    res_limb_26_col222.clone(),
                    res_limb_27_col223.clone(),
                ],
                msb_col224.clone(),
                mid_limbs_set_col225.clone(),
                dst_sum_squares_inv_col226.clone(),
                dst_sum_inv_col227.clone(),
                op1_as_rel_imm_cond_col228.clone(),
                msb_col229.clone(),
                mid_limbs_set_col230.clone(),
                next_pc_jnz_col231.clone(),
                next_pc_col232.clone(),
                next_ap_col233.clone(),
                next_fp_col234.clone(),
                &mut eval,
            );
        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::from(enabler.clone()),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::from(enabler.clone()),
            &[
                next_pc_col232.clone(),
                next_ap_col233.clone(),
                next_fp_col234.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

    use super::*;
    use crate::components::constraints_regression_test_values::GENERIC_OPCODE;

    #[test]
    fn generic_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, GENERIC_OPCODE);
    }
}

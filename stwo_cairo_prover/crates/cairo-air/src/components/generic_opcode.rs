// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::decode_generic_instruction::DecodeGenericInstruction;
use crate::components::subroutines::eval_operands::EvalOperands;
use crate::components::subroutines::handle_opcodes::HandleOpcodes;
use crate::components::subroutines::update_registers::UpdateRegisters;

pub const N_TRACE_COLUMNS: usize = 244;
pub const RELATION_USES_PER_ROW: [RelationUse; 22] = [
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
        relation_id: "RangeCheck_11",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_18",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_20",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_20_B",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_20_C",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_20_D",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_20_E",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_20_F",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_20_G",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_20_H",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 4,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 2,
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
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
    pub range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
    pub range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
    pub range_check_20_lookup_elements: relations::RangeCheck_20,
    pub range_check_20_b_lookup_elements: relations::RangeCheck_20_B,
    pub range_check_20_c_lookup_elements: relations::RangeCheck_20_C,
    pub range_check_20_d_lookup_elements: relations::RangeCheck_20_D,
    pub range_check_20_e_lookup_elements: relations::RangeCheck_20_E,
    pub range_check_20_f_lookup_elements: relations::RangeCheck_20_F,
    pub range_check_20_g_lookup_elements: relations::RangeCheck_20_G,
    pub range_check_20_h_lookup_elements: relations::RangeCheck_20_H,
    pub range_check_18_lookup_elements: relations::RangeCheck_18,
    pub range_check_11_lookup_elements: relations::RangeCheck_11,
    pub opcodes_lookup_elements: relations::Opcodes,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 34];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        let partial_limb_msb_col81 = eval.next_trace_mask();
        let op1_src_col82 = eval.next_trace_mask();
        let op1_id_col83 = eval.next_trace_mask();
        let op1_limb_0_col84 = eval.next_trace_mask();
        let op1_limb_1_col85 = eval.next_trace_mask();
        let op1_limb_2_col86 = eval.next_trace_mask();
        let op1_limb_3_col87 = eval.next_trace_mask();
        let op1_limb_4_col88 = eval.next_trace_mask();
        let op1_limb_5_col89 = eval.next_trace_mask();
        let op1_limb_6_col90 = eval.next_trace_mask();
        let op1_limb_7_col91 = eval.next_trace_mask();
        let op1_limb_8_col92 = eval.next_trace_mask();
        let op1_limb_9_col93 = eval.next_trace_mask();
        let op1_limb_10_col94 = eval.next_trace_mask();
        let op1_limb_11_col95 = eval.next_trace_mask();
        let op1_limb_12_col96 = eval.next_trace_mask();
        let op1_limb_13_col97 = eval.next_trace_mask();
        let op1_limb_14_col98 = eval.next_trace_mask();
        let op1_limb_15_col99 = eval.next_trace_mask();
        let op1_limb_16_col100 = eval.next_trace_mask();
        let op1_limb_17_col101 = eval.next_trace_mask();
        let op1_limb_18_col102 = eval.next_trace_mask();
        let op1_limb_19_col103 = eval.next_trace_mask();
        let op1_limb_20_col104 = eval.next_trace_mask();
        let op1_limb_21_col105 = eval.next_trace_mask();
        let op1_limb_22_col106 = eval.next_trace_mask();
        let op1_limb_23_col107 = eval.next_trace_mask();
        let op1_limb_24_col108 = eval.next_trace_mask();
        let op1_limb_25_col109 = eval.next_trace_mask();
        let op1_limb_26_col110 = eval.next_trace_mask();
        let op1_limb_27_col111 = eval.next_trace_mask();
        let add_res_limb_0_col112 = eval.next_trace_mask();
        let add_res_limb_1_col113 = eval.next_trace_mask();
        let add_res_limb_2_col114 = eval.next_trace_mask();
        let add_res_limb_3_col115 = eval.next_trace_mask();
        let add_res_limb_4_col116 = eval.next_trace_mask();
        let add_res_limb_5_col117 = eval.next_trace_mask();
        let add_res_limb_6_col118 = eval.next_trace_mask();
        let add_res_limb_7_col119 = eval.next_trace_mask();
        let add_res_limb_8_col120 = eval.next_trace_mask();
        let add_res_limb_9_col121 = eval.next_trace_mask();
        let add_res_limb_10_col122 = eval.next_trace_mask();
        let add_res_limb_11_col123 = eval.next_trace_mask();
        let add_res_limb_12_col124 = eval.next_trace_mask();
        let add_res_limb_13_col125 = eval.next_trace_mask();
        let add_res_limb_14_col126 = eval.next_trace_mask();
        let add_res_limb_15_col127 = eval.next_trace_mask();
        let add_res_limb_16_col128 = eval.next_trace_mask();
        let add_res_limb_17_col129 = eval.next_trace_mask();
        let add_res_limb_18_col130 = eval.next_trace_mask();
        let add_res_limb_19_col131 = eval.next_trace_mask();
        let add_res_limb_20_col132 = eval.next_trace_mask();
        let add_res_limb_21_col133 = eval.next_trace_mask();
        let add_res_limb_22_col134 = eval.next_trace_mask();
        let add_res_limb_23_col135 = eval.next_trace_mask();
        let add_res_limb_24_col136 = eval.next_trace_mask();
        let add_res_limb_25_col137 = eval.next_trace_mask();
        let add_res_limb_26_col138 = eval.next_trace_mask();
        let add_res_limb_27_col139 = eval.next_trace_mask();
        let sub_p_bit_col140 = eval.next_trace_mask();
        let mul_res_limb_0_col141 = eval.next_trace_mask();
        let mul_res_limb_1_col142 = eval.next_trace_mask();
        let mul_res_limb_2_col143 = eval.next_trace_mask();
        let mul_res_limb_3_col144 = eval.next_trace_mask();
        let mul_res_limb_4_col145 = eval.next_trace_mask();
        let mul_res_limb_5_col146 = eval.next_trace_mask();
        let mul_res_limb_6_col147 = eval.next_trace_mask();
        let mul_res_limb_7_col148 = eval.next_trace_mask();
        let mul_res_limb_8_col149 = eval.next_trace_mask();
        let mul_res_limb_9_col150 = eval.next_trace_mask();
        let mul_res_limb_10_col151 = eval.next_trace_mask();
        let mul_res_limb_11_col152 = eval.next_trace_mask();
        let mul_res_limb_12_col153 = eval.next_trace_mask();
        let mul_res_limb_13_col154 = eval.next_trace_mask();
        let mul_res_limb_14_col155 = eval.next_trace_mask();
        let mul_res_limb_15_col156 = eval.next_trace_mask();
        let mul_res_limb_16_col157 = eval.next_trace_mask();
        let mul_res_limb_17_col158 = eval.next_trace_mask();
        let mul_res_limb_18_col159 = eval.next_trace_mask();
        let mul_res_limb_19_col160 = eval.next_trace_mask();
        let mul_res_limb_20_col161 = eval.next_trace_mask();
        let mul_res_limb_21_col162 = eval.next_trace_mask();
        let mul_res_limb_22_col163 = eval.next_trace_mask();
        let mul_res_limb_23_col164 = eval.next_trace_mask();
        let mul_res_limb_24_col165 = eval.next_trace_mask();
        let mul_res_limb_25_col166 = eval.next_trace_mask();
        let mul_res_limb_26_col167 = eval.next_trace_mask();
        let mul_res_limb_27_col168 = eval.next_trace_mask();
        let k_col169 = eval.next_trace_mask();
        let carry_0_col170 = eval.next_trace_mask();
        let carry_1_col171 = eval.next_trace_mask();
        let carry_2_col172 = eval.next_trace_mask();
        let carry_3_col173 = eval.next_trace_mask();
        let carry_4_col174 = eval.next_trace_mask();
        let carry_5_col175 = eval.next_trace_mask();
        let carry_6_col176 = eval.next_trace_mask();
        let carry_7_col177 = eval.next_trace_mask();
        let carry_8_col178 = eval.next_trace_mask();
        let carry_9_col179 = eval.next_trace_mask();
        let carry_10_col180 = eval.next_trace_mask();
        let carry_11_col181 = eval.next_trace_mask();
        let carry_12_col182 = eval.next_trace_mask();
        let carry_13_col183 = eval.next_trace_mask();
        let carry_14_col184 = eval.next_trace_mask();
        let carry_15_col185 = eval.next_trace_mask();
        let carry_16_col186 = eval.next_trace_mask();
        let carry_17_col187 = eval.next_trace_mask();
        let carry_18_col188 = eval.next_trace_mask();
        let carry_19_col189 = eval.next_trace_mask();
        let carry_20_col190 = eval.next_trace_mask();
        let carry_21_col191 = eval.next_trace_mask();
        let carry_22_col192 = eval.next_trace_mask();
        let carry_23_col193 = eval.next_trace_mask();
        let carry_24_col194 = eval.next_trace_mask();
        let carry_25_col195 = eval.next_trace_mask();
        let carry_26_col196 = eval.next_trace_mask();
        let res_limb_0_col197 = eval.next_trace_mask();
        let res_limb_1_col198 = eval.next_trace_mask();
        let res_limb_2_col199 = eval.next_trace_mask();
        let res_limb_3_col200 = eval.next_trace_mask();
        let res_limb_4_col201 = eval.next_trace_mask();
        let res_limb_5_col202 = eval.next_trace_mask();
        let res_limb_6_col203 = eval.next_trace_mask();
        let res_limb_7_col204 = eval.next_trace_mask();
        let res_limb_8_col205 = eval.next_trace_mask();
        let res_limb_9_col206 = eval.next_trace_mask();
        let res_limb_10_col207 = eval.next_trace_mask();
        let res_limb_11_col208 = eval.next_trace_mask();
        let res_limb_12_col209 = eval.next_trace_mask();
        let res_limb_13_col210 = eval.next_trace_mask();
        let res_limb_14_col211 = eval.next_trace_mask();
        let res_limb_15_col212 = eval.next_trace_mask();
        let res_limb_16_col213 = eval.next_trace_mask();
        let res_limb_17_col214 = eval.next_trace_mask();
        let res_limb_18_col215 = eval.next_trace_mask();
        let res_limb_19_col216 = eval.next_trace_mask();
        let res_limb_20_col217 = eval.next_trace_mask();
        let res_limb_21_col218 = eval.next_trace_mask();
        let res_limb_22_col219 = eval.next_trace_mask();
        let res_limb_23_col220 = eval.next_trace_mask();
        let res_limb_24_col221 = eval.next_trace_mask();
        let res_limb_25_col222 = eval.next_trace_mask();
        let res_limb_26_col223 = eval.next_trace_mask();
        let res_limb_27_col224 = eval.next_trace_mask();
        let partial_limb_msb_col225 = eval.next_trace_mask();
        let partial_limb_msb_col226 = eval.next_trace_mask();
        let partial_limb_msb_col227 = eval.next_trace_mask();
        let partial_limb_msb_col228 = eval.next_trace_mask();
        let msb_col229 = eval.next_trace_mask();
        let mid_limbs_set_col230 = eval.next_trace_mask();
        let partial_limb_msb_col231 = eval.next_trace_mask();
        let dst_sum_squares_inv_col232 = eval.next_trace_mask();
        let dst_sum_inv_col233 = eval.next_trace_mask();
        let op1_as_rel_imm_cond_col234 = eval.next_trace_mask();
        let msb_col235 = eval.next_trace_mask();
        let mid_limbs_set_col236 = eval.next_trace_mask();
        let partial_limb_msb_col237 = eval.next_trace_mask();
        let next_pc_jnz_col238 = eval.next_trace_mask();
        let next_pc_col239 = eval.next_trace_mask();
        let next_ap_col240 = eval.next_trace_mask();
        let range_check_ap_bot11bits_col241 = eval.next_trace_mask();
        let next_fp_col242 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_generic_instruction_output_tmp_57455_26_op1_base_op0, decode_generic_instruction_output_tmp_57455_26_res_op1, decode_generic_instruction_output_tmp_57455_26_pc_update_regular, decode_generic_instruction_output_tmp_57455_26_fp_update_regular, decode_generic_instruction_output_tmp_57455_26_instruction_size, decode_generic_instruction_output_tmp_57455_26_offset0, decode_generic_instruction_output_tmp_57455_26_offset1, decode_generic_instruction_output_tmp_57455_26_offset2] =
            DecodeGenericInstruction::evaluate(
                [input_pc_col0.clone()],
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
                pc_update_jnz_col15.clone(),
                decode_generic_instruction_output_tmp_57455_26_op1_base_op0.clone(),
                decode_generic_instruction_output_tmp_57455_26_res_op1.clone(),
                decode_generic_instruction_output_tmp_57455_26_offset0.clone(),
                decode_generic_instruction_output_tmp_57455_26_offset1.clone(),
                decode_generic_instruction_output_tmp_57455_26_offset2.clone(),
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
            partial_limb_msb_col81.clone(),
            op1_src_col82.clone(),
            op1_id_col83.clone(),
            op1_limb_0_col84.clone(),
            op1_limb_1_col85.clone(),
            op1_limb_2_col86.clone(),
            op1_limb_3_col87.clone(),
            op1_limb_4_col88.clone(),
            op1_limb_5_col89.clone(),
            op1_limb_6_col90.clone(),
            op1_limb_7_col91.clone(),
            op1_limb_8_col92.clone(),
            op1_limb_9_col93.clone(),
            op1_limb_10_col94.clone(),
            op1_limb_11_col95.clone(),
            op1_limb_12_col96.clone(),
            op1_limb_13_col97.clone(),
            op1_limb_14_col98.clone(),
            op1_limb_15_col99.clone(),
            op1_limb_16_col100.clone(),
            op1_limb_17_col101.clone(),
            op1_limb_18_col102.clone(),
            op1_limb_19_col103.clone(),
            op1_limb_20_col104.clone(),
            op1_limb_21_col105.clone(),
            op1_limb_22_col106.clone(),
            op1_limb_23_col107.clone(),
            op1_limb_24_col108.clone(),
            op1_limb_25_col109.clone(),
            op1_limb_26_col110.clone(),
            op1_limb_27_col111.clone(),
            add_res_limb_0_col112.clone(),
            add_res_limb_1_col113.clone(),
            add_res_limb_2_col114.clone(),
            add_res_limb_3_col115.clone(),
            add_res_limb_4_col116.clone(),
            add_res_limb_5_col117.clone(),
            add_res_limb_6_col118.clone(),
            add_res_limb_7_col119.clone(),
            add_res_limb_8_col120.clone(),
            add_res_limb_9_col121.clone(),
            add_res_limb_10_col122.clone(),
            add_res_limb_11_col123.clone(),
            add_res_limb_12_col124.clone(),
            add_res_limb_13_col125.clone(),
            add_res_limb_14_col126.clone(),
            add_res_limb_15_col127.clone(),
            add_res_limb_16_col128.clone(),
            add_res_limb_17_col129.clone(),
            add_res_limb_18_col130.clone(),
            add_res_limb_19_col131.clone(),
            add_res_limb_20_col132.clone(),
            add_res_limb_21_col133.clone(),
            add_res_limb_22_col134.clone(),
            add_res_limb_23_col135.clone(),
            add_res_limb_24_col136.clone(),
            add_res_limb_25_col137.clone(),
            add_res_limb_26_col138.clone(),
            add_res_limb_27_col139.clone(),
            sub_p_bit_col140.clone(),
            mul_res_limb_0_col141.clone(),
            mul_res_limb_1_col142.clone(),
            mul_res_limb_2_col143.clone(),
            mul_res_limb_3_col144.clone(),
            mul_res_limb_4_col145.clone(),
            mul_res_limb_5_col146.clone(),
            mul_res_limb_6_col147.clone(),
            mul_res_limb_7_col148.clone(),
            mul_res_limb_8_col149.clone(),
            mul_res_limb_9_col150.clone(),
            mul_res_limb_10_col151.clone(),
            mul_res_limb_11_col152.clone(),
            mul_res_limb_12_col153.clone(),
            mul_res_limb_13_col154.clone(),
            mul_res_limb_14_col155.clone(),
            mul_res_limb_15_col156.clone(),
            mul_res_limb_16_col157.clone(),
            mul_res_limb_17_col158.clone(),
            mul_res_limb_18_col159.clone(),
            mul_res_limb_19_col160.clone(),
            mul_res_limb_20_col161.clone(),
            mul_res_limb_21_col162.clone(),
            mul_res_limb_22_col163.clone(),
            mul_res_limb_23_col164.clone(),
            mul_res_limb_24_col165.clone(),
            mul_res_limb_25_col166.clone(),
            mul_res_limb_26_col167.clone(),
            mul_res_limb_27_col168.clone(),
            k_col169.clone(),
            carry_0_col170.clone(),
            carry_1_col171.clone(),
            carry_2_col172.clone(),
            carry_3_col173.clone(),
            carry_4_col174.clone(),
            carry_5_col175.clone(),
            carry_6_col176.clone(),
            carry_7_col177.clone(),
            carry_8_col178.clone(),
            carry_9_col179.clone(),
            carry_10_col180.clone(),
            carry_11_col181.clone(),
            carry_12_col182.clone(),
            carry_13_col183.clone(),
            carry_14_col184.clone(),
            carry_15_col185.clone(),
            carry_16_col186.clone(),
            carry_17_col187.clone(),
            carry_18_col188.clone(),
            carry_19_col189.clone(),
            carry_20_col190.clone(),
            carry_21_col191.clone(),
            carry_22_col192.clone(),
            carry_23_col193.clone(),
            carry_24_col194.clone(),
            carry_25_col195.clone(),
            carry_26_col196.clone(),
            res_limb_0_col197.clone(),
            res_limb_1_col198.clone(),
            res_limb_2_col199.clone(),
            res_limb_3_col200.clone(),
            res_limb_4_col201.clone(),
            res_limb_5_col202.clone(),
            res_limb_6_col203.clone(),
            res_limb_7_col204.clone(),
            res_limb_8_col205.clone(),
            res_limb_9_col206.clone(),
            res_limb_10_col207.clone(),
            res_limb_11_col208.clone(),
            res_limb_12_col209.clone(),
            res_limb_13_col210.clone(),
            res_limb_14_col211.clone(),
            res_limb_15_col212.clone(),
            res_limb_16_col213.clone(),
            res_limb_17_col214.clone(),
            res_limb_18_col215.clone(),
            res_limb_19_col216.clone(),
            res_limb_20_col217.clone(),
            res_limb_21_col218.clone(),
            res_limb_22_col219.clone(),
            res_limb_23_col220.clone(),
            res_limb_24_col221.clone(),
            res_limb_25_col222.clone(),
            res_limb_26_col223.clone(),
            res_limb_27_col224.clone(),
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &self.range_check_9_9_lookup_elements,
            &self.range_check_9_9_b_lookup_elements,
            &self.range_check_9_9_c_lookup_elements,
            &self.range_check_9_9_d_lookup_elements,
            &self.range_check_9_9_e_lookup_elements,
            &self.range_check_9_9_f_lookup_elements,
            &self.range_check_9_9_g_lookup_elements,
            &self.range_check_9_9_h_lookup_elements,
            &self.range_check_20_lookup_elements,
            &self.range_check_20_b_lookup_elements,
            &self.range_check_20_c_lookup_elements,
            &self.range_check_20_d_lookup_elements,
            &self.range_check_20_e_lookup_elements,
            &self.range_check_20_f_lookup_elements,
            &self.range_check_20_g_lookup_elements,
            &self.range_check_20_h_lookup_elements,
            &mut eval,
        );
        HandleOpcodes::evaluate(
            [
                input_pc_col0.clone(),
                input_fp_col2.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_base_fp_col9.clone(),
                pc_update_jump_col13.clone(),
                opcode_call_col18.clone(),
                opcode_ret_col19.clone(),
                opcode_assert_eq_col20.clone(),
                decode_generic_instruction_output_tmp_57455_26_res_op1.clone(),
                decode_generic_instruction_output_tmp_57455_26_instruction_size.clone(),
                decode_generic_instruction_output_tmp_57455_26_offset0.clone(),
                decode_generic_instruction_output_tmp_57455_26_offset1.clone(),
                decode_generic_instruction_output_tmp_57455_26_offset2.clone(),
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
                res_limb_0_col197.clone(),
                res_limb_1_col198.clone(),
                res_limb_2_col199.clone(),
                res_limb_3_col200.clone(),
                res_limb_4_col201.clone(),
                res_limb_5_col202.clone(),
                res_limb_6_col203.clone(),
                res_limb_7_col204.clone(),
                res_limb_8_col205.clone(),
                res_limb_9_col206.clone(),
                res_limb_10_col207.clone(),
                res_limb_11_col208.clone(),
                res_limb_12_col209.clone(),
                res_limb_13_col210.clone(),
                res_limb_14_col211.clone(),
                res_limb_15_col212.clone(),
                res_limb_16_col213.clone(),
                res_limb_17_col214.clone(),
                res_limb_18_col215.clone(),
                res_limb_19_col216.clone(),
                res_limb_20_col217.clone(),
                res_limb_21_col218.clone(),
                res_limb_22_col219.clone(),
                res_limb_23_col220.clone(),
                res_limb_24_col221.clone(),
                res_limb_25_col222.clone(),
                res_limb_26_col223.clone(),
                res_limb_27_col224.clone(),
            ],
            partial_limb_msb_col225.clone(),
            partial_limb_msb_col226.clone(),
            &mut eval,
        );
        UpdateRegisters::evaluate(
            [
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
                pc_update_jump_col13.clone(),
                pc_update_jump_rel_col14.clone(),
                pc_update_jnz_col15.clone(),
                ap_update_add_col16.clone(),
                ap_update_add_1_col17.clone(),
                opcode_call_col18.clone(),
                opcode_ret_col19.clone(),
                decode_generic_instruction_output_tmp_57455_26_pc_update_regular.clone(),
                decode_generic_instruction_output_tmp_57455_26_fp_update_regular.clone(),
                decode_generic_instruction_output_tmp_57455_26_instruction_size.clone(),
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
                op1_limb_0_col84.clone(),
                op1_limb_1_col85.clone(),
                op1_limb_2_col86.clone(),
                op1_limb_3_col87.clone(),
                op1_limb_4_col88.clone(),
                op1_limb_5_col89.clone(),
                op1_limb_6_col90.clone(),
                op1_limb_7_col91.clone(),
                op1_limb_8_col92.clone(),
                op1_limb_9_col93.clone(),
                op1_limb_10_col94.clone(),
                op1_limb_11_col95.clone(),
                op1_limb_12_col96.clone(),
                op1_limb_13_col97.clone(),
                op1_limb_14_col98.clone(),
                op1_limb_15_col99.clone(),
                op1_limb_16_col100.clone(),
                op1_limb_17_col101.clone(),
                op1_limb_18_col102.clone(),
                op1_limb_19_col103.clone(),
                op1_limb_20_col104.clone(),
                op1_limb_21_col105.clone(),
                op1_limb_22_col106.clone(),
                op1_limb_23_col107.clone(),
                op1_limb_24_col108.clone(),
                op1_limb_25_col109.clone(),
                op1_limb_26_col110.clone(),
                op1_limb_27_col111.clone(),
                res_limb_0_col197.clone(),
                res_limb_1_col198.clone(),
                res_limb_2_col199.clone(),
                res_limb_3_col200.clone(),
                res_limb_4_col201.clone(),
                res_limb_5_col202.clone(),
                res_limb_6_col203.clone(),
                res_limb_7_col204.clone(),
                res_limb_8_col205.clone(),
                res_limb_9_col206.clone(),
                res_limb_10_col207.clone(),
                res_limb_11_col208.clone(),
                res_limb_12_col209.clone(),
                res_limb_13_col210.clone(),
                res_limb_14_col211.clone(),
                res_limb_15_col212.clone(),
                res_limb_16_col213.clone(),
                res_limb_17_col214.clone(),
                res_limb_18_col215.clone(),
                res_limb_19_col216.clone(),
                res_limb_20_col217.clone(),
                res_limb_21_col218.clone(),
                res_limb_22_col219.clone(),
                res_limb_23_col220.clone(),
                res_limb_24_col221.clone(),
                res_limb_25_col222.clone(),
                res_limb_26_col223.clone(),
                res_limb_27_col224.clone(),
            ],
            partial_limb_msb_col227.clone(),
            partial_limb_msb_col228.clone(),
            msb_col229.clone(),
            mid_limbs_set_col230.clone(),
            partial_limb_msb_col231.clone(),
            dst_sum_squares_inv_col232.clone(),
            dst_sum_inv_col233.clone(),
            op1_as_rel_imm_cond_col234.clone(),
            msb_col235.clone(),
            mid_limbs_set_col236.clone(),
            partial_limb_msb_col237.clone(),
            next_pc_jnz_col238.clone(),
            next_pc_col239.clone(),
            next_ap_col240.clone(),
            range_check_ap_bot11bits_col241.clone(),
            next_fp_col242.clone(),
            &self.range_check_18_lookup_elements,
            &self.range_check_11_lookup_elements,
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
                next_pc_col239.clone(),
                next_ap_col240.clone(),
                next_fp_col242.clone(),
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
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

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
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F::dummy(),
            range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G::dummy(),
            range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H::dummy(),
            range_check_20_lookup_elements: relations::RangeCheck_20::dummy(),
            range_check_20_b_lookup_elements: relations::RangeCheck_20_B::dummy(),
            range_check_20_c_lookup_elements: relations::RangeCheck_20_C::dummy(),
            range_check_20_d_lookup_elements: relations::RangeCheck_20_D::dummy(),
            range_check_20_e_lookup_elements: relations::RangeCheck_20_E::dummy(),
            range_check_20_f_lookup_elements: relations::RangeCheck_20_F::dummy(),
            range_check_20_g_lookup_elements: relations::RangeCheck_20_G::dummy(),
            range_check_20_h_lookup_elements: relations::RangeCheck_20_H::dummy(),
            range_check_18_lookup_elements: relations::RangeCheck_18::dummy(),
            range_check_11_lookup_elements: relations::RangeCheck_11::dummy(),
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

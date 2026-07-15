// This file was created by the AIR team.

use subroutines::decode_generic_instruction::DecodeGenericInstruction;
use subroutines::eval_operands::EvalOperands;
use subroutines::handle_opcodes::HandleOpcodes;
use subroutines::update_registers::UpdateRegisters;

use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 243;
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
    pub common_lookup_elements: relations::CommonLookupElements,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 34];
        TreeVec::new(vec![trace_log_sizes, interaction_log_sizes])
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
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
        let M31_428564188 = E::F::from(M31::from(428564188));
        let enabler_col0 = eval.next_trace_mask();
        let input_pc_col1 = eval.next_trace_mask();
        let input_ap_col2 = eval.next_trace_mask();
        let input_fp_col3 = eval.next_trace_mask();
        let offset0_col4 = eval.next_trace_mask();
        let offset1_col5 = eval.next_trace_mask();
        let offset2_col6 = eval.next_trace_mask();
        let dst_base_fp_col7 = eval.next_trace_mask();
        let op0_base_fp_col8 = eval.next_trace_mask();
        let op1_imm_col9 = eval.next_trace_mask();
        let op1_base_fp_col10 = eval.next_trace_mask();
        let op1_base_ap_col11 = eval.next_trace_mask();
        let res_add_col12 = eval.next_trace_mask();
        let res_mul_col13 = eval.next_trace_mask();
        let pc_update_jump_col14 = eval.next_trace_mask();
        let pc_update_jump_rel_col15 = eval.next_trace_mask();
        let pc_update_jnz_col16 = eval.next_trace_mask();
        let ap_update_add_col17 = eval.next_trace_mask();
        let ap_update_add_1_col18 = eval.next_trace_mask();
        let opcode_call_col19 = eval.next_trace_mask();
        let opcode_ret_col20 = eval.next_trace_mask();
        let opcode_assert_eq_col21 = eval.next_trace_mask();
        let dst_src_col22 = eval.next_trace_mask();
        let dst_id_col23 = eval.next_trace_mask();
        let dst_limb_0_col24 = eval.next_trace_mask();
        let dst_limb_1_col25 = eval.next_trace_mask();
        let dst_limb_2_col26 = eval.next_trace_mask();
        let dst_limb_3_col27 = eval.next_trace_mask();
        let dst_limb_4_col28 = eval.next_trace_mask();
        let dst_limb_5_col29 = eval.next_trace_mask();
        let dst_limb_6_col30 = eval.next_trace_mask();
        let dst_limb_7_col31 = eval.next_trace_mask();
        let dst_limb_8_col32 = eval.next_trace_mask();
        let dst_limb_9_col33 = eval.next_trace_mask();
        let dst_limb_10_col34 = eval.next_trace_mask();
        let dst_limb_11_col35 = eval.next_trace_mask();
        let dst_limb_12_col36 = eval.next_trace_mask();
        let dst_limb_13_col37 = eval.next_trace_mask();
        let dst_limb_14_col38 = eval.next_trace_mask();
        let dst_limb_15_col39 = eval.next_trace_mask();
        let dst_limb_16_col40 = eval.next_trace_mask();
        let dst_limb_17_col41 = eval.next_trace_mask();
        let dst_limb_18_col42 = eval.next_trace_mask();
        let dst_limb_19_col43 = eval.next_trace_mask();
        let dst_limb_20_col44 = eval.next_trace_mask();
        let dst_limb_21_col45 = eval.next_trace_mask();
        let dst_limb_22_col46 = eval.next_trace_mask();
        let dst_limb_23_col47 = eval.next_trace_mask();
        let dst_limb_24_col48 = eval.next_trace_mask();
        let dst_limb_25_col49 = eval.next_trace_mask();
        let dst_limb_26_col50 = eval.next_trace_mask();
        let dst_limb_27_col51 = eval.next_trace_mask();
        let op0_src_col52 = eval.next_trace_mask();
        let op0_id_col53 = eval.next_trace_mask();
        let op0_limb_0_col54 = eval.next_trace_mask();
        let op0_limb_1_col55 = eval.next_trace_mask();
        let op0_limb_2_col56 = eval.next_trace_mask();
        let op0_limb_3_col57 = eval.next_trace_mask();
        let op0_limb_4_col58 = eval.next_trace_mask();
        let op0_limb_5_col59 = eval.next_trace_mask();
        let op0_limb_6_col60 = eval.next_trace_mask();
        let op0_limb_7_col61 = eval.next_trace_mask();
        let op0_limb_8_col62 = eval.next_trace_mask();
        let op0_limb_9_col63 = eval.next_trace_mask();
        let op0_limb_10_col64 = eval.next_trace_mask();
        let op0_limb_11_col65 = eval.next_trace_mask();
        let op0_limb_12_col66 = eval.next_trace_mask();
        let op0_limb_13_col67 = eval.next_trace_mask();
        let op0_limb_14_col68 = eval.next_trace_mask();
        let op0_limb_15_col69 = eval.next_trace_mask();
        let op0_limb_16_col70 = eval.next_trace_mask();
        let op0_limb_17_col71 = eval.next_trace_mask();
        let op0_limb_18_col72 = eval.next_trace_mask();
        let op0_limb_19_col73 = eval.next_trace_mask();
        let op0_limb_20_col74 = eval.next_trace_mask();
        let op0_limb_21_col75 = eval.next_trace_mask();
        let op0_limb_22_col76 = eval.next_trace_mask();
        let op0_limb_23_col77 = eval.next_trace_mask();
        let op0_limb_24_col78 = eval.next_trace_mask();
        let op0_limb_25_col79 = eval.next_trace_mask();
        let op0_limb_26_col80 = eval.next_trace_mask();
        let op0_limb_27_col81 = eval.next_trace_mask();
        let partial_limb_msb_col82 = eval.next_trace_mask();
        let op1_src_col83 = eval.next_trace_mask();
        let op1_id_col84 = eval.next_trace_mask();
        let op1_limb_0_col85 = eval.next_trace_mask();
        let op1_limb_1_col86 = eval.next_trace_mask();
        let op1_limb_2_col87 = eval.next_trace_mask();
        let op1_limb_3_col88 = eval.next_trace_mask();
        let op1_limb_4_col89 = eval.next_trace_mask();
        let op1_limb_5_col90 = eval.next_trace_mask();
        let op1_limb_6_col91 = eval.next_trace_mask();
        let op1_limb_7_col92 = eval.next_trace_mask();
        let op1_limb_8_col93 = eval.next_trace_mask();
        let op1_limb_9_col94 = eval.next_trace_mask();
        let op1_limb_10_col95 = eval.next_trace_mask();
        let op1_limb_11_col96 = eval.next_trace_mask();
        let op1_limb_12_col97 = eval.next_trace_mask();
        let op1_limb_13_col98 = eval.next_trace_mask();
        let op1_limb_14_col99 = eval.next_trace_mask();
        let op1_limb_15_col100 = eval.next_trace_mask();
        let op1_limb_16_col101 = eval.next_trace_mask();
        let op1_limb_17_col102 = eval.next_trace_mask();
        let op1_limb_18_col103 = eval.next_trace_mask();
        let op1_limb_19_col104 = eval.next_trace_mask();
        let op1_limb_20_col105 = eval.next_trace_mask();
        let op1_limb_21_col106 = eval.next_trace_mask();
        let op1_limb_22_col107 = eval.next_trace_mask();
        let op1_limb_23_col108 = eval.next_trace_mask();
        let op1_limb_24_col109 = eval.next_trace_mask();
        let op1_limb_25_col110 = eval.next_trace_mask();
        let op1_limb_26_col111 = eval.next_trace_mask();
        let op1_limb_27_col112 = eval.next_trace_mask();
        let add_res_limb_0_col113 = eval.next_trace_mask();
        let add_res_limb_1_col114 = eval.next_trace_mask();
        let add_res_limb_2_col115 = eval.next_trace_mask();
        let add_res_limb_3_col116 = eval.next_trace_mask();
        let add_res_limb_4_col117 = eval.next_trace_mask();
        let add_res_limb_5_col118 = eval.next_trace_mask();
        let add_res_limb_6_col119 = eval.next_trace_mask();
        let add_res_limb_7_col120 = eval.next_trace_mask();
        let add_res_limb_8_col121 = eval.next_trace_mask();
        let add_res_limb_9_col122 = eval.next_trace_mask();
        let add_res_limb_10_col123 = eval.next_trace_mask();
        let add_res_limb_11_col124 = eval.next_trace_mask();
        let add_res_limb_12_col125 = eval.next_trace_mask();
        let add_res_limb_13_col126 = eval.next_trace_mask();
        let add_res_limb_14_col127 = eval.next_trace_mask();
        let add_res_limb_15_col128 = eval.next_trace_mask();
        let add_res_limb_16_col129 = eval.next_trace_mask();
        let add_res_limb_17_col130 = eval.next_trace_mask();
        let add_res_limb_18_col131 = eval.next_trace_mask();
        let add_res_limb_19_col132 = eval.next_trace_mask();
        let add_res_limb_20_col133 = eval.next_trace_mask();
        let add_res_limb_21_col134 = eval.next_trace_mask();
        let add_res_limb_22_col135 = eval.next_trace_mask();
        let add_res_limb_23_col136 = eval.next_trace_mask();
        let add_res_limb_24_col137 = eval.next_trace_mask();
        let add_res_limb_25_col138 = eval.next_trace_mask();
        let add_res_limb_26_col139 = eval.next_trace_mask();
        let add_res_limb_27_col140 = eval.next_trace_mask();
        let sub_p_bit_col141 = eval.next_trace_mask();
        let mul_res_limb_0_col142 = eval.next_trace_mask();
        let mul_res_limb_1_col143 = eval.next_trace_mask();
        let mul_res_limb_2_col144 = eval.next_trace_mask();
        let mul_res_limb_3_col145 = eval.next_trace_mask();
        let mul_res_limb_4_col146 = eval.next_trace_mask();
        let mul_res_limb_5_col147 = eval.next_trace_mask();
        let mul_res_limb_6_col148 = eval.next_trace_mask();
        let mul_res_limb_7_col149 = eval.next_trace_mask();
        let mul_res_limb_8_col150 = eval.next_trace_mask();
        let mul_res_limb_9_col151 = eval.next_trace_mask();
        let mul_res_limb_10_col152 = eval.next_trace_mask();
        let mul_res_limb_11_col153 = eval.next_trace_mask();
        let mul_res_limb_12_col154 = eval.next_trace_mask();
        let mul_res_limb_13_col155 = eval.next_trace_mask();
        let mul_res_limb_14_col156 = eval.next_trace_mask();
        let mul_res_limb_15_col157 = eval.next_trace_mask();
        let mul_res_limb_16_col158 = eval.next_trace_mask();
        let mul_res_limb_17_col159 = eval.next_trace_mask();
        let mul_res_limb_18_col160 = eval.next_trace_mask();
        let mul_res_limb_19_col161 = eval.next_trace_mask();
        let mul_res_limb_20_col162 = eval.next_trace_mask();
        let mul_res_limb_21_col163 = eval.next_trace_mask();
        let mul_res_limb_22_col164 = eval.next_trace_mask();
        let mul_res_limb_23_col165 = eval.next_trace_mask();
        let mul_res_limb_24_col166 = eval.next_trace_mask();
        let mul_res_limb_25_col167 = eval.next_trace_mask();
        let mul_res_limb_26_col168 = eval.next_trace_mask();
        let mul_res_limb_27_col169 = eval.next_trace_mask();
        let k_col170 = eval.next_trace_mask();
        let carry_0_col171 = eval.next_trace_mask();
        let carry_1_col172 = eval.next_trace_mask();
        let carry_2_col173 = eval.next_trace_mask();
        let carry_3_col174 = eval.next_trace_mask();
        let carry_4_col175 = eval.next_trace_mask();
        let carry_5_col176 = eval.next_trace_mask();
        let carry_6_col177 = eval.next_trace_mask();
        let carry_7_col178 = eval.next_trace_mask();
        let carry_8_col179 = eval.next_trace_mask();
        let carry_9_col180 = eval.next_trace_mask();
        let carry_10_col181 = eval.next_trace_mask();
        let carry_11_col182 = eval.next_trace_mask();
        let carry_12_col183 = eval.next_trace_mask();
        let carry_13_col184 = eval.next_trace_mask();
        let carry_14_col185 = eval.next_trace_mask();
        let carry_15_col186 = eval.next_trace_mask();
        let carry_16_col187 = eval.next_trace_mask();
        let carry_17_col188 = eval.next_trace_mask();
        let carry_18_col189 = eval.next_trace_mask();
        let carry_19_col190 = eval.next_trace_mask();
        let carry_20_col191 = eval.next_trace_mask();
        let carry_21_col192 = eval.next_trace_mask();
        let carry_22_col193 = eval.next_trace_mask();
        let carry_23_col194 = eval.next_trace_mask();
        let carry_24_col195 = eval.next_trace_mask();
        let carry_25_col196 = eval.next_trace_mask();
        let carry_26_col197 = eval.next_trace_mask();
        let res_limb_0_col198 = eval.next_trace_mask();
        let res_limb_1_col199 = eval.next_trace_mask();
        let res_limb_2_col200 = eval.next_trace_mask();
        let res_limb_3_col201 = eval.next_trace_mask();
        let res_limb_4_col202 = eval.next_trace_mask();
        let res_limb_5_col203 = eval.next_trace_mask();
        let res_limb_6_col204 = eval.next_trace_mask();
        let res_limb_7_col205 = eval.next_trace_mask();
        let res_limb_8_col206 = eval.next_trace_mask();
        let res_limb_9_col207 = eval.next_trace_mask();
        let res_limb_10_col208 = eval.next_trace_mask();
        let res_limb_11_col209 = eval.next_trace_mask();
        let res_limb_12_col210 = eval.next_trace_mask();
        let res_limb_13_col211 = eval.next_trace_mask();
        let res_limb_14_col212 = eval.next_trace_mask();
        let res_limb_15_col213 = eval.next_trace_mask();
        let res_limb_16_col214 = eval.next_trace_mask();
        let res_limb_17_col215 = eval.next_trace_mask();
        let res_limb_18_col216 = eval.next_trace_mask();
        let res_limb_19_col217 = eval.next_trace_mask();
        let res_limb_20_col218 = eval.next_trace_mask();
        let res_limb_21_col219 = eval.next_trace_mask();
        let res_limb_22_col220 = eval.next_trace_mask();
        let res_limb_23_col221 = eval.next_trace_mask();
        let res_limb_24_col222 = eval.next_trace_mask();
        let res_limb_25_col223 = eval.next_trace_mask();
        let res_limb_26_col224 = eval.next_trace_mask();
        let res_limb_27_col225 = eval.next_trace_mask();
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
        let range_check_29_bot11bits_col241 = eval.next_trace_mask();
        let next_fp_col242 = eval.next_trace_mask();

        // Enabler is a bit.
        eval.add_constraint(((enabler_col0.clone() * enabler_col0.clone()) - enabler_col0.clone()));
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [decode_generic_instruction_output_tmp_c8168_26_op1_base_op0, decode_generic_instruction_output_tmp_c8168_26_res_op1, decode_generic_instruction_output_tmp_c8168_26_pc_update_regular, decode_generic_instruction_output_tmp_c8168_26_fp_update_regular, decode_generic_instruction_output_tmp_c8168_26_instruction_size, decode_generic_instruction_output_tmp_c8168_26_offset0, decode_generic_instruction_output_tmp_c8168_26_offset1, decode_generic_instruction_output_tmp_c8168_26_offset2] =
            DecodeGenericInstruction::evaluate(
                [input_pc_col1.clone()],
                enabler_col0.clone(),
                offset0_col4.clone(),
                offset1_col5.clone(),
                offset2_col6.clone(),
                dst_base_fp_col7.clone(),
                op0_base_fp_col8.clone(),
                op1_imm_col9.clone(),
                op1_base_fp_col10.clone(),
                op1_base_ap_col11.clone(),
                res_add_col12.clone(),
                res_mul_col13.clone(),
                pc_update_jump_col14.clone(),
                pc_update_jump_rel_col15.clone(),
                pc_update_jnz_col16.clone(),
                ap_update_add_col17.clone(),
                ap_update_add_1_col18.clone(),
                opcode_call_col19.clone(),
                opcode_ret_col20.clone(),
                opcode_assert_eq_col21.clone(),
                &self.common_lookup_elements,
                &mut eval,
            );
        EvalOperands::evaluate(
            [
                input_pc_col1.clone(),
                input_ap_col2.clone(),
                input_fp_col3.clone(),
                dst_base_fp_col7.clone(),
                op0_base_fp_col8.clone(),
                op1_imm_col9.clone(),
                op1_base_fp_col10.clone(),
                op1_base_ap_col11.clone(),
                res_add_col12.clone(),
                res_mul_col13.clone(),
                pc_update_jnz_col16.clone(),
                decode_generic_instruction_output_tmp_c8168_26_op1_base_op0.clone(),
                decode_generic_instruction_output_tmp_c8168_26_res_op1.clone(),
                decode_generic_instruction_output_tmp_c8168_26_offset0.clone(),
                decode_generic_instruction_output_tmp_c8168_26_offset1.clone(),
                decode_generic_instruction_output_tmp_c8168_26_offset2.clone(),
            ],
            enabler_col0.clone(),
            dst_src_col22.clone(),
            dst_id_col23.clone(),
            dst_limb_0_col24.clone(),
            dst_limb_1_col25.clone(),
            dst_limb_2_col26.clone(),
            dst_limb_3_col27.clone(),
            dst_limb_4_col28.clone(),
            dst_limb_5_col29.clone(),
            dst_limb_6_col30.clone(),
            dst_limb_7_col31.clone(),
            dst_limb_8_col32.clone(),
            dst_limb_9_col33.clone(),
            dst_limb_10_col34.clone(),
            dst_limb_11_col35.clone(),
            dst_limb_12_col36.clone(),
            dst_limb_13_col37.clone(),
            dst_limb_14_col38.clone(),
            dst_limb_15_col39.clone(),
            dst_limb_16_col40.clone(),
            dst_limb_17_col41.clone(),
            dst_limb_18_col42.clone(),
            dst_limb_19_col43.clone(),
            dst_limb_20_col44.clone(),
            dst_limb_21_col45.clone(),
            dst_limb_22_col46.clone(),
            dst_limb_23_col47.clone(),
            dst_limb_24_col48.clone(),
            dst_limb_25_col49.clone(),
            dst_limb_26_col50.clone(),
            dst_limb_27_col51.clone(),
            op0_src_col52.clone(),
            op0_id_col53.clone(),
            op0_limb_0_col54.clone(),
            op0_limb_1_col55.clone(),
            op0_limb_2_col56.clone(),
            op0_limb_3_col57.clone(),
            op0_limb_4_col58.clone(),
            op0_limb_5_col59.clone(),
            op0_limb_6_col60.clone(),
            op0_limb_7_col61.clone(),
            op0_limb_8_col62.clone(),
            op0_limb_9_col63.clone(),
            op0_limb_10_col64.clone(),
            op0_limb_11_col65.clone(),
            op0_limb_12_col66.clone(),
            op0_limb_13_col67.clone(),
            op0_limb_14_col68.clone(),
            op0_limb_15_col69.clone(),
            op0_limb_16_col70.clone(),
            op0_limb_17_col71.clone(),
            op0_limb_18_col72.clone(),
            op0_limb_19_col73.clone(),
            op0_limb_20_col74.clone(),
            op0_limb_21_col75.clone(),
            op0_limb_22_col76.clone(),
            op0_limb_23_col77.clone(),
            op0_limb_24_col78.clone(),
            op0_limb_25_col79.clone(),
            op0_limb_26_col80.clone(),
            op0_limb_27_col81.clone(),
            partial_limb_msb_col82.clone(),
            op1_src_col83.clone(),
            op1_id_col84.clone(),
            op1_limb_0_col85.clone(),
            op1_limb_1_col86.clone(),
            op1_limb_2_col87.clone(),
            op1_limb_3_col88.clone(),
            op1_limb_4_col89.clone(),
            op1_limb_5_col90.clone(),
            op1_limb_6_col91.clone(),
            op1_limb_7_col92.clone(),
            op1_limb_8_col93.clone(),
            op1_limb_9_col94.clone(),
            op1_limb_10_col95.clone(),
            op1_limb_11_col96.clone(),
            op1_limb_12_col97.clone(),
            op1_limb_13_col98.clone(),
            op1_limb_14_col99.clone(),
            op1_limb_15_col100.clone(),
            op1_limb_16_col101.clone(),
            op1_limb_17_col102.clone(),
            op1_limb_18_col103.clone(),
            op1_limb_19_col104.clone(),
            op1_limb_20_col105.clone(),
            op1_limb_21_col106.clone(),
            op1_limb_22_col107.clone(),
            op1_limb_23_col108.clone(),
            op1_limb_24_col109.clone(),
            op1_limb_25_col110.clone(),
            op1_limb_26_col111.clone(),
            op1_limb_27_col112.clone(),
            add_res_limb_0_col113.clone(),
            add_res_limb_1_col114.clone(),
            add_res_limb_2_col115.clone(),
            add_res_limb_3_col116.clone(),
            add_res_limb_4_col117.clone(),
            add_res_limb_5_col118.clone(),
            add_res_limb_6_col119.clone(),
            add_res_limb_7_col120.clone(),
            add_res_limb_8_col121.clone(),
            add_res_limb_9_col122.clone(),
            add_res_limb_10_col123.clone(),
            add_res_limb_11_col124.clone(),
            add_res_limb_12_col125.clone(),
            add_res_limb_13_col126.clone(),
            add_res_limb_14_col127.clone(),
            add_res_limb_15_col128.clone(),
            add_res_limb_16_col129.clone(),
            add_res_limb_17_col130.clone(),
            add_res_limb_18_col131.clone(),
            add_res_limb_19_col132.clone(),
            add_res_limb_20_col133.clone(),
            add_res_limb_21_col134.clone(),
            add_res_limb_22_col135.clone(),
            add_res_limb_23_col136.clone(),
            add_res_limb_24_col137.clone(),
            add_res_limb_25_col138.clone(),
            add_res_limb_26_col139.clone(),
            add_res_limb_27_col140.clone(),
            sub_p_bit_col141.clone(),
            mul_res_limb_0_col142.clone(),
            mul_res_limb_1_col143.clone(),
            mul_res_limb_2_col144.clone(),
            mul_res_limb_3_col145.clone(),
            mul_res_limb_4_col146.clone(),
            mul_res_limb_5_col147.clone(),
            mul_res_limb_6_col148.clone(),
            mul_res_limb_7_col149.clone(),
            mul_res_limb_8_col150.clone(),
            mul_res_limb_9_col151.clone(),
            mul_res_limb_10_col152.clone(),
            mul_res_limb_11_col153.clone(),
            mul_res_limb_12_col154.clone(),
            mul_res_limb_13_col155.clone(),
            mul_res_limb_14_col156.clone(),
            mul_res_limb_15_col157.clone(),
            mul_res_limb_16_col158.clone(),
            mul_res_limb_17_col159.clone(),
            mul_res_limb_18_col160.clone(),
            mul_res_limb_19_col161.clone(),
            mul_res_limb_20_col162.clone(),
            mul_res_limb_21_col163.clone(),
            mul_res_limb_22_col164.clone(),
            mul_res_limb_23_col165.clone(),
            mul_res_limb_24_col166.clone(),
            mul_res_limb_25_col167.clone(),
            mul_res_limb_26_col168.clone(),
            mul_res_limb_27_col169.clone(),
            k_col170.clone(),
            carry_0_col171.clone(),
            carry_1_col172.clone(),
            carry_2_col173.clone(),
            carry_3_col174.clone(),
            carry_4_col175.clone(),
            carry_5_col176.clone(),
            carry_6_col177.clone(),
            carry_7_col178.clone(),
            carry_8_col179.clone(),
            carry_9_col180.clone(),
            carry_10_col181.clone(),
            carry_11_col182.clone(),
            carry_12_col183.clone(),
            carry_13_col184.clone(),
            carry_14_col185.clone(),
            carry_15_col186.clone(),
            carry_16_col187.clone(),
            carry_17_col188.clone(),
            carry_18_col189.clone(),
            carry_19_col190.clone(),
            carry_20_col191.clone(),
            carry_21_col192.clone(),
            carry_22_col193.clone(),
            carry_23_col194.clone(),
            carry_24_col195.clone(),
            carry_25_col196.clone(),
            carry_26_col197.clone(),
            res_limb_0_col198.clone(),
            res_limb_1_col199.clone(),
            res_limb_2_col200.clone(),
            res_limb_3_col201.clone(),
            res_limb_4_col202.clone(),
            res_limb_5_col203.clone(),
            res_limb_6_col204.clone(),
            res_limb_7_col205.clone(),
            res_limb_8_col206.clone(),
            res_limb_9_col207.clone(),
            res_limb_10_col208.clone(),
            res_limb_11_col209.clone(),
            res_limb_12_col210.clone(),
            res_limb_13_col211.clone(),
            res_limb_14_col212.clone(),
            res_limb_15_col213.clone(),
            res_limb_16_col214.clone(),
            res_limb_17_col215.clone(),
            res_limb_18_col216.clone(),
            res_limb_19_col217.clone(),
            res_limb_20_col218.clone(),
            res_limb_21_col219.clone(),
            res_limb_22_col220.clone(),
            res_limb_23_col221.clone(),
            res_limb_24_col222.clone(),
            res_limb_25_col223.clone(),
            res_limb_26_col224.clone(),
            res_limb_27_col225.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        HandleOpcodes::evaluate(
            [
                input_pc_col1.clone(),
                input_fp_col3.clone(),
                dst_base_fp_col7.clone(),
                op0_base_fp_col8.clone(),
                op1_base_fp_col10.clone(),
                pc_update_jump_col14.clone(),
                opcode_call_col19.clone(),
                opcode_ret_col20.clone(),
                opcode_assert_eq_col21.clone(),
                decode_generic_instruction_output_tmp_c8168_26_res_op1.clone(),
                decode_generic_instruction_output_tmp_c8168_26_instruction_size.clone(),
                decode_generic_instruction_output_tmp_c8168_26_offset0.clone(),
                decode_generic_instruction_output_tmp_c8168_26_offset1.clone(),
                decode_generic_instruction_output_tmp_c8168_26_offset2.clone(),
                dst_limb_0_col24.clone(),
                dst_limb_1_col25.clone(),
                dst_limb_2_col26.clone(),
                dst_limb_3_col27.clone(),
                dst_limb_4_col28.clone(),
                dst_limb_5_col29.clone(),
                dst_limb_6_col30.clone(),
                dst_limb_7_col31.clone(),
                dst_limb_8_col32.clone(),
                dst_limb_9_col33.clone(),
                dst_limb_10_col34.clone(),
                dst_limb_11_col35.clone(),
                dst_limb_12_col36.clone(),
                dst_limb_13_col37.clone(),
                dst_limb_14_col38.clone(),
                dst_limb_15_col39.clone(),
                dst_limb_16_col40.clone(),
                dst_limb_17_col41.clone(),
                dst_limb_18_col42.clone(),
                dst_limb_19_col43.clone(),
                dst_limb_20_col44.clone(),
                dst_limb_21_col45.clone(),
                dst_limb_22_col46.clone(),
                dst_limb_23_col47.clone(),
                dst_limb_24_col48.clone(),
                dst_limb_25_col49.clone(),
                dst_limb_26_col50.clone(),
                dst_limb_27_col51.clone(),
                op0_limb_0_col54.clone(),
                op0_limb_1_col55.clone(),
                op0_limb_2_col56.clone(),
                op0_limb_3_col57.clone(),
                op0_limb_4_col58.clone(),
                op0_limb_5_col59.clone(),
                op0_limb_6_col60.clone(),
                op0_limb_7_col61.clone(),
                op0_limb_8_col62.clone(),
                op0_limb_9_col63.clone(),
                op0_limb_10_col64.clone(),
                op0_limb_11_col65.clone(),
                op0_limb_12_col66.clone(),
                op0_limb_13_col67.clone(),
                op0_limb_14_col68.clone(),
                op0_limb_15_col69.clone(),
                op0_limb_16_col70.clone(),
                op0_limb_17_col71.clone(),
                op0_limb_18_col72.clone(),
                op0_limb_19_col73.clone(),
                op0_limb_20_col74.clone(),
                op0_limb_21_col75.clone(),
                op0_limb_22_col76.clone(),
                op0_limb_23_col77.clone(),
                op0_limb_24_col78.clone(),
                op0_limb_25_col79.clone(),
                op0_limb_26_col80.clone(),
                op0_limb_27_col81.clone(),
                res_limb_0_col198.clone(),
                res_limb_1_col199.clone(),
                res_limb_2_col200.clone(),
                res_limb_3_col201.clone(),
                res_limb_4_col202.clone(),
                res_limb_5_col203.clone(),
                res_limb_6_col204.clone(),
                res_limb_7_col205.clone(),
                res_limb_8_col206.clone(),
                res_limb_9_col207.clone(),
                res_limb_10_col208.clone(),
                res_limb_11_col209.clone(),
                res_limb_12_col210.clone(),
                res_limb_13_col211.clone(),
                res_limb_14_col212.clone(),
                res_limb_15_col213.clone(),
                res_limb_16_col214.clone(),
                res_limb_17_col215.clone(),
                res_limb_18_col216.clone(),
                res_limb_19_col217.clone(),
                res_limb_20_col218.clone(),
                res_limb_21_col219.clone(),
                res_limb_22_col220.clone(),
                res_limb_23_col221.clone(),
                res_limb_24_col222.clone(),
                res_limb_25_col223.clone(),
                res_limb_26_col224.clone(),
                res_limb_27_col225.clone(),
            ],
            enabler_col0.clone(),
            partial_limb_msb_col226.clone(),
            partial_limb_msb_col227.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        UpdateRegisters::evaluate(
            [
                input_pc_col1.clone(),
                input_ap_col2.clone(),
                input_fp_col3.clone(),
                pc_update_jump_col14.clone(),
                pc_update_jump_rel_col15.clone(),
                pc_update_jnz_col16.clone(),
                ap_update_add_col17.clone(),
                ap_update_add_1_col18.clone(),
                opcode_call_col19.clone(),
                opcode_ret_col20.clone(),
                decode_generic_instruction_output_tmp_c8168_26_pc_update_regular.clone(),
                decode_generic_instruction_output_tmp_c8168_26_fp_update_regular.clone(),
                decode_generic_instruction_output_tmp_c8168_26_instruction_size.clone(),
                dst_limb_0_col24.clone(),
                dst_limb_1_col25.clone(),
                dst_limb_2_col26.clone(),
                dst_limb_3_col27.clone(),
                dst_limb_4_col28.clone(),
                dst_limb_5_col29.clone(),
                dst_limb_6_col30.clone(),
                dst_limb_7_col31.clone(),
                dst_limb_8_col32.clone(),
                dst_limb_9_col33.clone(),
                dst_limb_10_col34.clone(),
                dst_limb_11_col35.clone(),
                dst_limb_12_col36.clone(),
                dst_limb_13_col37.clone(),
                dst_limb_14_col38.clone(),
                dst_limb_15_col39.clone(),
                dst_limb_16_col40.clone(),
                dst_limb_17_col41.clone(),
                dst_limb_18_col42.clone(),
                dst_limb_19_col43.clone(),
                dst_limb_20_col44.clone(),
                dst_limb_21_col45.clone(),
                dst_limb_22_col46.clone(),
                dst_limb_23_col47.clone(),
                dst_limb_24_col48.clone(),
                dst_limb_25_col49.clone(),
                dst_limb_26_col50.clone(),
                dst_limb_27_col51.clone(),
                op1_limb_0_col85.clone(),
                op1_limb_1_col86.clone(),
                op1_limb_2_col87.clone(),
                op1_limb_3_col88.clone(),
                op1_limb_4_col89.clone(),
                op1_limb_5_col90.clone(),
                op1_limb_6_col91.clone(),
                op1_limb_7_col92.clone(),
                op1_limb_8_col93.clone(),
                op1_limb_9_col94.clone(),
                op1_limb_10_col95.clone(),
                op1_limb_11_col96.clone(),
                op1_limb_12_col97.clone(),
                op1_limb_13_col98.clone(),
                op1_limb_14_col99.clone(),
                op1_limb_15_col100.clone(),
                op1_limb_16_col101.clone(),
                op1_limb_17_col102.clone(),
                op1_limb_18_col103.clone(),
                op1_limb_19_col104.clone(),
                op1_limb_20_col105.clone(),
                op1_limb_21_col106.clone(),
                op1_limb_22_col107.clone(),
                op1_limb_23_col108.clone(),
                op1_limb_24_col109.clone(),
                op1_limb_25_col110.clone(),
                op1_limb_26_col111.clone(),
                op1_limb_27_col112.clone(),
                res_limb_0_col198.clone(),
                res_limb_1_col199.clone(),
                res_limb_2_col200.clone(),
                res_limb_3_col201.clone(),
                res_limb_4_col202.clone(),
                res_limb_5_col203.clone(),
                res_limb_6_col204.clone(),
                res_limb_7_col205.clone(),
                res_limb_8_col206.clone(),
                res_limb_9_col207.clone(),
                res_limb_10_col208.clone(),
                res_limb_11_col209.clone(),
                res_limb_12_col210.clone(),
                res_limb_13_col211.clone(),
                res_limb_14_col212.clone(),
                res_limb_15_col213.clone(),
                res_limb_16_col214.clone(),
                res_limb_17_col215.clone(),
                res_limb_18_col216.clone(),
                res_limb_19_col217.clone(),
                res_limb_20_col218.clone(),
                res_limb_21_col219.clone(),
                res_limb_22_col220.clone(),
                res_limb_23_col221.clone(),
                res_limb_24_col222.clone(),
                res_limb_25_col223.clone(),
                res_limb_26_col224.clone(),
                res_limb_27_col225.clone(),
            ],
            enabler_col0.clone(),
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
            range_check_29_bot11bits_col241.clone(),
            next_fp_col242.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(enabler_col0.clone()),
            &[
                M31_428564188.clone(),
                input_pc_col1.clone(),
                input_ap_col2.clone(),
                input_fp_col3.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(enabler_col0.clone()),
            &[
                M31_428564188.clone(),
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

    #[test]
    fn generic_opcode_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.random::<QM31>();
        }

        constraints_regression_test_values::GENERIC_OPCODE.assert_debug_eq(&sum);
    }
}

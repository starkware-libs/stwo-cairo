// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::add_252::Add252;
use crate::components::subroutines::cond_felt_252_as_addr::CondFelt252AsAddr;
use crate::components::subroutines::mul_252::Mul252;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct EvalOperands {}

impl EvalOperands {
    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    #[allow(clippy::unused_unit)]
    #[allow(unused_variables)]
    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<E: EvalAtRow>(
        [eval_operands_input_pc, eval_operands_input_ap, eval_operands_input_fp, eval_operands_input_dst_base_fp, eval_operands_input_op0_base_fp, eval_operands_input_op1_imm, eval_operands_input_op1_base_fp, eval_operands_input_op1_base_ap, eval_operands_input_res_add, eval_operands_input_res_mul, eval_operands_input_pc_update_jnz, eval_operands_input_op1_base_op0, eval_operands_input_res_op1, eval_operands_input_offset0, eval_operands_input_offset1, eval_operands_input_offset2]: [E::F; 16],
        dst_src_col0: E::F,
        dst_id_col1: E::F,
        dst_limb_0_col2: E::F,
        dst_limb_1_col3: E::F,
        dst_limb_2_col4: E::F,
        dst_limb_3_col5: E::F,
        dst_limb_4_col6: E::F,
        dst_limb_5_col7: E::F,
        dst_limb_6_col8: E::F,
        dst_limb_7_col9: E::F,
        dst_limb_8_col10: E::F,
        dst_limb_9_col11: E::F,
        dst_limb_10_col12: E::F,
        dst_limb_11_col13: E::F,
        dst_limb_12_col14: E::F,
        dst_limb_13_col15: E::F,
        dst_limb_14_col16: E::F,
        dst_limb_15_col17: E::F,
        dst_limb_16_col18: E::F,
        dst_limb_17_col19: E::F,
        dst_limb_18_col20: E::F,
        dst_limb_19_col21: E::F,
        dst_limb_20_col22: E::F,
        dst_limb_21_col23: E::F,
        dst_limb_22_col24: E::F,
        dst_limb_23_col25: E::F,
        dst_limb_24_col26: E::F,
        dst_limb_25_col27: E::F,
        dst_limb_26_col28: E::F,
        dst_limb_27_col29: E::F,
        op0_src_col30: E::F,
        op0_id_col31: E::F,
        op0_limb_0_col32: E::F,
        op0_limb_1_col33: E::F,
        op0_limb_2_col34: E::F,
        op0_limb_3_col35: E::F,
        op0_limb_4_col36: E::F,
        op0_limb_5_col37: E::F,
        op0_limb_6_col38: E::F,
        op0_limb_7_col39: E::F,
        op0_limb_8_col40: E::F,
        op0_limb_9_col41: E::F,
        op0_limb_10_col42: E::F,
        op0_limb_11_col43: E::F,
        op0_limb_12_col44: E::F,
        op0_limb_13_col45: E::F,
        op0_limb_14_col46: E::F,
        op0_limb_15_col47: E::F,
        op0_limb_16_col48: E::F,
        op0_limb_17_col49: E::F,
        op0_limb_18_col50: E::F,
        op0_limb_19_col51: E::F,
        op0_limb_20_col52: E::F,
        op0_limb_21_col53: E::F,
        op0_limb_22_col54: E::F,
        op0_limb_23_col55: E::F,
        op0_limb_24_col56: E::F,
        op0_limb_25_col57: E::F,
        op0_limb_26_col58: E::F,
        op0_limb_27_col59: E::F,
        partial_limb_msb_col60: E::F,
        op1_src_col61: E::F,
        op1_id_col62: E::F,
        op1_limb_0_col63: E::F,
        op1_limb_1_col64: E::F,
        op1_limb_2_col65: E::F,
        op1_limb_3_col66: E::F,
        op1_limb_4_col67: E::F,
        op1_limb_5_col68: E::F,
        op1_limb_6_col69: E::F,
        op1_limb_7_col70: E::F,
        op1_limb_8_col71: E::F,
        op1_limb_9_col72: E::F,
        op1_limb_10_col73: E::F,
        op1_limb_11_col74: E::F,
        op1_limb_12_col75: E::F,
        op1_limb_13_col76: E::F,
        op1_limb_14_col77: E::F,
        op1_limb_15_col78: E::F,
        op1_limb_16_col79: E::F,
        op1_limb_17_col80: E::F,
        op1_limb_18_col81: E::F,
        op1_limb_19_col82: E::F,
        op1_limb_20_col83: E::F,
        op1_limb_21_col84: E::F,
        op1_limb_22_col85: E::F,
        op1_limb_23_col86: E::F,
        op1_limb_24_col87: E::F,
        op1_limb_25_col88: E::F,
        op1_limb_26_col89: E::F,
        op1_limb_27_col90: E::F,
        add_res_limb_0_col91: E::F,
        add_res_limb_1_col92: E::F,
        add_res_limb_2_col93: E::F,
        add_res_limb_3_col94: E::F,
        add_res_limb_4_col95: E::F,
        add_res_limb_5_col96: E::F,
        add_res_limb_6_col97: E::F,
        add_res_limb_7_col98: E::F,
        add_res_limb_8_col99: E::F,
        add_res_limb_9_col100: E::F,
        add_res_limb_10_col101: E::F,
        add_res_limb_11_col102: E::F,
        add_res_limb_12_col103: E::F,
        add_res_limb_13_col104: E::F,
        add_res_limb_14_col105: E::F,
        add_res_limb_15_col106: E::F,
        add_res_limb_16_col107: E::F,
        add_res_limb_17_col108: E::F,
        add_res_limb_18_col109: E::F,
        add_res_limb_19_col110: E::F,
        add_res_limb_20_col111: E::F,
        add_res_limb_21_col112: E::F,
        add_res_limb_22_col113: E::F,
        add_res_limb_23_col114: E::F,
        add_res_limb_24_col115: E::F,
        add_res_limb_25_col116: E::F,
        add_res_limb_26_col117: E::F,
        add_res_limb_27_col118: E::F,
        sub_p_bit_col119: E::F,
        mul_res_limb_0_col120: E::F,
        mul_res_limb_1_col121: E::F,
        mul_res_limb_2_col122: E::F,
        mul_res_limb_3_col123: E::F,
        mul_res_limb_4_col124: E::F,
        mul_res_limb_5_col125: E::F,
        mul_res_limb_6_col126: E::F,
        mul_res_limb_7_col127: E::F,
        mul_res_limb_8_col128: E::F,
        mul_res_limb_9_col129: E::F,
        mul_res_limb_10_col130: E::F,
        mul_res_limb_11_col131: E::F,
        mul_res_limb_12_col132: E::F,
        mul_res_limb_13_col133: E::F,
        mul_res_limb_14_col134: E::F,
        mul_res_limb_15_col135: E::F,
        mul_res_limb_16_col136: E::F,
        mul_res_limb_17_col137: E::F,
        mul_res_limb_18_col138: E::F,
        mul_res_limb_19_col139: E::F,
        mul_res_limb_20_col140: E::F,
        mul_res_limb_21_col141: E::F,
        mul_res_limb_22_col142: E::F,
        mul_res_limb_23_col143: E::F,
        mul_res_limb_24_col144: E::F,
        mul_res_limb_25_col145: E::F,
        mul_res_limb_26_col146: E::F,
        mul_res_limb_27_col147: E::F,
        k_col148: E::F,
        carry_0_col149: E::F,
        carry_1_col150: E::F,
        carry_2_col151: E::F,
        carry_3_col152: E::F,
        carry_4_col153: E::F,
        carry_5_col154: E::F,
        carry_6_col155: E::F,
        carry_7_col156: E::F,
        carry_8_col157: E::F,
        carry_9_col158: E::F,
        carry_10_col159: E::F,
        carry_11_col160: E::F,
        carry_12_col161: E::F,
        carry_13_col162: E::F,
        carry_14_col163: E::F,
        carry_15_col164: E::F,
        carry_16_col165: E::F,
        carry_17_col166: E::F,
        carry_18_col167: E::F,
        carry_19_col168: E::F,
        carry_20_col169: E::F,
        carry_21_col170: E::F,
        carry_22_col171: E::F,
        carry_23_col172: E::F,
        carry_24_col173: E::F,
        carry_25_col174: E::F,
        carry_26_col175: E::F,
        res_limb_0_col176: E::F,
        res_limb_1_col177: E::F,
        res_limb_2_col178: E::F,
        res_limb_3_col179: E::F,
        res_limb_4_col180: E::F,
        res_limb_5_col181: E::F,
        res_limb_6_col182: E::F,
        res_limb_7_col183: E::F,
        res_limb_8_col184: E::F,
        res_limb_9_col185: E::F,
        res_limb_10_col186: E::F,
        res_limb_11_col187: E::F,
        res_limb_12_col188: E::F,
        res_limb_13_col189: E::F,
        res_limb_14_col190: E::F,
        res_limb_15_col191: E::F,
        res_limb_16_col192: E::F,
        res_limb_17_col193: E::F,
        res_limb_18_col194: E::F,
        res_limb_19_col195: E::F,
        res_limb_20_col196: E::F,
        res_limb_21_col197: E::F,
        res_limb_22_col198: E::F,
        res_limb_23_col199: E::F,
        res_limb_24_col200: E::F,
        res_limb_25_col201: E::F,
        res_limb_26_col202: E::F,
        res_limb_27_col203: E::F,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        range_check_9_9_lookup_elements: &relations::RangeCheck_9_9,
        range_check_9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range_check_9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range_check_9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
        range_check_9_9_e_lookup_elements: &relations::RangeCheck_9_9_E,
        range_check_9_9_f_lookup_elements: &relations::RangeCheck_9_9_F,
        range_check_9_9_g_lookup_elements: &relations::RangeCheck_9_9_G,
        range_check_9_9_h_lookup_elements: &relations::RangeCheck_9_9_H,
        range_check_20_lookup_elements: &relations::RangeCheck_20,
        range_check_20_b_lookup_elements: &relations::RangeCheck_20_B,
        range_check_20_c_lookup_elements: &relations::RangeCheck_20_C,
        range_check_20_d_lookup_elements: &relations::RangeCheck_20_D,
        range_check_20_e_lookup_elements: &relations::RangeCheck_20_E,
        range_check_20_f_lookup_elements: &relations::RangeCheck_20_F,
        range_check_20_g_lookup_elements: &relations::RangeCheck_20_G,
        range_check_20_h_lookup_elements: &relations::RangeCheck_20_H,
        eval: &mut E,
    ) -> [E::F; 0] {
        let M31_1 = E::F::from(M31::from(1));

        // dst_src.
        eval.add_constraint(
            (dst_src_col0.clone()
                - ((eval_operands_input_dst_base_fp.clone() * eval_operands_input_fp.clone())
                    + ((M31_1.clone() - eval_operands_input_dst_base_fp.clone())
                        * eval_operands_input_ap.clone()))),
        );
        ReadPositiveNumBits252::evaluate(
            [(dst_src_col0.clone() + eval_operands_input_offset0.clone())],
            dst_id_col1.clone(),
            dst_limb_0_col2.clone(),
            dst_limb_1_col3.clone(),
            dst_limb_2_col4.clone(),
            dst_limb_3_col5.clone(),
            dst_limb_4_col6.clone(),
            dst_limb_5_col7.clone(),
            dst_limb_6_col8.clone(),
            dst_limb_7_col9.clone(),
            dst_limb_8_col10.clone(),
            dst_limb_9_col11.clone(),
            dst_limb_10_col12.clone(),
            dst_limb_11_col13.clone(),
            dst_limb_12_col14.clone(),
            dst_limb_13_col15.clone(),
            dst_limb_14_col16.clone(),
            dst_limb_15_col17.clone(),
            dst_limb_16_col18.clone(),
            dst_limb_17_col19.clone(),
            dst_limb_18_col20.clone(),
            dst_limb_19_col21.clone(),
            dst_limb_20_col22.clone(),
            dst_limb_21_col23.clone(),
            dst_limb_22_col24.clone(),
            dst_limb_23_col25.clone(),
            dst_limb_24_col26.clone(),
            dst_limb_25_col27.clone(),
            dst_limb_26_col28.clone(),
            dst_limb_27_col29.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        // op0_src.
        eval.add_constraint(
            (op0_src_col30.clone()
                - ((eval_operands_input_op0_base_fp.clone() * eval_operands_input_fp.clone())
                    + ((M31_1.clone() - eval_operands_input_op0_base_fp.clone())
                        * eval_operands_input_ap.clone()))),
        );
        ReadPositiveNumBits252::evaluate(
            [(op0_src_col30.clone() + eval_operands_input_offset1.clone())],
            op0_id_col31.clone(),
            op0_limb_0_col32.clone(),
            op0_limb_1_col33.clone(),
            op0_limb_2_col34.clone(),
            op0_limb_3_col35.clone(),
            op0_limb_4_col36.clone(),
            op0_limb_5_col37.clone(),
            op0_limb_6_col38.clone(),
            op0_limb_7_col39.clone(),
            op0_limb_8_col40.clone(),
            op0_limb_9_col41.clone(),
            op0_limb_10_col42.clone(),
            op0_limb_11_col43.clone(),
            op0_limb_12_col44.clone(),
            op0_limb_13_col45.clone(),
            op0_limb_14_col46.clone(),
            op0_limb_15_col47.clone(),
            op0_limb_16_col48.clone(),
            op0_limb_17_col49.clone(),
            op0_limb_18_col50.clone(),
            op0_limb_19_col51.clone(),
            op0_limb_20_col52.clone(),
            op0_limb_21_col53.clone(),
            op0_limb_22_col54.clone(),
            op0_limb_23_col55.clone(),
            op0_limb_24_col56.clone(),
            op0_limb_25_col57.clone(),
            op0_limb_26_col58.clone(),
            op0_limb_27_col59.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        let [cond_felt_252_as_addr_output_tmp_3172c_12] = CondFelt252AsAddr::evaluate(
            [
                op0_limb_0_col32.clone(),
                op0_limb_1_col33.clone(),
                op0_limb_2_col34.clone(),
                op0_limb_3_col35.clone(),
                op0_limb_4_col36.clone(),
                op0_limb_5_col37.clone(),
                op0_limb_6_col38.clone(),
                op0_limb_7_col39.clone(),
                op0_limb_8_col40.clone(),
                op0_limb_9_col41.clone(),
                op0_limb_10_col42.clone(),
                op0_limb_11_col43.clone(),
                op0_limb_12_col44.clone(),
                op0_limb_13_col45.clone(),
                op0_limb_14_col46.clone(),
                op0_limb_15_col47.clone(),
                op0_limb_16_col48.clone(),
                op0_limb_17_col49.clone(),
                op0_limb_18_col50.clone(),
                op0_limb_19_col51.clone(),
                op0_limb_20_col52.clone(),
                op0_limb_21_col53.clone(),
                op0_limb_22_col54.clone(),
                op0_limb_23_col55.clone(),
                op0_limb_24_col56.clone(),
                op0_limb_25_col57.clone(),
                op0_limb_26_col58.clone(),
                op0_limb_27_col59.clone(),
                eval_operands_input_op1_base_op0.clone(),
            ],
            partial_limb_msb_col60.clone(),
            eval,
        );
        // op1_src.
        eval.add_constraint(
            (op1_src_col61.clone()
                - ((((eval_operands_input_op1_base_fp.clone() * eval_operands_input_fp.clone())
                    + (eval_operands_input_op1_base_ap.clone()
                        * eval_operands_input_ap.clone()))
                    + (eval_operands_input_op1_imm.clone() * eval_operands_input_pc.clone()))
                    + (eval_operands_input_op1_base_op0.clone()
                        * cond_felt_252_as_addr_output_tmp_3172c_12.clone()))),
        );
        ReadPositiveNumBits252::evaluate(
            [(op1_src_col61.clone() + eval_operands_input_offset2.clone())],
            op1_id_col62.clone(),
            op1_limb_0_col63.clone(),
            op1_limb_1_col64.clone(),
            op1_limb_2_col65.clone(),
            op1_limb_3_col66.clone(),
            op1_limb_4_col67.clone(),
            op1_limb_5_col68.clone(),
            op1_limb_6_col69.clone(),
            op1_limb_7_col70.clone(),
            op1_limb_8_col71.clone(),
            op1_limb_9_col72.clone(),
            op1_limb_10_col73.clone(),
            op1_limb_11_col74.clone(),
            op1_limb_12_col75.clone(),
            op1_limb_13_col76.clone(),
            op1_limb_14_col77.clone(),
            op1_limb_15_col78.clone(),
            op1_limb_16_col79.clone(),
            op1_limb_17_col80.clone(),
            op1_limb_18_col81.clone(),
            op1_limb_19_col82.clone(),
            op1_limb_20_col83.clone(),
            op1_limb_21_col84.clone(),
            op1_limb_22_col85.clone(),
            op1_limb_23_col86.clone(),
            op1_limb_24_col87.clone(),
            op1_limb_25_col88.clone(),
            op1_limb_26_col89.clone(),
            op1_limb_27_col90.clone(),
            memory_address_to_id_lookup_elements,
            memory_id_to_big_lookup_elements,
            eval,
        );
        Add252::evaluate(
            [
                op0_limb_0_col32.clone(),
                op0_limb_1_col33.clone(),
                op0_limb_2_col34.clone(),
                op0_limb_3_col35.clone(),
                op0_limb_4_col36.clone(),
                op0_limb_5_col37.clone(),
                op0_limb_6_col38.clone(),
                op0_limb_7_col39.clone(),
                op0_limb_8_col40.clone(),
                op0_limb_9_col41.clone(),
                op0_limb_10_col42.clone(),
                op0_limb_11_col43.clone(),
                op0_limb_12_col44.clone(),
                op0_limb_13_col45.clone(),
                op0_limb_14_col46.clone(),
                op0_limb_15_col47.clone(),
                op0_limb_16_col48.clone(),
                op0_limb_17_col49.clone(),
                op0_limb_18_col50.clone(),
                op0_limb_19_col51.clone(),
                op0_limb_20_col52.clone(),
                op0_limb_21_col53.clone(),
                op0_limb_22_col54.clone(),
                op0_limb_23_col55.clone(),
                op0_limb_24_col56.clone(),
                op0_limb_25_col57.clone(),
                op0_limb_26_col58.clone(),
                op0_limb_27_col59.clone(),
                op1_limb_0_col63.clone(),
                op1_limb_1_col64.clone(),
                op1_limb_2_col65.clone(),
                op1_limb_3_col66.clone(),
                op1_limb_4_col67.clone(),
                op1_limb_5_col68.clone(),
                op1_limb_6_col69.clone(),
                op1_limb_7_col70.clone(),
                op1_limb_8_col71.clone(),
                op1_limb_9_col72.clone(),
                op1_limb_10_col73.clone(),
                op1_limb_11_col74.clone(),
                op1_limb_12_col75.clone(),
                op1_limb_13_col76.clone(),
                op1_limb_14_col77.clone(),
                op1_limb_15_col78.clone(),
                op1_limb_16_col79.clone(),
                op1_limb_17_col80.clone(),
                op1_limb_18_col81.clone(),
                op1_limb_19_col82.clone(),
                op1_limb_20_col83.clone(),
                op1_limb_21_col84.clone(),
                op1_limb_22_col85.clone(),
                op1_limb_23_col86.clone(),
                op1_limb_24_col87.clone(),
                op1_limb_25_col88.clone(),
                op1_limb_26_col89.clone(),
                op1_limb_27_col90.clone(),
            ],
            add_res_limb_0_col91.clone(),
            add_res_limb_1_col92.clone(),
            add_res_limb_2_col93.clone(),
            add_res_limb_3_col94.clone(),
            add_res_limb_4_col95.clone(),
            add_res_limb_5_col96.clone(),
            add_res_limb_6_col97.clone(),
            add_res_limb_7_col98.clone(),
            add_res_limb_8_col99.clone(),
            add_res_limb_9_col100.clone(),
            add_res_limb_10_col101.clone(),
            add_res_limb_11_col102.clone(),
            add_res_limb_12_col103.clone(),
            add_res_limb_13_col104.clone(),
            add_res_limb_14_col105.clone(),
            add_res_limb_15_col106.clone(),
            add_res_limb_16_col107.clone(),
            add_res_limb_17_col108.clone(),
            add_res_limb_18_col109.clone(),
            add_res_limb_19_col110.clone(),
            add_res_limb_20_col111.clone(),
            add_res_limb_21_col112.clone(),
            add_res_limb_22_col113.clone(),
            add_res_limb_23_col114.clone(),
            add_res_limb_24_col115.clone(),
            add_res_limb_25_col116.clone(),
            add_res_limb_26_col117.clone(),
            add_res_limb_27_col118.clone(),
            sub_p_bit_col119.clone(),
            range_check_9_9_lookup_elements,
            range_check_9_9_b_lookup_elements,
            range_check_9_9_c_lookup_elements,
            range_check_9_9_d_lookup_elements,
            range_check_9_9_e_lookup_elements,
            range_check_9_9_f_lookup_elements,
            range_check_9_9_g_lookup_elements,
            range_check_9_9_h_lookup_elements,
            eval,
        );
        Mul252::evaluate(
            [
                op0_limb_0_col32.clone(),
                op0_limb_1_col33.clone(),
                op0_limb_2_col34.clone(),
                op0_limb_3_col35.clone(),
                op0_limb_4_col36.clone(),
                op0_limb_5_col37.clone(),
                op0_limb_6_col38.clone(),
                op0_limb_7_col39.clone(),
                op0_limb_8_col40.clone(),
                op0_limb_9_col41.clone(),
                op0_limb_10_col42.clone(),
                op0_limb_11_col43.clone(),
                op0_limb_12_col44.clone(),
                op0_limb_13_col45.clone(),
                op0_limb_14_col46.clone(),
                op0_limb_15_col47.clone(),
                op0_limb_16_col48.clone(),
                op0_limb_17_col49.clone(),
                op0_limb_18_col50.clone(),
                op0_limb_19_col51.clone(),
                op0_limb_20_col52.clone(),
                op0_limb_21_col53.clone(),
                op0_limb_22_col54.clone(),
                op0_limb_23_col55.clone(),
                op0_limb_24_col56.clone(),
                op0_limb_25_col57.clone(),
                op0_limb_26_col58.clone(),
                op0_limb_27_col59.clone(),
                op1_limb_0_col63.clone(),
                op1_limb_1_col64.clone(),
                op1_limb_2_col65.clone(),
                op1_limb_3_col66.clone(),
                op1_limb_4_col67.clone(),
                op1_limb_5_col68.clone(),
                op1_limb_6_col69.clone(),
                op1_limb_7_col70.clone(),
                op1_limb_8_col71.clone(),
                op1_limb_9_col72.clone(),
                op1_limb_10_col73.clone(),
                op1_limb_11_col74.clone(),
                op1_limb_12_col75.clone(),
                op1_limb_13_col76.clone(),
                op1_limb_14_col77.clone(),
                op1_limb_15_col78.clone(),
                op1_limb_16_col79.clone(),
                op1_limb_17_col80.clone(),
                op1_limb_18_col81.clone(),
                op1_limb_19_col82.clone(),
                op1_limb_20_col83.clone(),
                op1_limb_21_col84.clone(),
                op1_limb_22_col85.clone(),
                op1_limb_23_col86.clone(),
                op1_limb_24_col87.clone(),
                op1_limb_25_col88.clone(),
                op1_limb_26_col89.clone(),
                op1_limb_27_col90.clone(),
            ],
            mul_res_limb_0_col120.clone(),
            mul_res_limb_1_col121.clone(),
            mul_res_limb_2_col122.clone(),
            mul_res_limb_3_col123.clone(),
            mul_res_limb_4_col124.clone(),
            mul_res_limb_5_col125.clone(),
            mul_res_limb_6_col126.clone(),
            mul_res_limb_7_col127.clone(),
            mul_res_limb_8_col128.clone(),
            mul_res_limb_9_col129.clone(),
            mul_res_limb_10_col130.clone(),
            mul_res_limb_11_col131.clone(),
            mul_res_limb_12_col132.clone(),
            mul_res_limb_13_col133.clone(),
            mul_res_limb_14_col134.clone(),
            mul_res_limb_15_col135.clone(),
            mul_res_limb_16_col136.clone(),
            mul_res_limb_17_col137.clone(),
            mul_res_limb_18_col138.clone(),
            mul_res_limb_19_col139.clone(),
            mul_res_limb_20_col140.clone(),
            mul_res_limb_21_col141.clone(),
            mul_res_limb_22_col142.clone(),
            mul_res_limb_23_col143.clone(),
            mul_res_limb_24_col144.clone(),
            mul_res_limb_25_col145.clone(),
            mul_res_limb_26_col146.clone(),
            mul_res_limb_27_col147.clone(),
            k_col148.clone(),
            carry_0_col149.clone(),
            carry_1_col150.clone(),
            carry_2_col151.clone(),
            carry_3_col152.clone(),
            carry_4_col153.clone(),
            carry_5_col154.clone(),
            carry_6_col155.clone(),
            carry_7_col156.clone(),
            carry_8_col157.clone(),
            carry_9_col158.clone(),
            carry_10_col159.clone(),
            carry_11_col160.clone(),
            carry_12_col161.clone(),
            carry_13_col162.clone(),
            carry_14_col163.clone(),
            carry_15_col164.clone(),
            carry_16_col165.clone(),
            carry_17_col166.clone(),
            carry_18_col167.clone(),
            carry_19_col168.clone(),
            carry_20_col169.clone(),
            carry_21_col170.clone(),
            carry_22_col171.clone(),
            carry_23_col172.clone(),
            carry_24_col173.clone(),
            carry_25_col174.clone(),
            carry_26_col175.clone(),
            range_check_9_9_lookup_elements,
            range_check_9_9_b_lookup_elements,
            range_check_9_9_c_lookup_elements,
            range_check_9_9_d_lookup_elements,
            range_check_9_9_e_lookup_elements,
            range_check_9_9_f_lookup_elements,
            range_check_9_9_g_lookup_elements,
            range_check_9_9_h_lookup_elements,
            range_check_20_lookup_elements,
            range_check_20_b_lookup_elements,
            range_check_20_c_lookup_elements,
            range_check_20_d_lookup_elements,
            range_check_20_e_lookup_elements,
            range_check_20_f_lookup_elements,
            range_check_20_g_lookup_elements,
            range_check_20_h_lookup_elements,
            eval,
        );
        let not_pc_update_jnz_tmp_3172c_54 =
            eval.add_intermediate((M31_1.clone() - eval_operands_input_pc_update_jnz.clone()));
        // constrain limb 0 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_0_col91.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_0_col120.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_0_col63.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_0_col176.clone())),
        );
        // constrain limb 1 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_1_col92.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_1_col121.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_1_col64.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_1_col177.clone())),
        );
        // constrain limb 2 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_2_col93.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_2_col122.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_2_col65.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_2_col178.clone())),
        );
        // constrain limb 3 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_3_col94.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_3_col123.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_3_col66.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_3_col179.clone())),
        );
        // constrain limb 4 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_4_col95.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_4_col124.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_4_col67.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_4_col180.clone())),
        );
        // constrain limb 5 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_5_col96.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_5_col125.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_5_col68.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_5_col181.clone())),
        );
        // constrain limb 6 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_6_col97.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_6_col126.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_6_col69.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_6_col182.clone())),
        );
        // constrain limb 7 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_7_col98.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_7_col127.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_7_col70.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_7_col183.clone())),
        );
        // constrain limb 8 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_8_col99.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_8_col128.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_8_col71.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_8_col184.clone())),
        );
        // constrain limb 9 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_9_col100.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_9_col129.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_9_col72.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_9_col185.clone())),
        );
        // constrain limb 10 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_10_col101.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_10_col130.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_10_col73.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_10_col186.clone())),
        );
        // constrain limb 11 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_11_col102.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_11_col131.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_11_col74.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_11_col187.clone())),
        );
        // constrain limb 12 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_12_col103.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_12_col132.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_12_col75.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_12_col188.clone())),
        );
        // constrain limb 13 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_13_col104.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_13_col133.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_13_col76.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_13_col189.clone())),
        );
        // constrain limb 14 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_14_col105.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_14_col134.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_14_col77.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_14_col190.clone())),
        );
        // constrain limb 15 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_15_col106.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_15_col135.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_15_col78.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_15_col191.clone())),
        );
        // constrain limb 16 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_16_col107.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_16_col136.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_16_col79.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_16_col192.clone())),
        );
        // constrain limb 17 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_17_col108.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_17_col137.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_17_col80.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_17_col193.clone())),
        );
        // constrain limb 18 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_18_col109.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_18_col138.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_18_col81.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_18_col194.clone())),
        );
        // constrain limb 19 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_19_col110.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_19_col139.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_19_col82.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_19_col195.clone())),
        );
        // constrain limb 20 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_20_col111.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_20_col140.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_20_col83.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_20_col196.clone())),
        );
        // constrain limb 21 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_21_col112.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_21_col141.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_21_col84.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_21_col197.clone())),
        );
        // constrain limb 22 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_22_col113.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_22_col142.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_22_col85.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_22_col198.clone())),
        );
        // constrain limb 23 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_23_col114.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_23_col143.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_23_col86.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_23_col199.clone())),
        );
        // constrain limb 24 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_24_col115.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_24_col144.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_24_col87.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_24_col200.clone())),
        );
        // constrain limb 25 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_25_col116.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_25_col145.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_25_col88.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_25_col201.clone())),
        );
        // constrain limb 26 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_26_col117.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_26_col146.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_26_col89.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_26_col202.clone())),
        );
        // constrain limb 27 of res.
        eval.add_constraint(
            ((((eval_operands_input_res_add.clone() * add_res_limb_27_col118.clone())
                + (eval_operands_input_res_mul.clone() * mul_res_limb_27_col147.clone()))
                + (eval_operands_input_res_op1.clone() * op1_limb_27_col90.clone()))
                - (not_pc_update_jnz_tmp_3172c_54.clone() * res_limb_27_col203.clone())),
        );
        []
    }
}

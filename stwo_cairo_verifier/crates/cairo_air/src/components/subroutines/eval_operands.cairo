// This file was created by the AIR team.

use crate::components::subroutines::add_252::add_252_evaluate;
use crate::components::subroutines::cond_felt_252_as_addr::cond_felt_252_as_addr_evaluate;
use crate::components::subroutines::mul_252::mul_252_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::prelude::*;


pub fn eval_operands_evaluate(
    input: [QM31; 16],
    dst_src_col0: QM31,
    dst_id_col1: QM31,
    dst_limb_0_col2: QM31,
    dst_limb_1_col3: QM31,
    dst_limb_2_col4: QM31,
    dst_limb_3_col5: QM31,
    dst_limb_4_col6: QM31,
    dst_limb_5_col7: QM31,
    dst_limb_6_col8: QM31,
    dst_limb_7_col9: QM31,
    dst_limb_8_col10: QM31,
    dst_limb_9_col11: QM31,
    dst_limb_10_col12: QM31,
    dst_limb_11_col13: QM31,
    dst_limb_12_col14: QM31,
    dst_limb_13_col15: QM31,
    dst_limb_14_col16: QM31,
    dst_limb_15_col17: QM31,
    dst_limb_16_col18: QM31,
    dst_limb_17_col19: QM31,
    dst_limb_18_col20: QM31,
    dst_limb_19_col21: QM31,
    dst_limb_20_col22: QM31,
    dst_limb_21_col23: QM31,
    dst_limb_22_col24: QM31,
    dst_limb_23_col25: QM31,
    dst_limb_24_col26: QM31,
    dst_limb_25_col27: QM31,
    dst_limb_26_col28: QM31,
    dst_limb_27_col29: QM31,
    op0_src_col30: QM31,
    op0_id_col31: QM31,
    op0_limb_0_col32: QM31,
    op0_limb_1_col33: QM31,
    op0_limb_2_col34: QM31,
    op0_limb_3_col35: QM31,
    op0_limb_4_col36: QM31,
    op0_limb_5_col37: QM31,
    op0_limb_6_col38: QM31,
    op0_limb_7_col39: QM31,
    op0_limb_8_col40: QM31,
    op0_limb_9_col41: QM31,
    op0_limb_10_col42: QM31,
    op0_limb_11_col43: QM31,
    op0_limb_12_col44: QM31,
    op0_limb_13_col45: QM31,
    op0_limb_14_col46: QM31,
    op0_limb_15_col47: QM31,
    op0_limb_16_col48: QM31,
    op0_limb_17_col49: QM31,
    op0_limb_18_col50: QM31,
    op0_limb_19_col51: QM31,
    op0_limb_20_col52: QM31,
    op0_limb_21_col53: QM31,
    op0_limb_22_col54: QM31,
    op0_limb_23_col55: QM31,
    op0_limb_24_col56: QM31,
    op0_limb_25_col57: QM31,
    op0_limb_26_col58: QM31,
    op0_limb_27_col59: QM31,
    partial_limb_msb_col60: QM31,
    op1_src_col61: QM31,
    op1_id_col62: QM31,
    op1_limb_0_col63: QM31,
    op1_limb_1_col64: QM31,
    op1_limb_2_col65: QM31,
    op1_limb_3_col66: QM31,
    op1_limb_4_col67: QM31,
    op1_limb_5_col68: QM31,
    op1_limb_6_col69: QM31,
    op1_limb_7_col70: QM31,
    op1_limb_8_col71: QM31,
    op1_limb_9_col72: QM31,
    op1_limb_10_col73: QM31,
    op1_limb_11_col74: QM31,
    op1_limb_12_col75: QM31,
    op1_limb_13_col76: QM31,
    op1_limb_14_col77: QM31,
    op1_limb_15_col78: QM31,
    op1_limb_16_col79: QM31,
    op1_limb_17_col80: QM31,
    op1_limb_18_col81: QM31,
    op1_limb_19_col82: QM31,
    op1_limb_20_col83: QM31,
    op1_limb_21_col84: QM31,
    op1_limb_22_col85: QM31,
    op1_limb_23_col86: QM31,
    op1_limb_24_col87: QM31,
    op1_limb_25_col88: QM31,
    op1_limb_26_col89: QM31,
    op1_limb_27_col90: QM31,
    add_res_limb_0_col91: QM31,
    add_res_limb_1_col92: QM31,
    add_res_limb_2_col93: QM31,
    add_res_limb_3_col94: QM31,
    add_res_limb_4_col95: QM31,
    add_res_limb_5_col96: QM31,
    add_res_limb_6_col97: QM31,
    add_res_limb_7_col98: QM31,
    add_res_limb_8_col99: QM31,
    add_res_limb_9_col100: QM31,
    add_res_limb_10_col101: QM31,
    add_res_limb_11_col102: QM31,
    add_res_limb_12_col103: QM31,
    add_res_limb_13_col104: QM31,
    add_res_limb_14_col105: QM31,
    add_res_limb_15_col106: QM31,
    add_res_limb_16_col107: QM31,
    add_res_limb_17_col108: QM31,
    add_res_limb_18_col109: QM31,
    add_res_limb_19_col110: QM31,
    add_res_limb_20_col111: QM31,
    add_res_limb_21_col112: QM31,
    add_res_limb_22_col113: QM31,
    add_res_limb_23_col114: QM31,
    add_res_limb_24_col115: QM31,
    add_res_limb_25_col116: QM31,
    add_res_limb_26_col117: QM31,
    add_res_limb_27_col118: QM31,
    sub_p_bit_col119: QM31,
    mul_res_limb_0_col120: QM31,
    mul_res_limb_1_col121: QM31,
    mul_res_limb_2_col122: QM31,
    mul_res_limb_3_col123: QM31,
    mul_res_limb_4_col124: QM31,
    mul_res_limb_5_col125: QM31,
    mul_res_limb_6_col126: QM31,
    mul_res_limb_7_col127: QM31,
    mul_res_limb_8_col128: QM31,
    mul_res_limb_9_col129: QM31,
    mul_res_limb_10_col130: QM31,
    mul_res_limb_11_col131: QM31,
    mul_res_limb_12_col132: QM31,
    mul_res_limb_13_col133: QM31,
    mul_res_limb_14_col134: QM31,
    mul_res_limb_15_col135: QM31,
    mul_res_limb_16_col136: QM31,
    mul_res_limb_17_col137: QM31,
    mul_res_limb_18_col138: QM31,
    mul_res_limb_19_col139: QM31,
    mul_res_limb_20_col140: QM31,
    mul_res_limb_21_col141: QM31,
    mul_res_limb_22_col142: QM31,
    mul_res_limb_23_col143: QM31,
    mul_res_limb_24_col144: QM31,
    mul_res_limb_25_col145: QM31,
    mul_res_limb_26_col146: QM31,
    mul_res_limb_27_col147: QM31,
    k_col148: QM31,
    carry_0_col149: QM31,
    carry_1_col150: QM31,
    carry_2_col151: QM31,
    carry_3_col152: QM31,
    carry_4_col153: QM31,
    carry_5_col154: QM31,
    carry_6_col155: QM31,
    carry_7_col156: QM31,
    carry_8_col157: QM31,
    carry_9_col158: QM31,
    carry_10_col159: QM31,
    carry_11_col160: QM31,
    carry_12_col161: QM31,
    carry_13_col162: QM31,
    carry_14_col163: QM31,
    carry_15_col164: QM31,
    carry_16_col165: QM31,
    carry_17_col166: QM31,
    carry_18_col167: QM31,
    carry_19_col168: QM31,
    carry_20_col169: QM31,
    carry_21_col170: QM31,
    carry_22_col171: QM31,
    carry_23_col172: QM31,
    carry_24_col173: QM31,
    carry_25_col174: QM31,
    carry_26_col175: QM31,
    res_limb_0_col176: QM31,
    res_limb_1_col177: QM31,
    res_limb_2_col178: QM31,
    res_limb_3_col179: QM31,
    res_limb_4_col180: QM31,
    res_limb_5_col181: QM31,
    res_limb_6_col182: QM31,
    res_limb_7_col183: QM31,
    res_limb_8_col184: QM31,
    res_limb_9_col185: QM31,
    res_limb_10_col186: QM31,
    res_limb_11_col187: QM31,
    res_limb_12_col188: QM31,
    res_limb_13_col189: QM31,
    res_limb_14_col190: QM31,
    res_limb_15_col191: QM31,
    res_limb_16_col192: QM31,
    res_limb_17_col193: QM31,
    res_limb_18_col194: QM31,
    res_limb_19_col195: QM31,
    res_limb_20_col196: QM31,
    res_limb_21_col197: QM31,
    res_limb_22_col198: QM31,
    res_limb_23_col199: QM31,
    res_limb_24_col200: QM31,
    res_limb_25_col201: QM31,
    res_limb_26_col202: QM31,
    res_limb_27_col203: QM31,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    range_check_9_9_lookup_elements: @crate::RangeCheck_9_9Elements,
    range_check_9_9_b_lookup_elements: @crate::RangeCheck_9_9_BElements,
    range_check_9_9_c_lookup_elements: @crate::RangeCheck_9_9_CElements,
    range_check_9_9_d_lookup_elements: @crate::RangeCheck_9_9_DElements,
    range_check_9_9_e_lookup_elements: @crate::RangeCheck_9_9_EElements,
    range_check_9_9_f_lookup_elements: @crate::RangeCheck_9_9_FElements,
    range_check_9_9_g_lookup_elements: @crate::RangeCheck_9_9_GElements,
    range_check_9_9_h_lookup_elements: @crate::RangeCheck_9_9_HElements,
    range_check_20_lookup_elements: @crate::RangeCheck_20Elements,
    range_check_20_b_lookup_elements: @crate::RangeCheck_20_BElements,
    range_check_20_c_lookup_elements: @crate::RangeCheck_20_CElements,
    range_check_20_d_lookup_elements: @crate::RangeCheck_20_DElements,
    range_check_20_e_lookup_elements: @crate::RangeCheck_20_EElements,
    range_check_20_f_lookup_elements: @crate::RangeCheck_20_FElements,
    range_check_20_g_lookup_elements: @crate::RangeCheck_20_GElements,
    range_check_20_h_lookup_elements: @crate::RangeCheck_20_HElements,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref memory_address_to_id_sum_2: QM31,
    ref memory_id_to_big_sum_3: QM31,
    ref memory_address_to_id_sum_4: QM31,
    ref memory_id_to_big_sum_5: QM31,
    ref range_check_9_9_sum_6: QM31,
    ref range_check_9_9_b_sum_7: QM31,
    ref range_check_9_9_c_sum_8: QM31,
    ref range_check_9_9_d_sum_9: QM31,
    ref range_check_9_9_e_sum_10: QM31,
    ref range_check_9_9_f_sum_11: QM31,
    ref range_check_9_9_g_sum_12: QM31,
    ref range_check_9_9_h_sum_13: QM31,
    ref range_check_9_9_sum_14: QM31,
    ref range_check_9_9_b_sum_15: QM31,
    ref range_check_9_9_c_sum_16: QM31,
    ref range_check_9_9_d_sum_17: QM31,
    ref range_check_9_9_e_sum_18: QM31,
    ref range_check_9_9_f_sum_19: QM31,
    ref range_check_9_9_sum_20: QM31,
    ref range_check_9_9_b_sum_21: QM31,
    ref range_check_9_9_c_sum_22: QM31,
    ref range_check_9_9_d_sum_23: QM31,
    ref range_check_9_9_e_sum_24: QM31,
    ref range_check_9_9_f_sum_25: QM31,
    ref range_check_9_9_g_sum_26: QM31,
    ref range_check_9_9_h_sum_27: QM31,
    ref range_check_9_9_sum_28: QM31,
    ref range_check_9_9_b_sum_29: QM31,
    ref range_check_9_9_c_sum_30: QM31,
    ref range_check_9_9_d_sum_31: QM31,
    ref range_check_9_9_e_sum_32: QM31,
    ref range_check_9_9_f_sum_33: QM31,
    ref range_check_20_sum_34: QM31,
    ref range_check_20_b_sum_35: QM31,
    ref range_check_20_c_sum_36: QM31,
    ref range_check_20_d_sum_37: QM31,
    ref range_check_20_e_sum_38: QM31,
    ref range_check_20_f_sum_39: QM31,
    ref range_check_20_g_sum_40: QM31,
    ref range_check_20_h_sum_41: QM31,
    ref range_check_20_sum_42: QM31,
    ref range_check_20_b_sum_43: QM31,
    ref range_check_20_c_sum_44: QM31,
    ref range_check_20_d_sum_45: QM31,
    ref range_check_20_e_sum_46: QM31,
    ref range_check_20_f_sum_47: QM31,
    ref range_check_20_g_sum_48: QM31,
    ref range_check_20_h_sum_49: QM31,
    ref range_check_20_sum_50: QM31,
    ref range_check_20_b_sum_51: QM31,
    ref range_check_20_c_sum_52: QM31,
    ref range_check_20_d_sum_53: QM31,
    ref range_check_20_e_sum_54: QM31,
    ref range_check_20_f_sum_55: QM31,
    ref range_check_20_g_sum_56: QM31,
    ref range_check_20_h_sum_57: QM31,
    ref range_check_20_sum_58: QM31,
    ref range_check_20_b_sum_59: QM31,
    ref range_check_20_c_sum_60: QM31,
    ref range_check_20_d_sum_61: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        eval_operands_input_pc,
        eval_operands_input_ap,
        eval_operands_input_fp,
        eval_operands_input_dst_base_fp,
        eval_operands_input_op0_base_fp,
        eval_operands_input_op1_imm,
        eval_operands_input_op1_base_fp,
        eval_operands_input_op1_base_ap,
        eval_operands_input_res_add,
        eval_operands_input_res_mul,
        eval_operands_input_pc_update_jnz,
        eval_operands_input_op1_base_op0,
        eval_operands_input_res_op1,
        eval_operands_input_offset0,
        eval_operands_input_offset1,
        eval_operands_input_offset2,
    ] =
        input;

    // Constraint - dst_src
    let constraint_quotient = ((dst_src_col0
        - ((eval_operands_input_dst_base_fp * eval_operands_input_fp)
            + ((qm31_const::<1, 0, 0, 0>() - eval_operands_input_dst_base_fp)
                * eval_operands_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    read_positive_num_bits_252_evaluate(
        (dst_src_col0 + eval_operands_input_offset0),
        dst_id_col1,
        dst_limb_0_col2,
        dst_limb_1_col3,
        dst_limb_2_col4,
        dst_limb_3_col5,
        dst_limb_4_col6,
        dst_limb_5_col7,
        dst_limb_6_col8,
        dst_limb_7_col9,
        dst_limb_8_col10,
        dst_limb_9_col11,
        dst_limb_10_col12,
        dst_limb_11_col13,
        dst_limb_12_col14,
        dst_limb_13_col15,
        dst_limb_14_col16,
        dst_limb_15_col17,
        dst_limb_16_col18,
        dst_limb_17_col19,
        dst_limb_18_col20,
        dst_limb_19_col21,
        dst_limb_20_col22,
        dst_limb_21_col23,
        dst_limb_22_col24,
        dst_limb_23_col25,
        dst_limb_24_col26,
        dst_limb_25_col27,
        dst_limb_26_col28,
        dst_limb_27_col29,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_0,
        ref memory_id_to_big_sum_1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - op0_src
    let constraint_quotient = ((op0_src_col30
        - ((eval_operands_input_op0_base_fp * eval_operands_input_fp)
            + ((qm31_const::<1, 0, 0, 0>() - eval_operands_input_op0_base_fp)
                * eval_operands_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    read_positive_num_bits_252_evaluate(
        (op0_src_col30 + eval_operands_input_offset1),
        op0_id_col31,
        op0_limb_0_col32,
        op0_limb_1_col33,
        op0_limb_2_col34,
        op0_limb_3_col35,
        op0_limb_4_col36,
        op0_limb_5_col37,
        op0_limb_6_col38,
        op0_limb_7_col39,
        op0_limb_8_col40,
        op0_limb_9_col41,
        op0_limb_10_col42,
        op0_limb_11_col43,
        op0_limb_12_col44,
        op0_limb_13_col45,
        op0_limb_14_col46,
        op0_limb_15_col47,
        op0_limb_16_col48,
        op0_limb_17_col49,
        op0_limb_18_col50,
        op0_limb_19_col51,
        op0_limb_20_col52,
        op0_limb_21_col53,
        op0_limb_22_col54,
        op0_limb_23_col55,
        op0_limb_24_col56,
        op0_limb_25_col57,
        op0_limb_26_col58,
        op0_limb_27_col59,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_2,
        ref memory_id_to_big_sum_3,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let cond_felt_252_as_addr_output_tmp_3172c_12: QM31 = cond_felt_252_as_addr_evaluate(
        [
            op0_limb_0_col32, op0_limb_1_col33, op0_limb_2_col34, op0_limb_3_col35,
            op0_limb_4_col36, op0_limb_5_col37, op0_limb_6_col38, op0_limb_7_col39,
            op0_limb_8_col40, op0_limb_9_col41, op0_limb_10_col42, op0_limb_11_col43,
            op0_limb_12_col44, op0_limb_13_col45, op0_limb_14_col46, op0_limb_15_col47,
            op0_limb_16_col48, op0_limb_17_col49, op0_limb_18_col50, op0_limb_19_col51,
            op0_limb_20_col52, op0_limb_21_col53, op0_limb_22_col54, op0_limb_23_col55,
            op0_limb_24_col56, op0_limb_25_col57, op0_limb_26_col58, op0_limb_27_col59,
            eval_operands_input_op1_base_op0,
        ],
        partial_limb_msb_col60,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - op1_src
    let constraint_quotient = ((op1_src_col61
        - ((((eval_operands_input_op1_base_fp * eval_operands_input_fp)
            + (eval_operands_input_op1_base_ap * eval_operands_input_ap))
            + (eval_operands_input_op1_imm * eval_operands_input_pc))
            + (eval_operands_input_op1_base_op0 * cond_felt_252_as_addr_output_tmp_3172c_12))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    read_positive_num_bits_252_evaluate(
        (op1_src_col61 + eval_operands_input_offset2),
        op1_id_col62,
        op1_limb_0_col63,
        op1_limb_1_col64,
        op1_limb_2_col65,
        op1_limb_3_col66,
        op1_limb_4_col67,
        op1_limb_5_col68,
        op1_limb_6_col69,
        op1_limb_7_col70,
        op1_limb_8_col71,
        op1_limb_9_col72,
        op1_limb_10_col73,
        op1_limb_11_col74,
        op1_limb_12_col75,
        op1_limb_13_col76,
        op1_limb_14_col77,
        op1_limb_15_col78,
        op1_limb_16_col79,
        op1_limb_17_col80,
        op1_limb_18_col81,
        op1_limb_19_col82,
        op1_limb_20_col83,
        op1_limb_21_col84,
        op1_limb_22_col85,
        op1_limb_23_col86,
        op1_limb_24_col87,
        op1_limb_25_col88,
        op1_limb_26_col89,
        op1_limb_27_col90,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_4,
        ref memory_id_to_big_sum_5,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    add_252_evaluate(
        [
            op0_limb_0_col32, op0_limb_1_col33, op0_limb_2_col34, op0_limb_3_col35,
            op0_limb_4_col36, op0_limb_5_col37, op0_limb_6_col38, op0_limb_7_col39,
            op0_limb_8_col40, op0_limb_9_col41, op0_limb_10_col42, op0_limb_11_col43,
            op0_limb_12_col44, op0_limb_13_col45, op0_limb_14_col46, op0_limb_15_col47,
            op0_limb_16_col48, op0_limb_17_col49, op0_limb_18_col50, op0_limb_19_col51,
            op0_limb_20_col52, op0_limb_21_col53, op0_limb_22_col54, op0_limb_23_col55,
            op0_limb_24_col56, op0_limb_25_col57, op0_limb_26_col58, op0_limb_27_col59,
            op1_limb_0_col63, op1_limb_1_col64, op1_limb_2_col65, op1_limb_3_col66,
            op1_limb_4_col67, op1_limb_5_col68, op1_limb_6_col69, op1_limb_7_col70,
            op1_limb_8_col71, op1_limb_9_col72, op1_limb_10_col73, op1_limb_11_col74,
            op1_limb_12_col75, op1_limb_13_col76, op1_limb_14_col77, op1_limb_15_col78,
            op1_limb_16_col79, op1_limb_17_col80, op1_limb_18_col81, op1_limb_19_col82,
            op1_limb_20_col83, op1_limb_21_col84, op1_limb_22_col85, op1_limb_23_col86,
            op1_limb_24_col87, op1_limb_25_col88, op1_limb_26_col89, op1_limb_27_col90,
        ],
        add_res_limb_0_col91,
        add_res_limb_1_col92,
        add_res_limb_2_col93,
        add_res_limb_3_col94,
        add_res_limb_4_col95,
        add_res_limb_5_col96,
        add_res_limb_6_col97,
        add_res_limb_7_col98,
        add_res_limb_8_col99,
        add_res_limb_9_col100,
        add_res_limb_10_col101,
        add_res_limb_11_col102,
        add_res_limb_12_col103,
        add_res_limb_13_col104,
        add_res_limb_14_col105,
        add_res_limb_15_col106,
        add_res_limb_16_col107,
        add_res_limb_17_col108,
        add_res_limb_18_col109,
        add_res_limb_19_col110,
        add_res_limb_20_col111,
        add_res_limb_21_col112,
        add_res_limb_22_col113,
        add_res_limb_23_col114,
        add_res_limb_24_col115,
        add_res_limb_25_col116,
        add_res_limb_26_col117,
        add_res_limb_27_col118,
        sub_p_bit_col119,
        range_check_9_9_lookup_elements,
        range_check_9_9_b_lookup_elements,
        range_check_9_9_c_lookup_elements,
        range_check_9_9_d_lookup_elements,
        range_check_9_9_e_lookup_elements,
        range_check_9_9_f_lookup_elements,
        range_check_9_9_g_lookup_elements,
        range_check_9_9_h_lookup_elements,
        ref range_check_9_9_sum_6,
        ref range_check_9_9_b_sum_7,
        ref range_check_9_9_c_sum_8,
        ref range_check_9_9_d_sum_9,
        ref range_check_9_9_e_sum_10,
        ref range_check_9_9_f_sum_11,
        ref range_check_9_9_g_sum_12,
        ref range_check_9_9_h_sum_13,
        ref range_check_9_9_sum_14,
        ref range_check_9_9_b_sum_15,
        ref range_check_9_9_c_sum_16,
        ref range_check_9_9_d_sum_17,
        ref range_check_9_9_e_sum_18,
        ref range_check_9_9_f_sum_19,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    mul_252_evaluate(
        [
            op0_limb_0_col32, op0_limb_1_col33, op0_limb_2_col34, op0_limb_3_col35,
            op0_limb_4_col36, op0_limb_5_col37, op0_limb_6_col38, op0_limb_7_col39,
            op0_limb_8_col40, op0_limb_9_col41, op0_limb_10_col42, op0_limb_11_col43,
            op0_limb_12_col44, op0_limb_13_col45, op0_limb_14_col46, op0_limb_15_col47,
            op0_limb_16_col48, op0_limb_17_col49, op0_limb_18_col50, op0_limb_19_col51,
            op0_limb_20_col52, op0_limb_21_col53, op0_limb_22_col54, op0_limb_23_col55,
            op0_limb_24_col56, op0_limb_25_col57, op0_limb_26_col58, op0_limb_27_col59,
            op1_limb_0_col63, op1_limb_1_col64, op1_limb_2_col65, op1_limb_3_col66,
            op1_limb_4_col67, op1_limb_5_col68, op1_limb_6_col69, op1_limb_7_col70,
            op1_limb_8_col71, op1_limb_9_col72, op1_limb_10_col73, op1_limb_11_col74,
            op1_limb_12_col75, op1_limb_13_col76, op1_limb_14_col77, op1_limb_15_col78,
            op1_limb_16_col79, op1_limb_17_col80, op1_limb_18_col81, op1_limb_19_col82,
            op1_limb_20_col83, op1_limb_21_col84, op1_limb_22_col85, op1_limb_23_col86,
            op1_limb_24_col87, op1_limb_25_col88, op1_limb_26_col89, op1_limb_27_col90,
        ],
        mul_res_limb_0_col120,
        mul_res_limb_1_col121,
        mul_res_limb_2_col122,
        mul_res_limb_3_col123,
        mul_res_limb_4_col124,
        mul_res_limb_5_col125,
        mul_res_limb_6_col126,
        mul_res_limb_7_col127,
        mul_res_limb_8_col128,
        mul_res_limb_9_col129,
        mul_res_limb_10_col130,
        mul_res_limb_11_col131,
        mul_res_limb_12_col132,
        mul_res_limb_13_col133,
        mul_res_limb_14_col134,
        mul_res_limb_15_col135,
        mul_res_limb_16_col136,
        mul_res_limb_17_col137,
        mul_res_limb_18_col138,
        mul_res_limb_19_col139,
        mul_res_limb_20_col140,
        mul_res_limb_21_col141,
        mul_res_limb_22_col142,
        mul_res_limb_23_col143,
        mul_res_limb_24_col144,
        mul_res_limb_25_col145,
        mul_res_limb_26_col146,
        mul_res_limb_27_col147,
        k_col148,
        carry_0_col149,
        carry_1_col150,
        carry_2_col151,
        carry_3_col152,
        carry_4_col153,
        carry_5_col154,
        carry_6_col155,
        carry_7_col156,
        carry_8_col157,
        carry_9_col158,
        carry_10_col159,
        carry_11_col160,
        carry_12_col161,
        carry_13_col162,
        carry_14_col163,
        carry_15_col164,
        carry_16_col165,
        carry_17_col166,
        carry_18_col167,
        carry_19_col168,
        carry_20_col169,
        carry_21_col170,
        carry_22_col171,
        carry_23_col172,
        carry_24_col173,
        carry_25_col174,
        carry_26_col175,
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
        ref range_check_9_9_sum_20,
        ref range_check_9_9_b_sum_21,
        ref range_check_9_9_c_sum_22,
        ref range_check_9_9_d_sum_23,
        ref range_check_9_9_e_sum_24,
        ref range_check_9_9_f_sum_25,
        ref range_check_9_9_g_sum_26,
        ref range_check_9_9_h_sum_27,
        ref range_check_9_9_sum_28,
        ref range_check_9_9_b_sum_29,
        ref range_check_9_9_c_sum_30,
        ref range_check_9_9_d_sum_31,
        ref range_check_9_9_e_sum_32,
        ref range_check_9_9_f_sum_33,
        ref range_check_20_sum_34,
        ref range_check_20_b_sum_35,
        ref range_check_20_c_sum_36,
        ref range_check_20_d_sum_37,
        ref range_check_20_e_sum_38,
        ref range_check_20_f_sum_39,
        ref range_check_20_g_sum_40,
        ref range_check_20_h_sum_41,
        ref range_check_20_sum_42,
        ref range_check_20_b_sum_43,
        ref range_check_20_c_sum_44,
        ref range_check_20_d_sum_45,
        ref range_check_20_e_sum_46,
        ref range_check_20_f_sum_47,
        ref range_check_20_g_sum_48,
        ref range_check_20_h_sum_49,
        ref range_check_20_sum_50,
        ref range_check_20_b_sum_51,
        ref range_check_20_c_sum_52,
        ref range_check_20_d_sum_53,
        ref range_check_20_e_sum_54,
        ref range_check_20_f_sum_55,
        ref range_check_20_g_sum_56,
        ref range_check_20_h_sum_57,
        ref range_check_20_sum_58,
        ref range_check_20_b_sum_59,
        ref range_check_20_c_sum_60,
        ref range_check_20_d_sum_61,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let not_pc_update_jnz_tmp_3172c_54: QM31 = (qm31_const::<1, 0, 0, 0>()
        - eval_operands_input_pc_update_jnz);

    // Constraint - constrain limb 0 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_0_col91)
        + (eval_operands_input_res_mul * mul_res_limb_0_col120))
        + (eval_operands_input_res_op1 * op1_limb_0_col63))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_0_col176)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 1 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_1_col92)
        + (eval_operands_input_res_mul * mul_res_limb_1_col121))
        + (eval_operands_input_res_op1 * op1_limb_1_col64))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_1_col177)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 2 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_2_col93)
        + (eval_operands_input_res_mul * mul_res_limb_2_col122))
        + (eval_operands_input_res_op1 * op1_limb_2_col65))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_2_col178)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 3 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_3_col94)
        + (eval_operands_input_res_mul * mul_res_limb_3_col123))
        + (eval_operands_input_res_op1 * op1_limb_3_col66))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_3_col179)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 4 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_4_col95)
        + (eval_operands_input_res_mul * mul_res_limb_4_col124))
        + (eval_operands_input_res_op1 * op1_limb_4_col67))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_4_col180)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 5 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_5_col96)
        + (eval_operands_input_res_mul * mul_res_limb_5_col125))
        + (eval_operands_input_res_op1 * op1_limb_5_col68))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_5_col181)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 6 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_6_col97)
        + (eval_operands_input_res_mul * mul_res_limb_6_col126))
        + (eval_operands_input_res_op1 * op1_limb_6_col69))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_6_col182)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 7 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_7_col98)
        + (eval_operands_input_res_mul * mul_res_limb_7_col127))
        + (eval_operands_input_res_op1 * op1_limb_7_col70))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_7_col183)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 8 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_8_col99)
        + (eval_operands_input_res_mul * mul_res_limb_8_col128))
        + (eval_operands_input_res_op1 * op1_limb_8_col71))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_8_col184)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 9 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_9_col100)
        + (eval_operands_input_res_mul * mul_res_limb_9_col129))
        + (eval_operands_input_res_op1 * op1_limb_9_col72))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_9_col185)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 10 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_10_col101)
        + (eval_operands_input_res_mul * mul_res_limb_10_col130))
        + (eval_operands_input_res_op1 * op1_limb_10_col73))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_10_col186)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 11 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_11_col102)
        + (eval_operands_input_res_mul * mul_res_limb_11_col131))
        + (eval_operands_input_res_op1 * op1_limb_11_col74))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_11_col187)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 12 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_12_col103)
        + (eval_operands_input_res_mul * mul_res_limb_12_col132))
        + (eval_operands_input_res_op1 * op1_limb_12_col75))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_12_col188)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 13 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_13_col104)
        + (eval_operands_input_res_mul * mul_res_limb_13_col133))
        + (eval_operands_input_res_op1 * op1_limb_13_col76))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_13_col189)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 14 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_14_col105)
        + (eval_operands_input_res_mul * mul_res_limb_14_col134))
        + (eval_operands_input_res_op1 * op1_limb_14_col77))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_14_col190)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 15 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_15_col106)
        + (eval_operands_input_res_mul * mul_res_limb_15_col135))
        + (eval_operands_input_res_op1 * op1_limb_15_col78))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_15_col191)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 16 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_16_col107)
        + (eval_operands_input_res_mul * mul_res_limb_16_col136))
        + (eval_operands_input_res_op1 * op1_limb_16_col79))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_16_col192)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 17 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_17_col108)
        + (eval_operands_input_res_mul * mul_res_limb_17_col137))
        + (eval_operands_input_res_op1 * op1_limb_17_col80))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_17_col193)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 18 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_18_col109)
        + (eval_operands_input_res_mul * mul_res_limb_18_col138))
        + (eval_operands_input_res_op1 * op1_limb_18_col81))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_18_col194)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 19 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_19_col110)
        + (eval_operands_input_res_mul * mul_res_limb_19_col139))
        + (eval_operands_input_res_op1 * op1_limb_19_col82))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_19_col195)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 20 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_20_col111)
        + (eval_operands_input_res_mul * mul_res_limb_20_col140))
        + (eval_operands_input_res_op1 * op1_limb_20_col83))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_20_col196)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 21 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_21_col112)
        + (eval_operands_input_res_mul * mul_res_limb_21_col141))
        + (eval_operands_input_res_op1 * op1_limb_21_col84))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_21_col197)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 22 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_22_col113)
        + (eval_operands_input_res_mul * mul_res_limb_22_col142))
        + (eval_operands_input_res_op1 * op1_limb_22_col85))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_22_col198)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 23 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_23_col114)
        + (eval_operands_input_res_mul * mul_res_limb_23_col143))
        + (eval_operands_input_res_op1 * op1_limb_23_col86))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_23_col199)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 24 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_24_col115)
        + (eval_operands_input_res_mul * mul_res_limb_24_col144))
        + (eval_operands_input_res_op1 * op1_limb_24_col87))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_24_col200)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 25 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_25_col116)
        + (eval_operands_input_res_mul * mul_res_limb_25_col145))
        + (eval_operands_input_res_op1 * op1_limb_25_col88))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_25_col201)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 26 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_26_col117)
        + (eval_operands_input_res_mul * mul_res_limb_26_col146))
        + (eval_operands_input_res_op1 * op1_limb_26_col89))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_26_col202)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - constrain limb 27 of res
    let constraint_quotient = (((((eval_operands_input_res_add * add_res_limb_27_col118)
        + (eval_operands_input_res_mul * mul_res_limb_27_col147))
        + (eval_operands_input_res_op1 * op1_limb_27_col90))
        - (not_pc_update_jnz_tmp_3172c_54 * res_limb_27_col203)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}

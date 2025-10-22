// This file was created by the AIR team.

use crate::components::subroutines::decode_instruction_472fe::decode_instruction_472fe_evaluate;
use crate::components::subroutines::read_positive_num_bits_29::read_positive_num_bits_29_evaluate;
use crate::components::subroutines::read_u_32::read_u_32_evaluate;
use crate::prelude::*;


pub fn decode_blake_opcode_evaluate(
    input: [QM31; 3],
    offset0_col0: QM31,
    offset1_col1: QM31,
    offset2_col2: QM31,
    dst_base_fp_col3: QM31,
    op0_base_fp_col4: QM31,
    op1_base_fp_col5: QM31,
    ap_update_add_1_col6: QM31,
    opcode_extension_col7: QM31,
    mem0_base_col8: QM31,
    op0_id_col9: QM31,
    op0_limb_0_col10: QM31,
    op0_limb_1_col11: QM31,
    op0_limb_2_col12: QM31,
    op0_limb_3_col13: QM31,
    partial_limb_msb_col14: QM31,
    mem1_base_col15: QM31,
    op1_id_col16: QM31,
    op1_limb_0_col17: QM31,
    op1_limb_1_col18: QM31,
    op1_limb_2_col19: QM31,
    op1_limb_3_col20: QM31,
    partial_limb_msb_col21: QM31,
    ap_id_col22: QM31,
    ap_limb_0_col23: QM31,
    ap_limb_1_col24: QM31,
    ap_limb_2_col25: QM31,
    ap_limb_3_col26: QM31,
    partial_limb_msb_col27: QM31,
    mem_dst_base_col28: QM31,
    low_16_bits_col29: QM31,
    high_16_bits_col30: QM31,
    low_7_ms_bits_col31: QM31,
    high_14_ms_bits_col32: QM31,
    high_5_ms_bits_col33: QM31,
    dst_id_col34: QM31,
    verify_instruction_lookup_elements: @crate::VerifyInstructionElements,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    range_check_7_2_5_lookup_elements: @crate::RangeCheck_7_2_5Elements,
    ref verify_instruction_sum_0: QM31,
    ref memory_address_to_id_sum_1: QM31,
    ref memory_id_to_big_sum_2: QM31,
    ref memory_address_to_id_sum_3: QM31,
    ref memory_id_to_big_sum_4: QM31,
    ref memory_address_to_id_sum_5: QM31,
    ref memory_id_to_big_sum_6: QM31,
    ref range_check_7_2_5_sum_7: QM31,
    ref memory_address_to_id_sum_8: QM31,
    ref memory_id_to_big_sum_9: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 4] {
    let [decode_blake_opcode_input_pc, decode_blake_opcode_input_ap, decode_blake_opcode_input_fp] =
        input;
    let [
        decode_instruction_472fe_output_tmp_47e62_9_offset0,
        decode_instruction_472fe_output_tmp_47e62_9_offset1,
        decode_instruction_472fe_output_tmp_47e62_9_offset2,
        decode_instruction_472fe_output_tmp_47e62_9_op1_base_ap,
    ] =
        decode_instruction_472fe_evaluate(
        decode_blake_opcode_input_pc,
        offset0_col0,
        offset1_col1,
        offset2_col2,
        dst_base_fp_col3,
        op0_base_fp_col4,
        op1_base_fp_col5,
        ap_update_add_1_col6,
        opcode_extension_col7,
        verify_instruction_lookup_elements,
        ref verify_instruction_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - OpcodeExtension is either Blake or BlakeFinalize
    let constraint_quotient = (((opcode_extension_col7 - qm31_const::<1, 0, 0, 0>())
        * (opcode_extension_col7 - qm31_const::<2, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - mem0_base
    let constraint_quotient = ((mem0_base_col8
        - ((op0_base_fp_col4 * decode_blake_opcode_input_fp)
            + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col4) * decode_blake_opcode_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    read_positive_num_bits_29_evaluate(
        (mem0_base_col8 + decode_instruction_472fe_output_tmp_47e62_9_offset1),
        op0_id_col9,
        op0_limb_0_col10,
        op0_limb_1_col11,
        op0_limb_2_col12,
        op0_limb_3_col13,
        partial_limb_msb_col14,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_1,
        ref memory_id_to_big_sum_2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - mem1_base
    let constraint_quotient = ((mem1_base_col15
        - ((op1_base_fp_col5 * decode_blake_opcode_input_fp)
            + (decode_instruction_472fe_output_tmp_47e62_9_op1_base_ap
                * decode_blake_opcode_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    read_positive_num_bits_29_evaluate(
        (mem1_base_col15 + decode_instruction_472fe_output_tmp_47e62_9_offset2),
        op1_id_col16,
        op1_limb_0_col17,
        op1_limb_1_col18,
        op1_limb_2_col19,
        op1_limb_3_col20,
        partial_limb_msb_col21,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_3,
        ref memory_id_to_big_sum_4,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    read_positive_num_bits_29_evaluate(
        decode_blake_opcode_input_ap,
        ap_id_col22,
        ap_limb_0_col23,
        ap_limb_1_col24,
        ap_limb_2_col25,
        ap_limb_3_col26,
        partial_limb_msb_col27,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_5,
        ref memory_id_to_big_sum_6,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    // Constraint - mem_dst_base
    let constraint_quotient = ((mem_dst_base_col28
        - ((dst_base_fp_col3 * decode_blake_opcode_input_fp)
            + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col3) * decode_blake_opcode_input_ap))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    read_u_32_evaluate(
        (mem_dst_base_col28 + decode_instruction_472fe_output_tmp_47e62_9_offset0),
        low_16_bits_col29,
        high_16_bits_col30,
        low_7_ms_bits_col31,
        high_14_ms_bits_col32,
        high_5_ms_bits_col33,
        dst_id_col34,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_7,
        ref memory_address_to_id_sum_8,
        ref memory_id_to_big_sum_9,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    [
        (((op0_limb_0_col10 + (op0_limb_1_col11 * qm31_const::<512, 0, 0, 0>()))
            + (op0_limb_2_col12 * qm31_const::<262144, 0, 0, 0>()))
            + (op0_limb_3_col13 * qm31_const::<134217728, 0, 0, 0>())),
        (((op1_limb_0_col17 + (op1_limb_1_col18 * qm31_const::<512, 0, 0, 0>()))
            + (op1_limb_2_col19 * qm31_const::<262144, 0, 0, 0>()))
            + (op1_limb_3_col20 * qm31_const::<134217728, 0, 0, 0>())),
        (((ap_limb_0_col23 + (ap_limb_1_col24 * qm31_const::<512, 0, 0, 0>()))
            + (ap_limb_2_col25 * qm31_const::<262144, 0, 0, 0>()))
            + (ap_limb_3_col26 * qm31_const::<134217728, 0, 0, 0>())),
        (opcode_extension_col7 - qm31_const::<1, 0, 0, 0>()),
    ]
}

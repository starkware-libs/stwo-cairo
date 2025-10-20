// This file was created by the AIR team.

use crate::prelude::*;


pub fn create_blake_output_evaluate(
    input: [QM31; 48],
    triple_xor_32_output_limb_0_col0: QM31,
    triple_xor_32_output_limb_1_col1: QM31,
    triple_xor_32_output_limb_0_col2: QM31,
    triple_xor_32_output_limb_1_col3: QM31,
    triple_xor_32_output_limb_0_col4: QM31,
    triple_xor_32_output_limb_1_col5: QM31,
    triple_xor_32_output_limb_0_col6: QM31,
    triple_xor_32_output_limb_1_col7: QM31,
    triple_xor_32_output_limb_0_col8: QM31,
    triple_xor_32_output_limb_1_col9: QM31,
    triple_xor_32_output_limb_0_col10: QM31,
    triple_xor_32_output_limb_1_col11: QM31,
    triple_xor_32_output_limb_0_col12: QM31,
    triple_xor_32_output_limb_1_col13: QM31,
    triple_xor_32_output_limb_0_col14: QM31,
    triple_xor_32_output_limb_1_col15: QM31,
    triple_xor_32_lookup_elements: @crate::TripleXor32Elements,
    ref triple_xor_32_sum_0: QM31,
    ref triple_xor_32_sum_1: QM31,
    ref triple_xor_32_sum_2: QM31,
    ref triple_xor_32_sum_3: QM31,
    ref triple_xor_32_sum_4: QM31,
    ref triple_xor_32_sum_5: QM31,
    ref triple_xor_32_sum_6: QM31,
    ref triple_xor_32_sum_7: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        create_blake_output_input_limb_0,
        create_blake_output_input_limb_1,
        create_blake_output_input_limb_2,
        create_blake_output_input_limb_3,
        create_blake_output_input_limb_4,
        create_blake_output_input_limb_5,
        create_blake_output_input_limb_6,
        create_blake_output_input_limb_7,
        create_blake_output_input_limb_8,
        create_blake_output_input_limb_9,
        create_blake_output_input_limb_10,
        create_blake_output_input_limb_11,
        create_blake_output_input_limb_12,
        create_blake_output_input_limb_13,
        create_blake_output_input_limb_14,
        create_blake_output_input_limb_15,
        create_blake_output_input_limb_16,
        create_blake_output_input_limb_17,
        create_blake_output_input_limb_18,
        create_blake_output_input_limb_19,
        create_blake_output_input_limb_20,
        create_blake_output_input_limb_21,
        create_blake_output_input_limb_22,
        create_blake_output_input_limb_23,
        create_blake_output_input_limb_24,
        create_blake_output_input_limb_25,
        create_blake_output_input_limb_26,
        create_blake_output_input_limb_27,
        create_blake_output_input_limb_28,
        create_blake_output_input_limb_29,
        create_blake_output_input_limb_30,
        create_blake_output_input_limb_31,
        create_blake_output_input_limb_32,
        create_blake_output_input_limb_33,
        create_blake_output_input_limb_34,
        create_blake_output_input_limb_35,
        create_blake_output_input_limb_36,
        create_blake_output_input_limb_37,
        create_blake_output_input_limb_38,
        create_blake_output_input_limb_39,
        create_blake_output_input_limb_40,
        create_blake_output_input_limb_41,
        create_blake_output_input_limb_42,
        create_blake_output_input_limb_43,
        create_blake_output_input_limb_44,
        create_blake_output_input_limb_45,
        create_blake_output_input_limb_46,
        create_blake_output_input_limb_47,
    ] =
        input;

    triple_xor_32_sum_0 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_16, create_blake_output_input_limb_17,
                create_blake_output_input_limb_32, create_blake_output_input_limb_33,
                create_blake_output_input_limb_0, create_blake_output_input_limb_1,
                triple_xor_32_output_limb_0_col0, triple_xor_32_output_limb_1_col1,
            ],
        );

    triple_xor_32_sum_1 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_18, create_blake_output_input_limb_19,
                create_blake_output_input_limb_34, create_blake_output_input_limb_35,
                create_blake_output_input_limb_2, create_blake_output_input_limb_3,
                triple_xor_32_output_limb_0_col2, triple_xor_32_output_limb_1_col3,
            ],
        );

    triple_xor_32_sum_2 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_20, create_blake_output_input_limb_21,
                create_blake_output_input_limb_36, create_blake_output_input_limb_37,
                create_blake_output_input_limb_4, create_blake_output_input_limb_5,
                triple_xor_32_output_limb_0_col4, triple_xor_32_output_limb_1_col5,
            ],
        );

    triple_xor_32_sum_3 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_22, create_blake_output_input_limb_23,
                create_blake_output_input_limb_38, create_blake_output_input_limb_39,
                create_blake_output_input_limb_6, create_blake_output_input_limb_7,
                triple_xor_32_output_limb_0_col6, triple_xor_32_output_limb_1_col7,
            ],
        );

    triple_xor_32_sum_4 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_24, create_blake_output_input_limb_25,
                create_blake_output_input_limb_40, create_blake_output_input_limb_41,
                create_blake_output_input_limb_8, create_blake_output_input_limb_9,
                triple_xor_32_output_limb_0_col8, triple_xor_32_output_limb_1_col9,
            ],
        );

    triple_xor_32_sum_5 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_26, create_blake_output_input_limb_27,
                create_blake_output_input_limb_42, create_blake_output_input_limb_43,
                create_blake_output_input_limb_10, create_blake_output_input_limb_11,
                triple_xor_32_output_limb_0_col10, triple_xor_32_output_limb_1_col11,
            ],
        );

    triple_xor_32_sum_6 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_28, create_blake_output_input_limb_29,
                create_blake_output_input_limb_44, create_blake_output_input_limb_45,
                create_blake_output_input_limb_12, create_blake_output_input_limb_13,
                triple_xor_32_output_limb_0_col12, triple_xor_32_output_limb_1_col13,
            ],
        );

    triple_xor_32_sum_7 = triple_xor_32_lookup_elements
        .combine_qm31(
            [
                create_blake_output_input_limb_30, create_blake_output_input_limb_31,
                create_blake_output_input_limb_46, create_blake_output_input_limb_47,
                create_blake_output_input_limb_14, create_blake_output_input_limb_15,
                triple_xor_32_output_limb_0_col14, triple_xor_32_output_limb_1_col15,
            ],
        );

    []
}

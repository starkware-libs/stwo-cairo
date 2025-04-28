use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::cube_252::{CUBE_252_RELATION_SIZE, cube_252_sum};
use crate::components::range_check_4_4::{RANGE_CHECK_4_4_RELATION_SIZE, range_check_4_4_sum};
use crate::components::range_check_4_4_4_4::{
    RANGE_CHECK_4_4_4_4_RELATION_SIZE, range_check_4_4_4_4_sum,
};
use crate::components::range_check_felt_252_width_27::{
    RANGE_CHECK_FELT_252_WIDTH_27_RELATION_SIZE, range_check_felt_252_width_27_sum,
};
use crate::components::subroutines::linear_combination_n_1_coefs_2::linear_combination_n_1_coefs_2_evaluate;
use crate::components::subroutines::linear_combination_n_6_coefs_4_2_3_1_m1_1::linear_combination_n_6_coefs_4_2_3_1_m1_1_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 32;


pub fn poseidon_partial_round_evaluate(
    input: [QM31; 50],
    cube_252_output_limb_0_col0: QM31,
    cube_252_output_limb_1_col1: QM31,
    cube_252_output_limb_2_col2: QM31,
    cube_252_output_limb_3_col3: QM31,
    cube_252_output_limb_4_col4: QM31,
    cube_252_output_limb_5_col5: QM31,
    cube_252_output_limb_6_col6: QM31,
    cube_252_output_limb_7_col7: QM31,
    cube_252_output_limb_8_col8: QM31,
    cube_252_output_limb_9_col9: QM31,
    combination_limb_0_col10: QM31,
    combination_limb_1_col11: QM31,
    combination_limb_2_col12: QM31,
    combination_limb_3_col13: QM31,
    combination_limb_4_col14: QM31,
    combination_limb_5_col15: QM31,
    combination_limb_6_col16: QM31,
    combination_limb_7_col17: QM31,
    combination_limb_8_col18: QM31,
    combination_limb_9_col19: QM31,
    p_coef_col20: QM31,
    combination_limb_0_col21: QM31,
    combination_limb_1_col22: QM31,
    combination_limb_2_col23: QM31,
    combination_limb_3_col24: QM31,
    combination_limb_4_col25: QM31,
    combination_limb_5_col26: QM31,
    combination_limb_6_col27: QM31,
    combination_limb_7_col28: QM31,
    combination_limb_8_col29: QM31,
    combination_limb_9_col30: QM31,
    p_coef_col31: QM31,
    cube_252_alphas: Span<QM31>,
    cube_252_z: QM31,
    range_check_4_4_4_4_alphas: Span<QM31>,
    range_check_4_4_4_4_z: QM31,
    range_check_4_4_alphas: Span<QM31>,
    range_check_4_4_z: QM31,
    range_check_felt_252_width_27_alphas: Span<QM31>,
    range_check_felt_252_width_27_z: QM31,
    ref cube_252_sum_0: QM31,
    ref range_check_4_4_4_4_sum_1: QM31,
    ref range_check_4_4_4_4_sum_2: QM31,
    ref range_check_4_4_sum_3: QM31,
    ref range_check_felt_252_width_27_sum_4: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 40] {
    let [
        poseidon_partial_round_input_limb_0,
        poseidon_partial_round_input_limb_1,
        poseidon_partial_round_input_limb_2,
        poseidon_partial_round_input_limb_3,
        poseidon_partial_round_input_limb_4,
        poseidon_partial_round_input_limb_5,
        poseidon_partial_round_input_limb_6,
        poseidon_partial_round_input_limb_7,
        poseidon_partial_round_input_limb_8,
        poseidon_partial_round_input_limb_9,
        poseidon_partial_round_input_limb_10,
        poseidon_partial_round_input_limb_11,
        poseidon_partial_round_input_limb_12,
        poseidon_partial_round_input_limb_13,
        poseidon_partial_round_input_limb_14,
        poseidon_partial_round_input_limb_15,
        poseidon_partial_round_input_limb_16,
        poseidon_partial_round_input_limb_17,
        poseidon_partial_round_input_limb_18,
        poseidon_partial_round_input_limb_19,
        poseidon_partial_round_input_limb_20,
        poseidon_partial_round_input_limb_21,
        poseidon_partial_round_input_limb_22,
        poseidon_partial_round_input_limb_23,
        poseidon_partial_round_input_limb_24,
        poseidon_partial_round_input_limb_25,
        poseidon_partial_round_input_limb_26,
        poseidon_partial_round_input_limb_27,
        poseidon_partial_round_input_limb_28,
        poseidon_partial_round_input_limb_29,
        poseidon_partial_round_input_limb_30,
        poseidon_partial_round_input_limb_31,
        poseidon_partial_round_input_limb_32,
        poseidon_partial_round_input_limb_33,
        poseidon_partial_round_input_limb_34,
        poseidon_partial_round_input_limb_35,
        poseidon_partial_round_input_limb_36,
        poseidon_partial_round_input_limb_37,
        poseidon_partial_round_input_limb_38,
        poseidon_partial_round_input_limb_39,
        poseidon_partial_round_input_limb_40,
        poseidon_partial_round_input_limb_41,
        poseidon_partial_round_input_limb_42,
        poseidon_partial_round_input_limb_43,
        poseidon_partial_round_input_limb_44,
        poseidon_partial_round_input_limb_45,
        poseidon_partial_round_input_limb_46,
        poseidon_partial_round_input_limb_47,
        poseidon_partial_round_input_limb_48,
        poseidon_partial_round_input_limb_49,
    ] =
        input;

    cube_252_sum_0 =
        cube_252_sum(
            cube_252_alphas,
            cube_252_z,
            [
                poseidon_partial_round_input_limb_30, poseidon_partial_round_input_limb_31,
                poseidon_partial_round_input_limb_32, poseidon_partial_round_input_limb_33,
                poseidon_partial_round_input_limb_34, poseidon_partial_round_input_limb_35,
                poseidon_partial_round_input_limb_36, poseidon_partial_round_input_limb_37,
                poseidon_partial_round_input_limb_38, poseidon_partial_round_input_limb_39,
                cube_252_output_limb_0_col0, cube_252_output_limb_1_col1,
                cube_252_output_limb_2_col2, cube_252_output_limb_3_col3,
                cube_252_output_limb_4_col4, cube_252_output_limb_5_col5,
                cube_252_output_limb_6_col6, cube_252_output_limb_7_col7,
                cube_252_output_limb_8_col8, cube_252_output_limb_9_col9,
            ],
        );

    let output: [QM31; 10] = linear_combination_n_6_coefs_4_2_3_1_m1_1_evaluate(
        [
            poseidon_partial_round_input_limb_0, poseidon_partial_round_input_limb_1,
            poseidon_partial_round_input_limb_2, poseidon_partial_round_input_limb_3,
            poseidon_partial_round_input_limb_4, poseidon_partial_round_input_limb_5,
            poseidon_partial_round_input_limb_6, poseidon_partial_round_input_limb_7,
            poseidon_partial_round_input_limb_8, poseidon_partial_round_input_limb_9,
            poseidon_partial_round_input_limb_10, poseidon_partial_round_input_limb_11,
            poseidon_partial_round_input_limb_12, poseidon_partial_round_input_limb_13,
            poseidon_partial_round_input_limb_14, poseidon_partial_round_input_limb_15,
            poseidon_partial_round_input_limb_16, poseidon_partial_round_input_limb_17,
            poseidon_partial_round_input_limb_18, poseidon_partial_round_input_limb_19,
            poseidon_partial_round_input_limb_20, poseidon_partial_round_input_limb_21,
            poseidon_partial_round_input_limb_22, poseidon_partial_round_input_limb_23,
            poseidon_partial_round_input_limb_24, poseidon_partial_round_input_limb_25,
            poseidon_partial_round_input_limb_26, poseidon_partial_round_input_limb_27,
            poseidon_partial_round_input_limb_28, poseidon_partial_round_input_limb_29,
            poseidon_partial_round_input_limb_30, poseidon_partial_round_input_limb_31,
            poseidon_partial_round_input_limb_32, poseidon_partial_round_input_limb_33,
            poseidon_partial_round_input_limb_34, poseidon_partial_round_input_limb_35,
            poseidon_partial_round_input_limb_36, poseidon_partial_round_input_limb_37,
            poseidon_partial_round_input_limb_38, poseidon_partial_round_input_limb_39,
            cube_252_output_limb_0_col0, cube_252_output_limb_1_col1, cube_252_output_limb_2_col2,
            cube_252_output_limb_3_col3, cube_252_output_limb_4_col4, cube_252_output_limb_5_col5,
            cube_252_output_limb_6_col6, cube_252_output_limb_7_col7, cube_252_output_limb_8_col8,
            cube_252_output_limb_9_col9, poseidon_partial_round_input_limb_40,
            poseidon_partial_round_input_limb_41, poseidon_partial_round_input_limb_42,
            poseidon_partial_round_input_limb_43, poseidon_partial_round_input_limb_44,
            poseidon_partial_round_input_limb_45, poseidon_partial_round_input_limb_46,
            poseidon_partial_round_input_limb_47, poseidon_partial_round_input_limb_48,
            poseidon_partial_round_input_limb_49,
        ],
        combination_limb_0_col10,
        combination_limb_1_col11,
        combination_limb_2_col12,
        combination_limb_3_col13,
        combination_limb_4_col14,
        combination_limb_5_col15,
        combination_limb_6_col16,
        combination_limb_7_col17,
        combination_limb_8_col18,
        combination_limb_9_col19,
        p_coef_col20,
        range_check_4_4_4_4_alphas,
        range_check_4_4_4_4_z,
        range_check_4_4_alphas,
        range_check_4_4_z,
        ref range_check_4_4_4_4_sum_1,
        ref range_check_4_4_4_4_sum_2,
        ref range_check_4_4_sum_3,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_0,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_1,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_2,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_3,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_4,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_5,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_6,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_7,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_8,
        linear_combination_n_6_coefs_4_2_3_1_m1_1_output_tmp_1c8d0_12_limb_9,
    ] =
        output;

    range_check_felt_252_width_27_sum_4 =
        range_check_felt_252_width_27_sum(
            range_check_felt_252_width_27_alphas,
            range_check_felt_252_width_27_z,
            [
                combination_limb_0_col10, combination_limb_1_col11, combination_limb_2_col12,
                combination_limb_3_col13, combination_limb_4_col14, combination_limb_5_col15,
                combination_limb_6_col16, combination_limb_7_col17, combination_limb_8_col18,
                combination_limb_9_col19,
            ],
        );

    let output: [QM31; 10] = linear_combination_n_1_coefs_2_evaluate(
        [
            combination_limb_0_col10, combination_limb_1_col11, combination_limb_2_col12,
            combination_limb_3_col13, combination_limb_4_col14, combination_limb_5_col15,
            combination_limb_6_col16, combination_limb_7_col17, combination_limb_8_col18,
            combination_limb_9_col19,
        ],
        combination_limb_0_col21,
        combination_limb_1_col22,
        combination_limb_2_col23,
        combination_limb_3_col24,
        combination_limb_4_col25,
        combination_limb_5_col26,
        combination_limb_6_col27,
        combination_limb_7_col28,
        combination_limb_8_col29,
        combination_limb_9_col30,
        p_coef_col31,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_0,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_1,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_2,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_3,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_4,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_5,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_6,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_7,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_8,
        linear_combination_n_1_coefs_2_output_tmp_1c8d0_34_limb_9,
    ] =
        output;

    [
        poseidon_partial_round_input_limb_20, poseidon_partial_round_input_limb_21,
        poseidon_partial_round_input_limb_22, poseidon_partial_round_input_limb_23,
        poseidon_partial_round_input_limb_24, poseidon_partial_round_input_limb_25,
        poseidon_partial_round_input_limb_26, poseidon_partial_round_input_limb_27,
        poseidon_partial_round_input_limb_28, poseidon_partial_round_input_limb_29,
        poseidon_partial_round_input_limb_30, poseidon_partial_round_input_limb_31,
        poseidon_partial_round_input_limb_32, poseidon_partial_round_input_limb_33,
        poseidon_partial_round_input_limb_34, poseidon_partial_round_input_limb_35,
        poseidon_partial_round_input_limb_36, poseidon_partial_round_input_limb_37,
        poseidon_partial_round_input_limb_38, poseidon_partial_round_input_limb_39,
        cube_252_output_limb_0_col0, cube_252_output_limb_1_col1, cube_252_output_limb_2_col2,
        cube_252_output_limb_3_col3, cube_252_output_limb_4_col4, cube_252_output_limb_5_col5,
        cube_252_output_limb_6_col6, cube_252_output_limb_7_col7, cube_252_output_limb_8_col8,
        cube_252_output_limb_9_col9, combination_limb_0_col21, combination_limb_1_col22,
        combination_limb_2_col23, combination_limb_3_col24, combination_limb_4_col25,
        combination_limb_5_col26, combination_limb_6_col27, combination_limb_7_col28,
        combination_limb_8_col29, combination_limb_9_col30,
    ]
}

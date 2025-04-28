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
use crate::components::range_check_9_9::{RANGE_CHECK_9_9_RELATION_SIZE, range_check_9_9_sum};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 0;


pub fn range_check_mem_value_n_28_evaluate(
    input: [QM31; 28],
    range_check_9_9_alphas: Span<QM31>,
    range_check_9_9_z: QM31,
    ref range_check_9_9_sum_0: QM31,
    ref range_check_9_9_sum_1: QM31,
    ref range_check_9_9_sum_2: QM31,
    ref range_check_9_9_sum_3: QM31,
    ref range_check_9_9_sum_4: QM31,
    ref range_check_9_9_sum_5: QM31,
    ref range_check_9_9_sum_6: QM31,
    ref range_check_9_9_sum_7: QM31,
    ref range_check_9_9_sum_8: QM31,
    ref range_check_9_9_sum_9: QM31,
    ref range_check_9_9_sum_10: QM31,
    ref range_check_9_9_sum_11: QM31,
    ref range_check_9_9_sum_12: QM31,
    ref range_check_9_9_sum_13: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> () {
    let [
        range_check_mem_value_n_28_input_limb_0,
        range_check_mem_value_n_28_input_limb_1,
        range_check_mem_value_n_28_input_limb_2,
        range_check_mem_value_n_28_input_limb_3,
        range_check_mem_value_n_28_input_limb_4,
        range_check_mem_value_n_28_input_limb_5,
        range_check_mem_value_n_28_input_limb_6,
        range_check_mem_value_n_28_input_limb_7,
        range_check_mem_value_n_28_input_limb_8,
        range_check_mem_value_n_28_input_limb_9,
        range_check_mem_value_n_28_input_limb_10,
        range_check_mem_value_n_28_input_limb_11,
        range_check_mem_value_n_28_input_limb_12,
        range_check_mem_value_n_28_input_limb_13,
        range_check_mem_value_n_28_input_limb_14,
        range_check_mem_value_n_28_input_limb_15,
        range_check_mem_value_n_28_input_limb_16,
        range_check_mem_value_n_28_input_limb_17,
        range_check_mem_value_n_28_input_limb_18,
        range_check_mem_value_n_28_input_limb_19,
        range_check_mem_value_n_28_input_limb_20,
        range_check_mem_value_n_28_input_limb_21,
        range_check_mem_value_n_28_input_limb_22,
        range_check_mem_value_n_28_input_limb_23,
        range_check_mem_value_n_28_input_limb_24,
        range_check_mem_value_n_28_input_limb_25,
        range_check_mem_value_n_28_input_limb_26,
        range_check_mem_value_n_28_input_limb_27,
    ] =
        input;

    range_check_9_9_sum_0 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_0, range_check_mem_value_n_28_input_limb_1],
        );

    range_check_9_9_sum_1 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_2, range_check_mem_value_n_28_input_limb_3],
        );

    range_check_9_9_sum_2 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_4, range_check_mem_value_n_28_input_limb_5],
        );

    range_check_9_9_sum_3 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_6, range_check_mem_value_n_28_input_limb_7],
        );

    range_check_9_9_sum_4 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_8, range_check_mem_value_n_28_input_limb_9],
        );

    range_check_9_9_sum_5 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_10, range_check_mem_value_n_28_input_limb_11],
        );

    range_check_9_9_sum_6 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_12, range_check_mem_value_n_28_input_limb_13],
        );

    range_check_9_9_sum_7 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_14, range_check_mem_value_n_28_input_limb_15],
        );

    range_check_9_9_sum_8 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_16, range_check_mem_value_n_28_input_limb_17],
        );

    range_check_9_9_sum_9 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_18, range_check_mem_value_n_28_input_limb_19],
        );

    range_check_9_9_sum_10 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_20, range_check_mem_value_n_28_input_limb_21],
        );

    range_check_9_9_sum_11 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_22, range_check_mem_value_n_28_input_limb_23],
        );

    range_check_9_9_sum_12 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_24, range_check_mem_value_n_28_input_limb_25],
        );

    range_check_9_9_sum_13 =
        range_check_9_9_sum(
            range_check_9_9_alphas,
            range_check_9_9_z,
            [range_check_mem_value_n_28_input_limb_26, range_check_mem_value_n_28_input_limb_27],
        );

    ()
}

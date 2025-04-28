// Constraints version: 9c495569

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
use crate::components::range_check_11::{RANGE_CHECK_11_RELATION_SIZE, range_check_11_sum};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 3;


pub fn verify_mul_small_evaluate(
    input: [QM31; 16],
    carry_1_col0: QM31,
    carry_3_col1: QM31,
    carry_5_col2: QM31,
    range_check_11_alphas: Span<QM31>,
    range_check_11_z: QM31,
    ref range_check_11_sum_0: QM31,
    ref range_check_11_sum_1: QM31,
    ref range_check_11_sum_2: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        verify_mul_small_input_a_limb_0,
        verify_mul_small_input_a_limb_1,
        verify_mul_small_input_a_limb_2,
        verify_mul_small_input_a_limb_3,
        verify_mul_small_input_b_limb_0,
        verify_mul_small_input_b_limb_1,
        verify_mul_small_input_b_limb_2,
        verify_mul_small_input_b_limb_3,
        verify_mul_small_input_c_limb_0,
        verify_mul_small_input_c_limb_1,
        verify_mul_small_input_c_limb_2,
        verify_mul_small_input_c_limb_3,
        verify_mul_small_input_c_limb_4,
        verify_mul_small_input_c_limb_5,
        verify_mul_small_input_c_limb_6,
        verify_mul_small_input_c_limb_7,
    ] =
        input;

    range_check_11_sum_0 =
        range_check_11_sum(range_check_11_alphas, range_check_11_z, [carry_1_col0]);

    // Constraint - carry 1 definition
    let constraint_quotient = (((carry_1_col0 * qm31_const::<262144, 0, 0, 0>())
        - (((((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_0)
            - verify_mul_small_input_c_limb_0)
            + ((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_1)
                * qm31_const::<512, 0, 0, 0>()))
            + ((verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_0)
                * qm31_const::<512, 0, 0, 0>()))
            - (verify_mul_small_input_c_limb_1 * qm31_const::<512, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_11_sum_1 =
        range_check_11_sum(range_check_11_alphas, range_check_11_z, [carry_3_col1]);

    // Constraint - carry 3 definition
    let constraint_quotient = (((carry_3_col1 * qm31_const::<262144, 0, 0, 0>())
        - (((((((((carry_1_col0
            + (verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_2))
            + (verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_1))
            + (verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_0))
            - verify_mul_small_input_c_limb_2)
            + ((verify_mul_small_input_a_limb_0 * verify_mul_small_input_b_limb_3)
                * qm31_const::<512, 0, 0, 0>()))
            + ((verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_2)
                * qm31_const::<512, 0, 0, 0>()))
            + ((verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_1)
                * qm31_const::<512, 0, 0, 0>()))
            + ((verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_0)
                * qm31_const::<512, 0, 0, 0>()))
            - (verify_mul_small_input_c_limb_3 * qm31_const::<512, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_11_sum_2 =
        range_check_11_sum(range_check_11_alphas, range_check_11_z, [carry_5_col2]);

    // Constraint - carry 5 definition
    let constraint_quotient = (((carry_5_col2 * qm31_const::<262144, 0, 0, 0>())
        - (((((((carry_3_col1 + (verify_mul_small_input_a_limb_1 * verify_mul_small_input_b_limb_3))
            + (verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_2))
            + (verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_1))
            - verify_mul_small_input_c_limb_4)
            + ((verify_mul_small_input_a_limb_2 * verify_mul_small_input_b_limb_3)
                * qm31_const::<512, 0, 0, 0>()))
            + ((verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_2)
                * qm31_const::<512, 0, 0, 0>()))
            - (verify_mul_small_input_c_limb_5 * qm31_const::<512, 0, 0, 0>()))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - final limb constraint
    let constraint_quotient = ((((carry_5_col2
        + (verify_mul_small_input_a_limb_3 * verify_mul_small_input_b_limb_3))
        - (verify_mul_small_input_c_limb_7 * qm31_const::<512, 0, 0, 0>()))
        - verify_mul_small_input_c_limb_6))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}

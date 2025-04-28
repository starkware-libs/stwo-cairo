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
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 2;


pub fn cond_decode_small_sign_evaluate(
    input: [QM31; 29],
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let [
        cond_decode_small_sign_input_limb_0,
        cond_decode_small_sign_input_limb_1,
        cond_decode_small_sign_input_limb_2,
        cond_decode_small_sign_input_limb_3,
        cond_decode_small_sign_input_limb_4,
        cond_decode_small_sign_input_limb_5,
        cond_decode_small_sign_input_limb_6,
        cond_decode_small_sign_input_limb_7,
        cond_decode_small_sign_input_limb_8,
        cond_decode_small_sign_input_limb_9,
        cond_decode_small_sign_input_limb_10,
        cond_decode_small_sign_input_limb_11,
        cond_decode_small_sign_input_limb_12,
        cond_decode_small_sign_input_limb_13,
        cond_decode_small_sign_input_limb_14,
        cond_decode_small_sign_input_limb_15,
        cond_decode_small_sign_input_limb_16,
        cond_decode_small_sign_input_limb_17,
        cond_decode_small_sign_input_limb_18,
        cond_decode_small_sign_input_limb_19,
        cond_decode_small_sign_input_limb_20,
        cond_decode_small_sign_input_limb_21,
        cond_decode_small_sign_input_limb_22,
        cond_decode_small_sign_input_limb_23,
        cond_decode_small_sign_input_limb_24,
        cond_decode_small_sign_input_limb_25,
        cond_decode_small_sign_input_limb_26,
        cond_decode_small_sign_input_limb_27,
        cond_decode_small_sign_input_limb_28,
    ] =
        input;

    // Constraint - msb is a bit
    let constraint_quotient = ((msb_col0 * (msb_col0 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - mid_limbs_set is a bit
    let constraint_quotient = ((mid_limbs_set_col1
        * (mid_limbs_set_col1 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Cannot have msb equals 0 and mid_limbs_set equals 1
    let constraint_quotient = (((cond_decode_small_sign_input_limb_28 * mid_limbs_set_col1)
        * (msb_col0 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [msb_col0, mid_limbs_set_col1]
}

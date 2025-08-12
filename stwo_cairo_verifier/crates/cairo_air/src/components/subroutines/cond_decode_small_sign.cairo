// AIR version e1943601-dirty
use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
    PreprocessedColumnSetImpl, LookupElementsImpl,
};
use stwo_verifier_core::circle::CirclePointQM31AddCirclePointM31Trait;
use stwo_verifier_core::circle::CirclePointIndexTrait;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{m31, M31};
use stwo_verifier_core::fields::qm31::{qm31_const, QM31, QM31Impl, QM31Serde, QM31Zero};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::cairo_component::CairoComponent;
use crate::PreprocessedColumnTrait;




pub fn cond_decode_small_sign_evaluate(
    input: QM31,
    msb_col0: QM31,mid_limbs_set_col1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let cond_decode_small_sign_input_limb_28 = input;
    

    // Constraint - msb is a bit
    let constraint_quotient = ((msb_col0 * (msb_col0 - qm31_const::<1, 0, 0, 0>()))) * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - mid_limbs_set is a bit
    let constraint_quotient = ((mid_limbs_set_col1 * (mid_limbs_set_col1 - qm31_const::<1, 0, 0, 0>()))) * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Cannot have msb equals 0 and mid_limbs_set equals 1
    let constraint_quotient = (((cond_decode_small_sign_input_limb_28 * mid_limbs_set_col1) * (msb_col0 - qm31_const::<1, 0, 0, 0>()))) * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}
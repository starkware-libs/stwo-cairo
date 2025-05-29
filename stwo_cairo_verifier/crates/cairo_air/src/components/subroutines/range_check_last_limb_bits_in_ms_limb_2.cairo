// Constraints version: 9330aaaf

use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Serde, QM31Zero, qm31_const};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;


pub fn range_check_last_limb_bits_in_ms_limb_2_evaluate(
    input: [QM31; 1],
    msb_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [range_check_last_limb_bits_in_ms_limb_2_input] = input;

    // Constraint - msb is a bit
    let constraint_quotient = ((msb_col0 * (qm31_const::<1, 0, 0, 0>() - msb_col0)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let bit_before_msb_tmp_f851f_1: QM31 = (range_check_last_limb_bits_in_ms_limb_2_input
        - (msb_col0 * qm31_const::<2, 0, 0, 0>()));

    // Constraint - bit before msb is a bit
    let constraint_quotient = ((bit_before_msb_tmp_f851f_1
        * (qm31_const::<1, 0, 0, 0>() - bit_before_msb_tmp_f851f_1)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}

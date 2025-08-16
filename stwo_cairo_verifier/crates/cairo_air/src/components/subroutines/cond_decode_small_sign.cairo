// AIR version 6ba93348
use crate::prelude::*;


pub fn cond_decode_small_sign_evaluate(
    input: QM31,
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let cond_decode_small_sign_input_limb_28 = input;

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

    []
}

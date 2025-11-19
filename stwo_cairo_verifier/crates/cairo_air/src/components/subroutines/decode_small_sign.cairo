// This file was created by the AIR team.

use crate::prelude::*;


pub fn decode_small_sign_evaluate(
    input: [QM31; 0],
    msb_col0: QM31,
    mid_limbs_set_col1: QM31,
    common_lookup_elements: @crate::CommonElements,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 4] {
    let [] = input;

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
    let constraint_quotient = ((mid_limbs_set_col1 * (msb_col0 - qm31_const::<1, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [
        (mid_limbs_set_col1 * qm31_const::<508, 0, 0, 0>()),
        (mid_limbs_set_col1 * qm31_const::<511, 0, 0, 0>()),
        ((msb_col0 * qm31_const::<136, 0, 0, 0>()) - mid_limbs_set_col1),
        (msb_col0 * qm31_const::<256, 0, 0, 0>()),
    ]
}

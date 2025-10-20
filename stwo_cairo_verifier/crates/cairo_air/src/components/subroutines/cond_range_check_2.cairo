// This file was created by the AIR team.

use crate::prelude::*;


pub fn cond_range_check_2_evaluate(
    input: [QM31; 2],
    partial_limb_msb_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [cond_range_check_2_input_limb_0, cond_range_check_2_input_limb_1] = input;

    // Constraint - msb is a bit or condition is 0
    let constraint_quotient = (((partial_limb_msb_col0
        * (qm31_const::<1, 0, 0, 0>() - partial_limb_msb_col0))
        * cond_range_check_2_input_limb_1))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let partial_limb_bit_before_msb_tmp_88401_1: QM31 = (cond_range_check_2_input_limb_0
        - (partial_limb_msb_col0 * qm31_const::<2, 0, 0, 0>()));

    // Constraint - bit before msb is a bit or condition is 0
    let constraint_quotient = (((partial_limb_bit_before_msb_tmp_88401_1
        * (qm31_const::<1, 0, 0, 0>() - partial_limb_bit_before_msb_tmp_88401_1))
        * cond_range_check_2_input_limb_1))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    []
}

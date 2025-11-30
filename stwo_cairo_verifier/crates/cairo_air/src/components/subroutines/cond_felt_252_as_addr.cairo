// This file was created by the AIR team.

use crate::components::subroutines::cond_range_check_2::cond_range_check_2_evaluate;
use crate::prelude::*;


pub fn cond_felt_252_as_addr_evaluate(
    input: [QM31; 29],
    partial_limb_msb_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> QM31 {
    let [
        cond_felt_252_as_addr_input_limb_0,
        cond_felt_252_as_addr_input_limb_1,
        cond_felt_252_as_addr_input_limb_2,
        cond_felt_252_as_addr_input_limb_3,
        cond_felt_252_as_addr_input_limb_4,
        cond_felt_252_as_addr_input_limb_5,
        cond_felt_252_as_addr_input_limb_6,
        cond_felt_252_as_addr_input_limb_7,
        cond_felt_252_as_addr_input_limb_8,
        cond_felt_252_as_addr_input_limb_9,
        cond_felt_252_as_addr_input_limb_10,
        cond_felt_252_as_addr_input_limb_11,
        cond_felt_252_as_addr_input_limb_12,
        cond_felt_252_as_addr_input_limb_13,
        cond_felt_252_as_addr_input_limb_14,
        cond_felt_252_as_addr_input_limb_15,
        cond_felt_252_as_addr_input_limb_16,
        cond_felt_252_as_addr_input_limb_17,
        cond_felt_252_as_addr_input_limb_18,
        cond_felt_252_as_addr_input_limb_19,
        cond_felt_252_as_addr_input_limb_20,
        cond_felt_252_as_addr_input_limb_21,
        cond_felt_252_as_addr_input_limb_22,
        cond_felt_252_as_addr_input_limb_23,
        cond_felt_252_as_addr_input_limb_24,
        cond_felt_252_as_addr_input_limb_25,
        cond_felt_252_as_addr_input_limb_26,
        cond_felt_252_as_addr_input_limb_27,
        cond_felt_252_as_addr_input_limb_28,
    ] =
        input;

    // Constraint - When the condition holds, the high limbs must be zero for an address
    let constraint_quotient = ((cond_felt_252_as_addr_input_limb_28
        * (((((((((((((((((((((((cond_felt_252_as_addr_input_limb_4
            + cond_felt_252_as_addr_input_limb_5)
            + cond_felt_252_as_addr_input_limb_6)
            + cond_felt_252_as_addr_input_limb_7)
            + cond_felt_252_as_addr_input_limb_8)
            + cond_felt_252_as_addr_input_limb_9)
            + cond_felt_252_as_addr_input_limb_10)
            + cond_felt_252_as_addr_input_limb_11)
            + cond_felt_252_as_addr_input_limb_12)
            + cond_felt_252_as_addr_input_limb_13)
            + cond_felt_252_as_addr_input_limb_14)
            + cond_felt_252_as_addr_input_limb_15)
            + cond_felt_252_as_addr_input_limb_16)
            + cond_felt_252_as_addr_input_limb_17)
            + cond_felt_252_as_addr_input_limb_18)
            + cond_felt_252_as_addr_input_limb_19)
            + cond_felt_252_as_addr_input_limb_20)
            + cond_felt_252_as_addr_input_limb_21)
            + cond_felt_252_as_addr_input_limb_22)
            + cond_felt_252_as_addr_input_limb_23)
            + cond_felt_252_as_addr_input_limb_24)
            + cond_felt_252_as_addr_input_limb_25)
            + cond_felt_252_as_addr_input_limb_26)
            + cond_felt_252_as_addr_input_limb_27)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    cond_range_check_2_evaluate(
        [cond_felt_252_as_addr_input_limb_3, cond_felt_252_as_addr_input_limb_28],
        partial_limb_msb_col0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    (((cond_felt_252_as_addr_input_limb_0
        + (cond_felt_252_as_addr_input_limb_1 * qm31_const::<512, 0, 0, 0>()))
        + (cond_felt_252_as_addr_input_limb_2 * qm31_const::<262144, 0, 0, 0>()))
        + (cond_felt_252_as_addr_input_limb_3 * qm31_const::<134217728, 0, 0, 0>()))
}

// This file was created by the AIR team.

use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::prelude::*;


pub fn mem_verify_cond_evaluate(
    input: [QM31; 29],
    cond_address_col0: QM31,
    cond_address_id_col1: QM31,
    cond_address_limb_0_col2: QM31,
    cond_address_limb_1_col3: QM31,
    cond_address_limb_2_col4: QM31,
    cond_address_limb_3_col5: QM31,
    cond_address_limb_4_col6: QM31,
    cond_address_limb_5_col7: QM31,
    cond_address_limb_6_col8: QM31,
    cond_address_limb_7_col9: QM31,
    cond_address_limb_8_col10: QM31,
    cond_address_limb_9_col11: QM31,
    cond_address_limb_10_col12: QM31,
    cond_address_limb_11_col13: QM31,
    cond_address_limb_12_col14: QM31,
    cond_address_limb_13_col15: QM31,
    cond_address_limb_14_col16: QM31,
    cond_address_limb_15_col17: QM31,
    cond_address_limb_16_col18: QM31,
    cond_address_limb_17_col19: QM31,
    cond_address_limb_18_col20: QM31,
    cond_address_limb_19_col21: QM31,
    cond_address_limb_20_col22: QM31,
    cond_address_limb_21_col23: QM31,
    cond_address_limb_22_col24: QM31,
    cond_address_limb_23_col25: QM31,
    cond_address_limb_24_col26: QM31,
    cond_address_limb_25_col27: QM31,
    cond_address_limb_26_col28: QM31,
    cond_address_limb_27_col29: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref memory_address_to_id_sum_0: QM31,
    ref numerator_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref numerator_1: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [
        mem_verify_cond_input_value_limb_0,
        mem_verify_cond_input_value_limb_1,
        mem_verify_cond_input_value_limb_2,
        mem_verify_cond_input_value_limb_3,
        mem_verify_cond_input_value_limb_4,
        mem_verify_cond_input_value_limb_5,
        mem_verify_cond_input_value_limb_6,
        mem_verify_cond_input_value_limb_7,
        mem_verify_cond_input_value_limb_8,
        mem_verify_cond_input_value_limb_9,
        mem_verify_cond_input_value_limb_10,
        mem_verify_cond_input_value_limb_11,
        mem_verify_cond_input_value_limb_12,
        mem_verify_cond_input_value_limb_13,
        mem_verify_cond_input_value_limb_14,
        mem_verify_cond_input_value_limb_15,
        mem_verify_cond_input_value_limb_16,
        mem_verify_cond_input_value_limb_17,
        mem_verify_cond_input_value_limb_18,
        mem_verify_cond_input_value_limb_19,
        mem_verify_cond_input_value_limb_20,
        mem_verify_cond_input_value_limb_21,
        mem_verify_cond_input_value_limb_22,
        mem_verify_cond_input_value_limb_23,
        mem_verify_cond_input_value_limb_24,
        mem_verify_cond_input_value_limb_25,
        mem_verify_cond_input_value_limb_26,
        mem_verify_cond_input_value_limb_27,
        mem_verify_cond_input_cond,
    ] =
        input;

    // Constraint - cond=0 or cond=1.
    let constraint_quotient = ((mem_verify_cond_input_cond
        * (qm31_const::<1, 0, 0, 0>() - mem_verify_cond_input_cond)));
    sum = sum * random_coeff + constraint_quotient;
    read_positive_num_bits_252_evaluate(
        cond_address_col0,
        cond_address_id_col1,
        cond_address_limb_0_col2,
        cond_address_limb_1_col3,
        cond_address_limb_2_col4,
        cond_address_limb_3_col5,
        cond_address_limb_4_col6,
        cond_address_limb_5_col7,
        cond_address_limb_6_col8,
        cond_address_limb_7_col9,
        cond_address_limb_8_col10,
        cond_address_limb_9_col11,
        cond_address_limb_10_col12,
        cond_address_limb_11_col13,
        cond_address_limb_12_col14,
        cond_address_limb_13_col15,
        cond_address_limb_14_col16,
        cond_address_limb_15_col17,
        cond_address_limb_16_col18,
        cond_address_limb_17_col19,
        cond_address_limb_18_col20,
        cond_address_limb_19_col21,
        cond_address_limb_20_col22,
        cond_address_limb_21_col23,
        cond_address_limb_22_col24,
        cond_address_limb_23_col25,
        cond_address_limb_24_col26,
        cond_address_limb_25_col27,
        cond_address_limb_26_col28,
        cond_address_limb_27_col29,
        common_lookup_elements,
        ref memory_address_to_id_sum_0,
        ref numerator_0,
        ref memory_id_to_big_sum_1,
        ref numerator_1,
        ref sum,
        random_coeff,
    );

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_0_col2 - mem_verify_cond_input_value_limb_0)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_1_col3 - mem_verify_cond_input_value_limb_1)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_2_col4 - mem_verify_cond_input_value_limb_2)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_3_col5 - mem_verify_cond_input_value_limb_3)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_4_col6 - mem_verify_cond_input_value_limb_4)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_5_col7 - mem_verify_cond_input_value_limb_5)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_6_col8 - mem_verify_cond_input_value_limb_6)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_7_col9 - mem_verify_cond_input_value_limb_7)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_8_col10 - mem_verify_cond_input_value_limb_8)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_9_col11 - mem_verify_cond_input_value_limb_9)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_10_col12 - mem_verify_cond_input_value_limb_10)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_11_col13 - mem_verify_cond_input_value_limb_11)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_12_col14 - mem_verify_cond_input_value_limb_12)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_13_col15 - mem_verify_cond_input_value_limb_13)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_14_col16 - mem_verify_cond_input_value_limb_14)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_15_col17 - mem_verify_cond_input_value_limb_15)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_16_col18 - mem_verify_cond_input_value_limb_16)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_17_col19 - mem_verify_cond_input_value_limb_17)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_18_col20 - mem_verify_cond_input_value_limb_18)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_19_col21 - mem_verify_cond_input_value_limb_19)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_20_col22 - mem_verify_cond_input_value_limb_20)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_21_col23 - mem_verify_cond_input_value_limb_21)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_22_col24 - mem_verify_cond_input_value_limb_22)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_23_col25 - mem_verify_cond_input_value_limb_23)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_24_col26 - mem_verify_cond_input_value_limb_24)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_25_col27 - mem_verify_cond_input_value_limb_25)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_26_col28 - mem_verify_cond_input_value_limb_26)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - felt252 limb is zero
    let constraint_quotient = (((cond_address_limb_27_col29 - mem_verify_cond_input_value_limb_27)
        * mem_verify_cond_input_cond));
    sum = sum * random_coeff + constraint_quotient;

    []
}

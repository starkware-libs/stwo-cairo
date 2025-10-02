// AIR version 98896da1
use crate::prelude::*;




pub fn bitwise_not_num_bits_16_evaluate(
    input: QM31,
    not_a_col0: QM31,verify_bitwise_not_16_lookup_elements: @crate::VerifyBitwiseNot_16Elements,ref verify_bitwise_not_16_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let bitwise_not_num_bits_16_input = input;
    

    verify_bitwise_not_16_sum_0 = verify_bitwise_not_16_lookup_elements.combine_qm31(
        [
            bitwise_not_num_bits_16_input,
not_a_col0
        ],
    );

    []
}
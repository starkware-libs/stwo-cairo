// AIR version 98896da1
use crate::prelude::*;




pub fn bitwise_and_num_bits_16_evaluate(
    input: [QM31; 2],
    and_col0: QM31,verify_bitwise_and_16_lookup_elements: @crate::VerifyBitwiseAnd_16Elements,ref verify_bitwise_and_16_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [bitwise_and_num_bits_16_input_limb_0, bitwise_and_num_bits_16_input_limb_1] = input;
    

    verify_bitwise_and_16_sum_0 = verify_bitwise_and_16_lookup_elements.combine_qm31(
        [
            bitwise_and_num_bits_16_input_limb_0,
bitwise_and_num_bits_16_input_limb_1,
and_col0
        ],
    );

    []
}
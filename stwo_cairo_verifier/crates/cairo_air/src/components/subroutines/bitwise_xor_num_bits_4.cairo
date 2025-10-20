// This file was created by the AIR team.

use crate::prelude::*;


pub fn bitwise_xor_num_bits_4_evaluate(
    input: [QM31; 2],
    xor_col0: QM31,
    verify_bitwise_xor_4_lookup_elements: @crate::VerifyBitwiseXor_4Elements,
    ref verify_bitwise_xor_4_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [bitwise_xor_num_bits_4_input_limb_0, bitwise_xor_num_bits_4_input_limb_1] = input;

    verify_bitwise_xor_4_sum_0 = verify_bitwise_xor_4_lookup_elements
        .combine_qm31(
            [bitwise_xor_num_bits_4_input_limb_0, bitwise_xor_num_bits_4_input_limb_1, xor_col0],
        );

    []
}

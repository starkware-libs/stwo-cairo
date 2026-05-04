// This file was created by the AIR team.

use crate::prelude::*;


pub fn bitwise_xor_num_bits_8_evaluate(
    input: [QM31; 2],
    xor_col0: QM31,
    common_lookup_elements: @CommonLookupElements,
    ref verify_bitwise_xor_8_sum_0: QM31,
    ref numerator_0: QM31,
    ref sum: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [bitwise_xor_num_bits_8_input_limb_0, bitwise_xor_num_bits_8_input_limb_1] = input;

    verify_bitwise_xor_8_sum_0 = common_lookup_elements
        .combine_qm31(
            [
                qm31_const::<112558620, 0, 0, 0>(), bitwise_xor_num_bits_8_input_limb_0,
                bitwise_xor_num_bits_8_input_limb_1, xor_col0,
            ]
                .span(),
        );
    numerator_0 = qm31_const::<1, 0, 0, 0>();

    []
}

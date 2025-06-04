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
use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
pub fn xor_rot_32_r_8_evaluate(
    input: [QM31; 4],
    ms_8_bits_col0: QM31,
    ms_8_bits_col1: QM31,
    ms_8_bits_col2: QM31,
    ms_8_bits_col3: QM31,
    xor_col4: QM31,
    xor_col5: QM31,
    xor_col6: QM31,
    xor_col7: QM31,
    verify_bitwise_xor_8_lookup_elements: @crate::VerifyBitwiseXor_8Elements,
    ref verify_bitwise_xor_8_sum_0: QM31,
    ref verify_bitwise_xor_8_sum_1: QM31,
    ref verify_bitwise_xor_8_sum_2: QM31,
    ref verify_bitwise_xor_8_sum_3: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let [
        xor_rot_32_r_8_input_limb_0,
        xor_rot_32_r_8_input_limb_1,
        xor_rot_32_r_8_input_limb_2,
        xor_rot_32_r_8_input_limb_3,
    ] =
        input;

    let output: [QM31; 1] = split_16_low_part_size_8_evaluate(
        [xor_rot_32_r_8_input_limb_0],
        ms_8_bits_col0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [split_16_low_part_size_8_output_tmp_aa6bd_1_limb_0] = output;

    let output: [QM31; 1] = split_16_low_part_size_8_evaluate(
        [xor_rot_32_r_8_input_limb_1],
        ms_8_bits_col1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [split_16_low_part_size_8_output_tmp_aa6bd_3_limb_0] = output;

    let output: [QM31; 1] = split_16_low_part_size_8_evaluate(
        [xor_rot_32_r_8_input_limb_2],
        ms_8_bits_col2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [split_16_low_part_size_8_output_tmp_aa6bd_5_limb_0] = output;

    let output: [QM31; 1] = split_16_low_part_size_8_evaluate(
        [xor_rot_32_r_8_input_limb_3],
        ms_8_bits_col3,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [split_16_low_part_size_8_output_tmp_aa6bd_7_limb_0] = output;

    bitwise_xor_num_bits_8_evaluate(
        [
            split_16_low_part_size_8_output_tmp_aa6bd_1_limb_0,
            split_16_low_part_size_8_output_tmp_aa6bd_5_limb_0,
        ],
        xor_col4,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_0,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    bitwise_xor_num_bits_8_evaluate(
        [ms_8_bits_col0, ms_8_bits_col2],
        xor_col5,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    bitwise_xor_num_bits_8_evaluate(
        [
            split_16_low_part_size_8_output_tmp_aa6bd_3_limb_0,
            split_16_low_part_size_8_output_tmp_aa6bd_7_limb_0,
        ],
        xor_col6,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    bitwise_xor_num_bits_8_evaluate(
        [ms_8_bits_col1, ms_8_bits_col3],
        xor_col7,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_3,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let xor_rot_8_output_tmp_aa6bd_16_limb_0: QM31 = (xor_col5
        + (xor_col6 * qm31_const::<256, 0, 0, 0>()));
    let xor_rot_8_output_tmp_aa6bd_16_limb_1: QM31 = (xor_col7
        + (xor_col4 * qm31_const::<256, 0, 0, 0>()));

    [xor_rot_8_output_tmp_aa6bd_16_limb_0, xor_rot_8_output_tmp_aa6bd_16_limb_1]
}

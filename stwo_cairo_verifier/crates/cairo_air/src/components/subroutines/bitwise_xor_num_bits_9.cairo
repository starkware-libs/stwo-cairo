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
use crate::cairo_component::CairoComponent;


pub fn bitwise_xor_num_bits_9_evaluate(
    input: [QM31; 2],
    xor_col0: QM31,
    verify_bitwise_xor_9_lookup_elements: @crate::VerifyBitwiseXor_9Elements,
    ref verify_bitwise_xor_9_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [bitwise_xor_num_bits_9_input_limb_0, bitwise_xor_num_bits_9_input_limb_1] = input;

    verify_bitwise_xor_9_sum_0 = verify_bitwise_xor_9_lookup_elements
        .combine_qm31(
            [bitwise_xor_num_bits_9_input_limb_0, bitwise_xor_num_bits_9_input_limb_1, xor_col0],
        );

    []
}

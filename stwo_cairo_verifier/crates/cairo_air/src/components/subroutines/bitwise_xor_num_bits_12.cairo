// Constraints version: 252b9d8a

use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::verify_bitwise_xor_12::{
    VERIFY_BITWISE_XOR_12_RELATION_SIZE, verify_bitwise_xor_12_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 1;


pub fn bitwise_xor_num_bits_12_evaluate(
    input: [QM31; 2],
    xor_col0: QM31,
    verify_bitwise_xor_12_alphas: Span<QM31>,
    verify_bitwise_xor_12_z: QM31,
    ref verify_bitwise_xor_12_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> QM31 {
    let [bitwise_xor_num_bits_12_input_limb_0, bitwise_xor_num_bits_12_input_limb_1] = input;

    verify_bitwise_xor_12_sum_0 =
        verify_bitwise_xor_12_sum(
            verify_bitwise_xor_12_alphas,
            verify_bitwise_xor_12_z,
            [bitwise_xor_num_bits_12_input_limb_0, bitwise_xor_num_bits_12_input_limb_1, xor_col0],
        );

    xor_col0
}

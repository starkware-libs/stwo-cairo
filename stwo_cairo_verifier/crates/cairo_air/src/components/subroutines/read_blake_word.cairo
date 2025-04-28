// Constraints version: 9c495569

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
use crate::components::memory_address_to_id::{
    MEMORY_ADDRESS_TO_ID_RELATION_SIZE, memory_address_to_id_sum,
};
use crate::components::memory_id_to_big::{MEMORY_ID_TO_BIG_RELATION_SIZE, memory_id_to_big_sum};
use crate::components::range_check_7_2_5::{RANGE_CHECK_7_2_5_RELATION_SIZE, range_check_7_2_5_sum};
use crate::components::subroutines::verify_blake_word::verify_blake_word_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 6;


pub fn read_blake_word_evaluate(
    input: [QM31; 1],
    low_16_bits_col0: QM31,
    high_16_bits_col1: QM31,
    low_7_ms_bits_col2: QM31,
    high_14_ms_bits_col3: QM31,
    high_5_ms_bits_col4: QM31,
    id_col5: QM31,
    range_check_7_2_5_alphas: Span<QM31>,
    range_check_7_2_5_z: QM31,
    memory_address_to_id_alphas: Span<QM31>,
    memory_address_to_id_z: QM31,
    memory_id_to_big_alphas: Span<QM31>,
    memory_id_to_big_z: QM31,
    ref range_check_7_2_5_sum_0: QM31,
    ref memory_address_to_id_sum_1: QM31,
    ref memory_id_to_big_sum_2: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [read_blake_word_input] = input;

    verify_blake_word_evaluate(
        [read_blake_word_input, low_16_bits_col0, high_16_bits_col1],
        low_7_ms_bits_col2,
        high_14_ms_bits_col3,
        high_5_ms_bits_col4,
        id_col5,
        range_check_7_2_5_alphas,
        range_check_7_2_5_z,
        memory_address_to_id_alphas,
        memory_address_to_id_z,
        memory_id_to_big_alphas,
        memory_id_to_big_z,
        ref range_check_7_2_5_sum_0,
        ref memory_address_to_id_sum_1,
        ref memory_id_to_big_sum_2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    []
}

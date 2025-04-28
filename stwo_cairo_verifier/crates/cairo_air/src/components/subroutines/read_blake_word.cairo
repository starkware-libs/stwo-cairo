// Constraints version: a82dac2f

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
use crate::components::subroutines::verify_blake_word::verify_blake_word_evaluate;
use crate::utils::U32Impl;


pub fn read_blake_word_evaluate(
    input: [QM31; 1],
    low_16_bits_col0: QM31,
    high_16_bits_col1: QM31,
    low_7_ms_bits_col2: QM31,
    high_14_ms_bits_col3: QM31,
    high_5_ms_bits_col4: QM31,
    id_col5: QM31,
    range_check_7_2_5_lookup_elements: @crate::RangeCheck_7_2_5Elements,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
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
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_0,
        ref memory_address_to_id_sum_1,
        ref memory_id_to_big_sum_2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    []
}

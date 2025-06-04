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
use crate::components::subroutines::read_blake_word::read_blake_word_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;

pub fn create_blake_round_input_evaluate(
    input: [QM31; 4],
    low_16_bits_col0: QM31,
    high_16_bits_col1: QM31,
    low_7_ms_bits_col2: QM31,
    high_14_ms_bits_col3: QM31,
    high_5_ms_bits_col4: QM31,
    state_0_id_col5: QM31,
    low_16_bits_col6: QM31,
    high_16_bits_col7: QM31,
    low_7_ms_bits_col8: QM31,
    high_14_ms_bits_col9: QM31,
    high_5_ms_bits_col10: QM31,
    state_1_id_col11: QM31,
    low_16_bits_col12: QM31,
    high_16_bits_col13: QM31,
    low_7_ms_bits_col14: QM31,
    high_14_ms_bits_col15: QM31,
    high_5_ms_bits_col16: QM31,
    state_2_id_col17: QM31,
    low_16_bits_col18: QM31,
    high_16_bits_col19: QM31,
    low_7_ms_bits_col20: QM31,
    high_14_ms_bits_col21: QM31,
    high_5_ms_bits_col22: QM31,
    state_3_id_col23: QM31,
    low_16_bits_col24: QM31,
    high_16_bits_col25: QM31,
    low_7_ms_bits_col26: QM31,
    high_14_ms_bits_col27: QM31,
    high_5_ms_bits_col28: QM31,
    state_4_id_col29: QM31,
    low_16_bits_col30: QM31,
    high_16_bits_col31: QM31,
    low_7_ms_bits_col32: QM31,
    high_14_ms_bits_col33: QM31,
    high_5_ms_bits_col34: QM31,
    state_5_id_col35: QM31,
    low_16_bits_col36: QM31,
    high_16_bits_col37: QM31,
    low_7_ms_bits_col38: QM31,
    high_14_ms_bits_col39: QM31,
    high_5_ms_bits_col40: QM31,
    state_6_id_col41: QM31,
    low_16_bits_col42: QM31,
    high_16_bits_col43: QM31,
    low_7_ms_bits_col44: QM31,
    high_14_ms_bits_col45: QM31,
    high_5_ms_bits_col46: QM31,
    state_7_id_col47: QM31,
    ms_8_bits_col48: QM31,
    ms_8_bits_col49: QM31,
    xor_col50: QM31,
    xor_col51: QM31,
    xor_col52: QM31,
    xor_col53: QM31,
    range_check_7_2_5_lookup_elements: @crate::RangeCheck_7_2_5Elements,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    verify_bitwise_xor_8_lookup_elements: @crate::VerifyBitwiseXor_8Elements,
    ref range_check_7_2_5_sum_0: QM31,
    ref memory_address_to_id_sum_1: QM31,
    ref memory_id_to_big_sum_2: QM31,
    ref range_check_7_2_5_sum_3: QM31,
    ref memory_address_to_id_sum_4: QM31,
    ref memory_id_to_big_sum_5: QM31,
    ref range_check_7_2_5_sum_6: QM31,
    ref memory_address_to_id_sum_7: QM31,
    ref memory_id_to_big_sum_8: QM31,
    ref range_check_7_2_5_sum_9: QM31,
    ref memory_address_to_id_sum_10: QM31,
    ref memory_id_to_big_sum_11: QM31,
    ref range_check_7_2_5_sum_12: QM31,
    ref memory_address_to_id_sum_13: QM31,
    ref memory_id_to_big_sum_14: QM31,
    ref range_check_7_2_5_sum_15: QM31,
    ref memory_address_to_id_sum_16: QM31,
    ref memory_id_to_big_sum_17: QM31,
    ref range_check_7_2_5_sum_18: QM31,
    ref memory_address_to_id_sum_19: QM31,
    ref memory_id_to_big_sum_20: QM31,
    ref range_check_7_2_5_sum_21: QM31,
    ref memory_address_to_id_sum_22: QM31,
    ref memory_id_to_big_sum_23: QM31,
    ref verify_bitwise_xor_8_sum_24: QM31,
    ref verify_bitwise_xor_8_sum_25: QM31,
    ref verify_bitwise_xor_8_sum_26: QM31,
    ref verify_bitwise_xor_8_sum_27: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 4] {
    let [
        create_blake_round_input_input_limb_0,
        create_blake_round_input_input_limb_1,
        create_blake_round_input_input_limb_2,
        create_blake_round_input_input_limb_3,
    ] =
        input;

    read_blake_word_evaluate(
        [create_blake_round_input_input_limb_0],
        low_16_bits_col0,
        high_16_bits_col1,
        low_7_ms_bits_col2,
        high_14_ms_bits_col3,
        high_5_ms_bits_col4,
        state_0_id_col5,
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

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<1, 0, 0, 0>())],
        low_16_bits_col6,
        high_16_bits_col7,
        low_7_ms_bits_col8,
        high_14_ms_bits_col9,
        high_5_ms_bits_col10,
        state_1_id_col11,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_3,
        ref memory_address_to_id_sum_4,
        ref memory_id_to_big_sum_5,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<2, 0, 0, 0>())],
        low_16_bits_col12,
        high_16_bits_col13,
        low_7_ms_bits_col14,
        high_14_ms_bits_col15,
        high_5_ms_bits_col16,
        state_2_id_col17,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_6,
        ref memory_address_to_id_sum_7,
        ref memory_id_to_big_sum_8,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<3, 0, 0, 0>())],
        low_16_bits_col18,
        high_16_bits_col19,
        low_7_ms_bits_col20,
        high_14_ms_bits_col21,
        high_5_ms_bits_col22,
        state_3_id_col23,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_9,
        ref memory_address_to_id_sum_10,
        ref memory_id_to_big_sum_11,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<4, 0, 0, 0>())],
        low_16_bits_col24,
        high_16_bits_col25,
        low_7_ms_bits_col26,
        high_14_ms_bits_col27,
        high_5_ms_bits_col28,
        state_4_id_col29,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_12,
        ref memory_address_to_id_sum_13,
        ref memory_id_to_big_sum_14,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<5, 0, 0, 0>())],
        low_16_bits_col30,
        high_16_bits_col31,
        low_7_ms_bits_col32,
        high_14_ms_bits_col33,
        high_5_ms_bits_col34,
        state_5_id_col35,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_15,
        ref memory_address_to_id_sum_16,
        ref memory_id_to_big_sum_17,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<6, 0, 0, 0>())],
        low_16_bits_col36,
        high_16_bits_col37,
        low_7_ms_bits_col38,
        high_14_ms_bits_col39,
        high_5_ms_bits_col40,
        state_6_id_col41,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_18,
        ref memory_address_to_id_sum_19,
        ref memory_id_to_big_sum_20,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    read_blake_word_evaluate(
        [(create_blake_round_input_input_limb_0 + qm31_const::<7, 0, 0, 0>())],
        low_16_bits_col42,
        high_16_bits_col43,
        low_7_ms_bits_col44,
        high_14_ms_bits_col45,
        high_5_ms_bits_col46,
        state_7_id_col47,
        range_check_7_2_5_lookup_elements,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref range_check_7_2_5_sum_21,
        ref memory_address_to_id_sum_22,
        ref memory_id_to_big_sum_23,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    let output: [QM31; 1] = split_16_low_part_size_8_evaluate(
        [create_blake_round_input_input_limb_1],
        ms_8_bits_col48,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [split_16_low_part_size_8_output_tmp_f95c3_73_limb_0] = output;

    let output: [QM31; 1] = split_16_low_part_size_8_evaluate(
        [create_blake_round_input_input_limb_2],
        ms_8_bits_col49,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );
    let [split_16_low_part_size_8_output_tmp_f95c3_75_limb_0] = output;

    bitwise_xor_num_bits_8_evaluate(
        [split_16_low_part_size_8_output_tmp_f95c3_73_limb_0, qm31_const::<127, 0, 0, 0>()],
        xor_col50,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_24,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    bitwise_xor_num_bits_8_evaluate(
        [ms_8_bits_col48, qm31_const::<82, 0, 0, 0>()],
        xor_col51,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_25,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    bitwise_xor_num_bits_8_evaluate(
        [split_16_low_part_size_8_output_tmp_f95c3_75_limb_0, qm31_const::<14, 0, 0, 0>()],
        xor_col52,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_26,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    bitwise_xor_num_bits_8_evaluate(
        [ms_8_bits_col49, qm31_const::<81, 0, 0, 0>()],
        xor_col53,
        verify_bitwise_xor_8_lookup_elements,
        ref verify_bitwise_xor_8_sum_27,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    [
        (xor_col50 + (xor_col51 * qm31_const::<256, 0, 0, 0>())),
        (xor_col52 + (xor_col53 * qm31_const::<256, 0, 0, 0>())),
        ((create_blake_round_input_input_limb_3 * qm31_const::<9812, 0, 0, 0>())
            + ((qm31_const::<1, 0, 0, 0>() - create_blake_round_input_input_limb_3)
                * qm31_const::<55723, 0, 0, 0>())),
        ((create_blake_round_input_input_limb_3 * qm31_const::<57468, 0, 0, 0>())
            + ((qm31_const::<1, 0, 0, 0>() - create_blake_round_input_input_limb_3)
                * qm31_const::<8067, 0, 0, 0>())),
    ]
}

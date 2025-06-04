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
use crate::components::subroutines::mem_verify::mem_verify_evaluate;

pub fn read_split_evaluate(
    input: [QM31; 1],
    value_limb_0_col0: QM31,
    value_limb_1_col1: QM31,
    value_limb_2_col2: QM31,
    value_limb_3_col3: QM31,
    value_limb_4_col4: QM31,
    value_limb_5_col5: QM31,
    value_limb_6_col6: QM31,
    value_limb_7_col7: QM31,
    value_limb_8_col8: QM31,
    value_limb_9_col9: QM31,
    value_limb_10_col10: QM31,
    value_limb_11_col11: QM31,
    value_limb_12_col12: QM31,
    value_limb_13_col13: QM31,
    value_limb_14_col14: QM31,
    value_limb_15_col15: QM31,
    value_limb_16_col16: QM31,
    value_limb_17_col17: QM31,
    value_limb_18_col18: QM31,
    value_limb_19_col19: QM31,
    value_limb_20_col20: QM31,
    value_limb_21_col21: QM31,
    value_limb_22_col22: QM31,
    value_limb_23_col23: QM31,
    value_limb_24_col24: QM31,
    value_limb_25_col25: QM31,
    value_limb_26_col26: QM31,
    ms_limb_low_col27: QM31,
    ms_limb_high_col28: QM31,
    id_col29: QM31,
    range_check_5_4_lookup_elements: @crate::RangeCheck_5_4Elements,
    memory_address_to_id_lookup_elements: @crate::MemoryAddressToIdElements,
    memory_id_to_big_lookup_elements: @crate::MemoryIdToBigElements,
    ref range_check_5_4_sum_0: QM31,
    ref memory_address_to_id_sum_1: QM31,
    ref memory_id_to_big_sum_2: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 1] {
    let [read_split_input_address] = input;

    range_check_5_4_sum_0 = range_check_5_4_lookup_elements
        .combine_qm31([ms_limb_low_col27, ms_limb_high_col28]);

    mem_verify_evaluate(
        [
            read_split_input_address, value_limb_0_col0, value_limb_1_col1, value_limb_2_col2,
            value_limb_3_col3, value_limb_4_col4, value_limb_5_col5, value_limb_6_col6,
            value_limb_7_col7, value_limb_8_col8, value_limb_9_col9, value_limb_10_col10,
            value_limb_11_col11, value_limb_12_col12, value_limb_13_col13, value_limb_14_col14,
            value_limb_15_col15, value_limb_16_col16, value_limb_17_col17, value_limb_18_col18,
            value_limb_19_col19, value_limb_20_col20, value_limb_21_col21, value_limb_22_col22,
            value_limb_23_col23, value_limb_24_col24, value_limb_25_col25, value_limb_26_col26,
            ((ms_limb_high_col28 * qm31_const::<32, 0, 0, 0>()) + ms_limb_low_col27),
        ],
        id_col29,
        memory_address_to_id_lookup_elements,
        memory_id_to_big_lookup_elements,
        ref memory_address_to_id_sum_1,
        ref memory_id_to_big_sum_2,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    [((ms_limb_high_col28 * qm31_const::<32, 0, 0, 0>()) + ms_limb_low_col27)]
}

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
use crate::components::memory_address_to_id::{
    MEMORY_ADDRESS_TO_ID_RELATION_SIZE, memory_address_to_id_sum,
};
use crate::components::memory_id_to_big::{MEMORY_ID_TO_BIG_RELATION_SIZE, memory_id_to_big_sum};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 29;


pub fn read_positive_num_bits_252_evaluate(
    input: QM31,
    id_col0: QM31,
    value_limb_0_col1: QM31,
    value_limb_1_col2: QM31,
    value_limb_2_col3: QM31,
    value_limb_3_col4: QM31,
    value_limb_4_col5: QM31,
    value_limb_5_col6: QM31,
    value_limb_6_col7: QM31,
    value_limb_7_col8: QM31,
    value_limb_8_col9: QM31,
    value_limb_9_col10: QM31,
    value_limb_10_col11: QM31,
    value_limb_11_col12: QM31,
    value_limb_12_col13: QM31,
    value_limb_13_col14: QM31,
    value_limb_14_col15: QM31,
    value_limb_15_col16: QM31,
    value_limb_16_col17: QM31,
    value_limb_17_col18: QM31,
    value_limb_18_col19: QM31,
    value_limb_19_col20: QM31,
    value_limb_20_col21: QM31,
    value_limb_21_col22: QM31,
    value_limb_22_col23: QM31,
    value_limb_23_col24: QM31,
    value_limb_24_col25: QM31,
    value_limb_25_col26: QM31,
    value_limb_26_col27: QM31,
    value_limb_27_col28: QM31,
    memory_address_to_id_alphas: Span<QM31>,
    memory_address_to_id_z: QM31,
    memory_id_to_big_alphas: Span<QM31>,
    memory_id_to_big_z: QM31,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 29] {
    let read_positive_num_bits_252_input = input;

    memory_address_to_id_sum_0 =
        memory_address_to_id_sum(
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            [read_positive_num_bits_252_input, id_col0],
        );

    memory_id_to_big_sum_1 =
        memory_id_to_big_sum(
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            [
                id_col0, value_limb_0_col1, value_limb_1_col2, value_limb_2_col3, value_limb_3_col4,
                value_limb_4_col5, value_limb_5_col6, value_limb_6_col7, value_limb_7_col8,
                value_limb_8_col9, value_limb_9_col10, value_limb_10_col11, value_limb_11_col12,
                value_limb_12_col13, value_limb_13_col14, value_limb_14_col15, value_limb_15_col16,
                value_limb_16_col17, value_limb_17_col18, value_limb_18_col19, value_limb_19_col20,
                value_limb_20_col21, value_limb_21_col22, value_limb_22_col23, value_limb_23_col24,
                value_limb_24_col25, value_limb_25_col26, value_limb_26_col27, value_limb_27_col28,
            ],
        );

    [
        value_limb_0_col1, value_limb_1_col2, value_limb_2_col3, value_limb_3_col4,
        value_limb_4_col5, value_limb_5_col6, value_limb_6_col7, value_limb_7_col8,
        value_limb_8_col9, value_limb_9_col10, value_limb_10_col11, value_limb_11_col12,
        value_limb_12_col13, value_limb_13_col14, value_limb_14_col15, value_limb_15_col16,
        value_limb_16_col17, value_limb_17_col18, value_limb_18_col19, value_limb_19_col20,
        value_limb_20_col21, value_limb_21_col22, value_limb_22_col23, value_limb_23_col24,
        value_limb_24_col25, value_limb_25_col26, value_limb_26_col27, value_limb_27_col28, id_col0,
    ]
}

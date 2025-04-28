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
use crate::components::range_check_4_4_4_4::{
    RANGE_CHECK_4_4_4_4_RELATION_SIZE, range_check_4_4_4_4_sum,
};
use crate::components::subroutines::read_positive_num_bits_144::read_positive_num_bits_144_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 19;


pub fn qm_31_read_reduced_evaluate(
    input: [QM31; 1],
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
    delta_ab_inv_col17: QM31,
    delta_cd_inv_col18: QM31,
    memory_address_to_id_alphas: Span<QM31>,
    memory_address_to_id_z: QM31,
    memory_id_to_big_alphas: Span<QM31>,
    memory_id_to_big_z: QM31,
    range_check_4_4_4_4_alphas: Span<QM31>,
    range_check_4_4_4_4_z: QM31,
    ref memory_address_to_id_sum_0: QM31,
    ref memory_id_to_big_sum_1: QM31,
    ref range_check_4_4_4_4_sum_2: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 4] {
    let [qm_31_read_reduced_input] = input;

    read_positive_num_bits_144_evaluate(
        [qm_31_read_reduced_input],
        id_col0,
        value_limb_0_col1,
        value_limb_1_col2,
        value_limb_2_col3,
        value_limb_3_col4,
        value_limb_4_col5,
        value_limb_5_col6,
        value_limb_6_col7,
        value_limb_7_col8,
        value_limb_8_col9,
        value_limb_9_col10,
        value_limb_10_col11,
        value_limb_11_col12,
        value_limb_12_col13,
        value_limb_13_col14,
        value_limb_14_col15,
        value_limb_15_col16,
        memory_address_to_id_alphas,
        memory_address_to_id_z,
        memory_id_to_big_alphas,
        memory_id_to_big_z,
        ref memory_address_to_id_sum_0,
        ref memory_id_to_big_sum_1,
        ref sum,
        domain_vanishing_eval_inv,
        random_coeff,
    );

    range_check_4_4_4_4_sum_2 =
        range_check_4_4_4_4_sum(
            range_check_4_4_4_4_alphas,
            range_check_4_4_4_4_z,
            [value_limb_3_col4, value_limb_7_col8, value_limb_11_col12, value_limb_15_col16],
        );

    // Constraint - delta_ab doesn't equal 0
    let constraint_quotient = ((((((((value_limb_0_col1 + value_limb_1_col2) + value_limb_2_col3)
        + value_limb_3_col4)
        - qm31_const::<1548, 0, 0, 0>())
        * ((((value_limb_4_col5 + value_limb_5_col6) + value_limb_6_col7) + value_limb_7_col8)
            - qm31_const::<1548, 0, 0, 0>()))
        * delta_ab_inv_col17)
        - qm31_const::<1, 0, 0, 0>()))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - delta_cd doesn't equal 0
    let constraint_quotient = ((((((((value_limb_8_col9 + value_limb_9_col10) + value_limb_10_col11)
        + value_limb_11_col12)
        - qm31_const::<1548, 0, 0, 0>())
        * ((((value_limb_12_col13 + value_limb_13_col14) + value_limb_14_col15)
            + value_limb_15_col16)
            - qm31_const::<1548, 0, 0, 0>()))
        * delta_cd_inv_col18)
        - qm31_const::<1, 0, 0, 0>()))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [
        (((value_limb_0_col1 + (value_limb_1_col2 * qm31_const::<512, 0, 0, 0>()))
            + (value_limb_2_col3 * qm31_const::<262144, 0, 0, 0>()))
            + (value_limb_3_col4 * qm31_const::<134217728, 0, 0, 0>())),
        (((value_limb_4_col5 + (value_limb_5_col6 * qm31_const::<512, 0, 0, 0>()))
            + (value_limb_6_col7 * qm31_const::<262144, 0, 0, 0>()))
            + (value_limb_7_col8 * qm31_const::<134217728, 0, 0, 0>())),
        (((value_limb_8_col9 + (value_limb_9_col10 * qm31_const::<512, 0, 0, 0>()))
            + (value_limb_10_col11 * qm31_const::<262144, 0, 0, 0>()))
            + (value_limb_11_col12 * qm31_const::<134217728, 0, 0, 0>())),
        (((value_limb_12_col13 + (value_limb_13_col14 * qm31_const::<512, 0, 0, 0>()))
            + (value_limb_14_col15 * qm31_const::<262144, 0, 0, 0>()))
            + (value_limb_15_col16 * qm31_const::<134217728, 0, 0, 0>())),
    ]
}

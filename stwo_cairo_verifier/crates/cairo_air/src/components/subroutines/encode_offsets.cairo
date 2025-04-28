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
use crate::components::range_check_4_3::{RANGE_CHECK_4_3_RELATION_SIZE, range_check_4_3_sum};
use crate::components::range_check_7_2_5::{RANGE_CHECK_7_2_5_RELATION_SIZE, range_check_7_2_5_sum};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 8;


pub fn encode_offsets_evaluate(
    input: [QM31; 3],
    offset0_low_col0: QM31,
    offset0_mid_col1: QM31,
    offset1_low_col2: QM31,
    offset1_mid_col3: QM31,
    offset1_high_col4: QM31,
    offset2_low_col5: QM31,
    offset2_mid_col6: QM31,
    offset2_high_col7: QM31,
    range_check_7_2_5_alphas: Span<QM31>,
    range_check_7_2_5_z: QM31,
    range_check_4_3_alphas: Span<QM31>,
    range_check_4_3_z: QM31,
    ref range_check_7_2_5_sum_0: QM31,
    ref range_check_4_3_sum_1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 6] {
    let [encode_offsets_input_offset0, encode_offsets_input_offset1, encode_offsets_input_offset2] =
        input;

    // Constraint - Reconstructed offset0 is correct
    let constraint_quotient = (((offset0_low_col0
        + (offset0_mid_col1 * qm31_const::<512, 0, 0, 0>()))
        - encode_offsets_input_offset0))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Reconstructed offset1 is correct
    let constraint_quotient = ((((offset1_low_col2
        + (offset1_mid_col3 * qm31_const::<4, 0, 0, 0>()))
        + (offset1_high_col4 * qm31_const::<2048, 0, 0, 0>()))
        - encode_offsets_input_offset1))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Reconstructed offset2 is correct
    let constraint_quotient = ((((offset2_low_col5
        + (offset2_mid_col6 * qm31_const::<16, 0, 0, 0>()))
        + (offset2_high_col7 * qm31_const::<8192, 0, 0, 0>()))
        - encode_offsets_input_offset2))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    range_check_7_2_5_sum_0 =
        range_check_7_2_5_sum(
            range_check_7_2_5_alphas,
            range_check_7_2_5_z,
            [offset0_mid_col1, offset1_low_col2, offset1_high_col4],
        );

    range_check_4_3_sum_1 =
        range_check_4_3_sum(
            range_check_4_3_alphas, range_check_4_3_z, [offset2_low_col5, offset2_high_col7],
        );

    [
        offset0_low_col0, (offset0_mid_col1 + (offset1_low_col2 * qm31_const::<128, 0, 0, 0>())),
        offset1_mid_col3, (offset1_high_col4 + (offset2_low_col5 * qm31_const::<32, 0, 0, 0>())),
        offset2_mid_col6, offset2_high_col7,
    ]
}

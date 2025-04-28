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
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 2;


pub fn triple_sum_32_evaluate(
    input: [QM31; 6],
    triple_sum32_res_limb_0_col0: QM31,
    triple_sum32_res_limb_1_col1: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 2] {
    let [
        triple_sum_32_input_limb_0,
        triple_sum_32_input_limb_1,
        triple_sum_32_input_limb_2,
        triple_sum_32_input_limb_3,
        triple_sum_32_input_limb_4,
        triple_sum_32_input_limb_5,
    ] =
        input;

    let carry_low_tmp_541fa_1: QM31 = ((((triple_sum_32_input_limb_0 + triple_sum_32_input_limb_2)
        + triple_sum_32_input_limb_4)
        - triple_sum32_res_limb_0_col0)
        * qm31_const::<32768, 0, 0, 0>());

    // Constraint - carry low is 0 or 1 or 2
    let constraint_quotient = (((carry_low_tmp_541fa_1
        * (carry_low_tmp_541fa_1 - qm31_const::<1, 0, 0, 0>()))
        * (carry_low_tmp_541fa_1 - qm31_const::<2, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
    let carry_high_tmp_541fa_2: QM31 = (((((triple_sum_32_input_limb_1 + triple_sum_32_input_limb_3)
        + triple_sum_32_input_limb_5)
        + carry_low_tmp_541fa_1)
        - triple_sum32_res_limb_1_col1)
        * qm31_const::<32768, 0, 0, 0>());

    // Constraint - carry high is 0 or 1 or 2
    let constraint_quotient = (((carry_high_tmp_541fa_2
        * (carry_high_tmp_541fa_2 - qm31_const::<1, 0, 0, 0>()))
        * (carry_high_tmp_541fa_2 - qm31_const::<2, 0, 0, 0>())))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    [triple_sum32_res_limb_0_col0, triple_sum32_res_limb_1_col1]
}

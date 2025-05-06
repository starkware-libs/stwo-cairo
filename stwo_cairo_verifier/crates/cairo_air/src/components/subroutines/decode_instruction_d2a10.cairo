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
use crate::components::verify_instruction::{
    VERIFY_INSTRUCTION_RELATION_SIZE, verify_instruction_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 3;


pub fn decode_instruction_d2a10_evaluate(
    input: QM31,
    offset2_col0: QM31,
    op1_imm_col1: QM31,
    op1_base_fp_col2: QM31,
    verify_instruction_alphas: Span<QM31>,
    verify_instruction_z: QM31,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 19] {
    let decode_instruction_d2a10_input_pc = input;

    // Constraint - Flag op1_imm is a bit
    let constraint_quotient = ((op1_imm_col1 * (qm31_const::<1, 0, 0, 0>() - op1_imm_col1)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op1_base_fp is a bit
    let constraint_quotient = ((op1_base_fp_col2 * (qm31_const::<1, 0, 0, 0>() - op1_base_fp_col2)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    // Constraint - Flag op1_base_ap is a bit
    let constraint_quotient = ((((qm31_const::<1, 0, 0, 0>() - op1_imm_col1) - op1_base_fp_col2)
        * (qm31_const::<1, 0, 0, 0>()
            - ((qm31_const::<1, 0, 0, 0>() - op1_imm_col1) - op1_base_fp_col2))))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    verify_instruction_sum_0 =
        verify_instruction_sum(
            verify_instruction_alphas,
            verify_instruction_z,
            [
                decode_instruction_d2a10_input_pc, qm31_const::<32767, 0, 0, 0>(),
                qm31_const::<32767, 0, 0, 0>(), offset2_col0,
                (((qm31_const::<24, 0, 0, 0>() + (op1_imm_col1 * qm31_const::<32, 0, 0, 0>()))
                    + (op1_base_fp_col2 * qm31_const::<64, 0, 0, 0>()))
                    + (((qm31_const::<1, 0, 0, 0>() - op1_imm_col1) - op1_base_fp_col2)
                        * qm31_const::<128, 0, 0, 0>())),
                qm31_const::<16, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            ],
        );

    [
        qm31_const::<2147483646, 0, 0, 0>(), qm31_const::<2147483646, 0, 0, 0>(),
        (offset2_col0 - qm31_const::<32768, 0, 0, 0>()), qm31_const::<1, 0, 0, 0>(),
        qm31_const::<1, 0, 0, 0>(), op1_imm_col1, op1_base_fp_col2,
        ((qm31_const::<1, 0, 0, 0>() - op1_imm_col1) - op1_base_fp_col2),
        qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
        qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<1, 0, 0, 0>(),
        qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
        qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
    ]
}

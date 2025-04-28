// Constraints version: cab09e9c

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
use crate::components::verify_instruction::VERIFY_INSTRUCTION_RELATION_SIZE;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 1;


pub fn decode_instruction_7ebc4_evaluate(
    input: [QM31; 1],
    ap_update_add_1_col0: QM31,
    verify_instruction_lookup_elements: @crate::VerifyInstructionElements,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [decode_instruction_7ebc4_input_pc] = input;

    // Constraint - Flag ap_update_add_1 is a bit
    let constraint_quotient = ((ap_update_add_1_col0
        * (qm31_const::<1, 0, 0, 0>() - ap_update_add_1_col0)))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    verify_instruction_sum_0 = verify_instruction_lookup_elements
        .combine_qm31(
            [
                decode_instruction_7ebc4_input_pc, qm31_const::<32767, 0, 0, 0>(),
                qm31_const::<32767, 0, 0, 0>(), qm31_const::<32769, 0, 0, 0>(),
                qm31_const::<56, 0, 0, 0>(),
                (qm31_const::<4, 0, 0, 0>() + (ap_update_add_1_col0 * qm31_const::<32, 0, 0, 0>())),
                qm31_const::<0, 0, 0, 0>(),
            ],
        );

    []
}

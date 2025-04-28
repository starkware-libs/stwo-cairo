// Constraints version: 9c495569

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
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::verify_instruction::VERIFY_INSTRUCTION_RELATION_SIZE;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 0;


pub fn decode_instruction_15a61_evaluate(
    input: [QM31; 1],
    verify_instruction_lookup_elements: @crate::VerifyInstructionElements,
    ref verify_instruction_sum_0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let [decode_instruction_15a61_input_pc] = input;

    verify_instruction_sum_0 = verify_instruction_lookup_elements
        .combine_qm31(
            [
                decode_instruction_15a61_input_pc, qm31_const::<32766, 0, 0, 0>(),
                qm31_const::<32767, 0, 0, 0>(), qm31_const::<32767, 0, 0, 0>(),
                qm31_const::<88, 0, 0, 0>(), qm31_const::<130, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(),
            ],
        );

    []
}

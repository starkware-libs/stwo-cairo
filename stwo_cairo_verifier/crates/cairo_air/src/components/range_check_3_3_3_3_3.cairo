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
use crate::components::{
    CairoComponent, OPCODES_RELATION_SIZE, RANGE_CHECK_3_3_3_3_3_LOG_SIZE, opcodes_sum,
};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = RANGE_CHECK_3_3_3_3_3_LOG_SIZE;
pub const RANGE_CHECK_3_3_3_3_3_RELATION_SIZE: usize = 5;


pub fn range_check_3_3_3_3_3_sum(mut alphas: Span<QM31>, z: QM31, values: [QM31; 5]) -> QM31 {
    let [alpha0, alpha1, alpha2, alpha3, alpha4] = (*alphas.multi_pop_front().unwrap()).unbox();
    let [val0, val1, val2, val3, val4] = values;

    alpha0 * val0 + alpha1 * val1 + alpha2 * val2 + alpha3 * val3 + alpha4 * val4 - z
}


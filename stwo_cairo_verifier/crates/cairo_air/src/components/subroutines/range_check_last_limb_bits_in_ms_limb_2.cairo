// AIR version c2e46f85
use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
    PreprocessedColumnSetImpl, LookupElementsImpl,
};
use stwo_verifier_core::circle::CirclePointQM31AddCirclePointM31Trait;
use stwo_verifier_core::circle::CirclePointIndexTrait;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{m31, M31};
use stwo_verifier_core::fields::qm31::{qm31_const, QM31, QM31Impl, QM31Serde, QM31Zero};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::cairo_component::CairoComponent;
use crate::PreprocessedColumnTrait;use crate::components::subroutines::cond_range_check_2::cond_range_check_2_evaluate;




pub fn range_check_last_limb_bits_in_ms_limb_2_evaluate(
    input: QM31,
    partial_limb_msb_col0: QM31,
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
) -> [QM31; 0] {
    let range_check_last_limb_bits_in_ms_limb_2_input = input;
    cond_range_check_2_evaluate(
            [range_check_last_limb_bits_in_ms_limb_2_input, qm31_const::<1, 0, 0, 0>()],
partial_limb_msb_col0,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

    []
}
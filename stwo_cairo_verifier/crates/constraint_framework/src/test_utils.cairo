//! Test helpers shared by the auto-generated AIR component tests (e.g. `cairo_air`,
//! `circuit_air`). The auto-generated test modules emit `use crate::test_utils::{...}`, so each
//! AIR crate re-exports these from here rather than duplicating them.
//!
//! Note: this module cannot be `#[cfg(test)]`-gated because `cfg(test)` only activates when
//! testing *this* crate, not when downstream crates compile their own tests.
use core::num::traits::Zero;
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl};
use stwo_verifier_core::utils::ArrayImpl;
use crate::{PreprocessedColumnIdx, PreprocessedMaskValues};

fn qm31_from_m31(m31: M31) -> QM31 {
    QM31Impl::from_fixed_array([m31, Zero::zero(), Zero::zero(), Zero::zero()])
}

pub fn make_interaction_trace(values: Array<QM31>, last_row_sum: QM31) -> ColumnSpan<Span<QM31>> {
    let last_row_sum_parts = last_row_sum.to_fixed_array().span();
    let mut result = array![];

    for i in 0..values.len() {
        let value_parts = values[i].to_fixed_array().span();
        let columns: Array<_> = if i < values.len() - 1 {
            (0..4_u32).into_iter().map(|j| [qm31_from_m31(*value_parts[j])].span()).collect()
        } else {
            (0..4_u32)
                .into_iter()
                .map(
                    |j| [qm31_from_m31(*last_row_sum_parts[j]), qm31_from_m31(*value_parts[j])]
                        .span(),
                )
                .collect()
        };
        result.append_span(columns.span());
    }
    result.span()
}

pub fn preprocessed_mask_add(
    mut mask_values: PreprocessedMaskValues, column_idx: PreprocessedColumnIdx, value: QM31,
) -> PreprocessedMaskValues {
    mask_values.values.insert(column_idx.into(), NullableTrait::new((value, false)));
    mask_values
}

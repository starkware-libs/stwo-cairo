use stwo_constraint_framework::PreprocessedMaskValues;
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use crate::CairoInteractionElements;

/// A component is a set of trace columns of the same sizes along with a set of constraints on them.
pub trait CairoComponent<T> {
    fn evaluate_constraints_at_point(
        self: @T,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    );
}

/// A trait for creating a new component.
pub trait NewComponent<T> {
    type Claim;
    type InteractionClaim;

    fn new(
        claim: @Self::Claim,
        interaction_claim: @Self::InteractionClaim,
        interaction_elements: @CairoInteractionElements,
    ) -> T;
}

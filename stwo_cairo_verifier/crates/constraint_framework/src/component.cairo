use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use crate::{CommonLookupElements, PreprocessedMaskValues};

/// A component is a set of trace columns of the same sizes along with a set of constraints on them.
pub trait AirComponent<T> {
    fn evaluate_constraints_at_point(
        self: @T,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        public_params: Span<u32>,
    );
}

/// A trait for creating a new component.
pub trait NewComponent<T> {
    fn new(log_size: @u32, claimed_sum: @QM31, common_lookup_elements: @CommonLookupElements) -> T;

    fn try_new(
        log_size: @Option<u32>,
        claimed_sum: @Option<QM31>,
        interaction_elements: @CommonLookupElements,
    ) -> Option<
        T,
    > {
        match (log_size, claimed_sum) {
            (
                Some(log_size), Some(claimed_sum),
            ) => Some(Self::new(log_size, claimed_sum, interaction_elements)),
            (None, None) => None,
            _ => panic!("inconsistent log_size and claimed_sum"),
        }
    }
}

use stwo_cairo_verifier::circle::CircleDomainImpl;
use stwo_cairo_verifier::evaluation::{CircleEvaluationQM31, BitReversedOrder};
use stwo_cairo_verifier::utils::pow;

/// Number of folds for univariate polynomials.
// TODO(andrew): Support different step sizes.
pub const FOLD_STEP: u32 = 1;
/// Number of folds when folding a circle polynomial to univariate polynomial.
pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;

/// Holds a foldable subset of circle polynomial evaluations.
#[derive(Debug, Drop, PartialEq)]
pub struct SparseCircleEvaluation {
    subcircle_evals: Array<CircleEvaluationQM31<BitReversedOrder>>,
}

#[generate_trait]
pub impl SparseCircleEvaluationImpl of SparseCircleEvaluationTrait {
    /// # Panics
    ///
    /// Panics if the evaluation domain sizes don't equal the folding factor.
    fn new(
        subcircle_evals: Array<CircleEvaluationQM31<BitReversedOrder>>
    ) -> SparseCircleEvaluation {
        let folding_factor = pow(2, CIRCLE_TO_LINE_FOLD_STEP);
        for eval in subcircle_evals.span() {
            assert!(eval.domain.size() == folding_factor);
        };
        SparseCircleEvaluation { subcircle_evals }
    }
}

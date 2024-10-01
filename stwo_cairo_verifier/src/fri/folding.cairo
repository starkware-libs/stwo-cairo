use stwo_cairo_verifier::fields::m31::M31Trait;
use stwo_cairo_verifier::circle::{Coset, CosetImpl};
use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
use stwo_cairo_verifier::poly::line::{LineDomain, LineDomainImpl};
use stwo_cairo_verifier::fields::qm31::{QM31, qm31, QM31Trait};
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::utils::{bit_reverse_index, pow};
use stwo_cairo_verifier::fri::verifier::{FOLD_STEP, CIRCLE_TO_LINE_FOLD_STEP};
use stwo_cairo_verifier::poly::circle::{
    CircleEvaluation, SparseCircleEvaluation, SparseCircleEvaluationImpl
};
use stwo_cairo_verifier::poly::line::{LineEvaluation, LineEvaluationImpl};
use stwo_cairo_verifier::fri::ibutterfly;

/// Folds and accumulates a degree `d` circle polynomial into a degree `d/2` univariate
/// polynomial.
///
/// Let `src` be the evaluation of a circle polynomial `f` on a
/// [`CircleDomain`] `E`. This function computes evaluations of `f' = f0
/// + alpha * f1` on the x-coordinates of `E` such that `2f(p) = f0(px) + py * f1(px)`. The
/// evaluations of `f'` are accumulated into `dst` by the formula `dst = dst * alpha^2 +
/// f'`.
pub fn fold_circle_into_line(eval: @CircleEvaluation, alpha: QM31) -> LineEvaluation {
    let domain = eval.domain;
    let mut values = array![];
    let mut i = 0;
    while i < eval.values.len() / 2 {
        let p = domain
            .at(bit_reverse_index(i * pow(2, CIRCLE_TO_LINE_FOLD_STEP), domain.log_size()));
        let f_p = eval.values[2 * i];
        let f_neg_p = eval.values[2 * i + 1];
        let (f0, f1) = ibutterfly(*f_p, *f_neg_p, p.y.inverse());
        values.append(f0 + alpha * f1);
        i += 1;
    };
    LineEvaluation { values, domain: LineDomainImpl::new(*domain.half_coset) }
}

#[cfg(test)]
mod test {
    use stwo_cairo_verifier::poly::line::{
        LineEvaluation, SparseLineEvaluation, SparseLineEvaluationImpl
    };
    use stwo_cairo_verifier::fields::m31::M31Trait;
    use stwo_cairo_verifier::circle::{Coset, CosetImpl};
    use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
    use stwo_cairo_verifier::poly::line::{LineDomain, LineDomainImpl};
    use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
    use stwo_cairo_verifier::fields::m31::M31;
    use stwo_cairo_verifier::utils::{bit_reverse_index, pow};
    use stwo_cairo_verifier::fri::verifier::{FOLD_STEP, CIRCLE_TO_LINE_FOLD_STEP};
    use stwo_cairo_verifier::poly::circle::{
        CircleEvaluation, SparseCircleEvaluation, SparseCircleEvaluationImpl
    };

    #[test]
    fn test_fold_circle_into_line_1() {
        let domain = CircleDomain { half_coset: CosetImpl::new(553648128, 0) };
        let values = array![qm31(807167738, 0, 0, 0), qm31(1359821401, 0, 0, 0)];
        let sparse_circle_evaluation: SparseCircleEvaluation = SparseCircleEvaluation {
            subcircle_evals: array![CircleEvaluation { values, domain }]
        };
        let alpha = qm31(260773061, 362745443, 1347591543, 1084609991);
        let result = sparse_circle_evaluation.fold(alpha);
        let expected_result = array![qm31(730692421, 1363821003, 2146256633, 106012305)];
        assert_eq!(expected_result, result);
    }
}

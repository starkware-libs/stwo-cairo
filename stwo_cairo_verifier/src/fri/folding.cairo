use stwo_cairo_verifier::fields::m31::M31Trait;
use stwo_cairo_verifier::circle::{Coset, CosetImpl};
use stwo_cairo_verifier::poly::line::{LineDomain, LineDomainImpl};
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::utils::{bit_reverse_index, pow};
use stwo_cairo_verifier::poly::line::{LineEvaluation, LineEvaluationImpl};
pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;
pub const FOLD_STEP: u32 = 1;

/// Folds a degree `d` polynomial into a degree `d/2` polynomial.
pub fn fold_line(eval: @LineEvaluation, alpha: QM31) -> LineEvaluation {
    let domain = eval.domain;
    let mut values: Array<QM31> = array![];
    let mut i = 0;
    while i < eval.values.len() / 2 {
        let x = domain.at(bit_reverse_index(i * pow(2, FOLD_STEP), domain.log_size()));
        let f_x = eval.values[2 * i];
        let f_neg_x = eval.values[2 * i + 1];
        let (f0, f1) = ibutterfly(*f_x, *f_neg_x, x.inverse());
        values.append(f0 + alpha * f1);
        i += 1;
    };
    LineEvaluationImpl::new(domain.double(), values)
}
pub fn ibutterfly(v0: QM31, v1: QM31, itwid: M31) -> (QM31, QM31) {
    (v0 + v1, (v0 - v1) * itwid.into())
}
mod test {
    use stwo_cairo_verifier::poly::line::{
        LineEvaluation, SparseLineEvaluation, SparseLineEvaluationImpl
    };
    use stwo_cairo_verifier::fields::m31::M31Trait;
    use stwo_cairo_verifier::circle::{Coset, CosetImpl};
    use stwo_cairo_verifier::poly::line::{LineDomain, LineDomainImpl};
    use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
    use stwo_cairo_verifier::fields::m31::M31;
    use stwo_cairo_verifier::utils::{bit_reverse_index, pow};
    use stwo_cairo_verifier::fri::folding::{FOLD_STEP, CIRCLE_TO_LINE_FOLD_STEP};
    #[test]
    fn test_fold_line_1() {
        let domain = LineDomainImpl::new(CosetImpl::new(67108864, 1));
        let values = array![
            qm31(440443058, 1252794525, 1129773609, 1309365757),
            qm31(974589897, 1592795796, 649052897, 863407657)
        ];
        let sparse_line_evaluation = SparseLineEvaluation {
            subline_evals: array![LineEvaluation { values, domain }]
        };
        let alpha = qm31(1047716961, 506143067, 1065078409, 990356067);
        let result = sparse_line_evaluation.fold(alpha);
        let expected_result = array![qm31(515899232, 1030391528, 1006544539, 11142505)];
        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_fold_line_2() {
        let domain = LineDomainImpl::new(CosetImpl::new(553648128, 1));
        let values = array![
            qm31(730692421, 1363821003, 2146256633, 106012305),
            qm31(1387266930, 149259209, 1148988082, 1930518101)
        ];
        let sparse_line_evaluation = SparseLineEvaluation {
            subline_evals: array![LineEvaluation { values, domain }]
        };
        let alpha = qm31(2084521793, 1326105747, 548635876, 1532708504);
        let result = sparse_line_evaluation.fold(alpha);
        let expected_result = array![qm31(1379727866, 1083096056, 1409020369, 1977903500)];
        assert_eq!(expected_result, result);
    }
}

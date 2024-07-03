use super::domain::{Coset, CosetImpl, LineDomain, CircleDomain, LineDomainImpl, dummy_line_domain};
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};

#[derive(Drop)]
pub struct LineEvaluation {
    pub values: Array<QM31>,
    pub domain: LineDomain
}

#[derive(Drop, Clone)]
pub struct CircleEvaluation {
    pub domain: CircleDomain,
    pub values: Array<QM31>
}

#[derive(Drop)]
pub struct SparseLineEvaluation {
    pub subline_evals: Array<LineEvaluation>,
}

#[derive(Drop, Clone)]
pub struct SparseCircleEvaluation {
    pub subcircle_evals: Array<CircleEvaluation>
}

#[generate_trait]
pub impl LineEvaluationImpl of LineEvaluationTrait {
    fn new(domain: LineDomain, values: Array<QM31>) -> LineEvaluation {
        // TODO: implement asserts
        LineEvaluation {
            values: values,
            domain: domain
        }
    }
}

#[generate_trait]
pub impl SparseCircleEvaluationImpl of SparseCircleEvaluationImplTrait {
    fn accumulate(self: @SparseCircleEvaluation, rhs: @SparseCircleEvaluation, alpha: QM31) -> SparseCircleEvaluation {
        // TODO: implement
        SparseCircleEvaluation {
            subcircle_evals: array![]
        }
    }

    fn fold(self: @SparseCircleEvaluation, alpha: QM31) -> Array<QM31> {
        // TODO: implement and remove clone in Queries
        array![qm31(0,0,0,0), qm31(0,0,0,0)]
    }
}
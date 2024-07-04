use stwo_cairo_verifier::fri::domain::CircleDomainTrait;
use stwo_cairo_verifier::fri::domain::CosetTrait;
use core::array::ArrayTrait;
use super::domain::{Coset, CosetImpl, LineDomain, CircleDomain, LineDomainImpl, dummy_line_domain};
use super::utils::bit_reverse_index;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};

#[derive(Drop)]
pub struct LineEvaluation {
    pub values: Array<QM31>,
    pub domain: LineDomain
}

#[derive(Debug, Drop, Clone, PartialEq, Eq)]
pub struct CircleEvaluation {
    pub domain: CircleDomain,
    pub values: Array<QM31>
}

#[derive(Drop)]
pub struct SparseLineEvaluation {
    pub subline_evals: Array<LineEvaluation>,
}

#[derive(Debug, Drop, Clone, PartialEq, Eq)]
pub struct SparseCircleEvaluation {
    pub subcircle_evals: Array<CircleEvaluation>
}

#[generate_trait]
pub impl LineEvaluationImpl of LineEvaluationTrait {
    fn new(domain: LineDomain, values: Array<QM31>) -> LineEvaluation {
        assert_eq!(values.len(), domain.size());
        LineEvaluation { values: values, domain: domain }
    }
}

#[generate_trait]
pub impl SparseCircleEvaluationImpl of SparseCircleEvaluationImplTrait {
    fn accumulate(
        self: @SparseCircleEvaluation, rhs: @SparseCircleEvaluation, alpha: QM31
    ) -> SparseCircleEvaluation {
        assert_eq!(self.subcircle_evals.len(), rhs.subcircle_evals.len());
        let mut subcircle_evals = array![];
        let mut i = 0;
        while i < self.subcircle_evals.len() {
            let lhs = self.subcircle_evals[i];
            let rhs = rhs.subcircle_evals[i];
            let mut values = array![];
            assert_eq!(lhs.values.len(), rhs.values.len());
            let mut j = 0;
            while j < lhs.values.len() {
                values.append(*lhs.values[j] * alpha + *rhs.values[j]);
                j += 1;
            };
            subcircle_evals.append(CircleEvaluation { domain: *lhs.domain, values });
            i += 1;
        };

        SparseCircleEvaluation { subcircle_evals }
    }

    fn fold(self: @SparseCircleEvaluation, alpha: QM31) -> Array<QM31> {
        // TODO: implement and remove clone in Queries
        array![qm31(0, 0, 0, 0), qm31(0, 0, 0, 0)]
    }
}

#[test]
fn test_accumulate_1() {
    let alpha = qm31(984186742, 463356387, 533637878, 1417871498);
    let lhs = SparseCircleEvaluation {
        subcircle_evals: array![
            CircleEvaluation {
                values: array![qm31(28672, 0, 0, 0), qm31(36864, 0, 0, 0)],
                domain: CircleDomain { half_coset: CosetImpl::new(16777216, 0) }
            },
            CircleEvaluation {
                values: array![qm31(28672, 0, 0, 0), qm31(36864, 0, 0, 0)],
                domain: CircleDomain { half_coset: CosetImpl::new(2030043136, 0) }
            },
            CircleEvaluation {
                values: array![qm31(2147454975, 0, 0, 0), qm31(2147446783, 0, 0, 0)],
                domain: CircleDomain { half_coset: CosetImpl::new(2097152000, 0) }
            },
        ]
    };
    let rhs = lhs.clone();
    let result = lhs.accumulate(@rhs, alpha);

    let expected_result = SparseCircleEvaluation {
        subcircle_evals: array![
            CircleEvaluation {
                values: array![
                    qm31(667173716, 1020487722, 1791736788, 1346152946),
                    qm31(1471361534, 84922130, 1076528072, 810417939)
                ],
                domain: CircleDomain { half_coset: CosetImpl::new(16777216, 0) }
            },
            CircleEvaluation {
                values: array![
                    qm31(667173716, 1020487722, 1791736788, 1346152946),
                    qm31(1471361534, 84922130, 1076528072, 810417939)
                ],
                domain: CircleDomain { half_coset: CosetImpl::new(2030043136, 0) }
            },
            CircleEvaluation {
                values: array![
                    qm31(1480309931, 1126995925, 355746859, 801330701),
                    qm31(676122113, 2062561517, 1070955575, 1337065708)
                ],
                domain: CircleDomain { half_coset: CosetImpl::new(2097152000, 0) }
            },
        ]
    };

    assert_eq!(expected_result, result);
}

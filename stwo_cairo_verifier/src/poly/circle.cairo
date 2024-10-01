use stwo_cairo_verifier::circle::CirclePointTrait;
use core::option::OptionTrait;
use core::clone::Clone;
use core::result::ResultTrait;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::utils::pow;
use stwo_cairo_verifier::circle::{
    Coset, CosetImpl, CirclePoint, CirclePointM31Impl, M31_CIRCLE_GEN, CIRCLE_ORDER
};
use stwo_cairo_verifier::fri::folding::fold_circle_into_line;

/// A valid domain for circle polynomial interpolation and evaluation.
/// Valid domains are a disjoint union of two conjugate cosets: `+-C + <G_n>`.
/// The ordering defined on this domain is `C + iG_n`, and then `-C - iG_n`.
#[derive(Debug, Copy, Drop, PartialEq, Eq)]
pub struct CircleDomain {
    pub half_coset: Coset
}

#[generate_trait]
pub impl CircleDomainImpl of CircleDomainTrait {
    /// Given a coset C + <G_n>, constructs the circle domain +-C + <G_n> (i.e.,
    /// this coset and its conjugate).
    fn new(half_coset: Coset) -> CircleDomain {
        CircleDomain { half_coset }
    }

    /// Returns the size of the domain.
    fn size(self: @CircleDomain) -> usize {
        pow(2, self.log_size())
    }

    /// Returns the log size of the domain.
    fn log_size(self: @CircleDomain) -> usize {
        *self.half_coset.log_size + 1
    }

    /// Returns the circle point index of the `i`th domain element.
    fn index_at(self: @CircleDomain, index: usize) -> usize {
        if index < self.half_coset.size() {
            self.half_coset.index_at(index)
        } else {
            CIRCLE_ORDER - self.half_coset.index_at(index - self.half_coset.size())
        }
    }

    /// Returns the `i` th domain element.
    fn at(self: @CircleDomain, index: usize) -> CirclePoint<M31> {
        M31_CIRCLE_GEN.mul(self.index_at(index).into())
    }

    fn new_with_log_size(log_size: u32) -> CircleDomain {
        CircleDomain { half_coset: CosetImpl::half_odds(log_size - 1) }
    }
}

/// An evaluation defined on a [CircleDomain].
/// The values are ordered according to the [CircleDomain] ordering.
#[derive(Debug, Drop, Clone, PartialEq, Eq)]
pub struct CircleEvaluation {
    pub values: Array<QM31>,
    pub domain: CircleDomain,
}

#[generate_trait]
pub impl CircleEvaluationImpl of CircleEvaluationTrait {
    fn new(domain: CircleDomain, values: Array<QM31>) -> CircleEvaluation {
        CircleEvaluation { values: values, domain: domain }
    }
}

/// Holds a foldable subset of circle polynomial evaluations.
#[derive(Drop, Clone, Debug, PartialEq)]
pub struct SparseCircleEvaluation {
    pub subcircle_evals: Array<CircleEvaluation>
}

#[generate_trait]
pub impl SparseCircleEvaluationImpl of SparseCircleEvaluationImplTrait {
    fn new(subcircle_evals: Array<CircleEvaluation>) -> SparseCircleEvaluation {
        SparseCircleEvaluation { subcircle_evals: subcircle_evals }
    }

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
        let mut i = 0;
        let mut res: Array<QM31> = array![];
        while i < self.subcircle_evals.len() {
            let circle_evaluation = fold_circle_into_line(self.subcircle_evals[i], alpha);
            res.append(*circle_evaluation.values.at(0));
            i += 1;
        };
        return res;
    }
}

#[test]
fn test_circle_domain_at_1() {
    let half_coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
    let domain = CircleDomain { half_coset };
    let index = 17;
    let result = domain.at(index);
    let expected_result = CirclePoint::<M31> { x: m31(7144319), y: m31(1742797653) };
    assert_eq!(expected_result, result);
}

#[test]
fn test_circle_domain_at_2() {
    let half_coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
    let domain = CircleDomain { half_coset };
    let index = 37;
    let result = domain.at(index);
    let expected_result = CirclePoint::<M31> { x: m31(9803698), y: m31(2079025011) };
    assert_eq!(expected_result, result);
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


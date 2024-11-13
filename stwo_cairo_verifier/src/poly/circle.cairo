use stwo_cairo_verifier::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointM31Impl, CirclePointTrait,
    Coset, CosetImpl,
};
use stwo_cairo_verifier::fields::FieldBatchInverse;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::fields::qm31::QM31;
use stwo_cairo_verifier::poly::utils::ibutterfly;
use stwo_cairo_verifier::utils::pow;

/// A valid domain for circle polynomial interpolation and evaluation.
///
/// Valid domains are a disjoint union of two conjugate cosets: `+-C + <G_n>`.
/// The ordering defined on this domain is `C + iG_n`, and then `-C - iG_n`.
#[derive(Debug, Copy, Drop, PartialEq)]
pub struct CircleDomain {
    pub half_coset: Coset,
}

#[generate_trait]
pub impl CircleDomainImpl of CircleDomainTrait {
    /// Given a coset `C + <G_n>`, constructs the circle domain `+-C + <G_n>` (i.e.
    /// this coset and its conjugate).
    fn new(half_coset: Coset) -> CircleDomain {
        CircleDomain { half_coset }
    }

    /// Returns the size of the domain.
    #[inline]
    fn size(self: @CircleDomain) -> usize {
        pow(2, self.log_size())
    }

    /// Returns the log size of the domain.
    #[inline]
    fn log_size(self: @CircleDomain) -> usize {
        *self.half_coset.log_size + 1
    }

    /// Returns the [`CirclePointIndex`] of the `i`th domain element.
    fn index_at(self: @CircleDomain, index: usize) -> CirclePointIndex {
        if index < self.half_coset.size() {
            self.half_coset.index_at(index)
        } else {
            -self.half_coset.index_at(index - self.half_coset.size())
        }
    }

    #[inline]
    fn at(self: @CircleDomain, index: usize) -> CirclePoint<M31> {
        self.index_at(index).to_point()
    }
}

/// A coset of the form `G_{2n} + <G_n>`, where `G_n` is the generator of the subgroup of order `n`.
///
/// The ordering on this coset is `G_{2n} + i * G_n`. These cosets can be used as a
/// [`CircleDomain`], and be interpolated on. Note that this changes the ordering on the coset to be
/// like [`CircleDomain`], which is `G_{2n} + i * G_{n/2}` and then `-G_{2n} -i * G_{n/2}`. For
/// example, the `X`s below are a canonic coset with `n = 8`.
///
/// ```text
///    X O X
///  O       O
/// X         X
/// O         O
/// X         X
///  O       O
///    X O X
/// ```
#[derive(Copy, Debug, PartialEq, Drop)]
pub struct CanonicCoset {
    pub coset: Coset,
}

#[generate_trait]
pub impl CanonicCosetImpl of CanonicCosetTrait {
    fn new(log_size: u32) -> CanonicCoset {
        assert!(log_size > 0);
        CanonicCoset { coset: CosetImpl::odds(log_size) }
    }

    /// Gets the full coset represented `G_{2n} + <G_n>`.
    fn coset(self: @CanonicCoset) -> @Coset {
        self.coset
    }

    /// Gets half of the coset (its conjugate complements to the whole coset), `G_{2n} + <G_{n/2}>`.
    fn half_coset(self: @CanonicCoset) -> Coset {
        assert!(*self.coset.log_size > 0);
        Coset {
            initial_index: *self.coset.initial_index,
            step_size: *self.coset.step_size + *self.coset.step_size,
            log_size: *self.coset.log_size - 1,
        }
    }

    /// Gets the [`CircleDomain`] representing the same point set (in another order).
    fn circle_domain(self: @CanonicCoset) -> CircleDomain {
        CircleDomainImpl::new(self.half_coset())
    }

    /// Evaluates the coset's vanishing polynomial at point `p`.
    fn eval_vanishing(self: @CanonicCoset, p: CirclePoint<QM31>) -> QM31 {
        let mut x = p.x;

        // The formula for the x coordinate of the double of a point.
        for _ in 1..*self.coset.log_size {
            x = CirclePointTrait::double_x(x);
        };

        x
    }
}

/// An evaluation defined on a [`CircleDomain`].
///
/// The values are ordered according to the [`CircleDomain`] ordering.
#[derive(Debug, Drop, Clone, PartialEq)]
pub struct CircleEvaluation {
    pub bit_reversed_values: Array<QM31>,
    pub domain: CircleDomain,
}

#[generate_trait]
pub impl CircleEvaluationImpl of CircleEvaluationTrait {
    fn new(domain: CircleDomain, bit_reversed_values: Array<QM31>) -> CircleEvaluation {
        CircleEvaluation { bit_reversed_values, domain }
    }
}

/// Holds a foldable subset of circle polynomial evaluations.
#[derive(Drop, Clone, Debug, PartialEq)]
pub struct SparseCircleEvaluation {
    pub subcircle_evals: Array<CircleEvaluation>,
}

#[generate_trait]
pub impl SparseCircleEvaluationImpl of SparseCircleEvaluationImplTrait {
    fn new(subcircle_evals: Array<CircleEvaluation>) -> SparseCircleEvaluation {
        SparseCircleEvaluation { subcircle_evals: subcircle_evals }
    }

    fn accumulate(
        self: @SparseCircleEvaluation, rhs: @SparseCircleEvaluation, alpha: QM31,
    ) -> SparseCircleEvaluation {
        assert!(self.subcircle_evals.len() == rhs.subcircle_evals.len());
        let mut subcircle_evals = array![];
        let mut i = 0;
        while i < self.subcircle_evals.len() {
            let lhs = self.subcircle_evals[i];
            let rhs = rhs.subcircle_evals[i];
            let mut values = array![];
            assert!(lhs.bit_reversed_values.len() == rhs.bit_reversed_values.len());
            let mut j = 0;
            while j < lhs.bit_reversed_values.len() {
                values.append(*lhs.bit_reversed_values[j] * alpha + *rhs.bit_reversed_values[j]);
                j += 1;
            };
            subcircle_evals.append(CircleEvaluationImpl::new(*lhs.domain, values));
            i += 1;
        };

        SparseCircleEvaluation { subcircle_evals }
    }

    /// Folds evaluations of a degree `d` circle polynomial into evaluations of a
    /// degree `d/2` univariate polynomial.
    fn fold(self: SparseCircleEvaluation, alpha: QM31) -> Array<QM31> {
        let mut domain_initial_ys = array![];

        for eval in self.subcircle_evals.span() {
            domain_initial_ys.append(eval.domain.at(0).y);
        };

        let mut domain_initial_ys_inv = FieldBatchInverse::batch_inverse(domain_initial_ys);
        let mut res = array![];

        for eval in self.subcircle_evals {
            let y_inv = domain_initial_ys_inv.pop_front().unwrap();
            let f_at_p = *eval.bit_reversed_values[0];
            let f_at_neg_p = *eval.bit_reversed_values[1];
            let (f0, f1) = ibutterfly(f_at_p, f_at_neg_p, y_inv);
            res.append(f0 + alpha * f1);
        };

        res
    }
}

#[cfg(test)]
mod tests {
    use stwo_cairo_verifier::circle::{CirclePoint, CirclePointIndexImpl, Coset, CosetImpl};
    use stwo_cairo_verifier::fields::m31::m31;
    use stwo_cairo_verifier::fields::qm31::qm31;
    use super::{
        CircleDomain, CircleDomainTrait, CircleEvaluationImpl, SparseCircleEvaluation,
        SparseCircleEvaluationImplTrait,
    };

    #[test]
    fn test_circle_domain_at_1() {
        let half_coset = Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            step_size: CirclePointIndexImpl::new(67108864),
            log_size: 5,
        };
        let domain = CircleDomain { half_coset };
        let index = 17;
        let result = domain.at(index);

        assert_eq!(result, CirclePoint { x: m31(7144319), y: m31(1742797653) });
    }

    #[test]
    fn test_circle_domain_at_2() {
        let half_coset = Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            step_size: CirclePointIndexImpl::new(67108864),
            log_size: 5,
        };
        let domain = CircleDomain { half_coset };
        let index = 37;
        let result = domain.at(index);

        assert_eq!(result, CirclePoint { x: m31(9803698), y: m31(2079025011) });
    }

    #[test]
    fn test_accumulate_1() {
        let alpha = qm31(984186742, 463356387, 533637878, 1417871498);
        let lhs = SparseCircleEvaluation {
            subcircle_evals: array![
                CircleEvaluationImpl::new(
                    CircleDomain {
                        half_coset: CosetImpl::new(CirclePointIndexImpl::new(16777216), 0),
                    },
                    array![qm31(28672, 0, 0, 0), qm31(36864, 0, 0, 0)],
                ),
                CircleEvaluationImpl::new(
                    CircleDomain {
                        half_coset: CosetImpl::new(CirclePointIndexImpl::new(2030043136), 0),
                    },
                    array![qm31(28672, 0, 0, 0), qm31(36864, 0, 0, 0)],
                ),
                CircleEvaluationImpl::new(
                    CircleDomain {
                        half_coset: CosetImpl::new(CirclePointIndexImpl::new(2097152000), 0),
                    },
                    array![qm31(2147454975, 0, 0, 0), qm31(2147446783, 0, 0, 0)],
                ),
            ],
        };
        let rhs = lhs.clone();
        let result = lhs.accumulate(@rhs, alpha);

        assert_eq!(
            result,
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: CosetImpl::new(CirclePointIndexImpl::new(16777216), 0),
                        },
                        array![
                            qm31(667173716, 1020487722, 1791736788, 1346152946),
                            qm31(1471361534, 84922130, 1076528072, 810417939),
                        ],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: CosetImpl::new(CirclePointIndexImpl::new(2030043136), 0),
                        },
                        array![
                            qm31(667173716, 1020487722, 1791736788, 1346152946),
                            qm31(1471361534, 84922130, 1076528072, 810417939),
                        ],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: CosetImpl::new(CirclePointIndexImpl::new(2097152000), 0),
                        },
                        array![
                            qm31(1480309931, 1126995925, 355746859, 801330701),
                            qm31(676122113, 2062561517, 1070955575, 1337065708),
                        ],
                    ),
                ],
            },
        );
    }
}

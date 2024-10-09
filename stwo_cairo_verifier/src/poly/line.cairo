use core::option::OptionTrait;
use core::clone::Clone;
use core::result::ResultTrait;
use stwo_cairo_verifier::poly::utils::fold;
use stwo_cairo_verifier::fields::SecureField;
use stwo_cairo_verifier::fields::m31::{M31, m31, M31Trait};
use stwo_cairo_verifier::fields::qm31::{QM31, qm31, QM31Zero};
use stwo_cairo_verifier::utils::pow;
use stwo_cairo_verifier::circle::{Coset, CosetImpl, CirclePointTrait, M31_CIRCLE_GEN};
use stwo_cairo_verifier::fri::fold_line;

/// A univariate polynomial defined on a [LineDomain].
#[derive(Debug, Drop, Clone)]
pub struct LinePoly {
    /// The coefficients of the polynomial stored in bit-reversed order.
    ///
    /// The coefficients are in a basis relating to the circle's x-coordinate doubling
    /// map `pi(x) = 2x^2 - 1` i.e.
    ///
    /// ```text
    /// B = { 1 } * { x } * { pi(x) } * { pi(pi(x)) } * ...
    ///   = { 1, x, pi(x), pi(x) * x, pi(pi(x)), pi(pi(x)) * x, pi(pi(x)) * pi(x), ... }
    /// ```
    pub coeffs: Array<SecureField>,
    /// The number of coefficients stored as `log2(len(coeffs))`.
    pub log_size: u32,
}

#[generate_trait]
pub impl LinePolyImpl of LinePolyTrait {
    /// Returns the number of coefficients.
    fn len(self: @LinePoly) -> usize {
        self.coeffs.len()
    }

    /// Evaluates the polynomial at a single point.
    fn eval_at_point(self: @LinePoly, mut x: SecureField) -> SecureField {
        let mut doublings = array![];
        let mut i = 0;
        while i < *self.log_size {
            doublings.append(x);
            let x_square = x * x;
            x = x_square + x_square - m31(1).into();
            i += 1;
        };

        fold(self.coeffs, @doublings, 0, 0, self.coeffs.len())
    }
}

/// Domain comprising of the x-coordinates of points in a [Coset].
///
/// For use with univariate polynomials.
#[derive(Copy, Drop, Debug)]
pub struct LineDomain {
    pub coset: Coset,
}

#[generate_trait]
pub impl LineDomainImpl of LineDomainTrait {
    /// Returns a domain comprising of the x-coordinates of points in a coset.
    fn new(coset: Coset) -> LineDomain {
        let coset_size = coset.size();
        if (coset_size == 2) {
            // If the coset with two points contains (0, y) then the coset is {(0, y), (0, -y)}.
            assert!(!coset.at(0).x.is_zero(), "coset x-coordinates not unique");
        } else if (coset_size > 2) {
            // Let our coset be `E = c + <G>` with `|E| > 2` then:
            // 1. if `ord(c) <= ord(G)` the coset contains two points at x=0
            // 2. if `ord(c) = 2 * ord(G)` then `c` and `-c` are in our coset
            let mut scalar = coset.step_size.into();
            let coset_step = M31_CIRCLE_GEN.mul(ref scalar);
            assert!(
                coset.at(0).log_order() >= coset_step.log_order() + 2,
                "coset x-coordinates not unique"
            );
        }
        LineDomain { coset: coset }
    }

    /// Returns the `i`th domain element.
    fn at(self: @LineDomain, index: usize) -> M31 {
        self.coset.at(index).x
    }

    /// Returns the size of the domain.
    fn size(self: @LineDomain) -> usize {
        self.coset.size()
    }

    /// Returns the log size of the domain.
    fn log_size(self: @LineDomain) -> usize {
        *self.coset.log_size
    }

    /// Returns a new domain comprising of all points in current domain doubled.
    fn double(self: @LineDomain) -> LineDomain {
        LineDomain { coset: self.coset.double() }
    }
}

/// Evaluations of a univariate polynomial on a [LineDomain].
#[derive(Drop)]
pub struct LineEvaluation {
    pub values: Array<QM31>,
    pub domain: LineDomain
}

#[generate_trait]
pub impl LineEvaluationImpl of LineEvaluationTrait {
    /// Creates new [LineEvaluation] from a set of polynomial evaluations over a [LineDomain].
    fn new(domain: LineDomain, values: Array<QM31>) -> LineEvaluation {
        assert_eq!(values.len(), domain.size());
        LineEvaluation { values: values, domain: domain }
    }
}

/// Holds a small foldable subset of univariate SecureField polynomial evaluations.
#[derive(Drop)]
pub struct SparseLineEvaluation {
    pub subline_evals: Array<LineEvaluation>,
}

#[generate_trait]
pub impl SparseLineEvaluationImpl of SparseLineEvaluationTrait {
    fn fold(self: @SparseLineEvaluation, alpha: QM31) -> Array<QM31> {
        let mut i = 0;
        let mut res: Array<QM31> = array![];
        while i < self.subline_evals.len() {
            let line_evaluation = fold_line(self.subline_evals[i], alpha);
            res.append(*line_evaluation.values.at(0));
            i += 1;
        };
        res
    }
}


#[cfg(test)]
mod tests {
    use super::{LinePoly, LinePolyTrait, LineDomain, LineDomainImpl};
    use stwo_cairo_verifier::fields::qm31::qm31;
    use stwo_cairo_verifier::fields::m31::m31;
    use stwo_cairo_verifier::circle::{Coset, CosetImpl, CIRCLE_LOG_ORDER};
    use stwo_cairo_verifier::utils::pow;

    #[test]
    #[should_panic]
    fn bad_line_domain() {
        // This coset doesn't have points with unique x-coordinates.
        let LOG_SIZE = 2;
        let initial_index = pow(2, CIRCLE_LOG_ORDER - (LOG_SIZE + 1));
        let coset = CosetImpl::new(initial_index, LOG_SIZE);

        LineDomainImpl::new(coset);
    }

    #[test]
    fn line_domain_of_size_two_works() {
        let LOG_SIZE: u32 = 1;
        let coset = CosetImpl::new(0, LOG_SIZE);

        LineDomainImpl::new(coset);
    }

    #[test]
    fn line_domain_of_size_one_works() {
        let LOG_SIZE: u32 = 0;
        let coset = CosetImpl::new(0, LOG_SIZE);

        LineDomainImpl::new(coset);
    }

    #[test]
    fn test_eval_at_point_1() {
        let line_poly = LinePoly {
            coeffs: array![
                qm31(1080276375, 1725024947, 477465525, 102017026),
                qm31(1080276375, 1725024947, 477465525, 102017026)
            ],
            log_size: 1
        };
        let x = m31(590768354);
        let result = line_poly.eval_at_point(x.into());
        let expected_result = qm31(515899232, 1030391528, 1006544539, 11142505);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_eval_at_point_2() {
        let line_poly = LinePoly {
            coeffs: array![qm31(1, 2, 3, 4), qm31(5, 6, 7, 8)], log_size: 1
        };
        let x = m31(10);
        let result = line_poly.eval_at_point(x.into());
        let expected_result = qm31(51, 62, 73, 84);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_eval_at_point_3() {
        let poly = LinePoly {
            coeffs: array![
                qm31(1, 2, 3, 4),
                qm31(5, 6, 7, 8),
                qm31(9, 10, 11, 12),
                qm31(13, 14, 15, 16),
                qm31(17, 18, 19, 20),
                qm31(21, 22, 23, 24),
                qm31(25, 26, 27, 28),
                qm31(29, 30, 31, 32),
            ],
            log_size: 3
        };
        let x = qm31(2, 5, 7, 11);

        let result = poly.eval_at_point(x);

        let expected_result = qm31(1857853974, 839310133, 939318020, 651207981);
        assert_eq!(expected_result, result);
    }
}

use core::iter::{IntoIterator, Iterator};
use core::num::traits::Zero;
use crate::circle::{CirclePointIndexImpl, CirclePointTrait, Coset, CosetImpl};
use crate::fields::m31::{M31, MulByM31Trait, m31};
use crate::fields::qm31::{QM31, qm31_const};
use crate::fields::{BaseField, SecureField};
use crate::poly::line::{
    LineDomain, LineDomainImpl, LineDomainIterator, LineEvaluationImpl, LinePoly,
};
use crate::utils::pow2;
use super::line::LineEvaluation;
use super::utils::butterfly;


#[generate_trait]
pub impl LinePolyImpl of LinePolyTrait {
    /// Returns the number of coefficients.
    fn len(self: @LinePoly) -> usize {
        self.coeffs.len()
    }

    /// Evaluates the polynomial at a single point.
    // TODO(andrew): Can remove if only use `Self::evaluate()` in the verifier.
    // Note there are tradeoffs depending on the blowup factor last FRI layer degree bound.
    fn eval_at_point(self: @LinePoly, mut x: BaseField) -> SecureField {
        let mut doublings = array![];
        for _ in 0..*self.log_size {
            doublings.append(x);
            let x_square = x * x;
            x = x_square + x_square - m31(1);
        }

        fold(self.coeffs, @doublings, 0, 0, self.coeffs.len())
    }

    fn evaluate(self: @LinePoly, domain: LineDomain) -> LineEvaluation {
        assert!(domain.size() >= self.coeffs.len());

        // The first few FFT layers may just copy coefficients so we do it directly.
        // See the docs for `n_skipped_layers` in `line_fft()`.
        let log_domain_size = domain.log_size();
        let log_degree_bound = *self.log_size;
        let n_skipped_layers = log_domain_size - log_degree_bound;
        let duplicity = pow2(n_skipped_layers);
        let coeffs = repeat_value(self.coeffs.span(), duplicity);

        LineEvaluationImpl::new(domain, line_fft(coeffs, domain, n_skipped_layers))
    }
}

/// Repeats each value sequentially `duplicity` many times.
pub fn repeat_value(values: Span<QM31>, duplicity: usize) -> Array<QM31> {
    let mut res = array![];
    for v in values {
        for _ in 0..duplicity {
            res.append(*v)
        };
    }
    res
}

/// Performs a FFT on a univariate polynomial.
///
/// `values` is the coefficients stored in bit-reversed order. The evaluations of the polynomial
/// over `domain` is returned in natural order.
///
/// `n_skipped_layers` specifies how many of the initial butterfly layers to skip. This is used for
/// more efficient degree aware FFTs as the butterflies in the first layers of the FFT only involve
/// copying coefficients to different locations (because one or more of the coefficients is zero).
/// This new algorithm is `O(n log d)` vs `O(n log n)` where `n` is the domain size and `d` is the
/// degree of the polynomial.
///
/// Note the algorithm does not operate on coefficients in the standard monomial basis but rather
/// coefficients in a basis relating to the circle's x-coordinate doubling map `pi(x) = 2x^2 - 1`
/// i.e.
///
/// ```text
/// B = { 1 } * { x } * { pi(x) } * { pi(pi(x)) } * ...
///   = { 1, x, pi(x), pi(x) * x, pi(pi(x)), pi(pi(x)) * x, pi(pi(x)) * pi(x), ... }
/// ```
///
/// # Panics
///
/// Panics if the number of values doesn't match the size of the domain.
#[inline]
fn line_fft(
    mut values: Array<QM31>, mut domain: LineDomain, n_skipped_layers: usize,
) -> Array<QM31> {
    let n = values.len();
    assert!(values.len() == domain.size());

    let mut domains = array![];
    while domain.log_size() != n_skipped_layers {
        domains.append(domain);
        domain = domain.double();
    }
    let mut domains = domains.span();

    while let Some(domain) = domains.pop_back() {
        let chunk_size = domain.size();
        let twiddles = gen_twiddles(domain).span();
        let n_chunks = n / chunk_size;
        let stride = chunk_size / 2;
        let values_span = values.span();
        let mut next_values = array![];
        for chunk_i in 0..n_chunks {
            let chunk_offset = chunk_i * chunk_size;
            let mut chunk_l_vals = values_span.slice(chunk_offset, stride).into_iter();
            let mut chunk_r_vals = values_span.slice(chunk_offset + stride, stride).into_iter();
            let mut next_r_values = array![];
            for twiddle in twiddles {
                let v0 = *chunk_l_vals.next().unwrap();
                let v1 = *chunk_r_vals.next().unwrap();
                let (v0, v1) = butterfly(v0, v1, *twiddle);
                next_values.append(v0);
                next_r_values.append(v1);
            }
            next_values.append_span(next_r_values.span());
        }
        values = next_values;
    }

    values
}


/// Folds values recursively in `O(n)` by a hierarchical application of folding factors.
///
/// i.e. folding `n = 8` values with `folding_factors = [x, y, z]`:
///
/// ```text
///               n2=n1+x*n2
///           /               \
///     n1=n3+y*n4          n2=n5+y*n6
///      /      \            /      \
/// n3=a+z*b  n4=c+z*d  n5=e+z*f  n6=g+z*h
///   /  \      /  \      /  \      /  \
///  a    b    c    d    e    f    g    h
/// ```
///
/// # Panics
///
/// Panics if the number of values is not a power of two or if an incorrect number of of folding
/// factors is provided.
pub fn fold(
    values: @Array<SecureField>,
    folding_factors: @Array<BaseField>,
    index: usize,
    level: usize,
    n: usize,
) -> SecureField {
    if n == 1 {
        return *values[index];
    }

    let lhs_val = fold(values, folding_factors, index, level + 1, n / 2);
    let rhs_val = fold(values, folding_factors, index + n / 2, level + 1, n / 2);
    return lhs_val + rhs_val.mul_m31(*folding_factors[level]);
}


#[inline]
fn gen_twiddles(self: @LineDomain) -> Array<M31> {
    let mut iter = LineDomainIterator {
        cur: self.coset.initial_index.to_point(),
        step: self.coset.step.to_point(),
        remaining: self.size() / 2,
    };
    let mut res = array![];
    while let Some(v) = iter.next() {
        res.append(v);
    }
    res
}

#[test]
fn line_domain_of_size_two_works() {
    let coset = CosetImpl::new(CirclePointIndexImpl::new(0), 1);
    LineDomainImpl::new_unchecked(coset);
}

#[test]
fn line_domain_of_size_one_works() {
    let coset = CosetImpl::new(CirclePointIndexImpl::new(0), 0);
    LineDomainImpl::new_unchecked(coset);
}

#[test]
fn test_eval_at_point_1() {
    let line_poly = LinePoly {
        coeffs: array![
            qm31_const::<1080276375, 1725024947, 477465525, 102017026>(),
            qm31_const::<1080276375, 1725024947, 477465525, 102017026>(),
        ],
        log_size: 1,
    };
    let x = m31(590768354);

    let result = line_poly.eval_at_point(x);

    assert_eq!(result, qm31_const::<515899232, 1030391528, 1006544539, 11142505>());
}

#[test]
fn test_eval_at_point_2() {
    let line_poly = LinePoly {
        coeffs: array![qm31_const::<1, 2, 3, 4>(), qm31_const::<5, 6, 7, 8>()], log_size: 1,
    };
    let x = m31(10);

    let result = line_poly.eval_at_point(x);

    assert_eq!(result, qm31_const::<51, 62, 73, 84>());
}

#[test]
fn test_eval_at_point_3() {
    let poly = LinePoly {
        coeffs: array![
            qm31_const::<1, 8, 0, 1>(), qm31_const::<2, 7, 1, 2>(), qm31_const::<3, 6, 0, 1>(),
            qm31_const::<4, 5, 1, 3>(), qm31_const::<5, 4, 0, 1>(), qm31_const::<6, 3, 1, 4>(),
            qm31_const::<7, 2, 0, 1>(), qm31_const::<8, 1, 1, 5>(),
        ],
        log_size: 3,
    };
    let x = m31(10);

    let result = poly.eval_at_point(x);

    assert_eq!(result, qm31_const::<1328848956, 239350644, 174242200, 838661589>());
}

#[test]
fn test_evaluate() {
    let log_size = 3;
    let domain = LineDomainImpl::new_unchecked(CosetImpl::half_odds(log_size));
    let poly = LinePoly {
        coeffs: array![
            qm31_const::<1, 8, 0, 1>(), qm31_const::<2, 7, 1, 2>(), qm31_const::<3, 6, 0, 1>(),
            qm31_const::<4, 5, 1, 3>(), qm31_const::<5, 4, 0, 1>(), qm31_const::<6, 3, 1, 4>(),
            qm31_const::<7, 2, 0, 1>(), qm31_const::<8, 1, 1, 5>(),
        ],
        log_size,
    };

    let result = poly.evaluate(domain);
    let mut result_iter = result.values.into_iter();

    for x in domain.into_iter() {
        assert_eq!(result_iter.next().unwrap(), poly.eval_at_point(x));
    }
}

#[test]
fn test_evaluate_with_large_domain() {
    let log_size = 3;
    let domain = LineDomainImpl::new_unchecked(CosetImpl::half_odds(log_size + 2));
    let poly = LinePoly {
        coeffs: array![
            qm31_const::<1, 8, 0, 1>(), qm31_const::<2, 7, 1, 2>(), qm31_const::<3, 6, 0, 1>(),
            qm31_const::<4, 5, 1, 3>(), qm31_const::<5, 4, 0, 1>(), qm31_const::<6, 3, 1, 4>(),
            qm31_const::<7, 2, 0, 1>(), qm31_const::<8, 1, 1, 5>(),
        ],
        log_size,
    };

    let result = poly.evaluate(domain);
    let mut result_iter = result.values.into_iter();

    for x in domain.into_iter() {
        assert_eq!(result_iter.next().unwrap(), poly.eval_at_point(x));
    }
}

#[generate_trait]
pub impl LineDomainNewImpl of LineDomainNewTrait {
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
            assert!(
                coset.initial_index.to_point().log_order() >= coset.log_size + 2,
                "coset x-coordinates not unique",
            );
        }
        LineDomain { coset: coset }
    }
}

impl LineDomainIntoIterator of IntoIterator<LineDomain> {
    type IntoIter = LineDomainIterator;

    fn into_iter(self: LineDomain) -> LineDomainIterator {
        LineDomainIterator {
            cur: self.coset.initial_index.to_point(),
            step: self.coset.step.to_point(),
            remaining: self.size(),
        }
    }
}

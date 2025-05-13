#[cfg(test)]
mod tests {
    use core::iter::{IntoIterator, Iterator};
    use crate::circle::{CirclePointIndexImpl, CosetImpl};
    use crate::fields::m31::m31;
    use crate::fields::qm31::qm31_const;
    use crate::poly::line::{LineDomain, LineDomainImpl, LineDomainIterator, LinePoly, LinePolyTrait};

    #[test]
    fn line_domain_of_size_two_works() {
        let coset = CosetImpl::new(CirclePointIndexImpl::new(0), 1);
        LineDomainImpl::new(coset);
    }

    #[test]
    fn line_domain_of_size_one_works() {
        let coset = CosetImpl::new(CirclePointIndexImpl::new(0), 0);
        LineDomainImpl::new(coset);
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
        let domain = LineDomainImpl::new(CosetImpl::half_odds(log_size));
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
        let domain = LineDomainImpl::new(CosetImpl::half_odds(log_size + 2));
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

    impl LineDomainIntoIterator of IntoIterator<LineDomain> {
        type IntoIter = LineDomainIterator;

        fn into_iter(self: LineDomain) -> LineDomainIterator {
            LineDomainIterator {
                cur: self.coset.initial_index.to_point(),
                step: self.coset.step_size.to_point(),
                remaining: self.size(),
            }
        }
    }
}

use crate::circle::{CirclePoint, CirclePointIndexImpl, Coset, CosetImpl};
use crate::fields::m31::m31;
use crate::poly::circle::{CircleDomain, CircleDomainTrait, CircleEvaluationImpl};

#[test]
fn test_circle_domain_at_1() {
    let half_coset = Coset {
        initial_index: CirclePointIndexImpl::new(16777216),
        step: CirclePointIndexImpl::new(67108864),
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
        step: CirclePointIndexImpl::new(67108864),
        log_size: 5,
    };
    let domain = CircleDomain { half_coset };
    let index = 37;
    let result = domain.at(index);

    assert_eq!(result, CirclePoint { x: m31(9803698), y: m31(2079025011) });
}

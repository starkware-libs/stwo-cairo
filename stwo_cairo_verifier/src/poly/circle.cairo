use stwo_cairo_verifier::circle::CirclePointTrait;
use core::option::OptionTrait;
use core::clone::Clone;
use core::result::ResultTrait;
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::utils::pow;
use stwo_cairo_verifier::circle::{
    Coset, CosetImpl, CirclePoint, CirclePointM31Impl, M31_CIRCLE_GEN, CIRCLE_ORDER
};

#[derive(Debug, Copy, Drop, PartialEq, Eq)]
pub struct CircleDomain {
    pub half_coset: Coset
}

#[generate_trait]
pub impl CircleDomainImpl of CircleDomainTrait {
    fn new(half_coset: Coset) -> CircleDomain {
        CircleDomain { half_coset }
    }

    fn log_size(self: @CircleDomain) -> usize {
        *self.half_coset.log_size + 1
    }

    fn size(self: @CircleDomain) -> usize {
        pow(2, self.log_size())
    }

    fn index_at(self: @CircleDomain, index: usize) -> usize {
        if index < self.half_coset.size() {
            self.half_coset.index_at(index)
        } else {
            CIRCLE_ORDER - self.half_coset.index_at(index - self.half_coset.size())
        }
    }

    fn at(self: @CircleDomain, index: usize) -> CirclePoint<M31> {
        M31_CIRCLE_GEN.mul(self.index_at(index).into())
    }

    fn new_with_log_size(log_size: u32) -> CircleDomain {
        CircleDomain { half_coset: CosetImpl::half_odds(log_size - 1) }
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


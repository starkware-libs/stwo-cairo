use stwo_cairo_verifier::fri::circle::CirclePointM31Trait;
use core::option::OptionTrait;
use core::clone::Clone;
use core::result::ResultTrait;
use stwo_cairo_verifier::fields::m31::{M31, m31};
use super::utils::pow;
use super::circle::{CirclePointM31, CirclePointM31Impl, M31_CIRCLE_GEN, CIRCLE_ORDER};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Drop)]
pub struct Coset {
    pub initial_index: usize,
    //pub initial: CirclePoint<M31>,
    pub step_size: usize,
    //pub step: CirclePoint<M31>,
    pub log_size: u32,
}

#[derive(Copy, Drop, Debug)]
pub struct LineDomain {
    pub coset: Coset,
}

#[derive(Debug, Copy, Drop, PartialEq, Eq)]
pub struct CircleDomain {
    pub half_coset: Coset
}

#[generate_trait]
pub impl CosetImpl of CosetTrait {
    fn index_at(self: @Coset, index: usize) -> usize {
        let initial_index = *self.initial_index;
        let step_size = *self.step_size;
        let index_times_step = (core::integer::u32_wide_mul(step_size, index) & 0x7fffffff)
            .try_into()
            .unwrap();
        let result = core::integer::u32_wrapping_add(initial_index, index_times_step) & 0x7fffffff;
        result
    }

    fn new(initial_index: usize, log_size: u32) -> Coset {
        let step_size = pow(2, 31 - log_size);
        Coset { initial_index, step_size, log_size }
    }

    fn double(self: @Coset) -> Coset {
        let initial_index = *self.initial_index;
        let step_size = *self.step_size;
        let double_initial_index = core::integer::u32_wrapping_add(initial_index, initial_index);
        let double_step_size = core::integer::u32_wrapping_add(step_size, step_size);
        let log_size = if *self.log_size == 0 {
            0
        } else {
            *self.log_size - 1
        };

        Coset {
            initial_index: double_initial_index & 0x7fffffff,
            step_size: double_step_size & 0x7fffffff,
            log_size
        }
    }

    fn at(self: @Coset, index: usize) -> CirclePointM31 {
        M31_CIRCLE_GEN.mul(self.index_at(index))
    }

    fn size(self: @Coset) -> usize {
        pow(2, *self.log_size)
    }
}

#[generate_trait]
pub impl LineDomainImpl of LineDomainTrait {
    fn new(coset: Coset) -> LineDomain {
        // TODO: Implement assertions.
        LineDomain { coset: coset }
    }

    fn double(self: @LineDomain) -> LineDomain {
        LineDomain { coset: self.coset.double() }
    }

    fn at(self: @LineDomain, index: usize) -> M31 {
        self.coset.at(index).x
    }

    fn log_size(self: @LineDomain) -> usize {
        *self.coset.log_size
    }

    fn size(self: @LineDomain) -> usize {
        self.coset.size()
    }
}

#[generate_trait]
pub impl CircleDomainImpl of CircleDomainTrait {
    fn log_size(self: @CircleDomain) -> usize {
        *self.half_coset.log_size + 1
    }

    fn index_at(self: @CircleDomain, index: usize) -> usize {
        if index < self.half_coset.size() {
            self.half_coset.index_at(index)
        } else {
            CIRCLE_ORDER - self.half_coset.index_at(index - self.half_coset.size())
        }
    }

    fn at(self: @CircleDomain, index: usize) -> CirclePointM31 {
        M31_CIRCLE_GEN.mul(self.index_at(index))
    }
}

#[test]
fn test_coset_index_at() {
    let coset = Coset { initial_index: 16777216, log_size: 5, step_size: 67108864 };
    let result = coset.index_at(8);
    let expected_result = 553648128;
    assert_eq!(expected_result, result);
}

#[test]
fn test_coset_constructor() {
    let result = CosetImpl::new(16777216, 5);
    let expected_result = Coset { initial_index: 16777216, log_size: 5, step_size: 67108864 };
    assert_eq!(expected_result, result);
}

#[test]
fn test_coset_double() {
    let coset = Coset {
        initial_index: 16777216, // initial: CirclePoint { x: M31(838195206), y: M31(1774253895) },
        step_size: 67108864, // step: CirclePoint { x: M31(1179735656), y: M31(1241207368) },
        log_size: 5
    };
    let result = coset.double();

    let expected_result = Coset {
        initial_index: 33554432, // initial: CirclePoint { x: M31(579625837), y: M31(1690787918) },
        step_size: 134217728, // step: CirclePoint { x: M31(590768354), y: M31(978592373) },
        log_size: 4
    };
    assert_eq!(expected_result, result);
}

#[test]
fn test_coset_at() {
    let coset = Coset {
        initial_index: 16777216, // initial: CirclePoint { x: M31(838195206), y: M31(1774253895) },
        step_size: 67108864, // step: CirclePoint { x: M31(1179735656), y: M31(1241207368) },
        log_size: 5
    };
    let result = coset.at(17);
    let expected_result = CirclePointM31 { x: m31(7144319), y: m31(1742797653) };
    assert_eq!(expected_result, result);
}

#[test]
fn test_circle_domain_at_1() {
    let half_coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
    let domain = CircleDomain { half_coset };
    let index = 17;
    let result = domain.at(index);
    let expected_result = CirclePointM31 { x: m31(7144319), y: m31(1742797653) };
    assert_eq!(expected_result, result);
}

#[test]
fn test_circle_domain_at_2() {
    let half_coset = Coset { initial_index: 16777216, step_size: 67108864, log_size: 5 };
    let domain = CircleDomain { half_coset };
    let index = 37;
    let result = domain.at(index);
    let expected_result = CirclePointM31 { x: m31(9803698), y: m31(2079025011) };
    assert_eq!(expected_result, result);
}


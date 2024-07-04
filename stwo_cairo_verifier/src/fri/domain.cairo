use core::option::OptionTrait;
use core::clone::Clone;
use core::result::ResultTrait;
use stwo_cairo_verifier::fields::m31::{M31, m31};
use super::utils::pow;


#[derive(Copy, Debug, PartialEq, Eq, Drop)]
pub struct CirclePointIndex {
    pub index: usize
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Drop)]
pub struct Coset {
    pub initial_index: CirclePointIndex,
    //pub initial: CirclePoint<M31>,
    pub step_size: CirclePointIndex,
    //pub step: CirclePoint<M31>,
    pub log_size: u32,
}

#[derive(Copy, Drop)]
pub struct LineDomain {
    pub coset: Coset,
}

#[derive(Copy, Drop)]
pub struct CircleDomain {
    pub half_coset: Coset
}

pub fn dummy_line_domain() -> LineDomain {
    LineDomain {
        coset: Coset {
            initial_index: CirclePointIndex { index: 0 },
            step_size: CirclePointIndex { index: 0 },
            log_size: 1
        }
    }
}

#[generate_trait]
pub impl CosetImpl of CosetTrait {
    fn index_at(self: @Coset, index: usize) -> CirclePointIndex {
        let initial_index = *self.initial_index.index;
        let step_size = *self.step_size.index;
        let index_times_step = (core::integer::u32_wide_mul(step_size, index) & 0x7fffffff)
            .try_into()
            .unwrap();
        let result = core::integer::u32_wrapping_add(initial_index, index_times_step) & 0x7fffffff;
        CirclePointIndex { index: result }
    }

    fn new(initial_index: CirclePointIndex, log_size: u32) -> Coset {
        let step_size = CirclePointIndex { index: pow(2, 31 - log_size) };
        Coset { initial_index, step_size, log_size }
    }

    fn double(self: @Coset) -> Coset {
        let initial_index = *self.initial_index.index;
        let step_size = *self.step_size.index;
        let double_initial_index = core::integer::u32_wrapping_add(initial_index, initial_index);
        let double_step_size = core::integer::u32_wrapping_add(step_size, step_size);
        let log_size = if *self.log_size == 0 {
            0
        } else {
            *self.log_size - 1
        };

        Coset {
            initial_index: CirclePointIndex { index: double_initial_index & 0x7fffffff },
            step_size: CirclePointIndex { index: double_step_size & 0x7fffffff },
            log_size
        }
    }
}

#[generate_trait]
pub impl LineDomainImpl of LineDomainTrait {
    fn new(coset: Coset) -> LineDomain {
        // TODO: Implement assertions.
        LineDomain { coset: coset }
    }
    fn double(self: LineDomain) -> LineDomain {
        LineDomain { coset: self.coset.double() }
    }

    fn at(self: @LineDomain, index: usize) -> M31 {
        // TODO: implement
        m31(1)
    }

    fn log_size(self: @LineDomain) -> usize {
        // TODO: implement
        1
    }
}


#[test]
fn test_coset_index_at() {
    let coset = Coset {
        initial_index: CirclePointIndex { index: 16777216 },
        log_size: 5,
        step_size: CirclePointIndex { index: 67108864 }
    };
    let result = coset.index_at(8);
    let expected_result = CirclePointIndex { index: 553648128 };
    assert_eq!(expected_result, result);
}

#[test]
fn test_coset_constructor() {
    let result = CosetImpl::new(CirclePointIndex { index: 16777216 }, 5);
    let expected_result = Coset {
        initial_index: CirclePointIndex { index: 16777216 },
        log_size: 5,
        step_size: CirclePointIndex { index: 67108864 }
    };
    assert_eq!(expected_result, result);
}

#[test]
fn test_coset_double() {
    let coset = Coset {
        initial_index: CirclePointIndex { index: 16777216 },
        // initial: CirclePoint { x: M31(838195206), y: M31(1774253895) },
        step_size: CirclePointIndex { index: 67108864 },
        // step: CirclePoint { x: M31(1179735656), y: M31(1241207368) },
        log_size: 5
    };
    let result = coset.double();

    let expected_result = Coset {
        initial_index: CirclePointIndex { index: 33554432 },
        // initial: CirclePoint { x: M31(579625837), y: M31(1690787918) },
        step_size: CirclePointIndex { index: 134217728 },
        // step: CirclePoint { x: M31(590768354), y: M31(978592373) },
        log_size: 4
    };
    assert_eq!(expected_result, result);
}

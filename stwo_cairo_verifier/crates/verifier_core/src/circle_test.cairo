use crate::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointM31Impl, CirclePointQM31AddCirclePointM31Impl,
    CirclePointQM31Impl, Coset, CosetImpl, M31_CIRCLE_GEN,
};
use crate::fields::m31::m31;

#[test]
fn test_to_point() {
    let index = CirclePointIndexImpl::new(index: 0b01111111111111111111111111111111);
    let index2 = CirclePointIndexImpl::new(index: 0b00111111111111111111111111111111);

    assert_eq!(index.to_point(), -M31_CIRCLE_GEN);
    assert_eq!(index2.to_point(), CirclePoint { x: -M31_CIRCLE_GEN.x, y: M31_CIRCLE_GEN.y });
}

#[test]
fn test_to_point_with_unreduced_index() {
    // All 32 bits are `1`.
    let index = CirclePointIndexImpl::new(index: 0b11111111111111111111111111111111);

    assert_eq!(index.to_point(), -M31_CIRCLE_GEN.into());
}

#[test]
fn test_add_1() {
    let g4 = CirclePoint { x: m31(0), y: m31(1) };
    assert_eq!(g4 + g4, CirclePoint { x: -m31(1), y: m31(0) });
}

#[test]
fn test_add_2() {
    let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
    let point_2 = CirclePoint { x: m31(1737427771), y: m31(309481134) };

    let result = point_1 + point_2;

    assert_eq!(result, CirclePoint { x: m31(1476625263), y: m31(1040927458) });
}

#[test]
fn test_zero_1() {
    let result = CirclePointM31Impl::zero();

    assert_eq!(result, CirclePoint { x: m31(1), y: m31(0) });
}

#[test]
fn test_zero_2() {
    let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
    let point_2 = CirclePointM31Impl::zero();

    let result = point_1 + point_2;

    assert_eq!(result, point_1.clone());
}

#[test]
fn test_coset_index_at() {
    let coset = Coset {
        initial_index: CirclePointIndexImpl::new(16777216),
        log_size: 5,
        step: CirclePointIndexImpl::new(67108864),
    };

    let result = coset.index_at(8);

    assert_eq!(result, CirclePointIndexImpl::new(553648128));
}

#[test]
fn test_coset_constructor() {
    let result = CosetImpl::new(CirclePointIndexImpl::new(16777216), 5);

    assert_eq!(
        result,
        Coset {
            initial_index: CirclePointIndexImpl::new(16777216),
            log_size: 5,
            step: CirclePointIndexImpl::new(67108864),
        },
    );
}

#[test]
fn test_coset_double() {
    let coset = Coset {
        initial_index: CirclePointIndexImpl::new(16777216),
        step: CirclePointIndexImpl::new(67108864),
        log_size: 5,
    };

    let result = coset.double();

    assert_eq!(
        result,
        Coset {
            initial_index: CirclePointIndexImpl::new(33554432),
            step: CirclePointIndexImpl::new(134217728),
            log_size: 4,
        },
    );
}

#[test]
fn test_coset_at() {
    let coset = Coset {
        initial_index: CirclePointIndexImpl::new(16777216),
        step: CirclePointIndexImpl::new(67108864),
        log_size: 5,
    };

    let result = coset.at(17);

    assert_eq!(result, CirclePoint { x: m31(7144319), y: m31(1742797653) });
}

#[test]
fn test_coset_size() {
    let coset = Coset {
        initial_index: CirclePointIndexImpl::new(16777216),
        step: CirclePointIndexImpl::new(67108864),
        log_size: 5,
    };

    let result = coset.size();

    assert_eq!(result, 32);
}

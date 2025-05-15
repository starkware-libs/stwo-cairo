use crate::circle::{CirclePointM31Impl, M31_CIRCLE_GEN};
use crate::circle_mul_table::{
    M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5, M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17,
    M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23, M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29,
    M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11,
};

#[test]
fn test_constants_valid() {
    let step_1 = M31_CIRCLE_GEN;
    let mut acc = CirclePointM31Impl::zero();
    for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5.span() {
        assert_eq!((*p).into(), acc);
        acc = acc + step_1.into();
    }

    let step_2_pow_6 = acc;
    let mut acc = CirclePointM31Impl::zero();
    for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11.span() {
        assert_eq!((*p).into(), acc);
        acc = acc + step_2_pow_6.into();
    }

    let step_2_pow_12 = acc;
    let mut acc = CirclePointM31Impl::zero();
    for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17.span() {
        assert_eq!((*p).into(), acc);
        acc = acc + step_2_pow_12.into();
    }

    let step_2_pow_18 = acc;
    let mut acc = CirclePointM31Impl::zero();
    for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23.span() {
        assert_eq!((*p).into(), acc);
        acc = acc + step_2_pow_18.into();
    }

    let step_2_pow_24 = acc;
    let mut acc = CirclePointM31Impl::zero();
    for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29.span() {
        assert_eq!((*p).into(), acc);
        acc = acc + step_2_pow_24.into();
    };
}

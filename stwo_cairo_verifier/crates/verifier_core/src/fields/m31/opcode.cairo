//! Implementation of M31 field using the QM31 opcode.
use core::num::traits::One;
use core::ops::{AddAssign, MulAssign, SubAssign};
use core::qm31::m31_ops;
use super::super::qm31::{QM31, QM31Trait};
use super::super::{BatchInvertible, Invertible};
use super::{M31, M31Trait};

pub impl M31InvertibleImpl of Invertible<M31> {
    #[inline(always)]
    fn inverse(self: M31) -> M31 {
        // TODO: Change impl to m31_ops::div when NonZero<M31InnerT> is possible.
        // Currently there is no way to construct a NonZero<M31InnerT>.
        let denom_qm31: QM31 = self.into();
        let denom_qm31_inv = One::one() / denom_qm31;
        let [v, _, _, _] = denom_qm31_inv.to_fixed_array();
        v
    }
}

pub impl M31BatchInvertibleImpl of BatchInvertible<M31> {
    /// Computes all `1/arr[i]` using one inversion per element.
    fn batch_inverse(values: Array<M31>) -> Array<M31> {
        values.span().into_iter().map(|v| v.inverse()).collect()
    }
}

pub impl M31Add of core::traits::Add<M31> {
    #[inline(always)]
    fn add(lhs: M31, rhs: M31) -> M31 {
        M31Trait::new(m31_ops::add(lhs.inner, rhs.inner))
    }
}

pub impl M31Sub of core::traits::Sub<M31> {
    #[inline(always)]
    fn sub(lhs: M31, rhs: M31) -> M31 {
        M31Trait::new(m31_ops::sub(lhs.inner, rhs.inner))
    }
}

pub impl M31Mul of core::traits::Mul<M31> {
    #[inline(always)]
    fn mul(lhs: M31, rhs: M31) -> M31 {
        M31Trait::new(m31_ops::mul(lhs.inner, rhs.inner))
    }
}


pub impl M31AddAssign of AddAssign<M31, M31> {
    #[inline(always)]
    fn add_assign(ref self: M31, rhs: M31) {
        self = self + rhs
    }
}

pub impl M31SubAssign of SubAssign<M31, M31> {
    #[inline(always)]
    fn sub_assign(ref self: M31, rhs: M31) {
        self = self - rhs
    }
}

pub impl M31MulAssign of MulAssign<M31, M31> {
    #[inline(always)]
    fn mul_assign(ref self: M31, rhs: M31) {
        self = self * rhs
    }
}

pub impl M31Neg of Neg<M31> {
    #[inline(always)]
    fn neg(a: M31) -> M31 {
        M31Trait::new(m31_ops::sub(0, a.inner))
    }
}

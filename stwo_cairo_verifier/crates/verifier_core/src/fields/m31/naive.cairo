//! Software only implementation of M31 field (i.e no QM31 opcode).
use bounded_int::{AddHelper, BoundedInt, SubHelper, upcast};
use core::num::traits::{WideMul, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::m31::M31Zero;
use super::super::{BatchInvertible, Invertible};
use super::{ConstValue, M31, M31InnerT, M31Trait, M31_P, P};

pub impl M31InvertibleImpl of Invertible<M31> {
    fn inverse(self: M31) -> M31 {
        assert!(self.is_non_zero());
        let t0 = repeated_square(self, 2) * self;
        let t1 = t0 * t0 * t0;
        let t2 = repeated_square(t1, 3) * t0;
        let t3 = t2 * t2 * t0;
        let t4 = repeated_square(t3, 8) * t3;
        let t5 = repeated_square(t4, 8) * t3;
        repeated_square(t5, 7) * t2
    }
}

pub impl M31BatchInvertibleImpl of BatchInvertible<M31> {}

pub impl M31Add of core::traits::Add<M31> {
    #[inline]
    fn add(lhs: M31, rhs: M31) -> M31 {
        let sum = bounded_int::add(lhs.inner, rhs.inner);
        match bounded_int::constrain::<BoundedInt<0, { 2 * (P - 1) }>, P>(sum) {
            Ok(lt) => M31Trait::new(lt),
            Err(gte) => M31Trait::new(upcast(bounded_int::sub(gte, M31_P))),
        }
    }
}

pub impl M31Sub of core::traits::Sub<M31> {
    #[inline]
    fn sub(lhs: M31, rhs: M31) -> M31 {
        let diff = bounded_int::sub(lhs.inner, rhs.inner);
        let res = match bounded_int::constrain::<BoundedInt<{ -(P - 1) }, { P - 1 }>, 0>(diff) {
            Ok(lt) => upcast(bounded_int::add(lt, M31_P)),
            Err(gte) => gte,
        };

        M31Trait::new(res)
    }
}

pub impl M31Mul of core::traits::Mul<M31> {
    #[inline]
    fn mul(lhs: M31, rhs: M31) -> M31 {
        let lhs_as_u32: u32 = upcast(lhs.inner);
        M31Trait::reduce_u64(lhs_as_u32.wide_mul(upcast(rhs.inner))).into()
    }
}


pub impl M31AddAssign of AddAssign<M31, M31> {
    #[inline]
    fn add_assign(ref self: M31, rhs: M31) {
        self = self + rhs
    }
}

pub impl M31SubAssign of SubAssign<M31, M31> {
    #[inline]
    fn sub_assign(ref self: M31, rhs: M31) {
        self = self - rhs
    }
}

pub impl M31MulAssign of MulAssign<M31, M31> {
    #[inline]
    fn mul_assign(ref self: M31, rhs: M31) {
        self = self * rhs
    }
}

pub impl M31Neg of Neg<M31> {
    #[inline]
    fn neg(a: M31) -> M31 {
        M31Zero::zero() - a
    }
}

/// Returns `v^(2^n)`.
fn repeated_square(v: M31, n: usize) -> M31 {
    if n == 0 {
        return v;
    }
    repeated_square(v * v, n - 1)
}

impl M31AddHelper of AddHelper<M31InnerT, M31InnerT> {
    type Result = BoundedInt<0, { 2 * (P - 1) }>;
}

impl M31SubHelper of SubHelper<M31InnerT, M31InnerT> {
    type Result = BoundedInt<{ -(P - 1) }, { P - 1 }>;
}

impl M31AddReduceHelper of SubHelper<BoundedInt<P, { 2 * (P - 1) }>, ConstValue<P>> {
    type Result = BoundedInt<0, { P - 2 }>;
}


impl M31SubReduceHelper of AddHelper<BoundedInt<{ -(P - 1) }, { -1 }>, ConstValue<P>> {
    type Result = BoundedInt<1, { P - 1 }>;
}

pub impl M31AddConstrainP of bounded_int::ConstrainHelper<BoundedInt<0, { 2 * (P - 1) }>, P> {
    type LowT = M31InnerT;
    type HighT = BoundedInt<{ P }, { 2 * (P - 1) }>;
}


pub impl M31SubConstrain0 of bounded_int::ConstrainHelper<BoundedInt<{ -(P - 1) }, { P - 1 }>, 0> {
    type LowT = BoundedInt<{ -(P - 1) }, { -1 }>;
    type HighT = M31InnerT;
}

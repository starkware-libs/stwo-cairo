//! Software only implementation of M31 field (i.e no QM31 opcode).
use bounded_int::{AddHelper, BoundedInt, SubHelper, upcast};
use core::num::traits::WideMul;
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::super::{BatchInvertible, Invertible};
use super::{ConstValue, M31InnerT, M31Trait, M31_P, M31_SHIFT, P, P_U32};

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct M31 {
    pub inner: M31InnerT,
}

pub impl M31InvertibleImpl of Invertible<M31> {
    fn inverse(self: M31) -> M31 {
        assert!(self.is_non_zero());
        let t0 = sqn(self, 2) * self;
        let t1 = sqn(t0, 1) * t0;
        let t2 = sqn(t1, 3) * t0;
        let t3 = sqn(t2, 1) * t0;
        let t4 = sqn(t3, 8) * t3;
        let t5 = sqn(t4, 8) * t3;
        sqn(t5, 7) * t2
    }
}

pub impl M31BatchInvertibleImpl of BatchInvertible<M31> {}

pub impl M31Add of core::traits::Add<M31> {
    #[inline]
    fn add(lhs: M31, rhs: M31) -> M31 {
        let sum = bounded_int::add(lhs.inner, rhs.inner);
        let res = match bounded_int::constrain::<BoundedInt<0, { 2 * (P - 1) }>, P>(sum) {
            Ok(lt) => lt,
            Err(gte) => upcast(bounded_int::sub(gte, M31_P)),
        };

        M31 { inner: res }
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

        M31 { inner: res }
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

pub impl M31Zero of core::num::traits::Zero<M31> {
    #[inline]
    fn zero() -> M31 {
        M31 { inner: 0 }
    }

    fn is_zero(self: @M31) -> bool {
        *self.inner == 0
    }

    fn is_non_zero(self: @M31) -> bool {
        *self.inner != 0
    }
}

pub impl M31One of core::num::traits::One<M31> {
    #[inline]
    fn one() -> M31 {
        M31 { inner: 1 }
    }

    fn is_one(self: @M31) -> bool {
        *self.inner == 1
    }

    fn is_non_one(self: @M31) -> bool {
        *self.inner != 1
    }
}

pub impl M31Neg of Neg<M31> {
    #[inline]
    fn neg(a: M31) -> M31 {
        M31 { inner: 0 } - a
    }
}

impl M31IntoU32 of Into<M31, u32> {
    #[inline]
    fn into(self: M31) -> u32 {
        upcast(self.inner)
    }
}

impl M31IntoFelt252 of Into<M31, felt252> {
    #[inline]
    fn into(self: M31) -> felt252 {
        self.inner.into()
    }
}

impl U32TryIntoM31 of TryInto<u32, M31> {
    #[inline]
    fn try_into(self: u32) -> Option<M31> {
        if self >= P_U32 {
            return None;
        }

        Some(M31Trait::reduce_u32(self).into())
    }
}

pub impl M31InnerTIntoM31 of Into<M31InnerT, M31> {
    #[inline]
    fn into(self: M31InnerT) -> M31 {
        M31 { inner: self }
    }
}

impl M31PartialOrd of PartialOrd<M31> {
    fn ge(lhs: M31, rhs: M31) -> bool {
        upcast::<_, u32>(lhs.inner) >= upcast(rhs.inner)
    }

    fn lt(lhs: M31, rhs: M31) -> bool {
        upcast::<_, u32>(lhs.inner) < upcast(rhs.inner)
    }
}

#[inline]
pub fn m31(val: u32) -> M31 {
    M31Trait::reduce_u32(val).into()
}

#[derive(Copy, Drop, Debug)]
pub struct UnreducedM31 {
    pub inner: felt252,
}

pub impl UnreducedM31Sub of Sub<UnreducedM31> {
    #[inline]
    fn sub(lhs: UnreducedM31, rhs: UnreducedM31) -> UnreducedM31 {
        UnreducedM31 { inner: lhs.inner - rhs.inner }
    }
}

pub impl UnreducedM31Add of Add<UnreducedM31> {
    #[inline]
    fn add(lhs: UnreducedM31, rhs: UnreducedM31) -> UnreducedM31 {
        UnreducedM31 { inner: lhs.inner + rhs.inner }
    }
}

impl M31IntoUnreducedM31 of Into<M31, UnreducedM31> {
    #[inline]
    fn into(self: M31) -> UnreducedM31 {
        UnreducedM31 { inner: self.inner.into() }
    }
}

/// Returns `v^(2^n)`.
fn sqn(v: M31, n: usize) -> M31 {
    if n == 0 {
        return v;
    }
    sqn(v * v, n - 1)
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

impl DisplayM31 of core::fmt::Display<M31> {
    fn fmt(self: @M31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        self.inner.fmt(ref f)
    }
}

use core::num::traits::{WideMul, CheckedSub};
use core::ops::{AddAssign, MulAssign, SubAssign};
use core::option::OptionTrait;
use core::traits::TryInto;

/// Equals `2^31 - 1`.
pub const P: u32 = 0x7fffffff;

/// Equals `2^31 - 1`.
const P32NZ: NonZero<u32> = 0x7fffffff;

/// Equals `2^31 - 1`.
const P64NZ: NonZero<u64> = 0x7fffffff;

/// Equals `2^31 - 1`.
const P128NZ: NonZero<u128> = 0x7fffffff;

#[derive(Copy, Drop, Debug, PartialEq)]
pub struct M31 {
    pub inner: u32
}

#[generate_trait]
pub impl M31Impl of M31Trait {
    #[inline]
    fn reduce_u32(val: u32) -> M31 {
        let (_, res) = core::integer::u32_safe_divmod(val, P32NZ);
        M31 { inner: res.try_into().unwrap() }
    }

    #[inline]
    fn reduce_u64(val: u64) -> M31 {
        let (_, res) = core::integer::u64_safe_divmod(val, P64NZ);
        M31 { inner: res.try_into().unwrap() }
    }

    #[inline]
    fn reduce_u128(val: u128) -> M31 {
        let (_, res) = core::integer::u128_safe_divmod(val, P128NZ);
        M31 { inner: res.try_into().unwrap() }
    }

    #[inline]
    fn sqn(v: M31, n: usize) -> M31 {
        if n == 0 {
            return v;
        }
        Self::sqn(v * v, n - 1)
    }

    fn inverse(self: M31) -> M31 {
        assert!(self.is_non_zero());
        let t0 = Self::sqn(self, 2) * self;
        let t1 = Self::sqn(t0, 1) * t0;
        let t2 = Self::sqn(t1, 3) * t0;
        let t3 = Self::sqn(t2, 1) * t0;
        let t4 = Self::sqn(t3, 8) * t3;
        let t5 = Self::sqn(t4, 8) * t3;
        Self::sqn(t5, 7) * t2
    }
}
pub impl M31Add of core::traits::Add<M31> {
    #[inline]
    fn add(lhs: M31, rhs: M31) -> M31 {
        let res = lhs.inner + rhs.inner;
        let res = res.checked_sub(P).unwrap_or(res);
        M31 { inner: res }
    }
}

pub impl M31Sub of core::traits::Sub<M31> {
    #[inline]
    fn sub(lhs: M31, rhs: M31) -> M31 {
        lhs + (-rhs)
    }
}

pub impl M31Mul of core::traits::Mul<M31> {
    #[inline]
    fn mul(lhs: M31, rhs: M31) -> M31 {
        M31Impl::reduce_u64(lhs.inner.wide_mul(rhs.inner))
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
        if a.inner == 0 {
            M31 { inner: 0 }
        } else {
            M31 { inner: P - a.inner }
        }
    }
}

impl M31IntoFelt252 of Into<M31, felt252> {
    #[inline]
    fn into(self: M31) -> felt252 {
        self.inner.into()
    }
}

impl M31PartialOrd of PartialOrd<M31> {
    fn ge(lhs: M31, rhs: M31) -> bool {
        lhs.inner >= rhs.inner
    }

    fn lt(lhs: M31, rhs: M31) -> bool {
        lhs.inner < rhs.inner
    }
}

#[inline]
pub fn m31(val: u32) -> M31 {
    M31Impl::reduce_u32(val)
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

#[cfg(test)]
mod tests {
    use super::{m31, P, M31Trait};
    const POW2_15: u32 = 0b1000000000000000;
    const POW2_16: u32 = 0b10000000000000000;

    #[test]
    fn test_m31() {
        assert_eq!(m31(P), m31(0));
        assert_eq!(m31(P + 1), m31(1));
        assert_eq!(m31(1) + m31(2), m31(3));
        assert_eq!(m31(3) - m31(2), m31(1));
        assert_eq!(m31(P - 1) + m31(1), m31(0));
        assert_eq!(m31(0) - m31(1), m31(P - 1));
        assert_eq!(m31(0) - m31(P - 1), m31(1));
    }

    #[test]
    fn test_m31_inv() {
        assert_eq!(m31(POW2_15).inverse(), m31(POW2_16));
    }
}

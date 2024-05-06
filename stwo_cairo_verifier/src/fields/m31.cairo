use core::option::OptionTrait;
use core::traits::TryInto;
use core::result::ResultTrait;

pub const P: u32 = 0x7fffffff;
const P32NZ: NonZero<u32> = 0x7fffffff;
const P64NZ: NonZero<u64> = 0x7fffffff;

#[derive(Copy, Drop, Debug, PartialEq, Eq)]
pub struct M31 {
    pub inner: u32
}

#[generate_trait]
pub impl M31Impl of M31Trait {
    fn reduce_u32(val: u32) -> M31 {
        let (_, res) = core::integer::u32_safe_divmod(val, P32NZ);
        M31 { inner: res.try_into().unwrap() }
    }

    fn reduce_u64(val: u64) -> M31 {
        let (_, res) = core::integer::u64_safe_divmod(val, P64NZ);
        M31 { inner: res.try_into().unwrap() }
    }
}
pub impl M31Add of core::traits::Add<M31> {
    fn add(lhs: M31, rhs: M31) -> M31 {
        let res = lhs.inner + rhs.inner;
        let res = core::integer::u32_overflowing_sub(res, P).unwrap_or(res);
        M31 { inner: res }
    }
}
pub impl M31Sub of core::traits::Sub<M31> {
    fn sub(lhs: M31, rhs: M31) -> M31 {
        lhs + (-rhs)
    }
}
pub impl M31Mul of core::traits::Mul<M31> {
    fn mul(lhs: M31, rhs: M31) -> M31 {
        M31Impl::reduce_u64(core::integer::u32_wide_mul(lhs.inner, rhs.inner))
    }
}
pub impl M31Zero of core::num::traits::Zero<M31> {
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
    fn neg(a: M31) -> M31 {
        if a.inner == 0 {
            M31 { inner: 0 }
        } else {
            M31 { inner: P - a.inner }
        }
    }
}

pub fn m31(val: u32) -> M31 {
    M31Impl::reduce_u32(val)
}

#[cfg(test)]
mod tests {
    use super::{m31, P};

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
}

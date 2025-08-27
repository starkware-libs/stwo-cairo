use bounded_int::{BoundedInt, DivRemHelper, div_rem, upcast};
pub use stwo_verifier_utils::M31_SHIFT;

#[cfg(not(feature: "qm31_opcode"))]
mod naive;
#[cfg(not(feature: "qm31_opcode"))]
use naive::*;
#[cfg(feature: "qm31_opcode")]
mod opcode;
#[cfg(feature: "qm31_opcode")]
use opcode::*;

pub const P: felt252 = 0x7fffffff;

pub const P_U32: u32 = 0x7fffffff;

const NZ_M31_P: NonZero<ConstValue<P>> = 0x7fffffff;

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;

const M31_P: ConstValue<P> = 0x7fffffff;

pub type M31InnerT = BoundedInt<0, 0x7ffffffe>;

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct M31 {
    pub inner: M31InnerT,
}

#[generate_trait]
pub impl M31Impl of M31Trait {
    #[inline]
    fn new(val: M31InnerT) -> M31 {
        M31 { inner: val }
    }

    #[inline]
    fn reduce_u32(val: u32) -> M31 {
        let (_, res) = div_rem(val, NZ_M31_P);
        Self::new(res)
    }

    #[inline]
    fn reduce_u64(val: u64) -> M31 {
        let (_, res) = div_rem(val, NZ_M31_P);
        Self::new(res)
    }

    #[inline]
    fn reduce_u128(val: u128) -> M31 {
        let (_, res) = div_rem(val, NZ_M31_P);
        Self::new(res)
    }
}

/// A trait for multiplying a value by an `M31`.
pub trait MulByM31Trait<T> {
    fn mul_m31(self: T, rhs: M31) -> T;
}

/// A trait for adding an `M31` element to a value.
pub trait AddM31Trait<T> {
    fn add_m31(self: T, rhs: M31) -> T;
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
    /// Returns Some if value is in the range `[0, P)`, else returns None.
    #[inline]
    fn try_into(self: u32) -> Option<M31> {
        match bounded_int::constrain::<u32, P>(self.into()) {
            Ok(x) => Some(M31Trait::new(x)),
            Err(_) => None,
        }
    }
}

pub impl M31Zero of core::num::traits::Zero<M31> {
    #[inline]
    fn zero() -> M31 {
        M31Trait::new(0)
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
        M31Trait::new(1)
    }

    fn is_one(self: @M31) -> bool {
        *self.inner == 1
    }

    fn is_non_one(self: @M31) -> bool {
        *self.inner != 1
    }
}

impl M31ConstrainHelper of bounded_int::ConstrainHelper<u32, P> {
    type LowT = M31InnerT;
    type HighT = BoundedInt<P, 0xffffffff>;
}

impl DivRemU32ByP of DivRemHelper<u32, ConstValue<P>> {
    // 0x2 = (2**32 - 1) / P.
    type DivT = BoundedInt<0, 2>;
    type RemT = M31InnerT;
}

impl DivRemU64ByP of DivRemHelper<u64, ConstValue<P>> {
    // 0x200000004 = (2**64 - 1) / P.
    type DivT = BoundedInt<0, 0x200000004>;
    type RemT = M31InnerT;
}

impl DivRemU128ByP of DivRemHelper<u128, ConstValue<P>> {
    // 0x2000000040000000800000010 = (2**128 - 1) / P.
    type DivT = BoundedInt<0, 0x2000000040000000800000010>;
    type RemT = M31InnerT;
}

impl DisplayM31 of core::fmt::Display<M31> {
    fn fmt(self: @M31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let v: u32 = (*self).into();
        core::fmt::Display::fmt(@v, ref f)
    }
}

#[inline]
pub fn m31(val: u32) -> M31 {
    val.try_into().unwrap()
}

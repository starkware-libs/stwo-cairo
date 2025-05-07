use bounded_int::{BoundedInt, DivRemHelper, div_rem, upcast};

// TODO: Scarb currently has issues with feature flags. Enable again once bugs fixed.
// #[cfg(not(feature: "qm31_opcode"))]
// mod naive;
// #[cfg(not(feature: "qm31_opcode"))]
// use naive::*;
// #[cfg(feature: "qm31_opcode")]
// mod opcode;
// #[cfg(feature: "qm31_opcode")]
// use opcode::*;

mod opcode;
use opcode::*;

pub const P: felt252 = 0x7fffffff;

pub const P_U32: u32 = 0x7fffffff;

const NZ_M31_P: NonZero<ConstValue<P>> = 0x7fffffff;

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;

const M31_P: ConstValue<P> = 0x7fffffff;

/// Equals `2^31`.
pub const M31_SHIFT: felt252 = 0x80000000; // 2**31.

pub type M31InnerT = BoundedInt<0, 0x7ffffffe>;

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
pub struct M31 {
    // in opcode version this should be core::qm31::qm31.
    pub inner: M31InnerT,
}

#[generate_trait]
pub impl M31Impl of M31Trait {
    #[inline]
    // how does it work?
    fn reduce_u32(val: u32) -> M31InnerT {
        let (_, res) = div_rem(val, NZ_M31_P);
        res
    }

    #[inline]
    fn reduce_u64(val: u64) -> M31InnerT {
        let (_, res) = div_rem(val, NZ_M31_P);
        res
    }

    #[inline]
    fn reduce_u128(val: u128) -> M31InnerT {
        let (_, res) = div_rem(val, NZ_M31_P);
        res
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

// why do you need this function?
pub impl M31InnerTIntoM31 of Into<M31InnerT, M31> {
    #[inline]
    fn into(self: M31InnerT) -> M31 {
        M31 { inner: self }
    }
}

impl U32TryIntoM31 of TryInto<u32, M31> {
    /// Returns Some if value is in the range `[0, P)`, else returns None.
    #[inline]
    fn try_into(self: u32) -> Option<M31> {
        if self >= P_U32 {
            return None;
        }
        // can be more efficient with splitting BoundedInt
        Some(M31Trait::reduce_u32(self).into())
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
    M31Trait::reduce_u32(val).into()
}

#[cfg(test)]
mod tests {
    use super::super::Invertible;
    use super::{P_U32 as P, m31};

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

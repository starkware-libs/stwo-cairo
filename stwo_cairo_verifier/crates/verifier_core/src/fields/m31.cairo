use bounded_int::{BoundedInt, DivRemHelper, div_rem, upcast};

mod naive;
use naive::*;

pub const P: felt252 = 0x7fffffff;

pub const P_U32: u32 = 0x7fffffff;

const NZ_M31_P: NonZero<ConstValue<P>> = 0x7fffffff;

const M31_P: ConstValue<P> = 0x7fffffff;

/// Equals `2^31`.
pub const M31_SHIFT: felt252 = 0x80000000; // 2**31.

pub type M31InnerT = BoundedInt<0, { P - 1 }>;

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;

#[generate_trait]
pub impl M31Impl of M31Trait {
    #[inline]
    fn reduce_u32(val: u32) -> M31InnerT {
        let (_, res) = div_rem(val, NZ_M31_P);
        upcast(res)
    }

    #[inline]
    fn reduce_u64(val: u64) -> M31InnerT {
        let (_, res) = div_rem(val, NZ_M31_P);
        upcast(res)
    }

    #[inline]
    fn reduce_u128(val: u128) -> M31InnerT {
        let (_, res) = div_rem(val, NZ_M31_P);
        upcast(res)
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

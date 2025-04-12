use bounded_int::{AddHelper, BoundedInt, DivRemHelper, SubHelper, div_rem, upcast};
use core::num::traits::{One, WideMul, Zero};
use core::ops::{AddAssign, MulAssign, SubAssign};
use super::qm31::QM31Trait;
use super::{BatchInvertible, Invertible};

pub const P: felt252 = 0x7fffffff;

pub const P_U32: u32 = 0x7fffffff;

/// Equals `2^31`.
pub const M31_SHIFT: felt252 = 0x80000000; // 2**31.

pub type M31InnerT = BoundedInt<0, { P - 1 }>;

#[derive(Copy, Drop, Debug, PartialEq)]
#[cfg(not(feature: "qm31_opcode"))]
pub struct M31 {
    pub inner: super::qm31::QM31,
}

#[derive(Copy, Drop, Debug, PartialEq, Serde)]
#[cfg(feature: "qm31_opcode")]
pub struct M31 {
    pub inner: M31InnerT,
}

pub impl M31InvertibleImpl of Invertible<M31> {
    #[cfg(not(feature: "qm31_opcode"))]
    fn inverse(self: M31) -> M31 {
        M31 { inner: self.inner.inverse() }
    }

    #[cfg(feature: "qm31_opcode")]
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

pub trait M31Trait {
    fn reduce_u32(val: u32) -> M31InnerT;

    fn reduce_u64(val: u64) -> M31InnerT;

    fn reduce_u128(val: u128) -> M31InnerT;
}

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

pub impl M31Add of core::traits::Add<M31> {
    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn add(lhs: M31, rhs: M31) -> M31 {
        M31 { inner: lhs.inner + rhs.inner }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
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
    #[cfg(not(feature: "qm31_opcode"))]
    fn sub(lhs: M31, rhs: M31) -> M31 {
        M31 { inner: lhs.inner - rhs.inner }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
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
    #[cfg(not(feature: "qm31_opcode"))]
    fn mul(lhs: M31, rhs: M31) -> M31 {
        M31 { inner: lhs.inner * rhs.inner }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
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
    #[cfg(not(feature: "qm31_opcode"))]
    fn zero() -> M31 {
        M31 { inner: Zero::zero() }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn zero() -> M31 {
        M31 { inner: 0 }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn is_zero(self: @M31) -> bool {
        self.inner.is_zero()
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn is_zero(self: @M31) -> bool {
        *self.inner == 0
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn is_non_zero(self: @M31) -> bool {
        self.inner.is_non_zero()
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn is_non_zero(self: @M31) -> bool {
        *self.inner != 0
    }
}

pub impl M31One of core::num::traits::One<M31> {
    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn one() -> M31 {
        M31 { inner: One::one() }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn one() -> M31 {
        M31 { inner: 1 }
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn is_one(self: @M31) -> bool {
        self.inner.is_one()
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn is_one(self: @M31) -> bool {
        *self.inner == 1
    }

    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn is_non_one(self: @M31) -> bool {
        self.inner.is_non_one()
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn is_non_one(self: @M31) -> bool {
        *self.inner != 1
    }
}

pub impl M31Neg of Neg<M31> {
    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn neg(a: M31) -> M31 {
        M31 { inner: -a.inner }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn neg(a: M31) -> M31 {
        M31 { inner: 0 } - a
    }
}

pub impl M31InnerTIntoM31 of Into<M31InnerT, M31> {
    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: M31InnerT) -> M31 {
        M31 { inner: QM31Trait::from_array([self, 0, 0, 0]) }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: M31InnerT) -> M31 {
        M31 { inner: self }
    }
}

impl M31IntoU32 of Into<M31, u32> {
    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: M31) -> u32 {
        let [a, _, _, _] = self.inner.to_array();
        upcast(a)
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: M31) -> u32 {
        upcast(self.inner)
    }
}

impl M31IntoFelt252 of Into<M31, felt252> {
    #[inline]
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: M31) -> felt252 {
        let [a, _, _, _] = self.inner.to_array();
        a.into()
    }


    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: M31) -> felt252 {
        self.inner.into()
    }
}

impl U32IntoM31 of Into<u32, M31> {
    #[inline]
    fn into(self: u32) -> M31 {
        M31Trait::reduce_u32(self).into()
    }
}

impl M31PartialOrd of PartialOrd<M31> {
    fn ge(lhs: M31, rhs: M31) -> bool {
        let lhs: u32 = lhs.into();
        let rhs: u32 = rhs.into();
        lhs >= rhs
    }

    #[inline]
    fn lt(lhs: M31, rhs: M31) -> bool {
        let lhs: u32 = lhs.into();
        let rhs: u32 = rhs.into();
        lhs < rhs
    }
}

#[inline]
pub fn m31(val: u32) -> M31 {
    M31Trait::reduce_u32(val).into()
}

#[derive(Copy, Drop)]
#[cfg(not(feature: "qm31_opcode"))]
pub struct UnreducedM31 {
    pub inner: M31,
}

#[derive(Copy, Drop, Debug)]
#[cfg(feature: "qm31_opcode")]
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
    #[cfg(not(feature: "qm31_opcode"))]
    fn into(self: M31) -> UnreducedM31 {
        UnreducedM31 { inner: self }
    }

    #[inline]
    #[cfg(feature: "qm31_opcode")]
    fn into(self: M31) -> UnreducedM31 {
        UnreducedM31 { inner: self.inner.into() }
    }
}

/// Returns `v^(2^n)`.
#[cfg(feature: "qm31_opcode")]
fn sqn(v: M31, n: usize) -> M31 {
    if n == 0 {
        return v;
    }
    sqn(v * v, n - 1)
}

type ConstValue<const VALUE: felt252> = BoundedInt<VALUE, VALUE>;

const NZ_M31_P: NonZero<ConstValue<P>> = 0x7fffffff;
const M31_P: ConstValue<P> = 0x7fffffff;

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
        let v: u32 = (*self).into();
        core::fmt::Display::fmt(@v, ref f)
    }
}

#[cfg(not(feature: "qm31_opcode"))]
pub impl M31Serde of Serde<M31> {
    fn serialize(self: @M31, ref output: Array<felt252>) {
        let [a, _, _, _] = self.inner.to_array();
        output.append(a.into());
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<M31> {
        let a: M31InnerT = Serde::deserialize(ref serialized)?;
        Some(a.into())
    }
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

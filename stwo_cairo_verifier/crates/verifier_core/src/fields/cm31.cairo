//! Complex extension field of M31.
//! Equivalent to M31\[x\] over (x^2 + 1) as the irreducible polynomial.
//! Represented as (a, b) of a + bi.

use super::m31::M31;

#[cfg(not(feature: "qm31_opcode"))]
mod naive;
#[cfg(not(feature: "qm31_opcode"))]
use naive::*;
#[cfg(feature: "qm31_opcode")]
mod opcode;
#[cfg(feature: "qm31_opcode")]
use opcode::*;

// TODO(andrew): When associated types are supported, support `Mul<QM31, CM31>`.
/// A trait for multiplying a value by a `CM31`.
pub trait MulByCM31Trait<T> {
    fn mul_cm31(self: T, rhs: CM31) -> T;
}

pub trait CM31Trait {
    // TODO(andrew): When associated types are supported, support `Sub<CM31, M31>`.
    fn sub_m31(self: CM31, rhs: M31) -> CM31;

    fn unpack(self: CM31) -> (M31, M31);

    fn pack(a: M31, b: M31) -> CM31;
}

impl DisplayCM31 of core::fmt::Display<CM31> {
    fn fmt(self: @CM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let (a, b) = (*self).unpack();
        let a: u32 = a.into();
        let b: u32 = b.into();
        write!(f, "{} + {}i", @a, @b)
    }
}

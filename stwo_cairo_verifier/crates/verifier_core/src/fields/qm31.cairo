//! Extension field of CM31.
//! Equivalent to CM31\[x\] over (x^2 - 2 - i) as the irreducible polynomial.
//! Represented as ((a, b), (c, d)) of (a + bi) + (c + di)u.

use bounded_int::upcast;
use super::cm31::CM31;
use super::m31::{M31, UnreducedM31};

// TODO: Scarb currently has issues with feature flags. Change to "qm31_opcode" when possible.
#[cfg(feature: "no_qm31_opcode")]
mod naive;
#[cfg(feature: "no_qm31_opcode")]
use naive::*;
#[cfg(not(feature: "no_qm31_opcode"))]
mod opcode;
#[cfg(not(feature: "no_qm31_opcode"))]
use opcode::*;

/// Equals `(2^31 - 1)^4`.
pub const P4: u128 = 0xFFFFFFF800000017FFFFFFE00000001;

pub const QM31_EXTENSION_DEGREE: usize = 4;

pub trait QM31Trait {
    fn from_fixed_array(arr: [M31; QM31_EXTENSION_DEGREE]) -> QM31;

    fn to_fixed_array(self: QM31) -> [M31; QM31_EXTENSION_DEGREE];

    fn mul_m31(self: QM31, rhs: M31) -> QM31;

    // TODO(andrew): When associated types are supported, support `Mul<QM31, CM31>`.
    fn mul_cm31(self: QM31, rhs: CM31) -> QM31;

    fn complex_conjugate(self: QM31) -> QM31;

    /// Returns a fused multiply-add i.e. returns `a * b + c`.
    fn fused_mul_add(a: QM31, b: QM31, c: QM31) -> QM31;

    /// Returns a fused multiply-subtract i.e. returns `a * b - c`.
    fn fused_mul_sub(a: QM31, b: QM31, c: QM31) -> QM31;

    /// Returns `lhs * rhs` in unreduced form.
    fn mul_unreduced(lhs: QM31, rhs: QM31) -> UnreducedQM31;

    /// Returns the combined value, given the values of its composing base field polynomials at that
    /// point.
    fn from_partial_evals(evals: [QM31; QM31_EXTENSION_DEGREE]) -> QM31;
}

pub impl QM31Serde of Serde<QM31> {
    fn serialize(self: @QM31, ref output: Array<felt252>) {
        let [a, b, c, d] = self.to_fixed_array();
        output.append(a.into());
        output.append(b.into());
        output.append(c.into());
        output.append(d.into());
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<QM31> {
        let a: M31 = Serde::deserialize(ref serialized)?;
        let b: M31 = Serde::deserialize(ref serialized)?;
        let c: M31 = Serde::deserialize(ref serialized)?;
        let d: M31 = Serde::deserialize(ref serialized)?;
        Some(QM31Trait::from_fixed_array([a, b, c, d]))
    }
}

trait UnreducedQM31Trait {
    fn reduce(self: UnreducedQM31) -> QM31;
}

pub trait PackedUnreducedQM31Trait {
    fn mul_m31(self: PackedUnreducedQM31, rhs: UnreducedM31) -> PackedUnreducedQM31;

    /// Returns a zero element with each coordinate set to `P*P*P`.
    fn large_zero() -> PackedUnreducedQM31;

    fn reduce(self: PackedUnreducedQM31) -> QM31;
}

impl QM31Dispaly of core::fmt::Display<QM31> {
    fn fmt(self: @QM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let [a, b, c, d] = (*self).to_fixed_array();
        let a: u32 = a.into();
        let b: u32 = b.into();
        let c: u32 = c.into();
        let d: u32 = d.into();
        write!(f, "({} + {}i) + ({} + {}i)u", a, b, c, d)
    }
}

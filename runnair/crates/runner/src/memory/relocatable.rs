use std::collections::HashMap;
use std::ops::{Add, Mul};

use num_traits::Zero;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::QM31;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Relocatable {
    segment: isize,
    offset: M31,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaybeRelocatable<T: From<M31>> {
    Relocatable(Relocatable),
    Absolute(T),
}

pub type RelocationTable = HashMap<isize, M31>;

impl Relocatable {
    pub fn relocate(&self, table: &RelocationTable) -> M31 {
        table[&self.segment] + self.offset
    }
}

impl<T: From<M31> + Copy> MaybeRelocatable<T> {
    pub fn relocate(&self, table: &RelocationTable) -> T {
        match self {
            MaybeRelocatable::Relocatable(relocatable) => relocatable.relocate(table).into(),
            MaybeRelocatable::Absolute(absolute) => *absolute,
        }
    }
}

impl From<(isize, M31)> for Relocatable {
    fn from((segment, offset): (isize, M31)) -> Self {
        Relocatable { segment, offset }
    }
}

impl From<(isize, u32)> for Relocatable {
    fn from((segment, offset): (isize, u32)) -> Self {
        Relocatable {
            segment,
            offset: M31(offset),
        }
    }
}

impl<T: From<M31>> From<Relocatable> for MaybeRelocatable<T> {
    fn from(relocatable: Relocatable) -> Self {
        MaybeRelocatable::Relocatable(relocatable)
    }
}

impl<T: From<M31>> From<T> for MaybeRelocatable<T> {
    fn from(value: T) -> Self {
        MaybeRelocatable::Absolute(value)
    }
}

impl Add<M31> for Relocatable {
    type Output = Self;
    fn add(self, rhs: M31) -> Self {
        Self {
            segment: self.segment,
            offset: self.offset + rhs,
        }
    }
}

// TODO(alont): Can this be generalized?
impl Add<MaybeRelocatable<M31>> for MaybeRelocatable<M31> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (MaybeRelocatable::Relocatable(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Relocatable(lhs + rhs)
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Relocatable(rhs)) => {
                MaybeRelocatable::Relocatable(rhs + lhs)
            }
            (MaybeRelocatable::Relocatable(_), MaybeRelocatable::Relocatable(_)) => {
                panic!("Cannot add two relocatables.")
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Absolute(lhs + rhs)
            }
        }
    }
}

/// Assert that the input is in the base field and return the projection to the base field.
pub fn assert_and_project(x: QM31) -> M31 {
    assert!(x.1.is_zero());
    assert!(x.0 .1.is_zero());
    x.0 .0
}

impl Add<MaybeRelocatable<QM31>> for MaybeRelocatable<M31> {
    type Output = MaybeRelocatable<QM31>;
    fn add(self, rhs: MaybeRelocatable<QM31>) -> MaybeRelocatable<QM31> {
        match (self, rhs) {
            (MaybeRelocatable::Relocatable(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Relocatable(lhs + assert_and_project(rhs))
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Relocatable(rhs)) => {
                MaybeRelocatable::Relocatable(rhs + lhs)
            }
            (MaybeRelocatable::Relocatable(_), MaybeRelocatable::Relocatable(_)) => {
                panic!("Cannot add two relocatables.")
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Absolute(lhs + rhs)
            }
        }
    }
}

impl Add<MaybeRelocatable<M31>> for MaybeRelocatable<QM31> {
    type Output = Self;
    fn add(self, rhs: MaybeRelocatable<M31>) -> Self {
        match (self, rhs) {
            (MaybeRelocatable::Relocatable(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Relocatable(lhs + rhs)
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Relocatable(rhs)) => {
                MaybeRelocatable::Relocatable(rhs + assert_and_project(lhs))
            }
            (MaybeRelocatable::Relocatable(_), MaybeRelocatable::Relocatable(_)) => {
                panic!("Cannot add two relocatables.")
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Absolute(lhs + rhs)
            }
        }
    }
}

impl Add<MaybeRelocatable<QM31>> for MaybeRelocatable<QM31> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (MaybeRelocatable::Relocatable(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Relocatable(lhs + assert_and_project(rhs))
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Relocatable(rhs)) => {
                MaybeRelocatable::Relocatable(rhs + assert_and_project(lhs))
            }
            (MaybeRelocatable::Relocatable(_), MaybeRelocatable::Relocatable(_)) => {
                panic!("Cannot add two relocatables.")
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Absolute(lhs + rhs)
            }
        }
    }
}

impl<T: Add<M31, Output = T> + From<M31>> Add<M31> for MaybeRelocatable<T> {
    type Output = Self;
    fn add(self, rhs: M31) -> Self {
        match self {
            MaybeRelocatable::Relocatable(lhs) => MaybeRelocatable::Relocatable(lhs + rhs),
            MaybeRelocatable::Absolute(lhs) => MaybeRelocatable::Absolute(lhs + rhs),
        }
    }
}

impl<T: From<M31> + Mul<S, Output = S>, S: From<M31>> Mul<MaybeRelocatable<S>>
    for MaybeRelocatable<T>
{
    type Output = MaybeRelocatable<S>;
    fn mul(self, rhs: MaybeRelocatable<S>) -> Self::Output {
        match (self, rhs) {
            (MaybeRelocatable::Relocatable(_), _) => {
                panic!("Cannot multiply a relocatable.")
            }
            (_, MaybeRelocatable::Relocatable(_)) => {
                panic!("Cannot multiply a relocatable.")
            }
            (MaybeRelocatable::Absolute(lhs), MaybeRelocatable::Absolute(rhs)) => {
                MaybeRelocatable::Absolute(lhs * rhs)
            }
        }
    }
}

impl<T: From<M31> + Mul<M31, Output = T>> Mul<M31> for MaybeRelocatable<T> {
    type Output = Self;
    fn mul(self, rhs: M31) -> Self {
        match self {
            MaybeRelocatable::Relocatable(_) => panic!("Cannot multiply a relocatable."),
            MaybeRelocatable::Absolute(lhs) => MaybeRelocatable::Absolute(lhs * rhs),
        }
    }
}

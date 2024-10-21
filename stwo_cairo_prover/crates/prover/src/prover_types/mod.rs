use std::ops::{Add, Mul, Sub};
use std::simd::num::SimdUint;
use std::simd::Simd;

use itertools::all;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::fields::m31::{self, M31};

// TODO(Ohad): implement rest of logic.
pub struct PackedBool(pub u16);

// TODO(Ohad): implement rest of logic.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PackedUInt16 {
    pub simd: Simd<u16, N_LANES>,
}

impl PackedUInt16 {
    pub fn from_bool(value: PackedBool) -> Self {
        let shr = Simd::<u16, N_LANES>::from_array([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        ]);
        Self {
            simd: (Simd::splat(value.0) >> shr) & Simd::splat(1),
        }
    }

    pub fn from_m31(felt: PackedM31) -> Self {
        debug_assert!(all(felt.to_array(), |f| f <= M31(0xFFFF)));
        Self {
            simd: felt.into_simd().cast(),
        }
    }

    pub fn as_m31(&self) -> PackedM31 {
        // Safe because u16 is in the range [0, P].
        unsafe { PackedM31::from_simd_unchecked(self.simd.cast()) }
    }
}

impl Add for PackedUInt16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd + rhs.simd,
        }
    }
}

impl Sub for PackedUInt16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd - rhs.simd,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PackedUInt32 {
    pub simd: Simd<u32, N_LANES>,
}

impl PackedUInt32 {
    pub fn low(&self) -> PackedUInt16 {
        PackedUInt16 {
            simd: (self.simd & Simd::splat(0xFFFF)).cast(),
        }
    }

    pub fn high(&self) -> PackedUInt16 {
        PackedUInt16 {
            simd: (self.simd >> 16).cast(),
        }
    }

    pub fn from_m31(felt: PackedM31) -> Self {
        Self {
            simd: felt.into_simd(),
        }
    }

    pub fn from_limbs(low: PackedM31, high: PackedM31) -> Self {
        Self {
            simd: (low.into_simd() & Simd::splat(0xFFFF))
                | ((high.into_simd() & Simd::splat(0xFFFF)) << 16),
        }
    }

    pub fn broadcast(value: u32) -> Self {
        Self {
            simd: Simd::splat(value),
        }
    }
    pub fn from_array(arr: [u32; N_LANES]) -> Self {
        // Safe because UInt32 is u32.
        Self {
            simd: Simd::from_array(arr),
        }
    }

    pub fn from_m31_array(arr: [m31::M31; N_LANES]) -> Self {
        // Safe because M32 is u32.
        Self {
            simd: Simd::from_array(arr.map(|v| v.0)),
        }
    }

    pub fn as_array(&self) -> [u32; N_LANES] {
        // Safe because UInt32 is u32.
        self.simd.to_array()
    }

    pub fn as_m31_unchecked(&self) -> PackedM31 {
        // Safe because M31 is u32 in memory.
        // NOTE: Safety is memory-wise, it is still unchecked and might get invalid M31 values.
        unsafe { PackedM31::from_simd_unchecked(self.simd) }
    }

    pub fn in_m31_range(&self) -> bool {
        all(self.as_array(), |v| v < m31::P)
    }
}

impl Add for PackedUInt32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd + rhs.simd,
        }
    }
}

impl Mul for PackedUInt32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd * rhs.simd,
        }
    }
}

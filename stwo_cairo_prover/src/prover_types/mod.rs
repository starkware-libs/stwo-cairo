use std::ops::Add;
use std::simd::Simd;

use itertools::all;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::fields::m31::{self};

#[derive(Copy, Clone, Debug, Default)]
pub struct PackedUInt32 {
    pub(crate) simd: Simd<u32, N_LANES>,
}

impl PackedUInt32 {
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

    pub fn as_array(&self) -> [u32; N_LANES] {
        // Safe because UInt32 is u32.
        self.simd.to_array()
    }

    pub fn one_hot(idx: usize) -> Self {
        let mut simd = Simd::splat(0);
        simd[idx] = 1;
        Self { simd }
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

pub fn packed_m31_increment() -> PackedM31 {
    let simd = Simd::splat(N_LANES as u32);
    unsafe { PackedM31::from_simd_unchecked(simd) }
}

use std::ops::{Add, Index};
use std::simd::num::SimdUint;
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

pub const N_M31_IN_FELT252: usize = 28;
// TODO(Ohad): implement a proper simd Felt252 type. this is just to get the code to compile.
#[derive(Copy, Clone, Debug)]
pub struct PackedFelt252 {
    pub value: [PackedM31; N_M31_IN_FELT252],
}
impl PackedFelt252 {
    pub fn get_m31(&self, index: usize) -> PackedM31 {
        self[index]
    }
}

impl AsRef<[PackedM31; N_M31_IN_FELT252]> for PackedFelt252 {
    fn as_ref(&self) -> &[PackedM31; N_M31_IN_FELT252] {
        &self.value
    }
}

impl From<[PackedM31; N_M31_IN_FELT252]> for PackedFelt252 {
    fn from(value: [PackedM31; N_M31_IN_FELT252]) -> Self {
        Self { value }
    }
}

impl Index<usize> for PackedFelt252 {
    type Output = PackedM31;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

// TODO(Ohad): implement this, consider in stwo.
pub trait EqExtend {
    fn eq(&self, other: Self) -> PackedBool;
}

impl EqExtend for PackedM31 {
    fn eq(&self, other: Self) -> PackedBool {
        let arr1 = self.to_array();
        let arr2 = other.to_array();
        let cmp: [u8; N_LANES] = std::array::from_fn(|i| (arr1[i] == arr2[i]) as u8);
        PackedBool {
            value: Simd::from(cmp),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PackedBool {
    value: Simd<u8, N_LANES>,
}
impl PackedBool {
    pub fn as_m31(&self) -> PackedM31 {
        // Safe.
        unsafe { PackedM31::from_simd_unchecked(self.value.cast()) }
    }
}

pub struct PackedCasmState {
    pub pc: PackedM31,
    pub ap: PackedM31,
    pub fp: PackedM31,
}

impl From<[PackedM31; 3]> for PackedCasmState {
    fn from(arr: [PackedM31; 3]) -> Self {
        Self {
            pc: arr[0],
            ap: arr[1],
            fp: arr[2],
        }
    }
}

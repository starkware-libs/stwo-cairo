use std::mem::transmute;
use std::ops::{Add, BitAnd, BitOr, BitXor, Rem, Shl, Shr};
use std::simd::num::SimdUint;
use std::simd::Simd;

use bytemuck::Zeroable;
use itertools::all;
use stwo_prover::core::backend::simd::conversion::{Pack, Unpack};
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::fields::m31;

use super::cpu::{UInt16, UInt32, UInt64, PRIME};
use crate::cpu::CasmState;

pub const LOG_N_LANES: u32 = 4;

pub const N_LANES: usize = 1 << LOG_N_LANES;

pub const P_BROADCAST: Simd<u32, N_LANES> = Simd::from_array([PRIME; N_LANES]);

pub trait PackedM31Type {
    fn as_m31(&self) -> PackedM31;
}

#[derive(Clone, Copy, Debug)]
pub struct PackedBool {
    pub(crate) value: Simd<u8, N_LANES>,
}

impl PackedM31Type for PackedBool {
    fn as_m31(&self) -> PackedM31 {
        // Safe.
        unsafe { PackedM31::from_simd_unchecked(self.value.cast()) }
    }
}
#[derive(Copy, Clone, Debug, Default)]
pub struct PackedUInt16 {
    value: Simd<u16, N_LANES>,
}

impl PackedUInt16 {
    pub fn broadcast(value: UInt16) -> Self {
        Self {
            value: Simd::splat(value.value),
        }
    }
    pub fn from_array(arr: [UInt16; N_LANES]) -> Self {
        // Safe because UInt16 is u16.
        unsafe {
            Self {
                value: Simd::from_array(transmute(arr)),
            }
        }
    }
    pub fn as_array(&self) -> [UInt16; N_LANES] {
        // Safe because UInt16 is u16.
        unsafe { transmute(self.value.to_array()) }
    }
    pub fn from_m31(val: PackedM31) -> Self {
        Self {
            value: val.into_simd().cast(),
        }
    }
}

impl PackedM31Type for PackedUInt16 {
    fn as_m31(&self) -> PackedM31 {
        // Safe.
        unsafe { PackedM31::from_simd_unchecked(self.value.cast()) }
    }
}

impl Add for PackedUInt16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Rem for PackedUInt16 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value % rhs.value,
        }
    }
}
impl Shl for PackedUInt16 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value << rhs.value,
        }
    }
}
impl Shr for PackedUInt16 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value >> rhs.value,
        }
    }
}
impl BitAnd for PackedUInt16 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}
impl BitOr for PackedUInt16 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}
impl BitXor for PackedUInt16 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value ^ rhs.value,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PackedUInt32 {
    pub simd: Simd<u32, N_LANES>,
}

impl PackedUInt32 {
    pub fn broadcast(value: UInt32) -> Self {
        Self {
            simd: Simd::splat(value.value),
        }
    }
    pub fn from_array(arr: [UInt32; N_LANES]) -> Self {
        // Safe because UInt32 is u32.
        unsafe {
            Self {
                simd: Simd::from_array(transmute(arr)),
            }
        }
    }

    pub fn as_array(&self) -> [UInt32; N_LANES] {
        // Safe because UInt32 is u32.
        unsafe { transmute(self.simd.to_array()) }
    }

    // TODO(Ohad): remove.
    pub fn as_m31_unchecked(&self) -> PackedM31 {
        // Safe because M31 is u32 in memory.
        // NOTE: Safety is memory-wise, it is still unchecked and might get invalid M31 values.
        unsafe { PackedM31::from_simd_unchecked(self.simd) }
    }

    // TODO(Ohad): remove.
    pub fn in_m31_range(&self) -> bool {
        all(self.as_array(), |v| v.value < m31::P)
    }
}

impl Rem for PackedUInt32 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd % rhs.simd,
        }
    }
}
impl Shl for PackedUInt32 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd << rhs.simd,
        }
    }
}
impl Shr for PackedUInt32 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd >> rhs.simd,
        }
    }
}
impl BitAnd for PackedUInt32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd & rhs.simd,
        }
    }
}
impl BitOr for PackedUInt32 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd | rhs.simd,
        }
    }
}
impl BitXor for PackedUInt32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            simd: self.simd ^ rhs.simd,
        }
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

unsafe impl Zeroable for PackedUInt32 {
    fn zeroed() -> Self {
        Self {
            simd: unsafe { core::mem::zeroed() },
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PackedUInt64 {
    pub(crate) value: Simd<u64, N_LANES>,
}

impl PackedUInt64 {
    pub fn broadcast(value: UInt64) -> Self {
        Self {
            value: Simd::splat(value.value),
        }
    }
    pub fn from_array(arr: [UInt64; N_LANES]) -> Self {
        // Safe because UInt64is u64.
        unsafe {
            Self {
                value: Simd::from_array(transmute(arr)),
            }
        }
    }
    pub fn as_array(&self) -> [UInt64; N_LANES] {
        // Safe because UInt64 is u64.
        unsafe { transmute(self.value.to_array()) }
    }
}

impl Add for PackedUInt64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl Rem for PackedUInt64 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value % rhs.value,
        }
    }
}
impl Shl for PackedUInt64 {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value << rhs.value,
        }
    }
}
impl Shr for PackedUInt64 {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value >> rhs.value,
        }
    }
}
impl BitAnd for PackedUInt64 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
    }
}
impl BitOr for PackedUInt64 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value | rhs.value,
        }
    }
}
impl BitXor for PackedUInt64 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value ^ rhs.value,
        }
    }
}

pub const N_M31_IN_FELT252: usize = 28;

// TODO(Ohad): implement ops and change to non-redundant representation.
#[derive(Copy, Clone, Debug)]
pub struct PackedFelt252 {
    pub value: [PackedM31; N_M31_IN_FELT252],
}
impl PackedFelt252 {
    pub fn get_m31(&self, index: usize) -> PackedM31 {
        self.value[index]
    }
}

impl AsRef<[PackedM31; N_M31_IN_FELT252]> for PackedFelt252 {
    fn as_ref(&self) -> &[PackedM31; N_M31_IN_FELT252] {
        &self.value
    }
}

pub trait EqExtend {
    fn eq(&self, other: Self) -> PackedBool;
}

impl EqExtend for PackedM31 {
    fn eq(&self, _other: Self) -> PackedBool {
        todo!()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PackedCasmState {
    pub pc: PackedM31,
    pub ap: PackedM31,
    pub fp: PackedM31,
}

// TODO(Ohad): When there are more structs, write a proc-macro in stwo.
// TODO(Ohad): Optimize copies.
impl Pack for CasmState {
    type SimdType = PackedCasmState;

    fn pack(inputs: [Self; N_LANES]) -> Self::SimdType {
        PackedCasmState {
            pc: PackedM31::from_array(std::array::from_fn(|i| inputs[i].pc)),
            ap: PackedM31::from_array(std::array::from_fn(|i| inputs[i].ap)),
            fp: PackedM31::from_array(std::array::from_fn(|i| inputs[i].fp)),
        }
    }
}

impl Unpack for PackedCasmState {
    type CpuType = CasmState;

    fn unpack(self) -> [Self::CpuType; N_LANES] {
        let (pc, ap, fp) = (self.pc.to_array(), self.ap.to_array(), self.fp.to_array());
        std::array::from_fn(|i| CasmState {
            pc: pc[i],
            ap: ap[i],
            fp: fp[i],
        })
    }
}

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}

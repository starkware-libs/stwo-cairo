use std::mem::transmute;
use std::ops::{Add, BitAnd, BitOr, BitXor, Rem, Shl, Shr};
use std::simd::num::SimdUint;
use std::simd::Simd;

use bytemuck::Zeroable;
use itertools::all;
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::fields::m31::{self, M31};

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

    pub fn from_m31(_val: PackedM31) -> Self {
        todo!()
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

impl Unpack for PackedCasmState {
    type Output = CasmState;
    fn unpack(self) -> [Self::Output; N_LANES] {
        std::array::from_fn(|i| CasmState {
            pc: self.pc.to_array()[i],
            ap: self.ap.to_array()[i],
            fp: self.fp.to_array()[i],
        })
    }
}

impl Pack for [CasmState; N_LANES] {
    type Output = PackedCasmState;
    fn pack(self) -> Self::Output {
        PackedCasmState {
            pc: PackedM31::from_array(self.map(|c| c.pc)),
            ap: PackedM31::from_array(self.map(|c| c.ap)),
            fp: PackedM31::from_array(self.map(|c| c.fp)),
        }
    }
}

pub trait Unpack {
    type Output;
    fn unpack(self) -> [Self::Output; N_LANES];
}
pub trait Pack {
    type Output;
    fn pack(self) -> Self::Output;
}

pub type VerifyInstructionInput = (M31, [M31; 3], [M31; 15]);
pub type PackedVerifyInstructionInput = (PackedM31, [PackedM31; 3], [PackedM31; 15]);

impl Pack for [VerifyInstructionInput; N_LANES] {
    type Output = PackedVerifyInstructionInput;

    fn pack(self) -> Self::Output {
        let zero = PackedM31::from_array(self.map(|(a, ..)| a));
        let one = std::array::from_fn(|i| PackedM31::from_array(self.map(|(_, b, _)| b[i])));
        let two = std::array::from_fn(|i| PackedM31::from_array(self.map(|(_, _, c)| c[i])));
        (zero, one, two)
    }
}

impl Unpack for PackedVerifyInstructionInput {
    type Output = VerifyInstructionInput;

    fn unpack(self) -> [Self::Output; N_LANES] {
        std::array::from_fn(|i| {
            (
                self.0.to_array()[i],
                std::array::from_fn(|j| self.1[j].to_array()[i]),
                std::array::from_fn(|j| self.2[j].to_array()[i]),
            )
        })
    }
}

impl Pack for [M31; N_LANES] {
    type Output = PackedM31;

    fn pack(self) -> Self::Output {
        PackedM31::from_array(self)
    }
}

impl Unpack for PackedM31 {
    type Output = M31;

    fn unpack(self) -> [Self::Output; N_LANES] {
        self.to_array()
    }
}

impl Pack for [[M31; 2]; N_LANES] {
    type Output = [PackedM31; 2];

    fn pack(self) -> Self::Output {
        [
            PackedM31::from_array(self.map(|[a, _]| a)),
            PackedM31::from_array(self.map(|[_, b]| b)),
        ]
    }
}

impl Unpack for [PackedM31; 2] {
    type Output = [M31; 2];

    fn unpack(self) -> [Self::Output; N_LANES] {
        std::array::from_fn(|i| [self[0].to_array()[i], self[1].to_array()[i]])
    }
}

impl Pack for [[M31; 3]; N_LANES] {
    type Output = [PackedM31; 3];

    fn pack(self) -> Self::Output {
        [
            PackedM31::from_array(self.map(|[a, _, _]| a)),
            PackedM31::from_array(self.map(|[_, b, _]| b)),
            PackedM31::from_array(self.map(|[_, _, c]| c)),
        ]
    }
}

impl Unpack for [PackedM31; 3] {
    type Output = [M31; 3];

    fn unpack(self) -> [Self::Output; N_LANES] {
        std::array::from_fn(|i| {
            [
                self[0].to_array()[i],
                self[1].to_array()[i],
                self[2].to_array()[i],
            ]
        })
    }
}

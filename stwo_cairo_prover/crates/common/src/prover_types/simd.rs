use std::mem::transmute;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};
use std::simd::cmp::SimdPartialEq;
use std::simd::num::{SimdInt, SimdUint};
use std::simd::Simd;

use bytemuck::Zeroable;
use itertools::Itertools;
use stwo::core::fields::FieldExpOps;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;

use super::cpu::{
    BigUInt, CasmState, Felt252, Felt252Width27, UInt16, UInt32, UInt64, FELT252WIDTH27_N_WORDS,
    PRIME,
};
use crate::memory::N_M31_IN_FELT252;

pub const LOG_N_LANES: u32 = 4;

pub const N_LANES: usize = 1 << LOG_N_LANES;

pub const P_BROADCAST: Simd<u32, N_LANES> = Simd::from_array([PRIME; N_LANES]);

pub trait PackedM31Type {
    fn as_m31(&self) -> PackedM31;
}

#[derive(Clone, Copy, Debug)]
pub struct PackedBool {
    pub(crate) value: Simd<i32, N_LANES>,
}
impl PackedBool {
    pub fn from_m31(value: PackedM31) -> Self {
        Self {
            value: value.into_simd().cast().bitand(Simd::splat(1)),
        }
    }
}

impl PackedM31Type for PackedBool {
    fn as_m31(&self) -> PackedM31 {
        // Safe.
        unsafe { PackedM31::from_simd_unchecked(self.value.cast()) }
    }
}

impl BitAnd for PackedBool {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value & rhs.value,
        }
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
                value: Simd::from_array(transmute::<[UInt16; 16], [u16; 16]>(arr)),
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
                simd: Simd::from_array(transmute::<[UInt32; 16], [u32; 16]>(arr)),
            }
        }
    }

    pub fn from_simd(value: Simd<u32, N_LANES>) -> Self {
        Self { simd: value }
    }

    pub fn as_array(&self) -> [UInt32; N_LANES] {
        // Safe because UInt32 is u32.
        unsafe { transmute(self.simd.to_array()) }
    }

    pub fn from_m31(val: PackedM31) -> Self {
        Self {
            simd: val.into_simd(),
        }
    }

    pub fn low(&self) -> PackedUInt16 {
        PackedUInt16 {
            value: (self.simd & Simd::splat(0xFFFF)).cast(),
        }
    }

    pub fn high(&self) -> PackedUInt16 {
        PackedUInt16 {
            value: (self.simd >> 16).cast(),
        }
    }

    pub fn from_limbs([low, high]: [PackedM31; 2]) -> Self {
        let [low, high] = [low, high].map(PackedM31::into_simd);
        Self {
            simd: low + (high << 16),
        }
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

impl Pack for UInt32 {
    type SimdType = PackedUInt32;

    fn pack(inputs: [UInt32; N_LANES]) -> Self::SimdType {
        PackedUInt32::from_array(inputs)
    }
}

impl Unpack for PackedUInt32 {
    type CpuType = UInt32;

    fn unpack(self) -> [Self::CpuType; N_LANES] {
        self.as_array()
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
                value: Simd::from_array(transmute::<[UInt64; 16], [u64; 16]>(arr)),
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

// TODO(Ohad): Change to non-redundant representation.
#[derive(Copy, Clone, Debug)]
pub struct PackedFelt252 {
    pub value: [PackedM31; N_M31_IN_FELT252],
}
impl PackedFelt252 {
    pub fn get_m31(&self, index: usize) -> PackedM31 {
        self.value[index]
    }

    pub fn from_limbs(limbs: [PackedM31; N_M31_IN_FELT252]) -> Self {
        Self { value: limbs }
    }

    pub fn from_m31(val: PackedM31) -> Self {
        Self::from_array(&val.to_array().map(Felt252::from_m31))
    }

    pub fn to_array(&self) -> [Felt252; N_LANES] {
        self.value
            .unpack()
            .each_ref()
            .map(|v| Felt252::from_limbs(v))
    }

    pub fn broadcast(value: Felt252) -> Self {
        Self::from_array(&[value; N_LANES])
    }

    pub fn from_array(arr: &[Felt252; N_LANES]) -> Self {
        let limbs = arr.map(|felt| std::array::from_fn(|i| felt.get_m31(i)));
        Self::from_limbs(<_ as Pack>::pack(limbs))
    }

    pub fn from_packed_felt252width27(packed27: PackedFelt252Width27) -> Self {
        Self::from_array(&std::array::from_fn(|i| Felt252::from(packed27.value[i])))
    }
}

// TODO(Ohad): These are very slow, optimize.
impl Add for PackedFelt252 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self.to_array();
        let rhs = rhs.to_array();
        let result = std::array::from_fn(|i| lhs[i] + rhs[i]);
        Self::from_array(&result)
    }
}

impl Sub for PackedFelt252 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self.to_array();
        let rhs = rhs.to_array();
        let result = std::array::from_fn(|i| lhs[i] - rhs[i]);
        Self::from_array(&result)
    }
}

impl Mul for PackedFelt252 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self.to_array();
        let rhs = rhs.to_array();
        let result = std::array::from_fn(|i| lhs[i] * rhs[i]);
        Self::from_array(&result)
    }
}

// TODO(ohadn): implement with batch_inverse
impl Div for PackedFelt252 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs = self.to_array();
        let rhs = rhs.to_array();
        let result = std::array::from_fn(|i| lhs[i] / rhs[i]);
        Self::from_array(&result)
    }
}

impl PartialEq for PackedFelt252 {
    fn eq(&self, other: &Self) -> bool {
        self.to_array() == other.to_array()
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
    fn eq(&self, other: Self) -> PackedBool {
        PackedBool {
            value: self
                .into_simd()
                .simd_eq(other.into_simd())
                .to_int()
                .bitand(Simd::splat(1))
                .cast(),
        }
    }
}
pub trait DivExtend {
    fn div(&self, other: Self) -> Self;
}

impl DivExtend for PackedM31 {
    fn div(&self, rhs: Self) -> Self {
        *self * rhs.inverse()
    }
}

#[derive(Clone, Copy)]
pub struct PackedFelt252Width27 {
    value: [Felt252Width27; N_LANES],
}
impl PackedFelt252Width27 {
    pub fn to_array(&self) -> [Felt252Width27; N_LANES] {
        self.value
    }

    pub fn from_array(arr: [Felt252Width27; N_LANES]) -> Self {
        Self { value: arr }
    }

    pub fn from_limbs(limbs: [PackedM31; FELT252WIDTH27_N_WORDS]) -> Self {
        let limbs = limbs.unpack();
        Self {
            value: std::array::from_fn(|i| Felt252Width27::from_limbs(&limbs[i])),
        }
    }

    pub fn from_packed_felt252width27(other: PackedFelt252Width27) -> Self {
        Self { value: other.value }
    }

    pub fn from_packed_felt252(other: PackedFelt252) -> Self {
        Self {
            value: other.to_array().map(Felt252Width27::from),
        }
    }

    pub fn get_m31(&self, index: usize) -> PackedM31 {
        PackedM31::from_array(std::array::from_fn(|i| self.value[i].get_m31(index)))
    }
}
impl Pack for Felt252Width27 {
    type SimdType = PackedFelt252Width27;

    fn pack(inputs: [Self; N_LANES]) -> Self::SimdType {
        PackedFelt252Width27 { value: inputs }
    }
}

impl Unpack for PackedFelt252Width27 {
    type CpuType = Felt252Width27;

    fn unpack(self) -> [Self::CpuType; N_LANES] {
        self.value
    }
}

// TODO(Gali): Change to an efficient implementation.
#[derive(Copy, Clone, Debug)]
pub struct PackedBigUInt<const B: usize, const L: usize, const F: usize> {
    value: [BigUInt<B, L, F>; N_LANES],
}
impl<const B: usize, const L: usize, const F: usize> PackedBigUInt<B, L, F> {
    pub fn to_array(&self) -> [BigUInt<B, L, F>; N_LANES] {
        self.value
    }

    pub fn broadcast(value: BigUInt<B, L, F>) -> Self {
        Self {
            value: [value; N_LANES],
        }
    }

    pub fn from_packed_felt252_array(felts: &[PackedFelt252]) -> Self {
        let felts = felts.iter().map(|felt| felt.to_array()).collect_vec();
        let value = std::array::from_fn(|i| {
            BigUInt::from_felt252_array(felts.iter().map(|felt| felt[i]).collect_vec())
        });
        Self { value }
    }

    pub fn from_packed_biguint<const OB: usize, const OL: usize, const OF: usize>(
        other: PackedBigUInt<OB, OL, OF>,
    ) -> Self {
        let result: [BigUInt<B, L, F>; N_LANES] =
            std::array::from_fn(|i| BigUInt::<B, L, F>::from_biguint(other.value[i]));
        Self { value: result }
    }

    pub fn widening_mul<const DB: usize, const DL: usize, const DF: usize>(
        self,
        rhs: PackedBigUInt<B, L, F>,
    ) -> PackedBigUInt<DB, DL, DF> {
        let result = std::array::from_fn(|i| self.value[i].widening_mul(rhs.value[i]));
        PackedBigUInt::<DB, DL, DF> { value: result }
    }

    pub fn get_m31(&self, index: usize) -> PackedM31 {
        PackedM31::from_array(std::array::from_fn(|i| self.value[i].get_m31(index)))
    }
}
impl<const B: usize, const L: usize, const F: usize> Add for PackedBigUInt<B, L, F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs = self.value;
        let rhs = rhs.value;
        let result = std::array::from_fn(|i| lhs[i] + rhs[i]);
        Self { value: result }
    }
}
impl<const B: usize, const L: usize, const F: usize> Sub for PackedBigUInt<B, L, F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self.value;
        let rhs = rhs.value;
        let result = std::array::from_fn(|i| lhs[i] - rhs[i]);
        Self { value: result }
    }
}
impl<const B: usize, const L: usize, const F: usize> Mul for PackedBigUInt<B, L, F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self.value;
        let rhs = rhs.value;
        let result = std::array::from_fn(|i| lhs[i] * rhs[i]);
        Self { value: result }
    }
}
impl<const B: usize, const L: usize, const F: usize> Div for PackedBigUInt<B, L, F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let lhs = self.value;
        let rhs = rhs.value;
        let result = std::array::from_fn(|i| lhs[i].div(rhs[i]));
        Self { value: result }
    }
}
impl<const B: usize, const L: usize, const F: usize> EqExtend for PackedBigUInt<B, L, F> {
    fn eq(&self, other: Self) -> PackedBool {
        let lhs = self.value;
        let rhs = other.value;
        PackedBool {
            value: Simd::from_array(std::array::from_fn(|i| lhs[i].eq(&rhs[i]) as i32)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PackedCasmState {
    pub pc: PackedM31,
    pub ap: PackedM31,
    pub fp: PackedM31,
}

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

impl Pack for Felt252 {
    type SimdType = PackedFelt252;

    fn pack(inputs: [Self; N_LANES]) -> Self::SimdType {
        PackedFelt252::from_array(&inputs)
    }
}

impl Unpack for PackedFelt252 {
    type CpuType = Felt252;

    fn unpack(self) -> [Self::CpuType; N_LANES] {
        self.to_array()
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::m31::{M31, P};
    type BigUInt384 = BigUInt<384, 6, 32>;
    type PackedBigUInt384 = PackedBigUInt<384, 6, 32>;
    type BigUInt768 = BigUInt<768, 12, 64>;
    type PackedBigUInt768 = PackedBigUInt<768, 12, 64>;

    use super::*;

    fn rand_f252(rng: &mut SmallRng) -> Felt252 {
        Felt252 {
            limbs: [rng.gen(), rng.gen(), rng.gen(), 0],
        }
    }

    fn rand_packed_f252(rng: &mut SmallRng) -> PackedFelt252 {
        PackedFelt252::from_array(&std::array::from_fn(|_| rand_f252(rng)))
    }

    #[test]
    fn test_f252_to_array() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let arr = std::array::from_fn(|_| rand_f252(&mut rng));
        let a = PackedFelt252::from_array(&arr);

        let unpacked_a = a.to_array();

        for i in 0..N_LANES {
            assert_eq!(arr[i], unpacked_a[i]);
        }
    }

    #[test]
    fn test_packed_f252_from_m31() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let input = std::array::from_fn(|_| M31(rng.gen::<u32>() % P));
        let expected = input.map(Felt252::from_m31);

        let packed_from = PackedFelt252::from_m31(PackedM31::from_array(input));

        assert_eq!(packed_from.to_array(), expected);
    }

    #[test]
    fn test_div_extend() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let a = std::array::from_fn(|_| M31(rng.gen::<u32>() % P));
        let b = std::array::from_fn(|_| M31(rng.gen::<u32>() % P));
        let expected_div = std::array::from_fn(|i| a[i] / b[i]);

        let div_result = PackedM31::from_array(a).div(PackedM31::from_array(b));

        assert_eq!(div_result.to_array(), expected_div);
    }

    #[test]
    fn test_packed_f252_ops() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let a = rand_packed_f252(&mut rng);
        let b = rand_packed_f252(&mut rng);
        let unpacked_a = a.to_array();
        let unpacked_b = b.to_array();
        let expected_add = std::array::from_fn(|i| unpacked_a[i] + unpacked_b[i]);
        let expected_sub = std::array::from_fn(|i| unpacked_a[i] - unpacked_b[i]);
        let expected_mul = std::array::from_fn(|i| unpacked_a[i] * unpacked_b[i]);

        let add = a + b;
        let sub = a - b;
        let mul = a * b;

        assert_eq!(add.to_array(), expected_add);
        assert_eq!(sub.to_array(), expected_sub);
        assert_eq!(mul.to_array(), expected_mul);
    }

    #[test]
    fn test_packed_biguint_from_packed_felt252_array() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let felts: Vec<Felt252> = (0..4).map(|_| rand_f252(&mut rng)).collect();
        let big_uint = BigUInt384::from_felt252_array(felts.clone());

        let packed_felts: Vec<PackedFelt252> = felts
            .into_iter()
            .map(|felt| PackedFelt252::from_array(&[felt; N_LANES]))
            .collect_vec();
        let packed_big_uint = PackedBigUInt384::from_packed_felt252_array(&packed_felts);

        for i in 0..32 {
            assert_eq!(packed_big_uint.to_array(), [big_uint; N_LANES], "i = {i}");
        }
    }

    #[test]
    fn test_packed_biguint_from_packed_biguint() {
        let big_uint = BigUInt384::from_felt252(Felt252::from_limbs(&[
            M31::from_u32_unchecked(32425),
            M31::from_u32_unchecked(1429),
            M31::from_u32_unchecked(243987),
            M31::from_u32_unchecked(63),
            M31::from_u32_unchecked(94753),
        ]));
        let expected_packed_big_uint = PackedBigUInt384::broadcast(big_uint);

        let packed_big_uint = PackedBigUInt384::from_packed_biguint(
            PackedBigUInt768::from_packed_biguint(expected_packed_big_uint),
        );
        let packed_big_uint_2 = PackedBigUInt384::from_packed_biguint(PackedBigUInt768::broadcast(
            BigUInt768::from_biguint(big_uint),
        ));

        assert_eq!(
            packed_big_uint.to_array(),
            expected_packed_big_uint.to_array()
        );
        assert_eq!(
            packed_big_uint_2.to_array(),
            expected_packed_big_uint.to_array()
        );
    }

    #[test]
    fn test_packed_biguint_widening_mul() {
        let big_uint = BigUInt384::from_felt252(Felt252::from_limbs(&[
            M31::from_u32_unchecked(32425),
            M31::from_u32_unchecked(1429),
            M31::from_u32_unchecked(243987),
            M31::from_u32_unchecked(63),
            M31::from_u32_unchecked(94753),
        ]));
        let packed_big_uint = PackedBigUInt384::broadcast(big_uint);

        let result = packed_big_uint.widening_mul::<768, 12, 64>(packed_big_uint);
        let expected_result = [big_uint.widening_mul::<768, 12, 64>(big_uint); N_LANES];

        assert_eq!(result.to_array(), expected_result);
    }

    #[test]
    fn test_packed_biguint_get_m31() {
        let big_uint = BigUInt384::from_felt252(Felt252::from_limbs(&[
            M31::from_u32_unchecked(32425),
            M31::from_u32_unchecked(1429),
            M31::from_u32_unchecked(243987),
            M31::from_u32_unchecked(63),
            M31::from_u32_unchecked(94753),
        ]));
        let packed_big_uint = PackedBigUInt384::broadcast(big_uint);
        for i in 0..32 {
            assert_eq!(
                packed_big_uint.get_m31(i).to_array(),
                [big_uint.get_m31(i); N_LANES],
                "i = {i}"
            );
        }
    }

    #[test]
    fn test_packed_biguint_ops() {
        let unpacked_a = BigUInt384::from_felt252(Felt252::from_limbs(&[
            M31::from_u32_unchecked(32425),
            M31::from_u32_unchecked(1429),
            M31::from_u32_unchecked(243987),
            M31::from_u32_unchecked(63),
            M31::from_u32_unchecked(94753),
        ]));
        let unpacked_b = BigUInt384::from_felt252(Felt252::from_limbs(&[
            M31::from_u32_unchecked(34),
            M31::from_u32_unchecked(2876),
            M31::from_u32_unchecked(943),
            M31::from_u32_unchecked(354832),
            M31::from_u32_unchecked(86760),
        ]));
        let a = PackedBigUInt384::broadcast(unpacked_a);
        let b = PackedBigUInt384::broadcast(unpacked_b);
        let expected_add = [unpacked_a + unpacked_b; N_LANES];
        let expected_sub = [unpacked_a - unpacked_b; N_LANES];
        let expected_mul = [unpacked_a * unpacked_b; N_LANES];
        let expected_div = [unpacked_a.div(unpacked_b); N_LANES];

        let add = a + b;
        let sub = a - b;
        let mul = a * b;
        let div = a / b;

        assert_eq!(add.to_array(), expected_add);
        assert_eq!(sub.to_array(), expected_sub);
        assert_eq!(mul.to_array(), expected_mul);
        assert_eq!(div.to_array(), expected_div);
        assert!(a.eq(b).value.to_array().iter().all(|&x| x == 0));
        assert!(a.eq(a).value.to_array().iter().all(|&x| x == 1));
    }
}

use std::ops::{Add, Mul, Sub};
use std::simd::num::SimdUint;
use std::simd::Simd;

use itertools::all;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::fields::m31::{self, M31};

use crate::input::instructions::VmState;

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

#[derive(Copy, Clone, Debug)]
pub struct PackedCasmState {
    pub pc: PackedM31,
    pub ap: PackedM31,
    pub fp: PackedM31,
}
impl PackedCasmState {
    pub fn from_vm_state_array(arr: [VmState; N_LANES]) -> Self {
        let pc = PackedM31::from_array(std::array::from_fn(|i| M31(arr[i].pc)));
        let ap = PackedM31::from_array(std::array::from_fn(|i| M31(arr[i].ap)));
        let fp = PackedM31::from_array(std::array::from_fn(|i| M31(arr[i].fp)));
        Self { pc, ap, fp }
    }
}

pub const FELT252_N_WORDS: usize = 28;
pub const FELT252_BITS_PER_WORD: usize = 9;

// TODO(Ohad): impl ops.
pub struct PackedFelt252 {
    pub limbs: [Simd<u64, N_LANES>; 4],
}

impl PackedFelt252 {
    pub fn get_m31(&self, index: usize) -> PackedM31 {
        const MASK: u64 = (1 << FELT252_BITS_PER_WORD) - 1;
        let shift = FELT252_BITS_PER_WORD * index;
        let low_limb = shift / 64;
        let shift_low = shift as u64 & 0x3F;
        let high_limb = (shift + FELT252_BITS_PER_WORD - 1) / 64;
        let value: Simd<u32, N_LANES> = if low_limb == high_limb {
            ((self.limbs[low_limb] >> shift_low) & Simd::splat(MASK)).cast()
        } else {
            (((self.limbs[low_limb] >> (shift_low)) | (self.limbs[high_limb] << (64 - shift_low)))
                & Simd::splat(MASK))
            .cast()
        };
        // Safe because M31 is u32.
        unsafe { PackedM31::from_simd_unchecked(value) }
    }

    pub fn from_m31(felt: PackedM31) -> Self {
        Self {
            limbs: [
                felt.into_simd().cast(),
                Simd::splat(0),
                Simd::splat(0),
                Simd::splat(0),
            ],
        }
    }

    pub fn from_m31_limbs(felts: [PackedM31; FELT252_N_WORDS]) -> Self {
        let mut limbs = [Simd::splat(0); 4];
        for (index, felt) in felts.iter().enumerate() {
            let shift = FELT252_BITS_PER_WORD * index;
            let low_limb = shift / 64;
            let shift_low = shift as u64 & 0x3F;
            let high_limb = (shift + FELT252_BITS_PER_WORD - 1) / 64;
            let value: Simd<u64, N_LANES> = felt.into_simd().cast();
            limbs[low_limb] |= value << shift_low;
            if low_limb != high_limb {
                limbs[high_limb] |= value >> (64 - shift_low);
            }
        }
        Self { limbs }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::fields::m31::M31;

    use super::FELT252_N_WORDS;
    use crate::components::memory::N_BITS_PER_FELT;

    #[test]
    fn test_from_and_to_limbs() {
        let mut rng = SmallRng::seed_from_u64(0);
        let limbs: [PackedM31; FELT252_N_WORDS] = std::array::from_fn(|_| {
            PackedM31::from_array(std::array::from_fn(|_| {
                M31(rng.gen::<u32>() & ((1 << N_BITS_PER_FELT) - 1))
            }))
        });

        let felt252 = super::PackedFelt252::from_m31_limbs(limbs);
        for (i, limb) in limbs.iter().enumerate() {
            assert_eq!(felt252.get_m31(i).to_array(), limb.to_array());
        }
    }
}

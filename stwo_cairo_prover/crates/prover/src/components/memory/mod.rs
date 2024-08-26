mod addr_to_id;
mod id_to_big;
mod instruction_mem;

use std::simd::cmp::SimdOrd;
use std::simd::num::SimdInt;
use std::simd::Simd;

pub use addr_to_id::{AddrToIdBuilder, AddrToIdClaim, AddrToIdComponent, AddrToIdProver};
pub use id_to_big::{IdToBigBuilder, IdToBigClaim, IdToBigComponent, IdToBigProver};
pub use instruction_mem::{InstMemBuilder, InstMemClaim, InstMemComponent, InstMemProver};
use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::backend::simd::m31::{self, PackedM31, N_LANES};
use stwo_prover::core::channel::Channel;

pub const N_MEM_BIG_LIMBS: usize = 14;
pub const N_MEM_BIG_LIMB_BITS: usize = 18;
pub const N_INSTR_LIMBS: usize = 4;

#[derive(Clone)]
pub struct MemoryElements {
    pub addr_to_id: LookupElements<2>,
    pub id_to_big: LookupElements<{ 1 + N_MEM_BIG_LIMBS }>,
    pub instructions: LookupElements<{ 1 + N_INSTR_LIMBS }>,
}
impl MemoryElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            addr_to_id: LookupElements::draw(channel),
            id_to_big: LookupElements::draw(channel),
            instructions: LookupElements::draw(channel),
        }
    }

    pub fn dummy() -> MemoryElements {
        Self {
            addr_to_id: LookupElements::dummy(),
            id_to_big: LookupElements::dummy(),
            instructions: LookupElements::dummy(),
        }
    }
}

pub fn m31_from_i32(x: Simd<i32, N_LANES>) -> PackedM31 {
    // Cast to unsigned.
    let x: Simd<u32, N_LANES> = x.cast();
    let x = Simd::simd_min(x, x + m31::MODULUS);
    unsafe { PackedM31::from_simd_unchecked(x) }
}

use std::simd::Simd;

use stwo_prover::core::backend::simd::m31::N_LANES;
use stwo_prover::core::channel::Channel;

use super::memory::{AddrToIdProverBuilder, InstructionElements, MemoryElements};
use super::range_check::{RangeElements, RangeProver};
use crate::input::instructions::VmState;

// pub mod component;
// pub mod prover;

pub struct PackedVmState {
    pub pc: Simd<u32, N_LANES>,
    pub ap: Simd<u32, N_LANES>,
    pub fp: Simd<u32, N_LANES>,
}
impl From<[VmState; N_LANES]> for PackedVmState {
    fn from(value: [VmState; N_LANES]) -> Self {
        PackedVmState {
            pc: Simd::from_array(std::array::from_fn(|i| value[i].pc)),
            ap: Simd::from_array(std::array::from_fn(|i| value[i].ap)),
            fp: Simd::from_array(std::array::from_fn(|i| value[i].fp)),
        }
    }
}

#[derive(Clone)]
pub struct OpcodeElements {
    pub memory_elements: MemoryElements,
    pub instruction_elements: InstructionElements,
    pub range_elements: RangeElements,
}
impl OpcodeElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            memory_elements: MemoryElements::draw(channel),
            instruction_elements: InstructionElements::draw(channel),
            range_elements: RangeElements::draw(channel),
        }
    }
}

// Move
#[allow(dead_code)]
pub struct CpuRangeElements {
    range2: RangeElements,
    range3: RangeElements,
}

pub struct OpcodeGenContext<'a> {
    pub addr_to_id: &'a mut AddrToIdProverBuilder,
    pub range: CpuRangeProvers<'a>,
}

// Move
pub struct CpuRangeProvers<'a> {
    pub range2: &'a mut RangeProver,
    pub range3: &'a mut RangeProver,
}

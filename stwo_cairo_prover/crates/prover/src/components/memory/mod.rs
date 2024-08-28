pub mod addr_to_id;
pub mod id_to_big;
pub mod instruction_mem;

use std::simd::cmp::SimdOrd;
use std::simd::num::SimdInt;
use std::simd::Simd;

use instruction_mem::InstMemCtx;
use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::m31::{self, PackedM31, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::opcode::CpuRangeElements;
use crate::input::mem::Memory;

pub const N_MEM_BIG_LIMBS: usize = 14;
pub const N_MEM_BIG_LIMB_BITS: usize = 18;
pub const N_INSTR_LIMBS: usize = 4;

#[derive(Clone)]
pub struct MemoryElements {
    pub instructions: LookupElements<{ 1 + N_INSTR_LIMBS }>,
    pub addr_to_id: LookupElements<2>,
    pub id_to_big: LookupElements<{ 1 + N_MEM_BIG_LIMBS }>,
}
impl MemoryElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            instructions: LookupElements::draw(channel),
            addr_to_id: LookupElements::draw(channel),
            id_to_big: LookupElements::draw(channel),
        }
    }

    pub fn dummy() -> MemoryElements {
        Self {
            instructions: LookupElements::dummy(),
            addr_to_id: LookupElements::dummy(),
            id_to_big: LookupElements::dummy(),
        }
    }
}

#[derive(Clone)]
pub struct MemoryAndRangeElements {
    pub mem: MemoryElements,
    pub range: CpuRangeElements,
}
impl MemoryAndRangeElements {
    pub fn dummy() -> MemoryAndRangeElements {
        Self {
            mem: MemoryElements::dummy(),
            range: CpuRangeElements::dummy(),
        }
    }
}

pub fn m31_from_i32(x: Simd<i32, N_LANES>) -> PackedM31 {
    // Cast to unsigned.
    let x: Simd<u32, N_LANES> = x.cast();
    let x = Simd::simd_min(x, x + m31::MODULUS);
    unsafe { PackedM31::from_simd_unchecked(x) }
}

pub struct MemoryProver {
    pub instruction_mem: instruction_mem::InstMemBuilder,
    pub addr_to_id: addr_to_id::AddrToIdBuilder,
    pub id_to_big: id_to_big::IdToBigBuilder,
}
impl MemoryProver {
    pub fn new(mem: &Memory) -> Self {
        Self {
            addr_to_id: addr_to_id::AddrToIdBuilder::new(mem),
            id_to_big: id_to_big::IdToBigBuilder::new(mem),
            instruction_mem: instruction_mem::InstMemBuilder::new(),
        }
    }
    pub fn generate(
        mut self,
        mem: Memory,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (MemoryClaim, MemoryInteractionProver) {
        let inst_mem = self.instruction_mem.build();
        let (inst_mem_claim, inst_mem) = inst_mem.write_trace(
            tree_builder,
            &mut InstMemCtx {
                addr_to_id: &mut self.addr_to_id,
                id_to_big: &mut self.id_to_big,
                mem: &mem,
            },
        );

        let addr_to_id = self.addr_to_id.build(mem.address_to_id);
        let (addr_to_id_claim, addr_to_id) = addr_to_id.write_trace(tree_builder, &mut ());

        let id_to_big = self.id_to_big.build(mem.f252_values);
        let (id_to_big_claim, id_to_big) = id_to_big.write_trace(tree_builder, &mut ());

        (
            MemoryClaim {
                instruction_mem: inst_mem_claim,
                addr_to_id: addr_to_id_claim,
                id_to_big: id_to_big_claim,
            },
            MemoryInteractionProver {
                inst_mem,
                addr_to_id,
                id_to_big,
            },
        )
    }
}

#[derive(Clone)]
pub struct MemoryClaim {
    pub addr_to_id: addr_to_id::AddrToIdClaim,
    pub id_to_big: id_to_big::IdToBigClaim,
    pub instruction_mem: instruction_mem::InstMemClaim,
}
impl MemoryClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.instruction_mem.mix_into(channel);
        self.addr_to_id.mix_into(channel);
        self.id_to_big.mix_into(channel);
    }
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(
            [
                self.instruction_mem.log_sizes(),
                self.addr_to_id.log_sizes(),
                self.id_to_big.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

pub struct MemoryInteractionProver {
    pub inst_mem: instruction_mem::InstMemInteractionProver,
    pub addr_to_id: addr_to_id::AddrToIdInteractionProver,
    pub id_to_big: id_to_big::IdToBigInteractionProver,
}
impl MemoryInteractionProver {
    pub fn generate(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        els: &MemoryAndRangeElements,
    ) -> MemoryInteractionClaim {
        let inst_mem = self.inst_mem.write_interaction_trace(tree_builder, els);
        let addr_to_id = self
            .addr_to_id
            .write_interaction_trace(tree_builder, &els.mem.addr_to_id);
        let id_to_big = self.id_to_big.write_interaction_trace(tree_builder, els);
        MemoryInteractionClaim {
            inst_mem,
            addr_to_id,
            id_to_big,
        }
    }
}

pub struct MemoryInteractionClaim {
    pub inst_mem: instruction_mem::InstMemInteractionClaim,
    pub addr_to_id: addr_to_id::AddrToIdInteractionClaim,
    pub id_to_big: id_to_big::IdToBigInteractionClaim,
}
impl MemoryInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.inst_mem.mix_into(channel);
        self.addr_to_id.mix_into(channel);
        self.id_to_big.mix_into(channel);
    }
    pub fn logup_sum(&self) -> SecureField {
        self.inst_mem.logup_sum() + self.addr_to_id.logup_sum() + self.id_to_big.logup_sum()
    }
}

pub struct MemoryComponents {
    pub inst_mem: instruction_mem::InstMemComponent,
    pub addr_to_id: addr_to_id::AddrToIdComponent,
    pub id_to_big: id_to_big::IdToBigComponent,
}
impl MemoryComponents {
    pub fn new(
        claim: &MemoryClaim,
        els: &MemoryAndRangeElements,
        interaction_claim: &MemoryInteractionClaim,
    ) -> Self {
        Self {
            inst_mem: instruction_mem::InstMemComponent::new(
                &claim.instruction_mem,
                els.clone(),
                &interaction_claim.inst_mem,
            ),
            addr_to_id: addr_to_id::AddrToIdComponent::new(
                &claim.addr_to_id,
                els.mem.clone(),
                &interaction_claim.addr_to_id,
            ),
            id_to_big: id_to_big::IdToBigComponent::new(
                &claim.id_to_big,
                els.clone(),
                &interaction_claim.id_to_big,
            ),
        }
    }
    pub fn provers(&self) -> impl Iterator<Item = &dyn ComponentProver<SimdBackend>> {
        let res: [&dyn ComponentProver<_>; 3] = [&self.inst_mem, &self.addr_to_id, &self.id_to_big];
        res.into_iter()
    }
    pub fn components(&self) -> impl Iterator<Item = &dyn Component> {
        let res: [&dyn Component; 3] = [&self.inst_mem, &self.addr_to_id, &self.id_to_big];
        res.into_iter()
    }
}

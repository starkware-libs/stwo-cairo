pub mod generic;
pub mod ret;

use std::simd::Simd;

use generic::{GenericOpcodeClaim, GenericOpcodeInteractionProver, GenericOpcodeProver};
use ret::{RetOpcode, RetOpcodeClaim, RetOpcodeInteractionProver, RetOpcodeProver};
use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::m31::N_LANES;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use stwo_prover::core::poly::circle::CirclePoly;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::memory::{MemoryElements, MemoryProver};
use super::range_check::RangeProver;
use super::{StandardComponent, StandardInteractionClaim};
use crate::input::instructions::{Instructions, VmState};
use crate::input::mem::Memory;

pub fn cairo_offset(val: i32) -> M31 {
    M31::from(val + (1 << 15))
}

pub struct PackedVmState {
    pub pc: Simd<u32, N_LANES>,
    pub ap: Simd<u32, N_LANES>,
    pub fp: Simd<u32, N_LANES>,
}
impl PackedVmState {
    pub fn first(&self) -> VmState {
        VmState {
            pc: self.pc.to_array()[0],
            ap: self.ap.to_array()[0],
            fp: self.fp.to_array()[0],
        }
    }
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

pub type StateElements = LookupElements<3>;
#[derive(Clone)]
pub struct OpcodeElements {
    pub mem: MemoryElements,
    pub range: CpuRangeElements,
    pub state: StateElements,
}
impl OpcodeElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            mem: MemoryElements::draw(channel),
            range: CpuRangeElements::draw(channel),
            state: LookupElements::draw(channel),
        }
    }

    pub fn dummy() -> OpcodeElements {
        Self {
            mem: MemoryElements::dummy(),
            range: CpuRangeElements::dummy(),
            state: LookupElements::dummy(),
        }
    }
}

// Move
#[derive(Clone)]
pub struct CpuRangeElements {
    // 18.
    pub rc18: LookupElements<2>,
    pub rc9_9: LookupElements<2>,
    // 16.
    pub rc16: LookupElements<1>,
    pub rc2_14: LookupElements<2>,
    pub rc4_12: LookupElements<2>,
    // 15.
    pub rc6_9: LookupElements<2>,
}
impl CpuRangeElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self {
            rc18: LookupElements::draw(channel),
            rc9_9: LookupElements::draw(channel),
            rc16: LookupElements::draw(channel),
            rc2_14: LookupElements::draw(channel),
            rc4_12: LookupElements::draw(channel),
            rc6_9: LookupElements::draw(channel),
        }
    }

    pub fn dummy() -> CpuRangeElements {
        Self {
            rc18: LookupElements::dummy(),
            rc9_9: LookupElements::dummy(),
            rc16: LookupElements::dummy(),
            rc2_14: LookupElements::dummy(),
            rc4_12: LookupElements::dummy(),
            rc6_9: LookupElements::dummy(),
        }
    }
}

pub struct OpcodeGenContext<'a> {
    pub mem_builder: &'a mut MemoryProver,
    pub mem: &'a Memory,
    pub range: &'a mut CpuRangeProvers,
}

// Move
pub struct CpuRangeProvers {
    pub range2: RangeProver,
    pub range3: RangeProver,
}

#[derive(Clone)]
pub struct OpcodesClaim {
    pub ret: RetOpcodeClaim,
    pub generic: GenericOpcodeClaim,
    pub extra_transitions: Vec<(u32, [VmState; 2])>,
}
impl OpcodesClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.mix_into(channel);
        self.generic.mix_into(channel);
    }
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols([self.ret.log_sizes(), self.generic.log_sizes()].into_iter())
    }
}

#[derive(Clone)]
pub struct OpcodesInteractionClaim {
    pub ret: StandardInteractionClaim,
    pub generic: StandardInteractionClaim,
}
impl OpcodesInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.mix_into(channel);
        self.generic.mix_into(channel);
    }

    pub fn logup_sum(&self) -> SecureField {
        self.ret.logup_sum() + self.generic.logup_sum()
    }
}

pub struct OpcodesProvers {
    pub ret: RetOpcodeProver,
    pub generic: GenericOpcodeProver,
}
impl OpcodesProvers {
    pub fn new(instructions: Instructions) -> Self {
        Self {
            ret: RetOpcodeProver::new((), instructions.ret.into_iter())
                .pop()
                .unwrap(),
            generic: GenericOpcodeProver::new((), instructions.generic.into_iter())
                .pop()
                .unwrap(),
        }
    }
    pub fn generate(
        self,
        mut ctx: OpcodeGenContext<'_>,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (OpcodesClaim, OpcodesInteractionProvers) {
        let mut extra_transitions = Vec::new();
        extra_transitions.push(self.ret.pad_transition(self.ret.inputs[0].first(), ctx.mem));
        let (ret_claim, ret) = self.ret.write_trace(tree_builder, &mut ctx);
        extra_transitions.push((
            self.generic.n_padding(),
            [
                self.generic.inputs[0].0[0].first(),
                self.generic.inputs[0].0[1].first(),
            ],
        ));
        let (generic_claim, generic) = self.generic.write_trace(tree_builder, &mut ctx);
        (
            OpcodesClaim {
                ret: ret_claim,
                generic: generic_claim,
                extra_transitions,
            },
            OpcodesInteractionProvers { ret, generic },
        )
    }
}

pub struct OpcodesInteractionProvers {
    pub ret: RetOpcodeInteractionProver,
    pub generic: GenericOpcodeInteractionProver,
}
impl OpcodesInteractionProvers {
    pub fn generate(
        self,
        elements: &OpcodeElements,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> OpcodesInteractionClaim {
        let ret = self.ret.write_interaction_trace(tree_builder, elements);
        let generic = self.generic.write_interaction_trace(tree_builder, elements);
        OpcodesInteractionClaim { ret, generic }
    }
}

pub struct OpcodesComponents {
    pub ret: StandardComponent<RetOpcode>,
}
impl OpcodesComponents {
    pub fn new(
        claim: &OpcodesClaim,
        elements: &OpcodeElements,
        interaction_claim: &OpcodesInteractionClaim,
    ) -> Self {
        Self {
            ret: StandardComponent::new(&claim.ret, elements.clone(), &interaction_claim.ret),
        }
    }
    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![&self.ret]
    }
    pub fn components(&self) -> Vec<&dyn Component> {
        vec![&self.ret]
    }

    pub fn assert_constraints(
        &self,
        trace_polys: &mut TreeVec<impl Iterator<Item = CirclePoly<SimdBackend>>>,
    ) {
        self.ret.assert_constraints(trace_polys);
    }
}

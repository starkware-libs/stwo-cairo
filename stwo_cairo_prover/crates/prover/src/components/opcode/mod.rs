pub mod generic;
pub mod jmp_abs;
pub mod ret;

use std::simd::Simd;

use generic::{
    GenericOpcodeClaim, GenericOpcodeComponent, GenericOpcodeInteractionProver, GenericOpcodeProver,
};
use itertools::chain;
use jmp_abs::{
    JmpAbsOpcodeClaim, JmpAbsOpcodeComponent, JmpAbsOpcodeInteractionProver, JmpAbsOpcodeProver,
};
use ret::{RetOpcodeClaim, RetOpcodeComponent, RetOpcodeInteractionProver, RetOpcodeProver};
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
use super::StandardInteractionClaimStack;
use crate::input::instructions::{Instructions, VmState};
use crate::input::mem::Memory;

pub const CAIRO_OFFSET_SHIFT: u32 = 1 << 15;
pub fn cairo_offset(val: i32) -> M31 {
    M31::from(val + CAIRO_OFFSET_SHIFT as i32)
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

    pub jmp_abs_ap: JmpAbsOpcodeClaim<false, false>,
    pub jmp_abs_fp: JmpAbsOpcodeClaim<true, false>,
    pub jmp_abs_ap_inc: JmpAbsOpcodeClaim<false, true>,
    pub jmp_abs_fp_inc: JmpAbsOpcodeClaim<true, true>,

    pub generic: GenericOpcodeClaim,
    pub extra_transitions: Vec<(u32, [VmState; 2])>,
}
impl OpcodesClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.mix_into(channel);

        self.jmp_abs_ap.mix_into(channel);
        self.jmp_abs_fp.mix_into(channel);
        self.jmp_abs_ap_inc.mix_into(channel);
        self.jmp_abs_fp_inc.mix_into(channel);

        self.generic.mix_into(channel);
    }
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(
            [
                self.ret.log_sizes(),
                self.jmp_abs_ap.log_sizes(),
                self.jmp_abs_fp.log_sizes(),
                self.jmp_abs_ap_inc.log_sizes(),
                self.jmp_abs_fp_inc.log_sizes(),
                self.generic.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

#[derive(Clone)]
pub struct OpcodesInteractionClaim {
    pub ret: StandardInteractionClaimStack,

    pub jmp_abs_ap: StandardInteractionClaimStack,
    pub jmp_abs_fp: StandardInteractionClaimStack,
    pub jmp_abs_ap_inc: StandardInteractionClaimStack,
    pub jmp_abs_fp_inc: StandardInteractionClaimStack,

    pub generic: StandardInteractionClaimStack,
}
impl OpcodesInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.mix_into(channel);

        self.jmp_abs_ap.mix_into(channel);
        self.jmp_abs_fp.mix_into(channel);
        self.jmp_abs_ap_inc.mix_into(channel);
        self.jmp_abs_fp_inc.mix_into(channel);

        self.generic.mix_into(channel);
    }

    pub fn logup_sum(&self) -> SecureField {
        self.ret.logup_sum()
            + self.jmp_abs_ap.logup_sum()
            + self.jmp_abs_fp.logup_sum()
            + self.jmp_abs_ap_inc.logup_sum()
            + self.jmp_abs_fp_inc.logup_sum()
            + self.generic.logup_sum()
    }
}

pub struct OpcodesProvers {
    pub ret: RetOpcodeProver,

    pub jmp_abs_ap: JmpAbsOpcodeProver<false, false>,
    pub jmp_abs_fp: JmpAbsOpcodeProver<true, false>,
    pub jmp_abs_ap_inc: JmpAbsOpcodeProver<false, true>,
    pub jmp_abs_fp_inc: JmpAbsOpcodeProver<true, true>,

    pub generic: GenericOpcodeProver,
}
impl OpcodesProvers {
    pub fn new(instructions: Instructions) -> Self {
        let [jmp_abs_ap, jmp_abs_fp, jmp_abs_ap_inc, jmp_abs_fp_inc] = instructions.jmp_abs;
        Self {
            ret: RetOpcodeProver::new((), instructions.ret.into_iter()),
            jmp_abs_ap: JmpAbsOpcodeProver::new((), jmp_abs_ap.into_iter()),
            jmp_abs_fp: JmpAbsOpcodeProver::new((), jmp_abs_fp.into_iter()),
            jmp_abs_ap_inc: JmpAbsOpcodeProver::new((), jmp_abs_ap_inc.into_iter()),
            jmp_abs_fp_inc: JmpAbsOpcodeProver::new((), jmp_abs_fp_inc.into_iter()),
            generic: GenericOpcodeProver::new((), instructions.generic.into_iter()),
        }
    }
    pub fn generate(
        self,
        mut ctx: OpcodeGenContext<'_>,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (OpcodesClaim, OpcodesInteractionProvers) {
        let mut extra_transitions = Vec::new();

        extra_transitions.push(self.ret.pad_transition(ctx.mem));
        let (ret_claim, ret) = self.ret.write_trace(tree_builder, &mut ctx);

        extra_transitions.push(self.jmp_abs_ap.pad_transition(ctx.mem));
        let (jmp_abs_ap_claim, jmp_abs_ap) = self.jmp_abs_ap.write_trace(tree_builder, &mut ctx);

        extra_transitions.push(self.jmp_abs_fp.pad_transition(ctx.mem));
        let (jmp_abs_fp_claim, jmp_abs_fp) = self.jmp_abs_fp.write_trace(tree_builder, &mut ctx);

        extra_transitions.push(self.jmp_abs_ap_inc.pad_transition(ctx.mem));
        let (jmp_abs_ap_inc_claim, jmp_abs_ap_inc) =
            self.jmp_abs_ap_inc.write_trace(tree_builder, &mut ctx);

        extra_transitions.push(self.jmp_abs_fp_inc.pad_transition(ctx.mem));
        let (jmp_abs_fp_inc_claim, jmp_abs_fp_inc) =
            self.jmp_abs_fp_inc.write_trace(tree_builder, &mut ctx);

        extra_transitions.push(self.generic.pad_transition());
        let (generic_claim, generic) = self.generic.write_trace(tree_builder, &mut ctx);
        (
            OpcodesClaim {
                ret: ret_claim,
                jmp_abs_ap: jmp_abs_ap_claim,
                jmp_abs_fp: jmp_abs_fp_claim,
                jmp_abs_ap_inc: jmp_abs_ap_inc_claim,
                jmp_abs_fp_inc: jmp_abs_fp_inc_claim,
                generic: generic_claim,
                extra_transitions,
            },
            OpcodesInteractionProvers {
                ret,
                jmp_abs_ap,
                jmp_abs_fp,
                jmp_abs_ap_inc,
                jmp_abs_fp_inc,
                generic,
            },
        )
    }
}

pub struct OpcodesInteractionProvers {
    pub ret: RetOpcodeInteractionProver,

    pub jmp_abs_ap: JmpAbsOpcodeInteractionProver<false, false>,
    pub jmp_abs_fp: JmpAbsOpcodeInteractionProver<true, false>,
    pub jmp_abs_ap_inc: JmpAbsOpcodeInteractionProver<false, true>,
    pub jmp_abs_fp_inc: JmpAbsOpcodeInteractionProver<true, true>,
    pub generic: GenericOpcodeInteractionProver,
}
impl OpcodesInteractionProvers {
    pub fn generate(
        self,
        elements: &OpcodeElements,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> OpcodesInteractionClaim {
        let ret = self.ret.write_interaction_trace(tree_builder, elements);

        let jmp_abs_ap = self
            .jmp_abs_ap
            .write_interaction_trace(tree_builder, elements);
        let jmp_abs_fp = self
            .jmp_abs_fp
            .write_interaction_trace(tree_builder, elements);
        let jmp_abs_ap_inc = self
            .jmp_abs_ap_inc
            .write_interaction_trace(tree_builder, elements);
        let jmp_abs_fp_inc = self
            .jmp_abs_fp_inc
            .write_interaction_trace(tree_builder, elements);

        let generic = self.generic.write_interaction_trace(tree_builder, elements);
        OpcodesInteractionClaim {
            ret,
            jmp_abs_ap,
            jmp_abs_fp,
            jmp_abs_ap_inc,
            jmp_abs_fp_inc,
            generic,
        }
    }
}

pub struct OpcodesComponents {
    pub ret: RetOpcodeComponent,

    pub jmp_abs_ap: JmpAbsOpcodeComponent<false, false>,
    pub jmp_abs_fp: JmpAbsOpcodeComponent<true, false>,
    pub jmp_abs_ap_inc: JmpAbsOpcodeComponent<false, true>,
    pub jmp_abs_fp_inc: JmpAbsOpcodeComponent<true, true>,

    pub generic: GenericOpcodeComponent,
}
impl OpcodesComponents {
    pub fn new(
        claim: &OpcodesClaim,
        elements: &OpcodeElements,
        interaction_claim: &OpcodesInteractionClaim,
    ) -> Self {
        Self {
            ret: RetOpcodeComponent::new(&claim.ret, elements.clone(), &interaction_claim.ret),

            jmp_abs_ap: JmpAbsOpcodeComponent::new(
                &claim.jmp_abs_ap,
                elements.clone(),
                &interaction_claim.jmp_abs_ap,
            ),
            jmp_abs_fp: JmpAbsOpcodeComponent::new(
                &claim.jmp_abs_fp,
                elements.clone(),
                &interaction_claim.jmp_abs_fp,
            ),
            jmp_abs_ap_inc: JmpAbsOpcodeComponent::new(
                &claim.jmp_abs_ap_inc,
                elements.clone(),
                &interaction_claim.jmp_abs_ap_inc,
            ),
            jmp_abs_fp_inc: JmpAbsOpcodeComponent::new(
                &claim.jmp_abs_fp_inc,
                elements.clone(),
                &interaction_claim.jmp_abs_fp_inc,
            ),

            generic: GenericOpcodeComponent::new(
                &claim.generic,
                elements.clone(),
                &interaction_claim.generic,
            ),
        }
    }
    pub fn provers(&self) -> impl Iterator<Item = &dyn ComponentProver<SimdBackend>> {
        chain![
            self.ret.provers(),
            self.jmp_abs_ap.provers(),
            self.jmp_abs_fp.provers(),
            self.jmp_abs_ap_inc.provers(),
            self.jmp_abs_fp_inc.provers(),
            self.generic.provers()
        ]
    }
    pub fn components(&self) -> impl Iterator<Item = &dyn Component> {
        chain![
            self.ret.components(),
            self.jmp_abs_ap.components(),
            self.jmp_abs_fp.components(),
            self.jmp_abs_ap_inc.components(),
            self.jmp_abs_fp_inc.components(),
            self.generic.components(),
        ]
    }

    pub fn assert_constraints(
        &self,
        trace_polys: &mut TreeVec<impl Iterator<Item = CirclePoly<SimdBackend>>>,
    ) {
        self.ret.assert_constraints(trace_polys);

        self.jmp_abs_ap.assert_constraints(trace_polys);
        self.jmp_abs_fp.assert_constraints(trace_polys);
        self.jmp_abs_ap_inc.assert_constraints(trace_polys);
        self.jmp_abs_fp_inc.assert_constraints(trace_polys);

        self.generic.assert_constraints(trace_polys);
    }
}

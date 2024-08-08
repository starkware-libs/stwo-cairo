//! Mock ret opcode prover.
#![allow(unused)]
use std::simd::u32x16;

use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::vcs::ops::{MerkleHasher, MerkleOps};

use super::opcode::OpcodeElements;
use crate::air::OpcodeGenContext;

// Top level prover.
#[derive(Copy, Clone)]
pub struct RetInput {
    pub pc: u32,
    pub ap: u32,
    pub fp: u32,
    pub new_pc: u32,
    pub new_fp: u32,
}

pub struct RetOpcodeProver {
    inputs: Vec<RetInput>,
}
impl RetOpcodeProver {
    pub fn new(inputs: Vec<RetInput>) -> Vec<Self> {
        todo!()
    }
    pub fn write_trace<H: MerkleHasher>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, H>,
        ctx: &mut OpcodeGenContext,
    ) -> RetOpcodeClaimProver
    where
        SimdBackend: MerkleOps<H>,
    {
        todo!()
    }
}

#[derive(Clone)]
pub struct RetOpcodeClaim {
    log_size: u32,
    n_rets: usize,
}
impl RetOpcodeClaim {
    pub fn log_sizes() -> Vec<u32> {
        todo!()
    }
}

pub struct RetOpcodeClaimProver {
    claim: RetOpcodeClaim,

    // Lookup data.
    pc: Vec<u32x16>,
    fp: Vec<u32x16>,
    instr: Vec<u32x16>,
    new_pc: Vec<u32x16>,
    new_fp: Vec<u32x16>,
}
impl RetOpcodeClaimProver {
    pub fn claim(&self) -> &RetOpcodeClaim {
        &self.claim
    }
    pub fn write_interaction_trace<H: MerkleHasher>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, H>,
        opcode_elements: &OpcodeElements,
    ) -> RetOpcodeInteractionClaimProver
    where
        SimdBackend: MerkleOps<H>,
    {
        todo!()
    }
}

#[derive(Clone)]
pub struct RetOpcodeInteractionClaim {
    claimed_sum: SecureField,
}
impl RetOpcodeInteractionClaim {
    pub fn log_sizes() -> Vec<u32> {
        todo!()
    }
}

pub struct RetOpcodeInteractionClaimProver {
    claim: RetOpcodeClaim,
    interaction_claim: RetOpcodeInteractionClaim,
    opcode_elements: OpcodeElements,
}
impl RetOpcodeInteractionClaimProver {
    pub fn claim(&self) -> &RetOpcodeInteractionClaim {
        &self.interaction_claim
    }
    pub fn into_component(self) -> RetOpcodeComponent {
        RetOpcodeComponent::new(self.claim, self.interaction_claim, self.opcode_elements)
    }
}

// Component
pub struct RetOpcodeComponent {
    log_size: u32,
    claimed_sum: SecureField,
    opcode_elements: OpcodeElements,
}
impl RetOpcodeComponent {
    pub fn new(
        claim: RetOpcodeClaim,
        interaction_claim: RetOpcodeInteractionClaim,
        opcode_elements: OpcodeElements,
    ) -> Self {
        Self {
            log_size: claim.log_size,
            claimed_sum: interaction_claim.claimed_sum,
            opcode_elements,
        }
    }
}
impl Component for RetOpcodeComponent {
    fn max_constraint_log_degree_bound(&self) -> u32 {
        todo!()
    }

    fn n_constraints(&self) -> usize {
        todo!()
    }

    fn trace_log_degree_bounds(
        &self,
    ) -> stwo_prover::core::pcs::TreeVec<stwo_prover::core::ColumnVec<u32>> {
        todo!()
    }

    fn mask_points(
        &self,
        point: stwo_prover::core::circle::CirclePoint<SecureField>,
    ) -> stwo_prover::core::pcs::TreeVec<
        stwo_prover::core::ColumnVec<Vec<stwo_prover::core::circle::CirclePoint<SecureField>>>,
    > {
        todo!()
    }

    fn evaluate_constraint_quotients_at_point(
        &self,
        point: stwo_prover::core::circle::CirclePoint<SecureField>,
        mask: &stwo_prover::core::pcs::TreeVec<stwo_prover::core::ColumnVec<Vec<SecureField>>>,
        evaluation_accumulator: &mut stwo_prover::core::air::accumulation::PointEvaluationAccumulator,
        interaction_elements: &stwo_prover::core::InteractionElements,
        lookup_values: &stwo_prover::core::LookupValues,
    ) {
        todo!()
    }
}
impl ComponentProver<SimdBackend> for RetOpcodeComponent {
    fn evaluate_constraint_quotients_on_domain(
        &self,
        trace: &stwo_prover::core::air::ComponentTrace<'_, SimdBackend>,
        evaluation_accumulator: &mut stwo_prover::core::air::accumulation::DomainEvaluationAccumulator<SimdBackend>,
        interaction_elements: &stwo_prover::core::InteractionElements,
        lookup_values: &stwo_prover::core::LookupValues,
    ) {
        todo!()
    }

    fn lookup_values(
        &self,
        _trace: &stwo_prover::core::air::ComponentTrace<'_, SimdBackend>,
    ) -> stwo_prover::core::LookupValues {
        todo!()
    }
}

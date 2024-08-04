use std::simd::u32x16;

use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::TreeBuilder;

use crate::air::OpcodeGenContext;
use crate::opcode::OpcodeInteractionElements;

// Top level prover.
#[derive(Copy, Clone)]
pub struct RetInput {
    pub pc: M31,
    pub ap: M31,
    pub fp: M31,
}

pub struct RetOpcodeProver {
    inputs: Vec<RetInput>,
}
impl RetOpcodeProver {
    pub fn new(inputs: Vec<RetInput>) -> Vec<Self> {
        todo!()
    }
    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend>,
        ctx: &mut OpcodeGenContext,
    ) -> RetOpcodeStatement0Prover {
        todo!()
    }
}

// Statement 0.
#[derive(Clone)]
pub struct RetOpcodeStatement0 {
    log_size: u32,
    n_rets: usize,
}
impl RetOpcodeStatement0 {
    pub fn log_sizes() -> Vec<u32> {
        todo!()
    }
}

pub struct RetOpcodeStatement0Prover {
    stmt0: RetOpcodeStatement0,

    // Lookup data.
    pc: Vec<u32x16>,
    fp: Vec<u32x16>,
    instr: Vec<u32x16>,
    new_pc: Vec<u32x16>,
    new_fp: Vec<u32x16>,
}
impl RetOpcodeStatement0Prover {
    pub fn stmt(&self) -> &RetOpcodeStatement0 {
        &self.stmt0
    }
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend>,
        opcode_elements: &OpcodeInteractionElements,
    ) -> RetOpcodeStatement1Prover {
        todo!()
    }
}

// Statement 1.
#[derive(Clone)]
pub struct RetOpcodeStatement1 {
    claimed_sum: SecureField,
}
impl RetOpcodeStatement1 {
    pub fn log_sizes() -> Vec<u32> {
        todo!()
    }
}

pub struct RetOpcodeStatement1Prover {
    stmt0: RetOpcodeStatement0,
    stmt1: RetOpcodeStatement1,
    opcode_elements: OpcodeInteractionElements,
}
impl RetOpcodeStatement1Prover {
    pub fn stmt(&self) -> &RetOpcodeStatement1 {
        &self.stmt1
    }
    pub fn into_component(self) -> RetOpcodeStatementComponent {
        RetOpcodeStatementComponent::new(self.stmt0, self.stmt1, self.opcode_elements)
    }
}

// Component
pub struct RetOpcodeStatementComponent {
    log_size: u32,
    claimed_sum: SecureField,
    opcode_elements: OpcodeInteractionElements,
}
impl RetOpcodeStatementComponent {
    pub fn new(
        stmt0: RetOpcodeStatement0,
        stmt1: RetOpcodeStatement1,
        opcode_elements: OpcodeInteractionElements,
    ) -> Self {
        Self {
            log_size: stmt0.log_size,
            claimed_sum: stmt1.claimed_sum,
            opcode_elements,
        }
    }
}
impl Component for RetOpcodeStatementComponent {
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
    ) {
        todo!()
    }
}
impl ComponentProver<SimdBackend> for RetOpcodeStatementComponent {
    fn evaluate_constraint_quotients_on_domain(
        &self,
        trace: &stwo_prover::core::air::ComponentTrace<'_, SimdBackend>,
        evaluation_accumulator: &mut stwo_prover::core::air::accumulation::DomainEvaluationAccumulator<SimdBackend>,
    ) {
        todo!()
    }
}

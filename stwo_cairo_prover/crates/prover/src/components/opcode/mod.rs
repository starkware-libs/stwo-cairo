use prover::PackedVmState;
use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::channel::Channel;

use super::memory::prover::AddrToIdProverBuilder;
use super::memory::{InstructionElements, MemoryElements};
use super::range_check::{RangeElements, RangeProver};
use super::StandardLookupData;

pub mod component;
pub mod prover;

pub struct FVmState<F> {
    pub pc: F,
    pub fp: F,
    pub ap: F,
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

pub trait Opcode: Clone {
    type LookupData: StandardLookupData;
    // TODO: Consider deducing from evaluate.
    fn n_columns() -> usize;
    fn new_lookup_data(log_size: u32) -> Self::LookupData;
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        state: FVmState<E::F>,
        elements: &OpcodeElements,
    ) -> FVmState<E::F>;
    fn write_trace_row(
        dst: &mut [BaseColumn],
        state_input: &PackedVmState,
        row_index: usize,
        ctx: &mut OpcodeGenContext<'_>,
        lookup_data: &mut Self::LookupData,
    );
}

use prover::PackedVmState;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::EvalAtRow;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::PackedM31;
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::lookups::utils::Fraction;

use super::memory::prover::MemoryProver;
use super::memory::MemoryElements;
use super::range_check_unit::component::RangeProver;
use super::range_check_unit::RangeElements;

pub mod component;
pub mod prover;

const OPCODE_N_TRACE_CELLS: usize = 7;

pub struct FVmState<F> {
    pub pc: F,
    pub fp: F,
    pub ap: F,
}

pub struct OpcodeElements {
    pub memory_elements: MemoryElements,
    pub instruction_elements: LookupElements<4>,
    pub range_elements: RangeElements,
}

// Move
pub struct CpuRangeElements {
    range2: RangeElements,
    range3: RangeElements,
}

pub struct OpcodeGenContext<'a> {
    pub mem: &'a mut MemoryProver,
    pub range: CpuRangeProvers<'a>,
}

// Move
pub struct CpuRangeProvers<'a> {
    range2: &'a mut RangeProver,
    range3: &'a mut RangeProver,
}

pub type LookupFunc<'a, 'b> = Box<dyn Fn(usize, &'b mut [Fraction<PackedM31, PackedQM31>]) + 'a>;
pub trait Opcode {
    type LookupData;
    // TODO: Consider deducing from evaluate.
    fn n_columns() -> usize;
    fn new_lookup_data(log_size: u32) -> Self::LookupData;
    fn lookups<'a, 'b>(
        lookup_data: &'a Self::LookupData,
        elements: &'a OpcodeElements,
    ) -> Vec<LookupFunc<'a, 'b>>;
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
        ctx: OpcodeGenContext<'_>,
        lookup_data: &mut Self::LookupData,
    );
}

// pub trait EvalAtRowWithLogup: EvalAtRow {
//     fn push_lookup<const N: usize>(
//         &mut self,
//         numerator: Self::EF,
//         values: &[Self::F],
//         lookup_elements: &LookupElements<N>,
//     );
// }

// pub struct VectorizedEvalAtRowWithLogup<'a, const N: usize, E: EvalAtRow> {
//     eval: &'a mut E,
//     logup: LogupAtRow<2, E>,
// }
// impl<'a, const N: usize, E: EvalAtRow> EvalAtRow for VectorizedEvalAtRowWithLogup<'a, N, E> {
//     type F = Vectorized<E::F, N>;
//     type EF = Vectorized<E::EF, N>;

//     fn next_interaction_mask<const M: usize>(
//         &mut self,
//         interaction: usize,
//         offsets: [isize; N],
//     ) -> [Self::F; M] {
//         Vectorized::from_fn(|i| self.eval.next_interaction_mask(interaction, offsets))
//     }

//     fn add_constraint<G>(&mut self, constraint: G)
//     where
//         Self::EF: std::ops::Mul<G, Output = Self::EF>,
//     {
//         // TODO.
//         (constraint * Self::EF::one())
//             .0
//             .for_each(|c| self.eval.add_constraint(c));
//     }

//     fn combine_ef(
//         values: [Self::F; stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE],
//     ) -> Self::EF {
//         Vectorized::from_fn(|i| E::combine_ef(values.map(|c| c[i])))
//     }
// }
// impl<'a, const N: usize, E: EvalAtRow> EvalAtRowWithLogup
//     for VectorizedEvalAtRowWithLogup<'a, N, E>
// {
//     fn push_lookup<const M: usize>(
//         &mut self,
//         numerator: Self::EF,
//         values: &[Self::F],
//         lookup_elements: &LookupElements<M>,
//     ) {
//         for i in 0..N {
//             self.logup.push_lookup(
//                 self.eval,
//                 numerator[i],
//                 &std::array::from_fn(|j| values[j][i]),
//                 lookup_elements,
//             );
//         }
//     }
// }

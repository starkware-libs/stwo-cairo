#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
use std::iter::zip;
use std::sync::Mutex;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col, Column};
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{
    bit_reverse_coset_to_circle_domain_order, bit_reverse_index, coset_index_to_circle_domain_index,
};

use super::component::{Claim, InteractionClaim, LOG_SIZE, N_BITS};
use crate::components::utils::{pack_values, AtomicMultiplicityColumn};
use crate::preprocessed::BitwiseXor;
use crate::relations;

pub type InputType = [M31; 3];
pub type PackedInputType = [PackedM31; 3];
const N_TRACE_COLUMNS: usize = 1;
const PACKED_LOG_SIZE: u32 = LOG_SIZE - LOG_N_LANES;

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << LOG_SIZE),
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mults = self.mults.into_simd_vec();
        let (trace, lookup_data) = write_trace_simd(mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at((input[0].0 << N_BITS) + input[1].0);
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let xor_a_column = BitwiseXor::new(N_BITS, 0);
    let xor_b_column = BitwiseXor::new(N_BITS, 1);
    let xor_c_column = BitwiseXor::new(N_BITS, 2);
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(LOG_SIZE),
            LookupData::uninitialized(PACKED_LOG_SIZE),
        )
    };

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, row), lookup_data)| {
            *row[0] = mults[row_index];

            *lookup_data.bitwise_xor_trios = [
                xor_a_column.packed_at(row_index),
                xor_b_column.packed_at(row_index),
                xor_c_column.packed_at(row_index),
            ];
            *lookup_data.mults = mults[row_index];
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    bitwise_xor_trios: Vec<[PackedM31; 3]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        verify_bitwise_xor_9: &relations::VerifyBitwiseXor_9,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        assert!(self.lookup_data.bitwise_xor_trios.len() == 1 << PACKED_LOG_SIZE);
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, (values, mults)) in self
            .lookup_data
            .bitwise_xor_trios
            .iter()
            .zip(self.lookup_data.mults)
            .enumerate()
        {
            let denom = verify_bitwise_xor_9.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * mults, denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

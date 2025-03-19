#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::simd::u32x16;

use itertools::{chain, Itertools};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::blake::deduce_output::BlakeRoundSigma;
use crate::components::prelude::proving::*;
use crate::components::range_check_vector::SIMD_ENUMERATION_0;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];
const N_TRACE_COLUMNS: usize = 0;
pub const BLAKE_SIGMA_LOG_SIZE: u32 = 4;

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(1 << 4),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let multiplicity_data = self.mults.into_simd_vec();
        let multiplicity_column = BaseColumn::from_simd(multiplicity_data.clone());

        let domain = CanonicCoset::new(BLAKE_SIGMA_LOG_SIZE).circle_domain();
        let trace = [multiplicity_column]
            .map(|col| CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, col));

        tree_builder.extend_evals(trace);

        let claim = Claim {};
        let lookup_data = LookupData {
            multiplicities: multiplicity_data,
        };
        let interaction_claim_prover = InteractionClaimGenerator { lookup_data };

        (claim, interaction_claim_prover)
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at(input[0].0);
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    multiplicities: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        blake_round_sigma: &relations::BlakeRoundSigma,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(BLAKE_SIGMA_LOG_SIZE);
        let mult = <_ as Into<PackedQM31>>::into(self.lookup_data.multiplicities[0]);
        let sigmas = BlakeRoundSigma::deduce_output(unsafe {
            PackedM31::from_simd_unchecked(u32x16::from_array([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0,
            ]))
        });
        let seq = unsafe { PackedM31::from_simd_unchecked(SIMD_ENUMERATION_0) };

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        let values = chain![[seq], sigmas].collect_vec();
        let denom = blake_round_sigma.combine(&values);
        col_gen.write_frac(0, -mult, denom);
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

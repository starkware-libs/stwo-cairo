use std::simd::Simd;

use itertools::{chain, Itertools};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{RangeCheckClaim, RangeCheckInteractionClaim};
use super::{
    exclusive_suffix_sum, generate_partitioned_enumeration, partition_into_bit_segments,
    simd_masks, RangeCheckLookupElements,
};

// TODO(Ohad): rustdoc.
pub struct RangeCheckClaimProver<const N: usize> {
    log_ranges: [u32; N],
    log_ranges_suffix_sum: [u32; N],
    pub multiplicities: Vec<Simd<u32, N_LANES>>,
}
impl<const N: usize> RangeCheckClaimProver<N> {
    pub fn new(log_ranges: [u32; N]) -> Self {
        let suffix_sum = exclusive_suffix_sum(log_ranges);
        let multiplicities = vec![
            Simd::<u32, N_LANES>::splat(0);
            1 << (log_ranges.iter().sum::<u32>() - LOG_N_LANES)
        ];
        Self {
            log_ranges,
            multiplicities,
            log_ranges_suffix_sum: suffix_sum,
        }
    }

    fn log_size(&self) -> u32 {
        self.log_ranges.iter().sum()
    }

    // TODO(Ohad): test.
    pub fn add_m31(&mut self, input: [M31; N]) {
        let num = self
            .log_ranges_suffix_sum
            .iter()
            .zip(input)
            .fold(0, |dense, (&trailing, val)| dense + (val.0 << trailing));
        let (input_row, input_lane) = (num / N_LANES as u32, num % N_LANES as u32);
        self.multiplicities[input_row as usize][input_lane as usize] += 1;
    }

    // TODO(Ohad): test.
    pub fn add_packed_m31(&mut self, input: &[PackedM31; N]) {
        let arrays: [_; N] = std::array::from_fn(|i| input[i].to_array());
        for i in 0..N_LANES {
            self.add_m31(std::array::from_fn(|j| arrays[j][i]));
        }
    }

    fn write_fixed_columns(&self) -> [BaseColumn; N] {
        let mut fixed_columns = generate_partitioned_enumeration(self.log_ranges).into_iter();
        std::array::from_fn(|_| BaseColumn {
            data: fixed_columns.next().unwrap(),
            length: 1 << self.log_size(),
        })
    }

    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (RangeCheckClaim<N>, RangeCheckInteractionClaimProver<N>) {
        let log_size = self.log_size();

        let fixed_columns = self.write_fixed_columns();
        let multiplicity_data = self
            .multiplicities
            .into_iter()
            .map(|m| unsafe { PackedM31::from_simd_unchecked(m) })
            .collect_vec();
        let multiplicity_column = BaseColumn {
            data: multiplicity_data.clone(),
            length: 1 << log_size,
        };

        let domain = CanonicCoset::new(log_size).circle_domain();
        let trace = chain!(fixed_columns, [multiplicity_column])
            .map(|col| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, col));

        tree_builder.extend_evals(trace);

        let claim = RangeCheckClaim {
            log_ranges: self.log_ranges,
        };

        let interaction_claim_prover = RangeCheckInteractionClaimProver {
            log_ranges: self.log_ranges,
            multiplicities: multiplicity_data,
        };

        (claim, interaction_claim_prover)
    }
}

#[derive(Debug)]
pub struct RangeCheckInteractionClaimProver<const N: usize> {
    pub log_ranges: [u32; N],
    pub multiplicities: Vec<PackedM31>,
}
impl<const N: usize> RangeCheckInteractionClaimProver<N> {
    pub fn with_capacity(capacity: usize, log_ranges: [u32; N]) -> Self {
        Self {
            multiplicities: Vec::with_capacity(capacity),
            log_ranges,
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &RangeCheckLookupElements,
    ) -> RangeCheckInteractionClaim {
        let log_size = self.log_ranges.iter().sum::<u32>();
        let mut logup_gen = LogupTraceGenerator::new(log_size);
        let mut col_gen = logup_gen.new_col();

        // Lookup values columns.
        let masks = simd_masks(self.log_ranges);
        let suffix_sum = exclusive_suffix_sum(self.log_ranges);
        for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
            let numerator = (-self.multiplicities[vec_row]).into();
            let denom = lookup_elements.combine(&partition_into_bit_segments(
                suffix_sum,
                masks,
                vec_row as u32,
            ));
            col_gen.write_frac(vec_row, numerator, denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        RangeCheckInteractionClaim { claimed_sum }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::{
        FrameworkComponent, FrameworkEval as _, TraceLocationAllocator,
    };
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::channel::Blake2sChannel;
    use stwo_prover::core::fields::m31::M31;
    use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
    use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};

    use super::RangeCheckClaimProver;
    use crate::components::range_check_vector::component::RangeCheckVectorEval;
    use crate::components::range_check_vector::tests::split_bits;
    use crate::components::range_check_vector::RangeCheckLookupElements;

    #[test]
    fn test_prove() {
        let mut rng = SmallRng::seed_from_u64(0);
        const LOG_HEIGHT: u32 = 9;
        const LOG_BLOWUP_FACTOR: u32 = 1;
        let log_ranges = [4, 3, 2];
        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(LOG_HEIGHT + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );

        let channel = &mut Blake2sChannel::default();
        let config = PcsConfig::default();
        let commitment_scheme = &mut CommitmentSchemeProver::new(config, &twiddles);

        let inputs: [[M31; 3]; 30] =
            std::array::from_fn(|_| split_bits(log_ranges, rng.gen::<u32>() % (1 << LOG_HEIGHT)));

        let mut claim_prover = RangeCheckClaimProver::new(log_ranges);
        inputs.into_iter().for_each(|input| {
            claim_prover.add_m31(input);
        });

        let mut tree_builder = commitment_scheme.tree_builder();
        let (_, interaction_claim_prover) = claim_prover.write_trace(&mut tree_builder);

        tree_builder.commit(channel);
        let mut tree_builder = commitment_scheme.tree_builder();

        let lookup_elements = RangeCheckLookupElements::draw(channel);
        let interaction_claim =
            interaction_claim_prover.write_interaction_trace(&mut tree_builder, &lookup_elements);
        tree_builder.commit(channel);

        let tree_span_provider = &mut TraceLocationAllocator::default();
        let component = FrameworkComponent::new(
            tree_span_provider,
            RangeCheckVectorEval {
                log_ranges,
                lookup_elements,
                claimed_sum: interaction_claim.claimed_sum,
            },
        );

        let trace_polys = commitment_scheme
            .trees
            .as_ref()
            .map(|t| t.polynomials.iter().cloned().collect_vec());

        stwo_prover::constraint_framework::assert_constraints(
            &trace_polys,
            CanonicCoset::new(LOG_HEIGHT),
            |eval| {
                component.evaluate(eval);
            },
        )
    }
}

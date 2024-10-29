use std::iter::zip;
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
    generate_partitioned_enumeration, partition_into_bit_segments, RangeCheckLookupElements,
    SIMD_ENUMERATION_0,
};

// TODO(Ohad): rustdoc.
pub struct RangeCheckClaimGenerator<const N: usize> {
    log_ranges: [u32; N],
    pub multiplicities: Vec<Simd<u32, N_LANES>>,
}
impl<const N: usize> RangeCheckClaimGenerator<N> {
    pub fn new(log_ranges: [u32; N]) -> Self {
        let multiplicities = vec![
            Simd::<u32, N_LANES>::splat(0);
            1 << (log_ranges.iter().sum::<u32>() - LOG_N_LANES)
        ];
        Self {
            log_ranges,
            multiplicities,
        }
    }

    fn log_size(&self) -> u32 {
        self.log_ranges.iter().sum()
    }

    // TODO(Ohad): test.
    pub fn add_m31(&mut self, input: [M31; N]) {
        let mut value = 0_u32;
        for (segment, segment_n_bits) in zip(input, &self.log_ranges) {
            value <<= segment_n_bits;
            value += segment.0;
        }

        let (input_row, input_lane) = (value / N_LANES as u32, value % N_LANES as u32);
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
    ) -> (RangeCheckClaim, RangeCheckInteractionClaimGenerator<N>) {
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
            log_ranges: self.log_ranges.to_vec(),
        };

        let interaction_claim_prover = RangeCheckInteractionClaimGenerator {
            log_ranges: self.log_ranges,
            multiplicities: multiplicity_data,
        };

        (claim, interaction_claim_prover)
    }
}

#[derive(Debug)]
pub struct RangeCheckInteractionClaimGenerator<const N: usize> {
    pub log_ranges: [u32; N],
    pub multiplicities: Vec<PackedM31>,
}
impl<const N: usize> RangeCheckInteractionClaimGenerator<N> {
    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &RangeCheckLookupElements,
    ) -> RangeCheckInteractionClaim {
        let log_size = self.log_ranges.iter().sum::<u32>();
        let mut logup_gen = LogupTraceGenerator::new(log_size);
        let mut col_gen = logup_gen.new_col();

        // Lookup values columns.
        for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
            let numerator = (-self.multiplicities[vec_row]).into();
            let partitions = partition_into_bit_segments(
                SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32),
                self.log_ranges,
            );
            let partitions: [_; N] =
                std::array::from_fn(|i| unsafe { PackedM31::from_simd_unchecked(partitions[i]) });
            let denom = lookup_elements.combine(&partitions);
            col_gen.write_frac(vec_row, numerator, denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        RangeCheckInteractionClaim { claimed_sum }
    }
}

#[cfg(test)]
mod tests {
    use std::simd::Simd;

    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::constant_columns::gen_is_first;
    use stwo_prover::constraint_framework::{
        FrameworkComponent, FrameworkEval as _, TraceLocationAllocator,
    };
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::channel::Blake2sChannel;
    use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
    use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};

    use super::RangeCheckClaimGenerator;
    use crate::components::range_check_vector::component::RangeCheckVectorEval;
    use crate::components::range_check_vector::{
        partition_into_bit_segments, RangeCheckLookupElements,
    };

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

        let inputs: [[PackedM31; 3]; 30] = std::array::from_fn(|_| {
            let values = Simd::from_array(std::array::from_fn(|_| {
                rng.gen::<u32>() & ((1 << LOG_HEIGHT) - 1)
            }));
            let partitions = partition_into_bit_segments(values, log_ranges);
            std::array::from_fn(|i| unsafe { PackedM31::from_simd_unchecked(partitions[i]) })
        });

        let mut claim_generator = RangeCheckClaimGenerator::new(log_ranges);
        inputs.into_iter().for_each(|input| {
            claim_generator.add_packed_m31(&input);
        });

        let mut tree_builder = commitment_scheme.tree_builder();
        let (_, interaction_claim_generator) = claim_generator.write_trace(&mut tree_builder);

        tree_builder.commit(channel);
        let mut tree_builder = commitment_scheme.tree_builder();

        let lookup_elements = RangeCheckLookupElements::draw(channel);
        let interaction_claim = interaction_claim_generator
            .write_interaction_trace(&mut tree_builder, &lookup_elements);
        tree_builder.commit(channel);

        // Fixed trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        let range_check_constant_trace = gen_is_first::<SimdBackend>(LOG_HEIGHT);
        tree_builder.extend_evals([range_check_constant_trace]);
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

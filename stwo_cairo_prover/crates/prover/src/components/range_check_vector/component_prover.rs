#[macro_export]
macro_rules! range_check_prover {
    ($($log_range:expr),+) => {
        paste::paste! {
            use std::iter::zip;
            use std::simd::Simd;

            use itertools::chain;
            use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
            use stwo_prover::constraint_framework::Relation;
            use stwo_prover::core::backend::simd::column::BaseColumn;
            use stwo_prover::core::fields::m31::BaseField;
            use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
            use stwo_prover::core::backend::simd::SimdBackend;
            use stwo_prover::core::backend::BackendForChannel;
            use stwo_prover::core::channel::MerkleChannel;
            use stwo_prover::core::fields::m31::M31;
            use stwo_prover::core::pcs::TreeBuilder;
            use stwo_prover::core::poly::BitReversedOrder;
            use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};

            use $crate::components::AtomicMultiplicityColumn;
            use $crate::components::range_check_vector::{generate_partitioned_enumeration,
                                            partition_into_bit_segments, SIMD_ENUMERATION_0};

            pub type PackedInputType = [PackedM31; N_RANGES];
            pub type InputType = [M31; N_RANGES];

            pub struct ClaimGenerator {
                multiplicities: AtomicMultiplicityColumn,
            }
            impl ClaimGenerator {
                #[allow(clippy::new_without_default)]
                pub fn new() -> Self {
                    let length = 1 << (RANGES.iter().sum::<u32>()) as usize;
                    let multiplicities = AtomicMultiplicityColumn::new(length);

                    Self {
                        multiplicities,
                    }
                }

                fn log_size(&self) -> u32 {
                    RANGES.iter().sum()
                }

                pub fn add_inputs(&self, inputs: &[[M31; N_RANGES]]) {
                    for input in inputs {
                        self.add_m31(*input);
                    }
                }

                // TODO(Ohad): test.
                pub fn add_m31(&self, input: InputType) {
                    let mut value = 0_u32;
                    for (segment, segment_n_bits) in zip(input, RANGES) {
                        value <<= segment_n_bits;
                        value += segment.0;
                    }
                    self.multiplicities.increase_at(value);
                }

                // TODO(Ohad): test.
                pub fn add_packed_m31(&self, input: &PackedInputType) {
                    let arrays: [_; N_RANGES] = std::array::from_fn(|i| input[i].to_array());
                    for i in 0..N_LANES {
                        self.add_m31(std::array::from_fn(|j| arrays[j][i]));
                    }
                }

                fn write_fixed_columns(&self) -> [BaseColumn; N_RANGES] {
                    let mut fixed_columns = generate_partitioned_enumeration(RANGES).into_iter();
                    std::array::from_fn(|_| BaseColumn::from_simd(fixed_columns.next().unwrap()))
                }

                pub fn write_trace<MC: MerkleChannel>(
                    self,
                    tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
                ) -> (Claim, InteractionClaimGenerator)
                where
                SimdBackend: BackendForChannel<MC>, {
                    let log_size = self.log_size();

                    let fixed_columns = self.write_fixed_columns();
                    let multiplicity_data = self.multiplicities.into_simd_vec();
                    let multiplicity_column = BaseColumn::from_simd(multiplicity_data.clone());

                    let domain = CanonicCoset::new(log_size).circle_domain();
                    let trace = chain!(fixed_columns, [multiplicity_column])
                        .map(|col|
                            CircleEvaluation::<SimdBackend, BaseField, BitReversedOrder>::new(domain, col)
                        );

                    tree_builder.extend_evals(trace);

                    let claim = Claim {
                        log_ranges: RANGES.to_vec(),
                    };

                    let interaction_claim_prover = InteractionClaimGenerator {
                        multiplicities: multiplicity_data,
                    };

                    (claim, interaction_claim_prover)
                }
            }

            #[derive(Debug)]
            pub struct InteractionClaimGenerator {
                pub multiplicities: Vec<PackedM31>,
            }
            impl InteractionClaimGenerator {
                pub fn write_interaction_trace<MC: MerkleChannel>(
                    &self,
                    tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
                    lookup_elements: &relations::[<RangeCheck_$($log_range)_*>],
                ) -> InteractionClaim
                where
                SimdBackend: BackendForChannel<MC>, {
                    let log_size = RANGES.iter().sum::<u32>();
                    let mut logup_gen = LogupTraceGenerator::new(log_size);
                    let mut col_gen = logup_gen.new_col();

                    // Lookup values columns.
                    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
                        let numerator = (-self.multiplicities[vec_row]).into();
                        let partitions = partition_into_bit_segments(
                            SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32),
                            RANGES,
                        );
                        let partitions: [_; N_RANGES] =
                            std::array::from_fn(|i| unsafe {
                                PackedM31::from_simd_unchecked(partitions[i])
                            });
                        let denom = lookup_elements.combine(&partitions);
                        col_gen.write_frac(vec_row, numerator, denom);
                    }
                    col_gen.finalize_col();

                    let (trace, claimed_sum) = logup_gen.finalize_last();
                    tree_builder.extend_evals(trace);

                    InteractionClaim { claimed_sum }
                }
            }



        }
    };
}

#[cfg(test)]
mod tests {
    use std::simd::Simd;

    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::preprocessed_columns::gen_is_first;
    use stwo_prover::constraint_framework::{
        FrameworkComponent, FrameworkEval as _, TraceLocationAllocator,
    };
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::channel::Blake2sChannel;
    use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
    use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    use crate::components::range_check_vector::{partition_into_bit_segments, range_check_7_2_5};
    use crate::relations;
    #[test]
    fn test_prove() {
        let mut rng = SmallRng::seed_from_u64(0);
        const LOG_HEIGHT: u32 = 14;
        const LOG_BLOWUP_FACTOR: u32 = 1;
        let log_ranges = [7, 2, 5];
        let claim_generator = range_check_7_2_5::ClaimGenerator::new();

        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(LOG_HEIGHT + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );

        let channel = &mut Blake2sChannel::default();
        let config = PcsConfig::default();
        let commitment_scheme =
            &mut CommitmentSchemeProver::<SimdBackend, Blake2sMerkleChannel>::new(
                config, &twiddles,
            );

        // Preprocessed trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        let range_check_preprocessed_trace = gen_is_first::<SimdBackend>(LOG_HEIGHT);
        tree_builder.extend_evals([range_check_preprocessed_trace]);
        tree_builder.commit(channel);

        let inputs: [[PackedM31; 3]; 30] = std::array::from_fn(|_| {
            let values = Simd::from_array(std::array::from_fn(|_| {
                rng.gen::<u32>() & ((1 << LOG_HEIGHT) - 1)
            }));
            let partitions = partition_into_bit_segments(values, log_ranges);
            std::array::from_fn(|i| unsafe { PackedM31::from_simd_unchecked(partitions[i]) })
        });

        inputs.into_iter().for_each(|input| {
            claim_generator.add_packed_m31(&input);
        });

        let mut tree_builder = commitment_scheme.tree_builder();
        let (_, interaction_claim_generator) = claim_generator.write_trace(&mut tree_builder);

        tree_builder.commit(channel);
        let mut tree_builder = commitment_scheme.tree_builder();

        let lookup_elements = relations::RangeCheck_7_2_5::draw(channel);
        let interaction_claim = interaction_claim_generator
            .write_interaction_trace(&mut tree_builder, &lookup_elements);
        tree_builder.commit(channel);

        let tree_span_provider = &mut TraceLocationAllocator::default();
        let component = FrameworkComponent::new(
            tree_span_provider,
            range_check_7_2_5::Eval { lookup_elements },
            (interaction_claim.claimed_sum, None),
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
            (interaction_claim.claimed_sum, None),
        )
    }
}

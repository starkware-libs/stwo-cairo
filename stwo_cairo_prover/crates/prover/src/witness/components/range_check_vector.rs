#[macro_export]
macro_rules! range_check_prover {
    ($($log_range:expr),+) => {
        paste::paste! {
            use cairo_air::components::range_check_vector::[<range_check_$($log_range)_*>]::{Claim, InteractionClaim};
            use $crate::witness::prelude::*;
            const N_RANGES: usize = cairo_air::count_elements!($($log_range),*);
            const RANGES : [u32; N_RANGES] = [$($log_range),+];
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
                        self.add_input(input);
                    }
                }

                pub fn add_packed_inputs(&self, inputs: &[[PackedM31; N_RANGES]]) {
                    inputs.into_par_iter().for_each(|input| {
                        self.add_packed_m31(input);
                    });
                }

                // TODO(Ohad): test.
                pub fn add_input(&self, input: &InputType) {
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
                        self.add_input(&std::array::from_fn(|j| arrays[j][i]));
                    }
                }

                pub fn write_trace(
                    self,
                    tree_builder: &mut impl TreeBuilder<SimdBackend>,
                ) -> (Claim, InteractionClaimGenerator) {
                    let log_size = self.log_size();

                    let multiplicity_data = self.multiplicities.into_simd_vec();
                    let multiplicity_column = BaseColumn::from_simd(multiplicity_data.clone());

                    let domain = CanonicCoset::new(log_size).circle_domain();
                    let trace = [multiplicity_column]
                        .map(|col|
                            CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, col)
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
                pub fn write_interaction_trace(
                    &self,
                    tree_builder: &mut impl TreeBuilder<SimdBackend>,
                    lookup_elements: &relations::[<RangeCheck_$($log_range)_*>],
                ) -> InteractionClaim {
                    let log_size = RANGES.iter().sum::<u32>();
                    let mut logup_gen = LogupTraceGenerator::new(log_size);
                    let mut col_gen = logup_gen.new_col();

                    // Lookup values columns.
                    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
                        let numerator = (-self.multiplicities[vec_row]).into();
                        let partitions = cairo_air::preprocessed::partition_into_bit_segments(
                            cairo_air::preprocessed::SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32),
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

#[macro_export]
macro_rules! generate_range_check_witness {
    ([$($log_range:expr),+]) => {
        paste::paste!{
            pub mod [<range_check_$($log_range)_*>] {
                $crate::range_check_prover!($($log_range),+);
            }
        }
    };
}

pub mod range_check_trace_generators {
    generate_range_check_witness!([6]);
    generate_range_check_witness!([8]);
    generate_range_check_witness!([11]);
    generate_range_check_witness!([12]);
    generate_range_check_witness!([18]);
    generate_range_check_witness!([19]);
    generate_range_check_witness!([3, 6]);
    generate_range_check_witness!([4, 3]);
    generate_range_check_witness!([4, 4]);
    generate_range_check_witness!([5, 4]);
    generate_range_check_witness!([9, 9]);
    generate_range_check_witness!([7, 2, 5]);
    generate_range_check_witness!([3, 6, 6, 3]);
    generate_range_check_witness!([4, 4, 4, 4]);
    generate_range_check_witness!([3, 3, 3, 3, 3]);
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::simd::Simd;

    use cairo_air::components::range_check_vector::range_check_7_2_5::Eval;
    use cairo_air::preprocessed::{
        generate_partitioned_enumeration, partition_into_bit_segments, PreProcessedColumn,
        RangeCheck,
    };
    use cairo_air::relations;
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::{
        FrameworkComponent, FrameworkEval as _, TraceLocationAllocator,
    };
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::channel::Blake2sChannel;
    use stwo_prover::core::fields::m31::M31;
    use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
    use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    use crate::witness::components::range_check_7_2_5;
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
        let preproceseed_column_0 = RangeCheck::new(log_ranges, 0).gen_column_simd();
        let preproceseed_column_1 = RangeCheck::new(log_ranges, 1).gen_column_simd();
        let preproceseed_column_2 = RangeCheck::new(log_ranges, 2).gen_column_simd();
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(vec![
            preproceseed_column_0,
            preproceseed_column_1,
            preproceseed_column_2,
        ]);
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
            Eval { lookup_elements },
            interaction_claim.claimed_sum,
        );

        let trace_polys = commitment_scheme
            .trees
            .as_ref()
            .map(|t| t.polynomials.iter().cloned().collect_vec());

        let component_eval = component.deref();
        stwo_prover::constraint_framework::assert_constraints_on_polys(
            &trace_polys,
            CanonicCoset::new(LOG_HEIGHT),
            |eval| {
                component_eval.evaluate(eval);
            },
            interaction_claim.claimed_sum,
        )
    }

    #[test]
    fn test_packed_partition_enumerate() {
        let log_ranges = [5, 3, 3];
        let mut expected = [vec![], vec![], vec![]];
        for i in 0..1 << 5 {
            for j in 0..1 << 3 {
                for k in 0..1 << 3 {
                    expected[0].push(M31(i));
                    expected[1].push(M31(j));
                    expected[2].push(M31(k));
                }
            }
        }

        let mut result = generate_partitioned_enumeration(log_ranges).into_iter();
        let result: [Vec<M31>; 3] =
            std::array::from_fn(|_| BaseColumn::from_simd(result.next().unwrap()).into_cpu_vec());

        assert_eq!(result, expected)
    }
}

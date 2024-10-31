use std::iter::zip;
use std::simd::Simd;

use component::RangeCheckVectorEval;
use stwo_prover::constraint_framework::logup::LookupElements;
pub mod component;
pub mod component_prover;

// TODO(Ohad): figure out n_alpha_powers.
pub type RangeCheckLookupElements = LookupElements<3>;

use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::fields::m31::MODULUS_BITS;

pub const SIMD_ENUMERATION_0: Simd<u32, N_LANES> =
    Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

/// Partitions a number into 'N' bit segments.
///
/// For example: partition_into_bit_segments(0b110101010, [3, 4, 2]) -> [0b110, 0b1010, 0b10]
///
///
/// # Arguments
pub fn partition_into_bit_segments<const N: usize>(
    mut value: Simd<u32, N_LANES>,
    n_bits_per_segment: [u32; N],
) -> [Simd<u32, N_LANES>; N] {
    let mut segments = [Simd::splat(0); N];
    for (segment, segment_n_bits) in zip(&mut segments, n_bits_per_segment).rev() {
        let mask = Simd::splat((1 << segment_n_bits) - 1);
        *segment = value & mask;
        value >>= segment_n_bits;
    }
    segments
}

/// Generates the map from 0..2^(sum_bits) to the corresponding value's partition segments.
pub fn generate_partitioned_enumeration<const N: usize>(
    n_bits_per_segmants: [u32; N],
) -> [Vec<PackedM31>; N] {
    let sum_bits = n_bits_per_segmants.iter().sum::<u32>();
    assert!(sum_bits < MODULUS_BITS);

    let mut res = std::array::from_fn(|_| vec![]);
    for vec_row in 0..1 << (sum_bits - LOG_N_LANES) {
        let value = SIMD_ENUMERATION_0 + Simd::splat(vec_row * N_LANES as u32);
        let segments = partition_into_bit_segments(value, n_bits_per_segmants);
        for i in 0..N {
            res[i].push(unsafe { PackedM31::from_simd_unchecked(segments[i]) });
        }
    }
    res
}

#[macro_export]
macro_rules! generate_range_check_component {
    ($($log_range:expr),+) => {
        paste::paste! {
            pub mod [<range_check_$($log_range)_*>]{
                use std::ops::{Deref, DerefMut};
                use serde::{Deserialize, Serialize};
                use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
                use stwo_prover::constraint_framework::FrameworkEval;
                use stwo_prover::constraint_framework::logup::LookupElements;
                use stwo_prover::core::backend::simd::m31::PackedM31;
                use stwo_prover::core::backend::simd::SimdBackend;
                use stwo_prover::core::fields::qm31::QM31;
                use stwo_prover::core::pcs::TreeBuilder;
                use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

                use $crate::components::range_check_vector::component::{
                    RangeCheckClaim, RangeCheckInteractionClaim,
                };
                use $crate::components::range_check_vector::component_prover::{
                    RangeCheckClaimGenerator, RangeCheckInteractionClaimGenerator,
                };
                use $crate::components::range_check_vector::RangeCheckVectorEval;



                const N_RANGES:usize = count_elements!($($log_range),*);
                pub type Component = FrameworkComponent<[<Eval>]>;

                #[derive(Clone, Serialize, Deserialize)]
                pub struct Claim(RangeCheckClaim);
                impl Deref for Claim {
                    type Target = RangeCheckClaim;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                #[derive(Clone, Serialize, Deserialize)]
                pub struct InteractionClaim(RangeCheckInteractionClaim);
                impl Deref for InteractionClaim {
                    type Target = RangeCheckInteractionClaim;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                pub type RelationElements = LookupElements<N_RANGES>;

                pub struct Eval {
                    eval: RangeCheckVectorEval<N_RANGES>,
                }

                impl Eval {
                    pub fn new(lookup_elements: RelationElements,
                                claimed_sum: QM31) -> Self {
                        Self {
                            eval: RangeCheckVectorEval {
                                log_ranges: [$($log_range),*],
                                lookup_elements,
                                claimed_sum,
                            },
                        }
                    }
                }

                impl FrameworkEval for Eval {
                    fn log_size(&self) -> u32 {
                        self.eval.log_size()
                    }

                    fn max_constraint_log_degree_bound(&self) -> u32 {
                        self.eval.max_constraint_log_degree_bound()
                    }

                    fn evaluate<E: EvalAtRow>(&self, eval: E) -> E {
                        self.eval.evaluate(eval)
                    }
                }

                pub type InputType = [PackedM31; N_RANGES];

                pub struct ClaimGenerator(RangeCheckClaimGenerator::<N_RANGES>);
                impl ClaimGenerator {
                    #[allow(clippy::new_without_default)]
                    pub fn new() -> Self {
                        Self(RangeCheckClaimGenerator::<N_RANGES>::new([$($log_range),*]))
                    }

                    pub fn write_trace(
                        self,
                        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
                    ) -> (Claim,InteractionClaimGenerator){
                        let (claim, interaction_claim) = self.0.write_trace(tree_builder);
                        (Claim(claim), InteractionClaimGenerator(interaction_claim))
                    }
                }

                impl Deref for ClaimGenerator {
                    type Target = RangeCheckClaimGenerator::<N_RANGES>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl DerefMut for ClaimGenerator {
                    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
                        &mut self.0
                    }
                }

                pub struct InteractionClaimGenerator(RangeCheckInteractionClaimGenerator<N_RANGES>);
                impl InteractionClaimGenerator {
                    pub fn write_interaction_trace(
                        &self,
                        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
                        lookup_elements: &RelationElements,
                    ) -> InteractionClaim {
                        InteractionClaim(self.0.write_interaction_trace(tree_builder, lookup_elements))
                    }
                }

                impl Deref for InteractionClaimGenerator {
                    type Target = RangeCheckInteractionClaimGenerator::<N_RANGES>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

            }
        }
    };
}

macro_rules! count_elements {
    ($x:expr) => (1);
    ($x:expr, $($xs:expr),*) => (1 + count_elements!($($xs),*));
}

generate_range_check_component!(9, 9);

#[cfg(test)]
mod tests {
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::fields::m31::M31;

    use crate::components::range_check_vector::generate_partitioned_enumeration;

    #[test]
    fn test_packed_partition_enumerate() {
        let log_ranges = [5, 3, 3];
        let log_size = log_ranges.iter().sum::<u32>();
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
        let result: [Vec<M31>; 3] = std::array::from_fn(|_| {
            BaseColumn {
                data: result.next().unwrap(),
                length: 1 << log_size,
            }
            .into_cpu_vec()
        });

        assert_eq!(result, expected)
    }
}

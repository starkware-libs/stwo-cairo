use itertools::chain;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};

use super::air::CairoInteractionElements;
use super::debug_tools::indented_component_display;
use crate::adapter::builtins::{BuiltinSegments, BITWISE_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS};
use crate::components::{
    bitwise_builtin, cube_252, memory_address_to_id, memory_id_to_big, range_check_19,
    range_check_6, range_check_9_9, range_check_builtin_bits_128, range_check_builtin_bits_96,
    verify_bitwise_xor_9,
};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsClaim {
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
    pub cube_252: Option<cube_252::Claim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Claim>,
}
impl BuiltinsClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            bitwise_builtin.mix_into(channel);
        }
        if let Some(cube_252) = &self.cube_252 {
            cube_252.mix_into(channel);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel);
        }
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.bitwise_builtin
                .map(|bitwise_builtin| bitwise_builtin.log_sizes())
                .into_iter(),
            self.cube_252
                .map(|cube_252| cube_252.log_sizes())
                .into_iter(),
            self.range_check_96_builtin
                .map(|range_check_96_builtin| range_check_96_builtin.log_sizes())
                .into_iter(),
            self.range_check_128_builtin
                .map(|range_check_128_builtin| range_check_128_builtin.log_sizes())
                .into_iter(),
        ))
    }
}

pub struct BuiltinsClaimGenerator {
    bitwise_builtin_trace_generator: Option<bitwise_builtin::ClaimGenerator>,
    cube_252_trace_generator: Option<cube_252::ClaimGenerator>,
    range_check_96_builtin_trace_generator: Option<range_check_builtin_bits_96::ClaimGenerator>,
    range_check_128_builtin_trace_generator: Option<range_check_builtin_bits_128::ClaimGenerator>,
}
impl BuiltinsClaimGenerator {
    pub fn new(builtin_segments: BuiltinSegments) -> Self {
        let bitwise_builtin_trace_generator = builtin_segments.bitwise.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                (segment_length % BITWISE_MEMORY_CELLS) == 0,
                "bitwise segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / BITWISE_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "bitwise instances number is not a power of two"
            );
            bitwise_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let cube_252_trace_generator = builtin_segments
            .poseidon
            .map(|_| cube_252::ClaimGenerator::new());
        let range_check_96_builtin_trace_generator =
            builtin_segments.range_check_bits_96.map(|segment| {
                let segment_length = segment.stop_ptr - segment.begin_addr;
                let n_instances = segment_length / RANGE_CHECK_MEMORY_CELLS;
                assert!(
                    n_instances.is_power_of_two(),
                    "range_check_bits_96 instances number is not a power of two"
                );
                range_check_builtin_bits_96::ClaimGenerator::new(
                    n_instances.ilog2(),
                    segment.begin_addr as u32,
                )
            });
        let range_check_128_builtin_trace_generator =
            builtin_segments.range_check_bits_128.map(|segment| {
                let segment_length = segment.stop_ptr - segment.begin_addr;
                let n_instances = segment_length / RANGE_CHECK_MEMORY_CELLS;
                assert!(
                    n_instances.is_power_of_two(),
                    "range_check_bits_128 instances number is not a power of two"
                );
                range_check_builtin_bits_128::ClaimGenerator::new(
                    n_instances.ilog2(),
                    segment.begin_addr as u32,
                )
            });
        Self {
            bitwise_builtin_trace_generator,
            cube_252_trace_generator,
            range_check_96_builtin_trace_generator,
            range_check_128_builtin_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_check_6_trace_generator: &range_check_6::ClaimGenerator,
        range_check_19_trace_generator: &range_check_19::ClaimGenerator,
        range_check_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
        verify_bitwise_xor_9_trace_generator: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (BuiltinsClaim, BuiltinsInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (bitwise_builtin_claim, bitwise_builtin_interaction_gen) = self
            .bitwise_builtin_trace_generator
            .map(|bitwise_builtin_trace_generator| {
                bitwise_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    verify_bitwise_xor_9_trace_generator,
                )
            })
            .unzip();
        let (cube_252_claim, cube_252_interaction_gen) = self
            .cube_252_trace_generator
            .map(|cube_252_trace_generator| {
                cube_252_trace_generator.write_trace(
                    tree_builder,
                    range_check_19_trace_generator,
                    range_check_9_9_trace_generator,
                )
            })
            .unzip();
        let (range_check_96_builtin_claim, range_check_96_builtin_interaction_gen) = self
            .range_check_96_builtin_trace_generator
            .map(|range_check_96_builtin_trace_generator| {
                range_check_96_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_6_trace_generator,
                )
            })
            .unzip();
        let (range_check_128_builtin_claim, range_check_128_builtin_interaction_gen) = self
            .range_check_128_builtin_trace_generator
            .map(|range_check_128_builtin_trace_generator| {
                range_check_128_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                )
            })
            .unzip();

        (
            BuiltinsClaim {
                bitwise_builtin: bitwise_builtin_claim,
                cube_252: cube_252_claim,
                range_check_96_builtin: range_check_96_builtin_claim,
                range_check_128_builtin: range_check_128_builtin_claim,
            },
            BuiltinsInteractionClaimGenerator {
                bitwise_builtin_interaction_gen,
                cube_252_interaction_gen,
                range_check_96_builtin_interaction_gen,
                range_check_128_builtin_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsInteractionClaim {
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
    pub cube_252: Option<cube_252::InteractionClaim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::InteractionClaim>,
}
impl BuiltinsInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(bitwise_builtin) = self.bitwise_builtin {
            bitwise_builtin.mix_into(channel)
        }
        if let Some(cube_252) = self.cube_252 {
            cube_252.mix_into(channel)
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
        if let Some(range_check_128_builtin) = self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel)
        }
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            sum += bitwise_builtin.claimed_sum;
        }
        if let Some(cube_252) = &self.cube_252 {
            sum += cube_252.claimed_sum;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            sum += range_check_96_builtin.claimed_sum;
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            sum += range_check_128_builtin.claimed_sum;
        }
        sum
    }
}

pub struct BuiltinsInteractionClaimGenerator {
    bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
    cube_252_interaction_gen: Option<cube_252::InteractionClaimGenerator>,
    range_check_96_builtin_interaction_gen:
        Option<range_check_builtin_bits_96::InteractionClaimGenerator>,
    range_check_128_builtin_interaction_gen:
        Option<range_check_builtin_bits_128::InteractionClaimGenerator>,
}
impl BuiltinsInteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> BuiltinsInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let bitwise_builtin_interaction_claim =
            self.bitwise_builtin_interaction_gen
                .map(|bitwise_builtin_interaction_gen| {
                    bitwise_builtin_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.memory_address_to_id,
                        &interaction_elements.memory_id_to_value,
                        &interaction_elements.verify_bitwise_xor_9,
                    )
                });
        let cube_252_interaction_claim =
            self.cube_252_interaction_gen
                .map(|cube_252_interaction_gen| {
                    cube_252_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.cube_252,
                        &interaction_elements.range_checks.rc_19,
                        &interaction_elements.range_checks.rc_9_9,
                    )
                });
        let range_check_96_builtin_interaction_claim = self
            .range_check_96_builtin_interaction_gen
            .map(|range_check_96_builtin_interaction_gen| {
                range_check_96_builtin_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                    &interaction_elements.range_checks.rc_6,
                )
            });
        let range_check_128_builtin_interaction_claim = self
            .range_check_128_builtin_interaction_gen
            .map(|range_check_128_builtin_interaction_gen| {
                range_check_128_builtin_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                )
            });

        BuiltinsInteractionClaim {
            bitwise_builtin: bitwise_builtin_interaction_claim,
            cube_252: cube_252_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
        }
    }
}

pub struct BuiltinComponents {
    bitwise_builtin: Option<bitwise_builtin::Component>,
    cube_252: Option<cube_252::Component>,
    range_check_96_builtin: Option<range_check_builtin_bits_96::Component>,
    range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
}
impl BuiltinComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BuiltinsClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &BuiltinsInteractionClaim,
    ) -> Self {
        let bitwise_builtin_component = claim.bitwise_builtin.map(|bitwise_builtin| {
            bitwise_builtin::Component::new(
                tree_span_provider,
                bitwise_builtin::Eval {
                    claim: bitwise_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    memory_id_to_big_lookup_elements: interaction_elements
                        .memory_id_to_value
                        .clone(),
                    verify_bitwise_xor_9_lookup_elements: interaction_elements
                        .verify_bitwise_xor_9
                        .clone(),
                },
                interaction_claim.bitwise_builtin.unwrap().claimed_sum,
            )
        });
        let cube_252_component = claim.cube_252.map(|cube_252| {
            cube_252::Component::new(
                tree_span_provider,
                cube_252::Eval {
                    claim: cube_252,
                    cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                    range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
                    range_check_9_9_lookup_elements: interaction_elements
                        .range_checks
                        .rc_9_9
                        .clone(),
                },
                interaction_claim.cube_252.unwrap().claimed_sum,
            )
        });
        let range_check_96_builtin_component =
            claim.range_check_96_builtin.map(|range_check_96_builtin| {
                range_check_builtin_bits_96::Component::new(
                    tree_span_provider,
                    range_check_builtin_bits_96::Eval {
                        claim: range_check_96_builtin,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_6_lookup_elements: interaction_elements
                            .range_checks
                            .rc_6
                            .clone(),
                    },
                    interaction_claim
                        .range_check_96_builtin
                        .unwrap()
                        .claimed_sum,
                )
            });
        let range_check_128_builtin_component =
            claim
                .range_check_128_builtin
                .map(|range_check_128_builtin| {
                    range_check_builtin_bits_128::Component::new(
                        tree_span_provider,
                        range_check_builtin_bits_128::Eval {
                            claim: range_check_128_builtin,
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                        },
                        interaction_claim
                            .range_check_128_builtin
                            .unwrap()
                            .claimed_sum,
                    )
                });
        Self {
            bitwise_builtin: bitwise_builtin_component,
            cube_252: cube_252_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            vec.push(bitwise_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(cube_252) = &self.cube_252 {
            vec.push(cube_252 as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            vec.push(range_check_96_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            vec.push(range_check_128_builtin as &dyn ComponentProver<SimdBackend>);
        }
        vec
    }
}

impl std::fmt::Display for BuiltinComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            writeln!(
                f,
                "BitwiseBuiltin: {}",
                indented_component_display(bitwise_builtin)
            )?;
        }
        if let Some(cube_252) = &self.cube_252 {
            writeln!(f, "Cube252: {}", indented_component_display(cube_252))?;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            writeln!(
                f,
                "RangeCheck96Builtin: {}",
                indented_component_display(range_check_96_builtin)
            )?;
        }
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            writeln!(
                f,
                "RangeCheck128Builtin: {}",
                indented_component_display(range_check_128_builtin)
            )?;
        }
        Ok(())
    }
}

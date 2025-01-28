use cairo_vm::types::builtin_name::BuiltinName;
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
use crate::components::{
    bitwise_builtin, memory_address_to_id, memory_id_to_big, range_check_6,
    range_check_builtin_bits_128, range_check_builtin_bits_96, verify_bitwise_xor_9,
};
use crate::input::builtin_segments::BuiltinSegments;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsClaim {
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Claim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Claim>,
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
}
impl BuiltinsClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            bitwise_builtin.mix_into(channel);
        }
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(chain!(
            self.range_check_128_builtin
                .map(|range_check_128_builtin| range_check_128_builtin.log_sizes())
                .into_iter(),
            self.range_check_96_builtin
                .map(|range_check_96_builtin| range_check_96_builtin.log_sizes())
                .into_iter(),
            self.bitwise_builtin
                .map(|bitwise_builtin| bitwise_builtin.log_sizes())
                .into_iter(),
        ))
    }
}

pub struct BuiltinsClaimGenerator {
    range_check_128_builtin_trace_generator: Option<range_check_builtin_bits_128::ClaimGenerator>,
    range_check_96_builtin_trace_generator: Option<range_check_builtin_bits_96::ClaimGenerator>,
    bitwise_builtin_trace_generator: Option<bitwise_builtin::ClaimGenerator>,
}
impl BuiltinsClaimGenerator {
    pub fn new(builtin_segments: BuiltinSegments) -> Self {
        let range_check_128_builtin_trace_generator =
            builtin_segments
                .range_check_bits_128
                .map(|range_check_bits_128| {
                    let rc128_builtin_cells_per_instance =
                    BuiltinSegments::builtin_memory_cells_per_instance(BuiltinName::range_check);
                    let rc128_builtin_segment_length = range_check_bits_128.stop_ptr - range_check_bits_128.begin_addr;
                    assert!(
                        (rc128_builtin_segment_length % rc128_builtin_cells_per_instance) == 0,
                        "range_check_bits_128 segment length is not a multiple of rc128_builtin_cells_per_instance"
                    );
                    assert!(
                        (rc128_builtin_segment_length / rc128_builtin_cells_per_instance)
                            .is_power_of_two(),
                        "range_check_bits_128 instances number is not a power of two"
                    );
                    range_check_builtin_bits_128::ClaimGenerator::new(
                        (rc128_builtin_segment_length / rc128_builtin_cells_per_instance)
                        .ilog2(),
                        range_check_bits_128.begin_addr as u32,
                    )
                });
        let range_check_96_builtin_trace_generator =
                builtin_segments
                .range_check_bits_96
                .map(|range_check_bits_96| {
                    let rc96_builtin_cells_per_instance =
                    BuiltinSegments::builtin_memory_cells_per_instance(BuiltinName::range_check96);
                    let rc96_builtin_segment_length = range_check_bits_96.stop_ptr - range_check_bits_96.begin_addr;
                    assert!(
                        (rc96_builtin_segment_length % rc96_builtin_cells_per_instance) == 0,
                        "range_check_bits_96 segment length is not a multiple of rc96_builtin_cells_per_instance"
                    );
                    assert!(
                        (rc96_builtin_segment_length / rc96_builtin_cells_per_instance)
                            .is_power_of_two(),
                        "range_check_bits_96 instances number is not a power of two"
                    );
                    range_check_builtin_bits_96::ClaimGenerator::new(
                        (rc96_builtin_segment_length / rc96_builtin_cells_per_instance)
                        .ilog2(),
                        range_check_bits_96.begin_addr as u32,
                    )
                });
        let bitwise_builtin_trace_generator = builtin_segments.bitwise.map(|bitwise| {
            let bitwise_builtin_cells_per_instance =
                BuiltinSegments::builtin_memory_cells_per_instance(BuiltinName::bitwise);
            assert!(
                ((bitwise.stop_ptr - bitwise.begin_addr) % bitwise_builtin_cells_per_instance) == 0,
                "bitwise segment length is not a multiple of bitwise_builtin_cells_per_instance"
            );
            assert!(
                ((bitwise.stop_ptr - bitwise.begin_addr) / bitwise_builtin_cells_per_instance)
                    .is_power_of_two(),
                "bitwise instances number is not a power of two"
            );
            bitwise_builtin::ClaimGenerator::new(
                ((bitwise.stop_ptr - bitwise.begin_addr) / bitwise_builtin_cells_per_instance)
                    .ilog2(),
                bitwise.begin_addr as u32,
            )
        });

        Self {
            range_check_128_builtin_trace_generator,
            range_check_96_builtin_trace_generator,
            bitwise_builtin_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_check_6_trace_generator: &range_check_6::ClaimGenerator,
        verify_bitwise_xor_9_trace_generator: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (BuiltinsClaim, BuiltinsInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (range_check_128_builtin_claim, range_check_128_builtin_interaction_gen) =
            if let Some(range_check_128_builtin_trace_generator) =
                self.range_check_128_builtin_trace_generator
            {
                let (claim, interaction_gen) = range_check_128_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                );
                (Some(claim), Some(interaction_gen))
            } else {
                (None, None)
            };
        let (range_check_96_builtin_claim, range_check_96_builtin_interaction_gen) =
            if let Some(range_check_96_builtin_trace_generator) =
                self.range_check_96_builtin_trace_generator
            {
                let (claim, interaction_gen) = range_check_96_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_6_trace_generator,
                );
                (Some(claim), Some(interaction_gen))
            } else {
                (None, None)
            };
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
            .map(|(claim, interaction_gen)| (Some(claim), Some(interaction_gen)))
            .unwrap_or((None, None));

        (
            BuiltinsClaim {
                range_check_128_builtin: range_check_128_builtin_claim,
                range_check_96_builtin: range_check_96_builtin_claim,
                bitwise_builtin: bitwise_builtin_claim,
            },
            BuiltinsInteractionClaimGenerator {
                range_check_128_builtin_interaction_gen,
                range_check_96_builtin_interaction_gen,
                bitwise_builtin_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsInteractionClaim {
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::InteractionClaim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::InteractionClaim>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
}
impl BuiltinsInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(range_check_128_builtin) = self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel)
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
        if let Some(bitwise_builtin) = self.bitwise_builtin {
            bitwise_builtin.mix_into(channel)
        }
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            sum += range_check_128_builtin.claimed_sum;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            sum += range_check_96_builtin.claimed_sum;
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            sum += bitwise_builtin.claimed_sum;
        }
        sum
    }
}

pub struct BuiltinsInteractionClaimGenerator {
    range_check_128_builtin_interaction_gen:
        Option<range_check_builtin_bits_128::InteractionClaimGenerator>,
    range_check_96_builtin_interaction_gen:
        Option<range_check_builtin_bits_96::InteractionClaimGenerator>,
    bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
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
        let range_check_128_builtin_interaction_claim = self
            .range_check_128_builtin_interaction_gen
            .map(|range_check_128_builtin_interaction_gen| {
                range_check_128_builtin_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
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
        BuiltinsInteractionClaim {
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            bitwise_builtin: bitwise_builtin_interaction_claim,
        }
    }
}

pub struct BuiltinComponents {
    range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
    range_check_96_builtin: Option<range_check_builtin_bits_96::Component>,
    bitwise_builtin: Option<bitwise_builtin::Component>,
}
impl BuiltinComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BuiltinsClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &BuiltinsInteractionClaim,
    ) -> Self {
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
        Self {
            range_check_128_builtin: range_check_128_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            vec.push(range_check_128_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            vec.push(range_check_96_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            vec.push(bitwise_builtin as &dyn ComponentProver<SimdBackend>);
        }
        vec
    }
}

impl std::fmt::Display for BuiltinComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            writeln!(
                f,
                "RangeCheck128Builtin: {}",
                indented_component_display(range_check_128_builtin)
            )?;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            writeln!(
                f,
                "RangeCheck96Builtin: {}",
                indented_component_display(range_check_96_builtin)
            )?;
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            writeln!(
                f,
                "BitwiseBuiltin: {}",
                indented_component_display(bitwise_builtin)
            )?;
        }
        Ok(())
    }
}

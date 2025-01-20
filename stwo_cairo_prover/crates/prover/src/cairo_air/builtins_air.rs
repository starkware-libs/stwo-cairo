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
    memory_address_to_id, memory_id_to_big, range_check_6, range_check_builtin_bits_128,
    range_check_builtin_bits_96,
};
use crate::input::builtin_segments::BuiltinSegments;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsClaim {
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Claim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Claim>,
}
impl BuiltinsClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel);
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
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
        ))
    }
}

pub struct BuiltinsClaimGenerator {
    range_check_128_builtin_trace_generator: Option<range_check_builtin_bits_128::ClaimGenerator>,
    range_check_96_builtin_trace_generator: Option<range_check_builtin_bits_96::ClaimGenerator>,
}
impl BuiltinsClaimGenerator {
    pub fn new(builtin_segments: BuiltinSegments) -> Self {
        let range_check_128_builtin_trace_generator =
            builtin_segments
                .range_check_bits_128
                .map(|range_check_bits_128| {
                    assert!(
                        (range_check_bits_128.stop_ptr - range_check_bits_128.begin_addr)
                            .is_power_of_two(),
                        "range_check_bits_128 segment length is not a power of two"
                    );
                    range_check_builtin_bits_128::ClaimGenerator::new(
                        (range_check_bits_128.stop_ptr - range_check_bits_128.begin_addr).ilog2(),
                        range_check_bits_128.begin_addr as u32,
                    )
                });
        let range_check_96_builtin_trace_generator =
            builtin_segments
                .range_check_bits_96
                .map(|range_check_bits_96| {
                    assert!(
                        (range_check_bits_96.stop_ptr - range_check_bits_96.begin_addr)
                            .is_power_of_two(),
                        "range_check_bits_96 segment length is not a power of two"
                    );
                    range_check_builtin_bits_96::ClaimGenerator::new(
                        (range_check_bits_96.stop_ptr - range_check_bits_96.begin_addr).ilog2(),
                        range_check_bits_96.begin_addr as u32,
                    )
                });

        Self {
            range_check_128_builtin_trace_generator,
            range_check_96_builtin_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_check_6_trace_generator: &range_check_6::ClaimGenerator,
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
        (
            BuiltinsClaim {
                range_check_128_builtin: range_check_128_builtin_claim,
                range_check_96_builtin: range_check_96_builtin_claim,
            },
            BuiltinsInteractionClaimGenerator {
                range_check_128_builtin_interaction_gen,
                range_check_96_builtin_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsInteractionClaim {
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::InteractionClaim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::InteractionClaim>,
}
impl BuiltinsInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(range_check_128_builtin) = self.range_check_128_builtin {
            range_check_128_builtin.mix_into(channel)
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            range_check_96_builtin.mix_into(channel);
        }
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        if let Some(range_check_128_builtin) = &self.range_check_128_builtin {
            sum += range_check_128_builtin.logup_sums.0;
        }
        if let Some(range_check_96_builtin) = &self.range_check_96_builtin {
            sum += range_check_96_builtin.logup_sums.0;
        }
        sum
    }
}

pub struct BuiltinsInteractionClaimGenerator {
    range_check_128_builtin_interaction_gen:
        Option<range_check_builtin_bits_128::InteractionClaimGenerator>,
    range_check_96_builtin_interaction_gen:
        Option<range_check_builtin_bits_96::InteractionClaimGenerator>,
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
                    &interaction_elements.range_check_6,
                )
            });
        BuiltinsInteractionClaim {
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
        }
    }
}

pub struct BuiltinComponents {
    range_check_128_builtin: Option<range_check_builtin_bits_128::Component>,
    range_check_96_builtin: Option<range_check_builtin_bits_96::Component>,
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
                            .logup_sums,
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
                        range_check_6_lookup_elements: interaction_elements.range_check_6.clone(),
                    },
                    interaction_claim.range_check_96_builtin.unwrap().logup_sums,
                )
            });
        Self {
            range_check_128_builtin: range_check_128_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
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
        Ok(())
    }
}

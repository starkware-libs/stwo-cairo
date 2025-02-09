use itertools::chain;
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::builtins::{
    BuiltinSegments, ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, POSEIDON_MEMORY_CELLS,
    RANGE_CHECK_MEMORY_CELLS,
};
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
use crate::components::range_check_vector::{
    range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
};
use crate::components::{
    add_mod_builtin, bitwise_builtin, cube_252, memory_address_to_id, memory_id_to_big,
    poseidon_3_partial_rounds_chain, poseidon_builtin, poseidon_full_round_chain,
    poseidon_round_keys, range_check_18, range_check_19, range_check_6, range_check_9_9,
    range_check_builtin_bits_128, range_check_builtin_bits_96, range_check_felt_252_width_27,
    verify_bitwise_xor_9,
};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsClaim {
    pub add_mod_builtin: Option<add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<bitwise_builtin::Claim>,
    pub poseidon_builtin: Option<poseidon_builtin::Claim>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::Claim>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::Claim>,
    pub cube_252: Option<cube_252::Claim>,
    pub poseidon_round_keys: Option<poseidon_round_keys::Claim>,
    pub range_check_felt_252_width_27: Option<range_check_felt_252_width_27::Claim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::Claim>,
}
impl BuiltinsClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            add_mod_builtin.mix_into(channel);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            bitwise_builtin.mix_into(channel);
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            poseidon_builtin.mix_into(channel);
        }
        if let Some(poseidon_3_partial_rounds_chain) = &self.poseidon_3_partial_rounds_chain {
            poseidon_3_partial_rounds_chain.mix_into(channel);
        }
        if let Some(poseidon_full_round_chain) = &self.poseidon_full_round_chain {
            poseidon_full_round_chain.mix_into(channel);
        }
        if let Some(cube_252) = &self.cube_252 {
            cube_252.mix_into(channel);
        }
        if let Some(poseidon_round_keys) = &self.poseidon_round_keys {
            poseidon_round_keys.mix_into(channel);
        }
        if let Some(range_check_felt_252_width_27) = &self.range_check_felt_252_width_27 {
            range_check_felt_252_width_27.mix_into(channel);
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
            self.add_mod_builtin
                .map(|add_mod_builtin| add_mod_builtin.log_sizes())
                .into_iter(),
            self.bitwise_builtin
                .map(|bitwise_builtin| bitwise_builtin.log_sizes())
                .into_iter(),
            self.poseidon_builtin
                .map(|poseidon_builtin| poseidon_builtin.log_sizes())
                .into_iter(),
            self.poseidon_3_partial_rounds_chain
                .map(|poseidon_3_partial_rounds_chain| poseidon_3_partial_rounds_chain.log_sizes())
                .into_iter(),
            self.poseidon_full_round_chain
                .map(|poseidon_full_round_chain| poseidon_full_round_chain.log_sizes())
                .into_iter(),
            self.cube_252
                .map(|cube_252| cube_252.log_sizes())
                .into_iter(),
            self.poseidon_round_keys
                .map(|poseidon_round_keys| poseidon_round_keys.log_sizes())
                .into_iter(),
            self.range_check_felt_252_width_27
                .map(|range_check_felt_252_width_27| range_check_felt_252_width_27.log_sizes())
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
    add_mod_builtin_trace_generator: Option<add_mod_builtin::ClaimGenerator>,
    bitwise_builtin_trace_generator: Option<bitwise_builtin::ClaimGenerator>,
    poseidon_builtin_trace_generator: Option<poseidon_builtin::ClaimGenerator>,
    poseidon_3_partial_rounds_chain_trace_generator:
        Option<poseidon_3_partial_rounds_chain::ClaimGenerator>,
    poseidon_full_round_chain_trace_generator: Option<poseidon_full_round_chain::ClaimGenerator>,
    cube_252_trace_generator: Option<cube_252::ClaimGenerator>,
    poseidon_round_keys_trace_generator: Option<poseidon_round_keys::ClaimGenerator>,
    range_check_felt_252_width_27_trace_generator:
        Option<range_check_felt_252_width_27::ClaimGenerator>,
    range_check_96_builtin_trace_generator: Option<range_check_builtin_bits_96::ClaimGenerator>,
    range_check_128_builtin_trace_generator: Option<range_check_builtin_bits_128::ClaimGenerator>,
}
impl BuiltinsClaimGenerator {
    pub fn new(builtin_segments: BuiltinSegments) -> Self {
        let add_mod_builtin_trace_generator = builtin_segments.add_mod.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                (segment_length % ADD_MOD_MEMORY_CELLS) == 0,
                "add mod segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / ADD_MOD_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "add mod instances number is not a power of two"
            );
            add_mod_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
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
        let (
            poseidon_builtin_trace_generator,
            poseidon_3_partial_rounds_chain_trace_generator,
            poseidon_full_round_chain_trace_generator,
            cube_252_trace_generator,
            poseidon_round_keys_trace_generator,
            range_check_felt_252_width_27_trace_generator,
        ) = builtin_segments
            .poseidon
            .map(|segment| {
                let segment_length = segment.stop_ptr - segment.begin_addr;
                assert!(
                    (segment_length % POSEIDON_MEMORY_CELLS) == 0,
                    "poseidon segment length is not a multiple of it's cells_per_instance"
                );
                let n_instances = segment_length / POSEIDON_MEMORY_CELLS;
                assert!(
                    n_instances.is_power_of_two(),
                    "poseidon instances number is not a power of two"
                );
                (
                    Some(poseidon_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )),
                    Some(poseidon_3_partial_rounds_chain::ClaimGenerator::new()),
                    Some(poseidon_full_round_chain::ClaimGenerator::new()),
                    Some(cube_252::ClaimGenerator::new()),
                    Some(poseidon_round_keys::ClaimGenerator::new()),
                    Some(range_check_felt_252_width_27::ClaimGenerator::new()),
                )
            })
            .unwrap_or((None, None, None, None, None, None));

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
            add_mod_builtin_trace_generator,
            bitwise_builtin_trace_generator,
            poseidon_builtin_trace_generator,
            poseidon_3_partial_rounds_chain_trace_generator,
            poseidon_full_round_chain_trace_generator,
            cube_252_trace_generator,
            poseidon_round_keys_trace_generator,
            range_check_felt_252_width_27_trace_generator,
            range_check_96_builtin_trace_generator,
            range_check_128_builtin_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_check_6_trace_generator: &range_check_6::ClaimGenerator,
        range_check_18_trace_generator: &range_check_18::ClaimGenerator,
        range_check_19_trace_generator: &range_check_19::ClaimGenerator,
        range_check_4_4_trace_generator: &range_check_4_4::ClaimGenerator,
        range_check_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
        range_check_4_4_4_4_trace_generator: &range_check_4_4_4_4::ClaimGenerator,
        range_check_3_3_3_3_3_trace_generator: &range_check_3_3_3_3_3::ClaimGenerator,
        verify_bitwise_xor_9_trace_generator: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (BuiltinsClaim, BuiltinsInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (add_mod_builtin_claim, add_mod_builtin_interaction_gen) = self
            .add_mod_builtin_trace_generator
            .map(|add_mod_builtin_trace_generator| {
                add_mod_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                )
            })
            .unzip();
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
        let (poseidon_builtin_claim, poseidon_builtin_interaction_gen) = self
            .poseidon_builtin_trace_generator
            .map(|poseidon_builtin_trace_generator| {
                poseidon_builtin_trace_generator.write_trace(
                    tree_builder,
                    self.cube_252_trace_generator.as_mut().unwrap(),
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    self.poseidon_3_partial_rounds_chain_trace_generator
                        .as_mut()
                        .unwrap(),
                    self.poseidon_full_round_chain_trace_generator
                        .as_mut()
                        .unwrap(),
                    range_check_3_3_3_3_3_trace_generator,
                    range_check_4_4_trace_generator,
                    range_check_4_4_4_4_trace_generator,
                    self.range_check_felt_252_width_27_trace_generator
                        .as_mut()
                        .unwrap(),
                )
            })
            .unzip();
        let (
            poseidon_3_partial_rounds_chain_claim,
            poseidon_3_partial_rounds_chain_interaction_gen,
        ) = (if let Some(poseidon_3_partial_rounds_chain_trace_generator) =
            self.poseidon_3_partial_rounds_chain_trace_generator
        {
            Some(
                poseidon_3_partial_rounds_chain_trace_generator.write_trace(
                    tree_builder,
                    self.cube_252_trace_generator.as_mut().unwrap(),
                    self.poseidon_round_keys_trace_generator.as_ref().unwrap(),
                    range_check_4_4_trace_generator,
                    range_check_4_4_4_4_trace_generator,
                    self.range_check_felt_252_width_27_trace_generator
                        .as_mut()
                        .unwrap(),
                ),
            )
        } else {
            None
        })
        .unzip();
        let (poseidon_full_round_chain_claim, poseidon_full_round_chain_interaction_gen) =
            (if let Some(poseidon_full_round_chain_trace_generator) =
                self.poseidon_full_round_chain_trace_generator
            {
                Some(poseidon_full_round_chain_trace_generator.write_trace(
                    tree_builder,
                    self.cube_252_trace_generator.as_mut().unwrap(),
                    self.poseidon_round_keys_trace_generator.as_ref().unwrap(),
                    range_check_3_3_3_3_3_trace_generator,
                ))
            } else {
                None
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
        let (poseidon_round_keys_claim, poseidon_round_keys_interaction_gen) = self
            .poseidon_round_keys_trace_generator
            .map(|poseidon_round_keys_trace_generator| {
                poseidon_round_keys_trace_generator.write_trace(tree_builder)
            })
            .unzip();
        let (range_check_felt_252_width_27_claim, range_check_felt_252_width_27_interaction_gen) =
            self.range_check_felt_252_width_27_trace_generator
                .map(|range_check_felt_252_width_27_trace_generator| {
                    range_check_felt_252_width_27_trace_generator.write_trace(
                        tree_builder,
                        range_check_18_trace_generator,
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
                add_mod_builtin: add_mod_builtin_claim,
                bitwise_builtin: bitwise_builtin_claim,
                poseidon_builtin: poseidon_builtin_claim,
                poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim,
                poseidon_full_round_chain: poseidon_full_round_chain_claim,
                cube_252: cube_252_claim,
                poseidon_round_keys: poseidon_round_keys_claim,
                range_check_felt_252_width_27: range_check_felt_252_width_27_claim,
                range_check_96_builtin: range_check_96_builtin_claim,
                range_check_128_builtin: range_check_128_builtin_claim,
            },
            BuiltinsInteractionClaimGenerator {
                add_mod_builtin_interaction_gen,
                bitwise_builtin_interaction_gen,
                poseidon_builtin_interaction_gen,
                poseidon_3_partial_rounds_chain_interaction_gen,
                poseidon_full_round_chain_interaction_gen,
                cube_252_interaction_gen,
                poseidon_round_keys_interaction_gen,
                range_check_felt_252_width_27_interaction_gen,
                range_check_96_builtin_interaction_gen,
                range_check_128_builtin_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BuiltinsInteractionClaim {
    pub add_mod_builtin: Option<add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaim>,
    pub poseidon_builtin: Option<poseidon_builtin::InteractionClaim>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::InteractionClaim>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::InteractionClaim>,
    pub cube_252: Option<cube_252::InteractionClaim>,
    pub poseidon_round_keys: Option<poseidon_round_keys::InteractionClaim>,
    pub range_check_felt_252_width_27: Option<range_check_felt_252_width_27::InteractionClaim>,
    pub range_check_96_builtin: Option<range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<range_check_builtin_bits_128::InteractionClaim>,
}
impl BuiltinsInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            add_mod_builtin.mix_into(channel);
        }
        if let Some(bitwise_builtin) = self.bitwise_builtin {
            bitwise_builtin.mix_into(channel)
        }
        if let Some(poseidon_builtin) = self.poseidon_builtin {
            poseidon_builtin.mix_into(channel)
        }
        if let Some(poseidon_3_partial_rounds_chain) = self.poseidon_3_partial_rounds_chain {
            poseidon_3_partial_rounds_chain.mix_into(channel)
        }
        if let Some(poseidon_full_round_chain) = self.poseidon_full_round_chain {
            poseidon_full_round_chain.mix_into(channel)
        }
        if let Some(cube_252) = self.cube_252 {
            cube_252.mix_into(channel)
        }
        if let Some(poseidon_round_keys) = self.poseidon_round_keys {
            poseidon_round_keys.mix_into(channel)
        }
        if let Some(range_check_felt_252_width_27) = self.range_check_felt_252_width_27 {
            range_check_felt_252_width_27.mix_into(channel)
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
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            sum += add_mod_builtin.claimed_sum;
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            sum += bitwise_builtin.claimed_sum;
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            sum += poseidon_builtin.claimed_sum;
        }
        if let Some(poseidon_3_partial_rounds_chain) = &self.poseidon_3_partial_rounds_chain {
            sum += poseidon_3_partial_rounds_chain.claimed_sum;
        }
        if let Some(poseidon_full_round_chain) = &self.poseidon_full_round_chain {
            sum += poseidon_full_round_chain.claimed_sum;
        }
        if let Some(cube_252) = &self.cube_252 {
            sum += cube_252.claimed_sum;
        }
        if let Some(poseidon_round_keys) = &self.poseidon_round_keys {
            sum += poseidon_round_keys.claimed_sum;
        }
        if let Some(range_check_felt_252_width_27) = &self.range_check_felt_252_width_27 {
            sum += range_check_felt_252_width_27.claimed_sum;
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
    add_mod_builtin_interaction_gen: Option<add_mod_builtin::InteractionClaimGenerator>,
    bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
    poseidon_builtin_interaction_gen: Option<poseidon_builtin::InteractionClaimGenerator>,
    poseidon_3_partial_rounds_chain_interaction_gen:
        Option<poseidon_3_partial_rounds_chain::InteractionClaimGenerator>,
    poseidon_full_round_chain_interaction_gen:
        Option<poseidon_full_round_chain::InteractionClaimGenerator>,
    cube_252_interaction_gen: Option<cube_252::InteractionClaimGenerator>,
    poseidon_round_keys_interaction_gen: Option<poseidon_round_keys::InteractionClaimGenerator>,
    range_check_felt_252_width_27_interaction_gen:
        Option<range_check_felt_252_width_27::InteractionClaimGenerator>,
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
        let add_mod_builtin_interaction_claim =
            self.add_mod_builtin_interaction_gen
                .map(|add_mod_builtin_interaction_gen| {
                    add_mod_builtin_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.memory_address_to_id,
                        &interaction_elements.memory_id_to_value,
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
        let poseidon_builtin_interaction_claim =
            self.poseidon_builtin_interaction_gen
                .map(|poseidon_builtin_interaction_gen| {
                    poseidon_builtin_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.cube_252,
                        &interaction_elements.memory_address_to_id,
                        &interaction_elements.memory_id_to_value,
                        &interaction_elements.poseidon_3_partial_rounds_chain,
                        &interaction_elements.poseidon_full_round_chain,
                        &interaction_elements.range_check_felt_252_width_27,
                        &interaction_elements.range_checks.rc_3_3_3_3_3,
                        &interaction_elements.range_checks.rc_4_4,
                        &interaction_elements.range_checks.rc_4_4_4_4,
                    )
                });
        let poseidon_3_partial_rounds_chain_interaction_claim = self
            .poseidon_3_partial_rounds_chain_interaction_gen
            .map(|poseidon_3_partial_rounds_chain_interaction_gen| {
                poseidon_3_partial_rounds_chain_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.cube_252,
                    &interaction_elements.poseidon_3_partial_rounds_chain,
                    &interaction_elements.poseidon_round_keys,
                    &interaction_elements.range_check_felt_252_width_27,
                    &interaction_elements.range_checks.rc_4_4,
                    &interaction_elements.range_checks.rc_4_4_4_4,
                )
            });
        let poseidon_full_round_chain_interaction_claim = self
            .poseidon_full_round_chain_interaction_gen
            .map(|poseidon_full_round_chain_interaction_gen| {
                poseidon_full_round_chain_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.cube_252,
                    &interaction_elements.poseidon_full_round_chain,
                    &interaction_elements.poseidon_round_keys,
                    &interaction_elements.range_checks.rc_3_3_3_3_3,
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
        let poseidon_round_keys_interaction_claim =
            self.poseidon_round_keys_interaction_gen
                .map(|poseidon_round_keys_interaction_gen| {
                    poseidon_round_keys_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.poseidon_round_keys,
                    )
                });
        let range_check_felt_252_width_27_interaction_claim = self
            .range_check_felt_252_width_27_interaction_gen
            .map(|range_check_felt_252_width_27_interaction_gen| {
                range_check_felt_252_width_27_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.range_check_felt_252_width_27,
                    &interaction_elements.range_checks.rc_18,
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
            add_mod_builtin: add_mod_builtin_interaction_claim,
            bitwise_builtin: bitwise_builtin_interaction_claim,
            poseidon_builtin: poseidon_builtin_interaction_claim,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_interaction_claim,
            poseidon_full_round_chain: poseidon_full_round_chain_interaction_claim,
            cube_252: cube_252_interaction_claim,
            poseidon_round_keys: poseidon_round_keys_interaction_claim,
            range_check_felt_252_width_27: range_check_felt_252_width_27_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
        }
    }
}

pub struct BuiltinComponents {
    add_mod_builtin: Option<add_mod_builtin::Component>,
    bitwise_builtin: Option<bitwise_builtin::Component>,
    poseidon_builtin: Option<poseidon_builtin::Component>,
    poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::Component>,
    poseidon_full_round_chain: Option<poseidon_full_round_chain::Component>,
    cube_252: Option<cube_252::Component>,
    poseidon_round_keys: Option<poseidon_round_keys::Component>,
    range_check_felt_252_width_27: Option<range_check_felt_252_width_27::Component>,
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
        let add_mod_builtin_component = claim.add_mod_builtin.map(|add_mod_builtin| {
            add_mod_builtin::Component::new(
                tree_span_provider,
                add_mod_builtin::Eval {
                    claim: add_mod_builtin,
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    memory_id_to_big_lookup_elements: interaction_elements
                        .memory_id_to_value
                        .clone(),
                },
                interaction_claim.add_mod_builtin.unwrap().claimed_sum,
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
        let poseidon_builtin_component = claim.poseidon_builtin.map(|poseidon_builtin| {
            poseidon_builtin::Component::new(
                tree_span_provider,
                poseidon_builtin::Eval {
                    claim: poseidon_builtin,
                    cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                    memory_address_to_id_lookup_elements: interaction_elements
                        .memory_address_to_id
                        .clone(),
                    memory_id_to_big_lookup_elements: interaction_elements
                        .memory_id_to_value
                        .clone(),
                    poseidon_3_partial_rounds_chain_lookup_elements: interaction_elements
                        .poseidon_3_partial_rounds_chain
                        .clone(),
                    poseidon_full_round_chain_lookup_elements: interaction_elements
                        .poseidon_full_round_chain
                        .clone(),
                    range_check_3_3_3_3_3_lookup_elements: interaction_elements
                        .range_checks
                        .rc_3_3_3_3_3
                        .clone(),
                    range_check_4_4_lookup_elements: interaction_elements
                        .range_checks
                        .rc_4_4
                        .clone(),
                    range_check_4_4_4_4_lookup_elements: interaction_elements
                        .range_checks
                        .rc_4_4_4_4
                        .clone(),
                    range_check_felt_252_width_27_lookup_elements: interaction_elements
                        .range_check_felt_252_width_27
                        .clone(),
                },
                interaction_claim.poseidon_builtin.unwrap().claimed_sum,
            )
        });
        let poseidon_3_partial_rounds_chain_component =
            claim
                .poseidon_3_partial_rounds_chain
                .map(|poseidon_3_partial_rounds_chain| {
                    poseidon_3_partial_rounds_chain::Component::new(
                        tree_span_provider,
                        poseidon_3_partial_rounds_chain::Eval {
                            claim: poseidon_3_partial_rounds_chain,
                            cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                            poseidon_3_partial_rounds_chain_lookup_elements: interaction_elements
                                .poseidon_3_partial_rounds_chain
                                .clone(),
                            poseidon_round_keys_lookup_elements: interaction_elements
                                .poseidon_round_keys
                                .clone(),
                            range_check_4_4_lookup_elements: interaction_elements
                                .range_checks
                                .rc_4_4
                                .clone(),
                            range_check_4_4_4_4_lookup_elements: interaction_elements
                                .range_checks
                                .rc_4_4_4_4
                                .clone(),
                            range_check_felt_252_width_27_lookup_elements: interaction_elements
                                .range_check_felt_252_width_27
                                .clone(),
                        },
                        interaction_claim
                            .poseidon_3_partial_rounds_chain
                            .unwrap()
                            .claimed_sum,
                    )
                });
        let poseidon_full_round_chain_component =
            claim
                .poseidon_full_round_chain
                .map(|poseidon_full_round_chain| {
                    poseidon_full_round_chain::Component::new(
                        tree_span_provider,
                        poseidon_full_round_chain::Eval {
                            claim: poseidon_full_round_chain,
                            cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                            poseidon_full_round_chain_lookup_elements: interaction_elements
                                .poseidon_full_round_chain
                                .clone(),
                            poseidon_round_keys_lookup_elements: interaction_elements
                                .poseidon_round_keys
                                .clone(),
                            range_check_3_3_3_3_3_lookup_elements: interaction_elements
                                .range_checks
                                .rc_3_3_3_3_3
                                .clone(),
                        },
                        interaction_claim
                            .poseidon_full_round_chain
                            .unwrap()
                            .claimed_sum,
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
        let poseidon_round_keys_component = claim.poseidon_round_keys.map(|poseidon_round_keys| {
            poseidon_round_keys::Component::new(
                tree_span_provider,
                poseidon_round_keys::Eval {
                    claim: poseidon_round_keys,
                    poseidon_round_keys_lookup_elements: interaction_elements
                        .poseidon_round_keys
                        .clone(),
                },
                interaction_claim.poseidon_round_keys.unwrap().claimed_sum,
            )
        });
        let range_check_felt_252_width_27_component =
            claim
                .range_check_felt_252_width_27
                .map(|range_check_felt_252_width_27| {
                    range_check_felt_252_width_27::Component::new(
                        tree_span_provider,
                        range_check_felt_252_width_27::Eval {
                            claim: (range_check_felt_252_width_27),
                            range_check_felt_252_width_27_lookup_elements: (interaction_elements
                                .range_check_felt_252_width_27
                                .clone()),
                            range_check_18_lookup_elements: (interaction_elements
                                .range_checks
                                .rc_18
                                .clone()),
                            range_check_9_9_lookup_elements: (interaction_elements
                                .range_checks
                                .rc_9_9
                                .clone()),
                        },
                        interaction_claim
                            .range_check_felt_252_width_27
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
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_felt_252_width_27: range_check_felt_252_width_27_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            vec.push(add_mod_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            vec.push(bitwise_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            vec.push(poseidon_builtin as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(poseidon_3_partial_rounds_chain) = &self.poseidon_3_partial_rounds_chain {
            vec.push(poseidon_3_partial_rounds_chain as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(poseidon_full_round_chain) = &self.poseidon_full_round_chain {
            vec.push(poseidon_full_round_chain as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(cube_252) = &self.cube_252 {
            vec.push(cube_252 as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(poseidon_round_keys) = &self.poseidon_round_keys {
            vec.push(poseidon_round_keys as &dyn ComponentProver<SimdBackend>);
        }
        if let Some(range_check_felt_252_width_27) = &self.range_check_felt_252_width_27 {
            vec.push(range_check_felt_252_width_27 as &dyn ComponentProver<SimdBackend>);
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
        if let Some(add_mod_builtin) = &self.add_mod_builtin {
            writeln!(
                f,
                "AddModBuiltin: {}",
                indented_component_display(add_mod_builtin)
            )?;
        }
        if let Some(bitwise_builtin) = &self.bitwise_builtin {
            writeln!(
                f,
                "BitwiseBuiltin: {}",
                indented_component_display(bitwise_builtin)
            )?;
        }
        if let Some(poseidon_builtin) = &self.poseidon_builtin {
            writeln!(
                f,
                "PoseidonBuiltin: {}",
                indented_component_display(poseidon_builtin)
            )?;
        }
        if let Some(poseidon_3_partial_rounds_chain) = &self.poseidon_3_partial_rounds_chain {
            writeln!(
                f,
                "Poseidon3PartialRoundsChain: {}",
                indented_component_display(poseidon_3_partial_rounds_chain)
            )?;
        }
        if let Some(poseidon_full_round_chain) = &self.poseidon_full_round_chain {
            writeln!(
                f,
                "PoseidonFullRoundChain: {}",
                indented_component_display(poseidon_full_round_chain)
            )?;
        }
        if let Some(cube_252) = &self.cube_252 {
            writeln!(f, "Cube252: {}", indented_component_display(cube_252))?;
        }
        if let Some(poseidon_round_keys) = &self.poseidon_round_keys {
            writeln!(
                f,
                "PoseidonRoundKeys: {}",
                indented_component_display(poseidon_round_keys)
            )?;
        }
        if let Some(range_check_felt_252_width_27) = &self.range_check_felt_252_width_27 {
            writeln!(
                f,
                "RangeCheckFelt252Width27: {}",
                indented_component_display(range_check_felt_252_width_27)
            )?;
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

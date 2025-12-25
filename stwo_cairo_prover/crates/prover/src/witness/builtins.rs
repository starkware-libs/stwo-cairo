use cairo_air::air::CairoInteractionElements;
use cairo_air::builtins_air::{BuiltinsClaim, BuiltinsInteractionClaim};
use stwo::prover::backend::simd::SimdBackend;

use crate::witness::components::{
    add_mod_builtin, bitwise_builtin, memory_address_to_id, memory_id_to_big, mul_mod_builtin,
    pedersen_aggregator, pedersen_builtin, poseidon_aggregator, poseidon_builtin,
    range_check96_builtin, range_check_12, range_check_18, range_check_3_6_6_3, range_check_6,
    range_check_builtin, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::witness::utils::TreeBuilder;

pub fn builtins_write_trace(
    add_mod_builtin: Option<add_mod_builtin::ClaimGenerator>,
    bitwise_builtin: Option<bitwise_builtin::ClaimGenerator>,
    mul_mod_builtin: Option<mul_mod_builtin::ClaimGenerator>,
    pedersen_builtin: Option<pedersen_builtin::ClaimGenerator>,
    poseidon_builtin: Option<poseidon_builtin::ClaimGenerator>,
    range_check_96_builtin: Option<range_check96_builtin::ClaimGenerator>,
    range_check_128_builtin: Option<range_check_builtin::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    pedersen_aggregator_trace_generator: Option<&pedersen_aggregator::ClaimGenerator>,
    poseidon_aggregator_trace_generator: Option<&poseidon_aggregator::ClaimGenerator>,
    range_check_6_trace_generator: Option<&range_check_6::ClaimGenerator>,
    range_check_12_trace_generator: Option<&range_check_12::ClaimGenerator>,
    range_check_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    range_check_3_6_6_3_trace_generator: Option<&range_check_3_6_6_3::ClaimGenerator>,
    verify_bitwise_xor_8_trace_generator: Option<&verify_bitwise_xor_8::ClaimGenerator>,
    verify_bitwise_xor_9_trace_generator: Option<&verify_bitwise_xor_9::ClaimGenerator>,
) -> (BuiltinsClaim, BuiltinsInteractionClaimGenerator) {
    let (add_mod_builtin_claim, add_mod_builtin_interaction_gen) = add_mod_builtin
        .map(|add_mod_builtin_trace_generator| {
            add_mod_builtin_trace_generator.write_trace(
                tree_builder,
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
            )
        })
        .unzip();
    let (bitwise_builtin_claim, bitwise_builtin_interaction_gen) = bitwise_builtin
        .map(|bitwise_builtin_trace_generator| {
            bitwise_builtin_trace_generator.write_trace(
                tree_builder,
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_bitwise_xor_9_trace_generator.unwrap(),
                verify_bitwise_xor_8_trace_generator.unwrap(),
            )
        })
        .unzip();
    let (mul_mod_builtin_claim, mul_mod_builtin_interaction_gen) = mul_mod_builtin
        .map(|mul_mod_builtin_trace_generator| {
            mul_mod_builtin_trace_generator.write_trace(
                tree_builder,
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                range_check_12_trace_generator.unwrap(),
                range_check_3_6_6_3_trace_generator.unwrap(),
                range_check_18_trace_generator.unwrap(),
            )
        })
        .unzip();
    let (pedersen_builtin_claim, pedersen_builtin_interaction_gen) = pedersen_builtin
        .map(|pedersen_builtin_trace_generator| {
            pedersen_builtin_trace_generator.write_trace(
                tree_builder,
                memory_address_to_id_trace_generator.unwrap(),
                pedersen_aggregator_trace_generator.unwrap(),
            )
        })
        .unzip();
    let (poseidon_builtin_claim, poseidon_builtin_interaction_gen) = poseidon_builtin
        .map(|poseidon_builtin_trace_generator| {
            poseidon_builtin_trace_generator.write_trace(
                tree_builder,
                memory_address_to_id_trace_generator.unwrap(),
                poseidon_aggregator_trace_generator.unwrap(),
            )
        })
        .unzip();

    let (range_check_96_builtin_claim, range_check_96_builtin_interaction_gen) =
        range_check_96_builtin
            .map(|range_check_96_builtin_trace_generator| {
                range_check_96_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    range_check_6_trace_generator.unwrap(),
                )
            })
            .unzip();
    let (range_check_128_builtin_claim, range_check_128_builtin_interaction_gen) =
        range_check_128_builtin
            .map(|range_check_128_builtin_trace_generator| {
                range_check_128_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                )
            })
            .unzip();

    (
        BuiltinsClaim {
            add_mod_builtin: add_mod_builtin_claim,
            bitwise_builtin: bitwise_builtin_claim,
            mul_mod_builtin: mul_mod_builtin_claim,
            pedersen_builtin: pedersen_builtin_claim,
            poseidon_builtin: poseidon_builtin_claim,
            range_check_96_builtin: range_check_96_builtin_claim,
            range_check_128_builtin: range_check_128_builtin_claim,
        },
        BuiltinsInteractionClaimGenerator {
            add_mod_builtin_interaction_gen,
            bitwise_builtin_interaction_gen,
            mul_mod_builtin_interaction_gen,
            pedersen_builtin_interaction_gen,
            poseidon_builtin_interaction_gen,
            range_check_96_builtin_interaction_gen,
            range_check_128_builtin_interaction_gen,
        },
    )
}

pub struct BuiltinsInteractionClaimGenerator {
    add_mod_builtin_interaction_gen: Option<add_mod_builtin::InteractionClaimGenerator>,
    bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
    mul_mod_builtin_interaction_gen: Option<mul_mod_builtin::InteractionClaimGenerator>,
    pedersen_builtin_interaction_gen: Option<pedersen_builtin::InteractionClaimGenerator>,
    poseidon_builtin_interaction_gen: Option<poseidon_builtin::InteractionClaimGenerator>,
    range_check_96_builtin_interaction_gen:
        Option<range_check96_builtin::InteractionClaimGenerator>,
    range_check_128_builtin_interaction_gen: Option<range_check_builtin::InteractionClaimGenerator>,
}
impl BuiltinsInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> BuiltinsInteractionClaim {
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
                        &interaction_elements.verify_bitwise_xor_8,
                    )
                });
        let mul_mod_builtin_interaction_claim =
            self.mul_mod_builtin_interaction_gen
                .map(|mul_mod_builtin_interaction_gen| {
                    mul_mod_builtin_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.memory_address_to_id,
                        &interaction_elements.memory_id_to_value,
                        &interaction_elements.range_checks.rc_12,
                        &interaction_elements.range_checks.rc_3_6_6_3,
                        &interaction_elements.range_checks.rc_18,
                    )
                });
        let pedersen_builtin_interaction_claim =
            self.pedersen_builtin_interaction_gen
                .map(|pedersen_builtin_interaction_gen| {
                    pedersen_builtin_interaction_gen.write_interaction_trace(
                        tree_builder,
                        &interaction_elements.memory_address_to_id,
                        &interaction_elements.pedersen_aggregator,
                    )
                });
        let poseidon_builtin_interaction_claim = self.poseidon_builtin_interaction_gen.map(
            |poseidon_builtin_interaction_gen: poseidon_builtin::InteractionClaimGenerator| {
                poseidon_builtin_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.poseidon_aggregator,
                )
            },
        );
        let range_check_96_builtin_interaction_claim = self
            .range_check_96_builtin_interaction_gen
            .map(|range_check_96_builtin_interaction_gen| {
                range_check_96_builtin_interaction_gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.range_checks.rc_6,
                    &interaction_elements.memory_id_to_value,
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
            mul_mod_builtin: mul_mod_builtin_interaction_claim,
            pedersen_builtin: pedersen_builtin_interaction_claim,
            poseidon_builtin: poseidon_builtin_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
        }
    }
}

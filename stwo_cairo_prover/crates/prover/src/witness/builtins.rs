use cairo_air::builtins_air::{BuiltinsClaim, BuiltinsInteractionClaim};
use cairo_air::relations::CommonLookupElements;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::builtins::BuiltinSegments;
use stwo_cairo_common::builtins::{
    ADD_MOD_BUILTIN_MEMORY_CELLS, BITWISE_BUILTIN_MEMORY_CELLS, MUL_MOD_BUILTIN_MEMORY_CELLS,
    PEDERSEN_BUILTIN_MEMORY_CELLS, POSEIDON_BUILTIN_MEMORY_CELLS,
    RANGE_CHECK_96_BUILTIN_MEMORY_CELLS, RANGE_CHECK_BUILTIN_MEMORY_CELLS,
};

use crate::witness::components::{
    add_mod_builtin, bitwise_builtin, memory_address_to_id, memory_id_to_big, mul_mod_builtin,
    pedersen_aggregator_window_bits_18, pedersen_builtin, poseidon_aggregator, poseidon_builtin,
    range_check96_builtin, range_check_12, range_check_18, range_check_3_6_6_3, range_check_6,
    range_check_builtin, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::witness::utils::TreeBuilder;

pub fn get_builtins(builtin_segments: &BuiltinSegments) -> Vec<&'static str> {
    let mut builtins = vec![];

    if let Some(segment) = builtin_segments.add_mod_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(ADD_MOD_BUILTIN_MEMORY_CELLS),
            "add mod segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / ADD_MOD_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "add mod instances number is not a power of two"
        );
        builtins.push("add_mod_builtin");
    }
    if let Some(segment) = builtin_segments.bitwise_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(BITWISE_BUILTIN_MEMORY_CELLS),
            "bitwise segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / BITWISE_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "bitwise instances number is not a power of two"
        );
        builtins.push("bitwise_builtin");
    }
    if let Some(segment) = builtin_segments.mul_mod_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(MUL_MOD_BUILTIN_MEMORY_CELLS),
            "mul mod segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / MUL_MOD_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "mul mod instances number is not a power of two"
        );
        builtins.push("mul_mod_builtin");
    }
    if let Some(segment) = builtin_segments.pedersen_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(PEDERSEN_BUILTIN_MEMORY_CELLS),
            "pedersen segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / PEDERSEN_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "pedersen instances number is not a power of two"
        );
        builtins.push("pedersen_builtin");
    }
    if let Some(segment) = builtin_segments.poseidon_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(POSEIDON_BUILTIN_MEMORY_CELLS),
            "poseidon segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / POSEIDON_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "poseidon instances number is not a power of two"
        );
        builtins.push("poseidon_builtin");
    }
    if let Some(segment) = builtin_segments.range_check96_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(RANGE_CHECK_96_BUILTIN_MEMORY_CELLS),
            "range_check96 segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / RANGE_CHECK_96_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "range_check96 instances number is not a power of two"
        );
        builtins.push("range_check96_builtin");
    }
    if let Some(segment) = builtin_segments.range_check_builtin {
        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(RANGE_CHECK_BUILTIN_MEMORY_CELLS),
            "range_check segment length is not a multiple of it's cells_per_instance"
        );
        let n_instances = segment_length / RANGE_CHECK_BUILTIN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "range_check instances number is not a power of two"
        );
        builtins.push("range_check_builtin");
    }

    builtins
}

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
    pedersen_aggregator_trace_generator: Option<
        &pedersen_aggregator_window_bits_18::ClaimGenerator,
    >,
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

/// Parts of the builtins interaction claim generator for parallel processing.
pub struct BuiltinsInteractionParts {
    pub add_mod_builtin: Option<add_mod_builtin::InteractionClaimGenerator>,
    pub bitwise_builtin: Option<bitwise_builtin::InteractionClaimGenerator>,
    pub mul_mod_builtin: Option<mul_mod_builtin::InteractionClaimGenerator>,
    pub pedersen_builtin: Option<pedersen_builtin::InteractionClaimGenerator>,
    pub poseidon_builtin: Option<poseidon_builtin::InteractionClaimGenerator>,
    pub range_check_96_builtin: Option<range_check96_builtin::InteractionClaimGenerator>,
    pub range_check_128_builtin: Option<range_check_builtin::InteractionClaimGenerator>,
}

impl BuiltinsInteractionClaimGenerator {
    /// Decompose into individual parts for parallel processing.
    pub fn into_parts(self) -> BuiltinsInteractionParts {
        BuiltinsInteractionParts {
            add_mod_builtin: self.add_mod_builtin_interaction_gen,
            bitwise_builtin: self.bitwise_builtin_interaction_gen,
            mul_mod_builtin: self.mul_mod_builtin_interaction_gen,
            pedersen_builtin: self.pedersen_builtin_interaction_gen,
            poseidon_builtin: self.poseidon_builtin_interaction_gen,
            range_check_96_builtin: self.range_check_96_builtin_interaction_gen,
            range_check_128_builtin: self.range_check_128_builtin_interaction_gen,
        }
    }

    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> BuiltinsInteractionClaim {
        let add_mod_builtin_interaction_claim =
            self.add_mod_builtin_interaction_gen
                .map(|add_mod_builtin_interaction_gen| {
                    add_mod_builtin_interaction_gen
                        .write_interaction_trace(tree_builder, common_lookup_elements)
                });
        let bitwise_builtin_interaction_claim =
            self.bitwise_builtin_interaction_gen
                .map(|bitwise_builtin_interaction_gen| {
                    bitwise_builtin_interaction_gen
                        .write_interaction_trace(tree_builder, common_lookup_elements)
                });
        let mul_mod_builtin_interaction_claim =
            self.mul_mod_builtin_interaction_gen
                .map(|mul_mod_builtin_interaction_gen| {
                    mul_mod_builtin_interaction_gen
                        .write_interaction_trace(tree_builder, common_lookup_elements)
                });
        let pedersen_builtin_interaction_claim =
            self.pedersen_builtin_interaction_gen
                .map(|pedersen_builtin_interaction_gen| {
                    pedersen_builtin_interaction_gen
                        .write_interaction_trace(tree_builder, common_lookup_elements)
                });
        let poseidon_builtin_interaction_claim = self.poseidon_builtin_interaction_gen.map(
            |poseidon_builtin_interaction_gen: poseidon_builtin::InteractionClaimGenerator| {
                poseidon_builtin_interaction_gen
                    .write_interaction_trace(tree_builder, common_lookup_elements)
            },
        );
        let range_check_96_builtin_interaction_claim = self
            .range_check_96_builtin_interaction_gen
            .map(|range_check_96_builtin_interaction_gen| {
                range_check_96_builtin_interaction_gen
                    .write_interaction_trace(tree_builder, common_lookup_elements)
            });
        let range_check_128_builtin_interaction_claim = self
            .range_check_128_builtin_interaction_gen
            .map(|range_check_128_builtin_interaction_gen| {
                range_check_128_builtin_interaction_gen
                    .write_interaction_trace(tree_builder, common_lookup_elements)
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

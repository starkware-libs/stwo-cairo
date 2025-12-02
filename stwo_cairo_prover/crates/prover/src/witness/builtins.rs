use cairo_air::air::CairoInteractionElements;
use cairo_air::builtins_air::{BuiltinsClaim, BuiltinsInteractionClaim};
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::builtins::BuiltinSegments;
use stwo_cairo_common::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};

use super::components::pedersen::PedersenContextClaimGenerator;
use super::components::poseidon::PoseidonContextClaimGenerator;
use crate::witness::components::{
    add_mod_builtin, bitwise_builtin, memory_address_to_id, memory_id_to_big, mul_mod_builtin,
    pedersen_builtin, poseidon_builtin, range_check_12, range_check_18, range_check_3_6_6_3,
    range_check_6, range_check_builtin_bits_128, range_check_builtin_bits_96, verify_bitwise_xor_8,
    verify_bitwise_xor_9,
};
use crate::witness::utils::TreeBuilder;
pub struct BuiltinsClaimGenerator {
    add_mod_builtin_trace_generator: Option<add_mod_builtin::ClaimGenerator>,
    bitwise_builtin_trace_generator: Option<bitwise_builtin::ClaimGenerator>,
    mul_mod_builtin_trace_generator: Option<mul_mod_builtin::ClaimGenerator>,
    pedersen_builtin_trace_generator: Option<pedersen_builtin::ClaimGenerator>,
    poseidon_builtin_trace_generator: Option<poseidon_builtin::ClaimGenerator>,
    range_check_96_builtin_trace_generator: Option<range_check_builtin_bits_96::ClaimGenerator>,
    range_check_128_builtin_trace_generator: Option<range_check_builtin_bits_128::ClaimGenerator>,
}
impl BuiltinsClaimGenerator {
    pub fn new(builtin_segments: BuiltinSegments) -> Self {
        let add_mod_builtin_trace_generator = builtin_segments.add_mod.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(ADD_MOD_MEMORY_CELLS),
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
                segment_length.is_multiple_of(BITWISE_MEMORY_CELLS),
                "bitwise segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / BITWISE_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "bitwise instances number is not a power of two"
            );
            bitwise_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let mul_mod_builtin_trace_generator = builtin_segments.mul_mod.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(MUL_MOD_MEMORY_CELLS),
                "mul mod segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / MUL_MOD_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "mul mod instances number is not a power of two"
            );
            mul_mod_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let pedersen_builtin_trace_generator = builtin_segments.pedersen.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(PEDERSEN_MEMORY_CELLS),
                "pedersen segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / PEDERSEN_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "pedersen instances number is not a power of two"
            );

            pedersen_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let poseidon_builtin_trace_generator = builtin_segments.poseidon.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(POSEIDON_MEMORY_CELLS),
                "poseidon segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / POSEIDON_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "poseidon instances number is not a power of two"
            );

            poseidon_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
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
            mul_mod_builtin_trace_generator,
            pedersen_builtin_trace_generator,
            poseidon_builtin_trace_generator,
            range_check_96_builtin_trace_generator,
            range_check_128_builtin_trace_generator,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        pedersen_context_trace_generator: &mut PedersenContextClaimGenerator,
        poseidon_context_trace_generator: &mut PoseidonContextClaimGenerator,
        range_check_6_trace_generator: &range_check_6::ClaimGenerator,
        range_check_12_trace_generator: &range_check_12::ClaimGenerator,
        range_check_18_trace_generator: &range_check_18::ClaimGenerator,
        range_check_3_6_6_3_trace_generator: &range_check_3_6_6_3::ClaimGenerator,
        verify_bitwise_xor_8_trace_generator: &verify_bitwise_xor_8::ClaimGenerator,
        verify_bitwise_xor_9_trace_generator: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (BuiltinsClaim, BuiltinsInteractionClaimGenerator) {
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
                    verify_bitwise_xor_8_trace_generator,
                )
            })
            .unzip();
        let (mul_mod_builtin_claim, mul_mod_builtin_interaction_gen) = self
            .mul_mod_builtin_trace_generator
            .map(|mul_mod_builtin_trace_generator| {
                mul_mod_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    memory_id_to_value_trace_generator,
                    range_check_12_trace_generator,
                    range_check_3_6_6_3_trace_generator,
                    range_check_18_trace_generator,
                )
            })
            .unzip();
        let (pedersen_builtin_claim, pedersen_builtin_interaction_gen) = self
            .pedersen_builtin_trace_generator
            .map(|pedersen_builtin_trace_generator| {
                pedersen_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    pedersen_context_trace_generator
                        .pedersen_aggregator_trace_generator
                        .as_ref()
                        .expect("If has Pedersen builtin, should have Pedersen aggregator"),
                )
            })
            .unzip();
        let (poseidon_builtin_claim, poseidon_builtin_interaction_gen) = self
            .poseidon_builtin_trace_generator
            .map(|poseidon_builtin_trace_generator| {
                poseidon_builtin_trace_generator.write_trace(
                    tree_builder,
                    memory_address_to_id_trace_generator,
                    &poseidon_context_trace_generator.poseidon_aggregator_trace_generator,
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
}

pub struct BuiltinsInteractionClaimGenerator {
    add_mod_builtin_interaction_gen: Option<add_mod_builtin::InteractionClaimGenerator>,
    bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
    mul_mod_builtin_interaction_gen: Option<mul_mod_builtin::InteractionClaimGenerator>,
    pedersen_builtin_interaction_gen: Option<pedersen_builtin::InteractionClaimGenerator>,
    poseidon_builtin_interaction_gen: Option<poseidon_builtin::InteractionClaimGenerator>,
    range_check_96_builtin_interaction_gen:
        Option<range_check_builtin_bits_96::InteractionClaimGenerator>,
    range_check_128_builtin_interaction_gen:
        Option<range_check_builtin_bits_128::InteractionClaimGenerator>,
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

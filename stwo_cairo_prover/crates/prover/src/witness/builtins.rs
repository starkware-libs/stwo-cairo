use cairo_air::builtins_air::BuiltinsClaim;
use stwo::core::fields::m31::BaseField;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
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

/// Type alias for trace evaluations.
pub type TraceEval = Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>;

/// Holds generated traces for all builtin components.
#[derive(Default)]
pub struct BuiltinsTraces {
    pub add_mod_builtin: Option<TraceEval>,
    pub bitwise_builtin: Option<TraceEval>,
    pub mul_mod_builtin: Option<TraceEval>,
    pub pedersen_builtin: Option<TraceEval>,
    pub poseidon_builtin: Option<TraceEval>,
    pub range_check_96_builtin: Option<TraceEval>,
    pub range_check_128_builtin: Option<TraceEval>,
}

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

/// Generates builtin traces without extending to tree_builder.
/// Returns the traces, claims, and interaction generators.
#[allow(clippy::too_many_arguments)]
pub fn builtins_generate_traces(
    add_mod_builtin: Option<add_mod_builtin::ClaimGenerator>,
    bitwise_builtin: Option<bitwise_builtin::ClaimGenerator>,
    mul_mod_builtin: Option<mul_mod_builtin::ClaimGenerator>,
    pedersen_builtin: Option<pedersen_builtin::ClaimGenerator>,
    poseidon_builtin: Option<poseidon_builtin::ClaimGenerator>,
    range_check_96_builtin: Option<range_check96_builtin::ClaimGenerator>,
    range_check_128_builtin: Option<range_check_builtin::ClaimGenerator>,
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
) -> (
    BuiltinsTraces,
    BuiltinsClaim,
    BuiltinsInteractionClaimGenerator,
) {
    let mut traces = BuiltinsTraces::default();

    let (add_mod_builtin_claim, add_mod_builtin_interaction_gen) = add_mod_builtin
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
            );
            traces.add_mod_builtin = Some(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (bitwise_builtin_claim, bitwise_builtin_interaction_gen) = bitwise_builtin
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                verify_bitwise_xor_9_trace_generator.unwrap(),
                verify_bitwise_xor_8_trace_generator.unwrap(),
            );
            traces.bitwise_builtin = Some(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (mul_mod_builtin_claim, mul_mod_builtin_interaction_gen) = mul_mod_builtin
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                memory_id_to_value_trace_generator.unwrap(),
                range_check_12_trace_generator.unwrap(),
                range_check_3_6_6_3_trace_generator.unwrap(),
                range_check_18_trace_generator.unwrap(),
            );
            traces.mul_mod_builtin = Some(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (pedersen_builtin_claim, pedersen_builtin_interaction_gen) = pedersen_builtin
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                pedersen_aggregator_trace_generator.unwrap(),
            );
            traces.pedersen_builtin = Some(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();
    let (poseidon_builtin_claim, poseidon_builtin_interaction_gen) = poseidon_builtin
        .map(|gen| {
            let (trace, claim, interaction_gen) = gen.write_trace(
                memory_address_to_id_trace_generator.unwrap(),
                poseidon_aggregator_trace_generator.unwrap(),
            );
            traces.poseidon_builtin = Some(trace.to_evals());
            (claim, interaction_gen)
        })
        .unzip();

    let (range_check_96_builtin_claim, range_check_96_builtin_interaction_gen) =
        range_check_96_builtin
            .map(|gen| {
                let (trace, claim, interaction_gen) = gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                    range_check_6_trace_generator.unwrap(),
                );
                traces.range_check_96_builtin = Some(trace.to_evals());
                (claim, interaction_gen)
            })
            .unzip();
    let (range_check_128_builtin_claim, range_check_128_builtin_interaction_gen) =
        range_check_128_builtin
            .map(|gen| {
                let (trace, claim, interaction_gen) = gen.write_trace(
                    memory_address_to_id_trace_generator.unwrap(),
                    memory_id_to_value_trace_generator.unwrap(),
                );
                traces.range_check_128_builtin = Some(trace.to_evals());
                (claim, interaction_gen)
            })
            .unzip();

    let claim = BuiltinsClaim {
        add_mod_builtin: add_mod_builtin_claim,
        bitwise_builtin: bitwise_builtin_claim,
        mul_mod_builtin: mul_mod_builtin_claim,
        pedersen_builtin: pedersen_builtin_claim,
        poseidon_builtin: poseidon_builtin_claim,
        range_check_96_builtin: range_check_96_builtin_claim,
        range_check_128_builtin: range_check_128_builtin_claim,
    };

    let interaction_gen = BuiltinsInteractionClaimGenerator {
        add_mod_builtin_interaction_gen,
        bitwise_builtin_interaction_gen,
        mul_mod_builtin_interaction_gen,
        pedersen_builtin_interaction_gen,
        poseidon_builtin_interaction_gen,
        range_check_96_builtin_interaction_gen,
        range_check_128_builtin_interaction_gen,
    };

    (traces, claim, interaction_gen)
}

/// Extends the tree builder with builtin traces in the correct order.
pub fn extend_builtins_traces(tree_builder: &mut impl TreeBuilder<SimdBackend>, traces: BuiltinsTraces) {
    if let Some(trace) = traces.add_mod_builtin {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.bitwise_builtin {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.mul_mod_builtin {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.pedersen_builtin {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.poseidon_builtin {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.range_check_96_builtin {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.range_check_128_builtin {
        tree_builder.extend_evals(trace);
    }
}

/// Legacy function that generates builtin traces and extends them to tree_builder.
/// Maintained for backward compatibility.
#[allow(clippy::too_many_arguments)]
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
    let (traces, claim, interaction_gen) = builtins_generate_traces(
        add_mod_builtin,
        bitwise_builtin,
        mul_mod_builtin,
        pedersen_builtin,
        poseidon_builtin,
        range_check_96_builtin,
        range_check_128_builtin,
        memory_address_to_id_trace_generator,
        memory_id_to_value_trace_generator,
        pedersen_aggregator_trace_generator,
        poseidon_aggregator_trace_generator,
        range_check_6_trace_generator,
        range_check_12_trace_generator,
        range_check_18_trace_generator,
        range_check_3_6_6_3_trace_generator,
        verify_bitwise_xor_8_trace_generator,
        verify_bitwise_xor_9_trace_generator,
    );
    extend_builtins_traces(tree_builder, traces);
    (claim, interaction_gen)
}

pub struct BuiltinsInteractionClaimGenerator {
    pub add_mod_builtin_interaction_gen: Option<add_mod_builtin::InteractionClaimGenerator>,
    pub bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
    pub mul_mod_builtin_interaction_gen: Option<mul_mod_builtin::InteractionClaimGenerator>,
    pub pedersen_builtin_interaction_gen: Option<pedersen_builtin::InteractionClaimGenerator>,
    pub poseidon_builtin_interaction_gen: Option<poseidon_builtin::InteractionClaimGenerator>,
    pub range_check_96_builtin_interaction_gen:
        Option<range_check96_builtin::InteractionClaimGenerator>,
    pub range_check_128_builtin_interaction_gen:
        Option<range_check_builtin::InteractionClaimGenerator>,
}

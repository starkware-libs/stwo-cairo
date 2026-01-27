use cairo_air::pedersen::air::{Claim, PedersenContextClaim};
use stwo::core::fields::m31::BaseField;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use tracing::{span, Level};

use crate::witness::components::{
    memory_id_to_big, partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18, range_check_20, range_check_8, range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

/// Type alias for trace evaluations.
pub type TraceEval = Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>;

/// Holds generated traces for pedersen context components.
#[derive(Default)]
pub struct PedersenContextTraces {
    pub pedersen_aggregator: Option<TraceEval>,
    pub partial_ec_mul: Vec<TraceEval>,
    pub pedersen_points_table: Option<TraceEval>,
}

/// Generates pedersen context traces without extending to tree_builder.
/// Returns the traces, claims, and interaction generators.
/// Note: This function still takes tree_builder because partial_ec_mul.write_trace
/// requires it internally. The traces returned can be used for extending later.
pub fn pedersen_context_generate_traces(
    pedersen_aggregator_trace_generator: Option<pedersen_aggregator_window_bits_18::ClaimGenerator>,
    partial_ec_mul_trace_generator: Option<partial_ec_mul_window_bits_18::ClaimGenerator>,
    pedersen_points_table_trace_generator: Option<
        pedersen_points_table_window_bits_18::ClaimGenerator,
    >,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_id_to_big_state: Option<&memory_id_to_big::ClaimGenerator>,
    rc_8_trace_generator: Option<&range_check_8::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
) -> (
    PedersenContextTraces,
    PedersenContextClaim,
    PedersenContextInteractionClaimGenerator,
) {
    let span = span!(Level::INFO, "write pedersen context trace").entered();
    if pedersen_aggregator_trace_generator
        .as_ref()
        .is_none_or(|tg| tg.is_empty())
    {
        return (
            PedersenContextTraces::default(),
            PedersenContextClaim { claim: None },
            PedersenContextInteractionClaimGenerator { gen: None },
        );
    }

    let pedersen_aggregator_trace_generator = pedersen_aggregator_trace_generator
        .expect("Should have pedersen aggregator trace generator at this point");
    let mut partial_ec_mul_trace_generator = partial_ec_mul_trace_generator
        .expect("Should have partial EC mul trace generator at this point");
    let pedersen_points_table_trace_generator = pedersen_points_table_trace_generator
        .expect("Should have pedersen points table trace generator at this point");

    let (pedersen_aggregator_trace, pedersen_aggregator_claim, pedersen_aggregator_interaction_gen) =
        pedersen_aggregator_trace_generator.write_trace(
            memory_id_to_big_state.unwrap(),
            rc_8_trace_generator.unwrap(),
            &mut partial_ec_mul_trace_generator,
        );
    // Note: partial_ec_mul.write_trace extends to tree_builder internally
    let (partial_ec_mul_claim, partial_ec_mul_interaction_gen) = partial_ec_mul_trace_generator
        .write_trace(
            tree_builder,
            &pedersen_points_table_trace_generator,
            rc_9_9_trace_generator.unwrap(),
            rc_20_trace_generator.unwrap(),
        );
    let (
        pedersen_points_table_trace,
        pedersen_points_table_claim,
        pedersen_points_table_interaction_gen,
    ) = pedersen_points_table_trace_generator.write_trace();
    span.exit();

    let traces = PedersenContextTraces {
        pedersen_aggregator: Some(pedersen_aggregator_trace.to_evals()),
        partial_ec_mul: vec![], // partial_ec_mul extends to tree_builder internally
        pedersen_points_table: Some(pedersen_points_table_trace.to_evals()),
    };

    let claim = Some(Claim {
        pedersen_aggregator: pedersen_aggregator_claim,
        partial_ec_mul: partial_ec_mul_claim,
        pedersen_points_table: pedersen_points_table_claim,
    });
    let gen = Some(PedersenInteractionClaimGenerator {
        pedersen_aggregator_interaction_gen,
        partial_ec_mul_interaction_gen,
        pedersen_points_table_interaction_gen,
    });
    (
        traces,
        PedersenContextClaim { claim },
        PedersenContextInteractionClaimGenerator { gen },
    )
}

/// Extends the tree builder with pedersen context traces.
/// Note: partial_ec_mul traces are already extended in generate_traces.
pub fn extend_pedersen_context_traces(
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    traces: PedersenContextTraces,
) {
    if let Some(trace) = traces.pedersen_aggregator {
        tree_builder.extend_evals(trace);
    }
    // partial_ec_mul traces are extended internally during generation
    for trace in traces.partial_ec_mul {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.pedersen_points_table {
        tree_builder.extend_evals(trace);
    }
}

/// Legacy function that generates pedersen context traces and extends them to tree_builder.
/// Maintained for backward compatibility.
pub fn pedersen_context_write_trace(
    pedersen_aggregator_trace_generator: Option<pedersen_aggregator_window_bits_18::ClaimGenerator>,
    partial_ec_mul_trace_generator: Option<partial_ec_mul_window_bits_18::ClaimGenerator>,
    pedersen_points_table_trace_generator: Option<
        pedersen_points_table_window_bits_18::ClaimGenerator,
    >,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_id_to_big_state: Option<&memory_id_to_big::ClaimGenerator>,
    rc_8_trace_generator: Option<&range_check_8::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
) -> (
    PedersenContextClaim,
    PedersenContextInteractionClaimGenerator,
) {
    let (traces, claim, interaction_gen) = pedersen_context_generate_traces(
        pedersen_aggregator_trace_generator,
        partial_ec_mul_trace_generator,
        pedersen_points_table_trace_generator,
        tree_builder,
        memory_id_to_big_state,
        rc_8_trace_generator,
        rc_9_9_trace_generator,
        rc_20_trace_generator,
    );
    extend_pedersen_context_traces(tree_builder, traces);
    (claim, interaction_gen)
}

pub struct PedersenContextInteractionClaimGenerator {
    pub gen: Option<PedersenInteractionClaimGenerator>,
}
pub struct PedersenInteractionClaimGenerator {
    pub pedersen_aggregator_interaction_gen:
        pedersen_aggregator_window_bits_18::InteractionClaimGenerator,
    pub partial_ec_mul_interaction_gen: partial_ec_mul_window_bits_18::InteractionClaimGenerator,
    pub pedersen_points_table_interaction_gen:
        pedersen_points_table_window_bits_18::InteractionClaimGenerator,
}

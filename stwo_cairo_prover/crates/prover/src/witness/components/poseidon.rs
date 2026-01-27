use cairo_air::poseidon::air::{Claim, PoseidonContextClaim};
use stwo::core::fields::m31::BaseField;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use tracing::{span, Level};

use crate::witness::components::{
    cube_252, memory_id_to_big, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_18, range_check_20,
    range_check_252_width_27, range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
    range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

/// Type alias for trace evaluations.
pub type TraceEval = Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>;

/// Holds generated traces for poseidon context components.
#[derive(Default)]
pub struct PoseidonContextTraces {
    pub poseidon_aggregator: Option<TraceEval>,
    pub poseidon_3_partial_rounds_chain: Option<TraceEval>,
    pub poseidon_full_round_chain: Option<TraceEval>,
    pub cube_252: Vec<TraceEval>,
    pub poseidon_round_keys: Option<TraceEval>,
    pub range_check_252_width_27: Option<TraceEval>,
}

/// Generates poseidon context traces without extending to tree_builder.
/// Returns the traces, claims, and interaction generators.
/// Note: This function still takes tree_builder because cube_252.write_trace
/// requires it internally. The traces returned can be used for extending later.
#[allow(clippy::too_many_arguments)]
pub fn poseidon_context_generate_traces(
    poseidon_aggregator_trace_generator: Option<poseidon_aggregator::ClaimGenerator>,
    poseidon_3_partial_rounds_chain_trace_generator: Option<
        poseidon_3_partial_rounds_chain::ClaimGenerator,
    >,
    poseidon_full_round_chain_trace_generator: Option<poseidon_full_round_chain::ClaimGenerator>,
    cube_252_trace_generator: Option<cube_252::ClaimGenerator>,
    poseidon_round_keys_trace_generator: Option<poseidon_round_keys::ClaimGenerator>,
    range_check_252_width_27_trace_generator: Option<range_check_252_width_27::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_id_to_big_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_3_3_3_3_3_trace_generator: Option<&range_check_3_3_3_3_3::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    rc_4_4_trace_generator: Option<&range_check_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
) -> (
    PoseidonContextTraces,
    PoseidonContextClaim,
    PoseidonContextInteractionClaimGenerator,
) {
    let span = span!(Level::INFO, "write poseidon context trace").entered();
    if poseidon_aggregator_trace_generator.is_none() {
        return (
            PoseidonContextTraces::default(),
            PoseidonContextClaim { claim: None },
            PoseidonContextInteractionClaimGenerator { gen: None },
        );
    }
    let poseidon_aggregator_trace_generator = poseidon_aggregator_trace_generator
        .expect("Should have poseidon aggregator trace generator at this point");
    let mut poseidon_3_partial_rounds_chain_trace_generator =
        poseidon_3_partial_rounds_chain_trace_generator
            .expect("Should have poseidon 3 partial rounds chain trace generator at this point");
    let mut poseidon_full_round_chain_trace_generator = poseidon_full_round_chain_trace_generator
        .expect("Should have poseidon full round chain trace generator at this point");
    let mut cube_252_trace_generator =
        cube_252_trace_generator.expect("Should have cube 252 trace generator at this point");
    let poseidon_round_keys_trace_generator = poseidon_round_keys_trace_generator
        .expect("Should have poseidon round keys trace generator at this point");
    let mut range_check_252_width_27_trace_generator = range_check_252_width_27_trace_generator
        .expect("Should have range check 252 width 27 trace generator at this point");

    let (poseidon_aggregator_trace, poseidon_aggregator_claim, poseidon_aggregator_interaction_gen) =
        poseidon_aggregator_trace_generator.write_trace(
            memory_id_to_big_trace_generator.unwrap(),
            &mut poseidon_full_round_chain_trace_generator,
            &mut range_check_252_width_27_trace_generator,
            &mut cube_252_trace_generator,
            rc_3_3_3_3_3_trace_generator.unwrap(),
            rc_4_4_4_4_trace_generator.unwrap(),
            rc_4_4_trace_generator.unwrap(),
            &mut poseidon_3_partial_rounds_chain_trace_generator,
        );
    let (
        poseidon_3_partial_rounds_chain_trace,
        poseidon_3_partial_rounds_chain_claim,
        poseidon_3_partial_rounds_chain_interaction_gen,
    ) = poseidon_3_partial_rounds_chain_trace_generator.write_trace(
        &poseidon_round_keys_trace_generator,
        &mut cube_252_trace_generator,
        rc_4_4_4_4_trace_generator.unwrap(),
        rc_4_4_trace_generator.unwrap(),
        &mut range_check_252_width_27_trace_generator,
    );
    let (
        poseidon_full_round_chain_trace,
        poseidon_full_round_chain_claim,
        poseidon_full_round_chain_interaction_gen,
    ) = poseidon_full_round_chain_trace_generator.write_trace(
        &mut cube_252_trace_generator,
        &poseidon_round_keys_trace_generator,
        rc_3_3_3_3_3_trace_generator.unwrap(),
    );
    // Note: cube_252.write_trace extends to tree_builder internally
    let (cube_252_claim, cube_252_interaction_gen) = cube_252_trace_generator.write_trace(
        tree_builder,
        rc_9_9_trace_generator.unwrap(),
        rc_20_trace_generator.unwrap(),
    );
    let (poseidon_round_keys_trace, poseidon_round_keys_claim, poseidon_round_keys_interaction_gen) =
        poseidon_round_keys_trace_generator.write_trace();
    let (
        range_check_felt_252_width_27_trace,
        range_check_felt_252_width_27_claim,
        range_check_felt_252_width_27_interaction_gen,
    ) = range_check_252_width_27_trace_generator.write_trace(
        rc_9_9_trace_generator.unwrap(),
        rc_18_trace_generator.unwrap(),
    );
    span.exit();

    let traces = PoseidonContextTraces {
        poseidon_aggregator: Some(poseidon_aggregator_trace.to_evals()),
        poseidon_3_partial_rounds_chain: Some(poseidon_3_partial_rounds_chain_trace.to_evals()),
        poseidon_full_round_chain: Some(poseidon_full_round_chain_trace.to_evals()),
        cube_252: vec![], // cube_252 extends to tree_builder internally
        poseidon_round_keys: Some(poseidon_round_keys_trace.to_evals()),
        range_check_252_width_27: Some(range_check_felt_252_width_27_trace.to_evals()),
    };

    let claim = Some(Claim {
        poseidon_aggregator: poseidon_aggregator_claim,
        poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim,
        poseidon_full_round_chain: poseidon_full_round_chain_claim,
        cube_252: cube_252_claim,
        poseidon_round_keys: poseidon_round_keys_claim,
        range_check_252_width_27: range_check_felt_252_width_27_claim,
    });
    let gen = Some(PoseidonInteractionClaimGenerator {
        poseidon_aggregator_interaction_gen,
        poseidon_3_partial_rounds_chain_interaction_gen,
        poseidon_full_round_chain_interaction_gen,
        cube_252_interaction_gen,
        poseidon_round_keys_interaction_gen,
        range_check_felt_252_width_27_interaction_gen,
    });
    (
        traces,
        PoseidonContextClaim { claim },
        PoseidonContextInteractionClaimGenerator { gen },
    )
}

/// Extends the tree builder with poseidon context traces.
/// Note: cube_252 traces are already extended in generate_traces.
pub fn extend_poseidon_context_traces(
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    traces: PoseidonContextTraces,
) {
    if let Some(trace) = traces.poseidon_aggregator {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.poseidon_3_partial_rounds_chain {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.poseidon_full_round_chain {
        tree_builder.extend_evals(trace);
    }
    // cube_252 traces are extended internally during generation
    for trace in traces.cube_252 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.poseidon_round_keys {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.range_check_252_width_27 {
        tree_builder.extend_evals(trace);
    }
}

/// Legacy function that generates poseidon context traces and extends them to tree_builder.
/// Maintained for backward compatibility.
#[allow(clippy::too_many_arguments)]
pub fn poseidon_context_write_trace(
    poseidon_aggregator_trace_generator: Option<poseidon_aggregator::ClaimGenerator>,
    poseidon_3_partial_rounds_chain_trace_generator: Option<
        poseidon_3_partial_rounds_chain::ClaimGenerator,
    >,
    poseidon_full_round_chain_trace_generator: Option<poseidon_full_round_chain::ClaimGenerator>,
    cube_252_trace_generator: Option<cube_252::ClaimGenerator>,
    poseidon_round_keys_trace_generator: Option<poseidon_round_keys::ClaimGenerator>,
    range_check_252_width_27_trace_generator: Option<range_check_252_width_27::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_id_to_big_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_3_3_3_3_3_trace_generator: Option<&range_check_3_3_3_3_3::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    rc_4_4_trace_generator: Option<&range_check_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
) -> (
    PoseidonContextClaim,
    PoseidonContextInteractionClaimGenerator,
) {
    let (traces, claim, interaction_gen) = poseidon_context_generate_traces(
        poseidon_aggregator_trace_generator,
        poseidon_3_partial_rounds_chain_trace_generator,
        poseidon_full_round_chain_trace_generator,
        cube_252_trace_generator,
        poseidon_round_keys_trace_generator,
        range_check_252_width_27_trace_generator,
        tree_builder,
        memory_id_to_big_trace_generator,
        rc_3_3_3_3_3_trace_generator,
        rc_4_4_4_4_trace_generator,
        rc_4_4_trace_generator,
        rc_9_9_trace_generator,
        rc_18_trace_generator,
        rc_20_trace_generator,
    );
    extend_poseidon_context_traces(tree_builder, traces);
    (claim, interaction_gen)
}

pub struct PoseidonContextInteractionClaimGenerator {
    pub gen: Option<PoseidonInteractionClaimGenerator>,
}

pub struct PoseidonInteractionClaimGenerator {
    pub poseidon_aggregator_interaction_gen: poseidon_aggregator::InteractionClaimGenerator,
    pub poseidon_3_partial_rounds_chain_interaction_gen:
        poseidon_3_partial_rounds_chain::InteractionClaimGenerator,
    pub poseidon_full_round_chain_interaction_gen:
        poseidon_full_round_chain::InteractionClaimGenerator,
    pub cube_252_interaction_gen: cube_252::InteractionClaimGenerator,
    pub poseidon_round_keys_interaction_gen: poseidon_round_keys::InteractionClaimGenerator,
    pub range_check_felt_252_width_27_interaction_gen:
        range_check_252_width_27::InteractionClaimGenerator,
}

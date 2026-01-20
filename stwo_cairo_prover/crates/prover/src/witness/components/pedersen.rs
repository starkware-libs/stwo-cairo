use cairo_air::components::{
    partial_ec_mul_window_bits_18 as partial_ec_mul_window_bits_18_claim,
    pedersen_aggregator_window_bits_18 as pedersen_aggregator_window_bits_18_claim,
    pedersen_points_table_window_bits_18 as pedersen_points_table_window_bits_18_claim,
};
use stwo::prover::backend::simd::SimdBackend;
use tracing::{span, Level};

use crate::witness::components::{
    memory_id_to_big, partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18, range_check_20, range_check_8, range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

#[allow(clippy::type_complexity)]
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
    Option<pedersen_aggregator_window_bits_18_claim::Claim>,
    Option<partial_ec_mul_window_bits_18_claim::Claim>,
    Option<pedersen_points_table_window_bits_18_claim::Claim>,
    PedersenContextInteractionClaimGenerator,
) {
    let span = span!(Level::INFO, "write pedersen context trace").entered();
    if pedersen_aggregator_trace_generator
        .as_ref()
        .is_none_or(|tg| tg.is_empty())
    {
        return (
            None,
            None,
            None,
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
    tree_builder.extend_evals(pedersen_aggregator_trace.to_evals());
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
    tree_builder.extend_evals(pedersen_points_table_trace.to_evals());
    span.exit();

    let gen = Some(PedersenInteractionClaimGenerator {
        pedersen_aggregator_interaction_gen,
        partial_ec_mul_interaction_gen,
        pedersen_points_table_interaction_gen,
    });

    (
        Some(pedersen_aggregator_claim),
        Some(partial_ec_mul_claim),
        Some(pedersen_points_table_claim),
        PedersenContextInteractionClaimGenerator { gen },
    )
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

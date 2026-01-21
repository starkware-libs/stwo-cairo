use cairo_air::pedersen::air::{
    Claim, InteractionClaim, PedersenContextClaim, PedersenContextInteractionClaim,
};
use cairo_air::relations::CommonLookupElements;
use stwo::core::fields::m31::M31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use tracing::{span, Level};

use crate::witness::components::{
    memory_id_to_big, partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18, range_check_20, range_check_8, range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

type InteractionTraces = Vec<Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>>;

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
    let span = span!(Level::INFO, "write pedersen context trace").entered();
    if pedersen_aggregator_trace_generator
        .as_ref()
        .is_none_or(|tg| tg.is_empty())
    {
        return (
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

    let claim = Some(Claim {
        pedersen_aggregator: pedersen_aggregator_claim,
        partial_ec_mul: partial_ec_mul_claim,
        pedersen_points_table: pedersen_points_table_claim,
    });
    let gen = Some(InteractionClaimGenerator {
        pedersen_aggregator_interaction_gen,
        partial_ec_mul_interaction_gen,
        pedersen_points_table_interaction_gen,
    });
    (
        PedersenContextClaim { claim },
        PedersenContextInteractionClaimGenerator { gen },
    )
}

pub struct PedersenContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl PedersenContextInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        common_lookup_elements: &CommonLookupElements,
    ) -> (InteractionTraces, PedersenContextInteractionClaim) {
        let (traces, claim) = self
            .gen
            .map(|gen| {
                let (traces, claim) = gen.write_interaction_trace(common_lookup_elements);
                (traces, Some(claim))
            })
            .unwrap_or_default();
        (traces, PedersenContextInteractionClaim { claim })
    }
}

struct InteractionClaimGenerator {
    pedersen_aggregator_interaction_gen:
        pedersen_aggregator_window_bits_18::InteractionClaimGenerator,
    partial_ec_mul_interaction_gen: partial_ec_mul_window_bits_18::InteractionClaimGenerator,
    pedersen_points_table_interaction_gen:
        pedersen_points_table_window_bits_18::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        common_lookup_elements: &CommonLookupElements,
    ) -> (InteractionTraces, InteractionClaim) {
        let mut all_traces = Vec::new();
        let (pedersen_aggregator_trace, pedersen_aggregator_interaction_claim) = self
            .pedersen_aggregator_interaction_gen
            .write_interaction_trace(common_lookup_elements);
        all_traces.push(pedersen_aggregator_trace);
        let (partial_ec_mul_trace, partial_ec_mul_interaction_claim) = self
            .partial_ec_mul_interaction_gen
            .write_interaction_trace(common_lookup_elements);
        all_traces.push(partial_ec_mul_trace);
        let (pedersen_points_table_trace, pedersen_points_table_interaction_claim) = self
            .pedersen_points_table_interaction_gen
            .write_interaction_trace(common_lookup_elements);
        all_traces.push(pedersen_points_table_trace);

        (
            all_traces,
            InteractionClaim {
                pedersen_aggregator: pedersen_aggregator_interaction_claim,
                partial_ec_mul: partial_ec_mul_interaction_claim,
                pedersen_points_table: pedersen_points_table_interaction_claim,
            },
        )
    }
}

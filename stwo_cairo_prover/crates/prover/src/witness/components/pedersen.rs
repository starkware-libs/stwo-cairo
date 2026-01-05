use cairo_air::pedersen::air::{
    Claim, InteractionClaim, PedersenContextClaim, PedersenContextInteractionClaim,
};
use cairo_air::relations::CommonLookupElements;
use tracing::{span, Level};

use crate::witness::components::{
    memory_id_to_big, partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18,
};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct PedersenContextClaimGenerator {
    pub pedersen_aggregator_trace_generator:
        Option<pedersen_aggregator_window_bits_18::ClaimGenerator>,
    pub partial_ec_mul_trace_generator: Option<partial_ec_mul_window_bits_18::ClaimGenerator>,
    pub pedersen_points_table_trace_generator:
        Option<pedersen_points_table_window_bits_18::ClaimGenerator>,
}

impl PedersenContextClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        if !preprocessed_trace.has_column(&PreProcessedColumnId {
            id: "pedersen_points_0".to_owned(),
        }) {
            // This is a preprocessed trace without the Pedersen points table - Don't
            // create Pedersen components.
            return Self {
                pedersen_aggregator_trace_generator: None,
                partial_ec_mul_trace_generator: None,
                pedersen_points_table_trace_generator: None,
            };
        }
        let pedersen_aggregator_trace_generator =
            pedersen_aggregator_window_bits_18::ClaimGenerator::new();
        let partial_ec_mul_trace_generator = partial_ec_mul_window_bits_18::ClaimGenerator::new();
        let pedersen_points_table_trace_generator =
            pedersen_points_table_window_bits_18::ClaimGenerator::new(preprocessed_trace);

        Self {
            pedersen_aggregator_trace_generator: Some(pedersen_aggregator_trace_generator),
            partial_ec_mul_trace_generator: Some(partial_ec_mul_trace_generator),
            pedersen_points_table_trace_generator: Some(pedersen_points_table_trace_generator),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
    ) -> (
        PedersenContextClaim,
        PedersenContextInteractionClaimGenerator,
    ) {
        let span = span!(Level::INFO, "write pedersen context trace").entered();
        if self
            .pedersen_aggregator_trace_generator
            .as_ref()
            .is_none_or(|tg| tg.is_empty())
        {
            return (
                PedersenContextClaim { claim: None },
                PedersenContextInteractionClaimGenerator { gen: None },
            );
        }
        let Self {
            pedersen_aggregator_trace_generator,
            partial_ec_mul_trace_generator,
            pedersen_points_table_trace_generator,
        } = self;
        let pedersen_aggregator_trace_generator = pedersen_aggregator_trace_generator
            .expect("Should have pedersen context components at this point");
        let mut partial_ec_mul_trace_generator = partial_ec_mul_trace_generator
            .expect("Should have pedersen context components at this point");
        let pedersen_points_table_trace_generator = pedersen_points_table_trace_generator
            .expect("Should have pedersen context components at this point");
        let (pedersen_aggregator_claim, pedersen_aggregator_interaction_gen) =
            pedersen_aggregator_trace_generator.write_trace(
                tree_builder,
                memory_id_to_big_state,
                &range_checks_trace_generator.rc_8_trace_generator,
                &mut partial_ec_mul_trace_generator,
            );
        let (partial_ec_mul_claim, partial_ec_mul_interaction_gen) = partial_ec_mul_trace_generator
            .write_trace(
                tree_builder,
                &pedersen_points_table_trace_generator,
                &range_checks_trace_generator.rc_9_9_trace_generator,
                &range_checks_trace_generator.rc_20_trace_generator,
            );
        let (pedersen_points_table_claim, pedersen_points_table_interaction_gen) =
            pedersen_points_table_trace_generator.write_trace(tree_builder);
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
}

pub struct PedersenContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl PedersenContextInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> PedersenContextInteractionClaim {
        PedersenContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, common_lookup_elements)),
        }
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
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> InteractionClaim {
        let pedersen_aggregator_interaction_claim = self
            .pedersen_aggregator_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let partial_ec_mul_interaction_claim = self
            .partial_ec_mul_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let pedersen_points_table_interaction_claim = self
            .pedersen_points_table_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);

        InteractionClaim {
            pedersen_aggregator: pedersen_aggregator_interaction_claim,
            partial_ec_mul: partial_ec_mul_interaction_claim,
            pedersen_points_table: pedersen_points_table_interaction_claim,
        }
    }
}

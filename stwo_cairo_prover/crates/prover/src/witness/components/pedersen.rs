use cairo_air::air::CairoInteractionElements;
use cairo_air::pedersen::air::{
    Claim, InteractionClaim, PedersenContextClaim, PedersenContextInteractionClaim,
};
use tracing::{span, Level};

use crate::witness::components::{
    memory_id_to_big, partial_ec_mul, pedersen_aggregator, pedersen_points_table,
};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct PedersenContextClaimGenerator {
    pub pedersen_aggregator_trace_generator: Option<pedersen_aggregator::ClaimGenerator>,
    pub partial_ec_mul_trace_generator: Option<partial_ec_mul::ClaimGenerator>,
    pub pedersen_points_table_trace_generator: Option<pedersen_points_table::ClaimGenerator>,
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
        let pedersen_aggregator_trace_generator = pedersen_aggregator::ClaimGenerator::new();
        let partial_ec_mul_trace_generator = partial_ec_mul::ClaimGenerator::new();
        let pedersen_points_table_trace_generator =
            pedersen_points_table::ClaimGenerator::new(preprocessed_trace);

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
                &range_checks_trace_generator.rc_5_4_trace_generator,
                &range_checks_trace_generator.rc_8_trace_generator,
                &pedersen_points_table_trace_generator,
                &mut partial_ec_mul_trace_generator,
            );
        let (partial_ec_mul_claim, partial_ec_mul_interaction_gen) = partial_ec_mul_trace_generator
            .write_trace(
                tree_builder,
                &pedersen_points_table_trace_generator,
                &range_checks_trace_generator.rc_9_9_trace_generator,
                &range_checks_trace_generator.rc_9_9_b_trace_generator,
                &range_checks_trace_generator.rc_9_9_c_trace_generator,
                &range_checks_trace_generator.rc_9_9_d_trace_generator,
                &range_checks_trace_generator.rc_9_9_e_trace_generator,
                &range_checks_trace_generator.rc_9_9_f_trace_generator,
                &range_checks_trace_generator.rc_9_9_g_trace_generator,
                &range_checks_trace_generator.rc_9_9_h_trace_generator,
                &range_checks_trace_generator.rc_20_trace_generator,
                &range_checks_trace_generator.rc_20_b_trace_generator,
                &range_checks_trace_generator.rc_20_c_trace_generator,
                &range_checks_trace_generator.rc_20_d_trace_generator,
                &range_checks_trace_generator.rc_20_e_trace_generator,
                &range_checks_trace_generator.rc_20_f_trace_generator,
                &range_checks_trace_generator.rc_20_g_trace_generator,
                &range_checks_trace_generator.rc_20_h_trace_generator,
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
        interaction_elements: &CairoInteractionElements,
    ) -> PedersenContextInteractionClaim {
        PedersenContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, interaction_elements)),
        }
    }
}

struct InteractionClaimGenerator {
    pedersen_aggregator_interaction_gen: pedersen_aggregator::InteractionClaimGenerator,
    partial_ec_mul_interaction_gen: partial_ec_mul::InteractionClaimGenerator,
    pedersen_points_table_interaction_gen: pedersen_points_table::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim {
        let pedersen_aggregator_interaction_claim = self
            .pedersen_aggregator_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.range_checks.rc_5_4,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_8,
                &interaction_elements.pedersen_points_table,
                &interaction_elements.partial_ec_mul,
                &interaction_elements.pedersen_aggregator,
            );
        let partial_ec_mul_interaction_claim =
            self.partial_ec_mul_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.pedersen_points_table,
                &interaction_elements.range_checks.rc_9_9,
                &interaction_elements.range_checks.rc_9_9_b,
                &interaction_elements.range_checks.rc_9_9_c,
                &interaction_elements.range_checks.rc_9_9_d,
                &interaction_elements.range_checks.rc_9_9_e,
                &interaction_elements.range_checks.rc_9_9_f,
                &interaction_elements.range_checks.rc_9_9_g,
                &interaction_elements.range_checks.rc_9_9_h,
                &interaction_elements.range_checks.rc_20,
                &interaction_elements.range_checks.rc_20_b,
                &interaction_elements.range_checks.rc_20_c,
                &interaction_elements.range_checks.rc_20_d,
                &interaction_elements.range_checks.rc_20_e,
                &interaction_elements.range_checks.rc_20_f,
                &interaction_elements.range_checks.rc_20_g,
                &interaction_elements.range_checks.rc_20_h,
                &interaction_elements.partial_ec_mul,
            );
        let pedersen_points_table_interaction_claim = self
            .pedersen_points_table_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.pedersen_points_table);

        InteractionClaim {
            pedersen_aggregator: pedersen_aggregator_interaction_claim,
            partial_ec_mul: partial_ec_mul_interaction_claim,
            pedersen_points_table: pedersen_points_table_interaction_claim,
        }
    }
}

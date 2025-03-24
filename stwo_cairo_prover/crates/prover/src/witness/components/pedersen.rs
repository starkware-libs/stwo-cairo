use cairo_air::air::CairoInteractionElements;
use cairo_air::pedersen::air::{
    Claim, InteractionClaim, PedersenContextClaim, PedersenContextInteractionClaim,
};
use tracing::{span, Level};

use crate::witness::components::{partial_ec_mul, pedersen_points_table};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct PedersenContextClaimGenerator {
    pub partial_ec_mul_trace_generator: partial_ec_mul::ClaimGenerator,
    pub pedersen_points_table_trace_generator: pedersen_points_table::ClaimGenerator,
}
impl Default for PedersenContextClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PedersenContextClaimGenerator {
    pub fn new() -> Self {
        let partial_ec_mul_trace_generator = partial_ec_mul::ClaimGenerator::new();
        let pedersen_points_table_trace_generator = pedersen_points_table::ClaimGenerator::new();

        Self {
            partial_ec_mul_trace_generator,
            pedersen_points_table_trace_generator,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
    ) -> (
        PedersenContextClaim,
        PedersenContextInteractionClaimGenerator,
    ) {
        let span = span!(Level::INFO, "write pedersen context trace").entered();
        if self.partial_ec_mul_trace_generator.is_empty() {
            return (
                PedersenContextClaim { claim: None },
                PedersenContextInteractionClaimGenerator { gen: None },
            );
        }
        let (partial_ec_mul_claim, partial_ec_mul_interaction_gen) =
            self.partial_ec_mul_trace_generator.write_trace(
                tree_builder,
                &self.pedersen_points_table_trace_generator,
                &range_checks_trace_generator.rc_19_trace_generator,
                &range_checks_trace_generator.rc_9_9_trace_generator,
            );
        let (pedersen_points_table_claim, pedersen_points_table_interaction_gen) = self
            .pedersen_points_table_trace_generator
            .write_trace(tree_builder);
        span.exit();

        let claim = Some(Claim {
            partial_ec_mul: partial_ec_mul_claim,
            pedersen_points_table: pedersen_points_table_claim,
        });
        let gen = Some(InteractionClaimGenerator {
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
    partial_ec_mul_interaction_gen: partial_ec_mul::InteractionClaimGenerator,
    pedersen_points_table_interaction_gen: pedersen_points_table::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim {
        let partial_ec_mul_interaction_claim =
            self.partial_ec_mul_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.partial_ec_mul,
                &interaction_elements.pedersen_points_table,
                &interaction_elements.range_checks.rc_19,
                &interaction_elements.range_checks.rc_9_9,
            );
        let pedersen_points_table_interaction_claim = self
            .pedersen_points_table_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.pedersen_points_table);

        InteractionClaim {
            partial_ec_mul: partial_ec_mul_interaction_claim,
            pedersen_points_table: pedersen_points_table_interaction_claim,
        }
    }
}

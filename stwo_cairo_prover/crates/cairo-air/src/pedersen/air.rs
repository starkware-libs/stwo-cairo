use stwo::core::air::Component;
use stwo::core::pcs::TreeVec;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    indented_component_display, partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18,
};
use crate::relations::CommonLookupElements;

pub fn pedersen_context_log_sizes(claim: &CairoClaim) -> TreeVec<Vec<u32>> {
    if claim.pedersen_aggregator_window_bits_18.is_some() {
        let log_sizes = [
            claim
                .pedersen_aggregator_window_bits_18
                .unwrap()
                .log_sizes(),
            claim.partial_ec_mul_window_bits_18.unwrap().log_sizes(),
            claim
                .pedersen_points_table_window_bits_18
                .unwrap()
                .log_sizes(),
        ]
        .into_iter();
        TreeVec::concat_cols(log_sizes)
    } else {
        TreeVec::concat_cols(vec![].into_iter())
    }
}

pub struct PedersenContextComponents {
    pub components: Option<Components>,
}
impl PedersenContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        cairo_claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let components = interaction_claim
            .pedersen_aggregator_window_bits_18
            .as_ref()
            .map(|_| {
                Components::new(
                    tree_span_provider,
                    cairo_claim,
                    common_lookup_elements,
                    interaction_claim,
                )
            });
        Self { components }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.components
            .as_ref()
            .map(|c| c.components())
            .unwrap_or_default()
    }
}

impl std::fmt::Display for PedersenContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{components}"),
            None => write!(f, "No Pedersen Context Components"),
        }
    }
}

pub struct Components {
    pub pedersen_aggregator: pedersen_aggregator_window_bits_18::Component,
    pub partial_ec_mul: partial_ec_mul_window_bits_18::Component,
    pub pedersen_points_table: pedersen_points_table_window_bits_18::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        cairo_claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let pedersen_aggregator_component = pedersen_aggregator_window_bits_18::Component::new(
            tree_span_provider,
            pedersen_aggregator_window_bits_18::Eval {
                claim: cairo_claim.pedersen_aggregator_window_bits_18.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim
                .pedersen_aggregator_window_bits_18
                .unwrap()
                .claimed_sum,
        );
        let partial_ec_mul_component = partial_ec_mul_window_bits_18::Component::new(
            tree_span_provider,
            partial_ec_mul_window_bits_18::Eval {
                claim: cairo_claim.partial_ec_mul_window_bits_18.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim
                .partial_ec_mul_window_bits_18
                .unwrap()
                .claimed_sum,
        );

        let pedersen_points_table_component = pedersen_points_table_window_bits_18::Component::new(
            tree_span_provider,
            pedersen_points_table_window_bits_18::Eval {
                claim: cairo_claim.pedersen_points_table_window_bits_18.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim
                .pedersen_points_table_window_bits_18
                .unwrap()
                .claimed_sum,
        );

        Self {
            pedersen_aggregator: pedersen_aggregator_component,
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        vec![
            &self.pedersen_aggregator,
            &self.partial_ec_mul,
            &self.pedersen_points_table,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "PedersenAggregatorWindowBits18: {}",
            indented_component_display(&self.pedersen_aggregator)
        )?;
        writeln!(
            f,
            "PartialEcMulWindowBits18: {}",
            indented_component_display(&self.partial_ec_mul)
        )?;
        writeln!(
            f,
            "PedersenPointsTableWindowBits18: {}",
            indented_component_display(&self.pedersen_points_table)
        )?;

        Ok(())
    }
}

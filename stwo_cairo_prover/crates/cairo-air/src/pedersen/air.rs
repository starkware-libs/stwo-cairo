use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::components::{
    indented_component_display, partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18,
};
use crate::relations::CommonLookupElements;

pub struct PedersenContextComponents {
    pub components: Option<Components>,
}
impl PedersenContextComponents {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        pedersen_aggregator_claim: &Option<pedersen_aggregator_window_bits_18::Claim>,
        partial_ec_mul_claim: &Option<partial_ec_mul_window_bits_18::Claim>,
        pedersen_points_table_claim: &Option<pedersen_points_table_window_bits_18::Claim>,
        common_lookup_elements: &CommonLookupElements,
        pedersen_aggregator_interaction_claim: &Option<
            pedersen_aggregator_window_bits_18::InteractionClaim,
        >,
        partial_ec_mul_interaction_claim: &Option<partial_ec_mul_window_bits_18::InteractionClaim>,
        pedersen_points_table_interaction_claim: &Option<
            pedersen_points_table_window_bits_18::InteractionClaim,
        >,
    ) -> Self {
        let components = (*pedersen_aggregator_interaction_claim).map(|_| {
            Components::new(
                tree_span_provider,
                pedersen_aggregator_claim,
                partial_ec_mul_claim,
                pedersen_points_table_claim,
                common_lookup_elements,
                pedersen_aggregator_interaction_claim,
                partial_ec_mul_interaction_claim,
                pedersen_points_table_interaction_claim,
            )
        });
        Self { components }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        self.components
            .as_ref()
            .map(|c| c.provers())
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
    #[allow(clippy::too_many_arguments)]
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        pedersen_aggregator_claim: &Option<pedersen_aggregator_window_bits_18::Claim>,
        partial_ec_mul_claim: &Option<partial_ec_mul_window_bits_18::Claim>,
        pedersen_points_table_claim: &Option<pedersen_points_table_window_bits_18::Claim>,
        common_lookup_elements: &CommonLookupElements,
        pedersen_aggregator_interaction_claim: &Option<
            pedersen_aggregator_window_bits_18::InteractionClaim,
        >,
        partial_ec_mul_interaction_claim: &Option<partial_ec_mul_window_bits_18::InteractionClaim>,
        pedersen_points_table_interaction_claim: &Option<
            pedersen_points_table_window_bits_18::InteractionClaim,
        >,
    ) -> Self {
        let pedersen_aggregator_component = pedersen_aggregator_window_bits_18::Component::new(
            tree_span_provider,
            pedersen_aggregator_window_bits_18::Eval {
                claim: (*pedersen_aggregator_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            pedersen_aggregator_interaction_claim.unwrap().claimed_sum,
        );
        let partial_ec_mul_component = partial_ec_mul_window_bits_18::Component::new(
            tree_span_provider,
            partial_ec_mul_window_bits_18::Eval {
                claim: (*partial_ec_mul_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            partial_ec_mul_interaction_claim.unwrap().claimed_sum,
        );

        let pedersen_points_table_component = pedersen_points_table_window_bits_18::Component::new(
            tree_span_provider,
            pedersen_points_table_window_bits_18::Eval {
                claim: (*pedersen_points_table_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            pedersen_points_table_interaction_claim.unwrap().claimed_sum,
        );

        Self {
            pedersen_aggregator: pedersen_aggregator_component,
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
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

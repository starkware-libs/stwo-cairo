use stwo::core::air::Component;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::{
    cube_252, indented_component_display, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_252_width_27,
};
use crate::relations::CommonLookupElements;

pub struct PoseidonContextComponents {
    pub components: Option<Components>,
}
impl PoseidonContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        cairo_claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let components = interaction_claim.poseidon_aggregator.as_ref().map(|_| {
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

impl std::fmt::Display for PoseidonContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{components}"),
            None => write!(f, "No Poseidon Context Components"),
        }
    }
}

pub struct Components {
    pub poseidon_aggregator: poseidon_aggregator::Component,
    pub poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain::Component,
    pub poseidon_full_round_chain: poseidon_full_round_chain::Component,
    pub cube_252: cube_252::Component,
    pub poseidon_round_keys: poseidon_round_keys::Component,
    pub range_check_252_width_27: range_check_252_width_27::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        cairo_claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let poseidon_aggregator_component = poseidon_aggregator::Component::new(
            tree_span_provider,
            poseidon_aggregator::Eval {
                claim: cairo_claim.poseidon_aggregator.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.poseidon_aggregator.unwrap().claimed_sum,
        );
        let poseidon_3_partial_rounds_chain_component =
            poseidon_3_partial_rounds_chain::Component::new(
                tree_span_provider,
                poseidon_3_partial_rounds_chain::Eval {
                    claim: cairo_claim.poseidon_3_partial_rounds_chain.unwrap(),
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim
                    .poseidon_3_partial_rounds_chain
                    .unwrap()
                    .claimed_sum,
            );
        let poseidon_full_round_chain_component = poseidon_full_round_chain::Component::new(
            tree_span_provider,
            poseidon_full_round_chain::Eval {
                claim: cairo_claim.poseidon_full_round_chain.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim
                .poseidon_full_round_chain
                .unwrap()
                .claimed_sum,
        );
        let cube_252_component = cube_252::Component::new(
            tree_span_provider,
            cube_252::Eval {
                claim: cairo_claim.cube_252.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.cube_252.unwrap().claimed_sum,
        );
        let poseidon_round_keys_component = poseidon_round_keys::Component::new(
            tree_span_provider,
            poseidon_round_keys::Eval {
                claim: cairo_claim.poseidon_round_keys.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.poseidon_round_keys.unwrap().claimed_sum,
        );
        let range_check_felt_252_width_27_component = range_check_252_width_27::Component::new(
            tree_span_provider,
            range_check_252_width_27::Eval {
                claim: cairo_claim.range_check_252_width_27.unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim
                .range_check_252_width_27
                .unwrap()
                .claimed_sum,
        );
        Self {
            poseidon_aggregator: poseidon_aggregator_component,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_252_width_27: range_check_felt_252_width_27_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        vec![
            &self.poseidon_aggregator,
            &self.poseidon_3_partial_rounds_chain,
            &self.poseidon_full_round_chain,
            &self.cube_252,
            &self.poseidon_round_keys,
            &self.range_check_252_width_27,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "PoseidonAggregator: {}",
            indented_component_display(&self.poseidon_aggregator)
        )?;
        writeln!(
            f,
            "Poseidon3PartialRoundsChain: {}",
            indented_component_display(&self.poseidon_3_partial_rounds_chain)
        )?;
        writeln!(
            f,
            "PoseidonFullRoundChain: {}",
            indented_component_display(&self.poseidon_full_round_chain)
        )?;
        writeln!(f, "Cube252: {}", indented_component_display(&self.cube_252))?;
        writeln!(
            f,
            "PoseidonRoundKeys: {}",
            indented_component_display(&self.poseidon_round_keys)
        )?;
        writeln!(
            f,
            "RangeCheck252Width27: {}",
            indented_component_display(&self.range_check_252_width_27)
        )?;
        Ok(())
    }
}

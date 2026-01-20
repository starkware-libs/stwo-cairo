use stwo::core::air::Component;
use stwo::core::pcs::TreeVec;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::claims::CairoClaim;
use crate::components::{
    cube_252, indented_component_display, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_252_width_27,
};
use crate::relations::CommonLookupElements;

pub fn poseidon_context_log_sizes(claim: &CairoClaim) -> TreeVec<Vec<u32>> {
    let mut log_sizes = vec![];
    claim
        .poseidon_aggregator
        .inspect(|c| log_sizes.push(c.log_sizes()));
    claim
        .poseidon_3_partial_rounds_chain
        .inspect(|c| log_sizes.push(c.log_sizes()));
    claim
        .poseidon_full_round_chain
        .inspect(|c| log_sizes.push(c.log_sizes()));
    claim.cube_252.inspect(|c| log_sizes.push(c.log_sizes()));
    claim
        .poseidon_round_keys
        .inspect(|c| log_sizes.push(c.log_sizes()));
    claim
        .range_check_252_width_27
        .inspect(|c| log_sizes.push(c.log_sizes()));

    TreeVec::concat_cols(log_sizes.into_iter())
}

pub struct PoseidonContextComponents {
    pub components: Option<Components>,
}
impl PoseidonContextComponents {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        poseidon_aggregator_claim: &Option<poseidon_aggregator::Claim>,
        poseidon_3_partial_rounds_chain_claim: &Option<poseidon_3_partial_rounds_chain::Claim>,
        poseidon_full_round_chain_claim: &Option<poseidon_full_round_chain::Claim>,
        cube_252_claim: &Option<cube_252::Claim>,
        poseidon_round_keys_claim: &Option<poseidon_round_keys::Claim>,
        range_check_252_width_27_claim: &Option<range_check_252_width_27::Claim>,
        common_lookup_elements: &CommonLookupElements,
        poseidon_aggregator_interaction_claim: &Option<poseidon_aggregator::InteractionClaim>,
        poseidon_3_partial_rounds_chain_interaction_claim: &Option<
            poseidon_3_partial_rounds_chain::InteractionClaim,
        >,
        poseidon_full_round_chain_interaction_claim: &Option<
            poseidon_full_round_chain::InteractionClaim,
        >,
        cube_252_interaction_claim: &Option<cube_252::InteractionClaim>,
        poseidon_round_keys_interaction_claim: &Option<poseidon_round_keys::InteractionClaim>,
        range_check_252_width_27_interaction_claim: &Option<
            range_check_252_width_27::InteractionClaim,
        >,
    ) -> Self {
        let components = poseidon_aggregator_interaction_claim.as_ref().map(|_| {
            Components::new(
                tree_span_provider,
                poseidon_aggregator_claim,
                poseidon_3_partial_rounds_chain_claim,
                poseidon_full_round_chain_claim,
                cube_252_claim,
                poseidon_round_keys_claim,
                range_check_252_width_27_claim,
                common_lookup_elements,
                poseidon_aggregator_interaction_claim,
                poseidon_3_partial_rounds_chain_interaction_claim,
                poseidon_full_round_chain_interaction_claim,
                cube_252_interaction_claim,
                poseidon_round_keys_interaction_claim,
                range_check_252_width_27_interaction_claim,
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
    #[allow(clippy::too_many_arguments)]
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        poseidon_aggregator_claim: &Option<poseidon_aggregator::Claim>,
        poseidon_3_partial_rounds_chain_claim: &Option<poseidon_3_partial_rounds_chain::Claim>,
        poseidon_full_round_chain_claim: &Option<poseidon_full_round_chain::Claim>,
        cube_252_claim: &Option<cube_252::Claim>,
        poseidon_round_keys_claim: &Option<poseidon_round_keys::Claim>,
        range_check_252_width_27_claim: &Option<range_check_252_width_27::Claim>,
        common_lookup_elements: &CommonLookupElements,
        poseidon_aggregator_interaction_claim: &Option<poseidon_aggregator::InteractionClaim>,
        poseidon_3_partial_rounds_chain_interaction_claim: &Option<
            poseidon_3_partial_rounds_chain::InteractionClaim,
        >,
        poseidon_full_round_chain_interaction_claim: &Option<
            poseidon_full_round_chain::InteractionClaim,
        >,
        cube_252_interaction_claim: &Option<cube_252::InteractionClaim>,
        poseidon_round_keys_interaction_claim: &Option<poseidon_round_keys::InteractionClaim>,
        range_check_252_width_27_interaction_claim: &Option<
            range_check_252_width_27::InteractionClaim,
        >,
    ) -> Self {
        let poseidon_aggregator_component = poseidon_aggregator::Component::new(
            tree_span_provider,
            poseidon_aggregator::Eval {
                claim: (*poseidon_aggregator_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            poseidon_aggregator_interaction_claim.unwrap().claimed_sum,
        );
        let poseidon_3_partial_rounds_chain_component =
            poseidon_3_partial_rounds_chain::Component::new(
                tree_span_provider,
                poseidon_3_partial_rounds_chain::Eval {
                    claim: (*poseidon_3_partial_rounds_chain_claim).unwrap(),
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                poseidon_3_partial_rounds_chain_interaction_claim
                    .unwrap()
                    .claimed_sum,
            );
        let poseidon_full_round_chain_component = poseidon_full_round_chain::Component::new(
            tree_span_provider,
            poseidon_full_round_chain::Eval {
                claim: (*poseidon_full_round_chain_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            poseidon_full_round_chain_interaction_claim
                .unwrap()
                .claimed_sum,
        );
        let cube_252_component = cube_252::Component::new(
            tree_span_provider,
            cube_252::Eval {
                claim: (*cube_252_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            cube_252_interaction_claim.unwrap().claimed_sum,
        );
        let poseidon_round_keys_component = poseidon_round_keys::Component::new(
            tree_span_provider,
            poseidon_round_keys::Eval {
                claim: (*poseidon_round_keys_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            poseidon_round_keys_interaction_claim.unwrap().claimed_sum,
        );
        let range_check_felt_252_width_27_component = range_check_252_width_27::Component::new(
            tree_span_provider,
            range_check_252_width_27::Eval {
                claim: (*range_check_252_width_27_claim).unwrap(),
                common_lookup_elements: common_lookup_elements.clone(),
            },
            range_check_252_width_27_interaction_claim
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

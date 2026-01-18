use num_traits::Zero;
use stwo::core::air::Component;
use stwo::core::fields::qm31::QM31;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::air::{accumulate_relation_uses, RelationUsesDict};
use crate::components::prelude::*;
use crate::components::{
    cube_252, indented_component_display, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_252_width_27,
};
use crate::relations::CommonLookupElements;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct PoseidonContextClaim {
    pub claim: Option<Claim>,
}
impl PoseidonContextClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.claim.is_some() as u64);
        if let Some(claim) = &self.claim {
            claim.mix_into(channel);
        }
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        self.claim
            .as_ref()
            .map(|claim| claim.log_sizes())
            .unwrap_or_default()
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        if let Some(claim) = &self.claim {
            claim.accumulate_relation_uses(relation_uses);
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub poseidon_aggregator: poseidon_aggregator::Claim,
    pub poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain::Claim,
    pub poseidon_full_round_chain: poseidon_full_round_chain::Claim,
    pub cube_252: cube_252::Claim,
    pub poseidon_round_keys: poseidon_round_keys::Claim,
    pub range_check_252_width_27: range_check_252_width_27::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.poseidon_aggregator.mix_into(channel);
        self.poseidon_3_partial_rounds_chain.mix_into(channel);
        self.poseidon_full_round_chain.mix_into(channel);
        self.cube_252.mix_into(channel);
        self.poseidon_round_keys.mix_into(channel);
        self.range_check_252_width_27.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.poseidon_aggregator.log_sizes(),
            self.poseidon_3_partial_rounds_chain.log_sizes(),
            self.poseidon_full_round_chain.log_sizes(),
            self.cube_252.log_sizes(),
            self.poseidon_round_keys.log_sizes(),
            self.range_check_252_width_27.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            poseidon_aggregator,
            poseidon_3_partial_rounds_chain,
            poseidon_full_round_chain,
            cube_252,
            poseidon_round_keys: _,
            range_check_252_width_27,
        } = self;

        // NOTE: The following components do not USE relations:
        // - poseidon_round_keys

        accumulate_relation_uses(
            relation_uses,
            poseidon_aggregator::RELATION_USES_PER_ROW,
            poseidon_aggregator.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            poseidon_3_partial_rounds_chain::RELATION_USES_PER_ROW,
            poseidon_3_partial_rounds_chain.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            poseidon_full_round_chain::RELATION_USES_PER_ROW,
            poseidon_full_round_chain.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            cube_252::RELATION_USES_PER_ROW,
            cube_252.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            range_check_252_width_27::RELATION_USES_PER_ROW,
            range_check_252_width_27.log_size,
        );
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct PoseidonContextInteractionClaim {
    pub claim: Option<InteractionClaim>,
}
impl PoseidonContextInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(claim) = &self.claim {
            claim.mix_into(channel);
        }
    }

    pub fn sum(&self) -> QM31 {
        self.claim
            .as_ref()
            .map(|claim| claim.sum())
            .unwrap_or_else(QM31::zero)
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub poseidon_aggregator: poseidon_aggregator::InteractionClaim,
    pub poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain::InteractionClaim,
    pub poseidon_full_round_chain: poseidon_full_round_chain::InteractionClaim,
    pub cube_252: cube_252::InteractionClaim,
    pub poseidon_round_keys: poseidon_round_keys::InteractionClaim,
    pub range_check_252_width_27: range_check_252_width_27::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.poseidon_aggregator.mix_into(channel);
        self.poseidon_3_partial_rounds_chain.mix_into(channel);
        self.poseidon_full_round_chain.mix_into(channel);
        self.cube_252.mix_into(channel);
        self.poseidon_round_keys.mix_into(channel);
        self.range_check_252_width_27.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        let mut sum = QM31::zero();
        sum += self.poseidon_aggregator.claimed_sum;
        sum += self.poseidon_3_partial_rounds_chain.claimed_sum;
        sum += self.poseidon_full_round_chain.claimed_sum;
        sum += self.cube_252.claimed_sum;
        sum += self.poseidon_round_keys.claimed_sum;
        sum += self.range_check_252_width_27.claimed_sum;
        sum
    }
}

pub struct PoseidonContextComponents {
    pub components: Option<Components>,
}
impl PoseidonContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PoseidonContextClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &PoseidonContextInteractionClaim,
    ) -> Self {
        let components = interaction_claim
            .claim
            .as_ref()
            .map(|ic| Components::new(tree_span_provider, claim, common_lookup_elements, ic));
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
        claim: &PoseidonContextClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let poseidon_aggregator_component = poseidon_aggregator::Component::new(
            tree_span_provider,
            poseidon_aggregator::Eval {
                claim: claim.claim.as_ref().unwrap().poseidon_aggregator,
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.poseidon_aggregator.claimed_sum,
        );
        let poseidon_3_partial_rounds_chain_component =
            poseidon_3_partial_rounds_chain::Component::new(
                tree_span_provider,
                poseidon_3_partial_rounds_chain::Eval {
                    claim: claim
                        .claim
                        .as_ref()
                        .unwrap()
                        .poseidon_3_partial_rounds_chain,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                interaction_claim
                    .poseidon_3_partial_rounds_chain
                    .claimed_sum,
            );
        let poseidon_full_round_chain_component = poseidon_full_round_chain::Component::new(
            tree_span_provider,
            poseidon_full_round_chain::Eval {
                claim: claim.claim.as_ref().unwrap().poseidon_full_round_chain,
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.poseidon_full_round_chain.claimed_sum,
        );
        let cube_252_component = cube_252::Component::new(
            tree_span_provider,
            cube_252::Eval {
                claim: claim.claim.as_ref().unwrap().cube_252,
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.cube_252.claimed_sum,
        );
        let poseidon_round_keys_component = poseidon_round_keys::Component::new(
            tree_span_provider,
            poseidon_round_keys::Eval {
                claim: claim.claim.as_ref().unwrap().poseidon_round_keys,
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.poseidon_round_keys.claimed_sum,
        );
        let range_check_felt_252_width_27_component = range_check_252_width_27::Component::new(
            tree_span_provider,
            range_check_252_width_27::Eval {
                claim: claim.claim.as_ref().unwrap().range_check_252_width_27,
                common_lookup_elements: common_lookup_elements.clone(),
            },
            interaction_claim.range_check_252_width_27.claimed_sum,
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

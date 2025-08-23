use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::QM31;
use stwo::core::pcs::TreeVec;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::TraceLocationAllocator;

use crate::air::{accumulate_relation_uses, CairoInteractionElements, RelationUsesDict};
use crate::components::{
    blake_g, blake_round, blake_round_sigma, triple_xor_32, verify_bitwise_xor_12,
};

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct BlakeContextClaim {
    pub claim: Option<Claim>,
}
impl BlakeContextClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
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
    pub blake_round: blake_round::Claim,
    pub blake_g: blake_g::Claim,
    pub blake_sigma: blake_round_sigma::Claim,
    pub triple_xor_32: triple_xor_32::Claim,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.blake_round.mix_into(channel);
        self.blake_g.mix_into(channel);
        self.blake_sigma.mix_into(channel);
        self.triple_xor_32.mix_into(channel);
        self.verify_bitwise_xor_12.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.blake_round.log_sizes(),
            self.blake_g.log_sizes(),
            self.blake_sigma.log_sizes(),
            self.triple_xor_32.log_sizes(),
            self.verify_bitwise_xor_12.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            blake_round,
            blake_g,
            blake_sigma: _,
            triple_xor_32,
            verify_bitwise_xor_12: _,
        } = self;

        // NOTE: The following components do not USE relations:
        // - blake_sigma
        // - verify_bitwise_xor_12

        accumulate_relation_uses(
            relation_uses,
            blake_round::RELATION_USES_PER_ROW,
            blake_round.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            blake_g::RELATION_USES_PER_ROW,
            blake_g.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            triple_xor_32::RELATION_USES_PER_ROW,
            triple_xor_32.log_size,
        );
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct BlakeContextInteractionClaim {
    pub claim: Option<InteractionClaim>,
}
impl BlakeContextInteractionClaim {
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
    pub blake_round: blake_round::InteractionClaim,
    pub blake_g: blake_g::InteractionClaim,
    pub blake_sigma: blake_round_sigma::InteractionClaim,
    pub triple_xor_32: triple_xor_32::InteractionClaim,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.blake_round.mix_into(channel);
        self.blake_g.mix_into(channel);
        self.blake_sigma.mix_into(channel);
        self.triple_xor_32.mix_into(channel);
        self.verify_bitwise_xor_12.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        self.blake_round.claimed_sum
            + self.blake_g.claimed_sum
            + self.blake_sigma.claimed_sum
            + self.triple_xor_32.claimed_sum
            + self.verify_bitwise_xor_12.claimed_sum
    }
}

pub struct BlakeContextComponents {
    pub components: Option<Components>,
}
impl BlakeContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BlakeContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &BlakeContextInteractionClaim,
    ) -> Self {
        let components = interaction_claim
            .claim
            .as_ref()
            .map(|ic| Components::new(tree_span_provider, claim, interaction_elements, ic));
        Self { components }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        self.components
            .as_ref()
            .map(|c| c.provers())
            .unwrap_or_default()
    }
}

impl std::fmt::Display for BlakeContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{components}"),
            None => write!(f, "No Blake Context Components"),
        }
    }
}

pub struct Components {
    pub blake_round: blake_round::Component,
    pub blake_g: blake_g::Component,
    pub blake_sigma: blake_round_sigma::Component,
    pub triple_xor_32: triple_xor_32::Component,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BlakeContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let blake_round_component = blake_round::Component::new(
            tree_span_provider,
            blake_round::Eval {
                claim: claim.claim.as_ref().unwrap().blake_round,
                blake_g_lookup_elements: interaction_elements.blake_g.clone(),
                blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
                memory_address_to_id_lookup_elements: interaction_elements
                    .memory_address_to_id
                    .clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            interaction_claim.blake_round.claimed_sum,
        );

        let blake_g_component = blake_g::Component::new(
            tree_span_provider,
            blake_g::Eval {
                claim: claim.claim.as_ref().unwrap().blake_g,
                blake_g_lookup_elements: interaction_elements.blake_g.clone(),
                verify_bitwise_xor_12_lookup_elements: interaction_elements
                    .verify_bitwise_xor_12
                    .clone(),
                verify_bitwise_xor_4_lookup_elements: interaction_elements
                    .verify_bitwise_xor_4
                    .clone(),
                verify_bitwise_xor_7_lookup_elements: interaction_elements
                    .verify_bitwise_xor_7
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
                verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8_b
                    .clone(),
                verify_bitwise_xor_9_lookup_elements: interaction_elements
                    .verify_bitwise_xor_9
                    .clone(),
            },
            interaction_claim.blake_g.claimed_sum,
        );

        let blake_sigma_component = blake_round_sigma::Component::new(
            tree_span_provider,
            blake_round_sigma::Eval {
                claim: claim.claim.as_ref().unwrap().blake_sigma,
                blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
            },
            interaction_claim.blake_sigma.claimed_sum,
        );

        let triple_xor_32_component = triple_xor_32::Component::new(
            tree_span_provider,
            triple_xor_32::Eval {
                claim: claim.claim.as_ref().unwrap().triple_xor_32,
                triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
                verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8_b
                    .clone(),
            },
            interaction_claim.triple_xor_32.claimed_sum,
        );
        let verify_bitwise_xor_12_component = verify_bitwise_xor_12::Component::new(
            tree_span_provider,
            verify_bitwise_xor_12::Eval {
                claim: claim.claim.as_ref().unwrap().verify_bitwise_xor_12,
                verify_bitwise_xor_12_lookup_elements: interaction_elements
                    .verify_bitwise_xor_12
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_12.claimed_sum,
        );
        Self {
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_sigma: blake_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.blake_round,
            &self.blake_g,
            &self.blake_sigma,
            &self.triple_xor_32,
            &self.verify_bitwise_xor_12,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BlakeRound: {}", self.blake_round)?;
        writeln!(f, "BlakeG: {}", self.blake_g)?;
        writeln!(f, "BlakeSigma: {}", self.blake_sigma)?;
        writeln!(f, "TripleXor32: {}", self.triple_xor_32)?;
        writeln!(f, "VerifyBitwiseXor12: {}", self.verify_bitwise_xor_12)?;
        Ok(())
    }
}

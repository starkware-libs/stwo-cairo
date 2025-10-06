use num_traits::Zero;
use stwo::core::fields::qm31::QM31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::air::{accumulate_relation_uses, CairoInteractionElements, RelationUsesDict};
use crate::components::prelude::*;
use crate::components::{
    indented_component_display, sha_256_big_sigma_0, sha_256_big_sigma_0_o_0,
    sha_256_big_sigma_0_o_1, sha_256_big_sigma_1, sha_256_big_sigma_1_o_0, sha_256_big_sigma_1_o_1,
    sha_256_k_table, sha_256_round, sha_256_schedule, sha_256_small_sigma_0,
    sha_256_small_sigma_0_o_0, sha_256_small_sigma_0_o_1, sha_256_small_sigma_1,
    sha_256_small_sigma_1_o_0, sha_256_small_sigma_1_o_1,
};

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Sha256ContextClaim {
    pub claim: Option<Claim>,
}
impl Sha256ContextClaim {
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
    pub sha_256_round: sha_256_round::Claim,
    pub sha_256_big_sigma_0: sha_256_big_sigma_0::Claim,
    pub sha_256_big_sigma_1: sha_256_big_sigma_1::Claim,
    pub sha_256_schedule: sha_256_schedule::Claim,
    pub sha_256_small_sigma_0: sha_256_small_sigma_0::Claim,
    pub sha_256_small_sigma_1: sha_256_small_sigma_1::Claim,
    pub sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0::Claim,
    pub sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1::Claim,
    pub sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0::Claim,
    pub sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1::Claim,
    pub sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0::Claim,
    pub sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1::Claim,
    pub sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0::Claim,
    pub sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1::Claim,
    pub sha_256_k_table: sha_256_k_table::Claim,
}

impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.sha_256_round.mix_into(channel);
        self.sha_256_big_sigma_0.mix_into(channel);
        self.sha_256_big_sigma_1.mix_into(channel);
        self.sha_256_schedule.mix_into(channel);
        self.sha_256_small_sigma_0.mix_into(channel);
        self.sha_256_small_sigma_1.mix_into(channel);
        self.sha_256_big_sigma_0_o_0.mix_into(channel);
        self.sha_256_big_sigma_0_o_1.mix_into(channel);
        self.sha_256_big_sigma_1_o_0.mix_into(channel);
        self.sha_256_big_sigma_1_o_1.mix_into(channel);
        self.sha_256_small_sigma_0_o_0.mix_into(channel);
        self.sha_256_small_sigma_0_o_1.mix_into(channel);
        self.sha_256_small_sigma_1_o_0.mix_into(channel);
        self.sha_256_small_sigma_1_o_1.mix_into(channel);
        self.sha_256_k_table.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.sha_256_round.log_sizes(),
            self.sha_256_big_sigma_0.log_sizes(),
            self.sha_256_big_sigma_1.log_sizes(),
            self.sha_256_schedule.log_sizes(),
            self.sha_256_small_sigma_0.log_sizes(),
            self.sha_256_small_sigma_1.log_sizes(),
            self.sha_256_big_sigma_0_o_0.log_sizes(),
            self.sha_256_big_sigma_0_o_1.log_sizes(),
            self.sha_256_big_sigma_1_o_0.log_sizes(),
            self.sha_256_big_sigma_1_o_1.log_sizes(),
            self.sha_256_small_sigma_0_o_0.log_sizes(),
            self.sha_256_small_sigma_0_o_1.log_sizes(),
            self.sha_256_small_sigma_1_o_0.log_sizes(),
            self.sha_256_small_sigma_1_o_1.log_sizes(),
            self.sha_256_k_table.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            sha_256_round,
            sha_256_big_sigma_0,
            sha_256_big_sigma_1,
            sha_256_schedule,
            sha_256_small_sigma_0,
            sha_256_small_sigma_1,
            ..
        } = self;

        accumulate_relation_uses(
            relation_uses,
            sha_256_round::RELATION_USES_PER_ROW,
            sha_256_round.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            sha_256_big_sigma_0::RELATION_USES_PER_ROW,
            sha_256_big_sigma_0.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            sha_256_big_sigma_1::RELATION_USES_PER_ROW,
            sha_256_big_sigma_1.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            sha_256_schedule::RELATION_USES_PER_ROW,
            sha_256_schedule.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            sha_256_small_sigma_0::RELATION_USES_PER_ROW,
            sha_256_small_sigma_0.log_size,
        );
        accumulate_relation_uses(
            relation_uses,
            sha_256_small_sigma_1::RELATION_USES_PER_ROW,
            sha_256_small_sigma_1.log_size,
        );
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Sha256ContextInteractionClaim {
    pub claim: Option<InteractionClaim>,
}
impl Sha256ContextInteractionClaim {
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
    pub sha_256_round: sha_256_round::InteractionClaim,
    pub sha_256_big_sigma_0: sha_256_big_sigma_0::InteractionClaim,
    pub sha_256_big_sigma_1: sha_256_big_sigma_1::InteractionClaim,
    pub sha_256_schedule: sha_256_schedule::InteractionClaim,
    pub sha_256_small_sigma_0: sha_256_small_sigma_0::InteractionClaim,
    pub sha_256_small_sigma_1: sha_256_small_sigma_1::InteractionClaim,
    pub sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0::InteractionClaim,
    pub sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1::InteractionClaim,
    pub sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0::InteractionClaim,
    pub sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1::InteractionClaim,
    pub sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0::InteractionClaim,
    pub sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1::InteractionClaim,
    pub sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0::InteractionClaim,
    pub sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1::InteractionClaim,
    pub sha_256_k_table: sha_256_k_table::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.sha_256_round.mix_into(channel);
        self.sha_256_big_sigma_0.mix_into(channel);
        self.sha_256_big_sigma_1.mix_into(channel);
        self.sha_256_schedule.mix_into(channel);
        self.sha_256_small_sigma_0.mix_into(channel);
        self.sha_256_small_sigma_1.mix_into(channel);
        self.sha_256_big_sigma_0_o_0.mix_into(channel);
        self.sha_256_big_sigma_0_o_1.mix_into(channel);
        self.sha_256_big_sigma_1_o_0.mix_into(channel);
        self.sha_256_big_sigma_1_o_1.mix_into(channel);
        self.sha_256_small_sigma_0_o_0.mix_into(channel);
        self.sha_256_small_sigma_0_o_1.mix_into(channel);
        self.sha_256_small_sigma_1_o_0.mix_into(channel);
        self.sha_256_small_sigma_1_o_1.mix_into(channel);
        self.sha_256_k_table.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        let mut sum = QM31::zero();
        sum += self.sha_256_round.claimed_sum;
        sum += self.sha_256_big_sigma_0.claimed_sum;
        sum += self.sha_256_big_sigma_1.claimed_sum;
        sum += self.sha_256_schedule.claimed_sum;
        sum += self.sha_256_small_sigma_0.claimed_sum;
        sum += self.sha_256_small_sigma_1.claimed_sum;
        sum += self.sha_256_big_sigma_0_o_0.claimed_sum;
        sum += self.sha_256_big_sigma_0_o_1.claimed_sum;
        sum += self.sha_256_big_sigma_1_o_0.claimed_sum;
        sum += self.sha_256_big_sigma_1_o_1.claimed_sum;
        sum += self.sha_256_small_sigma_0_o_0.claimed_sum;
        sum += self.sha_256_small_sigma_0_o_1.claimed_sum;
        sum += self.sha_256_small_sigma_1_o_0.claimed_sum;
        sum += self.sha_256_small_sigma_1_o_1.claimed_sum;
        sum += self.sha_256_k_table.claimed_sum;
        sum
    }
}

pub struct Sha256ContextComponents {
    pub components: Option<Components>,
}
impl Sha256ContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &Sha256ContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &Sha256ContextInteractionClaim,
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

impl std::fmt::Display for Sha256ContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{components}"),
            None => write!(f, "No Sha256 Context Components"),
        }
    }
}

pub struct Components {
    pub sha_256_round: sha_256_round::Component,
    pub sha_256_big_sigma_0: sha_256_big_sigma_0::Component,
    pub sha_256_big_sigma_1: sha_256_big_sigma_1::Component,
    pub sha_256_schedule: sha_256_schedule::Component,
    pub sha_256_small_sigma_0: sha_256_small_sigma_0::Component,
    pub sha_256_small_sigma_1: sha_256_small_sigma_1::Component,
    pub sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0::Component,
    pub sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1::Component,
    pub sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0::Component,
    pub sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1::Component,
    pub sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0::Component,
    pub sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1::Component,
    pub sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0::Component,
    pub sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1::Component,
    pub sha_256_k_table: sha_256_k_table::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &Sha256ContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let sha_256_round_component = sha_256_round::Component::new(
            tree_span_provider,
            sha_256_round::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_round,
                sha_256_big_sigma_0_lookup_elements: interaction_elements
                    .sha_256_big_sigma_0
                    .clone(),
                verify_bitwise_and_8_lookup_elements: interaction_elements
                    .verify_bitwise_and_8
                    .clone(),
                verify_bitwise_not_16_lookup_elements: interaction_elements
                    .verify_bitwise_not_16
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
                sha_256_big_sigma_1_lookup_elements: interaction_elements
                    .sha_256_big_sigma_1
                    .clone(),
                sha_256_schedule_lookup_elements: interaction_elements.sha_256_schedule.clone(),
                sha_256_k_table_lookup_elements: interaction_elements.sha_256_k_table.clone(),
                sha_256_round_lookup_elements: interaction_elements.sha_256_round.clone(),
            },
            interaction_claim.sha_256_round.claimed_sum,
        );

        let sha_256_big_sigma_0_component = sha_256_big_sigma_0::Component::new(
            tree_span_provider,
            sha_256_big_sigma_0::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_big_sigma_0,
                sha_256_big_sigma_0_lookup_elements: interaction_elements
                    .sha_256_big_sigma_0
                    .clone(),
                sha_256_big_sigma_0_o_0_lookup_elements: interaction_elements
                    .sha_256_big_sigma_0_o_0
                    .clone(),
                sha_256_big_sigma_0_o_1_lookup_elements: interaction_elements
                    .sha_256_big_sigma_0_o_1
                    .clone(),
                verify_bitwise_and_8_lookup_elements: interaction_elements
                    .verify_bitwise_and_8
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.sha_256_big_sigma_0.claimed_sum,
        );

        let sha_256_big_sigma_1_component = sha_256_big_sigma_1::Component::new(
            tree_span_provider,
            sha_256_big_sigma_1::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_big_sigma_1,
                sha_256_big_sigma_1_lookup_elements: interaction_elements
                    .sha_256_big_sigma_1
                    .clone(),
                sha_256_big_sigma_1_o_0_lookup_elements: interaction_elements
                    .sha_256_big_sigma_1_o_0
                    .clone(),
                sha_256_big_sigma_1_o_1_lookup_elements: interaction_elements
                    .sha_256_big_sigma_1_o_1
                    .clone(),
                verify_bitwise_and_8_lookup_elements: interaction_elements
                    .verify_bitwise_and_8
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.sha_256_big_sigma_1.claimed_sum,
        );

        let sha_256_schedule_component = sha_256_schedule::Component::new(
            tree_span_provider,
            sha_256_schedule::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_schedule,
                sha_256_small_sigma_0_lookup_elements: interaction_elements
                    .sha_256_small_sigma_0
                    .clone(),
                sha_256_small_sigma_1_lookup_elements: interaction_elements
                    .sha_256_small_sigma_1
                    .clone(),
                range_check_2_lookup_elements: interaction_elements.range_checks.rc_2.clone(),
                sha_256_schedule_lookup_elements: interaction_elements.sha_256_schedule.clone(),
            },
            interaction_claim.sha_256_schedule.claimed_sum,
        );
        let sha_256_small_sigma_0_component = sha_256_small_sigma_0::Component::new(
            tree_span_provider,
            sha_256_small_sigma_0::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_small_sigma_0,
                verify_bitwise_and_8_lookup_elements: interaction_elements
                    .verify_bitwise_and_8
                    .clone(),
                sha_256_small_sigma_0_o_0_lookup_elements: interaction_elements
                    .sha_256_small_sigma_0_o_0
                    .clone(),
                sha_256_small_sigma_0_o_1_lookup_elements: interaction_elements
                    .sha_256_small_sigma_0_o_1
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
                sha_256_small_sigma_0_lookup_elements: interaction_elements
                    .sha_256_small_sigma_0
                    .clone(),
            },
            interaction_claim.sha_256_small_sigma_0.claimed_sum,
        );

        let sha_256_small_sigma_1_component = sha_256_small_sigma_1::Component::new(
            tree_span_provider,
            sha_256_small_sigma_1::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_small_sigma_1,
                sha_256_small_sigma_1_lookup_elements: interaction_elements
                    .sha_256_small_sigma_1
                    .clone(),
                sha_256_small_sigma_1_o_0_lookup_elements: interaction_elements
                    .sha_256_small_sigma_1_o_0
                    .clone(),
                sha_256_small_sigma_1_o_1_lookup_elements: interaction_elements
                    .sha_256_small_sigma_1_o_1
                    .clone(),
                verify_bitwise_and_8_lookup_elements: interaction_elements
                    .verify_bitwise_and_8
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.sha_256_small_sigma_1.claimed_sum,
        );
        let sha_256_big_sigma_0_o_0_component = sha_256_big_sigma_0_o_0::Component::new(
            tree_span_provider,
            sha_256_big_sigma_0_o_0::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_big_sigma_0_o_0,
                sha_256_big_sigma_0_o_0_lookup_elements: interaction_elements
                    .sha_256_big_sigma_0_o_0
                    .clone(),
            },
            interaction_claim.sha_256_big_sigma_0_o_0.claimed_sum,
        );
        let sha_256_big_sigma_0_o_1_component = sha_256_big_sigma_0_o_1::Component::new(
            tree_span_provider,
            sha_256_big_sigma_0_o_1::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_big_sigma_0_o_1,
                sha_256_big_sigma_0_o_1_lookup_elements: interaction_elements
                    .sha_256_big_sigma_0_o_1
                    .clone(),
            },
            interaction_claim.sha_256_big_sigma_0_o_1.claimed_sum,
        );
        let sha_256_big_sigma_1_o_0_component = sha_256_big_sigma_1_o_0::Component::new(
            tree_span_provider,
            sha_256_big_sigma_1_o_0::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_big_sigma_1_o_0,
                sha_256_big_sigma_1_o_0_lookup_elements: interaction_elements
                    .sha_256_big_sigma_1_o_0
                    .clone(),
            },
            interaction_claim.sha_256_big_sigma_1_o_0.claimed_sum,
        );
        let sha_256_big_sigma_1_o_1_component = sha_256_big_sigma_1_o_1::Component::new(
            tree_span_provider,
            sha_256_big_sigma_1_o_1::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_big_sigma_1_o_1,
                sha_256_big_sigma_1_o_1_lookup_elements: interaction_elements
                    .sha_256_big_sigma_1_o_1
                    .clone(),
            },
            interaction_claim.sha_256_big_sigma_1_o_1.claimed_sum,
        );
        let sha_256_small_sigma_0_o_0_component = sha_256_small_sigma_0_o_0::Component::new(
            tree_span_provider,
            sha_256_small_sigma_0_o_0::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_small_sigma_0_o_0,
                sha_256_small_sigma_0_o_0_lookup_elements: interaction_elements
                    .sha_256_small_sigma_0_o_0
                    .clone(),
            },
            interaction_claim.sha_256_small_sigma_0_o_0.claimed_sum,
        );
        let sha_256_small_sigma_0_o_1_component = sha_256_small_sigma_0_o_1::Component::new(
            tree_span_provider,
            sha_256_small_sigma_0_o_1::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_small_sigma_0_o_1,
                sha_256_small_sigma_0_o_1_lookup_elements: interaction_elements
                    .sha_256_small_sigma_0_o_1
                    .clone(),
            },
            interaction_claim.sha_256_small_sigma_0_o_1.claimed_sum,
        );
        let sha_256_small_sigma_1_o_0_component = sha_256_small_sigma_1_o_0::Component::new(
            tree_span_provider,
            sha_256_small_sigma_1_o_0::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_small_sigma_1_o_0,
                sha_256_small_sigma_1_o_0_lookup_elements: interaction_elements
                    .sha_256_small_sigma_1_o_0
                    .clone(),
            },
            interaction_claim.sha_256_small_sigma_1_o_0.claimed_sum,
        );
        let sha_256_small_sigma_1_o_1_component = sha_256_small_sigma_1_o_1::Component::new(
            tree_span_provider,
            sha_256_small_sigma_1_o_1::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_small_sigma_1_o_1,
                sha_256_small_sigma_1_o_1_lookup_elements: interaction_elements
                    .sha_256_small_sigma_1_o_1
                    .clone(),
            },
            interaction_claim.sha_256_small_sigma_1_o_1.claimed_sum,
        );
        let sha_256_k_table_component = sha_256_k_table::Component::new(
            tree_span_provider,
            sha_256_k_table::Eval {
                claim: claim.claim.as_ref().unwrap().sha_256_k_table,
                sha_256_k_table_lookup_elements: interaction_elements.sha_256_k_table.clone(),
            },
            interaction_claim.sha_256_k_table.claimed_sum,
        );

        Self {
            sha_256_round: sha_256_round_component,
            sha_256_big_sigma_0: sha_256_big_sigma_0_component,
            sha_256_big_sigma_1: sha_256_big_sigma_1_component,
            sha_256_schedule: sha_256_schedule_component,
            sha_256_small_sigma_0: sha_256_small_sigma_0_component,
            sha_256_small_sigma_1: sha_256_small_sigma_1_component,
            sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0_component,
            sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1_component,
            sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0_component,
            sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1_component,
            sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0_component,
            sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1_component,
            sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0_component,
            sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1_component,
            sha_256_k_table: sha_256_k_table_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.sha_256_round,
            &self.sha_256_big_sigma_0,
            &self.sha_256_big_sigma_1,
            &self.sha_256_schedule,
            &self.sha_256_small_sigma_0,
            &self.sha_256_small_sigma_1,
            &self.sha_256_big_sigma_0_o_0,
            &self.sha_256_big_sigma_0_o_1,
            &self.sha_256_big_sigma_1_o_0,
            &self.sha_256_big_sigma_1_o_1,
            &self.sha_256_small_sigma_0_o_0,
            &self.sha_256_small_sigma_0_o_1,
            &self.sha_256_small_sigma_1_o_0,
            &self.sha_256_small_sigma_1_o_1,
            &self.sha_256_k_table,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Sha256Round: {}",
            indented_component_display(&self.sha_256_round)
        )?;
        writeln!(
            f,
            "Sha256BigSigma0: {}",
            indented_component_display(&self.sha_256_big_sigma_0)
        )?;
        writeln!(
            f,
            "Sha256BigSigma1: {}",
            indented_component_display(&self.sha_256_big_sigma_1)
        )?;
        writeln!(
            f,
            "Sha256Schedule: {}",
            indented_component_display(&self.sha_256_schedule)
        )?;
        writeln!(
            f,
            "Sha256SmallSigma0: {}",
            indented_component_display(&self.sha_256_small_sigma_0)
        )?;
        writeln!(
            f,
            "Sha256SmallSigma1: {}",
            indented_component_display(&self.sha_256_small_sigma_1)
        )?;
        writeln!(
            f,
            "Sha256BigSigma0O0: {}",
            indented_component_display(&self.sha_256_big_sigma_0_o_0)
        )?;
        writeln!(
            f,
            "Sha256BigSigma0O1: {}",
            indented_component_display(&self.sha_256_big_sigma_0_o_1)
        )?;
        writeln!(
            f,
            "Sha256BigSigma1O0: {}",
            indented_component_display(&self.sha_256_big_sigma_1_o_0)
        )?;
        writeln!(
            f,
            "Sha256BigSigma1O1: {}",
            indented_component_display(&self.sha_256_big_sigma_1_o_1)
        )?;
        writeln!(
            f,
            "Sha256SmallSigma0O0: {}",
            indented_component_display(&self.sha_256_small_sigma_0_o_0)
        )?;
        writeln!(
            f,
            "Sha256SmallSigma0O1: {}",
            indented_component_display(&self.sha_256_small_sigma_0_o_1)
        )?;
        writeln!(
            f,
            "Sha256SmallSigma1O0: {}",
            indented_component_display(&self.sha_256_small_sigma_1_o_0)
        )?;
        writeln!(
            f,
            "Sha256SmallSigma1O1: {}",
            indented_component_display(&self.sha_256_small_sigma_1_o_1)
        )?;
        writeln!(
            f,
            "Sha256KTable: {}",
            indented_component_display(&self.sha_256_k_table)
        )?;

        Ok(())
    }
}

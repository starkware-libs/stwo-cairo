use std::collections::HashMap;

use num_traits::Zero;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::qm31::QM31;

use crate::air::{accumulate_relation_uses, CairoInteractionElements};
use crate::components::prelude::*;
use crate::components::{indented_component_display, partial_ec_mul, pedersen_points_table};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PedersenContextClaim {
    pub claim: Option<Claim>,
}
impl PedersenContextClaim {
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

    pub fn accumulate_relation_uses(&self, relation_counts: &mut HashMap<&'static str, u32>) {
        if let Some(claim) = &self.claim {
            claim.accumulate_relation_uses(relation_counts);
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub partial_ec_mul: partial_ec_mul::Claim,
    pub pedersen_points_table: pedersen_points_table::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.partial_ec_mul.mix_into(channel);
        self.pedersen_points_table.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.partial_ec_mul.log_sizes(),
            self.pedersen_points_table.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }

    pub fn accumulate_relation_uses(&self, relation_counts: &mut HashMap<&'static str, u32>) {
        let Self {
            partial_ec_mul,
            pedersen_points_table: _,
        } = self;

        // NOTE: The following components do not USE relations:
        // - pedersen_points_table

        accumulate_relation_uses(
            relation_counts,
            partial_ec_mul::RELATION_USES_PER_ROW,
            partial_ec_mul.log_size,
        );
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PedersenContextInteractionClaim {
    pub claim: Option<InteractionClaim>,
}
impl PedersenContextInteractionClaim {
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

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct InteractionClaim {
    pub partial_ec_mul: partial_ec_mul::InteractionClaim,
    pub pedersen_points_table: pedersen_points_table::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.partial_ec_mul.mix_into(channel);
        self.pedersen_points_table.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        let mut sum = QM31::zero();
        sum += self.partial_ec_mul.claimed_sum;
        sum += self.pedersen_points_table.claimed_sum;
        sum
    }
}

pub struct PedersenContextComponents {
    pub components: Option<Components>,
}
impl PedersenContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PedersenContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &PedersenContextInteractionClaim,
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

impl std::fmt::Display for PedersenContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{}", components),
            None => write!(f, "No Pedersen Context Components"),
        }
    }
}

pub struct Components {
    pub partial_ec_mul: partial_ec_mul::Component,
    pub pedersen_points_table: pedersen_points_table::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PedersenContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let partial_ec_mul_component = partial_ec_mul::Component::new(
            tree_span_provider,
            partial_ec_mul::Eval {
                claim: claim.claim.as_ref().unwrap().partial_ec_mul,
                partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
                pedersen_points_table_lookup_elements: interaction_elements
                    .pedersen_points_table
                    .clone(),
                range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
                range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            },
            interaction_claim.partial_ec_mul.claimed_sum,
        );

        let pedersen_points_table_component = pedersen_points_table::Component::new(
            tree_span_provider,
            pedersen_points_table::Eval {
                claim: claim.claim.as_ref().unwrap().pedersen_points_table,
                pedersen_points_table_lookup_elements: interaction_elements
                    .pedersen_points_table
                    .clone(),
            },
            interaction_claim.pedersen_points_table.claimed_sum,
        );

        Self {
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![&self.partial_ec_mul, &self.pedersen_points_table]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "PartialEcMul: {}",
            indented_component_display(&self.partial_ec_mul)
        )?;
        writeln!(
            f,
            "PedersenPointsTable: {}",
            indented_component_display(&self.pedersen_points_table)
        )?;

        Ok(())
    }
}

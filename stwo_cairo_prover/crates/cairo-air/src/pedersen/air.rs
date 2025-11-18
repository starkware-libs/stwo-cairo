use num_traits::Zero;
use stwo::core::fields::qm31::QM31;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::air::{accumulate_relation_uses, CairoInteractionElements, RelationUsesDict};
use crate::components::prelude::*;
use crate::components::{
    indented_component_display, partial_ec_mul, pedersen_aggregator, pedersen_points_table,
};

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        if let Some(claim) = &self.claim {
            claim.accumulate_relation_uses(relation_uses);
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub pedersen_aggregator: pedersen_aggregator::Claim,
    pub partial_ec_mul: partial_ec_mul::Claim,
    pub pedersen_points_table: pedersen_points_table::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.pedersen_aggregator.mix_into(channel);
        self.partial_ec_mul.mix_into(channel);
        self.pedersen_points_table.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.pedersen_aggregator.log_sizes(),
            self.partial_ec_mul.log_sizes(),
            self.pedersen_points_table.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            pedersen_aggregator,
            partial_ec_mul,
            pedersen_points_table: _,
        } = self;

        // NOTE: The following components do not USE relations:
        // - pedersen_points_table

        accumulate_relation_uses(
            relation_uses,
            pedersen_aggregator::RELATION_USES_PER_ROW,
            pedersen_aggregator.log_size,
        );

        accumulate_relation_uses(
            relation_uses,
            partial_ec_mul::RELATION_USES_PER_ROW,
            partial_ec_mul.log_size,
        );
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub pedersen_aggregator: pedersen_aggregator::InteractionClaim,
    pub partial_ec_mul: partial_ec_mul::InteractionClaim,
    pub pedersen_points_table: pedersen_points_table::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.pedersen_aggregator.mix_into(channel);
        self.partial_ec_mul.mix_into(channel);
        self.pedersen_points_table.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        let mut sum = QM31::zero();
        sum += self.pedersen_aggregator.claimed_sum;
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
            Some(components) => write!(f, "{components}"),
            None => write!(f, "No Pedersen Context Components"),
        }
    }
}

pub struct Components {
    pub pedersen_aggregator: pedersen_aggregator::Component,
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
        let pedersen_aggregator_component = pedersen_aggregator::Component::new(
            tree_span_provider,
            pedersen_aggregator::Eval {
                claim: claim.claim.as_ref().unwrap().pedersen_aggregator,
                pedersen_aggregator_lookup_elements: interaction_elements
                    .pedersen_aggregator
                    .clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
                pedersen_points_table_lookup_elements: interaction_elements
                    .pedersen_points_table
                    .clone(),
                range_check_5_4_lookup_elements: interaction_elements.range_checks.rc_5_4.clone(),
                range_check_8_lookup_elements: interaction_elements.range_checks.rc_8.clone(),
            },
            interaction_claim.pedersen_aggregator.claimed_sum,
        );
        let partial_ec_mul_component = partial_ec_mul::Component::new(
            tree_span_provider,
            partial_ec_mul::Eval {
                claim: claim.claim.as_ref().unwrap().partial_ec_mul,
                partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
                pedersen_points_table_lookup_elements: interaction_elements
                    .pedersen_points_table
                    .clone(),
                range_check_20_lookup_elements: interaction_elements.range_checks.rc_20.clone(),
                range_check_20_b_lookup_elements: interaction_elements.range_checks.rc_20_b.clone(),
                range_check_20_c_lookup_elements: interaction_elements.range_checks.rc_20_c.clone(),
                range_check_20_d_lookup_elements: interaction_elements.range_checks.rc_20_d.clone(),
                range_check_20_e_lookup_elements: interaction_elements.range_checks.rc_20_e.clone(),
                range_check_20_f_lookup_elements: interaction_elements.range_checks.rc_20_f.clone(),
                range_check_20_g_lookup_elements: interaction_elements.range_checks.rc_20_g.clone(),
                range_check_20_h_lookup_elements: interaction_elements.range_checks.rc_20_h.clone(),
                range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
                range_check_9_9_b_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_b
                    .clone(),
                range_check_9_9_c_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_c
                    .clone(),
                range_check_9_9_d_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_d
                    .clone(),
                range_check_9_9_e_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_e
                    .clone(),
                range_check_9_9_f_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_f
                    .clone(),
                range_check_9_9_g_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_g
                    .clone(),
                range_check_9_9_h_lookup_elements: interaction_elements
                    .range_checks
                    .rc_9_9_h
                    .clone(),
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
            "PedersenAggregator: {}",
            indented_component_display(&self.pedersen_aggregator)
        )?;
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

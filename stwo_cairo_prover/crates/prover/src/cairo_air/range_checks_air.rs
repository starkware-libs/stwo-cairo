use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};

use super::air::CairoInteractionElements;
use super::debug_tools::indented_component_display;
use crate::components::range_check_vector::{
    range_check_11, range_check_12, range_check_18, range_check_19, range_check_3_6,
    range_check_3_6_6_3, range_check_4_3, range_check_7_2_5, range_check_9_9,
};
use crate::relations;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct RangeChecksClaim {
    pub range_check_11: range_check_11::Claim,
    pub range_check_12: range_check_12::Claim,
    pub range_check_18: range_check_18::Claim,
    pub range_check_19: range_check_19::Claim,
    pub range_check_3_6: range_check_3_6::Claim,
    pub range_check4_3: range_check_4_3::Claim,
    pub range_check9_9: range_check_9_9::Claim,
    pub range_check7_2_5: range_check_7_2_5::Claim,
    pub range_check_3_6_6_3: range_check_3_6_6_3::Claim,
}
impl RangeChecksClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.range_check_11.mix_into(channel);
        self.range_check_12.mix_into(channel);
        self.range_check_18.mix_into(channel);
        self.range_check_19.mix_into(channel);
        self.range_check_3_6.mix_into(channel);
        self.range_check4_3.mix_into(channel);
        self.range_check9_9.mix_into(channel);
        self.range_check7_2_5.mix_into(channel);
        self.range_check_3_6_6_3.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(
            vec![
                self.range_check_11.log_sizes(),
                self.range_check_12.log_sizes(),
                self.range_check_18.log_sizes(),
                self.range_check_19.log_sizes(),
                self.range_check_3_6.log_sizes(),
                self.range_check4_3.log_sizes(),
                self.range_check9_9.log_sizes(),
                self.range_check7_2_5.log_sizes(),
                self.range_check_3_6_6_3.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

pub struct RangeChecksClaimGenerator {
    pub range_check_11_trace_generator: range_check_11::ClaimGenerator,
    pub range_check_12_trace_generator: range_check_12::ClaimGenerator,
    pub range_check_18_trace_generator: range_check_18::ClaimGenerator,
    pub range_check_19_trace_generator: range_check_19::ClaimGenerator,
    pub range_check_3_6_trace_generator: range_check_3_6::ClaimGenerator,
    pub range_check_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    pub range_check_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    pub range_check_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator,
    pub range_check_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator,
}
impl Default for RangeChecksClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeChecksClaimGenerator {
    pub fn new() -> Self {
        Self {
            range_check_11_trace_generator: range_check_11::ClaimGenerator::new(),
            range_check_12_trace_generator: range_check_12::ClaimGenerator::new(),
            range_check_18_trace_generator: range_check_18::ClaimGenerator::new(),
            range_check_19_trace_generator: range_check_19::ClaimGenerator::new(),
            range_check_3_6_trace_generator: range_check_3_6::ClaimGenerator::new(),
            range_check_4_3_trace_generator: range_check_4_3::ClaimGenerator::new(),
            range_check_9_9_trace_generator: range_check_9_9::ClaimGenerator::new(),
            range_check_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator::new(),
            range_check_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator::new(),
        }
    }
    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (RangeChecksClaim, RangeChecksInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (range_check_11_claim, range_check_11_interaction_gen) = self
            .range_check_11_trace_generator
            .write_trace(tree_builder);
        let (range_check_12_claim, range_check_12_interaction_gen) = self
            .range_check_12_trace_generator
            .write_trace(tree_builder);
        let (range_check_18_claim, range_check_18_interaction_gen) = self
            .range_check_18_trace_generator
            .write_trace(tree_builder);
        let (range_check_19_claim, range_check_19_interaction_gen) = self
            .range_check_19_trace_generator
            .write_trace(tree_builder);
        let (range_check_3_6_claim, range_check_3_6_interaction_gen) = self
            .range_check_3_6_trace_generator
            .write_trace(tree_builder);
        let (range_check_4_3_claim, range_check_4_3_interaction_gen) = self
            .range_check_4_3_trace_generator
            .write_trace(tree_builder);
        let (range_check9_9_claim, range_check_9_9_interaction_gen) = self
            .range_check_9_9_trace_generator
            .write_trace(tree_builder);
        let (range_check_7_2_5_claim, range_check_7_2_5_interaction_gen) = self
            .range_check_7_2_5_trace_generator
            .write_trace(tree_builder);
        let (range_check_3_6_6_3_claim, range_check_3_6_6_3_interaction_gen) = self
            .range_check_3_6_6_3_trace_generator
            .write_trace(tree_builder);
        (
            RangeChecksClaim {
                range_check_11: range_check_11_claim,
                range_check_12: range_check_12_claim,
                range_check_18: range_check_18_claim,
                range_check_19: range_check_19_claim,
                range_check_3_6: range_check_3_6_claim,
                range_check4_3: range_check_4_3_claim,
                range_check9_9: range_check9_9_claim,
                range_check7_2_5: range_check_7_2_5_claim,
                range_check_3_6_6_3: range_check_3_6_6_3_claim,
            },
            RangeChecksInteractionClaimGenerator {
                range_check_11_interaction_gen,
                range_check_12_interaction_gen,
                range_check_18_interaction_gen,
                range_check_19_interaction_gen,
                range_check_3_6_interaction_gen,
                range_check_4_3_interaction_gen,
                range_check_9_9_interaction_gen,
                range_check_7_2_5_interaction_gen,
                range_check_3_6_6_3_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct RangeChecksInteractionClaim {
    pub range_check_11: range_check_11::InteractionClaim,
    pub range_check_12: range_check_12::InteractionClaim,
    pub range_check_18: range_check_18::InteractionClaim,
    pub range_check_19: range_check_19::InteractionClaim,
    pub range_check_3_6: range_check_3_6::InteractionClaim,
    pub range_check_4_3: range_check_4_3::InteractionClaim,
    pub range_check_9_9: range_check_9_9::InteractionClaim,
    pub range_check_7_2_5: range_check_7_2_5::InteractionClaim,
    pub range_check_3_6_6_3: range_check_3_6_6_3::InteractionClaim,
}
impl RangeChecksInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.range_check_11.mix_into(channel);
        self.range_check_12.mix_into(channel);
        self.range_check_18.mix_into(channel);
        self.range_check_19.mix_into(channel);
        self.range_check_3_6.mix_into(channel);
        self.range_check_4_3.mix_into(channel);
        self.range_check_9_9.mix_into(channel);
        self.range_check_7_2_5.mix_into(channel);
        self.range_check_3_6_6_3.mix_into(channel);
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        sum += self.range_check_11.claimed_sum;
        sum += self.range_check_12.claimed_sum;
        sum += self.range_check_18.claimed_sum;
        sum += self.range_check_19.claimed_sum;
        sum += self.range_check_3_6.claimed_sum;
        sum += self.range_check_4_3.claimed_sum;
        sum += self.range_check_9_9.claimed_sum;
        sum += self.range_check_7_2_5.claimed_sum;
        sum += self.range_check_3_6_6_3.claimed_sum;
        sum
    }
}

pub struct RangeChecksInteractionClaimGenerator {
    range_check_11_interaction_gen: range_check_11::InteractionClaimGenerator,
    range_check_12_interaction_gen: range_check_12::InteractionClaimGenerator,
    range_check_18_interaction_gen: range_check_18::InteractionClaimGenerator,
    range_check_19_interaction_gen: range_check_19::InteractionClaimGenerator,
    range_check_3_6_interaction_gen: range_check_3_6::InteractionClaimGenerator,
    range_check_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    range_check_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    range_check_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    range_check_3_6_6_3_interaction_gen: range_check_3_6_6_3::InteractionClaimGenerator,
}
impl RangeChecksInteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> RangeChecksInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let range_check_11_interaction_claim = self
            .range_check_11_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_11);
        let range_check_12_interaction_claim = self
            .range_check_12_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_12);
        let range_check_18_interaction_claim = self
            .range_check_18_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_18);
        let range_check_19_interaction_claim = self
            .range_check_19_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_19);
        let range_check_3_6_interaction_claim = self
            .range_check_3_6_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_3_6);
        let range_check_4_3_interaction_claim = self
            .range_check_4_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_4_3);
        let range_check9_9_interaction_claim = self
            .range_check_9_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_9_9);
        let range_check_7_2_5_interaction_claim = self
            .range_check_7_2_5_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_7_2_5);
        let range_check_3_6_6_3_interaction_claim = self
            .range_check_3_6_6_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_3_6_6_3);
        RangeChecksInteractionClaim {
            range_check_11: range_check_11_interaction_claim,
            range_check_12: range_check_12_interaction_claim,
            range_check_18: range_check_18_interaction_claim,
            range_check_19: range_check_19_interaction_claim,
            range_check_3_6: range_check_3_6_interaction_claim,
            range_check_4_3: range_check_4_3_interaction_claim,
            range_check_9_9: range_check9_9_interaction_claim,
            range_check_7_2_5: range_check_7_2_5_interaction_claim,
            range_check_3_6_6_3: range_check_3_6_6_3_interaction_claim,
        }
    }
}

pub struct RangeChecksInteractionElements {
    pub range_check_11: relations::RangeCheck_11,
    pub range_check_12: relations::RangeCheck_12,
    pub range_check_18: relations::RangeCheck_18,
    pub range_check_19: relations::RangeCheck_19,
    pub range_check_3_6: relations::RangeCheck_3_6,
    pub range_check_4_3: relations::RangeCheck_4_3,
    pub range_check_9_9: relations::RangeCheck_9_9,
    pub range_check_7_2_5: relations::RangeCheck_7_2_5,
    pub range_check_3_6_6_3: relations::RangeCheck_3_6_6_3,
    // ...
}
impl RangeChecksInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> RangeChecksInteractionElements {
        RangeChecksInteractionElements {
            range_check_11: relations::RangeCheck_11::draw(channel),
            range_check_12: relations::RangeCheck_12::draw(channel),
            range_check_18: relations::RangeCheck_18::draw(channel),
            range_check_19: relations::RangeCheck_19::draw(channel),
            range_check_3_6: relations::RangeCheck_3_6::draw(channel),
            range_check_4_3: relations::RangeCheck_4_3::draw(channel),
            range_check_9_9: relations::RangeCheck_9_9::draw(channel),
            range_check_7_2_5: relations::RangeCheck_7_2_5::draw(channel),
            range_check_3_6_6_3: relations::RangeCheck_3_6_6_3::draw(channel),
        }
    }
}

pub struct RangeChecksComponents {
    range_check_11: range_check_11::Component,
    range_check_12: range_check_12::Component,
    range_check_18: range_check_18::Component,
    range_check_19: range_check_19::Component,
    range_check_3_6: range_check_3_6::Component,
    range_check4_3: range_check_4_3::Component,
    range_check9_9: range_check_9_9::Component,
    range_check7_2_5: range_check_7_2_5::Component,
    range_check_3_6_6_3: range_check_3_6_6_3::Component,
}
impl RangeChecksComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &RangeChecksInteractionClaim,
    ) -> Self {
        let range_check_11_component = range_check_11::Component::new(
            tree_span_provider,
            range_check_11::Eval::new(interaction_elements.range_check_11.clone()),
            (interaction_claim.range_check_11.claimed_sum, None),
        );
        let range_check_12_component = range_check_12::Component::new(
            tree_span_provider,
            range_check_12::Eval::new(interaction_elements.range_check_12.clone()),
            (interaction_claim.range_check_12.claimed_sum, None),
        );
        let range_check_18_component = range_check_18::Component::new(
            tree_span_provider,
            range_check_18::Eval::new(interaction_elements.range_check_18.clone()),
            (interaction_claim.range_check_18.claimed_sum, None),
        );
        let range_check_19_component = range_check_19::Component::new(
            tree_span_provider,
            range_check_19::Eval::new(interaction_elements.range_check_19.clone()),
            (interaction_claim.range_check_19.claimed_sum, None),
        );
        let range_check_3_6_component = range_check_3_6::Component::new(
            tree_span_provider,
            range_check_3_6::Eval::new(interaction_elements.range_check_3_6.clone()),
            (interaction_claim.range_check_3_6.claimed_sum, None),
        );
        let range_check_4_3_component = range_check_4_3::Component::new(
            tree_span_provider,
            range_check_4_3::Eval::new(interaction_elements.range_check_4_3.clone()),
            (interaction_claim.range_check_4_3.claimed_sum, None),
        );
        let range_check9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval::new(interaction_elements.range_check_9_9.clone()),
            (interaction_claim.range_check_9_9.claimed_sum, None),
        );
        let range_check_7_2_5_component = range_check_7_2_5::Component::new(
            tree_span_provider,
            range_check_7_2_5::Eval::new(interaction_elements.range_check_7_2_5.clone()),
            (interaction_claim.range_check_7_2_5.claimed_sum, None),
        );
        let range_check_3_6_6_3_component = range_check_3_6_6_3::Component::new(
            tree_span_provider,
            range_check_3_6_6_3::Eval::new(interaction_elements.range_check_3_6_6_3.clone()),
            (interaction_claim.range_check_3_6_6_3.claimed_sum, None),
        );
        Self {
            range_check_11: range_check_11_component,
            range_check_12: range_check_12_component,
            range_check_18: range_check_18_component,
            range_check_19: range_check_19_component,
            range_check_3_6: range_check_3_6_component,
            range_check4_3: range_check_4_3_component,
            range_check9_9: range_check9_9_component,
            range_check7_2_5: range_check_7_2_5_component,
            range_check_3_6_6_3: range_check_3_6_6_3_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.range_check_11 as &dyn ComponentProver<SimdBackend>,
            &self.range_check_12 as &dyn ComponentProver<SimdBackend>,
            &self.range_check_18 as &dyn ComponentProver<SimdBackend>,
            &self.range_check_19 as &dyn ComponentProver<SimdBackend>,
            &self.range_check_3_6 as &dyn ComponentProver<SimdBackend>,
            &self.range_check4_3 as &dyn ComponentProver<SimdBackend>,
            &self.range_check9_9 as &dyn ComponentProver<SimdBackend>,
            &self.range_check7_2_5 as &dyn ComponentProver<SimdBackend>,
            &self.range_check_3_6_6_3 as &dyn ComponentProver<SimdBackend>,
        ]
    }
}

impl std::fmt::Display for RangeChecksComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "RangeCheck11: {}",
            indented_component_display(&self.range_check_11)
        )?;
        writeln!(
            f,
            "RangeCheck12: {}",
            indented_component_display(&self.range_check_12)
        )?;
        writeln!(
            f,
            "RangeCheck18: {}",
            indented_component_display(&self.range_check_18)
        )?;
        writeln!(
            f,
            "RangeCheck19: {}",
            indented_component_display(&self.range_check_19)
        )?;
        writeln!(
            f,
            "RangeCheck3_6: {}",
            indented_component_display(&self.range_check_3_6)
        )?;
        writeln!(
            f,
            "RangeCheck4_3: {}",
            indented_component_display(&self.range_check4_3)
        )?;
        writeln!(
            f,
            "RangeCheck9_9: {}",
            indented_component_display(&self.range_check9_9)
        )?;
        writeln!(
            f,
            "RangeCheck7_2_5: {}",
            indented_component_display(&self.range_check7_2_5)
        )?;
        writeln!(
            f,
            "RangeCheck3_6_6_3: {}",
            indented_component_display(&self.range_check_3_6_6_3)
        )?;
        Ok(())
    }
}

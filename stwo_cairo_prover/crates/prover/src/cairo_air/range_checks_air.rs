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

use super::debug_tools::indented_component_display;
use crate::components::range_check_vector::{
    range_check_11, range_check_19, range_check_4_3, range_check_7_2_5, range_check_9_9,
};
use crate::relations;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct RangeChecksClaim {
    pub rc_11: range_check_11::Claim,
    pub rc_19: range_check_19::Claim,
    pub rc_4_3: range_check_4_3::Claim,
    pub rc_9_9: range_check_9_9::Claim,
    pub rc_7_2_5: range_check_7_2_5::Claim,
}
impl RangeChecksClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.rc_11.mix_into(channel);
        self.rc_19.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_9_9.mix_into(channel);
        self.rc_7_2_5.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(
            vec![
                self.rc_11.log_sizes(),
                self.rc_19.log_sizes(),
                self.rc_4_3.log_sizes(),
                self.rc_9_9.log_sizes(),
                self.rc_7_2_5.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

pub struct RangeChecksClaimGenerator {
    pub rc_11_trace_generator: range_check_11::ClaimGenerator,
    pub rc_19_trace_generator: range_check_19::ClaimGenerator,
    pub rc_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    pub rc_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    pub rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator,
}
impl Default for RangeChecksClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeChecksClaimGenerator {
    pub fn new() -> Self {
        Self {
            rc_11_trace_generator: range_check_11::ClaimGenerator::new(),
            rc_19_trace_generator: range_check_19::ClaimGenerator::new(),
            rc_4_3_trace_generator: range_check_4_3::ClaimGenerator::new(),
            rc_9_9_trace_generator: range_check_9_9::ClaimGenerator::new(),
            rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator::new(),
        }
    }
    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (RangeChecksClaim, RangeChecksInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (rc_11_claim, rc_11_interaction_gen) =
            self.rc_11_trace_generator.write_trace(tree_builder);
        let (rc_19_claim, rc_19_interaction_gen) =
            self.rc_19_trace_generator.write_trace(tree_builder);
        let (rc_4_3_claim, rc_4_3_interaction_gen) =
            self.rc_4_3_trace_generator.write_trace(tree_builder);
        let (rc_9_9_claim, rc_9_9_interaction_gen) =
            self.rc_9_9_trace_generator.write_trace(tree_builder);
        let (rc_7_2_5_claim, rc_7_2_5_interaction_gen) =
            self.rc_7_2_5_trace_generator.write_trace(tree_builder);
        (
            RangeChecksClaim {
                rc_11: rc_11_claim,
                rc_19: rc_19_claim,
                rc_4_3: rc_4_3_claim,
                rc_9_9: rc_9_9_claim,
                rc_7_2_5: rc_7_2_5_claim,
            },
            RangeChecksInteractionClaimGenerator {
                rc_11_interaction_gen,
                rc_19_interaction_gen,
                rc_4_3_interaction_gen,
                rc_9_9_interaction_gen,
                rc_7_2_5_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct RangeChecksInteractionClaim {
    pub rc_11: range_check_11::InteractionClaim,
    pub rc_19: range_check_19::InteractionClaim,
    pub rc_4_3: range_check_4_3::InteractionClaim,
    pub rc_9_9: range_check_9_9::InteractionClaim,
    pub rc_7_2_5: range_check_7_2_5::InteractionClaim,
}
impl RangeChecksInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.rc_11.mix_into(channel);
        self.rc_19.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_9_9.mix_into(channel);
        self.rc_7_2_5.mix_into(channel);
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        sum += self.rc_11.claimed_sum;
        sum += self.rc_19.claimed_sum;
        sum += self.rc_4_3.claimed_sum;
        sum += self.rc_9_9.claimed_sum;
        sum += self.rc_7_2_5.claimed_sum;
        sum
    }
}

pub struct RangeChecksInteractionClaimGenerator {
    rc_11_interaction_gen: range_check_11::InteractionClaimGenerator,
    rc_19_interaction_gen: range_check_19::InteractionClaimGenerator,
    rc_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    rc_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    rc_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
}
impl RangeChecksInteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &RangeChecksInteractionElements,
    ) -> RangeChecksInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let rc_11_interaction_claim = self
            .rc_11_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_11);
        let rc_19_interaction_claim = self
            .rc_19_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
        let rc_4_3_interaction_claim = self
            .rc_4_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_4_3);
        let rc_9_9_interaction_claim = self
            .rc_9_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_9_9);
        let rc_7_2_5_interaction_claim = self
            .rc_7_2_5_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_7_2_5);
        RangeChecksInteractionClaim {
            rc_11: rc_11_interaction_claim,
            rc_19: rc_19_interaction_claim,
            rc_4_3: rc_4_3_interaction_claim,
            rc_9_9: rc_9_9_interaction_claim,
            rc_7_2_5: rc_7_2_5_interaction_claim,
        }
    }
}

pub struct RangeChecksInteractionElements {
    pub rc_11: relations::RangeCheck_11,
    pub rc_19: relations::RangeCheck_19,
    pub rc_4_3: relations::RangeCheck_4_3,
    pub rc_9_9: relations::RangeCheck_9_9,
    pub rc_7_2_5: relations::RangeCheck_7_2_5,
    // ...
}
impl RangeChecksInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> RangeChecksInteractionElements {
        RangeChecksInteractionElements {
            rc_11: relations::RangeCheck_11::draw(channel),
            rc_19: relations::RangeCheck_19::draw(channel),
            rc_4_3: relations::RangeCheck_4_3::draw(channel),
            rc_9_9: relations::RangeCheck_9_9::draw(channel),
            rc_7_2_5: relations::RangeCheck_7_2_5::draw(channel),
        }
    }
}

pub struct RangeChecksComponents {
    rc_11: range_check_11::Component,
    rc_19: range_check_19::Component,
    rc_4_3: range_check_4_3::Component,
    rc_9_9: range_check_9_9::Component,
    rc_7_2_5: range_check_7_2_5::Component,
}
impl RangeChecksComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        interaction_elements: &RangeChecksInteractionElements,
        interaction_claim: &RangeChecksInteractionClaim,
    ) -> Self {
        let rc_11_component = range_check_11::Component::new(
            tree_span_provider,
            range_check_11::Eval::new(interaction_elements.rc_11.clone()),
            (interaction_claim.rc_11.claimed_sum, None),
        );
        let rc_19_component = range_check_19::Component::new(
            tree_span_provider,
            range_check_19::Eval::new(interaction_elements.rc_19.clone()),
            (interaction_claim.rc_19.claimed_sum, None),
        );
        let rc_4_3_component = range_check_4_3::Component::new(
            tree_span_provider,
            range_check_4_3::Eval::new(interaction_elements.rc_4_3.clone()),
            (interaction_claim.rc_4_3.claimed_sum, None),
        );
        let rc_9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval::new(interaction_elements.rc_9_9.clone()),
            (interaction_claim.rc_9_9.claimed_sum, None),
        );
        let rc_7_2_5_component = range_check_7_2_5::Component::new(
            tree_span_provider,
            range_check_7_2_5::Eval::new(interaction_elements.rc_7_2_5.clone()),
            (interaction_claim.rc_7_2_5.claimed_sum, None),
        );
        Self {
            rc_11: rc_11_component,
            rc_19: rc_19_component,
            rc_4_3: rc_4_3_component,
            rc_9_9: rc_9_9_component,
            rc_7_2_5: rc_7_2_5_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.rc_11 as &dyn ComponentProver<SimdBackend>,
            &self.rc_19 as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_3 as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9 as &dyn ComponentProver<SimdBackend>,
            &self.rc_7_2_5 as &dyn ComponentProver<SimdBackend>,
        ]
    }
}

impl std::fmt::Display for RangeChecksComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "RangeCheck11: {}",
            indented_component_display(&self.rc_11)
        )?;
        writeln!(
            f,
            "RangeCheck19: {}",
            indented_component_display(&self.rc_19)
        )?;
        writeln!(
            f,
            "RangeCheck4_3: {}",
            indented_component_display(&self.rc_4_3)
        )?;
        writeln!(
            f,
            "RangeCheck9_9: {}",
            indented_component_display(&self.rc_9_9)
        )?;
        writeln!(
            f,
            "RangeCheck7_2_5: {}",
            indented_component_display(&self.rc_7_2_5)
        )?;
        Ok(())
    }
}

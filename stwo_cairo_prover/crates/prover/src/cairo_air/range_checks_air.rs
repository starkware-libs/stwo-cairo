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
use crate::cairo_air::relations;
use crate::components::range_check_vector::{
    range_check_11, range_check_12, range_check_18, range_check_19, range_check_3_3_3_3_3,
    range_check_3_6, range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4,
    range_check_5_4, range_check_6, range_check_7_2_5, range_check_8, range_check_9_9,
};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct RangeChecksClaim {
    //
    pub rc_6: range_check_6::Claim,
    pub rc_8: range_check_8::Claim,
    pub rc_11: range_check_11::Claim,
    pub rc_12: range_check_12::Claim,
    pub rc_18: range_check_18::Claim,
    pub rc_19: range_check_19::Claim,
    pub rc_3_6: range_check_3_6::Claim,
    pub rc_4_3: range_check_4_3::Claim,
    pub rc_4_4: range_check_4_4::Claim,
    pub rc_5_4: range_check_5_4::Claim, //
    pub rc_9_9: range_check_9_9::Claim,
    pub rc_7_2_5: range_check_7_2_5::Claim,
    pub rc_3_6_6_3: range_check_3_6_6_3::Claim,
    pub rc_4_4_4_4: range_check_4_4_4_4::Claim,
    pub rc_3_3_3_3_3: range_check_3_3_3_3_3::Claim,
}
impl RangeChecksClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.rc_6.mix_into(channel);
        self.rc_8.mix_into(channel);
        self.rc_11.mix_into(channel);
        self.rc_12.mix_into(channel);
        self.rc_18.mix_into(channel);
        self.rc_19.mix_into(channel);
        self.rc_3_6.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_4_4.mix_into(channel);
        self.rc_5_4.mix_into(channel); //
        self.rc_9_9.mix_into(channel);
        self.rc_7_2_5.mix_into(channel);
        self.rc_3_6_6_3.mix_into(channel);
        self.rc_4_4_4_4.mix_into(channel);
        self.rc_3_3_3_3_3.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        TreeVec::concat_cols(
            vec![
                self.rc_6.log_sizes(),
                self.rc_8.log_sizes(),
                self.rc_11.log_sizes(),
                self.rc_12.log_sizes(),
                self.rc_18.log_sizes(),
                self.rc_19.log_sizes(),
                self.rc_3_6.log_sizes(),
                self.rc_4_3.log_sizes(),
                self.rc_4_4.log_sizes(),
                self.rc_5_4.log_sizes(),
                self.rc_9_9.log_sizes(),
                self.rc_7_2_5.log_sizes(),
                self.rc_3_6_6_3.log_sizes(),
                self.rc_4_4_4_4.log_sizes(),
                self.rc_3_3_3_3_3.log_sizes(),
            ]
            .into_iter(),
        )
    }
}

pub struct RangeChecksClaimGenerator {
    pub rc_6_trace_generator: range_check_6::ClaimGenerator,
    pub rc_8_trace_generator: range_check_8::ClaimGenerator,
    pub rc_11_trace_generator: range_check_11::ClaimGenerator,
    pub rc_12_trace_generator: range_check_12::ClaimGenerator,
    pub rc_18_trace_generator: range_check_18::ClaimGenerator,
    pub rc_19_trace_generator: range_check_19::ClaimGenerator,
    pub rc_3_6_trace_generator: range_check_3_6::ClaimGenerator,
    pub rc_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    pub rc_4_4_trace_generator: range_check_4_4::ClaimGenerator,
    pub rc_5_4_trace_generator: range_check_5_4::ClaimGenerator,
    pub rc_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    pub rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator,
    pub rc_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator,
    pub rc_4_4_4_4_trace_generator: range_check_4_4_4_4::ClaimGenerator,
    pub rc_3_3_3_3_3_trace_generator: range_check_3_3_3_3_3::ClaimGenerator,
}
impl Default for RangeChecksClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeChecksClaimGenerator {
    pub fn new() -> Self {
        Self {
            rc_6_trace_generator: range_check_6::ClaimGenerator::new(),
            rc_8_trace_generator: range_check_8::ClaimGenerator::new(),
            rc_11_trace_generator: range_check_11::ClaimGenerator::new(),
            rc_12_trace_generator: range_check_12::ClaimGenerator::new(),
            rc_18_trace_generator: range_check_18::ClaimGenerator::new(),
            rc_19_trace_generator: range_check_19::ClaimGenerator::new(),
            rc_3_6_trace_generator: range_check_3_6::ClaimGenerator::new(),
            rc_4_3_trace_generator: range_check_4_3::ClaimGenerator::new(),
            rc_4_4_trace_generator: range_check_4_4::ClaimGenerator::new(),
            rc_5_4_trace_generator: range_check_5_4::ClaimGenerator::new(),
            rc_9_9_trace_generator: range_check_9_9::ClaimGenerator::new(),
            rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator::new(),
            rc_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator::new(),
            rc_4_4_4_4_trace_generator: range_check_4_4_4_4::ClaimGenerator::new(),
            rc_3_3_3_3_3_trace_generator: range_check_3_3_3_3_3::ClaimGenerator::new(),
        }
    }
    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (RangeChecksClaim, RangeChecksInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (rc_6_claim, rc_6_interaction_gen) =
            self.rc_6_trace_generator.write_trace(tree_builder);
        let (rc_8_claim, rc_8_interaction_gen) =
            self.rc_8_trace_generator.write_trace(tree_builder);
        let (rc_11_claim, rc_11_interaction_gen) =
            self.rc_11_trace_generator.write_trace(tree_builder);
        let (rc_12_claim, rc_12_interaction_gen) =
            self.rc_12_trace_generator.write_trace(tree_builder);
        let (rc_18_claim, rc_18_interaction_gen) =
            self.rc_18_trace_generator.write_trace(tree_builder);
        let (rc_19_claim, rc_19_interaction_gen) =
            self.rc_19_trace_generator.write_trace(tree_builder);
        let (rc_3_6_claim, rc_3_6_interaction_gen) =
            self.rc_3_6_trace_generator.write_trace(tree_builder);
        let (rc_4_3_claim, rc_4_3_interaction_gen) =
            self.rc_4_3_trace_generator.write_trace(tree_builder);
        let (rc_4_4_claim, rc_4_4_interaction_gen) =
            self.rc_4_4_trace_generator.write_trace(tree_builder);
        let (rc_5_4_claim, rc_5_4_interaction_gen) =
            self.rc_5_4_trace_generator.write_trace(tree_builder);
        let (rc_9_9_claim, rc_9_9_interaction_gen) =
            self.rc_9_9_trace_generator.write_trace(tree_builder);
        let (rc_7_2_5_claim, rc_7_2_5_interaction_gen) =
            self.rc_7_2_5_trace_generator.write_trace(tree_builder);
        let (rc_3_6_6_3_claim, rc_3_6_6_3_interaction_gen) =
            self.rc_3_6_6_3_trace_generator.write_trace(tree_builder);
        let (rc_4_4_4_4_claim, rc_4_4_4_4_interaction_gen) =
            self.rc_4_4_4_4_trace_generator.write_trace(tree_builder);
        let (rc_3_3_3_3_3_claim, rc_3_3_3_3_3_interaction_gen) =
            self.rc_3_3_3_3_3_trace_generator.write_trace(tree_builder);
        (
            RangeChecksClaim {
                rc_6: rc_6_claim,
                rc_8: rc_8_claim,
                rc_11: rc_11_claim,
                rc_12: rc_12_claim,
                rc_18: rc_18_claim,
                rc_19: rc_19_claim,
                rc_3_6: rc_3_6_claim,
                rc_4_3: rc_4_3_claim,
                rc_4_4: rc_4_4_claim,
                rc_5_4: rc_5_4_claim,
                rc_9_9: rc_9_9_claim,
                rc_7_2_5: rc_7_2_5_claim,
                rc_3_6_6_3: rc_3_6_6_3_claim,
                rc_4_4_4_4: rc_4_4_4_4_claim,
                rc_3_3_3_3_3: rc_3_3_3_3_3_claim,
            },
            RangeChecksInteractionClaimGenerator {
                rc_6_interaction_gen,
                rc_8_interaction_gen,
                rc_11_interaction_gen,
                rc_12_interaction_gen,
                rc_18_interaction_gen,
                rc_19_interaction_gen,
                rc_3_6_interaction_gen,
                rc_4_3_interaction_gen,
                rc_4_4_interaction_gen,
                rc_5_4_interaction_gen,
                rc_9_9_interaction_gen,
                rc_7_2_5_interaction_gen,
                rc_3_6_6_3_interaction_gen,
                rc_4_4_4_4_interaction_gen,
                rc_3_3_3_3_3_interaction_gen,
            },
        )
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct RangeChecksInteractionClaim {
    pub rc_6: range_check_6::InteractionClaim,
    pub rc_8: range_check_8::InteractionClaim, //
    pub rc_11: range_check_11::InteractionClaim,
    pub rc_12: range_check_12::InteractionClaim,
    pub rc_18: range_check_18::InteractionClaim,
    pub rc_19: range_check_19::InteractionClaim,
    pub rc_3_6: range_check_3_6::InteractionClaim,
    pub rc_4_3: range_check_4_3::InteractionClaim,
    pub rc_4_4: range_check_4_4::InteractionClaim,
    pub rc_5_4: range_check_5_4::InteractionClaim, //
    pub rc_9_9: range_check_9_9::InteractionClaim,
    pub rc_7_2_5: range_check_7_2_5::InteractionClaim,
    pub rc_3_6_6_3: range_check_3_6_6_3::InteractionClaim,
    pub rc_4_4_4_4: range_check_4_4_4_4::InteractionClaim,
    pub rc_3_3_3_3_3: range_check_3_3_3_3_3::InteractionClaim,
}
impl RangeChecksInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.rc_6.mix_into(channel);
        self.rc_8.mix_into(channel);
        self.rc_11.mix_into(channel);
        self.rc_12.mix_into(channel);
        self.rc_18.mix_into(channel);
        self.rc_19.mix_into(channel);
        self.rc_3_6.mix_into(channel);
        self.rc_4_3.mix_into(channel);
        self.rc_4_4.mix_into(channel);
        self.rc_5_4.mix_into(channel);
        self.rc_9_9.mix_into(channel);
        self.rc_7_2_5.mix_into(channel);
        self.rc_3_6_6_3.mix_into(channel);
        self.rc_4_4_4_4.mix_into(channel);
        self.rc_3_3_3_3_3.mix_into(channel);
    }

    pub fn sum(&self) -> SecureField {
        let mut sum = QM31::zero();
        sum += self.rc_6.claimed_sum;
        sum += self.rc_8.claimed_sum;
        sum += self.rc_11.claimed_sum;
        sum += self.rc_12.claimed_sum;
        sum += self.rc_18.claimed_sum;
        sum += self.rc_19.claimed_sum;
        sum += self.rc_3_6.claimed_sum;
        sum += self.rc_4_3.claimed_sum;
        sum += self.rc_4_4.claimed_sum;
        sum += self.rc_5_4.claimed_sum;
        sum += self.rc_9_9.claimed_sum;
        sum += self.rc_7_2_5.claimed_sum;
        sum += self.rc_3_6_6_3.claimed_sum;
        sum += self.rc_4_4_4_4.claimed_sum;
        sum += self.rc_3_3_3_3_3.claimed_sum;
        sum
    }
}

pub struct RangeChecksInteractionClaimGenerator {
    rc_6_interaction_gen: range_check_6::InteractionClaimGenerator,
    rc_8_interaction_gen: range_check_8::InteractionClaimGenerator, //
    rc_11_interaction_gen: range_check_11::InteractionClaimGenerator,
    rc_12_interaction_gen: range_check_12::InteractionClaimGenerator,
    rc_18_interaction_gen: range_check_18::InteractionClaimGenerator,
    rc_19_interaction_gen: range_check_19::InteractionClaimGenerator,
    rc_3_6_interaction_gen: range_check_3_6::InteractionClaimGenerator,
    rc_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    rc_4_4_interaction_gen: range_check_4_4::InteractionClaimGenerator,
    rc_5_4_interaction_gen: range_check_5_4::InteractionClaimGenerator, //
    rc_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    rc_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    rc_3_6_6_3_interaction_gen: range_check_3_6_6_3::InteractionClaimGenerator,
    rc_4_4_4_4_interaction_gen: range_check_4_4_4_4::InteractionClaimGenerator,
    rc_3_3_3_3_3_interaction_gen: range_check_3_3_3_3_3::InteractionClaimGenerator,
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
        let rc_6_interaction_claim = self
            .rc_6_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_6);
        let rc_8_interaction_claim = self
            .rc_8_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_8);
        let rc_11_interaction_claim = self
            .rc_11_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_11);
        let rc_12_interaction_claim = self
            .rc_12_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_12);
        let rc_18_interaction_claim = self
            .rc_18_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_18);
        let rc_19_interaction_claim = self
            .rc_19_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
        let rc_3_6_interaction_claim = self
            .rc_3_6_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_3_6);
        let rc_4_3_interaction_claim = self
            .rc_4_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_4_3);
        let rc_4_4_interaction_claim = self
            .rc_4_4_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_4_4);
        let rc_5_4_interaction_claim = self
            .rc_5_4_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_5_4);
        let rc_9_9_interaction_claim = self
            .rc_9_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_9_9);
        let rc_7_2_5_interaction_claim = self
            .rc_7_2_5_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_7_2_5);
        let rc_3_6_6_3_interaction_claim = self
            .rc_3_6_6_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_3_6_6_3);
        let rc_4_4_4_4_interaction_claim = self
            .rc_4_4_4_4_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_4_4_4_4);
        let rc_3_3_3_3_3_interaction_claim = self
            .rc_3_3_3_3_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_3_3_3_3_3);
        RangeChecksInteractionClaim {
            rc_6: rc_6_interaction_claim,
            rc_8: rc_8_interaction_claim,
            rc_11: rc_11_interaction_claim,
            rc_12: rc_12_interaction_claim,
            rc_18: rc_18_interaction_claim,
            rc_19: rc_19_interaction_claim,
            rc_3_6: rc_3_6_interaction_claim,
            rc_4_3: rc_4_3_interaction_claim,
            rc_4_4: rc_4_4_interaction_claim,
            rc_5_4: rc_5_4_interaction_claim,
            rc_9_9: rc_9_9_interaction_claim,
            rc_7_2_5: rc_7_2_5_interaction_claim,
            rc_3_6_6_3: rc_3_6_6_3_interaction_claim,
            rc_4_4_4_4: rc_4_4_4_4_interaction_claim,
            rc_3_3_3_3_3: rc_3_3_3_3_3_interaction_claim,
        }
    }
}

pub struct RangeChecksInteractionElements {
    pub rc_6: relations::RangeCheck_6,
    pub rc_8: relations::RangeCheck_8,
    pub rc_11: relations::RangeCheck_11,
    pub rc_12: relations::RangeCheck_12,
    pub rc_18: relations::RangeCheck_18,
    pub rc_19: relations::RangeCheck_19,
    pub rc_3_6: relations::RangeCheck_3_6,
    pub rc_4_3: relations::RangeCheck_4_3,
    pub rc_4_4: relations::RangeCheck_4_4,
    pub rc_5_4: relations::RangeCheck_5_4,
    pub rc_9_9: relations::RangeCheck_9_9,
    pub rc_7_2_5: relations::RangeCheck_7_2_5,
    pub rc_3_6_6_3: relations::RangeCheck_3_6_6_3,
    pub rc_4_4_4_4: relations::RangeCheck_4_4_4_4,
    pub rc_3_3_3_3_3: relations::RangeCheck_3_3_3_3_3,
    // ...
}
impl RangeChecksInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> RangeChecksInteractionElements {
        RangeChecksInteractionElements {
            rc_6: relations::RangeCheck_6::draw(channel),
            rc_8: relations::RangeCheck_8::draw(channel),
            rc_11: relations::RangeCheck_11::draw(channel),
            rc_12: relations::RangeCheck_12::draw(channel),
            rc_18: relations::RangeCheck_18::draw(channel),
            rc_19: relations::RangeCheck_19::draw(channel),
            rc_3_6: relations::RangeCheck_3_6::draw(channel),
            rc_4_3: relations::RangeCheck_4_3::draw(channel),
            rc_4_4: relations::RangeCheck_4_4::draw(channel),
            rc_5_4: relations::RangeCheck_5_4::draw(channel),
            rc_9_9: relations::RangeCheck_9_9::draw(channel),
            rc_7_2_5: relations::RangeCheck_7_2_5::draw(channel),
            rc_3_6_6_3: relations::RangeCheck_3_6_6_3::draw(channel),
            rc_4_4_4_4: relations::RangeCheck_4_4_4_4::draw(channel),
            rc_3_3_3_3_3: relations::RangeCheck_3_3_3_3_3::draw(channel),
        }
    }
}

pub struct RangeChecksComponents {
    pub rc_6: range_check_6::Component,
    pub rc_8: range_check_8::Component,
    pub rc_11: range_check_11::Component,
    pub rc_12: range_check_12::Component,
    pub rc_18: range_check_18::Component,
    pub rc_19: range_check_19::Component,
    pub rc_3_6: range_check_3_6::Component,
    pub rc_4_3: range_check_4_3::Component,
    pub rc_4_4: range_check_4_4::Component,
    pub rc_5_4: range_check_5_4::Component,
    pub rc_9_9: range_check_9_9::Component,
    pub rc_7_2_5: range_check_7_2_5::Component,
    pub rc_3_6_6_3: range_check_3_6_6_3::Component,
    pub rc_4_4_4_4: range_check_4_4_4_4::Component,
    pub rc_3_3_3_3_3: range_check_3_3_3_3_3::Component,
}
impl RangeChecksComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        interaction_elements: &RangeChecksInteractionElements,
        interaction_claim: &RangeChecksInteractionClaim,
    ) -> Self {
        let rc_6_component = range_check_6::Component::new(
            tree_span_provider,
            range_check_6::Eval::new(interaction_elements.rc_6.clone()),
            interaction_claim.rc_6.claimed_sum,
        );
        let rc_8_component = range_check_8::Component::new(
            tree_span_provider,
            range_check_8::Eval::new(interaction_elements.rc_8.clone()),
            interaction_claim.rc_8.claimed_sum,
        );
        let rc_11_component = range_check_11::Component::new(
            tree_span_provider,
            range_check_11::Eval::new(interaction_elements.rc_11.clone()),
            interaction_claim.rc_11.claimed_sum,
        );
        let rc_12_component = range_check_12::Component::new(
            tree_span_provider,
            range_check_12::Eval::new(interaction_elements.rc_12.clone()),
            interaction_claim.rc_12.claimed_sum,
        );
        let rc_18_component = range_check_18::Component::new(
            tree_span_provider,
            range_check_18::Eval::new(interaction_elements.rc_18.clone()),
            interaction_claim.rc_18.claimed_sum,
        );
        let rc_19_component = range_check_19::Component::new(
            tree_span_provider,
            range_check_19::Eval::new(interaction_elements.rc_19.clone()),
            interaction_claim.rc_19.claimed_sum,
        );
        let rc_3_6_component = range_check_3_6::Component::new(
            tree_span_provider,
            range_check_3_6::Eval::new(interaction_elements.rc_3_6.clone()),
            interaction_claim.rc_3_6.claimed_sum,
        );
        let rc_4_3_component = range_check_4_3::Component::new(
            tree_span_provider,
            range_check_4_3::Eval::new(interaction_elements.rc_4_3.clone()),
            interaction_claim.rc_4_3.claimed_sum,
        );
        let rc_4_4_component = range_check_4_4::Component::new(
            tree_span_provider,
            range_check_4_4::Eval::new(interaction_elements.rc_4_4.clone()),
            interaction_claim.rc_4_4.claimed_sum,
        );
        let rc_5_4_component = range_check_5_4::Component::new(
            tree_span_provider,
            range_check_5_4::Eval::new(interaction_elements.rc_5_4.clone()),
            interaction_claim.rc_5_4.claimed_sum,
        );
        let rc_9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval::new(interaction_elements.rc_9_9.clone()),
            interaction_claim.rc_9_9.claimed_sum,
        );
        let rc_7_2_5_component = range_check_7_2_5::Component::new(
            tree_span_provider,
            range_check_7_2_5::Eval::new(interaction_elements.rc_7_2_5.clone()),
            interaction_claim.rc_7_2_5.claimed_sum,
        );
        let rc_3_6_6_3_component = range_check_3_6_6_3::Component::new(
            tree_span_provider,
            range_check_3_6_6_3::Eval::new(interaction_elements.rc_3_6_6_3.clone()),
            interaction_claim.rc_3_6_6_3.claimed_sum,
        );
        let rc_4_4_4_4_component = range_check_4_4_4_4::Component::new(
            tree_span_provider,
            range_check_4_4_4_4::Eval::new(interaction_elements.rc_4_4_4_4.clone()),
            interaction_claim.rc_4_4_4_4.claimed_sum,
        );
        let rc_3_3_3_3_3_component = range_check_3_3_3_3_3::Component::new(
            tree_span_provider,
            range_check_3_3_3_3_3::Eval::new(interaction_elements.rc_3_3_3_3_3.clone()),
            interaction_claim.rc_3_3_3_3_3.claimed_sum,
        );
        Self {
            rc_6: rc_6_component,
            rc_8: rc_8_component,
            rc_11: rc_11_component,
            rc_12: rc_12_component,
            rc_18: rc_18_component,
            rc_19: rc_19_component,
            rc_3_6: rc_3_6_component,
            rc_4_3: rc_4_3_component,
            rc_4_4: rc_4_4_component,
            rc_5_4: rc_5_4_component,
            rc_9_9: rc_9_9_component,
            rc_7_2_5: rc_7_2_5_component,
            rc_3_6_6_3: rc_3_6_6_3_component,
            rc_4_4_4_4: rc_4_4_4_4_component,
            rc_3_3_3_3_3: rc_3_3_3_3_3_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.rc_6 as &dyn ComponentProver<SimdBackend>,
            &self.rc_8 as &dyn ComponentProver<SimdBackend>,
            &self.rc_11 as &dyn ComponentProver<SimdBackend>,
            &self.rc_12 as &dyn ComponentProver<SimdBackend>,
            &self.rc_18 as &dyn ComponentProver<SimdBackend>,
            &self.rc_19 as &dyn ComponentProver<SimdBackend>,
            &self.rc_3_6 as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_3 as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_4 as &dyn ComponentProver<SimdBackend>,
            &self.rc_5_4 as &dyn ComponentProver<SimdBackend>,
            &self.rc_9_9 as &dyn ComponentProver<SimdBackend>,
            &self.rc_7_2_5 as &dyn ComponentProver<SimdBackend>,
            &self.rc_3_6_6_3 as &dyn ComponentProver<SimdBackend>,
            &self.rc_4_4_4_4 as &dyn ComponentProver<SimdBackend>,
            &self.rc_3_3_3_3_3 as &dyn ComponentProver<SimdBackend>,
        ]
    }
}

impl std::fmt::Display for RangeChecksComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RangeCheck6: {}", indented_component_display(&self.rc_6))?;
        writeln!(f, "RangeCheck8: {}", indented_component_display(&self.rc_8))?;
        writeln!(
            f,
            "RangeCheck11: {}",
            indented_component_display(&self.rc_11)
        )?;
        writeln!(
            f,
            "RangeCheck12: {}",
            indented_component_display(&self.rc_12)
        )?;
        writeln!(
            f,
            "RangeCheck18: {}",
            indented_component_display(&self.rc_18)
        )?;
        writeln!(
            f,
            "RangeCheck19: {}",
            indented_component_display(&self.rc_19)
        )?;
        writeln!(
            f,
            "RangeCheck3_6: {}",
            indented_component_display(&self.rc_3_6)
        )?;
        writeln!(
            f,
            "RangeCheck4_3: {}",
            indented_component_display(&self.rc_4_3)
        )?;
        writeln!(
            f,
            "RangeCheck4_4: {}",
            indented_component_display(&self.rc_4_4)
        )?;
        writeln!(
            f,
            "RangeCheck5_4: {}",
            indented_component_display(&self.rc_5_4)
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
        writeln!(
            f,
            "RangeCheck3_6_6_3: {}",
            indented_component_display(&self.rc_3_6_6_3)
        )?;
        writeln!(
            f,
            "RangeCheck4_4_4_4: {}",
            indented_component_display(&self.rc_4_4_4_4)
        )?;
        writeln!(
            f,
            "RangeCheck3_3_3_3_3: {}",
            indented_component_display(&self.rc_3_3_3_3_3)
        )?;
        Ok(())
    }
}

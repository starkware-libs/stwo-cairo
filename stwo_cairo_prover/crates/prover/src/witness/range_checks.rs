use cairo_air::range_checks_air::{
    RangeChecksClaim, RangeChecksInteractionClaim, RangeChecksInteractionElements,
};
use stwo_prover::core::backend::simd::SimdBackend;

use crate::witness::components::{
    range_check_11, range_check_12, range_check_18, range_check_19, range_check_19_b,
    range_check_19_c, range_check_19_d, range_check_3_3_3_3_3, range_check_3_6_6_3,
    range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_5_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9, range_check_9_9_b, range_check_9_9_c,
    range_check_9_9_d,
};
use crate::witness::utils::TreeBuilder;

pub struct RangeChecksClaimGenerator {
    pub rc_6_trace_generator: range_check_6::ClaimGenerator,
    pub rc_8_trace_generator: range_check_8::ClaimGenerator,
    pub rc_11_trace_generator: range_check_11::ClaimGenerator,
    pub rc_12_trace_generator: range_check_12::ClaimGenerator,
    pub rc_18_trace_generator: range_check_18::ClaimGenerator,
    pub rc_19_trace_generator: range_check_19::ClaimGenerator,
    pub rc_19_b_trace_generator: range_check_19_b::ClaimGenerator,
    pub rc_19_c_trace_generator: range_check_19_c::ClaimGenerator,
    pub rc_19_d_trace_generator: range_check_19_d::ClaimGenerator,
    pub rc_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    pub rc_4_4_trace_generator: range_check_4_4::ClaimGenerator,
    pub rc_5_4_trace_generator: range_check_5_4::ClaimGenerator,
    pub rc_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    pub rc_9_9_b_trace_generator: range_check_9_9_b::ClaimGenerator,
    pub rc_9_9_c_trace_generator: range_check_9_9_c::ClaimGenerator,
    pub rc_9_9_d_trace_generator: range_check_9_9_d::ClaimGenerator,
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
            rc_19_b_trace_generator: range_check_19_b::ClaimGenerator::new(),
            rc_19_c_trace_generator: range_check_19_c::ClaimGenerator::new(),
            rc_19_d_trace_generator: range_check_19_d::ClaimGenerator::new(),
            rc_4_3_trace_generator: range_check_4_3::ClaimGenerator::new(),
            rc_4_4_trace_generator: range_check_4_4::ClaimGenerator::new(),
            rc_5_4_trace_generator: range_check_5_4::ClaimGenerator::new(),
            rc_9_9_trace_generator: range_check_9_9::ClaimGenerator::new(),
            rc_9_9_b_trace_generator: range_check_9_9_b::ClaimGenerator::new(),
            rc_9_9_c_trace_generator: range_check_9_9_c::ClaimGenerator::new(),
            rc_9_9_d_trace_generator: range_check_9_9_d::ClaimGenerator::new(),
            rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator::new(),
            rc_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator::new(),
            rc_4_4_4_4_trace_generator: range_check_4_4_4_4::ClaimGenerator::new(),
            rc_3_3_3_3_3_trace_generator: range_check_3_3_3_3_3::ClaimGenerator::new(),
        }
    }
    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (RangeChecksClaim, RangeChecksInteractionClaimGenerator) {
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
                rc_19_b: rc_19_b_claim,
                rc_19_c: rc_19_c_claim,
                rc_19_d: rc_19_d_claim,
                rc_4_3: rc_4_3_claim,
                rc_4_4: rc_4_4_claim,
                rc_5_4: rc_5_4_claim,
                rc_9_9: rc_9_9_claim,
                rc_9_9_b: rc_9_9_b_claim,
                rc_9_9_c: rc_9_9_c_claim,
                rc_9_9_d: rc_9_9_d_claim,
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
                rc_19_b_interaction_gen,
                rc_19_c_interaction_gen,
                rc_19_d_interaction_gen,
                rc_4_3_interaction_gen,
                rc_4_4_interaction_gen,
                rc_5_4_interaction_gen,
                rc_9_9_interaction_gen,
                rc_9_9_b_interaction_gen,
                rc_9_9_c_interaction_gen,
                rc_9_9_d_interaction_gen,
                rc_7_2_5_interaction_gen,
                rc_3_6_6_3_interaction_gen,
                rc_4_4_4_4_interaction_gen,
                rc_3_3_3_3_3_interaction_gen,
            },
        )
    }
}

pub struct RangeChecksInteractionClaimGenerator {
    rc_6_interaction_gen: range_check_6::InteractionClaimGenerator,
    rc_8_interaction_gen: range_check_8::InteractionClaimGenerator,
    rc_11_interaction_gen: range_check_11::InteractionClaimGenerator,
    rc_12_interaction_gen: range_check_12::InteractionClaimGenerator,
    rc_18_interaction_gen: range_check_18::InteractionClaimGenerator,
    rc_19_interaction_gen: range_check_19::InteractionClaimGenerator,
    rc_19_b_interaction_gen: range_check_19_b::InteractionClaimGenerator,
    rc_19_c_interaction_gen: range_check_19_c::InteractionClaimGenerator,
    rc_19_d_interaction_gen: range_check_19_d::InteractionClaimGenerator,
    rc_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    rc_4_4_interaction_gen: range_check_4_4::InteractionClaimGenerator,
    rc_5_4_interaction_gen: range_check_5_4::InteractionClaimGenerator,
    rc_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    rc_9_9_b_interaction_gen: range_check_9_9_b::InteractionClaimGenerator,
    rc_9_9_c_interaction_gen: range_check_9_9_c::InteractionClaimGenerator,
    rc_9_9_d_interaction_gen: range_check_9_9_d::InteractionClaimGenerator,
    rc_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    rc_3_6_6_3_interaction_gen: range_check_3_6_6_3::InteractionClaimGenerator,
    rc_4_4_4_4_interaction_gen: range_check_4_4_4_4::InteractionClaimGenerator,
    rc_3_3_3_3_3_interaction_gen: range_check_3_3_3_3_3::InteractionClaimGenerator,
}
impl RangeChecksInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &RangeChecksInteractionElements,
    ) -> RangeChecksInteractionClaim {
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
        let rc_19_b_interaction_claim = self
            .rc_19_b_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
        let rc_19_c_interaction_claim = self
            .rc_19_c_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
        let rc_19_d_interaction_claim = self
            .rc_19_d_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
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
        let rc_9_9_b_interaction_claim = self
            .rc_9_9_b_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
        let rc_9_9_c_interaction_claim = self
            .rc_9_9_c_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
        let rc_9_9_d_interaction_claim = self
            .rc_9_9_d_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.rc_19);
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
            rc_19_b: rc_19_b_interaction_claim,
            rc_19_c: rc_19_c_interaction_claim,
            rc_19_d: rc_19_d_interaction_claim,
            rc_4_3: rc_4_3_interaction_claim,
            rc_4_4: rc_4_4_interaction_claim,
            rc_5_4: rc_5_4_interaction_claim,
            rc_9_9: rc_9_9_interaction_claim,
            rc_9_9_b: rc_9_9_b_interaction_claim,
            rc_9_9_c: rc_9_9_c_interaction_claim,
            rc_9_9_d: rc_9_9_d_interaction_claim,
            rc_7_2_5: rc_7_2_5_interaction_claim,
            rc_3_6_6_3: rc_3_6_6_3_interaction_claim,
            rc_4_4_4_4: rc_4_4_4_4_interaction_claim,
            rc_3_3_3_3_3: rc_3_3_3_3_3_interaction_claim,
        }
    }
}

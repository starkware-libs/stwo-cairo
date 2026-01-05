use std::sync::Arc;

use cairo_air::range_checks_air::{RangeChecksClaim, RangeChecksInteractionClaim};
use cairo_air::relations;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;

use crate::witness::components::{
    range_check_11, range_check_12, range_check_18, range_check_20, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

pub struct RangeChecksClaimGenerator {
    pub rc_6_trace_generator: range_check_6::ClaimGenerator,
    pub rc_8_trace_generator: range_check_8::ClaimGenerator,
    pub rc_11_trace_generator: range_check_11::ClaimGenerator,
    pub rc_12_trace_generator: range_check_12::ClaimGenerator,
    pub rc_18_trace_generator: range_check_18::ClaimGenerator,
    pub rc_20_trace_generator: range_check_20::ClaimGenerator,
    pub rc_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    pub rc_4_4_trace_generator: range_check_4_4::ClaimGenerator,
    pub rc_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    pub rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator,
    pub rc_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator,
    pub rc_4_4_4_4_trace_generator: range_check_4_4_4_4::ClaimGenerator,
    pub rc_3_3_3_3_3_trace_generator: range_check_3_3_3_3_3::ClaimGenerator,
}

impl RangeChecksClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        Self {
            rc_6_trace_generator: range_check_6::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_8_trace_generator: range_check_8::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_11_trace_generator: range_check_11::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_12_trace_generator: range_check_12::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_18_trace_generator: range_check_18::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_20_trace_generator: range_check_20::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_4_3_trace_generator: range_check_4_3::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_4_4_trace_generator: range_check_4_4::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_9_9_trace_generator: range_check_9_9::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_3_6_6_3_trace_generator: range_check_3_6_6_3::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_4_4_4_4_trace_generator: range_check_4_4_4_4::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
            rc_3_3_3_3_3_trace_generator: range_check_3_3_3_3_3::ClaimGenerator::new(Arc::clone(
                &preprocessed_trace,
            )),
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
        let (rc_20_claim, rc_20_interaction_gen) =
            self.rc_20_trace_generator.write_trace(tree_builder);
        let (rc_4_3_claim, rc_4_3_interaction_gen) =
            self.rc_4_3_trace_generator.write_trace(tree_builder);
        let (rc_4_4_claim, rc_4_4_interaction_gen) =
            self.rc_4_4_trace_generator.write_trace(tree_builder);
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
                rc_20: rc_20_claim,
                rc_4_3: rc_4_3_claim,
                rc_4_4: rc_4_4_claim,
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
                rc_20_interaction_gen,
                rc_4_3_interaction_gen,
                rc_4_4_interaction_gen,
                rc_9_9_interaction_gen,
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
    rc_20_interaction_gen: range_check_20::InteractionClaimGenerator,
    rc_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    rc_4_4_interaction_gen: range_check_4_4::InteractionClaimGenerator,
    rc_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    rc_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    rc_3_6_6_3_interaction_gen: range_check_3_6_6_3::InteractionClaimGenerator,
    rc_4_4_4_4_interaction_gen: range_check_4_4_4_4::InteractionClaimGenerator,
    rc_3_3_3_3_3_interaction_gen: range_check_3_3_3_3_3::InteractionClaimGenerator,
}
impl RangeChecksInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &relations::CommonLookupElements,
    ) -> RangeChecksInteractionClaim {
        let rc_6_interaction_claim = self
            .rc_6_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_8_interaction_claim = self
            .rc_8_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_11_interaction_claim = self
            .rc_11_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_12_interaction_claim = self
            .rc_12_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_18_interaction_claim = self
            .rc_18_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_20_interaction_claim = self
            .rc_20_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_4_3_interaction_claim = self
            .rc_4_3_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_4_4_interaction_claim = self
            .rc_4_4_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_9_9_interaction_claim = self
            .rc_9_9_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_7_2_5_interaction_claim = self
            .rc_7_2_5_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_3_6_6_3_interaction_claim = self
            .rc_3_6_6_3_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_4_4_4_4_interaction_claim = self
            .rc_4_4_4_4_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let rc_3_3_3_3_3_interaction_claim = self
            .rc_3_3_3_3_3_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        RangeChecksInteractionClaim {
            rc_6: rc_6_interaction_claim,
            rc_8: rc_8_interaction_claim,
            rc_11: rc_11_interaction_claim,
            rc_12: rc_12_interaction_claim,
            rc_18: rc_18_interaction_claim,
            rc_20: rc_20_interaction_claim,
            rc_4_3: rc_4_3_interaction_claim,
            rc_4_4: rc_4_4_interaction_claim,
            rc_9_9: rc_9_9_interaction_claim,
            rc_7_2_5: rc_7_2_5_interaction_claim,
            rc_3_6_6_3: rc_3_6_6_3_interaction_claim,
            rc_4_4_4_4: rc_4_4_4_4_interaction_claim,
            rc_3_3_3_3_3: rc_3_3_3_3_3_interaction_claim,
        }
    }
}

use cairo_air::range_checks_air::{RangeChecksClaim, RangeChecksInteractionClaim};
use cairo_air::relations;
use stwo::prover::backend::simd::SimdBackend;

use crate::witness::components::{
    range_check_11, range_check_12, range_check_18, range_check_20, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

pub fn get_range_checks() -> Vec<&'static str> {
    vec![
        "range_check_6",
        "range_check_8",
        "range_check_11",
        "range_check_12",
        "range_check_18",
        "range_check_20",
        "range_check_4_3",
        "range_check_4_4",
        "range_check_9_9",
        "range_check_7_2_5",
        "range_check_3_6_6_3",
        "range_check_4_4_4_4",
        "range_check_3_3_3_3_3",
    ]
}

pub fn range_checks_write_trace(
    rc_6_trace_generator: Option<range_check_6::ClaimGenerator>,
    rc_8_trace_generator: Option<range_check_8::ClaimGenerator>,
    rc_11_trace_generator: Option<range_check_11::ClaimGenerator>,
    rc_12_trace_generator: Option<range_check_12::ClaimGenerator>,
    rc_18_trace_generator: Option<range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<range_check_20::ClaimGenerator>,
    rc_4_3_trace_generator: Option<range_check_4_3::ClaimGenerator>,
    rc_4_4_trace_generator: Option<range_check_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<range_check_9_9::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<range_check_7_2_5::ClaimGenerator>,
    rc_3_6_6_3_trace_generator: Option<range_check_3_6_6_3::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<range_check_4_4_4_4::ClaimGenerator>,
    rc_3_3_3_3_3_trace_generator: Option<range_check_3_3_3_3_3::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
) -> (RangeChecksClaim, RangeChecksInteractionClaimGenerator) {
    let (rc_6_claim, rc_6_interaction_gen) =
        rc_6_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_8_claim, rc_8_interaction_gen) =
        rc_8_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_11_claim, rc_11_interaction_gen) =
        rc_11_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_12_claim, rc_12_interaction_gen) =
        rc_12_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_18_claim, rc_18_interaction_gen) =
        rc_18_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_20_claim, rc_20_interaction_gen) =
        rc_20_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_4_3_claim, rc_4_3_interaction_gen) =
        rc_4_3_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_4_4_claim, rc_4_4_interaction_gen) =
        rc_4_4_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_9_9_claim, rc_9_9_interaction_gen) =
        rc_9_9_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_7_2_5_claim, rc_7_2_5_interaction_gen) =
        rc_7_2_5_trace_generator.unwrap().write_trace(tree_builder);
    let (rc_3_6_6_3_claim, rc_3_6_6_3_interaction_gen) = rc_3_6_6_3_trace_generator
        .unwrap()
        .write_trace(tree_builder);
    let (rc_4_4_4_4_claim, rc_4_4_4_4_interaction_gen) = rc_4_4_4_4_trace_generator
        .unwrap()
        .write_trace(tree_builder);
    let (rc_3_3_3_3_3_claim, rc_3_3_3_3_3_interaction_gen) = rc_3_3_3_3_3_trace_generator
        .unwrap()
        .write_trace(tree_builder);
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

pub struct RangeChecksInteractionClaimGenerator {
    pub rc_6_interaction_gen: range_check_6::InteractionClaimGenerator,
    pub rc_8_interaction_gen: range_check_8::InteractionClaimGenerator,
    pub rc_11_interaction_gen: range_check_11::InteractionClaimGenerator,
    pub rc_12_interaction_gen: range_check_12::InteractionClaimGenerator,
    pub rc_18_interaction_gen: range_check_18::InteractionClaimGenerator,
    pub rc_20_interaction_gen: range_check_20::InteractionClaimGenerator,
    pub rc_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    pub rc_4_4_interaction_gen: range_check_4_4::InteractionClaimGenerator,
    pub rc_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    pub rc_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    pub rc_3_6_6_3_interaction_gen: range_check_3_6_6_3::InteractionClaimGenerator,
    pub rc_4_4_4_4_interaction_gen: range_check_4_4_4_4::InteractionClaimGenerator,
    pub rc_3_3_3_3_3_interaction_gen: range_check_3_3_3_3_3::InteractionClaimGenerator,
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

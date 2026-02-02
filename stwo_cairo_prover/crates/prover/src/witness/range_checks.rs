use crate::witness::components::{
    range_check_11, range_check_12, range_check_18, range_check_20, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9,
};

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

use crate::witness::components::{
    partial_ec_mul_window_bits_18, pedersen_aggregator_window_bits_18,
    pedersen_points_table_window_bits_18,
};

pub struct PedersenContextInteractionClaimGenerator {
    pub gen: Option<PedersenInteractionClaimGenerator>,
}
pub struct PedersenInteractionClaimGenerator {
    pub pedersen_aggregator_interaction_gen:
        pedersen_aggregator_window_bits_18::InteractionClaimGenerator,
    pub partial_ec_mul_interaction_gen: partial_ec_mul_window_bits_18::InteractionClaimGenerator,
    pub pedersen_points_table_interaction_gen:
        pedersen_points_table_window_bits_18::InteractionClaimGenerator,
}

use crate::witness::components::{
    blake_g, blake_round, blake_round_sigma, triple_xor_32, verify_bitwise_xor_12,
};

pub struct BlakeContextInteractionClaimGenerator {
    pub gen: Option<BlakeInteractionClaimGenerator>,
}

pub struct BlakeInteractionClaimGenerator {
    pub blake_round_interaction_gen: blake_round::InteractionClaimGenerator,
    pub blake_g_interaction_gen: blake_g::InteractionClaimGenerator,
    pub blake_sigma_interaction_gen: blake_round_sigma::InteractionClaimGenerator,
    pub triple_xor_32_interaction_gen: triple_xor_32::InteractionClaimGenerator,
    pub verify_bitwise_xor_12_interaction_gen: verify_bitwise_xor_12::InteractionClaimGenerator,
}

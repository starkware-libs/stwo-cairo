use crate::witness::components::{
    cube_252, poseidon_3_partial_rounds_chain, poseidon_aggregator, poseidon_full_round_chain,
    poseidon_round_keys, range_check_252_width_27,
};

pub struct PoseidonContextInteractionClaimGenerator {
    pub gen: Option<PoseidonInteractionClaimGenerator>,
}

pub struct PoseidonInteractionClaimGenerator {
    pub poseidon_aggregator_interaction_gen: poseidon_aggregator::InteractionClaimGenerator,
    pub poseidon_3_partial_rounds_chain_interaction_gen:
        poseidon_3_partial_rounds_chain::InteractionClaimGenerator,
    pub poseidon_full_round_chain_interaction_gen:
        poseidon_full_round_chain::InteractionClaimGenerator,
    pub cube_252_interaction_gen: cube_252::InteractionClaimGenerator,
    pub poseidon_round_keys_interaction_gen: poseidon_round_keys::InteractionClaimGenerator,
    pub range_check_felt_252_width_27_interaction_gen:
        range_check_252_width_27::InteractionClaimGenerator,
}

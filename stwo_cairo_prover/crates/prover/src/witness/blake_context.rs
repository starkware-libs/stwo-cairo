use cairo_air::blake::air::{BlakeContextClaim, Claim};
use stwo::prover::backend::simd::SimdBackend;
use tracing::{span, Level};

use crate::witness::components::{
    blake_g, blake_round, blake_round_sigma, memory_address_to_id, memory_id_to_big,
    range_check_7_2_5, triple_xor_32, verify_bitwise_xor_12, verify_bitwise_xor_4,
    verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::witness::utils::TreeBuilder;

pub fn blake_context_write_trace(
    blake_round: Option<blake_round::ClaimGenerator>,
    blake_g: Option<blake_g::ClaimGenerator>,
    blake_sigma: Option<blake_round_sigma::ClaimGenerator>,
    triple_xor_32: Option<triple_xor_32::ClaimGenerator>,
    verify_bitwise_xor_12: Option<verify_bitwise_xor_12::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_address_to_id_trace_generator: Option<&memory_address_to_id::ClaimGenerator>,
    memory_id_to_value_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<&range_check_7_2_5::ClaimGenerator>,
    verify_bitwise_xor_4_trace_generator: Option<&verify_bitwise_xor_4::ClaimGenerator>,
    verify_bitwise_xor_7_trace_generator: Option<&verify_bitwise_xor_7::ClaimGenerator>,
    verify_bitwise_xor_8_trace_generator: Option<&verify_bitwise_xor_8::ClaimGenerator>,
    verify_bitwise_xor_9_trace_generator: Option<&verify_bitwise_xor_9::ClaimGenerator>,
) -> (BlakeContextClaim, BlakeContextInteractionClaimGenerator) {
    let span = span!(Level::INFO, "write blake context trace").entered();
    if blake_round.as_ref().is_none_or(|tg| tg.is_empty()) {
        return (
            BlakeContextClaim { claim: None },
            BlakeContextInteractionClaimGenerator { gen: None },
        );
    }
    let blake_round = blake_round.expect("Should have blake context components at this point");
    let mut blake_g = blake_g.expect("Should have blake context components at this point");
    let blake_sigma = blake_sigma.expect("Should have blake context components at this point");
    let triple_xor_32 = triple_xor_32.expect("Should have blake context components at this point");
    let verify_bitwise_xor_12 =
        verify_bitwise_xor_12.expect("Should have blake context components at this point");

    let (blake_round_claim, blake_round_interaction_gen) = blake_round.write_trace(
        tree_builder,
        &blake_sigma,
        memory_address_to_id_trace_generator.unwrap(),
        memory_id_to_value_trace_generator.unwrap(),
        rc_7_2_5_trace_generator.unwrap(),
        &mut blake_g,
    );
    let (blake_g_claim, blake_g_interaction_gen) = blake_g.write_trace(
        tree_builder,
        verify_bitwise_xor_8_trace_generator.unwrap(),
        &verify_bitwise_xor_12,
        verify_bitwise_xor_4_trace_generator.unwrap(),
        verify_bitwise_xor_7_trace_generator.unwrap(),
        verify_bitwise_xor_9_trace_generator.unwrap(),
    );
    let (blake_sigma_claim, blake_sigma_interaction_gen) = blake_sigma.write_trace(tree_builder);
    let (triple_xor_32_claim, triple_xor_32_interaction_gen) =
        triple_xor_32.write_trace(tree_builder, verify_bitwise_xor_8_trace_generator.unwrap());
    let (verify_bitwise_xor_12_claim, verify_bitwise_xor_12_interaction_gen) =
        verify_bitwise_xor_12.write_trace(tree_builder);
    span.exit();

    let claim = Some(Claim {
        blake_round: blake_round_claim,
        blake_g: blake_g_claim,
        blake_sigma: blake_sigma_claim,
        triple_xor_32: triple_xor_32_claim,
        verify_bitwise_xor_12: verify_bitwise_xor_12_claim,
    });
    let gen = Some(BlakeInteractionClaimGenerator {
        blake_round_interaction_gen,
        blake_g_interaction_gen,
        blake_sigma_interaction_gen,
        triple_xor_32_interaction_gen,
        verify_bitwise_xor_12_interaction_gen,
    });
    (
        BlakeContextClaim { claim },
        BlakeContextInteractionClaimGenerator { gen },
    )
}

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

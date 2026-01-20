use cairo_air::components::{
    blake_g as blake_g_claim, blake_round as blake_round_claim,
    blake_round_sigma as blake_round_sigma_claim, triple_xor_32 as triple_xor_32_claim,
    verify_bitwise_xor_12 as verify_bitwise_xor_12_claim,
};
use cairo_air::relations::CommonLookupElements;
use stwo::prover::backend::simd::SimdBackend;
use tracing::{span, Level};

use crate::witness::components::{
    blake_g, blake_round, blake_round_sigma, memory_address_to_id, memory_id_to_big,
    range_check_7_2_5, triple_xor_32, verify_bitwise_xor_12, verify_bitwise_xor_4,
    verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_9,
};
use crate::witness::utils::TreeBuilder;

#[allow(clippy::type_complexity)]
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
) -> (
    Option<blake_round_claim::Claim>,
    Option<blake_g_claim::Claim>,
    Option<blake_round_sigma_claim::Claim>,
    Option<triple_xor_32_claim::Claim>,
    Option<verify_bitwise_xor_12_claim::Claim>,
    BlakeContextInteractionClaimGenerator,
) {
    let span = span!(Level::INFO, "write blake context trace").entered();
    if blake_round.as_ref().is_none_or(|tg| tg.is_empty()) {
        span.exit();
        return (
            None,
            None,
            None,
            None,
            None,
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

    let gen = Some(InteractionClaimGenerator {
        blake_round_interaction_gen,
        blake_g_interaction_gen,
        blake_sigma_interaction_gen,
        triple_xor_32_interaction_gen,
        verify_bitwise_xor_12_interaction_gen,
    });

    (
        Some(blake_round_claim),
        Some(blake_g_claim),
        Some(blake_sigma_claim),
        Some(triple_xor_32_claim),
        Some(verify_bitwise_xor_12_claim),
        BlakeContextInteractionClaimGenerator { gen },
    )
}

pub struct BlakeContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl BlakeContextInteractionClaimGenerator {
    #[allow(clippy::type_complexity)]
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> (
        Option<blake_round_claim::InteractionClaim>,
        Option<blake_g_claim::InteractionClaim>,
        Option<blake_round_sigma_claim::InteractionClaim>,
        Option<triple_xor_32_claim::InteractionClaim>,
        Option<verify_bitwise_xor_12_claim::InteractionClaim>,
    ) {
        self.gen.map_or((None, None, None, None, None), |gen| {
            gen.write_interaction_trace(tree_builder, common_lookup_elements)
        })
    }
}

struct InteractionClaimGenerator {
    blake_round_interaction_gen: blake_round::InteractionClaimGenerator,
    blake_g_interaction_gen: blake_g::InteractionClaimGenerator,
    blake_sigma_interaction_gen: blake_round_sigma::InteractionClaimGenerator,
    triple_xor_32_interaction_gen: triple_xor_32::InteractionClaimGenerator,
    verify_bitwise_xor_12_interaction_gen: verify_bitwise_xor_12::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    #[allow(clippy::type_complexity)]
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> (
        Option<blake_round_claim::InteractionClaim>,
        Option<blake_g_claim::InteractionClaim>,
        Option<blake_round_sigma_claim::InteractionClaim>,
        Option<triple_xor_32_claim::InteractionClaim>,
        Option<verify_bitwise_xor_12_claim::InteractionClaim>,
    ) {
        let blake_round_interaction_claim = self
            .blake_round_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let blake_g_interaction_claim = self
            .blake_g_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let blake_sigma_interaction_claim = self
            .blake_sigma_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let triple_xor_32_interaction_claim = self
            .triple_xor_32_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let verify_bitwise_xor_12_interaction_claim = self
            .verify_bitwise_xor_12_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);

        (
            Some(blake_round_interaction_claim),
            Some(blake_g_interaction_claim),
            Some(blake_sigma_interaction_claim),
            Some(triple_xor_32_interaction_claim),
            Some(verify_bitwise_xor_12_interaction_claim),
        )
    }
}

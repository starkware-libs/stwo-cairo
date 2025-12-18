use std::sync::Arc;

use cairo_air::air::CairoInteractionElements;
use cairo_air::blake::air::{
    BlakeContextClaim, BlakeContextInteractionClaim, Claim, InteractionClaim,
};
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
use tracing::{span, Level};

use crate::witness::components::{
    blake_g, blake_round, blake_round_sigma, memory_address_to_id, memory_id_to_big, triple_xor_32,
    verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8,
    verify_bitwise_xor_8_b, verify_bitwise_xor_9,
};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct BlakeContextClaimGenerator {
    pub blake_round: Option<blake_round::ClaimGenerator>,
    pub blake_g: Option<blake_g::ClaimGenerator>,
    pub blake_sigma: Option<blake_round_sigma::ClaimGenerator>,
    pub triple_xor_32: Option<triple_xor_32::ClaimGenerator>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::ClaimGenerator>,
}
impl BlakeContextClaimGenerator {
    pub fn new(memory: Memory, preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        if !preprocessed_trace.has_column(&PreProcessedColumnId {
            id: "bitwise_xor_12_0".to_owned(),
        }) {
            // This is a preprocessed trace without the Pedersen points table - Don't
            // create Pedersen components.
            return Self {
                blake_round: None,
                blake_g: None,
                blake_sigma: None,
                triple_xor_32: None,
                verify_bitwise_xor_12: None,
            };
        }
        let blake_round = Some(blake_round::ClaimGenerator::new(memory));
        let blake_g = Some(blake_g::ClaimGenerator::new());
        let blake_sigma = Some(blake_round_sigma::ClaimGenerator::new(
            preprocessed_trace.clone(),
        ));
        let triple_xor_32 = Some(triple_xor_32::ClaimGenerator::new());
        let verify_bitwise_xor_12 = Some(verify_bitwise_xor_12::ClaimGenerator::new(
            preprocessed_trace,
        ));

        Self {
            blake_round,
            blake_g,
            blake_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
        verify_bitwise_xor_4_trace_generator: &verify_bitwise_xor_4::ClaimGenerator,
        verify_bitwise_xor_7_trace_generator: &verify_bitwise_xor_7::ClaimGenerator,
        verify_bitwise_xor_8_trace_generator: &verify_bitwise_xor_8::ClaimGenerator,
        verify_bitwise_xor_8_b_trace_generator: &verify_bitwise_xor_8_b::ClaimGenerator,
        verify_bitwise_xor_9_trace_generator: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (BlakeContextClaim, BlakeContextInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write blake context trace").entered();
        if self.blake_round.as_ref().is_none_or(|tg| tg.is_empty()) {
            return (
                BlakeContextClaim { claim: None },
                BlakeContextInteractionClaimGenerator { gen: None },
            );
        }
        let Self {
            blake_round,
            blake_g,
            blake_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
        } = self;
        let blake_round = blake_round.expect("Should have blake context components at this point");
        let mut blake_g = blake_g.expect("Should have blake context components at this point");
        let blake_sigma = blake_sigma.expect("Should have blake context components at this point");
        let triple_xor_32 =
            triple_xor_32.expect("Should have blake context components at this point");
        let verify_bitwise_xor_12 =
            verify_bitwise_xor_12.expect("Should have blake context components at this point");

        let (blake_round_claim, blake_round_interaction_gen) = blake_round.write_trace(
            tree_builder,
            &mut blake_g,
            &blake_sigma,
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            &range_checks_trace_generator.rc_7_2_5_trace_generator,
        );
        let (blake_g_claim, blake_g_interaction_gen) = blake_g.write_trace(
            tree_builder,
            verify_bitwise_xor_8_trace_generator,
            verify_bitwise_xor_8_b_trace_generator,
            &verify_bitwise_xor_12,
            verify_bitwise_xor_4_trace_generator,
            verify_bitwise_xor_7_trace_generator,
            verify_bitwise_xor_9_trace_generator,
        );
        let (blake_sigma_claim, blake_sigma_interaction_gen) =
            blake_sigma.write_trace(tree_builder);
        let (triple_xor_32_claim, triple_xor_32_interaction_gen) = triple_xor_32.write_trace(
            tree_builder,
            verify_bitwise_xor_8_trace_generator,
            verify_bitwise_xor_8_b_trace_generator,
        );
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
        let gen = Some(InteractionClaimGenerator {
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
}

pub struct BlakeContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl BlakeContextInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> BlakeContextInteractionClaim {
        BlakeContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, interaction_elements)),
        }
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
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim {
        let blake_round_interaction_claim =
            self.blake_round_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.blake_g,
                &interaction_elements.blake_round,
                &interaction_elements.blake_sigma,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_7_2_5,
            );
        let blake_g_interaction_claim = self.blake_g_interaction_gen.write_interaction_trace(
            tree_builder,
            &interaction_elements.verify_bitwise_xor_8,
            &interaction_elements.verify_bitwise_xor_8_b,
            &interaction_elements.verify_bitwise_xor_12,
            &interaction_elements.verify_bitwise_xor_4,
            &interaction_elements.verify_bitwise_xor_7,
            &interaction_elements.verify_bitwise_xor_9,
            &interaction_elements.blake_g,
        );
        let blake_sigma_interaction_claim = self
            .blake_sigma_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.blake_sigma);
        let triple_xor_32_interaction_claim =
            self.triple_xor_32_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.verify_bitwise_xor_8_b,
                &interaction_elements.triple_xor_32,
            );
        let verify_bitwise_xor_12_interaction_claim = self
            .verify_bitwise_xor_12_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_12);

        InteractionClaim {
            blake_round: blake_round_interaction_claim,
            blake_g: blake_g_interaction_claim,
            blake_sigma: blake_sigma_interaction_claim,
            triple_xor_32: triple_xor_32_interaction_claim,
            verify_bitwise_xor_12: verify_bitwise_xor_12_interaction_claim,
        }
    }
}

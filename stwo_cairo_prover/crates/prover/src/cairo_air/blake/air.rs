use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::TreeVec;
use tracing::{span, Level};

use crate::cairo_air::air::CairoInteractionElements;
use crate::cairo_air::range_checks_air::RangeChecksClaimGenerator;
use crate::components::utils::TreeBuilder;
use crate::components::{
    blake_g, blake_round, blake_round_sigma, memory_address_to_id, memory_id_to_big, triple_xor_32,
    verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8,
    verify_bitwise_xor_9,
};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BlakeContextClaim {
    claim: Option<Claim>,
}
impl BlakeContextClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(claim) = &self.claim {
            claim.mix_into(channel);
        }
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        self.claim
            .as_ref()
            .map(|claim| claim.log_sizes())
            .unwrap_or_default()
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
struct Claim {
    pub blake_round: blake_round::Claim,
    pub blake_g: blake_g::Claim,
    pub blake_sigma: blake_round_sigma::Claim,
    pub triple_xor_32: triple_xor_32::Claim,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.blake_round.mix_into(channel);
        self.blake_g.mix_into(channel);
        self.blake_sigma.mix_into(channel);
        self.triple_xor_32.mix_into(channel);
        self.verify_bitwise_xor_12.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.blake_round.log_sizes(),
            self.blake_g.log_sizes(),
            self.blake_sigma.log_sizes(),
            self.triple_xor_32.log_sizes(),
            self.verify_bitwise_xor_12.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }
}

pub struct BlakeContextClaimGenerator {
    pub blake_round: blake_round::ClaimGenerator,
    pub blake_g: blake_g::ClaimGenerator,
    pub blake_sigma: blake_round_sigma::ClaimGenerator,
    pub triple_xor_32: triple_xor_32::ClaimGenerator,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::ClaimGenerator,
}
impl BlakeContextClaimGenerator {
    pub fn new(memory: Memory) -> Self {
        let blake_round = blake_round::ClaimGenerator::new(memory);
        let blake_g = blake_g::ClaimGenerator::new();
        let blake_sigma = blake_round_sigma::ClaimGenerator::new();
        let triple_xor_32 = triple_xor_32::ClaimGenerator::new();
        let verify_bitwise_xor_12 = verify_bitwise_xor_12::ClaimGenerator::new();

        Self {
            blake_round,
            blake_g,
            blake_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_trace_generator: &memory_address_to_id::ClaimGenerator,
        memory_id_to_value_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
        verify_bitwise_xor_4_trace_generator: &verify_bitwise_xor_4::ClaimGenerator,
        verify_bitwise_xor_7_trace_generator: &verify_bitwise_xor_7::ClaimGenerator,
        verify_bitwise_xor_8_trace_generator: &verify_bitwise_xor_8::ClaimGenerator,
        verify_bitwise_xor_9_trace_generator: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (BlakeContextClaim, BlakeContextInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write blake context trace").entered();
        if self.blake_round.is_empty() {
            return (
                BlakeContextClaim { claim: None },
                BlakeContextInteractionClaimGenerator { gen: None },
            );
        }
        let (blake_round_claim, blake_round_interaction_gen) = self.blake_round.write_trace(
            tree_builder,
            &mut self.blake_g,
            &self.blake_sigma,
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            &range_checks_trace_generator.rc_7_2_5_trace_generator,
        );
        let (blake_g_claim, blake_g_interaction_gen) = self.blake_g.write_trace(
            tree_builder,
            &self.verify_bitwise_xor_12,
            verify_bitwise_xor_4_trace_generator,
            verify_bitwise_xor_7_trace_generator,
            verify_bitwise_xor_8_trace_generator,
            verify_bitwise_xor_9_trace_generator,
        );
        let (blake_sigma_claim, blake_sigma_interaction_gen) =
            self.blake_sigma.write_trace(tree_builder);
        let (triple_xor_32_claim, triple_xor_32_interaction_gen) = self
            .triple_xor_32
            .write_trace(tree_builder, verify_bitwise_xor_8_trace_generator);
        let (verify_bitwise_xor_12_claim, verify_bitwise_xor_12_interaction_gen) =
            self.verify_bitwise_xor_12.write_trace(tree_builder);
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
            &interaction_elements.blake_g,
            &interaction_elements.verify_bitwise_xor_12,
            &interaction_elements.verify_bitwise_xor_4,
            &interaction_elements.verify_bitwise_xor_7,
            &interaction_elements.verify_bitwise_xor_8,
            &interaction_elements.verify_bitwise_xor_9,
        );
        let blake_sigma_interaction_claim = self
            .blake_sigma_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.blake_sigma);
        let triple_xor_32_interaction_claim =
            self.triple_xor_32_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.triple_xor_32,
                &interaction_elements.verify_bitwise_xor_8,
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

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct BlakeContextInteractionClaim {
    claim: Option<InteractionClaim>,
}
impl BlakeContextInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        if let Some(claim) = &self.claim {
            claim.mix_into(channel);
        }
    }

    pub fn sum(&self) -> QM31 {
        self.claim
            .as_ref()
            .map(|claim| claim.sum())
            .unwrap_or_else(QM31::zero)
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
struct InteractionClaim {
    pub blake_round: blake_round::InteractionClaim,
    pub blake_g: blake_g::InteractionClaim,
    pub blake_sigma: blake_round_sigma::InteractionClaim,
    pub triple_xor_32: triple_xor_32::InteractionClaim,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.blake_round.mix_into(channel);
        self.blake_g.mix_into(channel);
        self.blake_sigma.mix_into(channel);
        self.triple_xor_32.mix_into(channel);
        self.verify_bitwise_xor_12.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        self.blake_round.claimed_sum
            + self.blake_g.claimed_sum
            + self.blake_sigma.claimed_sum
            + self.triple_xor_32.claimed_sum
            + self.verify_bitwise_xor_12.claimed_sum
    }
}

pub struct BlakeContextComponents {
    pub components: Option<Components>,
}
impl BlakeContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BlakeContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &BlakeContextInteractionClaim,
    ) -> Self {
        let components = interaction_claim
            .claim
            .as_ref()
            .map(|ic| Components::new(tree_span_provider, claim, interaction_elements, ic));
        Self { components }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        self.components
            .as_ref()
            .map(|c| c.provers())
            .unwrap_or_default()
    }
}

impl std::fmt::Display for BlakeContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{}", components),
            None => write!(f, "No Blake Context Components"),
        }
    }
}

pub struct Components {
    pub blake_round: blake_round::Component,
    pub blake_g: blake_g::Component,
    pub blake_sigma: blake_round_sigma::Component,
    pub triple_xor_32: triple_xor_32::Component,
    pub verify_bitwise_xor_12: verify_bitwise_xor_12::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &BlakeContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let blake_round_component = blake_round::Component::new(
            tree_span_provider,
            blake_round::Eval {
                claim: claim.claim.as_ref().unwrap().blake_round,
                blake_g_lookup_elements: interaction_elements.blake_g.clone(),
                blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
                memory_address_to_id_lookup_elements: interaction_elements
                    .memory_address_to_id
                    .clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            interaction_claim.blake_round.claimed_sum,
        );

        let blake_g_component = blake_g::Component::new(
            tree_span_provider,
            blake_g::Eval {
                claim: claim.claim.as_ref().unwrap().blake_g,
                blake_g_lookup_elements: interaction_elements.blake_g.clone(),
                verify_bitwise_xor_12_lookup_elements: interaction_elements
                    .verify_bitwise_xor_12
                    .clone(),
                verify_bitwise_xor_4_lookup_elements: interaction_elements
                    .verify_bitwise_xor_4
                    .clone(),
                verify_bitwise_xor_7_lookup_elements: interaction_elements
                    .verify_bitwise_xor_7
                    .clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
                verify_bitwise_xor_9_lookup_elements: interaction_elements
                    .verify_bitwise_xor_9
                    .clone(),
            },
            interaction_claim.blake_g.claimed_sum,
        );

        let blake_sigma_component = blake_round_sigma::Component::new(
            tree_span_provider,
            blake_round_sigma::Eval {
                blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
            },
            interaction_claim.blake_sigma.claimed_sum,
        );

        let triple_xor_32_component = triple_xor_32::Component::new(
            tree_span_provider,
            triple_xor_32::Eval {
                claim: claim.claim.as_ref().unwrap().triple_xor_32,
                triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.triple_xor_32.claimed_sum,
        );
        let verify_bitwise_xor_12_component = verify_bitwise_xor_12::Component::new(
            tree_span_provider,
            verify_bitwise_xor_12::Eval {
                verify_bitwise_xor_12_lookup_elements: interaction_elements
                    .verify_bitwise_xor_12
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_12.claimed_sum,
        );
        Self {
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_sigma: blake_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.blake_round,
            &self.blake_g,
            &self.blake_sigma,
            &self.triple_xor_32,
            &self.verify_bitwise_xor_12,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BlakeRound: {}", self.blake_round)?;
        writeln!(f, "BlakeG: {}", self.blake_g)?;
        writeln!(f, "BlakeSigma: {}", self.blake_sigma)?;
        writeln!(f, "TripleXor32: {}", self.triple_xor_32)?;
        writeln!(f, "VerifyBitwiseXor12: {}", self.verify_bitwise_xor_12)?;
        Ok(())
    }
}

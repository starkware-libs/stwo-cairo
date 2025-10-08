use cairo_air::air::CairoInteractionElements;
use cairo_air::sha256::air::{
    Claim, InteractionClaim, Sha256ContextClaim, Sha256ContextInteractionClaim,
};
use tracing::{span, Level};

use crate::witness::components::{
    sha_256_big_sigma_0, sha_256_big_sigma_0_o_0, sha_256_big_sigma_0_o_1, sha_256_big_sigma_1,
    sha_256_big_sigma_1_o_0, sha_256_big_sigma_1_o_1, sha_256_k_table, sha_256_round,
    sha_256_schedule, sha_256_small_sigma_0, sha_256_small_sigma_0_o_0, sha_256_small_sigma_0_o_1,
    sha_256_small_sigma_1, sha_256_small_sigma_1_o_0, sha_256_small_sigma_1_o_1,
    verify_bitwise_and_8, verify_bitwise_xor_8,
};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct Sha256ContextClaimGenerator {
    pub sha_256_round_trace_generator: sha_256_round::ClaimGenerator,
    pub sha_256_big_sigma_0_trace_generator: sha_256_big_sigma_0::ClaimGenerator,
    pub sha_256_big_sigma_1_trace_generator: sha_256_big_sigma_1::ClaimGenerator,
    pub sha_256_schedule_trace_generator: sha_256_schedule::ClaimGenerator,
    pub sha_256_small_sigma_0_trace_generator: sha_256_small_sigma_0::ClaimGenerator,
    pub sha_256_small_sigma_1_trace_generator: sha_256_small_sigma_1::ClaimGenerator,
    pub sha_256_big_sigma_0_o_0_trace_generator: sha_256_big_sigma_0_o_0::ClaimGenerator,
    pub sha_256_big_sigma_0_o_1_trace_generator: sha_256_big_sigma_0_o_1::ClaimGenerator,
    pub sha_256_big_sigma_1_o_0_trace_generator: sha_256_big_sigma_1_o_0::ClaimGenerator,
    pub sha_256_big_sigma_1_o_1_trace_generator: sha_256_big_sigma_1_o_1::ClaimGenerator,
    pub sha_256_small_sigma_0_o_0_trace_generator: sha_256_small_sigma_0_o_0::ClaimGenerator,
    pub sha_256_small_sigma_0_o_1_trace_generator: sha_256_small_sigma_0_o_1::ClaimGenerator,
    pub sha_256_small_sigma_1_o_0_trace_generator: sha_256_small_sigma_1_o_0::ClaimGenerator,
    pub sha_256_small_sigma_1_o_1_trace_generator: sha_256_small_sigma_1_o_1::ClaimGenerator,
    pub sha_256_k_table_trace_generator: sha_256_k_table::ClaimGenerator,
}
impl Default for Sha256ContextClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256ContextClaimGenerator {
    pub fn new() -> Self {
        let sha_256_round_trace_generator = sha_256_round::ClaimGenerator::new();
        let sha_256_big_sigma_0_trace_generator = sha_256_big_sigma_0::ClaimGenerator::new();
        let sha_256_big_sigma_1_trace_generator = sha_256_big_sigma_1::ClaimGenerator::new();
        let sha_256_schedule_trace_generator = sha_256_schedule::ClaimGenerator::new();
        let sha_256_small_sigma_0_trace_generator = sha_256_small_sigma_0::ClaimGenerator::new();
        let sha_256_small_sigma_1_trace_generator = sha_256_small_sigma_1::ClaimGenerator::new();
        let sha_256_big_sigma_0_o_0_trace_generator =
            sha_256_big_sigma_0_o_0::ClaimGenerator::default();
        let sha_256_big_sigma_0_o_1_trace_generator =
            sha_256_big_sigma_0_o_1::ClaimGenerator::default();
        let sha_256_big_sigma_1_o_0_trace_generator =
            sha_256_big_sigma_1_o_0::ClaimGenerator::default();
        let sha_256_big_sigma_1_o_1_trace_generator =
            sha_256_big_sigma_1_o_1::ClaimGenerator::default();
        let sha_256_small_sigma_0_o_0_trace_generator =
            sha_256_small_sigma_0_o_0::ClaimGenerator::default();
        let sha_256_small_sigma_0_o_1_trace_generator =
            sha_256_small_sigma_0_o_1::ClaimGenerator::default();
        let sha_256_small_sigma_1_o_0_trace_generator =
            sha_256_small_sigma_1_o_0::ClaimGenerator::default();
        let sha_256_small_sigma_1_o_1_trace_generator =
            sha_256_small_sigma_1_o_1::ClaimGenerator::default();
        let sha_256_k_table_trace_generator = sha_256_k_table::ClaimGenerator::default();

        Self {
            sha_256_round_trace_generator,
            sha_256_big_sigma_0_trace_generator,
            sha_256_big_sigma_1_trace_generator,
            sha_256_schedule_trace_generator,
            sha_256_small_sigma_0_trace_generator,
            sha_256_small_sigma_1_trace_generator,
            sha_256_big_sigma_0_o_0_trace_generator,
            sha_256_big_sigma_0_o_1_trace_generator,
            sha_256_big_sigma_1_o_0_trace_generator,
            sha_256_big_sigma_1_o_1_trace_generator,
            sha_256_small_sigma_0_o_0_trace_generator,
            sha_256_small_sigma_0_o_1_trace_generator,
            sha_256_small_sigma_1_o_0_trace_generator,
            sha_256_small_sigma_1_o_1_trace_generator,
            sha_256_k_table_trace_generator,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
        bitwise_and_8_trace_generator: &verify_bitwise_and_8::ClaimGenerator,
        bitwise_xor_8_trace_generator: &verify_bitwise_xor_8::ClaimGenerator,
    ) -> (Sha256ContextClaim, Sha256ContextInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write sha256 context trace").entered();

        // Debug: Check if sha256 round generator has data
        println!(
            "Sha256Context: sha_256_round_trace_generator is_empty: {}",
            self.sha_256_round_trace_generator.is_empty()
        );
        println!(
            "Sha256Context: packed_inputs len: {}",
            self.sha_256_round_trace_generator.packed_inputs.len()
        );

        if self.sha_256_round_trace_generator.is_empty() {
            println!(
                "Sha256Context: WARNING - sha_256_round_trace_generator is empty, returning None"
            );
            return (
                Sha256ContextClaim { claim: None },
                Sha256ContextInteractionClaimGenerator { gen: None },
            );
        }
        let (sha_256_round_claim, sha_256_round_interaction_gen) =
            self.sha_256_round_trace_generator.write_trace(
                tree_builder,
                &mut self.sha_256_big_sigma_0_trace_generator,
                &mut self.sha_256_big_sigma_1_trace_generator,
                &self.sha_256_k_table_trace_generator,
                &mut self.sha_256_schedule_trace_generator,
                bitwise_and_8_trace_generator,
                bitwise_xor_8_trace_generator,
            );
        let (sha_256_big_sigma_0_claim, sha_256_big_sigma_0_interaction_gen) =
            self.sha_256_big_sigma_0_trace_generator.write_trace(
                tree_builder,
                &self.sha_256_big_sigma_0_o_0_trace_generator,
                &self.sha_256_big_sigma_0_o_1_trace_generator,
                bitwise_and_8_trace_generator,
                bitwise_xor_8_trace_generator,
            );
        let (sha_256_big_sigma_1_claim, sha_256_big_sigma_1_interaction_gen) =
            self.sha_256_big_sigma_1_trace_generator.write_trace(
                tree_builder,
                &self.sha_256_big_sigma_1_o_0_trace_generator,
                &self.sha_256_big_sigma_1_o_1_trace_generator,
                bitwise_and_8_trace_generator,
                bitwise_xor_8_trace_generator,
            );
        let (sha_256_schedule_claim, sha_256_schedule_interaction_gen) =
            self.sha_256_schedule_trace_generator.write_trace(
                tree_builder,
                &range_checks_trace_generator.rc_2_trace_generator,
                &mut self.sha_256_small_sigma_0_trace_generator,
                &mut self.sha_256_small_sigma_1_trace_generator,
            );
        let (sha_256_small_sigma_0_claim, sha_256_small_sigma_0_interaction_gen) =
            self.sha_256_small_sigma_0_trace_generator.write_trace(
                tree_builder,
                &self.sha_256_small_sigma_0_o_0_trace_generator,
                &self.sha_256_small_sigma_0_o_1_trace_generator,
                bitwise_and_8_trace_generator,
                bitwise_xor_8_trace_generator,
            );
        let (sha_256_small_sigma_1_claim, sha_256_small_sigma_1_interaction_gen) =
            self.sha_256_small_sigma_1_trace_generator.write_trace(
                tree_builder,
                &self.sha_256_small_sigma_1_o_0_trace_generator,
                &self.sha_256_small_sigma_1_o_1_trace_generator,
                bitwise_and_8_trace_generator,
                bitwise_xor_8_trace_generator,
            );
        let (sha_256_big_sigma_0_o_0_claim, sha_256_big_sigma_0_o_0_interaction_gen) = self
            .sha_256_big_sigma_0_o_0_trace_generator
            .write_trace(tree_builder);
        let (sha_256_big_sigma_0_o_1_claim, sha_256_big_sigma_0_o_1_interaction_gen) = self
            .sha_256_big_sigma_0_o_1_trace_generator
            .write_trace(tree_builder);
        let (sha_256_big_sigma_1_o_0_claim, sha_256_big_sigma_1_o_0_interaction_gen) = self
            .sha_256_big_sigma_1_o_0_trace_generator
            .write_trace(tree_builder);
        let (sha_256_big_sigma_1_o_1_claim, sha_256_big_sigma_1_o_1_interaction_gen) = self
            .sha_256_big_sigma_1_o_1_trace_generator
            .write_trace(tree_builder);
        let (sha_256_small_sigma_0_o_0_claim, sha_256_small_sigma_0_o_0_interaction_gen) = self
            .sha_256_small_sigma_0_o_0_trace_generator
            .write_trace(tree_builder);
        let (sha_256_small_sigma_0_o_1_claim, sha_256_small_sigma_0_o_1_interaction_gen) = self
            .sha_256_small_sigma_0_o_1_trace_generator
            .write_trace(tree_builder);
        let (sha_256_small_sigma_1_o_0_claim, sha_256_small_sigma_1_o_0_interaction_gen) = self
            .sha_256_small_sigma_1_o_0_trace_generator
            .write_trace(tree_builder);
        let (sha_256_small_sigma_1_o_1_claim, sha_256_small_sigma_1_o_1_interaction_gen) = self
            .sha_256_small_sigma_1_o_1_trace_generator
            .write_trace(tree_builder);
        let (sha_256_k_table_claim, sha_256_k_table_interaction_gen) = self
            .sha_256_k_table_trace_generator
            .write_trace(tree_builder);

        span.exit();

        let claim = Some(Claim {
            sha_256_round: sha_256_round_claim,
            sha_256_big_sigma_0: sha_256_big_sigma_0_claim,
            sha_256_big_sigma_1: sha_256_big_sigma_1_claim,
            sha_256_schedule: sha_256_schedule_claim,
            sha_256_small_sigma_0: sha_256_small_sigma_0_claim,
            sha_256_small_sigma_1: sha_256_small_sigma_1_claim,
            sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0_claim,
            sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1_claim,
            sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0_claim,
            sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1_claim,
            sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0_claim,
            sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1_claim,
            sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0_claim,
            sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1_claim,
            sha_256_k_table: sha_256_k_table_claim,
        });
        let gen = Some(InteractionClaimGenerator {
            sha_256_round_interaction_gen,
            sha_256_big_sigma_0_interaction_gen,
            sha_256_big_sigma_1_interaction_gen,
            sha_256_schedule_interaction_gen,
            sha_256_small_sigma_0_interaction_gen,
            sha_256_small_sigma_1_interaction_gen,
            sha_256_big_sigma_0_o_0_interaction_gen,
            sha_256_big_sigma_0_o_1_interaction_gen,
            sha_256_big_sigma_1_o_0_interaction_gen,
            sha_256_big_sigma_1_o_1_interaction_gen,
            sha_256_small_sigma_0_o_0_interaction_gen,
            sha_256_small_sigma_0_o_1_interaction_gen,
            sha_256_small_sigma_1_o_0_interaction_gen,
            sha_256_small_sigma_1_o_1_interaction_gen,
            sha_256_k_table_interaction_gen,
        });
        (
            Sha256ContextClaim { claim },
            Sha256ContextInteractionClaimGenerator { gen },
        )
    }
}

pub struct Sha256ContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl Sha256ContextInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> Sha256ContextInteractionClaim {
        Sha256ContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, interaction_elements)),
        }
    }
}

struct InteractionClaimGenerator {
    sha_256_round_interaction_gen: sha_256_round::InteractionClaimGenerator,
    sha_256_big_sigma_0_interaction_gen: sha_256_big_sigma_0::InteractionClaimGenerator,
    sha_256_big_sigma_1_interaction_gen: sha_256_big_sigma_1::InteractionClaimGenerator,
    sha_256_schedule_interaction_gen: sha_256_schedule::InteractionClaimGenerator,
    sha_256_small_sigma_0_interaction_gen: sha_256_small_sigma_0::InteractionClaimGenerator,
    sha_256_small_sigma_1_interaction_gen: sha_256_small_sigma_1::InteractionClaimGenerator,
    sha_256_big_sigma_0_o_0_interaction_gen: sha_256_big_sigma_0_o_0::InteractionClaimGenerator,
    sha_256_big_sigma_0_o_1_interaction_gen: sha_256_big_sigma_0_o_1::InteractionClaimGenerator,
    sha_256_big_sigma_1_o_0_interaction_gen: sha_256_big_sigma_1_o_0::InteractionClaimGenerator,
    sha_256_big_sigma_1_o_1_interaction_gen: sha_256_big_sigma_1_o_1::InteractionClaimGenerator,
    sha_256_small_sigma_0_o_0_interaction_gen: sha_256_small_sigma_0_o_0::InteractionClaimGenerator,
    sha_256_small_sigma_0_o_1_interaction_gen: sha_256_small_sigma_0_o_1::InteractionClaimGenerator,
    sha_256_small_sigma_1_o_0_interaction_gen: sha_256_small_sigma_1_o_0::InteractionClaimGenerator,
    sha_256_small_sigma_1_o_1_interaction_gen: sha_256_small_sigma_1_o_1::InteractionClaimGenerator,
    sha_256_k_table_interaction_gen: sha_256_k_table::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim {
        let sha_256_round_interaction_claim =
            self.sha_256_round_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.sha_256_big_sigma_1,
                &interaction_elements.sha_256_big_sigma_0,
                &interaction_elements.verify_bitwise_and_8,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.sha_256_k_table,
                &interaction_elements.sha_256_schedule,
                &interaction_elements.sha_256_round,
            );
        let sha_256_big_sigma_0_interaction_claim = self
            .sha_256_big_sigma_0_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_and_8,
                &interaction_elements.sha_256_big_sigma_0_o_0,
                &interaction_elements.sha_256_big_sigma_0_o_1,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.sha_256_big_sigma_0,
            );
        let sha_256_big_sigma_1_interaction_claim = self
            .sha_256_big_sigma_1_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_and_8,
                &interaction_elements.sha_256_big_sigma_1_o_0,
                &interaction_elements.sha_256_big_sigma_1_o_1,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.sha_256_big_sigma_1,
            );
        let sha_256_schedule_interaction_claim = self
            .sha_256_schedule_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.sha_256_small_sigma_0,
                &interaction_elements.sha_256_small_sigma_1,
                &interaction_elements.range_checks.rc_2,
                &interaction_elements.sha_256_schedule,
            );
        let sha_256_small_sigma_0_interaction_claim = self
            .sha_256_small_sigma_0_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_and_8,
                &interaction_elements.sha_256_small_sigma_0_o_0,
                &interaction_elements.sha_256_small_sigma_0_o_1,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.sha_256_small_sigma_0,
            );
        let sha_256_small_sigma_1_interaction_claim = self
            .sha_256_small_sigma_1_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_and_8,
                &interaction_elements.sha_256_small_sigma_1_o_0,
                &interaction_elements.sha_256_small_sigma_1_o_1,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.sha_256_small_sigma_1,
            );
        let sha_256_big_sigma_0_o_0_interaction_claim = self
            .sha_256_big_sigma_0_o_0_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.sha_256_big_sigma_0_o_0);
        let sha_256_big_sigma_0_o_1_interaction_claim = self
            .sha_256_big_sigma_0_o_1_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.sha_256_big_sigma_0_o_1);
        let sha_256_big_sigma_1_o_0_interaction_claim = self
            .sha_256_big_sigma_1_o_0_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.sha_256_big_sigma_1_o_0);
        let sha_256_big_sigma_1_o_1_interaction_claim = self
            .sha_256_big_sigma_1_o_1_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.sha_256_big_sigma_1_o_1);
        let sha_256_small_sigma_0_o_0_interaction_claim = self
            .sha_256_small_sigma_0_o_0_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.sha_256_small_sigma_0_o_0,
            );
        let sha_256_small_sigma_0_o_1_interaction_claim = self
            .sha_256_small_sigma_0_o_1_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.sha_256_small_sigma_0_o_1,
            );
        let sha_256_small_sigma_1_o_0_interaction_claim = self
            .sha_256_small_sigma_1_o_0_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.sha_256_small_sigma_1_o_0,
            );
        let sha_256_small_sigma_1_o_1_interaction_claim = self
            .sha_256_small_sigma_1_o_1_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.sha_256_small_sigma_1_o_1,
            );
        let sha_256_k_table_interaction_claim = self
            .sha_256_k_table_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.sha_256_k_table);

        InteractionClaim {
            sha_256_round: sha_256_round_interaction_claim,
            sha_256_big_sigma_0: sha_256_big_sigma_0_interaction_claim,
            sha_256_big_sigma_1: sha_256_big_sigma_1_interaction_claim,
            sha_256_schedule: sha_256_schedule_interaction_claim,
            sha_256_small_sigma_0: sha_256_small_sigma_0_interaction_claim,
            sha_256_small_sigma_1: sha_256_small_sigma_1_interaction_claim,
            sha_256_big_sigma_0_o_0: sha_256_big_sigma_0_o_0_interaction_claim,
            sha_256_big_sigma_0_o_1: sha_256_big_sigma_0_o_1_interaction_claim,
            sha_256_big_sigma_1_o_0: sha_256_big_sigma_1_o_0_interaction_claim,
            sha_256_big_sigma_1_o_1: sha_256_big_sigma_1_o_1_interaction_claim,
            sha_256_small_sigma_0_o_0: sha_256_small_sigma_0_o_0_interaction_claim,
            sha_256_small_sigma_0_o_1: sha_256_small_sigma_0_o_1_interaction_claim,
            sha_256_small_sigma_1_o_0: sha_256_small_sigma_1_o_0_interaction_claim,
            sha_256_small_sigma_1_o_1: sha_256_small_sigma_1_o_1_interaction_claim,
            sha_256_k_table: sha_256_k_table_interaction_claim,
        }
    }
}

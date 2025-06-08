use cairo_air::air::CairoInteractionElements;
use cairo_air::poseidon::air::{
    Claim, InteractionClaim, PoseidonContextClaim, PoseidonContextInteractionClaim,
};
use tracing::{span, Level};

use crate::witness::components::{
    cube_252, poseidon_3_partial_rounds_chain, poseidon_full_round_chain, poseidon_round_keys,
    range_check_felt_252_width_27,
};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct PoseidonContextClaimGenerator {
    pub poseidon_3_partial_rounds_chain_trace_generator:
        poseidon_3_partial_rounds_chain::ClaimGenerator,
    pub poseidon_full_round_chain_trace_generator: poseidon_full_round_chain::ClaimGenerator,
    pub cube_252_trace_generator: cube_252::ClaimGenerator,
    pub poseidon_round_keys_trace_generator: poseidon_round_keys::ClaimGenerator,
    pub range_check_felt_252_width_27_trace_generator:
        range_check_felt_252_width_27::ClaimGenerator,
}
impl Default for PoseidonContextClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PoseidonContextClaimGenerator {
    pub fn new() -> Self {
        let poseidon_3_partial_rounds_chain_trace_generator =
            poseidon_3_partial_rounds_chain::ClaimGenerator::new();
        let poseidon_full_round_chain_trace_generator =
            poseidon_full_round_chain::ClaimGenerator::new();
        let cube_252_trace_generator = cube_252::ClaimGenerator::new();
        let poseidon_round_keys_trace_generator = poseidon_round_keys::ClaimGenerator::new();
        let range_check_felt_252_width_27_trace_generator =
            range_check_felt_252_width_27::ClaimGenerator::new();

        Self {
            poseidon_3_partial_rounds_chain_trace_generator,
            poseidon_full_round_chain_trace_generator,
            cube_252_trace_generator,
            poseidon_round_keys_trace_generator,
            range_check_felt_252_width_27_trace_generator,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
    ) -> (
        PoseidonContextClaim,
        PoseidonContextInteractionClaimGenerator,
    ) {
        let span = span!(Level::INFO, "write poseidon context trace").entered();
        if self
            .poseidon_3_partial_rounds_chain_trace_generator
            .is_empty()
        {
            return (
                PoseidonContextClaim { claim: None },
                PoseidonContextInteractionClaimGenerator { gen: None },
            );
        }
        let (
            poseidon_3_partial_rounds_chain_claim,
            poseidon_3_partial_rounds_chain_interaction_gen,
        ) = self
            .poseidon_3_partial_rounds_chain_trace_generator
            .write_trace(
                tree_builder,
                &mut self.cube_252_trace_generator,
                &self.poseidon_round_keys_trace_generator,
                &range_checks_trace_generator.rc_4_4_trace_generator,
                &range_checks_trace_generator.rc_4_4_4_4_trace_generator,
                &mut self.range_check_felt_252_width_27_trace_generator,
            );
        let (poseidon_full_round_chain_claim, poseidon_full_round_chain_interaction_gen) =
            self.poseidon_full_round_chain_trace_generator.write_trace(
                tree_builder,
                &mut self.cube_252_trace_generator,
                &self.poseidon_round_keys_trace_generator,
                &range_checks_trace_generator.rc_3_3_3_3_3_trace_generator,
            );
        let (cube_252_claim, cube_252_interaction_gen) = self.cube_252_trace_generator.write_trace(
            tree_builder,
            &range_checks_trace_generator.rc_19_trace_generator,
            &range_checks_trace_generator.rc_19_b_trace_generator,
            &range_checks_trace_generator.rc_19_c_trace_generator,
            &range_checks_trace_generator.rc_19_d_trace_generator,
            &range_checks_trace_generator.rc_19_e_trace_generator,
            &range_checks_trace_generator.rc_19_f_trace_generator,
            &range_checks_trace_generator.rc_19_g_trace_generator,
            &range_checks_trace_generator.rc_19_h_trace_generator,
            &range_checks_trace_generator.rc_9_9_trace_generator,
            &range_checks_trace_generator.rc_9_9_b_trace_generator,
            &range_checks_trace_generator.rc_9_9_c_trace_generator,
            &range_checks_trace_generator.rc_9_9_d_trace_generator,
            &range_checks_trace_generator.rc_9_9_e_trace_generator,
            &range_checks_trace_generator.rc_9_9_f_trace_generator,
            &range_checks_trace_generator.rc_9_9_g_trace_generator,
            &range_checks_trace_generator.rc_9_9_h_trace_generator,
        );
        let (poseidon_round_keys_claim, poseidon_round_keys_interaction_gen) = self
            .poseidon_round_keys_trace_generator
            .write_trace(tree_builder);
        let (range_check_felt_252_width_27_claim, range_check_felt_252_width_27_interaction_gen) =
            self.range_check_felt_252_width_27_trace_generator
                .write_trace(
                    tree_builder,
                    &range_checks_trace_generator.rc_18_trace_generator,
                    &range_checks_trace_generator.rc_18_b_trace_generator,
                    &range_checks_trace_generator.rc_9_9_trace_generator,
                    &range_checks_trace_generator.rc_9_9_b_trace_generator,
                    &range_checks_trace_generator.rc_9_9_c_trace_generator,
                    &range_checks_trace_generator.rc_9_9_d_trace_generator,
                    &range_checks_trace_generator.rc_9_9_e_trace_generator,
                );
        span.exit();

        let claim = Some(Claim {
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim,
            poseidon_full_round_chain: poseidon_full_round_chain_claim,
            cube_252: cube_252_claim,
            poseidon_round_keys: poseidon_round_keys_claim,
            range_check_felt_252_width_27: range_check_felt_252_width_27_claim,
        });
        let gen = Some(InteractionClaimGenerator {
            poseidon_3_partial_rounds_chain_interaction_gen,
            poseidon_full_round_chain_interaction_gen,
            cube_252_interaction_gen,
            poseidon_round_keys_interaction_gen,
            range_check_felt_252_width_27_interaction_gen,
        });
        (
            PoseidonContextClaim { claim },
            PoseidonContextInteractionClaimGenerator { gen },
        )
    }
}

pub struct PoseidonContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl PoseidonContextInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> PoseidonContextInteractionClaim {
        PoseidonContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, interaction_elements)),
        }
    }
}

struct InteractionClaimGenerator {
    poseidon_3_partial_rounds_chain_interaction_gen:
        poseidon_3_partial_rounds_chain::InteractionClaimGenerator,
    poseidon_full_round_chain_interaction_gen: poseidon_full_round_chain::InteractionClaimGenerator,
    cube_252_interaction_gen: cube_252::InteractionClaimGenerator,
    poseidon_round_keys_interaction_gen: poseidon_round_keys::InteractionClaimGenerator,
    range_check_felt_252_width_27_interaction_gen:
        range_check_felt_252_width_27::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim {
        let poseidon_3_partial_rounds_chain_interaction_claim = self
            .poseidon_3_partial_rounds_chain_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.cube_252,
                &interaction_elements.poseidon_3_partial_rounds_chain,
                &interaction_elements.poseidon_round_keys,
                &interaction_elements.range_check_felt_252_width_27,
                &interaction_elements.range_checks.rc_4_4,
                &interaction_elements.range_checks.rc_4_4_4_4,
            );
        let poseidon_full_round_chain_interaction_claim = self
            .poseidon_full_round_chain_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.cube_252,
                &interaction_elements.poseidon_full_round_chain,
                &interaction_elements.poseidon_round_keys,
                &interaction_elements.range_checks.rc_3_3_3_3_3,
            );
        let cube_252_interaction_claim = self.cube_252_interaction_gen.write_interaction_trace(
            tree_builder,
            &interaction_elements.cube_252,
            &interaction_elements.range_checks.rc_19,
            &interaction_elements.range_checks.rc_19_b,
            &interaction_elements.range_checks.rc_19_c,
            &interaction_elements.range_checks.rc_19_d,
            &interaction_elements.range_checks.rc_19_e,
            &interaction_elements.range_checks.rc_19_f,
            &interaction_elements.range_checks.rc_19_g,
            &interaction_elements.range_checks.rc_19_h,
            &interaction_elements.range_checks.rc_9_9,
            &interaction_elements.range_checks.rc_9_9_b,
            &interaction_elements.range_checks.rc_9_9_c,
            &interaction_elements.range_checks.rc_9_9_d,
            &interaction_elements.range_checks.rc_9_9_e,
            &interaction_elements.range_checks.rc_9_9_f,
            &interaction_elements.range_checks.rc_9_9_g,
            &interaction_elements.range_checks.rc_9_9_h,
        );
        let poseidon_round_keys_interaction_claim = self
            .poseidon_round_keys_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.poseidon_round_keys);
        let range_check_felt_252_width_27_interaction_claim = self
            .range_check_felt_252_width_27_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.range_check_felt_252_width_27,
                &interaction_elements.range_checks.rc_18,
                &interaction_elements.range_checks.rc_18_b,
                &interaction_elements.range_checks.rc_9_9,
                &interaction_elements.range_checks.rc_9_9_b,
                &interaction_elements.range_checks.rc_9_9_c,
                &interaction_elements.range_checks.rc_9_9_d,
                &interaction_elements.range_checks.rc_9_9_e,
            );

        InteractionClaim {
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_interaction_claim,
            poseidon_full_round_chain: poseidon_full_round_chain_interaction_claim,
            cube_252: cube_252_interaction_claim,
            poseidon_round_keys: poseidon_round_keys_interaction_claim,
            range_check_felt_252_width_27: range_check_felt_252_width_27_interaction_claim,
        }
    }
}

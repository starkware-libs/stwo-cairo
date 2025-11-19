use cairo_air::poseidon::air::{
    Claim, InteractionClaim, PoseidonContextClaim, PoseidonContextInteractionClaim,
};
use cairo_air::relations::CommonLookupElements;
use tracing::{span, Level};

use crate::witness::components::{
    cube_252, memory_id_to_big, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_252_width_27,
};
use crate::witness::prelude::*;
use crate::witness::range_checks::RangeChecksClaimGenerator;
use crate::witness::utils::TreeBuilder;

pub struct PoseidonContextClaimGenerator {
    pub poseidon_aggregator_trace_generator: poseidon_aggregator::ClaimGenerator,
    pub poseidon_3_partial_rounds_chain_trace_generator:
        poseidon_3_partial_rounds_chain::ClaimGenerator,
    pub poseidon_full_round_chain_trace_generator: poseidon_full_round_chain::ClaimGenerator,
    pub cube_252_trace_generator: cube_252::ClaimGenerator,
    pub poseidon_round_keys_trace_generator: poseidon_round_keys::ClaimGenerator,
    pub range_check_252_width_27_trace_generator: range_check_252_width_27::ClaimGenerator,
}

impl PoseidonContextClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let poseidon_aggregator_trace_generator = poseidon_aggregator::ClaimGenerator::new();
        let poseidon_3_partial_rounds_chain_trace_generator =
            poseidon_3_partial_rounds_chain::ClaimGenerator::new();
        let poseidon_full_round_chain_trace_generator =
            poseidon_full_round_chain::ClaimGenerator::new();
        let cube_252_trace_generator = cube_252::ClaimGenerator::new();
        let poseidon_round_keys_trace_generator =
            poseidon_round_keys::ClaimGenerator::new(preprocessed_trace);
        let range_check_252_width_27_trace_generator =
            range_check_252_width_27::ClaimGenerator::new();

        Self {
            poseidon_aggregator_trace_generator,
            poseidon_3_partial_rounds_chain_trace_generator,
            poseidon_full_round_chain_trace_generator,
            cube_252_trace_generator,
            poseidon_round_keys_trace_generator,
            range_check_252_width_27_trace_generator,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_id_to_big_trace_generator: &memory_id_to_big::ClaimGenerator,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
    ) -> (
        PoseidonContextClaim,
        PoseidonContextInteractionClaimGenerator,
    ) {
        let span = span!(Level::INFO, "write poseidon context trace").entered();
        if self.poseidon_aggregator_trace_generator.is_empty() {
            return (
                PoseidonContextClaim { claim: None },
                PoseidonContextInteractionClaimGenerator { gen: None },
            );
        }
        let (poseidon_aggregator_claim, poseidon_aggregator_interaction_gen) =
            self.poseidon_aggregator_trace_generator.write_trace(
                tree_builder,
                memory_id_to_big_trace_generator,
                &mut self.poseidon_full_round_chain_trace_generator,
                &mut self.range_check_252_width_27_trace_generator,
                &mut self.cube_252_trace_generator,
                &range_checks_trace_generator.rc_3_3_3_3_3_trace_generator,
                &range_checks_trace_generator.rc_4_4_4_4_trace_generator,
                &range_checks_trace_generator.rc_4_4_trace_generator,
                &mut self.poseidon_3_partial_rounds_chain_trace_generator,
            );
        let (
            poseidon_3_partial_rounds_chain_claim,
            poseidon_3_partial_rounds_chain_interaction_gen,
        ) = self
            .poseidon_3_partial_rounds_chain_trace_generator
            .write_trace(
                tree_builder,
                &self.poseidon_round_keys_trace_generator,
                &mut self.cube_252_trace_generator,
                &range_checks_trace_generator.rc_4_4_4_4_trace_generator,
                &range_checks_trace_generator.rc_4_4_trace_generator,
                &mut self.range_check_252_width_27_trace_generator,
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
            &range_checks_trace_generator.rc_9_9_trace_generator,
            &range_checks_trace_generator.rc_20_trace_generator,
        );
        let (poseidon_round_keys_claim, poseidon_round_keys_interaction_gen) = self
            .poseidon_round_keys_trace_generator
            .write_trace(tree_builder);
        let (range_check_felt_252_width_27_claim, range_check_felt_252_width_27_interaction_gen) =
            self.range_check_252_width_27_trace_generator.write_trace(
                tree_builder,
                &range_checks_trace_generator.rc_9_9_trace_generator,
                &range_checks_trace_generator.rc_18_trace_generator,
            );
        span.exit();

        let claim = Some(Claim {
            poseidon_aggregator: poseidon_aggregator_claim,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim,
            poseidon_full_round_chain: poseidon_full_round_chain_claim,
            cube_252: cube_252_claim,
            poseidon_round_keys: poseidon_round_keys_claim,
            range_check_252_width_27: range_check_felt_252_width_27_claim,
        });
        let gen = Some(InteractionClaimGenerator {
            poseidon_aggregator_interaction_gen,
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
        common_lookup_elements: &CommonLookupElements,
    ) -> PoseidonContextInteractionClaim {
        PoseidonContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, common_lookup_elements)),
        }
    }
}

struct InteractionClaimGenerator {
    poseidon_aggregator_interaction_gen: poseidon_aggregator::InteractionClaimGenerator,
    poseidon_3_partial_rounds_chain_interaction_gen:
        poseidon_3_partial_rounds_chain::InteractionClaimGenerator,
    poseidon_full_round_chain_interaction_gen: poseidon_full_round_chain::InteractionClaimGenerator,
    cube_252_interaction_gen: cube_252::InteractionClaimGenerator,
    poseidon_round_keys_interaction_gen: poseidon_round_keys::InteractionClaimGenerator,
    range_check_felt_252_width_27_interaction_gen:
        range_check_252_width_27::InteractionClaimGenerator,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> InteractionClaim {
        let poseidon_aggregator_interaction_claim = self
            .poseidon_aggregator_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let poseidon_3_partial_rounds_chain_interaction_claim = self
            .poseidon_3_partial_rounds_chain_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let poseidon_full_round_chain_interaction_claim = self
            .poseidon_full_round_chain_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let cube_252_interaction_claim = self
            .cube_252_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let poseidon_round_keys_interaction_claim = self
            .poseidon_round_keys_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let range_check_felt_252_width_27_interaction_claim = self
            .range_check_felt_252_width_27_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);

        InteractionClaim {
            poseidon_aggregator: poseidon_aggregator_interaction_claim,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_interaction_claim,
            poseidon_full_round_chain: poseidon_full_round_chain_interaction_claim,
            cube_252: cube_252_interaction_claim,
            poseidon_round_keys: poseidon_round_keys_interaction_claim,
            range_check_252_width_27: range_check_felt_252_width_27_interaction_claim,
        }
    }
}

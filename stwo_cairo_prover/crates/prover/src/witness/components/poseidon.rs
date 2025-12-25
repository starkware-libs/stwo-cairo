use cairo_air::air::CairoInteractionElements;
use cairo_air::poseidon::air::{
    Claim, InteractionClaim, PoseidonContextClaim, PoseidonContextInteractionClaim,
};
use stwo::prover::backend::simd::SimdBackend;
use tracing::{span, Level};

use crate::witness::components::{
    cube_252, memory_id_to_big, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_18, range_check_20,
    range_check_252_width_27, range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
    range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

pub fn poseidon_context_write_trace(
    poseidon_aggregator_trace_generator: Option<poseidon_aggregator::ClaimGenerator>,
    poseidon_3_partial_rounds_chain_trace_generator: Option<
        poseidon_3_partial_rounds_chain::ClaimGenerator,
    >,
    poseidon_full_round_chain_trace_generator: Option<poseidon_full_round_chain::ClaimGenerator>,
    cube_252_trace_generator: Option<cube_252::ClaimGenerator>,
    poseidon_round_keys_trace_generator: Option<poseidon_round_keys::ClaimGenerator>,
    range_check_252_width_27_trace_generator: Option<range_check_252_width_27::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    memory_id_to_big_trace_generator: Option<&memory_id_to_big::ClaimGenerator>,
    rc_3_3_3_3_3_trace_generator: Option<&range_check_3_3_3_3_3::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<&range_check_4_4_4_4::ClaimGenerator>,
    rc_4_4_trace_generator: Option<&range_check_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<&range_check_9_9::ClaimGenerator>,
    rc_18_trace_generator: Option<&range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<&range_check_20::ClaimGenerator>,
) -> (
    PoseidonContextClaim,
    PoseidonContextInteractionClaimGenerator,
) {
    let span = span!(Level::INFO, "write poseidon context trace").entered();
    if poseidon_aggregator_trace_generator.is_none() {
        return (
            PoseidonContextClaim { claim: None },
            PoseidonContextInteractionClaimGenerator { gen: None },
        );
    }
    let poseidon_aggregator_trace_generator = poseidon_aggregator_trace_generator
        .expect("Should have poseidon aggregator trace generator at this point");
    let mut poseidon_3_partial_rounds_chain_trace_generator =
        poseidon_3_partial_rounds_chain_trace_generator
            .expect("Should have poseidon 3 partial rounds chain trace generator at this point");
    let mut poseidon_full_round_chain_trace_generator =
        poseidon_full_round_chain_trace_generator
            .expect("Should have poseidon full round chain trace generator at this point");
    let mut cube_252_trace_generator = cube_252_trace_generator
        .expect("Should have cube 252 trace generator at this point");
    let poseidon_round_keys_trace_generator = poseidon_round_keys_trace_generator
        .expect("Should have poseidon round keys trace generator at this point");
    let mut range_check_252_width_27_trace_generator =
        range_check_252_width_27_trace_generator
            .expect("Should have range check 252 width 27 trace generator at this point");

    let (poseidon_aggregator_claim, poseidon_aggregator_interaction_gen) =
        poseidon_aggregator_trace_generator.write_trace(
            tree_builder,
            memory_id_to_big_trace_generator.unwrap(),
            &mut poseidon_full_round_chain_trace_generator,
            &mut range_check_252_width_27_trace_generator,
            &mut cube_252_trace_generator,
            rc_3_3_3_3_3_trace_generator.unwrap(),
            rc_4_4_4_4_trace_generator.unwrap(),
            rc_4_4_trace_generator.unwrap(),
            &mut poseidon_3_partial_rounds_chain_trace_generator,
        );
    let (poseidon_3_partial_rounds_chain_claim, poseidon_3_partial_rounds_chain_interaction_gen) =
        poseidon_3_partial_rounds_chain_trace_generator.write_trace(
            tree_builder,
            &poseidon_round_keys_trace_generator,
            &mut cube_252_trace_generator,
            rc_4_4_4_4_trace_generator.unwrap(),
            rc_4_4_trace_generator.unwrap(),
            &mut range_check_252_width_27_trace_generator,
        );
    let (poseidon_full_round_chain_claim, poseidon_full_round_chain_interaction_gen) =
        poseidon_full_round_chain_trace_generator.write_trace(
            tree_builder,
            &mut cube_252_trace_generator,
            &poseidon_round_keys_trace_generator,
            rc_3_3_3_3_3_trace_generator.unwrap(),
        );
    let (cube_252_claim, cube_252_interaction_gen) = cube_252_trace_generator.write_trace(
        tree_builder,
        rc_9_9_trace_generator.unwrap(),
        rc_20_trace_generator.unwrap(),
    );
    let (poseidon_round_keys_claim, poseidon_round_keys_interaction_gen) =
        poseidon_round_keys_trace_generator.write_trace(tree_builder);
    let (range_check_felt_252_width_27_claim, range_check_felt_252_width_27_interaction_gen) =
        range_check_252_width_27_trace_generator.write_trace(
            tree_builder,
            rc_9_9_trace_generator.unwrap(),
            rc_18_trace_generator.unwrap(),
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
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim {
        let poseidon_aggregator_interaction_claim = self
            .poseidon_aggregator_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.poseidon_full_round_chain,
                &interaction_elements.range_check_252_width_27,
                &interaction_elements.cube_252,
                &interaction_elements.range_checks.rc_3_3_3_3_3,
                &interaction_elements.range_checks.rc_4_4_4_4,
                &interaction_elements.range_checks.rc_4_4,
                &interaction_elements.poseidon_3_partial_rounds_chain,
                &interaction_elements.poseidon_aggregator,
            );
        let poseidon_3_partial_rounds_chain_interaction_claim = self
            .poseidon_3_partial_rounds_chain_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.poseidon_round_keys,
                &interaction_elements.cube_252,
                &interaction_elements.range_checks.rc_4_4_4_4,
                &interaction_elements.range_checks.rc_4_4,
                &interaction_elements.range_check_252_width_27,
                &interaction_elements.poseidon_3_partial_rounds_chain,
            );
        let poseidon_full_round_chain_interaction_claim = self
            .poseidon_full_round_chain_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.cube_252,
                &interaction_elements.poseidon_round_keys,
                &interaction_elements.range_checks.rc_3_3_3_3_3,
                &interaction_elements.poseidon_full_round_chain,
            );
        let cube_252_interaction_claim = self.cube_252_interaction_gen.write_interaction_trace(
            tree_builder,
            &interaction_elements.range_checks.rc_9_9,
            &interaction_elements.range_checks.rc_9_9_b,
            &interaction_elements.range_checks.rc_9_9_c,
            &interaction_elements.range_checks.rc_9_9_d,
            &interaction_elements.range_checks.rc_9_9_e,
            &interaction_elements.range_checks.rc_9_9_f,
            &interaction_elements.range_checks.rc_9_9_g,
            &interaction_elements.range_checks.rc_9_9_h,
            &interaction_elements.range_checks.rc_20,
            &interaction_elements.range_checks.rc_20_b,
            &interaction_elements.range_checks.rc_20_c,
            &interaction_elements.range_checks.rc_20_d,
            &interaction_elements.range_checks.rc_20_e,
            &interaction_elements.range_checks.rc_20_f,
            &interaction_elements.range_checks.rc_20_g,
            &interaction_elements.range_checks.rc_20_h,
            &interaction_elements.cube_252,
        );
        let poseidon_round_keys_interaction_claim = self
            .poseidon_round_keys_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.poseidon_round_keys);
        let range_check_felt_252_width_27_interaction_claim = self
            .range_check_felt_252_width_27_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.range_checks.rc_9_9,
                &interaction_elements.range_checks.rc_18,
                &interaction_elements.range_checks.rc_9_9_b,
                &interaction_elements.range_checks.rc_18_b,
                &interaction_elements.range_checks.rc_9_9_c,
                &interaction_elements.range_checks.rc_9_9_d,
                &interaction_elements.range_checks.rc_9_9_e,
                &interaction_elements.range_check_252_width_27,
            );

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

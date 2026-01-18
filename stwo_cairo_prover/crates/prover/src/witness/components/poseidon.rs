use cairo_air::components::{
    cube_252 as cube_252_claim,
    poseidon_3_partial_rounds_chain as poseidon_3_partial_rounds_chain_claim,
    poseidon_aggregator as poseidon_aggregator_claim,
    poseidon_full_round_chain as poseidon_full_round_chain_claim,
    poseidon_round_keys as poseidon_round_keys_claim,
    range_check_252_width_27 as range_check_252_width_27_claim,
};
use cairo_air::relations::CommonLookupElements;
use stwo::prover::backend::simd::SimdBackend;
use tracing::{span, Level};

use crate::witness::components::{
    cube_252, memory_id_to_big, poseidon_3_partial_rounds_chain, poseidon_aggregator,
    poseidon_full_round_chain, poseidon_round_keys, range_check_18, range_check_20,
    range_check_252_width_27, range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
    range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

#[allow(clippy::type_complexity)]
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
    Option<poseidon_aggregator_claim::Claim>,
    Option<poseidon_3_partial_rounds_chain_claim::Claim>,
    Option<poseidon_full_round_chain_claim::Claim>,
    Option<cube_252_claim::Claim>,
    Option<poseidon_round_keys_claim::Claim>,
    Option<range_check_252_width_27_claim::Claim>,
    PoseidonContextInteractionClaimGenerator,
) {
    let span = span!(Level::INFO, "write poseidon context trace").entered();
    if poseidon_aggregator_trace_generator.is_none() {
        return (
            None,
            None,
            None,
            None,
            None,
            None,
            PoseidonContextInteractionClaimGenerator { gen: None },
        );
    }
    let poseidon_aggregator_trace_generator = poseidon_aggregator_trace_generator
        .expect("Should have poseidon aggregator trace generator at this point");
    let mut poseidon_3_partial_rounds_chain_trace_generator =
        poseidon_3_partial_rounds_chain_trace_generator
            .expect("Should have poseidon 3 partial rounds chain trace generator at this point");
    let mut poseidon_full_round_chain_trace_generator = poseidon_full_round_chain_trace_generator
        .expect("Should have poseidon full round chain trace generator at this point");
    let mut cube_252_trace_generator =
        cube_252_trace_generator.expect("Should have cube 252 trace generator at this point");
    let poseidon_round_keys_trace_generator = poseidon_round_keys_trace_generator
        .expect("Should have poseidon round keys trace generator at this point");
    let mut range_check_252_width_27_trace_generator = range_check_252_width_27_trace_generator
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

    let gen = Some(InteractionClaimGenerator {
        poseidon_aggregator_interaction_gen,
        poseidon_3_partial_rounds_chain_interaction_gen,
        poseidon_full_round_chain_interaction_gen,
        cube_252_interaction_gen,
        poseidon_round_keys_interaction_gen,
        range_check_felt_252_width_27_interaction_gen,
    });

    (
        Some(poseidon_aggregator_claim),
        Some(poseidon_3_partial_rounds_chain_claim),
        Some(poseidon_full_round_chain_claim),
        Some(cube_252_claim),
        Some(poseidon_round_keys_claim),
        Some(range_check_felt_252_width_27_claim),
        PoseidonContextInteractionClaimGenerator { gen },
    )
}

pub struct PoseidonContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl PoseidonContextInteractionClaimGenerator {
    #[allow(clippy::type_complexity)]
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> (
        Option<poseidon_aggregator_claim::InteractionClaim>,
        Option<poseidon_3_partial_rounds_chain_claim::InteractionClaim>,
        Option<poseidon_full_round_chain_claim::InteractionClaim>,
        Option<cube_252_claim::InteractionClaim>,
        Option<poseidon_round_keys_claim::InteractionClaim>,
        Option<range_check_252_width_27_claim::InteractionClaim>,
    ) {
        self.gen
            .map_or((None, None, None, None, None, None), |gen| {
                gen.write_interaction_trace(tree_builder, common_lookup_elements)
            })
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
    #[allow(clippy::type_complexity)]
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> (
        Option<poseidon_aggregator_claim::InteractionClaim>,
        Option<poseidon_3_partial_rounds_chain_claim::InteractionClaim>,
        Option<poseidon_full_round_chain_claim::InteractionClaim>,
        Option<cube_252_claim::InteractionClaim>,
        Option<poseidon_round_keys_claim::InteractionClaim>,
        Option<range_check_252_width_27_claim::InteractionClaim>,
    ) {
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

        (
            Some(poseidon_aggregator_interaction_claim),
            Some(poseidon_3_partial_rounds_chain_interaction_claim),
            Some(poseidon_full_round_chain_interaction_claim),
            Some(cube_252_interaction_claim),
            Some(poseidon_round_keys_interaction_claim),
            Some(range_check_felt_252_width_27_interaction_claim),
        )
    }
}

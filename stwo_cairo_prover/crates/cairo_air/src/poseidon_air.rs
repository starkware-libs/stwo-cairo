use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_prover::components::{
    cube_252, poseidon_3_partial_rounds_chain, poseidon_full_round_chain, poseidon_round_keys,
    range_check_felt_252_width_27,
};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use tracing::{span, Level};

use crate::air::CairoInteractionElements;
use crate::debug_tools::indented_component_display;
use crate::range_checks_air::RangeChecksClaimGenerator;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PoseidonContextClaim {
    pub claim: Option<Claim>,
}
impl PoseidonContextClaim {
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
pub struct Claim {
    pub poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain::Claim,
    pub poseidon_full_round_chain: poseidon_full_round_chain::Claim,
    pub cube_252: cube_252::Claim,
    pub poseidon_round_keys: poseidon_round_keys::Claim,
    pub range_check_felt_252_width_27: range_check_felt_252_width_27::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.poseidon_3_partial_rounds_chain.mix_into(channel);
        self.poseidon_full_round_chain.mix_into(channel);
        self.cube_252.mix_into(channel);
        self.poseidon_round_keys.mix_into(channel);
        self.range_check_felt_252_width_27.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.poseidon_3_partial_rounds_chain.log_sizes(),
            self.poseidon_full_round_chain.log_sizes(),
            self.cube_252.log_sizes(),
            self.poseidon_round_keys.log_sizes(),
            self.range_check_felt_252_width_27.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }
}

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

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
    ) -> (
        PoseidonContextClaim,
        PoseidonContextInteractionClaimGenerator,
    )
    where
        SimdBackend: BackendForChannel<MC>,
    {
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
            &range_checks_trace_generator.rc_9_9_trace_generator,
        );
        let (poseidon_round_keys_claim, poseidon_round_keys_interaction_gen) = self
            .poseidon_round_keys_trace_generator
            .write_trace(tree_builder);
        let (range_check_felt_252_width_27_claim, range_check_felt_252_width_27_interaction_gen) =
            self.range_check_felt_252_width_27_trace_generator
                .write_trace(
                    tree_builder,
                    &range_checks_trace_generator.rc_18_trace_generator,
                    &range_checks_trace_generator.rc_9_9_trace_generator,
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
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> PoseidonContextInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
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
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
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
            &interaction_elements.range_checks.rc_9_9,
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
                &interaction_elements.range_checks.rc_9_9,
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

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PoseidonContextInteractionClaim {
    claim: Option<InteractionClaim>,
}
impl PoseidonContextInteractionClaim {
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
    poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain::InteractionClaim,
    poseidon_full_round_chain: poseidon_full_round_chain::InteractionClaim,
    cube_252: cube_252::InteractionClaim,
    poseidon_round_keys: poseidon_round_keys::InteractionClaim,
    range_check_felt_252_width_27: range_check_felt_252_width_27::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.poseidon_3_partial_rounds_chain.mix_into(channel);
        self.poseidon_full_round_chain.mix_into(channel);
        self.cube_252.mix_into(channel);
        self.poseidon_round_keys.mix_into(channel);
        self.range_check_felt_252_width_27.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        let mut sum = QM31::zero();
        sum += self.poseidon_3_partial_rounds_chain.claimed_sum;
        sum += self.poseidon_full_round_chain.claimed_sum;
        sum += self.cube_252.claimed_sum;
        sum += self.poseidon_round_keys.claimed_sum;
        sum += self.range_check_felt_252_width_27.claimed_sum;
        sum
    }
}

pub struct PoseidonContextComponents {
    pub components: Option<Components>,
}
impl PoseidonContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PoseidonContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &PoseidonContextInteractionClaim,
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

impl std::fmt::Display for PoseidonContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{}", components),
            None => write!(f, "No Poseidon Context Components"),
        }
    }
}

pub struct Components {
    pub poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain::Component,
    pub poseidon_full_round_chain: poseidon_full_round_chain::Component,
    pub cube_252: cube_252::Component,
    pub poseidon_round_keys: poseidon_round_keys::Component,
    pub range_check_felt_252_width_27: range_check_felt_252_width_27::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PoseidonContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let poseidon_3_partial_rounds_chain_component =
            poseidon_3_partial_rounds_chain::Component::new(
                tree_span_provider,
                poseidon_3_partial_rounds_chain::Eval {
                    claim: claim
                        .claim
                        .as_ref()
                        .unwrap()
                        .poseidon_3_partial_rounds_chain,
                    cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                    poseidon_3_partial_rounds_chain_lookup_elements: interaction_elements
                        .poseidon_3_partial_rounds_chain
                        .clone(),
                    poseidon_round_keys_lookup_elements: interaction_elements
                        .poseidon_round_keys
                        .clone(),
                    range_check_4_4_lookup_elements: interaction_elements
                        .range_checks
                        .rc_4_4
                        .clone(),
                    range_check_4_4_4_4_lookup_elements: interaction_elements
                        .range_checks
                        .rc_4_4_4_4
                        .clone(),
                    range_check_felt_252_width_27_lookup_elements: interaction_elements
                        .range_check_felt_252_width_27
                        .clone(),
                },
                interaction_claim
                    .poseidon_3_partial_rounds_chain
                    .claimed_sum,
            );
        let poseidon_full_round_chain_component = poseidon_full_round_chain::Component::new(
            tree_span_provider,
            poseidon_full_round_chain::Eval {
                claim: claim.claim.as_ref().unwrap().poseidon_full_round_chain,
                cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                poseidon_full_round_chain_lookup_elements: interaction_elements
                    .poseidon_full_round_chain
                    .clone(),
                poseidon_round_keys_lookup_elements: interaction_elements
                    .poseidon_round_keys
                    .clone(),
                range_check_3_3_3_3_3_lookup_elements: interaction_elements
                    .range_checks
                    .rc_3_3_3_3_3
                    .clone(),
            },
            interaction_claim.poseidon_full_round_chain.claimed_sum,
        );
        let cube_252_component = cube_252::Component::new(
            tree_span_provider,
            cube_252::Eval {
                claim: claim.claim.as_ref().unwrap().cube_252,
                cube_252_lookup_elements: interaction_elements.cube_252.clone(),
                range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
                range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            },
            interaction_claim.cube_252.claimed_sum,
        );
        let poseidon_round_keys_component = poseidon_round_keys::Component::new(
            tree_span_provider,
            poseidon_round_keys::Eval {
                claim: claim.claim.as_ref().unwrap().poseidon_round_keys,
                poseidon_round_keys_lookup_elements: interaction_elements
                    .poseidon_round_keys
                    .clone(),
            },
            interaction_claim.poseidon_round_keys.claimed_sum,
        );
        let range_check_felt_252_width_27_component = range_check_felt_252_width_27::Component::new(
            tree_span_provider,
            range_check_felt_252_width_27::Eval {
                claim: claim.claim.as_ref().unwrap().range_check_felt_252_width_27,
                range_check_felt_252_width_27_lookup_elements: (interaction_elements
                    .range_check_felt_252_width_27
                    .clone()),
                range_check_18_lookup_elements: (interaction_elements.range_checks.rc_18.clone()),
                range_check_9_9_lookup_elements: (interaction_elements.range_checks.rc_9_9.clone()),
            },
            interaction_claim.range_check_felt_252_width_27.claimed_sum,
        );
        Self {
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_felt_252_width_27: range_check_felt_252_width_27_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![
            &self.poseidon_3_partial_rounds_chain,
            &self.poseidon_full_round_chain,
            &self.cube_252,
            &self.poseidon_round_keys,
            &self.range_check_felt_252_width_27,
        ]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Poseidon3PartialRoundsChain: {}",
            indented_component_display(&self.poseidon_3_partial_rounds_chain)
        )?;
        writeln!(
            f,
            "PoseidonFullRoundChain: {}",
            indented_component_display(&self.poseidon_full_round_chain)
        )?;
        writeln!(f, "Cube252: {}", indented_component_display(&self.cube_252))?;
        writeln!(
            f,
            "PoseidonRoundKeys: {}",
            indented_component_display(&self.poseidon_round_keys)
        )?;
        writeln!(
            f,
            "RangeCheckFelt252Width27: {}",
            indented_component_display(&self.range_check_felt_252_width_27)
        )?;

        Ok(())
    }
}

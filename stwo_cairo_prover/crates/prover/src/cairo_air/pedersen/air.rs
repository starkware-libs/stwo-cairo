use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::TraceLocationAllocator;
use stwo_prover::core::air::ComponentProver;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use tracing::{span, Level};

use crate::cairo_air::air::CairoInteractionElements;
use crate::cairo_air::debug_tools::indented_component_display;
use crate::cairo_air::range_checks_air::RangeChecksClaimGenerator;
use crate::components::{partial_ec_mul, pedersen_points_table};

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PedersenContextClaim {
    pub claim: Option<Claim>,
}
impl PedersenContextClaim {
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
    pub partial_ec_mul: partial_ec_mul::Claim,
    pub pedersen_points_table: pedersen_points_table::Claim,
}
impl Claim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.partial_ec_mul.mix_into(channel);
        self.pedersen_points_table.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes = [
            self.partial_ec_mul.log_sizes(),
            self.pedersen_points_table.log_sizes(),
        ]
        .into_iter();

        TreeVec::concat_cols(log_sizes)
    }
}

pub struct PedersenContextClaimGenerator {
    pub partial_ec_mul_trace_generator: partial_ec_mul::ClaimGenerator,
    pub pedersen_points_table_trace_generator: pedersen_points_table::ClaimGenerator,
}
impl Default for PedersenContextClaimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl PedersenContextClaimGenerator {
    pub fn new() -> Self {
        let partial_ec_mul_trace_generator = partial_ec_mul::ClaimGenerator::new();
        let pedersen_points_table_trace_generator = pedersen_points_table::ClaimGenerator::new();

        Self {
            partial_ec_mul_trace_generator,
            pedersen_points_table_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_checks_trace_generator: &RangeChecksClaimGenerator,
    ) -> (
        PedersenContextClaim,
        PedersenContextInteractionClaimGenerator,
    )
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let span = span!(Level::INFO, "write pedersen context trace").entered();
        if self.partial_ec_mul_trace_generator.is_empty() {
            return (
                PedersenContextClaim { claim: None },
                PedersenContextInteractionClaimGenerator { gen: None },
            );
        }
        let (partial_ec_mul_claim, partial_ec_mul_interaction_gen) =
            self.partial_ec_mul_trace_generator.write_trace(
                tree_builder,
                &self.pedersen_points_table_trace_generator,
                &range_checks_trace_generator.rc_19_trace_generator,
                &range_checks_trace_generator.rc_9_9_trace_generator,
            );
        let (pedersen_points_table_claim, pedersen_points_table_interaction_gen) = self
            .pedersen_points_table_trace_generator
            .write_trace(tree_builder);
        span.exit();

        let claim = Some(Claim {
            partial_ec_mul: partial_ec_mul_claim,
            pedersen_points_table: pedersen_points_table_claim,
        });
        let gen = Some(InteractionClaimGenerator {
            partial_ec_mul_interaction_gen,
            pedersen_points_table_interaction_gen,
        });
        (
            PedersenContextClaim { claim },
            PedersenContextInteractionClaimGenerator { gen },
        )
    }
}

pub struct PedersenContextInteractionClaimGenerator {
    gen: Option<InteractionClaimGenerator>,
}
impl PedersenContextInteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> PedersenContextInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        PedersenContextInteractionClaim {
            claim: self
                .gen
                .map(|gen| gen.write_interaction_trace(tree_builder, interaction_elements)),
        }
    }
}

struct InteractionClaimGenerator {
    partial_ec_mul_interaction_gen: partial_ec_mul::InteractionClaimGenerator,
    pedersen_points_table_interaction_gen: pedersen_points_table::InteractionClaimGenerator,
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
        let partial_ec_mul_interaction_claim =
            self.partial_ec_mul_interaction_gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.partial_ec_mul,
                &interaction_elements.pedersen_points_table,
                &interaction_elements.range_checks.rc_19,
                &interaction_elements.range_checks.rc_9_9,
            );
        let pedersen_points_table_interaction_claim = self
            .pedersen_points_table_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.pedersen_points_table);

        InteractionClaim {
            partial_ec_mul: partial_ec_mul_interaction_claim,
            pedersen_points_table: pedersen_points_table_interaction_claim,
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PedersenContextInteractionClaim {
    claim: Option<InteractionClaim>,
}
impl PedersenContextInteractionClaim {
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
    partial_ec_mul: partial_ec_mul::InteractionClaim,
    pedersen_points_table: pedersen_points_table::InteractionClaim,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.partial_ec_mul.mix_into(channel);
        self.pedersen_points_table.mix_into(channel);
    }

    pub fn sum(&self) -> QM31 {
        let mut sum = QM31::zero();
        sum += self.partial_ec_mul.claimed_sum;
        sum += self.pedersen_points_table.claimed_sum;
        sum
    }
}

pub struct PedersenContextComponents {
    pub components: Option<Components>,
}
impl PedersenContextComponents {
    pub fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PedersenContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &PedersenContextInteractionClaim,
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

impl std::fmt::Display for PedersenContextComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.components {
            Some(components) => write!(f, "{}", components),
            None => write!(f, "No Pedersen Context Components"),
        }
    }
}

pub struct Components {
    pub partial_ec_mul: partial_ec_mul::Component,
    pub pedersen_points_table: pedersen_points_table::Component,
}
impl Components {
    fn new(
        tree_span_provider: &mut TraceLocationAllocator,
        claim: &PedersenContextClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &InteractionClaim,
    ) -> Self {
        let partial_ec_mul_component = partial_ec_mul::Component::new(
            tree_span_provider,
            partial_ec_mul::Eval {
                claim: claim.claim.as_ref().unwrap().partial_ec_mul,
                partial_ec_mul_lookup_elements: interaction_elements.partial_ec_mul.clone(),
                pedersen_points_table_lookup_elements: interaction_elements
                    .pedersen_points_table
                    .clone(),
                range_check_19_lookup_elements: interaction_elements.range_checks.rc_19.clone(),
                range_check_9_9_lookup_elements: interaction_elements.range_checks.rc_9_9.clone(),
            },
            interaction_claim.partial_ec_mul.claimed_sum,
        );

        let pedersen_points_table_component = pedersen_points_table::Component::new(
            tree_span_provider,
            pedersen_points_table::Eval {
                claim: claim.claim.as_ref().unwrap().pedersen_points_table,
                pedersen_points_table_lookup_elements: interaction_elements
                    .pedersen_points_table
                    .clone(),
            },
            interaction_claim.pedersen_points_table.claimed_sum,
        );

        Self {
            partial_ec_mul: partial_ec_mul_component,
            pedersen_points_table: pedersen_points_table_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        vec![&self.partial_ec_mul, &self.pedersen_points_table]
    }
}

impl std::fmt::Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "PartialEcMul: {}",
            indented_component_display(&self.partial_ec_mul)
        )?;
        writeln!(
            f,
            "PedersenPointsTable: {}",
            indented_component_display(&self.pedersen_points_table)
        )?;

        Ok(())
    }
}

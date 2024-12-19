use itertools::{chain, Itertools};
use num_traits::Zero;
use prover_types::cpu::CasmState;
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::LogupSumsExt;
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{Relation, TraceLocationAllocator, PREPROCESSED_TRACE_IDX};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use stwo_prover::core::prover::StarkProof;
use stwo_prover::core::vcs::ops::MerkleHasher;
use tracing::{span, Level};

use super::debug_tools::indented_component_display;
use super::opcodes_air::{
    OpcodeClaim, OpcodeComponents, OpcodeInteractionClaim, OpcodesClaimGenerator,
    OpcodesInteractionClaimGenerator,
};
use super::IS_FIRST_LOG_SIZES;
use crate::components::memory::{memory_address_to_id, memory_id_to_big};
use crate::components::range_check_vector::{
    range_check_19, range_check_4_3, range_check_7_2_5, range_check_9_9,
};
use crate::components::verify_instruction;
use crate::felt::split_f252;
use crate::input::CairoInput;
use crate::relations;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

impl<H: MerkleHasher> CairoSerialize for CairoProof<H>
where
    H::Hash: CairoSerialize,
{
    fn serialize(&self, output: &mut Vec<starknet_ff::FieldElement>) {
        let Self {
            claim,
            interaction_claim,
            stark_proof,
        } = self;
        CairoSerialize::serialize(claim, output);
        CairoSerialize::serialize(interaction_claim, output);
        CairoSerialize::serialize(stark_proof, output);
    }
}

// (Address, Id, Value)
pub type PublicMemory = Vec<(u32, u32, [u32; 8])>;

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
    pub memory_address_to_id: memory_address_to_id::Claim,
    pub memory_id_to_value: memory_id_to_big::Claim,
    pub verify_instruction: verify_instruction::Claim,
    pub range_check_19: range_check_19::Claim,
    pub range_check9_9: range_check_9_9::Claim,
    pub range_check7_2_5: range_check_7_2_5::Claim,
    pub range_check4_3: range_check_4_3::Claim,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        // TODO(Ohad): add components.
        self.opcodes.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let mut log_sizes = TreeVec::concat_cols(
            [
                self.opcodes.log_sizes(),
                self.verify_instruction.log_sizes(),
                self.memory_address_to_id.log_sizes(),
                self.memory_id_to_value.log_sizes(),
                self.range_check_19.log_sizes(),
                self.range_check9_9.log_sizes(),
                self.range_check7_2_5.log_sizes(),
                self.range_check4_3.log_sizes(),
            ]
            .into_iter(),
        );
        // Overwrite the preprocessed trace log sizes.
        log_sizes[PREPROCESSED_TRACE_IDX] = IS_FIRST_LOG_SIZES.to_vec();
        log_sizes
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}
impl PublicData {
    pub fn logup_sum(&self, lookup_elements: &CairoInteractionElements) -> QM31 {
        // TODO(Ohad): Optimized inverse.
        let mut sum = QM31::zero();
        sum += self
            .public_memory
            .iter()
            .map(|(addr, id, val)| {
                let memory_address_to_id =
                    <relations::MemoryAddressToId as Relation<M31, QM31>>::combine(
                        &lookup_elements.memory_address_to_id,
                        &[M31::from_u32_unchecked(*addr), M31::from_u32_unchecked(*id)],
                    )
                    .inverse();
                let id_to_value = <relations::MemoryIdToBig as Relation<M31, QM31>>::combine(
                    &lookup_elements.memory_id_to_value,
                    &[
                        [M31::from_u32_unchecked(*id)].as_slice(),
                        split_f252(*val).as_slice(),
                    ]
                    .concat(),
                )
                .inverse();
                memory_address_to_id + id_to_value
            })
            .sum::<QM31>();

        // Yield initial state and use the final.
        sum += <relations::Opcodes as Relation<M31, QM31>>::combine(
            &lookup_elements.opcodes,
            &self.final_state.values(),
        )
        .inverse();
        sum -= <relations::Opcodes as Relation<M31, QM31>>::combine(
            &lookup_elements.opcodes,
            &self.initial_state.values(),
        )
        .inverse();
        sum
    }
}

/// Responsible for generating the CairoClaim and writing the trace.
/// NOTE: Order of writing the trace is important, and should be consistent with [`CairoClaim`],
/// [`CairoInteractionClaim`], [`CairoComponents`], [`CairoInteractionElements`].
pub struct CairoClaimGenerator {
    public_data: PublicData,

    opcodes: OpcodesClaimGenerator,

    // Internal components.
    verify_instruction_trace_generator: verify_instruction::ClaimGenerator,
    memory_address_to_id_trace_generator: memory_address_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: memory_id_to_big::ClaimGenerator,
    range_check_19_trace_generator: range_check_19::ClaimGenerator,
    range_check_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    range_check_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator,
    range_check_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    // ...
}
impl CairoClaimGenerator {
    pub fn new(input: CairoInput) -> Self {
        let initial_state = input.state_transitions.initial_state;
        let final_state = input.state_transitions.final_state;
        let opcodes = OpcodesClaimGenerator::new(input.state_transitions);
        let verify_instruction_trace_generator = verify_instruction::ClaimGenerator::default();
        let mut memory_address_to_id_trace_generator =
            memory_address_to_id::ClaimGenerator::new(&input.mem);
        let mut memory_id_to_value_trace_generator =
            memory_id_to_big::ClaimGenerator::new(&input.mem);
        let range_check_19_trace_generator = range_check_19::ClaimGenerator::new();
        let range_check_9_9_trace_generator = range_check_9_9::ClaimGenerator::new();
        let range_check_7_2_5_trace_generator = range_check_7_2_5::ClaimGenerator::new();
        let range_check_4_3_trace_generator = range_check_4_3::ClaimGenerator::new();

        // Yield public memory.
        for &addr in &input.public_mem_addresses {
            let id = memory_address_to_id_trace_generator.ids[addr as usize];
            memory_address_to_id_trace_generator.add_m31(M31::from_u32_unchecked(addr));
            memory_id_to_value_trace_generator.add_m31(M31::from_u32_unchecked(id));
        }

        // Public data.
        let public_memory = input
            .public_mem_addresses
            .iter()
            .copied()
            .map(|addr| {
                let id = input.mem.get_raw_id(addr);
                (addr, id, input.mem.get(addr).as_u256())
            })
            .collect_vec();
        let public_data = PublicData {
            public_memory,
            initial_state,
            final_state,
        };

        Self {
            public_data,
            opcodes,
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            range_check_19_trace_generator,
            range_check_9_9_trace_generator,
            range_check_7_2_5_trace_generator,
            range_check_4_3_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let span = span!(Level::INFO, "write opcode trace").entered();
        let (opcodes_claim, opcodes_interaction_gen) = self.opcodes.write_trace(
            tree_builder,
            &mut self.memory_address_to_id_trace_generator,
            &mut self.memory_id_to_value_trace_generator,
            &mut self.range_check_19_trace_generator,
            &mut self.range_check_9_9_trace_generator,
            &mut self.verify_instruction_trace_generator,
        );
        span.exit();
        let span = span!(Level::INFO, "internal component trace").entered();
        let (verify_instruction_claim, verify_instruction_interaction_gen) =
            self.verify_instruction_trace_generator.write_trace(
                tree_builder,
                &mut self.memory_address_to_id_trace_generator,
                &mut self.memory_id_to_value_trace_generator,
                &mut self.range_check_4_3_trace_generator,
                &mut self.range_check_7_2_5_trace_generator,
            );
        let (memory_address_to_id_claim, memory_address_to_id_interaction_gen) = self
            .memory_address_to_id_trace_generator
            .write_trace(tree_builder);
        let (memory_id_to_value_claim, memory_id_to_value_interaction_gen) = self
            .memory_id_to_value_trace_generator
            .write_trace(tree_builder, &mut self.range_check_9_9_trace_generator);
        let (range_check_19_claim, range_check_19_interaction_gen) = self
            .range_check_19_trace_generator
            .write_trace(tree_builder);
        let (range_check9_9_claim, range_check_9_9_interaction_gen) = self
            .range_check_9_9_trace_generator
            .write_trace(tree_builder);
        let (range_check_7_2_5_claim, range_check_7_2_5_interaction_gen) = self
            .range_check_7_2_5_trace_generator
            .write_trace(tree_builder);
        let (range_check_4_3_claim, range_check_4_3_interaction_gen) = self
            .range_check_4_3_trace_generator
            .write_trace(tree_builder);
        span.exit();
        (
            CairoClaim {
                public_data: self.public_data,
                opcodes: opcodes_claim,
                verify_instruction: verify_instruction_claim,
                memory_address_to_id: memory_address_to_id_claim,
                memory_id_to_value: memory_id_to_value_claim,
                range_check_19: range_check_19_claim,
                range_check9_9: range_check9_9_claim,
                range_check7_2_5: range_check_7_2_5_claim,
                range_check4_3: range_check_4_3_claim,
            },
            CairoInteractionClaimGenerator {
                opcodes_interaction_gen,
                verify_instruction_interaction_gen,
                memory_address_to_id_interaction_gen,
                memory_id_to_value_interaction_gen,
                range_check_19_interaction_gen,
                range_check_9_9_interaction_gen,
                range_check_7_2_5_interaction_gen,
                range_check_4_3_interaction_gen,
            },
        )
    }
}

pub struct CairoInteractionClaimGenerator {
    opcodes_interaction_gen: OpcodesInteractionClaimGenerator,
    verify_instruction_interaction_gen: verify_instruction::InteractionClaimGenerator,
    memory_address_to_id_interaction_gen: memory_address_to_id::InteractionClaimGenerator,
    memory_id_to_value_interaction_gen: memory_id_to_big::InteractionClaimGenerator,
    range_check_19_interaction_gen: range_check_19::InteractionClaimGenerator,
    range_check_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    range_check_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    range_check_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    // ...
}
impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        interaction_elements: &CairoInteractionElements,
    ) -> CairoInteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let opcodes_interaction_claims = self
            .opcodes_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let verify_instruction_interaction_claim = self
            .verify_instruction_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_check_4_3,
                &interaction_elements.range_check_7_2_5,
                &interaction_elements.verify_instruction,
            );
        let memory_address_to_id_interaction_claim = self
            .memory_address_to_id_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.memory_address_to_id);
        let memory_id_to_value_interaction_claim = self
            .memory_id_to_value_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_check_9_9,
            );
        let range_check_19_interaction_claim = self
            .range_check_19_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_19);
        let range_check9_9_interaction_claim = self
            .range_check_9_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_9_9);
        let range_check_7_2_5_interaction_claim = self
            .range_check_7_2_5_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_7_2_5);
        let range_check_4_3_interaction_claim = self
            .range_check_4_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_check_4_3);

        CairoInteractionClaim {
            opcodes: opcodes_interaction_claims,
            verify_instruction: verify_instruction_interaction_claim,
            memory_address_to_id: memory_address_to_id_interaction_claim,
            memory_id_to_value: memory_id_to_value_interaction_claim,
            range_check_19: range_check_19_interaction_claim,
            range_check_9_9: range_check9_9_interaction_claim,
            range_check_7_2_5: range_check_7_2_5_interaction_claim,
            range_check_4_3: range_check_4_3_interaction_claim,
        }
    }
}

pub struct CairoInteractionElements {
    pub opcodes: relations::Opcodes,
    pub verify_instruction: relations::VerifyInstruction,
    pub memory_address_to_id: relations::MemoryAddressToId,
    pub memory_id_to_value: relations::MemoryIdToBig,
    pub range_check_19: relations::RangeCheck_19,
    pub range_check_9_9: relations::RangeCheck_9_9,
    pub range_check_7_2_5: relations::RangeCheck_7_2_5,
    pub range_check_4_3: relations::RangeCheck_4_3,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: relations::Opcodes::draw(channel),
            verify_instruction: relations::VerifyInstruction::draw(channel),
            memory_address_to_id: relations::MemoryAddressToId::draw(channel),
            memory_id_to_value: relations::MemoryIdToBig::draw(channel),
            range_check_19: relations::RangeCheck_19::draw(channel),
            range_check_9_9: relations::RangeCheck_9_9::draw(channel),
            range_check_7_2_5: relations::RangeCheck_7_2_5::draw(channel),
            range_check_4_3: relations::RangeCheck_4_3::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: verify_instruction::InteractionClaim,
    pub memory_address_to_id: memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: memory_id_to_big::InteractionClaim,
    pub range_check_19: range_check_19::InteractionClaim,
    pub range_check_9_9: range_check_9_9::InteractionClaim,
    pub range_check_7_2_5: range_check_7_2_5::InteractionClaim,
    pub range_check_4_3: range_check_4_3::InteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.opcodes.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = QM31::zero();
    sum += claim.public_data.logup_sum(elements);

    // If the table is padded, take the sum of the non-padded values.
    // Otherwise, the claimed_sum is the total_sum.
    // TODO(Ohad): hide this logic behind `InteractionClaim`, and only sum here.
    sum += interaction_claim.opcodes.sum();
    sum += interaction_claim.verify_instruction.logup_sums.value();
    sum += interaction_claim.range_check_19.claimed_sum;
    sum += interaction_claim.range_check_9_9.claimed_sum;
    sum += interaction_claim.range_check_7_2_5.claimed_sum;
    sum += interaction_claim.range_check_4_3.claimed_sum;
    sum += interaction_claim.memory_address_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += interaction_claim.memory_id_to_value.small_claimed_sum;
    sum
}

pub struct CairoComponents {
    opcodes: OpcodeComponents,
    verify_instruction: verify_instruction::Component,
    memory_address_to_id: memory_address_to_id::Component,
    memory_id_to_value: (
        memory_id_to_big::BigComponent,
        memory_id_to_big::SmallComponent,
    ),
    range_check_19: range_check_19::Component,
    range_check9_9: range_check_9_9::Component,
    range_check7_2_5: range_check_7_2_5::Component,
    range_check4_3: range_check_4_3::Component,
    // ...
}
impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let tree_span_provider = &mut TraceLocationAllocator::new_with_preproccessed_columns(
            &IS_FIRST_LOG_SIZES
                .iter()
                .copied()
                .map(PreprocessedColumn::IsFirst)
                .collect_vec(),
        );

        let opcode_components = OpcodeComponents::new(
            tree_span_provider,
            &cairo_claim.opcodes,
            interaction_elements,
            &interaction_claim.opcodes,
        );
        let verify_instruction_component = verify_instruction::Component::new(
            tree_span_provider,
            verify_instruction::Eval {
                claim: cairo_claim.verify_instruction,
                memory_address_to_id_lookup_elements: interaction_elements
                    .memory_address_to_id
                    .clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_4_3_lookup_elements: interaction_elements.range_check_4_3.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements.range_check_7_2_5.clone(),
                verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
            },
            interaction_claim.verify_instruction.logup_sums,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.clone(),
                interaction_elements.memory_address_to_id.clone(),
            ),
            (
                interaction_claim.memory_address_to_id.clone().claimed_sum,
                None,
            ),
        );
        let memory_id_to_value_component = memory_id_to_big::BigComponent::new(
            tree_span_provider,
            memory_id_to_big::BigEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_check_9_9.clone(),
            ),
            (
                interaction_claim.memory_id_to_value.clone().big_claimed_sum,
                None,
            ),
        );
        let small_memory_id_to_value_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_check_9_9.clone(),
            ),
            (
                interaction_claim
                    .memory_id_to_value
                    .clone()
                    .small_claimed_sum,
                None,
            ),
        );
        let range_check_19_component = range_check_19::Component::new(
            tree_span_provider,
            range_check_19::Eval::new(interaction_elements.range_check_19.clone()),
            (interaction_claim.range_check_19.claimed_sum, None),
        );
        let range_check9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval::new(interaction_elements.range_check_9_9.clone()),
            (interaction_claim.range_check_9_9.claimed_sum, None),
        );
        let range_check_7_2_5_component = range_check_7_2_5::Component::new(
            tree_span_provider,
            range_check_7_2_5::Eval::new(interaction_elements.range_check_7_2_5.clone()),
            (interaction_claim.range_check_7_2_5.claimed_sum, None),
        );
        let range_check_4_3_component = range_check_4_3::Component::new(
            tree_span_provider,
            range_check_4_3::Eval::new(interaction_elements.range_check_4_3.clone()),
            (interaction_claim.range_check_4_3.claimed_sum, None),
        );
        Self {
            opcodes: opcode_components,
            verify_instruction: verify_instruction_component,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_component,
                small_memory_id_to_value_component,
            ),
            range_check_19: range_check_19_component,
            range_check9_9: range_check9_9_component,
            range_check7_2_5: range_check_7_2_5_component,
            range_check4_3: range_check_4_3_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        chain!(
            self.opcodes.provers(),
            [
                &self.verify_instruction as &dyn ComponentProver<SimdBackend>,
                &self.memory_address_to_id,
                &self.memory_id_to_value.0,
                &self.memory_id_to_value.1,
                &self.range_check_19,
                &self.range_check9_9,
                &self.range_check7_2_5,
                &self.range_check4_3,
            ],
        )
        .collect()
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.provers()
            .into_iter()
            .map(|component| component as &dyn Component)
            .collect()
    }
}

impl std::fmt::Display for CairoComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CairoComponents")?;
        writeln!(f, "Opcodes: {}", self.opcodes)?;
        writeln!(
            f,
            "VerifyInstruction: {}",
            indented_component_display(&self.verify_instruction)
        )?;
        writeln!(
            f,
            "MemoryAddressToId: {}",
            indented_component_display(&self.memory_address_to_id)
        )?;
        writeln!(
            f,
            "MemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_value.0)
        )?;
        writeln!(
            f,
            "SmallMemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_value.1)
        )?;
        writeln!(
            f,
            "RangeCheck19: {}",
            indented_component_display(&self.range_check_19)
        )?;
        writeln!(
            f,
            "RangeCheck9_9: {}",
            indented_component_display(&self.range_check9_9)
        )?;
        writeln!(
            f,
            "RangeCheck7_2_5: {}",
            indented_component_display(&self.range_check7_2_5)
        )?;
        writeln!(
            f,
            "RangeCheck4_3: {}",
            indented_component_display(&self.range_check4_3)
        )?;
        Ok(())
    }
}

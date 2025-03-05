use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_common::prover_types::cpu::CasmState;
use stwo_cairo_serialize::CairoSerialize;
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

use super::builtins_air::{
    BuiltinComponents, BuiltinsClaim, BuiltinsClaimGenerator, BuiltinsInteractionClaim,
    BuiltinsInteractionClaimGenerator,
};
use super::debug_tools::indented_component_display;
use super::opcodes_air::{
    OpcodeClaim, OpcodeComponents, OpcodeInteractionClaim, OpcodesClaimGenerator,
    OpcodesInteractionClaimGenerator,
};
use super::preprocessed::PreProcessedTrace;
use super::range_checks_air::{
    RangeChecksClaim, RangeChecksClaimGenerator, RangeChecksComponents,
    RangeChecksInteractionClaim, RangeChecksInteractionClaimGenerator,
    RangeChecksInteractionElements,
};
use crate::cairo_air::relations;
use crate::components::memory::{memory_address_to_id, memory_id_to_big};
use crate::components::{verify_bitwise_xor_9, verify_instruction};
use crate::felt::split_f252;

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
    pub verify_instruction: verify_instruction::Claim,
    pub builtins: BuiltinsClaim,
    pub memory_address_to_id: memory_address_to_id::Claim,
    pub memory_id_to_value: memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Claim,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.opcodes.mix_into(channel);
        self.verify_instruction.mix_into(channel);
        self.builtins.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
        self.range_checks.mix_into(channel);
        self.verify_bitwise_xor_9.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes_list = vec![
            self.opcodes.log_sizes(),
            self.verify_instruction.log_sizes(),
            self.builtins.log_sizes(),
            self.memory_address_to_id.log_sizes(),
            self.memory_id_to_value.log_sizes(),
            self.range_checks.log_sizes(),
            self.verify_bitwise_xor_9.log_sizes(),
        ];

        let mut log_sizes = TreeVec::concat_cols(log_sizes_list.into_iter());

        // Add preprocessed trace log sizes.
        log_sizes[PREPROCESSED_TRACE_IDX] = PreProcessedTrace::new().log_sizes();
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
    /// Sums the logup of the public data.
    pub fn logup_sum(&self, lookup_elements: &CairoInteractionElements) -> QM31 {
        let mut values_to_inverse = vec![];

        // Use public memory in the memory relations.
        self.public_memory.iter().for_each(|(addr, id, val)| {
            values_to_inverse.push(
                <relations::MemoryAddressToId as Relation<M31, QM31>>::combine(
                    &lookup_elements.memory_address_to_id,
                    &[M31::from_u32_unchecked(*addr), M31::from_u32_unchecked(*id)],
                ),
            );
            values_to_inverse.push(<relations::MemoryIdToBig as Relation<M31, QM31>>::combine(
                &lookup_elements.memory_id_to_value,
                &[
                    [M31::from_u32_unchecked(*id)].as_slice(),
                    split_f252(*val).as_slice(),
                ]
                .concat(),
            ));
        });

        // Yield initial state and use the final.
        values_to_inverse.push(<relations::Opcodes as Relation<M31, QM31>>::combine(
            &lookup_elements.opcodes,
            &self.final_state.values(),
        ));
        values_to_inverse.push(-<relations::Opcodes as Relation<M31, QM31>>::combine(
            &lookup_elements.opcodes,
            &self.initial_state.values(),
        ));

        let inverted_values = QM31::batch_inverse(&values_to_inverse);
        inverted_values.iter().sum::<QM31>()
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
    builtins: BuiltinsClaimGenerator,
    memory_address_to_id_trace_generator: memory_address_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: memory_id_to_big::ClaimGenerator,
    range_checks_trace_generator: RangeChecksClaimGenerator,
    verify_bitwise_xor_9_trace_generator: verify_bitwise_xor_9::ClaimGenerator,
    // ...
}
impl CairoClaimGenerator {
    pub fn new(input: ProverInput) -> Self {
        let initial_state = input.state_transitions.initial_state;
        let final_state = input.state_transitions.final_state;
        let opcodes = OpcodesClaimGenerator::new(input.state_transitions);
        let verify_instruction_trace_generator =
            verify_instruction::ClaimGenerator::new(input.instruction_by_pc);
        let builtins = BuiltinsClaimGenerator::new(input.builtins_segments);
        let memory_address_to_id_trace_generator =
            memory_address_to_id::ClaimGenerator::new(&input.memory);
        let memory_id_to_value_trace_generator =
            memory_id_to_big::ClaimGenerator::new(&input.memory);
        let range_checks_trace_generator = RangeChecksClaimGenerator::new();
        let verify_bitwise_xor_9_trace_generator = verify_bitwise_xor_9::ClaimGenerator::new();

        // Yield public memory.
        for addr in input
            .public_memory_addresses
            .iter()
            .copied()
            .map(M31::from_u32_unchecked)
        {
            let id = memory_address_to_id_trace_generator.get_id(addr);
            memory_address_to_id_trace_generator.add_input(&addr);
            memory_id_to_value_trace_generator.add_input(&id);
        }

        // Public data.
        let public_memory = input
            .public_memory_addresses
            .iter()
            .copied()
            .map(|addr| {
                let id = input.memory.get_raw_id(addr);
                (addr, id, input.memory.get(addr).as_u256())
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
            verify_instruction_trace_generator,
            builtins,
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            range_checks_trace_generator,
            verify_bitwise_xor_9_trace_generator,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let span = span!(Level::INFO, "write opcode trace").entered();
        let (opcodes_claim, opcodes_interaction_gen) = self.opcodes.write_trace(
            tree_builder,
            &self.memory_address_to_id_trace_generator,
            &self.memory_id_to_value_trace_generator,
            &self.range_checks_trace_generator.rc_11_trace_generator,
            &self.range_checks_trace_generator.rc_19_trace_generator,
            &self.range_checks_trace_generator.rc_9_9_trace_generator,
            &self.verify_instruction_trace_generator,
        );
        span.exit();
        let span = span!(Level::INFO, "internal component trace").entered();
        let (verify_instruction_claim, verify_instruction_interaction_gen) =
            self.verify_instruction_trace_generator.write_trace(
                tree_builder,
                &self.memory_address_to_id_trace_generator,
                &self.memory_id_to_value_trace_generator,
                &self.range_checks_trace_generator.rc_4_3_trace_generator,
                &self.range_checks_trace_generator.rc_7_2_5_trace_generator,
            );
        let (builtins_claim, builtins_interaction_gen) = self.builtins.write_trace(
            tree_builder,
            &self.memory_address_to_id_trace_generator,
            &self.memory_id_to_value_trace_generator,
            &self.range_checks_trace_generator.rc_6_trace_generator,
            &self.range_checks_trace_generator.rc_18_trace_generator,
            &self.range_checks_trace_generator.rc_19_trace_generator,
            &self.range_checks_trace_generator.rc_4_4_trace_generator,
            &self.range_checks_trace_generator.rc_9_9_trace_generator,
            &self.range_checks_trace_generator.rc_4_4_4_4_trace_generator,
            &self
                .range_checks_trace_generator
                .rc_3_3_3_3_3_trace_generator,
            &self.verify_bitwise_xor_9_trace_generator,
        );
        let (memory_address_to_id_claim, memory_address_to_id_interaction_gen) = self
            .memory_address_to_id_trace_generator
            .write_trace(tree_builder);
        let (memory_id_to_value_claim, memory_id_to_value_interaction_gen) =
            self.memory_id_to_value_trace_generator.write_trace(
                tree_builder,
                &self.range_checks_trace_generator.rc_9_9_trace_generator,
            );
        let (range_checks_claim, range_checks_interaction_gen) =
            self.range_checks_trace_generator.write_trace(tree_builder);
        let (verify_bitwise_xor_9_claim, verify_bitwise_xor_9_interaction_gen) = self
            .verify_bitwise_xor_9_trace_generator
            .write_trace(tree_builder);
        span.exit();
        (
            CairoClaim {
                public_data: self.public_data,
                opcodes: opcodes_claim,
                verify_instruction: verify_instruction_claim,
                builtins: builtins_claim,
                memory_address_to_id: memory_address_to_id_claim,
                memory_id_to_value: memory_id_to_value_claim,
                range_checks: range_checks_claim,
                verify_bitwise_xor_9: verify_bitwise_xor_9_claim,
            },
            CairoInteractionClaimGenerator {
                opcodes_interaction_gen,
                verify_instruction_interaction_gen,
                builtins_interaction_gen,
                memory_address_to_id_interaction_gen,
                memory_id_to_value_interaction_gen,
                range_checks_interaction_gen,
                verify_bitwise_xor_9_interaction_gen,
            },
        )
    }
}

pub struct CairoInteractionClaimGenerator {
    opcodes_interaction_gen: OpcodesInteractionClaimGenerator,
    verify_instruction_interaction_gen: verify_instruction::InteractionClaimGenerator,
    builtins_interaction_gen: BuiltinsInteractionClaimGenerator,
    memory_address_to_id_interaction_gen: memory_address_to_id::InteractionClaimGenerator,
    memory_id_to_value_interaction_gen: memory_id_to_big::InteractionClaimGenerator,
    range_checks_interaction_gen: RangeChecksInteractionClaimGenerator,
    verify_bitwise_xor_9_interaction_gen: verify_bitwise_xor_9::InteractionClaimGenerator,
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
                &interaction_elements.range_checks.rc_4_3,
                &interaction_elements.range_checks.rc_7_2_5,
                &interaction_elements.verify_instruction,
            );
        let builtins_interaction_claims = self
            .builtins_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let memory_address_to_id_interaction_claim = self
            .memory_address_to_id_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.memory_address_to_id);
        let memory_id_to_value_interaction_claim = self
            .memory_id_to_value_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_9_9,
            );

        let range_checks_interaction_claim = self
            .range_checks_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_checks);
        let verify_bitwise_xor_9_interaction_claim = self
            .verify_bitwise_xor_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_9);

        CairoInteractionClaim {
            opcodes: opcodes_interaction_claims,
            verify_instruction: verify_instruction_interaction_claim,
            builtins: builtins_interaction_claims,
            memory_address_to_id: memory_address_to_id_interaction_claim,
            memory_id_to_value: memory_id_to_value_interaction_claim,
            range_checks: range_checks_interaction_claim,
            verify_bitwise_xor_9: verify_bitwise_xor_9_interaction_claim,
        }
    }
}

pub struct CairoInteractionElements {
    pub opcodes: relations::Opcodes,
    pub verify_instruction: relations::VerifyInstruction,
    pub poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain,
    pub poseidon_full_round_chain: relations::PoseidonFullRoundChain,
    pub cube_252: relations::Cube252,
    pub verify_mul_252: relations::VerifyMul252,
    pub poseidon_round_keys: relations::PoseidonRoundKeys,
    pub range_check_felt_252_width_27: relations::RangeCheckFelt252Width27,
    pub memory_address_to_id: relations::MemoryAddressToId,
    pub memory_id_to_value: relations::MemoryIdToBig,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_9: relations::VerifyBitwiseXor_9,
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: relations::Opcodes::draw(channel),
            verify_instruction: relations::VerifyInstruction::draw(channel),
            poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain::draw(channel),
            poseidon_full_round_chain: relations::PoseidonFullRoundChain::draw(channel),
            cube_252: relations::Cube252::draw(channel),
            verify_mul_252: relations::VerifyMul252::draw(channel),
            poseidon_round_keys: relations::PoseidonRoundKeys::draw(channel),
            range_check_felt_252_width_27: relations::RangeCheckFelt252Width27::draw(channel),
            memory_address_to_id: relations::MemoryAddressToId::draw(channel),
            memory_id_to_value: relations::MemoryIdToBig::draw(channel),
            range_checks: RangeChecksInteractionElements::draw(channel),
            verify_bitwise_xor_9: relations::VerifyBitwiseXor_9::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: verify_instruction::InteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub memory_address_to_id: memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::InteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.opcodes.mix_into(channel);
        self.verify_instruction.mix_into(channel);
        self.builtins.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
        self.range_checks.mix_into(channel);
        self.verify_bitwise_xor_9.mix_into(channel);
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
    sum += interaction_claim.opcodes.sum();
    sum += interaction_claim.verify_instruction.claimed_sum;
    sum += interaction_claim.builtins.sum();
    sum += interaction_claim.memory_address_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += interaction_claim.memory_id_to_value.small_claimed_sum;
    sum += interaction_claim.range_checks.sum();
    sum += interaction_claim.verify_bitwise_xor_9.claimed_sum;
    sum
}

pub struct CairoComponents {
    opcodes: OpcodeComponents,
    verify_instruction: verify_instruction::Component,
    builtins: BuiltinComponents,
    memory_address_to_id: memory_address_to_id::Component,
    memory_id_to_value: (
        memory_id_to_big::BigComponent,
        memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_9: verify_bitwise_xor_9::Component,
    // ...
}
impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
    ) -> Self {
        let tree_span_provider = &mut TraceLocationAllocator::new_with_preproccessed_columns(
            &PreProcessedTrace::new().ids(),
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
                verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            interaction_claim.verify_instruction.claimed_sum,
        );
        let builtin_components = BuiltinComponents::new(
            tree_span_provider,
            &cairo_claim.builtins,
            interaction_elements,
            &interaction_claim.builtins,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.clone(),
                interaction_elements.memory_address_to_id.clone(),
            ),
            interaction_claim.memory_address_to_id.clone().claimed_sum,
        );
        let memory_id_to_value_component = memory_id_to_big::BigComponent::new(
            tree_span_provider,
            memory_id_to_big::BigEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_checks.rc_9_9.clone(),
            ),
            interaction_claim.memory_id_to_value.clone().big_claimed_sum,
        );
        let small_memory_id_to_value_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_checks.rc_9_9.clone(),
            ),
            interaction_claim
                .memory_id_to_value
                .clone()
                .small_claimed_sum,
        );
        let range_checks_component = RangeChecksComponents::new(
            tree_span_provider,
            &interaction_elements.range_checks,
            &interaction_claim.range_checks,
        );
        let verify_bitwise_xor_9_component = verify_bitwise_xor_9::Component::new(
            tree_span_provider,
            verify_bitwise_xor_9::Eval {
                verify_bitwise_xor_9_lookup_elements: interaction_elements
                    .verify_bitwise_xor_9
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_9.claimed_sum,
        );
        Self {
            opcodes: opcode_components,
            verify_instruction: verify_instruction_component,
            builtins: builtin_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_component,
                small_memory_id_to_value_component,
            ),
            range_checks: range_checks_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        chain!(
            self.opcodes.provers(),
            [&self.verify_instruction as &dyn ComponentProver<SimdBackend>,],
            self.builtins.provers(),
            [
                &self.memory_address_to_id as &dyn ComponentProver<SimdBackend>,
                &self.memory_id_to_value.0 as &dyn ComponentProver<SimdBackend>,
                &self.memory_id_to_value.1 as &dyn ComponentProver<SimdBackend>,
            ],
            self.range_checks.provers(),
            [&self.verify_bitwise_xor_9 as &dyn ComponentProver<SimdBackend>,],
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
        writeln!(f, "Builtins: {}", self.builtins)?;
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
        writeln!(f, "RangeChecks: {}", self.range_checks)?;
        writeln!(
            f,
            "VerifyBitwiseXor9: {}",
            indented_component_display(&self.verify_bitwise_xor_9)
        )?;
        Ok(())
    }
}

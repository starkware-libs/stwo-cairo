use itertools::{chain, Itertools};
use num_traits::Zero;
use prover_types::cpu::CasmState;
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{TraceLocationAllocator, PREPROCESSED_TRACE_IDX};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::{TreeBuilder, TreeVec};
use stwo_prover::core::prover::StarkProof;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo_prover::core::vcs::ops::MerkleHasher;

use super::IS_FIRST_LOG_SIZES;
use crate::components::memory::{addr_to_id, id_to_f252};
use crate::components::range_check_vector::{range_check_4_3, range_check_7_2_5, range_check_9_9};
use crate::components::{opcodes, ret_opcode, verifyinstruction};
use crate::felt::split_f252;
use crate::input::CairoInput;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

// (Address, Id, Value)
pub type PublicMemory = Vec<(u32, u32, [u32; 8])>;

#[derive(Serialize, Deserialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub ret: Vec<ret_opcode::Claim>,
    pub memory_addr_to_id: addr_to_id::Claim,
    pub memory_id_to_value: id_to_f252::Claim,
    pub verify_instruction: verifyinstruction::Claim,
    pub range_check9_9: range_check_9_9::Claim,
    pub range_check7_2_5: range_check_7_2_5::Claim,
    pub range_check4_3: range_check_4_3::Claim,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        // TODO(Ohad): add components.
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.memory_addr_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let mut log_sizes = TreeVec::concat_cols(chain!(
            self.ret.iter().map(|c| c.log_sizes()),
            [self.verify_instruction.log_sizes()],
            [self.memory_addr_to_id.log_sizes()],
            [self.memory_id_to_value.log_sizes()],
            [self.range_check9_9.log_sizes()],
            [self.range_check7_2_5.log_sizes()],
            [self.range_check4_3.log_sizes()],
        ));
        // Overwrite the preprocessed trace log sizes.
        log_sizes[PREPROCESSED_TRACE_IDX] = IS_FIRST_LOG_SIZES.to_vec();
        log_sizes
    }
}

#[derive(Serialize, Deserialize)]
pub struct PublicData {
    pub public_memory: PublicMemory,
    pub initial_state: CasmState,
    pub final_state: CasmState,
}

/// Responsible for generating the CairoClaim and writing the trace.
/// NOTE: Order of writing the trace is important, and should be consistent with [`CairoClaim`],
/// [`CairoInteractionClaim`], [`CairoComponents`], [`CairoInteractionElements`].
pub struct CairoClaimGenerator {
    public_data: PublicData,

    // Opcodes
    ret_trace_generator: ret_opcode::ClaimGenerator,

    // Internal components.
    verify_instruction_trace_generator: verifyinstruction::ClaimGenerator,
    memory_addr_to_id_trace_generator: addr_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: id_to_f252::ClaimGenerator,
    range_check_9_9_trace_generator: range_check_9_9::ClaimGenerator,
    range_check_7_2_5_trace_generator: range_check_7_2_5::ClaimGenerator,
    range_check_4_3_trace_generator: range_check_4_3::ClaimGenerator,
    // ...
}
impl CairoClaimGenerator {
    pub fn new(input: CairoInput) -> Self {
        let ret_trace_generator = ret_opcode::ClaimGenerator::new(input.instructions.ret);
        let verify_instruction_trace_generator = verifyinstruction::ClaimGenerator::default();
        let mut memory_addr_to_id_trace_generator = addr_to_id::ClaimGenerator::new(&input.mem);
        let mut memory_id_to_value_trace_generator = id_to_f252::ClaimGenerator::new(&input.mem);
        let range_check_9_9_trace_generator = range_check_9_9::ClaimGenerator::new();
        let range_check_7_2_5_trace_generator = range_check_7_2_5::ClaimGenerator::new();
        let range_check_4_3_trace_generator = range_check_4_3::ClaimGenerator::new();

        // Yield public memory.
        for &addr in &input.public_mem_addresses {
            let id = memory_addr_to_id_trace_generator.ids[addr as usize];
            memory_addr_to_id_trace_generator.add_m31(M31::from_u32_unchecked(addr));
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
            initial_state: input.instructions.initial_state,
            final_state: input.instructions.final_state,
        };

        Self {
            public_data,
            memory_addr_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            verify_instruction_trace_generator,
            range_check_9_9_trace_generator,
            range_check_7_2_5_trace_generator,
            range_check_4_3_trace_generator,
            ret_trace_generator,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator) {
        let (ret_claim, ret_interaction_gen) = self.ret_trace_generator.write_trace(
            tree_builder,
            &mut self.memory_addr_to_id_trace_generator,
            &mut self.memory_id_to_value_trace_generator,
            &mut self.verify_instruction_trace_generator,
        );
        let (verify_instruction_claim, verify_instruction_interaction_gen) =
            self.verify_instruction_trace_generator.write_trace(
                tree_builder,
                &mut self.memory_addr_to_id_trace_generator,
                &mut self.range_check_4_3_trace_generator,
                &mut self.range_check_7_2_5_trace_generator,
            );
        let (memory_addr_to_id_claim, memory_addr_to_id_interaction_gen) = self
            .memory_addr_to_id_trace_generator
            .write_trace(tree_builder);
        let (memory_id_to_value_claim, memory_id_to_value_interaction_gen) = self
            .memory_id_to_value_trace_generator
            .write_trace(tree_builder, &mut self.range_check_9_9_trace_generator);
        let (range_check9_9_claim, range_check_9_9_interaction_gen) = self
            .range_check_9_9_trace_generator
            .write_trace(tree_builder);
        let (range_check_7_2_5_claim, range_check_7_2_5_interaction_gen) = self
            .range_check_7_2_5_trace_generator
            .write_trace(tree_builder);
        let (range_check_4_3_claim, range_check_4_3_interaction_gen) = self
            .range_check_4_3_trace_generator
            .write_trace(tree_builder);
        (
            CairoClaim {
                public_data: self.public_data,
                ret: vec![ret_claim],
                verify_instruction: verify_instruction_claim,
                memory_addr_to_id: memory_addr_to_id_claim.clone(),
                memory_id_to_value: memory_id_to_value_claim.clone(),
                range_check9_9: range_check9_9_claim.clone(),
                range_check7_2_5: range_check_7_2_5_claim.clone(),
                range_check4_3: range_check_4_3_claim.clone(),
            },
            CairoInteractionClaimGenerator {
                ret_interaction_gen,
                verify_instruction_interaction_gen,
                memory_addr_to_id_interaction_gen,
                memory_id_to_value_interaction_gen,
                range_check_9_9_interaction_gen,
                range_check_7_2_5_interaction_gen,
                range_check_4_3_interaction_gen,
            },
        )
    }
}

pub struct CairoInteractionClaimGenerator {
    ret_interaction_gen: ret_opcode::InteractionClaimGenerator,
    verify_instruction_interaction_gen: verifyinstruction::InteractionClaimGenerator,
    memory_addr_to_id_interaction_gen: addr_to_id::InteractionClaimGenerator,
    memory_id_to_value_interaction_gen: id_to_f252::InteractionClaimGenerator,
    range_check_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    range_check_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    range_check_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    // ...
}
impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        interaction_elements: &CairoInteractionElements,
    ) -> CairoInteractionClaim {
        let ret_interaction_claim = self.ret_interaction_gen.write_interaction_trace(
            tree_builder,
            &interaction_elements.memory_addr_to_id_lookup,
            &interaction_elements.memory_id_to_value_lookup,
            &interaction_elements.verify_instruction_lookup,
            &interaction_elements.opcodes_lookup_elements,
        );
        let verifyinstruction_interaction_claim = self
            .verify_instruction_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_addr_to_id_lookup,
                &interaction_elements.memory_id_to_value_lookup,
                &interaction_elements.range4_3_lookup,
                &interaction_elements.range7_2_5_lookup,
                &interaction_elements.verify_instruction_lookup,
            );
        let memory_addr_to_id_interaction_claim = self
            .memory_addr_to_id_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.memory_addr_to_id_lookup);
        let memory_id_to_value_interaction_claim = self
            .memory_id_to_value_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_id_to_value_lookup,
                &interaction_elements.range9_9_lookup,
            );
        let range_check9_9_interaction_claim = self
            .range_check_9_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range9_9_lookup);
        let range_check_7_2_5_interaction_claim = self
            .range_check_7_2_5_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range7_2_5_lookup);
        let range_check_4_3_interaction_claim = self
            .range_check_4_3_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range4_3_lookup);

        CairoInteractionClaim {
            ret: vec![ret_interaction_claim],
            verify_instruction: verifyinstruction_interaction_claim,
            memory_addr_to_id: memory_addr_to_id_interaction_claim,
            memory_id_to_value: memory_id_to_value_interaction_claim,
            range_check9_9: range_check9_9_interaction_claim,
            range_check7_2_5: range_check_7_2_5_interaction_claim,
            range_check4_3: range_check_4_3_interaction_claim,
        }
    }
}

pub struct CairoInteractionElements {
    pub opcodes_lookup_elements: opcodes::VmRelationElements,
    pub verify_instruction_lookup: verifyinstruction::RelationElements,
    pub memory_addr_to_id_lookup: addr_to_id::RelationElements,
    pub memory_id_to_value_lookup: id_to_f252::RelationElements,
    pub range9_9_lookup: range_check_9_9::RelationElements,
    pub range7_2_5_lookup: range_check_7_2_5::RelationElements,
    pub range4_3_lookup: range_check_4_3::RelationElements,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes_lookup_elements: opcodes::VmRelationElements::draw(channel),
            verify_instruction_lookup: verifyinstruction::RelationElements::draw(channel),
            memory_addr_to_id_lookup: addr_to_id::RelationElements::draw(channel),
            memory_id_to_value_lookup: id_to_f252::RelationElements::draw(channel),
            range9_9_lookup: range_check_9_9::RelationElements::draw(channel),
            range7_2_5_lookup: range_check_7_2_5::RelationElements::draw(channel),
            range4_3_lookup: range_check_4_3::RelationElements::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CairoInteractionClaim {
    pub ret: Vec<ret_opcode::InteractionClaim>,
    pub verify_instruction: verifyinstruction::InteractionClaim,
    pub memory_addr_to_id: addr_to_id::InteractionClaim,
    pub memory_id_to_value: id_to_f252::InteractionClaim,
    pub range_check9_9: range_check_9_9::InteractionClaim,
    pub range_check7_2_5: range_check_7_2_5::InteractionClaim,
    pub range_check4_3: range_check_4_3::InteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.memory_addr_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = QM31::zero();
    // TODO(Ohad): Optimized inverse.
    // TODO(Ohad): move public data to a function.
    sum += claim
        .public_data
        .public_memory
        .iter()
        .map(|(addr, id, val)| {
            let addr_to_id = elements
                .memory_addr_to_id_lookup
                .combine::<M31, QM31>(&[
                    M31::from_u32_unchecked(*addr),
                    M31::from_u32_unchecked(*id),
                ])
                .inverse();
            let id_to_value = elements
                .memory_id_to_value_lookup
                .combine::<M31, QM31>(
                    &[
                        [M31::from_u32_unchecked(*id)].as_slice(),
                        split_f252(*val).as_slice(),
                    ]
                    .concat(),
                )
                .inverse();
            addr_to_id + id_to_value
        })
        .sum::<SecureField>();
    sum += interaction_claim.range_check9_9.claimed_sum;
    sum += interaction_claim.memory_addr_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += interaction_claim.memory_id_to_value.small_claimed_sum;
    sum += interaction_claim.range_check7_2_5.claimed_sum;
    sum += interaction_claim.range_check4_3.claimed_sum;
    sum += interaction_claim.ret[0].claimed_sum.unwrap().0;
    sum += interaction_claim.verify_instruction.claimed_sum.unwrap().0;
    sum
}

pub struct CairoComponents {
    ret: Vec<ret_opcode::Component>,
    verify_instruction: verifyinstruction::Component,
    memory_addr_to_id: addr_to_id::Component,
    memory_id_to_value: (id_to_f252::BigComponent, id_to_f252::SmallComponent),
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
        let tree_span_provider = &mut TraceLocationAllocator::new_with_preproccessed_columnds(
            &IS_FIRST_LOG_SIZES
                .iter()
                .copied()
                .map(PreprocessedColumn::IsFirst)
                .collect_vec(),
        );

        let ret_components = cairo_claim
            .ret
            .iter()
            .zip(interaction_claim.ret.iter())
            .map(|(&claim, &interaction_claim)| {
                ret_opcode::Component::new(
                    tree_span_provider,
                    ret_opcode::Eval {
                        claim,
                        interaction_claim,
                        memoryaddresstoid_lookup_elements: interaction_elements
                            .memory_addr_to_id_lookup
                            .clone(),
                        memoryidtobig_lookup_elements: interaction_elements
                            .memory_id_to_value_lookup
                            .clone(),
                        verifyinstruction_lookup_elements: interaction_elements
                            .verify_instruction_lookup
                            .clone(),
                        opcodes_lookup_elements: interaction_elements
                            .opcodes_lookup_elements
                            .clone(),
                    },
                )
            })
            .collect_vec();
        let verifyinstruction_component = verifyinstruction::Component::new(
            tree_span_provider,
            verifyinstruction::Eval::new(
                cairo_claim.verify_instruction,
                interaction_elements.memory_addr_to_id_lookup.clone(),
                interaction_elements.memory_id_to_value_lookup.clone(),
                interaction_elements.range4_3_lookup.clone(),
                interaction_elements.range7_2_5_lookup.clone(),
                interaction_elements.verify_instruction_lookup.clone(),
                interaction_claim.verify_instruction,
            ),
        );
        let memory_addr_to_id_component = addr_to_id::Component::new(
            tree_span_provider,
            addr_to_id::Eval::new(
                cairo_claim.memory_addr_to_id.clone(),
                interaction_elements.memory_addr_to_id_lookup.clone(),
                interaction_claim.memory_addr_to_id.clone(),
            ),
        );
        let memory_id_to_value_component = id_to_f252::BigComponent::new(
            tree_span_provider,
            id_to_f252::BigEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value_lookup.clone(),
                interaction_elements.range9_9_lookup.clone(),
                interaction_claim.memory_id_to_value.clone(),
            ),
        );
        let small_memory_id_to_value_component = id_to_f252::SmallComponent::new(
            tree_span_provider,
            id_to_f252::SmallEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value_lookup.clone(),
                interaction_elements.range9_9_lookup.clone(),
                interaction_claim.memory_id_to_value.clone(),
            ),
        );
        let range_check9_9_component = range_check_9_9::Component::new(
            tree_span_provider,
            range_check_9_9::Eval::new(
                interaction_elements.range9_9_lookup.clone(),
                interaction_claim.range_check9_9.claimed_sum,
            ),
        );
        let range_check_7_2_5_component = range_check_7_2_5::Component::new(
            tree_span_provider,
            range_check_7_2_5::Eval::new(
                interaction_elements.range7_2_5_lookup.clone(),
                interaction_claim.range_check7_2_5.claimed_sum,
            ),
        );
        let range_check_4_3_component = range_check_4_3::Component::new(
            tree_span_provider,
            range_check_4_3::Eval::new(
                interaction_elements.range4_3_lookup.clone(),
                interaction_claim.range_check4_3.claimed_sum,
            ),
        );
        Self {
            ret: ret_components,
            verify_instruction: verifyinstruction_component,
            memory_addr_to_id: memory_addr_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_component,
                small_memory_id_to_value_component,
            ),
            range_check9_9: range_check9_9_component,
            range_check7_2_5: range_check_7_2_5_component,
            range_check4_3: range_check_4_3_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        let mut vec: Vec<&dyn ComponentProver<SimdBackend>> = vec![];
        for ret in self.ret.iter() {
            vec.push(ret);
        }
        vec.push(&self.verify_instruction);
        vec.push(&self.memory_addr_to_id);
        vec.push(&self.memory_id_to_value.0);
        vec.push(&self.memory_id_to_value.1);
        vec.push(&self.range_check9_9);
        vec.push(&self.range_check7_2_5);
        vec.push(&self.range_check4_3);

        vec
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.provers()
            .into_iter()
            .map(|component| component as &dyn Component)
            .collect()
    }
}

use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::preprocessed_columns::{gen_is_first, PreprocessedColumn};
use stwo_prover::constraint_framework::{TraceLocationAllocator, PREPROCESSED_TRACE_IDX};
use stwo_prover::core::air::{Component, ComponentProver};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::{
    CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig, TreeVec,
};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, ProvingError, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::core::vcs::ops::MerkleHasher;
use thiserror::Error;
use tracing::{span, Level};

use crate::components::memory::{addr_to_id, id_to_f252};
use crate::components::opcodes::{
    addapopcode_is_imm_t_op1_base_fp_f, callopcode_is_rel_t_op1_base_fp_f, genericopcode,
    jumpopcode_is_rel_t_is_imm_t_is_double_deref_f, ret_opcode,
};
use crate::components::range_check_vector::{
    range_check_19, range_check_4_3, range_check_7_2_5, range_check_9_9,
};
use crate::components::{opcodes, verifyinstruction, GLOBAL_TRACKER};
use crate::felt::split_f252;
use crate::input::instructions::VmState;
use crate::input::CairoInput;

#[derive(Serialize, Deserialize)]
pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

#[derive(Serialize, Deserialize)]
pub struct CairoClaim {
    // Common claim values.
    pub public_memory: Vec<(u32, u32, [u32; 8])>,
    pub initial_state: VmState,
    pub final_state: VmState,

    pub ret: Vec<ret_opcode::Claim>,
    pub add_ap_imm: addapopcode_is_imm_t_op1_base_fp_f::Claim,
    pub jmp_rel_imm: jumpopcode_is_rel_t_is_imm_t_is_double_deref_f::Claim,
    pub call_opcode: callopcode_is_rel_t_op1_base_fp_f::Claim,
    pub generic_opcode: genericopcode::Claim,
    pub memory_addr_to_id: addr_to_id::Claim,
    pub memory_id_to_value: id_to_f252::Claim,
    pub verify_instruction: verifyinstruction::Claim,
    pub range_check_19: range_check_19::Claim,
    pub range_check9_9: range_check_9_9::Claim,
    pub range_check7_2_5: range_check_7_2_5::Claim,
    pub range_check4_3: range_check_4_3::Claim,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        // TODO(spapini): Add common values.
        self.ret.iter().for_each(|c| c.mix_into(channel));
        self.memory_addr_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let mut log_sizes = TreeVec::concat_cols(chain!(
            self.ret.iter().map(|c| c.log_sizes()),
            [self.add_ap_imm.log_sizes()],
            [self.jmp_rel_imm.log_sizes()],
            [self.call_opcode.log_sizes()],
            [self.generic_opcode.log_sizes()],
            [self.memory_addr_to_id.log_sizes()],
            [self.memory_id_to_value.log_sizes()],
            [self.verify_instruction.log_sizes()],
            [self.range_check9_9.log_sizes()],
            [self.range_check7_2_5.log_sizes()],
            [self.range_check4_3.log_sizes()],
        ));
        // Overwrite the preprocessed trace log sizes.
        log_sizes[PREPROCESSED_TRACE_IDX] = IS_FIRST_LOG_SIZES.to_vec();
        log_sizes
    }
}

pub struct CairoInteractionElements {
    memory_addr_to_id_lookup: addr_to_id::RelationElements,
    memory_id_to_value_lookup: id_to_f252::RelationElements,
    opcodes_lookup_elements: opcodes::RelationElements,
    verify_instruction_lookup: verifyinstruction::RelationElements,
    range_check_19_lookup: range_check_19::RelationElements,
    range9_9_lookup: range_check_9_9::RelationElements,
    range7_2_5_lookup: range_check_7_2_5::RelationElements,
    range4_3_lookup: range_check_4_3::RelationElements,
    // ...
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            memory_addr_to_id_lookup: addr_to_id::RelationElements::draw(channel),
            memory_id_to_value_lookup: id_to_f252::RelationElements::draw(channel),
            opcodes_lookup_elements: opcodes::RelationElements::draw(channel),
            range_check_19_lookup: range_check_19::RelationElements::draw(channel),
            range9_9_lookup: range_check_9_9::RelationElements::draw(channel),
            verify_instruction_lookup: verifyinstruction::RelationElements::draw(channel),
            range7_2_5_lookup: range_check_7_2_5::RelationElements::draw(channel),
            range4_3_lookup: range_check_4_3::RelationElements::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CairoInteractionClaim {
    pub ret: Vec<ret_opcode::InteractionClaim>,
    pub add_ap_im: addapopcode_is_imm_t_op1_base_fp_f::InteractionClaim,
    pub jmp_rel_imm: jumpopcode_is_rel_t_is_imm_t_is_double_deref_f::InteractionClaim,
    pub call_rel: callopcode_is_rel_t_op1_base_fp_f::InteractionClaim,
    pub generic_opcode: genericopcode::InteractionClaim,
    pub memory_addr_to_id: addr_to_id::InteractionClaim,
    pub memory_id_to_value: id_to_f252::InteractionClaim,
    pub range_check_19: range_check_19::InteractionClaim,
    pub range_check9_9: range_check_9_9::InteractionClaim,
    pub range_check7_2_5: range_check_7_2_5::InteractionClaim,
    pub range_check4_3: range_check_4_3::InteractionClaim,
    pub verify_instruction: verifyinstruction::InteractionClaim,
    // ...
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
    sum += claim
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
            GLOBAL_TRACKER.lock().unwrap().add_to_relation(
                "addr_to_id",
                &[M31::from_u32_unchecked(*addr), M31::from_u32_unchecked(*id)],
                "pub",
            );
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
            GLOBAL_TRACKER.lock().unwrap().add_to_relation(
                "id_to_big",
                &[
                    [M31::from_u32_unchecked(*id)].as_slice(),
                    split_f252(*val).as_slice(),
                ]
                .concat(),
                "pub",
            );
            addr_to_id + id_to_value
        })
        .sum::<SecureField>();
    sum += interaction_claim.range_check9_9.claimed_sum;
    sum += interaction_claim.memory_addr_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.big_claimed_sum;
    sum += interaction_claim.memory_id_to_value.small_claimed_sum;
    sum += interaction_claim.range_check_19.claimed_sum;
    sum += interaction_claim.range_check7_2_5.claimed_sum;
    sum += interaction_claim.range_check4_3.claimed_sum;
    sum += interaction_claim.ret[0].claimed_sum.0;
    sum += interaction_claim.verify_instruction.claimed_sum.0;
    sum += interaction_claim.add_ap_im.claimed_sum.unwrap().0;
    sum += if let Some(claimed_sum) = &interaction_claim.jmp_rel_imm.claimed_sum {
        claimed_sum.0
    } else {
        interaction_claim.jmp_rel_imm.total_sum
    };

    sum += if let Some(claimed_sum) = &interaction_claim.call_rel.claimed_sum {
        claimed_sum.0
    } else {
        interaction_claim.call_rel.total_sum
    };

    sum += if let Some(claimed_sum) = &interaction_claim.generic_opcode.claimed_sum {
        claimed_sum.0
    } else {
        interaction_claim.generic_opcode.total_sum
    };

    sum
}

pub struct CairoComponents {
    ret: Vec<ret_opcode::Component>,
    add_ap_imm: addapopcode_is_imm_t_op1_base_fp_f::Component,
    jmp_rel_imm: jumpopcode_is_rel_t_is_imm_t_is_double_deref_f::Component,
    call_rel: callopcode_is_rel_t_op1_base_fp_f::Component,
    generic_opcode: genericopcode::Component,
    memory_addr_to_id: addr_to_id::Component,
    memory_id_to_value: (id_to_f252::BigComponent, id_to_f252::SmallComponent),
    verify_instruction: verifyinstruction::Component,
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
        let addapopcode_is_imm_t_op1_base_fp_f_component =
            addapopcode_is_imm_t_op1_base_fp_f::Component::new(
                tree_span_provider,
                addapopcode_is_imm_t_op1_base_fp_f::Eval {
                    claim: cairo_claim.add_ap_imm,
                    interaction_claim: interaction_claim.add_ap_im,
                    memoryaddresstoid_lookup_elements: interaction_elements
                        .memory_addr_to_id_lookup
                        .clone(),
                    memoryidtobig_lookup_elements: interaction_elements
                        .memory_id_to_value_lookup
                        .clone(),
                    verifyinstruction_lookup_elements: interaction_elements
                        .verify_instruction_lookup
                        .clone(),
                    opcodes_lookup_elements: interaction_elements.opcodes_lookup_elements.clone(),
                },
            );
        let jmp_rel_component = jumpopcode_is_rel_t_is_imm_t_is_double_deref_f::Component::new(
            tree_span_provider,
            jumpopcode_is_rel_t_is_imm_t_is_double_deref_f::Eval {
                claim: cairo_claim.jmp_rel_imm,
                interaction_claim: interaction_claim.jmp_rel_imm,
                addr_to_id_lookup_elements: interaction_elements.memory_addr_to_id_lookup.clone(),
                id_to_f252_lookup_elements: interaction_elements.memory_id_to_value_lookup.clone(),
                verifyinstruction_lookup_elements: interaction_elements
                    .verify_instruction_lookup
                    .clone(),
                opcodes_lookup_elements: interaction_elements.opcodes_lookup_elements.clone(),
            },
        );
        let call_rel_component = callopcode_is_rel_t_op1_base_fp_f::Component::new(
            tree_span_provider,
            callopcode_is_rel_t_op1_base_fp_f::Eval {
                claim: cairo_claim.call_opcode,
                interaction_claim: interaction_claim.call_rel,
                memoryaddresstoid_lookup_elements: interaction_elements
                    .memory_addr_to_id_lookup
                    .clone(),
                memoryidtobig_lookup_elements: interaction_elements
                    .memory_id_to_value_lookup
                    .clone(),
                verifyinstruction_lookup_elements: interaction_elements
                    .verify_instruction_lookup
                    .clone(),
                opcodes_lookup_elements: interaction_elements.opcodes_lookup_elements.clone(),
            },
        );
        let genericopcode_component = genericopcode::Component::new(
            tree_span_provider,
            genericopcode::Eval {
                claim: cairo_claim.generic_opcode,
                interaction_claim: interaction_claim.generic_opcode,
                memoryaddresstoid_lookup_elements: interaction_elements
                    .memory_addr_to_id_lookup
                    .clone(),
                memoryidtobig_lookup_elements: interaction_elements
                    .memory_id_to_value_lookup
                    .clone(),
                verifyinstruction_lookup_elements: interaction_elements
                    .verify_instruction_lookup
                    .clone(),
                opcodes_lookup_elements: interaction_elements.opcodes_lookup_elements.clone(),
                range_check_9_9_lookup_elements: interaction_elements.range9_9_lookup.clone(),
                range_check_19_lookup_elements: interaction_elements.range_check_19_lookup.clone(),
            },
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
        let range_check_19_component = range_check_19::Component::new(
            tree_span_provider,
            range_check_19::Eval::new(
                interaction_elements.range_check_19_lookup.clone(),
                interaction_claim.range_check_19.claimed_sum,
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
            add_ap_imm: addapopcode_is_imm_t_op1_base_fp_f_component,
            jmp_rel_imm: jmp_rel_component,
            call_rel: call_rel_component,
            generic_opcode: genericopcode_component,
            memory_addr_to_id: memory_addr_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_component,
                small_memory_id_to_value_component,
            ),
            verify_instruction: verifyinstruction_component,
            range_check_19: range_check_19_component,
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
        vec.push(&self.add_ap_imm);
        vec.push(&self.jmp_rel_imm);
        vec.push(&self.call_rel);
        vec.push(&self.generic_opcode);
        vec.push(&self.memory_addr_to_id);
        vec.push(&self.memory_id_to_value.0);
        vec.push(&self.memory_id_to_value.1);
        vec.push(&self.verify_instruction);
        vec.push(&self.range_check_19);
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

const LOG_MAX_ROWS: u32 = 20;

const IS_FIRST_LOG_SIZES: [u32; 7] = [4, 5, 6, 7, 14, 18, 19];
pub fn prove_cairo(input: CairoInput) -> Result<CairoProof<Blake2sMerkleHasher>, ProvingError> {
    let _span = span!(Level::INFO, "prove_cairo").entered();
    let config = PcsConfig::default();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme = &mut CommitmentSchemeProver::new(config, &twiddles);

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();

    tree_builder.extend_evals(
        IS_FIRST_LOG_SIZES
            .iter()
            .cloned()
            .map(gen_is_first::<SimdBackend>),
    );
    tree_builder.commit(channel);

    // Extract public memory.
    let public_memory = input
        .public_mem_addresses
        .iter()
        .copied()
        .map(|addr| {
            let id = input.mem.address_to_id[addr as usize];
            (addr, id.0, input.mem.get(addr).as_u256())
        })
        .collect_vec();

    // TODO: Table interaction.

    // Base trace.
    // TODO(Ohad): change to OpcodeClaimProvers, and integrate padding.
    let ret_trace_generator = ret_opcode::ClaimGenerator::new(input.instructions.ret);
    let add_ap_imm_trace_generator =
        addapopcode_is_imm_t_op1_base_fp_f::ClaimGenerator::new(input.instructions.add_ap);
    let jumpopcode_is_rel_t_is_imm_t_is_double_deref_f_trace_generator =
        jumpopcode_is_rel_t_is_imm_t_is_double_deref_f::ClaimGenerator::new(
            input.instructions.jmp_rel_imm[0].clone(),
        );
    let call_opcode_is_rel_t_op1_base_fp_f_trace_generator =
        callopcode_is_rel_t_op1_base_fp_f::ClaimGenerator::new(input.instructions.call_rel_imm);
    let generic_opcode_trace_generator =
        genericopcode::ClaimGenerator::new(input.instructions.generic);
    let mut memory_addr_to_id_trace_generator = addr_to_id::ClaimGenerator::new(&input.mem);
    let mut memory_id_to_value_trace_generator = id_to_f252::ClaimGenerator::new(&input.mem);
    let mut range_check_19_trace_generator = range_check_19::ClaimGenerator::new();
    let mut range_check_9_9_trace_generator = range_check_9_9::ClaimGenerator::new();
    let mut verify_instruction_trace_generator = verifyinstruction::ClaimGenerator::default();
    let mut range_check_7_2_5_trace_generator = range_check_7_2_5::ClaimGenerator::new();
    let mut range_check_4_3_trace_generator = range_check_4_3::ClaimGenerator::new();

    // TODO(Ohad): Add public memory.
    for &addr in &input.public_mem_addresses {
        let id = memory_addr_to_id_trace_generator.ids[addr as usize];
        memory_addr_to_id_trace_generator.add_m31(M31::from_u32_unchecked(addr));
        memory_id_to_value_trace_generator.add_m31(M31::from_u32_unchecked(id));
    }

    println!(
        "d_mults: {:?}",
        memory_id_to_value_trace_generator.small_mults[7]
    );

    let mut tree_builder = commitment_scheme.tree_builder();

    let (ret_claim, ret_interaction_prover) = ret_trace_generator.write_trace(
        &mut tree_builder,
        &mut memory_addr_to_id_trace_generator,
        &mut memory_id_to_value_trace_generator,
        &mut verify_instruction_trace_generator,
    );
    let (add_ap_imm_claim, add_ap_imm_interaction_prover) = add_ap_imm_trace_generator.write_trace(
        &mut tree_builder,
        &mut memory_addr_to_id_trace_generator,
        &mut memory_id_to_value_trace_generator,
        &mut verify_instruction_trace_generator,
    );
    let (jmp_rel_imm_claim, jmp_rel_imm_interaction_prover) =
        jumpopcode_is_rel_t_is_imm_t_is_double_deref_f_trace_generator.write_trace(
            &mut tree_builder,
            &mut memory_addr_to_id_trace_generator,
            &mut memory_id_to_value_trace_generator,
            &mut verify_instruction_trace_generator,
        );
    let (call_rel_claim, call_rel_interaction_prover) =
        call_opcode_is_rel_t_op1_base_fp_f_trace_generator.write_trace(
            &mut tree_builder,
            &mut memory_addr_to_id_trace_generator,
            &mut memory_id_to_value_trace_generator,
            &mut verify_instruction_trace_generator,
        );
    let (generic_opcode_claim, generic_opcode_interaction_prover) = generic_opcode_trace_generator
        .write_trace(
            &mut tree_builder,
            &mut memory_addr_to_id_trace_generator,
            &mut memory_id_to_value_trace_generator,
            &mut range_check_19_trace_generator,
            &mut range_check_9_9_trace_generator,
            &mut verify_instruction_trace_generator,
        );
    let (memory_addr_to_id_claim, memory_addr_to_id_interaction_prover) =
        memory_addr_to_id_trace_generator.write_trace(&mut tree_builder);
    let (memory_id_to_value_claim, memory_id_to_value_interaction_prover) =
        memory_id_to_value_trace_generator
            .write_trace(&mut tree_builder, &mut range_check_9_9_trace_generator);
    let (vi_claim, verify_instruction_interaction_generator) = verify_instruction_trace_generator
        .write_trace(
            &mut tree_builder,
            &mut memory_addr_to_id_trace_generator,
            &mut range_check_4_3_trace_generator,
            &mut range_check_7_2_5_trace_generator,
        );
    let (range_check_19_claim, range_check_19_interaction_prover) =
        range_check_19_trace_generator.write_trace(&mut tree_builder);
    let (range_check9_9_claim, range_check9_9_interaction_prover) =
        range_check_9_9_trace_generator.write_trace(&mut tree_builder);
    let (range_check_7_2_5_claim, range_check_7_2_5_interaction_prover) =
        range_check_7_2_5_trace_generator.write_trace(&mut tree_builder);
    let (range_check_4_3_claim, range_check_4_3_interaction_prover) =
        range_check_4_3_trace_generator.write_trace(&mut tree_builder);
    // Commit to the claim and the trace.
    let claim = CairoClaim {
        public_memory,
        initial_state: input.instructions.initial_state,
        final_state: input.instructions.final_state,
        ret: vec![ret_claim],
        memory_addr_to_id: memory_addr_to_id_claim.clone(),
        add_ap_imm: add_ap_imm_claim,
        jmp_rel_imm: jmp_rel_imm_claim,
        call_opcode: call_rel_claim,
        memory_id_to_value: memory_id_to_value_claim.clone(),
        generic_opcode: generic_opcode_claim,
        range_check_19: range_check_19_claim,
        range_check9_9: range_check9_9_claim.clone(),
        range_check7_2_5: range_check_7_2_5_claim.clone(),
        range_check4_3: range_check_4_3_claim.clone(),
        verify_instruction: vi_claim,
    };
    claim.mix_into(channel);
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let ret_interaction_claim = ret_interaction_prover.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.memory_addr_to_id_lookup,
        &interaction_elements.memory_id_to_value_lookup,
        &interaction_elements.verify_instruction_lookup,
        &interaction_elements.opcodes_lookup_elements,
    );
    let add_ap_imm_interaction_claim = add_ap_imm_interaction_prover.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.memory_addr_to_id_lookup,
        &interaction_elements.memory_id_to_value_lookup,
        &interaction_elements.verify_instruction_lookup,
        &interaction_elements.opcodes_lookup_elements,
    );
    let jmp_rel_imm_interaction_claim = jmp_rel_imm_interaction_prover.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.memory_addr_to_id_lookup,
        &interaction_elements.memory_id_to_value_lookup,
        &interaction_elements.verify_instruction_lookup,
        &interaction_elements.opcodes_lookup_elements,
    );
    let call_rel_interaction_claim = call_rel_interaction_prover.write_interaction_trace(
        &mut tree_builder,
        &interaction_elements.memory_addr_to_id_lookup,
        &interaction_elements.memory_id_to_value_lookup,
        &interaction_elements.verify_instruction_lookup,
        &interaction_elements.opcodes_lookup_elements,
    );
    let generic_opcode_interaction_claim = generic_opcode_interaction_prover
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_addr_to_id_lookup,
            &interaction_elements.memory_id_to_value_lookup,
            &interaction_elements.range_check_19_lookup,
            &interaction_elements.range9_9_lookup,
            &interaction_elements.verify_instruction_lookup,
            &interaction_elements.opcodes_lookup_elements,
        );
    let memory_addr_to_id_interaction_claim = memory_addr_to_id_interaction_prover
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_addr_to_id_lookup,
        );
    let memory_id_to_value_interaction_claim = memory_id_to_value_interaction_prover
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_id_to_value_lookup,
            &interaction_elements.range9_9_lookup,
        );
    let verifyinstruction_interaction_claim = verify_instruction_interaction_generator
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_addr_to_id_lookup,
            &interaction_elements.memory_id_to_value_lookup,
            &interaction_elements.range4_3_lookup,
            &interaction_elements.range7_2_5_lookup,
            &interaction_elements.verify_instruction_lookup,
        );
    let range_check_19_interaction_claim = range_check_19_interaction_prover
        .write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.range_check_19_lookup,
        );
    let range_check9_9_interaction_claim = range_check9_9_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range9_9_lookup);
    let range_check_7_2_5_interaction_claim = range_check_7_2_5_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range7_2_5_lookup);
    let range_check_4_3_interaction_claim = range_check_4_3_interaction_prover
        .write_interaction_trace(&mut tree_builder, &interaction_elements.range4_3_lookup);

    // Commit to the interaction claim and the interaction trace.
    let interaction_claim = CairoInteractionClaim {
        ret: vec![ret_interaction_claim],
        add_ap_im: add_ap_imm_interaction_claim,
        jmp_rel_imm: jmp_rel_imm_interaction_claim,
        call_rel: call_rel_interaction_claim,
        memory_addr_to_id: memory_addr_to_id_interaction_claim.clone(),
        memory_id_to_value: memory_id_to_value_interaction_claim.clone(),
        generic_opcode: generic_opcode_interaction_claim,
        range_check_19: range_check_19_interaction_claim,
        range_check9_9: range_check9_9_interaction_claim.clone(),
        range_check7_2_5: range_check_7_2_5_interaction_claim.clone(),
        range_check4_3: range_check_4_3_interaction_claim.clone(),
        verify_instruction: verifyinstruction_interaction_claim,
    };
    let sum = lookup_sum(&claim, &interaction_elements, &interaction_claim);
    GLOBAL_TRACKER.lock().unwrap().summarize_relations();
    // debug_assert!(sum.is_zero());
    // debug_assert_eq!(
    //     lookup_sum(&claim, &interaction_elements, &interaction_claim),
    //     SecureField::zero()
    // );
    println!("sum: {:?}", sum);
    interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_builder.provers();

    // Prove stark.
    let proof = prove::<SimdBackend, _>(&components, channel, commitment_scheme)?;

    Ok(CairoProof {
        claim,
        interaction_claim,
        stark_proof: proof,
    })
}

pub fn verify_cairo(
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<Blake2sMerkleHasher>,
) -> Result<(), CairoVerificationError> {
    // Verify.
    let config = PcsConfig::default();
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme_verifier =
        &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);

    let log_sizes = claim.log_sizes();

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);
    let interaction_elements = CairoInteractionElements::draw(channel);
    // if lookup_sum(&claim, &interaction_elements, &interaction_claim) != SecureField::zero() {
    //     return Err(CairoVerificationError::InvalidLogupSum);
    // }
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[2], &log_sizes[2], channel);

    let component_generator =
        CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_generator.components();

    verify(
        &components,
        channel,
        commitment_scheme_verifier,
        stark_proof,
    )
    .map_err(CairoVerificationError::Stark)
}

#[derive(Error, Debug)]
pub enum CairoVerificationError {
    #[error("Invalid logup sum")]
    InvalidLogupSum,
    #[error("Stark verification error: {0}")]
    Stark(#[from] VerificationError),
}

#[cfg(test)]
mod tests {
    use cairo_lang_casm::casm;
    use itertools::Itertools;

    use crate::cairo_air::{prove_cairo, verify_cairo, CairoInput};
    use crate::felt::split_f252;
    use crate::input::mem::u128_to_4_limbs;
    use crate::input::plain::input_from_plain_casm;
    use crate::input::vm_import::tests::small_cairo_input;

    fn test_input() -> CairoInput {
        let u128_max = u128::MAX;
        let instructions = casm! {
            // TODO(AlonH): Add actual range check segment.
            // Manually writing range check builtin segment of size 40 to memory.
            [ap] = u128_max, ap++;
            [ap + 38] = 1, ap++;
            ap += 38;

            [ap] = 10, ap++;
            call rel 4;
            jmp rel 11;

            jmp rel 4 if [fp-3] != 0;
            jmp rel 6;
            [ap] = [fp-3] + (-1), ap++;
            call rel (-6);
            ret;
        }
        .instructions;

        input_from_plain_casm(instructions)
    }

    #[test]
    fn test_basic_cairo_air() {
        let input = test_input();
        println!(
            "{:?}",
            input
                .mem
                .f252_values
                .into_iter()
                .map(split_f252)
                .collect_vec()
        );
        println!(
            "small values: {:?}",
            input
                .mem
                .small_values
                .into_iter()
                .map(u128_to_4_limbs)
                .collect_vec()
        );
        let cairo_proof = prove_cairo(test_input()).unwrap();
        verify_cairo(cairo_proof).unwrap();
    }

    #[ignore]
    #[test]
    fn test_full_cairo_air() {
        let cairo_proof = prove_cairo(small_cairo_input()).unwrap();
        verify_cairo(cairo_proof).unwrap();
    }
}

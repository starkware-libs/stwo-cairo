use std::array;
use std::sync::Arc;

use cairo_air::air::{
    CairoClaim, CairoInteractionClaim, MemorySmallValue, PublicData, PublicMemory,
    PublicSegmentRanges, SegmentRange,
};
use cairo_air::blake::air as blake_air;
use cairo_air::builtins_air::BuiltinsInteractionClaim;
use cairo_air::opcodes_air::OpcodeInteractionClaim;
use cairo_air::pedersen::air as pedersen_air;
use cairo_air::poseidon::air as poseidon_air;
use cairo_air::range_checks_air::RangeChecksInteractionClaim;
use cairo_air::relations::CommonLookupElements;
use indexmap::IndexSet;
use itertools::Itertools;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::MAX_SEQUENCE_LOG_SIZE;
use tracing::{span, Level};

use super::builtins::BuiltinsInteractionClaimGenerator;
use super::opcodes::{
    process_interaction_gens, InteractionTraceResult, OpcodesInteractionClaimGenerator,
};
use super::range_checks::RangeChecksInteractionClaimGenerator;
use crate::witness::blake_context::{
    blake_context_write_trace, BlakeContextInteractionClaimGenerator,
};
use crate::witness::builtins::{builtins_write_trace, get_builtins};
use crate::witness::cairo_claim_generator::{get_sub_components, CairoClaimGenerator};
use crate::witness::components::pedersen::{
    pedersen_context_write_trace, PedersenContextInteractionClaimGenerator,
};
use crate::witness::components::poseidon::{
    poseidon_context_write_trace, PoseidonContextInteractionClaimGenerator,
};
use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_9, verify_instruction,
};
use crate::witness::opcodes::{get_opcodes, opcodes_write_trace};
use crate::witness::prelude::{PreProcessedTrace, M31};
use crate::witness::range_checks::{get_range_checks, range_checks_write_trace};
use crate::witness::utils::{CollectingTreeBuilder, TreeBuilder};

fn extract_public_segments(
    memory: &Memory,
    initial_ap: u32,
    final_ap: u32,
    public_segment_context: PublicSegmentContext,
) -> PublicSegmentRanges {
    let n_public_segments = public_segment_context.iter().filter(|&b| *b).count() as u32;

    let to_memory_value = |addr: u32| {
        let id = memory.get_raw_id(addr);
        let value = memory.get(addr).as_small() as u32;
        MemorySmallValue { id, value }
    };

    let start_ptrs = (initial_ap..initial_ap + n_public_segments).map(to_memory_value);
    let end_ptrs = (final_ap - n_public_segments..final_ap).map(to_memory_value);
    let mut ranges = start_ptrs
        .zip(end_ptrs)
        .map(|(start_ptr, stop_ptr)| SegmentRange {
            start_ptr,
            stop_ptr,
        });
    let mut present = public_segment_context.into_iter();
    let mut next = || {
        let present = present.next().unwrap();
        if present {
            ranges.next()
        } else {
            None
        }
    };

    PublicSegmentRanges {
        output: next().unwrap(),
        pedersen: next(),
        range_check_128: next(),
        ecdsa: next(),
        bitwise: next(),
        ec_op: next(),
        keccak: next(),
        poseidon: next(),
        range_check_96: next(),
        add_mod: next(),
        mul_mod: next(),
    }
}

fn extract_sections_from_memory(
    memory: &Memory,
    initial_pc: u32,
    initial_ap: u32,
    final_ap: u32,
    public_segment_context: PublicSegmentContext,
) -> PublicMemory {
    let public_segments =
        extract_public_segments(memory, initial_ap, final_ap, public_segment_context);
    let program_memory_addresses = initial_pc..initial_ap - 2;
    let safe_call_addresses = initial_ap - 2..initial_ap;
    let output_memory_addresses =
        public_segments.output.start_ptr.value..public_segments.output.stop_ptr.value;
    let [program, safe_call, output] = [
        program_memory_addresses,
        safe_call_addresses,
        output_memory_addresses,
    ]
    .map(|range| {
        range
            .map(|addr| {
                let id = memory.get_raw_id(addr);
                let value = memory.get(addr).as_u256();
                (id, value)
            })
            .collect_vec()
    });

    assert!(safe_call.len() == 2);

    assert_eq!(safe_call[0].1, [initial_ap, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(safe_call[1].1, [0, 0, 0, 0, 0, 0, 0, 0]);

    PublicMemory {
        program,
        safe_call_ids: array::from_fn(|i| safe_call[i].0),
        public_segments,
        output,
    }
}

/// CairoClaimGenerator responsible for generating the CairoClaim and writing the trace.
/// NOTE: Order of writing the trace is important, and should be consistent with [`CairoClaim`],
/// [`CairoInteractionClaim`], [`CairoComponents`].
pub fn create_cairo_claim_generator(
    ProverInput {
        state_transitions,
        memory,
        public_memory_addresses,
        builtin_segments,
        public_segment_context,
        ..
    }: ProverInput,
    preprocessed_trace: Arc<PreProcessedTrace>,
) -> CairoClaimGenerator {
    let initial_state = state_transitions.initial_state;
    let final_state = state_transitions.final_state;

    let mut all_components = IndexSet::new();
    for opcode in get_opcodes(&state_transitions.casm_states_by_opcode) {
        all_components.extend(get_sub_components(opcode));
    }
    for builtin in get_builtins(&builtin_segments) {
        all_components.extend(get_sub_components(builtin));
    }
    // TODO(Stav): remove after range checks and verify bitwise xor are optional in the claim.
    all_components.extend(get_range_checks());
    all_components.insert("verify_bitwise_xor_4");
    all_components.insert("verify_bitwise_xor_7");
    all_components.insert("verify_bitwise_xor_8");
    all_components.insert("verify_bitwise_xor_9");

    // Public data.
    let initial_pc = initial_state.pc.0;
    let initial_ap = initial_state.ap.0;
    let final_ap = final_state.ap.0;
    let public_memory = extract_sections_from_memory(
        &memory,
        initial_pc,
        initial_ap,
        final_ap,
        public_segment_context,
    );

    let public_data = PublicData {
        public_memory,
        initial_state,
        final_state,
    };

    let mut cairo_claim_generator = CairoClaimGenerator {
        public_data,
        ..Default::default()
    };
    cairo_claim_generator.fill_components(
        &all_components,
        state_transitions.casm_states_by_opcode,
        &builtin_segments,
        Arc::new(memory),
        preprocessed_trace,
    );

    let memory_address_to_id_trace_generator =
        cairo_claim_generator.memory_address_to_id.as_ref().unwrap();
    let memory_id_to_value_trace_generator =
        cairo_claim_generator.memory_id_to_big.as_ref().unwrap();
    // Yield public memory.
    for addr in public_memory_addresses
        .iter()
        .copied()
        .map(M31::from_u32_unchecked)
    {
        let id = memory_address_to_id_trace_generator.get_id(addr);
        memory_address_to_id_trace_generator.add_input(&addr);
        memory_id_to_value_trace_generator.add_input(&id);
    }

    cairo_claim_generator
}

impl CairoClaimGenerator {
    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write opcode trace").entered();
        let (opcodes_claim, opcodes_interaction_gen) = opcodes_write_trace(
            self.add_opcode,
            self.add_opcode_small,
            self.add_ap_opcode,
            self.assert_eq_opcode,
            self.assert_eq_opcode_imm,
            self.assert_eq_opcode_double_deref,
            self.blake_compress_opcode,
            self.call_opcode_abs,
            self.call_opcode_rel_imm,
            self.generic_opcode,
            self.jnz_opcode_non_taken,
            self.jnz_opcode_taken,
            self.jump_opcode_abs,
            self.jump_opcode_double_deref,
            self.jump_opcode_rel,
            self.jump_opcode_rel_imm,
            self.mul_opcode,
            self.mul_opcode_small,
            self.qm_31_add_mul_opcode,
            self.ret_opcode,
            tree_builder,
            &mut self.blake_round,
            &mut self.triple_xor_32,
            self.memory_address_to_id.as_ref(),
            self.memory_id_to_big.as_ref(),
            self.range_check_7_2_5.as_ref(),
            self.range_check_11.as_ref(),
            self.range_check_18.as_ref(),
            self.range_check_20.as_ref(),
            self.range_check_4_4_4_4.as_ref(),
            self.range_check_9_9.as_ref(),
            self.verify_instruction.as_ref(),
            self.verify_bitwise_xor_8.as_mut(),
        );
        span.exit();
        let span = span!(Level::INFO, "internal component trace").entered();
        let (verify_instruction_claim, verify_instruction_interaction_gen) =
            self.verify_instruction.unwrap().write_trace(
                tree_builder,
                self.range_check_7_2_5.as_ref().unwrap(),
                self.range_check_4_3.as_ref().unwrap(),
                self.memory_address_to_id.as_ref().unwrap(),
                self.memory_id_to_big.as_ref().unwrap(),
            );
        let (blake_context_claim, blake_context_interaction_gen) = blake_context_write_trace(
            self.blake_round,
            self.blake_g,
            self.blake_round_sigma,
            self.triple_xor_32,
            self.verify_bitwise_xor_12,
            tree_builder,
            self.memory_address_to_id.as_ref(),
            self.memory_id_to_big.as_ref(),
            self.range_check_7_2_5.as_ref(),
            self.verify_bitwise_xor_4.as_ref(),
            self.verify_bitwise_xor_7.as_ref(),
            self.verify_bitwise_xor_8.as_ref(),
            self.verify_bitwise_xor_9.as_ref(),
        );
        let (builtins_claim, builtins_interaction_gen) = builtins_write_trace(
            self.add_mod_builtin,
            self.bitwise_builtin,
            self.mul_mod_builtin,
            self.pedersen_builtin,
            self.poseidon_builtin,
            self.range_check96_builtin,
            self.range_check_builtin,
            tree_builder,
            self.memory_address_to_id.as_ref(),
            self.memory_id_to_big.as_ref(),
            self.pedersen_aggregator_window_bits_18.as_ref(),
            self.poseidon_aggregator.as_ref(),
            self.range_check_6.as_ref(),
            self.range_check_12.as_ref(),
            self.range_check_18.as_ref(),
            self.range_check_3_6_6_3.as_ref(),
            self.verify_bitwise_xor_8.as_ref(),
            self.verify_bitwise_xor_9.as_ref(),
        );
        let (pedersen_context_claim, pedersen_context_interaction_gen) =
            pedersen_context_write_trace(
                self.pedersen_aggregator_window_bits_18,
                self.partial_ec_mul_window_bits_18,
                self.pedersen_points_table_window_bits_18,
                tree_builder,
                self.memory_id_to_big.as_ref(),
                self.range_check_8.as_ref(),
                self.range_check_9_9.as_ref(),
                self.range_check_20.as_ref(),
            );
        let (poseidon_context_claim, poseidon_context_interaction_gen) =
            poseidon_context_write_trace(
                self.poseidon_aggregator,
                self.poseidon_3_partial_rounds_chain,
                self.poseidon_full_round_chain,
                self.cube_252,
                self.poseidon_round_keys,
                self.range_check_252_width_27,
                tree_builder,
                self.memory_id_to_big.as_ref(),
                self.range_check_3_3_3_3_3.as_ref(),
                self.range_check_4_4_4_4.as_ref(),
                self.range_check_4_4.as_ref(),
                self.range_check_9_9.as_ref(),
                self.range_check_18.as_ref(),
                self.range_check_20.as_ref(),
            );
        let (memory_address_to_id_claim, memory_address_to_id_interaction_gen) =
            self.memory_address_to_id.unwrap().write_trace(tree_builder);

        // Memory uses "Sequence", split it according to `MAX_SEQUENCE_LOG_SIZE`.
        const LOG_MAX_BIG_SIZE: u32 = MAX_SEQUENCE_LOG_SIZE;
        let (memory_id_to_value_claim, memory_id_to_value_interaction_gen) =
            self.memory_id_to_big.unwrap().write_trace(
                tree_builder,
                self.range_check_9_9.as_ref().unwrap(),
                LOG_MAX_BIG_SIZE,
            );
        let (range_checks_claim, range_checks_interaction_gen) = range_checks_write_trace(
            self.range_check_6,
            self.range_check_8,
            self.range_check_11,
            self.range_check_12,
            self.range_check_18,
            self.range_check_20,
            self.range_check_4_3,
            self.range_check_4_4,
            self.range_check_9_9,
            self.range_check_7_2_5,
            self.range_check_3_6_6_3,
            self.range_check_4_4_4_4,
            self.range_check_3_3_3_3_3,
            tree_builder,
        );
        let (verify_bitwise_xor_4_claim, verify_bitwise_xor_4_interaction_gen) =
            self.verify_bitwise_xor_4.unwrap().write_trace(tree_builder);
        let (verify_bitwise_xor_7_claim, verify_bitwise_xor_7_interaction_gen) =
            self.verify_bitwise_xor_7.unwrap().write_trace(tree_builder);
        let (verify_bitwise_xor_8_claim, verify_bitwise_xor_8_interaction_gen) =
            self.verify_bitwise_xor_8.unwrap().write_trace(tree_builder);
        let (verify_bitwise_xor_9_claim, verify_bitwise_xor_9_interaction_gen) =
            self.verify_bitwise_xor_9.unwrap().write_trace(tree_builder);
        span.exit();
        (
            CairoClaim {
                public_data: self.public_data,
                opcodes: opcodes_claim,
                verify_instruction: verify_instruction_claim,
                blake_context: blake_context_claim,
                builtins: builtins_claim,
                pedersen_context: pedersen_context_claim,
                poseidon_context: poseidon_context_claim,
                memory_address_to_id: memory_address_to_id_claim,
                memory_id_to_value: memory_id_to_value_claim,
                range_checks: range_checks_claim,
                verify_bitwise_xor_4: verify_bitwise_xor_4_claim,
                verify_bitwise_xor_7: verify_bitwise_xor_7_claim,
                verify_bitwise_xor_8: verify_bitwise_xor_8_claim,
                verify_bitwise_xor_9: verify_bitwise_xor_9_claim,
            },
            CairoInteractionClaimGenerator {
                opcodes_interaction_gen,
                verify_instruction_interaction_gen,
                blake_context_interaction_gen,
                builtins_interaction_gen,
                pedersen_context_interaction_gen,
                poseidon_context_interaction_gen,
                memory_address_to_id_interaction_gen,
                memory_id_to_value_interaction_gen,
                range_checks_interaction_gen,
                verify_bitwise_xor_4_interaction_gen,
                verify_bitwise_xor_7_interaction_gen,
                verify_bitwise_xor_8_interaction_gen,
                verify_bitwise_xor_9_interaction_gen,
            },
        )
    }
}

pub struct CairoInteractionClaimGenerator {
    opcodes_interaction_gen: OpcodesInteractionClaimGenerator,
    verify_instruction_interaction_gen: verify_instruction::InteractionClaimGenerator,
    blake_context_interaction_gen: BlakeContextInteractionClaimGenerator,
    builtins_interaction_gen: BuiltinsInteractionClaimGenerator,
    pedersen_context_interaction_gen: PedersenContextInteractionClaimGenerator,
    poseidon_context_interaction_gen: PoseidonContextInteractionClaimGenerator,
    memory_address_to_id_interaction_gen: memory_address_to_id::InteractionClaimGenerator,
    memory_id_to_value_interaction_gen: memory_id_to_big::InteractionClaimGenerator,
    range_checks_interaction_gen: RangeChecksInteractionClaimGenerator,
    verify_bitwise_xor_4_interaction_gen: verify_bitwise_xor_4::InteractionClaimGenerator,
    verify_bitwise_xor_7_interaction_gen: verify_bitwise_xor_7::InteractionClaimGenerator,
    verify_bitwise_xor_8_interaction_gen: verify_bitwise_xor_8::InteractionClaimGenerator,
    verify_bitwise_xor_9_interaction_gen: verify_bitwise_xor_9::InteractionClaimGenerator,
    // ...
}
/// Helper struct to hold a claim and its collected evaluations.
struct ClaimWithEvals<T> {
    claim: T,
    evals: CollectingTreeBuilder,
}

/// Helper function to process a single interaction generator.
fn process_single_gen<G, C>(
    gen: G,
    common_lookup_elements: &CommonLookupElements,
) -> ClaimWithEvals<C>
where
    G: FnOnce(&mut CollectingTreeBuilder, &CommonLookupElements) -> C,
{
    let mut builder = CollectingTreeBuilder::new();
    let claim = gen(&mut builder, common_lookup_elements);
    ClaimWithEvals {
        claim,
        evals: builder,
    }
}

// === Result structs for parallel processing ===

struct OpcodesResults {
    add: InteractionTraceResult<cairo_air::components::add_opcode::InteractionClaim>,
    add_small: InteractionTraceResult<cairo_air::components::add_opcode_small::InteractionClaim>,
    add_ap: InteractionTraceResult<cairo_air::components::add_ap_opcode::InteractionClaim>,
    assert_eq: InteractionTraceResult<cairo_air::components::assert_eq_opcode::InteractionClaim>,
    assert_eq_imm:
        InteractionTraceResult<cairo_air::components::assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_double_deref: InteractionTraceResult<
        cairo_air::components::assert_eq_opcode_double_deref::InteractionClaim,
    >,
    blake: InteractionTraceResult<cairo_air::components::blake_compress_opcode::InteractionClaim>,
    call: InteractionTraceResult<cairo_air::components::call_opcode_abs::InteractionClaim>,
    call_rel_imm:
        InteractionTraceResult<cairo_air::components::call_opcode_rel_imm::InteractionClaim>,
    generic: InteractionTraceResult<cairo_air::components::generic_opcode::InteractionClaim>,
    jnz: InteractionTraceResult<cairo_air::components::jnz_opcode_non_taken::InteractionClaim>,
    jnz_taken: InteractionTraceResult<cairo_air::components::jnz_opcode_taken::InteractionClaim>,
    jump: InteractionTraceResult<cairo_air::components::jump_opcode_abs::InteractionClaim>,
    jump_double_deref:
        InteractionTraceResult<cairo_air::components::jump_opcode_double_deref::InteractionClaim>,
    jump_rel: InteractionTraceResult<cairo_air::components::jump_opcode_rel::InteractionClaim>,
    jump_rel_imm:
        InteractionTraceResult<cairo_air::components::jump_opcode_rel_imm::InteractionClaim>,
    mul: InteractionTraceResult<cairo_air::components::mul_opcode::InteractionClaim>,
    mul_small: InteractionTraceResult<cairo_air::components::mul_opcode_small::InteractionClaim>,
    qm31: InteractionTraceResult<cairo_air::components::qm_31_add_mul_opcode::InteractionClaim>,
    ret: InteractionTraceResult<cairo_air::components::ret_opcode::InteractionClaim>,
}

// === Enums for parallel iterator processing ===

use rayon::prelude::*;

use crate::witness::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, blake_g, blake_round, blake_round_sigma,
    call_opcode_abs, call_opcode_rel_imm, cube_252, generic_opcode, jnz_opcode_non_taken,
    jnz_opcode_taken, jump_opcode_abs, jump_opcode_double_deref, jump_opcode_rel,
    jump_opcode_rel_imm, mul_opcode, mul_opcode_small, partial_ec_mul_window_bits_18,
    pedersen_aggregator_window_bits_18, pedersen_points_table_window_bits_18,
    poseidon_3_partial_rounds_chain, poseidon_aggregator, poseidon_full_round_chain,
    poseidon_round_keys, qm_31_add_mul_opcode, range_check_11, range_check_12,
    range_check_252_width_27, range_check_18, range_check_20, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9, ret_opcode, triple_xor_32,
    verify_bitwise_xor_12,
};

/// Enum for opcode interaction task inputs.
enum OpcodeInteractionTask {
    Add(Vec<add_opcode::InteractionClaimGenerator>),
    AddSmall(Vec<add_opcode_small::InteractionClaimGenerator>),
    AddAp(Vec<add_ap_opcode::InteractionClaimGenerator>),
    AssertEq(Vec<assert_eq_opcode::InteractionClaimGenerator>),
    AssertEqImm(Vec<assert_eq_opcode_imm::InteractionClaimGenerator>),
    AssertEqDoubleDeref(Vec<assert_eq_opcode_double_deref::InteractionClaimGenerator>),
    Blake(Vec<blake_compress_opcode::InteractionClaimGenerator>),
    Call(Vec<call_opcode_abs::InteractionClaimGenerator>),
    CallRelImm(Vec<call_opcode_rel_imm::InteractionClaimGenerator>),
    Generic(Vec<generic_opcode::InteractionClaimGenerator>),
    Jnz(Vec<jnz_opcode_non_taken::InteractionClaimGenerator>),
    JnzTaken(Vec<jnz_opcode_taken::InteractionClaimGenerator>),
    Jump(Vec<jump_opcode_abs::InteractionClaimGenerator>),
    JumpDoubleDeref(Vec<jump_opcode_double_deref::InteractionClaimGenerator>),
    JumpRel(Vec<jump_opcode_rel::InteractionClaimGenerator>),
    JumpRelImm(Vec<jump_opcode_rel_imm::InteractionClaimGenerator>),
    Mul(Vec<mul_opcode::InteractionClaimGenerator>),
    MulSmall(Vec<mul_opcode_small::InteractionClaimGenerator>),
    Qm31(Vec<qm_31_add_mul_opcode::InteractionClaimGenerator>),
    Ret(Vec<ret_opcode::InteractionClaimGenerator>),
}

/// Enum for opcode interaction task results.
enum OpcodeInteractionResult {
    Add(InteractionTraceResult<cairo_air::components::add_opcode::InteractionClaim>),
    AddSmall(InteractionTraceResult<cairo_air::components::add_opcode_small::InteractionClaim>),
    AddAp(InteractionTraceResult<cairo_air::components::add_ap_opcode::InteractionClaim>),
    AssertEq(InteractionTraceResult<cairo_air::components::assert_eq_opcode::InteractionClaim>),
    AssertEqImm(InteractionTraceResult<cairo_air::components::assert_eq_opcode_imm::InteractionClaim>),
    AssertEqDoubleDeref(InteractionTraceResult<cairo_air::components::assert_eq_opcode_double_deref::InteractionClaim>),
    Blake(InteractionTraceResult<cairo_air::components::blake_compress_opcode::InteractionClaim>),
    Call(InteractionTraceResult<cairo_air::components::call_opcode_abs::InteractionClaim>),
    CallRelImm(InteractionTraceResult<cairo_air::components::call_opcode_rel_imm::InteractionClaim>),
    Generic(InteractionTraceResult<cairo_air::components::generic_opcode::InteractionClaim>),
    Jnz(InteractionTraceResult<cairo_air::components::jnz_opcode_non_taken::InteractionClaim>),
    JnzTaken(InteractionTraceResult<cairo_air::components::jnz_opcode_taken::InteractionClaim>),
    Jump(InteractionTraceResult<cairo_air::components::jump_opcode_abs::InteractionClaim>),
    JumpDoubleDeref(InteractionTraceResult<cairo_air::components::jump_opcode_double_deref::InteractionClaim>),
    JumpRel(InteractionTraceResult<cairo_air::components::jump_opcode_rel::InteractionClaim>),
    JumpRelImm(InteractionTraceResult<cairo_air::components::jump_opcode_rel_imm::InteractionClaim>),
    Mul(InteractionTraceResult<cairo_air::components::mul_opcode::InteractionClaim>),
    MulSmall(InteractionTraceResult<cairo_air::components::mul_opcode_small::InteractionClaim>),
    Qm31(InteractionTraceResult<cairo_air::components::qm_31_add_mul_opcode::InteractionClaim>),
    Ret(InteractionTraceResult<cairo_air::components::ret_opcode::InteractionClaim>),
}

impl OpcodeInteractionTask {
    fn process(self, common_lookup_elements: &CommonLookupElements) -> OpcodeInteractionResult {
        match self {
            OpcodeInteractionTask::Add(gens) => OpcodeInteractionResult::Add(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::AddSmall(gens) => OpcodeInteractionResult::AddSmall(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::AddAp(gens) => OpcodeInteractionResult::AddAp(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::AssertEq(gens) => OpcodeInteractionResult::AssertEq(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::AssertEqImm(gens) => OpcodeInteractionResult::AssertEqImm(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::AssertEqDoubleDeref(gens) => OpcodeInteractionResult::AssertEqDoubleDeref(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Blake(gens) => OpcodeInteractionResult::Blake(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Call(gens) => OpcodeInteractionResult::Call(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::CallRelImm(gens) => OpcodeInteractionResult::CallRelImm(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Generic(gens) => OpcodeInteractionResult::Generic(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Jnz(gens) => OpcodeInteractionResult::Jnz(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::JnzTaken(gens) => OpcodeInteractionResult::JnzTaken(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Jump(gens) => OpcodeInteractionResult::Jump(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::JumpDoubleDeref(gens) => OpcodeInteractionResult::JumpDoubleDeref(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::JumpRel(gens) => OpcodeInteractionResult::JumpRel(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::JumpRelImm(gens) => OpcodeInteractionResult::JumpRelImm(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Mul(gens) => OpcodeInteractionResult::Mul(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::MulSmall(gens) => OpcodeInteractionResult::MulSmall(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Qm31(gens) => OpcodeInteractionResult::Qm31(process_interaction_gens(gens, common_lookup_elements)),
            OpcodeInteractionTask::Ret(gens) => OpcodeInteractionResult::Ret(process_interaction_gens(gens, common_lookup_elements)),
        }
    }
}

// === Helper functions for parallel processing ===

/// Process all opcodes using a parallel iterator.
fn process_all_opcodes(
    parts: super::opcodes::OpcodesInteractionParts,
    common_lookup_elements: &CommonLookupElements,
) -> OpcodesResults {
    // Build task list
    let tasks = vec![
        OpcodeInteractionTask::Add(parts.add),
        OpcodeInteractionTask::AddSmall(parts.add_small),
        OpcodeInteractionTask::AddAp(parts.add_ap),
        OpcodeInteractionTask::AssertEq(parts.assert_eq),
        OpcodeInteractionTask::AssertEqImm(parts.assert_eq_imm),
        OpcodeInteractionTask::AssertEqDoubleDeref(parts.assert_eq_double_deref),
        OpcodeInteractionTask::Blake(parts.blake),
        OpcodeInteractionTask::Call(parts.call),
        OpcodeInteractionTask::CallRelImm(parts.call_rel_imm),
        OpcodeInteractionTask::Generic(parts.generic_opcode),
        OpcodeInteractionTask::Jnz(parts.jnz),
        OpcodeInteractionTask::JnzTaken(parts.jnz_taken),
        OpcodeInteractionTask::Jump(parts.jump),
        OpcodeInteractionTask::JumpDoubleDeref(parts.jump_double_deref),
        OpcodeInteractionTask::JumpRel(parts.jump_rel),
        OpcodeInteractionTask::JumpRelImm(parts.jump_rel_imm),
        OpcodeInteractionTask::Mul(parts.mul),
        OpcodeInteractionTask::MulSmall(parts.mul_small),
        OpcodeInteractionTask::Qm31(parts.qm31),
        OpcodeInteractionTask::Ret(parts.ret),
    ];

    // Process all tasks in parallel
    let results: Vec<OpcodeInteractionResult> = tasks
        .into_par_iter()
        .map(|task| task.process(common_lookup_elements))
        .collect();

    // Extract results into typed struct
    let mut opcodes_results = OpcodesResults {
        add: InteractionTraceResult { claims: vec![], evals: vec![] },
        add_small: InteractionTraceResult { claims: vec![], evals: vec![] },
        add_ap: InteractionTraceResult { claims: vec![], evals: vec![] },
        assert_eq: InteractionTraceResult { claims: vec![], evals: vec![] },
        assert_eq_imm: InteractionTraceResult { claims: vec![], evals: vec![] },
        assert_eq_double_deref: InteractionTraceResult { claims: vec![], evals: vec![] },
        blake: InteractionTraceResult { claims: vec![], evals: vec![] },
        call: InteractionTraceResult { claims: vec![], evals: vec![] },
        call_rel_imm: InteractionTraceResult { claims: vec![], evals: vec![] },
        generic: InteractionTraceResult { claims: vec![], evals: vec![] },
        jnz: InteractionTraceResult { claims: vec![], evals: vec![] },
        jnz_taken: InteractionTraceResult { claims: vec![], evals: vec![] },
        jump: InteractionTraceResult { claims: vec![], evals: vec![] },
        jump_double_deref: InteractionTraceResult { claims: vec![], evals: vec![] },
        jump_rel: InteractionTraceResult { claims: vec![], evals: vec![] },
        jump_rel_imm: InteractionTraceResult { claims: vec![], evals: vec![] },
        mul: InteractionTraceResult { claims: vec![], evals: vec![] },
        mul_small: InteractionTraceResult { claims: vec![], evals: vec![] },
        qm31: InteractionTraceResult { claims: vec![], evals: vec![] },
        ret: InteractionTraceResult { claims: vec![], evals: vec![] },
    };

    for result in results {
        match result {
            OpcodeInteractionResult::Add(r) => opcodes_results.add = r,
            OpcodeInteractionResult::AddSmall(r) => opcodes_results.add_small = r,
            OpcodeInteractionResult::AddAp(r) => opcodes_results.add_ap = r,
            OpcodeInteractionResult::AssertEq(r) => opcodes_results.assert_eq = r,
            OpcodeInteractionResult::AssertEqImm(r) => opcodes_results.assert_eq_imm = r,
            OpcodeInteractionResult::AssertEqDoubleDeref(r) => opcodes_results.assert_eq_double_deref = r,
            OpcodeInteractionResult::Blake(r) => opcodes_results.blake = r,
            OpcodeInteractionResult::Call(r) => opcodes_results.call = r,
            OpcodeInteractionResult::CallRelImm(r) => opcodes_results.call_rel_imm = r,
            OpcodeInteractionResult::Generic(r) => opcodes_results.generic = r,
            OpcodeInteractionResult::Jnz(r) => opcodes_results.jnz = r,
            OpcodeInteractionResult::JnzTaken(r) => opcodes_results.jnz_taken = r,
            OpcodeInteractionResult::Jump(r) => opcodes_results.jump = r,
            OpcodeInteractionResult::JumpDoubleDeref(r) => opcodes_results.jump_double_deref = r,
            OpcodeInteractionResult::JumpRel(r) => opcodes_results.jump_rel = r,
            OpcodeInteractionResult::JumpRelImm(r) => opcodes_results.jump_rel_imm = r,
            OpcodeInteractionResult::Mul(r) => opcodes_results.mul = r,
            OpcodeInteractionResult::MulSmall(r) => opcodes_results.mul_small = r,
            OpcodeInteractionResult::Qm31(r) => opcodes_results.qm31 = r,
            OpcodeInteractionResult::Ret(r) => opcodes_results.ret = r,
        }
    }

    opcodes_results
}

/// Enum for all component interaction tasks (non-opcode components).
enum ComponentInteractionTask {
    // Memory
    MemoryAddressToId(memory_address_to_id::InteractionClaimGenerator),
    MemoryIdToValue(memory_id_to_big::InteractionClaimGenerator),
    // Verify instruction
    VerifyInstruction(verify_instruction::InteractionClaimGenerator),
    // Blake context
    BlakeRound(blake_round::InteractionClaimGenerator),
    BlakeG(blake_g::InteractionClaimGenerator),
    BlakeSigma(blake_round_sigma::InteractionClaimGenerator),
    TripleXor32(triple_xor_32::InteractionClaimGenerator),
    VerifyBitwiseXor12(verify_bitwise_xor_12::InteractionClaimGenerator),
    // Builtins
    AddModBuiltin(super::components::add_mod_builtin::InteractionClaimGenerator),
    BitwiseBuiltin(super::components::bitwise_builtin::InteractionClaimGenerator),
    MulModBuiltin(super::components::mul_mod_builtin::InteractionClaimGenerator),
    PedersenBuiltin(super::components::pedersen_builtin::InteractionClaimGenerator),
    PoseidonBuiltin(super::components::poseidon_builtin::InteractionClaimGenerator),
    RangeCheck96Builtin(super::components::range_check96_builtin::InteractionClaimGenerator),
    RangeCheck128Builtin(super::components::range_check_builtin::InteractionClaimGenerator),
    // Pedersen context
    PedersenAggregator(pedersen_aggregator_window_bits_18::InteractionClaimGenerator),
    PartialEcMul(partial_ec_mul_window_bits_18::InteractionClaimGenerator),
    PedersenPointsTable(pedersen_points_table_window_bits_18::InteractionClaimGenerator),
    // Poseidon context
    PoseidonAggregator(poseidon_aggregator::InteractionClaimGenerator),
    Poseidon3PartialRoundsChain(poseidon_3_partial_rounds_chain::InteractionClaimGenerator),
    PoseidonFullRoundChain(poseidon_full_round_chain::InteractionClaimGenerator),
    Cube252(cube_252::InteractionClaimGenerator),
    PoseidonRoundKeys(poseidon_round_keys::InteractionClaimGenerator),
    RangeCheck252Width27(range_check_252_width_27::InteractionClaimGenerator),
    // Range checks
    RangeCheck6(range_check_6::InteractionClaimGenerator),
    RangeCheck8(range_check_8::InteractionClaimGenerator),
    RangeCheck11(range_check_11::InteractionClaimGenerator),
    RangeCheck12(range_check_12::InteractionClaimGenerator),
    RangeCheck18(range_check_18::InteractionClaimGenerator),
    RangeCheck20(range_check_20::InteractionClaimGenerator),
    RangeCheck4_3(range_check_4_3::InteractionClaimGenerator),
    RangeCheck4_4(range_check_4_4::InteractionClaimGenerator),
    RangeCheck9_9(range_check_9_9::InteractionClaimGenerator),
    RangeCheck7_2_5(range_check_7_2_5::InteractionClaimGenerator),
    RangeCheck3_6_6_3(range_check_3_6_6_3::InteractionClaimGenerator),
    RangeCheck4_4_4_4(range_check_4_4_4_4::InteractionClaimGenerator),
    RangeCheck3_3_3_3_3(range_check_3_3_3_3_3::InteractionClaimGenerator),
    // Verify bitwise xor
    VerifyBitwiseXor4(verify_bitwise_xor_4::InteractionClaimGenerator),
    VerifyBitwiseXor7(verify_bitwise_xor_7::InteractionClaimGenerator),
    VerifyBitwiseXor8(verify_bitwise_xor_8::InteractionClaimGenerator),
    VerifyBitwiseXor9(verify_bitwise_xor_9::InteractionClaimGenerator),
}

/// Enum for all component interaction results.
enum ComponentInteractionResult {
    // Memory
    MemoryAddressToId(ClaimWithEvals<cairo_air::components::memory_address_to_id::InteractionClaim>),
    MemoryIdToValue(ClaimWithEvals<cairo_air::components::memory_id_to_big::InteractionClaim>),
    // Verify instruction
    VerifyInstruction(ClaimWithEvals<cairo_air::components::verify_instruction::InteractionClaim>),
    // Blake context
    BlakeRound(ClaimWithEvals<cairo_air::components::blake_round::InteractionClaim>),
    BlakeG(ClaimWithEvals<cairo_air::components::blake_g::InteractionClaim>),
    BlakeSigma(ClaimWithEvals<cairo_air::components::blake_round_sigma::InteractionClaim>),
    TripleXor32(ClaimWithEvals<cairo_air::components::triple_xor_32::InteractionClaim>),
    VerifyBitwiseXor12(ClaimWithEvals<cairo_air::components::verify_bitwise_xor_12::InteractionClaim>),
    // Builtins
    AddModBuiltin(ClaimWithEvals<cairo_air::components::add_mod_builtin::InteractionClaim>),
    BitwiseBuiltin(ClaimWithEvals<cairo_air::components::bitwise_builtin::InteractionClaim>),
    MulModBuiltin(ClaimWithEvals<cairo_air::components::mul_mod_builtin::InteractionClaim>),
    PedersenBuiltin(ClaimWithEvals<cairo_air::components::pedersen_builtin::InteractionClaim>),
    PoseidonBuiltin(ClaimWithEvals<cairo_air::components::poseidon_builtin::InteractionClaim>),
    RangeCheck96Builtin(ClaimWithEvals<cairo_air::components::range_check96_builtin::InteractionClaim>),
    RangeCheck128Builtin(ClaimWithEvals<cairo_air::components::range_check_builtin::InteractionClaim>),
    // Pedersen context
    PedersenAggregator(ClaimWithEvals<cairo_air::components::pedersen_aggregator_window_bits_18::InteractionClaim>),
    PartialEcMul(ClaimWithEvals<cairo_air::components::partial_ec_mul_window_bits_18::InteractionClaim>),
    PedersenPointsTable(ClaimWithEvals<cairo_air::components::pedersen_points_table_window_bits_18::InteractionClaim>),
    // Poseidon context
    PoseidonAggregator(ClaimWithEvals<cairo_air::components::poseidon_aggregator::InteractionClaim>),
    Poseidon3PartialRoundsChain(ClaimWithEvals<cairo_air::components::poseidon_3_partial_rounds_chain::InteractionClaim>),
    PoseidonFullRoundChain(ClaimWithEvals<cairo_air::components::poseidon_full_round_chain::InteractionClaim>),
    Cube252(ClaimWithEvals<cairo_air::components::cube_252::InteractionClaim>),
    PoseidonRoundKeys(ClaimWithEvals<cairo_air::components::poseidon_round_keys::InteractionClaim>),
    RangeCheck252Width27(ClaimWithEvals<cairo_air::components::range_check_252_width_27::InteractionClaim>),
    // Range checks
    RangeCheck6(ClaimWithEvals<cairo_air::components::range_check_6::InteractionClaim>),
    RangeCheck8(ClaimWithEvals<cairo_air::components::range_check_8::InteractionClaim>),
    RangeCheck11(ClaimWithEvals<cairo_air::components::range_check_11::InteractionClaim>),
    RangeCheck12(ClaimWithEvals<cairo_air::components::range_check_12::InteractionClaim>),
    RangeCheck18(ClaimWithEvals<cairo_air::components::range_check_18::InteractionClaim>),
    RangeCheck20(ClaimWithEvals<cairo_air::components::range_check_20::InteractionClaim>),
    RangeCheck4_3(ClaimWithEvals<cairo_air::components::range_check_4_3::InteractionClaim>),
    RangeCheck4_4(ClaimWithEvals<cairo_air::components::range_check_4_4::InteractionClaim>),
    RangeCheck9_9(ClaimWithEvals<cairo_air::components::range_check_9_9::InteractionClaim>),
    RangeCheck7_2_5(ClaimWithEvals<cairo_air::components::range_check_7_2_5::InteractionClaim>),
    RangeCheck3_6_6_3(ClaimWithEvals<cairo_air::components::range_check_3_6_6_3::InteractionClaim>),
    RangeCheck4_4_4_4(ClaimWithEvals<cairo_air::components::range_check_4_4_4_4::InteractionClaim>),
    RangeCheck3_3_3_3_3(ClaimWithEvals<cairo_air::components::range_check_3_3_3_3_3::InteractionClaim>),
    // Verify bitwise xor
    VerifyBitwiseXor4(ClaimWithEvals<cairo_air::components::verify_bitwise_xor_4::InteractionClaim>),
    VerifyBitwiseXor7(ClaimWithEvals<cairo_air::components::verify_bitwise_xor_7::InteractionClaim>),
    VerifyBitwiseXor8(ClaimWithEvals<cairo_air::components::verify_bitwise_xor_8::InteractionClaim>),
    VerifyBitwiseXor9(ClaimWithEvals<cairo_air::components::verify_bitwise_xor_9::InteractionClaim>),
}

impl ComponentInteractionTask {
    fn process(self, common_lookup_elements: &CommonLookupElements) -> ComponentInteractionResult {
        match self {
            ComponentInteractionTask::MemoryAddressToId(gen) => ComponentInteractionResult::MemoryAddressToId(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::MemoryIdToValue(gen) => ComponentInteractionResult::MemoryIdToValue(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::VerifyInstruction(gen) => ComponentInteractionResult::VerifyInstruction(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::BlakeRound(gen) => ComponentInteractionResult::BlakeRound(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::BlakeG(gen) => ComponentInteractionResult::BlakeG(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::BlakeSigma(gen) => ComponentInteractionResult::BlakeSigma(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::TripleXor32(gen) => ComponentInteractionResult::TripleXor32(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::VerifyBitwiseXor12(gen) => ComponentInteractionResult::VerifyBitwiseXor12(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::AddModBuiltin(gen) => ComponentInteractionResult::AddModBuiltin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::BitwiseBuiltin(gen) => ComponentInteractionResult::BitwiseBuiltin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::MulModBuiltin(gen) => ComponentInteractionResult::MulModBuiltin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PedersenBuiltin(gen) => ComponentInteractionResult::PedersenBuiltin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PoseidonBuiltin(gen) => ComponentInteractionResult::PoseidonBuiltin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck96Builtin(gen) => ComponentInteractionResult::RangeCheck96Builtin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck128Builtin(gen) => ComponentInteractionResult::RangeCheck128Builtin(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PedersenAggregator(gen) => ComponentInteractionResult::PedersenAggregator(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PartialEcMul(gen) => ComponentInteractionResult::PartialEcMul(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PedersenPointsTable(gen) => ComponentInteractionResult::PedersenPointsTable(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PoseidonAggregator(gen) => ComponentInteractionResult::PoseidonAggregator(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::Poseidon3PartialRoundsChain(gen) => ComponentInteractionResult::Poseidon3PartialRoundsChain(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PoseidonFullRoundChain(gen) => ComponentInteractionResult::PoseidonFullRoundChain(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::Cube252(gen) => ComponentInteractionResult::Cube252(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::PoseidonRoundKeys(gen) => ComponentInteractionResult::PoseidonRoundKeys(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck252Width27(gen) => ComponentInteractionResult::RangeCheck252Width27(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck6(gen) => ComponentInteractionResult::RangeCheck6(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck8(gen) => ComponentInteractionResult::RangeCheck8(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck11(gen) => ComponentInteractionResult::RangeCheck11(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck12(gen) => ComponentInteractionResult::RangeCheck12(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck18(gen) => ComponentInteractionResult::RangeCheck18(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck20(gen) => ComponentInteractionResult::RangeCheck20(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck4_3(gen) => ComponentInteractionResult::RangeCheck4_3(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck4_4(gen) => ComponentInteractionResult::RangeCheck4_4(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck9_9(gen) => ComponentInteractionResult::RangeCheck9_9(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck7_2_5(gen) => ComponentInteractionResult::RangeCheck7_2_5(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck3_6_6_3(gen) => ComponentInteractionResult::RangeCheck3_6_6_3(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck4_4_4_4(gen) => ComponentInteractionResult::RangeCheck4_4_4_4(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::RangeCheck3_3_3_3_3(gen) => ComponentInteractionResult::RangeCheck3_3_3_3_3(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::VerifyBitwiseXor4(gen) => ComponentInteractionResult::VerifyBitwiseXor4(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::VerifyBitwiseXor7(gen) => ComponentInteractionResult::VerifyBitwiseXor7(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::VerifyBitwiseXor8(gen) => ComponentInteractionResult::VerifyBitwiseXor8(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
            ComponentInteractionTask::VerifyBitwiseXor9(gen) => ComponentInteractionResult::VerifyBitwiseXor9(
                process_single_gen(|b, e| gen.write_interaction_trace(b, e), common_lookup_elements)
            ),
        }
    }
}

impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> CairoInteractionClaim {
        // Decompose all nested generators into their parts
        let opcodes_parts = self.opcodes_interaction_gen.into_parts();
        let blake_parts = self.blake_context_interaction_gen.into_parts();
        let builtins_parts = self.builtins_interaction_gen.into_parts();
        let pedersen_parts = self.pedersen_context_interaction_gen.into_parts();
        let poseidon_parts = self.poseidon_context_interaction_gen.into_parts();
        let range_checks_parts = self.range_checks_interaction_gen.into_parts();

        // Build task list for all non-opcode components
        let mut component_tasks: Vec<ComponentInteractionTask> = Vec::new();

        // Memory (always present)
        component_tasks.push(ComponentInteractionTask::MemoryAddressToId(self.memory_address_to_id_interaction_gen));
        component_tasks.push(ComponentInteractionTask::MemoryIdToValue(self.memory_id_to_value_interaction_gen));

        // Verify instruction (always present)
        component_tasks.push(ComponentInteractionTask::VerifyInstruction(self.verify_instruction_interaction_gen));

        // Blake context (optional)
        if let Some(parts) = blake_parts {
            component_tasks.push(ComponentInteractionTask::BlakeRound(parts.blake_round));
            component_tasks.push(ComponentInteractionTask::BlakeG(parts.blake_g));
            component_tasks.push(ComponentInteractionTask::BlakeSigma(parts.blake_sigma));
            component_tasks.push(ComponentInteractionTask::TripleXor32(parts.triple_xor_32));
            component_tasks.push(ComponentInteractionTask::VerifyBitwiseXor12(parts.verify_bitwise_xor_12));
        }

        // Builtins (optional)
        if let Some(gen) = builtins_parts.add_mod_builtin {
            component_tasks.push(ComponentInteractionTask::AddModBuiltin(gen));
        }
        if let Some(gen) = builtins_parts.bitwise_builtin {
            component_tasks.push(ComponentInteractionTask::BitwiseBuiltin(gen));
        }
        if let Some(gen) = builtins_parts.mul_mod_builtin {
            component_tasks.push(ComponentInteractionTask::MulModBuiltin(gen));
        }
        if let Some(gen) = builtins_parts.pedersen_builtin {
            component_tasks.push(ComponentInteractionTask::PedersenBuiltin(gen));
        }
        if let Some(gen) = builtins_parts.poseidon_builtin {
            component_tasks.push(ComponentInteractionTask::PoseidonBuiltin(gen));
        }
        if let Some(gen) = builtins_parts.range_check_96_builtin {
            component_tasks.push(ComponentInteractionTask::RangeCheck96Builtin(gen));
        }
        if let Some(gen) = builtins_parts.range_check_128_builtin {
            component_tasks.push(ComponentInteractionTask::RangeCheck128Builtin(gen));
        }

        // Pedersen context (optional)
        if let Some(parts) = pedersen_parts {
            component_tasks.push(ComponentInteractionTask::PedersenAggregator(parts.pedersen_aggregator));
            component_tasks.push(ComponentInteractionTask::PartialEcMul(parts.partial_ec_mul));
            component_tasks.push(ComponentInteractionTask::PedersenPointsTable(parts.pedersen_points_table));
        }

        // Poseidon context (optional)
        if let Some(parts) = poseidon_parts {
            component_tasks.push(ComponentInteractionTask::PoseidonAggregator(parts.poseidon_aggregator));
            component_tasks.push(ComponentInteractionTask::Poseidon3PartialRoundsChain(parts.poseidon_3_partial_rounds_chain));
            component_tasks.push(ComponentInteractionTask::PoseidonFullRoundChain(parts.poseidon_full_round_chain));
            component_tasks.push(ComponentInteractionTask::Cube252(parts.cube_252));
            component_tasks.push(ComponentInteractionTask::PoseidonRoundKeys(parts.poseidon_round_keys));
            component_tasks.push(ComponentInteractionTask::RangeCheck252Width27(parts.range_check_252_width_27));
        }

        // Range checks (always present)
        component_tasks.push(ComponentInteractionTask::RangeCheck6(range_checks_parts.rc_6));
        component_tasks.push(ComponentInteractionTask::RangeCheck8(range_checks_parts.rc_8));
        component_tasks.push(ComponentInteractionTask::RangeCheck11(range_checks_parts.rc_11));
        component_tasks.push(ComponentInteractionTask::RangeCheck12(range_checks_parts.rc_12));
        component_tasks.push(ComponentInteractionTask::RangeCheck18(range_checks_parts.rc_18));
        component_tasks.push(ComponentInteractionTask::RangeCheck20(range_checks_parts.rc_20));
        component_tasks.push(ComponentInteractionTask::RangeCheck4_3(range_checks_parts.rc_4_3));
        component_tasks.push(ComponentInteractionTask::RangeCheck4_4(range_checks_parts.rc_4_4));
        component_tasks.push(ComponentInteractionTask::RangeCheck9_9(range_checks_parts.rc_9_9));
        component_tasks.push(ComponentInteractionTask::RangeCheck7_2_5(range_checks_parts.rc_7_2_5));
        component_tasks.push(ComponentInteractionTask::RangeCheck3_6_6_3(range_checks_parts.rc_3_6_6_3));
        component_tasks.push(ComponentInteractionTask::RangeCheck4_4_4_4(range_checks_parts.rc_4_4_4_4));
        component_tasks.push(ComponentInteractionTask::RangeCheck3_3_3_3_3(range_checks_parts.rc_3_3_3_3_3));

        // Verify bitwise xor (always present)
        component_tasks.push(ComponentInteractionTask::VerifyBitwiseXor4(self.verify_bitwise_xor_4_interaction_gen));
        component_tasks.push(ComponentInteractionTask::VerifyBitwiseXor7(self.verify_bitwise_xor_7_interaction_gen));
        component_tasks.push(ComponentInteractionTask::VerifyBitwiseXor8(self.verify_bitwise_xor_8_interaction_gen));
        component_tasks.push(ComponentInteractionTask::VerifyBitwiseXor9(self.verify_bitwise_xor_9_interaction_gen));

        // Process opcodes and components in parallel
        let (opcodes_results, component_results) = rayon::join(
            || process_all_opcodes(opcodes_parts, common_lookup_elements),
            || {
                component_tasks
                    .into_par_iter()
                    .map(|task| task.process(common_lookup_elements))
                    .collect::<Vec<_>>()
            },
        );

        // Extract component results into typed variables
        let mut memory_address_to_id_result = None;
        let mut memory_id_to_value_result = None;
        let mut verify_instruction_result = None;
        let mut blake_round_result = None;
        let mut blake_g_result = None;
        let mut blake_sigma_result = None;
        let mut triple_xor_32_result = None;
        let mut verify_bitwise_xor_12_result = None;
        let mut add_mod_builtin_result = None;
        let mut bitwise_builtin_result = None;
        let mut mul_mod_builtin_result = None;
        let mut pedersen_builtin_result = None;
        let mut poseidon_builtin_result = None;
        let mut range_check_96_builtin_result = None;
        let mut range_check_128_builtin_result = None;
        let mut pedersen_aggregator_result = None;
        let mut partial_ec_mul_result = None;
        let mut pedersen_points_table_result = None;
        let mut poseidon_aggregator_result = None;
        let mut poseidon_3_partial_rounds_chain_result = None;
        let mut poseidon_full_round_chain_result = None;
        let mut cube_252_result = None;
        let mut poseidon_round_keys_result = None;
        let mut range_check_252_width_27_result = None;
        let mut rc_6_result = None;
        let mut rc_8_result = None;
        let mut rc_11_result = None;
        let mut rc_12_result = None;
        let mut rc_18_result = None;
        let mut rc_20_result = None;
        let mut rc_4_3_result = None;
        let mut rc_4_4_result = None;
        let mut rc_9_9_result = None;
        let mut rc_7_2_5_result = None;
        let mut rc_3_6_6_3_result = None;
        let mut rc_4_4_4_4_result = None;
        let mut rc_3_3_3_3_3_result = None;
        let mut xor_4_result = None;
        let mut xor_7_result = None;
        let mut xor_8_result = None;
        let mut xor_9_result = None;

        for result in component_results {
            match result {
                ComponentInteractionResult::MemoryAddressToId(r) => memory_address_to_id_result = Some(r),
                ComponentInteractionResult::MemoryIdToValue(r) => memory_id_to_value_result = Some(r),
                ComponentInteractionResult::VerifyInstruction(r) => verify_instruction_result = Some(r),
                ComponentInteractionResult::BlakeRound(r) => blake_round_result = Some(r),
                ComponentInteractionResult::BlakeG(r) => blake_g_result = Some(r),
                ComponentInteractionResult::BlakeSigma(r) => blake_sigma_result = Some(r),
                ComponentInteractionResult::TripleXor32(r) => triple_xor_32_result = Some(r),
                ComponentInteractionResult::VerifyBitwiseXor12(r) => verify_bitwise_xor_12_result = Some(r),
                ComponentInteractionResult::AddModBuiltin(r) => add_mod_builtin_result = Some(r),
                ComponentInteractionResult::BitwiseBuiltin(r) => bitwise_builtin_result = Some(r),
                ComponentInteractionResult::MulModBuiltin(r) => mul_mod_builtin_result = Some(r),
                ComponentInteractionResult::PedersenBuiltin(r) => pedersen_builtin_result = Some(r),
                ComponentInteractionResult::PoseidonBuiltin(r) => poseidon_builtin_result = Some(r),
                ComponentInteractionResult::RangeCheck96Builtin(r) => range_check_96_builtin_result = Some(r),
                ComponentInteractionResult::RangeCheck128Builtin(r) => range_check_128_builtin_result = Some(r),
                ComponentInteractionResult::PedersenAggregator(r) => pedersen_aggregator_result = Some(r),
                ComponentInteractionResult::PartialEcMul(r) => partial_ec_mul_result = Some(r),
                ComponentInteractionResult::PedersenPointsTable(r) => pedersen_points_table_result = Some(r),
                ComponentInteractionResult::PoseidonAggregator(r) => poseidon_aggregator_result = Some(r),
                ComponentInteractionResult::Poseidon3PartialRoundsChain(r) => poseidon_3_partial_rounds_chain_result = Some(r),
                ComponentInteractionResult::PoseidonFullRoundChain(r) => poseidon_full_round_chain_result = Some(r),
                ComponentInteractionResult::Cube252(r) => cube_252_result = Some(r),
                ComponentInteractionResult::PoseidonRoundKeys(r) => poseidon_round_keys_result = Some(r),
                ComponentInteractionResult::RangeCheck252Width27(r) => range_check_252_width_27_result = Some(r),
                ComponentInteractionResult::RangeCheck6(r) => rc_6_result = Some(r),
                ComponentInteractionResult::RangeCheck8(r) => rc_8_result = Some(r),
                ComponentInteractionResult::RangeCheck11(r) => rc_11_result = Some(r),
                ComponentInteractionResult::RangeCheck12(r) => rc_12_result = Some(r),
                ComponentInteractionResult::RangeCheck18(r) => rc_18_result = Some(r),
                ComponentInteractionResult::RangeCheck20(r) => rc_20_result = Some(r),
                ComponentInteractionResult::RangeCheck4_3(r) => rc_4_3_result = Some(r),
                ComponentInteractionResult::RangeCheck4_4(r) => rc_4_4_result = Some(r),
                ComponentInteractionResult::RangeCheck9_9(r) => rc_9_9_result = Some(r),
                ComponentInteractionResult::RangeCheck7_2_5(r) => rc_7_2_5_result = Some(r),
                ComponentInteractionResult::RangeCheck3_6_6_3(r) => rc_3_6_6_3_result = Some(r),
                ComponentInteractionResult::RangeCheck4_4_4_4(r) => rc_4_4_4_4_result = Some(r),
                ComponentInteractionResult::RangeCheck3_3_3_3_3(r) => rc_3_3_3_3_3_result = Some(r),
                ComponentInteractionResult::VerifyBitwiseXor4(r) => xor_4_result = Some(r),
                ComponentInteractionResult::VerifyBitwiseXor7(r) => xor_7_result = Some(r),
                ComponentInteractionResult::VerifyBitwiseXor8(r) => xor_8_result = Some(r),
                ComponentInteractionResult::VerifyBitwiseXor9(r) => xor_9_result = Some(r),
            }
        }

        // Unwrap required results
        let memory_address_to_id_result = memory_address_to_id_result.unwrap();
        let memory_id_to_value_result = memory_id_to_value_result.unwrap();
        let verify_instruction_result = verify_instruction_result.unwrap();
        let rc_6_result = rc_6_result.unwrap();
        let rc_8_result = rc_8_result.unwrap();
        let rc_11_result = rc_11_result.unwrap();
        let rc_12_result = rc_12_result.unwrap();
        let rc_18_result = rc_18_result.unwrap();
        let rc_20_result = rc_20_result.unwrap();
        let rc_4_3_result = rc_4_3_result.unwrap();
        let rc_4_4_result = rc_4_4_result.unwrap();
        let rc_9_9_result = rc_9_9_result.unwrap();
        let rc_7_2_5_result = rc_7_2_5_result.unwrap();
        let rc_3_6_6_3_result = rc_3_6_6_3_result.unwrap();
        let rc_4_4_4_4_result = rc_4_4_4_4_result.unwrap();
        let rc_3_3_3_3_3_result = rc_3_3_3_3_3_result.unwrap();
        let xor_4_result = xor_4_result.unwrap();
        let xor_7_result = xor_7_result.unwrap();
        let xor_8_result = xor_8_result.unwrap();
        let xor_9_result = xor_9_result.unwrap();

        // === WRITE EVALS TO TREE BUILDER IN DETERMINISTIC ORDER ===
        // Opcodes
        for builder in opcodes_results.add.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.add_small.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.add_ap.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.assert_eq.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.assert_eq_imm.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.assert_eq_double_deref.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.blake.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.call.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.call_rel_imm.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.generic.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.jnz.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.jnz_taken.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.jump.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.jump_double_deref.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.jump_rel.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.jump_rel_imm.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.mul.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.mul_small.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.qm31.evals {
            builder.write_to(tree_builder);
        }
        for builder in opcodes_results.ret.evals {
            builder.write_to(tree_builder);
        }

        // Verify instruction
        verify_instruction_result.evals.write_to(tree_builder);

        // Blake context
        let blake_round_claim = blake_round_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let blake_g_claim = blake_g_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let blake_sigma_claim = blake_sigma_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let triple_xor_32_claim = triple_xor_32_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let verify_bitwise_xor_12_claim = verify_bitwise_xor_12_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Builtins
        let add_mod_builtin_claim = add_mod_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let bitwise_builtin_claim = bitwise_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let mul_mod_builtin_claim = mul_mod_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let pedersen_builtin_claim = pedersen_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let poseidon_builtin_claim = poseidon_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let range_check_96_builtin_claim = range_check_96_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let range_check_128_builtin_claim = range_check_128_builtin_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Pedersen context
        let pedersen_aggregator_claim = pedersen_aggregator_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let partial_ec_mul_claim = partial_ec_mul_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let pedersen_points_table_claim = pedersen_points_table_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Poseidon context
        let poseidon_aggregator_claim = poseidon_aggregator_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let poseidon_3_partial_rounds_chain_claim =
            poseidon_3_partial_rounds_chain_result.map(|r| {
                r.evals.write_to(tree_builder);
                r.claim
            });
        let poseidon_full_round_chain_claim = poseidon_full_round_chain_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let cube_252_claim = cube_252_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let poseidon_round_keys_claim = poseidon_round_keys_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let range_check_252_width_27_claim = range_check_252_width_27_result.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Memory
        memory_address_to_id_result.evals.write_to(tree_builder);
        memory_id_to_value_result.evals.write_to(tree_builder);

        // Range checks
        rc_6_result.evals.write_to(tree_builder);
        rc_8_result.evals.write_to(tree_builder);
        rc_11_result.evals.write_to(tree_builder);
        rc_12_result.evals.write_to(tree_builder);
        rc_18_result.evals.write_to(tree_builder);
        rc_20_result.evals.write_to(tree_builder);
        rc_4_3_result.evals.write_to(tree_builder);
        rc_4_4_result.evals.write_to(tree_builder);
        rc_9_9_result.evals.write_to(tree_builder);
        rc_7_2_5_result.evals.write_to(tree_builder);
        rc_3_6_6_3_result.evals.write_to(tree_builder);
        rc_4_4_4_4_result.evals.write_to(tree_builder);
        rc_3_3_3_3_3_result.evals.write_to(tree_builder);

        // Verify bitwise xor
        xor_4_result.evals.write_to(tree_builder);
        xor_7_result.evals.write_to(tree_builder);
        xor_8_result.evals.write_to(tree_builder);
        xor_9_result.evals.write_to(tree_builder);

        // === CONSTRUCT FINAL CLAIMS ===
        let opcodes_interaction_claims = OpcodeInteractionClaim {
            add: opcodes_results.add.claims,
            add_small: opcodes_results.add_small.claims,
            add_ap: opcodes_results.add_ap.claims,
            assert_eq: opcodes_results.assert_eq.claims,
            assert_eq_imm: opcodes_results.assert_eq_imm.claims,
            assert_eq_double_deref: opcodes_results.assert_eq_double_deref.claims,
            blake: opcodes_results.blake.claims,
            call: opcodes_results.call.claims,
            call_rel_imm: opcodes_results.call_rel_imm.claims,
            generic: opcodes_results.generic.claims,
            jnz: opcodes_results.jnz.claims,
            jnz_taken: opcodes_results.jnz_taken.claims,
            jump: opcodes_results.jump.claims,
            jump_double_deref: opcodes_results.jump_double_deref.claims,
            jump_rel: opcodes_results.jump_rel.claims,
            jump_rel_imm: opcodes_results.jump_rel_imm.claims,
            mul: opcodes_results.mul.claims,
            mul_small: opcodes_results.mul_small.claims,
            qm31: opcodes_results.qm31.claims,
            ret: opcodes_results.ret.claims,
        };

        let blake_context_interaction_claim = blake_air::BlakeContextInteractionClaim {
            claim: blake_round_claim.map(|blake_round| blake_air::InteractionClaim {
                blake_round,
                blake_g: blake_g_claim.unwrap(),
                blake_sigma: blake_sigma_claim.unwrap(),
                triple_xor_32: triple_xor_32_claim.unwrap(),
                verify_bitwise_xor_12: verify_bitwise_xor_12_claim.unwrap(),
            }),
        };

        let builtins_interaction_claims = BuiltinsInteractionClaim {
            add_mod_builtin: add_mod_builtin_claim,
            bitwise_builtin: bitwise_builtin_claim,
            mul_mod_builtin: mul_mod_builtin_claim,
            pedersen_builtin: pedersen_builtin_claim,
            poseidon_builtin: poseidon_builtin_claim,
            range_check_96_builtin: range_check_96_builtin_claim,
            range_check_128_builtin: range_check_128_builtin_claim,
        };

        let pedersen_context_interaction_claim = pedersen_air::PedersenContextInteractionClaim {
            claim: pedersen_aggregator_claim.map(|pedersen_aggregator| {
                pedersen_air::InteractionClaim {
                    pedersen_aggregator,
                    partial_ec_mul: partial_ec_mul_claim.unwrap(),
                    pedersen_points_table: pedersen_points_table_claim.unwrap(),
                }
            }),
        };

        let poseidon_context_interaction_claim = poseidon_air::PoseidonContextInteractionClaim {
            claim: poseidon_aggregator_claim.map(|poseidon_aggregator| {
                poseidon_air::InteractionClaim {
                    poseidon_aggregator,
                    poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim.unwrap(),
                    poseidon_full_round_chain: poseidon_full_round_chain_claim.unwrap(),
                    cube_252: cube_252_claim.unwrap(),
                    poseidon_round_keys: poseidon_round_keys_claim.unwrap(),
                    range_check_252_width_27: range_check_252_width_27_claim.unwrap(),
                }
            }),
        };

        let range_checks_interaction_claim = RangeChecksInteractionClaim {
            rc_6: rc_6_result.claim,
            rc_8: rc_8_result.claim,
            rc_11: rc_11_result.claim,
            rc_12: rc_12_result.claim,
            rc_18: rc_18_result.claim,
            rc_20: rc_20_result.claim,
            rc_4_3: rc_4_3_result.claim,
            rc_4_4: rc_4_4_result.claim,
            rc_9_9: rc_9_9_result.claim,
            rc_7_2_5: rc_7_2_5_result.claim,
            rc_3_6_6_3: rc_3_6_6_3_result.claim,
            rc_4_4_4_4: rc_4_4_4_4_result.claim,
            rc_3_3_3_3_3: rc_3_3_3_3_3_result.claim,
        };

        CairoInteractionClaim {
            opcodes: opcodes_interaction_claims,
            verify_instruction: verify_instruction_result.claim,
            blake_context: blake_context_interaction_claim,
            builtins: builtins_interaction_claims,
            pedersen_context: pedersen_context_interaction_claim,
            poseidon_context: poseidon_context_interaction_claim,
            memory_address_to_id: memory_address_to_id_result.claim,
            memory_id_to_value: memory_id_to_value_result.claim,
            range_checks: range_checks_interaction_claim,
            verify_bitwise_xor_4: xor_4_result.claim,
            verify_bitwise_xor_7: xor_7_result.claim,
            verify_bitwise_xor_8: xor_8_result.claim,
            verify_bitwise_xor_9: xor_9_result.claim,
        }
    }
}

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
use rayon::scope;
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

struct MemoryResults {
    address_to_id: ClaimWithEvals<cairo_air::components::memory_address_to_id::InteractionClaim>,
    id_to_value: ClaimWithEvals<cairo_air::components::memory_id_to_big::InteractionClaim>,
}

struct PedersenContextResults {
    aggregator: Option<
        ClaimWithEvals<cairo_air::components::pedersen_aggregator_window_bits_18::InteractionClaim>,
    >,
    partial_ec_mul: Option<
        ClaimWithEvals<cairo_air::components::partial_ec_mul_window_bits_18::InteractionClaim>,
    >,
    points_table: Option<
        ClaimWithEvals<
            cairo_air::components::pedersen_points_table_window_bits_18::InteractionClaim,
        >,
    >,
}

struct PoseidonContextResults {
    aggregator:
        Option<ClaimWithEvals<cairo_air::components::poseidon_aggregator::InteractionClaim>>,
    partial_rounds_chain: Option<
        ClaimWithEvals<cairo_air::components::poseidon_3_partial_rounds_chain::InteractionClaim>,
    >,
    full_round_chain:
        Option<ClaimWithEvals<cairo_air::components::poseidon_full_round_chain::InteractionClaim>>,
    cube_252: Option<ClaimWithEvals<cairo_air::components::cube_252::InteractionClaim>>,
    round_keys:
        Option<ClaimWithEvals<cairo_air::components::poseidon_round_keys::InteractionClaim>>,
    range_check_252_width_27:
        Option<ClaimWithEvals<cairo_air::components::range_check_252_width_27::InteractionClaim>>,
}

struct BlakeContextResults {
    blake_round: Option<ClaimWithEvals<cairo_air::components::blake_round::InteractionClaim>>,
    blake_g: Option<ClaimWithEvals<cairo_air::components::blake_g::InteractionClaim>>,
    blake_sigma: Option<ClaimWithEvals<cairo_air::components::blake_round_sigma::InteractionClaim>>,
    triple_xor_32: Option<ClaimWithEvals<cairo_air::components::triple_xor_32::InteractionClaim>>,
    verify_bitwise_xor_12:
        Option<ClaimWithEvals<cairo_air::components::verify_bitwise_xor_12::InteractionClaim>>,
}

struct BuiltinsResults {
    add_mod: Option<ClaimWithEvals<cairo_air::components::add_mod_builtin::InteractionClaim>>,
    bitwise: Option<ClaimWithEvals<cairo_air::components::bitwise_builtin::InteractionClaim>>,
    mul_mod: Option<ClaimWithEvals<cairo_air::components::mul_mod_builtin::InteractionClaim>>,
    pedersen: Option<ClaimWithEvals<cairo_air::components::pedersen_builtin::InteractionClaim>>,
    poseidon: Option<ClaimWithEvals<cairo_air::components::poseidon_builtin::InteractionClaim>>,
    range_check_96:
        Option<ClaimWithEvals<cairo_air::components::range_check96_builtin::InteractionClaim>>,
    range_check_128:
        Option<ClaimWithEvals<cairo_air::components::range_check_builtin::InteractionClaim>>,
}

struct RangeChecksResults {
    rc_6: ClaimWithEvals<cairo_air::components::range_check_6::InteractionClaim>,
    rc_8: ClaimWithEvals<cairo_air::components::range_check_8::InteractionClaim>,
    rc_11: ClaimWithEvals<cairo_air::components::range_check_11::InteractionClaim>,
    rc_12: ClaimWithEvals<cairo_air::components::range_check_12::InteractionClaim>,
    rc_18: ClaimWithEvals<cairo_air::components::range_check_18::InteractionClaim>,
    rc_20: ClaimWithEvals<cairo_air::components::range_check_20::InteractionClaim>,
    rc_4_3: ClaimWithEvals<cairo_air::components::range_check_4_3::InteractionClaim>,
    rc_4_4: ClaimWithEvals<cairo_air::components::range_check_4_4::InteractionClaim>,
    rc_9_9: ClaimWithEvals<cairo_air::components::range_check_9_9::InteractionClaim>,
    rc_7_2_5: ClaimWithEvals<cairo_air::components::range_check_7_2_5::InteractionClaim>,
    rc_3_6_6_3: ClaimWithEvals<cairo_air::components::range_check_3_6_6_3::InteractionClaim>,
    rc_4_4_4_4: ClaimWithEvals<cairo_air::components::range_check_4_4_4_4::InteractionClaim>,
    rc_3_3_3_3_3: ClaimWithEvals<cairo_air::components::range_check_3_3_3_3_3::InteractionClaim>,
}

struct VerifyXorResults {
    xor_4: ClaimWithEvals<cairo_air::components::verify_bitwise_xor_4::InteractionClaim>,
    xor_7: ClaimWithEvals<cairo_air::components::verify_bitwise_xor_7::InteractionClaim>,
    xor_8: ClaimWithEvals<cairo_air::components::verify_bitwise_xor_8::InteractionClaim>,
    xor_9: ClaimWithEvals<cairo_air::components::verify_bitwise_xor_9::InteractionClaim>,
}

// === Helper functions for parallel processing ===

/// Small opcodes results struct for grouping tiny opcodes together.
struct SmallOpcodesResults {
    call: InteractionTraceResult<cairo_air::components::call_opcode_abs::InteractionClaim>,
    generic: InteractionTraceResult<cairo_air::components::generic_opcode::InteractionClaim>,
    jump: InteractionTraceResult<cairo_air::components::jump_opcode_abs::InteractionClaim>,
    jump_double_deref:
        InteractionTraceResult<cairo_air::components::jump_opcode_double_deref::InteractionClaim>,
    jump_rel: InteractionTraceResult<cairo_air::components::jump_opcode_rel::InteractionClaim>,
    qm31: InteractionTraceResult<cairo_air::components::qm_31_add_mul_opcode::InteractionClaim>,
}

/// Process all opcodes using rayon::spawn for parallelism at the opcode-type level.
/// Small opcodes (<5ms) are grouped into a single spawn with 1-thread pool.
fn process_all_opcodes(
    parts: super::opcodes::OpcodesInteractionParts,
    common_lookup_elements: &CommonLookupElements,
) -> OpcodesResults {
    // 1-thread pool for small opcodes to run them sequentially
    let small_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();

    // Declare result variables for heavy opcodes
    let mut add_result = None;
    let mut add_small_result = None;
    let mut add_ap_result = None;
    let mut assert_eq_result = None;
    let mut assert_eq_imm_result = None;
    let mut assert_eq_double_deref_result = None;
    let mut blake_result = None;
    let mut call_rel_imm_result = None;
    let mut jnz_result = None;
    let mut jnz_taken_result = None;
    let mut jump_rel_imm_result = None;
    let mut mul_result = None;
    let mut mul_small_result = None;
    let mut ret_result = None;
    // Small opcodes grouped together
    let mut small_opcodes_result = None;

    // Create mutable references for heavy opcodes
    let add_ref = &mut add_result;
    let add_small_ref = &mut add_small_result;
    let add_ap_ref = &mut add_ap_result;
    let assert_eq_ref = &mut assert_eq_result;
    let assert_eq_imm_ref = &mut assert_eq_imm_result;
    let assert_eq_double_deref_ref = &mut assert_eq_double_deref_result;
    let blake_ref = &mut blake_result;
    let call_rel_imm_ref = &mut call_rel_imm_result;
    let jnz_ref = &mut jnz_result;
    let jnz_taken_ref = &mut jnz_taken_result;
    let jump_rel_imm_ref = &mut jump_rel_imm_result;
    let mul_ref = &mut mul_result;
    let mul_small_ref = &mut mul_small_result;
    let ret_ref = &mut ret_result;
    let small_opcodes_ref = &mut small_opcodes_result;

    scope(|s| {
        // Heavy opcodes (>50ms) - spawn individually
        s.spawn(|_| {
            *add_ref = Some(process_interaction_gens(parts.add, common_lookup_elements));
        });
        s.spawn(|_| {
            *add_small_ref = Some(process_interaction_gens(
                parts.add_small,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *add_ap_ref = Some(process_interaction_gens(
                parts.add_ap,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *assert_eq_ref = Some(process_interaction_gens(
                parts.assert_eq,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *assert_eq_imm_ref = Some(process_interaction_gens(
                parts.assert_eq_imm,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *assert_eq_double_deref_ref = Some(process_interaction_gens(
                parts.assert_eq_double_deref,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *blake_ref = Some(process_interaction_gens(
                parts.blake,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *call_rel_imm_ref = Some(process_interaction_gens(
                parts.call_rel_imm,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *jnz_ref = Some(process_interaction_gens(parts.jnz, common_lookup_elements));
        });
        s.spawn(|_| {
            *jnz_taken_ref = Some(process_interaction_gens(
                parts.jnz_taken,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *jump_rel_imm_ref = Some(process_interaction_gens(
                parts.jump_rel_imm,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *mul_ref = Some(process_interaction_gens(parts.mul, common_lookup_elements));
        });
        s.spawn(|_| {
            *mul_small_ref = Some(process_interaction_gens(
                parts.mul_small,
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *ret_ref = Some(process_interaction_gens(parts.ret, common_lookup_elements));
        });

        // Small opcodes (<5ms) - group together in 1-thread pool
        s.spawn(|_| {
            *small_opcodes_ref = Some(small_pool.install(|| {
                let call = process_interaction_gens(parts.call, common_lookup_elements);
                let generic =
                    process_interaction_gens(parts.generic_opcode, common_lookup_elements);
                let jump = process_interaction_gens(parts.jump, common_lookup_elements);
                let jump_double_deref =
                    process_interaction_gens(parts.jump_double_deref, common_lookup_elements);
                let jump_rel = process_interaction_gens(parts.jump_rel, common_lookup_elements);
                let qm31 = process_interaction_gens(parts.qm31, common_lookup_elements);
                SmallOpcodesResults {
                    call,
                    generic,
                    jump,
                    jump_double_deref,
                    jump_rel,
                    qm31,
                }
            }));
        });
    });

    let small = small_opcodes_result.unwrap();
    OpcodesResults {
        add: add_result.unwrap(),
        add_small: add_small_result.unwrap(),
        add_ap: add_ap_result.unwrap(),
        assert_eq: assert_eq_result.unwrap(),
        assert_eq_imm: assert_eq_imm_result.unwrap(),
        assert_eq_double_deref: assert_eq_double_deref_result.unwrap(),
        blake: blake_result.unwrap(),
        call: small.call,
        call_rel_imm: call_rel_imm_result.unwrap(),
        generic: small.generic,
        jnz: jnz_result.unwrap(),
        jnz_taken: jnz_taken_result.unwrap(),
        jump: small.jump,
        jump_double_deref: small.jump_double_deref,
        jump_rel: small.jump_rel,
        jump_rel_imm: jump_rel_imm_result.unwrap(),
        mul: mul_result.unwrap(),
        mul_small: mul_small_result.unwrap(),
        qm31: small.qm31,
        ret: ret_result.unwrap(),
    }
}

/// Process memory components in parallel using spawn.
fn process_memory(
    address_to_id_gen: memory_address_to_id::InteractionClaimGenerator,
    id_to_value_gen: memory_id_to_big::InteractionClaimGenerator,
    common_lookup_elements: &CommonLookupElements,
) -> MemoryResults {
    let mut address_to_id_result = None;
    let mut id_to_value_result = None;

    let address_to_id_ref = &mut address_to_id_result;
    let id_to_value_ref = &mut id_to_value_result;

    scope(|s| {
        s.spawn(|_| {
            *address_to_id_ref = Some(process_single_gen(
                |builder, elems| address_to_id_gen.write_interaction_trace(builder, elems),
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *id_to_value_ref = Some(process_single_gen(
                |builder, elems| id_to_value_gen.write_interaction_trace(builder, elems),
                common_lookup_elements,
            ));
        });
    });

    MemoryResults {
        address_to_id: address_to_id_result.unwrap(),
        id_to_value: id_to_value_result.unwrap(),
    }
}

/// Process pedersen context components in parallel using spawn.
/// Heavy components (partial_ec_mul 2856ms, points_table 831ms) spawn individually.
/// Small component (aggregator 41ms) runs in 1-thread pool to avoid thread stealing.
fn process_pedersen_context(
    parts: Option<super::components::pedersen::PedersenContextInteractionParts>,
    common_lookup_elements: &CommonLookupElements,
) -> PedersenContextResults {
    match parts {
        Some(p) => {
            // 1-thread pool for small component to avoid thread stealing
            let small_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .build()
                .unwrap();

            // Heavy components
            let mut partial_ec_mul_result = None;
            let mut points_table_result = None;
            // Small component
            let mut aggregator_result = None;

            let partial_ec_mul_ref = &mut partial_ec_mul_result;
            let points_table_ref = &mut points_table_result;
            let aggregator_ref = &mut aggregator_result;

            scope(|s| {
                // Heavy components - spawn individually
                s.spawn(|_| {
                    *partial_ec_mul_ref = Some(Some(process_single_gen(
                        |builder, elems| p.partial_ec_mul.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )));
                });
                s.spawn(|_| {
                    *points_table_ref = Some(Some(process_single_gen(
                        |builder, elems| {
                            p.pedersen_points_table
                                .write_interaction_trace(builder, elems)
                        },
                        common_lookup_elements,
                    )));
                });

                // Small component (<50ms) - run in 1-thread pool
                s.spawn(|_| {
                    *aggregator_ref = Some(small_pool.install(|| {
                        Some(process_single_gen(
                            |builder, elems| {
                                p.pedersen_aggregator
                                    .write_interaction_trace(builder, elems)
                            },
                            common_lookup_elements,
                        ))
                    }));
                });
            });

            PedersenContextResults {
                aggregator: aggregator_result.unwrap(),
                partial_ec_mul: partial_ec_mul_result.unwrap(),
                points_table: points_table_result.unwrap(),
            }
        }
        None => PedersenContextResults {
            aggregator: None,
            partial_ec_mul: None,
            points_table: None,
        },
    }
}

/// Small poseidon results struct for grouping tiny components together.
struct SmallPoseidonResults {
    aggregator: ClaimWithEvals<cairo_air::components::poseidon_aggregator::InteractionClaim>,
    full_round_chain:
        ClaimWithEvals<cairo_air::components::poseidon_full_round_chain::InteractionClaim>,
    round_keys: ClaimWithEvals<cairo_air::components::poseidon_round_keys::InteractionClaim>,
}

/// Process poseidon context components in parallel using spawn.
/// Heavy components (cube_252 1161ms, partial_rounds 126ms, range_check_252 161ms) spawn individually.
/// Small components (aggregator 10ms, full_round_chain 25ms, round_keys 0.05ms) grouped in 1-thread pool.
fn process_poseidon_context(
    parts: Option<super::components::poseidon::PoseidonContextInteractionParts>,
    common_lookup_elements: &CommonLookupElements,
) -> PoseidonContextResults {
    match parts {
        Some(p) => {
            // 1-thread pool for small components to run them sequentially
            let small_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .build()
                .unwrap();

            // Heavy components
            let mut partial_rounds_result = None;
            let mut cube_252_result = None;
            let mut range_check_252_result = None;
            // Small components grouped together
            let mut small_poseidon_result = None;

            let partial_rounds_ref = &mut partial_rounds_result;
            let cube_252_ref = &mut cube_252_result;
            let range_check_252_ref = &mut range_check_252_result;
            let small_poseidon_ref = &mut small_poseidon_result;

            scope(|s| {
                // Heavy components - spawn individually
                s.spawn(|_| {
                    *cube_252_ref = Some(Some(process_single_gen(
                        |builder, elems| p.cube_252.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )));
                });
                s.spawn(|_| {
                    *range_check_252_ref = Some(Some(process_single_gen(
                        |builder, elems| {
                            p.range_check_252_width_27
                                .write_interaction_trace(builder, elems)
                        },
                        common_lookup_elements,
                    )));
                });
                s.spawn(|_| {
                    *partial_rounds_ref = Some(Some(process_single_gen(
                        |builder, elems| {
                            p.poseidon_3_partial_rounds_chain
                                .write_interaction_trace(builder, elems)
                        },
                        common_lookup_elements,
                    )));
                });

                // Small components (<50ms) - group together in 1-thread pool
                s.spawn(|_| {
                    *small_poseidon_ref = Some(small_pool.install(|| {
                        let aggregator = process_single_gen(
                            |builder, elems| {
                                p.poseidon_aggregator
                                    .write_interaction_trace(builder, elems)
                            },
                            common_lookup_elements,
                        );
                        let full_round_chain = process_single_gen(
                            |builder, elems| {
                                p.poseidon_full_round_chain
                                    .write_interaction_trace(builder, elems)
                            },
                            common_lookup_elements,
                        );
                        let round_keys = process_single_gen(
                            |builder, elems| {
                                p.poseidon_round_keys
                                    .write_interaction_trace(builder, elems)
                            },
                            common_lookup_elements,
                        );
                        SmallPoseidonResults {
                            aggregator,
                            full_round_chain,
                            round_keys,
                        }
                    }));
                });
            });

            let small = small_poseidon_result.unwrap();
            PoseidonContextResults {
                aggregator: Some(small.aggregator),
                partial_rounds_chain: partial_rounds_result.unwrap(),
                full_round_chain: Some(small.full_round_chain),
                cube_252: cube_252_result.unwrap(),
                round_keys: Some(small.round_keys),
                range_check_252_width_27: range_check_252_result.unwrap(),
            }
        }
        None => PoseidonContextResults {
            aggregator: None,
            partial_rounds_chain: None,
            full_round_chain: None,
            cube_252: None,
            round_keys: None,
            range_check_252_width_27: None,
        },
    }
}

/// Small blake results struct for grouping tiny components together.
struct SmallBlakeResults {
    triple_xor_32: ClaimWithEvals<cairo_air::components::triple_xor_32::InteractionClaim>,
    blake_sigma: ClaimWithEvals<cairo_air::components::blake_round_sigma::InteractionClaim>,
}

/// Process blake context components in parallel using spawn.
/// Heavy components (blake_g 833ms, blake_round 401ms, verify_xor_12 259ms) spawn individually.
/// Small components (triple_xor_32 21ms, sigma 0.1ms) grouped in 1-thread pool.
fn process_blake_context(
    parts: Option<super::blake_context::BlakeContextInteractionParts>,
    common_lookup_elements: &CommonLookupElements,
) -> BlakeContextResults {
    match parts {
        Some(p) => {
            // 1-thread pool for small components to run them sequentially
            let small_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .build()
                .unwrap();

            // Heavy components
            let mut blake_round_result = None;
            let mut blake_g_result = None;
            let mut verify_xor_12_result = None;
            // Small components grouped together
            let mut small_blake_result = None;

            let blake_round_ref = &mut blake_round_result;
            let blake_g_ref = &mut blake_g_result;
            let verify_xor_12_ref = &mut verify_xor_12_result;
            let small_blake_ref = &mut small_blake_result;

            scope(|s| {
                // Heavy components - spawn individually
                s.spawn(|_| {
                    *blake_g_ref = Some(Some(process_single_gen(
                        |builder, elems| p.blake_g.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )));
                });
                s.spawn(|_| {
                    *blake_round_ref = Some(Some(process_single_gen(
                        |builder, elems| p.blake_round.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )));
                });
                s.spawn(|_| {
                    *verify_xor_12_ref = Some(Some(process_single_gen(
                        |builder, elems| {
                            p.verify_bitwise_xor_12
                                .write_interaction_trace(builder, elems)
                        },
                        common_lookup_elements,
                    )));
                });

                // Small components (<50ms) - group together in 1-thread pool
                s.spawn(|_| {
                    *small_blake_ref = Some(small_pool.install(|| {
                        let triple_xor_32 = process_single_gen(
                            |builder, elems| p.triple_xor_32.write_interaction_trace(builder, elems),
                            common_lookup_elements,
                        );
                        let blake_sigma = process_single_gen(
                            |builder, elems| p.blake_sigma.write_interaction_trace(builder, elems),
                            common_lookup_elements,
                        );
                        SmallBlakeResults {
                            triple_xor_32,
                            blake_sigma,
                        }
                    }));
                });
            });

            let small = small_blake_result.unwrap();
            BlakeContextResults {
                blake_round: blake_round_result.unwrap(),
                blake_g: blake_g_result.unwrap(),
                blake_sigma: Some(small.blake_sigma),
                triple_xor_32: Some(small.triple_xor_32),
                verify_bitwise_xor_12: verify_xor_12_result.unwrap(),
            }
        }
        None => BlakeContextResults {
            blake_round: None,
            blake_g: None,
            blake_sigma: None,
            triple_xor_32: None,
            verify_bitwise_xor_12: None,
        },
    }
}

/// Small builtins results struct for grouping tiny components together.
struct SmallBuiltinsResults {
    add_mod: Option<ClaimWithEvals<cairo_air::components::add_mod_builtin::InteractionClaim>>,
    bitwise: Option<ClaimWithEvals<cairo_air::components::bitwise_builtin::InteractionClaim>>,
    mul_mod: Option<ClaimWithEvals<cairo_air::components::mul_mod_builtin::InteractionClaim>>,
    pedersen: Option<ClaimWithEvals<cairo_air::components::pedersen_builtin::InteractionClaim>>,
    poseidon: Option<ClaimWithEvals<cairo_air::components::poseidon_builtin::InteractionClaim>>,
    range_check_96:
        Option<ClaimWithEvals<cairo_air::components::range_check96_builtin::InteractionClaim>>,
}

/// Process builtins using spawn. range_check_128 (432ms) is heaviest and spawns individually.
/// Small builtins (pedersen 16ms, poseidon 1.8ms, etc.) grouped in 1-thread pool.
fn process_builtins(
    parts: super::builtins::BuiltinsInteractionParts,
    common_lookup_elements: &CommonLookupElements,
) -> BuiltinsResults {
    // 1-thread pool for small components to run them sequentially
    let small_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();

    // Heavy component
    let mut range_check_128_result: Option<
        Option<ClaimWithEvals<cairo_air::components::range_check_builtin::InteractionClaim>>,
    > = None;
    // Small components grouped together
    let mut small_builtins_result = None;

    let range_check_128_ref = &mut range_check_128_result;
    let small_builtins_ref = &mut small_builtins_result;

    scope(|s| {
        // Heavy component - spawn individually
        s.spawn(|_| {
            *range_check_128_ref = Some(parts.range_check_128_builtin.map(|gen| {
                process_single_gen(
                    |builder, elems| gen.write_interaction_trace(builder, elems),
                    common_lookup_elements,
                )
            }));
        });

        // Small components (<50ms) - group together in 1-thread pool
        s.spawn(|_| {
            *small_builtins_ref = Some(small_pool.install(|| {
                let add_mod = parts.add_mod_builtin.map(|gen| {
                    process_single_gen(
                        |builder, elems| gen.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )
                });
                let bitwise = parts.bitwise_builtin.map(|gen| {
                    process_single_gen(
                        |builder, elems| gen.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )
                });
                let mul_mod = parts.mul_mod_builtin.map(|gen| {
                    process_single_gen(
                        |builder, elems| gen.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )
                });
                let pedersen = parts.pedersen_builtin.map(|gen| {
                    process_single_gen(
                        |builder, elems| gen.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )
                });
                let poseidon = parts.poseidon_builtin.map(|gen| {
                    process_single_gen(
                        |builder, elems| gen.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )
                });
                let range_check_96 = parts.range_check_96_builtin.map(|gen| {
                    process_single_gen(
                        |builder, elems| gen.write_interaction_trace(builder, elems),
                        common_lookup_elements,
                    )
                });
                SmallBuiltinsResults {
                    add_mod,
                    bitwise,
                    mul_mod,
                    pedersen,
                    poseidon,
                    range_check_96,
                }
            }));
        });
    });

    let small = small_builtins_result.unwrap();
    BuiltinsResults {
        add_mod: small.add_mod,
        bitwise: small.bitwise,
        mul_mod: small.mul_mod,
        pedersen: small.pedersen,
        poseidon: small.poseidon,
        range_check_96: small.range_check_96,
        range_check_128: range_check_128_result.unwrap(),
    }
}

/// Small range checks results struct for grouping tiny components together.
struct SmallRangeChecksResults {
    rc_6: ClaimWithEvals<cairo_air::components::range_check_6::InteractionClaim>,
    rc_8: ClaimWithEvals<cairo_air::components::range_check_8::InteractionClaim>,
    rc_11: ClaimWithEvals<cairo_air::components::range_check_11::InteractionClaim>,
    rc_12: ClaimWithEvals<cairo_air::components::range_check_12::InteractionClaim>,
    rc_18: ClaimWithEvals<cairo_air::components::range_check_18::InteractionClaim>,
    rc_4_3: ClaimWithEvals<cairo_air::components::range_check_4_3::InteractionClaim>,
    rc_4_4: ClaimWithEvals<cairo_air::components::range_check_4_4::InteractionClaim>,
    rc_7_2_5: ClaimWithEvals<cairo_air::components::range_check_7_2_5::InteractionClaim>,
    rc_3_6_6_3: ClaimWithEvals<cairo_air::components::range_check_3_6_6_3::InteractionClaim>,
    rc_4_4_4_4: ClaimWithEvals<cairo_air::components::range_check_4_4_4_4::InteractionClaim>,
    rc_3_3_3_3_3: ClaimWithEvals<cairo_air::components::range_check_3_3_3_3_3::InteractionClaim>,
}

/// Process range checks. rc_20 (92ms) and rc_9_9 (32ms) spawn individually.
/// Small range checks (<15ms) grouped in 1-thread pool.
fn process_range_checks(
    parts: super::range_checks::RangeChecksInteractionParts,
    common_lookup_elements: &CommonLookupElements,
) -> RangeChecksResults {
    // 1-thread pool for small components to run them sequentially
    let small_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();

    // Heavy components
    let mut rc_20_result = None;
    let mut rc_9_9_result = None;
    // Small components grouped together
    let mut small_rc_result = None;

    let rc_20_ref = &mut rc_20_result;
    let rc_9_9_ref = &mut rc_9_9_result;
    let small_rc_ref = &mut small_rc_result;

    scope(|s| {
        // Heavy components - spawn individually
        s.spawn(|_| {
            *rc_20_ref = Some(process_single_gen(
                |builder, elems| parts.rc_20.write_interaction_trace(builder, elems),
                common_lookup_elements,
            ));
        });
        s.spawn(|_| {
            *rc_9_9_ref = Some(process_single_gen(
                |b, e| parts.rc_9_9.write_interaction_trace(b, e),
                common_lookup_elements,
            ));
        });

        // Small range checks (<15ms) - group together in 1-thread pool
        s.spawn(|_| {
            *small_rc_ref = Some(small_pool.install(|| {
                let rc_6 = process_single_gen(
                    |b, e| parts.rc_6.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_8 = process_single_gen(
                    |b, e| parts.rc_8.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_11 = process_single_gen(
                    |b, e| parts.rc_11.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_12 = process_single_gen(
                    |b, e| parts.rc_12.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_18 = process_single_gen(
                    |b, e| parts.rc_18.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_4_3 = process_single_gen(
                    |b, e| parts.rc_4_3.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_4_4 = process_single_gen(
                    |b, e| parts.rc_4_4.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_7_2_5 = process_single_gen(
                    |b, e| parts.rc_7_2_5.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_3_6_6_3 = process_single_gen(
                    |b, e| parts.rc_3_6_6_3.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_4_4_4_4 = process_single_gen(
                    |b, e| parts.rc_4_4_4_4.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                let rc_3_3_3_3_3 = process_single_gen(
                    |b, e| parts.rc_3_3_3_3_3.write_interaction_trace(b, e),
                    common_lookup_elements,
                );
                SmallRangeChecksResults {
                    rc_6,
                    rc_8,
                    rc_11,
                    rc_12,
                    rc_18,
                    rc_4_3,
                    rc_4_4,
                    rc_7_2_5,
                    rc_3_6_6_3,
                    rc_4_4_4_4,
                    rc_3_3_3_3_3,
                }
            }));
        });
    });

    let small = small_rc_result.unwrap();
    RangeChecksResults {
        rc_6: small.rc_6,
        rc_8: small.rc_8,
        rc_11: small.rc_11,
        rc_12: small.rc_12,
        rc_18: small.rc_18,
        rc_20: rc_20_result.unwrap(),
        rc_4_3: small.rc_4_3,
        rc_4_4: small.rc_4_4,
        rc_9_9: rc_9_9_result.unwrap(),
        rc_7_2_5: small.rc_7_2_5,
        rc_3_6_6_3: small.rc_3_6_6_3,
        rc_4_4_4_4: small.rc_4_4_4_4,
        rc_3_3_3_3_3: small.rc_3_3_3_3_3,
    }
}

/// Process verify bitwise xor components. All are tiny (<10ms each).
fn process_verify_xor(
    xor_4_gen: verify_bitwise_xor_4::InteractionClaimGenerator,
    xor_7_gen: verify_bitwise_xor_7::InteractionClaimGenerator,
    xor_8_gen: verify_bitwise_xor_8::InteractionClaimGenerator,
    xor_9_gen: verify_bitwise_xor_9::InteractionClaimGenerator,
    common_lookup_elements: &CommonLookupElements,
) -> VerifyXorResults {
    // All tiny, run sequentially to avoid spawn overhead
    let xor_4 = process_single_gen(
        |b, e| xor_4_gen.write_interaction_trace(b, e),
        common_lookup_elements,
    );
    let xor_7 = process_single_gen(
        |b, e| xor_7_gen.write_interaction_trace(b, e),
        common_lookup_elements,
    );
    let xor_8 = process_single_gen(
        |b, e| xor_8_gen.write_interaction_trace(b, e),
        common_lookup_elements,
    );
    let xor_9 = process_single_gen(
        |b, e| xor_9_gen.write_interaction_trace(b, e),
        common_lookup_elements,
    );
    VerifyXorResults {
        xor_4,
        xor_7,
        xor_8,
        xor_9,
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

        // Process all components in parallel using rayon::scope with spawn.
        // Structure: Group heavy tasks (>50ms) individually, batch tiny tasks (<10ms) together.

        // Declare result variables
        let mut opcodes_results = None;
        let mut verify_instruction_result = None;
        let mut memory_results = None;
        let mut pedersen_results = None;
        let mut poseidon_results = None;
        let mut blake_results = None;
        let mut builtins_results = None;
        let mut range_checks_results = None;
        let mut verify_xor_results = None;

        // Create mutable references
        let opcodes_ref = &mut opcodes_results;
        let verify_instruction_ref = &mut verify_instruction_result;
        let memory_ref = &mut memory_results;
        let pedersen_ref = &mut pedersen_results;
        let poseidon_ref = &mut poseidon_results;
        let blake_ref = &mut blake_results;
        let builtins_ref = &mut builtins_results;
        let range_checks_ref = &mut range_checks_results;
        let verify_xor_ref = &mut verify_xor_results;

        scope(|s| {
            // Opcodes
            s.spawn(|_| {
                *opcodes_ref = Some(process_all_opcodes(opcodes_parts, common_lookup_elements));
            });

            // Verify instruction
            s.spawn(|_| {
                *verify_instruction_ref = Some(process_single_gen(
                    |builder, elems| {
                        self.verify_instruction_interaction_gen
                            .write_interaction_trace(builder, elems)
                    },
                    common_lookup_elements,
                ));
            });

            // Memory
            s.spawn(|_| {
                *memory_ref = Some(process_memory(
                    self.memory_address_to_id_interaction_gen,
                    self.memory_id_to_value_interaction_gen,
                    common_lookup_elements,
                ));
            });

            // Pedersen
            s.spawn(|_| {
                *pedersen_ref = Some(process_pedersen_context(
                    pedersen_parts,
                    common_lookup_elements,
                ));
            });

            // Poseidon
            s.spawn(|_| {
                *poseidon_ref = Some(process_poseidon_context(
                    poseidon_parts,
                    common_lookup_elements,
                ));
            });

            // Blake
            s.spawn(|_| {
                *blake_ref = Some(process_blake_context(blake_parts, common_lookup_elements));
            });

            // Builtins
            s.spawn(|_| {
                *builtins_ref = Some(process_builtins(builtins_parts, common_lookup_elements));
            });

            // Range checks
            s.spawn(|_| {
                *range_checks_ref = Some(process_range_checks(
                    range_checks_parts,
                    common_lookup_elements,
                ));
            });

            // Verify XOR
            s.spawn(|_| {
                *verify_xor_ref = Some(process_verify_xor(
                    self.verify_bitwise_xor_4_interaction_gen,
                    self.verify_bitwise_xor_7_interaction_gen,
                    self.verify_bitwise_xor_8_interaction_gen,
                    self.verify_bitwise_xor_9_interaction_gen,
                    common_lookup_elements,
                ));
            });
        });

        let opcodes_results = opcodes_results.unwrap();
        let verify_instruction_result = verify_instruction_result.unwrap();
        let memory_results = memory_results.unwrap();
        let pedersen_results = pedersen_results.unwrap();
        let poseidon_results = poseidon_results.unwrap();
        let blake_results = blake_results.unwrap();
        let builtins_results = builtins_results.unwrap();
        let range_checks_results = range_checks_results.unwrap();
        let verify_xor_results = verify_xor_results.unwrap();

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
        let blake_round_claim = blake_results.blake_round.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let blake_g_claim = blake_results.blake_g.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let blake_sigma_claim = blake_results.blake_sigma.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let triple_xor_32_claim = blake_results.triple_xor_32.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let verify_bitwise_xor_12_claim = blake_results.verify_bitwise_xor_12.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Builtins
        let add_mod_builtin_claim = builtins_results.add_mod.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let bitwise_builtin_claim = builtins_results.bitwise.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let mul_mod_builtin_claim = builtins_results.mul_mod.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let pedersen_builtin_claim = builtins_results.pedersen.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let poseidon_builtin_claim = builtins_results.poseidon.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let range_check_96_builtin_claim = builtins_results.range_check_96.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let range_check_128_builtin_claim = builtins_results.range_check_128.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Pedersen context
        let pedersen_aggregator_claim = pedersen_results.aggregator.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let partial_ec_mul_claim = pedersen_results.partial_ec_mul.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let pedersen_points_table_claim = pedersen_results.points_table.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Poseidon context
        let poseidon_aggregator_claim = poseidon_results.aggregator.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let poseidon_3_partial_rounds_chain_claim =
            poseidon_results.partial_rounds_chain.map(|r| {
                r.evals.write_to(tree_builder);
                r.claim
            });
        let poseidon_full_round_chain_claim = poseidon_results.full_round_chain.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let cube_252_claim = poseidon_results.cube_252.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let poseidon_round_keys_claim = poseidon_results.round_keys.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });
        let range_check_252_width_27_claim = poseidon_results.range_check_252_width_27.map(|r| {
            r.evals.write_to(tree_builder);
            r.claim
        });

        // Memory
        memory_results.address_to_id.evals.write_to(tree_builder);
        memory_results.id_to_value.evals.write_to(tree_builder);

        // Range checks
        range_checks_results.rc_6.evals.write_to(tree_builder);
        range_checks_results.rc_8.evals.write_to(tree_builder);
        range_checks_results.rc_11.evals.write_to(tree_builder);
        range_checks_results.rc_12.evals.write_to(tree_builder);
        range_checks_results.rc_18.evals.write_to(tree_builder);
        range_checks_results.rc_20.evals.write_to(tree_builder);
        range_checks_results.rc_4_3.evals.write_to(tree_builder);
        range_checks_results.rc_4_4.evals.write_to(tree_builder);
        range_checks_results.rc_9_9.evals.write_to(tree_builder);
        range_checks_results.rc_7_2_5.evals.write_to(tree_builder);
        range_checks_results.rc_3_6_6_3.evals.write_to(tree_builder);
        range_checks_results.rc_4_4_4_4.evals.write_to(tree_builder);
        range_checks_results
            .rc_3_3_3_3_3
            .evals
            .write_to(tree_builder);

        // Verify bitwise xor
        verify_xor_results.xor_4.evals.write_to(tree_builder);
        verify_xor_results.xor_7.evals.write_to(tree_builder);
        verify_xor_results.xor_8.evals.write_to(tree_builder);
        verify_xor_results.xor_9.evals.write_to(tree_builder);

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
            rc_6: range_checks_results.rc_6.claim,
            rc_8: range_checks_results.rc_8.claim,
            rc_11: range_checks_results.rc_11.claim,
            rc_12: range_checks_results.rc_12.claim,
            rc_18: range_checks_results.rc_18.claim,
            rc_20: range_checks_results.rc_20.claim,
            rc_4_3: range_checks_results.rc_4_3.claim,
            rc_4_4: range_checks_results.rc_4_4.claim,
            rc_9_9: range_checks_results.rc_9_9.claim,
            rc_7_2_5: range_checks_results.rc_7_2_5.claim,
            rc_3_6_6_3: range_checks_results.rc_3_6_6_3.claim,
            rc_4_4_4_4: range_checks_results.rc_4_4_4_4.claim,
            rc_3_3_3_3_3: range_checks_results.rc_3_3_3_3_3.claim,
        };

        CairoInteractionClaim {
            opcodes: opcodes_interaction_claims,
            verify_instruction: verify_instruction_result.claim,
            blake_context: blake_context_interaction_claim,
            builtins: builtins_interaction_claims,
            pedersen_context: pedersen_context_interaction_claim,
            poseidon_context: poseidon_context_interaction_claim,
            memory_address_to_id: memory_results.address_to_id.claim,
            memory_id_to_value: memory_results.id_to_value.claim,
            range_checks: range_checks_interaction_claim,
            verify_bitwise_xor_4: verify_xor_results.xor_4.claim,
            verify_bitwise_xor_7: verify_xor_results.xor_7.claim,
            verify_bitwise_xor_8: verify_xor_results.xor_8.claim,
            verify_bitwise_xor_9: verify_xor_results.xor_9.claim,
        }
    }
}

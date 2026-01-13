use std::array;
use std::sync::{Arc, Mutex};

use cairo_air::air::{
    CairoClaim, CairoInteractionClaim, MemorySmallValue, PublicData, PublicMemory,
    PublicSegmentRanges, SegmentRange,
};
use cairo_air::relations::CommonLookupElements;
use indexmap::IndexSet;
use itertools::Itertools;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::MAX_SEQUENCE_LOG_SIZE;
use tracing::{span, Level};

use super::builtins::BuiltinsInteractionClaimGenerator;
use super::opcodes::OpcodesInteractionClaimGenerator;
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
/// Helper struct to hold the result of parallel interaction trace computation.
struct CairoInteractionTraceResult<T> {
    claim: T,
    evals: CollectingTreeBuilder,
}

impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> CairoInteractionClaim {
        // Use Mutex to store results from parallel tasks
        let opcodes_result: Mutex<Option<CairoInteractionTraceResult<_>>> = Mutex::new(None);
        let verify_instruction_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let blake_context_result: Mutex<Option<CairoInteractionTraceResult<_>>> = Mutex::new(None);
        let builtins_result: Mutex<Option<CairoInteractionTraceResult<_>>> = Mutex::new(None);
        let pedersen_context_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let poseidon_context_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let memory_address_to_id_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let memory_id_to_value_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let range_checks_result: Mutex<Option<CairoInteractionTraceResult<_>>> = Mutex::new(None);
        let verify_bitwise_xor_4_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let verify_bitwise_xor_7_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let verify_bitwise_xor_8_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);
        let verify_bitwise_xor_9_result: Mutex<Option<CairoInteractionTraceResult<_>>> =
            Mutex::new(None);

        // Process all generators in parallel using rayon::scope
        rayon::scope(|s| {
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .opcodes_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *opcodes_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .verify_instruction_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *verify_instruction_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .blake_context_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *blake_context_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .builtins_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *builtins_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .pedersen_context_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *pedersen_context_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .poseidon_context_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *poseidon_context_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .memory_address_to_id_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *memory_address_to_id_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .memory_id_to_value_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *memory_id_to_value_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .range_checks_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *range_checks_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .verify_bitwise_xor_4_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *verify_bitwise_xor_4_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .verify_bitwise_xor_7_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *verify_bitwise_xor_7_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .verify_bitwise_xor_8_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *verify_bitwise_xor_8_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
            s.spawn(|_| {
                let mut builder = CollectingTreeBuilder::new();
                let claim = self
                    .verify_bitwise_xor_9_interaction_gen
                    .write_interaction_trace(&mut builder, common_lookup_elements);
                *verify_bitwise_xor_9_result.lock().unwrap() =
                    Some(CairoInteractionTraceResult { claim, evals: builder });
            });
        });

        // Extract results from mutexes
        let opcodes_result = opcodes_result.into_inner().unwrap().unwrap();
        let verify_instruction_result = verify_instruction_result.into_inner().unwrap().unwrap();
        let blake_context_result = blake_context_result.into_inner().unwrap().unwrap();
        let builtins_result = builtins_result.into_inner().unwrap().unwrap();
        let pedersen_context_result = pedersen_context_result.into_inner().unwrap().unwrap();
        let poseidon_context_result = poseidon_context_result.into_inner().unwrap().unwrap();
        let memory_address_to_id_result =
            memory_address_to_id_result.into_inner().unwrap().unwrap();
        let memory_id_to_value_result = memory_id_to_value_result.into_inner().unwrap().unwrap();
        let range_checks_result = range_checks_result.into_inner().unwrap().unwrap();
        let verify_bitwise_xor_4_result =
            verify_bitwise_xor_4_result.into_inner().unwrap().unwrap();
        let verify_bitwise_xor_7_result =
            verify_bitwise_xor_7_result.into_inner().unwrap().unwrap();
        let verify_bitwise_xor_8_result =
            verify_bitwise_xor_8_result.into_inner().unwrap().unwrap();
        let verify_bitwise_xor_9_result =
            verify_bitwise_xor_9_result.into_inner().unwrap().unwrap();

        // Sequentially extend the tree builder with all collected evaluations in deterministic
        // order
        opcodes_result.evals.write_to(tree_builder);
        verify_instruction_result.evals.write_to(tree_builder);
        blake_context_result.evals.write_to(tree_builder);
        builtins_result.evals.write_to(tree_builder);
        pedersen_context_result.evals.write_to(tree_builder);
        poseidon_context_result.evals.write_to(tree_builder);
        memory_address_to_id_result.evals.write_to(tree_builder);
        memory_id_to_value_result.evals.write_to(tree_builder);
        range_checks_result.evals.write_to(tree_builder);
        verify_bitwise_xor_4_result.evals.write_to(tree_builder);
        verify_bitwise_xor_7_result.evals.write_to(tree_builder);
        verify_bitwise_xor_8_result.evals.write_to(tree_builder);
        verify_bitwise_xor_9_result.evals.write_to(tree_builder);

        CairoInteractionClaim {
            opcodes: opcodes_result.claim,
            verify_instruction: verify_instruction_result.claim,
            blake_context: blake_context_result.claim,
            builtins: builtins_result.claim,
            pedersen_context: pedersen_context_result.claim,
            poseidon_context: poseidon_context_result.claim,
            memory_address_to_id: memory_address_to_id_result.claim,
            memory_id_to_value: memory_id_to_value_result.claim,
            range_checks: range_checks_result.claim,
            verify_bitwise_xor_4: verify_bitwise_xor_4_result.claim,
            verify_bitwise_xor_7: verify_bitwise_xor_7_result.claim,
            verify_bitwise_xor_8: verify_bitwise_xor_8_result.claim,
            verify_bitwise_xor_9: verify_bitwise_xor_9_result.claim,
        }
    }
}

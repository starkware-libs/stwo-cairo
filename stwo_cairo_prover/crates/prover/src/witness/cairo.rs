use std::array;
use std::sync::Arc;

use cairo_air::air::{
    CairoClaim, CairoInteractionClaim, MemorySmallValue, PublicData, PublicMemory,
    PublicSegmentRanges, SegmentRange,
};
use cairo_air::blake::air::InteractionClaim as BlakeInteractionClaim;
use cairo_air::builtins_air::BuiltinsInteractionClaim;
use cairo_air::opcodes_air::OpcodeInteractionClaim;
use cairo_air::pedersen::air::InteractionClaim as PedersenInteractionClaim;
use cairo_air::poseidon::air::InteractionClaim as PoseidonInteractionClaim;
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
use super::opcodes::OpcodesInteractionClaimGenerator;
use super::range_checks::RangeChecksInteractionClaimGenerator;
use crate::witness::blake_context::{
    blake_context_write_trace, BlakeContextInteractionClaimGenerator,
};
use crate::witness::builtins::{builtins_write_trace, get_builtins};
use crate::witness::cairo_claim_generator::{get_sub_components, CairoClaimGenerator};
use crate::witness::components::pedersen::{
    extend_pedersen_context_traces, pedersen_context_write_trace_gen,
    PedersenContextInteractionClaimGenerator,
};
use crate::witness::components::poseidon::{
    extend_poseidon_context_traces, poseidon_context_write_trace_gen,
    PoseidonContextInteractionClaimGenerator,
};
use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_9, verify_instruction,
};
use crate::witness::opcodes::{
    complete_opcodes_with_blake, get_opcodes, opcodes_write_trace, pre_blake_opcodes_generate,
    PreBlakeOpcodeResult,
};
use crate::witness::prelude::{PreProcessedTrace, M31};
use crate::witness::range_checks::{get_range_checks, range_checks_write_trace};
use crate::witness::utils::TreeBuilder;

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
        let (
            verify_instruction_trace,
            verify_instruction_claim,
            verify_instruction_interaction_gen,
        ) = self.verify_instruction.unwrap().write_trace(
            self.range_check_7_2_5.as_ref().unwrap(),
            self.range_check_4_3.as_ref().unwrap(),
            self.memory_address_to_id.as_ref().unwrap(),
            self.memory_id_to_big.as_ref().unwrap(),
        );
        tree_builder.extend_evals(verify_instruction_trace.to_evals());
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
        // Run pedersen, poseidon, and memory_address_to_id trace generation in parallel.
        // Note: memory_id_to_big.write_trace() must run after since it takes ownership.
        let memory_address_to_id_gen = self.memory_address_to_id.take().unwrap();
        let pedersen_aggregator = self.pedersen_aggregator_window_bits_18.take();
        let partial_ec_mul = self.partial_ec_mul_window_bits_18.take();
        let pedersen_points_table = self.pedersen_points_table_window_bits_18.take();
        let poseidon_aggregator = self.poseidon_aggregator.take();
        let poseidon_3_partial = self.poseidon_3_partial_rounds_chain.take();
        let poseidon_full_round = self.poseidon_full_round_chain.take();
        let cube_252 = self.cube_252.take();
        let poseidon_round_keys = self.poseidon_round_keys.take();
        let range_check_252_width_27 = self.range_check_252_width_27.take();

        let span_parallel = span!(Level::INFO, "parallel pedersen+poseidon+memory_addr").entered();
        let mut pedersen_result = None;
        let mut poseidon_result = None;
        let mut memory_addr_result = None;
        scope(|s| {
            s.spawn(|_| {
                pedersen_result = Some(pedersen_context_write_trace_gen(
                    pedersen_aggregator,
                    partial_ec_mul,
                    pedersen_points_table,
                    self.memory_id_to_big.as_ref(),
                    self.range_check_8.as_ref(),
                    self.range_check_9_9.as_ref(),
                    self.range_check_20.as_ref(),
                ));
            });
            s.spawn(|_| {
                poseidon_result = Some(poseidon_context_write_trace_gen(
                    poseidon_aggregator,
                    poseidon_3_partial,
                    poseidon_full_round,
                    cube_252,
                    poseidon_round_keys,
                    range_check_252_width_27,
                    self.memory_id_to_big.as_ref(),
                    self.range_check_3_3_3_3_3.as_ref(),
                    self.range_check_4_4_4_4.as_ref(),
                    self.range_check_4_4.as_ref(),
                    self.range_check_9_9.as_ref(),
                    self.range_check_18.as_ref(),
                    self.range_check_20.as_ref(),
                ));
            });
            s.spawn(|_| {
                memory_addr_result = Some(memory_address_to_id_gen.write_trace());
            });
        });
        span_parallel.exit();

        let (pedersen_traces, pedersen_context_claim, pedersen_context_interaction_gen) =
            pedersen_result.unwrap();
        let (poseidon_traces, poseidon_context_claim, poseidon_context_interaction_gen) =
            poseidon_result.unwrap();
        let (memory_address_to_id_trace, memory_address_to_id_claim, memory_address_to_id_interaction_gen) =
            memory_addr_result.unwrap();

        // Extend traces in the correct order
        if let Some(traces) = pedersen_traces {
            extend_pedersen_context_traces(traces, tree_builder);
        }
        if let Some(traces) = poseidon_traces {
            extend_poseidon_context_traces(traces, tree_builder);
        }
        tree_builder.extend_evals(memory_address_to_id_trace);

        // Memory uses "Sequence", split it according to `MAX_SEQUENCE_LOG_SIZE`.
        const LOG_MAX_BIG_SIZE: u32 = MAX_SEQUENCE_LOG_SIZE;
        let (
            memory_id_to_value_big_traces,
            memory_id_to_value_small_trace,
            memory_id_to_value_claim,
            memory_id_to_value_interaction_gen,
        ) = self
            .memory_id_to_big
            .take()
            .unwrap()
            .write_trace(self.range_check_9_9.as_ref().unwrap(), LOG_MAX_BIG_SIZE);
        for big_trace in memory_id_to_value_big_traces {
            tree_builder.extend_evals(big_trace);
        }
        tree_builder.extend_evals(memory_id_to_value_small_trace);
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
        let (
            verify_bitwise_xor_4_trace,
            verify_bitwise_xor_4_claim,
            verify_bitwise_xor_4_interaction_gen,
        ) = self.verify_bitwise_xor_4.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_4_trace.to_evals());
        let (
            verify_bitwise_xor_7_trace,
            verify_bitwise_xor_7_claim,
            verify_bitwise_xor_7_interaction_gen,
        ) = self.verify_bitwise_xor_7.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_7_trace.to_evals());
        let (
            verify_bitwise_xor_8_trace,
            verify_bitwise_xor_8_claim,
            verify_bitwise_xor_8_interaction_gen,
        ) = self.verify_bitwise_xor_8.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_8_trace.to_evals());
        let (
            verify_bitwise_xor_9_trace,
            verify_bitwise_xor_9_claim,
            verify_bitwise_xor_9_interaction_gen,
        ) = self.verify_bitwise_xor_9.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_9_trace.to_evals());
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

    /// Generate pre-blake opcode traces that can run in parallel with preprocessed trace generation.
    /// Returns (PreBlakeOpcodeResult, self) where self has opcode generators removed.
    pub fn generate_pre_blake_opcode_traces(&mut self) -> PreBlakeOpcodeResult {
        pre_blake_opcodes_generate(
            self.add_opcode.take(),
            self.add_opcode_small.take(),
            self.add_ap_opcode.take(),
            self.assert_eq_opcode.take(),
            self.assert_eq_opcode_imm.take(),
            self.assert_eq_opcode_double_deref.take(),
            self.call_opcode_abs.take(),
            self.call_opcode_rel_imm.take(),
            self.generic_opcode.take(),
            self.jnz_opcode_non_taken.take(),
            self.jnz_opcode_taken.take(),
            self.jump_opcode_abs.take(),
            self.jump_opcode_double_deref.take(),
            self.jump_opcode_rel.take(),
            self.jump_opcode_rel_imm.take(),
            self.mul_opcode.take(),
            self.mul_opcode_small.take(),
            self.qm_31_add_mul_opcode.take(),
            self.ret_opcode.take(),
            self.memory_address_to_id.as_ref(),
            self.memory_id_to_big.as_ref(),
            self.range_check_11.as_ref(),
            self.range_check_18.as_ref(),
            self.range_check_20.as_ref(),
            self.range_check_4_4_4_4.as_ref(),
            self.range_check_9_9.as_ref(),
            self.verify_instruction.as_ref(),
        )
    }

    /// Write trace using pre-computed pre-blake opcode results.
    /// Use this when pre-blake traces were generated in parallel with preprocessed trace.
    pub fn write_trace_with_pre_blake(
        mut self,
        pre_blake: PreBlakeOpcodeResult,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write opcode trace").entered();
        let (opcodes_claim, opcodes_interaction_gen) = complete_opcodes_with_blake(
            pre_blake,
            self.blake_compress_opcode,
            tree_builder,
            &mut self.blake_round,
            &mut self.triple_xor_32,
            self.memory_address_to_id.as_ref(),
            self.memory_id_to_big.as_ref(),
            self.range_check_7_2_5.as_ref(),
            self.verify_instruction.as_ref(),
            self.verify_bitwise_xor_8.as_mut(),
        );
        span.exit();
        let span = span!(Level::INFO, "internal component trace").entered();
        let (
            verify_instruction_trace,
            verify_instruction_claim,
            verify_instruction_interaction_gen,
        ) = self.verify_instruction.unwrap().write_trace(
            self.range_check_7_2_5.as_ref().unwrap(),
            self.range_check_4_3.as_ref().unwrap(),
            self.memory_address_to_id.as_ref().unwrap(),
            self.memory_id_to_big.as_ref().unwrap(),
        );
        tree_builder.extend_evals(verify_instruction_trace.to_evals());
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
        // Run pedersen, poseidon, and memory_address_to_id trace generation in parallel.
        // Note: memory_id_to_big.write_trace() must run after since it takes ownership.
        let memory_address_to_id_gen = self.memory_address_to_id.take().unwrap();
        let pedersen_aggregator = self.pedersen_aggregator_window_bits_18.take();
        let partial_ec_mul = self.partial_ec_mul_window_bits_18.take();
        let pedersen_points_table = self.pedersen_points_table_window_bits_18.take();
        let poseidon_aggregator = self.poseidon_aggregator.take();
        let poseidon_3_partial = self.poseidon_3_partial_rounds_chain.take();
        let poseidon_full_round = self.poseidon_full_round_chain.take();
        let cube_252 = self.cube_252.take();
        let poseidon_round_keys = self.poseidon_round_keys.take();
        let range_check_252_width_27 = self.range_check_252_width_27.take();

        let span_parallel = span!(Level::INFO, "parallel pedersen+poseidon+memory_addr").entered();
        let mut pedersen_result = None;
        let mut poseidon_result = None;
        let mut memory_addr_result = None;
        scope(|s| {
            s.spawn(|_| {
                pedersen_result = Some(pedersen_context_write_trace_gen(
                    pedersen_aggregator,
                    partial_ec_mul,
                    pedersen_points_table,
                    self.memory_id_to_big.as_ref(),
                    self.range_check_8.as_ref(),
                    self.range_check_9_9.as_ref(),
                    self.range_check_20.as_ref(),
                ));
            });
            s.spawn(|_| {
                poseidon_result = Some(poseidon_context_write_trace_gen(
                    poseidon_aggregator,
                    poseidon_3_partial,
                    poseidon_full_round,
                    cube_252,
                    poseidon_round_keys,
                    range_check_252_width_27,
                    self.memory_id_to_big.as_ref(),
                    self.range_check_3_3_3_3_3.as_ref(),
                    self.range_check_4_4_4_4.as_ref(),
                    self.range_check_4_4.as_ref(),
                    self.range_check_9_9.as_ref(),
                    self.range_check_18.as_ref(),
                    self.range_check_20.as_ref(),
                ));
            });
            s.spawn(|_| {
                memory_addr_result = Some(memory_address_to_id_gen.write_trace());
            });
        });
        span_parallel.exit();

        let (pedersen_traces, pedersen_context_claim, pedersen_context_interaction_gen) =
            pedersen_result.unwrap();
        let (poseidon_traces, poseidon_context_claim, poseidon_context_interaction_gen) =
            poseidon_result.unwrap();
        let (memory_address_to_id_trace, memory_address_to_id_claim, memory_address_to_id_interaction_gen) =
            memory_addr_result.unwrap();

        // Extend traces in the correct order
        if let Some(traces) = pedersen_traces {
            extend_pedersen_context_traces(traces, tree_builder);
        }
        if let Some(traces) = poseidon_traces {
            extend_poseidon_context_traces(traces, tree_builder);
        }
        tree_builder.extend_evals(memory_address_to_id_trace);

        // Memory uses "Sequence", split it according to `MAX_SEQUENCE_LOG_SIZE`.
        const LOG_MAX_BIG_SIZE: u32 = MAX_SEQUENCE_LOG_SIZE;
        let (
            memory_id_to_value_big_traces,
            memory_id_to_value_small_trace,
            memory_id_to_value_claim,
            memory_id_to_value_interaction_gen,
        ) = self
            .memory_id_to_big
            .take()
            .unwrap()
            .write_trace(self.range_check_9_9.as_ref().unwrap(), LOG_MAX_BIG_SIZE);
        for big_trace in memory_id_to_value_big_traces {
            tree_builder.extend_evals(big_trace);
        }
        tree_builder.extend_evals(memory_id_to_value_small_trace);
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
        let (
            verify_bitwise_xor_4_trace,
            verify_bitwise_xor_4_claim,
            verify_bitwise_xor_4_interaction_gen,
        ) = self.verify_bitwise_xor_4.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_4_trace.to_evals());
        let (
            verify_bitwise_xor_7_trace,
            verify_bitwise_xor_7_claim,
            verify_bitwise_xor_7_interaction_gen,
        ) = self.verify_bitwise_xor_7.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_7_trace.to_evals());
        let (
            verify_bitwise_xor_8_trace,
            verify_bitwise_xor_8_claim,
            verify_bitwise_xor_8_interaction_gen,
        ) = self.verify_bitwise_xor_8.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_8_trace.to_evals());
        let (
            verify_bitwise_xor_9_trace,
            verify_bitwise_xor_9_claim,
            verify_bitwise_xor_9_interaction_gen,
        ) = self.verify_bitwise_xor_9.unwrap().write_trace();
        tree_builder.extend_evals(verify_bitwise_xor_9_trace.to_evals());
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
impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> CairoInteractionClaim {
        // Destructure optional context generators before the scope.
        let (
            blake_round_gen,
            blake_g_gen,
            blake_sigma_gen,
            blake_triple_xor_32_gen,
            blake_verify_bitwise_xor_12_gen,
        ) = match self.blake_context_interaction_gen.gen {
            Some(gen) => (
                Some(gen.blake_round_interaction_gen),
                Some(gen.blake_g_interaction_gen),
                Some(gen.blake_sigma_interaction_gen),
                Some(gen.triple_xor_32_interaction_gen),
                Some(gen.verify_bitwise_xor_12_interaction_gen),
            ),
            None => (None, None, None, None, None),
        };

        let (pedersen_aggregator_gen, partial_ec_mul_gen, pedersen_points_table_gen) =
            match self.pedersen_context_interaction_gen.gen {
                Some(gen) => (
                    Some(gen.pedersen_aggregator_interaction_gen),
                    Some(gen.partial_ec_mul_interaction_gen),
                    Some(gen.pedersen_points_table_interaction_gen),
                ),
                None => (None, None, None),
            };

        let (
            poseidon_aggregator_gen,
            poseidon_3_partial_rounds_chain_gen,
            poseidon_full_round_chain_gen,
            cube_252_gen,
            poseidon_round_keys_gen,
            range_check_252_width_27_gen,
        ) = match self.poseidon_context_interaction_gen.gen {
            Some(gen) => (
                Some(gen.poseidon_aggregator_interaction_gen),
                Some(gen.poseidon_3_partial_rounds_chain_interaction_gen),
                Some(gen.poseidon_full_round_chain_interaction_gen),
                Some(gen.cube_252_interaction_gen),
                Some(gen.poseidon_round_keys_interaction_gen),
                Some(gen.range_check_felt_252_width_27_interaction_gen),
            ),
            None => (None, None, None, None, None, None),
        };

        // Result holders for parallel execution.
        let mut add_result = None;
        let mut add_small_result = None;
        let mut add_ap_result = None;
        let mut assert_eq_result = None;
        let mut assert_eq_imm_result = None;
        let mut assert_eq_double_deref_result = None;
        let mut blake_opcode_result = None;
        let mut call_result = None;
        let mut call_rel_imm_result = None;
        let mut generic_opcode_result = None;
        let mut jnz_result = None;
        let mut jnz_taken_result = None;
        let mut jump_result = None;
        let mut jump_double_deref_result = None;
        let mut jump_rel_result = None;
        let mut jump_rel_imm_result = None;
        let mut mul_result = None;
        let mut mul_small_result = None;
        let mut qm31_result = None;
        let mut ret_result = None;
        let mut verify_instruction_result = None;
        // Blake context components
        let mut blake_round_result = None;
        let mut blake_g_result = None;
        let mut blake_sigma_result = None;
        let mut blake_triple_xor_32_result = None;
        let mut blake_verify_bitwise_xor_12_result = None;
        // Builtins
        let mut add_mod_builtin_result = None;
        let mut bitwise_builtin_result = None;
        let mut mul_mod_builtin_result = None;
        let mut pedersen_builtin_result = None;
        let mut poseidon_builtin_result = None;
        let mut range_check_96_builtin_result = None;
        let mut range_check_128_builtin_result = None;
        // Pedersen context components
        let mut pedersen_aggregator_result = None;
        let mut partial_ec_mul_result = None;
        let mut pedersen_points_table_result = None;
        // Poseidon context components
        let mut poseidon_aggregator_result = None;
        let mut poseidon_3_partial_rounds_chain_result = None;
        let mut poseidon_full_round_chain_result = None;
        let mut cube_252_result = None;
        let mut poseidon_round_keys_result = None;
        let mut range_check_252_width_27_result = None;
        // Memory
        let mut memory_address_to_id_result = None;
        let mut memory_id_to_value_result = None;
        // Range checks
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
        // Verify bitwise xor
        let mut verify_bitwise_xor_4_result = None;
        let mut verify_bitwise_xor_7_result = None;
        let mut verify_bitwise_xor_8_result = None;
        let mut verify_bitwise_xor_9_result = None;

        scope(|s| {
            s.spawn(|_| {
                partial_ec_mul_result = Some(
                    partial_ec_mul_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            // Opcodes
            s.spawn(|_| {
                add_result = Some(
                    self.opcodes_interaction_gen
                        .add
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                add_small_result = Some(
                    self.opcodes_interaction_gen
                        .add_small
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                add_ap_result = Some(
                    self.opcodes_interaction_gen
                        .add_ap
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                assert_eq_result = Some(
                    self.opcodes_interaction_gen
                        .assert_eq
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                assert_eq_imm_result = Some(
                    self.opcodes_interaction_gen
                        .assert_eq_imm
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                assert_eq_double_deref_result = Some(
                    self.opcodes_interaction_gen
                        .assert_eq_double_deref
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                blake_opcode_result = Some(
                    self.opcodes_interaction_gen
                        .blake
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                call_result = Some(
                    self.opcodes_interaction_gen
                        .call
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                call_rel_imm_result = Some(
                    self.opcodes_interaction_gen
                        .call_rel_imm
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                generic_opcode_result = Some(
                    self.opcodes_interaction_gen
                        .generic_opcode_interaction_gens
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                jnz_result = Some(
                    self.opcodes_interaction_gen
                        .jnz
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                jnz_taken_result = Some(
                    self.opcodes_interaction_gen
                        .jnz_taken
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                jump_result = Some(
                    self.opcodes_interaction_gen
                        .jump
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                jump_double_deref_result = Some(
                    self.opcodes_interaction_gen
                        .jump_double_deref
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                jump_rel_result = Some(
                    self.opcodes_interaction_gen
                        .jump_rel
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                jump_rel_imm_result = Some(
                    self.opcodes_interaction_gen
                        .jump_rel_imm
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                mul_result = Some(
                    self.opcodes_interaction_gen
                        .mul
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                mul_small_result = Some(
                    self.opcodes_interaction_gen
                        .mul_small
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                qm31_result = Some(
                    self.opcodes_interaction_gen
                        .qm31
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                ret_result = Some(
                    self.opcodes_interaction_gen
                        .ret_interaction_gens
                        .into_iter()
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });

            // Verify instruction
            s.spawn(|_| {
                verify_instruction_result = Some(
                    self.verify_instruction_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });

            // Blake
            s.spawn(|_| {
                blake_round_result = Some(
                    blake_round_gen.map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                blake_g_result = Some(
                    blake_g_gen.map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                blake_sigma_result = Some(
                    blake_sigma_gen.map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                blake_triple_xor_32_result = Some(
                    blake_triple_xor_32_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                blake_verify_bitwise_xor_12_result = Some(
                    blake_verify_bitwise_xor_12_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });

            // Builtins
            s.spawn(|_| {
                add_mod_builtin_result = Some(
                    self.builtins_interaction_gen
                        .add_mod_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                bitwise_builtin_result = Some(
                    self.builtins_interaction_gen
                        .bitwise_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                mul_mod_builtin_result = Some(
                    self.builtins_interaction_gen
                        .mul_mod_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                pedersen_builtin_result = Some(
                    self.builtins_interaction_gen
                        .pedersen_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                poseidon_builtin_result = Some(
                    self.builtins_interaction_gen
                        .poseidon_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                range_check_96_builtin_result = Some(
                    self.builtins_interaction_gen
                        .range_check_96_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });
            s.spawn(|_| {
                range_check_128_builtin_result = Some(
                    self.builtins_interaction_gen
                        .range_check_128_builtin_interaction_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements))
                        .unzip(),
                );
            });

            // Pedersen
            s.spawn(|_| {
                pedersen_aggregator_result = Some(
                    pedersen_aggregator_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                pedersen_points_table_result = Some(
                    pedersen_points_table_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });

            // Poseidon
            s.spawn(|_| {
                poseidon_aggregator_result = Some(
                    poseidon_aggregator_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                poseidon_3_partial_rounds_chain_result = Some(
                    poseidon_3_partial_rounds_chain_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                poseidon_full_round_chain_result = Some(
                    poseidon_full_round_chain_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                cube_252_result = Some(
                    cube_252_gen.map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                poseidon_round_keys_result = Some(
                    poseidon_round_keys_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });
            s.spawn(|_| {
                range_check_252_width_27_result = Some(
                    range_check_252_width_27_gen
                        .map(|gen| gen.write_interaction_trace(common_lookup_elements)),
                );
            });

            // Memory
            s.spawn(|_| {
                memory_address_to_id_result = Some(
                    self.memory_address_to_id_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                memory_id_to_value_result = Some(
                    self.memory_id_to_value_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });

            // Range checks
            s.spawn(|_| {
                rc_6_result = Some(
                    self.range_checks_interaction_gen
                        .rc_6_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_8_result = Some(
                    self.range_checks_interaction_gen
                        .rc_8_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_11_result = Some(
                    self.range_checks_interaction_gen
                        .rc_11_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_12_result = Some(
                    self.range_checks_interaction_gen
                        .rc_12_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_18_result = Some(
                    self.range_checks_interaction_gen
                        .rc_18_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_20_result = Some(
                    self.range_checks_interaction_gen
                        .rc_20_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_4_3_result = Some(
                    self.range_checks_interaction_gen
                        .rc_4_3_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_4_4_result = Some(
                    self.range_checks_interaction_gen
                        .rc_4_4_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_9_9_result = Some(
                    self.range_checks_interaction_gen
                        .rc_9_9_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_7_2_5_result = Some(
                    self.range_checks_interaction_gen
                        .rc_7_2_5_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_3_6_6_3_result = Some(
                    self.range_checks_interaction_gen
                        .rc_3_6_6_3_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_4_4_4_4_result = Some(
                    self.range_checks_interaction_gen
                        .rc_4_4_4_4_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                rc_3_3_3_3_3_result = Some(
                    self.range_checks_interaction_gen
                        .rc_3_3_3_3_3_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });

            // Verify bitwise xor
            s.spawn(|_| {
                verify_bitwise_xor_4_result = Some(
                    self.verify_bitwise_xor_4_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                verify_bitwise_xor_7_result = Some(
                    self.verify_bitwise_xor_7_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                verify_bitwise_xor_8_result = Some(
                    self.verify_bitwise_xor_8_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
            s.spawn(|_| {
                verify_bitwise_xor_9_result = Some(
                    self.verify_bitwise_xor_9_interaction_gen
                        .write_interaction_trace(common_lookup_elements),
                );
            });
        });

        // Extract results from spawns.
        let (add_traces, add_interaction_claims): (Vec<_>, Vec<_>) = add_result.unwrap();
        let (add_small_traces, add_small_interaction_claims): (Vec<_>, Vec<_>) =
            add_small_result.unwrap();
        let (add_ap_traces, add_ap_interaction_claims): (Vec<_>, Vec<_>) = add_ap_result.unwrap();
        let (assert_eq_traces, assert_eq_interaction_claims): (Vec<_>, Vec<_>) =
            assert_eq_result.unwrap();
        let (assert_eq_imm_traces, assert_eq_imm_interaction_claims): (Vec<_>, Vec<_>) =
            assert_eq_imm_result.unwrap();
        let (assert_eq_double_deref_traces, assert_eq_double_deref_interaction_claims): (
            Vec<_>,
            Vec<_>,
        ) = assert_eq_double_deref_result.unwrap();
        let (blake_opcode_traces, blake_opcode_interaction_claims): (Vec<_>, Vec<_>) =
            blake_opcode_result.unwrap();
        let (call_traces, call_interaction_claims): (Vec<_>, Vec<_>) = call_result.unwrap();
        let (call_rel_imm_traces, call_rel_imm_interaction_claims): (Vec<_>, Vec<_>) =
            call_rel_imm_result.unwrap();
        let (generic_opcode_traces, generic_opcode_interaction_claims): (Vec<_>, Vec<_>) =
            generic_opcode_result.unwrap();
        let (jnz_traces, jnz_interaction_claims): (Vec<_>, Vec<_>) = jnz_result.unwrap();
        let (jnz_taken_traces, jnz_taken_interaction_claims): (Vec<_>, Vec<_>) =
            jnz_taken_result.unwrap();
        let (jump_traces, jump_interaction_claims): (Vec<_>, Vec<_>) = jump_result.unwrap();
        let (jump_double_deref_traces, jump_double_deref_interaction_claims): (Vec<_>, Vec<_>) =
            jump_double_deref_result.unwrap();
        let (jump_rel_traces, jump_rel_interaction_claims): (Vec<_>, Vec<_>) =
            jump_rel_result.unwrap();
        let (jump_rel_imm_traces, jump_rel_imm_interaction_claims): (Vec<_>, Vec<_>) =
            jump_rel_imm_result.unwrap();
        let (mul_traces, mul_interaction_claims): (Vec<_>, Vec<_>) = mul_result.unwrap();
        let (mul_small_traces, mul_small_interaction_claims): (Vec<_>, Vec<_>) =
            mul_small_result.unwrap();
        let (qm31_traces, qm31_interaction_claims): (Vec<_>, Vec<_>) = qm31_result.unwrap();
        let (ret_traces, ret_interaction_claims): (Vec<_>, Vec<_>) = ret_result.unwrap();

        let opcodes_interaction_claims = OpcodeInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_opcode_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_opcode_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
        };

        let (verify_instruction_trace, verify_instruction_interaction_claim) =
            verify_instruction_result.unwrap();

        // Blake context - extract individual component results
        let (blake_round_trace, blake_round_interaction_claim) =
            blake_round_result.unwrap().unzip();
        let (blake_g_trace, blake_g_interaction_claim) = blake_g_result.unwrap().unzip();
        let (blake_sigma_trace, blake_sigma_interaction_claim) =
            blake_sigma_result.unwrap().unzip();
        let (blake_triple_xor_32_trace, blake_triple_xor_32_interaction_claim) =
            blake_triple_xor_32_result.unwrap().unzip();
        let (blake_verify_bitwise_xor_12_trace, blake_verify_bitwise_xor_12_interaction_claim) =
            blake_verify_bitwise_xor_12_result.unwrap().unzip();
        let blake_context_traces: Vec<_> = [
            blake_round_trace,
            blake_g_trace,
            blake_sigma_trace,
            blake_triple_xor_32_trace,
            blake_verify_bitwise_xor_12_trace,
        ]
        .into_iter()
        .flatten()
        .collect();
        let blake_context_interaction_claim = cairo_air::blake::air::BlakeContextInteractionClaim {
            claim: blake_round_interaction_claim.map(|blake_round| BlakeInteractionClaim {
                blake_round,
                blake_g: blake_g_interaction_claim.unwrap(),
                blake_sigma: blake_sigma_interaction_claim.unwrap(),
                triple_xor_32: blake_triple_xor_32_interaction_claim.unwrap(),
                verify_bitwise_xor_12: blake_verify_bitwise_xor_12_interaction_claim.unwrap(),
            }),
        };

        let (add_mod_builtin_trace, add_mod_builtin_interaction_claim) =
            add_mod_builtin_result.unwrap();
        let (bitwise_builtin_trace, bitwise_builtin_interaction_claim) =
            bitwise_builtin_result.unwrap();
        let (mul_mod_builtin_trace, mul_mod_builtin_interaction_claim) =
            mul_mod_builtin_result.unwrap();
        let (pedersen_builtin_trace, pedersen_builtin_interaction_claim) =
            pedersen_builtin_result.unwrap();
        let (poseidon_builtin_trace, poseidon_builtin_interaction_claim) =
            poseidon_builtin_result.unwrap();
        let (range_check_96_builtin_trace, range_check_96_builtin_interaction_claim) =
            range_check_96_builtin_result.unwrap();
        let (range_check_128_builtin_trace, range_check_128_builtin_interaction_claim) =
            range_check_128_builtin_result.unwrap();
        let builtins_interaction_claims = BuiltinsInteractionClaim {
            add_mod_builtin: add_mod_builtin_interaction_claim,
            bitwise_builtin: bitwise_builtin_interaction_claim,
            mul_mod_builtin: mul_mod_builtin_interaction_claim,
            pedersen_builtin: pedersen_builtin_interaction_claim,
            poseidon_builtin: poseidon_builtin_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
        };

        // Pedersen context - extract individual component results
        let (pedersen_aggregator_trace, pedersen_aggregator_interaction_claim) =
            pedersen_aggregator_result.unwrap().unzip();
        let (partial_ec_mul_trace, partial_ec_mul_interaction_claim) =
            partial_ec_mul_result.unwrap().unzip();
        let (pedersen_points_table_trace, pedersen_points_table_interaction_claim) =
            pedersen_points_table_result.unwrap().unzip();
        let pedersen_context_traces: Vec<_> = [
            pedersen_aggregator_trace,
            partial_ec_mul_trace,
            pedersen_points_table_trace,
        ]
        .into_iter()
        .flatten()
        .collect();
        let pedersen_context_interaction_claim =
            cairo_air::pedersen::air::PedersenContextInteractionClaim {
                claim: pedersen_aggregator_interaction_claim.map(|pedersen_aggregator| {
                    PedersenInteractionClaim {
                        pedersen_aggregator,
                        partial_ec_mul: partial_ec_mul_interaction_claim.unwrap(),
                        pedersen_points_table: pedersen_points_table_interaction_claim.unwrap(),
                    }
                }),
            };

        // Poseidon context - extract individual component results
        let (poseidon_aggregator_trace, poseidon_aggregator_interaction_claim) =
            poseidon_aggregator_result.unwrap().unzip();
        let (
            poseidon_3_partial_rounds_chain_trace,
            poseidon_3_partial_rounds_chain_interaction_claim,
        ) = poseidon_3_partial_rounds_chain_result.unwrap().unzip();
        let (poseidon_full_round_chain_trace, poseidon_full_round_chain_interaction_claim) =
            poseidon_full_round_chain_result.unwrap().unzip();
        let (cube_252_trace, cube_252_interaction_claim) = cube_252_result.unwrap().unzip();
        let (poseidon_round_keys_trace, poseidon_round_keys_interaction_claim) =
            poseidon_round_keys_result.unwrap().unzip();
        let (range_check_252_width_27_trace, range_check_252_width_27_interaction_claim) =
            range_check_252_width_27_result.unwrap().unzip();
        let poseidon_context_traces: Vec<_> = [
            poseidon_aggregator_trace,
            poseidon_3_partial_rounds_chain_trace,
            poseidon_full_round_chain_trace,
            cube_252_trace,
            poseidon_round_keys_trace,
            range_check_252_width_27_trace,
        ]
        .into_iter()
        .flatten()
        .collect();
        let poseidon_context_interaction_claim =
            cairo_air::poseidon::air::PoseidonContextInteractionClaim {
                claim: poseidon_aggregator_interaction_claim.map(|poseidon_aggregator| {
                    PoseidonInteractionClaim {
                        poseidon_aggregator,
                        poseidon_3_partial_rounds_chain:
                            poseidon_3_partial_rounds_chain_interaction_claim.unwrap(),
                        poseidon_full_round_chain: poseidon_full_round_chain_interaction_claim
                            .unwrap(),
                        cube_252: cube_252_interaction_claim.unwrap(),
                        poseidon_round_keys: poseidon_round_keys_interaction_claim.unwrap(),
                        range_check_252_width_27: range_check_252_width_27_interaction_claim
                            .unwrap(),
                    }
                }),
            };

        let (memory_address_to_id_trace, memory_address_to_id_interaction_claim) =
            memory_address_to_id_result.unwrap();

        let (
            memory_id_to_value_big_traces,
            memory_id_to_value_small_trace,
            memory_id_to_value_interaction_claim,
        ) = memory_id_to_value_result.unwrap();

        let (rc_6_trace, rc_6_interaction_claim) = rc_6_result.unwrap();
        let (rc_8_trace, rc_8_interaction_claim) = rc_8_result.unwrap();
        let (rc_11_trace, rc_11_interaction_claim) = rc_11_result.unwrap();
        let (rc_12_trace, rc_12_interaction_claim) = rc_12_result.unwrap();
        let (rc_18_trace, rc_18_interaction_claim) = rc_18_result.unwrap();
        let (rc_20_trace, rc_20_interaction_claim) = rc_20_result.unwrap();
        let (rc_4_3_trace, rc_4_3_interaction_claim) = rc_4_3_result.unwrap();
        let (rc_4_4_trace, rc_4_4_interaction_claim) = rc_4_4_result.unwrap();
        let (rc_9_9_trace, rc_9_9_interaction_claim) = rc_9_9_result.unwrap();
        let (rc_7_2_5_trace, rc_7_2_5_interaction_claim) = rc_7_2_5_result.unwrap();
        let (rc_3_6_6_3_trace, rc_3_6_6_3_interaction_claim) = rc_3_6_6_3_result.unwrap();
        let (rc_4_4_4_4_trace, rc_4_4_4_4_interaction_claim) = rc_4_4_4_4_result.unwrap();
        let (rc_3_3_3_3_3_trace, rc_3_3_3_3_3_interaction_claim) = rc_3_3_3_3_3_result.unwrap();
        let range_checks_interaction_claim = RangeChecksInteractionClaim {
            rc_6: rc_6_interaction_claim,
            rc_8: rc_8_interaction_claim,
            rc_11: rc_11_interaction_claim,
            rc_12: rc_12_interaction_claim,
            rc_18: rc_18_interaction_claim,
            rc_20: rc_20_interaction_claim,
            rc_4_3: rc_4_3_interaction_claim,
            rc_4_4: rc_4_4_interaction_claim,
            rc_9_9: rc_9_9_interaction_claim,
            rc_7_2_5: rc_7_2_5_interaction_claim,
            rc_3_6_6_3: rc_3_6_6_3_interaction_claim,
            rc_4_4_4_4: rc_4_4_4_4_interaction_claim,
            rc_3_3_3_3_3: rc_3_3_3_3_3_interaction_claim,
        };

        let (verify_bitwise_xor_4_trace, verify_bitwise_xor_4_interaction_claim) =
            verify_bitwise_xor_4_result.unwrap();
        let (verify_bitwise_xor_7_trace, verify_bitwise_xor_7_interaction_claim) =
            verify_bitwise_xor_7_result.unwrap();
        let (verify_bitwise_xor_8_trace, verify_bitwise_xor_8_interaction_claim) =
            verify_bitwise_xor_8_result.unwrap();
        let (verify_bitwise_xor_9_trace, verify_bitwise_xor_9_interaction_claim) =
            verify_bitwise_xor_9_result.unwrap();

        // Extend tree_builder with all traces in order
        for trace in add_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in add_small_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in add_ap_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in assert_eq_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in assert_eq_imm_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in assert_eq_double_deref_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in blake_opcode_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in call_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in call_rel_imm_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in generic_opcode_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in jnz_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in jnz_taken_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in jump_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in jump_double_deref_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in jump_rel_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in jump_rel_imm_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in mul_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in mul_small_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in qm31_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in ret_traces {
            tree_builder.extend_evals(trace);
        }
        tree_builder.extend_evals(verify_instruction_trace);
        for trace in blake_context_traces {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = add_mod_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = bitwise_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = mul_mod_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = pedersen_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = poseidon_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = range_check_96_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        if let Some(trace) = range_check_128_builtin_trace {
            tree_builder.extend_evals(trace);
        }
        for trace in pedersen_context_traces {
            tree_builder.extend_evals(trace);
        }
        for trace in poseidon_context_traces {
            tree_builder.extend_evals(trace);
        }
        tree_builder.extend_evals(memory_address_to_id_trace);
        for big_trace in memory_id_to_value_big_traces {
            tree_builder.extend_evals(big_trace);
        }
        tree_builder.extend_evals(memory_id_to_value_small_trace);
        tree_builder.extend_evals(rc_6_trace);
        tree_builder.extend_evals(rc_8_trace);
        tree_builder.extend_evals(rc_11_trace);
        tree_builder.extend_evals(rc_12_trace);
        tree_builder.extend_evals(rc_18_trace);
        tree_builder.extend_evals(rc_20_trace);
        tree_builder.extend_evals(rc_4_3_trace);
        tree_builder.extend_evals(rc_4_4_trace);
        tree_builder.extend_evals(rc_9_9_trace);
        tree_builder.extend_evals(rc_7_2_5_trace);
        tree_builder.extend_evals(rc_3_6_6_3_trace);
        tree_builder.extend_evals(rc_4_4_4_4_trace);
        tree_builder.extend_evals(rc_3_3_3_3_3_trace);
        tree_builder.extend_evals(verify_bitwise_xor_4_trace);
        tree_builder.extend_evals(verify_bitwise_xor_7_trace);
        tree_builder.extend_evals(verify_bitwise_xor_8_trace);
        tree_builder.extend_evals(verify_bitwise_xor_9_trace);

        CairoInteractionClaim {
            opcodes: opcodes_interaction_claims,
            verify_instruction: verify_instruction_interaction_claim,
            blake_context: blake_context_interaction_claim,
            builtins: builtins_interaction_claims,
            pedersen_context: pedersen_context_interaction_claim,
            poseidon_context: poseidon_context_interaction_claim,
            memory_address_to_id: memory_address_to_id_interaction_claim,
            memory_id_to_value: memory_id_to_value_interaction_claim,
            range_checks: range_checks_interaction_claim,
            verify_bitwise_xor_4: verify_bitwise_xor_4_interaction_claim,
            verify_bitwise_xor_7: verify_bitwise_xor_7_interaction_claim,
            verify_bitwise_xor_8: verify_bitwise_xor_8_interaction_claim,
            verify_bitwise_xor_9: verify_bitwise_xor_9_interaction_claim,
        }
    }
}

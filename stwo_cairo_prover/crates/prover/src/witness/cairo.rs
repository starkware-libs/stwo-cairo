use std::array;
use std::sync::Arc;

use cairo_air::air::{
    MemorySmallValue, PublicData, PublicMemory, PublicSegmentRanges, SegmentRange,
};
use cairo_air::claims::{CairoClaim, CairoInteractionClaim};
use cairo_air::relations::CommonLookupElements;
use indexmap::IndexSet;
use itertools::Itertools;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::MAX_SEQUENCE_LOG_SIZE;
use tracing::{span, Level};

use crate::witness::blake_context::{
    blake_context_write_trace, BlakeContextInteractionClaimGenerator,
};
use crate::witness::builtins::{
    builtins_write_trace, get_builtins, BuiltinsInteractionClaimGenerator,
};
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
use crate::witness::opcodes::{get_opcodes, opcodes_write_trace, OpcodesInteractionClaimGenerator};
use crate::witness::prelude::{PreProcessedTrace, M31};
use crate::witness::range_checks::{
    get_range_checks, range_checks_write_trace, RangeChecksInteractionClaimGenerator,
};
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
        let (
            add_claim,
            add_small_claim,
            add_ap_claim,
            assert_eq_claim,
            assert_eq_imm_claim,
            assert_eq_double_deref_claim,
            blake_claim,
            call_claim,
            call_rel_imm_claim,
            generic_opcode_claim,
            jnz_claim,
            jnz_taken_claim,
            jump_claim,
            jump_double_deref_claim,
            jump_rel_claim,
            jump_rel_imm_claim,
            mul_claim,
            mul_small_claim,
            qm31_claim,
            ret_claim,
            opcodes_interaction_gen,
        ) = opcodes_write_trace(
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
        let (
            blake_round_claim,
            blake_g_claim,
            blake_round_sigma_claim,
            triple_xor_32_claim,
            verify_bitwise_xor_12_claim,
            blake_context_interaction_gen,
        ) = blake_context_write_trace(
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
        let (
            add_mod_builtin_claim,
            bitwise_builtin_claim,
            mul_mod_builtin_claim,
            pedersen_builtin_claim,
            poseidon_builtin_claim,
            range_check96_builtin_claim,
            range_check_builtin_claim,
            builtins_interaction_gen,
        ) = builtins_write_trace(
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
        let (
            pedersen_aggregator_claim,
            partial_ec_mul_claim,
            pedersen_points_table_claim,
            pedersen_context_interaction_gen,
        ) = pedersen_context_write_trace(
            self.pedersen_aggregator_window_bits_18,
            self.partial_ec_mul_window_bits_18,
            self.pedersen_points_table_window_bits_18,
            tree_builder,
            self.memory_id_to_big.as_ref(),
            self.range_check_8.as_ref(),
            self.range_check_9_9.as_ref(),
            self.range_check_20.as_ref(),
        );
        let (
            poseidon_aggregator_claim,
            poseidon_3_partial_rounds_chain_claim,
            poseidon_full_round_chain_claim,
            cube_252_claim,
            poseidon_round_keys_claim,
            range_check_felt_252_width_27_claim,
            poseidon_context_interaction_gen,
        ) = poseidon_context_write_trace(
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
        let (
            range_check_6_claim,
            range_check_8_claim,
            range_check_11_claim,
            range_check_12_claim,
            range_check_18_claim,
            range_check_20_claim,
            range_check_4_3_claim,
            range_check_4_4_claim,
            range_check_9_9_claim,
            range_check_7_2_5_claim,
            range_check_3_6_6_3_claim,
            range_check_4_4_4_4_claim,
            range_check_3_3_3_3_3_claim,
            range_checks_interaction_gen,
        ) = range_checks_write_trace(
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
                add_opcode: add_claim,
                add_opcode_small: add_small_claim,
                add_ap_opcode: add_ap_claim,
                assert_eq_opcode: assert_eq_claim,
                assert_eq_opcode_imm: assert_eq_imm_claim,
                assert_eq_opcode_double_deref: assert_eq_double_deref_claim,
                blake_compress_opcode: blake_claim,
                call_opcode_abs: call_claim,
                call_opcode_rel_imm: call_rel_imm_claim,
                generic_opcode: generic_opcode_claim,
                jnz_opcode_non_taken: jnz_claim,
                jnz_opcode_taken: jnz_taken_claim,
                jump_opcode_abs: jump_claim,
                jump_opcode_double_deref: jump_double_deref_claim,
                jump_opcode_rel: jump_rel_claim,
                jump_opcode_rel_imm: jump_rel_imm_claim,
                mul_opcode: mul_claim,
                mul_opcode_small: mul_small_claim,
                qm_31_add_mul_opcode: qm31_claim,
                ret_opcode: ret_claim,
                verify_instruction: Some(verify_instruction_claim),
                blake_round: blake_round_claim,
                blake_g: blake_g_claim,
                blake_round_sigma: blake_round_sigma_claim,
                triple_xor_32: triple_xor_32_claim,
                verify_bitwise_xor_12: verify_bitwise_xor_12_claim,
                add_mod_builtin: add_mod_builtin_claim,
                bitwise_builtin: bitwise_builtin_claim,
                mul_mod_builtin: mul_mod_builtin_claim,
                pedersen_builtin: pedersen_builtin_claim,
                poseidon_builtin: poseidon_builtin_claim,
                range_check96_builtin: range_check96_builtin_claim,
                range_check_builtin: range_check_builtin_claim,
                pedersen_aggregator_window_bits_18: pedersen_aggregator_claim,
                partial_ec_mul_window_bits_18: partial_ec_mul_claim,
                pedersen_points_table_window_bits_18: pedersen_points_table_claim,
                poseidon_aggregator: poseidon_aggregator_claim,
                poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim,
                poseidon_full_round_chain: poseidon_full_round_chain_claim,
                cube_252: cube_252_claim,
                poseidon_round_keys: poseidon_round_keys_claim,
                range_check_252_width_27: range_check_felt_252_width_27_claim,
                memory_address_to_id: Some(memory_address_to_id_claim),
                memory_id_to_big: Some(memory_id_to_value_claim),
                range_check_6: range_check_6_claim,
                range_check_8: range_check_8_claim,
                range_check_11: range_check_11_claim,
                range_check_12: range_check_12_claim,
                range_check_18: range_check_18_claim,
                range_check_20: range_check_20_claim,
                range_check_4_3: range_check_4_3_claim,
                range_check_4_4: range_check_4_4_claim,
                range_check_9_9: range_check_9_9_claim,
                range_check_7_2_5: range_check_7_2_5_claim,
                range_check_3_6_6_3: range_check_3_6_6_3_claim,
                range_check_4_4_4_4: range_check_4_4_4_4_claim,
                range_check_3_3_3_3_3: range_check_3_3_3_3_3_claim,
                verify_bitwise_xor_4: Some(verify_bitwise_xor_4_claim),
                verify_bitwise_xor_7: Some(verify_bitwise_xor_7_claim),
                verify_bitwise_xor_8: Some(verify_bitwise_xor_8_claim),
                verify_bitwise_xor_9: Some(verify_bitwise_xor_9_claim),
            },
            CairoInteractionClaimGenerator {
                opcodes_interaction_gen,
                verify_instruction_interaction_gen,
                blake_context_interaction_gen,
                memory_address_to_id_interaction_gen,
                memory_id_to_value_interaction_gen,
                verify_bitwise_xor_4_interaction_gen,
                verify_bitwise_xor_7_interaction_gen,
                verify_bitwise_xor_8_interaction_gen,
                verify_bitwise_xor_9_interaction_gen,
                builtins_interaction_gen,
                pedersen_context_interaction_gen,
                poseidon_context_interaction_gen,
                range_checks_interaction_gen,
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
        let (
            add_interaction_claim,
            add_small_interaction_claim,
            add_ap_interaction_claim,
            assert_eq_interaction_claim,
            assert_eq_imm_interaction_claim,
            assert_eq_double_deref_interaction_claim,
            blake_interaction_claim,
            call_interaction_claim,
            call_rel_imm_interaction_claim,
            generic_opcode_interaction_claim,
            jnz_interaction_claim,
            jnz_taken_interaction_claim,
            jump_interaction_claim,
            jump_double_deref_interaction_claim,
            jump_rel_interaction_claim,
            jump_rel_imm_interaction_claim,
            mul_interaction_claim,
            mul_small_interaction_claim,
            qm31_interaction_claim,
            ret_interaction_claim,
        ) = self
            .opcodes_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let verify_instruction_interaction_claim = self
            .verify_instruction_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let (
            blake_round_interaction_claim,
            blake_g_interaction_claim,
            blake_round_sigma_interaction_claim,
            triple_xor_32_interaction_claim,
            verify_bitwise_xor_12_interaction_claim,
        ) = self
            .blake_context_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let (
            add_mod_builtin_interaction_claim,
            bitwise_builtin_interaction_claim,
            mul_mod_builtin_interaction_claim,
            pedersen_builtin_interaction_claim,
            poseidon_builtin_interaction_claim,
            range_check96_builtin_interaction_claim,
            range_check_builtin_interaction_claim,
        ) = self
            .builtins_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let (
            pedersen_aggregator_interaction_claim,
            partial_ec_mul_interaction_claim,
            pedersen_points_table_interaction_claim,
        ) = self
            .pedersen_context_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let (
            poseidon_aggregator_interaction_claim,
            poseidon_3_partial_rounds_chain_interaction_claim,
            poseidon_full_round_chain_interaction_claim,
            cube_252_interaction_claim,
            poseidon_round_keys_interaction_claim,
            range_check_felt_252_width_27_interaction_claim,
        ) = self
            .poseidon_context_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let memory_address_to_id_interaction_claim = self
            .memory_address_to_id_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let memory_id_to_value_interaction_claim = self
            .memory_id_to_value_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let (
            range_check_6_interaction_claim,
            range_check_8_interaction_claim,
            range_check_11_interaction_claim,
            range_check_12_interaction_claim,
            range_check_18_interaction_claim,
            range_check_20_interaction_claim,
            range_check_4_3_interaction_claim,
            range_check_4_4_interaction_claim,
            range_check_9_9_interaction_claim,
            range_check_7_2_5_interaction_claim,
            range_check_3_6_6_3_interaction_claim,
            range_check_4_4_4_4_interaction_claim,
            range_check_3_3_3_3_3_interaction_claim,
        ) = self
            .range_checks_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let verify_bitwise_xor_4_interaction_claim = self
            .verify_bitwise_xor_4_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let verify_bitwise_xor_7_interaction_claim = self
            .verify_bitwise_xor_7_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let verify_bitwise_xor_8_interaction_claim = self
            .verify_bitwise_xor_8_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);
        let verify_bitwise_xor_9_interaction_claim = self
            .verify_bitwise_xor_9_interaction_gen
            .write_interaction_trace(tree_builder, common_lookup_elements);

        CairoInteractionClaim {
            add_opcode: add_interaction_claim,
            add_opcode_small: add_small_interaction_claim,
            add_ap_opcode: add_ap_interaction_claim,
            assert_eq_opcode: assert_eq_interaction_claim,
            assert_eq_opcode_imm: assert_eq_imm_interaction_claim,
            assert_eq_opcode_double_deref: assert_eq_double_deref_interaction_claim,
            blake_compress_opcode: blake_interaction_claim,
            call_opcode_abs: call_interaction_claim,
            call_opcode_rel_imm: call_rel_imm_interaction_claim,
            generic_opcode: generic_opcode_interaction_claim,
            jnz_opcode_non_taken: jnz_interaction_claim,
            jnz_opcode_taken: jnz_taken_interaction_claim,
            jump_opcode_abs: jump_interaction_claim,
            jump_opcode_double_deref: jump_double_deref_interaction_claim,
            jump_opcode_rel: jump_rel_interaction_claim,
            jump_opcode_rel_imm: jump_rel_imm_interaction_claim,
            mul_opcode: mul_interaction_claim,
            mul_opcode_small: mul_small_interaction_claim,
            qm_31_add_mul_opcode: qm31_interaction_claim,
            ret_opcode: ret_interaction_claim,
            verify_instruction: Some(verify_instruction_interaction_claim),
            blake_round: blake_round_interaction_claim,
            blake_g: blake_g_interaction_claim,
            blake_round_sigma: blake_round_sigma_interaction_claim,
            triple_xor_32: triple_xor_32_interaction_claim,
            verify_bitwise_xor_12: verify_bitwise_xor_12_interaction_claim,
            add_mod_builtin: add_mod_builtin_interaction_claim,
            bitwise_builtin: bitwise_builtin_interaction_claim,
            mul_mod_builtin: mul_mod_builtin_interaction_claim,
            pedersen_builtin: pedersen_builtin_interaction_claim,
            poseidon_builtin: poseidon_builtin_interaction_claim,
            range_check96_builtin: range_check96_builtin_interaction_claim,
            range_check_builtin: range_check_builtin_interaction_claim,
            pedersen_aggregator_window_bits_18: pedersen_aggregator_interaction_claim,
            partial_ec_mul_window_bits_18: partial_ec_mul_interaction_claim,
            pedersen_points_table_window_bits_18: pedersen_points_table_interaction_claim,
            poseidon_aggregator: poseidon_aggregator_interaction_claim,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_interaction_claim,
            poseidon_full_round_chain: poseidon_full_round_chain_interaction_claim,
            cube_252: cube_252_interaction_claim,
            poseidon_round_keys: poseidon_round_keys_interaction_claim,
            range_check_252_width_27: range_check_felt_252_width_27_interaction_claim,
            memory_address_to_id: Some(memory_address_to_id_interaction_claim),
            memory_id_to_big: Some(memory_id_to_value_interaction_claim),
            range_check_6: range_check_6_interaction_claim,
            range_check_8: range_check_8_interaction_claim,
            range_check_11: range_check_11_interaction_claim,
            range_check_12: range_check_12_interaction_claim,
            range_check_18: range_check_18_interaction_claim,
            range_check_20: range_check_20_interaction_claim,
            range_check_4_3: range_check_4_3_interaction_claim,
            range_check_4_4: range_check_4_4_interaction_claim,
            range_check_9_9: range_check_9_9_interaction_claim,
            range_check_7_2_5: range_check_7_2_5_interaction_claim,
            range_check_3_6_6_3: range_check_3_6_6_3_interaction_claim,
            range_check_4_4_4_4: range_check_4_4_4_4_interaction_claim,
            range_check_3_3_3_3_3: range_check_3_3_3_3_3_interaction_claim,
            verify_bitwise_xor_4: Some(verify_bitwise_xor_4_interaction_claim),
            verify_bitwise_xor_7: Some(verify_bitwise_xor_7_interaction_claim),
            verify_bitwise_xor_8: Some(verify_bitwise_xor_8_interaction_claim),
            verify_bitwise_xor_9: Some(verify_bitwise_xor_9_interaction_claim),
        }
    }
}

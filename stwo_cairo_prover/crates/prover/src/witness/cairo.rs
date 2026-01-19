use std::array;
use std::sync::Arc;

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
impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &CommonLookupElements,
    ) -> CairoInteractionClaim {
        use cairo_air::blake::air::{
            BlakeContextInteractionClaim, InteractionClaim as BlakeInteractionClaim,
        };
        use cairo_air::builtins_air::BuiltinsInteractionClaim;
        use cairo_air::opcodes_air::OpcodeInteractionClaim;
        use cairo_air::pedersen::air::{
            InteractionClaim as PedersenInteractionClaim, PedersenContextInteractionClaim,
        };
        use cairo_air::poseidon::air::{
            InteractionClaim as PoseidonInteractionClaim, PoseidonContextInteractionClaim,
        };
        use cairo_air::range_checks_air::RangeChecksInteractionClaim;
        use rayon::scope;

        use crate::witness::blake_context::BlakeInteractionClaimGenerator;
        use crate::witness::components::pedersen::PedersenInteractionClaimGenerator;
        use crate::witness::components::poseidon::PoseidonInteractionClaimGenerator;
        use crate::witness::utils::DeferredTreeBuilder;

        // Destructure all nested interaction claim generators to access atomic generators.
        let OpcodesInteractionClaimGenerator {
            add: opcodes_add,
            add_small: opcodes_add_small,
            add_ap: opcodes_add_ap,
            assert_eq: opcodes_assert_eq,
            assert_eq_imm: opcodes_assert_eq_imm,
            assert_eq_double_deref: opcodes_assert_eq_double_deref,
            blake: opcodes_blake,
            call: opcodes_call,
            call_rel_imm: opcodes_call_rel_imm,
            generic_opcode_interaction_gens: opcodes_generic,
            jnz: opcodes_jnz,
            jnz_taken: opcodes_jnz_taken,
            jump: opcodes_jump,
            jump_double_deref: opcodes_jump_double_deref,
            jump_rel: opcodes_jump_rel,
            jump_rel_imm: opcodes_jump_rel_imm,
            mul: opcodes_mul,
            mul_small: opcodes_mul_small,
            qm31: opcodes_qm31,
            ret_interaction_gens: opcodes_ret,
        } = self.opcodes_interaction_gen;

        let RangeChecksInteractionClaimGenerator {
            rc_6_interaction_gen,
            rc_8_interaction_gen,
            rc_11_interaction_gen,
            rc_12_interaction_gen,
            rc_18_interaction_gen,
            rc_20_interaction_gen,
            rc_4_3_interaction_gen,
            rc_4_4_interaction_gen,
            rc_9_9_interaction_gen,
            rc_7_2_5_interaction_gen,
            rc_3_6_6_3_interaction_gen,
            rc_4_4_4_4_interaction_gen,
            rc_3_3_3_3_3_interaction_gen,
        } = self.range_checks_interaction_gen;

        let BuiltinsInteractionClaimGenerator {
            add_mod_builtin_interaction_gen,
            bitwise_builtin_interaction_gen,
            mul_mod_builtin_interaction_gen,
            pedersen_builtin_interaction_gen,
            poseidon_builtin_interaction_gen,
            range_check_96_builtin_interaction_gen,
            range_check_128_builtin_interaction_gen,
        } = self.builtins_interaction_gen;

        // Extract optional context generators and destructure them into individual Option fields.
        // This allows us to move each field independently into different spawn closures.
        let (
            blake_round_gen,
            blake_g_gen,
            blake_sigma_gen,
            blake_triple_xor_32_gen,
            blake_verify_bitwise_xor_12_gen,
        ) = match self.blake_context_interaction_gen.gen {
            Some(BlakeInteractionClaimGenerator {
                blake_round_interaction_gen,
                blake_g_interaction_gen,
                blake_sigma_interaction_gen,
                triple_xor_32_interaction_gen,
                verify_bitwise_xor_12_interaction_gen,
            }) => (
                Some(blake_round_interaction_gen),
                Some(blake_g_interaction_gen),
                Some(blake_sigma_interaction_gen),
                Some(triple_xor_32_interaction_gen),
                Some(verify_bitwise_xor_12_interaction_gen),
            ),
            None => (None, None, None, None, None),
        };

        let (pedersen_aggregator_gen, partial_ec_mul_gen, pedersen_points_table_gen) =
            match self.pedersen_context_interaction_gen.gen {
                Some(PedersenInteractionClaimGenerator {
                    pedersen_aggregator_interaction_gen,
                    partial_ec_mul_interaction_gen,
                    pedersen_points_table_interaction_gen,
                }) => (
                    Some(pedersen_aggregator_interaction_gen),
                    Some(partial_ec_mul_interaction_gen),
                    Some(pedersen_points_table_interaction_gen),
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
            Some(PoseidonInteractionClaimGenerator {
                poseidon_aggregator_interaction_gen,
                poseidon_3_partial_rounds_chain_interaction_gen,
                poseidon_full_round_chain_interaction_gen,
                cube_252_interaction_gen,
                poseidon_round_keys_interaction_gen,
                range_check_felt_252_width_27_interaction_gen,
            }) => (
                Some(poseidon_aggregator_interaction_gen),
                Some(poseidon_3_partial_rounds_chain_interaction_gen),
                Some(poseidon_full_round_chain_interaction_gen),
                Some(cube_252_interaction_gen),
                Some(poseidon_round_keys_interaction_gen),
                Some(range_check_felt_252_width_27_interaction_gen),
            ),
            None => (None, None, None, None, None, None),
        };

        // Result holders for parallel execution.
        // Opcodes
        let mut opcodes_add_result = None;
        let mut opcodes_add_small_result = None;
        let mut opcodes_add_ap_result = None;
        let mut opcodes_assert_eq_result = None;
        let mut opcodes_assert_eq_imm_result = None;
        let mut opcodes_assert_eq_double_deref_result = None;
        let mut opcodes_blake_result = None;
        let mut opcodes_call_result = None;
        let mut opcodes_call_rel_imm_result = None;
        let mut opcodes_generic_result = None;
        let mut opcodes_jnz_result = None;
        let mut opcodes_jnz_taken_result = None;
        let mut opcodes_jump_result = None;
        let mut opcodes_jump_double_deref_result = None;
        let mut opcodes_jump_rel_result = None;
        let mut opcodes_jump_rel_imm_result = None;
        let mut opcodes_mul_result = None;
        let mut opcodes_mul_small_result = None;
        let mut opcodes_qm31_result = None;
        let mut opcodes_ret_result = None;

        // Verify instruction
        let mut verify_instruction_result = None;

        // Memory
        let mut memory_address_to_id_result = None;
        let mut memory_id_to_value_result = None;

        // Blake
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

        // Pedersen
        let mut pedersen_aggregator_result = None;
        let mut partial_ec_mul_result = None;
        let mut pedersen_points_table_result = None;

        // Poseidon
        let mut poseidon_aggregator_result = None;
        let mut poseidon_3_partial_rounds_chain_result = None;
        let mut poseidon_full_round_chain_result = None;
        let mut cube_252_result = None;
        let mut poseidon_round_keys_result = None;
        let mut range_check_252_width_27_result = None;

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

        // Run all atomic write_interaction_trace calls in parallel.
        // Use join to separate partial_ec_mul from the rest.
        // partial_ec_mul gets its own thread pool with 16 threads.
        let partial_ec_mul_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(16)
            .build()
            .expect("Failed to build partial_ec_mul thread pool");

        // Sequential pool with 1 thread for small generators that don't benefit from parallelism.
        let sequential_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .expect("Failed to build sequential thread pool");

        rayon::join(
            // Left side: partial_ec_mul with dedicated 16-thread pool
            || {
                partial_ec_mul_result = partial_ec_mul_gen.map(|gen| {
                    partial_ec_mul_pool.install(|| {
                        let mut deferred = DeferredTreeBuilder::new();
                        let claim =
                            gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                        (claim, deferred)
                    })
                });
            },
            // Right side: all other generators
            || {
                scope(|s| {
                    // Opcodes
                    s.spawn(|_| {
                        opcodes_add_result = Some(
                            opcodes_add
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_add_small_result = Some(
                            opcodes_add_small
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_add_ap_result = Some(
                            opcodes_add_ap
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_assert_eq_result = Some(
                            opcodes_assert_eq
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_assert_eq_imm_result = Some(
                            opcodes_assert_eq_imm
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_assert_eq_double_deref_result = Some(
                            opcodes_assert_eq_double_deref
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_blake_result = Some(
                            opcodes_blake
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_call_result = Some(
                            opcodes_call
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_call_rel_imm_result = Some(
                            opcodes_call_rel_imm
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_generic_result = Some(
                            opcodes_generic
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_jnz_result = Some(
                            opcodes_jnz
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_jnz_taken_result = Some(
                            opcodes_jnz_taken
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_jump_result = Some(
                            opcodes_jump
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_jump_double_deref_result = Some(
                            opcodes_jump_double_deref
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_jump_rel_result = Some(
                            opcodes_jump_rel
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_jump_rel_imm_result = Some(
                            opcodes_jump_rel_imm
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_mul_result = Some(
                            opcodes_mul
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_mul_small_result = Some(
                            opcodes_mul_small
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_qm31_result = Some(
                            opcodes_qm31
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });
                    s.spawn(|_| {
                        opcodes_ret_result = Some(
                            opcodes_ret
                                .into_iter()
                                .map(|gen| {
                                    let mut deferred = DeferredTreeBuilder::new();
                                    let claim = gen.write_interaction_trace(
                                        &mut deferred,
                                        common_lookup_elements,
                                    );
                                    (claim, deferred)
                                })
                                .collect::<Vec<_>>(),
                        );
                    });

                    // Blake
                    s.spawn(|_| {
                        blake_round_result = blake_round_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        blake_g_result = blake_g_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        blake_sigma_result = blake_sigma_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        blake_triple_xor_32_result = blake_triple_xor_32_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        blake_verify_bitwise_xor_12_result =
                            blake_verify_bitwise_xor_12_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                    });

                    // Builtins
                    s.spawn(|_| {
                        add_mod_builtin_result = add_mod_builtin_interaction_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        bitwise_builtin_result = bitwise_builtin_interaction_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        mul_mod_builtin_result = mul_mod_builtin_interaction_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        range_check_96_builtin_result =
                            range_check_96_builtin_interaction_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                    });
                    s.spawn(|_| {
                        range_check_128_builtin_result = range_check_128_builtin_interaction_gen
                            .map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                    });

                    // Pedersen points table
                    s.spawn(|_| {
                        pedersen_points_table_result = pedersen_points_table_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });

                    // Poseidon
                    s.spawn(|_| {
                        poseidon_3_partial_rounds_chain_result =
                            poseidon_3_partial_rounds_chain_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                    });
                    s.spawn(|_| {
                        poseidon_full_round_chain_result =
                            poseidon_full_round_chain_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                    });
                    s.spawn(|_| {
                        cube_252_result = cube_252_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        poseidon_round_keys_result = poseidon_round_keys_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });
                    s.spawn(|_| {
                        range_check_252_width_27_result = range_check_252_width_27_gen.map(|gen| {
                            let mut deferred = DeferredTreeBuilder::new();
                            let claim =
                                gen.write_interaction_trace(&mut deferred, common_lookup_elements);
                            (claim, deferred)
                        });
                    });

                    // Memory
                    s.spawn(|_| {
                        let mut deferred = DeferredTreeBuilder::new();
                        let claim = self
                            .memory_address_to_id_interaction_gen
                            .write_interaction_trace(&mut deferred, common_lookup_elements);
                        memory_address_to_id_result = Some((claim, deferred));
                    });
                    s.spawn(|_| {
                        let mut deferred = DeferredTreeBuilder::new();
                        let claim = self
                            .memory_id_to_value_interaction_gen
                            .write_interaction_trace(&mut deferred, common_lookup_elements);
                        memory_id_to_value_result = Some((claim, deferred));
                    });

                    // Heavy range checks
                    s.spawn(|_| {
                        let mut deferred = DeferredTreeBuilder::new();
                        let claim = rc_9_9_interaction_gen
                            .write_interaction_trace(&mut deferred, common_lookup_elements);
                        rc_9_9_result = Some((claim, deferred));
                    });
                    s.spawn(|_| {
                        let mut deferred = DeferredTreeBuilder::new();
                        let claim = rc_20_interaction_gen
                            .write_interaction_trace(&mut deferred, common_lookup_elements);
                        rc_20_result = Some((claim, deferred));
                    });

                    // Sequential spawn for small generators (1-thread pool)
                    s.spawn(|_| {
                        sequential_pool.install(|| {
                            pedersen_aggregator_result = pedersen_aggregator_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                            poseidon_aggregator_result = poseidon_aggregator_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                            pedersen_builtin_result = pedersen_builtin_interaction_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                            poseidon_builtin_result = poseidon_builtin_interaction_gen.map(|gen| {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                (claim, deferred)
                            });
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_6_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_6_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_8_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_8_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_11_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_11_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_12_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_12_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_18_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_18_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_4_3_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_4_3_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_4_4_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_4_4_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_7_2_5_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_7_2_5_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_3_6_6_3_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_3_6_6_3_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_4_4_4_4_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_4_4_4_4_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = rc_3_3_3_3_3_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                rc_3_3_3_3_3_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = self
                                    .verify_bitwise_xor_4_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                verify_bitwise_xor_4_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = self
                                    .verify_bitwise_xor_7_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                verify_bitwise_xor_7_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = self
                                    .verify_bitwise_xor_8_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                verify_bitwise_xor_8_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = self
                                    .verify_bitwise_xor_9_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                verify_bitwise_xor_9_result = Some((claim, deferred));
                            }
                            {
                                let mut deferred = DeferredTreeBuilder::new();
                                let claim = self
                                    .verify_instruction_interaction_gen
                                    .write_interaction_trace(&mut deferred, common_lookup_elements);
                                verify_instruction_result = Some((claim, deferred));
                            }
                        });
                    });
                });
            },
        );

        // Now flush all deferred evals to tree_builder in the correct order.
        // The order must match the original sequential order.

        // Opcodes (in order) - inline the flush logic to avoid closure type inference issues
        let add_interaction_claims: Vec<_> = opcodes_add_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let add_small_interaction_claims: Vec<_> = opcodes_add_small_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let add_ap_interaction_claims: Vec<_> = opcodes_add_ap_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let assert_eq_interaction_claims: Vec<_> = opcodes_assert_eq_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let assert_eq_imm_interaction_claims: Vec<_> = opcodes_assert_eq_imm_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let assert_eq_double_deref_interaction_claims: Vec<_> =
            opcodes_assert_eq_double_deref_result
                .unwrap()
                .into_iter()
                .map(|(claim, deferred)| {
                    deferred.flush_to(tree_builder);
                    claim
                })
                .collect();
        let blake_interaction_claims: Vec<_> = opcodes_blake_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let call_interaction_claims: Vec<_> = opcodes_call_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let call_rel_imm_interaction_claims: Vec<_> = opcodes_call_rel_imm_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let generic_opcode_interaction_claims: Vec<_> = opcodes_generic_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let jnz_interaction_claims: Vec<_> = opcodes_jnz_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let jnz_taken_interaction_claims: Vec<_> = opcodes_jnz_taken_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let jump_interaction_claims: Vec<_> = opcodes_jump_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let jump_double_deref_interaction_claims: Vec<_> = opcodes_jump_double_deref_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let jump_rel_interaction_claims: Vec<_> = opcodes_jump_rel_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let jump_rel_imm_interaction_claims: Vec<_> = opcodes_jump_rel_imm_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let mul_interaction_claims: Vec<_> = opcodes_mul_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let mul_small_interaction_claims: Vec<_> = opcodes_mul_small_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let qm31_interaction_claims: Vec<_> = opcodes_qm31_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();
        let ret_interaction_claims: Vec<_> = opcodes_ret_result
            .unwrap()
            .into_iter()
            .map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            })
            .collect();

        let opcodes_interaction_claims = OpcodeInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
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

        // Verify instruction
        let (verify_instruction_interaction_claim, deferred) = verify_instruction_result.unwrap();
        deferred.flush_to(tree_builder);

        // Blake context
        let blake_context_interaction_claim =
            if let Some((blake_round_claim, deferred)) = blake_round_result {
                deferred.flush_to(tree_builder);
                let (blake_g_claim, deferred) = blake_g_result.unwrap();
                deferred.flush_to(tree_builder);
                let (blake_sigma_claim, deferred) = blake_sigma_result.unwrap();
                deferred.flush_to(tree_builder);
                let (triple_xor_32_claim, deferred) = blake_triple_xor_32_result.unwrap();
                deferred.flush_to(tree_builder);
                let (verify_bitwise_xor_12_claim, deferred) =
                    blake_verify_bitwise_xor_12_result.unwrap();
                deferred.flush_to(tree_builder);

                BlakeContextInteractionClaim {
                    claim: Some(BlakeInteractionClaim {
                        blake_round: blake_round_claim,
                        blake_g: blake_g_claim,
                        blake_sigma: blake_sigma_claim,
                        triple_xor_32: triple_xor_32_claim,
                        verify_bitwise_xor_12: verify_bitwise_xor_12_claim,
                    }),
                }
            } else {
                BlakeContextInteractionClaim { claim: None }
            };

        // Builtins
        let add_mod_builtin_interaction_claim = add_mod_builtin_result.map(|(claim, deferred)| {
            deferred.flush_to(tree_builder);
            claim
        });
        let bitwise_builtin_interaction_claim = bitwise_builtin_result.map(|(claim, deferred)| {
            deferred.flush_to(tree_builder);
            claim
        });
        let mul_mod_builtin_interaction_claim = mul_mod_builtin_result.map(|(claim, deferred)| {
            deferred.flush_to(tree_builder);
            claim
        });
        let pedersen_builtin_interaction_claim =
            pedersen_builtin_result.map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            });
        let poseidon_builtin_interaction_claim =
            poseidon_builtin_result.map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            });
        let range_check_96_builtin_interaction_claim =
            range_check_96_builtin_result.map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            });
        let range_check_128_builtin_interaction_claim =
            range_check_128_builtin_result.map(|(claim, deferred)| {
                deferred.flush_to(tree_builder);
                claim
            });

        let builtins_interaction_claims = BuiltinsInteractionClaim {
            add_mod_builtin: add_mod_builtin_interaction_claim,
            bitwise_builtin: bitwise_builtin_interaction_claim,
            mul_mod_builtin: mul_mod_builtin_interaction_claim,
            pedersen_builtin: pedersen_builtin_interaction_claim,
            poseidon_builtin: poseidon_builtin_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
        };

        // Pedersen context
        let pedersen_context_interaction_claim =
            if let Some((pedersen_aggregator_claim, deferred)) = pedersen_aggregator_result {
                deferred.flush_to(tree_builder);
                let (partial_ec_mul_claim, deferred) = partial_ec_mul_result.unwrap();
                deferred.flush_to(tree_builder);
                let (pedersen_points_table_claim, deferred) = pedersen_points_table_result.unwrap();
                deferred.flush_to(tree_builder);

                PedersenContextInteractionClaim {
                    claim: Some(PedersenInteractionClaim {
                        pedersen_aggregator: pedersen_aggregator_claim,
                        partial_ec_mul: partial_ec_mul_claim,
                        pedersen_points_table: pedersen_points_table_claim,
                    }),
                }
            } else {
                PedersenContextInteractionClaim { claim: None }
            };

        // Poseidon context
        let poseidon_context_interaction_claim =
            if let Some((poseidon_aggregator_claim, deferred)) = poseidon_aggregator_result {
                deferred.flush_to(tree_builder);
                let (poseidon_3_partial_rounds_chain_claim, deferred) =
                    poseidon_3_partial_rounds_chain_result.unwrap();
                deferred.flush_to(tree_builder);
                let (poseidon_full_round_chain_claim, deferred) =
                    poseidon_full_round_chain_result.unwrap();
                deferred.flush_to(tree_builder);
                let (cube_252_claim, deferred) = cube_252_result.unwrap();
                deferred.flush_to(tree_builder);
                let (poseidon_round_keys_claim, deferred) = poseidon_round_keys_result.unwrap();
                deferred.flush_to(tree_builder);
                let (range_check_252_width_27_claim, deferred) =
                    range_check_252_width_27_result.unwrap();
                deferred.flush_to(tree_builder);

                PoseidonContextInteractionClaim {
                    claim: Some(PoseidonInteractionClaim {
                        poseidon_aggregator: poseidon_aggregator_claim,
                        poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim,
                        poseidon_full_round_chain: poseidon_full_round_chain_claim,
                        cube_252: cube_252_claim,
                        poseidon_round_keys: poseidon_round_keys_claim,
                        range_check_252_width_27: range_check_252_width_27_claim,
                    }),
                }
            } else {
                PoseidonContextInteractionClaim { claim: None }
            };

        // Memory
        let (memory_address_to_id_interaction_claim, deferred) =
            memory_address_to_id_result.unwrap();
        deferred.flush_to(tree_builder);
        let (memory_id_to_value_interaction_claim, deferred) = memory_id_to_value_result.unwrap();
        deferred.flush_to(tree_builder);

        // Range checks
        let (rc_6_interaction_claim, deferred) = rc_6_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_8_interaction_claim, deferred) = rc_8_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_11_interaction_claim, deferred) = rc_11_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_12_interaction_claim, deferred) = rc_12_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_18_interaction_claim, deferred) = rc_18_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_20_interaction_claim, deferred) = rc_20_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_4_3_interaction_claim, deferred) = rc_4_3_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_4_4_interaction_claim, deferred) = rc_4_4_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_9_9_interaction_claim, deferred) = rc_9_9_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_7_2_5_interaction_claim, deferred) = rc_7_2_5_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_3_6_6_3_interaction_claim, deferred) = rc_3_6_6_3_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_4_4_4_4_interaction_claim, deferred) = rc_4_4_4_4_result.unwrap();
        deferred.flush_to(tree_builder);
        let (rc_3_3_3_3_3_interaction_claim, deferred) = rc_3_3_3_3_3_result.unwrap();
        deferred.flush_to(tree_builder);

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

        // Verify bitwise xor
        let (verify_bitwise_xor_4_interaction_claim, deferred) =
            verify_bitwise_xor_4_result.unwrap();
        deferred.flush_to(tree_builder);
        let (verify_bitwise_xor_7_interaction_claim, deferred) =
            verify_bitwise_xor_7_result.unwrap();
        deferred.flush_to(tree_builder);
        let (verify_bitwise_xor_8_interaction_claim, deferred) =
            verify_bitwise_xor_8_result.unwrap();
        deferred.flush_to(tree_builder);
        let (verify_bitwise_xor_9_interaction_claim, deferred) =
            verify_bitwise_xor_9_result.unwrap();
        deferred.flush_to(tree_builder);

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

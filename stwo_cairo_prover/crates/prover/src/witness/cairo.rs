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
use super::opcodes::{process_interaction_gens, OpcodesInteractionClaimGenerator};
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

        // Move the "self.*" single generators out so we can freely capture them in FnOnce closures.
        let verify_instruction_gen = self.verify_instruction_interaction_gen;

        let memory_address_to_id_gen = self.memory_address_to_id_interaction_gen;
        let memory_id_to_value_gen = self.memory_id_to_value_interaction_gen;

        let verify_bitwise_xor_4_gen = self.verify_bitwise_xor_4_interaction_gen;
        let verify_bitwise_xor_7_gen = self.verify_bitwise_xor_7_interaction_gen;
        let verify_bitwise_xor_8_gen = self.verify_bitwise_xor_8_interaction_gen;
        let verify_bitwise_xor_9_gen = self.verify_bitwise_xor_9_interaction_gen;

        // ======== COMPUTE (PARALLEL) ========
        //
        // Key change vs your version:
        // - no Mutex<Option<...>>
        // - no rayon::scope + spawn forest
        // - purely functional: compute values in parallel, then serially write to tree_builder
        //
        // We use a balanced-ish tree of rayon::join calls. This gives:
        // - minimal overhead
        // - no shared-state contention
        // - deterministic structure (and it composes nicely if you later group work)

        // --- OPCODES (20 types) ---
        //
        // We'll build a join tree. Each leaf computes one InteractionTraceResult<_>.
        // Types are inferred exactly like your original code (with `_`).
        let (
            (
                ((add_result, add_small_result), (add_ap_result, assert_eq_result)),
                (
                    (assert_eq_imm_result, assert_eq_double_deref_result),
                    (blake_opcode_result, call_result),
                ),
            ),
            (
                ((call_rel_imm_result, generic_opcode_result), (jnz_result, jnz_taken_result)),
                (
                    (jump_result, jump_double_deref_result),
                    ((jump_rel_result, jump_rel_imm_result), (mul_result, mul_small_result)),
                ),
            ),
        ) = rayon::join(
            || {
                rayon::join(
                    || {
                        rayon::join(
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.add,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.add_small,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.add_ap,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.assert_eq,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                        )
                    },
                    || {
                        rayon::join(
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.assert_eq_imm,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.assert_eq_double_deref,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.blake,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.call,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                        )
                    },
                )
            },
            || {
                rayon::join(
                    || {
                        rayon::join(
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.call_rel_imm,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.generic_opcode,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.jnz,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.jnz_taken,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                        )
                    },
                    || {
                        rayon::join(
                            || {
                                rayon::join(
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.jump,
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_interaction_gens(
                                            opcodes_parts.jump_double_deref,
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                            || {
                                rayon::join(
                                    || {
                                        rayon::join(
                                            || {
                                                process_interaction_gens(
                                                    opcodes_parts.jump_rel,
                                                    common_lookup_elements,
                                                )
                                            },
                                            || {
                                                process_interaction_gens(
                                                    opcodes_parts.jump_rel_imm,
                                                    common_lookup_elements,
                                                )
                                            },
                                        )
                                    },
                                    || {
                                        rayon::join(
                                            || {
                                                process_interaction_gens(
                                                    opcodes_parts.mul,
                                                    common_lookup_elements,
                                                )
                                            },
                                            || {
                                                process_interaction_gens(
                                                    opcodes_parts.mul_small,
                                                    common_lookup_elements,
                                                )
                                            },
                                        )
                                    },
                                )
                            },
                        )
                    },
                )
            },
        );

        let ((qm31_result, ret_result), verify_instruction_result) = rayon::join(
            || {
                rayon::join(
                    || process_interaction_gens(opcodes_parts.qm31, common_lookup_elements),
                    || process_interaction_gens(opcodes_parts.ret, common_lookup_elements),
                )
            },
            || {
                process_single_gen(
                    |builder, elems| verify_instruction_gen.write_interaction_trace(builder, elems),
                    common_lookup_elements,
                )
            },
        );

        // --- Remaining opcode: mul already done; we still need opcode "mul" results above and
        // "qm31/ret" here --- (All 20 are now present: add, add_small, add_ap, assert_eq,
        // assert_eq_imm,  assert_eq_double_deref, blake, call, call_rel_imm,
        // generic_opcode, jnz,  jnz_taken, jump, jump_double_deref, jump_rel, jump_rel_imm,
        // mul, mul_small,  qm31, ret.)

        // --- Optional contexts + memory/range/xor ---
        //
        // We compute these in parallel too. The option blocks stay option blocks,
        // but we avoid spawning tasks just to set mutexes.
        let ((blake_ctx_results, builtins_results), (pedersen_ctx_results, poseidon_ctx_results)) =
            rayon::join(
                || {
                    rayon::join(
                        || {
                            blake_parts.map(|blake| {
                                // 5 components
                                let ((blake_round, blake_g), (blake_sigma, triple_xor_32)) =
                                    rayon::join(
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            blake
                                                                .blake_round
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            blake
                                                                .blake_g
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            blake
                                                                .blake_sigma
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            blake
                                                                .triple_xor_32
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                    );

                                let verify_bitwise_xor_12 = process_single_gen(
                                    |b, e| {
                                        blake.verify_bitwise_xor_12.write_interaction_trace(b, e)
                                    },
                                    common_lookup_elements,
                                );

                                (
                                    blake_round,
                                    blake_g,
                                    blake_sigma,
                                    triple_xor_32,
                                    verify_bitwise_xor_12,
                                )
                            })
                        },
                        || {
                            // builtins: 7 optional
                            let (
                                (add_mod_builtin, bitwise_builtin),
                                (mul_mod_builtin, pedersen_builtin),
                            ) = rayon::join(
                                || {
                                    rayon::join(
                                        || {
                                            builtins_parts.add_mod_builtin.map(|gen| {
                                                process_single_gen(
                                                    |b, e| gen.write_interaction_trace(b, e),
                                                    common_lookup_elements,
                                                )
                                            })
                                        },
                                        || {
                                            builtins_parts.bitwise_builtin.map(|gen| {
                                                process_single_gen(
                                                    |b, e| gen.write_interaction_trace(b, e),
                                                    common_lookup_elements,
                                                )
                                            })
                                        },
                                    )
                                },
                                || {
                                    rayon::join(
                                        || {
                                            builtins_parts.mul_mod_builtin.map(|gen| {
                                                process_single_gen(
                                                    |b, e| gen.write_interaction_trace(b, e),
                                                    common_lookup_elements,
                                                )
                                            })
                                        },
                                        || {
                                            builtins_parts.pedersen_builtin.map(|gen| {
                                                process_single_gen(
                                                    |b, e| gen.write_interaction_trace(b, e),
                                                    common_lookup_elements,
                                                )
                                            })
                                        },
                                    )
                                },
                            );

                            let (
                                (poseidon_builtin, range_check_96_builtin),
                                range_check_128_builtin,
                            ) = rayon::join(
                                || {
                                    rayon::join(
                                        || {
                                            builtins_parts.poseidon_builtin.map(|gen| {
                                                process_single_gen(
                                                    |b, e| gen.write_interaction_trace(b, e),
                                                    common_lookup_elements,
                                                )
                                            })
                                        },
                                        || {
                                            builtins_parts.range_check_96_builtin.map(|gen| {
                                                process_single_gen(
                                                    |b, e| gen.write_interaction_trace(b, e),
                                                    common_lookup_elements,
                                                )
                                            })
                                        },
                                    )
                                },
                                || {
                                    builtins_parts.range_check_128_builtin.map(|gen| {
                                        process_single_gen(
                                            |b, e| gen.write_interaction_trace(b, e),
                                            common_lookup_elements,
                                        )
                                    })
                                },
                            );

                            (
                                add_mod_builtin,
                                bitwise_builtin,
                                mul_mod_builtin,
                                pedersen_builtin,
                                poseidon_builtin,
                                range_check_96_builtin,
                                range_check_128_builtin,
                            )
                        },
                    )
                },
                || {
                    rayon::join(
                        || {
                            pedersen_parts.map(|pedersen| {
                                let ((pedersen_aggregator, partial_ec_mul), pedersen_points_table) =
                                    rayon::join(
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            pedersen
                                                                .pedersen_aggregator
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            pedersen
                                                                .partial_ec_mul
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                        || {
                                            process_single_gen(
                                                |b, e| {
                                                    pedersen
                                                        .pedersen_points_table
                                                        .write_interaction_trace(b, e)
                                                },
                                                common_lookup_elements,
                                            )
                                        },
                                    );
                                (pedersen_aggregator, partial_ec_mul, pedersen_points_table)
                            })
                        },
                        || {
                            poseidon_parts.map(|poseidon| {
                                let (
                                    (poseidon_aggregator, poseidon_3_partial_rounds_chain),
                                    (poseidon_full_round_chain, cube_252),
                                ) = rayon::join(
                                    || {
                                        rayon::join(
                                            || {
                                                process_single_gen(
                                                    |b, e| {
                                                        poseidon
                                                            .poseidon_aggregator
                                                            .write_interaction_trace(b, e)
                                                    },
                                                    common_lookup_elements,
                                                )
                                            },
                                            || {
                                                process_single_gen(
                                                    |b, e| {
                                                        poseidon
                                                            .poseidon_3_partial_rounds_chain
                                                            .write_interaction_trace(b, e)
                                                    },
                                                    common_lookup_elements,
                                                )
                                            },
                                        )
                                    },
                                    || {
                                        rayon::join(
                                            || {
                                                process_single_gen(
                                                    |b, e| {
                                                        poseidon
                                                            .poseidon_full_round_chain
                                                            .write_interaction_trace(b, e)
                                                    },
                                                    common_lookup_elements,
                                                )
                                            },
                                            || {
                                                process_single_gen(
                                                    |b, e| {
                                                        poseidon
                                                            .cube_252
                                                            .write_interaction_trace(b, e)
                                                    },
                                                    common_lookup_elements,
                                                )
                                            },
                                        )
                                    },
                                );

                                let (poseidon_round_keys, range_check_252_width_27) = rayon::join(
                                    || {
                                        process_single_gen(
                                            |b, e| {
                                                poseidon
                                                    .poseidon_round_keys
                                                    .write_interaction_trace(b, e)
                                            },
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_single_gen(
                                            |b, e| {
                                                poseidon
                                                    .range_check_252_width_27
                                                    .write_interaction_trace(b, e)
                                            },
                                            common_lookup_elements,
                                        )
                                    },
                                );

                                (
                                    poseidon_aggregator,
                                    poseidon_3_partial_rounds_chain,
                                    poseidon_full_round_chain,
                                    cube_252,
                                    poseidon_round_keys,
                                    range_check_252_width_27,
                                )
                            })
                        },
                    )
                },
            );

        let (
            (memory_address_to_id_result, memory_id_to_value_result),
            (range_checks_results, xor_results),
        ) = rayon::join(
            || {
                rayon::join(
                    || {
                        process_single_gen(
                            |b, e| memory_address_to_id_gen.write_interaction_trace(b, e),
                            common_lookup_elements,
                        )
                    },
                    || {
                        process_single_gen(
                            |b, e| memory_id_to_value_gen.write_interaction_trace(b, e),
                            common_lookup_elements,
                        )
                    },
                )
            },
            || {
                rayon::join(
                    || {
                        // range checks (13)
                        let (((rc_6, rc_8), (rc_11, rc_12)), ((rc_18, rc_20), (rc_4_3, rc_4_4))) =
                            rayon::join(
                                || {
                                    rayon::join(
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_6
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_8
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_11
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_12
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                    )
                                },
                                || {
                                    rayon::join(
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_18
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_20
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_4_3
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_4_4
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                    )
                                },
                            );

                        let (((rc_9_9, rc_7_2_5), (rc_3_6_6_3, rc_4_4_4_4)), rc_3_3_3_3_3) =
                            rayon::join(
                                || {
                                    rayon::join(
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_9_9
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_7_2_5
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                        || {
                                            rayon::join(
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_3_6_6_3
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                                || {
                                                    process_single_gen(
                                                        |b, e| {
                                                            range_checks_parts
                                                                .rc_4_4_4_4
                                                                .write_interaction_trace(b, e)
                                                        },
                                                        common_lookup_elements,
                                                    )
                                                },
                                            )
                                        },
                                    )
                                },
                                || {
                                    process_single_gen(
                                        |b, e| {
                                            range_checks_parts
                                                .rc_3_3_3_3_3
                                                .write_interaction_trace(b, e)
                                        },
                                        common_lookup_elements,
                                    )
                                },
                            );

                        (
                            rc_6,
                            rc_8,
                            rc_11,
                            rc_12,
                            rc_18,
                            rc_20,
                            rc_4_3,
                            rc_4_4,
                            rc_9_9,
                            rc_7_2_5,
                            rc_3_6_6_3,
                            rc_4_4_4_4,
                            rc_3_3_3_3_3,
                        )
                    },
                    || {
                        // verify bitwise xor (4)
                        let ((xor_4, xor_7), (xor_8, xor_9)) = rayon::join(
                            || {
                                rayon::join(
                                    || {
                                        process_single_gen(
                                            |b, e| {
                                                verify_bitwise_xor_4_gen
                                                    .write_interaction_trace(b, e)
                                            },
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_single_gen(
                                            |b, e| {
                                                verify_bitwise_xor_7_gen
                                                    .write_interaction_trace(b, e)
                                            },
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                            || {
                                rayon::join(
                                    || {
                                        process_single_gen(
                                            |b, e| {
                                                verify_bitwise_xor_8_gen
                                                    .write_interaction_trace(b, e)
                                            },
                                            common_lookup_elements,
                                        )
                                    },
                                    || {
                                        process_single_gen(
                                            |b, e| {
                                                verify_bitwise_xor_9_gen
                                                    .write_interaction_trace(b, e)
                                            },
                                            common_lookup_elements,
                                        )
                                    },
                                )
                            },
                        );
                        (xor_4, xor_7, xor_8, xor_9)
                    },
                )
            },
        );

        // Unpack the big tuples into named values (readability)
        let (jump_rel_result, jump_rel_imm_result) = (jump_rel_result, jump_rel_imm_result);
        let (mul_result, mul_small_result) = (mul_result, mul_small_result);

        let (
            rc_6_result,
            rc_8_result,
            rc_11_result,
            rc_12_result,
            rc_18_result,
            rc_20_result,
            rc_4_3_result,
            rc_4_4_result,
            rc_9_9_result,
            rc_7_2_5_result,
            rc_3_6_6_3_result,
            rc_4_4_4_4_result,
            rc_3_3_3_3_3_result,
        ) = range_checks_results;

        let (
            verify_bitwise_xor_4_result,
            verify_bitwise_xor_7_result,
            verify_bitwise_xor_8_result,
            verify_bitwise_xor_9_result,
        ) = xor_results;

        // Optional group unpack
        let (
            blake_round_result,
            blake_g_result,
            blake_sigma_result,
            triple_xor_32_result,
            verify_bitwise_xor_12_result,
        ) = match blake_ctx_results {
            Some((a, b, c, d, e)) => (Some(a), Some(b), Some(c), Some(d), Some(e)),
            None => (None, None, None, None, None),
        };

        let (
            add_mod_builtin_result,
            bitwise_builtin_result,
            mul_mod_builtin_result,
            pedersen_builtin_result,
            poseidon_builtin_result,
            range_check_96_builtin_result,
            range_check_128_builtin_result,
        ) = builtins_results;

        let (pedersen_aggregator_result, partial_ec_mul_result, pedersen_points_table_result) =
            match pedersen_ctx_results {
                Some((a, b, c)) => (Some(a), Some(b), Some(c)),
                None => (None, None, None),
            };

        let (
            poseidon_aggregator_result,
            poseidon_3_partial_rounds_chain_result,
            poseidon_full_round_chain_result,
            cube_252_result,
            poseidon_round_keys_result,
            range_check_252_width_27_result,
        ) = match poseidon_ctx_results {
            Some((a, b, c, d, e, f)) => (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)),
            None => (None, None, None, None, None, None),
        };

        // ======== SERIAL “WRITE EVALS” (DETERMINISTIC) + CLAIM ASSEMBLY ========

        // Opcodes: write evals in deterministic order
        for builder in add_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in add_small_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in add_ap_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in assert_eq_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in assert_eq_imm_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in assert_eq_double_deref_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in blake_opcode_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in call_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in call_rel_imm_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in generic_opcode_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jnz_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jnz_taken_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_double_deref_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_rel_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in jump_rel_imm_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in mul_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in mul_small_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in qm31_result.evals {
            builder.write_to(tree_builder);
        }
        for builder in ret_result.evals {
            builder.write_to(tree_builder);
        }

        // Verify instruction
        verify_instruction_result.evals.write_to(tree_builder);

        // Blake context: write evals + extract claims
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
        verify_bitwise_xor_4_result.evals.write_to(tree_builder);
        verify_bitwise_xor_7_result.evals.write_to(tree_builder);
        verify_bitwise_xor_8_result.evals.write_to(tree_builder);
        verify_bitwise_xor_9_result.evals.write_to(tree_builder);

        // Final opcode claims
        let opcodes_interaction_claims = OpcodeInteractionClaim {
            add: add_result.claims,
            add_small: add_small_result.claims,
            add_ap: add_ap_result.claims,
            assert_eq: assert_eq_result.claims,
            assert_eq_imm: assert_eq_imm_result.claims,
            assert_eq_double_deref: assert_eq_double_deref_result.claims,
            blake: blake_opcode_result.claims,
            call: call_result.claims,
            call_rel_imm: call_rel_imm_result.claims,
            generic: generic_opcode_result.claims,
            jnz: jnz_result.claims,
            jnz_taken: jnz_taken_result.claims,
            jump: jump_result.claims,
            jump_double_deref: jump_double_deref_result.claims,
            jump_rel: jump_rel_result.claims,
            jump_rel_imm: jump_rel_imm_result.claims,
            mul: mul_result.claims,
            mul_small: mul_small_result.claims,
            qm31: qm31_result.claims,
            ret: ret_result.claims,
        };

        // Optional context claims (same semantics as your original)
        let blake_context_interaction_claim = blake_air::BlakeContextInteractionClaim {
            claim: if blake_round_claim.is_some() {
                Some(blake_air::InteractionClaim {
                    blake_round: blake_round_claim.unwrap(),
                    blake_g: blake_g_claim.unwrap(),
                    blake_sigma: blake_sigma_claim.unwrap(),
                    triple_xor_32: triple_xor_32_claim.unwrap(),
                    verify_bitwise_xor_12: verify_bitwise_xor_12_claim.unwrap(),
                })
            } else {
                None
            },
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
            claim: if pedersen_aggregator_claim.is_some() {
                Some(pedersen_air::InteractionClaim {
                    pedersen_aggregator: pedersen_aggregator_claim.unwrap(),
                    partial_ec_mul: partial_ec_mul_claim.unwrap(),
                    pedersen_points_table: pedersen_points_table_claim.unwrap(),
                })
            } else {
                None
            },
        };

        let poseidon_context_interaction_claim = poseidon_air::PoseidonContextInteractionClaim {
            claim: if poseidon_aggregator_claim.is_some() {
                Some(poseidon_air::InteractionClaim {
                    poseidon_aggregator: poseidon_aggregator_claim.unwrap(),
                    poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_claim.unwrap(),
                    poseidon_full_round_chain: poseidon_full_round_chain_claim.unwrap(),
                    cube_252: cube_252_claim.unwrap(),
                    poseidon_round_keys: poseidon_round_keys_claim.unwrap(),
                    range_check_252_width_27: range_check_252_width_27_claim.unwrap(),
                })
            } else {
                None
            },
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
            verify_bitwise_xor_4: verify_bitwise_xor_4_result.claim,
            verify_bitwise_xor_7: verify_bitwise_xor_7_result.claim,
            verify_bitwise_xor_8: verify_bitwise_xor_8_result.claim,
            verify_bitwise_xor_9: verify_bitwise_xor_9_result.claim,
        }
    }
}

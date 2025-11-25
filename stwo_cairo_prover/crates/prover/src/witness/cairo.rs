use std::array;

use cairo_air::air::{
    CairoClaim, CairoInteractionClaim, MemorySmallValue, PublicData, PublicMemory,
    PublicSegmentRanges, SegmentRange,
};
use cairo_air::cairo_interaction_elements::CairoInteractionElements;
use itertools::Itertools;
use stwo::core::fields::m31::M31;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::MAX_SEQUENCE_LOG_SIZE;
use tracing::{span, Level};

use super::blake_context::{BlakeContextClaimGenerator, BlakeContextInteractionClaimGenerator};
use super::builtins::{BuiltinsClaimGenerator, BuiltinsInteractionClaimGenerator};
use super::range_checks::{RangeChecksClaimGenerator, RangeChecksInteractionClaimGenerator};
use crate::witness::components::pedersen::{
    PedersenContextClaimGenerator, PedersenContextInteractionClaimGenerator,
};
use crate::witness::components::poseidon::{
    PoseidonContextClaimGenerator, PoseidonContextInteractionClaimGenerator,
};
use crate::witness::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode, ret_opcode,
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_8_b,
    verify_bitwise_xor_9, verify_instruction,
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

/// Responsible for generating the CairoClaim and writing the trace.
/// NOTE: Order of writing the trace is important, and should be consistent with [`CairoClaim`],
/// [`CairoInteractionClaim`], [`CairoComponents`], [`CairoInteractionElements`].
pub struct CairoClaimGenerator {
    public_data: PublicData,

    add: Option<add_opcode::ClaimGenerator>,
    add_small: Option<add_opcode_small::ClaimGenerator>,
    add_ap: Option<add_ap_opcode::ClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::ClaimGenerator>,
    assert_eq_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    blake: Option<blake_compress_opcode::ClaimGenerator>,
    call: Option<call_opcode_abs::ClaimGenerator>,
    call_rel_imm: Option<call_opcode_rel_imm::ClaimGenerator>,
    generic: Option<generic_opcode::ClaimGenerator>,
    jnz: Option<jnz_opcode_non_taken::ClaimGenerator>,
    jnz_taken: Option<jnz_opcode_taken::ClaimGenerator>,
    jump: Option<jump_opcode_abs::ClaimGenerator>,
    jump_double_deref: Option<jump_opcode_double_deref::ClaimGenerator>,
    jump_rel: Option<jump_opcode_rel::ClaimGenerator>,
    jump_rel_imm: Option<jump_opcode_rel_imm::ClaimGenerator>,
    mul: Option<mul_opcode::ClaimGenerator>,
    mul_small: Option<mul_opcode_small::ClaimGenerator>,
    qm31: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    ret: Option<ret_opcode::ClaimGenerator>,

    // Internal components.
    verify_instruction_trace_generator: verify_instruction::ClaimGenerator,
    blake_context_trace_generator: BlakeContextClaimGenerator,
    builtins: BuiltinsClaimGenerator,
    pedersen_context_trace_generator: PedersenContextClaimGenerator,
    poseidon_context_trace_generator: PoseidonContextClaimGenerator,
    memory_address_to_id_trace_generator: memory_address_to_id::ClaimGenerator,
    memory_id_to_value_trace_generator: memory_id_to_big::ClaimGenerator,
    range_checks_trace_generator: RangeChecksClaimGenerator,
    verify_bitwise_xor_4_trace_generator: verify_bitwise_xor_4::ClaimGenerator,
    verify_bitwise_xor_7_trace_generator: verify_bitwise_xor_7::ClaimGenerator,
    verify_bitwise_xor_8_trace_generator: verify_bitwise_xor_8::ClaimGenerator,
    verify_bitwise_xor_8_b_trace_generator: verify_bitwise_xor_8_b::ClaimGenerator,
    verify_bitwise_xor_9_trace_generator: verify_bitwise_xor_9::ClaimGenerator,
    // ...
}
impl CairoClaimGenerator {
    pub fn new(
        ProverInput {
            state_transitions,
            memory,
            public_memory_addresses,
            builtin_segments,
            public_segment_context,
            ..
        }: ProverInput,
    ) -> Self {
        let initial_state = state_transitions.initial_state;
        let final_state = state_transitions.final_state;

        let add = (!state_transitions
            .casm_states_by_opcode
            .add_opcode
            .is_empty())
        .then(|| {
            add_opcode::ClaimGenerator::new(state_transitions.casm_states_by_opcode.add_opcode)
        });
        let add_small = (!state_transitions
            .casm_states_by_opcode
            .add_opcode_small
            .is_empty())
        .then(|| {
            add_opcode_small::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.add_opcode_small,
            )
        });
        let add_ap = (!state_transitions
            .casm_states_by_opcode
            .add_ap_opcode
            .is_empty())
        .then(|| {
            add_ap_opcode::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.add_ap_opcode,
            )
        });
        let assert_eq = (!state_transitions
            .casm_states_by_opcode
            .assert_eq_opcode
            .is_empty())
        .then(|| {
            assert_eq_opcode::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.assert_eq_opcode,
            )
        });
        let assert_eq_imm = (!state_transitions
            .casm_states_by_opcode
            .assert_eq_opcode_imm
            .is_empty())
        .then(|| {
            assert_eq_opcode_imm::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.assert_eq_opcode_imm,
            )
        });
        let assert_eq_double_deref = (!state_transitions
            .casm_states_by_opcode
            .assert_eq_opcode_double_deref
            .is_empty())
        .then(|| {
            assert_eq_opcode_double_deref::ClaimGenerator::new(
                state_transitions
                    .casm_states_by_opcode
                    .assert_eq_opcode_double_deref,
            )
        });
        let blake = (!state_transitions
            .casm_states_by_opcode
            .blake_compress_opcode
            .is_empty())
        .then(|| {
            blake_compress_opcode::ClaimGenerator::new(
                state_transitions
                    .casm_states_by_opcode
                    .blake_compress_opcode,
            )
        });
        let call = (!state_transitions
            .casm_states_by_opcode
            .call_opcode_abs
            .is_empty())
        .then(|| {
            call_opcode_abs::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.call_opcode_abs,
            )
        });
        let call_rel_imm = (!state_transitions
            .casm_states_by_opcode
            .call_opcode_rel_imm
            .is_empty())
        .then(|| {
            call_opcode_rel_imm::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.call_opcode_rel_imm,
            )
        });
        let generic = (!state_transitions
            .casm_states_by_opcode
            .generic_opcode
            .is_empty())
        .then(|| {
            generic_opcode::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.generic_opcode,
            )
        });
        let jnz = (!state_transitions
            .casm_states_by_opcode
            .jnz_opcode_non_taken
            .is_empty())
        .then(|| {
            jnz_opcode_non_taken::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.jnz_opcode_non_taken,
            )
        });
        let jnz_taken = (!state_transitions
            .casm_states_by_opcode
            .jnz_opcode_taken
            .is_empty())
        .then(|| {
            jnz_opcode_taken::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.jnz_opcode_taken,
            )
        });
        let jump = (!state_transitions
            .casm_states_by_opcode
            .jump_opcode_abs
            .is_empty())
        .then(|| {
            jump_opcode_abs::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.jump_opcode_abs,
            )
        });
        let jump_double_deref = (!state_transitions
            .casm_states_by_opcode
            .jump_opcode_double_deref
            .is_empty())
        .then(|| {
            jump_opcode_double_deref::ClaimGenerator::new(
                state_transitions
                    .casm_states_by_opcode
                    .jump_opcode_double_deref,
            )
        });
        let jump_rel = (!state_transitions
            .casm_states_by_opcode
            .jump_opcode_rel
            .is_empty())
        .then(|| {
            jump_opcode_rel::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.jump_opcode_rel,
            )
        });
        let jump_rel_imm = (!state_transitions
            .casm_states_by_opcode
            .jump_opcode_rel_imm
            .is_empty())
        .then(|| {
            jump_opcode_rel_imm::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.jump_opcode_rel_imm,
            )
        });
        let mul = (!state_transitions
            .casm_states_by_opcode
            .mul_opcode
            .is_empty())
        .then(|| {
            mul_opcode::ClaimGenerator::new(state_transitions.casm_states_by_opcode.mul_opcode)
        });
        let mul_small = (!state_transitions
            .casm_states_by_opcode
            .mul_opcode_small
            .is_empty())
        .then(|| {
            mul_opcode_small::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.mul_opcode_small,
            )
        });
        let qm31 = (!state_transitions
            .casm_states_by_opcode
            .qm_31_add_mul_opcode
            .is_empty())
        .then(|| {
            qm_31_add_mul_opcode::ClaimGenerator::new(
                state_transitions.casm_states_by_opcode.qm_31_add_mul_opcode,
            )
        });
        let ret = (!state_transitions
            .casm_states_by_opcode
            .ret_opcode
            .is_empty())
        .then(|| {
            ret_opcode::ClaimGenerator::new(state_transitions.casm_states_by_opcode.ret_opcode)
        });
        let verify_instruction_trace_generator = verify_instruction::ClaimGenerator::new();
        let builtins = BuiltinsClaimGenerator::new(builtin_segments);
        let pedersen_context_trace_generator = PedersenContextClaimGenerator::new();
        let poseidon_context_trace_generator = PoseidonContextClaimGenerator::new();
        let memory_address_to_id_trace_generator =
            memory_address_to_id::ClaimGenerator::new(&memory);
        let memory_id_to_value_trace_generator = memory_id_to_big::ClaimGenerator::new(&memory);
        let range_checks_trace_generator = RangeChecksClaimGenerator::new();
        let verify_bitwise_xor_4_trace_generator = verify_bitwise_xor_4::ClaimGenerator::new();
        let verify_bitwise_xor_7_trace_generator = verify_bitwise_xor_7::ClaimGenerator::new();
        let verify_bitwise_xor_8_trace_generator = verify_bitwise_xor_8::ClaimGenerator::new();
        let verify_bitwise_xor_8_b_trace_generator = verify_bitwise_xor_8_b::ClaimGenerator::new();
        let verify_bitwise_xor_9_trace_generator = verify_bitwise_xor_9::ClaimGenerator::new();

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

        let blake_context_trace_generator = BlakeContextClaimGenerator::new(memory);

        Self {
            public_data,
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
            generic,
            jnz,
            jnz_taken,
            jump,
            jump_double_deref,
            jump_rel,
            jump_rel_imm,
            mul,
            mul_small,
            qm31,
            ret,
            verify_instruction_trace_generator,
            blake_context_trace_generator,
            builtins,
            pedersen_context_trace_generator,
            poseidon_context_trace_generator,
            memory_address_to_id_trace_generator,
            memory_id_to_value_trace_generator,
            range_checks_trace_generator,
            verify_bitwise_xor_4_trace_generator,
            verify_bitwise_xor_7_trace_generator,
            verify_bitwise_xor_8_trace_generator,
            verify_bitwise_xor_8_b_trace_generator,
            verify_bitwise_xor_9_trace_generator,
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (CairoClaim, CairoInteractionClaimGenerator) {
        let span = span!(Level::INFO, "write opcode trace").entered();
        let (add, add_interaction_gens) = self
            .add
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_small, add_small_interaction_gens) = self
            .add_small
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (add_ap, add_ap_interaction_gens) = self
            .add_ap
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                    &self.range_checks_trace_generator.rc_18_trace_generator,
                    &self.range_checks_trace_generator.rc_11_trace_generator,
                )
            })
            .unzip();
        let (assert_eq, assert_eq_interaction_gens) = self
            .assert_eq
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (assert_eq_imm, assert_eq_imm_interaction_gens) = self
            .assert_eq_imm
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (assert_eq_double_deref, assert_eq_double_deref_interaction_gens) = self
            .assert_eq_double_deref
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (blake, blake_interaction_gens) = self
            .blake
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                    &self.range_checks_trace_generator.rc_7_2_5_trace_generator,
                    &self.verify_bitwise_xor_8_trace_generator,
                    &mut self.blake_context_trace_generator.blake_round,
                    &mut self.blake_context_trace_generator.triple_xor_32,
                )
            })
            .unzip();
        let (call, call_interaction_gens) = self
            .call
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (call_rel_imm, call_rel_imm_interaction_gens) = self
            .call_rel_imm
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (generic, generic_opcode_interaction_gens) = self
            .generic
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_b_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_c_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_d_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_e_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_f_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_g_trace_generator,
                    &self.range_checks_trace_generator.rc_9_9_h_trace_generator,
                    &self.range_checks_trace_generator.rc_20_trace_generator,
                    &self.range_checks_trace_generator.rc_20_b_trace_generator,
                    &self.range_checks_trace_generator.rc_20_c_trace_generator,
                    &self.range_checks_trace_generator.rc_20_d_trace_generator,
                    &self.range_checks_trace_generator.rc_20_e_trace_generator,
                    &self.range_checks_trace_generator.rc_20_f_trace_generator,
                    &self.range_checks_trace_generator.rc_20_g_trace_generator,
                    &self.range_checks_trace_generator.rc_20_h_trace_generator,
                    &self.range_checks_trace_generator.rc_18_trace_generator,
                    &self.range_checks_trace_generator.rc_11_trace_generator,
                )
            })
            .unzip();
        let (jnz, jnz_interaction_gens) = self
            .jnz
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jnz_taken, jnz_taken_interaction_gens) = self
            .jnz_taken
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump, jump_interaction_gens) = self
            .jump
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_double_deref, jump_double_deref_interaction_gens) = self
            .jump_double_deref
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_rel, jump_rel_interaction_gens) = self
            .jump_rel
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (jump_rel_imm, jump_rel_imm_interaction_gens) = self
            .jump_rel_imm
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        let (mul, mul_interaction_gens) = self
            .mul
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                    &self.range_checks_trace_generator.rc_20_trace_generator,
                    &self.range_checks_trace_generator.rc_20_b_trace_generator,
                    &self.range_checks_trace_generator.rc_20_c_trace_generator,
                    &self.range_checks_trace_generator.rc_20_d_trace_generator,
                    &self.range_checks_trace_generator.rc_20_e_trace_generator,
                    &self.range_checks_trace_generator.rc_20_f_trace_generator,
                    &self.range_checks_trace_generator.rc_20_g_trace_generator,
                    &self.range_checks_trace_generator.rc_20_h_trace_generator,
                )
            })
            .unzip();
        let (mul_small, mul_small_interaction_gens) = self
            .mul_small
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                    &self.range_checks_trace_generator.rc_11_trace_generator,
                )
            })
            .unzip();
        let (qm31, qm31_interaction_gens) = self
            .qm31
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                    &self.range_checks_trace_generator.rc_4_4_4_4_trace_generator,
                )
            })
            .unzip();
        let (ret, ret_interaction_gens) = self
            .ret
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_instruction_trace_generator,
                )
            })
            .unzip();
        span.exit();
        let span = span!(Level::INFO, "internal component trace").entered();
        let (verify_instruction_claim, verify_instruction_interaction_gen) =
            self.verify_instruction_trace_generator.write_trace(
                tree_builder,
                &self.range_checks_trace_generator.rc_7_2_5_trace_generator,
                &self.range_checks_trace_generator.rc_4_3_trace_generator,
                &self.memory_address_to_id_trace_generator,
                &self.memory_id_to_value_trace_generator,
            );
        let (blake_context_claim, blake_context_interaction_gen) =
            self.blake_context_trace_generator.write_trace(
                tree_builder,
                &self.memory_address_to_id_trace_generator,
                &self.memory_id_to_value_trace_generator,
                &self.range_checks_trace_generator,
                &self.verify_bitwise_xor_4_trace_generator,
                &self.verify_bitwise_xor_7_trace_generator,
                &self.verify_bitwise_xor_8_trace_generator,
                &self.verify_bitwise_xor_8_b_trace_generator,
                &self.verify_bitwise_xor_9_trace_generator,
            );
        let (builtins_claim, builtins_interaction_gen) = self.builtins.write_trace(
            tree_builder,
            &self.memory_address_to_id_trace_generator,
            &self.memory_id_to_value_trace_generator,
            &mut self.pedersen_context_trace_generator,
            &self.range_checks_trace_generator.rc_5_4_trace_generator,
            &self.range_checks_trace_generator.rc_8_trace_generator,
            &mut self.poseidon_context_trace_generator,
            &self.range_checks_trace_generator.rc_6_trace_generator,
            &self.range_checks_trace_generator.rc_12_trace_generator,
            &self.range_checks_trace_generator.rc_18_trace_generator,
            &self.range_checks_trace_generator.rc_3_6_6_3_trace_generator,
            &self.verify_bitwise_xor_8_trace_generator,
            &self.verify_bitwise_xor_9_trace_generator,
        );
        let (pedersen_context_claim, pedersen_context_interaction_gen) = self
            .pedersen_context_trace_generator
            .write_trace(tree_builder, &self.range_checks_trace_generator);
        let (poseidon_context_claim, poseidon_context_interaction_gen) =
            self.poseidon_context_trace_generator.write_trace(
                tree_builder,
                &self.memory_id_to_value_trace_generator,
                &self.range_checks_trace_generator,
            );
        let (memory_address_to_id_claim, memory_address_to_id_interaction_gen) = self
            .memory_address_to_id_trace_generator
            .write_trace(tree_builder);

        // Memory uses "Sequence", split it according to `MAX_SEQUENCE_LOG_SIZE`.
        const LOG_MAX_BIG_SIZE: u32 = MAX_SEQUENCE_LOG_SIZE;
        let (memory_id_to_value_claim, memory_id_to_value_interaction_gen) =
            self.memory_id_to_value_trace_generator.write_trace(
                tree_builder,
                &self.range_checks_trace_generator.rc_9_9_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_b_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_c_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_d_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_e_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_f_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_g_trace_generator,
                &self.range_checks_trace_generator.rc_9_9_h_trace_generator,
                LOG_MAX_BIG_SIZE,
            );
        let (range_checks_claim, range_checks_interaction_gen) =
            self.range_checks_trace_generator.write_trace(tree_builder);
        let (verify_bitwise_xor_4_claim, verify_bitwise_xor_4_interaction_gen) = self
            .verify_bitwise_xor_4_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_7_claim, verify_bitwise_xor_7_interaction_gen) = self
            .verify_bitwise_xor_7_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_8_claim, verify_bitwise_xor_8_interaction_gen) = self
            .verify_bitwise_xor_8_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_8_b_claim, verify_bitwise_xor_8_b_interaction_gen) = self
            .verify_bitwise_xor_8_b_trace_generator
            .write_trace(tree_builder);
        let (verify_bitwise_xor_9_claim, verify_bitwise_xor_9_interaction_gen) = self
            .verify_bitwise_xor_9_trace_generator
            .write_trace(tree_builder);
        span.exit();
        (
            CairoClaim {
                public_data: self.public_data,
                add,
                add_small,
                add_ap,
                assert_eq,
                assert_eq_imm,
                assert_eq_double_deref,
                blake,
                call,
                call_rel_imm,
                generic,
                jnz,
                jnz_taken,
                jump,
                jump_double_deref,
                jump_rel,
                jump_rel_imm,
                mul,
                mul_small,
                qm31,
                ret,
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
                verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_claim,
                verify_bitwise_xor_9: verify_bitwise_xor_9_claim,
            },
            CairoInteractionClaimGenerator {
                add: add_interaction_gens,
                add_small: add_small_interaction_gens,
                add_ap: add_ap_interaction_gens,
                assert_eq: assert_eq_interaction_gens,
                assert_eq_imm: assert_eq_imm_interaction_gens,
                assert_eq_double_deref: assert_eq_double_deref_interaction_gens,
                blake: blake_interaction_gens,
                call: call_interaction_gens,
                call_rel_imm: call_rel_imm_interaction_gens,
                generic_opcode_interaction_gens,
                jnz: jnz_interaction_gens,
                jnz_taken: jnz_taken_interaction_gens,
                jump: jump_interaction_gens,
                jump_double_deref: jump_double_deref_interaction_gens,
                jump_rel: jump_rel_interaction_gens,
                jump_rel_imm: jump_rel_imm_interaction_gens,
                mul: mul_interaction_gens,
                mul_small: mul_small_interaction_gens,
                qm31: qm31_interaction_gens,
                ret_interaction_gens,
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
                verify_bitwise_xor_8_b_interaction_gen,
                verify_bitwise_xor_9_interaction_gen,
            },
        )
    }
}

pub struct CairoInteractionClaimGenerator {
    add: Option<add_opcode::InteractionClaimGenerator>,
    add_small: Option<add_opcode_small::InteractionClaimGenerator>,
    add_ap: Option<add_ap_opcode::InteractionClaimGenerator>,
    assert_eq: Option<assert_eq_opcode::InteractionClaimGenerator>,
    assert_eq_imm: Option<assert_eq_opcode_imm::InteractionClaimGenerator>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::InteractionClaimGenerator>,
    blake: Option<blake_compress_opcode::InteractionClaimGenerator>,
    call: Option<call_opcode_abs::InteractionClaimGenerator>,
    call_rel_imm: Option<call_opcode_rel_imm::InteractionClaimGenerator>,
    generic_opcode_interaction_gens: Option<generic_opcode::InteractionClaimGenerator>,
    jnz: Option<jnz_opcode_non_taken::InteractionClaimGenerator>,
    jnz_taken: Option<jnz_opcode_taken::InteractionClaimGenerator>,
    jump: Option<jump_opcode_abs::InteractionClaimGenerator>,
    jump_double_deref: Option<jump_opcode_double_deref::InteractionClaimGenerator>,
    jump_rel: Option<jump_opcode_rel::InteractionClaimGenerator>,
    jump_rel_imm: Option<jump_opcode_rel_imm::InteractionClaimGenerator>,
    mul: Option<mul_opcode::InteractionClaimGenerator>,
    mul_small: Option<mul_opcode_small::InteractionClaimGenerator>,
    qm31: Option<qm_31_add_mul_opcode::InteractionClaimGenerator>,
    ret_interaction_gens: Option<ret_opcode::InteractionClaimGenerator>,
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
    verify_bitwise_xor_8_b_interaction_gen: verify_bitwise_xor_8_b::InteractionClaimGenerator,
    verify_bitwise_xor_9_interaction_gen: verify_bitwise_xor_9::InteractionClaimGenerator,
    // ...
}
impl CairoInteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        interaction_elements: &CairoInteractionElements,
    ) -> CairoInteractionClaim {
        let add = self.add.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let add_small = self.add_small.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let add_ap = self.add_ap.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_18,
                &interaction_elements.range_checks.rc_11,
                &interaction_elements.opcodes,
            )
        });
        let assert_eq = self.assert_eq.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.opcodes,
            )
        });
        let assert_eq_imm = self.assert_eq_imm.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.opcodes,
            )
        });
        let assert_eq_double_deref = self.assert_eq_double_deref.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let blake = self.blake.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_7_2_5,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.blake_round,
                &interaction_elements.triple_xor_32,
                &interaction_elements.opcodes,
            )
        });
        let call = self.call.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let call_rel_imm = self.call_rel_imm.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let generic = self.generic_opcode_interaction_gens.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_9_9,
                &interaction_elements.range_checks.rc_9_9_b,
                &interaction_elements.range_checks.rc_9_9_c,
                &interaction_elements.range_checks.rc_9_9_d,
                &interaction_elements.range_checks.rc_9_9_e,
                &interaction_elements.range_checks.rc_9_9_f,
                &interaction_elements.range_checks.rc_9_9_g,
                &interaction_elements.range_checks.rc_9_9_h,
                &interaction_elements.range_checks.rc_20,
                &interaction_elements.range_checks.rc_20_b,
                &interaction_elements.range_checks.rc_20_c,
                &interaction_elements.range_checks.rc_20_d,
                &interaction_elements.range_checks.rc_20_e,
                &interaction_elements.range_checks.rc_20_f,
                &interaction_elements.range_checks.rc_20_g,
                &interaction_elements.range_checks.rc_20_h,
                &interaction_elements.range_checks.rc_18,
                &interaction_elements.range_checks.rc_11,
                &interaction_elements.opcodes,
            )
        });
        let jnz = self.jnz.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let jnz_taken = self.jnz_taken.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let jump = self.jump.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let jump_double_deref = self.jump_double_deref.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let jump_rel = self.jump_rel.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let jump_rel_imm = self.jump_rel_imm.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let mul = self.mul.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_20,
                &interaction_elements.range_checks.rc_20_b,
                &interaction_elements.range_checks.rc_20_c,
                &interaction_elements.range_checks.rc_20_d,
                &interaction_elements.range_checks.rc_20_e,
                &interaction_elements.range_checks.rc_20_f,
                &interaction_elements.range_checks.rc_20_g,
                &interaction_elements.range_checks.rc_20_h,
                &interaction_elements.opcodes,
            )
        });
        let mul_small = self.mul_small.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_11,
                &interaction_elements.opcodes,
            )
        });
        let qm31 = self.qm31.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_4_4_4_4,
                &interaction_elements.opcodes,
            )
        });
        let ret = self.ret_interaction_gens.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_instruction,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.opcodes,
            )
        });
        let verify_instruction_interaction_claim = self
            .verify_instruction_interaction_gen
            .write_interaction_trace(
                tree_builder,
                &interaction_elements.range_checks.rc_7_2_5,
                &interaction_elements.range_checks.rc_4_3,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.verify_instruction,
            );
        let blake_context_interaction_claim = self
            .blake_context_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let builtins_interaction_claims = self
            .builtins_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let pedersen_context_interaction_claim = self
            .pedersen_context_interaction_gen
            .write_interaction_trace(tree_builder, interaction_elements);
        let poseidon_context_interaction_claim = self
            .poseidon_context_interaction_gen
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
                &interaction_elements.range_checks.rc_9_9_b,
                &interaction_elements.range_checks.rc_9_9_c,
                &interaction_elements.range_checks.rc_9_9_d,
                &interaction_elements.range_checks.rc_9_9_e,
                &interaction_elements.range_checks.rc_9_9_f,
                &interaction_elements.range_checks.rc_9_9_g,
                &interaction_elements.range_checks.rc_9_9_h,
            );

        let range_checks_interaction_claim = self
            .range_checks_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.range_checks);
        let verify_bitwise_xor_4_interaction_claim = self
            .verify_bitwise_xor_4_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_4);
        let verify_bitwise_xor_7_interaction_claim = self
            .verify_bitwise_xor_7_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_7);
        let verify_bitwise_xor_8_interaction_claim = self
            .verify_bitwise_xor_8_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_8);
        let verify_bitwise_xor_8_b_interaction_claim = self
            .verify_bitwise_xor_8_b_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_8_b);
        let verify_bitwise_xor_9_interaction_claim = self
            .verify_bitwise_xor_9_interaction_gen
            .write_interaction_trace(tree_builder, &interaction_elements.verify_bitwise_xor_9);

        CairoInteractionClaim {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
            generic,
            jnz,
            jnz_taken,
            jump,
            jump_double_deref,
            jump_rel,
            jump_rel_imm,
            mul,
            mul_small,
            qm31,
            ret,
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
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_interaction_claim,
            verify_bitwise_xor_9: verify_bitwise_xor_9_interaction_claim,
        }
    }
}

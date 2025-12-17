use cairo_air::air::{CairoClaim, CairoInteractionClaim};
use cairo_air::cairo_interaction_elements::CairoInteractionElements;
use cairo_air::public_data::PublicData;
use stwo::core::fields::m31::M31;
use stwo::prover::backend::simd::SimdBackend;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_common::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::MAX_SEQUENCE_LOG_SIZE;
use tracing::{span, Level};

use super::components::{
    add_mod_builtin, bitwise_builtin, mul_mod_builtin, pedersen_builtin, poseidon_builtin,
    range_check_builtin_bits_128, range_check_builtin_bits_96,
};
use super::public_data::create_public_data;
use super::range_checks::{RangeChecksClaimGenerator, RangeChecksInteractionClaimGenerator};
use crate::witness::components::pedersen::{
    PedersenContextClaimGenerator, PedersenContextInteractionClaimGenerator,
};
use crate::witness::components::poseidon::{
    PoseidonContextClaimGenerator, PoseidonContextInteractionClaimGenerator,
};
use crate::witness::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, blake_g, blake_round, blake_round_sigma,
    call_opcode_abs, call_opcode_rel_imm, generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken,
    jump_opcode_abs, jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm,
    memory_address_to_id, memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode,
    ret_opcode, triple_xor_32, verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_8_b, verify_bitwise_xor_9, verify_instruction,
};
use crate::witness::utils::TreeBuilder;

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
    blake_round_trace_generator: Option<blake_round::ClaimGenerator>,
    blake_g_trace_generator: Option<blake_g::ClaimGenerator>,
    blake_sigma_trace_generator: Option<blake_round_sigma::ClaimGenerator>,
    triple_xor_32_trace_generator: Option<triple_xor_32::ClaimGenerator>,
    verify_bitwise_xor_12_trace_generator: Option<verify_bitwise_xor_12::ClaimGenerator>,
    add_mod_builtin_trace_generator: Option<add_mod_builtin::ClaimGenerator>,
    bitwise_builtin_trace_generator: Option<bitwise_builtin::ClaimGenerator>,
    mul_mod_builtin_trace_generator: Option<mul_mod_builtin::ClaimGenerator>,
    pedersen_builtin_trace_generator: Option<pedersen_builtin::ClaimGenerator>,
    poseidon_builtin_trace_generator: Option<poseidon_builtin::ClaimGenerator>,
    range_check_96_builtin_trace_generator: Option<range_check_builtin_bits_96::ClaimGenerator>,
    range_check_128_builtin_trace_generator: Option<range_check_builtin_bits_128::ClaimGenerator>,
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
        let add_mod_builtin_trace_generator = builtin_segments.add_mod.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(ADD_MOD_MEMORY_CELLS),
                "add mod segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / ADD_MOD_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "add mod instances number is not a power of two"
            );
            add_mod_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let bitwise_builtin_trace_generator = builtin_segments.bitwise.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(BITWISE_MEMORY_CELLS),
                "bitwise segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / BITWISE_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "bitwise instances number is not a power of two"
            );
            bitwise_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let mul_mod_builtin_trace_generator = builtin_segments.mul_mod.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(MUL_MOD_MEMORY_CELLS),
                "mul mod segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / MUL_MOD_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "mul mod instances number is not a power of two"
            );
            mul_mod_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let pedersen_builtin_trace_generator = builtin_segments.pedersen.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(PEDERSEN_MEMORY_CELLS),
                "pedersen segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / PEDERSEN_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "pedersen instances number is not a power of two"
            );
            pedersen_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let poseidon_builtin_trace_generator = builtin_segments.poseidon.map(|segment| {
            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(POSEIDON_MEMORY_CELLS),
                "poseidon segment length is not a multiple of it's cells_per_instance"
            );
            let n_instances = segment_length / POSEIDON_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "poseidon instances number is not a power of two"
            );
            poseidon_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
        });
        let range_check_96_builtin_trace_generator =
            builtin_segments.range_check_bits_96.map(|segment| {
                let segment_length = segment.stop_ptr - segment.begin_addr;
                let n_instances = segment_length / RANGE_CHECK_MEMORY_CELLS;
                assert!(
                    n_instances.is_power_of_two(),
                    "range_check_bits_96 instances number is not a power of two"
                );
                range_check_builtin_bits_96::ClaimGenerator::new(
                    n_instances.ilog2(),
                    segment.begin_addr as u32,
                )
            });
        let range_check_128_builtin_trace_generator =
            builtin_segments.range_check_bits_128.map(|segment| {
                let segment_length = segment.stop_ptr - segment.begin_addr;
                let n_instances = segment_length / RANGE_CHECK_MEMORY_CELLS;
                assert!(
                    n_instances.is_power_of_two(),
                    "range_check_bits_128 instances number is not a power of two"
                );
                range_check_builtin_bits_128::ClaimGenerator::new(
                    n_instances.ilog2(),
                    segment.begin_addr as u32,
                )
            });
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
        let public_data =
            create_public_data(&memory, initial_state, final_state, public_segment_context);

        let (
            blake_round_trace_generator,
            blake_g_trace_generator,
            blake_sigma_trace_generator,
            triple_xor_32_trace_generator,
            verify_bitwise_xor_12_trace_generator,
        ) = if blake.is_some() {
            (
                Some(blake_round::ClaimGenerator::new(memory)),
                Some(blake_g::ClaimGenerator::new()),
                Some(blake_round_sigma::ClaimGenerator::new()),
                Some(triple_xor_32::ClaimGenerator::new()),
                Some(verify_bitwise_xor_12::ClaimGenerator::new()),
            )
        } else {
            (None, None, None, None, None)
        };

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
            blake_round_trace_generator,
            blake_g_trace_generator,
            blake_sigma_trace_generator,
            triple_xor_32_trace_generator,
            verify_bitwise_xor_12_trace_generator,
            add_mod_builtin_trace_generator,
            bitwise_builtin_trace_generator,
            mul_mod_builtin_trace_generator,
            pedersen_builtin_trace_generator,
            poseidon_builtin_trace_generator,
            range_check_96_builtin_trace_generator,
            range_check_128_builtin_trace_generator,
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
                    self.blake_round_trace_generator.as_mut().unwrap(),
                    self.triple_xor_32_trace_generator.as_mut().unwrap(),
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
        let (blake_round_claim, blake_round_interaction_gen) = self
            .blake_round_trace_generator
            .map(|gen| {
                let (claim, interaction_claim_generator) = gen.write_trace(
                    tree_builder,
                    self.blake_g_trace_generator.as_mut().unwrap(),
                    self.blake_sigma_trace_generator.as_ref().unwrap(),
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.range_checks_trace_generator.rc_7_2_5_trace_generator,
                );
                (claim, interaction_claim_generator)
            })
            .unzip();

        let (blake_g_claim, blake_g_interaction_gen) = self
            .blake_g_trace_generator
            .map(|gen| {
                let (claim, interaction_claim_generator) = gen.write_trace(
                    tree_builder,
                    &self.verify_bitwise_xor_8_trace_generator,
                    &self.verify_bitwise_xor_8_b_trace_generator,
                    self.verify_bitwise_xor_12_trace_generator.as_ref().unwrap(),
                    &self.verify_bitwise_xor_4_trace_generator,
                    &self.verify_bitwise_xor_7_trace_generator,
                    &self.verify_bitwise_xor_9_trace_generator,
                );
                (claim, interaction_claim_generator)
            })
            .unzip();

        let (blake_sigma_claim, blake_sigma_interaction_gen) = self
            .blake_sigma_trace_generator
            .map(|gen| gen.write_trace(tree_builder))
            .unzip();

        let (triple_xor_32_claim, triple_xor_32_interaction_gen) = self
            .triple_xor_32_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.verify_bitwise_xor_8_trace_generator,
                    &self.verify_bitwise_xor_8_b_trace_generator,
                )
            })
            .unzip();

        let (verify_bitwise_xor_12_claim, verify_bitwise_xor_12_interaction_gen) = self
            .verify_bitwise_xor_12_trace_generator
            .map(|gen| gen.write_trace(tree_builder))
            .unzip();

        let (add_mod_builtin_claim, add_mod_builtin_interaction_gen) = self
            .add_mod_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                )
            })
            .unzip();

        let (bitwise_builtin_claim, bitwise_builtin_interaction_gen) = self
            .bitwise_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.verify_bitwise_xor_9_trace_generator,
                    &self.verify_bitwise_xor_8_trace_generator,
                )
            })
            .unzip();

        let (mul_mod_builtin_claim, mul_mod_builtin_interaction_gen) = self
            .mul_mod_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.range_checks_trace_generator.rc_12_trace_generator,
                    &self.range_checks_trace_generator.rc_3_6_6_3_trace_generator,
                    &self.range_checks_trace_generator.rc_18_trace_generator,
                )
            })
            .unzip();

        let (pedersen_builtin_claim, pedersen_builtin_interaction_gen) = self
            .pedersen_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.range_checks_trace_generator.rc_5_4_trace_generator,
                    &self.range_checks_trace_generator.rc_8_trace_generator,
                    &self
                        .pedersen_context_trace_generator
                        .pedersen_points_table_trace_generator,
                    &mut self
                        .pedersen_context_trace_generator
                        .partial_ec_mul_trace_generator,
                )
            })
            .unzip();

        let (poseidon_builtin_claim, poseidon_builtin_interaction_gen) = self
            .poseidon_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self
                        .poseidon_context_trace_generator
                        .poseidon_aggregator_trace_generator,
                )
            })
            .unzip();

        let (range_check_96_builtin_claim, range_check_96_builtin_interaction_gen) = self
            .range_check_96_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                    &self.range_checks_trace_generator.rc_6_trace_generator,
                )
            })
            .unzip();

        let (range_check_128_builtin_claim, range_check_128_builtin_interaction_gen) = self
            .range_check_128_builtin_trace_generator
            .map(|gen| {
                gen.write_trace(
                    tree_builder,
                    &self.memory_address_to_id_trace_generator,
                    &self.memory_id_to_value_trace_generator,
                )
            })
            .unzip();
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
                blake_round: blake_round_claim,
                blake_g: blake_g_claim,
                blake_sigma: blake_sigma_claim,
                triple_xor_32: triple_xor_32_claim,
                verify_bitwise_xor_12: verify_bitwise_xor_12_claim,
                add_mod_builtin: add_mod_builtin_claim,
                bitwise_builtin: bitwise_builtin_claim,
                mul_mod_builtin: mul_mod_builtin_claim,
                pedersen_builtin: pedersen_builtin_claim,
                poseidon_builtin: poseidon_builtin_claim,
                range_check_96_builtin: range_check_96_builtin_claim,
                range_check_128_builtin: range_check_128_builtin_claim,
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
                blake_round_interaction_gen,
                blake_g_interaction_gen,
                blake_sigma_interaction_gen,
                triple_xor_32_interaction_gen,
                verify_bitwise_xor_12_interaction_gen,
                add_mod_builtin_interaction_gen,
                bitwise_builtin_interaction_gen,
                mul_mod_builtin_interaction_gen,
                pedersen_builtin_interaction_gen,
                poseidon_builtin_interaction_gen,
                range_check_96_builtin_interaction_gen,
                range_check_128_builtin_interaction_gen,
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
    blake_round_interaction_gen: Option<blake_round::InteractionClaimGenerator>,
    blake_g_interaction_gen: Option<blake_g::InteractionClaimGenerator>,
    blake_sigma_interaction_gen: Option<blake_round_sigma::InteractionClaimGenerator>,
    triple_xor_32_interaction_gen: Option<triple_xor_32::InteractionClaimGenerator>,
    verify_bitwise_xor_12_interaction_gen: Option<verify_bitwise_xor_12::InteractionClaimGenerator>,
    add_mod_builtin_interaction_gen: Option<add_mod_builtin::InteractionClaimGenerator>,
    bitwise_builtin_interaction_gen: Option<bitwise_builtin::InteractionClaimGenerator>,
    mul_mod_builtin_interaction_gen: Option<mul_mod_builtin::InteractionClaimGenerator>,
    pedersen_builtin_interaction_gen: Option<pedersen_builtin::InteractionClaimGenerator>,
    poseidon_builtin_interaction_gen: Option<poseidon_builtin::InteractionClaimGenerator>,
    range_check_96_builtin_interaction_gen:
        Option<range_check_builtin_bits_96::InteractionClaimGenerator>,
    range_check_128_builtin_interaction_gen:
        Option<range_check_builtin_bits_128::InteractionClaimGenerator>,
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
        let blake_round_interaction_claim = self.blake_round_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.blake_g,
                &interaction_elements.blake_round,
                &interaction_elements.blake_sigma,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_7_2_5,
            )
        });
        let blake_g_interaction_claim = self.blake_g_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.verify_bitwise_xor_8_b,
                &interaction_elements.verify_bitwise_xor_12,
                &interaction_elements.verify_bitwise_xor_4,
                &interaction_elements.verify_bitwise_xor_7,
                &interaction_elements.verify_bitwise_xor_9,
                &interaction_elements.blake_g,
            )
        });
        let blake_sigma_interaction_claim = self.blake_sigma_interaction_gen.map(|gen| {
            gen.write_interaction_trace(tree_builder, &interaction_elements.blake_sigma)
        });
        let triple_xor_32_interaction_claim = self.triple_xor_32_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.verify_bitwise_xor_8,
                &interaction_elements.verify_bitwise_xor_8_b,
                &interaction_elements.triple_xor_32,
            )
        });
        let verify_bitwise_xor_12_interaction_claim =
            self.verify_bitwise_xor_12_interaction_gen.map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.verify_bitwise_xor_12,
                )
            });
        let add_mod_builtin_interaction_claim = self.add_mod_builtin_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
            )
        });
        let bitwise_builtin_interaction_claim = self.bitwise_builtin_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.verify_bitwise_xor_9,
                &interaction_elements.verify_bitwise_xor_8,
            )
        });
        let mul_mod_builtin_interaction_claim = self.mul_mod_builtin_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_12,
                &interaction_elements.range_checks.rc_3_6_6_3,
                &interaction_elements.range_checks.rc_18,
            )
        });
        let pedersen_builtin_interaction_claim = self.pedersen_builtin_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.range_checks.rc_5_4,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.memory_id_to_value,
                &interaction_elements.range_checks.rc_8,
                &interaction_elements.pedersen_points_table,
                &interaction_elements.partial_ec_mul,
            )
        });
        let poseidon_builtin_interaction_claim = self.poseidon_builtin_interaction_gen.map(|gen| {
            gen.write_interaction_trace(
                tree_builder,
                &interaction_elements.memory_address_to_id,
                &interaction_elements.poseidon_aggregator,
            )
        });
        let range_check_96_builtin_interaction_claim =
            self.range_check_96_builtin_interaction_gen.map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.range_checks.rc_6,
                    &interaction_elements.memory_id_to_value,
                )
            });
        let range_check_128_builtin_interaction_claim =
            self.range_check_128_builtin_interaction_gen.map(|gen| {
                gen.write_interaction_trace(
                    tree_builder,
                    &interaction_elements.memory_address_to_id,
                    &interaction_elements.memory_id_to_value,
                )
            });
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
            blake_round: blake_round_interaction_claim,
            blake_g: blake_g_interaction_claim,
            blake_sigma: blake_sigma_interaction_claim,
            triple_xor_32: triple_xor_32_interaction_claim,
            verify_bitwise_xor_12: verify_bitwise_xor_12_interaction_claim,
            add_mod_builtin: add_mod_builtin_interaction_claim,
            bitwise_builtin: bitwise_builtin_interaction_claim,
            mul_mod_builtin: mul_mod_builtin_interaction_claim,
            pedersen_builtin: pedersen_builtin_interaction_claim,
            poseidon_builtin: poseidon_builtin_interaction_claim,
            range_check_96_builtin: range_check_96_builtin_interaction_claim,
            range_check_128_builtin: range_check_128_builtin_interaction_claim,
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

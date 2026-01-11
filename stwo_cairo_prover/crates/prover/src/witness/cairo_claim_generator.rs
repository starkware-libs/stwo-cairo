// This file was created by the AIR team.

use std::sync::Arc;

use cairo_air::air::PublicData;
use indexmap::IndexSet;
use rayon::scope;
use stwo_cairo_adapter::builtins::BuiltinSegments;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;
use stwo_cairo_common::builtins::*;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;

use crate::witness::components::*;

#[derive(Default)]
pub struct CairoClaimGenerator {
    pub public_data: PublicData,
    pub blake_compress_opcode: Option<blake_compress_opcode::ClaimGenerator>,
    pub triple_xor_32: Option<triple_xor_32::ClaimGenerator>,
    pub blake_round: Option<blake_round::ClaimGenerator>,
    pub blake_g: Option<blake_g::ClaimGenerator>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::ClaimGenerator>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::ClaimGenerator>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::ClaimGenerator>,
    pub blake_round_sigma: Option<blake_round_sigma::ClaimGenerator>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    pub ret_opcode: Option<ret_opcode::ClaimGenerator>,
    pub mul_opcode: Option<mul_opcode::ClaimGenerator>,
    pub mul_opcode_small: Option<mul_opcode_small::ClaimGenerator>,
    pub jump_opcode_abs: Option<jump_opcode_abs::ClaimGenerator>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::ClaimGenerator>,
    pub jump_opcode_rel: Option<jump_opcode_rel::ClaimGenerator>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::ClaimGenerator>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::ClaimGenerator>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::ClaimGenerator>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::ClaimGenerator>,
    pub call_opcode_abs: Option<call_opcode_abs::ClaimGenerator>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::ClaimGenerator>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::ClaimGenerator>,
    pub assert_eq_opcode: Option<assert_eq_opcode::ClaimGenerator>,
    pub add_opcode: Option<add_opcode::ClaimGenerator>,
    pub add_opcode_small: Option<add_opcode_small::ClaimGenerator>,
    pub add_ap_opcode: Option<add_ap_opcode::ClaimGenerator>,
    pub generic_opcode: Option<generic_opcode::ClaimGenerator>,
    pub range_check_11: Option<range_check_11::ClaimGenerator>,
    pub verify_instruction: Option<verify_instruction::ClaimGenerator>,
    pub range_check_4_3: Option<range_check_4_3::ClaimGenerator>,
    pub range_check_7_2_5: Option<range_check_7_2_5::ClaimGenerator>,
    pub pedersen_builtin: Option<pedersen_builtin::ClaimGenerator>,
    pub pedersen_aggregator_window_bits_18:
        Option<pedersen_aggregator_window_bits_18::ClaimGenerator>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::ClaimGenerator>,
    pub pedersen_points_table_window_bits_18:
        Option<pedersen_points_table_window_bits_18::ClaimGenerator>,
    pub range_check_8: Option<range_check_8::ClaimGenerator>,
    pub poseidon_builtin: Option<poseidon_builtin::ClaimGenerator>,
    pub poseidon_aggregator: Option<poseidon_aggregator::ClaimGenerator>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::ClaimGenerator>,
    pub range_check_4_4: Option<range_check_4_4::ClaimGenerator>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::ClaimGenerator>,
    pub range_check_252_width_27: Option<range_check_252_width_27::ClaimGenerator>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::ClaimGenerator>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::ClaimGenerator>,
    pub poseidon_round_keys: Option<poseidon_round_keys::ClaimGenerator>,
    pub cube_252: Option<cube_252::ClaimGenerator>,
    pub range_check_20: Option<range_check_20::ClaimGenerator>,
    pub mul_mod_builtin: Option<mul_mod_builtin::ClaimGenerator>,
    pub range_check_18: Option<range_check_18::ClaimGenerator>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::ClaimGenerator>,
    pub range_check_12: Option<range_check_12::ClaimGenerator>,
    pub add_mod_builtin: Option<add_mod_builtin::ClaimGenerator>,
    pub range_check96_builtin: Option<range_check96_builtin::ClaimGenerator>,
    pub range_check_6: Option<range_check_6::ClaimGenerator>,
    pub range_check_builtin: Option<range_check_builtin::ClaimGenerator>,
    pub bitwise_builtin: Option<bitwise_builtin::ClaimGenerator>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::ClaimGenerator>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::ClaimGenerator>,
    pub memory_id_to_big: Option<memory_id_to_big::ClaimGenerator>,
    pub range_check_9_9: Option<range_check_9_9::ClaimGenerator>,
    pub memory_address_to_id: Option<memory_address_to_id::ClaimGenerator>,
}

impl CairoClaimGenerator {
    #[allow(clippy::redundant_closure)]
    pub fn fill_components(
        &mut self,
        components: &IndexSet<&str>,
        casm_states_by_opcode: CasmStatesByOpcode,
        builtin_segments: &BuiltinSegments,
        memory: Arc<Memory>,
        preprocessed_trace: Arc<PreProcessedTrace>,
    ) {
        // Destructure casm_states_by_opcode to allow parallel consumption of its fields.
        let CasmStatesByOpcode {
            blake_compress_opcode: casm_blake_compress_opcode,
            qm_31_add_mul_opcode: casm_qm_31_add_mul_opcode,
            ret_opcode: casm_ret_opcode,
            mul_opcode: casm_mul_opcode,
            mul_opcode_small: casm_mul_opcode_small,
            jump_opcode_abs: casm_jump_opcode_abs,
            jump_opcode_double_deref: casm_jump_opcode_double_deref,
            jump_opcode_rel: casm_jump_opcode_rel,
            jump_opcode_rel_imm: casm_jump_opcode_rel_imm,
            jnz_opcode_non_taken: casm_jnz_opcode_non_taken,
            jnz_opcode_taken: casm_jnz_opcode_taken,
            call_opcode_rel_imm: casm_call_opcode_rel_imm,
            call_opcode_abs: casm_call_opcode_abs,
            assert_eq_opcode_imm: casm_assert_eq_opcode_imm,
            assert_eq_opcode_double_deref: casm_assert_eq_opcode_double_deref,
            assert_eq_opcode: casm_assert_eq_opcode,
            add_opcode: casm_add_opcode,
            add_opcode_small: casm_add_opcode_small,
            add_ap_opcode: casm_add_ap_opcode,
            generic_opcode: casm_generic_opcode,
        } = casm_states_by_opcode;

        // Take mutable references to each field for parallel initialization.
        let blake_compress_opcode_ref = &mut self.blake_compress_opcode;
        let triple_xor_32_ref = &mut self.triple_xor_32;
        let blake_round_ref = &mut self.blake_round;
        let blake_g_ref = &mut self.blake_g;
        let verify_bitwise_xor_7_ref = &mut self.verify_bitwise_xor_7;
        let verify_bitwise_xor_4_ref = &mut self.verify_bitwise_xor_4;
        let verify_bitwise_xor_12_ref = &mut self.verify_bitwise_xor_12;
        let blake_round_sigma_ref = &mut self.blake_round_sigma;
        let qm_31_add_mul_opcode_ref = &mut self.qm_31_add_mul_opcode;
        let ret_opcode_ref = &mut self.ret_opcode;
        let mul_opcode_ref = &mut self.mul_opcode;
        let mul_opcode_small_ref = &mut self.mul_opcode_small;
        let jump_opcode_abs_ref = &mut self.jump_opcode_abs;
        let jump_opcode_double_deref_ref = &mut self.jump_opcode_double_deref;
        let jump_opcode_rel_ref = &mut self.jump_opcode_rel;
        let jump_opcode_rel_imm_ref = &mut self.jump_opcode_rel_imm;
        let jnz_opcode_non_taken_ref = &mut self.jnz_opcode_non_taken;
        let jnz_opcode_taken_ref = &mut self.jnz_opcode_taken;
        let call_opcode_rel_imm_ref = &mut self.call_opcode_rel_imm;
        let call_opcode_abs_ref = &mut self.call_opcode_abs;
        let assert_eq_opcode_imm_ref = &mut self.assert_eq_opcode_imm;
        let assert_eq_opcode_double_deref_ref = &mut self.assert_eq_opcode_double_deref;
        let assert_eq_opcode_ref = &mut self.assert_eq_opcode;
        let add_opcode_ref = &mut self.add_opcode;
        let add_opcode_small_ref = &mut self.add_opcode_small;
        let add_ap_opcode_ref = &mut self.add_ap_opcode;
        let generic_opcode_ref = &mut self.generic_opcode;
        let range_check_11_ref = &mut self.range_check_11;
        let verify_instruction_ref = &mut self.verify_instruction;
        let range_check_4_3_ref = &mut self.range_check_4_3;
        let range_check_7_2_5_ref = &mut self.range_check_7_2_5;
        let pedersen_builtin_ref = &mut self.pedersen_builtin;
        let pedersen_aggregator_window_bits_18_ref = &mut self.pedersen_aggregator_window_bits_18;
        let partial_ec_mul_window_bits_18_ref = &mut self.partial_ec_mul_window_bits_18;
        let pedersen_points_table_window_bits_18_ref =
            &mut self.pedersen_points_table_window_bits_18;
        let range_check_8_ref = &mut self.range_check_8;
        let poseidon_builtin_ref = &mut self.poseidon_builtin;
        let poseidon_aggregator_ref = &mut self.poseidon_aggregator;
        let poseidon_3_partial_rounds_chain_ref = &mut self.poseidon_3_partial_rounds_chain;
        let range_check_4_4_ref = &mut self.range_check_4_4;
        let range_check_4_4_4_4_ref = &mut self.range_check_4_4_4_4;
        let range_check_252_width_27_ref = &mut self.range_check_252_width_27;
        let poseidon_full_round_chain_ref = &mut self.poseidon_full_round_chain;
        let range_check_3_3_3_3_3_ref = &mut self.range_check_3_3_3_3_3;
        let poseidon_round_keys_ref = &mut self.poseidon_round_keys;
        let cube_252_ref = &mut self.cube_252;
        let range_check_20_ref = &mut self.range_check_20;
        let mul_mod_builtin_ref = &mut self.mul_mod_builtin;
        let range_check_18_ref = &mut self.range_check_18;
        let range_check_3_6_6_3_ref = &mut self.range_check_3_6_6_3;
        let range_check_12_ref = &mut self.range_check_12;
        let add_mod_builtin_ref = &mut self.add_mod_builtin;
        let range_check96_builtin_ref = &mut self.range_check96_builtin;
        let range_check_6_ref = &mut self.range_check_6;
        let range_check_builtin_ref = &mut self.range_check_builtin;
        let bitwise_builtin_ref = &mut self.bitwise_builtin;
        let verify_bitwise_xor_8_ref = &mut self.verify_bitwise_xor_8;
        let verify_bitwise_xor_9_ref = &mut self.verify_bitwise_xor_9;
        let memory_id_to_big_ref = &mut self.memory_id_to_big;
        let range_check_9_9_ref = &mut self.range_check_9_9;
        let memory_address_to_id_ref = &mut self.memory_address_to_id;

        // Use rayon::scope to spawn parallel tasks for each component.
        scope(|s| {
            s.spawn(|_| {
                *blake_compress_opcode_ref =
                    components.contains(&"blake_compress_opcode").then(|| {
                        blake_compress_opcode::ClaimGenerator::new(casm_blake_compress_opcode)
                    });
            });
            s.spawn(|_| {
                *triple_xor_32_ref = components
                    .contains(&"triple_xor_32")
                    .then(|| triple_xor_32::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *blake_round_ref = components
                    .contains(&"blake_round")
                    .then(|| blake_round::ClaimGenerator::new(memory.clone()));
            });
            s.spawn(|_| {
                *blake_g_ref = components
                    .contains(&"blake_g")
                    .then(|| blake_g::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *verify_bitwise_xor_7_ref =
                    components.contains(&"verify_bitwise_xor_7").then(|| {
                        verify_bitwise_xor_7::ClaimGenerator::new(preprocessed_trace.clone())
                    });
            });
            s.spawn(|_| {
                *verify_bitwise_xor_4_ref =
                    components.contains(&"verify_bitwise_xor_4").then(|| {
                        verify_bitwise_xor_4::ClaimGenerator::new(preprocessed_trace.clone())
                    });
            });
            s.spawn(|_| {
                *verify_bitwise_xor_12_ref =
                    components.contains(&"verify_bitwise_xor_12").then(|| {
                        verify_bitwise_xor_12::ClaimGenerator::new(preprocessed_trace.clone())
                    });
            });
            s.spawn(|_| {
                *blake_round_sigma_ref = components.contains(&"blake_round_sigma").then(|| {
                    blake_round_sigma::ClaimGenerator::new(preprocessed_trace.clone())
                });
            });
            s.spawn(|_| {
                *qm_31_add_mul_opcode_ref =
                    components.contains(&"qm_31_add_mul_opcode").then(|| {
                        qm_31_add_mul_opcode::ClaimGenerator::new(casm_qm_31_add_mul_opcode)
                    });
            });
            s.spawn(|_| {
                *ret_opcode_ref = components
                    .contains(&"ret_opcode")
                    .then(|| ret_opcode::ClaimGenerator::new(casm_ret_opcode));
            });
            s.spawn(|_| {
                *mul_opcode_ref = components
                    .contains(&"mul_opcode")
                    .then(|| mul_opcode::ClaimGenerator::new(casm_mul_opcode));
            });
            s.spawn(|_| {
                *mul_opcode_small_ref = components
                    .contains(&"mul_opcode_small")
                    .then(|| mul_opcode_small::ClaimGenerator::new(casm_mul_opcode_small));
            });
            s.spawn(|_| {
                *jump_opcode_abs_ref = components
                    .contains(&"jump_opcode_abs")
                    .then(|| jump_opcode_abs::ClaimGenerator::new(casm_jump_opcode_abs));
            });
            s.spawn(|_| {
                *jump_opcode_double_deref_ref =
                    components.contains(&"jump_opcode_double_deref").then(|| {
                        jump_opcode_double_deref::ClaimGenerator::new(casm_jump_opcode_double_deref)
                    });
            });
            s.spawn(|_| {
                *jump_opcode_rel_ref = components
                    .contains(&"jump_opcode_rel")
                    .then(|| jump_opcode_rel::ClaimGenerator::new(casm_jump_opcode_rel));
            });
            s.spawn(|_| {
                *jump_opcode_rel_imm_ref = components
                    .contains(&"jump_opcode_rel_imm")
                    .then(|| jump_opcode_rel_imm::ClaimGenerator::new(casm_jump_opcode_rel_imm));
            });
            s.spawn(|_| {
                *jnz_opcode_non_taken_ref = components
                    .contains(&"jnz_opcode_non_taken")
                    .then(|| jnz_opcode_non_taken::ClaimGenerator::new(casm_jnz_opcode_non_taken));
            });
            s.spawn(|_| {
                *jnz_opcode_taken_ref = components
                    .contains(&"jnz_opcode_taken")
                    .then(|| jnz_opcode_taken::ClaimGenerator::new(casm_jnz_opcode_taken));
            });
            s.spawn(|_| {
                *call_opcode_rel_imm_ref = components
                    .contains(&"call_opcode_rel_imm")
                    .then(|| call_opcode_rel_imm::ClaimGenerator::new(casm_call_opcode_rel_imm));
            });
            s.spawn(|_| {
                *call_opcode_abs_ref = components
                    .contains(&"call_opcode_abs")
                    .then(|| call_opcode_abs::ClaimGenerator::new(casm_call_opcode_abs));
            });
            s.spawn(|_| {
                *assert_eq_opcode_imm_ref = components
                    .contains(&"assert_eq_opcode_imm")
                    .then(|| assert_eq_opcode_imm::ClaimGenerator::new(casm_assert_eq_opcode_imm));
            });
            s.spawn(|_| {
                *assert_eq_opcode_double_deref_ref = components
                    .contains(&"assert_eq_opcode_double_deref")
                    .then(|| {
                        assert_eq_opcode_double_deref::ClaimGenerator::new(
                            casm_assert_eq_opcode_double_deref,
                        )
                    });
            });
            s.spawn(|_| {
                *assert_eq_opcode_ref = components
                    .contains(&"assert_eq_opcode")
                    .then(|| assert_eq_opcode::ClaimGenerator::new(casm_assert_eq_opcode));
            });
            s.spawn(|_| {
                *add_opcode_ref = components
                    .contains(&"add_opcode")
                    .then(|| add_opcode::ClaimGenerator::new(casm_add_opcode));
            });
            s.spawn(|_| {
                *add_opcode_small_ref = components
                    .contains(&"add_opcode_small")
                    .then(|| add_opcode_small::ClaimGenerator::new(casm_add_opcode_small));
            });
            s.spawn(|_| {
                *add_ap_opcode_ref = components
                    .contains(&"add_ap_opcode")
                    .then(|| add_ap_opcode::ClaimGenerator::new(casm_add_ap_opcode));
            });
            s.spawn(|_| {
                *generic_opcode_ref = components
                    .contains(&"generic_opcode")
                    .then(|| generic_opcode::ClaimGenerator::new(casm_generic_opcode));
            });
            s.spawn(|_| {
                *range_check_11_ref = components
                    .contains(&"range_check_11")
                    .then(|| range_check_11::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *verify_instruction_ref = components
                    .contains(&"verify_instruction")
                    .then(|| verify_instruction::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *range_check_4_3_ref = components
                    .contains(&"range_check_4_3")
                    .then(|| range_check_4_3::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *range_check_7_2_5_ref = components
                    .contains(&"range_check_7_2_5")
                    .then(|| range_check_7_2_5::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *pedersen_builtin_ref = components.contains(&"pedersen_builtin").then(|| {
                    let segment = builtin_segments.pedersen_builtin.unwrap();
                    let segment_length = segment.stop_ptr - segment.begin_addr;
                    assert!(
                        segment_length.is_multiple_of(PEDERSEN_BUILTIN_MEMORY_CELLS),
                        "pedersen_builtin segment length is not a multiple of it's cells_per_instance"
                    );
                    let n_instances = segment_length / PEDERSEN_BUILTIN_MEMORY_CELLS;
                    assert!(
                        n_instances.is_power_of_two(),
                        "pedersen_builtin instances number is not a power of two"
                    );
                    pedersen_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )
                });
            });
            s.spawn(|_| {
                *pedersen_aggregator_window_bits_18_ref = components
                    .contains(&"pedersen_aggregator_window_bits_18")
                    .then(|| pedersen_aggregator_window_bits_18::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *partial_ec_mul_window_bits_18_ref = components
                    .contains(&"partial_ec_mul_window_bits_18")
                    .then(|| partial_ec_mul_window_bits_18::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *pedersen_points_table_window_bits_18_ref = components
                    .contains(&"pedersen_points_table_window_bits_18")
                    .then(|| {
                        pedersen_points_table_window_bits_18::ClaimGenerator::new(
                            preprocessed_trace.clone(),
                        )
                    });
            });
            s.spawn(|_| {
                *range_check_8_ref = components
                    .contains(&"range_check_8")
                    .then(|| range_check_8::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *poseidon_builtin_ref = components.contains(&"poseidon_builtin").then(|| {
                    let segment = builtin_segments.poseidon_builtin.unwrap();
                    let segment_length = segment.stop_ptr - segment.begin_addr;
                    assert!(
                        segment_length.is_multiple_of(POSEIDON_BUILTIN_MEMORY_CELLS),
                        "poseidon_builtin segment length is not a multiple of it's cells_per_instance"
                    );
                    let n_instances = segment_length / POSEIDON_BUILTIN_MEMORY_CELLS;
                    assert!(
                        n_instances.is_power_of_two(),
                        "poseidon_builtin instances number is not a power of two"
                    );
                    poseidon_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )
                });
            });
            s.spawn(|_| {
                *poseidon_aggregator_ref = components
                    .contains(&"poseidon_aggregator")
                    .then(|| poseidon_aggregator::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *poseidon_3_partial_rounds_chain_ref = components
                    .contains(&"poseidon_3_partial_rounds_chain")
                    .then(|| poseidon_3_partial_rounds_chain::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *range_check_4_4_ref = components
                    .contains(&"range_check_4_4")
                    .then(|| range_check_4_4::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *range_check_4_4_4_4_ref = components
                    .contains(&"range_check_4_4_4_4")
                    .then(|| range_check_4_4_4_4::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *range_check_252_width_27_ref = components
                    .contains(&"range_check_252_width_27")
                    .then(|| range_check_252_width_27::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *poseidon_full_round_chain_ref = components
                    .contains(&"poseidon_full_round_chain")
                    .then(|| poseidon_full_round_chain::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *range_check_3_3_3_3_3_ref = components
                    .contains(&"range_check_3_3_3_3_3")
                    .then(|| range_check_3_3_3_3_3::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *poseidon_round_keys_ref = components
                    .contains(&"poseidon_round_keys")
                    .then(|| poseidon_round_keys::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *cube_252_ref = components
                    .contains(&"cube_252")
                    .then(|| cube_252::ClaimGenerator::new());
            });
            s.spawn(|_| {
                *range_check_20_ref = components
                    .contains(&"range_check_20")
                    .then(|| range_check_20::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *mul_mod_builtin_ref = components.contains(&"mul_mod_builtin").then(|| {
                    let segment = builtin_segments.mul_mod_builtin.unwrap();
                    let segment_length = segment.stop_ptr - segment.begin_addr;
                    assert!(
                        segment_length.is_multiple_of(MUL_MOD_BUILTIN_MEMORY_CELLS),
                        "mul_mod_builtin segment length is not a multiple of it's cells_per_instance"
                    );
                    let n_instances = segment_length / MUL_MOD_BUILTIN_MEMORY_CELLS;
                    assert!(
                        n_instances.is_power_of_two(),
                        "mul_mod_builtin instances number is not a power of two"
                    );
                    mul_mod_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )
                });
            });
            s.spawn(|_| {
                *range_check_18_ref = components
                    .contains(&"range_check_18")
                    .then(|| range_check_18::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *range_check_3_6_6_3_ref = components
                    .contains(&"range_check_3_6_6_3")
                    .then(|| range_check_3_6_6_3::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *range_check_12_ref = components
                    .contains(&"range_check_12")
                    .then(|| range_check_12::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *add_mod_builtin_ref = components.contains(&"add_mod_builtin").then(|| {
                    let segment = builtin_segments.add_mod_builtin.unwrap();
                    let segment_length = segment.stop_ptr - segment.begin_addr;
                    assert!(
                        segment_length.is_multiple_of(ADD_MOD_BUILTIN_MEMORY_CELLS),
                        "add_mod_builtin segment length is not a multiple of it's cells_per_instance"
                    );
                    let n_instances = segment_length / ADD_MOD_BUILTIN_MEMORY_CELLS;
                    assert!(
                        n_instances.is_power_of_two(),
                        "add_mod_builtin instances number is not a power of two"
                    );
                    add_mod_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )
                });
            });
            s.spawn(|_| {
                *range_check96_builtin_ref =
                    components.contains(&"range_check96_builtin").then(|| {
                        let segment = builtin_segments.range_check96_builtin.unwrap();
                        let segment_length = segment.stop_ptr - segment.begin_addr;
                        assert!(
                            segment_length.is_multiple_of(RANGE_CHECK_96_BUILTIN_MEMORY_CELLS),
                            "range_check96_builtin segment length is not a multiple of it's cells_per_instance"
                        );
                        let n_instances = segment_length / RANGE_CHECK_96_BUILTIN_MEMORY_CELLS;
                        assert!(
                            n_instances.is_power_of_two(),
                            "range_check96_builtin instances number is not a power of two"
                        );
                        range_check96_builtin::ClaimGenerator::new(
                            n_instances.ilog2(),
                            segment.begin_addr as u32,
                        )
                    });
            });
            s.spawn(|_| {
                *range_check_6_ref = components
                    .contains(&"range_check_6")
                    .then(|| range_check_6::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *range_check_builtin_ref = components.contains(&"range_check_builtin").then(|| {
                    let segment = builtin_segments.range_check_builtin.unwrap();
                    let segment_length = segment.stop_ptr - segment.begin_addr;
                    assert!(
                        segment_length.is_multiple_of(RANGE_CHECK_BUILTIN_MEMORY_CELLS),
                        "range_check_builtin segment length is not a multiple of it's cells_per_instance"
                    );
                    let n_instances = segment_length / RANGE_CHECK_BUILTIN_MEMORY_CELLS;
                    assert!(
                        n_instances.is_power_of_two(),
                        "range_check_builtin instances number is not a power of two"
                    );
                    range_check_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )
                });
            });
            s.spawn(|_| {
                *bitwise_builtin_ref = components.contains(&"bitwise_builtin").then(|| {
                    let segment = builtin_segments.bitwise_builtin.unwrap();
                    let segment_length = segment.stop_ptr - segment.begin_addr;
                    assert!(
                        segment_length.is_multiple_of(BITWISE_BUILTIN_MEMORY_CELLS),
                        "bitwise_builtin segment length is not a multiple of it's cells_per_instance"
                    );
                    let n_instances = segment_length / BITWISE_BUILTIN_MEMORY_CELLS;
                    assert!(
                        n_instances.is_power_of_two(),
                        "bitwise_builtin instances number is not a power of two"
                    );
                    bitwise_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    )
                });
            });
            s.spawn(|_| {
                *verify_bitwise_xor_8_ref =
                    components.contains(&"verify_bitwise_xor_8").then(|| {
                        verify_bitwise_xor_8::ClaimGenerator::new(preprocessed_trace.clone())
                    });
            });
            s.spawn(|_| {
                *verify_bitwise_xor_9_ref =
                    components.contains(&"verify_bitwise_xor_9").then(|| {
                        verify_bitwise_xor_9::ClaimGenerator::new(preprocessed_trace.clone())
                    });
            });
            s.spawn(|_| {
                *memory_id_to_big_ref = components
                    .contains(&"memory_id_to_big")
                    .then(|| memory_id_to_big::ClaimGenerator::new(memory.clone()));
            });
            s.spawn(|_| {
                *range_check_9_9_ref = components
                    .contains(&"range_check_9_9")
                    .then(|| range_check_9_9::ClaimGenerator::new(preprocessed_trace.clone()));
            });
            s.spawn(|_| {
                *memory_address_to_id_ref = components
                    .contains(&"memory_address_to_id")
                    .then(|| memory_address_to_id::ClaimGenerator::new(memory.clone()));
            });
        });
    }
}

pub fn get_sub_components(component_name: &str) -> Vec<&'static str> {
    match component_name {
        "generic_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "range_check_20",
                "range_check_18",
                "range_check_11",
                "generic_opcode",
            ]
        }
        "add_ap_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "range_check_18",
                "range_check_11",
                "add_ap_opcode",
            ]
        }
        "add_opcode_small" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "add_opcode_small",
            ]
        }
        "add_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "add_opcode",
            ]
        }
        "assert_eq_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "assert_eq_opcode",
            ]
        }
        "assert_eq_opcode_double_deref" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "assert_eq_opcode_double_deref",
            ]
        }
        "assert_eq_opcode_imm" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "assert_eq_opcode_imm",
            ]
        }
        "call_opcode_abs" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "call_opcode_abs",
            ]
        }
        "call_opcode_rel_imm" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "call_opcode_rel_imm",
            ]
        }
        "jnz_opcode_taken" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "jnz_opcode_taken",
            ]
        }
        "jnz_opcode_non_taken" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "jnz_opcode_non_taken",
            ]
        }
        "jump_opcode_rel_imm" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "jump_opcode_rel_imm",
            ]
        }
        "jump_opcode_rel" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "jump_opcode_rel",
            ]
        }
        "jump_opcode_double_deref" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "jump_opcode_double_deref",
            ]
        }
        "jump_opcode_abs" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "jump_opcode_abs",
            ]
        }
        "mul_opcode_small" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "range_check_11",
                "mul_opcode_small",
            ]
        }
        "mul_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "range_check_20",
                "mul_opcode",
            ]
        }
        "ret_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "ret_opcode",
            ]
        }
        "qm_31_add_mul_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "range_check_4_4_4_4",
                "qm_31_add_mul_opcode",
            ]
        }
        "blake_compress_opcode" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_7_2_5",
                "range_check_4_3",
                "verify_instruction",
                "verify_bitwise_xor_8",
                "blake_round_sigma",
                "verify_bitwise_xor_12",
                "verify_bitwise_xor_4",
                "verify_bitwise_xor_7",
                "verify_bitwise_xor_9",
                "blake_g",
                "blake_round",
                "triple_xor_32",
                "blake_compress_opcode",
            ]
        }
        "bitwise_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "verify_bitwise_xor_9",
                "verify_bitwise_xor_8",
                "bitwise_builtin",
            ]
        }
        "range_check_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_builtin",
            ]
        }
        "range_check96_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_6",
                "range_check96_builtin",
            ]
        }
        "add_mod_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "add_mod_builtin",
            ]
        }
        "mul_mod_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_12",
                "range_check_3_6_6_3",
                "range_check_18",
                "mul_mod_builtin",
            ]
        }
        "poseidon_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_20",
                "cube_252",
                "poseidon_round_keys",
                "range_check_3_3_3_3_3",
                "poseidon_full_round_chain",
                "range_check_18",
                "range_check_252_width_27",
                "range_check_4_4_4_4",
                "range_check_4_4",
                "poseidon_3_partial_rounds_chain",
                "poseidon_aggregator",
                "poseidon_builtin",
            ]
        }
        "pedersen_builtin" => {
            vec![
                "memory_address_to_id",
                "range_check_9_9",
                "memory_id_to_big",
                "range_check_8",
                "pedersen_points_table_window_bits_18",
                "range_check_20",
                "partial_ec_mul_window_bits_18",
                "pedersen_aggregator_window_bits_18",
                "pedersen_builtin",
            ]
        }
        _ => panic!("Unknown component: {component_name}"),
    }
}

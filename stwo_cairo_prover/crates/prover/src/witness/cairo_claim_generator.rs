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
        let Self {
            blake_compress_opcode: blake_compress_opcode_ref,
            triple_xor_32: triple_xor_32_ref,
            blake_round: blake_round_ref,
            blake_g: blake_g_ref,
            verify_bitwise_xor_7: verify_bitwise_xor_7_ref,
            verify_bitwise_xor_4: verify_bitwise_xor_4_ref,
            verify_bitwise_xor_12: verify_bitwise_xor_12_ref,
            blake_round_sigma: blake_round_sigma_ref,
            qm_31_add_mul_opcode: qm_31_add_mul_opcode_ref,
            ret_opcode: ret_opcode_ref,
            mul_opcode: mul_opcode_ref,
            mul_opcode_small: mul_opcode_small_ref,
            jump_opcode_abs: jump_opcode_abs_ref,
            jump_opcode_double_deref: jump_opcode_double_deref_ref,
            jump_opcode_rel: jump_opcode_rel_ref,
            jump_opcode_rel_imm: jump_opcode_rel_imm_ref,
            jnz_opcode_non_taken: jnz_opcode_non_taken_ref,
            jnz_opcode_taken: jnz_opcode_taken_ref,
            call_opcode_rel_imm: call_opcode_rel_imm_ref,
            call_opcode_abs: call_opcode_abs_ref,
            assert_eq_opcode_imm: assert_eq_opcode_imm_ref,
            assert_eq_opcode_double_deref: assert_eq_opcode_double_deref_ref,
            assert_eq_opcode: assert_eq_opcode_ref,
            add_opcode: add_opcode_ref,
            add_opcode_small: add_opcode_small_ref,
            add_ap_opcode: add_ap_opcode_ref,
            generic_opcode: generic_opcode_ref,
            range_check_11: range_check_11_ref,
            verify_instruction: verify_instruction_ref,
            range_check_4_3: range_check_4_3_ref,
            range_check_7_2_5: range_check_7_2_5_ref,
            pedersen_builtin: pedersen_builtin_ref,
            pedersen_aggregator_window_bits_18: pedersen_aggregator_window_bits_18_ref,
            partial_ec_mul_window_bits_18: partial_ec_mul_window_bits_18_ref,
            pedersen_points_table_window_bits_18: pedersen_points_table_window_bits_18_ref,
            range_check_8: range_check_8_ref,
            poseidon_builtin: poseidon_builtin_ref,
            poseidon_aggregator: poseidon_aggregator_ref,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_ref,
            range_check_4_4: range_check_4_4_ref,
            range_check_4_4_4_4: range_check_4_4_4_4_ref,
            range_check_252_width_27: range_check_252_width_27_ref,
            poseidon_full_round_chain: poseidon_full_round_chain_ref,
            range_check_3_3_3_3_3: range_check_3_3_3_3_3_ref,
            poseidon_round_keys: poseidon_round_keys_ref,
            cube_252: cube_252_ref,
            range_check_20: range_check_20_ref,
            mul_mod_builtin: mul_mod_builtin_ref,
            range_check_18: range_check_18_ref,
            range_check_3_6_6_3: range_check_3_6_6_3_ref,
            range_check_12: range_check_12_ref,
            add_mod_builtin: add_mod_builtin_ref,
            range_check96_builtin: range_check96_builtin_ref,
            range_check_6: range_check_6_ref,
            range_check_builtin: range_check_builtin_ref,
            bitwise_builtin: bitwise_builtin_ref,
            verify_bitwise_xor_8: verify_bitwise_xor_8_ref,
            verify_bitwise_xor_9: verify_bitwise_xor_9_ref,
            memory_id_to_big: memory_id_to_big_ref,
            range_check_9_9: range_check_9_9_ref,
            memory_address_to_id: memory_address_to_id_ref,
            public_data: _,
        } = self;

        scope(|s| {
            if components.contains(&"blake_compress_opcode") {
                s.spawn(|_| {
                    *blake_compress_opcode_ref = Some(blake_compress_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.blake_compress_opcode,
                    ));
                });
            }
            if components.contains(&"triple_xor_32") {
                s.spawn(|_| {
                    *triple_xor_32_ref = Some(triple_xor_32::ClaimGenerator::new());
                });
            }
            if components.contains(&"blake_round") {
                s.spawn(|_| {
                    *blake_round_ref = Some(blake_round::ClaimGenerator::new(memory.clone()));
                });
            }
            if components.contains(&"blake_g") {
                s.spawn(|_| {
                    *blake_g_ref = Some(blake_g::ClaimGenerator::new());
                });
            }
            if components.contains(&"verify_bitwise_xor_7") {
                s.spawn(|_| {
                    *verify_bitwise_xor_7_ref = Some(verify_bitwise_xor_7::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"verify_bitwise_xor_4") {
                s.spawn(|_| {
                    *verify_bitwise_xor_4_ref = Some(verify_bitwise_xor_4::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"verify_bitwise_xor_12") {
                s.spawn(|_| {
                    *verify_bitwise_xor_12_ref = Some(verify_bitwise_xor_12::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"blake_round_sigma") {
                s.spawn(|_| {
                    *blake_round_sigma_ref = Some(blake_round_sigma::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"qm_31_add_mul_opcode") {
                s.spawn(|_| {
                    *qm_31_add_mul_opcode_ref = Some(qm_31_add_mul_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.qm_31_add_mul_opcode,
                    ));
                });
            }
            if components.contains(&"ret_opcode") {
                s.spawn(|_| {
                    *ret_opcode_ref = Some(ret_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.ret_opcode,
                    ));
                });
            }
            if components.contains(&"mul_opcode") {
                s.spawn(|_| {
                    *mul_opcode_ref = Some(mul_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.mul_opcode,
                    ));
                });
            }
            if components.contains(&"mul_opcode_small") {
                s.spawn(|_| {
                    *mul_opcode_small_ref = Some(mul_opcode_small::ClaimGenerator::new(
                        casm_states_by_opcode.mul_opcode_small,
                    ));
                });
            }
            if components.contains(&"jump_opcode_abs") {
                s.spawn(|_| {
                    *jump_opcode_abs_ref = Some(jump_opcode_abs::ClaimGenerator::new(
                        casm_states_by_opcode.jump_opcode_abs,
                    ));
                });
            }
            if components.contains(&"jump_opcode_double_deref") {
                s.spawn(|_| {
                    *jump_opcode_double_deref_ref =
                        Some(jump_opcode_double_deref::ClaimGenerator::new(
                            casm_states_by_opcode.jump_opcode_double_deref,
                        ));
                });
            }
            if components.contains(&"jump_opcode_rel") {
                s.spawn(|_| {
                    *jump_opcode_rel_ref = Some(jump_opcode_rel::ClaimGenerator::new(
                        casm_states_by_opcode.jump_opcode_rel,
                    ));
                });
            }
            if components.contains(&"jump_opcode_rel_imm") {
                s.spawn(|_| {
                    *jump_opcode_rel_imm_ref = Some(jump_opcode_rel_imm::ClaimGenerator::new(
                        casm_states_by_opcode.jump_opcode_rel_imm,
                    ));
                });
            }
            if components.contains(&"jnz_opcode_non_taken") {
                s.spawn(|_| {
                    *jnz_opcode_non_taken_ref = Some(jnz_opcode_non_taken::ClaimGenerator::new(
                        casm_states_by_opcode.jnz_opcode_non_taken,
                    ));
                });
            }
            if components.contains(&"jnz_opcode_taken") {
                s.spawn(|_| {
                    *jnz_opcode_taken_ref = Some(jnz_opcode_taken::ClaimGenerator::new(
                        casm_states_by_opcode.jnz_opcode_taken,
                    ));
                });
            }
            if components.contains(&"call_opcode_rel_imm") {
                s.spawn(|_| {
                    *call_opcode_rel_imm_ref = Some(call_opcode_rel_imm::ClaimGenerator::new(
                        casm_states_by_opcode.call_opcode_rel_imm,
                    ));
                });
            }
            if components.contains(&"call_opcode_abs") {
                s.spawn(|_| {
                    *call_opcode_abs_ref = Some(call_opcode_abs::ClaimGenerator::new(
                        casm_states_by_opcode.call_opcode_abs,
                    ));
                });
            }
            if components.contains(&"assert_eq_opcode_imm") {
                s.spawn(|_| {
                    *assert_eq_opcode_imm_ref = Some(assert_eq_opcode_imm::ClaimGenerator::new(
                        casm_states_by_opcode.assert_eq_opcode_imm,
                    ));
                });
            }
            if components.contains(&"assert_eq_opcode_double_deref") {
                s.spawn(|_| {
                    *assert_eq_opcode_double_deref_ref =
                        Some(assert_eq_opcode_double_deref::ClaimGenerator::new(
                            casm_states_by_opcode.assert_eq_opcode_double_deref,
                        ));
                });
            }
            if components.contains(&"assert_eq_opcode") {
                s.spawn(|_| {
                    *assert_eq_opcode_ref = Some(assert_eq_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.assert_eq_opcode,
                    ));
                });
            }
            if components.contains(&"add_opcode") {
                s.spawn(|_| {
                    *add_opcode_ref = Some(add_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.add_opcode,
                    ));
                });
            }
            if components.contains(&"add_opcode_small") {
                s.spawn(|_| {
                    *add_opcode_small_ref = Some(add_opcode_small::ClaimGenerator::new(
                        casm_states_by_opcode.add_opcode_small,
                    ));
                });
            }
            if components.contains(&"add_ap_opcode") {
                s.spawn(|_| {
                    *add_ap_opcode_ref = Some(add_ap_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.add_ap_opcode,
                    ));
                });
            }
            if components.contains(&"generic_opcode") {
                s.spawn(|_| {
                    *generic_opcode_ref = Some(generic_opcode::ClaimGenerator::new(
                        casm_states_by_opcode.generic_opcode,
                    ));
                });
            }
            if components.contains(&"range_check_11") {
                s.spawn(|_| {
                    *range_check_11_ref = Some(range_check_11::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"verify_instruction") {
                s.spawn(|_| {
                    *verify_instruction_ref = Some(verify_instruction::ClaimGenerator::new());
                });
            }
            if components.contains(&"range_check_4_3") {
                s.spawn(|_| {
                    *range_check_4_3_ref = Some(range_check_4_3::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"range_check_7_2_5") {
                s.spawn(|_| {
                    *range_check_7_2_5_ref = Some(range_check_7_2_5::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"pedersen_builtin") {
                s.spawn(|_| {
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
                    *pedersen_builtin_ref = Some(pedersen_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    ));
                });
            }
            if components.contains(&"pedersen_aggregator_window_bits_18") {
                s.spawn(|_| {
                    *pedersen_aggregator_window_bits_18_ref =
                        Some(pedersen_aggregator_window_bits_18::ClaimGenerator::new());
                });
            }
            if components.contains(&"partial_ec_mul_window_bits_18") {
                s.spawn(|_| {
                    *partial_ec_mul_window_bits_18_ref =
                        Some(partial_ec_mul_window_bits_18::ClaimGenerator::new());
                });
            }
            if components.contains(&"pedersen_points_table_window_bits_18") {
                s.spawn(|_| {
                    *pedersen_points_table_window_bits_18_ref =
                        Some(pedersen_points_table_window_bits_18::ClaimGenerator::new(
                            preprocessed_trace.clone(),
                        ));
                });
            }
            if components.contains(&"range_check_8") {
                s.spawn(|_| {
                    *range_check_8_ref = Some(range_check_8::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"poseidon_builtin") {
                s.spawn(|_| {
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
                    *poseidon_builtin_ref = Some(poseidon_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    ));
                });
            }
            if components.contains(&"poseidon_aggregator") {
                s.spawn(|_| {
                    *poseidon_aggregator_ref = Some(poseidon_aggregator::ClaimGenerator::new());
                });
            }
            if components.contains(&"poseidon_3_partial_rounds_chain") {
                s.spawn(|_| {
                    *poseidon_3_partial_rounds_chain_ref =
                        Some(poseidon_3_partial_rounds_chain::ClaimGenerator::new());
                });
            }
            if components.contains(&"range_check_4_4") {
                s.spawn(|_| {
                    *range_check_4_4_ref = Some(range_check_4_4::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"range_check_4_4_4_4") {
                s.spawn(|_| {
                    *range_check_4_4_4_4_ref = Some(range_check_4_4_4_4::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"range_check_252_width_27") {
                s.spawn(|_| {
                    *range_check_252_width_27_ref =
                        Some(range_check_252_width_27::ClaimGenerator::new());
                });
            }
            if components.contains(&"poseidon_full_round_chain") {
                s.spawn(|_| {
                    *poseidon_full_round_chain_ref =
                        Some(poseidon_full_round_chain::ClaimGenerator::new());
                });
            }
            if components.contains(&"range_check_3_3_3_3_3") {
                s.spawn(|_| {
                    *range_check_3_3_3_3_3_ref = Some(range_check_3_3_3_3_3::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"poseidon_round_keys") {
                s.spawn(|_| {
                    *poseidon_round_keys_ref = Some(poseidon_round_keys::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"cube_252") {
                s.spawn(|_| {
                    *cube_252_ref = Some(cube_252::ClaimGenerator::new());
                });
            }
            if components.contains(&"range_check_20") {
                s.spawn(|_| {
                    *range_check_20_ref = Some(range_check_20::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"mul_mod_builtin") {
                s.spawn(|_| {
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
                    *mul_mod_builtin_ref = Some(mul_mod_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    ));
                });
            }
            if components.contains(&"range_check_18") {
                s.spawn(|_| {
                    *range_check_18_ref = Some(range_check_18::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"range_check_3_6_6_3") {
                s.spawn(|_| {
                    *range_check_3_6_6_3_ref = Some(range_check_3_6_6_3::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"range_check_12") {
                s.spawn(|_| {
                    *range_check_12_ref = Some(range_check_12::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"add_mod_builtin") {
                s.spawn(|_| {
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
                    *add_mod_builtin_ref = Some(add_mod_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    ));
                });
            }
            if components.contains(&"range_check96_builtin") {
                s.spawn(|_| {
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
                *range_check96_builtin_ref = Some(range_check96_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32));
            });
            }
            if components.contains(&"range_check_6") {
                s.spawn(|_| {
                    *range_check_6_ref = Some(range_check_6::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"range_check_builtin") {
                s.spawn(|_| {
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
                *range_check_builtin_ref = Some(range_check_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32));
            });
            }
            if components.contains(&"bitwise_builtin") {
                s.spawn(|_| {
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
                    *bitwise_builtin_ref = Some(bitwise_builtin::ClaimGenerator::new(
                        n_instances.ilog2(),
                        segment.begin_addr as u32,
                    ));
                });
            }
            if components.contains(&"verify_bitwise_xor_8") {
                s.spawn(|_| {
                    *verify_bitwise_xor_8_ref = Some(verify_bitwise_xor_8::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"verify_bitwise_xor_9") {
                s.spawn(|_| {
                    *verify_bitwise_xor_9_ref = Some(verify_bitwise_xor_9::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"memory_id_to_big") {
                s.spawn(|_| {
                    *memory_id_to_big_ref =
                        Some(memory_id_to_big::ClaimGenerator::new(memory.clone()));
                });
            }
            if components.contains(&"range_check_9_9") {
                s.spawn(|_| {
                    *range_check_9_9_ref = Some(range_check_9_9::ClaimGenerator::new(
                        preprocessed_trace.clone(),
                    ));
                });
            }
            if components.contains(&"memory_address_to_id") {
                s.spawn(|_| {
                    *memory_address_to_id_ref =
                        Some(memory_address_to_id::ClaimGenerator::new(memory.clone()));
                });
            }
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

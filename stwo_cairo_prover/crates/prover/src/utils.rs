// This file was created by the AIR team.

use cairo_air::cairo_components::CairoComponents;
pub use stwo::prover::backend::simd::SimdBackend;

pub fn cairo_provers(
    components: &CairoComponents,
) -> Vec<&dyn stwo::prover::ComponentProver<SimdBackend>> {
    let mut vec: Vec<&dyn stwo::prover::ComponentProver<SimdBackend>> = vec![];
    if let Some(component) = &components.add_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.add_opcode_small {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.add_ap_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.assert_eq_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.assert_eq_opcode_imm {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.assert_eq_opcode_double_deref {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.blake_compress_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.call_opcode_abs {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.call_opcode_rel_imm {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.generic_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.jnz_opcode_non_taken {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.jnz_opcode_taken {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.jump_opcode_abs {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.jump_opcode_double_deref {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.jump_opcode_rel {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.jump_opcode_rel_imm {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.mul_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.mul_opcode_small {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.qm_31_add_mul_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.ret_opcode {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.verify_instruction {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.blake_round {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.blake_g {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.blake_round_sigma {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.triple_xor_32 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.verify_bitwise_xor_12 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.add_mod_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.bitwise_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.mul_mod_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.pedersen_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.pedersen_builtin_narrow_windows {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.poseidon_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check96_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_builtin {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.pedersen_aggregator_window_bits_18 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.partial_ec_mul_window_bits_18 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.pedersen_points_table_window_bits_18 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.pedersen_aggregator_window_bits_9 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.partial_ec_mul_window_bits_9 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.pedersen_points_table_window_bits_9 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.poseidon_aggregator {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.poseidon_3_partial_rounds_chain {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.poseidon_full_round_chain {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.cube_252 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.poseidon_round_keys {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_252_width_27 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.memory_address_to_id {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    for component in &components.memory_id_to_big {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.memory_id_to_small {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_6 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_8 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_11 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_12 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_18 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_20 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_4_3 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_4_4 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_9_9 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_7_2_5 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_3_6_6_3 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_4_4_4_4 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.range_check_3_3_3_3_3 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.verify_bitwise_xor_4 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.verify_bitwise_xor_7 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.verify_bitwise_xor_8 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    if let Some(component) = &components.verify_bitwise_xor_9 {
        vec.push(component as &dyn stwo::prover::ComponentProver<SimdBackend>);
    }
    vec
}

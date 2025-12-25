use std::sync::Arc;

use cairo_air::air::PublicData;
use indexmap::IndexSet;
use stwo_cairo_adapter::builtins::BuiltinSegments;
use stwo_cairo_adapter::memory::Memory;
use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;
use stwo_cairo_common::builtins::{
    ADD_MOD_MEMORY_CELLS, BITWISE_MEMORY_CELLS, MUL_MOD_MEMORY_CELLS, PEDERSEN_MEMORY_CELLS,
    POSEIDON_MEMORY_CELLS, RANGE_CHECK96_MEMORY_CELLS, RANGE_CHECK_MEMORY_CELLS,
};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;

use crate::witness::components::{
    add_ap_opcode, add_mod_builtin, add_opcode, add_opcode_small, assert_eq_opcode,
    assert_eq_opcode_double_deref, assert_eq_opcode_imm, bitwise_builtin, blake_compress_opcode,
    blake_g, blake_round, blake_round_sigma, call_opcode_abs, call_opcode_rel_imm, cube_252,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_mod_builtin, mul_opcode, mul_opcode_small, partial_ec_mul,
    pedersen_aggregator, pedersen_builtin, pedersen_points_table, poseidon_3_partial_rounds_chain,
    poseidon_aggregator, poseidon_builtin, poseidon_full_round_chain, poseidon_round_keys,
    qm_31_add_mul_opcode, range_check96_builtin, range_check_11, range_check_12, range_check_18,
    range_check_20, range_check_252_width_27, range_check_3_3_3_3_3, range_check_3_6_6_3,
    range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6, range_check_7_2_5,
    range_check_8, range_check_9_9, range_check_builtin, ret_opcode, triple_xor_32,
    verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8,
    verify_bitwise_xor_9, verify_instruction,
};

#[derive(Default)]
pub struct CairoClaimGenerator {
    pub public_data: PublicData,
    pub pedersen_builtin: Option<pedersen_builtin::ClaimGenerator>,
    pub pedersen_aggregator: Option<pedersen_aggregator::ClaimGenerator>,
    pub partial_ec_mul: Option<partial_ec_mul::ClaimGenerator>,
    pub pedersen_points_table: Option<pedersen_points_table::ClaimGenerator>,
    pub range_check_8: Option<range_check_8::ClaimGenerator>,
    pub poseidon_builtin: Option<poseidon_builtin::ClaimGenerator>,
    pub poseidon_aggregator: Option<poseidon_aggregator::ClaimGenerator>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::ClaimGenerator>,
    pub range_check_4_4: Option<range_check_4_4::ClaimGenerator>,
    pub range_check_252_width_27: Option<range_check_252_width_27::ClaimGenerator>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::ClaimGenerator>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::ClaimGenerator>,
    pub poseidon_round_keys: Option<poseidon_round_keys::ClaimGenerator>,
    pub cube_252: Option<cube_252::ClaimGenerator>,
    pub mul_mod_builtin: Option<mul_mod_builtin::ClaimGenerator>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::ClaimGenerator>,
    pub range_check_12: Option<range_check_12::ClaimGenerator>,
    pub add_mod_builtin: Option<add_mod_builtin::ClaimGenerator>,
    pub range_check96_builtin: Option<range_check96_builtin::ClaimGenerator>,
    pub range_check_6: Option<range_check_6::ClaimGenerator>,
    pub range_check_builtin: Option<range_check_builtin::ClaimGenerator>,
    pub bitwise_builtin: Option<bitwise_builtin::ClaimGenerator>,
    pub blake_compress_opcode: Option<blake_compress_opcode::ClaimGenerator>,
    pub triple_xor_32: Option<triple_xor_32::ClaimGenerator>,
    pub blake_round: Option<blake_round::ClaimGenerator>,
    pub blake_g: Option<blake_g::ClaimGenerator>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::ClaimGenerator>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::ClaimGenerator>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::ClaimGenerator>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::ClaimGenerator>,
    pub blake_round_sigma: Option<blake_round_sigma::ClaimGenerator>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::ClaimGenerator>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::ClaimGenerator>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::ClaimGenerator>,
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
    pub range_check_18: Option<range_check_18::ClaimGenerator>,
    pub range_check_20: Option<range_check_20::ClaimGenerator>,
    pub verify_instruction: Option<verify_instruction::ClaimGenerator>,
    pub range_check_4_3: Option<range_check_4_3::ClaimGenerator>,
    pub range_check_7_2_5: Option<range_check_7_2_5::ClaimGenerator>,
    pub memory_id_to_big: Option<memory_id_to_big::ClaimGenerator>,
    pub range_check_9_9: Option<range_check_9_9::ClaimGenerator>,
    pub memory_address_to_id: Option<memory_address_to_id::ClaimGenerator>,
}

#[allow(clippy::redundant_closure)]
pub fn fill_cairo_claim_generator(
    claim_generator: &mut CairoClaimGenerator,
    components: &IndexSet<&str>,
    casm_states_by_opcode: CasmStatesByOpcode,
    builtin_segments: &BuiltinSegments,
    memory: &Memory,
    preprocessed_trace: &Arc<PreProcessedTrace>,
) {
    claim_generator.pedersen_builtin = components.contains(&"pedersen_builtin").then(|| {
        let segment = builtin_segments.pedersen_builtin.unwrap();

        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(PEDERSEN_MEMORY_CELLS),
            "pedersen_builtin segment length is not a multiple of it's cells_per_instance"
        );

        let n_instances = segment_length / PEDERSEN_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "pedersen_builtin instances number is not a power of two"
        );
        pedersen_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
    });

    claim_generator.pedersen_aggregator = components
        .contains(&"pedersen_aggregator")
        .then(|| pedersen_aggregator::ClaimGenerator::new());

    claim_generator.partial_ec_mul = components
        .contains(&"partial_ec_mul")
        .then(|| partial_ec_mul::ClaimGenerator::new());

    claim_generator.pedersen_points_table = components
        .contains(&"pedersen_points_table")
        .then(|| pedersen_points_table::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_8 = components
        .contains(&"range_check_8")
        .then(|| range_check_8::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.poseidon_builtin = components.contains(&"poseidon_builtin").then(|| {
        let segment = builtin_segments.poseidon_builtin.unwrap();

        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(POSEIDON_MEMORY_CELLS),
            "poseidon_builtin segment length is not a multiple of it's cells_per_instance"
        );

        let n_instances = segment_length / POSEIDON_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "poseidon_builtin instances number is not a power of two"
        );
        poseidon_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
    });

    claim_generator.poseidon_aggregator = components
        .contains(&"poseidon_aggregator")
        .then(|| poseidon_aggregator::ClaimGenerator::new());

    claim_generator.poseidon_3_partial_rounds_chain = components
        .contains(&"poseidon_3_partial_rounds_chain")
        .then(|| poseidon_3_partial_rounds_chain::ClaimGenerator::new());

    claim_generator.range_check_4_4 = components
        .contains(&"range_check_4_4")
        .then(|| range_check_4_4::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_252_width_27 = components
        .contains(&"range_check_252_width_27")
        .then(|| range_check_252_width_27::ClaimGenerator::new());

    claim_generator.poseidon_full_round_chain = components
        .contains(&"poseidon_full_round_chain")
        .then(|| poseidon_full_round_chain::ClaimGenerator::new());

    claim_generator.range_check_3_3_3_3_3 = components
        .contains(&"range_check_3_3_3_3_3")
        .then(|| range_check_3_3_3_3_3::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.poseidon_round_keys = components
        .contains(&"poseidon_round_keys")
        .then(|| poseidon_round_keys::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.cube_252 = components
        .contains(&"cube_252")
        .then(|| cube_252::ClaimGenerator::new());

    claim_generator.mul_mod_builtin = components.contains(&"mul_mod_builtin").then(|| {
        let segment = builtin_segments.mul_mod_builtin.unwrap();

        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(MUL_MOD_MEMORY_CELLS),
            "mul_mod_builtin segment length is not a multiple of it's cells_per_instance"
        );

        let n_instances = segment_length / MUL_MOD_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "mul_mod_builtin instances number is not a power of two"
        );
        mul_mod_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
    });

    claim_generator.range_check_3_6_6_3 = components
        .contains(&"range_check_3_6_6_3")
        .then(|| range_check_3_6_6_3::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_12 = components
        .contains(&"range_check_12")
        .then(|| range_check_12::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.add_mod_builtin = components.contains(&"add_mod_builtin").then(|| {
        let segment = builtin_segments.add_mod_builtin.unwrap();

        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(ADD_MOD_MEMORY_CELLS),
            "add_mod_builtin segment length is not a multiple of it's cells_per_instance"
        );

        let n_instances = segment_length / ADD_MOD_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "add_mod_builtin instances number is not a power of two"
        );
        add_mod_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
    });

    claim_generator.range_check96_builtin =
        components.contains(&"range_check96_builtin").then(|| {
            let segment = builtin_segments.range_check96_builtin.unwrap();

            let segment_length = segment.stop_ptr - segment.begin_addr;
            assert!(
                segment_length.is_multiple_of(RANGE_CHECK96_MEMORY_CELLS),
                "range_check96_builtin segment length is not a multiple of it's cells_per_instance"
            );

            let n_instances = segment_length / RANGE_CHECK96_MEMORY_CELLS;
            assert!(
                n_instances.is_power_of_two(),
                "range_check96_builtin instances number is not a power of two"
            );
            range_check96_builtin::ClaimGenerator::new(
                n_instances.ilog2(),
                segment.begin_addr as u32,
            )
        });

    claim_generator.range_check_6 = components
        .contains(&"range_check_6")
        .then(|| range_check_6::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_builtin = components.contains(&"range_check_builtin").then(|| {
        let segment = builtin_segments.range_check_builtin.unwrap();

        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(RANGE_CHECK_MEMORY_CELLS),
            "range_check_builtin segment length is not a multiple of it's cells_per_instance"
        );

        let n_instances = segment_length / RANGE_CHECK_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "range_check_builtin instances number is not a power of two"
        );
        range_check_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
    });

    claim_generator.bitwise_builtin = components.contains(&"bitwise_builtin").then(|| {
        let segment = builtin_segments.bitwise_builtin.unwrap();

        let segment_length = segment.stop_ptr - segment.begin_addr;
        assert!(
            segment_length.is_multiple_of(BITWISE_MEMORY_CELLS),
            "bitwise_builtin segment length is not a multiple of it's cells_per_instance"
        );

        let n_instances = segment_length / BITWISE_MEMORY_CELLS;
        assert!(
            n_instances.is_power_of_two(),
            "bitwise_builtin instances number is not a power of two"
        );
        bitwise_builtin::ClaimGenerator::new(n_instances.ilog2(), segment.begin_addr as u32)
    });

    claim_generator.blake_compress_opcode =
        components.contains(&"blake_compress_opcode").then(|| {
            blake_compress_opcode::ClaimGenerator::new(casm_states_by_opcode.blake_compress_opcode)
        });

    claim_generator.triple_xor_32 = components
        .contains(&"triple_xor_32")
        .then(|| triple_xor_32::ClaimGenerator::new());

    claim_generator.blake_round = components
        .contains(&"blake_round")
        .then(|| blake_round::ClaimGenerator::new(memory));

    claim_generator.blake_g = components
        .contains(&"blake_g")
        .then(|| blake_g::ClaimGenerator::new());

    claim_generator.verify_bitwise_xor_9 = components
        .contains(&"verify_bitwise_xor_9")
        .then(|| verify_bitwise_xor_9::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.verify_bitwise_xor_7 = components
        .contains(&"verify_bitwise_xor_7")
        .then(|| verify_bitwise_xor_7::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.verify_bitwise_xor_4 = components
        .contains(&"verify_bitwise_xor_4")
        .then(|| verify_bitwise_xor_4::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.verify_bitwise_xor_12 = components
        .contains(&"verify_bitwise_xor_12")
        .then(|| verify_bitwise_xor_12::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.blake_round_sigma = components
        .contains(&"blake_round_sigma")
        .then(|| blake_round_sigma::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.verify_bitwise_xor_8 = components
        .contains(&"verify_bitwise_xor_8")
        .then(|| verify_bitwise_xor_8::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.qm_31_add_mul_opcode =
        components.contains(&"qm_31_add_mul_opcode").then(|| {
            qm_31_add_mul_opcode::ClaimGenerator::new(casm_states_by_opcode.qm_31_add_mul_opcode)
        });

    claim_generator.range_check_4_4_4_4 = components
        .contains(&"range_check_4_4_4_4")
        .then(|| range_check_4_4_4_4::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.ret_opcode = components
        .contains(&"ret_opcode")
        .then(|| ret_opcode::ClaimGenerator::new(casm_states_by_opcode.ret_opcode));

    claim_generator.mul_opcode = components
        .contains(&"mul_opcode")
        .then(|| mul_opcode::ClaimGenerator::new(casm_states_by_opcode.mul_opcode));

    claim_generator.mul_opcode_small = components
        .contains(&"mul_opcode_small")
        .then(|| mul_opcode_small::ClaimGenerator::new(casm_states_by_opcode.mul_opcode_small));

    claim_generator.jump_opcode_abs = components
        .contains(&"jump_opcode_abs")
        .then(|| jump_opcode_abs::ClaimGenerator::new(casm_states_by_opcode.jump_opcode_abs));

    claim_generator.jump_opcode_double_deref =
        components.contains(&"jump_opcode_double_deref").then(|| {
            jump_opcode_double_deref::ClaimGenerator::new(
                casm_states_by_opcode.jump_opcode_double_deref,
            )
        });

    claim_generator.jump_opcode_rel = components
        .contains(&"jump_opcode_rel")
        .then(|| jump_opcode_rel::ClaimGenerator::new(casm_states_by_opcode.jump_opcode_rel));

    claim_generator.jump_opcode_rel_imm = components.contains(&"jump_opcode_rel_imm").then(|| {
        jump_opcode_rel_imm::ClaimGenerator::new(casm_states_by_opcode.jump_opcode_rel_imm)
    });

    claim_generator.jnz_opcode_non_taken =
        components.contains(&"jnz_opcode_non_taken").then(|| {
            jnz_opcode_non_taken::ClaimGenerator::new(casm_states_by_opcode.jnz_opcode_non_taken)
        });

    claim_generator.jnz_opcode_taken = components
        .contains(&"jnz_opcode_taken")
        .then(|| jnz_opcode_taken::ClaimGenerator::new(casm_states_by_opcode.jnz_opcode_taken));

    claim_generator.call_opcode_rel_imm = components.contains(&"call_opcode_rel_imm").then(|| {
        call_opcode_rel_imm::ClaimGenerator::new(casm_states_by_opcode.call_opcode_rel_imm)
    });

    claim_generator.call_opcode_abs = components
        .contains(&"call_opcode_abs")
        .then(|| call_opcode_abs::ClaimGenerator::new(casm_states_by_opcode.call_opcode_abs));

    claim_generator.assert_eq_opcode_imm =
        components.contains(&"assert_eq_opcode_imm").then(|| {
            assert_eq_opcode_imm::ClaimGenerator::new(casm_states_by_opcode.assert_eq_opcode_imm)
        });

    claim_generator.assert_eq_opcode_double_deref = components
        .contains(&"assert_eq_opcode_double_deref")
        .then(|| {
            assert_eq_opcode_double_deref::ClaimGenerator::new(
                casm_states_by_opcode.assert_eq_opcode_double_deref,
            )
        });

    claim_generator.assert_eq_opcode = components
        .contains(&"assert_eq_opcode")
        .then(|| assert_eq_opcode::ClaimGenerator::new(casm_states_by_opcode.assert_eq_opcode));

    claim_generator.add_opcode = components
        .contains(&"add_opcode")
        .then(|| add_opcode::ClaimGenerator::new(casm_states_by_opcode.add_opcode));

    claim_generator.add_opcode_small = components
        .contains(&"add_opcode_small")
        .then(|| add_opcode_small::ClaimGenerator::new(casm_states_by_opcode.add_opcode_small));

    claim_generator.add_ap_opcode = components
        .contains(&"add_ap_opcode")
        .then(|| add_ap_opcode::ClaimGenerator::new(casm_states_by_opcode.add_ap_opcode));

    claim_generator.generic_opcode = components
        .contains(&"generic_opcode")
        .then(|| generic_opcode::ClaimGenerator::new(casm_states_by_opcode.generic_opcode));

    claim_generator.range_check_11 = components
        .contains(&"range_check_11")
        .then(|| range_check_11::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_18 = components
        .contains(&"range_check_18")
        .then(|| range_check_18::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_20 = components
        .contains(&"range_check_20")
        .then(|| range_check_20::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.verify_instruction = components
        .contains(&"verify_instruction")
        .then(|| verify_instruction::ClaimGenerator::new());

    claim_generator.range_check_4_3 = components
        .contains(&"range_check_4_3")
        .then(|| range_check_4_3::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.range_check_7_2_5 = components
        .contains(&"range_check_7_2_5")
        .then(|| range_check_7_2_5::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.memory_id_to_big = components
        .contains(&"memory_id_to_big")
        .then(|| memory_id_to_big::ClaimGenerator::new(memory));

    claim_generator.range_check_9_9 = components
        .contains(&"range_check_9_9")
        .then(|| range_check_9_9::ClaimGenerator::new(Arc::clone(preprocessed_trace)));

    claim_generator.memory_address_to_id = components
        .contains(&"memory_address_to_id")
        .then(|| memory_address_to_id::ClaimGenerator::new(memory));
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
                "pedersen_points_table",
                "range_check_20",
                "partial_ec_mul",
                "pedersen_aggregator",
                "pedersen_builtin",
            ]
        }
        _ => panic!("Unknown component: {component_name}",),
    }
}
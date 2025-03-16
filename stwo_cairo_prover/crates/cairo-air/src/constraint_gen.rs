mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    use itertools::{chain, Itertools};
    use stwo_prover::constraint_framework::expr::{BaseExpr, ColumnExpr, ExprEvaluator, ExtExpr};
    use stwo_prover::constraint_framework::FrameworkEval;

    use crate::components::range_check_vector::{
        range_check_12, range_check_18, range_check_3_3_3_3_3, range_check_3_6,
        range_check_3_6_6_3, range_check_4_4,
    };
    use crate::components::*;
    use crate::relations::{
        self, Cube252, MemoryAddressToId, MemoryIdToBig, PartialEcMul, Poseidon3PartialRoundsChain,
        PoseidonFullRoundChain, RangeCheckFelt252Width27, RangeCheck_3_3_3_3_3, RangeCheck_4_4,
        RangeCheck_4_4_4_4, RangeCheck_5_4, RangeCheck_6, RangeCheck_7_2_5, RangeCheck_8,
        VerifyBitwiseXor_12, VerifyBitwiseXor_4, VerifyBitwiseXor_7, VerifyBitwiseXor_8,
        VerifyBitwiseXor_9,
    };

    #[test]
    fn gen_code() {
        let path = "/home/andrew/projects/stwo-cairo/stwo_cairo_verifier/crates/cairo_air/src/";
        gen_memory_id_to_big(path);
        gen_memory_id_to_small(path);
        gen_memory_addr_to_id(path);

        gen_add_constraint_code(path);
        gen_add_small_constraint_code(path);
        gen_add_ap_constraint_code(path);

        gen_assert_eq_constraint_code(path);
        gen_assert_eq_imm_constraint_code(path);
        gen_assert_eq_double_deref_constraint_code(path);

        gen_blake2s_compress_constraint_code(path);

        gen_call_constraint_code(path);
        gen_call_op_1_base_fp_constraint_code(path);
        gen_call_rel_constraint_code(path);

        gen_jump_constraint_code(path);
        gen_jump_double_deref_constraint_code(path);
        gen_jump_rel_constraint_code(path);
        gen_jump_rel_imm_constraint_code(path);

        gen_blake_g_constraint_code(path);
        gen_blake_round_constraint_code(path);
        gen_blake_round_sigma_constraint_code(path);
        gen_tripple_xor_32_constraint_code(path);

        gen_jnz_constraint_code(path);
        // gen_jnz_dst_base_fp_constraint_code(path);
        gen_jnz_taken_constraint_code(path);
        // gen_jnz_taken_dst_base_fp_constraint_code(path);

        // gen_mul_small_imm_constraint_code(path);

        gen_qm31_add_mul_opcode_constraint_code(path);
        gen_ret_constraint_code(path);

        gen_pedersen_points_table_constraint_code(path);
        gen_partial_ec_mul_constraint_code(path);
        gen_poseidon_round_keys_constraint_code(path);
        gen_poseidon_full_round_chain_constraint_code(path);
        gen_poseidon_3_partial_rounds_chain_constraint_code(path);
        gen_cube_252_constraint_code(path);
        gen_range_check_felt_252_width_27_constraint_code(path);
        gen_poseidon_builtin_constraint_code(path);
        gen_pedersen_builtin_constraint_code(path);

        gen_generic_opcode_code(path);
        gen_mul_constraint_code(path);
        gen_mul_small_constraint_code(path);
        // gen_mul_imm_constraint_code(path);

        gen_verify_instruction_code(path);

        gen_rc_4_3_code(path);
        gen_rc_19_code(path);
        gen_rc_7_2_5_code(path);
        gen_rc_9_9_code(path);
        gen_rc_3_6_code(path);
        gen_rc_11_code(path);
        gen_rc_3_6_6_3_code(path);
        gen_rc_6_code(path);
        gen_rc_8_code(path);
        gen_rc_12_code(path);
        gen_rc_18_code(path);
        gen_rc_4_4_code(path);
        gen_rc_5_4_code(path);
        gen_rc_4_4_4_4_code(path);
        gen_rc_3_3_3_3_3_code(path);

        gen_range_check_builtin_96(path);
        gen_range_check_builtin_128(path);
        gen_bitwise_builtin(path);
        gen_add_mod_builtin_constraint_code(path);
        gen_mul_mod_builtin_constraint_code(path);
        gen_xor_4_bit(path);
        gen_xor_7_bit(path);
        gen_xor_8_bit(path);
        gen_xor_9_bit(path);
        gen_xor_12_bit(path);
    }

    fn gen_range_check_builtin_96(path: &str) {
        let componenet_eval = range_check_builtin_bits_96::Eval {
            claim: range_check_builtin_bits_96::Claim {
                log_size: 16,
                range_check96_builtin_segment_start: 987612,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
            range_check_6_lookup_elements: RangeCheck_6::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_builtin_bits_96/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_poseidon_builtin_constraint_code(path: &str) {
        let componenet_eval = poseidon_builtin::Eval {
            claim: poseidon_builtin::Claim {
                log_size: 16,
                poseidon_builtin_segment_start: 999,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            range_check_4_4_lookup_elements: RangeCheck_4_4::dummy(),
            range_check_4_4_4_4_lookup_elements: RangeCheck_4_4_4_4::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
            cube_252_lookup_elements: Cube252::dummy(),
            poseidon_3_partial_rounds_chain_lookup_elements: Poseidon3PartialRoundsChain::dummy(),
            poseidon_full_round_chain_lookup_elements: PoseidonFullRoundChain::dummy(),
            range_check_felt_252_width_27_lookup_elements: RangeCheckFelt252Width27::dummy(),
            range_check_3_3_3_3_3_lookup_elements: RangeCheck_3_3_3_3_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/poseidon_builtin/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_pedersen_builtin_constraint_code(path: &str) {
        let componenet_eval = pedersen_builtin::Eval {
            claim: pedersen_builtin::Claim {
                log_size: 16,
                pedersen_builtin_segment_start: 999,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
            partial_ec_mul_lookup_elements: PartialEcMul::dummy(),
            range_check_5_4_lookup_elements: RangeCheck_5_4::dummy(),
            range_check_8_lookup_elements: RangeCheck_8::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/pedersen_builtin/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_pedersen_points_table_constraint_code(path: &str) {
        let componenet_eval = pedersen_points_table::Eval {
            claim: pedersen_points_table::Claim {},
            pedersen_points_table_lookup_elements: relations::PedersenPointsTable::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/pedersen_points_table/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_partial_ec_mul_constraint_code(path: &str) {
        let componenet_eval = partial_ec_mul::Eval {
            claim: partial_ec_mul::Claim { log_size: 16 },
            partial_ec_mul_lookup_elements: relations::PartialEcMul::dummy(),
            pedersen_points_table_lookup_elements: relations::PedersenPointsTable::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/partial_ec_mul/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_poseidon_round_keys_constraint_code(path: &str) {
        let componenet_eval = poseidon_round_keys::Eval {
            claim: poseidon_round_keys::Claim {},
            poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/poseidon_round_keys/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_poseidon_full_round_chain_constraint_code(path: &str) {
        let componenet_eval = poseidon_full_round_chain::Eval {
            claim: poseidon_full_round_chain::Claim { log_size: 16 },
            cube_252_lookup_elements: relations::Cube252::dummy(),
            poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain::dummy(),
            poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys::dummy(),
            range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/poseidon_full_round_chain/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_poseidon_3_partial_rounds_chain_constraint_code(path: &str) {
        let componenet_eval = poseidon_3_partial_rounds_chain::Eval {
            claim: poseidon_3_partial_rounds_chain::Claim { log_size: 16 },
            cube_252_lookup_elements: relations::Cube252::dummy(),
            poseidon_3_partial_rounds_chain_lookup_elements:
                relations::Poseidon3PartialRoundsChain::dummy(),
            poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys::dummy(),
            range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
            range_check_4_4_lookup_elements: relations::RangeCheck_4_4::dummy(),
            range_check_felt_252_width_27_lookup_elements:
                relations::RangeCheckFelt252Width27::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/poseidon_3_partial_rounds_chain/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_cube_252_constraint_code(path: &str) {
        let componenet_eval = cube_252::Eval {
            claim: cube_252::Claim { log_size: 16 },
            cube_252_lookup_elements: relations::Cube252::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/cube_252/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_range_check_felt_252_width_27_constraint_code(path: &str) {
        let componenet_eval = range_check_felt_252_width_27::Eval {
            claim: range_check_felt_252_width_27::Claim { log_size: 16 },
            range_check_felt_252_width_27_lookup_elements:
                relations::RangeCheckFelt252Width27::dummy(),
            range_check_18_lookup_elements: relations::RangeCheck_18::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_felt_252_width_27/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_blake_g_constraint_code(path: &str) {
        let componenet_eval = blake_g::Eval {
            claim: blake_g::Claim { log_size: 16 },
            blake_g_lookup_elements: relations::BlakeG::dummy(),
            verify_bitwise_xor_12_lookup_elements: VerifyBitwiseXor_12::dummy(),
            verify_bitwise_xor_4_lookup_elements: VerifyBitwiseXor_4::dummy(),
            verify_bitwise_xor_7_lookup_elements: VerifyBitwiseXor_7::dummy(),
            verify_bitwise_xor_8_lookup_elements: VerifyBitwiseXor_8::dummy(),
            verify_bitwise_xor_9_lookup_elements: VerifyBitwiseXor_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/blake_g/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_tripple_xor_32_constraint_code(path: &str) {
        let componenet_eval = triple_xor_32::Eval {
            claim: triple_xor_32::Claim { log_size: 16 },
            triple_xor_32_lookup_elements: relations::TripleXor32::dummy(),
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/tripple_xor_32/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_add_mod_builtin_constraint_code(path: &str) {
        let componenet_eval = add_mod_builtin::Eval {
            claim: add_mod_builtin::Claim {
                log_size: 16,
                add_mod_builtin_segment_start: 999,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/add_mod_builtin/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_mul_mod_builtin_constraint_code(path: &str) {
        let componenet_eval = mul_mod_builtin::Eval {
            claim: mul_mod_builtin::Claim {
                log_size: 16,
                mul_mod_builtin_segment_start: 999,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
            range_check_12_lookup_elements: relations::RangeCheck_12::dummy(),
            range_check_18_lookup_elements: relations::RangeCheck_18::dummy(),
            range_check_3_6_6_3_lookup_elements: relations::RangeCheck_3_6_6_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/mul_mod_builtin/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_blake_round_constraint_code(path: &str) {
        let componenet_eval = blake_round::Eval {
            claim: blake_round::Claim { log_size: 16 },
            blake_g_lookup_elements: relations::BlakeG::dummy(),
            blake_round_lookup_elements: relations::BlakeRound::dummy(),
            blake_round_sigma_lookup_elements: relations::BlakeRoundSigma::dummy(),
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
            range_check_7_2_5_lookup_elements: RangeCheck_7_2_5::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/blake_round/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_blake_round_sigma_constraint_code(path: &str) {
        let componenet_eval = blake_round_sigma::Eval {
            claim: blake_round_sigma::Claim {},
            blake_round_sigma_lookup_elements: relations::BlakeRoundSigma::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/blake_round_sigma/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_range_check_builtin_128(path: &str) {
        let componenet_eval = range_check_builtin_bits_128::Eval {
            claim: range_check_builtin_bits_128::Claim {
                log_size: 16,
                range_check_builtin_segment_start: 777373,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_builtin_bits_128/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_xor_4_bit(path: &str) {
        let componenet_eval = verify_bitwise_xor_4::Eval {
            claim: verify_bitwise_xor_4::Claim {},
            verify_bitwise_xor_4_lookup_elements: VerifyBitwiseXor_4::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/verify_bitwise_xor_4/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_xor_7_bit(path: &str) {
        let componenet_eval = verify_bitwise_xor_7::Eval {
            claim: verify_bitwise_xor_7::Claim {},
            verify_bitwise_xor_7_lookup_elements: VerifyBitwiseXor_7::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/verify_bitwise_xor_7/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_xor_8_bit(path: &str) {
        let componenet_eval = verify_bitwise_xor_8::Eval {
            claim: verify_bitwise_xor_8::Claim {},
            verify_bitwise_xor_8_lookup_elements: VerifyBitwiseXor_8::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/verify_bitwise_xor_8/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_xor_9_bit(path: &str) {
        let componenet_eval = verify_bitwise_xor_9::Eval {
            claim: verify_bitwise_xor_9::Claim {},
            verify_bitwise_xor_9_lookup_elements: VerifyBitwiseXor_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/verify_bitwise_xor_9/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_xor_12_bit(path: &str) {
        let componenet_eval = verify_bitwise_xor_12::Eval {
            claim: verify_bitwise_xor_12::Claim {},
            verify_bitwise_xor_12_lookup_elements: VerifyBitwiseXor_12::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/verify_bitwise_xor_12/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_bitwise_builtin(path: &str) {
        let componenet_eval = bitwise_builtin::Eval {
            claim: bitwise_builtin::Claim {
                log_size: 16,
                bitwise_builtin_segment_start: 777373,
            },
            memory_address_to_id_lookup_elements: MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: MemoryIdToBig::dummy(),
            verify_bitwise_xor_9_lookup_elements: VerifyBitwiseXor_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/bitwise_builtin/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_memory_id_to_big(path: &str) {
        let componenet_eval = memory_id_to_big::BigEval {
            log_n_rows: 4,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/memory_id_to_big/constraints_big.cairo",
        )
        .unwrap();
    }

    fn gen_memory_id_to_small(path: &str) {
        let componenet_eval = memory_id_to_big::SmallEval {
            log_n_rows: 4,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_relation: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/memory_id_to_big/constraints_small.cairo",
        )
        .unwrap();
    }

    fn gen_memory_addr_to_id(path: &str) {
        let componenet_eval = memory_address_to_id::Eval {
            log_size: 4,
            lookup_elements: relations::MemoryAddressToId::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/memory_address_to_id/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_verify_instruction_code(path: &str) {
        let componenet_eval = verify_instruction::Eval {
            claim: verify_instruction::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            range_check_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/verify_instruction/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_qm31_add_mul_opcode_constraint_code(path: &str) {
        let componenet_eval = qm_31_add_mul_opcode::Eval {
            claim: qm_31_add_mul_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/qm31_add_mul_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_ret_constraint_code(path: &str) {
        let componenet_eval = ret_opcode::Eval {
            claim: ret_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/ret_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_generic_opcode_code(path: &str) {
        let componenet_eval = generic_opcode::Eval {
            claim: generic_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/generic_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_4_3_code(path: &str) {
        let componenet_eval = range_check_4_3::Eval {
            lookup_elements: relations::RangeCheck_4_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_4_3_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_4_4_code(path: &str) {
        let componenet_eval = range_check_4_4::Eval {
            lookup_elements: relations::RangeCheck_4_4::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_4_4_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_5_4_code(path: &str) {
        let componenet_eval = range_check_5_4::Eval {
            lookup_elements: relations::RangeCheck_5_4::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_5_4_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_7_2_5_code(path: &str) {
        let componenet_eval = range_check_7_2_5::Eval {
            lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_7_2_5_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_9_9_code(path: &str) {
        let componenet_eval = range_check_9_9::Eval {
            lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_9_9_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_3_6_code(path: &str) {
        let componenet_eval = range_check_3_6::Eval {
            lookup_elements: relations::RangeCheck_3_6::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_3_6_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_19_code(path: &str) {
        let componenet_eval = range_check_19::Eval {
            lookup_elements: relations::RangeCheck_19::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_19_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_18_code(path: &str) {
        let componenet_eval = range_check_18::Eval {
            lookup_elements: relations::RangeCheck_18::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_18_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_11_code(path: &str) {
        let componenet_eval = range_check_11::Eval {
            lookup_elements: relations::RangeCheck_11::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_11_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_3_6_6_3_code(path: &str) {
        let componenet_eval = range_check_3_6_6_3::Eval {
            lookup_elements: relations::RangeCheck_3_6_6_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_3_6_6_3_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_4_4_4_4_code(path: &str) {
        let componenet_eval = range_check_4_4_4_4::Eval {
            lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_4_4_4_4_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_3_3_3_3_3_code(path: &str) {
        let componenet_eval = range_check_3_3_3_3_3::Eval {
            lookup_elements: relations::RangeCheck_3_3_3_3_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_3_3_3_3_3_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_6_code(path: &str) {
        let componenet_eval = range_check_6::Eval {
            lookup_elements: relations::RangeCheck_6::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_6_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_8_code(path: &str) {
        let componenet_eval = range_check_8::Eval {
            lookup_elements: relations::RangeCheck_8::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_8_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_12_code(path: &str) {
        let componenet_eval = range_check_12::Eval {
            lookup_elements: relations::RangeCheck_12::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/range_check_vector/rc_12_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_add_constraint_code(path: &str) {
        println!("WHOOOOOOO\n=====\n=====");

        let componenet_eval = add_opcode::Eval {
            claim: add_opcode::Claim { log_size: 1 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/add_opcode/constraints.cairo",
        )
        .unwrap();
    }

    // fn gen_add_imm_constraint_code(path: &str) {
    //     let componenet_eval = add_opcode_imm::Eval {
    //         claim: add_opcode_imm::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/add_opcode_imm/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    fn gen_add_small_constraint_code(path: &str) {
        let componenet_eval = add_opcode_small::Eval {
            claim: add_opcode_small::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/add_opcode_small/constraints.cairo",
        )
        .unwrap();
    }

    // fn gen_add_small_imm_constraint_code(path: &str) {
    //     let componenet_eval = add_opcode_small_imm::Eval {
    //         claim: add_opcode_small_imm::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/add_opcode_small_imm/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    fn gen_add_ap_constraint_code(path: &str) {
        let componenet_eval = add_ap_opcode::Eval {
            claim: add_ap_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/add_ap_opcode/constraints.cairo",
        )
        .unwrap();
    }

    // fn gen_add_ap_op_1_base_fp_constraint_code(path: &str) {
    //     let componenet_eval = add_ap_opcode_op_1_base_fp::Eval {
    //         claim: add_ap_opcode_op_1_base_fp::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/add_ap_opcode_op_1_base_fp/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    // fn gen_add_ap_imm_constraint_code(path: &str) {
    //     let componenet_eval = add_ap_opcode_imm::Eval {
    //         claim: add_ap_opcode_imm::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/add_ap_opcode_imm/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    fn gen_assert_eq_constraint_code(path: &str) {
        let componenet_eval = assert_eq_opcode::Eval {
            claim: assert_eq_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/assert_eq_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_assert_eq_imm_constraint_code(path: &str) {
        let componenet_eval = assert_eq_opcode_imm::Eval {
            claim: assert_eq_opcode_imm::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/assert_eq_opcode_imm/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_assert_eq_double_deref_constraint_code(path: &str) {
        let componenet_eval = assert_eq_opcode_double_deref::Eval {
            claim: assert_eq_opcode_double_deref::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/assert_eq_opcode_double_deref/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_blake2s_compress_constraint_code(path: &str) {
        let componenet_eval = blake_compress_opcode::Eval {
            claim: blake_compress_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            blake_round_lookup_elements: relations::BlakeRound::dummy(),
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
            triple_xor_32_lookup_elements: relations::TripleXor32::dummy(),
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/blake_compress_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_call_constraint_code(path: &str) {
        let componenet_eval = call_opcode::Eval {
            claim: call_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/call_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_call_op_1_base_fp_constraint_code(path: &str) {
        let componenet_eval = call_opcode_op_1_base_fp::Eval {
            claim: call_opcode_op_1_base_fp::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/call_opcode_op_1_base_fp/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_call_rel_constraint_code(path: &str) {
        let componenet_eval = call_opcode_rel::Eval {
            claim: call_opcode_rel::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/call_opcode_rel/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_jump_constraint_code(path: &str) {
        let componenet_eval = jump_opcode::Eval {
            claim: jump_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/jump_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_jump_double_deref_constraint_code(path: &str) {
        let componenet_eval = jump_opcode_double_deref::Eval {
            claim: jump_opcode_double_deref::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/jump_opcode_double_deref/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_jump_rel_constraint_code(path: &str) {
        let componenet_eval = jump_opcode_rel::Eval {
            claim: jump_opcode_rel::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/jump_opcode_rel/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_jump_rel_imm_constraint_code(path: &str) {
        let componenet_eval = jump_opcode_rel_imm::Eval {
            claim: jump_opcode_rel_imm::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/jump_opcode_rel_imm/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_jnz_constraint_code(path: &str) {
        let componenet_eval = jnz_opcode::Eval {
            claim: jnz_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/jnz_opcode/constraints.cairo",
        )
        .unwrap();
    }

    // fn gen_jnz_dst_base_fp_constraint_code(path: &str) {
    //     let componenet_eval = jnz_opcode_dst_base_fp::Eval {
    //         claim: jnz_opcode_dst_base_fp::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/jnz_opcode_dst_base_fp/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    fn gen_jnz_taken_constraint_code(path: &str) {
        let componenet_eval = jnz_opcode_taken::Eval {
            claim: jnz_opcode_taken::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/jnz_opcode_taken/constraints.cairo",
        )
        .unwrap();
    }

    // fn gen_jnz_taken_dst_base_fp_constraint_code(path: &str) {
    //     let componenet_eval = jnz_opcode_taken_dst_base_fp::Eval {
    //         claim: jnz_opcode_taken_dst_base_fp::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/jnz_opcode_taken_dst_base_fp/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    fn gen_mul_constraint_code(path: &str) {
        let componenet_eval = mul_opcode::Eval {
            claim: mul_opcode::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/mul_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_mul_small_constraint_code(path: &str) {
        let componenet_eval = mul_opcode_small::Eval {
            claim: mul_opcode_small::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            range_check_11_lookup_elements: relations::RangeCheck_11::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new()),
            path.to_string() + "components/mul_opcode_small/constraints.cairo",
        )
        .unwrap();
    }

    // fn gen_mul_small_imm_constraint_code(path: &str) {
    //     let componenet_eval = mul_opcode_small_imm::Eval {
    //         claim: mul_opcode_small_imm::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //         range_check_11_lookup_elements: relations::RangeCheck_11::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/mul_opcode_small_imm/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    // fn gen_mul_imm_constraint_code(path: &str) {
    //     let componenet_eval = mul_opcode_imm::Eval {
    //         claim: mul_opcode_imm::Claim { log_size: 0 },
    //         memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
    //         memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
    //         verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
    //         opcodes_lookup_elements: relations::Opcodes::dummy(),
    //         range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
    //     };

    //     gen_cairo_constraint_code(
    //         componenet_eval.evaluate(ExprEvaluator::new()),
    //         path.to_string() + "components/mul_opcode_imm/constraints.cairo",
    //     )
    //     .unwrap();
    // }

    fn gen_cairo_constraint_code(
        constraint_expr: ExprEvaluator,
        output_path: impl AsRef<Path>,
    ) -> std::io::Result<()> {
        if let Some(parent_path) = output_path.as_ref().parent() {
            fs::create_dir_all(parent_path)?;
        }

        let mut file = File::create(&output_path)?;
        let code = component_constraint_code(constraint_expr);
        let code = code.replace("seq_4", "seq");
        let code = code.replace("seq_16", "seq");
        file.write_all(code.as_bytes());

        Ok(())
    }

    #[derive(Clone, Copy)]
    pub struct ColumnExprPub {
        pub interaction: usize,
        pub idx: usize,
        pub offset: isize,
    }

    const CLAIMED_SUM_DUMMY_OFFSET: usize = stwo_prover::core::fields::m31::P as usize;

    fn component_constraint_code(expression: ExprEvaluator) -> String {
        // Offsets relative to the component.
        let mut preprocessed_column_indices = BTreeSet::new();
        let mut trace_column_offsets = BTreeMap::<usize, BTreeSet<isize>>::new();
        let mut interaction_column_offsets = BTreeMap::<usize, BTreeSet<isize>>::new();

        let collect_cols = &mut |node: &BaseExpr| {
            if let BaseExpr::Col(column_expr) = node {
                let ColumnExprPub {
                    interaction,
                    idx,
                    offset,
                } = *unsafe { core::mem::transmute::<&ColumnExpr, &ColumnExprPub>(column_expr) };

                match interaction {
                    // Preprocessed column.
                    0 => {
                        assert!(offset == 0);
                        preprocessed_column_indices.insert(idx);
                    }
                    // Base trace.
                    1 => {
                        trace_column_offsets.entry(idx).or_default().insert(offset);
                    }
                    // Interaction trace.
                    2 => {
                        interaction_column_offsets
                            .entry(idx)
                            .or_default()
                            .insert(offset);
                    }
                    _ => panic!(),
                }
            }
        };

        let mut parameters = BTreeSet::new();

        for intermediate in expression
            .intermediates
            .iter()
            .map(|(_, expr)| expr.simplify())
        {
            intermediate.traverse(collect_cols);
            intermediate.traverse(&mut |base_expr| {
                if let BaseExpr::Param(param) = base_expr {
                    parameters.insert(param.clone());
                }
            });
        }

        for constraint in chain![
            expression
                .ext_intermediates
                .iter()
                .map(|(_, expr)| expr.simplify()),
            expression.constraints.iter().map(|expr| expr.simplify())
        ] {
            constraint.traverse(&mut |node| {
                if let ExtExpr::SecureCol(base_exprs) = node {
                    base_exprs.iter().for_each(|base_expr| {
                        base_expr.traverse(collect_cols);
                        base_expr.traverse(&mut |base_expr| {
                            if let BaseExpr::Param(param) = base_expr {
                                parameters.insert(param.clone());
                            }
                        });
                    });
                }

                if let ExtExpr::Param(param) = node {
                    parameters.insert(param.clone());
                }
            });
        }

        assert!(preprocessed_column_indices.is_empty());

        // Sanity check uses all columns.
        let trace_col_min = *trace_column_offsets.keys().min().unwrap_or(&0);
        let trace_col_max = *trace_column_offsets.keys().max().unwrap_or(&0);
        (trace_col_min..=trace_col_max)
            .for_each(|i| assert!(trace_column_offsets.get(&i).is_some()));
        let interaction_col_min = *interaction_column_offsets
            .keys()
            .min()
            .unwrap_or(&trace_col_max);
        let interaction_col_max = *interaction_column_offsets
            .keys()
            .max()
            .unwrap_or(&interaction_col_min);
        (interaction_col_min..=interaction_col_max)
            .for_each(|i| assert!(interaction_column_offsets.get(&i).is_some()));

        let unique_offsets = chain![
            trace_column_offsets.values(),
            interaction_column_offsets.values()
        ]
        .fold(BTreeSet::new(), |a, b| &a | b);

        let offset_var_name = |offset: isize| {
            if offset == 0 {
                return "point".to_string();
            }

            if offset == CLAIMED_SUM_DUMMY_OFFSET as isize {
                return "point_offset_claimed_sum".to_string();
            }

            let offset_abs = offset.abs();

            if offset > 0 {
                format!("point_offset_{offset_abs}")
            } else {
                format!("point_offset_neg_{offset_abs}")
            }
        };

        let offset_variables = unique_offsets.iter().filter_map(|offset| {
            if *offset == 0 {
                return None;
            }

            let offset_abs = offset.abs();

            if *offset > 0 {
                Some(format!("let point_offset_{offset_abs} = point.add_circle_point_m31(trace_gen.mul({offset_abs}).to_point());"))
            } else {
                Some(format!("let point_offset_neg_{offset_abs} = point.add_circle_point_m31(-trace_gen.mul({offset_abs}).to_point());"))
            }
        }).join("\n");

        let trace_mask_points = (trace_col_min..=trace_col_max)
            .map(|i| {
                let column_offsets = &trace_column_offsets[&i];
                let column_mask_points = column_offsets
                    .iter()
                    .map(|&offset| offset_var_name(offset))
                    .join(",");
                format!("trace_mask_points.append(array![{column_mask_points}]);")
            })
            .join("\n");

        let interaction_mask_points = (interaction_col_min..=interaction_col_max)
            .map(|i| {
                let column_offsets = &interaction_column_offsets[&i];
                let column_mask_points = column_offsets
                    .iter()
                    .map(|&offset| offset_var_name(offset))
                    .join(",");
                format!("interaction_trace_mask_points.append(array![{column_mask_points}]);")
            })
            .join("\n");

        let mut all_params = BTreeSet::new();

        let intermediate_functions = expression
            .intermediates
            .iter()
            .map(|(name, expr)| {
                let params = extract_params(&expr.simplify());
                all_params.extend(params.clone());
                let params_str = params.into_iter().map(|v| format!("{v}: QM31")).join(",");
                let expr_str = expr.simplify_and_format();
                format!(
                    r#"pub fn {name}({params_str}) -> QM31 {{
                    {expr_str}
                }}"#
                )
            })
            .join("\n\n");

        let intermediate_ext_functions = expression
            .ext_intermediates
            .iter()
            .map(|(name, expr)| {
                let params = extract_ext_params(&expr.simplify());
                all_params.extend(params.clone());
                let params_str = params.into_iter().map(|v| format!("{v}: QM31")).join(",");
                let expr_str = expr.simplify_and_format();
                format!(
                    r#"pub fn {name}({params_str}) -> QM31 {{
                    {expr_str}
                }}"#
                )
            })
            .join("\n\n");

        let all_params = all_params
            .iter()
            .filter(|v| !v.starts_with("intermediate"))
            .collect::<BTreeSet<&String>>();
        let all_params_str = all_params.iter().map(|v| format!("{v}: QM31")).join(",");
        let all_params_simple_str = all_params.iter().map(|v| format!("{v}")).join(",");

        let preprocessed_params = all_params
            .iter()
            .filter_map(|&param| {
                if param.starts_with("is_first") {
                    Some("preprocessed_column_set.insert(PreprocessedColumn::IsFirst(log_size));")
                } else if param.starts_with("seq_") {
                    Some("preprocessed_column_set.insert(PreprocessedColumn::Seq(log_size));")
                } else if param.starts_with("bitwise_xor_") {
                    // TODO
                    None
                } else {
                    None
                }
            })
            .join("\n");

        let mask_function = format!(
            r#"
        pub fn mask_points(
            ref preprocessed_column_set: PreprocessedColumnSet,
            ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
            ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
            point: CirclePoint<QM31>,
            trace_gen: CirclePointIndex,
            log_size: u32,
        ) {{
            {preprocessed_params}
            {offset_variables}
            {trace_mask_points}
            {interaction_mask_points}
        }}"#
        );

        let n_trace_columns = trace_col_max - trace_col_min + 1;
        let trace_column_labels = (trace_col_min..=trace_col_max)
            .map(|i| format!("trace_1_column_{i}"))
            .join(",");
        let trace_columns = format!("let [{trace_column_labels}]: [Span<QM31>; {n_trace_columns}] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();");

        let trace_mask_columns_and_offsets = (trace_col_min..=trace_col_max)
            .map(|i| {
                let offsets = &trace_column_offsets[&i];
                let n_offsets = offsets.len();
                let offset_labels = offsets.iter().map(|&offset| {
                    let offset_abs = offset.abs();
                    if offset >= 0 {
                        format!("trace_1_column_{i}_offset_{offset_abs}")
                    } else {
                        format!("trace_1_column_{i}_offset_neg_{offset_abs}")
                    }
                }).join(",");


                format!(r#"
                    let [{offset_labels}]: [QM31; {n_offsets}] = (*trace_1_column_{i}.try_into().unwrap()).unbox();
                "#)
            })
            .join("\n");

        let n_interaction_columns = interaction_col_max - interaction_col_min + 1;
        let interaction_column_labels = (interaction_col_min..=interaction_col_max)
            .map(|i| format!("trace_2_column_{i}"))
            .join(",");
        let interaction_columns = format!("let [{interaction_column_labels}]: [Span<QM31>; {n_interaction_columns}] = (*interaction_mask_values.multi_pop_front().unwrap()).unbox();");

        let interaction_mask_columns_and_offsets = (interaction_col_min..=interaction_col_max)
            .map(|i| {
                let offsets = &interaction_column_offsets[&i];
                let n_offsets = offsets.len();
                let offset_labels = offsets.iter().map(|&offset| {
                    let offset_abs = offset.abs();
                    if offset >= 0 {
                        format!("trace_2_column_{i}_offset_{offset_abs}")
                    } else {
                        format!("trace_2_column_{i}_offset_neg_{offset_abs}")
                    }
                }).join(",");


                format!(r#"
                    let [{offset_labels}]: [QM31; {n_offsets}] = (*trace_2_column_{i}.try_into().unwrap()).unbox();
                "#)
            })
            .join("\n");

        let lets_string = expression
            .intermediates
            .iter()
            .sorted_by_key(|(name, _)| name[12..].parse::<usize>().unwrap())
            .map(|(name, expr)| {
                let params = extract_params(&expr.simplify()).join(",");
                format!("let {name} = {name}({params});")
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let secure_lets_string = expression
            .ext_intermediates
            .iter()
            .map(|(name, expr)| {
                let params = extract_ext_params(&expr.simplify()).join(",");
                format!("let {name} = {name}({params});")
            })
            .collect::<Vec<String>>()
            .join("\n\n");
        let all_vals_str = (0..expression.intermediates.len() + expression.ext_intermediates.len())
            .map(|i| format!("intermediate{}", i))
            .join(",");
        let get_intermediates = format!(
            r#"
        fn intermediates({all_params_str}) -> Array<QM31> {{
        {lets_string}
        {secure_lets_string}
        array![{all_vals_str}]
        }}"#
        );
        let unwrap_intermediates = (0..expression.intermediates.len()
            + expression.ext_intermediates.len())
            .map(|i| format!("let intermediate{i} = *intermediates.pop_front().unwrap();"))
            .join("\n");

        let constraint_str = expression.custom_fmt();

        let mut constraint_parameters = parameters
            .iter()
            .filter_map(|parameter| {
                if parameter.starts_with("intermediate") {
                    return None;
                }

                // if parameter

                Some(format!("pub {parameter}: QM31,"))
            })
            .join("\n");

        constraint_parameters += "pub column_size: M31,";

        let mut constraint_parameter_keys = parameters
            .iter()
            .filter_map(|parameter| {
                if parameter.starts_with("intermediate") {
                    return None;
                }

                Some(format!("{parameter},"))
            })
            .join("\n");
        constraint_parameter_keys += "column_size,";

        let constraint_eval_function = format!(
            r#" 
        #[derive(Drop)]
        pub struct ConstraintParams {{
            {constraint_parameters}
        }}

        pub fn evaluate_constraints_at_point(
            ref sum: QM31,
            ref trace_mask_values: ColumnSpan<Span<QM31>>,
            ref interaction_mask_values: ColumnSpan<Span<QM31>>,
            params: ConstraintParams,
            random_coeff: QM31,
            domain_vanish_at_point_inv: QM31,
        ) {{
            let ConstraintParams {{
                {constraint_parameter_keys}
            }} = params;
            {trace_columns}
            {trace_mask_columns_and_offsets}
            {interaction_columns}
            {interaction_mask_columns_and_offsets}
            core::internal::revoke_ap_tracking();

            let mut intermediates = intermediates({all_params_simple_str}).span();
            {unwrap_intermediates}

            {constraint_str}
        }}

        {get_intermediates}

        {intermediate_functions}
        {intermediate_ext_functions}
        "#
        );

        format!(
            r#"
            use stwo_constraint_framework::{{PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl}};
            use stwo_verifier_core::{{ColumnSpan, ColumnArray}};
            use stwo_verifier_core::circle::{{CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl}};
            use stwo_verifier_core::fields::m31::{{m31, M31}};
            use stwo_verifier_core::fields::qm31::{{QM31, QM31Impl, qm31_const}};
            use stwo_verifier_core::fields::Invertible;

        {mask_function}
        {constraint_eval_function}
        "#
        )
    }

    trait Traversable {
        fn traverse(&self, f: &mut impl FnMut(&Self));
    }

    impl Traversable for ExtExpr {
        fn traverse(&self, f: &mut impl FnMut(&Self)) {
            match self {
                // Tree types are recursed first
                ExtExpr::Neg(a) => a.traverse(f),
                ExtExpr::SecureCol(_) | ExtExpr::Param(_) | ExtExpr::Const(_) => {}
                ExtExpr::Sub(a, b) | ExtExpr::Add(a, b) | ExtExpr::Mul(a, b) => {
                    a.traverse(f);
                    b.traverse(f);
                }
            };

            f(self);
        }
    }

    impl Traversable for BaseExpr {
        fn traverse(&self, f: &mut impl FnMut(&Self)) {
            match self {
                // Tree types are recursed first
                BaseExpr::Neg(a) | BaseExpr::Inv(a) => a.traverse(f),
                BaseExpr::Col(_) | BaseExpr::Param(_) | BaseExpr::Const(_) => {}
                BaseExpr::Sub(a, b) | BaseExpr::Add(a, b) | BaseExpr::Mul(a, b) => {
                    a.traverse(f);
                    b.traverse(f);
                }
            };

            f(self);
        }
    }

    trait CustomFmt {
        fn custom_fmt(&self) -> String;
    }

    impl CustomFmt for ExprEvaluator {
        fn custom_fmt(&self) -> String {
            let constraints_str = self
                .constraints
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let expr = c.simplify_and_format();
                    format!(
                        r#"
                        // Constraint {i}
                        let constraint_quotient = ({expr}) * domain_vanish_at_point_inv;
                        sum = sum * random_coeff + constraint_quotient;
                    "#
                    )
                })
                .collect::<Vec<String>>()
                .join("");

            [constraints_str]
                .iter()
                .filter(|x| !x.is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join("\n\n")
        }
    }

    fn extract_params(expr: &BaseExpr) -> Vec<String> {
        let mut params = BTreeSet::new();
        expr.traverse(&mut |e| {
            if let BaseExpr::Param(p) = e {
                params.insert(p.clone());
            }
        });
        expr.traverse(&mut |e| {
            if let BaseExpr::Col(col) = e {
                // let col = unsafe { core::mem::transmute::<&ColumnExpr, &ColumnExprPub>(col) };
                params.insert(e.format_expr());
            }
        });
        params.into_iter().collect()
    }

    fn extract_ext_params(expr: &ExtExpr) -> Vec<String> {
        let mut params = BTreeSet::new();
        expr.traverse(&mut |e| {
            if let ExtExpr::Param(p) = e {
                params.insert(p.clone());
            }
        });
        expr.traverse(&mut |e| {
            if let ExtExpr::SecureCol(col) = e {
                col.each_ref().map(|v| params.extend(extract_params(v)));
            }
        });
        params.into_iter().collect()
    }
}

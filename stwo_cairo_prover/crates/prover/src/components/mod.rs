pub mod add_ap_opcode;
pub mod add_ap_opcode_imm;
pub mod add_ap_opcode_op_1_base_fp;
pub mod add_mod_builtin;
pub mod add_opcode;
pub mod add_opcode_imm;
pub mod add_opcode_small;
pub mod add_opcode_small_imm;
pub mod assert_eq_opcode;
pub mod assert_eq_opcode_double_deref;
pub mod assert_eq_opcode_imm;
pub mod bitwise_builtin;
pub mod call_opcode;
pub mod call_opcode_op_1_base_fp;
pub mod call_opcode_rel;
pub mod generic_opcode;
pub mod jnz_opcode;
pub mod jnz_opcode_dst_base_fp;
pub mod jnz_opcode_taken;
pub mod jnz_opcode_taken_dst_base_fp;
pub mod jump_opcode;
pub mod jump_opcode_double_deref;
pub mod jump_opcode_rel;
pub mod jump_opcode_rel_imm;
pub mod memory;
pub mod mul_opcode;
pub mod mul_opcode_imm;
pub mod mul_opcode_small;
pub mod mul_opcode_small_imm;
pub mod qm_31_add_mul_opcode;
pub mod range_check_builtin_bits_128;
pub mod range_check_builtin_bits_96;
pub mod range_check_vector;
pub mod ret_opcode;
pub mod triple_xor_32;
pub mod utils;
pub mod verify_bitwise_xor_12;
pub mod verify_bitwise_xor_4;
pub mod verify_bitwise_xor_7;
pub mod verify_bitwise_xor_8;
pub mod verify_bitwise_xor_9;
pub mod verify_instruction;

mod prelude;

pub use memory::{memory_address_to_id, memory_id_to_big};
pub use range_check_vector::{
    range_check_11, range_check_19, range_check_4_3, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_9_9,
};
pub mod blake_compress_opcode;
pub mod blake_g;
pub mod blake_round;
pub mod blake_round_sigma;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use cairo_vm::air_public_input::MemorySegmentAddresses;
    use itertools::Itertools;
    use stwo_cairo_adapter::memory::MemoryEntryIter;
    use stwo_cairo_adapter::vm_import::generate_test_input;

    use crate::cairo_air::debug_tools::assert_constraints::assert_cairo_constraints;
    use crate::cairo_air::preprocessed::tests::testing_preprocessed_tree;

    #[test]
    fn test_add_mod_builtin_constraints() {
        let input = generate_test_input("test_prove_verify_add_mod_builtin");
        let pp_tree = testing_preprocessed_tree(19);
        assert_cairo_constraints(input, pp_tree);
    }

    fn assert_bitwise_builtin_has_holes(
        test_name: &str,
        bitwise_segment: &Option<MemorySegmentAddresses>,
    ) {
        let bitwise_segment = bitwise_segment.as_ref().unwrap();
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("../../test_data/");
        d.push(test_name);
        let mut memory_file = std::io::BufReader::new(File::open(d.join("mem").as_path()).unwrap());
        let memory_entries = MemoryEntryIter(&mut memory_file).collect_vec();
        assert!(memory_entries
            .iter()
            .all(|entry| entry.address != (bitwise_segment.begin_addr + 2) as u64));
    }

    #[test]
    fn test_bitwise_builtin_constraints() {
        let input = generate_test_input("test_prove_verify_bitwise_builtin");
        assert_bitwise_builtin_has_holes(
            "test_prove_verify_bitwise_builtin",
            &input.builtins_segments.bitwise,
        );
        let pp_tree = testing_preprocessed_tree(19);
        assert_cairo_constraints(input, pp_tree);
    }

    #[test]
    fn test_rc_96_builtin_constraints() {
        let input = generate_test_input("test_prove_verify_range_check_bits_96_builtin");
        let pp_tree = testing_preprocessed_tree(19);
        assert_cairo_constraints(input, pp_tree);
    }

    #[test]
    fn test_rc_128_builtin_constraints() {
        let input = generate_test_input("test_prove_verify_range_check_bits_128_builtin");
        let pp_tree = testing_preprocessed_tree(19);
        assert_cairo_constraints(input, pp_tree);
    }
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
/// These tests' inputs were generated using cairo-vm with 50 instances of each builtin.
pub mod builtin_tests {
    use itertools::Itertools;
    use stwo_cairo_adapter::vm_import::generate_test_input;
    use stwo_cairo_adapter::ProverInput;
    use stwo_prover::core::pcs::PcsConfig;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    use crate::cairo_air::prover::prove_cairo;
    use crate::cairo_air::prover::tests::test_cfg;
    use crate::cairo_air::verifier::verify_cairo;

    /// Asserts that all builtins are present in the input.
    /// Panics if any of the builtins is missing.
    fn assert_all_builtins_in_input(input: &ProverInput) {
        let empty_builtins = input
            .builtins_segments
            .get_counts()
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(name, _)| format!("{:?}", name))
            .collect_vec();
        assert!(
            empty_builtins.is_empty(),
            "The following builtins are missing: {}",
            empty_builtins.join(", ")
        );
    }

    #[test]
    fn test_prove_verify_all_builtins() {
        let input = generate_test_input("test_prove_verify_all_builtins");
        assert_all_builtins_in_input(&input);
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(input, test_cfg(), PcsConfig::default()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof, PcsConfig::default()).unwrap();
    }

    #[test]
    fn test_prove_verify_add_mod_builtin() {
        let input = generate_test_input("test_prove_verify_add_mod_builtin");
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(input, test_cfg(), PcsConfig::default()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof, PcsConfig::default()).unwrap();
    }

    #[test]
    fn test_prove_verify_bitwise_builtin() {
        let input = generate_test_input("test_prove_verify_bitwise_builtin");
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(input, test_cfg(), PcsConfig::default()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof, PcsConfig::default()).unwrap();
    }

    #[test]
    fn test_prove_verify_mul_mod_builtin() {
        let input = generate_test_input("test_prove_verify_mul_mod_builtin");
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(input, test_cfg(), PcsConfig::default()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof, PcsConfig::default()).unwrap();
    }

    #[test]
    fn test_prove_verify_range_check_bits_96_builtin() {
        let input = generate_test_input("test_prove_verify_range_check_bits_96_builtin");
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(input, test_cfg(), PcsConfig::default()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof, PcsConfig::default()).unwrap();
    }

    #[test]
    fn test_prove_verify_range_check_bits_128_builtin() {
        let input = generate_test_input("test_prove_verify_range_check_bits_128_builtin");
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(input, test_cfg(), PcsConfig::default()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof, PcsConfig::default()).unwrap();
    }
}

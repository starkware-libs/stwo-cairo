// This file was created by the AIR team.

use std::array::from_fn;

use stwo::core::air::Component;
use stwo::core::fields::qm31::QM31;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::TraceLocationAllocator;

use crate::claims::{CairoClaim, CairoInteractionClaim};
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::*;
use crate::relations::CommonLookupElements;

pub struct CairoComponents {
    pub add_opcode: Option<add_opcode::Component>,
    pub add_opcode_small: Option<add_opcode_small::Component>,
    pub add_ap_opcode: Option<add_ap_opcode::Component>,
    pub assert_eq_opcode: Option<assert_eq_opcode::Component>,
    pub assert_eq_opcode_imm: Option<assert_eq_opcode_imm::Component>,
    pub assert_eq_opcode_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    pub blake_compress_opcode: Option<blake_compress_opcode::Component>,
    pub call_opcode_abs: Option<call_opcode_abs::Component>,
    pub call_opcode_rel_imm: Option<call_opcode_rel_imm::Component>,
    pub generic_opcode: Option<generic_opcode::Component>,
    pub jnz_opcode_non_taken: Option<jnz_opcode_non_taken::Component>,
    pub jnz_opcode_taken: Option<jnz_opcode_taken::Component>,
    pub jump_opcode_abs: Option<jump_opcode_abs::Component>,
    pub jump_opcode_double_deref: Option<jump_opcode_double_deref::Component>,
    pub jump_opcode_rel: Option<jump_opcode_rel::Component>,
    pub jump_opcode_rel_imm: Option<jump_opcode_rel_imm::Component>,
    pub mul_opcode: Option<mul_opcode::Component>,
    pub mul_opcode_small: Option<mul_opcode_small::Component>,
    pub qm_31_add_mul_opcode: Option<qm_31_add_mul_opcode::Component>,
    pub ret_opcode: Option<ret_opcode::Component>,
    pub verify_instruction: Option<verify_instruction::Component>,
    pub blake_round: Option<blake_round::Component>,
    pub blake_g: Option<blake_g::Component>,
    pub blake_round_sigma: Option<blake_round_sigma::Component>,
    pub triple_xor_32: Option<triple_xor_32::Component>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::Component>,
    pub add_mod_builtin: Option<add_mod_builtin::Component>,
    pub bitwise_builtin: Option<bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<pedersen_builtin::Component>,
    pub poseidon_builtin: Option<poseidon_builtin::Component>,
    pub range_check96_builtin: Option<range_check96_builtin::Component>,
    pub range_check_builtin: Option<range_check_builtin::Component>,
    pub pedersen_aggregator_window_bits_18: Option<pedersen_aggregator_window_bits_18::Component>,
    pub partial_ec_mul_window_bits_18: Option<partial_ec_mul_window_bits_18::Component>,
    pub pedersen_points_table_window_bits_18:
        Option<pedersen_points_table_window_bits_18::Component>,
    pub poseidon_aggregator: Option<poseidon_aggregator::Component>,
    pub poseidon_3_partial_rounds_chain: Option<poseidon_3_partial_rounds_chain::Component>,
    pub poseidon_full_round_chain: Option<poseidon_full_round_chain::Component>,
    pub cube_252: Option<cube_252::Component>,
    pub poseidon_round_keys: Option<poseidon_round_keys::Component>,
    pub range_check_252_width_27: Option<range_check_252_width_27::Component>,
    pub memory_address_to_id: Option<memory_address_to_id::Component>,
    pub memory_id_to_value: (
        Vec<memory_id_to_big::BigComponent>,
        memory_id_to_big::SmallComponent,
    ),
    pub range_check_6: Option<range_check_6::Component>,
    pub range_check_8: Option<range_check_8::Component>,
    pub range_check_11: Option<range_check_11::Component>,
    pub range_check_12: Option<range_check_12::Component>,
    pub range_check_18: Option<range_check_18::Component>,
    pub range_check_20: Option<range_check_20::Component>,
    pub range_check_4_3: Option<range_check_4_3::Component>,
    pub range_check_4_4: Option<range_check_4_4::Component>,
    pub range_check_9_9: Option<range_check_9_9::Component>,
    pub range_check_7_2_5: Option<range_check_7_2_5::Component>,
    pub range_check_3_6_6_3: Option<range_check_3_6_6_3::Component>,
    pub range_check_4_4_4_4: Option<range_check_4_4_4_4::Component>,
    pub range_check_3_3_3_3_3: Option<range_check_3_3_3_3_3::Component>,
    pub verify_bitwise_xor_4: Option<verify_bitwise_xor_4::Component>,
    pub verify_bitwise_xor_7: Option<verify_bitwise_xor_7::Component>,
    pub verify_bitwise_xor_8: Option<verify_bitwise_xor_8::Component>,
    pub verify_bitwise_xor_9: Option<verify_bitwise_xor_9::Component>,
}

impl CairoComponents {
    pub fn new(
        claim: &CairoClaim,
        common_lookup_elements: &CommonLookupElements,
        interaction_claim: &CairoInteractionClaim,

        preprocessed_column_ids: &[PreProcessedColumnId],
    ) -> Self {
        let tree_span_provider =
            &mut TraceLocationAllocator::new_with_preprocessed_columns(preprocessed_column_ids);
        let claim_iter = claim.components_log_sizes.iter();
        let interaction_claim_iter = interaction_claim.iter();

        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let add_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let add_opcode_component = add_opcode::Component::new(
                tree_span_provider,
                add_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let add_opcode_small_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let add_opcode_small_component = add_opcode_small::Component::new(
                tree_span_provider,
                add_opcode_small::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let add_ap_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let add_ap_opcode_component = add_ap_opcode::Component::new(
                tree_span_provider,
                add_ap_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let assert_eq_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let assert_eq_opcode_component = assert_eq_opcode::Component::new(
                tree_span_provider,
                assert_eq_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let assert_eq_opcode_imm_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let assert_eq_opcode_imm_component = assert_eq_opcode_imm::Component::new(
                tree_span_provider,
                assert_eq_opcode_imm::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let assert_eq_opcode_double_deref_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let assert_eq_opcode_double_deref_component =
                assert_eq_opcode_double_deref::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_double_deref::Eval {
                        log_size: *log_size,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    *claimed_sum,
                );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let blake_compress_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let blake_compress_opcode_component = blake_compress_opcode::Component::new(
                tree_span_provider,
                blake_compress_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let call_opcode_abs_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let call_opcode_abs_component = call_opcode_abs::Component::new(
                tree_span_provider,
                call_opcode_abs::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let call_opcode_rel_imm_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let call_opcode_rel_imm_component = call_opcode_rel_imm::Component::new(
                tree_span_provider,
                call_opcode_rel_imm::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let generic_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let generic_opcode_component = generic_opcode::Component::new(
                tree_span_provider,
                generic_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let jnz_opcode_non_taken_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let jnz_opcode_non_taken_component = jnz_opcode_non_taken::Component::new(
                tree_span_provider,
                jnz_opcode_non_taken::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let jnz_opcode_taken_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let jnz_opcode_taken_component = jnz_opcode_taken::Component::new(
                tree_span_provider,
                jnz_opcode_taken::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let jump_opcode_abs_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let jump_opcode_abs_component = jump_opcode_abs::Component::new(
                tree_span_provider,
                jump_opcode_abs::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let jump_opcode_double_deref_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let jump_opcode_double_deref_component = jump_opcode_double_deref::Component::new(
                tree_span_provider,
                jump_opcode_double_deref::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let jump_opcode_rel_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let jump_opcode_rel_component = jump_opcode_rel::Component::new(
                tree_span_provider,
                jump_opcode_rel::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let jump_opcode_rel_imm_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let jump_opcode_rel_imm_component = jump_opcode_rel_imm::Component::new(
                tree_span_provider,
                jump_opcode_rel_imm::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let mul_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let mul_opcode_component = mul_opcode::Component::new(
                tree_span_provider,
                mul_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let mul_opcode_small_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let mul_opcode_small_component = mul_opcode_small::Component::new(
                tree_span_provider,
                mul_opcode_small::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let qm_31_add_mul_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let qm_31_add_mul_opcode_component = qm_31_add_mul_opcode::Component::new(
                tree_span_provider,
                qm_31_add_mul_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let ret_opcode_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let ret_opcode_component = ret_opcode::Component::new(
                tree_span_provider,
                ret_opcode::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let verify_instruction_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let verify_instruction_component = verify_instruction::Component::new(
                tree_span_provider,
                verify_instruction::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let blake_round_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let blake_round_component = blake_round::Component::new(
                tree_span_provider,
                blake_round::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let blake_g_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let blake_g_component = blake_g::Component::new(
                tree_span_provider,
                blake_g::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let blake_round_sigma_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let blake_round_sigma_component = blake_round_sigma::Component::new(
                tree_span_provider,
                blake_round_sigma::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let triple_xor_32_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let triple_xor_32_component = triple_xor_32::Component::new(
                tree_span_provider,
                triple_xor_32::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let verify_bitwise_xor_12_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let verify_bitwise_xor_12_component = verify_bitwise_xor_12::Component::new(
                tree_span_provider,
                verify_bitwise_xor_12::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let add_mod_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let add_mod_builtin_component = add_mod_builtin::Component::new(
                tree_span_provider,
                add_mod_builtin::Eval {
                    log_size: *log_size,
                    add_mod_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .add_mod_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let bitwise_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let bitwise_builtin_component = bitwise_builtin::Component::new(
                tree_span_provider,
                bitwise_builtin::Eval {
                    log_size: *log_size,
                    bitwise_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .bitwise_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let mul_mod_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let mul_mod_builtin_component = mul_mod_builtin::Component::new(
                tree_span_provider,
                mul_mod_builtin::Eval {
                    log_size: *log_size,
                    mul_mod_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .mul_mod_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let pedersen_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let pedersen_builtin_component = pedersen_builtin::Component::new(
                tree_span_provider,
                pedersen_builtin::Eval {
                    log_size: *log_size,
                    pedersen_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .pedersen_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let poseidon_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let poseidon_builtin_component = poseidon_builtin::Component::new(
                tree_span_provider,
                poseidon_builtin::Eval {
                    log_size: *log_size,
                    poseidon_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .poseidon_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check96_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check96_builtin_component = range_check96_builtin::Component::new(
                tree_span_provider,
                range_check96_builtin::Eval {
                    log_size: *log_size,
                    range_check96_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .range_check96_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_builtin_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_builtin_component = range_check_builtin::Component::new(
                tree_span_provider,
                range_check_builtin::Eval {
                    log_size: *log_size,
                    range_check_builtin_segment_start: claim
                        .public_data
                        .public_memory
                        .public_segments
                        .range_check_builtin
                        .as_ref()
                        .unwrap()
                        .start_ptr
                        .value as u32,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let pedersen_aggregator_window_bits_18_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let pedersen_aggregator_window_bits_18_component =
                pedersen_aggregator_window_bits_18::Component::new(
                    tree_span_provider,
                    pedersen_aggregator_window_bits_18::Eval {
                        log_size: *log_size,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    *claimed_sum,
                );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let partial_ec_mul_window_bits_18_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let partial_ec_mul_window_bits_18_component =
                partial_ec_mul_window_bits_18::Component::new(
                    tree_span_provider,
                    partial_ec_mul_window_bits_18::Eval {
                        log_size: *log_size,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    *claimed_sum,
                );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let pedersen_points_table_window_bits_18_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let pedersen_points_table_window_bits_18_component =
                pedersen_points_table_window_bits_18::Component::new(
                    tree_span_provider,
                    pedersen_points_table_window_bits_18::Eval {
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    *claimed_sum,
                );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let poseidon_aggregator_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let poseidon_aggregator_component = poseidon_aggregator::Component::new(
                tree_span_provider,
                poseidon_aggregator::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let poseidon_3_partial_rounds_chain_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let poseidon_3_partial_rounds_chain_component =
                poseidon_3_partial_rounds_chain::Component::new(
                    tree_span_provider,
                    poseidon_3_partial_rounds_chain::Eval {
                        log_size: *log_size,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                    *claimed_sum,
                );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let poseidon_full_round_chain_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let poseidon_full_round_chain_component = poseidon_full_round_chain::Component::new(
                tree_span_provider,
                poseidon_full_round_chain::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let cube_252_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let cube_252_component = cube_252::Component::new(
                tree_span_provider,
                cube_252::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let poseidon_round_keys_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let poseidon_round_keys_component = poseidon_round_keys::Component::new(
                tree_span_provider,
                poseidon_round_keys::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_252_width_27_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_252_width_27_component = range_check_252_width_27::Component::new(
                tree_span_provider,
                range_check_252_width_27::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let memory_address_to_id_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let memory_address_to_id_component = memory_address_to_id::Component::new(
                tree_span_provider,
                memory_address_to_id::Eval {
                    log_size: *log_size,
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let big_log_sizes: [Option<u32>; MEMORY_ADDRESS_TO_ID_SPLIT] =
            from_fn(|_| *claim_iter.next().unwrap());
        let big_log_sizes: Vec<u32> = big_log_sizes
            .into_iter()
            .flatten()
            .collect();
        let big_claimed_sums: [Option<QM31>; MEMORY_ADDRESS_TO_ID_SPLIT] =
            from_fn(|_| *interaction_claim_iter.next().unwrap());
        let big_claimed_sums: Vec<QM31> = big_log_sizes
            .into_iter()
            .flatten()
            .collect();
        let memory_id_to_big_component = memory_id_to_big::big_components_from_claim(
            &big_log_sizes,
            &big_claimed_sums,
            &common_lookup_elements.clone(),
            tree_span_provider,
        );

        let small_log_size = claim_iter.next().unwrap().unwrap();
        let small_claimed_sum = interaction_claim_iter.next().unwrap().unwrap();
        let memory_id_to_small_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(small_log_size, common_lookup_elements.clone()),
            small_claimed_sum,
        );
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_6_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_6_component = range_check_6::Component::new(
                tree_span_provider,
                range_check_6::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_8_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_8_component = range_check_8::Component::new(
                tree_span_provider,
                range_check_8::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_11_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_11_component = range_check_11::Component::new(
                tree_span_provider,
                range_check_11::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_12_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_12_component = range_check_12::Component::new(
                tree_span_provider,
                range_check_12::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_18_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_18_component = range_check_18::Component::new(
                tree_span_provider,
                range_check_18::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_20_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_20_component = range_check_20::Component::new(
                tree_span_provider,
                range_check_20::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_4_3_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_4_3_component = range_check_4_3::Component::new(
                tree_span_provider,
                range_check_4_3::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_4_4_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_4_4_component = range_check_4_4::Component::new(
                tree_span_provider,
                range_check_4_4::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_9_9_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_9_9_component = range_check_9_9::Component::new(
                tree_span_provider,
                range_check_9_9::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_7_2_5_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_7_2_5_component = range_check_7_2_5::Component::new(
                tree_span_provider,
                range_check_7_2_5::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_3_6_6_3_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_3_6_6_3_component = range_check_3_6_6_3::Component::new(
                tree_span_provider,
                range_check_3_6_6_3::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_4_4_4_4_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_4_4_4_4_component = range_check_4_4_4_4::Component::new(
                tree_span_provider,
                range_check_4_4_4_4::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let range_check_3_3_3_3_3_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let range_check_3_3_3_3_3_component = range_check_3_3_3_3_3::Component::new(
                tree_span_provider,
                range_check_3_3_3_3_3::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let verify_bitwise_xor_4_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let verify_bitwise_xor_4_component = verify_bitwise_xor_4::Component::new(
                tree_span_provider,
                verify_bitwise_xor_4::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let verify_bitwise_xor_7_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let verify_bitwise_xor_7_component = verify_bitwise_xor_7::Component::new(
                tree_span_provider,
                verify_bitwise_xor_7::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let verify_bitwise_xor_8_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let verify_bitwise_xor_8_component = verify_bitwise_xor_8::Component::new(
                tree_span_provider,
                verify_bitwise_xor_8::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        let log_size = claim_iter.next().unwrap();
        let claimed_sum = interaction_claim_iter.next().unwrap();
        assert!(log_size.is_some() == claimed_sum.is_some());
        let verify_bitwise_xor_9_component = None;
        if let (Some(log_size), Some(claimed_sum)) = (log_size, claimed_sum) {
            let verify_bitwise_xor_9_component = verify_bitwise_xor_9::Component::new(
                tree_span_provider,
                verify_bitwise_xor_9::Eval {
                    common_lookup_elements: common_lookup_elements.clone(),
                },
                *claimed_sum,
            );
        }
        assert!(claim_iter.next().is_none());
        assert!(interaction_claim_iter.next().is_none());

        Self {
            add_opcode: add_opcode_component,
            add_opcode_small: add_opcode_small_component,
            add_ap_opcode: add_ap_opcode_component,
            assert_eq_opcode: assert_eq_opcode_component,
            assert_eq_opcode_imm: assert_eq_opcode_imm_component,
            assert_eq_opcode_double_deref: assert_eq_opcode_double_deref_component,
            blake_compress_opcode: blake_compress_opcode_component,
            call_opcode_abs: call_opcode_abs_component,
            call_opcode_rel_imm: call_opcode_rel_imm_component,
            generic_opcode: generic_opcode_component,
            jnz_opcode_non_taken: jnz_opcode_non_taken_component,
            jnz_opcode_taken: jnz_opcode_taken_component,
            jump_opcode_abs: jump_opcode_abs_component,
            jump_opcode_double_deref: jump_opcode_double_deref_component,
            jump_opcode_rel: jump_opcode_rel_component,
            jump_opcode_rel_imm: jump_opcode_rel_imm_component,
            mul_opcode: mul_opcode_component,
            mul_opcode_small: mul_opcode_small_component,
            qm_31_add_mul_opcode: qm_31_add_mul_opcode_component,
            ret_opcode: ret_opcode_component,
            verify_instruction: verify_instruction_component,
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_round_sigma: blake_round_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check96_builtin: range_check96_builtin_component,
            range_check_builtin: range_check_builtin_component,
            pedersen_aggregator_window_bits_18: pedersen_aggregator_window_bits_18_component,
            partial_ec_mul_window_bits_18: partial_ec_mul_window_bits_18_component,
            pedersen_points_table_window_bits_18: pedersen_points_table_window_bits_18_component,
            poseidon_aggregator: poseidon_aggregator_component,
            poseidon_3_partial_rounds_chain: poseidon_3_partial_rounds_chain_component,
            poseidon_full_round_chain: poseidon_full_round_chain_component,
            cube_252: cube_252_component,
            poseidon_round_keys: poseidon_round_keys_component,
            range_check_252_width_27: range_check_252_width_27_component,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_big_component, memory_id_to_small_component),
            range_check_6: range_check_6_component,
            range_check_8: range_check_8_component,
            range_check_11: range_check_11_component,
            range_check_12: range_check_12_component,
            range_check_18: range_check_18_component,
            range_check_20: range_check_20_component,
            range_check_4_3: range_check_4_3_component,
            range_check_4_4: range_check_4_4_component,
            range_check_9_9: range_check_9_9_component,
            range_check_7_2_5: range_check_7_2_5_component,
            range_check_3_6_6_3: range_check_3_6_6_3_component,
            range_check_4_4_4_4: range_check_4_4_4_4_component,
            range_check_3_3_3_3_3: range_check_3_3_3_3_3_component,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        let mut components = vec![];
        if let Some(component) = &self.add_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.add_opcode_small {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.add_ap_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.assert_eq_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.assert_eq_opcode_imm {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.assert_eq_opcode_double_deref {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_compress_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.call_opcode_abs {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.call_opcode_rel_imm {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.generic_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.jnz_opcode_non_taken {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.jnz_opcode_taken {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_abs {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_double_deref {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_rel {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.jump_opcode_rel_imm {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.mul_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.mul_opcode_small {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.qm_31_add_mul_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.ret_opcode {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_instruction {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_round {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_g {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.blake_round_sigma {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.triple_xor_32 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_12 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.add_mod_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.bitwise_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.mul_mod_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check96_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_builtin {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_aggregator_window_bits_18 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.partial_ec_mul_window_bits_18 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.pedersen_points_table_window_bits_18 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_aggregator {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_3_partial_rounds_chain {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_full_round_chain {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.cube_252 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.poseidon_round_keys {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_252_width_27 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.memory_address_to_id {
            components.push(component as &dyn Component);
        }
        components.extend(
            self.memory_id_to_value
                .0
                .iter()
                .map(|component| component as &dyn Component),
        );
        components.push(&self.memory_id_to_value.1 as &dyn Component);
        if let Some(component) = &self.range_check_6 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_8 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_11 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_12 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_18 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_20 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_4_3 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_4_4 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_9_9 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_7_2_5 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_3_6_6_3 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_4_4_4_4 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.range_check_3_3_3_3_3 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_4 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_7 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_8 {
            components.push(component as &dyn Component);
        }
        if let Some(component) = &self.verify_bitwise_xor_9 {
            components.push(component as &dyn Component);
        }
        components
    }
}

impl std::fmt::Display for CairoComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(component) = &self.add_opcode {
            writeln!(f, "add_opcode: {}", indented_component_display(component))?;
        }

        if let Some(component) = &self.add_opcode_small {
            writeln!(
                f,
                "add_opcode_small: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.add_ap_opcode {
            writeln!(
                f,
                "add_ap_opcode: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.assert_eq_opcode {
            writeln!(
                f,
                "assert_eq_opcode: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.assert_eq_opcode_imm {
            writeln!(
                f,
                "assert_eq_opcode_imm: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.assert_eq_opcode_double_deref {
            writeln!(
                f,
                "assert_eq_opcode_double_deref: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.blake_compress_opcode {
            writeln!(
                f,
                "blake_compress_opcode: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.call_opcode_abs {
            writeln!(
                f,
                "call_opcode_abs: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.call_opcode_rel_imm {
            writeln!(
                f,
                "call_opcode_rel_imm: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.generic_opcode {
            writeln!(
                f,
                "generic_opcode: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.jnz_opcode_non_taken {
            writeln!(
                f,
                "jnz_opcode_non_taken: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.jnz_opcode_taken {
            writeln!(
                f,
                "jnz_opcode_taken: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.jump_opcode_abs {
            writeln!(
                f,
                "jump_opcode_abs: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.jump_opcode_double_deref {
            writeln!(
                f,
                "jump_opcode_double_deref: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.jump_opcode_rel {
            writeln!(
                f,
                "jump_opcode_rel: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.jump_opcode_rel_imm {
            writeln!(
                f,
                "jump_opcode_rel_imm: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.mul_opcode {
            writeln!(f, "mul_opcode: {}", indented_component_display(component))?;
        }

        if let Some(component) = &self.mul_opcode_small {
            writeln!(
                f,
                "mul_opcode_small: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.qm_31_add_mul_opcode {
            writeln!(
                f,
                "qm_31_add_mul_opcode: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.ret_opcode {
            writeln!(f, "ret_opcode: {}", indented_component_display(component))?;
        }

        if let Some(component) = &self.verify_instruction {
            writeln!(
                f,
                "verify_instruction: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.blake_round {
            writeln!(f, "blake_round: {}", indented_component_display(component))?;
        }

        if let Some(component) = &self.blake_g {
            writeln!(f, "blake_g: {}", indented_component_display(component))?;
        }

        if let Some(component) = &self.blake_round_sigma {
            writeln!(
                f,
                "blake_round_sigma: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.triple_xor_32 {
            writeln!(
                f,
                "triple_xor_32: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.verify_bitwise_xor_12 {
            writeln!(
                f,
                "verify_bitwise_xor_12: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.add_mod_builtin {
            writeln!(
                f,
                "add_mod_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.bitwise_builtin {
            writeln!(
                f,
                "bitwise_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.mul_mod_builtin {
            writeln!(
                f,
                "mul_mod_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.pedersen_builtin {
            writeln!(
                f,
                "pedersen_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.poseidon_builtin {
            writeln!(
                f,
                "poseidon_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check96_builtin {
            writeln!(
                f,
                "range_check96_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_builtin {
            writeln!(
                f,
                "range_check_builtin: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.pedersen_aggregator_window_bits_18 {
            writeln!(
                f,
                "pedersen_aggregator_window_bits_18: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.partial_ec_mul_window_bits_18 {
            writeln!(
                f,
                "partial_ec_mul_window_bits_18: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.pedersen_points_table_window_bits_18 {
            writeln!(
                f,
                "pedersen_points_table_window_bits_18: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.poseidon_aggregator {
            writeln!(
                f,
                "poseidon_aggregator: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.poseidon_3_partial_rounds_chain {
            writeln!(
                f,
                "poseidon_3_partial_rounds_chain: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.poseidon_full_round_chain {
            writeln!(
                f,
                "poseidon_full_round_chain: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.cube_252 {
            writeln!(f, "cube_252: {}", indented_component_display(component))?;
        }

        if let Some(component) = &self.poseidon_round_keys {
            writeln!(
                f,
                "poseidon_round_keys: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_252_width_27 {
            writeln!(
                f,
                "range_check_252_width_27: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.memory_address_to_id {
            writeln!(
                f,
                "memory_address_to_id: {}",
                indented_component_display(component)
            )?;
        }

        for component in &self.memory_id_to_value.0 {
            {
                writeln!(
                    f,
                    "memory_id_to_value big: {}",
                    indented_component_display(component)
                )?;
            }
        }
        writeln!(
            f,
            "memory_id_to_value small: {}",
            indented_component_display(&self.memory_id_to_value.1)
        )?;

        if let Some(component) = &self.range_check_6 {
            writeln!(
                f,
                "range_check_6: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_8 {
            writeln!(
                f,
                "range_check_8: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_11 {
            writeln!(
                f,
                "range_check_11: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_12 {
            writeln!(
                f,
                "range_check_12: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_18 {
            writeln!(
                f,
                "range_check_18: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_20 {
            writeln!(
                f,
                "range_check_20: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_4_3 {
            writeln!(
                f,
                "range_check_4_3: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_4_4 {
            writeln!(
                f,
                "range_check_4_4: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_9_9 {
            writeln!(
                f,
                "range_check_9_9: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_7_2_5 {
            writeln!(
                f,
                "range_check_7_2_5: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_3_6_6_3 {
            writeln!(
                f,
                "range_check_3_6_6_3: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_4_4_4_4 {
            writeln!(
                f,
                "range_check_4_4_4_4: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.range_check_3_3_3_3_3 {
            writeln!(
                f,
                "range_check_3_3_3_3_3: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.verify_bitwise_xor_4 {
            writeln!(
                f,
                "verify_bitwise_xor_4: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.verify_bitwise_xor_7 {
            writeln!(
                f,
                "verify_bitwise_xor_7: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.verify_bitwise_xor_8 {
            writeln!(
                f,
                "verify_bitwise_xor_8: {}",
                indented_component_display(component)
            )?;
        }

        if let Some(component) = &self.verify_bitwise_xor_9 {
            writeln!(
                f,
                "verify_bitwise_xor_9: {}",
                indented_component_display(component)
            )?;
        }
        Ok(())
    }
}

// This file was created by the AIR team.

use core::array::Span;
use stwo_cairo_air::{PublicData, PublicDataImpl, RelationUsesDict};
use stwo_constraint_framework::claim::ClaimTrait;
use stwo_constraint_framework::{CommonLookupElements, accumulate_relation_uses};
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use stwo_verifier_core::utils::pack_into_qm31s;
use crate::component_indices::*;
use crate::components::*;
use crate::preprocessed_columns::PREPROCESSED_COLUMN_LOG_SIZE;
use crate::{ChannelTrait, PublicDataTrait};

#[derive(Drop, Serde, Clone)]
pub struct CairoClaim {
    pub component_log_sizes: Array<Option<u32>>,
    pub public_data: PublicData,
}

#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub component_claimed_sums: Array<Option<QM31>>,
}

pub impl CairoClaimImpl of ClaimTrait<CairoClaim> {
    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        let log_sizes = self.component_log_sizes.span();
        let mut enable_bits_as_u32s = array![];
        let mut dense_log_sizes = array![];
        for opt in log_sizes {
            if let Option::Some(log_size) = *opt {
                enable_bits_as_u32s.append(1_u32);
                dense_log_sizes.append(log_size);
            } else {
                enable_bits_as_u32s.append(0_u32);
            }
        }
        channel.mix_felts(pack_into_qm31s(array![log_sizes.len()].span()));
        channel.mix_felts(pack_into_qm31s(enable_bits_as_u32s.span()));
        channel.mix_felts(pack_into_qm31s(dense_log_sizes.span()));
        channel
            .mix_felts(
                pack_into_qm31s(array![self.public_data.public_memory.program.len()].span()),
            );
        self.public_data.mix_into(ref channel);
    }

    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let log_size_per_component = self.component_log_sizes.span();
        let mut trace_log_sizes = array![];
        let mut interaction_trace_log_sizes = array![];
        for component_idx in 0..N_COMPONENTS {
            if let Option::Some(log_size) = *log_size_per_component.at(component_idx) {
                for _ in 0..*N_TRACE_COLUMNS_PER_COMPONENT_IDX.span().at(component_idx) {
                    trace_log_sizes.append(log_size);
                }
                for _ in 0..*N_INTERACTION_COLUMNS_PER_COMPONENT_IDX.span().at(component_idx) {
                    interaction_trace_log_sizes.append(log_size);
                }
            }
        }
        array![
            PREPROCESSED_COLUMN_LOG_SIZE.span(), trace_log_sizes.span(),
            interaction_trace_log_sizes.span(),
        ]
    }

    fn accumulate_relation_uses(self: @CairoClaim, ref relation_uses: RelationUsesDict) {
        // Const-size components export no RELATION_USES_PER_ROW, so they are skipped here.
        if let Option::Some(log_size) = *self.component_log_sizes.at(ADD_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, add_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(ADD_OPCODE_SMALL_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, add_opcode_small::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(ADD_AP_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, add_ap_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(ASSERT_EQ_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, assert_eq_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(ASSERT_EQ_OPCODE_IMM_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, assert_eq_opcode_imm::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(ASSERT_EQ_OPCODE_DOUBLE_DEREF_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                assert_eq_opcode_double_deref::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(BLAKE_COMPRESS_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, blake_compress_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(CALL_OPCODE_ABS_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, call_opcode_abs::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(CALL_OPCODE_REL_IMM_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, call_opcode_rel_imm::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(GENERIC_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, generic_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(JNZ_OPCODE_NON_TAKEN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, jnz_opcode_non_taken::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(JNZ_OPCODE_TAKEN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, jnz_opcode_taken::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(JUMP_OPCODE_ABS_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, jump_opcode_abs::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(JUMP_OPCODE_DOUBLE_DEREF_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, jump_opcode_double_deref::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(JUMP_OPCODE_REL_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, jump_opcode_rel::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(JUMP_OPCODE_REL_IMM_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, jump_opcode_rel_imm::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(MUL_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, mul_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(MUL_OPCODE_SMALL_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, mul_opcode_small::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(QM_31_ADD_MUL_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, qm_31_add_mul_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(RET_OPCODE_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, ret_opcode::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(VERIFY_INSTRUCTION_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, verify_instruction::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(BLAKE_ROUND_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, blake_round::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(BLAKE_G_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, blake_g::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(TRIPLE_XOR_32_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, triple_xor_32::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(ADD_MOD_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, add_mod_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(BITWISE_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, bitwise_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(MUL_MOD_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, mul_mod_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PEDERSEN_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, pedersen_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                pedersen_builtin_narrow_windows::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(POSEIDON_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, poseidon_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, range_check96_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(RANGE_CHECK_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, range_check_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(EC_OP_BUILTIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, ec_op_builtin::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PARTIAL_EC_MUL_GENERIC_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, partial_ec_mul_generic::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                pedersen_aggregator_window_bits_18::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                partial_ec_mul_window_bits_18::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PEDERSEN_AGGREGATOR_WINDOW_BITS_9_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                pedersen_aggregator_window_bits_9::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(PARTIAL_EC_MUL_WINDOW_BITS_9_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                partial_ec_mul_window_bits_9::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(POSEIDON_AGGREGATOR_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, poseidon_aggregator::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                poseidon_3_partial_rounds_chain::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses,
                poseidon_full_round_chain::RELATION_USES_PER_ROW.span(),
                log_size,
            );
        }
        if let Option::Some(log_size) = *self.component_log_sizes.at(CUBE_252_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, cube_252::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, range_check_252_width_27::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(MEMORY_ADDRESS_TO_ID_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, memory_address_to_id::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
        for i in 0..memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT {
            if let Option::Some(log_size) = *self
                .component_log_sizes
                .at(MEMORY_ID_TO_BIG_BASE_COMPONENT_IDX + i) {
                accumulate_relation_uses(
                    ref relation_uses, memory_id_to_big::RELATION_USES_PER_ROW_BIG.span(), log_size,
                );
            }
        }
        if let Option::Some(log_size) = *self
            .component_log_sizes
            .at(MEMORY_ID_TO_SMALL_COMPONENT_IDX) {
            accumulate_relation_uses(
                ref relation_uses, memory_id_to_small::RELATION_USES_PER_ROW.span(), log_size,
            );
        }
    }
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrait {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        // Preserve the previous wire format: mix the dense list of present claimed sums.
        let mut dense_claimed_sums = array![];
        for opt in self.component_claimed_sums.span() {
            if let Option::Some(cs) = *opt {
                dense_claimed_sums.append(cs);
            }
        }
        channel.mix_felts(dense_claimed_sums.span());
    }
}

pub fn lookup_sum(
    claim: @CairoClaim, elements: @CommonLookupElements, interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);
    for cs in interaction_claim.component_claimed_sums {
        if let Option::Some(cs) = *cs {
            sum += cs;
        }
    }
    sum
}

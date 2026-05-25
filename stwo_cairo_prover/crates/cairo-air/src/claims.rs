// This file was created by the AIR team.

use serde::{Deserialize, Serialize};
use stwo::core::channel::{Channel, MerkleChannel};
use stwo::core::fields::qm31::SecureField;
use stwo::core::pcs::TreeVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::air::{accumulate_relation_uses, PublicData, RelationUsesDict};
use crate::component_indices::*;
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_SPLIT;
use crate::components::*;
use crate::relations::CommonLookupElements;
use crate::utils::pack_into_secure_felts;

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoClaim {
    pub component_log_sizes: Vec<Option<u32>>,
    pub public_data: PublicData,
}

impl CairoClaim {
    pub fn mix_into<MC: MerkleChannel>(&self, channel: &mut MC::C) {
        let log_sizes = &self.component_log_sizes;
        let mut enable_bits_as_u32s = Vec::with_capacity(log_sizes.len());
        let mut dense_log_sizes = Vec::new();
        for opt in log_sizes {
            if let Some(log_size) = *opt {
                enable_bits_as_u32s.push(1_u32);
                dense_log_sizes.push(log_size);
            } else {
                enable_bits_as_u32s.push(0_u32);
            }
        }
        channel.mix_felts(&pack_into_secure_felts(
            [log_sizes.len() as u32].into_iter(),
        ));
        channel.mix_felts(&pack_into_secure_felts(enable_bits_as_u32s.into_iter()));
        channel.mix_felts(&pack_into_secure_felts(dense_log_sizes.into_iter()));
        channel.mix_felts(&pack_into_secure_felts(
            [self.public_data.public_memory.program.len() as u32].into_iter(),
        ));
        self.public_data.mix_into::<MC>(channel);
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        // Const-size components export no RELATION_USES_PER_ROW, so they are skipped here.
        if let Some(log_size) = self.component_log_sizes[ADD_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(relation_uses, add_opcode::RELATION_USES_PER_ROW, log_size);
        }
        if let Some(log_size) = self.component_log_sizes[ADD_OPCODE_SMALL_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                add_opcode_small::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[ADD_AP_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                add_ap_opcode::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[ASSERT_EQ_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[ASSERT_EQ_OPCODE_IMM_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_imm::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) =
            self.component_log_sizes[ASSERT_EQ_OPCODE_DOUBLE_DEREF_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_double_deref::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[BLAKE_COMPRESS_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                blake_compress_opcode::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[CALL_OPCODE_ABS_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_abs::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[CALL_OPCODE_REL_IMM_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_rel_imm::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[GENERIC_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                generic_opcode::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[JNZ_OPCODE_NON_TAKEN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_non_taken::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[JNZ_OPCODE_TAKEN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_taken::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[JUMP_OPCODE_ABS_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_abs::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[JUMP_OPCODE_DOUBLE_DEREF_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_double_deref::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[JUMP_OPCODE_REL_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[JUMP_OPCODE_REL_IMM_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel_imm::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[MUL_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(relation_uses, mul_opcode::RELATION_USES_PER_ROW, log_size);
        }
        if let Some(log_size) = self.component_log_sizes[MUL_OPCODE_SMALL_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                mul_opcode_small::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[QM_31_ADD_MUL_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                qm_31_add_mul_opcode::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[RET_OPCODE_COMPONENT_IDX] {
            accumulate_relation_uses(relation_uses, ret_opcode::RELATION_USES_PER_ROW, log_size);
        }
        if let Some(log_size) = self.component_log_sizes[VERIFY_INSTRUCTION_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                verify_instruction::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[BLAKE_ROUND_COMPONENT_IDX] {
            accumulate_relation_uses(relation_uses, blake_round::RELATION_USES_PER_ROW, log_size);
        }
        if let Some(log_size) = self.component_log_sizes[BLAKE_G_COMPONENT_IDX] {
            accumulate_relation_uses(relation_uses, blake_g::RELATION_USES_PER_ROW, log_size);
        }
        if let Some(log_size) = self.component_log_sizes[TRIPLE_XOR_32_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                triple_xor_32::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[ADD_MOD_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                add_mod_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[BITWISE_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                bitwise_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[MUL_MOD_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                mul_mod_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[PEDERSEN_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                pedersen_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) =
            self.component_log_sizes[PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                pedersen_builtin_narrow_windows::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[POSEIDON_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                poseidon_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[RANGE_CHECK96_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                range_check96_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[RANGE_CHECK_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                range_check_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[EC_OP_BUILTIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                ec_op_builtin::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[PARTIAL_EC_MUL_GENERIC_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                partial_ec_mul_generic::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) =
            self.component_log_sizes[PEDERSEN_AGGREGATOR_WINDOW_BITS_18_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                pedersen_aggregator_window_bits_18::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) =
            self.component_log_sizes[PARTIAL_EC_MUL_WINDOW_BITS_18_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                partial_ec_mul_window_bits_18::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) =
            self.component_log_sizes[PEDERSEN_AGGREGATOR_WINDOW_BITS_9_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                pedersen_aggregator_window_bits_9::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[PARTIAL_EC_MUL_WINDOW_BITS_9_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                partial_ec_mul_window_bits_9::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[POSEIDON_AGGREGATOR_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                poseidon_aggregator::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) =
            self.component_log_sizes[POSEIDON_3_PARTIAL_ROUNDS_CHAIN_COMPONENT_IDX]
        {
            accumulate_relation_uses(
                relation_uses,
                poseidon_3_partial_rounds_chain::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[POSEIDON_FULL_ROUND_CHAIN_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                poseidon_full_round_chain::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[CUBE_252_COMPONENT_IDX] {
            accumulate_relation_uses(relation_uses, cube_252::RELATION_USES_PER_ROW, log_size);
        }
        if let Some(log_size) = self.component_log_sizes[RANGE_CHECK_252_WIDTH_27_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                range_check_252_width_27::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        if let Some(log_size) = self.component_log_sizes[MEMORY_ADDRESS_TO_ID_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                memory_address_to_id::RELATION_USES_PER_ROW,
                log_size,
            );
        }
        for i in 0..MEMORY_ADDRESS_TO_ID_SPLIT {
            if let Some(log_size) =
                self.component_log_sizes[MEMORY_ID_TO_BIG_BASE_COMPONENT_IDX + i]
            {
                accumulate_relation_uses(
                    relation_uses,
                    memory_id_to_big::RELATION_USES_PER_ROW,
                    log_size,
                );
            }
        }
        if let Some(log_size) = self.component_log_sizes[MEMORY_ID_TO_SMALL_COMPONENT_IDX] {
            accumulate_relation_uses(
                relation_uses,
                memory_id_to_small::RELATION_USES_PER_ROW,
                log_size,
            );
        }
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let mut trace_log_sizes = Vec::new();
        let mut interaction_log_sizes = Vec::new();
        for idx in 0..N_COMPONENTS {
            if let Some(log_size) = self.component_log_sizes[idx] {
                for _ in 0..N_TRACE_COLUMNS_PER_COMPONENT_IDX[idx] {
                    trace_log_sizes.push(log_size);
                }
                for _ in 0..N_INTERACTION_COLUMNS_PER_COMPONENT_IDX[idx] {
                    interaction_log_sizes.push(log_size);
                }
            }
        }
        TreeVec::new(vec![trace_log_sizes, interaction_log_sizes])
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoInteractionClaim {
    pub component_claimed_sums: Vec<Option<SecureField>>,
}

impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let dense_claimed_sums: Vec<SecureField> = self
            .component_claimed_sums
            .iter()
            .filter_map(|opt| *opt)
            .collect();
        channel.mix_felts(dense_claimed_sums.as_slice());
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    common_lookup_elements: &CommonLookupElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = claim.public_data.logup_sum(common_lookup_elements);
    for cs in &interaction_claim.component_claimed_sums {
        if let Some(cs) = *cs {
            sum += cs;
        }
    }
    sum
}

use std::collections::HashMap;

use itertools::chain;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::pcs::TreeVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::blake::air::BlakeContextClaim;
use crate::builtins_air::BuiltinsClaim;
use crate::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode, ret_opcode,
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_8_b,
    verify_bitwise_xor_9, verify_instruction,
};
use crate::pedersen::air::PedersenContextClaim;
use crate::poseidon::air::PoseidonContextClaim;
use crate::public_data::PublicData;
use crate::range_checks_air::RangeChecksClaim;
use crate::verifier::RelationUse;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    // Opcode claim fields (inlined)
    pub add: Vec<add_opcode::Claim>,
    pub add_small: Vec<add_opcode_small::Claim>,
    pub add_ap: Vec<add_ap_opcode::Claim>,
    pub assert_eq: Vec<assert_eq_opcode::Claim>,
    pub assert_eq_imm: Vec<assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Vec<assert_eq_opcode_double_deref::Claim>,
    pub blake: Vec<blake_compress_opcode::Claim>,
    pub call: Vec<call_opcode_abs::Claim>,
    pub call_rel_imm: Vec<call_opcode_rel_imm::Claim>,
    pub generic: Vec<generic_opcode::Claim>,
    pub jnz: Vec<jnz_opcode_non_taken::Claim>,
    pub jnz_taken: Vec<jnz_opcode_taken::Claim>,
    pub jump: Vec<jump_opcode_abs::Claim>,
    pub jump_double_deref: Vec<jump_opcode_double_deref::Claim>,
    pub jump_rel: Vec<jump_opcode_rel::Claim>,
    pub jump_rel_imm: Vec<jump_opcode_rel_imm::Claim>,
    pub mul: Vec<mul_opcode::Claim>,
    pub mul_small: Vec<mul_opcode_small::Claim>,
    pub qm31: Vec<qm_31_add_mul_opcode::Claim>,
    pub ret: Vec<ret_opcode::Claim>,
    pub verify_instruction: verify_instruction::Claim,
    pub blake_context: BlakeContextClaim,
    pub builtins: BuiltinsClaim,
    pub pedersen_context: PedersenContextClaim,
    pub poseidon_context: PoseidonContextClaim,
    pub memory_address_to_id: memory_address_to_id::Claim,
    pub memory_id_to_value: memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::Claim,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::Claim,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::Claim,
    pub verify_bitwise_xor_8_b: verify_bitwise_xor_8_b::Claim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Claim,
    // ...
}

impl CairoClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let Self {
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
            verify_instruction,
            blake_context,
            builtins,
            pedersen_context,
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
        } = self;
        public_data.mix_into(channel);
        // Mix opcode vectors: first length then each claim
        macro_rules! mix_component_vector {
            ($field:ident) => {
                channel.mix_u64($field.len() as u64);
                $field.iter().for_each(|c| c.mix_into(channel));
            };
        }
        mix_component_vector!(add);
        mix_component_vector!(add_small);
        mix_component_vector!(add_ap);
        mix_component_vector!(assert_eq);
        mix_component_vector!(assert_eq_imm);
        mix_component_vector!(assert_eq_double_deref);
        mix_component_vector!(blake);
        mix_component_vector!(call);
        mix_component_vector!(call_rel_imm);
        mix_component_vector!(generic);
        mix_component_vector!(jnz);
        mix_component_vector!(jnz_taken);
        mix_component_vector!(jump);
        mix_component_vector!(jump_double_deref);
        mix_component_vector!(jump_rel);
        mix_component_vector!(jump_rel_imm);
        mix_component_vector!(mul);
        mix_component_vector!(mul_small);
        mix_component_vector!(qm31);
        mix_component_vector!(ret);
        verify_instruction.mix_into(channel);
        blake_context.mix_into(channel);
        builtins.mix_into(channel);
        pedersen_context.mix_into(channel);
        poseidon_context.mix_into(channel);
        memory_address_to_id.mix_into(channel);
        memory_id_to_value.mix_into(channel);
        range_checks.mix_into(channel);
        verify_bitwise_xor_4.mix_into(channel);
        verify_bitwise_xor_7.mix_into(channel);
        verify_bitwise_xor_8.mix_into(channel);
        verify_bitwise_xor_8_b.mix_into(channel);
        verify_bitwise_xor_9.mix_into(channel);
    }

    /// Returns the log sizes of the components.
    /// Does not include the preprocessed trace log sizes.
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_sizes_list = vec![
            TreeVec::concat_cols(chain!(
                self.add.iter().map(|c| c.log_sizes()),
                self.add_small.iter().map(|c| c.log_sizes()),
                self.add_ap.iter().map(|c| c.log_sizes()),
                self.assert_eq.iter().map(|c| c.log_sizes()),
                self.assert_eq_imm.iter().map(|c| c.log_sizes()),
                self.assert_eq_double_deref.iter().map(|c| c.log_sizes()),
                self.blake.iter().map(|c| c.log_sizes()),
                self.call.iter().map(|c| c.log_sizes()),
                self.call_rel_imm.iter().map(|c| c.log_sizes()),
                self.generic.iter().map(|c| c.log_sizes()),
                self.jnz.iter().map(|c| c.log_sizes()),
                self.jnz_taken.iter().map(|c| c.log_sizes()),
                self.jump.iter().map(|c| c.log_sizes()),
                self.jump_double_deref.iter().map(|c| c.log_sizes()),
                self.jump_rel.iter().map(|c| c.log_sizes()),
                self.jump_rel_imm.iter().map(|c| c.log_sizes()),
                self.mul.iter().map(|c| c.log_sizes()),
                self.mul_small.iter().map(|c| c.log_sizes()),
                self.qm31.iter().map(|c| c.log_sizes()),
                self.ret.iter().map(|c| c.log_sizes()),
            )),
            self.verify_instruction.log_sizes(),
            self.blake_context.log_sizes(),
            self.builtins.log_sizes(),
            self.pedersen_context.log_sizes(),
            self.poseidon_context.log_sizes(),
            self.memory_address_to_id.log_sizes(),
            self.memory_id_to_value.log_sizes(),
            self.range_checks.log_sizes(),
            self.verify_bitwise_xor_4.log_sizes(),
            self.verify_bitwise_xor_7.log_sizes(),
            self.verify_bitwise_xor_8.log_sizes(),
            self.verify_bitwise_xor_8_b.log_sizes(),
            self.verify_bitwise_xor_9.log_sizes(),
        ];

        TreeVec::concat_cols(log_sizes_list.into_iter())
    }

    pub fn accumulate_relation_uses(&self, relation_uses: &mut RelationUsesDict) {
        let Self {
            public_data: _,
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
            verify_instruction,
            blake_context,
            builtins,
            pedersen_context,
            poseidon_context,
            memory_address_to_id: _,
            memory_id_to_value,
            range_checks: _,
            verify_bitwise_xor_4: _,
            verify_bitwise_xor_7: _,
            verify_bitwise_xor_8: _,
            verify_bitwise_xor_8_b: _,
            verify_bitwise_xor_9: _,
        } = self;
        // NOTE: The following components do not USE relations:
        // - range_checks
        // - verify_bitwise_xor_*
        // - memory_address_to_id

        macro_rules! relation_uses_vec {
            ($field:ident, $module:ident) => {
                $field.iter().for_each(|c| {
                    accumulate_relation_uses(
                        relation_uses,
                        $module::RELATION_USES_PER_ROW,
                        c.log_size,
                    )
                });
            };
        }
        relation_uses_vec!(add, add_opcode);
        relation_uses_vec!(add_small, add_opcode_small);
        relation_uses_vec!(add_ap, add_ap_opcode);
        relation_uses_vec!(assert_eq, assert_eq_opcode);
        relation_uses_vec!(assert_eq_imm, assert_eq_opcode_imm);
        relation_uses_vec!(assert_eq_double_deref, assert_eq_opcode_double_deref);
        relation_uses_vec!(blake, blake_compress_opcode);
        relation_uses_vec!(call, call_opcode_abs);
        relation_uses_vec!(call_rel_imm, call_opcode_rel_imm);
        relation_uses_vec!(generic, generic_opcode);
        relation_uses_vec!(jnz, jnz_opcode_non_taken);
        relation_uses_vec!(jnz_taken, jnz_opcode_taken);
        relation_uses_vec!(jump, jump_opcode_abs);
        relation_uses_vec!(jump_double_deref, jump_opcode_double_deref);
        relation_uses_vec!(jump_rel, jump_opcode_rel);
        relation_uses_vec!(jump_rel_imm, jump_opcode_rel_imm);
        relation_uses_vec!(mul, mul_opcode);
        relation_uses_vec!(mul_small, mul_opcode_small);
        relation_uses_vec!(qm31, qm_31_add_mul_opcode);
        relation_uses_vec!(ret, ret_opcode);
        builtins.accumulate_relation_uses(relation_uses);
        blake_context.accumulate_relation_uses(relation_uses);
        pedersen_context.accumulate_relation_uses(relation_uses);
        poseidon_context.accumulate_relation_uses(relation_uses);
        accumulate_relation_uses(
            relation_uses,
            verify_instruction::RELATION_USES_PER_ROW,
            verify_instruction.log_size,
        );

        // TODO(ShaharS): Look into the file name of memory_id_to_big.
        // memory_id_to_value has a big value component and a small value component.
        for &log_size in &memory_id_to_value.big_log_sizes {
            accumulate_relation_uses(
                relation_uses,
                memory_id_to_big::RELATION_USES_PER_ROW_BIG,
                log_size,
            );
        }
        accumulate_relation_uses(
            relation_uses,
            memory_id_to_big::RELATION_USES_PER_ROW_SMALL,
            memory_id_to_value.small_log_size,
        );
    }
}

pub type RelationUsesDict = HashMap<&'static str, u64>;

/// Accumulates the number of uses of each relation in a map.
pub fn accumulate_relation_uses<const N: usize>(
    relation_uses: &mut RelationUsesDict,
    relation_uses_per_row: [RelationUse; N],
    log_size: u32,
) {
    let component_size = 1 << log_size;
    for relation_use in relation_uses_per_row {
        let relation_uses_in_component = relation_use.uses.checked_mul(component_size).unwrap();
        let prev = relation_uses.entry(relation_use.relation_id).or_insert(0);
        *prev = prev.checked_add(relation_uses_in_component).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::accumulate_relation_uses;
    use crate::verifier::RelationUse;

    #[test]
    fn test_accumulate_relation_uses() {
        let mut relation_uses = HashMap::from([("relation_1", 4), ("relation_2", 10)]);
        let log_size = 2;
        let relation_uses_per_row = [
            RelationUse {
                relation_id: "relation_1",
                uses: 2,
            },
            RelationUse {
                relation_id: "relation_2",
                uses: 4,
            },
        ];

        accumulate_relation_uses(&mut relation_uses, relation_uses_per_row, log_size);

        assert_eq!(relation_uses.len(), 2);
        assert_eq!(relation_uses.get("relation_1"), Some(&12));
        assert_eq!(relation_uses.get("relation_2"), Some(&26));
    }
}

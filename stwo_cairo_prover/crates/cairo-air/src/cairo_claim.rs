use stwo::core::channel::Channel;
use stwo::core::pcs::TreeVec;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use serde::{Deserialize, Serialize};

use crate::blake::air::BlakeContextClaim;
use crate::builtins_air::BuiltinsClaim;
use std::collections::HashMap;
use crate::verifier::RelationUse;
use crate::public_data::PublicData;
use crate::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_8_b, verify_bitwise_xor_9, verify_instruction,
};
use crate::opcodes_air::OpcodeClaim;
use crate::pedersen::air::PedersenContextClaim;
use crate::poseidon::air::PoseidonContextClaim;
use crate::range_checks_air::RangeChecksClaim;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub opcodes: OpcodeClaim,
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
            opcodes,
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
        opcodes.mix_into(channel);
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
            self.opcodes.log_sizes(),
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
            opcodes,
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

        opcodes.accumulate_relation_uses(relation_uses);
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
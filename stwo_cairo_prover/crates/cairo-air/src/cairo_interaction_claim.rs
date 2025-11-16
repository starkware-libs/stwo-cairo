use serde::{Deserialize, Serialize};
use num_traits::Zero;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::{SecureField, QM31};
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::cairo_claim::CairoClaim;
use crate::cairo_interaction_elements::CairoInteractionElements;
use crate::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_8_b, verify_bitwise_xor_9, verify_instruction,
};
use crate::opcodes_air::OpcodeInteractionClaim;
use crate::blake::air::BlakeContextInteractionClaim;
use crate::builtins_air::BuiltinsInteractionClaim;
use crate::pedersen::air::PedersenContextInteractionClaim;
use crate::poseidon::air::PoseidonContextInteractionClaim;
use crate::range_checks_air::RangeChecksInteractionClaim;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoInteractionClaim {
    pub opcodes: OpcodeInteractionClaim,
    pub verify_instruction: verify_instruction::InteractionClaim,
    pub blake_context: BlakeContextInteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub pedersen_context: PedersenContextInteractionClaim,
    pub poseidon_context: PoseidonContextInteractionClaim,
    pub memory_address_to_id: memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_8_b: verify_bitwise_xor_8_b::InteractionClaim,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::InteractionClaim,
}
impl CairoInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.opcodes.mix_into(channel);
        self.verify_instruction.mix_into(channel);
        self.blake_context.mix_into(channel);
        self.builtins.mix_into(channel);
        self.pedersen_context.mix_into(channel);
        self.poseidon_context.mix_into(channel);
        self.memory_address_to_id.mix_into(channel);
        self.memory_id_to_value.mix_into(channel);
        self.range_checks.mix_into(channel);
        self.verify_bitwise_xor_4.mix_into(channel);
        self.verify_bitwise_xor_7.mix_into(channel);
        self.verify_bitwise_xor_8.mix_into(channel);
        self.verify_bitwise_xor_8_b.mix_into(channel);
        self.verify_bitwise_xor_9.mix_into(channel);
    }
}

pub fn lookup_sum(
    claim: &CairoClaim,
    elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> SecureField {
    let mut sum = QM31::zero();
    sum += claim.public_data.logup_sum(elements);

    // If the table is padded, take the sum of the non-padded values.
    // Otherwise, the claimed_sum is the total_sum.
    sum += interaction_claim.opcodes.sum();
    sum += interaction_claim.verify_instruction.claimed_sum;
    sum += interaction_claim.blake_context.sum();
    sum += interaction_claim.builtins.sum();
    sum += interaction_claim.pedersen_context.sum();
    sum += interaction_claim.poseidon_context.sum();
    sum += interaction_claim.memory_address_to_id.claimed_sum;
    sum += interaction_claim.memory_id_to_value.claimed_sum();
    sum += interaction_claim.range_checks.sum();
    sum += interaction_claim.verify_bitwise_xor_4.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_7.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_8.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_8_b.claimed_sum;
    sum += interaction_claim.verify_bitwise_xor_9.claimed_sum;

    sum
}

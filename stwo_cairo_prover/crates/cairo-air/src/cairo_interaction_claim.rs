use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::{SecureField, QM31};
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};

use crate::blake::air::BlakeContextInteractionClaim;
use crate::builtins_air::BuiltinsInteractionClaim;
use crate::cairo_claim::CairoClaim;
use crate::cairo_interaction_elements::CairoInteractionElements;
use crate::components::{
    memory_address_to_id, memory_id_to_big, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_8_b, verify_bitwise_xor_9, verify_instruction,
};
use crate::pedersen::air::PedersenContextInteractionClaim;
use crate::poseidon::air::PoseidonContextInteractionClaim;
use crate::range_checks_air::RangeChecksInteractionClaim;

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct CairoInteractionClaim {
    // Inline opcode interaction claim vectors
    pub add: Vec<crate::components::add_opcode::InteractionClaim>,
    pub add_small: Vec<crate::components::add_opcode_small::InteractionClaim>,
    pub add_ap: Vec<crate::components::add_ap_opcode::InteractionClaim>,
    pub assert_eq: Vec<crate::components::assert_eq_opcode::InteractionClaim>,
    pub assert_eq_imm: Vec<crate::components::assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_double_deref:
        Vec<crate::components::assert_eq_opcode_double_deref::InteractionClaim>,
    pub blake: Vec<crate::components::blake_compress_opcode::InteractionClaim>,
    pub call: Vec<crate::components::call_opcode_abs::InteractionClaim>,
    pub call_rel_imm: Vec<crate::components::call_opcode_rel_imm::InteractionClaim>,
    pub generic: Vec<crate::components::generic_opcode::InteractionClaim>,
    pub jnz: Vec<crate::components::jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_taken: Vec<crate::components::jnz_opcode_taken::InteractionClaim>,
    pub jump: Vec<crate::components::jump_opcode_abs::InteractionClaim>,
    pub jump_double_deref: Vec<crate::components::jump_opcode_double_deref::InteractionClaim>,
    pub jump_rel: Vec<crate::components::jump_opcode_rel::InteractionClaim>,
    pub jump_rel_imm: Vec<crate::components::jump_opcode_rel_imm::InteractionClaim>,
    pub mul: Vec<crate::components::mul_opcode::InteractionClaim>,
    pub mul_small: Vec<crate::components::mul_opcode_small::InteractionClaim>,
    pub qm31: Vec<crate::components::qm_31_add_mul_opcode::InteractionClaim>,
    pub ret: Vec<crate::components::ret_opcode::InteractionClaim>,
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
        // Mix all opcode interaction claims
        macro_rules! mix_vec {
            ($field:ident) => {
                self.$field.iter().for_each(|c| c.mix_into(channel));
            };
        }
        mix_vec!(add);
        mix_vec!(add_small);
        mix_vec!(add_ap);
        mix_vec!(assert_eq);
        mix_vec!(assert_eq_imm);
        mix_vec!(assert_eq_double_deref);
        mix_vec!(blake);
        mix_vec!(call);
        mix_vec!(call_rel_imm);
        mix_vec!(generic);
        mix_vec!(jnz);
        mix_vec!(jnz_taken);
        mix_vec!(jump);
        mix_vec!(jump_double_deref);
        mix_vec!(jump_rel);
        mix_vec!(jump_rel_imm);
        mix_vec!(mul);
        mix_vec!(mul_small);
        mix_vec!(qm31);
        mix_vec!(ret);
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
    macro_rules! add_vec_sum {
        ($field:ident) => {
            for ic in &interaction_claim.$field {
                sum += ic.claimed_sum;
            }
        };
    }
    add_vec_sum!(add);
    add_vec_sum!(add_small);
    add_vec_sum!(add_ap);
    add_vec_sum!(assert_eq);
    add_vec_sum!(assert_eq_imm);
    add_vec_sum!(assert_eq_double_deref);
    add_vec_sum!(blake);
    add_vec_sum!(call);
    add_vec_sum!(call_rel_imm);
    add_vec_sum!(generic);
    add_vec_sum!(jnz);
    add_vec_sum!(jnz_taken);
    add_vec_sum!(jump);
    add_vec_sum!(jump_double_deref);
    add_vec_sum!(jump_rel);
    add_vec_sum!(jump_rel_imm);
    add_vec_sum!(mul);
    add_vec_sum!(mul_small);
    add_vec_sum!(qm31);
    add_vec_sum!(ret);
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

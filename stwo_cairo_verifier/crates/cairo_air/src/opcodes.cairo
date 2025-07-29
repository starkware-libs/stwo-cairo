use components::add_ap_opcode::{
    ClaimImpl as AddApOpcodeClaimImpl, InteractionClaimImpl as AddApOpcodeInteractionClaimImpl,
};
use components::add_opcode::{
    ClaimImpl as AddOpcodeClaimImpl, InteractionClaimImpl as AddOpcodeInteractionClaimImpl,
};
use components::add_opcode_small::{
    ClaimImpl as AddOpcodeSmallClaimImpl,
    InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl,
};
use components::assert_eq_opcode::{
    ClaimImpl as AssertEqOpcodeClaimImpl,
    InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl,
};
use components::assert_eq_opcode_double_deref::{
    ClaimImpl as AssertEqOpcodeDoubleDerefClaimImpl,
    InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl,
};
use components::assert_eq_opcode_imm::{
    ClaimImpl as AssertEqOpcodeImmClaimImpl,
    InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl,
};
use components::blake_compress_opcode::{
    ClaimImpl as BlakeCompressOpcodeClaimImpl,
    InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl,
};
use components::call_opcode::{
    ClaimImpl as CallOpcodeClaimImpl, InteractionClaimImpl as CallOpcodeInteractionClaimImpl,
};
use components::call_opcode_rel_imm::{
    ClaimImpl as CallOpcodeRelClaimImpl, InteractionClaimImpl as CallOpcodeRelInteractionClaimImpl,
};
use components::generic_opcode::{
    ClaimImpl as GenericOpcodeClaimImpl, InteractionClaimImpl as GenericOpcodeInteractionClaimImpl,
};
use components::jnz_opcode::{
    ClaimImpl as JnzOpcodeClaimImpl, InteractionClaimImpl as JnzOpcodeInteractionClaimImpl,
};
use components::jnz_opcode_taken::{
    ClaimImpl as JnzOpcodeTakenClaimImpl,
    InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl,
};
use components::jump_opcode::{
    ClaimImpl as JumpOpcodeClaimImpl, InteractionClaimImpl as JumpOpcodeInteractionClaimImpl,
};
use components::jump_opcode_double_deref::{
    ClaimImpl as JumpOpcodeDoubleDerefClaimImpl,
    InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl,
};
use components::jump_opcode_rel::{
    ClaimImpl as JumpOpcodeRelClaimImpl, InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl,
};
use components::jump_opcode_rel_imm::{
    ClaimImpl as JumpOpcodeRelImmClaimImpl,
    InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl,
};
use components::mul_opcode::{
    ClaimImpl as MulOpcodeClaimImpl, InteractionClaimImpl as MulOpcodeInteractionClaimImpl,
};
use components::mul_opcode_small::{
    ClaimImpl as MulOpcodeSmallClaimImpl,
    InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl,
};
use components::qm_31_add_mul_opcode::{
    ClaimImpl as Qm31AddMulOpcodeClaimImpl,
    InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl,
};
use components::ret_opcode::{
    ClaimImpl as RetOpcodeClaimImpl, InteractionClaimImpl as RetOpcodeInteractionClaimImpl,
};
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::{
    CairoInteractionElements, RelationUsesDict, accumulate_relation_uses, components, utils,
};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumnImpl, PreprocessedColumnKey, PreprocessedColumnSet,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};


#[derive(Drop, Serde)]
pub struct OpcodeInteractionClaim {
    add: Array<components::add_opcode::InteractionClaim>,
    add_small: Array<components::add_opcode_small::InteractionClaim>,
    add_ap: Array<components::add_ap_opcode::InteractionClaim>,
    assert_eq: Array<components::assert_eq_opcode::InteractionClaim>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::InteractionClaim>,
    blake: Array<components::blake_compress_opcode::InteractionClaim>,
    call: Array<components::call_opcode::InteractionClaim>,
    call_rel_imm: Array<components::call_opcode_rel_imm::InteractionClaim>,
    generic: Array<components::generic_opcode::InteractionClaim>,
    jnz: Array<components::jnz_opcode::InteractionClaim>,
    jnz_taken: Array<components::jnz_opcode_taken::InteractionClaim>,
    jump: Array<components::jump_opcode::InteractionClaim>,
    jump_double_deref: Array<components::jump_opcode_double_deref::InteractionClaim>,
    jump_rel: Array<components::jump_opcode_rel::InteractionClaim>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::InteractionClaim>,
    mul: Array<components::mul_opcode::InteractionClaim>,
    mul_small: Array<components::mul_opcode_small::InteractionClaim>,
    qm31: Array<components::qm_31_add_mul_opcode::InteractionClaim>,
    ret: Array<components::ret_opcode::InteractionClaim>,
}

#[generate_trait]
pub impl OpcodeInteractionClaimImpl of OpcodeInteractionClaimTrait {
    fn mix_into(self: @OpcodeInteractionClaim, ref channel: Channel) {
        for interaction_claim in self.add.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_small.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.add_ap.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.assert_eq.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.assert_eq_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.assert_eq_double_deref.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.blake.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.call.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.call_rel_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.generic.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jnz_taken.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump_double_deref.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump_rel.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.jump_rel_imm.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.mul.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.mul_small.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.qm31.span() {
            interaction_claim.mix_into(ref channel);
        }

        for interaction_claim in self.ret.span() {
            interaction_claim.mix_into(ref channel);
        }
    }

    fn sum(self: @OpcodeInteractionClaim) -> QM31 {
        let mut sum = Zero::zero();

        for interaction_claim in self.add.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_small.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.add_ap.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.assert_eq.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.assert_eq_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.assert_eq_double_deref.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.blake.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.call.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.call_rel_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.generic.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jnz_taken.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump_double_deref.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump_rel.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.jump_rel_imm.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.mul.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.mul_small.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.qm31.span() {
            sum += *interaction_claim.claimed_sum;
        }

        for interaction_claim in self.ret.span() {
            sum += *interaction_claim.claimed_sum;
        }

        sum
    }
}

#[derive(Drop, Serde)]
pub struct OpcodeClaim {
    pub add: Array<components::add_opcode::Claim>,
    pub add_small: Array<components::add_opcode_small::Claim>,
    pub add_ap: Array<components::add_ap_opcode::Claim>,
    pub assert_eq: Array<components::assert_eq_opcode::Claim>,
    pub assert_eq_imm: Array<components::assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Claim>,
    pub blake: Array<components::blake_compress_opcode::Claim>,
    pub call: Array<components::call_opcode::Claim>,
    pub call_rel_imm: Array<components::call_opcode_rel_imm::Claim>,
    pub generic: Array<components::generic_opcode::Claim>,
    pub jnz: Array<components::jnz_opcode::Claim>,
    pub jnz_taken: Array<components::jnz_opcode_taken::Claim>,
    pub jump: Array<components::jump_opcode::Claim>,
    pub jump_double_deref: Array<components::jump_opcode_double_deref::Claim>,
    pub jump_rel: Array<components::jump_opcode_rel::Claim>,
    pub jump_rel_imm: Array<components::jump_opcode_rel_imm::Claim>,
    pub mul: Array<components::mul_opcode::Claim>,
    pub mul_small: Array<components::mul_opcode_small::Claim>,
    pub qm31: Array<components::qm_31_add_mul_opcode::Claim>,
    pub ret: Array<components::ret_opcode::Claim>,
}

#[generate_trait]
pub impl OpcodeClaimImpl of OpcodeClaimTrait {
    fn mix_into(self: @OpcodeClaim, ref channel: Channel) {
        channel.mix_u64(self.add.len().into());
        for claim in self.add.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.add_small.len().into());
        for claim in self.add_small.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.add_ap.len().into());
        for claim in self.add_ap.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.assert_eq.len().into());
        for claim in self.assert_eq.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.assert_eq_imm.len().into());
        for claim in self.assert_eq_imm.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.assert_eq_double_deref.len().into());
        for claim in self.assert_eq_double_deref.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.blake.len().into());
        for claim in self.blake.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.call.len().into());
        for claim in self.call.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.call_rel_imm.len().into());
        for claim in self.call_rel_imm.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.generic.len().into());
        for claim in self.generic.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jnz.len().into());
        for claim in self.jnz.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jnz_taken.len().into());
        for claim in self.jnz_taken.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump.len().into());
        for claim in self.jump.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump_double_deref.len().into());
        for claim in self.jump_double_deref.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump_rel.len().into());
        for claim in self.jump_rel.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.jump_rel_imm.len().into());
        for claim in self.jump_rel_imm.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.mul.len().into());
        for claim in self.mul.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.mul_small.len().into());
        for claim in self.mul_small.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.qm31.len().into());
        for claim in self.qm31.span() {
            claim.mix_into(ref channel);
        }

        channel.mix_u64(self.ret.len().into());
        for claim in self.ret.span() {
            claim.mix_into(ref channel);
        }
    }

    fn log_sizes(self: @OpcodeClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes = array![];

        for claim in self.add.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_small.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.add_ap.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.assert_eq.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.assert_eq_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.assert_eq_double_deref.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.blake.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.call.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.call_rel_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.generic.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jnz_taken.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump_double_deref.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump_rel.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.jump_rel_imm.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.mul.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.mul_small.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.qm31.span() {
            log_sizes.append(claim.log_sizes());
        }

        for claim in self.ret.span() {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }

    fn accumulate_relation_uses(self: @OpcodeClaim, ref relation_uses: RelationUsesDict) {
        let OpcodeClaim {
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
        } = self;
        for claim in add.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in add_small.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_opcode_small::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in add_ap.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::add_ap_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in assert_eq.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::assert_eq_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in assert_eq_imm.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::assert_eq_opcode_imm::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in assert_eq_double_deref.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::assert_eq_opcode_double_deref::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in blake.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::blake_compress_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in call.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::call_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in call_rel_imm.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::call_opcode_rel_imm::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in generic.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::generic_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jnz.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jnz_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jnz_taken.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jnz_opcode_taken::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump_double_deref.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode_double_deref::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump_rel.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode_rel::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in jump_rel_imm.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::jump_opcode_rel_imm::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in mul.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::mul_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in mul_small.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::mul_opcode_small::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in qm31.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::qm_31_add_mul_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }

        for claim in ret.span() {
            accumulate_relation_uses(
                ref relation_uses,
                components::ret_opcode::RELATION_USES_PER_ROW.span(),
                *claim.log_size,
            );
        }
    }
}


#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    blake: Array<components::blake_compress_opcode::Component>,
    call: Array<components::call_opcode::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    generic: Array<components::generic_opcode::Component>,
    jnz: Array<components::jnz_opcode::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    qm31: Array<components::qm_31_add_mul_opcode::Component>,
    ret: Array<components::ret_opcode::Component>,
}

#[cfg(feature: "poseidon252_verifier")]
#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    blake: Array<components::blake_compress_opcode::Component>,
    call: Array<components::call_opcode::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    jnz: Array<components::jnz_opcode::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    qm31: Array<components::qm_31_add_mul_opcode::Component>,
    ret: Array<components::ret_opcode::Component>,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
pub impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        claim: @OpcodeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @OpcodeInteractionClaim,
    ) -> OpcodeComponents {
        // Add components
        let mut add_components = array![];
        let mut add_claims = claim.add.span();
        let mut add_interaction_claims = interaction_claim.add.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_claims.pop_front(), add_interaction_claims.pop_front()) {
            add_components
                .append(
                    components::add_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_claims.is_empty());
        assert!(add_interaction_claims.is_empty());

        // Add Small components
        let mut add_small_components = array![];
        let mut add_small_claims = claim.add_small.span();
        let mut add_small_interaction_claims = interaction_claim.add_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_small_claims.pop_front(), add_small_interaction_claims.pop_front()) {
            add_small_components
                .append(
                    components::add_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_small_claims.is_empty());
        assert!(add_small_interaction_claims.is_empty());

        // Add AP components
        let mut add_ap_components = array![];
        let mut add_ap_claims = claim.add_ap.span();
        let mut add_ap_interaction_claims = interaction_claim.add_ap.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_ap_claims.pop_front(), add_ap_interaction_claims.pop_front()) {
            add_ap_components
                .append(
                    components::add_ap_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_8_lookup_elements: interaction_elements
                            .range_checks
                            .rc_8
                            .clone(),
                    },
                );
        }
        assert!(add_ap_claims.is_empty());
        assert!(add_ap_interaction_claims.is_empty());

        // Assert Eq components
        let mut assert_eq_components = array![];
        let mut assert_eq_claims = claim.assert_eq.span();
        let mut assert_eq_interaction_claims = interaction_claim.assert_eq.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_claims.pop_front(), assert_eq_interaction_claims.pop_front()) {
            assert_eq_components
                .append(
                    components::assert_eq_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_claims.is_empty());
        assert!(assert_eq_interaction_claims.is_empty());

        // Assert Eq Imm components
        let mut assert_eq_imm_components = array![];
        let mut assert_eq_imm_claims = claim.assert_eq_imm.span();
        let mut assert_eq_imm_interaction_claims = interaction_claim.assert_eq_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_imm_claims.pop_front(), assert_eq_imm_interaction_claims.pop_front()) {
            assert_eq_imm_components
                .append(
                    components::assert_eq_opcode_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_imm_claims.is_empty());
        assert!(assert_eq_imm_interaction_claims.is_empty());

        // Assert Eq Double Deref components
        let mut assert_eq_double_deref_components = array![];
        let mut assert_eq_double_deref_claims = claim.assert_eq_double_deref.span();
        let mut assert_eq_double_deref_interaction_claims = interaction_claim
            .assert_eq_double_deref
            .span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                assert_eq_double_deref_claims.pop_front(),
                assert_eq_double_deref_interaction_claims.pop_front(),
            ) {
            assert_eq_double_deref_components
                .append(
                    components::assert_eq_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_double_deref_claims.is_empty());
        assert!(assert_eq_double_deref_interaction_claims.is_empty());

        let mut blake_components = array![];
        let mut blake_claims = claim.blake.span();
        let mut blake_interaction_claims = interaction_claim.blake.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (blake_claims.pop_front(), blake_interaction_claims.pop_front()) {
            blake_components
                .append(
                    components::blake_compress_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_7_2_5_lookup_elements: interaction_elements
                            .range_checks
                            .rc_7_2_5
                            .clone(),
                        triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                        verify_bitwise_xor_8_lookup_elements: interaction_elements
                            .verify_bitwise_xor_8
                            .clone(),
                        blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                    },
                );
        }
        assert!(blake_claims.is_empty());
        assert!(blake_interaction_claims.is_empty());

        // Call components
        let mut call_components = array![];
        let mut call_claims = claim.call.span();
        let mut call_interaction_claims = interaction_claim.call.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_claims.pop_front(), call_interaction_claims.pop_front()) {
            call_components
                .append(
                    components::call_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_claims.is_empty());
        assert!(call_interaction_claims.is_empty());

        // Call Rel_imm components
        let mut call_rel_imm_components = array![];
        let mut call_rel_imm_claims = claim.call_rel_imm.span();
        let mut call_rel_imm_interaction_claims = interaction_claim.call_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_rel_imm_claims.pop_front(), call_rel_imm_interaction_claims.pop_front()) {
            call_rel_imm_components
                .append(
                    components::call_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_rel_imm_claims.is_empty());
        assert!(call_rel_imm_interaction_claims.is_empty());

        // Generic components
        let mut generic_components = array![];
        let mut generic_claims = claim.generic.span();
        let mut generic_interaction_claims = interaction_claim.generic.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (generic_claims.pop_front(), generic_interaction_claims.pop_front()) {
            generic_components
                .append(
                    components::generic_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_19_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_b
                            .clone(),
                        range_check_19_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_c
                            .clone(),
                        range_check_19_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_d
                            .clone(),
                        range_check_19_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_e
                            .clone(),
                        range_check_19_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_f
                            .clone(),
                        range_check_19_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_g
                            .clone(),
                        range_check_19_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_h
                            .clone(),
                        range_check_9_9_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9
                            .clone(),
                        range_check_9_9_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_b
                            .clone(),
                        range_check_9_9_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_c
                            .clone(),
                        range_check_9_9_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_d
                            .clone(),
                        range_check_9_9_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_e
                            .clone(),
                        range_check_9_9_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_f
                            .clone(),
                        range_check_9_9_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_g
                            .clone(),
                        range_check_9_9_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_9_9_h
                            .clone(),
                        range_check_8_lookup_elements: interaction_elements
                            .range_checks
                            .rc_8
                            .clone(),
                    },
                );
        }
        assert!(generic_claims.is_empty());
        assert!(generic_interaction_claims.is_empty());

        // Jnz components
        let mut jnz_components = array![];
        let mut jnz_claims = claim.jnz.span();
        let mut jnz_interaction_claims = interaction_claim.jnz.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_claims.pop_front(), jnz_interaction_claims.pop_front()) {
            jnz_components
                .append(
                    components::jnz_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_claims.is_empty());
        assert!(jnz_interaction_claims.is_empty());

        // Jnz Taken components
        let mut jnz_taken_components = array![];
        let mut jnz_taken_claims = claim.jnz_taken.span();
        let mut jnz_taken_interaction_claims = interaction_claim.jnz_taken.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_taken_claims.pop_front(), jnz_taken_interaction_claims.pop_front()) {
            jnz_taken_components
                .append(
                    components::jnz_opcode_taken::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_taken_claims.is_empty());
        assert!(jnz_taken_interaction_claims.is_empty());

        // Jump components
        let mut jump_components = array![];
        let mut jump_claims = claim.jump.span();
        let mut jump_interaction_claims = interaction_claim.jump.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_claims.pop_front(), jump_interaction_claims.pop_front()) {
            jump_components
                .append(
                    components::jump_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_claims.is_empty());
        assert!(jump_interaction_claims.is_empty());

        // Jump Double Deref components
        let mut jump_double_deref_components = array![];
        let mut jump_double_deref_claims = claim.jump_double_deref.span();
        let mut jump_double_deref_interaction_claims = interaction_claim.jump_double_deref.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                jump_double_deref_claims.pop_front(),
                jump_double_deref_interaction_claims.pop_front(),
            ) {
            jump_double_deref_components
                .append(
                    components::jump_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_double_deref_claims.is_empty());
        assert!(jump_double_deref_interaction_claims.is_empty());

        // Jump Rel components
        let mut jump_rel_components = array![];
        let mut jump_rel_claims = claim.jump_rel.span();
        let mut jump_rel_interaction_claims = interaction_claim.jump_rel.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_claims.pop_front(), jump_rel_interaction_claims.pop_front()) {
            jump_rel_components
                .append(
                    components::jump_opcode_rel::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_claims.is_empty());
        assert!(jump_rel_interaction_claims.is_empty());

        // Jump Rel Imm components
        let mut jump_rel_imm_components = array![];
        let mut jump_rel_imm_claims = claim.jump_rel_imm.span();
        let mut jump_rel_imm_interaction_claims = interaction_claim.jump_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_imm_claims.pop_front(), jump_rel_imm_interaction_claims.pop_front()) {
            jump_rel_imm_components
                .append(
                    components::jump_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_imm_claims.is_empty());
        assert!(jump_rel_imm_interaction_claims.is_empty());

        // Mul components
        let mut mul_components = array![];
        let mut mul_claims = claim.mul.span();
        let mut mul_interaction_claims = interaction_claim.mul.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_claims.pop_front(), mul_interaction_claims.pop_front()) {
            mul_components
                .append(
                    components::mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_19_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_b
                            .clone(),
                        range_check_19_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_c
                            .clone(),
                        range_check_19_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_d
                            .clone(),
                        range_check_19_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_e
                            .clone(),
                        range_check_19_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_f
                            .clone(),
                        range_check_19_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_g
                            .clone(),
                        range_check_19_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_h
                            .clone(),
                    },
                );
        }
        assert!(mul_claims.is_empty());
        assert!(mul_interaction_claims.is_empty());

        // Mul Small components
        let mut mul_small_components = array![];
        let mut mul_small_claims = claim.mul_small.span();
        let mut mul_small_interaction_claims = interaction_claim.mul_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_small_claims.pop_front(), mul_small_interaction_claims.pop_front()) {
            mul_small_components
                .append(
                    components::mul_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                    },
                );
        }
        assert!(mul_small_claims.is_empty());
        assert!(mul_small_interaction_claims.is_empty());

        // QM31 components
        let mut qm31_components = array![];
        let mut qm31_claims = claim.qm31.span();
        let mut qm31_interaction_claims = interaction_claim.qm31.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (qm31_claims.pop_front(), qm31_interaction_claims.pop_front()) {
            qm31_components
                .append(
                    components::qm_31_add_mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                    },
                );
        }
        assert!(qm31_claims.is_empty());
        assert!(qm31_interaction_claims.is_empty());

        // Ret components
        let mut ret_components = array![];
        let mut ret_claims = claim.ret.span();
        let mut ret_interaction_claims = interaction_claim.ret.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (ret_claims.pop_front(), ret_interaction_claims.pop_front()) {
            ret_components
                .append(
                    components::ret_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(ret_claims.is_empty());
        assert!(ret_interaction_claims.is_empty());

        OpcodeComponents {
            add: add_components,
            add_small: add_small_components,
            add_ap: add_ap_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_rel_imm: call_rel_imm_components,
            generic: generic_components,
            jnz: jnz_components,
            jnz_taken: jnz_taken_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_small: mul_small_components,
            qm31: qm31_components,
            ret: ret_components,
        }
    }

    fn mask_points(
        self: @OpcodeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_ap.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.generic.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };
    }

    fn max_constraint_log_degree_bound(self: @OpcodeComponents) -> u32 {
        let mut max_degree = 0;

        for component in self.add.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.blake.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.generic.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_taken.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.qm31.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.ret.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_ap.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.generic.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.mul_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };
    }
}

#[cfg(feature: "poseidon252_verifier")]
#[generate_trait]
pub impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        claim: @OpcodeClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @OpcodeInteractionClaim,
    ) -> OpcodeComponents {
        // Add components
        let mut add_components = array![];
        let mut add_claims = claim.add.span();
        let mut add_interaction_claims = interaction_claim.add.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_claims.pop_front(), add_interaction_claims.pop_front()) {
            add_components
                .append(
                    components::add_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_claims.is_empty());
        assert!(add_interaction_claims.is_empty());

        // Add Small components
        let mut add_small_components = array![];
        let mut add_small_claims = claim.add_small.span();
        let mut add_small_interaction_claims = interaction_claim.add_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_small_claims.pop_front(), add_small_interaction_claims.pop_front()) {
            add_small_components
                .append(
                    components::add_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(add_small_claims.is_empty());
        assert!(add_small_interaction_claims.is_empty());

        // Add AP components
        let mut add_ap_components = array![];
        let mut add_ap_claims = claim.add_ap.span();
        let mut add_ap_interaction_claims = interaction_claim.add_ap.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (add_ap_claims.pop_front(), add_ap_interaction_claims.pop_front()) {
            add_ap_components
                .append(
                    components::add_ap_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_8_lookup_elements: interaction_elements
                            .range_checks
                            .rc_8
                            .clone(),
                    },
                );
        }
        assert!(add_ap_claims.is_empty());
        assert!(add_ap_interaction_claims.is_empty());

        // Assert Eq components
        let mut assert_eq_components = array![];
        let mut assert_eq_claims = claim.assert_eq.span();
        let mut assert_eq_interaction_claims = interaction_claim.assert_eq.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_claims.pop_front(), assert_eq_interaction_claims.pop_front()) {
            assert_eq_components
                .append(
                    components::assert_eq_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_claims.is_empty());
        assert!(assert_eq_interaction_claims.is_empty());

        // Assert Eq Imm components
        let mut assert_eq_imm_components = array![];
        let mut assert_eq_imm_claims = claim.assert_eq_imm.span();
        let mut assert_eq_imm_interaction_claims = interaction_claim.assert_eq_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (assert_eq_imm_claims.pop_front(), assert_eq_imm_interaction_claims.pop_front()) {
            assert_eq_imm_components
                .append(
                    components::assert_eq_opcode_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_imm_claims.is_empty());
        assert!(assert_eq_imm_interaction_claims.is_empty());

        // Assert Eq Double Deref components
        let mut assert_eq_double_deref_components = array![];
        let mut assert_eq_double_deref_claims = claim.assert_eq_double_deref.span();
        let mut assert_eq_double_deref_interaction_claims = interaction_claim
            .assert_eq_double_deref
            .span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                assert_eq_double_deref_claims.pop_front(),
                assert_eq_double_deref_interaction_claims.pop_front(),
            ) {
            assert_eq_double_deref_components
                .append(
                    components::assert_eq_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(assert_eq_double_deref_claims.is_empty());
        assert!(assert_eq_double_deref_interaction_claims.is_empty());

        let mut blake_components = array![];
        let mut blake_claims = claim.blake.span();
        let mut blake_interaction_claims = interaction_claim.blake.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (blake_claims.pop_front(), blake_interaction_claims.pop_front()) {
            blake_components
                .append(
                    components::blake_compress_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_7_2_5_lookup_elements: interaction_elements
                            .range_checks
                            .rc_7_2_5
                            .clone(),
                        triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                        verify_bitwise_xor_8_lookup_elements: interaction_elements
                            .verify_bitwise_xor_8
                            .clone(),
                        blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                    },
                );
        }
        assert!(blake_claims.is_empty());
        assert!(blake_interaction_claims.is_empty());

        // Call components
        let mut call_components = array![];
        let mut call_claims = claim.call.span();
        let mut call_interaction_claims = interaction_claim.call.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_claims.pop_front(), call_interaction_claims.pop_front()) {
            call_components
                .append(
                    components::call_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_claims.is_empty());
        assert!(call_interaction_claims.is_empty());

        // Call Rel_imm components
        let mut call_rel_imm_components = array![];
        let mut call_rel_imm_claims = claim.call_rel_imm.span();
        let mut call_rel_imm_interaction_claims = interaction_claim.call_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (call_rel_imm_claims.pop_front(), call_rel_imm_interaction_claims.pop_front()) {
            call_rel_imm_components
                .append(
                    components::call_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(call_rel_imm_claims.is_empty());
        assert!(call_rel_imm_interaction_claims.is_empty());

        // Jnz components
        let mut jnz_components = array![];
        let mut jnz_claims = claim.jnz.span();
        let mut jnz_interaction_claims = interaction_claim.jnz.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_claims.pop_front(), jnz_interaction_claims.pop_front()) {
            jnz_components
                .append(
                    components::jnz_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_claims.is_empty());
        assert!(jnz_interaction_claims.is_empty());

        // Jnz Taken components
        let mut jnz_taken_components = array![];
        let mut jnz_taken_claims = claim.jnz_taken.span();
        let mut jnz_taken_interaction_claims = interaction_claim.jnz_taken.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jnz_taken_claims.pop_front(), jnz_taken_interaction_claims.pop_front()) {
            jnz_taken_components
                .append(
                    components::jnz_opcode_taken::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jnz_taken_claims.is_empty());
        assert!(jnz_taken_interaction_claims.is_empty());

        // Jump components
        let mut jump_components = array![];
        let mut jump_claims = claim.jump.span();
        let mut jump_interaction_claims = interaction_claim.jump.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_claims.pop_front(), jump_interaction_claims.pop_front()) {
            jump_components
                .append(
                    components::jump_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_claims.is_empty());
        assert!(jump_interaction_claims.is_empty());

        // Jump Double Deref components
        let mut jump_double_deref_components = array![];
        let mut jump_double_deref_claims = claim.jump_double_deref.span();
        let mut jump_double_deref_interaction_claims = interaction_claim.jump_double_deref.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (
                jump_double_deref_claims.pop_front(),
                jump_double_deref_interaction_claims.pop_front(),
            ) {
            jump_double_deref_components
                .append(
                    components::jump_opcode_double_deref::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_double_deref_claims.is_empty());
        assert!(jump_double_deref_interaction_claims.is_empty());

        // Jump Rel components
        let mut jump_rel_components = array![];
        let mut jump_rel_claims = claim.jump_rel.span();
        let mut jump_rel_interaction_claims = interaction_claim.jump_rel.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_claims.pop_front(), jump_rel_interaction_claims.pop_front()) {
            jump_rel_components
                .append(
                    components::jump_opcode_rel::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_claims.is_empty());
        assert!(jump_rel_interaction_claims.is_empty());

        // Jump Rel Imm components
        let mut jump_rel_imm_components = array![];
        let mut jump_rel_imm_claims = claim.jump_rel_imm.span();
        let mut jump_rel_imm_interaction_claims = interaction_claim.jump_rel_imm.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (jump_rel_imm_claims.pop_front(), jump_rel_imm_interaction_claims.pop_front()) {
            jump_rel_imm_components
                .append(
                    components::jump_opcode_rel_imm::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(jump_rel_imm_claims.is_empty());
        assert!(jump_rel_imm_interaction_claims.is_empty());

        // Mul components
        let mut mul_components = array![];
        let mut mul_claims = claim.mul.span();
        let mut mul_interaction_claims = interaction_claim.mul.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_claims.pop_front(), mul_interaction_claims.pop_front()) {
            mul_components
                .append(
                    components::mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_19_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19
                            .clone(),
                        range_check_19_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_b
                            .clone(),
                        range_check_19_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_c
                            .clone(),
                        range_check_19_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_d
                            .clone(),
                        range_check_19_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_e
                            .clone(),
                        range_check_19_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_f
                            .clone(),
                        range_check_19_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_g
                            .clone(),
                        range_check_19_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_19_h
                            .clone(),
                    },
                );
        }
        assert!(mul_claims.is_empty());
        assert!(mul_interaction_claims.is_empty());

        // Mul Small components
        let mut mul_small_components = array![];
        let mut mul_small_claims = claim.mul_small.span();
        let mut mul_small_interaction_claims = interaction_claim.mul_small.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (mul_small_claims.pop_front(), mul_small_interaction_claims.pop_front()) {
            mul_small_components
                .append(
                    components::mul_opcode_small::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                    },
                );
        }
        assert!(mul_small_claims.is_empty());
        assert!(mul_small_interaction_claims.is_empty());

        // QM31 components
        let mut qm31_components = array![];
        let mut qm31_claims = claim.qm31.span();
        let mut qm31_interaction_claims = interaction_claim.qm31.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (qm31_claims.pop_front(), qm31_interaction_claims.pop_front()) {
            qm31_components
                .append(
                    components::qm_31_add_mul_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        range_check_4_4_4_4_lookup_elements: interaction_elements
                            .range_checks
                            .rc_4_4_4_4
                            .clone(),
                    },
                );
        }
        assert!(qm31_claims.is_empty());
        assert!(qm31_interaction_claims.is_empty());

        // Ret components
        let mut ret_components = array![];
        let mut ret_claims = claim.ret.span();
        let mut ret_interaction_claims = interaction_claim.ret.span();
        while let (Option::Some(claim), Option::Some(interaction_claim)) =
            (ret_claims.pop_front(), ret_interaction_claims.pop_front()) {
            ret_components
                .append(
                    components::ret_opcode::Component {
                        claim: *claim,
                        interaction_claim: *interaction_claim,
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                    },
                );
        }
        assert!(ret_claims.is_empty());
        assert!(ret_interaction_claims.is_empty());

        OpcodeComponents {
            add: add_components,
            add_small: add_small_components,
            add_ap: add_ap_components,
            assert_eq: assert_eq_components,
            assert_eq_imm: assert_eq_imm_components,
            assert_eq_double_deref: assert_eq_double_deref_components,
            blake: blake_components,
            call: call_components,
            call_rel_imm: call_rel_imm_components,
            jnz: jnz_components,
            jnz_taken: jnz_taken_components,
            jump: jump_components,
            jump_double_deref: jump_double_deref_components,
            jump_rel: jump_rel_components,
            jump_rel_imm: jump_rel_imm_components,
            mul: mul_components,
            mul_small: mul_small_components,
            qm31: qm31_components,
            ret: ret_components,
        }
    }

    fn mask_points(
        self: @OpcodeComponents,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: Array<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.add_ap.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.assert_eq_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.mul_small.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .mask_points(
                    ref preprocessed_column_set,
                    ref trace_mask_points,
                    ref interaction_trace_mask_points,
                    point,
                );
        };
    }

    fn max_constraint_log_degree_bound(self: @OpcodeComponents) -> u32 {
        let mut max_degree = 0;

        for component in self.add.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.add_ap.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.assert_eq_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.blake.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.call_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jnz_taken.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_double_deref.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.jump_rel_imm.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.mul_small.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.qm31.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        for component in self.ret.span() {
            max_degree = core::cmp::max(max_degree, component.max_constraint_log_degree_bound());
        }

        max_degree
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        for component in self.add.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.add_ap.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.assert_eq_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.blake.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.mul.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        for component in self.mul_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.qm31.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        };
    }
}

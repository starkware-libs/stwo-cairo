use components::add_ap_opcode::InteractionClaimImpl as AddApOpcodeInteractionClaimImpl;
use components::add_opcode::InteractionClaimImpl as AddOpcodeInteractionClaimImpl;
use components::add_opcode_small::InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl;
use components::assert_eq_opcode::InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl;
use components::assert_eq_opcode_double_deref::InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl;
use components::assert_eq_opcode_imm::InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl;
use components::blake_compress_opcode::InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl;
use components::call_opcode_abs::InteractionClaimImpl as CallOpcodeInteractionClaimImpl;
use components::call_opcode_rel_imm::InteractionClaimImpl as CallOpcodeRelInteractionClaimImpl;
use components::generic_opcode::InteractionClaimImpl as GenericOpcodeInteractionClaimImpl;
use components::jnz_opcode_non_taken::InteractionClaimImpl as JnzOpcodeInteractionClaimImpl;
use components::jnz_opcode_taken::InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl;
use components::jump_opcode_abs::InteractionClaimImpl as JumpOpcodeInteractionClaimImpl;
use components::jump_opcode_double_deref::InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl;
use components::jump_opcode_rel::InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl;
use components::jump_opcode_rel_imm::InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl;
use components::mul_opcode::InteractionClaimImpl as MulOpcodeInteractionClaimImpl;
use components::mul_opcode_small::InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl;
use components::qm_31_add_mul_opcode::InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl;
use components::ret_opcode::InteractionClaimImpl as RetOpcodeInteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{CairoInteractionElements, RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};
use stwo_verifier_utils::zip_eq::zip_eq;


#[derive(Drop, Serde)]
pub struct OpcodeInteractionClaim {
    add: Array<components::add_opcode::InteractionClaim>,
    add_small: Array<components::add_opcode_small::InteractionClaim>,
    add_ap: Array<components::add_ap_opcode::InteractionClaim>,
    assert_eq: Array<components::assert_eq_opcode::InteractionClaim>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::InteractionClaim>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::InteractionClaim>,
    blake: Array<components::blake_compress_opcode::InteractionClaim>,
    call: Array<components::call_opcode_abs::InteractionClaim>,
    call_rel_imm: Array<components::call_opcode_rel_imm::InteractionClaim>,
    generic: Array<components::generic_opcode::InteractionClaim>,
    jnz: Array<components::jnz_opcode_non_taken::InteractionClaim>,
    jnz_taken: Array<components::jnz_opcode_taken::InteractionClaim>,
    jump: Array<components::jump_opcode_abs::InteractionClaim>,
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
    pub call: Array<components::call_opcode_abs::Claim>,
    pub call_rel_imm: Array<components::call_opcode_rel_imm::Claim>,
    pub generic: Array<components::generic_opcode::Claim>,
    pub jnz: Array<components::jnz_opcode_non_taken::Claim>,
    pub jnz_taken: Array<components::jnz_opcode_taken::Claim>,
    pub jump: Array<components::jump_opcode_abs::Claim>,
    pub jump_double_deref: Array<components::jump_opcode_double_deref::Claim>,
    pub jump_rel: Array<components::jump_opcode_rel::Claim>,
    pub jump_rel_imm: Array<components::jump_opcode_rel_imm::Claim>,
    pub mul: Array<components::mul_opcode::Claim>,
    pub mul_small: Array<components::mul_opcode_small::Claim>,
    pub qm31: Array<components::qm_31_add_mul_opcode::Claim>,
    pub ret: Array<components::ret_opcode::Claim>,
}

pub impl OpcodeClaimImpl of ClaimTrait<OpcodeClaim> {
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
            claim.accumulate_relation_uses(ref relation_uses);
        }
        for claim in add_small.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in add_ap.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in assert_eq.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in assert_eq_imm.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in assert_eq_double_deref.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in blake.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in call.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in call_rel_imm.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in generic.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in jnz.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in jnz_taken.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in jump.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in jump_double_deref.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in jump_rel.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in jump_rel_imm.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in mul.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in mul_small.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in qm31.span() {
            claim.accumulate_relation_uses(ref relation_uses);
        }

        for claim in ret.span() {
            claim.accumulate_relation_uses(ref relation_uses);
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
    call: Array<components::call_opcode_abs::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    generic: Array<components::generic_opcode::Component>,
    jnz: Array<components::jnz_opcode_non_taken::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode_abs::Component>,
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
    call: Array<components::call_opcode_abs::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    jnz: Array<components::jnz_opcode_non_taken::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode_abs::Component>,
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
        let OpcodeClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
        } = claim;

        let OpcodeInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
        } = interaction_claim;

        // Add components
        let mut add_components = array![];
        for (claim, interaction_claim) in zip_eq(add_claims.span(), add_interaction_claims.span()) {
            add_components
                .append(
                    components::add_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Add Small components
        let mut add_small_components = array![];
        for (claim, interaction_claim) in zip_eq(
            add_small_claims.span(), add_small_interaction_claims.span(),
        ) {
            add_small_components
                .append(
                    components::add_opcode_small::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Add AP components
        let mut add_ap_components = array![];
        for (claim, interaction_claim) in zip_eq(
            add_ap_claims.span(), add_ap_interaction_claims.span(),
        ) {
            add_ap_components
                .append(
                    components::add_ap_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Assert Eq components
        let mut assert_eq_components = array![];
        for (claim, interaction_claim) in zip_eq(
            assert_eq_claims.span(), assert_eq_interaction_claims.span(),
        ) {
            assert_eq_components
                .append(
                    components::assert_eq_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Assert Eq Imm components
        let mut assert_eq_imm_components = array![];
        for (claim, interaction_claim) in zip_eq(
            assert_eq_imm_claims.span(), assert_eq_imm_interaction_claims.span(),
        ) {
            assert_eq_imm_components
                .append(
                    components::assert_eq_opcode_imm::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Assert Eq Double Deref components
        let mut assert_eq_double_deref_components = array![];
        for (claim, interaction_claim) in zip_eq(
            assert_eq_double_deref_claims.span(), assert_eq_double_deref_interaction_claims.span(),
        ) {
            assert_eq_double_deref_components
                .append(
                    components::assert_eq_opcode_double_deref::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        let mut blake_components = array![];
        for (claim, interaction_claim) in zip_eq(
            blake_claims.span(), blake_interaction_claims.span(),
        ) {
            blake_components
                .append(
                    components::blake_compress_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Call components
        let mut call_components = array![];
        for (claim, interaction_claim) in zip_eq(
            call_claims.span(), call_interaction_claims.span(),
        ) {
            call_components
                .append(
                    components::call_opcode_abs::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Call Rel_imm components
        let mut call_rel_imm_components = array![];
        for (claim, interaction_claim) in zip_eq(
            call_rel_imm_claims.span(), call_rel_imm_interaction_claims.span(),
        ) {
            call_rel_imm_components
                .append(
                    components::call_opcode_rel_imm::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Generic components
        let mut generic_components = array![];
        for (claim, interaction_claim) in zip_eq(
            generic_claims.span(), generic_interaction_claims.span(),
        ) {
            generic_components
                .append(
                    components::generic_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jnz components
        let mut jnz_components = array![];
        for (claim, interaction_claim) in zip_eq(jnz_claims.span(), jnz_interaction_claims.span()) {
            jnz_components
                .append(
                    components::jnz_opcode_non_taken::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jnz Taken components
        let mut jnz_taken_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jnz_taken_claims.span(), jnz_taken_interaction_claims.span(),
        ) {
            jnz_taken_components
                .append(
                    components::jnz_opcode_taken::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump components
        let mut jump_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_claims.span(), jump_interaction_claims.span(),
        ) {
            jump_components
                .append(
                    components::jump_opcode_abs::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump Double Deref components
        let mut jump_double_deref_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_double_deref_claims.span(), jump_double_deref_interaction_claims.span(),
        ) {
            jump_double_deref_components
                .append(
                    components::jump_opcode_double_deref::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump Rel components
        let mut jump_rel_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_rel_claims.span(), jump_rel_interaction_claims.span(),
        ) {
            jump_rel_components
                .append(
                    components::jump_opcode_rel::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump Rel Imm components
        let mut jump_rel_imm_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_rel_imm_claims.span(), jump_rel_imm_interaction_claims.span(),
        ) {
            jump_rel_imm_components
                .append(
                    components::jump_opcode_rel_imm::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Mul components
        let mut mul_components = array![];
        for (claim, interaction_claim) in zip_eq(mul_claims.span(), mul_interaction_claims.span()) {
            mul_components
                .append(
                    components::mul_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Mul Small components
        let mut mul_small_components = array![];
        for (claim, interaction_claim) in zip_eq(
            mul_small_claims.span(), mul_small_interaction_claims.span(),
        ) {
            mul_small_components
                .append(
                    components::mul_opcode_small::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // QM31 components
        let mut qm31_components = array![];
        for (claim, interaction_claim) in zip_eq(
            qm31_claims.span(), qm31_interaction_claims.span(),
        ) {
            qm31_components
                .append(
                    components::qm_31_add_mul_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Ret components
        let mut ret_components = array![];
        for (claim, interaction_claim) in zip_eq(ret_claims.span(), ret_interaction_claims.span()) {
            ret_components
                .append(
                    components::ret_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

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
        let OpcodeClaim {
            add: add_claims,
            add_small: add_small_claims,
            add_ap: add_ap_claims,
            assert_eq: assert_eq_claims,
            assert_eq_imm: assert_eq_imm_claims,
            assert_eq_double_deref: assert_eq_double_deref_claims,
            blake: blake_claims,
            call: call_claims,
            call_rel_imm: call_rel_imm_claims,
            generic: generic_claims,
            jnz: jnz_claims,
            jnz_taken: jnz_taken_claims,
            jump: jump_claims,
            jump_double_deref: jump_double_deref_claims,
            jump_rel: jump_rel_claims,
            jump_rel_imm: jump_rel_imm_claims,
            mul: mul_claims,
            mul_small: mul_small_claims,
            qm31: qm31_claims,
            ret: ret_claims,
        } = claim;
        assert!(generic_claims.is_empty(), "The generic opcode is not supported.");

        let OpcodeInteractionClaim {
            add: add_interaction_claims,
            add_small: add_small_interaction_claims,
            add_ap: add_ap_interaction_claims,
            assert_eq: assert_eq_interaction_claims,
            assert_eq_imm: assert_eq_imm_interaction_claims,
            assert_eq_double_deref: assert_eq_double_deref_interaction_claims,
            blake: blake_interaction_claims,
            call: call_interaction_claims,
            call_rel_imm: call_rel_imm_interaction_claims,
            generic: generic_interaction_claims,
            jnz: jnz_interaction_claims,
            jnz_taken: jnz_taken_interaction_claims,
            jump: jump_interaction_claims,
            jump_double_deref: jump_double_deref_interaction_claims,
            jump_rel: jump_rel_interaction_claims,
            jump_rel_imm: jump_rel_imm_interaction_claims,
            mul: mul_interaction_claims,
            mul_small: mul_small_interaction_claims,
            qm31: qm31_interaction_claims,
            ret: ret_interaction_claims,
        } = interaction_claim;

        for _ in zip_eq(generic_claims.span(), generic_interaction_claims.span()) {
            panic!("The generic opcode is not supported.");
        }

        // Add components
        let mut add_components = array![];
        for (claim, interaction_claim) in zip_eq(add_claims.span(), add_interaction_claims.span()) {
            add_components
                .append(
                    components::add_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Add Small components
        let mut add_small_components = array![];
        for (claim, interaction_claim) in zip_eq(
            add_small_claims.span(), add_small_interaction_claims.span(),
        ) {
            add_small_components
                .append(
                    components::add_opcode_small::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Add AP components
        let mut add_ap_components = array![];
        for (claim, interaction_claim) in zip_eq(
            add_ap_claims.span(), add_ap_interaction_claims.span(),
        ) {
            add_ap_components
                .append(
                    components::add_ap_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Assert Eq components
        let mut assert_eq_components = array![];
        for (claim, interaction_claim) in zip_eq(
            assert_eq_claims.span(), assert_eq_interaction_claims.span(),
        ) {
            assert_eq_components
                .append(
                    components::assert_eq_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Assert Eq Imm components
        let mut assert_eq_imm_components = array![];
        for (claim, interaction_claim) in zip_eq(
            assert_eq_imm_claims.span(), assert_eq_imm_interaction_claims.span(),
        ) {
            assert_eq_imm_components
                .append(
                    components::assert_eq_opcode_imm::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Assert Eq Double Deref components
        let mut assert_eq_double_deref_components = array![];
        for (claim, interaction_claim) in zip_eq(
            assert_eq_double_deref_claims.span(), assert_eq_double_deref_interaction_claims.span(),
        ) {
            assert_eq_double_deref_components
                .append(
                    components::assert_eq_opcode_double_deref::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        let mut blake_components = array![];
        for (claim, interaction_claim) in zip_eq(
            blake_claims.span(), blake_interaction_claims.span(),
        ) {
            blake_components
                .append(
                    components::blake_compress_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Call components
        let mut call_components = array![];
        for (claim, interaction_claim) in zip_eq(
            call_claims.span(), call_interaction_claims.span(),
        ) {
            call_components
                .append(
                    components::call_opcode_abs::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Call Rel_imm components
        let mut call_rel_imm_components = array![];
        for (claim, interaction_claim) in zip_eq(
            call_rel_imm_claims.span(), call_rel_imm_interaction_claims.span(),
        ) {
            call_rel_imm_components
                .append(
                    components::call_opcode_rel_imm::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jnz components
        let mut jnz_components = array![];
        for (claim, interaction_claim) in zip_eq(jnz_claims.span(), jnz_interaction_claims.span()) {
            jnz_components
                .append(
                    components::jnz_opcode_non_taken::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jnz Taken components
        let mut jnz_taken_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jnz_taken_claims.span(), jnz_taken_interaction_claims.span(),
        ) {
            jnz_taken_components
                .append(
                    components::jnz_opcode_taken::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump components
        let mut jump_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_claims.span(), jump_interaction_claims.span(),
        ) {
            jump_components
                .append(
                    components::jump_opcode_abs::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump Double Deref components
        let mut jump_double_deref_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_double_deref_claims.span(), jump_double_deref_interaction_claims.span(),
        ) {
            jump_double_deref_components
                .append(
                    components::jump_opcode_double_deref::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump Rel components
        let mut jump_rel_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_rel_claims.span(), jump_rel_interaction_claims.span(),
        ) {
            jump_rel_components
                .append(
                    components::jump_opcode_rel::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Jump Rel Imm components
        let mut jump_rel_imm_components = array![];
        for (claim, interaction_claim) in zip_eq(
            jump_rel_imm_claims.span(), jump_rel_imm_interaction_claims.span(),
        ) {
            jump_rel_imm_components
                .append(
                    components::jump_opcode_rel_imm::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Mul components
        let mut mul_components = array![];
        for (claim, interaction_claim) in zip_eq(mul_claims.span(), mul_interaction_claims.span()) {
            mul_components
                .append(
                    components::mul_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Mul Small components
        let mut mul_small_components = array![];
        for (claim, interaction_claim) in zip_eq(
            mul_small_claims.span(), mul_small_interaction_claims.span(),
        ) {
            mul_small_components
                .append(
                    components::mul_opcode_small::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // QM31 components
        let mut qm31_components = array![];
        for (claim, interaction_claim) in zip_eq(
            qm31_claims.span(), qm31_interaction_claims.span(),
        ) {
            qm31_components
                .append(
                    components::qm_31_add_mul_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

        // Ret components
        let mut ret_components = array![];
        for (claim, interaction_claim) in zip_eq(ret_claims.span(), ret_interaction_claims.span()) {
            ret_components
                .append(
                    components::ret_opcode::NewComponentImpl::new(
                        claim, interaction_claim, interaction_elements,
                    ),
                );
        }

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

use components::add_ap_opcode::InteractionClaimImpl as AddApOpcodeInteractionClaimImpl;
use components::add_opcode::InteractionClaimImpl as AddOpcodeInteractionClaimImpl;
use components::add_opcode_small::InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl;
use components::assert_eq_opcode::InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl;
use components::assert_eq_opcode_double_deref::InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl;
use components::assert_eq_opcode_imm::InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl;
use components::blake_compress_opcode::InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl;
use components::call_opcode_abs::InteractionClaimImpl as CallOpcodeAbsInteractionClaimImpl;
use components::call_opcode_rel_imm::InteractionClaimImpl as CallOpcodeRelImmInteractionClaimImpl;
use components::generic_opcode::InteractionClaimImpl as GenericOpcodeInteractionClaimImpl;
use components::jnz_opcode_non_taken::InteractionClaimImpl as JnzOpcodeNonTakenInteractionClaimImpl;
use components::jnz_opcode_taken::InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl;
use components::jump_opcode_abs::InteractionClaimImpl as JumpOpcodeAbsInteractionClaimImpl;
use components::jump_opcode_double_deref::InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl;
use components::jump_opcode_rel::InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl;
use components::jump_opcode_rel_imm::InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl;
use components::memory_address_to_id::InteractionClaimImpl as MemoryAddressToIdInteractionClaimImpl;
use components::memory_id_to_big::{
    InteractionClaimImpl as MemoryIdToBigInteractionClaimImpl, LARGE_MEMORY_VALUE_ID_BASE,
};
use components::mul_opcode::InteractionClaimImpl as MulOpcodeInteractionClaimImpl;
use components::mul_opcode_small::InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl;
use components::qm_31_add_mul_opcode::InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl;
use components::ret_opcode::InteractionClaimImpl as RetOpcodeInteractionClaimImpl;
use components::triple_xor_32::InteractionClaimImpl as TripleXor32InteractionClaimImpl;
use components::verify_bitwise_xor_12::InteractionClaimImpl as VerifyBitwiseXor12InteractionClaimImpl;
use components::verify_bitwise_xor_4::InteractionClaimImpl as VerifyBitwiseXor4InteractionClaimImpl;
use components::verify_bitwise_xor_7::InteractionClaimImpl as VerifyBitwiseXor7InteractionClaimImpl;
use components::verify_bitwise_xor_8::InteractionClaimImpl as VerifyBitwiseXor8InteractionClaimImpl;
use components::verify_bitwise_xor_8_b::InteractionClaimImpl as VerifyBitwiseXor8BInteractionClaimImpl;
use components::verify_bitwise_xor_9::InteractionClaimImpl as VerifyBitwiseXor9InteractionClaimImpl;
use components::verify_instruction::InteractionClaimImpl as VerifyInstructionInteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::blake::*;
use stwo_cairo_air::builtins::*;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::cairo_interaction_elements::CairoInteractionElements;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, mul_opcode, mul_opcode_small,
    qm_31_add_mul_opcode, ret_opcode,
};
use crate::P_U32;

#[cfg(not(feature: "poseidon252_verifier"))]
pub mod pedersen_context_imports {
    pub use stwo_cairo_air::pedersen::{PedersenContextComponents, PedersenContextComponentsImpl};
}
#[cfg(not(feature: "poseidon252_verifier"))]
use pedersen_context_imports::*;
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
pub mod poseidon_context_imports {
    pub use stwo_cairo_air::poseidon::{PoseidonContextComponents, PoseidonContextComponentsImpl};
}
#[cfg(or(not(feature: "poseidon252_verifier"), feature: "poseidon_outputs_packing"))]
use poseidon_context_imports::*;
use stwo_cairo_air::blake::{
    BlakeContextClaim, BlakeContextComponents, BlakeContextComponentsImpl,
    BlakeContextInteractionClaim, BlakeContextInteractionClaimImpl,
};
use stwo_cairo_air::builtins::{
    BuiltinsClaim, BuiltinsInteractionClaim, BuiltinsInteractionClaimImpl,
};
use stwo_cairo_air::pedersen::{
    PedersenContextClaim, PedersenContextInteractionClaim, PedersenContextInteractionClaimImpl,
};
use stwo_cairo_air::poseidon::{
    PoseidonContextClaim, PoseidonContextInteractionClaim, PoseidonContextInteractionClaimImpl,
};
use stwo_cairo_air::preprocessed_columns::{NUM_PREPROCESSED_COLUMNS, PREPROCESSED_COLUMN_LOG_SIZE};
use stwo_cairo_air::range_checks::{
    RangeChecksClaim, RangeChecksComponents, RangeChecksComponentsImpl, RangeChecksInteractionClaim,
    RangeChecksInteractionClaimImpl, RangeChecksInteractionElementsImpl,
};
use stwo_cairo_air::{PublicData, PublicDataImpl, RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    LookupElements, LookupElementsImpl, PreprocessedColumnSet, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl, pow2};
use stwo_verifier_core::verifier::Air;
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray, TreeSpan};


pub type Cube252Elements = LookupElements<20>;

pub type MemoryAddressToIdElements = LookupElements<2>;

pub type MemoryIdToBigElements = LookupElements<29>;

pub type OpcodesElements = LookupElements<3>;

pub type PartialEcMulElements = LookupElements<72>;

pub type PedersenPointsTableElements = LookupElements<57>;

pub type PoseidonFullRoundChainElements = LookupElements<32>;

pub type Poseidon3PartialRoundsChainElements = LookupElements<42>;

pub type PoseidonAggregatorElements = LookupElements<6>;

pub type PoseidonRoundKeysElements = LookupElements<31>;

pub type BlakeGElements = LookupElements<20>;

pub type BlakeRoundElements = LookupElements<35>;

pub type BlakeRoundSigmaElements = LookupElements<17>;

pub type TripleXor32Elements = LookupElements<8>;

pub type RangeCheck252Width27Elements = LookupElements<10>;

pub type VerifyInstructionElements = LookupElements<7>;

pub type VerifyBitwiseXor_4Elements = LookupElements<3>;

pub type VerifyBitwiseXor_7Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8Elements = LookupElements<3>;

pub type VerifyBitwiseXor_8_BElements = LookupElements<3>;

pub type VerifyBitwiseXor_9Elements = LookupElements<3>;

pub type VerifyBitwiseXor_12Elements = LookupElements<3>;


#[derive(Drop, Serde)]
pub struct CairoClaim {
    pub public_data: PublicData,
    pub add: Option<add_opcode::Claim>,
    pub add_small: Option<add_opcode_small::Claim>,
    pub add_ap: Option<add_ap_opcode::Claim>,
    pub assert_eq: Option<assert_eq_opcode::Claim>,
    pub assert_eq_imm: Option<assert_eq_opcode_imm::Claim>,
    pub assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Claim>,
    pub blake: Option<blake_compress_opcode::Claim>,
    pub call: Option<call_opcode_abs::Claim>,
    pub call_rel_imm: Option<call_opcode_rel_imm::Claim>,
    pub generic: Option<generic_opcode::Claim>,
    pub jnz: Option<jnz_opcode_non_taken::Claim>,
    pub jnz_taken: Option<jnz_opcode_taken::Claim>,
    pub jump: Option<jump_opcode_abs::Claim>,
    pub jump_double_deref: Option<jump_opcode_double_deref::Claim>,
    pub jump_rel: Option<jump_opcode_rel::Claim>,
    pub jump_rel_imm: Option<jump_opcode_rel_imm::Claim>,
    pub mul: Option<mul_opcode::Claim>,
    pub mul_small: Option<mul_opcode_small::Claim>,
    pub qm31: Option<qm_31_add_mul_opcode::Claim>,
    pub ret: Option<ret_opcode::Claim>,
    pub verify_instruction: components::verify_instruction::Claim,
    pub blake_context: BlakeContextClaim,
    pub builtins: BuiltinsClaim,
    pub pedersen_context: PedersenContextClaim,
    pub poseidon_context: PoseidonContextClaim,
    pub memory_address_to_id: components::memory_address_to_id::Claim,
    pub memory_id_to_value: components::memory_id_to_big::Claim,
    pub range_checks: RangeChecksClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::Claim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::Claim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::Claim,
    pub verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Claim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::Claim,
    // ...
}

pub impl CairoClaimImpl of ClaimTrait<CairoClaim> {
    fn log_sizes(self: @CairoClaim) -> TreeArray<Span<u32>> {
        let mut log_sizes_list = array![];
        if let Some(claim) = self.add {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.add_small {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.add_ap {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.assert_eq_double_deref {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.blake {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.call {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.call_rel_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.generic {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jnz {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jnz_taken {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_double_deref {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_rel {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.jump_rel_imm {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.mul_small {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.qm31 {
            log_sizes_list.append(claim.log_sizes());
        }
        if let Some(claim) = self.ret {
            log_sizes_list.append(claim.log_sizes());
        }

        log_sizes_list.append(self.verify_instruction.log_sizes());
        log_sizes_list.append(self.blake_context.log_sizes());
        log_sizes_list.append(self.builtins.log_sizes());
        log_sizes_list.append(self.pedersen_context.log_sizes());
        log_sizes_list.append(self.poseidon_context.log_sizes());
        log_sizes_list.append(self.memory_address_to_id.log_sizes());
        log_sizes_list.append(self.memory_id_to_value.log_sizes());
        log_sizes_list.append(self.range_checks.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_4.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_7.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_8.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_8_b.log_sizes());
        log_sizes_list.append(self.verify_bitwise_xor_9.log_sizes());

        let mut aggregated_log_sizes = utils::tree_array_concat_cols(log_sizes_list);

        // Overwrite the preprocessed trace log sizes.
        let _invalid_preprocessed_trace_log_sizes = aggregated_log_sizes.pop_front();

        let mut preprocessed_trace_log_sizes = array![];

        for log_size in PREPROCESSED_COLUMN_LOG_SIZE.span() {
            preprocessed_trace_log_sizes.append(*log_size);
        }

        let trace_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        let interaction_log_sizes = aggregated_log_sizes.pop_front().unwrap();
        assert!(aggregated_log_sizes.is_empty());

        array![preprocessed_trace_log_sizes.span(), trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @CairoClaim, ref channel: Channel) {
        let CairoClaim {
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

        public_data.mix_into(ref channel);

        if let Some(claim) = add {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = add_small {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = add_ap {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = assert_eq {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = assert_eq_imm {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = assert_eq_double_deref {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = blake {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = call {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = call_rel_imm {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = generic {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jnz {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jnz_taken {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump_double_deref {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump_rel {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = jump_rel_imm {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = mul {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = mul_small {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = qm31 {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        if let Some(claim) = ret {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }

        verify_instruction.mix_into(ref channel);
        blake_context.mix_into(ref channel);
        builtins.mix_into(ref channel);
        pedersen_context.mix_into(ref channel);
        poseidon_context.mix_into(ref channel);
        memory_address_to_id.mix_into(ref channel);
        memory_id_to_value.mix_into(ref channel);
        range_checks.mix_into(ref channel);
        verify_bitwise_xor_4.mix_into(ref channel);
        verify_bitwise_xor_7.mix_into(ref channel);
        verify_bitwise_xor_8.mix_into(ref channel);
        verify_bitwise_xor_8_b.mix_into(ref channel);
        verify_bitwise_xor_9.mix_into(ref channel);
    }

    fn accumulate_relation_uses(self: @CairoClaim, ref relation_uses: RelationUsesDict) {
        let CairoClaim {
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

        if let Some(claim) = add {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = add_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = add_ap {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = assert_eq {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = assert_eq_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = assert_eq_double_deref {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = blake {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = call {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = call_rel_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = generic {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jnz {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jnz_taken {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump_double_deref {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump_rel {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = jump_rel_imm {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = mul {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = mul_small {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = qm31 {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = ret {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        blake_context.accumulate_relation_uses(ref relation_uses);
        builtins.accumulate_relation_uses(ref relation_uses);
        pedersen_context.accumulate_relation_uses(ref relation_uses);
        poseidon_context.accumulate_relation_uses(ref relation_uses);
        verify_instruction.accumulate_relation_uses(ref relation_uses);
        memory_id_to_value.accumulate_relation_uses(ref relation_uses);
    }
}


#[derive(Drop, Serde)]
pub struct CairoInteractionClaim {
    pub add: Option<add_opcode::InteractionClaim>,
    pub add_small: Option<add_opcode_small::InteractionClaim>,
    pub add_ap: Option<add_ap_opcode::InteractionClaim>,
    pub assert_eq: Option<assert_eq_opcode::InteractionClaim>,
    pub assert_eq_imm: Option<assert_eq_opcode_imm::InteractionClaim>,
    pub assert_eq_double_deref: Option<assert_eq_opcode_double_deref::InteractionClaim>,
    pub blake: Option<blake_compress_opcode::InteractionClaim>,
    pub call: Option<call_opcode_abs::InteractionClaim>,
    pub call_rel_imm: Option<call_opcode_rel_imm::InteractionClaim>,
    pub generic: Option<generic_opcode::InteractionClaim>,
    pub jnz: Option<jnz_opcode_non_taken::InteractionClaim>,
    pub jnz_taken: Option<jnz_opcode_taken::InteractionClaim>,
    pub jump: Option<jump_opcode_abs::InteractionClaim>,
    pub jump_double_deref: Option<jump_opcode_double_deref::InteractionClaim>,
    pub jump_rel: Option<jump_opcode_rel::InteractionClaim>,
    pub jump_rel_imm: Option<jump_opcode_rel_imm::InteractionClaim>,
    pub mul: Option<mul_opcode::InteractionClaim>,
    pub mul_small: Option<mul_opcode_small::InteractionClaim>,
    pub qm31: Option<qm_31_add_mul_opcode::InteractionClaim>,
    pub ret: Option<ret_opcode::InteractionClaim>,
    pub verify_instruction: components::verify_instruction::InteractionClaim,
    pub blake_context: BlakeContextInteractionClaim,
    pub builtins: BuiltinsInteractionClaim,
    pub pedersen_context: PedersenContextInteractionClaim,
    pub poseidon_context: PoseidonContextInteractionClaim,
    pub memory_address_to_id: components::memory_address_to_id::InteractionClaim,
    pub memory_id_to_value: components::memory_id_to_big::InteractionClaim,
    pub range_checks: RangeChecksInteractionClaim,
    pub verify_bitwise_xor_4: components::verify_bitwise_xor_4::InteractionClaim,
    pub verify_bitwise_xor_7: components::verify_bitwise_xor_7::InteractionClaim,
    pub verify_bitwise_xor_8: components::verify_bitwise_xor_8::InteractionClaim,
    pub verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::InteractionClaim,
    pub verify_bitwise_xor_9: components::verify_bitwise_xor_9::InteractionClaim,
}

#[generate_trait]
pub impl CairoInteractionClaimImpl of CairoInteractionClaimTrace {
    fn mix_into(self: @CairoInteractionClaim, ref channel: Channel) {
        let CairoInteractionClaim {
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

        if let Some(interaction_claim) = add {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = add_small {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = add_ap {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = assert_eq {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = assert_eq_imm {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = assert_eq_double_deref {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = blake {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = call {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = call_rel_imm {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = generic {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jnz {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jnz_taken {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump_double_deref {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump_rel {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = jump_rel_imm {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = mul {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = mul_small {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = qm31 {
            interaction_claim.mix_into(ref channel);
        }
        if let Some(interaction_claim) = ret {
            interaction_claim.mix_into(ref channel);
        }
        verify_instruction.mix_into(ref channel);
        blake_context.mix_into(ref channel);
        builtins.mix_into(ref channel);
        pedersen_context.mix_into(ref channel);
        poseidon_context.mix_into(ref channel);
        memory_address_to_id.mix_into(ref channel);
        memory_id_to_value.mix_into(ref channel);
        range_checks.mix_into(ref channel);
        verify_bitwise_xor_4.mix_into(ref channel);
        verify_bitwise_xor_7.mix_into(ref channel);
        verify_bitwise_xor_8.mix_into(ref channel);
        verify_bitwise_xor_8_b.mix_into(ref channel);
        verify_bitwise_xor_9.mix_into(ref channel);
    }
}

pub fn lookup_sum(
    claim: @CairoClaim,
    elements: @CairoInteractionElements,
    interaction_claim: @CairoInteractionClaim,
) -> QM31 {
    let mut sum = claim.public_data.logup_sum(elements);
    // If the table is padded, take the sum of the non-padded values.
    // Otherwise, the claimed_sum is the total_sum.
    // TODO(Ohad): hide this logic behind `InteractionClaim`, and only sum here.

    // TODO(Andrew): double check this is correct order.
    let CairoInteractionClaim {
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
    } = interaction_claim;

    if let Some(interaction_claim) = add {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = add_small {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = add_ap {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = assert_eq {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = assert_eq_imm {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = assert_eq_double_deref {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = blake {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = call {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = call_rel_imm {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = generic {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jnz {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jnz_taken {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump_double_deref {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump_rel {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = jump_rel_imm {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = mul {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = mul_small {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = qm31 {
        sum += *interaction_claim.claimed_sum;
    }
    if let Some(interaction_claim) = ret {
        sum += *interaction_claim.claimed_sum;
    }

    sum += *verify_instruction.claimed_sum;
    sum += blake_context.sum();
    sum += builtins.sum();
    sum += pedersen_context.sum();
    sum += poseidon_context.sum();
    sum += *memory_address_to_id.claimed_sum;
    sum += memory_id_to_value.sum();
    sum += range_checks.sum();
    sum += *verify_bitwise_xor_4.claimed_sum;
    sum += *verify_bitwise_xor_7.claimed_sum;
    sum += *verify_bitwise_xor_8.claimed_sum;
    sum += *verify_bitwise_xor_8_b.claimed_sum;
    sum += *verify_bitwise_xor_9.claimed_sum;
    sum
}

#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct CairoAir {
    add: Option<add_opcode::Component>,
    add_small: Option<add_opcode_small::Component>,
    add_ap: Option<add_ap_opcode::Component>,
    assert_eq: Option<assert_eq_opcode::Component>,
    assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    blake: Option<blake_compress_opcode::Component>,
    call: Option<call_opcode_abs::Component>,
    call_rel_imm: Option<call_opcode_rel_imm::Component>,
    generic: Option<generic_opcode::Component>,
    jnz: Option<jnz_opcode_non_taken::Component>,
    jnz_taken: Option<jnz_opcode_taken::Component>,
    jump: Option<jump_opcode_abs::Component>,
    jump_double_deref: Option<jump_opcode_double_deref::Component>,
    jump_rel: Option<jump_opcode_rel::Component>,
    jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    mul: Option<mul_opcode::Component>,
    mul_small: Option<mul_opcode_small::Component>,
    qm31: Option<qm_31_add_mul_opcode::Component>,
    ret: Option<ret_opcode::Component>,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    pedersen_context: PedersenContextComponents,
    poseidon_context: PoseidonContextComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let CairoClaim {
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
            ..,
        } = cairo_claim;

        let CairoInteractionClaim {
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
            ..,
        } = interaction_claim;

        // Add component
        let add_component = add_claims
            .map(
                |claim| {
                    let interaction_claim = add_interaction_claims.unwrap();
                    add_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );
        // Add Small component
        let add_small_component = add_small_claims
            .map(
                |claim| {
                    let interaction_claim = add_small_interaction_claims.unwrap();
                    add_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add AP component
        let add_ap_component = add_ap_claims
            .map(
                |claim| {
                    let interaction_claim = add_ap_interaction_claims.unwrap();
                    add_ap_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq component
        let assert_eq_component = assert_eq_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_interaction_claims.unwrap();
                    assert_eq_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Imm component
        let assert_eq_imm_component = assert_eq_imm_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_imm_interaction_claims.unwrap();
                    assert_eq_opcode_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Double Deref component
        let assert_eq_double_deref_component = assert_eq_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_double_deref_interaction_claims.unwrap();
                    assert_eq_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_component = blake_claims
            .map(
                |claim| {
                    let interaction_claim = blake_interaction_claims.unwrap();
                    blake_compress_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call component
        let call_component = call_claims
            .map(
                |claim| {
                    let interaction_claim = call_interaction_claims.unwrap();
                    call_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call Rel_imm component
        let call_rel_imm_component = call_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = call_rel_imm_interaction_claims.unwrap();
                    call_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Generic component
        let generic_component = generic_claims
            .map(
                |claim| {
                    let interaction_claim = generic_interaction_claims.unwrap();
                    generic_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz component
        let jnz_component = jnz_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_interaction_claims.unwrap();
                    jnz_opcode_non_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz Taken component
        let jnz_taken_component = jnz_taken_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_taken_interaction_claims.unwrap();
                    jnz_opcode_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump component
        let jump_component = jump_claims
            .map(
                |claim| {
                    let interaction_claim = jump_interaction_claims.unwrap();
                    jump_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Double Deref component
        let jump_double_deref_component = jump_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = jump_double_deref_interaction_claims.unwrap();
                    jump_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel component
        let jump_rel_component = jump_rel_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_interaction_claims.unwrap();
                    jump_opcode_rel::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel Imm component
        let jump_rel_imm_component = jump_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_imm_interaction_claims.unwrap();
                    jump_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul component
        let mul_component = mul_claims
            .map(
                |claim| {
                    let interaction_claim = mul_interaction_claims.unwrap();
                    mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul Small component
        let mul_small_component = mul_small_claims
            .map(
                |claim| {
                    let interaction_claim = mul_small_interaction_claims.unwrap();
                    mul_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // QM31 component
        let qm31_component = qm31_claims
            .map(
                |claim| {
                    let interaction_claim = qm31_interaction_claims.unwrap();
                    qm_31_add_mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Ret component
        let ret_component = ret_claims
            .map(
                |claim| {
                    let interaction_claim = ret_interaction_claims.unwrap();
                    ret_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let pedersen_context_components = PedersenContextComponentsImpl::new(
            cairo_claim.pedersen_context, interaction_elements, interaction_claim.pedersen_context,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_8_b_component =
            components::verify_bitwise_xor_8_b::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8_b,
            interaction_claim.verify_bitwise_xor_8_b,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
        );

        CairoAir {
            add: add_component,
            add_small: add_small_component,
            add_ap: add_ap_component,
            assert_eq: assert_eq_component,
            assert_eq_imm: assert_eq_imm_component,
            assert_eq_double_deref: assert_eq_double_deref_component,
            blake: blake_component,
            call: call_component,
            call_rel_imm: call_rel_imm_component,
            generic: generic_component,
            jnz: jnz_component,
            jnz_taken: jnz_taken_component,
            jump: jump_component,
            jump_double_deref: jump_double_deref_component,
            jump_rel: jump_rel_component,
            jump_rel_imm: jump_rel_imm_component,
            mul: mul_component,
            mul_small: mul_small_component,
            qm31: qm31_component,
            ret: ret_component,
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            pedersen_context: pedersen_context_components,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(not(feature: "poseidon252_verifier"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );
        let CairoAir {
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
            composition_log_degree_bound: _,
        } = self;

        if let Some(component) = add {
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
        if let Some(component) = add_small {
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
        if let Some(component) = add_ap {
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
        if let Some(component) = assert_eq {
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
        if let Some(component) = assert_eq_imm {
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
        if let Some(component) = assert_eq_double_deref {
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
        if let Some(component) = blake {
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
        if let Some(component) = call {
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
        if let Some(component) = call_rel_imm {
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
        if let Some(component) = generic {
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
        if let Some(component) = jnz {
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
        if let Some(component) = jnz_taken {
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
        if let Some(component) = jump {
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
        if let Some(component) = jump_double_deref {
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
        if let Some(component) = jump_rel {
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
        if let Some(component) = jump_rel_imm {
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
        if let Some(component) = mul {
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
        if let Some(component) = mul_small {
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
        if let Some(component) = qm31 {
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
        if let Some(component) = ret {
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
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        pedersen_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );

        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct CairoAir {
    add: Option<add_opcode::Component>,
    add_small: Option<add_opcode_small::Component>,
    add_ap: Option<add_ap_opcode::Component>,
    assert_eq: Option<assert_eq_opcode::Component>,
    assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    blake: Option<blake_compress_opcode::Component>,
    call: Option<call_opcode_abs::Component>,
    call_rel_imm: Option<call_opcode_rel_imm::Component>,
    jnz: Option<jnz_opcode_non_taken::Component>,
    jnz_taken: Option<jnz_opcode_taken::Component>,
    jump: Option<jump_opcode_abs::Component>,
    jump_double_deref: Option<jump_opcode_double_deref::Component>,
    jump_rel: Option<jump_opcode_rel::Component>,
    jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    mul: Option<mul_opcode::Component>,
    mul_small: Option<mul_opcode_small::Component>,
    qm31: Option<qm_31_add_mul_opcode::Component>,
    ret: Option<ret_opcode::Component>,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let CairoClaim {
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
            ..,
        } = cairo_claim;
        assert!(generic_claims.is_none(), "The generic opcode is not supported.");

        let CairoInteractionClaim {
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
            ..,
        } = interaction_claim;

        assert!(generic_claims.is_none(), "The generic opcode is not supported.");
        assert!(generic_interaction_claims.is_none(), "The generic opcode is not supported.");

        // Add components
        let add_components = add_claims
            .map(
                |claim| {
                    let interaction_claim = add_interaction_claims.unwrap();
                    add_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add Small components
        let add_small_components = add_small_claims
            .map(
                |claim| {
                    let interaction_claim = add_small_interaction_claims.unwrap();
                    add_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add AP components
        let add_ap_components = add_ap_claims
            .map(
                |claim| {
                    let interaction_claim = add_ap_interaction_claims.unwrap();
                    add_ap_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq components
        let assert_eq_components = assert_eq_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_interaction_claims.unwrap();
                    assert_eq_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Imm components
        let assert_eq_imm_components = assert_eq_imm_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_imm_interaction_claims.unwrap();
                    assert_eq_opcode_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Double Deref components
        let assert_eq_double_deref_components = assert_eq_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_double_deref_interaction_claims.unwrap();
                    assert_eq_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_components = blake_claims
            .map(
                |claim| {
                    let interaction_claim = blake_interaction_claims.unwrap();
                    blake_compress_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call components
        let call_components = call_claims
            .map(
                |claim| {
                    let interaction_claim = call_interaction_claims.unwrap();
                    call_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call Rel_imm components
        let call_rel_imm_components = call_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = call_rel_imm_interaction_claims.unwrap();
                    call_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz components
        let jnz_components = jnz_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_interaction_claims.unwrap();
                    jnz_opcode_non_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz Taken components
        let jnz_taken_components = jnz_taken_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_taken_interaction_claims.unwrap();
                    jnz_opcode_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump components
        let jump_components = jump_claims
            .map(
                |claim| {
                    let interaction_claim = jump_interaction_claims.unwrap();
                    jump_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Double Deref components
        let jump_double_deref_components = jump_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = jump_double_deref_interaction_claims.unwrap();
                    jump_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel components
        let jump_rel_components = jump_rel_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_interaction_claims.unwrap();
                    jump_opcode_rel::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel Imm components
        let jump_rel_imm_components = jump_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_imm_interaction_claims.unwrap();
                    jump_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul components
        let mul_components = mul_claims
            .map(
                |claim| {
                    let interaction_claim = mul_interaction_claims.unwrap();
                    mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul Small components
        let mul_small_components = mul_small_claims
            .map(
                |claim| {
                    let interaction_claim = mul_small_interaction_claims.unwrap();
                    mul_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // QM31 components
        let qm31_components = qm31_claims
            .map(
                |claim| {
                    let interaction_claim = qm31_interaction_claims.unwrap();
                    qm_31_add_mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Ret components
        let ret_components = ret_claims
            .map(
                |claim| {
                    let interaction_claim = ret_interaction_claims.unwrap();
                    ret_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_8_b_component =
            components::verify_bitwise_xor_8_b::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8_b,
            interaction_claim.verify_bitwise_xor_8_b,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
        );

        CairoAir {
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
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        let CairoAir {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
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
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
        } = self;

        if let Some(component) = add {
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
        if let Some(component) = add_small {
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
        if let Some(component) = add_ap {
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
        if let Some(component) = assert_eq {
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
        if let Some(component) = assert_eq_imm {
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
        if let Some(component) = assert_eq_double_deref {
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
        if let Some(component) = blake {
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
        if let Some(component) = call {
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
        if let Some(component) = call_rel_imm {
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
        if let Some(component) = jnz {
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
        if let Some(component) = jnz_taken {
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
        if let Some(component) = jump {
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
        if let Some(component) = jump_double_deref {
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
        if let Some(component) = jump_rel {
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
        if let Some(component) = jump_rel_imm {
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
        if let Some(component) = mul {
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
        if let Some(component) = mul_small {
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
        if let Some(component) = qm31 {
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
        if let Some(component) = ret {
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
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct CairoAir {
    add: Option<add_opcode::Component>,
    add_small: Option<add_opcode_small::Component>,
    add_ap: Option<add_ap_opcode::Component>,
    assert_eq: Option<assert_eq_opcode::Component>,
    assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    blake: Option<blake_compress_opcode::Component>,
    call: Option<call_opcode_abs::Component>,
    call_rel_imm: Option<call_opcode_rel_imm::Component>,
    jnz: Option<jnz_opcode_non_taken::Component>,
    jnz_taken: Option<jnz_opcode_taken::Component>,
    jump: Option<jump_opcode_abs::Component>,
    jump_double_deref: Option<jump_opcode_double_deref::Component>,
    jump_rel: Option<jump_opcode_rel::Component>,
    jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    mul: Option<mul_opcode::Component>,
    mul_small: Option<mul_opcode_small::Component>,
    qm31: Option<qm_31_add_mul_opcode::Component>,
    ret: Option<ret_opcode::Component>,
    verify_instruction: components::verify_instruction::Component,
    blake_context: BlakeContextComponents,
    builtins: BuiltinComponents,
    poseidon_context: PoseidonContextComponents,
    memory_address_to_id: components::memory_address_to_id::Component,
    memory_id_to_value: (
        Array<components::memory_id_to_big::BigComponent>,
        components::memory_id_to_big::SmallComponent,
    ),
    range_checks: RangeChecksComponents,
    verify_bitwise_xor_4: components::verify_bitwise_xor_4::Component,
    verify_bitwise_xor_7: components::verify_bitwise_xor_7::Component,
    verify_bitwise_xor_8: components::verify_bitwise_xor_8::Component,
    verify_bitwise_xor_8_b: components::verify_bitwise_xor_8_b::Component,
    verify_bitwise_xor_9: components::verify_bitwise_xor_9::Component,
    /// The degree bound of the cairo air.
    composition_log_degree_bound: u32,
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirNewImpl of CairoAirNewTrait {
    fn new(
        cairo_claim: @CairoClaim,
        interaction_elements: @CairoInteractionElements,
        interaction_claim: @CairoInteractionClaim,
        composition_log_degree_bound: u32,
    ) -> CairoAir {
        let CairoClaim {
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
            ..,
        } = cairo_claim;
        assert!(generic_claims.is_none(), "The generic opcode is not supported.");

        let CairoInteractionClaim {
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
            ..,
        } = interaction_claim;

        assert!(generic_claims.is_none(), "The generic opcode is not supported.");
        assert!(generic_interaction_claims.is_none(), "The generic opcode is not supported.");

        // Add components
        let add_components = add_claims
            .map(
                |claim| {
                    let interaction_claim = add_interaction_claims.unwrap();
                    add_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add Small components
        let add_small_components = add_small_claims
            .map(
                |claim| {
                    let interaction_claim = add_small_interaction_claims.unwrap();
                    add_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Add AP components
        let add_ap_components = add_ap_claims
            .map(
                |claim| {
                    let interaction_claim = add_ap_interaction_claims.unwrap();
                    add_ap_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq components
        let assert_eq_components = assert_eq_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_interaction_claims.unwrap();
                    assert_eq_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Imm components
        let assert_eq_imm_components = assert_eq_imm_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_imm_interaction_claims.unwrap();
                    assert_eq_opcode_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Assert Eq Double Deref components
        let assert_eq_double_deref_components = assert_eq_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = assert_eq_double_deref_interaction_claims.unwrap();
                    assert_eq_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_components = blake_claims
            .map(
                |claim| {
                    let interaction_claim = blake_interaction_claims.unwrap();
                    blake_compress_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call components
        let call_components = call_claims
            .map(
                |claim| {
                    let interaction_claim = call_interaction_claims.unwrap();
                    call_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Call Rel_imm components
        let call_rel_imm_components = call_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = call_rel_imm_interaction_claims.unwrap();
                    call_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz components
        let jnz_components = jnz_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_interaction_claims.unwrap();
                    jnz_opcode_non_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jnz Taken components
        let jnz_taken_components = jnz_taken_claims
            .map(
                |claim| {
                    let interaction_claim = jnz_taken_interaction_claims.unwrap();
                    jnz_opcode_taken::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump components
        let jump_components = jump_claims
            .map(
                |claim| {
                    let interaction_claim = jump_interaction_claims.unwrap();
                    jump_opcode_abs::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Double Deref components
        let jump_double_deref_components = jump_double_deref_claims
            .map(
                |claim| {
                    let interaction_claim = jump_double_deref_interaction_claims.unwrap();
                    jump_opcode_double_deref::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel components
        let jump_rel_components = jump_rel_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_interaction_claims.unwrap();
                    jump_opcode_rel::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Jump Rel Imm components
        let jump_rel_imm_components = jump_rel_imm_claims
            .map(
                |claim| {
                    let interaction_claim = jump_rel_imm_interaction_claims.unwrap();
                    jump_opcode_rel_imm::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul components
        let mul_components = mul_claims
            .map(
                |claim| {
                    let interaction_claim = mul_interaction_claims.unwrap();
                    mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Mul Small components
        let mul_small_components = mul_small_claims
            .map(
                |claim| {
                    let interaction_claim = mul_small_interaction_claims.unwrap();
                    mul_opcode_small::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // QM31 components
        let qm31_components = qm31_claims
            .map(
                |claim| {
                    let interaction_claim = qm31_interaction_claims.unwrap();
                    qm_31_add_mul_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        // Ret components
        let ret_components = ret_claims
            .map(
                |claim| {
                    let interaction_claim = ret_interaction_claims.unwrap();
                    ret_opcode::NewComponentImpl::new(
                        @claim, @interaction_claim, interaction_elements,
                    )
                },
            );

        let blake_context_component = BlakeContextComponentsImpl::new(
            cairo_claim.blake_context, interaction_elements, interaction_claim.blake_context,
        );

        let builtins_components = BuiltinComponentsImpl::new(
            cairo_claim.builtins, interaction_elements, interaction_claim.builtins,
        );

        let poseidon_context_components = PoseidonContextComponentsImpl::new(
            cairo_claim.poseidon_context, interaction_elements, interaction_claim.poseidon_context,
        );

        let verifyinstruction_component = components::verify_instruction::NewComponentImpl::new(
            cairo_claim.verify_instruction,
            interaction_claim.verify_instruction,
            interaction_elements,
        );

        let memory_address_to_id_component =
            components::memory_address_to_id::NewComponentImpl::new(
            cairo_claim.memory_address_to_id,
            interaction_claim.memory_address_to_id,
            interaction_elements,
        );

        assert!(
            cairo_claim
                .memory_id_to_value
                .big_log_sizes
                .len() == interaction_claim
                .memory_id_to_value
                .big_claimed_sums
                .len(),
        );
        let mut memory_id_to_value_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..cairo_claim.memory_id_to_value.big_log_sizes.len() {
            let log_size = *cairo_claim.memory_id_to_value.big_log_sizes[i];
            let claimed_sum = *interaction_claim.memory_id_to_value.big_claimed_sums[i];
            memory_id_to_value_components
                .append(
                    components::memory_id_to_big::NewBigComponentImpl::new(
                        log_size, offset, claimed_sum, interaction_elements,
                    ),
                );
            offset = offset + pow2(log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        let small_memory_id_to_value_component =
            components::memory_id_to_big::NewSmallComponentImpl::new(
            *cairo_claim.memory_id_to_value.small_log_size,
            *interaction_claim.memory_id_to_value.small_claimed_sum,
            interaction_elements,
        );

        let range_checks_components = RangeChecksComponentsImpl::new(
            cairo_claim.range_checks, interaction_elements, interaction_claim.range_checks,
        );

        let verify_bitwise_xor_4_component =
            components::verify_bitwise_xor_4::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_4,
            interaction_claim.verify_bitwise_xor_4,
            interaction_elements,
        );

        let verify_bitwise_xor_7_component =
            components::verify_bitwise_xor_7::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_7,
            interaction_claim.verify_bitwise_xor_7,
            interaction_elements,
        );

        let verify_bitwise_xor_8_component =
            components::verify_bitwise_xor_8::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8,
            interaction_claim.verify_bitwise_xor_8,
            interaction_elements,
        );

        let verify_bitwise_xor_8_b_component =
            components::verify_bitwise_xor_8_b::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_8_b,
            interaction_claim.verify_bitwise_xor_8_b,
            interaction_elements,
        );

        let verify_bitwise_xor_9_component =
            components::verify_bitwise_xor_9::NewComponentImpl::new(
            cairo_claim.verify_bitwise_xor_9,
            interaction_claim.verify_bitwise_xor_9,
            interaction_elements,
        );

        CairoAir {
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
            verify_instruction: verifyinstruction_component,
            blake_context: blake_context_component,
            builtins: builtins_components,
            poseidon_context: poseidon_context_components,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (memory_id_to_value_components, small_memory_id_to_value_component),
            range_checks: range_checks_components,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
            composition_log_degree_bound,
        }
    }
}

#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl CairoAirImpl of Air<CairoAir> {
    fn composition_log_degree_bound(self: @CairoAir) -> u32 {
        *self.composition_log_degree_bound
    }

    fn eval_composition_polynomial_at_point(
        self: @CairoAir,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let mut sum = Zero::zero();

        let [
            preprocessed_mask_values,
            mut trace_mask_values,
            mut interaction_trace_mask_values,
            _composition_trace_mask_values,
        ]: [ColumnSpan<Span<QM31>>; 4] =
            (*mask_values
            .try_into()
            .unwrap())
            .unbox();

        let mut preprocessed_mask_values = PreprocessedMaskValuesImpl::new(
            preprocessed_mask_values,
        );

        let CairoAir {
            add,
            add_small,
            add_ap,
            assert_eq,
            assert_eq_imm,
            assert_eq_double_deref,
            blake,
            call,
            call_rel_imm,
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
            poseidon_context,
            memory_address_to_id,
            memory_id_to_value,
            range_checks,
            verify_bitwise_xor_4,
            verify_bitwise_xor_7,
            verify_bitwise_xor_8,
            verify_bitwise_xor_8_b,
            verify_bitwise_xor_9,
            composition_log_degree_bound: _,
        } = self;

        if let Some(component) = add {
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
        if let Some(component) = add_small {
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
        if let Some(component) = add_ap {
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
        if let Some(component) = assert_eq {
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
        if let Some(component) = assert_eq_imm {
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
        if let Some(component) = assert_eq_double_deref {
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
        if let Some(component) = blake {
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
        if let Some(component) = call {
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
        if let Some(component) = call_rel_imm {
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
        if let Some(component) = jnz {
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
        if let Some(component) = jnz_taken {
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
        if let Some(component) = jump {
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
        if let Some(component) = jump_double_deref {
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
        if let Some(component) = jump_rel {
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
        if let Some(component) = jump_rel_imm {
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
        if let Some(component) = mul {
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
        if let Some(component) = mul_small {
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
        if let Some(component) = qm31 {
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
        if let Some(component) = ret {
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
        verify_instruction
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        blake_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        builtins
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        poseidon_context
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        memory_address_to_id
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        let (memory_id_to_value_big, memory_id_to_value_small) = memory_id_to_value;
        for memory_id_to_value_big_component in memory_id_to_value_big.span() {
            memory_id_to_value_big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
        memory_id_to_value_small
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        range_checks
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_4
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_7
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_8_b
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        verify_bitwise_xor_9
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_mask_values,
                ref trace_mask_values,
                ref interaction_trace_mask_values,
                random_coeff,
                point,
            );
        sum
    }
}

fn preprocessed_trace_mask_points(
    preprocessed_column_set: PreprocessedColumnSet, point: CirclePoint<QM31>,
) -> ColumnArray<Array<CirclePoint<QM31>>> {
    let mut mask_points = array![];

    let PreprocessedColumnSet { mut contains } = preprocessed_column_set;

    for idx in 0..NUM_PREPROCESSED_COLUMNS {
        if contains.get(idx.into()) {
            mask_points.append(array![point]);
            // Remove the item from the set.
            contains.insert(idx.into(), false);
        } else {
            mask_points.append(array![]);
        }
    }

    mask_points
}

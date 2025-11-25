use itertools::{chain, Itertools};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::air::Component;
use stwo::core::channel::Channel;
use stwo::core::fields::qm31::{SecureField, QM31};
use stwo::core::pcs::TreeVec;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::ComponentProver;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::TraceLocationAllocator;

use super::blake::air::{BlakeContextClaim, BlakeContextComponents, BlakeContextInteractionClaim};
use super::builtins_air::{BuiltinComponents, BuiltinsClaim, BuiltinsInteractionClaim};
use super::components::indented_component_display;
use super::pedersen::air::{
    PedersenContextClaim, PedersenContextComponents, PedersenContextInteractionClaim,
};
use super::poseidon::air::{
    PoseidonContextClaim, PoseidonContextComponents, PoseidonContextInteractionClaim,
};
use super::range_checks_air::{
    RangeChecksClaim, RangeChecksComponents, RangeChecksInteractionClaim,
    RangeChecksInteractionElements,
};
use crate::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, call_opcode_abs, call_opcode_rel_imm,
    generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken, jump_opcode_abs,
    jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm, memory_address_to_id,
    memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode, ret_opcode,
    verify_bitwise_xor_4, verify_bitwise_xor_7, verify_bitwise_xor_8, verify_bitwise_xor_8_b,
    verify_bitwise_xor_9, verify_instruction,
};
pub use crate::public_data::{
    MemorySection, MemorySmallValue, PublicData, PublicMemory, PublicSegmentRanges, SegmentRange,
};
use crate::relations;
use crate::utils::{accumulate_relation_uses, RelationUsesDict};

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

        channel.mix_u64(add.is_some() as u64);
        add.iter().for_each(|add| add.mix_into(channel));

        channel.mix_u64(add_small.is_some() as u64);
        add_small
            .iter()
            .for_each(|add_small| add_small.mix_into(channel));

        channel.mix_u64(add_ap.is_some() as u64);
        add_ap.iter().for_each(|add_ap| add_ap.mix_into(channel));

        channel.mix_u64(assert_eq.is_some() as u64);
        assert_eq
            .iter()
            .for_each(|assert_eq| assert_eq.mix_into(channel));

        channel.mix_u64(assert_eq_imm.is_some() as u64);
        assert_eq_imm
            .iter()
            .for_each(|assert_eq_imm| assert_eq_imm.mix_into(channel));

        channel.mix_u64(assert_eq_double_deref.is_some() as u64);
        assert_eq_double_deref
            .iter()
            .for_each(|assert_eq_double_deref| assert_eq_double_deref.mix_into(channel));

        channel.mix_u64(blake.is_some() as u64);
        blake.iter().for_each(|blake| blake.mix_into(channel));

        channel.mix_u64(call.is_some() as u64);
        call.iter().for_each(|call| call.mix_into(channel));

        channel.mix_u64(call_rel_imm.is_some() as u64);
        call_rel_imm
            .iter()
            .for_each(|call_rel_imm| call_rel_imm.mix_into(channel));

        channel.mix_u64(generic.is_some() as u64);
        generic.iter().for_each(|generic| generic.mix_into(channel));

        channel.mix_u64(jnz.is_some() as u64);
        jnz.iter().for_each(|jnz| jnz.mix_into(channel));

        channel.mix_u64(jnz_taken.is_some() as u64);
        jnz_taken
            .iter()
            .for_each(|jnz_taken| jnz_taken.mix_into(channel));

        channel.mix_u64(jump.is_some() as u64);
        jump.iter().for_each(|jump| jump.mix_into(channel));

        channel.mix_u64(jump_double_deref.is_some() as u64);
        jump_double_deref
            .iter()
            .for_each(|jump_double_deref| jump_double_deref.mix_into(channel));

        channel.mix_u64(jump_rel.is_some() as u64);
        jump_rel
            .iter()
            .for_each(|jump_rel| jump_rel.mix_into(channel));

        channel.mix_u64(jump_rel_imm.is_some() as u64);
        jump_rel_imm
            .iter()
            .for_each(|jump_rel_imm| jump_rel_imm.mix_into(channel));

        channel.mix_u64(mul.is_some() as u64);
        mul.iter().for_each(|mul| mul.mix_into(channel));

        channel.mix_u64(mul_small.is_some() as u64);
        mul_small
            .iter()
            .for_each(|mul_small| mul_small.mix_into(channel));

        channel.mix_u64(qm31.is_some() as u64);
        qm31.iter().for_each(|qm31| qm31.mix_into(channel));

        channel.mix_u64(ret.is_some() as u64);
        ret.iter().for_each(|ret| ret.mix_into(channel));

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

        add.iter().for_each(|c| {
            accumulate_relation_uses(relation_uses, add_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        add_small.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_opcode_small::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        add_ap.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_ap_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        assert_eq.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        assert_eq_imm.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        assert_eq_double_deref.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_double_deref::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        blake.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_compress_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        call.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_abs::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        call_rel_imm.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_rel_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        generic.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                generic_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jnz.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_non_taken::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jnz_taken.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_taken::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_abs::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump_double_deref.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_double_deref::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump_rel.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump_rel_imm.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        mul.iter().for_each(|c| {
            accumulate_relation_uses(relation_uses, mul_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        mul_small.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                mul_opcode_small::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        qm31.iter().for_each(|c| {
            accumulate_relation_uses(
                relation_uses,
                qm_31_add_mul_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        ret.iter().for_each(|c| {
            accumulate_relation_uses(relation_uses, ret_opcode::RELATION_USES_PER_ROW, c.log_size)
        });

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

pub struct CairoInteractionElements {
    pub opcodes: relations::Opcodes,
    pub verify_instruction: relations::VerifyInstruction,
    pub blake_round: relations::BlakeRound,
    pub blake_g: relations::BlakeG,
    pub blake_sigma: relations::BlakeRoundSigma,
    pub triple_xor_32: relations::TripleXor32,
    pub partial_ec_mul: relations::PartialEcMul,
    pub poseidon_aggregator: relations::PoseidonAggregator,
    pub pedersen_points_table: relations::PedersenPointsTable,
    pub poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain,
    pub poseidon_full_round_chain: relations::PoseidonFullRoundChain,
    pub cube_252: relations::Cube252,
    pub poseidon_round_keys: relations::PoseidonRoundKeys,
    pub range_check_252_width_27: relations::RangeCheck252Width27,
    pub memory_address_to_id: relations::MemoryAddressToId,
    pub memory_id_to_value: relations::MemoryIdToBig,
    pub range_checks: RangeChecksInteractionElements,
    pub verify_bitwise_xor_4: relations::VerifyBitwiseXor_4,
    pub verify_bitwise_xor_7: relations::VerifyBitwiseXor_7,
    pub verify_bitwise_xor_8: relations::VerifyBitwiseXor_8,
    pub verify_bitwise_xor_8_b: relations::VerifyBitwiseXor_8_B,
    pub verify_bitwise_xor_9: relations::VerifyBitwiseXor_9,
    pub verify_bitwise_xor_12: relations::VerifyBitwiseXor_12,
}
impl CairoInteractionElements {
    pub fn draw(channel: &mut impl Channel) -> CairoInteractionElements {
        CairoInteractionElements {
            opcodes: relations::Opcodes::draw(channel),
            verify_instruction: relations::VerifyInstruction::draw(channel),
            blake_round: relations::BlakeRound::draw(channel),
            blake_g: relations::BlakeG::draw(channel),
            blake_sigma: relations::BlakeRoundSigma::draw(channel),
            triple_xor_32: relations::TripleXor32::draw(channel),
            poseidon_aggregator: relations::PoseidonAggregator::draw(channel),
            poseidon_3_partial_rounds_chain: relations::Poseidon3PartialRoundsChain::draw(channel),
            poseidon_full_round_chain: relations::PoseidonFullRoundChain::draw(channel),
            cube_252: relations::Cube252::draw(channel),
            poseidon_round_keys: relations::PoseidonRoundKeys::draw(channel),
            range_check_252_width_27: relations::RangeCheck252Width27::draw(channel),
            partial_ec_mul: relations::PartialEcMul::draw(channel),
            pedersen_points_table: relations::PedersenPointsTable::draw(channel),
            memory_address_to_id: relations::MemoryAddressToId::draw(channel),
            memory_id_to_value: relations::MemoryIdToBig::draw(channel),
            range_checks: RangeChecksInteractionElements::draw(channel),
            verify_bitwise_xor_4: relations::VerifyBitwiseXor_4::draw(channel),
            verify_bitwise_xor_7: relations::VerifyBitwiseXor_7::draw(channel),
            verify_bitwise_xor_8: relations::VerifyBitwiseXor_8::draw(channel),
            verify_bitwise_xor_8_b: relations::VerifyBitwiseXor_8_B::draw(channel),
            verify_bitwise_xor_9: relations::VerifyBitwiseXor_9::draw(channel),
            verify_bitwise_xor_12: relations::VerifyBitwiseXor_12::draw(channel),
        }
    }
}

#[derive(Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        self.add.iter().for_each(|c| c.mix_into(channel));
        self.add_small.iter().for_each(|c| c.mix_into(channel));
        self.add_ap.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_imm.iter().for_each(|c| c.mix_into(channel));
        self.assert_eq_double_deref
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.blake.iter().for_each(|c| c.mix_into(channel));
        self.call.iter().for_each(|c| c.mix_into(channel));
        self.call_rel_imm.iter().for_each(|c| c.mix_into(channel));
        self.generic.iter().for_each(|c| c.mix_into(channel));
        self.jnz.iter().for_each(|c| c.mix_into(channel));
        self.jnz_taken.iter().for_each(|c| c.mix_into(channel));
        self.jump.iter().for_each(|c| c.mix_into(channel));
        self.jump_double_deref
            .iter()
            .for_each(|c| c.mix_into(channel));
        self.jump_rel.iter().for_each(|c| c.mix_into(channel));
        self.jump_rel_imm.iter().for_each(|c| c.mix_into(channel));
        self.mul.iter().for_each(|c| c.mix_into(channel));
        self.mul_small.iter().for_each(|c| c.mix_into(channel));
        self.qm31.iter().for_each(|c| c.mix_into(channel));
        self.ret.iter().for_each(|c| c.mix_into(channel));
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
    interaction_claim
        .add
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .add_small
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .add_ap
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .assert_eq
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .assert_eq_imm
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .assert_eq_double_deref
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .blake
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .call
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .call_rel_imm
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .generic
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .jnz
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .jnz_taken
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .jump
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .jump_double_deref
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .jump_rel
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .jump_rel_imm
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .mul
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .mul_small
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .qm31
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
    interaction_claim
        .ret
        .as_ref()
        .inspect(|ic| sum += ic.claimed_sum);
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

pub struct CairoComponents {
    pub add: Option<add_opcode::Component>,
    pub add_small: Option<add_opcode_small::Component>,
    pub add_ap: Option<add_ap_opcode::Component>,
    pub assert_eq: Option<assert_eq_opcode::Component>,
    pub assert_eq_imm: Option<assert_eq_opcode_imm::Component>,
    pub assert_eq_double_deref: Option<assert_eq_opcode_double_deref::Component>,
    pub blake: Option<blake_compress_opcode::Component>,
    pub call: Option<call_opcode_abs::Component>,
    pub call_rel_imm: Option<call_opcode_rel_imm::Component>,
    pub generic: Option<generic_opcode::Component>,
    pub jnz: Option<jnz_opcode_non_taken::Component>,
    pub jnz_taken: Option<jnz_opcode_taken::Component>,
    pub jump: Option<jump_opcode_abs::Component>,
    pub jump_double_deref: Option<jump_opcode_double_deref::Component>,
    pub jump_rel: Option<jump_opcode_rel::Component>,
    pub jump_rel_imm: Option<jump_opcode_rel_imm::Component>,
    pub mul: Option<mul_opcode::Component>,
    pub mul_small: Option<mul_opcode_small::Component>,
    pub qm31: Option<qm_31_add_mul_opcode::Component>,
    pub ret: Option<ret_opcode::Component>,
    pub verify_instruction: verify_instruction::Component,
    pub blake_context: BlakeContextComponents,
    pub builtins: BuiltinComponents,
    pub pedersen_context: PedersenContextComponents,
    pub poseidon_context: PoseidonContextComponents,
    pub memory_address_to_id: memory_address_to_id::Component,
    pub memory_id_to_value: (
        Vec<memory_id_to_big::BigComponent>,
        memory_id_to_big::SmallComponent,
    ),
    pub range_checks: RangeChecksComponents,
    pub verify_bitwise_xor_4: verify_bitwise_xor_4::Component,
    pub verify_bitwise_xor_7: verify_bitwise_xor_7::Component,
    pub verify_bitwise_xor_8: verify_bitwise_xor_8::Component,
    pub verify_bitwise_xor_8_b: verify_bitwise_xor_8_b::Component,
    pub verify_bitwise_xor_9: verify_bitwise_xor_9::Component,
    // ...
}
impl CairoComponents {
    pub fn new(
        cairo_claim: &CairoClaim,
        interaction_elements: &CairoInteractionElements,
        interaction_claim: &CairoInteractionClaim,
        // Describes the structure of the preprocessed trace. Sensitive to order.
        preprocessed_column_ids: &[PreProcessedColumnId],
    ) -> Self {
        let tree_span_provider =
            &mut TraceLocationAllocator::new_with_preprocessed_columns(preprocessed_column_ids);

        let add = cairo_claim
            .add
            .zip(interaction_claim.add)
            .map(|(claim, interaction_claim)| {
                add_opcode::Component::new(
                    tree_span_provider,
                    add_opcode::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let add_small = cairo_claim.add_small.zip(interaction_claim.add_small).map(
            |(claim, interaction_claim)| {
                add_opcode_small::Component::new(
                    tree_span_provider,
                    add_opcode_small::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            },
        );
        let add_ap =
            cairo_claim
                .add_ap
                .zip(interaction_claim.add_ap)
                .map(|(claim, interaction_claim)| {
                    add_ap_opcode::Component::new(
                        tree_span_provider,
                        add_ap_opcode::Eval {
                            claim,
                            verify_instruction_lookup_elements: interaction_elements
                                .verify_instruction
                                .clone(),
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                            range_check_18_lookup_elements: interaction_elements
                                .range_checks
                                .rc_18
                                .clone(),
                            range_check_11_lookup_elements: interaction_elements
                                .range_checks
                                .rc_11
                                .clone(),
                        },
                        interaction_claim.claimed_sum,
                    )
                });
        let assert_eq = cairo_claim.assert_eq.zip(interaction_claim.assert_eq).map(
            |(claim, interaction_claim)| {
                assert_eq_opcode::Component::new(
                    tree_span_provider,
                    assert_eq_opcode::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            },
        );
        let assert_eq_imm = cairo_claim
            .assert_eq_imm
            .zip(interaction_claim.assert_eq_imm)
            .map(|(claim, interaction_claim)| {
                assert_eq_opcode_imm::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_imm::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let assert_eq_double_deref = cairo_claim
            .assert_eq_double_deref
            .zip(interaction_claim.assert_eq_double_deref)
            .map(|(claim, interaction_claim)| {
                assert_eq_opcode_double_deref::Component::new(
                    tree_span_provider,
                    assert_eq_opcode_double_deref::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let blake =
            cairo_claim
                .blake
                .zip(interaction_claim.blake)
                .map(|(claim, interaction_claim)| {
                    blake_compress_opcode::Component::new(
                        tree_span_provider,
                        blake_compress_opcode::Eval {
                            claim,
                            verify_instruction_lookup_elements: interaction_elements
                                .verify_instruction
                                .clone(),
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                            range_check_7_2_5_lookup_elements: interaction_elements
                                .range_checks
                                .rc_7_2_5
                                .clone(),
                            verify_bitwise_xor_8_lookup_elements: interaction_elements
                                .verify_bitwise_xor_8
                                .clone(),
                            blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                            triple_xor_32_lookup_elements: interaction_elements
                                .triple_xor_32
                                .clone(),
                            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        },
                        interaction_claim.claimed_sum,
                    )
                });
        let call =
            cairo_claim
                .call
                .zip(interaction_claim.call)
                .map(|(claim, interaction_claim)| {
                    call_opcode_abs::Component::new(
                        tree_span_provider,
                        call_opcode_abs::Eval {
                            claim,
                            verify_instruction_lookup_elements: interaction_elements
                                .verify_instruction
                                .clone(),
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        },
                        interaction_claim.claimed_sum,
                    )
                });
        let call_rel_imm = cairo_claim
            .call_rel_imm
            .zip(interaction_claim.call_rel_imm)
            .map(|(claim, interaction_claim)| {
                call_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    call_opcode_rel_imm::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let generic =
            cairo_claim
                .generic
                .zip(interaction_claim.generic)
                .map(|(claim, interaction_claim)| {
                    generic_opcode::Component::new(
                        tree_span_provider,
                        generic_opcode::Eval {
                            claim,
                            verify_instruction_lookup_elements: interaction_elements
                                .verify_instruction
                                .clone(),
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
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
                            range_check_20_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20
                                .clone(),
                            range_check_20_b_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_b
                                .clone(),
                            range_check_20_c_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_c
                                .clone(),
                            range_check_20_d_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_d
                                .clone(),
                            range_check_20_e_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_e
                                .clone(),
                            range_check_20_f_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_f
                                .clone(),
                            range_check_20_g_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_g
                                .clone(),
                            range_check_20_h_lookup_elements: interaction_elements
                                .range_checks
                                .rc_20_h
                                .clone(),
                            range_check_18_lookup_elements: interaction_elements
                                .range_checks
                                .rc_18
                                .clone(),
                            range_check_11_lookup_elements: interaction_elements
                                .range_checks
                                .rc_11
                                .clone(),
                            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        },
                        interaction_claim.claimed_sum,
                    )
                });
        let jnz = cairo_claim
            .jnz
            .zip(interaction_claim.jnz)
            .map(|(claim, interaction_claim)| {
                jnz_opcode_non_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_non_taken::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let jnz_taken = cairo_claim.jnz_taken.zip(interaction_claim.jnz_taken).map(
            |(claim, interaction_claim)| {
                jnz_opcode_taken::Component::new(
                    tree_span_provider,
                    jnz_opcode_taken::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            },
        );
        let jump =
            cairo_claim
                .jump
                .zip(interaction_claim.jump)
                .map(|(claim, interaction_claim)| {
                    jump_opcode_abs::Component::new(
                        tree_span_provider,
                        jump_opcode_abs::Eval {
                            claim,
                            verify_instruction_lookup_elements: interaction_elements
                                .verify_instruction
                                .clone(),
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        },
                        interaction_claim.claimed_sum,
                    )
                });
        let jump_double_deref = cairo_claim
            .jump_double_deref
            .zip(interaction_claim.jump_double_deref)
            .map(|(claim, interaction_claim)| {
                jump_opcode_double_deref::Component::new(
                    tree_span_provider,
                    jump_opcode_double_deref::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let jump_rel = cairo_claim.jump_rel.zip(interaction_claim.jump_rel).map(
            |(claim, interaction_claim)| {
                jump_opcode_rel::Component::new(
                    tree_span_provider,
                    jump_opcode_rel::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            },
        );
        let jump_rel_imm = cairo_claim
            .jump_rel_imm
            .zip(interaction_claim.jump_rel_imm)
            .map(|(claim, interaction_claim)| {
                jump_opcode_rel_imm::Component::new(
                    tree_span_provider,
                    jump_opcode_rel_imm::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let mul = cairo_claim
            .mul
            .zip(interaction_claim.mul)
            .map(|(claim, interaction_claim)| {
                mul_opcode::Component::new(
                    tree_span_provider,
                    mul_opcode::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_20_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20
                            .clone(),
                        range_check_20_b_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_b
                            .clone(),
                        range_check_20_c_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_c
                            .clone(),
                        range_check_20_d_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_d
                            .clone(),
                        range_check_20_e_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_e
                            .clone(),
                        range_check_20_f_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_f
                            .clone(),
                        range_check_20_g_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_g
                            .clone(),
                        range_check_20_h_lookup_elements: interaction_elements
                            .range_checks
                            .rc_20_h
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });
        let mul_small = cairo_claim.mul_small.zip(interaction_claim.mul_small).map(
            |(claim, interaction_claim)| {
                mul_opcode_small::Component::new(
                    tree_span_provider,
                    mul_opcode_small::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        range_check_11_lookup_elements: interaction_elements
                            .range_checks
                            .rc_11
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            },
        );
        let qm31 =
            cairo_claim
                .qm31
                .zip(interaction_claim.qm31)
                .map(|(claim, interaction_claim)| {
                    qm_31_add_mul_opcode::Component::new(
                        tree_span_provider,
                        qm_31_add_mul_opcode::Eval {
                            claim,
                            verify_instruction_lookup_elements: interaction_elements
                                .verify_instruction
                                .clone(),
                            memory_address_to_id_lookup_elements: interaction_elements
                                .memory_address_to_id
                                .clone(),
                            memory_id_to_big_lookup_elements: interaction_elements
                                .memory_id_to_value
                                .clone(),
                            range_check_4_4_4_4_lookup_elements: interaction_elements
                                .range_checks
                                .rc_4_4_4_4
                                .clone(),
                            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                        },
                        interaction_claim.claimed_sum,
                    )
                });
        let ret = cairo_claim
            .ret
            .zip(interaction_claim.ret)
            .map(|(claim, interaction_claim)| {
                ret_opcode::Component::new(
                    tree_span_provider,
                    ret_opcode::Eval {
                        claim,
                        verify_instruction_lookup_elements: interaction_elements
                            .verify_instruction
                            .clone(),
                        memory_address_to_id_lookup_elements: interaction_elements
                            .memory_address_to_id
                            .clone(),
                        memory_id_to_big_lookup_elements: interaction_elements
                            .memory_id_to_value
                            .clone(),
                        opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                    },
                    interaction_claim.claimed_sum,
                )
            });

        let verify_instruction_component = verify_instruction::Component::new(
            tree_span_provider,
            verify_instruction::Eval {
                claim: cairo_claim.verify_instruction,
                memory_address_to_id_lookup_elements: interaction_elements
                    .memory_address_to_id
                    .clone(),
                verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
                memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_4_3_lookup_elements: interaction_elements.range_checks.rc_4_3.clone(),
                range_check_7_2_5_lookup_elements: interaction_elements
                    .range_checks
                    .rc_7_2_5
                    .clone(),
            },
            interaction_claim.verify_instruction.claimed_sum,
        );

        let blake_context = BlakeContextComponents::new(
            tree_span_provider,
            &cairo_claim.blake_context,
            interaction_elements,
            &interaction_claim.blake_context,
        );
        let builtin_components = BuiltinComponents::new(
            tree_span_provider,
            &cairo_claim.builtins,
            interaction_elements,
            &interaction_claim.builtins,
        );
        let pedersen_context = PedersenContextComponents::new(
            tree_span_provider,
            &cairo_claim.pedersen_context,
            interaction_elements,
            &interaction_claim.pedersen_context,
        );
        let poseidon_context = PoseidonContextComponents::new(
            tree_span_provider,
            &cairo_claim.poseidon_context,
            interaction_elements,
            &interaction_claim.poseidon_context,
        );
        let memory_address_to_id_component = memory_address_to_id::Component::new(
            tree_span_provider,
            memory_address_to_id::Eval::new(
                cairo_claim.memory_address_to_id.clone(),
                interaction_elements.memory_address_to_id.clone(),
            ),
            interaction_claim.memory_address_to_id.clone().claimed_sum,
        );

        let memory_id_to_value_components = memory_id_to_big::big_components_from_claim(
            &cairo_claim.memory_id_to_value.big_log_sizes,
            &interaction_claim.memory_id_to_value.big_claimed_sums,
            &interaction_elements.memory_id_to_value,
            &interaction_elements.range_checks.rc_9_9,
            &interaction_elements.range_checks.rc_9_9_b,
            &interaction_elements.range_checks.rc_9_9_c,
            &interaction_elements.range_checks.rc_9_9_d,
            &interaction_elements.range_checks.rc_9_9_e,
            &interaction_elements.range_checks.rc_9_9_f,
            &interaction_elements.range_checks.rc_9_9_g,
            &interaction_elements.range_checks.rc_9_9_h,
            tree_span_provider,
        );
        let small_memory_id_to_value_component = memory_id_to_big::SmallComponent::new(
            tree_span_provider,
            memory_id_to_big::SmallEval::new(
                cairo_claim.memory_id_to_value.clone(),
                interaction_elements.memory_id_to_value.clone(),
                interaction_elements.range_checks.rc_9_9.clone(),
                interaction_elements.range_checks.rc_9_9_b.clone(),
                interaction_elements.range_checks.rc_9_9_c.clone(),
                interaction_elements.range_checks.rc_9_9_d.clone(),
            ),
            interaction_claim
                .memory_id_to_value
                .clone()
                .small_claimed_sum,
        );
        let range_checks_component = RangeChecksComponents::new(
            tree_span_provider,
            &interaction_elements.range_checks,
            &interaction_claim.range_checks,
        );
        let verify_bitwise_xor_4_component = verify_bitwise_xor_4::Component::new(
            tree_span_provider,
            verify_bitwise_xor_4::Eval {
                claim: cairo_claim.verify_bitwise_xor_4,
                verify_bitwise_xor_4_lookup_elements: interaction_elements
                    .verify_bitwise_xor_4
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_4.claimed_sum,
        );
        let verify_bitwise_xor_7_component = verify_bitwise_xor_7::Component::new(
            tree_span_provider,
            verify_bitwise_xor_7::Eval {
                claim: cairo_claim.verify_bitwise_xor_7,
                verify_bitwise_xor_7_lookup_elements: interaction_elements
                    .verify_bitwise_xor_7
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_7.claimed_sum,
        );
        let verify_bitwise_xor_8_component = verify_bitwise_xor_8::Component::new(
            tree_span_provider,
            verify_bitwise_xor_8::Eval {
                claim: cairo_claim.verify_bitwise_xor_8,
                verify_bitwise_xor_8_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_8.claimed_sum,
        );
        let verify_bitwise_xor_8_b_component = verify_bitwise_xor_8_b::Component::new(
            tree_span_provider,
            verify_bitwise_xor_8_b::Eval {
                claim: cairo_claim.verify_bitwise_xor_8_b,
                verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                    .verify_bitwise_xor_8_b
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_8_b.claimed_sum,
        );
        let verify_bitwise_xor_9_component = verify_bitwise_xor_9::Component::new(
            tree_span_provider,
            verify_bitwise_xor_9::Eval {
                claim: cairo_claim.verify_bitwise_xor_9,
                verify_bitwise_xor_9_lookup_elements: interaction_elements
                    .verify_bitwise_xor_9
                    .clone(),
            },
            interaction_claim.verify_bitwise_xor_9.claimed_sum,
        );
        Self {
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
            verify_instruction: verify_instruction_component,
            blake_context,
            builtins: builtin_components,
            pedersen_context,
            poseidon_context,
            memory_address_to_id: memory_address_to_id_component,
            memory_id_to_value: (
                memory_id_to_value_components,
                small_memory_id_to_value_component,
            ),
            range_checks: range_checks_component,
            verify_bitwise_xor_4: verify_bitwise_xor_4_component,
            verify_bitwise_xor_7: verify_bitwise_xor_7_component,
            verify_bitwise_xor_8: verify_bitwise_xor_8_component,
            verify_bitwise_xor_8_b: verify_bitwise_xor_8_b_component,
            verify_bitwise_xor_9: verify_bitwise_xor_9_component,
        }
    }

    pub fn provers(&self) -> Vec<&dyn ComponentProver<SimdBackend>> {
        chain!(
            self.add
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.add_small
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.add_ap
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.assert_eq
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.assert_eq_imm
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.assert_eq_double_deref
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.blake
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.call
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.call_rel_imm
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.generic
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jnz
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jnz_taken
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump_double_deref
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump_rel
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.jump_rel_imm
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.mul
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.mul_small
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.qm31
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.ret
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            [&self.verify_instruction as &dyn ComponentProver<SimdBackend>,],
            self.blake_context.provers(),
            self.builtins.provers(),
            self.pedersen_context.provers(),
            self.poseidon_context.provers(),
            [&self.memory_address_to_id as &dyn ComponentProver<SimdBackend>,],
            self.memory_id_to_value
                .0
                .iter()
                .map(|component| component as &dyn ComponentProver<SimdBackend>),
            [&self.memory_id_to_value.1 as &dyn ComponentProver<SimdBackend>,],
            self.range_checks.provers(),
            [
                &self.verify_bitwise_xor_4 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_7 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_8 as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_8_b as &dyn ComponentProver<SimdBackend>,
                &self.verify_bitwise_xor_9 as &dyn ComponentProver<SimdBackend>,
            ]
        )
        .collect()
    }

    pub fn components(&self) -> Vec<&dyn Component> {
        self.provers()
            .into_iter()
            .map(|component| component as &dyn Component)
            .collect()
    }
}

impl std::fmt::Display for CairoComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CairoComponents")?;
        writeln!(
            f,
            "AddOpcode: {}",
            self.add.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "AddOpcodeSmall: {}",
            self.add_small
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "AddApOpcode: {}",
            self.add_ap
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "AssertEqOpcode: {}",
            self.assert_eq
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "AssertEqOpcodeImm: {}",
            self.assert_eq_imm
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "AssertEqOpcodeDoubleDeref: {}",
            self.assert_eq_double_deref
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "BlakeCompressOpcode: {}",
            self.blake.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "CallOpcodeAbs: {}",
            self.call.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "CallOpcodeRelImm: {}",
            self.call_rel_imm
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "GenericOpcode: {}",
            self.generic
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "JnzOpcodeNonTaken: {}",
            self.jnz.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "JnzOpcodeTaken: {}",
            self.jnz_taken
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "JumpOpcodeAbs: {}",
            self.jump.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "JumpOpcodeDoubleDeref: {}",
            self.jump_double_deref
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "JumpOpcodeRel: {}",
            self.jump_rel
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "JumpOpcodeRelImm: {}",
            self.jump_rel_imm
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "MulOpcode: {}",
            self.mul.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "MulOpcodeSmall: {}",
            self.mul_small
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "Qm31AddMulOpcode: {}",
            self.qm31.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "RetOpcode: {}",
            self.ret.iter().map(indented_component_display).join("\n")
        )?;
        writeln!(
            f,
            "VerifyInstruction: {}",
            indented_component_display(&self.verify_instruction)
        )?;
        writeln!(f, "BlakeContext: {}", self.blake_context)?;
        writeln!(f, "Builtins: {}", self.builtins)?;
        writeln!(f, "PedersenContext: {}", self.pedersen_context)?;
        writeln!(f, "PoseidonContext: {}", self.poseidon_context)?;
        writeln!(
            f,
            "MemoryAddressToId: {}",
            indented_component_display(&self.memory_address_to_id)
        )?;
        for component in &self.memory_id_to_value.0 {
            writeln!(
                f,
                "MemoryIdToValue: {}",
                indented_component_display(component)
            )?;
        }
        writeln!(
            f,
            "SmallMemoryIdToValue: {}",
            indented_component_display(&self.memory_id_to_value.1)
        )?;
        writeln!(f, "RangeChecks: {}", self.range_checks)?;
        writeln!(
            f,
            "VerifyBitwiseXor4: {}",
            indented_component_display(&self.verify_bitwise_xor_4)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor7: {}",
            indented_component_display(&self.verify_bitwise_xor_7)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor8: {}",
            indented_component_display(&self.verify_bitwise_xor_8)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor8B: {}",
            indented_component_display(&self.verify_bitwise_xor_8_b)
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor9: {}",
            indented_component_display(&self.verify_bitwise_xor_9)
        )?;
        Ok(())
    }
}

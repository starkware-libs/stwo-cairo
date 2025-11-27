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
};
use crate::cairo_interaction_elements::CairoInteractionElements;
use crate::components::{
    add_ap_opcode, add_opcode, add_opcode_small, assert_eq_opcode, assert_eq_opcode_double_deref,
    assert_eq_opcode_imm, blake_compress_opcode, blake_g, blake_round, blake_round_sigma,
    call_opcode_abs, call_opcode_rel_imm, generic_opcode, jnz_opcode_non_taken, jnz_opcode_taken,
    jump_opcode_abs, jump_opcode_double_deref, jump_opcode_rel, jump_opcode_rel_imm,
    memory_address_to_id, memory_id_to_big, mul_opcode, mul_opcode_small, qm_31_add_mul_opcode,
    ret_opcode, triple_xor_32, verify_bitwise_xor_12, verify_bitwise_xor_4, verify_bitwise_xor_7,
    verify_bitwise_xor_8, verify_bitwise_xor_8_b, verify_bitwise_xor_9, verify_instruction,
};
pub use crate::public_data::{
    MemorySection, MemorySmallValue, PublicData, PublicMemory, PublicSegmentRanges, SegmentRange,
};
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
    pub blake_round: Option<blake_round::Claim>,
    pub blake_g: Option<blake_g::Claim>,
    pub blake_sigma: Option<blake_round_sigma::Claim>,
    pub triple_xor_32: Option<triple_xor_32::Claim>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::Claim>,
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
            blake_round,
            blake_g,
            blake_sigma,
            triple_xor_32,
            verify_bitwise_xor_12,
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
        add.inspect(|c| c.mix_into(channel));

        channel.mix_u64(add_small.is_some() as u64);
        add_small.inspect(|c| c.mix_into(channel));

        channel.mix_u64(add_ap.is_some() as u64);
        add_ap.inspect(|c| c.mix_into(channel));

        channel.mix_u64(assert_eq.is_some() as u64);
        assert_eq.inspect(|c| c.mix_into(channel));

        channel.mix_u64(assert_eq_imm.is_some() as u64);
        assert_eq_imm.inspect(|c| c.mix_into(channel));

        channel.mix_u64(assert_eq_double_deref.is_some() as u64);
        assert_eq_double_deref.inspect(|c| c.mix_into(channel));

        channel.mix_u64(blake.is_some() as u64);
        blake.inspect(|c| c.mix_into(channel));

        channel.mix_u64(call.is_some() as u64);
        call.inspect(|c| c.mix_into(channel));

        channel.mix_u64(call_rel_imm.is_some() as u64);
        call_rel_imm.inspect(|c| c.mix_into(channel));

        channel.mix_u64(generic.is_some() as u64);
        generic.inspect(|c| c.mix_into(channel));

        channel.mix_u64(jnz.is_some() as u64);
        jnz.inspect(|c| c.mix_into(channel));

        channel.mix_u64(jnz_taken.is_some() as u64);
        jnz_taken.inspect(|c| c.mix_into(channel));

        channel.mix_u64(jump.is_some() as u64);
        jump.inspect(|c| c.mix_into(channel));

        channel.mix_u64(jump_double_deref.is_some() as u64);
        jump_double_deref.inspect(|c| c.mix_into(channel));

        channel.mix_u64(jump_rel.is_some() as u64);
        jump_rel.inspect(|c| c.mix_into(channel));

        channel.mix_u64(jump_rel_imm.is_some() as u64);
        jump_rel_imm.inspect(|c| c.mix_into(channel));

        channel.mix_u64(mul.is_some() as u64);
        mul.inspect(|c| c.mix_into(channel));

        channel.mix_u64(mul_small.is_some() as u64);
        mul_small.inspect(|c| c.mix_into(channel));

        channel.mix_u64(qm31.is_some() as u64);
        qm31.inspect(|c| c.mix_into(channel));

        channel.mix_u64(ret.is_some() as u64);
        ret.inspect(|c| c.mix_into(channel));

        verify_instruction.mix_into(channel);
        blake_round.inspect(|c| c.mix_into(channel));
        blake_g.inspect(|c| c.mix_into(channel));
        blake_sigma.inspect(|c| c.mix_into(channel));
        triple_xor_32.inspect(|c| c.mix_into(channel));
        verify_bitwise_xor_12.inspect(|c| c.mix_into(channel));

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
        let mut log_sizes_list = vec![];
        self.add.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.add_small
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.add_ap.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.assert_eq
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.assert_eq_imm
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.assert_eq_double_deref
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.call.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.call_rel_imm
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.generic.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jnz.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jnz_taken
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_double_deref
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_rel
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.jump_rel_imm
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.mul.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.mul_small
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.qm31.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.ret.inspect(|c| log_sizes_list.push(c.log_sizes()));
        log_sizes_list.push(self.verify_instruction.log_sizes());
        self.blake_round
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake_g.inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.blake_sigma
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.triple_xor_32
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        self.verify_bitwise_xor_12
            .inspect(|c| log_sizes_list.push(c.log_sizes()));
        log_sizes_list.push(self.builtins.log_sizes());
        log_sizes_list.push(self.pedersen_context.log_sizes());
        log_sizes_list.push(self.poseidon_context.log_sizes());
        log_sizes_list.push(self.memory_address_to_id.log_sizes());
        log_sizes_list.push(self.memory_id_to_value.log_sizes());
        log_sizes_list.push(self.range_checks.log_sizes());
        log_sizes_list.push(self.verify_bitwise_xor_4.log_sizes());
        log_sizes_list.push(self.verify_bitwise_xor_7.log_sizes());
        log_sizes_list.push(self.verify_bitwise_xor_8.log_sizes());
        log_sizes_list.push(self.verify_bitwise_xor_8_b.log_sizes());
        log_sizes_list.push(self.verify_bitwise_xor_9.log_sizes());
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
            blake_round,
            blake_g,
            blake_sigma: _,
            triple_xor_32,
            verify_bitwise_xor_12: _,
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

        add.inspect(|c| {
            accumulate_relation_uses(relation_uses, add_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        add_small.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_opcode_small::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        add_ap.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                add_ap_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        assert_eq.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        assert_eq_imm.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        assert_eq_double_deref.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                assert_eq_opcode_double_deref::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        blake.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_compress_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        call.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_abs::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        call_rel_imm.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                call_opcode_rel_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        generic.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                generic_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jnz.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_non_taken::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jnz_taken.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jnz_opcode_taken::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_abs::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump_double_deref.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_double_deref::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump_rel.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        jump_rel_imm.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                jump_opcode_rel_imm::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        mul.inspect(|c| {
            accumulate_relation_uses(relation_uses, mul_opcode::RELATION_USES_PER_ROW, c.log_size)
        });
        mul_small.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                mul_opcode_small::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        qm31.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                qm_31_add_mul_opcode::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        ret.inspect(|c| {
            accumulate_relation_uses(relation_uses, ret_opcode::RELATION_USES_PER_ROW, c.log_size)
        });

        builtins.accumulate_relation_uses(relation_uses);

        blake_round.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                blake_round::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });
        blake_g.inspect(|c| {
            accumulate_relation_uses(relation_uses, blake_g::RELATION_USES_PER_ROW, c.log_size)
        });
        triple_xor_32.inspect(|c| {
            accumulate_relation_uses(
                relation_uses,
                triple_xor_32::RELATION_USES_PER_ROW,
                c.log_size,
            )
        });

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
    pub blake_round: Option<blake_round::InteractionClaim>,
    pub blake_g: Option<blake_g::InteractionClaim>,
    pub blake_sigma: Option<blake_round_sigma::InteractionClaim>,
    pub triple_xor_32: Option<triple_xor_32::InteractionClaim>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::InteractionClaim>,
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
        self.add.inspect(|c| c.mix_into(channel));
        self.add_small.inspect(|c| c.mix_into(channel));
        self.add_ap.inspect(|c| c.mix_into(channel));
        self.assert_eq.inspect(|c| c.mix_into(channel));
        self.assert_eq_imm.inspect(|c| c.mix_into(channel));
        self.assert_eq_double_deref.inspect(|c| c.mix_into(channel));
        self.blake.inspect(|c| c.mix_into(channel));
        self.call.inspect(|c| c.mix_into(channel));
        self.call_rel_imm.inspect(|c| c.mix_into(channel));
        self.generic.inspect(|c| c.mix_into(channel));
        self.jnz.inspect(|c| c.mix_into(channel));
        self.jnz_taken.inspect(|c| c.mix_into(channel));
        self.jump.inspect(|c| c.mix_into(channel));
        self.jump_double_deref.inspect(|c| c.mix_into(channel));
        self.jump_rel.inspect(|c| c.mix_into(channel));
        self.jump_rel_imm.inspect(|c| c.mix_into(channel));
        self.mul.inspect(|c| c.mix_into(channel));
        self.mul_small.inspect(|c| c.mix_into(channel));
        self.qm31.inspect(|c| c.mix_into(channel));
        self.ret.inspect(|c| c.mix_into(channel));
        self.verify_instruction.mix_into(channel);
        self.blake_round.as_ref().inspect(|c| c.mix_into(channel));
        self.blake_g.inspect(|c| c.mix_into(channel));
        self.blake_sigma.inspect(|c| c.mix_into(channel));
        self.triple_xor_32.inspect(|c| c.mix_into(channel));
        self.verify_bitwise_xor_12
            .as_ref()
            .inspect(|c| c.mix_into(channel));
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
    interaction_claim.add.inspect(|ic| {
        assert!(
            claim.add.is_some(),
            "add interaction claim is present but add claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.add_small.inspect(|ic| {
        assert!(
            claim.add_small.is_some(),
            "add_small interaction claim is present but add_small claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.add_ap.inspect(|ic| {
        assert!(
            claim.add_ap.is_some(),
            "add_ap interaction claim is present but add_ap claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.assert_eq.inspect(|ic| {
        assert!(
            claim.assert_eq.is_some(),
            "assert_eq interaction claim is present but assert_eq claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.assert_eq_imm.inspect(|ic| {
        assert!(
            claim.assert_eq_imm.is_some(),
            "assert_eq_imm interaction claim is present but assert_eq_imm claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim
        .assert_eq_double_deref
        .inspect(|ic| {
            assert!(claim.assert_eq_double_deref.is_some(), "assert_eq_double_deref interaction claim is present but assert_eq_double_deref claim isn't");
            sum += ic.claimed_sum;
        });
    interaction_claim.blake.inspect(|ic| {
        assert!(
            claim.blake.is_some(),
            "blake interaction claim is present but blake claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.call.inspect(|ic| {
        assert!(
            claim.call.is_some(),
            "call interaction claim is present but call claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.call_rel_imm.inspect(|ic| {
        assert!(
            claim.call_rel_imm.is_some(),
            "call_rel_imm interaction claim is present but call_rel_imm claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.generic.inspect(|ic| {
        assert!(
            claim.generic.is_some(),
            "generic interaction claim is present but generic claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.jnz.inspect(|ic| {
        assert!(
            claim.jnz.is_some(),
            "jnz interaction claim is present but jnz claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.jnz_taken.inspect(|ic| {
        assert!(
            claim.jnz_taken.is_some(),
            "jnz_taken interaction claim is present but jnz_taken claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.jump.inspect(|ic| {
        assert!(
            claim.jump.is_some(),
            "jump interaction claim is present but jump claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.jump_double_deref.inspect(|ic| {
        assert!(
            claim.jump_double_deref.is_some(),
            "jump_double_deref interaction claim is present but jump_double_deref claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.jump_rel.inspect(|ic| {
        assert!(
            claim.jump_rel.is_some(),
            "jump_rel interaction claim is present but jump_rel claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.jump_rel_imm.inspect(|ic| {
        assert!(
            claim.jump_rel_imm.is_some(),
            "jump_rel_imm interaction claim is present but jump_rel_imm claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.mul.inspect(|ic| {
        assert!(
            claim.mul.is_some(),
            "mul interaction claim is present but mul claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.mul_small.inspect(|ic| {
        assert!(
            claim.mul_small.is_some(),
            "mul_small interaction claim is present but mul_small claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.qm31.inspect(|ic| {
        assert!(
            claim.qm31.is_some(),
            "qm31 interaction claim is present but qm31 claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.ret.inspect(|ic| {
        assert!(
            claim.ret.is_some(),
            "ret interaction claim is present but ret claim isn't"
        );
        sum += ic.claimed_sum;
    });
    sum += interaction_claim.verify_instruction.claimed_sum;
    interaction_claim.blake_round.inspect(|ic| {
        assert!(
            claim.blake_round.is_some(),
            "blake_round interaction claim is present but blake_round claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_g.inspect(|ic| {
        assert!(
            claim.blake_g.is_some(),
            "blake_g interaction claim is present but blake_g claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.blake_sigma.inspect(|ic| {
        assert!(
            claim.blake_sigma.is_some(),
            "blake_sigma interaction claim is present but blake_sigma claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim.triple_xor_32.inspect(|ic| {
        assert!(
            claim.triple_xor_32.is_some(),
            "triple_xor_32 interaction claim is present but triple_xor_32 claim isn't"
        );
        sum += ic.claimed_sum;
    });
    interaction_claim
        .verify_bitwise_xor_12
        .inspect(|ic| {
            assert!(claim.verify_bitwise_xor_12.is_some(), "verify_bitwise_xor_12 interaction claim is present but verify_bitwise_xor_12 claim isn't");
            sum += ic.claimed_sum;
        });
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
    pub blake_round: Option<blake_round::Component>,
    pub blake_g: Option<blake_g::Component>,
    pub blake_sigma: Option<blake_round_sigma::Component>,
    pub triple_xor_32: Option<triple_xor_32::Component>,
    pub verify_bitwise_xor_12: Option<verify_bitwise_xor_12::Component>,
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

        let add = cairo_claim.add.map(|claim| {
            let interaction_claim = interaction_claim.add.unwrap();
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
        let add_small = cairo_claim.add_small.map(|claim| {
            let interaction_claim = interaction_claim.add_small.unwrap();
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
        });
        let add_ap = cairo_claim.add_ap.map(|claim| {
            let interaction_claim = interaction_claim.add_ap.unwrap();
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
                    range_check_18_lookup_elements: interaction_elements.range_checks.rc_18.clone(),
                    range_check_11_lookup_elements: interaction_elements.range_checks.rc_11.clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let assert_eq = cairo_claim.assert_eq.map(|claim| {
            let interaction_claim = interaction_claim.assert_eq.unwrap();
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
        });
        let assert_eq_imm = cairo_claim.assert_eq_imm.map(|claim| {
            let interaction_claim = interaction_claim.assert_eq_imm.unwrap();
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
        let assert_eq_double_deref = cairo_claim.assert_eq_double_deref.map(|claim| {
            let interaction_claim = interaction_claim.assert_eq_double_deref.unwrap();
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
        let blake = cairo_claim.blake.map(|claim| {
            let interaction_claim = interaction_claim.blake.unwrap();
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
                    triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                    opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let call = cairo_claim.call.map(|claim| {
            let interaction_claim = interaction_claim.call.unwrap();
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
        let call_rel_imm = cairo_claim.call_rel_imm.map(|claim| {
            let interaction_claim = interaction_claim.call_rel_imm.unwrap();
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
        let generic = cairo_claim.generic.map(|claim| {
            let interaction_claim = interaction_claim.generic.unwrap();
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
                    range_check_20_lookup_elements: interaction_elements.range_checks.rc_20.clone(),
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
                    range_check_18_lookup_elements: interaction_elements.range_checks.rc_18.clone(),
                    range_check_11_lookup_elements: interaction_elements.range_checks.rc_11.clone(),
                    opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let jnz = cairo_claim.jnz.map(|claim| {
            let interaction_claim = interaction_claim.jnz.unwrap();
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
        let jnz_taken = cairo_claim.jnz_taken.map(|claim| {
            let interaction_claim = interaction_claim.jnz_taken.unwrap();
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
        });
        let jump = cairo_claim.jump.map(|claim| {
            let interaction_claim = interaction_claim.jump.unwrap();
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
        let jump_double_deref = cairo_claim.jump_double_deref.map(|claim| {
            let interaction_claim = interaction_claim.jump_double_deref.unwrap();
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
        let jump_rel = cairo_claim.jump_rel.map(|claim| {
            let interaction_claim = interaction_claim.jump_rel.unwrap();
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
        });
        let jump_rel_imm = cairo_claim.jump_rel_imm.map(|claim| {
            let interaction_claim = interaction_claim.jump_rel_imm.unwrap();
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
        let mul = cairo_claim.mul.map(|claim| {
            let interaction_claim = interaction_claim.mul.unwrap();
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
                    range_check_20_lookup_elements: interaction_elements.range_checks.rc_20.clone(),
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
        let mul_small = cairo_claim.mul_small.map(|claim| {
            let interaction_claim = interaction_claim.mul_small.unwrap();
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
                    range_check_11_lookup_elements: interaction_elements.range_checks.rc_11.clone(),
                    opcodes_lookup_elements: interaction_elements.opcodes.clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let qm31 = cairo_claim.qm31.map(|claim| {
            let interaction_claim = interaction_claim.qm31.unwrap();
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
        let ret = cairo_claim.ret.map(|claim| {
            let interaction_claim = interaction_claim.ret.unwrap();
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

        let blake_round_component = cairo_claim.blake_round.map(|claim| {
            let interaction_claim = interaction_claim.blake_round.unwrap();
            blake_round::Component::new(
                tree_span_provider,
                blake_round::Eval {
                    claim,
                    blake_g_lookup_elements: interaction_elements.blake_g.clone(),
                    blake_round_lookup_elements: interaction_elements.blake_round.clone(),
                    blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
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
                },
                interaction_claim.claimed_sum,
            )
        });
        let blake_g_component = cairo_claim.blake_g.map(|claim| {
            let interaction_claim = interaction_claim.blake_g.unwrap();
            blake_g::Component::new(
                tree_span_provider,
                blake_g::Eval {
                    claim,
                    blake_g_lookup_elements: interaction_elements.blake_g.clone(),
                    verify_bitwise_xor_12_lookup_elements: interaction_elements
                        .verify_bitwise_xor_12
                        .clone(),
                    verify_bitwise_xor_4_lookup_elements: interaction_elements
                        .verify_bitwise_xor_4
                        .clone(),
                    verify_bitwise_xor_7_lookup_elements: interaction_elements
                        .verify_bitwise_xor_7
                        .clone(),
                    verify_bitwise_xor_8_lookup_elements: interaction_elements
                        .verify_bitwise_xor_8
                        .clone(),
                    verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                        .verify_bitwise_xor_8_b
                        .clone(),
                    verify_bitwise_xor_9_lookup_elements: interaction_elements
                        .verify_bitwise_xor_9
                        .clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let blake_sigma_component = cairo_claim.blake_sigma.map(|claim| {
            let interaction_claim = interaction_claim.blake_sigma.unwrap();
            blake_round_sigma::Component::new(
                tree_span_provider,
                blake_round_sigma::Eval {
                    claim,
                    blake_round_sigma_lookup_elements: interaction_elements.blake_sigma.clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let triple_xor_32_component = cairo_claim.triple_xor_32.map(|claim| {
            let interaction_claim = interaction_claim.triple_xor_32.unwrap();
            triple_xor_32::Component::new(
                tree_span_provider,
                triple_xor_32::Eval {
                    claim,
                    triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
                    verify_bitwise_xor_8_lookup_elements: interaction_elements
                        .verify_bitwise_xor_8
                        .clone(),
                    verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                        .verify_bitwise_xor_8_b
                        .clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
        let verify_bitwise_xor_12_component = cairo_claim.verify_bitwise_xor_12.map(|claim| {
            let interaction_claim = interaction_claim.verify_bitwise_xor_12.unwrap();
            verify_bitwise_xor_12::Component::new(
                tree_span_provider,
                verify_bitwise_xor_12::Eval {
                    claim,
                    verify_bitwise_xor_12_lookup_elements: interaction_elements
                        .verify_bitwise_xor_12
                        .clone(),
                },
                interaction_claim.claimed_sum,
            )
        });
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
            blake_round: blake_round_component,
            blake_g: blake_g_component,
            blake_sigma: blake_sigma_component,
            triple_xor_32: triple_xor_32_component,
            verify_bitwise_xor_12: verify_bitwise_xor_12_component,
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
            self.blake_round
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.blake_g
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.blake_sigma
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.triple_xor_32
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
            self.verify_bitwise_xor_12
                .iter()
                .map(|c| c as &dyn ComponentProver<SimdBackend>),
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
        writeln!(
            f,
            "BlakeRound: {}",
            self.blake_round
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "BlakeG: {}",
            self.blake_g
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "BlakeSigma: {}",
            self.blake_sigma
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "TripleXor32: {}",
            self.triple_xor_32
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
        writeln!(
            f,
            "VerifyBitwiseXor12: {}",
            self.verify_bitwise_xor_12
                .iter()
                .map(indented_component_display)
                .join("\n")
        )?;
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

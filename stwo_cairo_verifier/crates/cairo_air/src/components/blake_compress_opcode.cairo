use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(171, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 37, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.claimed_sum].span());
    }
}

#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub opcodes_lookup_elements: crate::OpcodeElements,
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck7Bit2Bit5BitElements,
    pub triple_xor32_lookup_elements: crate::TripleXor32Elements,
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor8BitElements,
    pub blake_round_lookup_elements: crate::BlakeRoundElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *self.claim.log_size;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *self.claim.log_size + 1
    }

    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *self.claim.log_size;
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();

        let VerifyInstruction_z = *self.verify_instruction_lookup_elements.z;
        let mut verify_instruction_alpha_powers = self
            .verify_instruction_lookup_elements
            .alpha_powers
            .span();
        let VerifyInstruction_alpha0 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha1 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha2 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha3 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha4 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha5 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha6 = *verify_instruction_alpha_powers.pop_front().unwrap();

        let MemoryAddressToId_z = *self.memory_address_to_id_lookup_elements.z;
        let mut memory_address_to_id_alpha_powers = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let MemoryAddressToId_alpha0 = *memory_address_to_id_alpha_powers.pop_front().unwrap();
        let MemoryAddressToId_alpha1 = *memory_address_to_id_alpha_powers.pop_front().unwrap();

        let Opcodes_z = *self.opcodes_lookup_elements.z;
        let mut opcodes_alpha_powers = self.opcodes_lookup_elements.alpha_powers.span();
        let Opcodes_alpha0 = *opcodes_alpha_powers.pop_front().unwrap();
        let Opcodes_alpha1 = *opcodes_alpha_powers.pop_front().unwrap();
        let Opcodes_alpha2 = *opcodes_alpha_powers.pop_front().unwrap();

        let BlakeRound_z = *self.blake_round_lookup_elements.z;
        let mut blake_round_alpha_powers = self.blake_round_lookup_elements.alpha_powers.span();
        let BlakeRound_alpha0 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha1 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha2 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha3 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha4 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha5 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha6 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha7 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha8 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha9 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha10 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha11 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha12 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha13 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha14 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha15 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha16 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha17 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha18 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha19 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha20 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha21 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha22 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha23 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha24 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha25 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha26 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha27 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha28 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha29 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha30 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha31 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha32 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha33 = *blake_round_alpha_powers.pop_front().unwrap();
        let BlakeRound_alpha34 = *blake_round_alpha_powers.pop_front().unwrap();

        let MemoryIdToBig_z = *self.memory_id_to_big_lookup_elements.z;
        let mut memory_id_to_big_alpha_powers = self
            .memory_id_to_big_lookup_elements
            .alpha_powers
            .span();
        let MemoryIdToBig_alpha0 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha1 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha2 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha3 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha4 = *memory_id_to_big_alpha_powers.pop_front().unwrap();

        let RangeCheck_7_2_5_z = *self.range_check_7_2_5_lookup_elements.z;
        let mut range_check_7_2_5_alpha_powers = self
            .range_check_7_2_5_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_7_2_5_alpha0 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let RangeCheck_7_2_5_alpha1 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let RangeCheck_7_2_5_alpha2 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();

        let TripleXor32_z = *self.triple_xor32_lookup_elements.z;
        let mut triple_xor32_alpha_powers = self.triple_xor32_lookup_elements.alpha_powers.span();
        let TripleXor32_alpha0 = *triple_xor32_alpha_powers.pop_front().unwrap();
        let TripleXor32_alpha1 = *triple_xor32_alpha_powers.pop_front().unwrap();
        let TripleXor32_alpha2 = *triple_xor32_alpha_powers.pop_front().unwrap();

        let VerifyBitwiseXor_8_z = *self.verify_bitwise_xor_8_lookup_elements.z;
        let mut verify_bitwise_xor_8_alpha_powers = self
            .verify_bitwise_xor_8_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_8_alpha0 = *verify_bitwise_xor_8_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_8_alpha1 = *verify_bitwise_xor_8_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_8_alpha2 = *verify_bitwise_xor_8_alpha_powers.pop_front().unwrap();

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: m31(pow2(log_size)),
            VerifyInstruction_z,
            VerifyInstruction_alpha0,
            VerifyInstruction_alpha1,
            VerifyInstruction_alpha2,
            VerifyInstruction_alpha3,
            VerifyInstruction_alpha4,
            VerifyInstruction_alpha5,
            VerifyInstruction_alpha6,
            MemoryAddressToId_z,
            MemoryAddressToId_alpha0,
            MemoryAddressToId_alpha1,
            Opcodes_z,
            Opcodes_alpha0,
            Opcodes_alpha1,
            Opcodes_alpha2,
            BlakeRound_z,
            BlakeRound_alpha0,
            BlakeRound_alpha1,
            BlakeRound_alpha2,
            BlakeRound_alpha3,
            BlakeRound_alpha4,
            BlakeRound_alpha5,
            BlakeRound_alpha6,
            BlakeRound_alpha7,
            BlakeRound_alpha8,
            BlakeRound_alpha9,
            BlakeRound_alpha10,
            BlakeRound_alpha11,
            BlakeRound_alpha12,
            BlakeRound_alpha13,
            BlakeRound_alpha14,
            BlakeRound_alpha15,
            BlakeRound_alpha16,
            BlakeRound_alpha17,
            BlakeRound_alpha18,
            BlakeRound_alpha19,
            BlakeRound_alpha20,
            BlakeRound_alpha21,
            BlakeRound_alpha22,
            BlakeRound_alpha23,
            BlakeRound_alpha24,
            BlakeRound_alpha25,
            BlakeRound_alpha26,
            BlakeRound_alpha27,
            BlakeRound_alpha28,
            BlakeRound_alpha29,
            BlakeRound_alpha30,
            BlakeRound_alpha31,
            BlakeRound_alpha32,
            BlakeRound_alpha33,
            BlakeRound_alpha34,
            MemoryIdToBig_z,
            MemoryIdToBig_alpha0,
            MemoryIdToBig_alpha1,
            MemoryIdToBig_alpha2,
            MemoryIdToBig_alpha3,
            MemoryIdToBig_alpha4,
            RangeCheck_7_2_5_z,
            RangeCheck_7_2_5_alpha0,
            RangeCheck_7_2_5_alpha1,
            RangeCheck_7_2_5_alpha2,
            TripleXor32_z,
            TripleXor32_alpha0,
            TripleXor32_alpha1,
            TripleXor32_alpha2,
            VerifyBitwiseXor_8_z,
            VerifyBitwiseXor_8_alpha0,
            VerifyBitwiseXor_8_alpha1,
            VerifyBitwiseXor_8_alpha2,
            seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
            claimed_sum,
        };

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            domain_vanishing_eval_inv,
        )
    }
}

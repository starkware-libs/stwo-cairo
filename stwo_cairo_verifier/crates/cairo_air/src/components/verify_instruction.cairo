use stwo_constraint_framework::{
    ClaimedPrefixSum, PreprocessedColumn, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;
use super::super::utils::UsizeExTrait;
use super::super::Invertible;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        *self.log_size
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        const N_LOOKUPS: usize = 5;
        let log_size = self.log_size();
        let preprocesed_trace_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(29, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(
            QM31_EXTENSION_DEGREE * N_LOOKUPS.div_ceil(2), log_size,
        )
            .span();
        array![preprocesed_trace_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_nonce((*self.log_size).into());
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
    pub memoryaddresstoid_lookup_elements: super::super::MemoryAddressToIdElements,
    pub memoryidtobig_lookup_elements: super::super::MemoryIdToBigElements,
    pub rangecheck_4_3_lookup_elements: super::super::RangeCheck4Bit3BitElements,
    pub range_check_7_2_5_lookup_elements: super::super::RangeCheck7Bit2Bit5BitElements,
    pub verifyinstruction_lookup_elements: super::super::VerifyInstructionElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(self.claim.log_size()).coset.step_size;
        constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        self.claim.log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let mut addr_to_id_alpha_powers = self
            .memoryaddresstoid_lookup_elements
            .alpha_powers
            .span();

        let addr_to_id_alpha_0 = *addr_to_id_alpha_powers.pop_front().unwrap();

        let addr_to_id_alpha_1 = *addr_to_id_alpha_powers.pop_front().unwrap();
        let addr_to_id_z = *self.memoryaddresstoid_lookup_elements.z;

        let mut id_to_value_alpha_powers = self.memoryidtobig_lookup_elements.alpha_powers.span();
        let id_to_value_alpha_0 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_1 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_2 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_3 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_4 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_5 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_6 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_7 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_z = *self.memoryidtobig_lookup_elements.z;

        let mut range_check_4_3_alpha_powers = self
            .rangecheck_4_3_lookup_elements
            .alpha_powers
            .span();
        let range_check_4_3_alpha_0 = *range_check_4_3_alpha_powers.pop_front().unwrap();
        let range_check_4_3_alpha_1 = *range_check_4_3_alpha_powers.pop_front().unwrap();
        let range_check_4_3_z = *self.rangecheck_4_3_lookup_elements.z;

        let mut range_check_7_2_5_alpha_powers = self
            .range_check_7_2_5_lookup_elements
            .alpha_powers
            .span();
        let range_check_7_2_5_alpha_0 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_alpha_1 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_alpha_2 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_z = *self.range_check_7_2_5_lookup_elements.z;

        let mut verify_instruction_alpha_powers = self
            .verifyinstruction_lookup_elements
            .alpha_powers
            .span();

        let verify_instruction_alpha_0 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_1 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_2 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_3 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_4 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_5 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_6 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_7 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_8 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_9 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_10 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_11 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_12 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_13 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_14 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_15 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_16 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_17 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_18 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_z = *self.verifyinstruction_lookup_elements.z;

        let log_size = self.claim.log_size();

        let _preprocessed_is_first = preprocessed_mask_values
            .get(PreprocessedColumn::IsFirst(log_size));

        let params = constraints::ConstraintParams {
            MemoryAddressToId_alpha0: addr_to_id_alpha_0,
            MemoryAddressToId_alpha1: addr_to_id_alpha_1,
            MemoryAddressToId_z: addr_to_id_z,
            MemoryIdToBig_alpha0: id_to_value_alpha_0,
            MemoryIdToBig_alpha1: id_to_value_alpha_1,
            MemoryIdToBig_alpha2: id_to_value_alpha_2,
            MemoryIdToBig_alpha3: id_to_value_alpha_3,
            MemoryIdToBig_alpha4: id_to_value_alpha_4,
            MemoryIdToBig_alpha5: id_to_value_alpha_5,
            MemoryIdToBig_alpha6: id_to_value_alpha_6,
            MemoryIdToBig_alpha7: id_to_value_alpha_7,
            MemoryIdToBig_z: id_to_value_z,
            RangeCheck_4_3_alpha0: range_check_4_3_alpha_0,
            RangeCheck_4_3_alpha1: range_check_4_3_alpha_1,
            RangeCheck_4_3_z: range_check_4_3_z,
            RangeCheck_7_2_5_alpha0: range_check_7_2_5_alpha_0,
            RangeCheck_7_2_5_alpha1: range_check_7_2_5_alpha_1,
            RangeCheck_7_2_5_alpha2: range_check_7_2_5_alpha_2,
            RangeCheck_7_2_5_z: range_check_7_2_5_z,
            VerifyInstruction_alpha0: verify_instruction_alpha_0,
            VerifyInstruction_alpha1: verify_instruction_alpha_1,
            VerifyInstruction_alpha10: verify_instruction_alpha_10,
            VerifyInstruction_alpha11: verify_instruction_alpha_11,
            VerifyInstruction_alpha12: verify_instruction_alpha_12,
            VerifyInstruction_alpha13: verify_instruction_alpha_13,
            VerifyInstruction_alpha14: verify_instruction_alpha_14,
            VerifyInstruction_alpha15: verify_instruction_alpha_15,
            VerifyInstruction_alpha16: verify_instruction_alpha_16,
            VerifyInstruction_alpha17: verify_instruction_alpha_17,
            VerifyInstruction_alpha18: verify_instruction_alpha_18,
            VerifyInstruction_alpha2: verify_instruction_alpha_2,
            VerifyInstruction_alpha3: verify_instruction_alpha_3,
            VerifyInstruction_alpha4: verify_instruction_alpha_4,
            VerifyInstruction_alpha5: verify_instruction_alpha_5,
            VerifyInstruction_alpha6: verify_instruction_alpha_6,
            VerifyInstruction_alpha7: verify_instruction_alpha_7,
            VerifyInstruction_alpha8: verify_instruction_alpha_8,
            VerifyInstruction_alpha9: verify_instruction_alpha_9,
            VerifyInstruction_z: verify_instruction_z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(log_size)),
            total_sum: *self.interaction_claim.claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

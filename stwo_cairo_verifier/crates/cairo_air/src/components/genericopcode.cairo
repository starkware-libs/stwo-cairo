use crate::components::CairoComponent;
use crate::utils::U32Impl;
use stwo_constraint_framework::{
    ClaimedPrefixSum, PreprocessedColumn, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use super::super::Invertible;

mod constraints;

pub const LOG_N_LANES: u32 = 4;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub n_calls: usize,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        core::cmp::max((*self.n_calls).next_power_of_two().ilog2(), LOG_N_LANES)
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = self.log_size();
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(229, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 65, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_nonce((*self.n_calls).into());
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub total_sum: QM31,
    pub claimed_sum: Option<ClaimedPrefixSum>,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.total_sum].span());

        if let Option::Some((sum_at_index, index)) = *self.claimed_sum {
            channel.mix_felts([sum_at_index].span());
            channel.mix_nonce(index.into());
        }
    }
}

#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub memoryaddresstoid_lookup_elements: super::super::AddrToIdElements,
    pub memoryidtobig_lookup_elements: super::super::IdToValueElements,
    pub range_check_19_lookup_elements: super::super::RangeCheck19BitElements,
    pub range_check_9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
    pub verifyinstruction_lookup_elements: super::super::VerifyInstructionElements,
    pub opcodes_lookup_elements: super::super::VmElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let claimed_sum_offset = *self.claim.n_calls;
        let trace_gen = CanonicCosetImpl::new(self.claim.log_size()).coset.step_size;
        constraints::mask_points(
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            claimed_sum_offset,
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

        let mut id_to_value_alpha_powers = self.memoryidtobig_lookup_elements.alpha_powers.span();
        let id_to_value_alpha_0 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_1 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_2 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_3 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_4 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_5 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_6 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_7 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_8 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_9 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_10 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_11 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_12 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_13 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_14 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_15 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_16 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_17 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_18 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_19 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_20 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_21 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_22 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_23 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_24 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_25 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_26 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_27 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_28 = *id_to_value_alpha_powers.pop_front().unwrap();

        let mut range_check_19_alpha_powers = self
            .range_check_19_lookup_elements
            .alpha_powers
            .span();
        let range_check_19_alpha_0 = *range_check_19_alpha_powers.pop_front().unwrap();

        let mut range_check_9_9_alpha_powers = self
            .range_check_9_9_lookup_elements
            .alpha_powers
            .span();
        let range_check_9_9_alpha_0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let range_check_9_9_alpha_1 = *range_check_9_9_alpha_powers.pop_front().unwrap();

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

        let mut vm_alpha_powers = self.opcodes_lookup_elements.alpha_powers.span();
        let vm_alpha_0 = *vm_alpha_powers.pop_front().unwrap();
        let vm_alpha_1 = *vm_alpha_powers.pop_front().unwrap();
        let vm_alpha_2 = *vm_alpha_powers.pop_front().unwrap();

        let (claimed_sum, _) = (*self.interaction_claim.claimed_sum).unwrap();

        let log_size = self.claim.log_size();

        let params = constraints::ConstraintParams {
            AddrToId_alpha0: addr_to_id_alpha_0,
            AddrToId_alpha1: addr_to_id_alpha_1,
            AddrToId_z: *self.memoryaddresstoid_lookup_elements.z,
            IdToValue_alpha0: id_to_value_alpha_0,
            IdToValue_alpha1: id_to_value_alpha_1,
            IdToValue_alpha10: id_to_value_alpha_10,
            IdToValue_alpha11: id_to_value_alpha_11,
            IdToValue_alpha12: id_to_value_alpha_12,
            IdToValue_alpha13: id_to_value_alpha_13,
            IdToValue_alpha14: id_to_value_alpha_14,
            IdToValue_alpha15: id_to_value_alpha_15,
            IdToValue_alpha16: id_to_value_alpha_16,
            IdToValue_alpha17: id_to_value_alpha_17,
            IdToValue_alpha18: id_to_value_alpha_18,
            IdToValue_alpha19: id_to_value_alpha_19,
            IdToValue_alpha2: id_to_value_alpha_2,
            IdToValue_alpha20: id_to_value_alpha_20,
            IdToValue_alpha21: id_to_value_alpha_21,
            IdToValue_alpha22: id_to_value_alpha_22,
            IdToValue_alpha23: id_to_value_alpha_23,
            IdToValue_alpha24: id_to_value_alpha_24,
            IdToValue_alpha25: id_to_value_alpha_25,
            IdToValue_alpha26: id_to_value_alpha_26,
            IdToValue_alpha27: id_to_value_alpha_27,
            IdToValue_alpha28: id_to_value_alpha_28,
            IdToValue_alpha3: id_to_value_alpha_3,
            IdToValue_alpha4: id_to_value_alpha_4,
            IdToValue_alpha5: id_to_value_alpha_5,
            IdToValue_alpha6: id_to_value_alpha_6,
            IdToValue_alpha7: id_to_value_alpha_7,
            IdToValue_alpha8: id_to_value_alpha_8,
            IdToValue_alpha9: id_to_value_alpha_9,
            IdToValue_z: *self.memoryidtobig_lookup_elements.z,
            RangeCheck_19_alpha0: range_check_19_alpha_0,
            RangeCheck_19_z: *self.range_check_19_lookup_elements.z,
            RangeCheck_9_9_alpha0: range_check_9_9_alpha_0,
            RangeCheck_9_9_alpha1: range_check_9_9_alpha_1,
            RangeCheck_9_9_z: *self.range_check_9_9_lookup_elements.z,
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
            VerifyInstruction_z: *self.verifyinstruction_lookup_elements.z,
            Vm_alpha0: vm_alpha_0,
            Vm_alpha1: vm_alpha_1,
            Vm_alpha2: vm_alpha_2,
            Vm_z: *self.opcodes_lookup_elements.z,
            claimed_sum: claimed_sum,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(log_size)),
            total_sum: *self.interaction_claim.total_sum,
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
        )
    }
}

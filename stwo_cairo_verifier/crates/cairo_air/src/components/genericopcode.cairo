use crate::components::CairoComponent;
use crate::utils::U32Impl;
use stwo_constraint_framework::{
    ClaimedPrefixSum, PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
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
    pub memoryaddresstoid_lookup_elements: super::super::MemoryAddressToIdElements,
    pub memoryidtobig_lookup_elements: super::super::MemoryIdToBigElements,
    pub range_check_19_lookup_elements: super::super::RangeCheck19BitElements,
    pub range_check_9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
    pub verifyinstruction_lookup_elements: super::super::VerifyInstructionElements,
    pub opcodes_lookup_elements: super::super::OpcodeElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = self.claim.log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        let claimed_sum_offset = *self.claim.n_calls;
        constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            claimed_sum_offset,
            log_size,
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
        let mut MemoryAddressToId_alpha_powers = self
            .memoryaddresstoid_lookup_elements
            .alpha_powers
            .span();
        let MemoryAddressToId_alpha_0 = *MemoryAddressToId_alpha_powers.pop_front().unwrap();
        let MemoryAddressToId_alpha_1 = *MemoryAddressToId_alpha_powers.pop_front().unwrap();

        let mut MemoryIdToBig_alpha_powers = self.memoryidtobig_lookup_elements.alpha_powers.span();
        let MemoryIdToBig_alpha_0 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_1 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_2 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_3 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_4 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_5 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_6 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_7 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_8 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_9 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_10 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_11 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_12 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_13 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_14 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_15 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_16 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_17 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_18 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_19 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_20 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_21 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_22 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_23 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_24 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_25 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_26 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_27 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha_28 = *MemoryIdToBig_alpha_powers.pop_front().unwrap();

        let mut RangeCheck_19_alpha_powers = self
            .range_check_19_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_19_alpha_0 = *RangeCheck_19_alpha_powers.pop_front().unwrap();

        let mut RangeCheck_9_9_alpha_powers = self
            .range_check_9_9_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_9_9_alpha_0 = *RangeCheck_9_9_alpha_powers.pop_front().unwrap();
        let RangeCheck_9_9_alpha_1 = *RangeCheck_9_9_alpha_powers.pop_front().unwrap();

        let mut VerifyInstruction_alpha_powers = self
            .verifyinstruction_lookup_elements
            .alpha_powers
            .span();
        let VerifyInstruction_alpha_0 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_1 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_2 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_3 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_4 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_5 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_6 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_7 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_8 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_9 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_10 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_11 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_12 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_13 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_14 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_15 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_16 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_17 = *VerifyInstruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha_18 = *VerifyInstruction_alpha_powers.pop_front().unwrap();

        let mut Opcodes_alpha_powers = self.opcodes_lookup_elements.alpha_powers.span();
        let Opcodes_alpha_0 = *Opcodes_alpha_powers.pop_front().unwrap();
        let Opcodes_alpha_1 = *Opcodes_alpha_powers.pop_front().unwrap();
        let Opcodes_alpha_2 = *Opcodes_alpha_powers.pop_front().unwrap();

        let (claimed_sum, _) = (*self.interaction_claim.claimed_sum).unwrap();

        let log_size = self.claim.log_size();

        let params = constraints::ConstraintParams {
            MemoryAddressToId_alpha0: MemoryAddressToId_alpha_0,
            MemoryAddressToId_alpha1: MemoryAddressToId_alpha_1,
            MemoryAddressToId_z: *self.memoryaddresstoid_lookup_elements.z,
            MemoryIdToBig_alpha0: MemoryIdToBig_alpha_0,
            MemoryIdToBig_alpha1: MemoryIdToBig_alpha_1,
            MemoryIdToBig_alpha10: MemoryIdToBig_alpha_10,
            MemoryIdToBig_alpha11: MemoryIdToBig_alpha_11,
            MemoryIdToBig_alpha12: MemoryIdToBig_alpha_12,
            MemoryIdToBig_alpha13: MemoryIdToBig_alpha_13,
            MemoryIdToBig_alpha14: MemoryIdToBig_alpha_14,
            MemoryIdToBig_alpha15: MemoryIdToBig_alpha_15,
            MemoryIdToBig_alpha16: MemoryIdToBig_alpha_16,
            MemoryIdToBig_alpha17: MemoryIdToBig_alpha_17,
            MemoryIdToBig_alpha18: MemoryIdToBig_alpha_18,
            MemoryIdToBig_alpha19: MemoryIdToBig_alpha_19,
            MemoryIdToBig_alpha2: MemoryIdToBig_alpha_2,
            MemoryIdToBig_alpha20: MemoryIdToBig_alpha_20,
            MemoryIdToBig_alpha21: MemoryIdToBig_alpha_21,
            MemoryIdToBig_alpha22: MemoryIdToBig_alpha_22,
            MemoryIdToBig_alpha23: MemoryIdToBig_alpha_23,
            MemoryIdToBig_alpha24: MemoryIdToBig_alpha_24,
            MemoryIdToBig_alpha25: MemoryIdToBig_alpha_25,
            MemoryIdToBig_alpha26: MemoryIdToBig_alpha_26,
            MemoryIdToBig_alpha27: MemoryIdToBig_alpha_27,
            MemoryIdToBig_alpha28: MemoryIdToBig_alpha_28,
            MemoryIdToBig_alpha3: MemoryIdToBig_alpha_3,
            MemoryIdToBig_alpha4: MemoryIdToBig_alpha_4,
            MemoryIdToBig_alpha5: MemoryIdToBig_alpha_5,
            MemoryIdToBig_alpha6: MemoryIdToBig_alpha_6,
            MemoryIdToBig_alpha7: MemoryIdToBig_alpha_7,
            MemoryIdToBig_alpha8: MemoryIdToBig_alpha_8,
            MemoryIdToBig_alpha9: MemoryIdToBig_alpha_9,
            MemoryIdToBig_z: *self.memoryidtobig_lookup_elements.z,
            Opcodes_alpha0: Opcodes_alpha_0,
            Opcodes_alpha1: Opcodes_alpha_1,
            Opcodes_alpha2: Opcodes_alpha_2,
            Opcodes_z: *self.opcodes_lookup_elements.z,
            RangeCheck_19_alpha0: RangeCheck_19_alpha_0,
            RangeCheck_19_z: *self.range_check_19_lookup_elements.z,
            RangeCheck_9_9_alpha0: RangeCheck_9_9_alpha_0,
            RangeCheck_9_9_alpha1: RangeCheck_9_9_alpha_1,
            RangeCheck_9_9_z: *self.range_check_9_9_lookup_elements.z,
            VerifyInstruction_alpha0: VerifyInstruction_alpha_0,
            VerifyInstruction_alpha1: VerifyInstruction_alpha_1,
            VerifyInstruction_alpha10: VerifyInstruction_alpha_10,
            VerifyInstruction_alpha11: VerifyInstruction_alpha_11,
            VerifyInstruction_alpha12: VerifyInstruction_alpha_12,
            VerifyInstruction_alpha13: VerifyInstruction_alpha_13,
            VerifyInstruction_alpha14: VerifyInstruction_alpha_14,
            VerifyInstruction_alpha15: VerifyInstruction_alpha_15,
            VerifyInstruction_alpha16: VerifyInstruction_alpha_16,
            VerifyInstruction_alpha17: VerifyInstruction_alpha_17,
            VerifyInstruction_alpha18: VerifyInstruction_alpha_18,
            VerifyInstruction_alpha2: VerifyInstruction_alpha_2,
            VerifyInstruction_alpha3: VerifyInstruction_alpha_3,
            VerifyInstruction_alpha4: VerifyInstruction_alpha_4,
            VerifyInstruction_alpha5: VerifyInstruction_alpha_5,
            VerifyInstruction_alpha6: VerifyInstruction_alpha_6,
            VerifyInstruction_alpha7: VerifyInstruction_alpha_7,
            VerifyInstruction_alpha8: VerifyInstruction_alpha_8,
            VerifyInstruction_alpha9: VerifyInstruction_alpha_9,
            VerifyInstruction_z: *self.verifyinstruction_lookup_elements.z,
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

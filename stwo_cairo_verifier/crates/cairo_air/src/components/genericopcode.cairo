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
            MemoryAddressToId_alpha0: QM31Zero::zero(),
            MemoryAddressToId_alpha1: QM31Zero::zero(),
            MemoryAddressToId_z: QM31Zero::zero(),
            MemoryIdToBig_alpha0: QM31Zero::zero(),
            MemoryIdToBig_alpha1: QM31Zero::zero(),
            MemoryIdToBig_alpha10: QM31Zero::zero(),
            MemoryIdToBig_alpha11: QM31Zero::zero(),
            MemoryIdToBig_alpha12: QM31Zero::zero(),
            MemoryIdToBig_alpha13: QM31Zero::zero(),
            MemoryIdToBig_alpha14: QM31Zero::zero(),
            MemoryIdToBig_alpha15: QM31Zero::zero(),
            MemoryIdToBig_alpha16: QM31Zero::zero(),
            MemoryIdToBig_alpha17: QM31Zero::zero(),
            MemoryIdToBig_alpha18: QM31Zero::zero(),
            MemoryIdToBig_alpha19: QM31Zero::zero(),
            MemoryIdToBig_alpha2: QM31Zero::zero(),
            MemoryIdToBig_alpha20: QM31Zero::zero(),
            MemoryIdToBig_alpha21: QM31Zero::zero(),
            MemoryIdToBig_alpha22: QM31Zero::zero(),
            MemoryIdToBig_alpha23: QM31Zero::zero(),
            MemoryIdToBig_alpha24: QM31Zero::zero(),
            MemoryIdToBig_alpha25: QM31Zero::zero(),
            MemoryIdToBig_alpha26: QM31Zero::zero(),
            MemoryIdToBig_alpha27: QM31Zero::zero(),
            MemoryIdToBig_alpha28: QM31Zero::zero(),
            MemoryIdToBig_alpha3: QM31Zero::zero(),
            MemoryIdToBig_alpha4: QM31Zero::zero(),
            MemoryIdToBig_alpha5: QM31Zero::zero(),
            MemoryIdToBig_alpha6: QM31Zero::zero(),
            MemoryIdToBig_alpha7: QM31Zero::zero(),
            MemoryIdToBig_alpha8: QM31Zero::zero(),
            MemoryIdToBig_alpha9: QM31Zero::zero(),
            MemoryIdToBig_z: QM31Zero::zero(),
            Opcodes_alpha0: QM31Zero::zero(),
            Opcodes_alpha1: QM31Zero::zero(),
            Opcodes_alpha2: QM31Zero::zero(),
            Opcodes_z: QM31Zero::zero(),
            RangeCheck_19_alpha0: QM31Zero::zero(),
            RangeCheck_19_z: QM31Zero::zero(),
            RangeCheck_9_9_alpha0: QM31Zero::zero(),
            RangeCheck_9_9_alpha1: QM31Zero::zero(),
            RangeCheck_9_9_z: QM31Zero::zero(),
            VerifyInstruction_alpha0: QM31Zero::zero(),
            VerifyInstruction_alpha1: QM31Zero::zero(),
            VerifyInstruction_alpha10: QM31Zero::zero(),
            VerifyInstruction_alpha11: QM31Zero::zero(),
            VerifyInstruction_alpha12: QM31Zero::zero(),
            VerifyInstruction_alpha13: QM31Zero::zero(),
            VerifyInstruction_alpha14: QM31Zero::zero(),
            VerifyInstruction_alpha15: QM31Zero::zero(),
            VerifyInstruction_alpha16: QM31Zero::zero(),
            VerifyInstruction_alpha17: QM31Zero::zero(),
            VerifyInstruction_alpha18: QM31Zero::zero(),
            VerifyInstruction_alpha2: QM31Zero::zero(),
            VerifyInstruction_alpha3: QM31Zero::zero(),
            VerifyInstruction_alpha4: QM31Zero::zero(),
            VerifyInstruction_alpha5: QM31Zero::zero(),
            VerifyInstruction_alpha6: QM31Zero::zero(),
            VerifyInstruction_alpha7: QM31Zero::zero(),
            VerifyInstruction_alpha8: QM31Zero::zero(),
            VerifyInstruction_alpha9: QM31Zero::zero(),
            VerifyInstruction_z: QM31Zero::zero(),
            claimed_sum: QM31Zero::zero(),
            preprocessed_is_first: QM31Zero::zero(),
            total_sum: QM31Zero::zero(),
        };

        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval,
        )
    }
}

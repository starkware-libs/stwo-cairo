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
use crate::components::CairoComponent;
use crate::utils::U32Impl;
use super::super::Invertible;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    n_calls: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        (*self.n_calls).next_power_of_two().ilog2()
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = self.log_size();
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(11, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 7, log_size)
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
        if let Option::Some((sum_at_index, index)) = *self.claimed_sum {
            channel.mix_felts([*self.total_sum, sum_at_index].span());
            channel.mix_nonce(index.into());
        } else {
            channel.mix_felts([*self.total_sum].span());
        }
    }
}

#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub memoryaddresstoid_lookup_elements: super::super::MemoryAddressToIdElements,
    pub memoryidtobig_lookup_elements: super::super::MemoryIdToBigElements,
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
        let claimed_sum_offset = *self.claim.n_calls;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
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
        let mut id_to_value_alpha_powers = self.memoryidtobig_lookup_elements.alpha_powers.span();
        let id_to_value_alpha_pow_0 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_pow_1 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_pow_2 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_pow_3 = *id_to_value_alpha_powers.pop_front().unwrap();
        let mut addr_to_id_alpha_powers = self
            .memoryaddresstoid_lookup_elements
            .alpha_powers
            .span();
        let addr_to_id_alpha_pow_0 = *addr_to_id_alpha_powers.pop_front().unwrap();
        let addr_to_id_alpha_pow_1 = *addr_to_id_alpha_powers.pop_front().unwrap();

        let mut verify_instruction_alpha_powers = self
            .verifyinstruction_lookup_elements
            .alpha_powers
            .span();
        let verify_instruction_alpha_pow_0 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_1 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_2 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_3 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_4 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_5 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_6 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_7 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_8 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_9 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_10 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_11 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_12 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_13 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_14 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_15 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_16 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let verify_instruction_alpha_pow_17 = *verify_instruction_alpha_powers.pop_front().unwrap();

        let mut vm_alpha_powers = self.opcodes_lookup_elements.alpha_powers.span();
        let vm_alpha_pow_0 = *vm_alpha_powers.pop_front().unwrap();
        let vm_alpha_pow_1 = *vm_alpha_powers.pop_front().unwrap();
        let vm_alpha_pow_2 = *vm_alpha_powers.pop_front().unwrap();
        let vm_z = *self.opcodes_lookup_elements.z;

        let (claimed_sum, _) = (*self.interaction_claim.claimed_sum).unwrap();

        let log_size = self.claim.log_size();

        let params = constraints::ConstraintParams {
            MemoryAddressToId_alpha0: addr_to_id_alpha_pow_0,
            MemoryAddressToId_alpha1: addr_to_id_alpha_pow_1,
            MemoryAddressToId_z: *self.memoryaddresstoid_lookup_elements.z,
            MemoryIdToBig_alpha0: id_to_value_alpha_pow_0,
            MemoryIdToBig_alpha1: id_to_value_alpha_pow_1,
            MemoryIdToBig_alpha2: id_to_value_alpha_pow_2,
            MemoryIdToBig_alpha3: id_to_value_alpha_pow_3,
            MemoryIdToBig_z: *self.memoryidtobig_lookup_elements.z,
            VerifyInstruction_alpha0: verify_instruction_alpha_pow_0,
            VerifyInstruction_alpha1: verify_instruction_alpha_pow_1,
            VerifyInstruction_alpha2: verify_instruction_alpha_pow_2,
            VerifyInstruction_alpha3: verify_instruction_alpha_pow_3,
            VerifyInstruction_alpha4: verify_instruction_alpha_pow_4,
            VerifyInstruction_alpha5: verify_instruction_alpha_pow_5,
            VerifyInstruction_alpha6: verify_instruction_alpha_pow_6,
            VerifyInstruction_alpha7: verify_instruction_alpha_pow_7,
            VerifyInstruction_alpha8: verify_instruction_alpha_pow_8,
            VerifyInstruction_alpha9: verify_instruction_alpha_pow_9,
            VerifyInstruction_alpha10: verify_instruction_alpha_pow_10,
            VerifyInstruction_alpha11: verify_instruction_alpha_pow_11,
            VerifyInstruction_alpha12: verify_instruction_alpha_pow_12,
            VerifyInstruction_alpha13: verify_instruction_alpha_pow_13,
            VerifyInstruction_alpha14: verify_instruction_alpha_pow_14,
            VerifyInstruction_alpha15: verify_instruction_alpha_pow_15,
            VerifyInstruction_alpha16: verify_instruction_alpha_pow_16,
            VerifyInstruction_alpha17: verify_instruction_alpha_pow_17,
            VerifyInstruction_z: *self.verifyinstruction_lookup_elements.z,
            Opcodes_alpha0: vm_alpha_pow_0,
            Opcodes_alpha1: vm_alpha_pow_1,
            Opcodes_alpha2: vm_alpha_pow_2,
            Opcodes_z: vm_z,
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

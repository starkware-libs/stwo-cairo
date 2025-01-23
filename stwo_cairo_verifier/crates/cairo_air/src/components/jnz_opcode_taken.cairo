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
use super::super::Invertible;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    n_rows: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        core::cmp::max((*self.n_rows).next_power_of_two().ilog2(), 4)
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = self.log_size();
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(42, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 4, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_nonce((*self.n_rows).into());
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
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub opcodes_lookup_elements: crate::OpcodeElements,
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let claimed_sum_offset = *self.claim.n_rows - 1;
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
        let log_size = self.claim.log_size();
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
        let VerifyInstruction_alpha7 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha8 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha9 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha10 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha11 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha12 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha13 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha14 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha15 = *verify_instruction_alpha_powers.pop_front().unwrap();

        let MemoryAddressToId_z = *self.memory_address_to_id_lookup_elements.z;
        let mut memory_address_to_id_alpha_powers = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let MemoryAddressToId_alpha0 = *memory_address_to_id_alpha_powers.pop_front().unwrap();
        let MemoryAddressToId_alpha1 = *memory_address_to_id_alpha_powers.pop_front().unwrap();

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
        let MemoryIdToBig_alpha5 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha6 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha7 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha8 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha9 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha10 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha11 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha12 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha13 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha14 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha15 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha16 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha17 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha18 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha19 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha20 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha21 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha22 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha23 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha24 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha25 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha26 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha27 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha28 = *memory_id_to_big_alpha_powers.pop_front().unwrap();

        let Opcodes_z = *self.opcodes_lookup_elements.z;
        let mut opcodes_alpha_powers = self.opcodes_lookup_elements.alpha_powers.span();
        let Opcodes_alpha0 = *opcodes_alpha_powers.pop_front().unwrap();
        let Opcodes_alpha1 = *opcodes_alpha_powers.pop_front().unwrap();
        let Opcodes_alpha2 = *opcodes_alpha_powers.pop_front().unwrap();

        let (claimed_sum, _) = (*self.interaction_claim.claimed_sum).unwrap();
        let total_sum = *self.interaction_claim.total_sum;

        let params = constraints::ConstraintParams {
            VerifyInstruction_z,
            VerifyInstruction_alpha0,
            VerifyInstruction_alpha1,
            VerifyInstruction_alpha2,
            VerifyInstruction_alpha3,
            VerifyInstruction_alpha4,
            VerifyInstruction_alpha5,
            VerifyInstruction_alpha6,
            VerifyInstruction_alpha7,
            VerifyInstruction_alpha8,
            VerifyInstruction_alpha9,
            VerifyInstruction_alpha10,
            VerifyInstruction_alpha11,
            VerifyInstruction_alpha12,
            VerifyInstruction_alpha13,
            VerifyInstruction_alpha14,
            VerifyInstruction_alpha15,
            MemoryAddressToId_z,
            MemoryAddressToId_alpha0,
            MemoryAddressToId_alpha1,
            MemoryIdToBig_z,
            MemoryIdToBig_alpha0,
            MemoryIdToBig_alpha1,
            MemoryIdToBig_alpha2,
            MemoryIdToBig_alpha3,
            MemoryIdToBig_alpha4,
            MemoryIdToBig_alpha5,
            MemoryIdToBig_alpha6,
            MemoryIdToBig_alpha7,
            MemoryIdToBig_alpha8,
            MemoryIdToBig_alpha9,
            MemoryIdToBig_alpha10,
            MemoryIdToBig_alpha11,
            MemoryIdToBig_alpha12,
            MemoryIdToBig_alpha13,
            MemoryIdToBig_alpha14,
            MemoryIdToBig_alpha15,
            MemoryIdToBig_alpha16,
            MemoryIdToBig_alpha17,
            MemoryIdToBig_alpha18,
            MemoryIdToBig_alpha19,
            MemoryIdToBig_alpha20,
            MemoryIdToBig_alpha21,
            MemoryIdToBig_alpha22,
            MemoryIdToBig_alpha23,
            MemoryIdToBig_alpha24,
            MemoryIdToBig_alpha25,
            MemoryIdToBig_alpha26,
            MemoryIdToBig_alpha27,
            MemoryIdToBig_alpha28,
            Opcodes_z,
            Opcodes_alpha0,
            Opcodes_alpha1,
            Opcodes_alpha2,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(log_size)),
            claimed_sum,
            total_sum,
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

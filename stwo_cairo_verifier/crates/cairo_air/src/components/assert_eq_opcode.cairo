use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

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
        let trace_log_sizes = ArrayImpl::new_repeated(13, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 3, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.n_rows).into());
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
        self.claim.log_size() + 1
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
        let VerifyInstruction_alpha16 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha17 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha18 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha19 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha20 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha21 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha22 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha23 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha24 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha25 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha26 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha27 = *verify_instruction_alpha_powers.pop_front().unwrap();
        let VerifyInstruction_alpha28 = *verify_instruction_alpha_powers.pop_front().unwrap();

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
            VerifyInstruction_alpha7,
            VerifyInstruction_alpha8,
            VerifyInstruction_alpha15,
            VerifyInstruction_alpha18,
            MemoryAddressToId_z,
            MemoryAddressToId_alpha0,
            MemoryAddressToId_alpha1,
            Opcodes_z,
            Opcodes_alpha0,
            Opcodes_alpha1,
            Opcodes_alpha2,
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

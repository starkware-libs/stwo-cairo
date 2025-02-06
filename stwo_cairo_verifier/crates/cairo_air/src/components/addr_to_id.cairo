use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
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

pub const N_ADDR_TO_ID_COLUMNS: usize = 16;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_ADDR_TO_ID_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 4, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
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
    pub lookup_elements: super::super::AddrToIdElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(*self.claim.log_size).coset.step_size;
        constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *self.claim.log_size + 1
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
        let mut addr_to_id_alpha_powers = self.lookup_elements.alpha_powers.span();
        let addr_to_id_alpha_0 = *addr_to_id_alpha_powers.pop_front().unwrap();
        let addr_to_id_alpha_1 = *addr_to_id_alpha_powers.pop_front().unwrap();
        let addr_to_id_z = *self.lookup_elements.z;

        let log_size = *self.claim.log_size;

        let params = constraints::ConstraintParams {
            MemoryAddressToId_alpha0: addr_to_id_alpha_0,
            MemoryAddressToId_alpha1: addr_to_id_alpha_1,
            MemoryAddressToId_z: addr_to_id_z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(log_size)),
            preprocessed_seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
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

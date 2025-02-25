use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{ChannelTrait, Channel};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::utils::{U32Impl, UsizeExTrait};
use crate::components::CairoComponent;
use super::super::Invertible;

mod constraints;

/// Split the (ID , Multiplicity) columns to shorter chunks. This is done to improve the performance
/// during The merkle commitment and FRI, as this component is usually the tallest in the Cairo AIR.
///
/// 1. The ID and Multiplicity vectors are split to 'MEMORY_ADDRESS_TO_ID_SPLIT' chunks of size
///    `ids.len()`/`MEMORY_ADDRESS_TO_ID_SPLIT`.
/// 2. The chunks are padded with 0s to the next power of 2.
///
/// #  Example
/// ID = [id0..id10], MEMORY_ADDRESS_TO_ID_SPLIT = 4:
/// ID0 = [id0, id1, id2, 0]
/// ID1 = [id3, id4, id5, 0]
/// ID2 = [id6, id7, id8, 0]
/// ID3 = [id9, id10, 0, 0]
pub const MEMORY_ADDRESS_TO_ID_SPLIT: usize = 8;
pub const N_ID_AND_MULT_COLUMNS_PER_CHUNK: usize = 2;
pub const N_TRACE_COLUMNS: usize = MEMORY_ADDRESS_TO_ID_SPLIT * N_ID_AND_MULT_COLUMNS_PER_CHUNK;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(
            QM31_EXTENSION_DEGREE * MEMORY_ADDRESS_TO_ID_SPLIT.div_ceil(2), log_size,
        )
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
    pub lookup_elements: super::super::MemoryAddressToIdElements,
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
        let mut addr_to_id_alpha_powers = self.lookup_elements.alpha_powers.span();
        let addr_to_id_alpha_0 = *addr_to_id_alpha_powers.pop_front().unwrap();
        let addr_to_id_alpha_1 = *addr_to_id_alpha_powers.pop_front().unwrap();
        let addr_to_id_z = *self.lookup_elements.z;

        let log_size = *self.claim.log_size;

        let params = constraints::ConstraintParams {
            log_size,
            MemoryAddressToId_alpha0: addr_to_id_alpha_0,
            MemoryAddressToId_alpha1: addr_to_id_alpha_1,
            MemoryAddressToId_z: addr_to_id_z,
            seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
            claimed_sum: *self.interaction_claim.claimed_sum,
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

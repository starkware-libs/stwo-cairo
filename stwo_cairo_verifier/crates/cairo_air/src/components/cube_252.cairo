use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(141, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 50, log_size)
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
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub range_check_6_lookup_elements: crate::RangeCheck6BitElements,
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub range_check_19_lookup_elements: crate::RangeCheck19BitElements,
    pub range_check_9_9_lookup_elements: crate::RangeCheck9Bit9BitElements,
}

pub impl CairoComponentImpl of CairoComponent<Component> {
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
        let Cube252_z = *self.cube_252_lookup_elements.z;
        let mut Cube252_alpha_powers = self.cube_252_lookup_elements.alpha_powers.span();
        let Cube252_alpha0 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha1 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha2 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha3 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha4 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha5 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha6 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha7 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha8 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha9 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha10 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha11 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha12 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha13 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha14 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha15 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha16 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha17 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha18 = *Cube252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha19 = *Cube252_alpha_powers.pop_front().unwrap();

        let RangeCheck_19_z = *self.range_check_19_lookup_elements.z;
        let mut range_check_19_alpha_powers = self
            .range_check_19_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_19_alpha0 = *range_check_19_alpha_powers.pop_front().unwrap();

        let RangeCheck_9_9_z = *self.range_check_9_9_lookup_elements.z;
        let mut range_check_9_9_alpha_powers = self
            .range_check_9_9_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_9_9_alpha0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let RangeCheck_9_9_alpha1 = *range_check_9_9_alpha_powers.pop_front().unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: m31(pow2(log_size)),
            Cube252_z,
            Cube252_alpha0,
            Cube252_alpha1,
            Cube252_alpha2,
            Cube252_alpha3,
            Cube252_alpha4,
            Cube252_alpha5,
            Cube252_alpha6,
            Cube252_alpha7,
            Cube252_alpha8,
            Cube252_alpha9,
            Cube252_alpha10,
            Cube252_alpha11,
            Cube252_alpha12,
            Cube252_alpha13,
            Cube252_alpha14,
            Cube252_alpha15,
            Cube252_alpha16,
            Cube252_alpha17,
            Cube252_alpha18,
            Cube252_alpha19,
            RangeCheck_19_alpha0,
            RangeCheck_19_z,
            RangeCheck_9_9_alpha0,
            RangeCheck_9_9_alpha1,
            RangeCheck_9_9_z,
            claimed_sum,
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


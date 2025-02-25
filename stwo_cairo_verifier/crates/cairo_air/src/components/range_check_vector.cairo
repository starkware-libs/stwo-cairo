use stwo_constraint_framework::{
    PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{ChannelTrait, Channel};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;
use crate::Invertible;
use super::memory_id_to_big::N_MULTIPLICITY_COLUMNS;

mod rc_11_constraints;
mod rc_12_constraints;
mod rc_18_constraints;
mod rc_19_constraints;
mod rc_3_6_6_3_constraints;
mod rc_3_6_constraints;
mod rc_4_3_constraints;
mod rc_6_constraints;
mod rc_7_2_5_constraints;
mod rc_9_9_constraints;

#[derive(Drop, Serde, Clone)]
pub struct Claim {
    pub log_ranges: Array<u32>,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_size(self: @Claim) -> u32 {
        let mut sum = 0;
        for log_range in self.log_ranges.span() {
            sum += *log_range;
        }
        sum
    }

    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = self.log_size();
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(
            self.log_ranges.len() + N_MULTIPLICITY_COLUMNS, log_size,
        )
            .span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        for log_range in self.log_ranges.span() {
            channel.mix_u64((*log_range).into());
        };
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
pub struct Rc6BitComponent {
    pub lookup_elements: crate::RangeCheck6BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc6BitComponentImpl of CairoComponent<Rc6BitComponent> {
    fn mask_points(
        self: @Rc6BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_6_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_6_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc6BitComponent) -> u32 {
        rc_6_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc6BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_6_log_size();
        let mut range_check_6_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_6_alpha_0 = *range_check_6_alpha_powers.pop_front().unwrap();
        let params = rc_6_constraints::ConstraintParams {
            RangeCheck_6_alpha0: range_check_6_alpha_0,
            RangeCheck_6_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_6_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc11BitComponent {
    pub lookup_elements: crate::RangeCheck11BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc11BitComponentImpl of CairoComponent<Rc11BitComponent> {
    fn mask_points(
        self: @Rc11BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_11_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_11_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc11BitComponent) -> u32 {
        rc_11_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc11BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_11_log_size();
        let mut range_check_11_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_11_alpha_0 = *range_check_11_alpha_powers.pop_front().unwrap();
        let params = rc_11_constraints::ConstraintParams {
            RangeCheck_11_alpha0: range_check_11_alpha_0,
            RangeCheck_11_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_11_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc12BitComponent {
    pub lookup_elements: crate::RangeCheck12BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc12ComponentImpl of CairoComponent<Rc12BitComponent> {
    fn mask_points(
        self: @Rc12BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_12_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_12_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc12BitComponent) -> u32 {
        rc_12_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc12BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_12_log_size();
        let mut range_check_12_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_12_alpha_0 = *range_check_12_alpha_powers.pop_front().unwrap();
        let params = rc_12_constraints::ConstraintParams {
            RangeCheck_12_alpha0: range_check_12_alpha_0,
            RangeCheck_12_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_12_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc18BitComponent {
    pub lookup_elements: crate::RangeCheck18BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc18BitComponentImpl of CairoComponent<Rc18BitComponent> {
    fn mask_points(
        self: @Rc18BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_18_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_18_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc18BitComponent) -> u32 {
        rc_18_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc18BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_18_log_size();
        let mut range_check_18_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_18_alpha_0 = *range_check_18_alpha_powers.pop_front().unwrap();
        let params = rc_18_constraints::ConstraintParams {
            RangeCheck_18_alpha0: range_check_18_alpha_0,
            RangeCheck_18_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_18_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc19BitComponent {
    pub lookup_elements: crate::RangeCheck19BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc19BitComponentImpl of CairoComponent<Rc19BitComponent> {
    fn mask_points(
        self: @Rc19BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_19_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_19_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc19BitComponent) -> u32 {
        rc_19_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc19BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_19_log_size();
        let mut range_check_19_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_19_alpha_0 = *range_check_19_alpha_powers.pop_front().unwrap();
        let params = rc_19_constraints::ConstraintParams {
            RangeCheck_19_alpha0: range_check_19_alpha_0,
            RangeCheck_19_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            log_size,
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_19_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}


#[derive(Drop)]
pub struct Rc9Bit9BitComponent {
    pub lookup_elements: crate::RangeCheck9Bit9BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc9Bit9BitComponentImpl of CairoComponent<Rc9Bit9BitComponent> {
    fn mask_points(
        self: @Rc9Bit9BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_9_9_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_9_9_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc9Bit9BitComponent) -> u32 {
        rc_9_9_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc9Bit9BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_9_9_log_size();
        let mut range_check_9_9_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_9_9_alpha0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let range_check_9_9_alpha1 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let params = rc_9_9_constraints::ConstraintParams {
            RangeCheck_9_9_alpha0: range_check_9_9_alpha0,
            RangeCheck_9_9_alpha1: range_check_9_9_alpha1,
            RangeCheck_9_9_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_9_9_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc3Bit6BitComponent {
    pub lookup_elements: crate::RangeCheck3Bit6BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc3Bit6BitComponentImpl of CairoComponent<Rc3Bit6BitComponent> {
    fn mask_points(
        self: @Rc3Bit6BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_3_6_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_3_6_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc3Bit6BitComponent) -> u32 {
        rc_3_6_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc3Bit6BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_3_6_log_size();
        let mut range_check_3_6_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_3_6_alpha0 = *range_check_3_6_alpha_powers.pop_front().unwrap();
        let range_check_3_6_alpha1 = *range_check_3_6_alpha_powers.pop_front().unwrap();
        let params = rc_3_6_constraints::ConstraintParams {
            RangeCheck_3_6_alpha0: range_check_3_6_alpha0,
            RangeCheck_3_6_alpha1: range_check_3_6_alpha1,
            RangeCheck_3_6_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_3_6_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc4Bit3BitComponent {
    pub lookup_elements: crate::RangeCheck4Bit3BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc4Bit3BitComponentImpl of CairoComponent<Rc4Bit3BitComponent> {
    fn mask_points(
        self: @Rc4Bit3BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_4_3_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_4_3_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc4Bit3BitComponent) -> u32 {
        rc_4_3_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc4Bit3BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_4_3_log_size();
        let mut range_check_4_3_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_4_3_alpha_0 = *range_check_4_3_alpha_powers.pop_front().unwrap();
        let range_check_4_3_alpha_1 = *range_check_4_3_alpha_powers.pop_front().unwrap();
        let params = rc_4_3_constraints::ConstraintParams {
            RangeCheck_4_3_alpha0: range_check_4_3_alpha_0,
            RangeCheck_4_3_alpha1: range_check_4_3_alpha_1,
            RangeCheck_4_3_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_4_3_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc7Bit2Bit5BitComponent {
    pub lookup_elements: crate::RangeCheck7Bit2Bit5BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc7Bit2Bit5BitComponentImpl of CairoComponent<Rc7Bit2Bit5BitComponent> {
    fn mask_points(
        self: @Rc7Bit2Bit5BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_7_2_5_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_7_2_5_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc7Bit2Bit5BitComponent) -> u32 {
        rc_7_2_5_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc7Bit2Bit5BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_7_2_5_log_size();
        let mut range_check_7_2_5_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_7_2_5_alpha0 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_alpha1 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_alpha2 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let params = rc_7_2_5_constraints::ConstraintParams {
            RangeCheck_7_2_5_alpha0: range_check_7_2_5_alpha0,
            RangeCheck_7_2_5_alpha1: range_check_7_2_5_alpha1,
            RangeCheck_7_2_5_alpha2: range_check_7_2_5_alpha2,
            RangeCheck_7_2_5_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_7_2_5_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

#[derive(Drop)]
pub struct Rc3Bit6Bit6Bit3BitComponent {
    pub lookup_elements: crate::RangeCheck3Bit6Bit6Bit3BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc3Bit6Bit6Bit3BitComponentImpl of CairoComponent<Rc3Bit6Bit6Bit3BitComponent> {
    fn mask_points(
        self: @Rc3Bit6Bit6Bit3BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_3_6_6_3_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_3_6_6_3_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc3Bit6Bit6Bit3BitComponent) -> u32 {
        rc_3_6_6_3_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc3Bit6Bit6Bit3BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_3_6_6_3_log_size();
        let mut range_check_3_6_6_3_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_3_6_6_3_alpha0 = *range_check_3_6_6_3_alpha_powers.pop_front().unwrap();
        let range_check_3_6_6_3_alpha1 = *range_check_3_6_6_3_alpha_powers.pop_front().unwrap();
        let range_check_3_6_6_3_alpha2 = *range_check_3_6_6_3_alpha_powers.pop_front().unwrap();
        let range_check_3_6_6_3_alpha3 = *range_check_3_6_6_3_alpha_powers.pop_front().unwrap();
        let params = rc_3_6_6_3_constraints::ConstraintParams {
            RangeCheck_3_6_6_3_alpha0: range_check_3_6_6_3_alpha0,
            RangeCheck_3_6_6_3_alpha1: range_check_3_6_6_3_alpha1,
            RangeCheck_3_6_6_3_alpha2: range_check_3_6_6_3_alpha2,
            RangeCheck_3_6_6_3_alpha3: range_check_3_6_6_3_alpha3,
            RangeCheck_3_6_6_3_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_3_6_6_3_constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

fn rc_6_log_size() -> u32 {
    6
}

fn rc_11_log_size() -> u32 {
    11
}

fn rc_12_log_size() -> u32 {
    12
}

fn rc_18_log_size() -> u32 {
    18
}

fn rc_19_log_size() -> u32 {
    19
}

fn rc_9_9_log_size() -> u32 {
    9 + 9
}

fn rc_3_6_log_size() -> u32 {
    3 + 6
}

fn rc_4_3_log_size() -> u32 {
    4 + 3
}

fn rc_7_2_5_log_size() -> u32 {
    7 + 2 + 5
}

fn rc_3_6_6_3_log_size() -> u32 {
    3 + 6 + 6 + 3
}

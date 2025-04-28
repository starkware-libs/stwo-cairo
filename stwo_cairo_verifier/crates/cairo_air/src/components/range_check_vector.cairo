use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::Invertible;
use crate::components::CairoComponent;
use crate::utils::U32Impl;
use super::memory_id_to_big::N_MULTIPLICITY_COLUMNS;

mod rc_11_constraints;
mod rc_12_constraints;
mod rc_18_constraints;
mod rc_19_constraints;
mod rc_3_3_3_3_3_constraints;
mod rc_3_6_6_3_constraints;
mod rc_3_6_constraints;
mod rc_4_3_constraints;
mod rc_4_4_4_4_constraints;
mod rc_4_4_constraints;
mod rc_5_4_constraints;
mod rc_6_constraints;
mod rc_7_2_5_constraints;
mod rc_8_constraints;
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
        let trace_log_sizes = ArrayImpl::new_repeated(N_MULTIPLICITY_COLUMNS, log_size).span();
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
    pub lookup_elements: crate::RangeCheck_6Elements,
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
            range_check_6_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([6, 0, 0, 0, 0], 0))),
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
pub struct Rc8BitComponent {
    pub lookup_elements: crate::RangeCheck_8Elements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc8BitComponentImpl of CairoComponent<Rc8BitComponent> {
    fn mask_points(
        self: @Rc8BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_8_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_8_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc8BitComponent) -> u32 {
        rc_8_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc8BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_8_log_size();
        let mut range_check_8_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_8_alpha_0 = *range_check_8_alpha_powers.pop_front().unwrap();
        let params = rc_8_constraints::ConstraintParams {
            RangeCheck_8_alpha0: range_check_8_alpha_0,
            RangeCheck_8_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            range_check_8_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([8, 0, 0, 0, 0], 0))),
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_8_constraints::evaluate_constraints_at_point(
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
    pub lookup_elements: crate::RangeCheck_11Elements,
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
            range_check_11_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([11, 0, 0, 0, 0], 0))),
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
    pub lookup_elements: crate::RangeCheck_12Elements,
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
            range_check_12_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([12, 0, 0, 0, 0], 0))),
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
    pub lookup_elements: crate::RangeCheck_18Elements,
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
            range_check_18_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([18, 0, 0, 0, 0], 0))),
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
    pub lookup_elements: crate::RangeCheck_19Elements,
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
            range_check_19_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([19, 0, 0, 0, 0], 0))),
            column_size: m31(pow2(log_size)),
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
    pub lookup_elements: crate::RangeCheck_9_9Elements,
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
            range_check_9_9_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([9, 9, 0, 0, 0], 0))),
            range_check_9_9_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([9, 9, 0, 0, 0], 1))),
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
    pub lookup_elements: crate::RangeCheck_3_6Elements,
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
            range_check_3_6_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 6, 0, 0, 0], 0))),
            range_check_3_6_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 6, 0, 0, 0], 1))),
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
    pub lookup_elements: crate::RangeCheck_4_3Elements,
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
            range_check_4_3_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 3, 0, 0, 0], 0))),
            range_check_4_3_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 3, 0, 0, 0], 1))),
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
pub struct Rc4Bit4BitComponent {
    pub lookup_elements: crate::RangeCheck_4_4Elements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc4Bit4BitComponentImpl of CairoComponent<Rc4Bit4BitComponent> {
    fn mask_points(
        self: @Rc4Bit4BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_4_4_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_4_4_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc4Bit4BitComponent) -> u32 {
        rc_4_4_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc4Bit4BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_4_4_log_size();
        let mut range_check_4_4_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_4_4_alpha0 = *range_check_4_4_alpha_powers.pop_front().unwrap();
        let range_check_4_4_alpha1 = *range_check_4_4_alpha_powers.pop_front().unwrap();
        let params = rc_4_4_constraints::ConstraintParams {
            RangeCheck_4_4_alpha0: range_check_4_4_alpha0,
            RangeCheck_4_4_alpha1: range_check_4_4_alpha1,
            RangeCheck_4_4_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            range_check_4_4_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 4, 0, 0, 0], 0))),
            range_check_4_4_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 4, 0, 0, 0], 1))),
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_4_4_constraints::evaluate_constraints_at_point(
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
pub struct Rc5Bit4BitComponent {
    pub lookup_elements: crate::RangeCheck_5_4Elements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc5Bit4BitComponentImpl of CairoComponent<Rc5Bit4BitComponent> {
    fn mask_points(
        self: @Rc5Bit4BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_5_4_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_5_4_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc5Bit4BitComponent) -> u32 {
        rc_5_4_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc5Bit4BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_5_4_log_size();
        let mut range_check_5_4_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_5_4_alpha0 = *range_check_5_4_alpha_powers.pop_front().unwrap();
        let range_check_5_4_alpha1 = *range_check_5_4_alpha_powers.pop_front().unwrap();
        let params = rc_5_4_constraints::ConstraintParams {
            RangeCheck_5_4_alpha0: range_check_5_4_alpha0,
            RangeCheck_5_4_alpha1: range_check_5_4_alpha1,
            RangeCheck_5_4_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            range_check_5_4_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([5, 4, 0, 0, 0], 0))),
            range_check_5_4_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([5, 4, 0, 0, 0], 1))),
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_5_4_constraints::evaluate_constraints_at_point(
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
    pub lookup_elements: crate::RangeCheck_7_2_5Elements,
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
            range_check_7_2_5_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([7, 2, 5, 0, 0], 0))),
            range_check_7_2_5_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([7, 2, 5, 0, 0], 1))),
            range_check_7_2_5_column_2: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([7, 2, 5, 0, 0], 2))),
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
    pub lookup_elements: crate::RangeCheck_3_6_6_3Elements,
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
            range_check_3_6_6_3_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 0))),
            range_check_3_6_6_3_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 1))),
            range_check_3_6_6_3_column_2: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 2))),
            range_check_3_6_6_3_column_3: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 6, 6, 3, 0], 3))),
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

#[derive(Drop)]
pub struct Rc4Bit4Bit4Bit4BitComponent {
    pub lookup_elements: crate::RangeCheck_4_4_4_4Elements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc4Bit4Bit4Bit4BitComponentImpl of CairoComponent<Rc4Bit4Bit4Bit4BitComponent> {
    fn mask_points(
        self: @Rc4Bit4Bit4Bit4BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_4_4_4_4_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_4_4_4_4_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc4Bit4Bit4Bit4BitComponent) -> u32 {
        rc_4_4_4_4_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc4Bit4Bit4Bit4BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_4_4_4_4_log_size();
        let mut range_check_4_4_4_4_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_4_4_4_4_alpha0 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let range_check_4_4_4_4_alpha1 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let range_check_4_4_4_4_alpha2 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let range_check_4_4_4_4_alpha3 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let params = rc_4_4_4_4_constraints::ConstraintParams {
            RangeCheck_4_4_4_4_alpha0: range_check_4_4_4_4_alpha0,
            RangeCheck_4_4_4_4_alpha1: range_check_4_4_4_4_alpha1,
            RangeCheck_4_4_4_4_alpha2: range_check_4_4_4_4_alpha2,
            RangeCheck_4_4_4_4_alpha3: range_check_4_4_4_4_alpha3,
            RangeCheck_4_4_4_4_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            range_check_4_4_4_4_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 0))),
            range_check_4_4_4_4_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 1))),
            range_check_4_4_4_4_column_2: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 2))),
            range_check_4_4_4_4_column_3: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([4, 4, 4, 4, 0], 3))),
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_4_4_4_4_constraints::evaluate_constraints_at_point(
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
pub struct Rc3Bit3Bit3Bit3Bit3BitComponent {
    pub lookup_elements: crate::RangeCheck_3_3_3_3_3Elements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc3Bit3Bit3Bit3Bit3BitComponentImpl of CairoComponent<Rc3Bit3Bit3Bit3Bit3BitComponent> {
    fn mask_points(
        self: @Rc3Bit3Bit3Bit3Bit3BitComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_3_3_3_3_3_log_size();
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        rc_3_3_3_3_3_constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc3Bit3Bit3Bit3Bit3BitComponent) -> u32 {
        rc_3_3_3_3_3_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc3Bit3Bit3Bit3Bit3BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let log_size = rc_3_3_3_3_3_log_size();
        let mut range_check_3_3_3_3_3_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_3_3_3_3_3_alpha0 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let range_check_3_3_3_3_3_alpha1 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let range_check_3_3_3_3_3_alpha2 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let range_check_3_3_3_3_3_alpha3 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let range_check_3_3_3_3_3_alpha4 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let params = rc_3_3_3_3_3_constraints::ConstraintParams {
            RangeCheck_3_3_3_3_3_alpha0: range_check_3_3_3_3_3_alpha0,
            RangeCheck_3_3_3_3_3_alpha1: range_check_3_3_3_3_3_alpha1,
            RangeCheck_3_3_3_3_3_alpha2: range_check_3_3_3_3_3_alpha2,
            RangeCheck_3_3_3_3_3_alpha3: range_check_3_3_3_3_3_alpha3,
            RangeCheck_3_3_3_3_3_alpha4: range_check_3_3_3_3_3_alpha4,
            RangeCheck_3_3_3_3_3_z: *self.lookup_elements.z,
            claimed_sum: *self.interaction_claim.claimed_sum,
            range_check_3_3_3_3_3_column_0: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 0))),
            range_check_3_3_3_3_3_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 1))),
            range_check_3_3_3_3_3_column_2: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 2))),
            range_check_3_3_3_3_3_column_3: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 3))),
            range_check_3_3_3_3_3_column_4: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck(([3, 3, 3, 3, 3], 4))),
            column_size: m31(pow2(log_size)),
        };
        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);
        rc_3_3_3_3_3_constraints::evaluate_constraints_at_point(
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

fn rc_8_log_size() -> u32 {
    8
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

fn rc_4_4_log_size() -> u32 {
    4 + 4
}

fn rc_5_4_log_size() -> u32 {
    5 + 4
}

fn rc_7_2_5_log_size() -> u32 {
    7 + 2 + 5
}

fn rc_3_6_6_3_log_size() -> u32 {
    3 + 6 + 6 + 3
}

fn rc_4_4_4_4_log_size() -> u32 {
    4 + 4 + 4 + 4
}

fn rc_3_3_3_3_3_log_size() -> u32 {
    3 + 3 + 3 + 3 + 3
}

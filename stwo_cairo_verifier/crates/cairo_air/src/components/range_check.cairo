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
use super::id_to_f252::N_MULTIPLICITY_COLUMNS;
use super::super::Invertible;

mod rc_19_constraints;
mod rc_4_3_constraints;
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
            channel.mix_nonce((*log_range).into());
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
pub struct Rc19BitComponent {
    pub lookup_elements: super::super::RangeCheck19BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc19BitComponentImpl of CairoComponent<Rc19BitComponent> {
    fn mask_points(
        self: @Rc19BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(rc_19_log_size()).coset.step_size;
        rc_19_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc19BitComponent) -> u32 {
        rc_19_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc19BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        // TODO: Note this is equals `1`.
        let mut range_check_19_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_19_alpha_0 = *range_check_19_alpha_powers.pop_front().unwrap();

        let params = rc_19_constraints::ConstraintParams {
            RangeCheck_19_alpha0: range_check_19_alpha_0,
            RangeCheck_19_z: *self.lookup_elements.z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(rc_19_log_size())),
            total_sum: *self.interaction_claim.claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(rc_19_log_size());
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
    pub lookup_elements: super::super::RangeCheck9Bit9BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc9Bit9BitComponentImpl of CairoComponent<Rc9Bit9BitComponent> {
    fn mask_points(
        self: @Rc9Bit9BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(rc_9_9_log_size()).coset.step_size;
        rc_9_9_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc9Bit9BitComponent) -> u32 {
        rc_9_9_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc9Bit9BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        // TODO: Note this is equals `1`.
        let mut range_check_9_9_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_9_9_alpha_0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let range_check_9_9_alpha_1 = *range_check_9_9_alpha_powers.pop_front().unwrap();

        let params = rc_9_9_constraints::ConstraintParams {
            RangeCheck_9_9_alpha0: range_check_9_9_alpha_0,
            RangeCheck_9_9_alpha1: range_check_9_9_alpha_1,
            RangeCheck_9_9_z: *self.lookup_elements.z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(rc_9_9_log_size())),
            total_sum: *self.interaction_claim.claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(rc_9_9_log_size());
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
pub struct Rc4Bit3BitComponent {
    pub lookup_elements: super::super::RangeCheck4Bit3BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc4Bit3BitComponentImpl of CairoComponent<Rc4Bit3BitComponent> {
    fn mask_points(
        self: @Rc4Bit3BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(rc_4_3_log_size()).coset.step_size;
        rc_4_3_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc4Bit3BitComponent) -> u32 {
        rc_4_3_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc4Bit3BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        // TODO: Note this is equals `1`.
        let mut range_check_4_3_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_4_3_alpha_0 = *range_check_4_3_alpha_powers.pop_front().unwrap();
        let range_check_4_3_alpha_1 = *range_check_4_3_alpha_powers.pop_front().unwrap();

        let params = rc_4_3_constraints::ConstraintParams {
            RangeCheck_4_3_alpha0: range_check_4_3_alpha_0,
            RangeCheck_4_3_alpha1: range_check_4_3_alpha_1,
            RangeCheck_4_3_z: *self.lookup_elements.z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(rc_4_3_log_size())),
            total_sum: *self.interaction_claim.claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(rc_4_3_log_size());
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
    pub lookup_elements: super::super::RangeCheck7Bit2Bit5BitElements,
    pub interaction_claim: InteractionClaim,
}

pub impl Rc7Bit2Bit5BitComponentImpl of CairoComponent<Rc7Bit2Bit5BitComponent> {
    fn mask_points(
        self: @Rc7Bit2Bit5BitComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(rc_7_2_5_log_size()).coset.step_size;
        rc_7_2_5_constraints::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @Rc7Bit2Bit5BitComponent) -> u32 {
        rc_7_2_5_log_size() + 1
    }

    fn evaluate_constraints_at_point(
        self: @Rc7Bit2Bit5BitComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let mut range_check_7_2_5_alpha_powers = self.lookup_elements.alpha_powers.span();
        let range_check_7_2_5_alpha_0 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_alpha_1 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();
        let range_check_7_2_5_alpha_2 = *range_check_7_2_5_alpha_powers.pop_front().unwrap();

        let params = rc_7_2_5_constraints::ConstraintParams {
            RangeCheck_7_2_5_alpha0: range_check_7_2_5_alpha_0,
            RangeCheck_7_2_5_alpha1: range_check_7_2_5_alpha_1,
            RangeCheck_7_2_5_alpha2: range_check_7_2_5_alpha_2,
            RangeCheck_7_2_5_z: *self.lookup_elements.z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(rc_7_2_5_log_size())),
            total_sum: *self.interaction_claim.claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(rc_7_2_5_log_size());
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

fn rc_19_log_size() -> u32 {
    19
}

fn rc_9_9_log_size() -> u32 {
    9 + 9
}

fn rc_7_2_5_log_size() -> u32 {
    7 + 2 + 5
}

fn rc_4_3_log_size() -> u32 {
    4 + 3
}

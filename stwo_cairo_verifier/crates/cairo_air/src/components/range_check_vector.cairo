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

mod rc_3_6_constraints;


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
                .get(PreprocessedColumn::RangeCheck2(([3, 6], 0))),
            range_check_3_6_column_1: preprocessed_mask_values
                .get(PreprocessedColumn::RangeCheck2(([3, 6], 1))),
            column_size: pow2(log_size).try_into().unwrap(),
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

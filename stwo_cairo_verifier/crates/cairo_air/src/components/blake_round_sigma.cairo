// Constraints version: bc855610

use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Serde, QM31Zero, qm31_const};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::{BLAKE_ROUND_SIGMA_LOG_SIZE, CairoComponent};
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = BLAKE_ROUND_SIGMA_LOG_SIZE;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(4, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {}
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
    pub blake_round_sigma_lookup_elements: crate::BlakeRoundSigmaElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = LOG_SIZE;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        preprocessed_column_set.insert(PreprocessedColumn::Seq(LOG_SIZE));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((0)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((1)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((2)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((3)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((4)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((5)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((6)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((7)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((8)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((9)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((10)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((11)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((12)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((13)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((14)));
        preprocessed_column_set.insert(PreprocessedColumn::BlakeSigma((15)));
        trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        LOG_SIZE + 1
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
        let log_size = LOG_SIZE;
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut blake_round_sigma_sum_0: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(LOG_SIZE));
        let blakesigma_0 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((0)));
        let blakesigma_1 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((1)));
        let blakesigma_2 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((2)));
        let blakesigma_3 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((3)));
        let blakesigma_4 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((4)));
        let blakesigma_5 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((5)));
        let blakesigma_6 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((6)));
        let blakesigma_7 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((7)));
        let blakesigma_8 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((8)));
        let blakesigma_9 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((9)));
        let blakesigma_10 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((10)));
        let blakesigma_11 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((11)));
        let blakesigma_12 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((12)));
        let blakesigma_13 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((13)));
        let blakesigma_14 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((14)));
        let blakesigma_15 = preprocessed_mask_values.get(PreprocessedColumn::BlakeSigma((15)));

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        blake_round_sigma_sum_0 = self
            .blake_round_sigma_lookup_elements
            .combine_qm31(
                [
                    seq, blakesigma_0, blakesigma_1, blakesigma_2, blakesigma_3, blakesigma_4,
                    blakesigma_5, blakesigma_6, blakesigma_7, blakesigma_8, blakesigma_9,
                    blakesigma_10, blakesigma_11, blakesigma_12, blakesigma_13, blakesigma_14,
                    blakesigma_15,
                ],
            );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            enabler,
            column_size,
            ref interaction_trace_mask_values,
            blake_round_sigma_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    blake_round_sigma_sum_0: QM31,
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * blake_round_sigma_sum_0)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

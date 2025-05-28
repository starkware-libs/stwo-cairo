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
use crate::components::{CairoComponent, PEDERSEN_POINTS_TABLE_LOG_SIZE};

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = PEDERSEN_POINTS_TABLE_LOG_SIZE;

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
    pub pedersen_points_table_lookup_elements: crate::PedersenPointsTableElements,
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
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((0)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((1)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((2)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((3)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((4)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((5)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((6)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((7)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((8)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((9)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((10)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((11)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((12)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((13)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((14)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((15)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((16)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((17)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((18)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((19)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((20)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((21)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((22)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((23)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((24)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((25)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((26)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((27)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((28)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((29)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((30)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((31)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((32)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((33)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((34)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((35)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((36)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((37)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((38)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((39)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((40)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((41)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((42)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((43)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((44)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((45)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((46)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((47)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((48)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((49)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((50)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((51)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((52)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((53)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((54)));
        preprocessed_column_set.insert(PreprocessedColumn::PedersenPoints((55)));
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
        let mut pedersen_points_table_sum_0: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(LOG_SIZE));
        let pedersenpoints_0 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((0)));
        let pedersenpoints_1 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((1)));
        let pedersenpoints_2 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((2)));
        let pedersenpoints_3 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((3)));
        let pedersenpoints_4 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((4)));
        let pedersenpoints_5 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((5)));
        let pedersenpoints_6 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((6)));
        let pedersenpoints_7 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((7)));
        let pedersenpoints_8 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((8)));
        let pedersenpoints_9 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((9)));
        let pedersenpoints_10 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((10)));
        let pedersenpoints_11 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((11)));
        let pedersenpoints_12 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((12)));
        let pedersenpoints_13 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((13)));
        let pedersenpoints_14 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((14)));
        let pedersenpoints_15 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((15)));
        let pedersenpoints_16 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((16)));
        let pedersenpoints_17 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((17)));
        let pedersenpoints_18 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((18)));
        let pedersenpoints_19 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((19)));
        let pedersenpoints_20 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((20)));
        let pedersenpoints_21 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((21)));
        let pedersenpoints_22 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((22)));
        let pedersenpoints_23 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((23)));
        let pedersenpoints_24 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((24)));
        let pedersenpoints_25 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((25)));
        let pedersenpoints_26 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((26)));
        let pedersenpoints_27 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((27)));
        let pedersenpoints_28 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((28)));
        let pedersenpoints_29 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((29)));
        let pedersenpoints_30 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((30)));
        let pedersenpoints_31 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((31)));
        let pedersenpoints_32 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((32)));
        let pedersenpoints_33 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((33)));
        let pedersenpoints_34 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((34)));
        let pedersenpoints_35 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((35)));
        let pedersenpoints_36 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((36)));
        let pedersenpoints_37 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((37)));
        let pedersenpoints_38 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((38)));
        let pedersenpoints_39 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((39)));
        let pedersenpoints_40 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((40)));
        let pedersenpoints_41 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((41)));
        let pedersenpoints_42 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((42)));
        let pedersenpoints_43 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((43)));
        let pedersenpoints_44 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((44)));
        let pedersenpoints_45 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((45)));
        let pedersenpoints_46 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((46)));
        let pedersenpoints_47 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((47)));
        let pedersenpoints_48 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((48)));
        let pedersenpoints_49 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((49)));
        let pedersenpoints_50 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((50)));
        let pedersenpoints_51 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((51)));
        let pedersenpoints_52 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((52)));
        let pedersenpoints_53 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((53)));
        let pedersenpoints_54 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((54)));
        let pedersenpoints_55 = preprocessed_mask_values
            .get(PreprocessedColumn::PedersenPoints((55)));

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().expect('pedersen_points_table.cairo:268')).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().expect('pedersen_points_table.cairo:269')).unbox();

        core::internal::revoke_ap_tracking();

        pedersen_points_table_sum_0 = self
            .pedersen_points_table_lookup_elements
            .combine_qm31(
                [
                    seq, pedersenpoints_0, pedersenpoints_1, pedersenpoints_2, pedersenpoints_3,
                    pedersenpoints_4, pedersenpoints_5, pedersenpoints_6, pedersenpoints_7,
                    pedersenpoints_8, pedersenpoints_9, pedersenpoints_10, pedersenpoints_11,
                    pedersenpoints_12, pedersenpoints_13, pedersenpoints_14, pedersenpoints_15,
                    pedersenpoints_16, pedersenpoints_17, pedersenpoints_18, pedersenpoints_19,
                    pedersenpoints_20, pedersenpoints_21, pedersenpoints_22, pedersenpoints_23,
                    pedersenpoints_24, pedersenpoints_25, pedersenpoints_26, pedersenpoints_27,
                    pedersenpoints_28, pedersenpoints_29, pedersenpoints_30, pedersenpoints_31,
                    pedersenpoints_32, pedersenpoints_33, pedersenpoints_34, pedersenpoints_35,
                    pedersenpoints_36, pedersenpoints_37, pedersenpoints_38, pedersenpoints_39,
                    pedersenpoints_40, pedersenpoints_41, pedersenpoints_42, pedersenpoints_43,
                    pedersenpoints_44, pedersenpoints_45, pedersenpoints_46, pedersenpoints_47,
                    pedersenpoints_48, pedersenpoints_49, pedersenpoints_50, pedersenpoints_51,
                    pedersenpoints_52, pedersenpoints_53, pedersenpoints_54, pedersenpoints_55,
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
            pedersen_points_table_sum_0,
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
    pedersen_points_table_sum_0: QM31,
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .expect('pedersen_points_table.cairo:321'))
        .unbox();

    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().expect('pedersen_points_table.cairo:324')).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().expect('pedersen_points_table.cairo:325')).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().expect('pedersen_points_table.cairo:326')).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().expect('pedersen_points_table.cairo:327')).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * pedersen_points_table_sum_0)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

// Constraints version: 9330aaaf

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
use crate::components::{CairoComponent, POSEIDON_ROUND_KEYS_LOG_SIZE};
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = POSEIDON_ROUND_KEYS_LOG_SIZE;

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

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((LOG_SIZE).into());
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
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
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
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        preprocessed_column_set.insert(PreprocessedColumn::Seq(LOG_SIZE));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((0)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((1)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((2)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((3)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((4)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((5)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((6)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((7)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((8)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((9)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((10)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((11)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((12)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((13)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((14)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((15)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((16)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((17)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((18)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((19)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((20)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((21)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((22)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((23)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((24)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((25)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((26)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((27)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((28)));
        preprocessed_column_set.insert(PreprocessedColumn::PoseidonRoundKeys((29)));
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
        let mut poseidon_round_keys_sum_0: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(LOG_SIZE));
        let poseidonroundkeys_0 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((0)));
        let poseidonroundkeys_1 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((1)));
        let poseidonroundkeys_2 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((2)));
        let poseidonroundkeys_3 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((3)));
        let poseidonroundkeys_4 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((4)));
        let poseidonroundkeys_5 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((5)));
        let poseidonroundkeys_6 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((6)));
        let poseidonroundkeys_7 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((7)));
        let poseidonroundkeys_8 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((8)));
        let poseidonroundkeys_9 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((9)));
        let poseidonroundkeys_10 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((10)));
        let poseidonroundkeys_11 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((11)));
        let poseidonroundkeys_12 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((12)));
        let poseidonroundkeys_13 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((13)));
        let poseidonroundkeys_14 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((14)));
        let poseidonroundkeys_15 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((15)));
        let poseidonroundkeys_16 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((16)));
        let poseidonroundkeys_17 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((17)));
        let poseidonroundkeys_18 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((18)));
        let poseidonroundkeys_19 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((19)));
        let poseidonroundkeys_20 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((20)));
        let poseidonroundkeys_21 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((21)));
        let poseidonroundkeys_22 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((22)));
        let poseidonroundkeys_23 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((23)));
        let poseidonroundkeys_24 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((24)));
        let poseidonroundkeys_25 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((25)));
        let poseidonroundkeys_26 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((26)));
        let poseidonroundkeys_27 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((27)));
        let poseidonroundkeys_28 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((28)));
        let poseidonroundkeys_29 = preprocessed_mask_values
            .get(PreprocessedColumn::PoseidonRoundKeys((29)));

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        poseidon_round_keys_sum_0 = self
            .poseidon_round_keys_lookup_elements
            .combine_qm31(
                [
                    seq, poseidonroundkeys_0, poseidonroundkeys_1, poseidonroundkeys_2,
                    poseidonroundkeys_3, poseidonroundkeys_4, poseidonroundkeys_5,
                    poseidonroundkeys_6, poseidonroundkeys_7, poseidonroundkeys_8,
                    poseidonroundkeys_9, poseidonroundkeys_10, poseidonroundkeys_11,
                    poseidonroundkeys_12, poseidonroundkeys_13, poseidonroundkeys_14,
                    poseidonroundkeys_15, poseidonroundkeys_16, poseidonroundkeys_17,
                    poseidonroundkeys_18, poseidonroundkeys_19, poseidonroundkeys_20,
                    poseidonroundkeys_21, poseidonroundkeys_22, poseidonroundkeys_23,
                    poseidonroundkeys_24, poseidonroundkeys_25, poseidonroundkeys_26,
                    poseidonroundkeys_27, poseidonroundkeys_28, poseidonroundkeys_29,
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
            poseidon_round_keys_sum_0,
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
    poseidon_round_keys_sum_0: QM31,
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
        * poseidon_round_keys_sum_0)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

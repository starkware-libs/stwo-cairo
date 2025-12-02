// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
const LOG_SIZE: u32 = 20;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 16].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {}

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {}
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
    pub range_check_20_lookup_elements: crate::RangeCheck_20Elements,
    pub range_check_20_b_lookup_elements: crate::RangeCheck_20_BElements,
    pub range_check_20_c_lookup_elements: crate::RangeCheck_20_CElements,
    pub range_check_20_d_lookup_elements: crate::RangeCheck_20_DElements,
    pub range_check_20_e_lookup_elements: crate::RangeCheck_20_EElements,
    pub range_check_20_f_lookup_elements: crate::RangeCheck_20_FElements,
    pub range_check_20_g_lookup_elements: crate::RangeCheck_20_GElements,
    pub range_check_20_h_lookup_elements: crate::RangeCheck_20_HElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        interaction_elements: @CairoInteractionElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            range_check_20_lookup_elements: interaction_elements.range_checks.rc_20.clone(),
            range_check_20_b_lookup_elements: interaction_elements.range_checks.rc_20_b.clone(),
            range_check_20_c_lookup_elements: interaction_elements.range_checks.rc_20_c.clone(),
            range_check_20_d_lookup_elements: interaction_elements.range_checks.rc_20_d.clone(),
            range_check_20_e_lookup_elements: interaction_elements.range_checks.rc_20_e.clone(),
            range_check_20_f_lookup_elements: interaction_elements.range_checks.rc_20_f.clone(),
            range_check_20_g_lookup_elements: interaction_elements.range_checks.rc_20_g.clone(),
            range_check_20_h_lookup_elements: interaction_elements.range_checks.rc_20_h.clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
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
        let mut range_check_20_sum_0: QM31 = Zero::zero();
        let mut range_check_20_b_sum_1: QM31 = Zero::zero();
        let mut range_check_20_c_sum_2: QM31 = Zero::zero();
        let mut range_check_20_d_sum_3: QM31 = Zero::zero();
        let mut range_check_20_e_sum_4: QM31 = Zero::zero();
        let mut range_check_20_f_sum_5: QM31 = Zero::zero();
        let mut range_check_20_g_sum_6: QM31 = Zero::zero();
        let mut range_check_20_h_sum_7: QM31 = Zero::zero();
        let seq_20 = preprocessed_mask_values.get_and_mark_used(SEQ_20_IDX);

        let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        range_check_20_sum_0 = self.range_check_20_lookup_elements.combine_qm31([seq_20]);

        range_check_20_b_sum_1 = self.range_check_20_b_lookup_elements.combine_qm31([seq_20]);

        range_check_20_c_sum_2 = self.range_check_20_c_lookup_elements.combine_qm31([seq_20]);

        range_check_20_d_sum_3 = self.range_check_20_d_lookup_elements.combine_qm31([seq_20]);

        range_check_20_e_sum_4 = self.range_check_20_e_lookup_elements.combine_qm31([seq_20]);

        range_check_20_f_sum_5 = self.range_check_20_f_lookup_elements.combine_qm31([seq_20]);

        range_check_20_g_sum_6 = self.range_check_20_g_lookup_elements.combine_qm31([seq_20]);

        range_check_20_h_sum_7 = self.range_check_20_h_lookup_elements.combine_qm31([seq_20]);

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            enabler,
            column_size,
            ref interaction_trace_mask_values,
            range_check_20_sum_0,
            range_check_20_b_sum_1,
            range_check_20_c_sum_2,
            range_check_20_d_sum_3,
            range_check_20_e_sum_4,
            range_check_20_f_sum_5,
            range_check_20_g_sum_6,
            range_check_20_h_sum_7,
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
    range_check_20_sum_0: QM31,
    range_check_20_b_sum_1: QM31,
    range_check_20_c_sum_2: QM31,
    range_check_20_d_sum_3: QM31,
    range_check_20_e_sum_4: QM31,
    range_check_20_f_sum_5: QM31,
    range_check_20_g_sum_6: QM31,
    range_check_20_h_sum_7: QM31,
) {
    let [
        trace_2_col0,
        trace_2_col1,
        trace_2_col2,
        trace_2_col3,
        trace_2_col4,
        trace_2_col5,
        trace_2_col6,
        trace_2_col7,
        trace_2_col8,
        trace_2_col9,
        trace_2_col10,
        trace_2_col11,
        trace_2_col12,
        trace_2_col13,
        trace_2_col14,
        trace_2_col15,
    ]: [Span<QM31>; 16] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4]: [QM31; 1] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5]: [QM31; 1] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6]: [QM31; 1] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7]: [QM31; 1] = (*trace_2_col7.try_into().unwrap()).unbox();
    let [trace_2_col8]: [QM31; 1] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9]: [QM31; 1] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10]: [QM31; 1] = (*trace_2_col10.try_into().unwrap()).unbox();
    let [trace_2_col11]: [QM31; 1] = (*trace_2_col11.try_into().unwrap()).unbox();
    let [trace_2_col12_neg1, trace_2_col12]: [QM31; 2] = (*trace_2_col12.try_into().unwrap())
        .unbox();
    let [trace_2_col13_neg1, trace_2_col13]: [QM31; 2] = (*trace_2_col13.try_into().unwrap())
        .unbox();
    let [trace_2_col14_neg1, trace_2_col14]: [QM31; 2] = (*trace_2_col14.try_into().unwrap())
        .unbox();
    let [trace_2_col15_neg1, trace_2_col15]: [QM31; 2] = (*trace_2_col15.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_20_sum_0
        * range_check_20_b_sum_1)
        + range_check_20_sum_0
        + range_check_20_b_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_20_c_sum_2
        * range_check_20_d_sum_3)
        + range_check_20_c_sum_2
        + range_check_20_d_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_20_e_sum_4
        * range_check_20_f_sum_5)
        + range_check_20_e_sum_4
        + range_check_20_f_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
        - QM31Impl::from_partial_evals(
            [trace_2_col12_neg1, trace_2_col13_neg1, trace_2_col14_neg1, trace_2_col15_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * range_check_20_g_sum_6
        * range_check_20_h_sum_7)
        + (range_check_20_g_sum_6 * enabler)
        + range_check_20_h_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

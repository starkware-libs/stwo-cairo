// AIR version 98896da1
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 39;pub const RELATION_USES_PER_ROW: [(felt252, u32); 3] = [
    ('Sha256SmallSigma0', 1), ('Sha256SmallSigma1', 1), ('RangeCheck_2', 2)
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 12].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        accumulate_relation_uses(ref relation_uses, RELATION_USES_PER_ROW.span(), *self.log_size);
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
    pub sha_256_small_sigma_0_lookup_elements: crate::Sha256SmallSigma0Elements,
pub sha_256_small_sigma_1_lookup_elements: crate::Sha256SmallSigma1Elements,
pub range_check_2_lookup_elements: crate::RangeCheck_2Elements,
pub sha_256_schedule_lookup_elements: crate::Sha256ScheduleElements,
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
            sha_256_small_sigma_0_lookup_elements: interaction_elements.sha_256_small_sigma_0.clone(),
sha_256_small_sigma_1_lookup_elements: interaction_elements.sha_256_small_sigma_1.clone(),
range_check_2_lookup_elements: interaction_elements.range_checks.rc_2.clone(),
sha_256_schedule_lookup_elements: interaction_elements.sha_256_schedule.clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *(self.claim.log_size);
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *(self.claim.log_size) + 1
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
        let log_size = *(self.claim.log_size);
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut sha_256_small_sigma_0_sum_0: QM31 = Zero::zero();let mut sha_256_small_sigma_1_sum_1: QM31 = Zero::zero();let mut range_check_2_sum_2: QM31 = Zero::zero();let mut range_check_2_sum_3: QM31 = Zero::zero();let mut sha_256_schedule_sum_4: QM31 = Zero::zero();

        let [input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7, input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11, input_limb_12_col12, input_limb_13_col13, input_limb_14_col14, input_limb_15_col15, input_limb_16_col16, input_limb_17_col17, input_limb_18_col18, input_limb_19_col19, input_limb_20_col20, input_limb_21_col21, input_limb_22_col22, input_limb_23_col23, input_limb_24_col24, input_limb_25_col25, input_limb_26_col26, input_limb_27_col27, input_limb_28_col28, input_limb_29_col29, input_limb_30_col30, input_limb_31_col31, sha_256_small_sigma_0_output_limb_0_col32, sha_256_small_sigma_0_output_limb_1_col33, sha_256_small_sigma_1_output_limb_0_col34, sha_256_small_sigma_1_output_limb_1_col35, w_i_limb_0_col36, w_i_limb_1_col37, enabler]: [Span<QM31>; 39]
            = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();let [input_limb_3_col3]: [QM31; 1] = (*input_limb_3_col3.try_into().unwrap()).unbox();let [input_limb_4_col4]: [QM31; 1] = (*input_limb_4_col4.try_into().unwrap()).unbox();let [input_limb_5_col5]: [QM31; 1] = (*input_limb_5_col5.try_into().unwrap()).unbox();let [input_limb_6_col6]: [QM31; 1] = (*input_limb_6_col6.try_into().unwrap()).unbox();let [input_limb_7_col7]: [QM31; 1] = (*input_limb_7_col7.try_into().unwrap()).unbox();let [input_limb_8_col8]: [QM31; 1] = (*input_limb_8_col8.try_into().unwrap()).unbox();let [input_limb_9_col9]: [QM31; 1] = (*input_limb_9_col9.try_into().unwrap()).unbox();let [input_limb_10_col10]: [QM31; 1] = (*input_limb_10_col10.try_into().unwrap()).unbox();let [input_limb_11_col11]: [QM31; 1] = (*input_limb_11_col11.try_into().unwrap()).unbox();let [input_limb_12_col12]: [QM31; 1] = (*input_limb_12_col12.try_into().unwrap()).unbox();let [input_limb_13_col13]: [QM31; 1] = (*input_limb_13_col13.try_into().unwrap()).unbox();let [input_limb_14_col14]: [QM31; 1] = (*input_limb_14_col14.try_into().unwrap()).unbox();let [input_limb_15_col15]: [QM31; 1] = (*input_limb_15_col15.try_into().unwrap()).unbox();let [input_limb_16_col16]: [QM31; 1] = (*input_limb_16_col16.try_into().unwrap()).unbox();let [input_limb_17_col17]: [QM31; 1] = (*input_limb_17_col17.try_into().unwrap()).unbox();let [input_limb_18_col18]: [QM31; 1] = (*input_limb_18_col18.try_into().unwrap()).unbox();let [input_limb_19_col19]: [QM31; 1] = (*input_limb_19_col19.try_into().unwrap()).unbox();let [input_limb_20_col20]: [QM31; 1] = (*input_limb_20_col20.try_into().unwrap()).unbox();let [input_limb_21_col21]: [QM31; 1] = (*input_limb_21_col21.try_into().unwrap()).unbox();let [input_limb_22_col22]: [QM31; 1] = (*input_limb_22_col22.try_into().unwrap()).unbox();let [input_limb_23_col23]: [QM31; 1] = (*input_limb_23_col23.try_into().unwrap()).unbox();let [input_limb_24_col24]: [QM31; 1] = (*input_limb_24_col24.try_into().unwrap()).unbox();let [input_limb_25_col25]: [QM31; 1] = (*input_limb_25_col25.try_into().unwrap()).unbox();let [input_limb_26_col26]: [QM31; 1] = (*input_limb_26_col26.try_into().unwrap()).unbox();let [input_limb_27_col27]: [QM31; 1] = (*input_limb_27_col27.try_into().unwrap()).unbox();let [input_limb_28_col28]: [QM31; 1] = (*input_limb_28_col28.try_into().unwrap()).unbox();let [input_limb_29_col29]: [QM31; 1] = (*input_limb_29_col29.try_into().unwrap()).unbox();let [input_limb_30_col30]: [QM31; 1] = (*input_limb_30_col30.try_into().unwrap()).unbox();let [input_limb_31_col31]: [QM31; 1] = (*input_limb_31_col31.try_into().unwrap()).unbox();let [sha_256_small_sigma_0_output_limb_0_col32]: [QM31; 1] = (*sha_256_small_sigma_0_output_limb_0_col32.try_into().unwrap()).unbox();let [sha_256_small_sigma_0_output_limb_1_col33]: [QM31; 1] = (*sha_256_small_sigma_0_output_limb_1_col33.try_into().unwrap()).unbox();let [sha_256_small_sigma_1_output_limb_0_col34]: [QM31; 1] = (*sha_256_small_sigma_1_output_limb_0_col34.try_into().unwrap()).unbox();let [sha_256_small_sigma_1_output_limb_1_col35]: [QM31; 1] = (*sha_256_small_sigma_1_output_limb_1_col35.try_into().unwrap()).unbox();let [w_i_limb_0_col36]: [QM31; 1] = (*w_i_limb_0_col36.try_into().unwrap()).unbox();let [w_i_limb_1_col37]: [QM31; 1] = (*w_i_limb_1_col37.try_into().unwrap()).unbox();let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();


        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        sha_256_small_sigma_0_sum_0 = self.sha_256_small_sigma_0_lookup_elements.combine_qm31(
            [
                input_limb_2_col2,
input_limb_3_col3,
sha_256_small_sigma_0_output_limb_0_col32,
sha_256_small_sigma_0_output_limb_1_col33
            ],
        );

        sha_256_small_sigma_1_sum_1 = self.sha_256_small_sigma_1_lookup_elements.combine_qm31(
            [
                input_limb_28_col28,
input_limb_29_col29,
sha_256_small_sigma_1_output_limb_0_col34,
sha_256_small_sigma_1_output_limb_1_col35
            ],
        );let carry_low_tmp_3b5f1_3: QM31 = (((((input_limb_0_col0 + sha_256_small_sigma_0_output_limb_0_col32) + input_limb_18_col18) + sha_256_small_sigma_1_output_limb_0_col34) - w_i_limb_0_col36) * qm31_const::<32768, 0, 0, 0>());

        range_check_2_sum_2 = self.range_check_2_lookup_elements.combine_qm31(
            [
                carry_low_tmp_3b5f1_3
            ],
        );let carry_high_tmp_3b5f1_4: QM31 = ((((((input_limb_1_col1 + sha_256_small_sigma_0_output_limb_1_col33) + input_limb_19_col19) + sha_256_small_sigma_1_output_limb_1_col35) + carry_low_tmp_3b5f1_3) - w_i_limb_1_col37) * qm31_const::<32768, 0, 0, 0>());

        range_check_2_sum_3 = self.range_check_2_lookup_elements.combine_qm31(
            [
                carry_high_tmp_3b5f1_4
            ],
        );

        sha_256_schedule_sum_4 = self.sha_256_schedule_lookup_elements.combine_qm31(
            [
                input_limb_0_col0,
input_limb_1_col1,
input_limb_2_col2,
input_limb_3_col3,
input_limb_4_col4,
input_limb_5_col5,
input_limb_6_col6,
input_limb_7_col7,
input_limb_8_col8,
input_limb_9_col9,
input_limb_10_col10,
input_limb_11_col11,
input_limb_12_col12,
input_limb_13_col13,
input_limb_14_col14,
input_limb_15_col15,
input_limb_16_col16,
input_limb_17_col17,
input_limb_18_col18,
input_limb_19_col19,
input_limb_20_col20,
input_limb_21_col21,
input_limb_22_col22,
input_limb_23_col23,
input_limb_24_col24,
input_limb_25_col25,
input_limb_26_col26,
input_limb_27_col27,
input_limb_28_col28,
input_limb_29_col29,
input_limb_30_col30,
input_limb_31_col31,
w_i_limb_0_col36,
w_i_limb_1_col37
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
            sha_256_small_sigma_0_sum_0,
sha_256_small_sigma_1_sum_1,
range_check_2_sum_2,
range_check_2_sum_3,
sha_256_schedule_sum_4
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
    sha_256_small_sigma_0_sum_0: QM31,
sha_256_small_sigma_1_sum_1: QM31,
range_check_2_sum_2: QM31,
range_check_2_sum_3: QM31,
sha_256_schedule_sum_4: QM31
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3, trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7, trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]: [Span<QM31>; 12]
        = (*interaction_trace_mask_values.multi_pop_front().unwrap()).unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
let [trace_2_col4]: [QM31; 1] = (*trace_2_col4.try_into().unwrap()).unbox();
let [trace_2_col5]: [QM31; 1] = (*trace_2_col5.try_into().unwrap()).unbox();
let [trace_2_col6]: [QM31; 1] = (*trace_2_col6.try_into().unwrap()).unbox();
let [trace_2_col7]: [QM31; 1] = (*trace_2_col7.try_into().unwrap()).unbox();
let [trace_2_col8_neg1, trace_2_col8]: [QM31; 2] = (*trace_2_col8.try_into().unwrap()).unbox();
let [trace_2_col9_neg1, trace_2_col9]: [QM31; 2] = (*trace_2_col9.try_into().unwrap()).unbox();
let [trace_2_col10_neg1, trace_2_col10]: [QM31; 2] = (*trace_2_col10.try_into().unwrap()).unbox();
let [trace_2_col11_neg1, trace_2_col11]: [QM31; 2] = (*trace_2_col11.try_into().unwrap()).unbox();


    core::internal::revoke_ap_tracking();

    
let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * sha_256_small_sigma_0_sum_0 * sha_256_small_sigma_1_sum_1
        ) - sha_256_small_sigma_0_sum_0 - sha_256_small_sigma_1_sum_1
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]) 
                - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * range_check_2_sum_2 * range_check_2_sum_3
        ) - range_check_2_sum_2 - range_check_2_sum_3
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]) 
                - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]) 
                - QM31Impl::from_partial_evals([trace_2_col8_neg1, trace_2_col9_neg1, trace_2_col10_neg1, trace_2_col11_neg1])
                + (claimed_sum * (column_size.inverse().into()))
            ) * sha_256_schedule_sum_4
        ) + enabler
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

}
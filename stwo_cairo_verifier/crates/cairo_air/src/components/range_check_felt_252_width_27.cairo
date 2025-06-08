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
use crate::components::CairoComponent;

pub const N_TRACE_COLUMNS: usize = 20;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 7] = [
    ('RangeCheck_9_9', 1), ('RangeCheck_18', 7), ('RangeCheck_9_9_B', 1), ('RangeCheck_18_B', 2),
    ('RangeCheck_9_9_C', 1), ('RangeCheck_9_9_D', 1), ('RangeCheck_9_9_E', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(32, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
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
    pub range_check_9_9_lookup_elements: crate::RangeCheck_9_9Elements,
    pub range_check_18_lookup_elements: crate::RangeCheck_18Elements,
    pub range_check_9_9_b_lookup_elements: crate::RangeCheck_9_9_BElements,
    pub range_check_18_b_lookup_elements: crate::RangeCheck_18_BElements,
    pub range_check_9_9_c_lookup_elements: crate::RangeCheck_9_9_CElements,
    pub range_check_9_9_d_lookup_elements: crate::RangeCheck_9_9_DElements,
    pub range_check_9_9_e_lookup_elements: crate::RangeCheck_9_9_EElements,
    pub range_check_felt_252_width_27_lookup_elements: crate::RangeCheckFelt252Width27Elements,
}

pub impl ComponentImpl of CairoComponent<Component> {
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
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
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
        let mut range_check_9_9_sum_0: QM31 = Zero::zero();
        let mut range_check_18_sum_1: QM31 = Zero::zero();
        let mut range_check_18_sum_2: QM31 = Zero::zero();
        let mut range_check_9_9_b_sum_3: QM31 = Zero::zero();
        let mut range_check_18_b_sum_4: QM31 = Zero::zero();
        let mut range_check_18_sum_5: QM31 = Zero::zero();
        let mut range_check_9_9_c_sum_6: QM31 = Zero::zero();
        let mut range_check_18_sum_7: QM31 = Zero::zero();
        let mut range_check_18_sum_8: QM31 = Zero::zero();
        let mut range_check_9_9_d_sum_9: QM31 = Zero::zero();
        let mut range_check_18_b_sum_10: QM31 = Zero::zero();
        let mut range_check_18_sum_11: QM31 = Zero::zero();
        let mut range_check_9_9_e_sum_12: QM31 = Zero::zero();
        let mut range_check_18_sum_13: QM31 = Zero::zero();
        let mut range_check_felt_252_width_27_sum_14: QM31 = Zero::zero();

        let [
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
            limb_0_high_part_col10,
            limb_1_low_part_col11,
            limb_2_high_part_col12,
            limb_3_low_part_col13,
            limb_4_high_part_col14,
            limb_5_low_part_col15,
            limb_6_high_part_col16,
            limb_7_low_part_col17,
            limb_8_high_part_col18,
            enabler,
        ]: [Span<QM31>; 20] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();
        let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();
        let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();
        let [input_limb_3_col3]: [QM31; 1] = (*input_limb_3_col3.try_into().unwrap()).unbox();
        let [input_limb_4_col4]: [QM31; 1] = (*input_limb_4_col4.try_into().unwrap()).unbox();
        let [input_limb_5_col5]: [QM31; 1] = (*input_limb_5_col5.try_into().unwrap()).unbox();
        let [input_limb_6_col6]: [QM31; 1] = (*input_limb_6_col6.try_into().unwrap()).unbox();
        let [input_limb_7_col7]: [QM31; 1] = (*input_limb_7_col7.try_into().unwrap()).unbox();
        let [input_limb_8_col8]: [QM31; 1] = (*input_limb_8_col8.try_into().unwrap()).unbox();
        let [input_limb_9_col9]: [QM31; 1] = (*input_limb_9_col9.try_into().unwrap()).unbox();
        let [limb_0_high_part_col10]: [QM31; 1] = (*limb_0_high_part_col10.try_into().unwrap())
            .unbox();
        let [limb_1_low_part_col11]: [QM31; 1] = (*limb_1_low_part_col11.try_into().unwrap())
            .unbox();
        let [limb_2_high_part_col12]: [QM31; 1] = (*limb_2_high_part_col12.try_into().unwrap())
            .unbox();
        let [limb_3_low_part_col13]: [QM31; 1] = (*limb_3_low_part_col13.try_into().unwrap())
            .unbox();
        let [limb_4_high_part_col14]: [QM31; 1] = (*limb_4_high_part_col14.try_into().unwrap())
            .unbox();
        let [limb_5_low_part_col15]: [QM31; 1] = (*limb_5_low_part_col15.try_into().unwrap())
            .unbox();
        let [limb_6_high_part_col16]: [QM31; 1] = (*limb_6_high_part_col16.try_into().unwrap())
            .unbox();
        let [limb_7_low_part_col17]: [QM31; 1] = (*limb_7_low_part_col17.try_into().unwrap())
            .unbox();
        let [limb_8_high_part_col18]: [QM31; 1] = (*limb_8_high_part_col18.try_into().unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        range_check_9_9_sum_0 = self
            .range_check_9_9_lookup_elements
            .combine_qm31([limb_0_high_part_col10, limb_1_low_part_col11]);

        range_check_18_sum_1 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [(input_limb_0_col0 - (limb_0_high_part_col10 * qm31_const::<262144, 0, 0, 0>()))],
            );

        range_check_18_sum_2 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [((input_limb_1_col1 - limb_1_low_part_col11) * qm31_const::<4194304, 0, 0, 0>())],
            );

        range_check_9_9_b_sum_3 = self
            .range_check_9_9_b_lookup_elements
            .combine_qm31([limb_2_high_part_col12, limb_3_low_part_col13]);

        range_check_18_b_sum_4 = self
            .range_check_18_b_lookup_elements
            .combine_qm31(
                [(input_limb_2_col2 - (limb_2_high_part_col12 * qm31_const::<262144, 0, 0, 0>()))],
            );

        range_check_18_sum_5 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [((input_limb_3_col3 - limb_3_low_part_col13) * qm31_const::<4194304, 0, 0, 0>())],
            );

        range_check_9_9_c_sum_6 = self
            .range_check_9_9_c_lookup_elements
            .combine_qm31([limb_4_high_part_col14, limb_5_low_part_col15]);

        range_check_18_sum_7 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [(input_limb_4_col4 - (limb_4_high_part_col14 * qm31_const::<262144, 0, 0, 0>()))],
            );

        range_check_18_sum_8 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [((input_limb_5_col5 - limb_5_low_part_col15) * qm31_const::<4194304, 0, 0, 0>())],
            );

        range_check_9_9_d_sum_9 = self
            .range_check_9_9_d_lookup_elements
            .combine_qm31([limb_6_high_part_col16, limb_7_low_part_col17]);

        range_check_18_b_sum_10 = self
            .range_check_18_b_lookup_elements
            .combine_qm31(
                [(input_limb_6_col6 - (limb_6_high_part_col16 * qm31_const::<262144, 0, 0, 0>()))],
            );

        range_check_18_sum_11 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [((input_limb_7_col7 - limb_7_low_part_col17) * qm31_const::<4194304, 0, 0, 0>())],
            );

        range_check_9_9_e_sum_12 = self
            .range_check_9_9_e_lookup_elements
            .combine_qm31([limb_8_high_part_col18, input_limb_9_col9]);

        range_check_18_sum_13 = self
            .range_check_18_lookup_elements
            .combine_qm31(
                [(input_limb_8_col8 - (limb_8_high_part_col18 * qm31_const::<262144, 0, 0, 0>()))],
            );

        range_check_felt_252_width_27_sum_14 = self
            .range_check_felt_252_width_27_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                    input_limb_8_col8, input_limb_9_col9,
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
            range_check_9_9_sum_0,
            range_check_18_sum_1,
            range_check_18_sum_2,
            range_check_9_9_b_sum_3,
            range_check_18_b_sum_4,
            range_check_18_sum_5,
            range_check_9_9_c_sum_6,
            range_check_18_sum_7,
            range_check_18_sum_8,
            range_check_9_9_d_sum_9,
            range_check_18_b_sum_10,
            range_check_18_sum_11,
            range_check_9_9_e_sum_12,
            range_check_18_sum_13,
            range_check_felt_252_width_27_sum_14,
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
    range_check_9_9_sum_0: QM31,
    range_check_18_sum_1: QM31,
    range_check_18_sum_2: QM31,
    range_check_9_9_b_sum_3: QM31,
    range_check_18_b_sum_4: QM31,
    range_check_18_sum_5: QM31,
    range_check_9_9_c_sum_6: QM31,
    range_check_18_sum_7: QM31,
    range_check_18_sum_8: QM31,
    range_check_9_9_d_sum_9: QM31,
    range_check_18_b_sum_10: QM31,
    range_check_18_sum_11: QM31,
    range_check_9_9_e_sum_12: QM31,
    range_check_18_sum_13: QM31,
    range_check_felt_252_width_27_sum_14: QM31,
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
        trace_2_col16,
        trace_2_col17,
        trace_2_col18,
        trace_2_col19,
        trace_2_col20,
        trace_2_col21,
        trace_2_col22,
        trace_2_col23,
        trace_2_col24,
        trace_2_col25,
        trace_2_col26,
        trace_2_col27,
        trace_2_col28,
        trace_2_col29,
        trace_2_col30,
        trace_2_col31,
    ]: [Span<QM31>; 32] =
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
    let [trace_2_col12]: [QM31; 1] = (*trace_2_col12.try_into().unwrap()).unbox();
    let [trace_2_col13]: [QM31; 1] = (*trace_2_col13.try_into().unwrap()).unbox();
    let [trace_2_col14]: [QM31; 1] = (*trace_2_col14.try_into().unwrap()).unbox();
    let [trace_2_col15]: [QM31; 1] = (*trace_2_col15.try_into().unwrap()).unbox();
    let [trace_2_col16]: [QM31; 1] = (*trace_2_col16.try_into().unwrap()).unbox();
    let [trace_2_col17]: [QM31; 1] = (*trace_2_col17.try_into().unwrap()).unbox();
    let [trace_2_col18]: [QM31; 1] = (*trace_2_col18.try_into().unwrap()).unbox();
    let [trace_2_col19]: [QM31; 1] = (*trace_2_col19.try_into().unwrap()).unbox();
    let [trace_2_col20]: [QM31; 1] = (*trace_2_col20.try_into().unwrap()).unbox();
    let [trace_2_col21]: [QM31; 1] = (*trace_2_col21.try_into().unwrap()).unbox();
    let [trace_2_col22]: [QM31; 1] = (*trace_2_col22.try_into().unwrap()).unbox();
    let [trace_2_col23]: [QM31; 1] = (*trace_2_col23.try_into().unwrap()).unbox();
    let [trace_2_col24]: [QM31; 1] = (*trace_2_col24.try_into().unwrap()).unbox();
    let [trace_2_col25]: [QM31; 1] = (*trace_2_col25.try_into().unwrap()).unbox();
    let [trace_2_col26]: [QM31; 1] = (*trace_2_col26.try_into().unwrap()).unbox();
    let [trace_2_col27]: [QM31; 1] = (*trace_2_col27.try_into().unwrap()).unbox();
    let [trace_2_col28_neg1, trace_2_col28]: [QM31; 2] = (*trace_2_col28.try_into().unwrap())
        .unbox();
    let [trace_2_col29_neg1, trace_2_col29]: [QM31; 2] = (*trace_2_col29.try_into().unwrap())
        .unbox();
    let [trace_2_col30_neg1, trace_2_col30]: [QM31; 2] = (*trace_2_col30.try_into().unwrap())
        .unbox();
    let [trace_2_col31_neg1, trace_2_col31]: [QM31; 2] = (*trace_2_col31.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_9_9_sum_0
        * range_check_18_sum_1)
        - range_check_9_9_sum_0
        - range_check_18_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_18_sum_2
        * range_check_9_9_b_sum_3)
        - range_check_18_sum_2
        - range_check_9_9_b_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_18_b_sum_4
        * range_check_18_sum_5)
        - range_check_18_b_sum_4
        - range_check_18_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_9_9_c_sum_6
        * range_check_18_sum_7)
        - range_check_9_9_c_sum_6
        - range_check_18_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_18_sum_8
        * range_check_9_9_d_sum_9)
        - range_check_18_sum_8
        - range_check_9_9_d_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_18_b_sum_10
        * range_check_18_sum_11)
        - range_check_18_b_sum_10
        - range_check_18_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * range_check_9_9_e_sum_12
        * range_check_18_sum_13)
        - range_check_9_9_e_sum_12
        - range_check_18_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27])
        - QM31Impl::from_partial_evals(
            [trace_2_col28_neg1, trace_2_col29_neg1, trace_2_col30_neg1, trace_2_col31_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * range_check_felt_252_width_27_sum_14)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

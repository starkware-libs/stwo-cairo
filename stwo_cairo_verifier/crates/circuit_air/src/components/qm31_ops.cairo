// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 12;
pub const N_INTERACTION_COLUMNS: usize = 8;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 1] = [('Gate', 2)];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; N_INTERACTION_COLUMNS].span();
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
    pub common_lookup_elements: CommonLookupElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl AirComponentImpl of AirComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let log_size = *(self.claim.log_size);
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut gate_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut gate_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let mut gate_sum_2: QM31 = Zero::zero();
        let mut numerator_2: QM31 = Zero::zero();
        let add_flag = preprocessed_mask_values.get_and_mark_used(ADD_FLAG_IDX);
        let mul_flag = preprocessed_mask_values.get_and_mark_used(MUL_FLAG_IDX);
        let pointwise_mul_flag = preprocessed_mask_values.get_and_mark_used(POINTWISE_MUL_FLAG_IDX);
        let sub_flag = preprocessed_mask_values.get_and_mark_used(SUB_FLAG_IDX);
        let op0_addr = preprocessed_mask_values.get_and_mark_used(OP_0_ADDR_IDX);
        let op1_addr = preprocessed_mask_values.get_and_mark_used(OP_1_ADDR_IDX);
        let dst_addr = preprocessed_mask_values.get_and_mark_used(DST_ADDR_IDX);
        let qm31_ops_multiplicity = preprocessed_mask_values
            .get_and_mark_used(QM_31_OPS_MULTIPLICITY_IDX);

        let [
            input_op0_limb0_col0,
            input_op0_limb1_col1,
            input_op0_limb2_col2,
            input_op0_limb3_col3,
            input_op1_limb0_col4,
            input_op1_limb1_col5,
            input_op1_limb2_col6,
            input_op1_limb3_col7,
            input_dst_limb0_col8,
            input_dst_limb1_col9,
            input_dst_limb2_col10,
            input_dst_limb3_col11,
        ]: [Span<QM31>; 12] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_op0_limb0_col0]: [QM31; 1] = (*input_op0_limb0_col0.try_into().unwrap()).unbox();
        let [input_op0_limb1_col1]: [QM31; 1] = (*input_op0_limb1_col1.try_into().unwrap()).unbox();
        let [input_op0_limb2_col2]: [QM31; 1] = (*input_op0_limb2_col2.try_into().unwrap()).unbox();
        let [input_op0_limb3_col3]: [QM31; 1] = (*input_op0_limb3_col3.try_into().unwrap()).unbox();
        let [input_op1_limb0_col4]: [QM31; 1] = (*input_op1_limb0_col4.try_into().unwrap()).unbox();
        let [input_op1_limb1_col5]: [QM31; 1] = (*input_op1_limb1_col5.try_into().unwrap()).unbox();
        let [input_op1_limb2_col6]: [QM31; 1] = (*input_op1_limb2_col6.try_into().unwrap()).unbox();
        let [input_op1_limb3_col7]: [QM31; 1] = (*input_op1_limb3_col7.try_into().unwrap()).unbox();
        let [input_dst_limb0_col8]: [QM31; 1] = (*input_dst_limb0_col8.try_into().unwrap()).unbox();
        let [input_dst_limb1_col9]: [QM31; 1] = (*input_dst_limb1_col9.try_into().unwrap()).unbox();
        let [input_dst_limb2_col10]: [QM31; 1] = (*input_dst_limb2_col10.try_into().unwrap())
            .unbox();
        let [input_dst_limb3_col11]: [QM31; 1] = (*input_dst_limb3_col11.try_into().unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        // Constraint - all flags sum to 1
        let constraint_quotient = (((((add_flag + sub_flag) + mul_flag) + pointwise_mul_flag)
            - qm31_const::<1, 0, 0, 0>()));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - add_flag is a bit
        let constraint_quotient = ((add_flag * (add_flag - qm31_const::<1, 0, 0, 0>())));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - sub_flag is a bit
        let constraint_quotient = ((sub_flag * (sub_flag - qm31_const::<1, 0, 0, 0>())));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mul_flag is a bit
        let constraint_quotient = ((mul_flag * (mul_flag - qm31_const::<1, 0, 0, 0>())));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - pointwise_mul_flag is a bit
        let constraint_quotient = ((pointwise_mul_flag
            * (pointwise_mul_flag - qm31_const::<1, 0, 0, 0>())));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint -
        let constraint_quotient = ((input_dst_limb0_col8
            - (((((((((input_op0_limb0_col0 * input_op1_limb0_col4)
                - (input_op0_limb1_col1 * input_op1_limb1_col5))
                + (qm31_const::<2, 0, 0, 0>()
                    * ((input_op0_limb2_col2 * input_op1_limb2_col6)
                        - (input_op0_limb3_col3 * input_op1_limb3_col7))))
                - (input_op0_limb2_col2 * input_op1_limb3_col7))
                - (input_op0_limb3_col3 * input_op1_limb2_col6))
                * mul_flag)
                + ((input_op0_limb0_col0 + input_op1_limb0_col4) * add_flag))
                + ((input_op0_limb0_col0 - input_op1_limb0_col4) * sub_flag))
                + ((input_op0_limb0_col0 * input_op1_limb0_col4) * pointwise_mul_flag))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint -
        let constraint_quotient = ((input_dst_limb1_col9
            - (((((((((input_op0_limb0_col0 * input_op1_limb1_col5)
                + (input_op0_limb1_col1 * input_op1_limb0_col4))
                + (qm31_const::<2, 0, 0, 0>()
                    * ((input_op0_limb2_col2 * input_op1_limb3_col7)
                        + (input_op0_limb3_col3 * input_op1_limb2_col6))))
                + (input_op0_limb2_col2 * input_op1_limb2_col6))
                - (input_op0_limb3_col3 * input_op1_limb3_col7))
                * mul_flag)
                + ((input_op0_limb1_col1 + input_op1_limb1_col5) * add_flag))
                + ((input_op0_limb1_col1 - input_op1_limb1_col5) * sub_flag))
                + ((input_op0_limb1_col1 * input_op1_limb1_col5) * pointwise_mul_flag))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint -
        let constraint_quotient = ((input_dst_limb2_col10
            - ((((((((input_op0_limb0_col0 * input_op1_limb2_col6)
                - (input_op0_limb1_col1 * input_op1_limb3_col7))
                + (input_op0_limb2_col2 * input_op1_limb0_col4))
                - (input_op0_limb3_col3 * input_op1_limb1_col5))
                * mul_flag)
                + ((input_op0_limb2_col2 + input_op1_limb2_col6) * add_flag))
                + ((input_op0_limb2_col2 - input_op1_limb2_col6) * sub_flag))
                + ((input_op0_limb2_col2 * input_op1_limb2_col6) * pointwise_mul_flag))));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint -
        let constraint_quotient = ((input_dst_limb3_col11
            - ((((((((input_op0_limb0_col0 * input_op1_limb3_col7)
                + (input_op0_limb1_col1 * input_op1_limb2_col6))
                + (input_op0_limb2_col2 * input_op1_limb1_col5))
                + (input_op0_limb3_col3 * input_op1_limb0_col4))
                * mul_flag)
                + ((input_op0_limb3_col3 + input_op1_limb3_col7) * add_flag))
                + ((input_op0_limb3_col3 - input_op1_limb3_col7) * sub_flag))
                + ((input_op0_limb3_col3 * input_op1_limb3_col7) * pointwise_mul_flag))));
        sum = sum * random_coeff + constraint_quotient;

        gate_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), op0_addr, input_op0_limb0_col0,
                    input_op0_limb1_col1, input_op0_limb2_col2, input_op0_limb3_col3,
                ]
                    .span(),
            );
        numerator_0 = qm31_const::<1, 0, 0, 0>();

        gate_sum_1 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), op1_addr, input_op1_limb0_col4,
                    input_op1_limb1_col5, input_op1_limb2_col6, input_op1_limb3_col7,
                ]
                    .span(),
            );
        numerator_1 = qm31_const::<1, 0, 0, 0>();

        gate_sum_2 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), dst_addr, input_dst_limb0_col8,
                    input_dst_limb1_col9, input_dst_limb2_col10, input_dst_limb3_col11,
                ]
                    .span(),
            );
        numerator_2 = qm31_ops_multiplicity;

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            numerator_2,
            column_size,
            ref interaction_trace_mask_values,
            gate_sum_0,
            gate_sum_1,
            gate_sum_2,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    numerator_1: QM31,
    numerator_2: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    gate_sum_0: QM31,
    gate_sum_1: QM31,
    gate_sum_2: QM31,
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
    ]: [Span<QM31>; 8] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4_neg1, trace_2_col4]: [QM31; 2] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5_neg1, trace_2_col5]: [QM31; 2] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6_neg1, trace_2_col6]: [QM31; 2] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7_neg1, trace_2_col7]: [QM31; 2] = (*trace_2_col7.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * gate_sum_0
        * gate_sum_1)
        - (gate_sum_0 * numerator_1)
        - (gate_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
        - QM31Impl::from_partial_evals(
            [trace_2_col4_neg1, trace_2_col5_neg1, trace_2_col6_neg1, trace_2_col7_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * gate_sum_2)
        + numerator_2);
    sum = sum * random_coeff + constraint_quotient;
}

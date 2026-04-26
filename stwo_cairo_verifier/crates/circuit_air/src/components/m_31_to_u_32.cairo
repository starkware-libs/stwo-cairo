// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 4;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [('RangeCheck_16', 3), ('Gate', 1)];

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

pub impl CircuitComponentImpl of CircuitComponent<Component> {
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
        let mut range_check_16_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut range_check_16_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let mut range_check_16_sum_2: QM31 = Zero::zero();
        let mut numerator_2: QM31 = Zero::zero();
        let mut gate_sum_3: QM31 = Zero::zero();
        let mut numerator_3: QM31 = Zero::zero();
        let mut gate_sum_4: QM31 = Zero::zero();
        let mut numerator_4: QM31 = Zero::zero();
        let m31_to_u32_input_addr = preprocessed_mask_values
            .get_and_mark_used(M_31_TO_U_32_INPUT_ADDR_IDX);
        let m31_to_u32_output_addr = preprocessed_mask_values
            .get_and_mark_used(M_31_TO_U_32_OUTPUT_ADDR_IDX);
        let m31_to_u32_multiplicity = preprocessed_mask_values
            .get_and_mark_used(M_31_TO_U_32_MULTIPLICITY_IDX);

        let [
            input_m31_col0, input_u32_limb_0_col1, input_u32_limb_1_col2, inv_or_one_col3,
        ]: [Span<QM31>; 4] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_m31_col0]: [QM31; 1] = (*input_m31_col0.try_into().unwrap()).unbox();
        let [input_u32_limb_0_col1]: [QM31; 1] = (*input_u32_limb_0_col1.try_into().unwrap())
            .unbox();
        let [input_u32_limb_1_col2]: [QM31; 1] = (*input_u32_limb_1_col2.try_into().unwrap())
            .unbox();
        let [inv_or_one_col3]: [QM31; 1] = (*inv_or_one_col3.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        range_check_16_sum_0 = self
            .common_lookup_elements
            .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), input_u32_limb_0_col1].span());
        numerator_0 = qm31_const::<1, 0, 0, 0>();

        range_check_16_sum_1 = self
            .common_lookup_elements
            .combine_qm31([qm31_const::<1008385708, 0, 0, 0>(), input_u32_limb_1_col2].span());
        numerator_1 = qm31_const::<1, 0, 0, 0>();

        range_check_16_sum_2 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1008385708, 0, 0, 0>(),
                    (qm31_const::<32767, 0, 0, 0>() - input_u32_limb_1_col2),
                ]
                    .span(),
            );
        numerator_2 = qm31_const::<1, 0, 0, 0>();

        // Constraint - input is zero then limb_low is zero
        let constraint_quotient = ((((input_m31_col0 * inv_or_one_col3)
            - qm31_const::<1, 0, 0, 0>())
            * input_u32_limb_0_col1));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - input reconstruction
        let constraint_quotient = ((input_m31_col0
            - (input_u32_limb_0_col1 + (input_u32_limb_1_col2 * qm31_const::<65536, 0, 0, 0>()))));
        sum = sum * random_coeff + constraint_quotient;

        gate_sum_3 = self
            .common_lookup_elements
            .combine_qm31(
                [qm31_const::<378353459, 0, 0, 0>(), m31_to_u32_input_addr, input_m31_col0].span(),
            );
        numerator_3 = qm31_const::<1, 0, 0, 0>();

        gate_sum_4 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<378353459, 0, 0, 0>(), m31_to_u32_output_addr,
                    input_u32_limb_0_col1, input_u32_limb_1_col2,
                ]
                    .span(),
            );
        numerator_4 = m31_to_u32_multiplicity;

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            numerator_2,
            numerator_3,
            numerator_4,
            column_size,
            ref interaction_trace_mask_values,
            range_check_16_sum_0,
            range_check_16_sum_1,
            range_check_16_sum_2,
            gate_sum_3,
            gate_sum_4,
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
    numerator_3: QM31,
    numerator_4: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_16_sum_0: QM31,
    range_check_16_sum_1: QM31,
    range_check_16_sum_2: QM31,
    gate_sum_3: QM31,
    gate_sum_4: QM31,
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
    ]: [Span<QM31>; 12] =
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
    let [trace_2_col8_neg1, trace_2_col8]: [QM31; 2] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9_neg1, trace_2_col9]: [QM31; 2] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10_neg1, trace_2_col10]: [QM31; 2] = (*trace_2_col10.try_into().unwrap())
        .unbox();
    let [trace_2_col11_neg1, trace_2_col11]: [QM31; 2] = (*trace_2_col11.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_16_sum_0
        * range_check_16_sum_1)
        - (range_check_16_sum_0 * numerator_1)
        - (range_check_16_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_16_sum_2
        * gate_sum_3)
        - (range_check_16_sum_2 * numerator_3)
        - (gate_sum_3 * numerator_2));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
        - QM31Impl::from_partial_evals(
            [trace_2_col8_neg1, trace_2_col9_neg1, trace_2_col10_neg1, trace_2_col11_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * gate_sum_4)
        + numerator_4);
    sum = sum * random_coeff + constraint_quotient;
}

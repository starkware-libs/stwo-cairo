// AIR version 97774321-dirty
use crate::components::subroutines::read_id::read_id_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 6;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [
    ('MemoryAddressToId', 6), ('PoseidonAggregator', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 16].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.poseidon_builtin_segment_start).into());
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
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub poseidon_aggregator_lookup_elements: crate::PoseidonAggregatorElements,
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
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            poseidon_aggregator_lookup_elements: interaction_elements.poseidon_aggregator.clone(),
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
        preprocessed_column_set.insert(PreprocessedColumn::Seq(*(self.claim.log_size)));
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
        let poseidon_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.poseidon_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_3: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_4: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_5: QM31 = Zero::zero();
        let mut poseidon_aggregator_sum_6: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            state_0_id_col0,
            state_1_id_col1,
            state_2_id_col2,
            state_3_id_col3,
            state_4_id_col4,
            state_5_id_col5,
        ]: [Span<QM31>; 6] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [state_0_id_col0]: [QM31; 1] = (*state_0_id_col0.try_into().unwrap()).unbox();
        let [state_1_id_col1]: [QM31; 1] = (*state_1_id_col1.try_into().unwrap()).unbox();
        let [state_2_id_col2]: [QM31; 1] = (*state_2_id_col2.try_into().unwrap()).unbox();
        let [state_3_id_col3]: [QM31; 1] = (*state_3_id_col3.try_into().unwrap()).unbox();
        let [state_4_id_col4]: [QM31; 1] = (*state_4_id_col4.try_into().unwrap()).unbox();
        let [state_5_id_col5]: [QM31; 1] = (*state_5_id_col5.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        read_id_evaluate(
            (poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>())),
            state_0_id_col0,
            self.memory_address_to_id_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_id_evaluate(
            ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                + qm31_const::<1, 0, 0, 0>()),
            state_1_id_col1,
            self.memory_address_to_id_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_id_evaluate(
            ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                + qm31_const::<2, 0, 0, 0>()),
            state_2_id_col2,
            self.memory_address_to_id_lookup_elements,
            ref memory_address_to_id_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_id_evaluate(
            ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                + qm31_const::<3, 0, 0, 0>()),
            state_3_id_col3,
            self.memory_address_to_id_lookup_elements,
            ref memory_address_to_id_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_id_evaluate(
            ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                + qm31_const::<4, 0, 0, 0>()),
            state_4_id_col4,
            self.memory_address_to_id_lookup_elements,
            ref memory_address_to_id_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_id_evaluate(
            ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                + qm31_const::<5, 0, 0, 0>()),
            state_5_id_col5,
            self.memory_address_to_id_lookup_elements,
            ref memory_address_to_id_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        poseidon_aggregator_sum_6 = self
            .poseidon_aggregator_lookup_elements
            .combine_qm31(
                [
                    state_0_id_col0, state_1_id_col1, state_2_id_col2, state_3_id_col3,
                    state_4_id_col4, state_5_id_col5,
                ],
            );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_address_to_id_sum_1,
            memory_address_to_id_sum_2,
            memory_address_to_id_sum_3,
            memory_address_to_id_sum_4,
            memory_address_to_id_sum_5,
            poseidon_aggregator_sum_6,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_address_to_id_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_address_to_id_sum_3: QM31,
    memory_address_to_id_sum_4: QM31,
    memory_address_to_id_sum_5: QM31,
    poseidon_aggregator_sum_6: QM31,
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
        * memory_address_to_id_sum_0
        * memory_address_to_id_sum_1)
        - memory_address_to_id_sum_0
        - memory_address_to_id_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_address_to_id_sum_2
        * memory_address_to_id_sum_3)
        - memory_address_to_id_sum_2
        - memory_address_to_id_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * memory_address_to_id_sum_4
        * memory_address_to_id_sum_5)
        - memory_address_to_id_sum_4
        - memory_address_to_id_sum_5)
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
        * poseidon_aggregator_sum_6)
        - qm31_const::<1, 0, 0, 0>())
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

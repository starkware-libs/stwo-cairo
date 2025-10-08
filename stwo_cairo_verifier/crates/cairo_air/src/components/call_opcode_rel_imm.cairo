// AIR version 54d95c0d
use crate::components::subroutines::decode_instruction_2a7a2::decode_instruction_2a7a2_evaluate;
use crate::components::subroutines::read_positive_num_bits_29::read_positive_num_bits_29_evaluate;
use crate::components::subroutines::read_small::read_small_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 24;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 3), ('MemoryIdToBig', 3), ('Opcodes', 1),
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
        let interaction_log_sizes = [log_size; 20].span();
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
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub opcodes_lookup_elements: crate::OpcodesElements,
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
            verify_instruction_lookup_elements: interaction_elements.verify_instruction.clone(),
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            opcodes_lookup_elements: interaction_elements.opcodes.clone(),
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
        let mut verify_instruction_sum_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_3: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_4: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_5: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_6: QM31 = Zero::zero();
        let mut opcodes_sum_7: QM31 = Zero::zero();
        let mut opcodes_sum_8: QM31 = Zero::zero();

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            stored_fp_id_col3,
            stored_fp_limb_0_col4,
            stored_fp_limb_1_col5,
            stored_fp_limb_2_col6,
            stored_fp_limb_3_col7,
            partial_limb_msb_col8,
            stored_ret_pc_id_col9,
            stored_ret_pc_limb_0_col10,
            stored_ret_pc_limb_1_col11,
            stored_ret_pc_limb_2_col12,
            stored_ret_pc_limb_3_col13,
            partial_limb_msb_col14,
            distance_to_next_pc_id_col15,
            msb_col16,
            mid_limbs_set_col17,
            distance_to_next_pc_limb_0_col18,
            distance_to_next_pc_limb_1_col19,
            distance_to_next_pc_limb_2_col20,
            remainder_bits_col21,
            partial_limb_msb_col22,
            enabler,
        ]: [Span<QM31>; 24] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [stored_fp_id_col3]: [QM31; 1] = (*stored_fp_id_col3.try_into().unwrap()).unbox();
        let [stored_fp_limb_0_col4]: [QM31; 1] = (*stored_fp_limb_0_col4.try_into().unwrap())
            .unbox();
        let [stored_fp_limb_1_col5]: [QM31; 1] = (*stored_fp_limb_1_col5.try_into().unwrap())
            .unbox();
        let [stored_fp_limb_2_col6]: [QM31; 1] = (*stored_fp_limb_2_col6.try_into().unwrap())
            .unbox();
        let [stored_fp_limb_3_col7]: [QM31; 1] = (*stored_fp_limb_3_col7.try_into().unwrap())
            .unbox();
        let [partial_limb_msb_col8]: [QM31; 1] = (*partial_limb_msb_col8.try_into().unwrap())
            .unbox();
        let [stored_ret_pc_id_col9]: [QM31; 1] = (*stored_ret_pc_id_col9.try_into().unwrap())
            .unbox();
        let [stored_ret_pc_limb_0_col10]: [QM31; 1] = (*stored_ret_pc_limb_0_col10
            .try_into()
            .unwrap())
            .unbox();
        let [stored_ret_pc_limb_1_col11]: [QM31; 1] = (*stored_ret_pc_limb_1_col11
            .try_into()
            .unwrap())
            .unbox();
        let [stored_ret_pc_limb_2_col12]: [QM31; 1] = (*stored_ret_pc_limb_2_col12
            .try_into()
            .unwrap())
            .unbox();
        let [stored_ret_pc_limb_3_col13]: [QM31; 1] = (*stored_ret_pc_limb_3_col13
            .try_into()
            .unwrap())
            .unbox();
        let [partial_limb_msb_col14]: [QM31; 1] = (*partial_limb_msb_col14.try_into().unwrap())
            .unbox();
        let [distance_to_next_pc_id_col15]: [QM31; 1] = (*distance_to_next_pc_id_col15
            .try_into()
            .unwrap())
            .unbox();
        let [msb_col16]: [QM31; 1] = (*msb_col16.try_into().unwrap()).unbox();
        let [mid_limbs_set_col17]: [QM31; 1] = (*mid_limbs_set_col17.try_into().unwrap()).unbox();
        let [distance_to_next_pc_limb_0_col18]: [QM31; 1] = (*distance_to_next_pc_limb_0_col18
            .try_into()
            .unwrap())
            .unbox();
        let [distance_to_next_pc_limb_1_col19]: [QM31; 1] = (*distance_to_next_pc_limb_1_col19
            .try_into()
            .unwrap())
            .unbox();
        let [distance_to_next_pc_limb_2_col20]: [QM31; 1] = (*distance_to_next_pc_limb_2_col20
            .try_into()
            .unwrap())
            .unbox();
        let [remainder_bits_col21]: [QM31; 1] = (*remainder_bits_col21.try_into().unwrap()).unbox();
        let [partial_limb_msb_col22]: [QM31; 1] = (*partial_limb_msb_col22.try_into().unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        decode_instruction_2a7a2_evaluate(
            input_pc_col0,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        read_positive_num_bits_29_evaluate(
            input_ap_col1,
            stored_fp_id_col3,
            stored_fp_limb_0_col4,
            stored_fp_limb_1_col5,
            stored_fp_limb_2_col6,
            stored_fp_limb_3_col7,
            partial_limb_msb_col8,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - [ap] = fp
        let constraint_quotient = (((((stored_fp_limb_0_col4
            + (stored_fp_limb_1_col5 * qm31_const::<512, 0, 0, 0>()))
            + (stored_fp_limb_2_col6 * qm31_const::<262144, 0, 0, 0>()))
            + (stored_fp_limb_3_col7 * qm31_const::<134217728, 0, 0, 0>()))
            - input_fp_col2))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        read_positive_num_bits_29_evaluate(
            (input_ap_col1 + qm31_const::<1, 0, 0, 0>()),
            stored_ret_pc_id_col9,
            stored_ret_pc_limb_0_col10,
            stored_ret_pc_limb_1_col11,
            stored_ret_pc_limb_2_col12,
            stored_ret_pc_limb_3_col13,
            partial_limb_msb_col14,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - [ap+1] = return_pc
        let constraint_quotient = (((((stored_ret_pc_limb_0_col10
            + (stored_ret_pc_limb_1_col11 * qm31_const::<512, 0, 0, 0>()))
            + (stored_ret_pc_limb_2_col12 * qm31_const::<262144, 0, 0, 0>()))
            + (stored_ret_pc_limb_3_col13 * qm31_const::<134217728, 0, 0, 0>()))
            - (input_pc_col0 + qm31_const::<2, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let read_small_output_tmp_9db06_26_limb_0: QM31 = read_small_evaluate(
            (input_pc_col0 + qm31_const::<1, 0, 0, 0>()),
            distance_to_next_pc_id_col15,
            msb_col16,
            mid_limbs_set_col17,
            distance_to_next_pc_limb_0_col18,
            distance_to_next_pc_limb_1_col19,
            distance_to_next_pc_limb_2_col20,
            remainder_bits_col21,
            partial_limb_msb_col22,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        opcodes_sum_7 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_8 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    (input_pc_col0 + read_small_output_tmp_9db06_26_limb_0),
                    (input_ap_col1 + qm31_const::<2, 0, 0, 0>()),
                    (input_ap_col1 + qm31_const::<2, 0, 0, 0>()),
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
            verify_instruction_sum_0,
            memory_address_to_id_sum_1,
            memory_id_to_big_sum_2,
            memory_address_to_id_sum_3,
            memory_id_to_big_sum_4,
            memory_address_to_id_sum_5,
            memory_id_to_big_sum_6,
            opcodes_sum_7,
            opcodes_sum_8,
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
    verify_instruction_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
    memory_address_to_id_sum_3: QM31,
    memory_id_to_big_sum_4: QM31,
    memory_address_to_id_sum_5: QM31,
    memory_id_to_big_sum_6: QM31,
    opcodes_sum_7: QM31,
    opcodes_sum_8: QM31,
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
    ]: [Span<QM31>; 20] =
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
    let [trace_2_col16_neg1, trace_2_col16]: [QM31; 2] = (*trace_2_col16.try_into().unwrap())
        .unbox();
    let [trace_2_col17_neg1, trace_2_col17]: [QM31; 2] = (*trace_2_col17.try_into().unwrap())
        .unbox();
    let [trace_2_col18_neg1, trace_2_col18]: [QM31; 2] = (*trace_2_col18.try_into().unwrap())
        .unbox();
    let [trace_2_col19_neg1, trace_2_col19]: [QM31; 2] = (*trace_2_col19.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_instruction_sum_0
        * memory_address_to_id_sum_1)
        - verify_instruction_sum_0
        - memory_address_to_id_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_id_to_big_sum_2
        * memory_address_to_id_sum_3)
        - memory_id_to_big_sum_2
        - memory_address_to_id_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * memory_id_to_big_sum_4
        * memory_address_to_id_sum_5)
        - memory_id_to_big_sum_4
        - memory_address_to_id_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * memory_id_to_big_sum_6
        * opcodes_sum_7)
        - (memory_id_to_big_sum_6 * enabler)
        - opcodes_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
        - QM31Impl::from_partial_evals(
            [trace_2_col16_neg1, trace_2_col17_neg1, trace_2_col18_neg1, trace_2_col19_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_8)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

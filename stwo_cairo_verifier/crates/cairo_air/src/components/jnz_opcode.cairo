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
use crate::components::CairoComponent;
use crate::components::subroutines::decode_instruction_de75a::decode_instruction_de75a_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;

pub const N_TRACE_COLUMNS: usize = 37;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 1), ('MemoryIdToBig', 1), ('Opcodes', 1),
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
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 12].span();
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
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub opcodes_lookup_elements: crate::OpcodesElements,
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
        let mut opcodes_sum_3: QM31 = Zero::zero();
        let mut opcodes_sum_4: QM31 = Zero::zero();

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset0_col3,
            dst_base_fp_col4,
            ap_update_add_1_col5,
            mem_dst_base_col6,
            dst_id_col7,
            dst_limb_0_col8,
            dst_limb_1_col9,
            dst_limb_2_col10,
            dst_limb_3_col11,
            dst_limb_4_col12,
            dst_limb_5_col13,
            dst_limb_6_col14,
            dst_limb_7_col15,
            dst_limb_8_col16,
            dst_limb_9_col17,
            dst_limb_10_col18,
            dst_limb_11_col19,
            dst_limb_12_col20,
            dst_limb_13_col21,
            dst_limb_14_col22,
            dst_limb_15_col23,
            dst_limb_16_col24,
            dst_limb_17_col25,
            dst_limb_18_col26,
            dst_limb_19_col27,
            dst_limb_20_col28,
            dst_limb_21_col29,
            dst_limb_22_col30,
            dst_limb_23_col31,
            dst_limb_24_col32,
            dst_limb_25_col33,
            dst_limb_26_col34,
            dst_limb_27_col35,
            enabler,
        ]: [Span<QM31>; 37] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset0_col3]: [QM31; 1] = (*offset0_col3.try_into().unwrap()).unbox();
        let [dst_base_fp_col4]: [QM31; 1] = (*dst_base_fp_col4.try_into().unwrap()).unbox();
        let [ap_update_add_1_col5]: [QM31; 1] = (*ap_update_add_1_col5.try_into().unwrap()).unbox();
        let [mem_dst_base_col6]: [QM31; 1] = (*mem_dst_base_col6.try_into().unwrap()).unbox();
        let [dst_id_col7]: [QM31; 1] = (*dst_id_col7.try_into().unwrap()).unbox();
        let [dst_limb_0_col8]: [QM31; 1] = (*dst_limb_0_col8.try_into().unwrap()).unbox();
        let [dst_limb_1_col9]: [QM31; 1] = (*dst_limb_1_col9.try_into().unwrap()).unbox();
        let [dst_limb_2_col10]: [QM31; 1] = (*dst_limb_2_col10.try_into().unwrap()).unbox();
        let [dst_limb_3_col11]: [QM31; 1] = (*dst_limb_3_col11.try_into().unwrap()).unbox();
        let [dst_limb_4_col12]: [QM31; 1] = (*dst_limb_4_col12.try_into().unwrap()).unbox();
        let [dst_limb_5_col13]: [QM31; 1] = (*dst_limb_5_col13.try_into().unwrap()).unbox();
        let [dst_limb_6_col14]: [QM31; 1] = (*dst_limb_6_col14.try_into().unwrap()).unbox();
        let [dst_limb_7_col15]: [QM31; 1] = (*dst_limb_7_col15.try_into().unwrap()).unbox();
        let [dst_limb_8_col16]: [QM31; 1] = (*dst_limb_8_col16.try_into().unwrap()).unbox();
        let [dst_limb_9_col17]: [QM31; 1] = (*dst_limb_9_col17.try_into().unwrap()).unbox();
        let [dst_limb_10_col18]: [QM31; 1] = (*dst_limb_10_col18.try_into().unwrap()).unbox();
        let [dst_limb_11_col19]: [QM31; 1] = (*dst_limb_11_col19.try_into().unwrap()).unbox();
        let [dst_limb_12_col20]: [QM31; 1] = (*dst_limb_12_col20.try_into().unwrap()).unbox();
        let [dst_limb_13_col21]: [QM31; 1] = (*dst_limb_13_col21.try_into().unwrap()).unbox();
        let [dst_limb_14_col22]: [QM31; 1] = (*dst_limb_14_col22.try_into().unwrap()).unbox();
        let [dst_limb_15_col23]: [QM31; 1] = (*dst_limb_15_col23.try_into().unwrap()).unbox();
        let [dst_limb_16_col24]: [QM31; 1] = (*dst_limb_16_col24.try_into().unwrap()).unbox();
        let [dst_limb_17_col25]: [QM31; 1] = (*dst_limb_17_col25.try_into().unwrap()).unbox();
        let [dst_limb_18_col26]: [QM31; 1] = (*dst_limb_18_col26.try_into().unwrap()).unbox();
        let [dst_limb_19_col27]: [QM31; 1] = (*dst_limb_19_col27.try_into().unwrap()).unbox();
        let [dst_limb_20_col28]: [QM31; 1] = (*dst_limb_20_col28.try_into().unwrap()).unbox();
        let [dst_limb_21_col29]: [QM31; 1] = (*dst_limb_21_col29.try_into().unwrap()).unbox();
        let [dst_limb_22_col30]: [QM31; 1] = (*dst_limb_22_col30.try_into().unwrap()).unbox();
        let [dst_limb_23_col31]: [QM31; 1] = (*dst_limb_23_col31.try_into().unwrap()).unbox();
        let [dst_limb_24_col32]: [QM31; 1] = (*dst_limb_24_col32.try_into().unwrap()).unbox();
        let [dst_limb_25_col33]: [QM31; 1] = (*dst_limb_25_col33.try_into().unwrap()).unbox();
        let [dst_limb_26_col34]: [QM31; 1] = (*dst_limb_26_col34.try_into().unwrap()).unbox();
        let [dst_limb_27_col35]: [QM31; 1] = (*dst_limb_27_col35.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 1] = decode_instruction_de75a_evaluate(
            [input_pc_col0],
            offset0_col3,
            dst_base_fp_col4,
            ap_update_add_1_col5,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [decode_instruction_de75a_output_tmp_e1597_5_offset0] = output;

        // Constraint - mem_dst_base
        let constraint_quotient = ((mem_dst_base_col6
            - ((dst_base_fp_col4 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col4) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        read_positive_num_bits_252_evaluate(
            [(mem_dst_base_col6 + decode_instruction_de75a_output_tmp_e1597_5_offset0)],
            dst_id_col7,
            dst_limb_0_col8,
            dst_limb_1_col9,
            dst_limb_2_col10,
            dst_limb_3_col11,
            dst_limb_4_col12,
            dst_limb_5_col13,
            dst_limb_6_col14,
            dst_limb_7_col15,
            dst_limb_8_col16,
            dst_limb_9_col17,
            dst_limb_10_col18,
            dst_limb_11_col19,
            dst_limb_12_col20,
            dst_limb_13_col21,
            dst_limb_14_col22,
            dst_limb_15_col23,
            dst_limb_16_col24,
            dst_limb_17_col25,
            dst_limb_18_col26,
            dst_limb_19_col27,
            dst_limb_20_col28,
            dst_limb_21_col29,
            dst_limb_22_col30,
            dst_limb_23_col31,
            dst_limb_24_col32,
            dst_limb_25_col33,
            dst_limb_26_col34,
            dst_limb_27_col35,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - dst equals 0
        let constraint_quotient = ((((((((((((((((((((((((((((dst_limb_0_col8 + dst_limb_1_col9)
            + dst_limb_2_col10)
            + dst_limb_3_col11)
            + dst_limb_4_col12)
            + dst_limb_5_col13)
            + dst_limb_6_col14)
            + dst_limb_7_col15)
            + dst_limb_8_col16)
            + dst_limb_9_col17)
            + dst_limb_10_col18)
            + dst_limb_11_col19)
            + dst_limb_12_col20)
            + dst_limb_13_col21)
            + dst_limb_14_col22)
            + dst_limb_15_col23)
            + dst_limb_16_col24)
            + dst_limb_17_col25)
            + dst_limb_18_col26)
            + dst_limb_19_col27)
            + dst_limb_20_col28)
            + dst_limb_21_col29)
            + dst_limb_22_col30)
            + dst_limb_23_col31)
            + dst_limb_24_col32)
            + dst_limb_25_col33)
            + dst_limb_26_col34)
            + dst_limb_27_col35))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        opcodes_sum_3 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_4 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    (input_pc_col0 + qm31_const::<2, 0, 0, 0>()),
                    (input_ap_col1 + ap_update_add_1_col5), input_fp_col2,
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
            opcodes_sum_3,
            opcodes_sum_4,
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
    opcodes_sum_3: QM31,
    opcodes_sum_4: QM31,
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
        * opcodes_sum_3)
        - (memory_id_to_big_sum_2 * enabler)
        - opcodes_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
        - QM31Impl::from_partial_evals(
            [trace_2_col8_neg1, trace_2_col9_neg1, trace_2_col10_neg1, trace_2_col11_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_4)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

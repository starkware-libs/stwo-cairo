// This file was created by the AIR team.

use crate::components::subroutines::triple_sum_32::triple_sum_32_evaluate;
use crate::components::subroutines::xor_rot_32_r_12::xor_rot_32_r_12_evaluate;
use crate::components::subroutines::xor_rot_32_r_16::xor_rot_32_r_16_evaluate;
use crate::components::subroutines::xor_rot_32_r_7::xor_rot_32_r_7_evaluate;
use crate::components::subroutines::xor_rot_32_r_8::xor_rot_32_r_8_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 53;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 6] = [
    ('VerifyBitwiseXor_8', 4), ('VerifyBitwiseXor_8_B', 4), ('VerifyBitwiseXor_12', 2),
    ('VerifyBitwiseXor_4', 2), ('VerifyBitwiseXor_7', 2), ('VerifyBitwiseXor_9', 2),
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
        let interaction_log_sizes = [log_size; 36].span();
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
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_8_b_lookup_elements: crate::VerifyBitwiseXor_8_BElements,
    pub verify_bitwise_xor_12_lookup_elements: crate::VerifyBitwiseXor_12Elements,
    pub verify_bitwise_xor_4_lookup_elements: crate::VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7_lookup_elements: crate::VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_9_lookup_elements: crate::VerifyBitwiseXor_9Elements,
    pub blake_g_lookup_elements: crate::BlakeGElements,
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
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                .verify_bitwise_xor_8_b
                .clone(),
            verify_bitwise_xor_12_lookup_elements: interaction_elements
                .verify_bitwise_xor_12
                .clone(),
            verify_bitwise_xor_4_lookup_elements: interaction_elements.verify_bitwise_xor_4.clone(),
            verify_bitwise_xor_7_lookup_elements: interaction_elements.verify_bitwise_xor_7.clone(),
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
            blake_g_lookup_elements: interaction_elements.blake_g.clone(),
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
        let log_size = *(self.claim.log_size);
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut verify_bitwise_xor_8_sum_0: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_1: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_2: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_3: QM31 = Zero::zero();
        let mut verify_bitwise_xor_12_sum_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_4_sum_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_12_sum_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_4_sum_7: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_8: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_9: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_10: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_11: QM31 = Zero::zero();
        let mut verify_bitwise_xor_7_sum_12: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_13: QM31 = Zero::zero();
        let mut verify_bitwise_xor_7_sum_14: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_15: QM31 = Zero::zero();
        let mut blake_g_sum_16: QM31 = Zero::zero();

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
            input_limb_10_col10,
            input_limb_11_col11,
            triple_sum32_res_limb_0_col12,
            triple_sum32_res_limb_1_col13,
            ms_8_bits_col14,
            ms_8_bits_col15,
            ms_8_bits_col16,
            ms_8_bits_col17,
            xor_col18,
            xor_col19,
            xor_col20,
            xor_col21,
            triple_sum32_res_limb_0_col22,
            triple_sum32_res_limb_1_col23,
            ms_4_bits_col24,
            ms_4_bits_col25,
            ms_4_bits_col26,
            ms_4_bits_col27,
            xor_col28,
            xor_col29,
            xor_col30,
            xor_col31,
            triple_sum32_res_limb_0_col32,
            triple_sum32_res_limb_1_col33,
            ms_8_bits_col34,
            ms_8_bits_col35,
            ms_8_bits_col36,
            ms_8_bits_col37,
            xor_col38,
            xor_col39,
            xor_col40,
            xor_col41,
            triple_sum32_res_limb_0_col42,
            triple_sum32_res_limb_1_col43,
            ms_9_bits_col44,
            ms_9_bits_col45,
            ms_9_bits_col46,
            ms_9_bits_col47,
            xor_col48,
            xor_col49,
            xor_col50,
            xor_col51,
            enabler,
        ]: [Span<QM31>; 53] =
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
        let [input_limb_10_col10]: [QM31; 1] = (*input_limb_10_col10.try_into().unwrap()).unbox();
        let [input_limb_11_col11]: [QM31; 1] = (*input_limb_11_col11.try_into().unwrap()).unbox();
        let [triple_sum32_res_limb_0_col12]: [QM31; 1] = (*triple_sum32_res_limb_0_col12
            .try_into()
            .unwrap())
            .unbox();
        let [triple_sum32_res_limb_1_col13]: [QM31; 1] = (*triple_sum32_res_limb_1_col13
            .try_into()
            .unwrap())
            .unbox();
        let [ms_8_bits_col14]: [QM31; 1] = (*ms_8_bits_col14.try_into().unwrap()).unbox();
        let [ms_8_bits_col15]: [QM31; 1] = (*ms_8_bits_col15.try_into().unwrap()).unbox();
        let [ms_8_bits_col16]: [QM31; 1] = (*ms_8_bits_col16.try_into().unwrap()).unbox();
        let [ms_8_bits_col17]: [QM31; 1] = (*ms_8_bits_col17.try_into().unwrap()).unbox();
        let [xor_col18]: [QM31; 1] = (*xor_col18.try_into().unwrap()).unbox();
        let [xor_col19]: [QM31; 1] = (*xor_col19.try_into().unwrap()).unbox();
        let [xor_col20]: [QM31; 1] = (*xor_col20.try_into().unwrap()).unbox();
        let [xor_col21]: [QM31; 1] = (*xor_col21.try_into().unwrap()).unbox();
        let [triple_sum32_res_limb_0_col22]: [QM31; 1] = (*triple_sum32_res_limb_0_col22
            .try_into()
            .unwrap())
            .unbox();
        let [triple_sum32_res_limb_1_col23]: [QM31; 1] = (*triple_sum32_res_limb_1_col23
            .try_into()
            .unwrap())
            .unbox();
        let [ms_4_bits_col24]: [QM31; 1] = (*ms_4_bits_col24.try_into().unwrap()).unbox();
        let [ms_4_bits_col25]: [QM31; 1] = (*ms_4_bits_col25.try_into().unwrap()).unbox();
        let [ms_4_bits_col26]: [QM31; 1] = (*ms_4_bits_col26.try_into().unwrap()).unbox();
        let [ms_4_bits_col27]: [QM31; 1] = (*ms_4_bits_col27.try_into().unwrap()).unbox();
        let [xor_col28]: [QM31; 1] = (*xor_col28.try_into().unwrap()).unbox();
        let [xor_col29]: [QM31; 1] = (*xor_col29.try_into().unwrap()).unbox();
        let [xor_col30]: [QM31; 1] = (*xor_col30.try_into().unwrap()).unbox();
        let [xor_col31]: [QM31; 1] = (*xor_col31.try_into().unwrap()).unbox();
        let [triple_sum32_res_limb_0_col32]: [QM31; 1] = (*triple_sum32_res_limb_0_col32
            .try_into()
            .unwrap())
            .unbox();
        let [triple_sum32_res_limb_1_col33]: [QM31; 1] = (*triple_sum32_res_limb_1_col33
            .try_into()
            .unwrap())
            .unbox();
        let [ms_8_bits_col34]: [QM31; 1] = (*ms_8_bits_col34.try_into().unwrap()).unbox();
        let [ms_8_bits_col35]: [QM31; 1] = (*ms_8_bits_col35.try_into().unwrap()).unbox();
        let [ms_8_bits_col36]: [QM31; 1] = (*ms_8_bits_col36.try_into().unwrap()).unbox();
        let [ms_8_bits_col37]: [QM31; 1] = (*ms_8_bits_col37.try_into().unwrap()).unbox();
        let [xor_col38]: [QM31; 1] = (*xor_col38.try_into().unwrap()).unbox();
        let [xor_col39]: [QM31; 1] = (*xor_col39.try_into().unwrap()).unbox();
        let [xor_col40]: [QM31; 1] = (*xor_col40.try_into().unwrap()).unbox();
        let [xor_col41]: [QM31; 1] = (*xor_col41.try_into().unwrap()).unbox();
        let [triple_sum32_res_limb_0_col42]: [QM31; 1] = (*triple_sum32_res_limb_0_col42
            .try_into()
            .unwrap())
            .unbox();
        let [triple_sum32_res_limb_1_col43]: [QM31; 1] = (*triple_sum32_res_limb_1_col43
            .try_into()
            .unwrap())
            .unbox();
        let [ms_9_bits_col44]: [QM31; 1] = (*ms_9_bits_col44.try_into().unwrap()).unbox();
        let [ms_9_bits_col45]: [QM31; 1] = (*ms_9_bits_col45.try_into().unwrap()).unbox();
        let [ms_9_bits_col46]: [QM31; 1] = (*ms_9_bits_col46.try_into().unwrap()).unbox();
        let [ms_9_bits_col47]: [QM31; 1] = (*ms_9_bits_col47.try_into().unwrap()).unbox();
        let [xor_col48]: [QM31; 1] = (*xor_col48.try_into().unwrap()).unbox();
        let [xor_col49]: [QM31; 1] = (*xor_col49.try_into().unwrap()).unbox();
        let [xor_col50]: [QM31; 1] = (*xor_col50.try_into().unwrap()).unbox();
        let [xor_col51]: [QM31; 1] = (*xor_col51.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        triple_sum_32_evaluate(
            [
                input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                input_limb_8_col8, input_limb_9_col9,
            ],
            triple_sum32_res_limb_0_col12,
            triple_sum32_res_limb_1_col13,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            xor_rot_32_r_16_output_tmp_f72c8_21_limb_0, xor_rot_32_r_16_output_tmp_f72c8_21_limb_1,
        ] =
            xor_rot_32_r_16_evaluate(
            [
                triple_sum32_res_limb_0_col12, triple_sum32_res_limb_1_col13, input_limb_6_col6,
                input_limb_7_col7,
            ],
            ms_8_bits_col14,
            ms_8_bits_col15,
            ms_8_bits_col16,
            ms_8_bits_col17,
            xor_col18,
            xor_col19,
            xor_col20,
            xor_col21,
            self.verify_bitwise_xor_8_lookup_elements,
            self.verify_bitwise_xor_8_b_lookup_elements,
            ref verify_bitwise_xor_8_sum_0,
            ref verify_bitwise_xor_8_sum_1,
            ref verify_bitwise_xor_8_b_sum_2,
            ref verify_bitwise_xor_8_b_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        triple_sum_32_evaluate(
            [
                input_limb_4_col4, input_limb_5_col5, xor_rot_32_r_16_output_tmp_f72c8_21_limb_0,
                xor_rot_32_r_16_output_tmp_f72c8_21_limb_1, qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(),
            ],
            triple_sum32_res_limb_0_col22,
            triple_sum32_res_limb_1_col23,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            xor_rot_32_r_12_output_tmp_f72c8_43_limb_0, xor_rot_32_r_12_output_tmp_f72c8_43_limb_1,
        ] =
            xor_rot_32_r_12_evaluate(
            [
                input_limb_2_col2, input_limb_3_col3, triple_sum32_res_limb_0_col22,
                triple_sum32_res_limb_1_col23,
            ],
            ms_4_bits_col24,
            ms_4_bits_col25,
            ms_4_bits_col26,
            ms_4_bits_col27,
            xor_col28,
            xor_col29,
            xor_col30,
            xor_col31,
            self.verify_bitwise_xor_12_lookup_elements,
            self.verify_bitwise_xor_4_lookup_elements,
            ref verify_bitwise_xor_12_sum_4,
            ref verify_bitwise_xor_4_sum_5,
            ref verify_bitwise_xor_12_sum_6,
            ref verify_bitwise_xor_4_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        triple_sum_32_evaluate(
            [
                triple_sum32_res_limb_0_col12, triple_sum32_res_limb_1_col13,
                xor_rot_32_r_12_output_tmp_f72c8_43_limb_0,
                xor_rot_32_r_12_output_tmp_f72c8_43_limb_1, input_limb_10_col10,
                input_limb_11_col11,
            ],
            triple_sum32_res_limb_0_col32,
            triple_sum32_res_limb_1_col33,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [xor_rot_32_r_8_output_tmp_f72c8_65_limb_0, xor_rot_32_r_8_output_tmp_f72c8_65_limb_1] =
            xor_rot_32_r_8_evaluate(
            [
                triple_sum32_res_limb_0_col32, triple_sum32_res_limb_1_col33,
                xor_rot_32_r_16_output_tmp_f72c8_21_limb_0,
                xor_rot_32_r_16_output_tmp_f72c8_21_limb_1,
            ],
            ms_8_bits_col34,
            ms_8_bits_col35,
            ms_8_bits_col36,
            ms_8_bits_col37,
            xor_col38,
            xor_col39,
            xor_col40,
            xor_col41,
            self.verify_bitwise_xor_8_lookup_elements,
            self.verify_bitwise_xor_8_b_lookup_elements,
            ref verify_bitwise_xor_8_sum_8,
            ref verify_bitwise_xor_8_sum_9,
            ref verify_bitwise_xor_8_b_sum_10,
            ref verify_bitwise_xor_8_b_sum_11,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        triple_sum_32_evaluate(
            [
                triple_sum32_res_limb_0_col22, triple_sum32_res_limb_1_col23,
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_0,
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_1, qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(),
            ],
            triple_sum32_res_limb_0_col42,
            triple_sum32_res_limb_1_col43,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [xor_rot_32_r_7_output_tmp_f72c8_87_limb_0, xor_rot_32_r_7_output_tmp_f72c8_87_limb_1] =
            xor_rot_32_r_7_evaluate(
            [
                xor_rot_32_r_12_output_tmp_f72c8_43_limb_0,
                xor_rot_32_r_12_output_tmp_f72c8_43_limb_1, triple_sum32_res_limb_0_col42,
                triple_sum32_res_limb_1_col43,
            ],
            ms_9_bits_col44,
            ms_9_bits_col45,
            ms_9_bits_col46,
            ms_9_bits_col47,
            xor_col48,
            xor_col49,
            xor_col50,
            xor_col51,
            self.verify_bitwise_xor_7_lookup_elements,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_7_sum_12,
            ref verify_bitwise_xor_9_sum_13,
            ref verify_bitwise_xor_7_sum_14,
            ref verify_bitwise_xor_9_sum_15,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        blake_g_sum_16 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                    input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11,
                    triple_sum32_res_limb_0_col32, triple_sum32_res_limb_1_col33,
                    xor_rot_32_r_7_output_tmp_f72c8_87_limb_0,
                    xor_rot_32_r_7_output_tmp_f72c8_87_limb_1, triple_sum32_res_limb_0_col42,
                    triple_sum32_res_limb_1_col43, xor_rot_32_r_8_output_tmp_f72c8_65_limb_0,
                    xor_rot_32_r_8_output_tmp_f72c8_65_limb_1,
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
            verify_bitwise_xor_8_sum_0,
            verify_bitwise_xor_8_sum_1,
            verify_bitwise_xor_8_b_sum_2,
            verify_bitwise_xor_8_b_sum_3,
            verify_bitwise_xor_12_sum_4,
            verify_bitwise_xor_4_sum_5,
            verify_bitwise_xor_12_sum_6,
            verify_bitwise_xor_4_sum_7,
            verify_bitwise_xor_8_sum_8,
            verify_bitwise_xor_8_sum_9,
            verify_bitwise_xor_8_b_sum_10,
            verify_bitwise_xor_8_b_sum_11,
            verify_bitwise_xor_7_sum_12,
            verify_bitwise_xor_9_sum_13,
            verify_bitwise_xor_7_sum_14,
            verify_bitwise_xor_9_sum_15,
            blake_g_sum_16,
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
    verify_bitwise_xor_8_sum_0: QM31,
    verify_bitwise_xor_8_sum_1: QM31,
    verify_bitwise_xor_8_b_sum_2: QM31,
    verify_bitwise_xor_8_b_sum_3: QM31,
    verify_bitwise_xor_12_sum_4: QM31,
    verify_bitwise_xor_4_sum_5: QM31,
    verify_bitwise_xor_12_sum_6: QM31,
    verify_bitwise_xor_4_sum_7: QM31,
    verify_bitwise_xor_8_sum_8: QM31,
    verify_bitwise_xor_8_sum_9: QM31,
    verify_bitwise_xor_8_b_sum_10: QM31,
    verify_bitwise_xor_8_b_sum_11: QM31,
    verify_bitwise_xor_7_sum_12: QM31,
    verify_bitwise_xor_9_sum_13: QM31,
    verify_bitwise_xor_7_sum_14: QM31,
    verify_bitwise_xor_9_sum_15: QM31,
    blake_g_sum_16: QM31,
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
        trace_2_col32,
        trace_2_col33,
        trace_2_col34,
        trace_2_col35,
    ]: [Span<QM31>; 36] =
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
    let [trace_2_col28]: [QM31; 1] = (*trace_2_col28.try_into().unwrap()).unbox();
    let [trace_2_col29]: [QM31; 1] = (*trace_2_col29.try_into().unwrap()).unbox();
    let [trace_2_col30]: [QM31; 1] = (*trace_2_col30.try_into().unwrap()).unbox();
    let [trace_2_col31]: [QM31; 1] = (*trace_2_col31.try_into().unwrap()).unbox();
    let [trace_2_col32_neg1, trace_2_col32]: [QM31; 2] = (*trace_2_col32.try_into().unwrap())
        .unbox();
    let [trace_2_col33_neg1, trace_2_col33]: [QM31; 2] = (*trace_2_col33.try_into().unwrap())
        .unbox();
    let [trace_2_col34_neg1, trace_2_col34]: [QM31; 2] = (*trace_2_col34.try_into().unwrap())
        .unbox();
    let [trace_2_col35_neg1, trace_2_col35]: [QM31; 2] = (*trace_2_col35.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_bitwise_xor_8_sum_0
        * verify_bitwise_xor_8_sum_1)
        - verify_bitwise_xor_8_sum_0
        - verify_bitwise_xor_8_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * verify_bitwise_xor_8_b_sum_2
        * verify_bitwise_xor_8_b_sum_3)
        - verify_bitwise_xor_8_b_sum_2
        - verify_bitwise_xor_8_b_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * verify_bitwise_xor_12_sum_4
        * verify_bitwise_xor_4_sum_5)
        - verify_bitwise_xor_12_sum_4
        - verify_bitwise_xor_4_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_xor_12_sum_6
        * verify_bitwise_xor_4_sum_7)
        - verify_bitwise_xor_12_sum_6
        - verify_bitwise_xor_4_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * verify_bitwise_xor_8_sum_8
        * verify_bitwise_xor_8_sum_9)
        - verify_bitwise_xor_8_sum_8
        - verify_bitwise_xor_8_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * verify_bitwise_xor_8_b_sum_10
        * verify_bitwise_xor_8_b_sum_11)
        - verify_bitwise_xor_8_b_sum_10
        - verify_bitwise_xor_8_b_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * verify_bitwise_xor_7_sum_12
        * verify_bitwise_xor_9_sum_13)
        - verify_bitwise_xor_7_sum_12
        - verify_bitwise_xor_9_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * verify_bitwise_xor_7_sum_14
        * verify_bitwise_xor_9_sum_15)
        - verify_bitwise_xor_7_sum_14
        - verify_bitwise_xor_9_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31])
        - QM31Impl::from_partial_evals(
            [trace_2_col32_neg1, trace_2_col33_neg1, trace_2_col34_neg1, trace_2_col35_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * blake_g_sum_16)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::{NUM_PREPROCESSED_COLUMNS, seq_column_idx};
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElements, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::circle::CirclePoint;
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::cairo_component::*;
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, make_lookup_elements, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            blake_g_lookup_elements: make_lookup_elements(
                qm31_const::<1303027045, 1098741784, 1663692553, 948339060>(),
                qm31_const::<435770977, 566354259, 805606465, 2102625819>(),
            ),
            verify_bitwise_xor_12_lookup_elements: make_lookup_elements(
                qm31_const::<945317093, 1585289492, 1707952848, 1762242875>(),
                qm31_const::<1792777951, 1068271132, 1214640617, 746256740>(),
            ),
            verify_bitwise_xor_4_lookup_elements: make_lookup_elements(
                qm31_const::<1603259073, 105214626, 153538940, 1227631974>(),
                qm31_const::<1335982337, 626217582, 425804684, 1947714472>(),
            ),
            verify_bitwise_xor_7_lookup_elements: make_lookup_elements(
                qm31_const::<1849565511, 507903873, 354901595, 1227643995>(),
                qm31_const::<353740951, 600319398, 1603025159, 2094055458>(),
            ),
            verify_bitwise_xor_8_lookup_elements: make_lookup_elements(
                qm31_const::<390097169, 1715941348, 958959293, 1227669969>(),
                qm31_const::<105167513, 476596518, 1027059816, 1879697407>(),
            ),
            verify_bitwise_xor_8_b_lookup_elements: make_lookup_elements(
                qm31_const::<281609569, 2020003995, 58077116, 764105642>(),
                qm31_const::<797062783, 1701269078, 1114861254, 2119266818>(),
            ),
            verify_bitwise_xor_9_lookup_elements: make_lookup_elements(
                qm31_const::<974507519, 776396310, 1562918127, 1227662988>(),
                qm31_const::<1834779873, 2002531844, 159681682, 1478723240>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();
        let point = CirclePoint {
            x: qm31_const::<461666434, 38651694, 1083586041, 510305943>(),
            y: qm31_const::<817798294, 862569777, 2091320744, 1178484122>(),
        };

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };

        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
            [qm31_const::<1591990121, 771341002, 584090809, 1375009625>()].span(),
            [qm31_const::<1793317658, 1173994186, 785417401, 1375009625>()].span(),
            [qm31_const::<1726208479, 1039776458, 718308537, 1375009625>()].span(),
            [qm31_const::<1390662584, 368687818, 382764217, 1375009625>()].span(),
            [qm31_const::<1323553405, 234470090, 315655353, 1375009625>()].span(),
            [qm31_const::<1524880942, 637123274, 516981945, 1375009625>()].span(),
            [qm31_const::<1457771763, 502905546, 449873081, 1375009625>()].span(),
            [qm31_const::<48489085, 1979300555, 1188070585, 1375009625>()].span(),
            [qm31_const::<2128863553, 1845082826, 1120961721, 1375009625>()].span(),
            [qm31_const::<1852335767, 645078115, 2059236183, 343880121>()].span(),
            [qm31_const::<1919444946, 779295843, 2126345047, 343880121>()].span(),
            [qm31_const::<1986554125, 913513571, 45970264, 343880122>()].span(),
            [qm31_const::<2053663304, 1047731299, 113079128, 343880122>()].span(),
            [qm31_const::<1583899051, 108207203, 1790800727, 343880121>()].span(),
            [qm31_const::<1651008230, 242424931, 1857909591, 343880121>()].span(),
            [qm31_const::<1718117409, 376642659, 1925018455, 343880121>()].span(),
            [qm31_const::<1785226588, 510860387, 1992127319, 343880121>()].span(),
            [qm31_const::<1315462335, 1718819938, 1522365270, 343880121>()].span(),
            [qm31_const::<1382571514, 1853037666, 1589474134, 343880121>()].span(),
            [qm31_const::<1986820986, 913513739, 45970432, 343880178>()].span(),
            [qm31_const::<1919711807, 779296011, 2126345215, 343880177>()].span(),
            [qm31_const::<2121039344, 1181949195, 180188160, 343880178>()].span(),
            [qm31_const::<2053930165, 1047731467, 113079296, 343880178>()].span(),
            [qm31_const::<1718384270, 376642827, 1925018623, 343880177>()].span(),
            [qm31_const::<1651275091, 242425099, 1857909759, 343880177>()].span(),
            [qm31_const::<1852602628, 645078283, 2059236351, 343880177>()].span(),
            [qm31_const::<1785493449, 510860555, 1992127487, 343880177>()].span(),
            [qm31_const::<1449947554, 1987255562, 1656583166, 343880177>()].span(),
            [qm31_const::<1382838375, 1853037834, 1589474302, 343880177>()].span(),
            [qm31_const::<510356977, 108207322, 717059022, 343880161>()].span(),
            [qm31_const::<577466156, 242425050, 784167886, 343880161>()].span(),
            [qm31_const::<376138619, 1987255513, 582841293, 343880161>()].span(),
            [qm31_const::<443247798, 2121473241, 649950157, 343880161>()].span(),
            [qm31_const::<778793693, 645078234, 985494478, 343880161>()].span(),
            [qm31_const::<845902872, 779295962, 1052603342, 343880161>()].span(),
            [qm31_const::<644575335, 376642778, 851276750, 343880161>()].span(),
            [qm31_const::<711684514, 510860506, 918385614, 343880161>()].span(),
            [qm31_const::<1047230409, 1181949146, 1253929934, 343880161>()].span(),
            [qm31_const::<1114339588, 1316166874, 1321038798, 343880161>()].span(),
            [qm31_const::<1717810224, 376642479, 1925018275, 343880061>()].span(),
            [qm31_const::<1650701045, 242424751, 1857909411, 343880061>()].span(),
            [qm31_const::<1583591866, 108207023, 1790800547, 343880061>()].span(),
            [qm31_const::<1516482687, 2121472942, 1723691682, 343880061>()].span(),
            [qm31_const::<1986246940, 913513391, 45970084, 343880062>()].span(),
            [qm31_const::<1919137761, 779295663, 2126344867, 343880061>()].span(),
            [qm31_const::<1852028582, 645077935, 2059236003, 343880061>()].span(),
            [qm31_const::<1784919403, 510860207, 1992127139, 343880061>()].span(),
            [qm31_const::<1180936792, 1450384302, 1388147362, 343880061>()].span(),
            [qm31_const::<1113827613, 1316166574, 1321038498, 343880061>()].span(),
            [qm31_const::<241305891, 1718819697, 448623205, 343880041>()].span(),
            [qm31_const::<308415070, 1853037425, 515732069, 343880041>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
            qm31_const::<803840495, 1824811459, 1646561508, 1941984119>(),
            qm31_const::<870949674, 1959029187, 1713670372, 1941984119>(),
            qm31_const::<938058853, 2093246915, 1780779236, 1941984119>(),
            qm31_const::<1542041464, 1153722820, 237275366, 1941984120>(),
        ];
        let mut interaction_columns = make_interaction_trace(
            interaction_values, qm31_const::<1115374022, 1127856551, 489657863, 643630026>(),
        );
        component
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_trace,
                ref trace_columns,
                ref interaction_columns,
                qm31_const::<474642921, 876336632, 1911695779, 974600512>(),
                point,
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(BLAKE_G_SAMPLE_EVAL_RESULT))
    }
}

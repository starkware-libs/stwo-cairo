// AIR version aca38612
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
use crate::PreprocessedColumnTrait;
use crate::cairo_component::CairoComponent;
use crate::components::subroutines::triple_sum_32::triple_sum_32_evaluate;
use crate::components::subroutines::xor_rot_32_r_12::xor_rot_32_r_12_evaluate;
use crate::components::subroutines::xor_rot_32_r_16::xor_rot_32_r_16_evaluate;
use crate::components::subroutines::xor_rot_32_r_7::xor_rot_32_r_7_evaluate;
use crate::components::subroutines::xor_rot_32_r_8::xor_rot_32_r_8_evaluate;

pub const N_TRACE_COLUMNS: usize = 53;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 5] = [
    ('VerifyBitwiseXor_8', 8), ('VerifyBitwiseXor_12', 2), ('VerifyBitwiseXor_4', 2),
    ('VerifyBitwiseXor_7', 2), ('VerifyBitwiseXor_9', 2),
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
        let interaction_log_sizes = [log_size; 36].span();
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
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_12_lookup_elements: crate::VerifyBitwiseXor_12Elements,
    pub verify_bitwise_xor_4_lookup_elements: crate::VerifyBitwiseXor_4Elements,
    pub verify_bitwise_xor_7_lookup_elements: crate::VerifyBitwiseXor_7Elements,
    pub verify_bitwise_xor_9_lookup_elements: crate::VerifyBitwiseXor_9Elements,
    pub blake_g_lookup_elements: crate::BlakeGElements,
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
        let mut verify_bitwise_xor_8_sum_0: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_1: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_2: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_3: QM31 = Zero::zero();
        let mut verify_bitwise_xor_12_sum_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_4_sum_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_12_sum_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_4_sum_7: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_8: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_9: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_10: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_11: QM31 = Zero::zero();
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

        let output: [QM31; 2] = xor_rot_32_r_16_evaluate(
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
            ref verify_bitwise_xor_8_sum_0,
            ref verify_bitwise_xor_8_sum_1,
            ref verify_bitwise_xor_8_sum_2,
            ref verify_bitwise_xor_8_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            xor_rot_32_r_16_output_tmp_f72c8_21_limb_0, xor_rot_32_r_16_output_tmp_f72c8_21_limb_1,
        ] =
            output;

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

        let output: [QM31; 2] = xor_rot_32_r_12_evaluate(
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
        let [
            xor_rot_32_r_12_output_tmp_f72c8_43_limb_0, xor_rot_32_r_12_output_tmp_f72c8_43_limb_1,
        ] =
            output;

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

        let output: [QM31; 2] = xor_rot_32_r_8_evaluate(
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
            ref verify_bitwise_xor_8_sum_8,
            ref verify_bitwise_xor_8_sum_9,
            ref verify_bitwise_xor_8_sum_10,
            ref verify_bitwise_xor_8_sum_11,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [xor_rot_32_r_8_output_tmp_f72c8_65_limb_0, xor_rot_32_r_8_output_tmp_f72c8_65_limb_1] =
            output;

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

        let output: [QM31; 2] = xor_rot_32_r_7_evaluate(
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
        let [xor_rot_32_r_7_output_tmp_f72c8_87_limb_0, xor_rot_32_r_7_output_tmp_f72c8_87_limb_1] =
            output;

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
            verify_bitwise_xor_8_sum_2,
            verify_bitwise_xor_8_sum_3,
            verify_bitwise_xor_12_sum_4,
            verify_bitwise_xor_4_sum_5,
            verify_bitwise_xor_12_sum_6,
            verify_bitwise_xor_4_sum_7,
            verify_bitwise_xor_8_sum_8,
            verify_bitwise_xor_8_sum_9,
            verify_bitwise_xor_8_sum_10,
            verify_bitwise_xor_8_sum_11,
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
    verify_bitwise_xor_8_sum_2: QM31,
    verify_bitwise_xor_8_sum_3: QM31,
    verify_bitwise_xor_12_sum_4: QM31,
    verify_bitwise_xor_4_sum_5: QM31,
    verify_bitwise_xor_12_sum_6: QM31,
    verify_bitwise_xor_4_sum_7: QM31,
    verify_bitwise_xor_8_sum_8: QM31,
    verify_bitwise_xor_8_sum_9: QM31,
    verify_bitwise_xor_8_sum_10: QM31,
    verify_bitwise_xor_8_sum_11: QM31,
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
        * verify_bitwise_xor_8_sum_2
        * verify_bitwise_xor_8_sum_3)
        - verify_bitwise_xor_8_sum_2
        - verify_bitwise_xor_8_sum_3)
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
        * verify_bitwise_xor_8_sum_10
        * verify_bitwise_xor_8_sum_11)
        - verify_bitwise_xor_8_sum_10
        - verify_bitwise_xor_8_sum_11)
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

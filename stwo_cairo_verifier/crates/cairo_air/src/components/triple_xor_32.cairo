use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::components::verify_bitwise_xor_8::{
    VERIFY_BITWISE_XOR_8_RELATION_SIZE, verify_bitwise_xor_8_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 21;
pub const TRIPLE_XOR_32_RELATION_SIZE: usize = 8;


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
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 5, log_size)
            .span();
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
    pub triple_xor_32_lookup_elements: crate::TripleXor32Elements,
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
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
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
        let mut verify_bitwise_xor_8_sum_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_7: QM31 = Zero::zero();
        let mut triple_xor_32_sum_8: QM31 = Zero::zero();
        let verify_bitwise_xor_8_alphas = self
            .verify_bitwise_xor_8_lookup_elements
            .alpha_powers
            .span();
        let verify_bitwise_xor_8_z = *self.verify_bitwise_xor_8_lookup_elements.z;
        let triple_xor_32_alphas = self.triple_xor_32_lookup_elements.alpha_powers.span();
        let triple_xor_32_z = *self.triple_xor_32_lookup_elements.z;

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            input_limb_2_col2,
            input_limb_3_col3,
            input_limb_4_col4,
            input_limb_5_col5,
            ms_8_bits_col6,
            ms_8_bits_col7,
            ms_8_bits_col8,
            ms_8_bits_col9,
            ms_8_bits_col10,
            ms_8_bits_col11,
            xor_col12,
            xor_col13,
            xor_col14,
            xor_col15,
            xor_col16,
            xor_col17,
            xor_col18,
            xor_col19,
            enabler,
        ]: [Span<QM31>; 21] =
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
        let [ms_8_bits_col6]: [QM31; 1] = (*ms_8_bits_col6.try_into().unwrap()).unbox();
        let [ms_8_bits_col7]: [QM31; 1] = (*ms_8_bits_col7.try_into().unwrap()).unbox();
        let [ms_8_bits_col8]: [QM31; 1] = (*ms_8_bits_col8.try_into().unwrap()).unbox();
        let [ms_8_bits_col9]: [QM31; 1] = (*ms_8_bits_col9.try_into().unwrap()).unbox();
        let [ms_8_bits_col10]: [QM31; 1] = (*ms_8_bits_col10.try_into().unwrap()).unbox();
        let [ms_8_bits_col11]: [QM31; 1] = (*ms_8_bits_col11.try_into().unwrap()).unbox();
        let [xor_col12]: [QM31; 1] = (*xor_col12.try_into().unwrap()).unbox();
        let [xor_col13]: [QM31; 1] = (*xor_col13.try_into().unwrap()).unbox();
        let [xor_col14]: [QM31; 1] = (*xor_col14.try_into().unwrap()).unbox();
        let [xor_col15]: [QM31; 1] = (*xor_col15.try_into().unwrap()).unbox();
        let [xor_col16]: [QM31; 1] = (*xor_col16.try_into().unwrap()).unbox();
        let [xor_col17]: [QM31; 1] = (*xor_col17.try_into().unwrap()).unbox();
        let [xor_col18]: [QM31; 1] = (*xor_col18.try_into().unwrap()).unbox();
        let [xor_col19]: [QM31; 1] = (*xor_col19.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 2] = split_16_low_part_size_8_evaluate(
            [input_limb_0_col0], ms_8_bits_col6, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let [
            split_16_low_part_size_8_output_tmp_298db_1_limb_0,
            split_16_low_part_size_8_output_tmp_298db_1_limb_1,
        ] =
            output;

        let output: [QM31; 2] = split_16_low_part_size_8_evaluate(
            [input_limb_1_col1], ms_8_bits_col7, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let [
            split_16_low_part_size_8_output_tmp_298db_3_limb_0,
            split_16_low_part_size_8_output_tmp_298db_3_limb_1,
        ] =
            output;

        let output: [QM31; 2] = split_16_low_part_size_8_evaluate(
            [input_limb_2_col2], ms_8_bits_col8, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let [
            split_16_low_part_size_8_output_tmp_298db_5_limb_0,
            split_16_low_part_size_8_output_tmp_298db_5_limb_1,
        ] =
            output;

        let output: [QM31; 2] = split_16_low_part_size_8_evaluate(
            [input_limb_3_col3], ms_8_bits_col9, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let [
            split_16_low_part_size_8_output_tmp_298db_7_limb_0,
            split_16_low_part_size_8_output_tmp_298db_7_limb_1,
        ] =
            output;

        let output: [QM31; 2] = split_16_low_part_size_8_evaluate(
            [input_limb_4_col4], ms_8_bits_col10, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let [
            split_16_low_part_size_8_output_tmp_298db_9_limb_0,
            split_16_low_part_size_8_output_tmp_298db_9_limb_1,
        ] =
            output;

        let output: [QM31; 2] = split_16_low_part_size_8_evaluate(
            [input_limb_5_col5], ms_8_bits_col11, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let [
            split_16_low_part_size_8_output_tmp_298db_11_limb_0,
            split_16_low_part_size_8_output_tmp_298db_11_limb_1,
        ] =
            output;
        let bitwise_xor_num_bits_8_output_tmp_298db_13: QM31 = bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_298db_1_limb_0,
                split_16_low_part_size_8_output_tmp_298db_5_limb_0,
            ],
            xor_col12,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_15: QM31 = bitwise_xor_num_bits_8_evaluate(
            [xor_col12, split_16_low_part_size_8_output_tmp_298db_9_limb_0],
            xor_col13,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_17: QM31 = bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col6, ms_8_bits_col8],
            xor_col14,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_19: QM31 = bitwise_xor_num_bits_8_evaluate(
            [xor_col14, ms_8_bits_col10],
            xor_col15,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_21: QM31 = bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_298db_3_limb_0,
                split_16_low_part_size_8_output_tmp_298db_7_limb_0,
            ],
            xor_col16,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_23: QM31 = bitwise_xor_num_bits_8_evaluate(
            [xor_col16, split_16_low_part_size_8_output_tmp_298db_11_limb_0],
            xor_col17,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_25: QM31 = bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col7, ms_8_bits_col9],
            xor_col18,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let bitwise_xor_num_bits_8_output_tmp_298db_27: QM31 = bitwise_xor_num_bits_8_evaluate(
            [xor_col18, ms_8_bits_col11],
            xor_col19,
            verify_bitwise_xor_8_alphas,
            verify_bitwise_xor_8_z,
            ref verify_bitwise_xor_8_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let triple_xor32_output_tmp_298db_28_limb_0: QM31 = (xor_col13
            + (xor_col15 * qm31_const::<256, 0, 0, 0>()));
        let triple_xor32_output_tmp_298db_28_limb_1: QM31 = (xor_col17
            + (xor_col19 * qm31_const::<256, 0, 0, 0>()));

        triple_xor_32_sum_8 =
            triple_xor_32_sum(
                triple_xor_32_alphas,
                triple_xor_32_z,
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, triple_xor32_output_tmp_298db_28_limb_0,
                    triple_xor32_output_tmp_298db_28_limb_1,
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
            verify_bitwise_xor_8_sum_4,
            verify_bitwise_xor_8_sum_5,
            verify_bitwise_xor_8_sum_6,
            verify_bitwise_xor_8_sum_7,
            triple_xor_32_sum_8,
        );
    }
}


pub fn triple_xor_32_sum(mut alphas: Span<QM31>, z: QM31, values: [QM31; 8]) -> QM31 {
    let [alpha0, alpha1, alpha2, alpha3, alpha4, alpha5, alpha6, alpha7] = (*alphas
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [val0, val1, val2, val3, val4, val5, val6, val7] = values;

    alpha0 * val0
        + alpha1 * val1
        + alpha2 * val2
        + alpha3 * val3
        + alpha4 * val4
        + alpha5 * val5
        + alpha6 * val6
        + alpha7 * val7
        - z
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
    verify_bitwise_xor_8_sum_4: QM31,
    verify_bitwise_xor_8_sum_5: QM31,
    verify_bitwise_xor_8_sum_6: QM31,
    verify_bitwise_xor_8_sum_7: QM31,
    triple_xor_32_sum_8: QM31,
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
        * verify_bitwise_xor_8_sum_4
        * verify_bitwise_xor_8_sum_5)
        - verify_bitwise_xor_8_sum_4
        - verify_bitwise_xor_8_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_xor_8_sum_6
        * verify_bitwise_xor_8_sum_7)
        - verify_bitwise_xor_8_sum_6
        - verify_bitwise_xor_8_sum_7)
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
        * triple_xor_32_sum_8)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

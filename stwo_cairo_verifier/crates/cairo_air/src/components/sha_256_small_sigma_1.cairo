// AIR version 96541c91-dirty
use crate::components::subroutines::bitwise_and_num_bits_8::bitwise_and_num_bits_8_evaluate;
use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 43;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('VerifyBitwiseAnd_8', 12), ('Sha256SmallSigma1O0', 1), ('Sha256SmallSigma1O1', 1),
    ('VerifyBitwiseXor_8', 4),
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
        let interaction_log_sizes = [log_size; 40].span();
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
    pub verify_bitwise_and_8_lookup_elements: crate::VerifyBitwiseAnd_8Elements,
    pub sha_256_small_sigma_1_o_0_lookup_elements: crate::Sha256SmallSigma1O0Elements,
    pub sha_256_small_sigma_1_o_1_lookup_elements: crate::Sha256SmallSigma1O1Elements,
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub sha_256_small_sigma_1_lookup_elements: crate::Sha256SmallSigma1Elements,
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
            verify_bitwise_and_8_lookup_elements: interaction_elements.verify_bitwise_and_8.clone(),
            sha_256_small_sigma_1_o_0_lookup_elements: interaction_elements
                .sha_256_small_sigma_1_o_0
                .clone(),
            sha_256_small_sigma_1_o_1_lookup_elements: interaction_elements
                .sha_256_small_sigma_1_o_1
                .clone(),
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            sha_256_small_sigma_1_lookup_elements: interaction_elements
                .sha_256_small_sigma_1
                .clone(),
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
        let mut verify_bitwise_and_8_sum_0: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_1: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_2: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_3: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_4: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_5: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_6: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_7: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_8: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_9: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_10: QM31 = Zero::zero();
        let mut verify_bitwise_and_8_sum_11: QM31 = Zero::zero();
        let mut sha_256_small_sigma_1_o_0_sum_12: QM31 = Zero::zero();
        let mut sha_256_small_sigma_1_o_1_sum_13: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_14: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_15: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_16: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_17: QM31 = Zero::zero();
        let mut sha_256_small_sigma_1_sum_18: QM31 = Zero::zero();

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            ms_8_bits_col2,
            ms_8_bits_col3,
            and_col4,
            and_col5,
            l0_col6,
            and_col7,
            and_col8,
            l1_col9,
            and_col10,
            and_col11,
            l2_col12,
            and_col13,
            and_col14,
            h0_col15,
            and_col16,
            and_col17,
            h1_col18,
            and_col19,
            and_col20,
            h2_col21,
            sigma_O0_L_col22,
            sigma_O0_H_col23,
            sigma_O1_L_col24,
            sigma_O1_H_col25,
            sigma_O2_L_col26,
            sigma_O2_H_col27,
            sigma_O2_prime_L_col28,
            sigma_O2_prime_H_col29,
            ms_8_bits_col30,
            ms_8_bits_col31,
            xor_col32,
            xor_col33,
            output2l_col34,
            ms_8_bits_col35,
            ms_8_bits_col36,
            xor_col37,
            xor_col38,
            output2h_col39,
            output_low_col40,
            output_high_col41,
            enabler,
        ]: [Span<QM31>; 43] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();
        let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();
        let [ms_8_bits_col2]: [QM31; 1] = (*ms_8_bits_col2.try_into().unwrap()).unbox();
        let [ms_8_bits_col3]: [QM31; 1] = (*ms_8_bits_col3.try_into().unwrap()).unbox();
        let [and_col4]: [QM31; 1] = (*and_col4.try_into().unwrap()).unbox();
        let [and_col5]: [QM31; 1] = (*and_col5.try_into().unwrap()).unbox();
        let [l0_col6]: [QM31; 1] = (*l0_col6.try_into().unwrap()).unbox();
        let [and_col7]: [QM31; 1] = (*and_col7.try_into().unwrap()).unbox();
        let [and_col8]: [QM31; 1] = (*and_col8.try_into().unwrap()).unbox();
        let [l1_col9]: [QM31; 1] = (*l1_col9.try_into().unwrap()).unbox();
        let [and_col10]: [QM31; 1] = (*and_col10.try_into().unwrap()).unbox();
        let [and_col11]: [QM31; 1] = (*and_col11.try_into().unwrap()).unbox();
        let [l2_col12]: [QM31; 1] = (*l2_col12.try_into().unwrap()).unbox();
        let [and_col13]: [QM31; 1] = (*and_col13.try_into().unwrap()).unbox();
        let [and_col14]: [QM31; 1] = (*and_col14.try_into().unwrap()).unbox();
        let [h0_col15]: [QM31; 1] = (*h0_col15.try_into().unwrap()).unbox();
        let [and_col16]: [QM31; 1] = (*and_col16.try_into().unwrap()).unbox();
        let [and_col17]: [QM31; 1] = (*and_col17.try_into().unwrap()).unbox();
        let [h1_col18]: [QM31; 1] = (*h1_col18.try_into().unwrap()).unbox();
        let [and_col19]: [QM31; 1] = (*and_col19.try_into().unwrap()).unbox();
        let [and_col20]: [QM31; 1] = (*and_col20.try_into().unwrap()).unbox();
        let [h2_col21]: [QM31; 1] = (*h2_col21.try_into().unwrap()).unbox();
        let [sigma_O0_L_col22]: [QM31; 1] = (*sigma_O0_L_col22.try_into().unwrap()).unbox();
        let [sigma_O0_H_col23]: [QM31; 1] = (*sigma_O0_H_col23.try_into().unwrap()).unbox();
        let [sigma_O1_L_col24]: [QM31; 1] = (*sigma_O1_L_col24.try_into().unwrap()).unbox();
        let [sigma_O1_H_col25]: [QM31; 1] = (*sigma_O1_H_col25.try_into().unwrap()).unbox();
        let [sigma_O2_L_col26]: [QM31; 1] = (*sigma_O2_L_col26.try_into().unwrap()).unbox();
        let [sigma_O2_H_col27]: [QM31; 1] = (*sigma_O2_H_col27.try_into().unwrap()).unbox();
        let [sigma_O2_prime_L_col28]: [QM31; 1] = (*sigma_O2_prime_L_col28.try_into().unwrap())
            .unbox();
        let [sigma_O2_prime_H_col29]: [QM31; 1] = (*sigma_O2_prime_H_col29.try_into().unwrap())
            .unbox();
        let [ms_8_bits_col30]: [QM31; 1] = (*ms_8_bits_col30.try_into().unwrap()).unbox();
        let [ms_8_bits_col31]: [QM31; 1] = (*ms_8_bits_col31.try_into().unwrap()).unbox();
        let [xor_col32]: [QM31; 1] = (*xor_col32.try_into().unwrap()).unbox();
        let [xor_col33]: [QM31; 1] = (*xor_col33.try_into().unwrap()).unbox();
        let [output2l_col34]: [QM31; 1] = (*output2l_col34.try_into().unwrap()).unbox();
        let [ms_8_bits_col35]: [QM31; 1] = (*ms_8_bits_col35.try_into().unwrap()).unbox();
        let [ms_8_bits_col36]: [QM31; 1] = (*ms_8_bits_col36.try_into().unwrap()).unbox();
        let [xor_col37]: [QM31; 1] = (*xor_col37.try_into().unwrap()).unbox();
        let [xor_col38]: [QM31; 1] = (*xor_col38.try_into().unwrap()).unbox();
        let [output2h_col39]: [QM31; 1] = (*output2h_col39.try_into().unwrap()).unbox();
        let [output_low_col40]: [QM31; 1] = (*output_low_col40.try_into().unwrap()).unbox();
        let [output_high_col41]: [QM31; 1] = (*output_high_col41.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let split_16_low_part_size_8_output_tmp_2ae59_1_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_0_col0, ms_8_bits_col2, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_2ae59_3_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_1_col1, ms_8_bits_col3, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_2ae59_1_limb_0, qm31_const::<133, 0, 0, 0>()],
            and_col4,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col2, qm31_const::<66, 0, 0, 0>()],
            and_col5,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - l0
        let constraint_quotient = ((l0_col6
            - (and_col4 + (and_col5 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_2ae59_1_limb_0, qm31_const::<122, 0, 0, 0>()],
            and_col7,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col2, qm31_const::<0, 0, 0, 0>()],
            and_col8,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - l1
        let constraint_quotient = ((l1_col9
            - (and_col7 + (and_col8 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_2ae59_1_limb_0, qm31_const::<0, 0, 0, 0>()],
            and_col10,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col2, qm31_const::<189, 0, 0, 0>()],
            and_col11,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - l2
        let constraint_quotient = ((l2_col12
            - (and_col10 + (and_col11 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_2ae59_3_limb_0, qm31_const::<165, 0, 0, 0>()],
            and_col13,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col3, qm31_const::<74, 0, 0, 0>()],
            and_col14,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - h0
        let constraint_quotient = ((h0_col15
            - (and_col13 + (and_col14 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_2ae59_3_limb_0, qm31_const::<90, 0, 0, 0>()],
            and_col16,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_8,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col3, qm31_const::<1, 0, 0, 0>()],
            and_col17,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - h1
        let constraint_quotient = ((h1_col18
            - (and_col16 + (and_col17 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_2ae59_3_limb_0, qm31_const::<0, 0, 0, 0>()],
            and_col19,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_10,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col3, qm31_const::<180, 0, 0, 0>()],
            and_col20,
            self.verify_bitwise_and_8_lookup_elements,
            ref verify_bitwise_and_8_sum_11,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - h2
        let constraint_quotient = ((h2_col21
            - (and_col19 + (and_col20 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        sha_256_small_sigma_1_o_0_sum_12 = self
            .sha_256_small_sigma_1_o_0_lookup_elements
            .combine_qm31(
                [
                    l0_col6, h0_col15, sigma_O0_L_col22, sigma_O0_H_col23, sigma_O2_L_col26,
                    sigma_O2_H_col27,
                ],
            );

        sha_256_small_sigma_1_o_1_sum_13 = self
            .sha_256_small_sigma_1_o_1_lookup_elements
            .combine_qm31(
                [
                    (l1_col9 + l2_col12), (h1_col18 + h2_col21), sigma_O1_L_col24, sigma_O1_H_col25,
                    sigma_O2_prime_L_col28, sigma_O2_prime_H_col29,
                ],
            );
        let split_16_low_part_size_8_output_tmp_2ae59_39_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            sigma_O2_prime_L_col28,
            ms_8_bits_col30,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_2ae59_41_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            sigma_O2_L_col26, ms_8_bits_col31, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_2ae59_39_limb_0,
                split_16_low_part_size_8_output_tmp_2ae59_41_limb_0,
            ],
            xor_col32,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_14,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col30, ms_8_bits_col31],
            xor_col33,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_15,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - output2l
        let constraint_quotient = ((output2l_col34
            - (xor_col32 + (xor_col33 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let split_16_low_part_size_8_output_tmp_2ae59_47_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            sigma_O2_prime_H_col29,
            ms_8_bits_col35,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_2ae59_49_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            sigma_O2_H_col27, ms_8_bits_col36, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_2ae59_47_limb_0,
                split_16_low_part_size_8_output_tmp_2ae59_49_limb_0,
            ],
            xor_col37,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_16,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col35, ms_8_bits_col36],
            xor_col38,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_17,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - output2h
        let constraint_quotient = ((output2h_col39
            - (xor_col37 + (xor_col38 * qm31_const::<256, 0, 0, 0>()))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        sha_256_small_sigma_1_sum_18 = self
            .sha_256_small_sigma_1_lookup_elements
            .combine_qm31(
                [input_limb_0_col0, input_limb_1_col1, output_low_col40, output_high_col41],
            );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            enabler,
            column_size,
            ref interaction_trace_mask_values,
            verify_bitwise_and_8_sum_0,
            verify_bitwise_and_8_sum_1,
            verify_bitwise_and_8_sum_2,
            verify_bitwise_and_8_sum_3,
            verify_bitwise_and_8_sum_4,
            verify_bitwise_and_8_sum_5,
            verify_bitwise_and_8_sum_6,
            verify_bitwise_and_8_sum_7,
            verify_bitwise_and_8_sum_8,
            verify_bitwise_and_8_sum_9,
            verify_bitwise_and_8_sum_10,
            verify_bitwise_and_8_sum_11,
            sha_256_small_sigma_1_o_0_sum_12,
            sha_256_small_sigma_1_o_1_sum_13,
            verify_bitwise_xor_8_sum_14,
            verify_bitwise_xor_8_sum_15,
            verify_bitwise_xor_8_sum_16,
            verify_bitwise_xor_8_sum_17,
            sha_256_small_sigma_1_sum_18,
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
    verify_bitwise_and_8_sum_0: QM31,
    verify_bitwise_and_8_sum_1: QM31,
    verify_bitwise_and_8_sum_2: QM31,
    verify_bitwise_and_8_sum_3: QM31,
    verify_bitwise_and_8_sum_4: QM31,
    verify_bitwise_and_8_sum_5: QM31,
    verify_bitwise_and_8_sum_6: QM31,
    verify_bitwise_and_8_sum_7: QM31,
    verify_bitwise_and_8_sum_8: QM31,
    verify_bitwise_and_8_sum_9: QM31,
    verify_bitwise_and_8_sum_10: QM31,
    verify_bitwise_and_8_sum_11: QM31,
    sha_256_small_sigma_1_o_0_sum_12: QM31,
    sha_256_small_sigma_1_o_1_sum_13: QM31,
    verify_bitwise_xor_8_sum_14: QM31,
    verify_bitwise_xor_8_sum_15: QM31,
    verify_bitwise_xor_8_sum_16: QM31,
    verify_bitwise_xor_8_sum_17: QM31,
    sha_256_small_sigma_1_sum_18: QM31,
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
        trace_2_col36,
        trace_2_col37,
        trace_2_col38,
        trace_2_col39,
    ]: [Span<QM31>; 40] =
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
    let [trace_2_col32]: [QM31; 1] = (*trace_2_col32.try_into().unwrap()).unbox();
    let [trace_2_col33]: [QM31; 1] = (*trace_2_col33.try_into().unwrap()).unbox();
    let [trace_2_col34]: [QM31; 1] = (*trace_2_col34.try_into().unwrap()).unbox();
    let [trace_2_col35]: [QM31; 1] = (*trace_2_col35.try_into().unwrap()).unbox();
    let [trace_2_col36_neg1, trace_2_col36]: [QM31; 2] = (*trace_2_col36.try_into().unwrap())
        .unbox();
    let [trace_2_col37_neg1, trace_2_col37]: [QM31; 2] = (*trace_2_col37.try_into().unwrap())
        .unbox();
    let [trace_2_col38_neg1, trace_2_col38]: [QM31; 2] = (*trace_2_col38.try_into().unwrap())
        .unbox();
    let [trace_2_col39_neg1, trace_2_col39]: [QM31; 2] = (*trace_2_col39.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_bitwise_and_8_sum_0
        * verify_bitwise_and_8_sum_1)
        - verify_bitwise_and_8_sum_0
        - verify_bitwise_and_8_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * verify_bitwise_and_8_sum_2
        * verify_bitwise_and_8_sum_3)
        - verify_bitwise_and_8_sum_2
        - verify_bitwise_and_8_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * verify_bitwise_and_8_sum_4
        * verify_bitwise_and_8_sum_5)
        - verify_bitwise_and_8_sum_4
        - verify_bitwise_and_8_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_and_8_sum_6
        * verify_bitwise_and_8_sum_7)
        - verify_bitwise_and_8_sum_6
        - verify_bitwise_and_8_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * verify_bitwise_and_8_sum_8
        * verify_bitwise_and_8_sum_9)
        - verify_bitwise_and_8_sum_8
        - verify_bitwise_and_8_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * verify_bitwise_and_8_sum_10
        * verify_bitwise_and_8_sum_11)
        - verify_bitwise_and_8_sum_10
        - verify_bitwise_and_8_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * sha_256_small_sigma_1_o_0_sum_12
        * sha_256_small_sigma_1_o_1_sum_13)
        - sha_256_small_sigma_1_o_0_sum_12
        - sha_256_small_sigma_1_o_1_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * verify_bitwise_xor_8_sum_14
        * verify_bitwise_xor_8_sum_15)
        - verify_bitwise_xor_8_sum_14
        - verify_bitwise_xor_8_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * verify_bitwise_xor_8_sum_16
        * verify_bitwise_xor_8_sum_17)
        - verify_bitwise_xor_8_sum_16
        - verify_bitwise_xor_8_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35])
        - QM31Impl::from_partial_evals(
            [trace_2_col36_neg1, trace_2_col37_neg1, trace_2_col38_neg1, trace_2_col39_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * sha_256_small_sigma_1_sum_18)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

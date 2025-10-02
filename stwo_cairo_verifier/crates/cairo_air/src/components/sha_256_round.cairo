// AIR version 98896da1
use crate::prelude::*;use crate::components::subroutines::bitwise_and_num_bits_16::bitwise_and_num_bits_16_evaluate;use crate::components::subroutines::bitwise_not_num_bits_16::bitwise_not_num_bits_16_evaluate;use crate::components::subroutines::bitwise_xor_num_bits_16::bitwise_xor_num_bits_16_evaluate;use crate::components::subroutines::triple_sum_32::triple_sum_32_evaluate;

pub const N_TRACE_COLUMNS: usize = 91;pub const RELATION_USES_PER_ROW: [(felt252, u32); 8] = [
    ('Sha256BigSigma1', 1), ('Sha256BigSigma0', 1), ('VerifyBitwiseAnd_16', 10), ('VerifyBitwiseNot_16', 2), ('VerifyBitwiseXor_16', 6), ('Sha256KTable', 1), ('Sha256Schedule', 1), ('Sha256Round', 1)
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
        let interaction_log_sizes = [log_size; 48].span();
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
    pub sha_256_big_sigma_1_lookup_elements: crate::Sha256BigSigma1Elements,
pub sha_256_big_sigma_0_lookup_elements: crate::Sha256BigSigma0Elements,
pub verify_bitwise_and_16_lookup_elements: crate::VerifyBitwiseAnd_16Elements,
pub verify_bitwise_not_16_lookup_elements: crate::VerifyBitwiseNot_16Elements,
pub verify_bitwise_xor_16_lookup_elements: crate::VerifyBitwiseXor_16Elements,
pub sha_256_k_table_lookup_elements: crate::Sha256KTableElements,
pub sha_256_schedule_lookup_elements: crate::Sha256ScheduleElements,
pub sha_256_round_lookup_elements: crate::Sha256RoundElements,
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
            sha_256_big_sigma_1_lookup_elements: interaction_elements.sha_256_big_sigma_1.clone(),
sha_256_big_sigma_0_lookup_elements: interaction_elements.sha_256_big_sigma_0.clone(),
verify_bitwise_and_16_lookup_elements: interaction_elements.verify_bitwise_and_16.clone(),
verify_bitwise_not_16_lookup_elements: interaction_elements.verify_bitwise_not_16.clone(),
verify_bitwise_xor_16_lookup_elements: interaction_elements.verify_bitwise_xor_16.clone(),
sha_256_k_table_lookup_elements: interaction_elements.sha_256_k_table.clone(),
sha_256_schedule_lookup_elements: interaction_elements.sha_256_schedule.clone(),
sha_256_round_lookup_elements: interaction_elements.sha_256_round.clone(),
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
        trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
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
        let mut sha_256_big_sigma_1_sum_0: QM31 = Zero::zero();let mut sha_256_big_sigma_0_sum_1: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_2: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_3: QM31 = Zero::zero();let mut verify_bitwise_not_16_sum_4: QM31 = Zero::zero();let mut verify_bitwise_not_16_sum_5: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_6: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_7: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_8: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_9: QM31 = Zero::zero();let mut sha_256_k_table_sum_10: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_11: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_12: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_13: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_14: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_15: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_16: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_17: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_18: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_19: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_20: QM31 = Zero::zero();let mut sha_256_schedule_sum_21: QM31 = Zero::zero();let mut sha_256_round_sum_22: QM31 = Zero::zero();let mut sha_256_round_sum_23: QM31 = Zero::zero();

        let [input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7, input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11, input_limb_12_col12, input_limb_13_col13, input_limb_14_col14, input_limb_15_col15, input_limb_16_col16, input_limb_17_col17, input_limb_18_col18, input_limb_19_col19, input_limb_20_col20, input_limb_21_col21, input_limb_22_col22, input_limb_23_col23, input_limb_24_col24, input_limb_25_col25, input_limb_26_col26, input_limb_27_col27, input_limb_28_col28, input_limb_29_col29, input_limb_30_col30, input_limb_31_col31, input_limb_32_col32, input_limb_33_col33, input_limb_34_col34, input_limb_35_col35, input_limb_36_col36, input_limb_37_col37, input_limb_38_col38, input_limb_39_col39, input_limb_40_col40, input_limb_41_col41, input_limb_42_col42, input_limb_43_col43, input_limb_44_col44, input_limb_45_col45, input_limb_46_col46, input_limb_47_col47, input_limb_48_col48, input_limb_49_col49, sha_256_big_sigma_1_output_limb_0_col50, sha_256_big_sigma_1_output_limb_1_col51, sha_256_big_sigma_0_output_limb_0_col52, sha_256_big_sigma_0_output_limb_1_col53, and_col54, and_col55, not_a_col56, not_a_col57, and_col58, and_col59, xor_col60, xor_col61, ch_limb_0_col62, ch_limb_1_col63, sha_256_k_table_output_limb_0_col64, sha_256_k_table_output_limb_1_col65, triple_sum32_res_limb_0_col66, triple_sum32_res_limb_1_col67, triple_sum32_res_limb_0_col68, triple_sum32_res_limb_1_col69, and_col70, and_col71, and_col72, and_col73, and_col74, and_col75, xor_col76, xor_col77, xor_col78, xor_col79, maj_limb_0_col80, maj_limb_1_col81, triple_sum32_res_limb_0_col82, triple_sum32_res_limb_1_col83, sha_256_schedule_output_limb_0_col84, sha_256_schedule_output_limb_1_col85, triple_sum32_res_limb_0_col86, triple_sum32_res_limb_1_col87, triple_sum32_res_limb_0_col88, triple_sum32_res_limb_1_col89, enabler]: [Span<QM31>; 91]
            = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();let [input_limb_3_col3]: [QM31; 1] = (*input_limb_3_col3.try_into().unwrap()).unbox();let [input_limb_4_col4]: [QM31; 1] = (*input_limb_4_col4.try_into().unwrap()).unbox();let [input_limb_5_col5]: [QM31; 1] = (*input_limb_5_col5.try_into().unwrap()).unbox();let [input_limb_6_col6]: [QM31; 1] = (*input_limb_6_col6.try_into().unwrap()).unbox();let [input_limb_7_col7]: [QM31; 1] = (*input_limb_7_col7.try_into().unwrap()).unbox();let [input_limb_8_col8]: [QM31; 1] = (*input_limb_8_col8.try_into().unwrap()).unbox();let [input_limb_9_col9]: [QM31; 1] = (*input_limb_9_col9.try_into().unwrap()).unbox();let [input_limb_10_col10]: [QM31; 1] = (*input_limb_10_col10.try_into().unwrap()).unbox();let [input_limb_11_col11]: [QM31; 1] = (*input_limb_11_col11.try_into().unwrap()).unbox();let [input_limb_12_col12]: [QM31; 1] = (*input_limb_12_col12.try_into().unwrap()).unbox();let [input_limb_13_col13]: [QM31; 1] = (*input_limb_13_col13.try_into().unwrap()).unbox();let [input_limb_14_col14]: [QM31; 1] = (*input_limb_14_col14.try_into().unwrap()).unbox();let [input_limb_15_col15]: [QM31; 1] = (*input_limb_15_col15.try_into().unwrap()).unbox();let [input_limb_16_col16]: [QM31; 1] = (*input_limb_16_col16.try_into().unwrap()).unbox();let [input_limb_17_col17]: [QM31; 1] = (*input_limb_17_col17.try_into().unwrap()).unbox();let [input_limb_18_col18]: [QM31; 1] = (*input_limb_18_col18.try_into().unwrap()).unbox();let [input_limb_19_col19]: [QM31; 1] = (*input_limb_19_col19.try_into().unwrap()).unbox();let [input_limb_20_col20]: [QM31; 1] = (*input_limb_20_col20.try_into().unwrap()).unbox();let [input_limb_21_col21]: [QM31; 1] = (*input_limb_21_col21.try_into().unwrap()).unbox();let [input_limb_22_col22]: [QM31; 1] = (*input_limb_22_col22.try_into().unwrap()).unbox();let [input_limb_23_col23]: [QM31; 1] = (*input_limb_23_col23.try_into().unwrap()).unbox();let [input_limb_24_col24]: [QM31; 1] = (*input_limb_24_col24.try_into().unwrap()).unbox();let [input_limb_25_col25]: [QM31; 1] = (*input_limb_25_col25.try_into().unwrap()).unbox();let [input_limb_26_col26]: [QM31; 1] = (*input_limb_26_col26.try_into().unwrap()).unbox();let [input_limb_27_col27]: [QM31; 1] = (*input_limb_27_col27.try_into().unwrap()).unbox();let [input_limb_28_col28]: [QM31; 1] = (*input_limb_28_col28.try_into().unwrap()).unbox();let [input_limb_29_col29]: [QM31; 1] = (*input_limb_29_col29.try_into().unwrap()).unbox();let [input_limb_30_col30]: [QM31; 1] = (*input_limb_30_col30.try_into().unwrap()).unbox();let [input_limb_31_col31]: [QM31; 1] = (*input_limb_31_col31.try_into().unwrap()).unbox();let [input_limb_32_col32]: [QM31; 1] = (*input_limb_32_col32.try_into().unwrap()).unbox();let [input_limb_33_col33]: [QM31; 1] = (*input_limb_33_col33.try_into().unwrap()).unbox();let [input_limb_34_col34]: [QM31; 1] = (*input_limb_34_col34.try_into().unwrap()).unbox();let [input_limb_35_col35]: [QM31; 1] = (*input_limb_35_col35.try_into().unwrap()).unbox();let [input_limb_36_col36]: [QM31; 1] = (*input_limb_36_col36.try_into().unwrap()).unbox();let [input_limb_37_col37]: [QM31; 1] = (*input_limb_37_col37.try_into().unwrap()).unbox();let [input_limb_38_col38]: [QM31; 1] = (*input_limb_38_col38.try_into().unwrap()).unbox();let [input_limb_39_col39]: [QM31; 1] = (*input_limb_39_col39.try_into().unwrap()).unbox();let [input_limb_40_col40]: [QM31; 1] = (*input_limb_40_col40.try_into().unwrap()).unbox();let [input_limb_41_col41]: [QM31; 1] = (*input_limb_41_col41.try_into().unwrap()).unbox();let [input_limb_42_col42]: [QM31; 1] = (*input_limb_42_col42.try_into().unwrap()).unbox();let [input_limb_43_col43]: [QM31; 1] = (*input_limb_43_col43.try_into().unwrap()).unbox();let [input_limb_44_col44]: [QM31; 1] = (*input_limb_44_col44.try_into().unwrap()).unbox();let [input_limb_45_col45]: [QM31; 1] = (*input_limb_45_col45.try_into().unwrap()).unbox();let [input_limb_46_col46]: [QM31; 1] = (*input_limb_46_col46.try_into().unwrap()).unbox();let [input_limb_47_col47]: [QM31; 1] = (*input_limb_47_col47.try_into().unwrap()).unbox();let [input_limb_48_col48]: [QM31; 1] = (*input_limb_48_col48.try_into().unwrap()).unbox();let [input_limb_49_col49]: [QM31; 1] = (*input_limb_49_col49.try_into().unwrap()).unbox();let [sha_256_big_sigma_1_output_limb_0_col50]: [QM31; 1] = (*sha_256_big_sigma_1_output_limb_0_col50.try_into().unwrap()).unbox();let [sha_256_big_sigma_1_output_limb_1_col51]: [QM31; 1] = (*sha_256_big_sigma_1_output_limb_1_col51.try_into().unwrap()).unbox();let [sha_256_big_sigma_0_output_limb_0_col52]: [QM31; 1] = (*sha_256_big_sigma_0_output_limb_0_col52.try_into().unwrap()).unbox();let [sha_256_big_sigma_0_output_limb_1_col53]: [QM31; 1] = (*sha_256_big_sigma_0_output_limb_1_col53.try_into().unwrap()).unbox();let [and_col54]: [QM31; 1] = (*and_col54.try_into().unwrap()).unbox();let [and_col55]: [QM31; 1] = (*and_col55.try_into().unwrap()).unbox();let [not_a_col56]: [QM31; 1] = (*not_a_col56.try_into().unwrap()).unbox();let [not_a_col57]: [QM31; 1] = (*not_a_col57.try_into().unwrap()).unbox();let [and_col58]: [QM31; 1] = (*and_col58.try_into().unwrap()).unbox();let [and_col59]: [QM31; 1] = (*and_col59.try_into().unwrap()).unbox();let [xor_col60]: [QM31; 1] = (*xor_col60.try_into().unwrap()).unbox();let [xor_col61]: [QM31; 1] = (*xor_col61.try_into().unwrap()).unbox();let [ch_limb_0_col62]: [QM31; 1] = (*ch_limb_0_col62.try_into().unwrap()).unbox();let [ch_limb_1_col63]: [QM31; 1] = (*ch_limb_1_col63.try_into().unwrap()).unbox();let [sha_256_k_table_output_limb_0_col64]: [QM31; 1] = (*sha_256_k_table_output_limb_0_col64.try_into().unwrap()).unbox();let [sha_256_k_table_output_limb_1_col65]: [QM31; 1] = (*sha_256_k_table_output_limb_1_col65.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col66]: [QM31; 1] = (*triple_sum32_res_limb_0_col66.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col67]: [QM31; 1] = (*triple_sum32_res_limb_1_col67.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col68]: [QM31; 1] = (*triple_sum32_res_limb_0_col68.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col69]: [QM31; 1] = (*triple_sum32_res_limb_1_col69.try_into().unwrap()).unbox();let [and_col70]: [QM31; 1] = (*and_col70.try_into().unwrap()).unbox();let [and_col71]: [QM31; 1] = (*and_col71.try_into().unwrap()).unbox();let [and_col72]: [QM31; 1] = (*and_col72.try_into().unwrap()).unbox();let [and_col73]: [QM31; 1] = (*and_col73.try_into().unwrap()).unbox();let [and_col74]: [QM31; 1] = (*and_col74.try_into().unwrap()).unbox();let [and_col75]: [QM31; 1] = (*and_col75.try_into().unwrap()).unbox();let [xor_col76]: [QM31; 1] = (*xor_col76.try_into().unwrap()).unbox();let [xor_col77]: [QM31; 1] = (*xor_col77.try_into().unwrap()).unbox();let [xor_col78]: [QM31; 1] = (*xor_col78.try_into().unwrap()).unbox();let [xor_col79]: [QM31; 1] = (*xor_col79.try_into().unwrap()).unbox();let [maj_limb_0_col80]: [QM31; 1] = (*maj_limb_0_col80.try_into().unwrap()).unbox();let [maj_limb_1_col81]: [QM31; 1] = (*maj_limb_1_col81.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col82]: [QM31; 1] = (*triple_sum32_res_limb_0_col82.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col83]: [QM31; 1] = (*triple_sum32_res_limb_1_col83.try_into().unwrap()).unbox();let [sha_256_schedule_output_limb_0_col84]: [QM31; 1] = (*sha_256_schedule_output_limb_0_col84.try_into().unwrap()).unbox();let [sha_256_schedule_output_limb_1_col85]: [QM31; 1] = (*sha_256_schedule_output_limb_1_col85.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col86]: [QM31; 1] = (*triple_sum32_res_limb_0_col86.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col87]: [QM31; 1] = (*triple_sum32_res_limb_1_col87.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col88]: [QM31; 1] = (*triple_sum32_res_limb_0_col88.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col89]: [QM31; 1] = (*triple_sum32_res_limb_1_col89.try_into().unwrap()).unbox();let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();


        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        sha_256_big_sigma_1_sum_0 = self.sha_256_big_sigma_1_lookup_elements.combine_qm31(
            [
                input_limb_10_col10,
input_limb_11_col11,
sha_256_big_sigma_1_output_limb_0_col50,
sha_256_big_sigma_1_output_limb_1_col51
            ],
        );

        sha_256_big_sigma_0_sum_1 = self.sha_256_big_sigma_0_lookup_elements.combine_qm31(
            [
                input_limb_2_col2,
input_limb_3_col3,
sha_256_big_sigma_0_output_limb_0_col52,
sha_256_big_sigma_0_output_limb_1_col53
            ],
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_10_col10, input_limb_12_col12],
and_col54,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_2,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_11_col11, input_limb_13_col13],
and_col55,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_3,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_not_num_bits_16_evaluate(
            input_limb_10_col10,
not_a_col56,
self.verify_bitwise_not_16_lookup_elements,
ref verify_bitwise_not_16_sum_4,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_not_num_bits_16_evaluate(
            input_limb_11_col11,
not_a_col57,
self.verify_bitwise_not_16_lookup_elements,
ref verify_bitwise_not_16_sum_5,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [not_a_col56, input_limb_14_col14],
and_col58,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_6,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [not_a_col57, input_limb_15_col15],
and_col59,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_7,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [and_col54, and_col58],
xor_col60,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_8,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [and_col55, and_col59],
xor_col61,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_9,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_k_table_sum_10 = self.sha_256_k_table_lookup_elements.combine_qm31(
            [
                input_limb_1_col1,
sha_256_k_table_output_limb_0_col64,
sha_256_k_table_output_limb_1_col65
            ],
        );triple_sum_32_evaluate(
            [input_limb_16_col16, input_limb_17_col17, sha_256_big_sigma_1_output_limb_0_col50, sha_256_big_sigma_1_output_limb_1_col51, ch_limb_0_col62, ch_limb_1_col63],
triple_sum32_res_limb_0_col66,
triple_sum32_res_limb_1_col67,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );triple_sum_32_evaluate(
            [triple_sum32_res_limb_0_col66, triple_sum32_res_limb_1_col67, sha_256_k_table_output_limb_0_col64, sha_256_k_table_output_limb_1_col65, input_limb_18_col18, input_limb_19_col19],
triple_sum32_res_limb_0_col68,
triple_sum32_res_limb_1_col69,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_2_col2, input_limb_4_col4],
and_col70,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_11,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_3_col3, input_limb_5_col5],
and_col71,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_12,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_2_col2, input_limb_6_col6],
and_col72,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_13,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_3_col3, input_limb_7_col7],
and_col73,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_14,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_4_col4, input_limb_6_col6],
and_col74,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_15,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_5_col5, input_limb_7_col7],
and_col75,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_16,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [and_col70, and_col72],
xor_col76,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_17,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [and_col71, and_col73],
xor_col77,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_18,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [xor_col76, and_col74],
xor_col78,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_19,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [xor_col77, and_col75],
xor_col79,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_20,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );triple_sum_32_evaluate(
            [sha_256_big_sigma_0_output_limb_0_col52, sha_256_big_sigma_0_output_limb_1_col53, maj_limb_0_col80, maj_limb_1_col81, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>()],
triple_sum32_res_limb_0_col82,
triple_sum32_res_limb_1_col83,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_schedule_sum_21 = self.sha_256_schedule_lookup_elements.combine_qm31(
            [
                input_limb_18_col18,
input_limb_19_col19,
input_limb_20_col20,
input_limb_21_col21,
input_limb_22_col22,
input_limb_23_col23,
input_limb_24_col24,
input_limb_25_col25,
input_limb_26_col26,
input_limb_27_col27,
input_limb_28_col28,
input_limb_29_col29,
input_limb_30_col30,
input_limb_31_col31,
input_limb_32_col32,
input_limb_33_col33,
input_limb_34_col34,
input_limb_35_col35,
input_limb_36_col36,
input_limb_37_col37,
input_limb_38_col38,
input_limb_39_col39,
input_limb_40_col40,
input_limb_41_col41,
input_limb_42_col42,
input_limb_43_col43,
input_limb_44_col44,
input_limb_45_col45,
input_limb_46_col46,
input_limb_47_col47,
input_limb_48_col48,
input_limb_49_col49,
sha_256_schedule_output_limb_0_col84,
sha_256_schedule_output_limb_1_col85
            ],
        );triple_sum_32_evaluate(
            [triple_sum32_res_limb_0_col68, triple_sum32_res_limb_1_col69, input_limb_8_col8, input_limb_9_col9, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>()],
triple_sum32_res_limb_0_col86,
triple_sum32_res_limb_1_col87,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );triple_sum_32_evaluate(
            [triple_sum32_res_limb_0_col68, triple_sum32_res_limb_1_col69, triple_sum32_res_limb_0_col82, triple_sum32_res_limb_1_col83, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>()],
triple_sum32_res_limb_0_col88,
triple_sum32_res_limb_1_col89,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_round_sum_22 = self.sha_256_round_lookup_elements.combine_qm31(
            [
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
input_limb_12_col12,
input_limb_13_col13,
input_limb_14_col14,
input_limb_15_col15,
input_limb_16_col16,
input_limb_17_col17,
input_limb_18_col18,
input_limb_19_col19,
input_limb_20_col20,
input_limb_21_col21,
input_limb_22_col22,
input_limb_23_col23,
input_limb_24_col24,
input_limb_25_col25,
input_limb_26_col26,
input_limb_27_col27,
input_limb_28_col28,
input_limb_29_col29,
input_limb_30_col30,
input_limb_31_col31,
input_limb_32_col32,
input_limb_33_col33,
input_limb_34_col34,
input_limb_35_col35,
input_limb_36_col36,
input_limb_37_col37,
input_limb_38_col38,
input_limb_39_col39,
input_limb_40_col40,
input_limb_41_col41,
input_limb_42_col42,
input_limb_43_col43,
input_limb_44_col44,
input_limb_45_col45,
input_limb_46_col46,
input_limb_47_col47,
input_limb_48_col48,
input_limb_49_col49
            ],
        );

        sha_256_round_sum_23 = self.sha_256_round_lookup_elements.combine_qm31(
            [
                input_limb_0_col0,
(input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
triple_sum32_res_limb_0_col88,
triple_sum32_res_limb_1_col89,
input_limb_2_col2,
input_limb_3_col3,
input_limb_4_col4,
input_limb_5_col5,
input_limb_6_col6,
input_limb_7_col7,
triple_sum32_res_limb_0_col86,
triple_sum32_res_limb_1_col87,
input_limb_10_col10,
input_limb_11_col11,
input_limb_12_col12,
input_limb_13_col13,
input_limb_14_col14,
input_limb_15_col15,
input_limb_20_col20,
input_limb_21_col21,
input_limb_22_col22,
input_limb_23_col23,
input_limb_24_col24,
input_limb_25_col25,
input_limb_26_col26,
input_limb_27_col27,
input_limb_28_col28,
input_limb_29_col29,
input_limb_30_col30,
input_limb_31_col31,
input_limb_32_col32,
input_limb_33_col33,
input_limb_34_col34,
input_limb_35_col35,
input_limb_36_col36,
input_limb_37_col37,
input_limb_38_col38,
input_limb_39_col39,
input_limb_40_col40,
input_limb_41_col41,
input_limb_42_col42,
input_limb_43_col43,
input_limb_44_col44,
input_limb_45_col45,
input_limb_46_col46,
input_limb_47_col47,
input_limb_48_col48,
input_limb_49_col49,
sha_256_schedule_output_limb_0_col84,
sha_256_schedule_output_limb_1_col85
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
            sha_256_big_sigma_1_sum_0,
sha_256_big_sigma_0_sum_1,
verify_bitwise_and_16_sum_2,
verify_bitwise_and_16_sum_3,
verify_bitwise_not_16_sum_4,
verify_bitwise_not_16_sum_5,
verify_bitwise_and_16_sum_6,
verify_bitwise_and_16_sum_7,
verify_bitwise_xor_16_sum_8,
verify_bitwise_xor_16_sum_9,
sha_256_k_table_sum_10,
verify_bitwise_and_16_sum_11,
verify_bitwise_and_16_sum_12,
verify_bitwise_and_16_sum_13,
verify_bitwise_and_16_sum_14,
verify_bitwise_and_16_sum_15,
verify_bitwise_and_16_sum_16,
verify_bitwise_xor_16_sum_17,
verify_bitwise_xor_16_sum_18,
verify_bitwise_xor_16_sum_19,
verify_bitwise_xor_16_sum_20,
sha_256_schedule_sum_21,
sha_256_round_sum_22,
sha_256_round_sum_23
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
    sha_256_big_sigma_1_sum_0: QM31,
sha_256_big_sigma_0_sum_1: QM31,
verify_bitwise_and_16_sum_2: QM31,
verify_bitwise_and_16_sum_3: QM31,
verify_bitwise_not_16_sum_4: QM31,
verify_bitwise_not_16_sum_5: QM31,
verify_bitwise_and_16_sum_6: QM31,
verify_bitwise_and_16_sum_7: QM31,
verify_bitwise_xor_16_sum_8: QM31,
verify_bitwise_xor_16_sum_9: QM31,
sha_256_k_table_sum_10: QM31,
verify_bitwise_and_16_sum_11: QM31,
verify_bitwise_and_16_sum_12: QM31,
verify_bitwise_and_16_sum_13: QM31,
verify_bitwise_and_16_sum_14: QM31,
verify_bitwise_and_16_sum_15: QM31,
verify_bitwise_and_16_sum_16: QM31,
verify_bitwise_xor_16_sum_17: QM31,
verify_bitwise_xor_16_sum_18: QM31,
verify_bitwise_xor_16_sum_19: QM31,
verify_bitwise_xor_16_sum_20: QM31,
sha_256_schedule_sum_21: QM31,
sha_256_round_sum_22: QM31,
sha_256_round_sum_23: QM31
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3, trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7, trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11, trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15, trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19, trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23, trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27, trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31, trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35, trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39, trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43, trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47]: [Span<QM31>; 48]
        = (*interaction_trace_mask_values.multi_pop_front().unwrap()).unbox();

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
let [trace_2_col36]: [QM31; 1] = (*trace_2_col36.try_into().unwrap()).unbox();
let [trace_2_col37]: [QM31; 1] = (*trace_2_col37.try_into().unwrap()).unbox();
let [trace_2_col38]: [QM31; 1] = (*trace_2_col38.try_into().unwrap()).unbox();
let [trace_2_col39]: [QM31; 1] = (*trace_2_col39.try_into().unwrap()).unbox();
let [trace_2_col40]: [QM31; 1] = (*trace_2_col40.try_into().unwrap()).unbox();
let [trace_2_col41]: [QM31; 1] = (*trace_2_col41.try_into().unwrap()).unbox();
let [trace_2_col42]: [QM31; 1] = (*trace_2_col42.try_into().unwrap()).unbox();
let [trace_2_col43]: [QM31; 1] = (*trace_2_col43.try_into().unwrap()).unbox();
let [trace_2_col44_neg1, trace_2_col44]: [QM31; 2] = (*trace_2_col44.try_into().unwrap()).unbox();
let [trace_2_col45_neg1, trace_2_col45]: [QM31; 2] = (*trace_2_col45.try_into().unwrap()).unbox();
let [trace_2_col46_neg1, trace_2_col46]: [QM31; 2] = (*trace_2_col46.try_into().unwrap()).unbox();
let [trace_2_col47_neg1, trace_2_col47]: [QM31; 2] = (*trace_2_col47.try_into().unwrap()).unbox();


    core::internal::revoke_ap_tracking();

    
let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * sha_256_big_sigma_1_sum_0 * sha_256_big_sigma_0_sum_1
        ) - sha_256_big_sigma_1_sum_0 - sha_256_big_sigma_0_sum_1
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]) 
                - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * verify_bitwise_and_16_sum_2 * verify_bitwise_and_16_sum_3
        ) - verify_bitwise_and_16_sum_2 - verify_bitwise_and_16_sum_3
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]) 
                - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
            ) * verify_bitwise_not_16_sum_4 * verify_bitwise_not_16_sum_5
        ) - verify_bitwise_not_16_sum_4 - verify_bitwise_not_16_sum_5
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15]) 
                - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
            ) * verify_bitwise_and_16_sum_6 * verify_bitwise_and_16_sum_7
        ) - verify_bitwise_and_16_sum_6 - verify_bitwise_and_16_sum_7
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19]) 
                - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
            ) * verify_bitwise_xor_16_sum_8 * verify_bitwise_xor_16_sum_9
        ) - verify_bitwise_xor_16_sum_8 - verify_bitwise_xor_16_sum_9
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23]) 
                - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19])
            ) * sha_256_k_table_sum_10 * verify_bitwise_and_16_sum_11
        ) - sha_256_k_table_sum_10 - verify_bitwise_and_16_sum_11
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27]) 
                - QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23])
            ) * verify_bitwise_and_16_sum_12 * verify_bitwise_and_16_sum_13
        ) - verify_bitwise_and_16_sum_12 - verify_bitwise_and_16_sum_13
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31]) 
                - QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27])
            ) * verify_bitwise_and_16_sum_14 * verify_bitwise_and_16_sum_15
        ) - verify_bitwise_and_16_sum_14 - verify_bitwise_and_16_sum_15
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35]) 
                - QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31])
            ) * verify_bitwise_and_16_sum_16 * verify_bitwise_xor_16_sum_17
        ) - verify_bitwise_and_16_sum_16 - verify_bitwise_xor_16_sum_17
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39]) 
                - QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35])
            ) * verify_bitwise_xor_16_sum_18 * verify_bitwise_xor_16_sum_19
        ) - verify_bitwise_xor_16_sum_18 - verify_bitwise_xor_16_sum_19
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43]) 
                - QM31Impl::from_partial_evals([trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39])
            ) * verify_bitwise_xor_16_sum_20 * sha_256_schedule_sum_21
        ) - verify_bitwise_xor_16_sum_20 - sha_256_schedule_sum_21
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47]) 
                - QM31Impl::from_partial_evals([trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43]) 
                - QM31Impl::from_partial_evals([trace_2_col44_neg1, trace_2_col45_neg1, trace_2_col46_neg1, trace_2_col47_neg1])
                + (claimed_sum * (column_size.inverse().into()))
            ) * sha_256_round_sum_22 * sha_256_round_sum_23
        ) + (sha_256_round_sum_22 * enabler) - (sha_256_round_sum_23 * enabler)
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

}
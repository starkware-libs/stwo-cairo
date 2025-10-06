// AIR version 98896da1-dirty
use crate::prelude::*;use crate::components::subroutines::bitwise_and_num_bits_8::bitwise_and_num_bits_8_evaluate;use crate::components::subroutines::bitwise_not_num_bits_16::bitwise_not_num_bits_16_evaluate;use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;use crate::components::subroutines::triple_sum_32::triple_sum_32_evaluate;

pub const N_TRACE_COLUMNS: usize = 125;pub const RELATION_USES_PER_ROW: [(felt252, u32); 8] = [
    ('Sha256BigSigma1', 1), ('Sha256BigSigma0', 1), ('VerifyBitwiseAnd_8', 20), ('VerifyBitwiseNot_16', 2), ('VerifyBitwiseXor_8', 12), ('Sha256KTable', 1), ('Sha256Schedule', 1), ('Sha256Round', 1)
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
        let interaction_log_sizes = [log_size; 80].span();
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
pub verify_bitwise_and_8_lookup_elements: crate::VerifyBitwiseAnd_8Elements,
pub verify_bitwise_not_16_lookup_elements: crate::VerifyBitwiseNot_16Elements,
pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
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
verify_bitwise_and_8_lookup_elements: interaction_elements.verify_bitwise_and_8.clone(),
verify_bitwise_not_16_lookup_elements: interaction_elements.verify_bitwise_not_16.clone(),
verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
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
        trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
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
        let mut sha_256_big_sigma_1_sum_0: QM31 = Zero::zero();let mut sha_256_big_sigma_0_sum_1: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_2: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_3: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_4: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_5: QM31 = Zero::zero();let mut verify_bitwise_not_16_sum_6: QM31 = Zero::zero();let mut verify_bitwise_not_16_sum_7: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_8: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_9: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_10: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_11: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_12: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_13: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_14: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_15: QM31 = Zero::zero();let mut sha_256_k_table_sum_16: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_17: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_18: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_19: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_20: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_21: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_22: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_23: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_24: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_25: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_26: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_27: QM31 = Zero::zero();let mut verify_bitwise_and_8_sum_28: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_29: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_30: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_31: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_32: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_33: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_34: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_35: QM31 = Zero::zero();let mut verify_bitwise_xor_8_sum_36: QM31 = Zero::zero();let mut sha_256_schedule_sum_37: QM31 = Zero::zero();let mut sha_256_round_sum_38: QM31 = Zero::zero();let mut sha_256_round_sum_39: QM31 = Zero::zero();

        let [input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7, input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11, input_limb_12_col12, input_limb_13_col13, input_limb_14_col14, input_limb_15_col15, input_limb_16_col16, input_limb_17_col17, input_limb_18_col18, input_limb_19_col19, input_limb_20_col20, input_limb_21_col21, input_limb_22_col22, input_limb_23_col23, input_limb_24_col24, input_limb_25_col25, input_limb_26_col26, input_limb_27_col27, input_limb_28_col28, input_limb_29_col29, input_limb_30_col30, input_limb_31_col31, input_limb_32_col32, input_limb_33_col33, input_limb_34_col34, input_limb_35_col35, input_limb_36_col36, input_limb_37_col37, input_limb_38_col38, input_limb_39_col39, input_limb_40_col40, input_limb_41_col41, input_limb_42_col42, input_limb_43_col43, input_limb_44_col44, input_limb_45_col45, input_limb_46_col46, input_limb_47_col47, input_limb_48_col48, input_limb_49_col49, sha_256_big_sigma_1_output_limb_0_col50, sha_256_big_sigma_1_output_limb_1_col51, sha_256_big_sigma_0_output_limb_0_col52, sha_256_big_sigma_0_output_limb_1_col53, ms_8_bits_col54, ms_8_bits_col55, ms_8_bits_col56, ms_8_bits_col57, and_col58, and_col59, and_col60, and_col61, not_a_col62, not_a_col63, ms_8_bits_col64, ms_8_bits_col65, ms_8_bits_col66, ms_8_bits_col67, and_col68, and_col69, and_col70, and_col71, xor_col72, xor_col73, xor_col74, xor_col75, chl_col76, chh_col77, ch_limb_0_col78, ch_limb_1_col79, sha_256_k_table_output_limb_0_col80, sha_256_k_table_output_limb_1_col81, triple_sum32_res_limb_0_col82, triple_sum32_res_limb_1_col83, triple_sum32_res_limb_0_col84, triple_sum32_res_limb_1_col85, ms_8_bits_col86, ms_8_bits_col87, ms_8_bits_col88, ms_8_bits_col89, ms_8_bits_col90, ms_8_bits_col91, and_col92, and_col93, and_col94, and_col95, and_col96, and_col97, and_col98, and_col99, and_col100, and_col101, and_col102, and_col103, xor_col104, xor_col105, xor_col106, xor_col107, xor_col108, xor_col109, xor_col110, xor_col111, majl_col112, majh_col113, maj_limb_0_col114, maj_limb_1_col115, triple_sum32_res_limb_0_col116, triple_sum32_res_limb_1_col117, sha_256_schedule_output_limb_0_col118, sha_256_schedule_output_limb_1_col119, triple_sum32_res_limb_0_col120, triple_sum32_res_limb_1_col121, triple_sum32_res_limb_0_col122, triple_sum32_res_limb_1_col123, enabler]: [Span<QM31>; 125]
            = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();let [input_limb_3_col3]: [QM31; 1] = (*input_limb_3_col3.try_into().unwrap()).unbox();let [input_limb_4_col4]: [QM31; 1] = (*input_limb_4_col4.try_into().unwrap()).unbox();let [input_limb_5_col5]: [QM31; 1] = (*input_limb_5_col5.try_into().unwrap()).unbox();let [input_limb_6_col6]: [QM31; 1] = (*input_limb_6_col6.try_into().unwrap()).unbox();let [input_limb_7_col7]: [QM31; 1] = (*input_limb_7_col7.try_into().unwrap()).unbox();let [input_limb_8_col8]: [QM31; 1] = (*input_limb_8_col8.try_into().unwrap()).unbox();let [input_limb_9_col9]: [QM31; 1] = (*input_limb_9_col9.try_into().unwrap()).unbox();let [input_limb_10_col10]: [QM31; 1] = (*input_limb_10_col10.try_into().unwrap()).unbox();let [input_limb_11_col11]: [QM31; 1] = (*input_limb_11_col11.try_into().unwrap()).unbox();let [input_limb_12_col12]: [QM31; 1] = (*input_limb_12_col12.try_into().unwrap()).unbox();let [input_limb_13_col13]: [QM31; 1] = (*input_limb_13_col13.try_into().unwrap()).unbox();let [input_limb_14_col14]: [QM31; 1] = (*input_limb_14_col14.try_into().unwrap()).unbox();let [input_limb_15_col15]: [QM31; 1] = (*input_limb_15_col15.try_into().unwrap()).unbox();let [input_limb_16_col16]: [QM31; 1] = (*input_limb_16_col16.try_into().unwrap()).unbox();let [input_limb_17_col17]: [QM31; 1] = (*input_limb_17_col17.try_into().unwrap()).unbox();let [input_limb_18_col18]: [QM31; 1] = (*input_limb_18_col18.try_into().unwrap()).unbox();let [input_limb_19_col19]: [QM31; 1] = (*input_limb_19_col19.try_into().unwrap()).unbox();let [input_limb_20_col20]: [QM31; 1] = (*input_limb_20_col20.try_into().unwrap()).unbox();let [input_limb_21_col21]: [QM31; 1] = (*input_limb_21_col21.try_into().unwrap()).unbox();let [input_limb_22_col22]: [QM31; 1] = (*input_limb_22_col22.try_into().unwrap()).unbox();let [input_limb_23_col23]: [QM31; 1] = (*input_limb_23_col23.try_into().unwrap()).unbox();let [input_limb_24_col24]: [QM31; 1] = (*input_limb_24_col24.try_into().unwrap()).unbox();let [input_limb_25_col25]: [QM31; 1] = (*input_limb_25_col25.try_into().unwrap()).unbox();let [input_limb_26_col26]: [QM31; 1] = (*input_limb_26_col26.try_into().unwrap()).unbox();let [input_limb_27_col27]: [QM31; 1] = (*input_limb_27_col27.try_into().unwrap()).unbox();let [input_limb_28_col28]: [QM31; 1] = (*input_limb_28_col28.try_into().unwrap()).unbox();let [input_limb_29_col29]: [QM31; 1] = (*input_limb_29_col29.try_into().unwrap()).unbox();let [input_limb_30_col30]: [QM31; 1] = (*input_limb_30_col30.try_into().unwrap()).unbox();let [input_limb_31_col31]: [QM31; 1] = (*input_limb_31_col31.try_into().unwrap()).unbox();let [input_limb_32_col32]: [QM31; 1] = (*input_limb_32_col32.try_into().unwrap()).unbox();let [input_limb_33_col33]: [QM31; 1] = (*input_limb_33_col33.try_into().unwrap()).unbox();let [input_limb_34_col34]: [QM31; 1] = (*input_limb_34_col34.try_into().unwrap()).unbox();let [input_limb_35_col35]: [QM31; 1] = (*input_limb_35_col35.try_into().unwrap()).unbox();let [input_limb_36_col36]: [QM31; 1] = (*input_limb_36_col36.try_into().unwrap()).unbox();let [input_limb_37_col37]: [QM31; 1] = (*input_limb_37_col37.try_into().unwrap()).unbox();let [input_limb_38_col38]: [QM31; 1] = (*input_limb_38_col38.try_into().unwrap()).unbox();let [input_limb_39_col39]: [QM31; 1] = (*input_limb_39_col39.try_into().unwrap()).unbox();let [input_limb_40_col40]: [QM31; 1] = (*input_limb_40_col40.try_into().unwrap()).unbox();let [input_limb_41_col41]: [QM31; 1] = (*input_limb_41_col41.try_into().unwrap()).unbox();let [input_limb_42_col42]: [QM31; 1] = (*input_limb_42_col42.try_into().unwrap()).unbox();let [input_limb_43_col43]: [QM31; 1] = (*input_limb_43_col43.try_into().unwrap()).unbox();let [input_limb_44_col44]: [QM31; 1] = (*input_limb_44_col44.try_into().unwrap()).unbox();let [input_limb_45_col45]: [QM31; 1] = (*input_limb_45_col45.try_into().unwrap()).unbox();let [input_limb_46_col46]: [QM31; 1] = (*input_limb_46_col46.try_into().unwrap()).unbox();let [input_limb_47_col47]: [QM31; 1] = (*input_limb_47_col47.try_into().unwrap()).unbox();let [input_limb_48_col48]: [QM31; 1] = (*input_limb_48_col48.try_into().unwrap()).unbox();let [input_limb_49_col49]: [QM31; 1] = (*input_limb_49_col49.try_into().unwrap()).unbox();let [sha_256_big_sigma_1_output_limb_0_col50]: [QM31; 1] = (*sha_256_big_sigma_1_output_limb_0_col50.try_into().unwrap()).unbox();let [sha_256_big_sigma_1_output_limb_1_col51]: [QM31; 1] = (*sha_256_big_sigma_1_output_limb_1_col51.try_into().unwrap()).unbox();let [sha_256_big_sigma_0_output_limb_0_col52]: [QM31; 1] = (*sha_256_big_sigma_0_output_limb_0_col52.try_into().unwrap()).unbox();let [sha_256_big_sigma_0_output_limb_1_col53]: [QM31; 1] = (*sha_256_big_sigma_0_output_limb_1_col53.try_into().unwrap()).unbox();let [ms_8_bits_col54]: [QM31; 1] = (*ms_8_bits_col54.try_into().unwrap()).unbox();let [ms_8_bits_col55]: [QM31; 1] = (*ms_8_bits_col55.try_into().unwrap()).unbox();let [ms_8_bits_col56]: [QM31; 1] = (*ms_8_bits_col56.try_into().unwrap()).unbox();let [ms_8_bits_col57]: [QM31; 1] = (*ms_8_bits_col57.try_into().unwrap()).unbox();let [and_col58]: [QM31; 1] = (*and_col58.try_into().unwrap()).unbox();let [and_col59]: [QM31; 1] = (*and_col59.try_into().unwrap()).unbox();let [and_col60]: [QM31; 1] = (*and_col60.try_into().unwrap()).unbox();let [and_col61]: [QM31; 1] = (*and_col61.try_into().unwrap()).unbox();let [not_a_col62]: [QM31; 1] = (*not_a_col62.try_into().unwrap()).unbox();let [not_a_col63]: [QM31; 1] = (*not_a_col63.try_into().unwrap()).unbox();let [ms_8_bits_col64]: [QM31; 1] = (*ms_8_bits_col64.try_into().unwrap()).unbox();let [ms_8_bits_col65]: [QM31; 1] = (*ms_8_bits_col65.try_into().unwrap()).unbox();let [ms_8_bits_col66]: [QM31; 1] = (*ms_8_bits_col66.try_into().unwrap()).unbox();let [ms_8_bits_col67]: [QM31; 1] = (*ms_8_bits_col67.try_into().unwrap()).unbox();let [and_col68]: [QM31; 1] = (*and_col68.try_into().unwrap()).unbox();let [and_col69]: [QM31; 1] = (*and_col69.try_into().unwrap()).unbox();let [and_col70]: [QM31; 1] = (*and_col70.try_into().unwrap()).unbox();let [and_col71]: [QM31; 1] = (*and_col71.try_into().unwrap()).unbox();let [xor_col72]: [QM31; 1] = (*xor_col72.try_into().unwrap()).unbox();let [xor_col73]: [QM31; 1] = (*xor_col73.try_into().unwrap()).unbox();let [xor_col74]: [QM31; 1] = (*xor_col74.try_into().unwrap()).unbox();let [xor_col75]: [QM31; 1] = (*xor_col75.try_into().unwrap()).unbox();let [chl_col76]: [QM31; 1] = (*chl_col76.try_into().unwrap()).unbox();let [chh_col77]: [QM31; 1] = (*chh_col77.try_into().unwrap()).unbox();let [ch_limb_0_col78]: [QM31; 1] = (*ch_limb_0_col78.try_into().unwrap()).unbox();let [ch_limb_1_col79]: [QM31; 1] = (*ch_limb_1_col79.try_into().unwrap()).unbox();let [sha_256_k_table_output_limb_0_col80]: [QM31; 1] = (*sha_256_k_table_output_limb_0_col80.try_into().unwrap()).unbox();let [sha_256_k_table_output_limb_1_col81]: [QM31; 1] = (*sha_256_k_table_output_limb_1_col81.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col82]: [QM31; 1] = (*triple_sum32_res_limb_0_col82.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col83]: [QM31; 1] = (*triple_sum32_res_limb_1_col83.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col84]: [QM31; 1] = (*triple_sum32_res_limb_0_col84.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col85]: [QM31; 1] = (*triple_sum32_res_limb_1_col85.try_into().unwrap()).unbox();let [ms_8_bits_col86]: [QM31; 1] = (*ms_8_bits_col86.try_into().unwrap()).unbox();let [ms_8_bits_col87]: [QM31; 1] = (*ms_8_bits_col87.try_into().unwrap()).unbox();let [ms_8_bits_col88]: [QM31; 1] = (*ms_8_bits_col88.try_into().unwrap()).unbox();let [ms_8_bits_col89]: [QM31; 1] = (*ms_8_bits_col89.try_into().unwrap()).unbox();let [ms_8_bits_col90]: [QM31; 1] = (*ms_8_bits_col90.try_into().unwrap()).unbox();let [ms_8_bits_col91]: [QM31; 1] = (*ms_8_bits_col91.try_into().unwrap()).unbox();let [and_col92]: [QM31; 1] = (*and_col92.try_into().unwrap()).unbox();let [and_col93]: [QM31; 1] = (*and_col93.try_into().unwrap()).unbox();let [and_col94]: [QM31; 1] = (*and_col94.try_into().unwrap()).unbox();let [and_col95]: [QM31; 1] = (*and_col95.try_into().unwrap()).unbox();let [and_col96]: [QM31; 1] = (*and_col96.try_into().unwrap()).unbox();let [and_col97]: [QM31; 1] = (*and_col97.try_into().unwrap()).unbox();let [and_col98]: [QM31; 1] = (*and_col98.try_into().unwrap()).unbox();let [and_col99]: [QM31; 1] = (*and_col99.try_into().unwrap()).unbox();let [and_col100]: [QM31; 1] = (*and_col100.try_into().unwrap()).unbox();let [and_col101]: [QM31; 1] = (*and_col101.try_into().unwrap()).unbox();let [and_col102]: [QM31; 1] = (*and_col102.try_into().unwrap()).unbox();let [and_col103]: [QM31; 1] = (*and_col103.try_into().unwrap()).unbox();let [xor_col104]: [QM31; 1] = (*xor_col104.try_into().unwrap()).unbox();let [xor_col105]: [QM31; 1] = (*xor_col105.try_into().unwrap()).unbox();let [xor_col106]: [QM31; 1] = (*xor_col106.try_into().unwrap()).unbox();let [xor_col107]: [QM31; 1] = (*xor_col107.try_into().unwrap()).unbox();let [xor_col108]: [QM31; 1] = (*xor_col108.try_into().unwrap()).unbox();let [xor_col109]: [QM31; 1] = (*xor_col109.try_into().unwrap()).unbox();let [xor_col110]: [QM31; 1] = (*xor_col110.try_into().unwrap()).unbox();let [xor_col111]: [QM31; 1] = (*xor_col111.try_into().unwrap()).unbox();let [majl_col112]: [QM31; 1] = (*majl_col112.try_into().unwrap()).unbox();let [majh_col113]: [QM31; 1] = (*majh_col113.try_into().unwrap()).unbox();let [maj_limb_0_col114]: [QM31; 1] = (*maj_limb_0_col114.try_into().unwrap()).unbox();let [maj_limb_1_col115]: [QM31; 1] = (*maj_limb_1_col115.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col116]: [QM31; 1] = (*triple_sum32_res_limb_0_col116.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col117]: [QM31; 1] = (*triple_sum32_res_limb_1_col117.try_into().unwrap()).unbox();let [sha_256_schedule_output_limb_0_col118]: [QM31; 1] = (*sha_256_schedule_output_limb_0_col118.try_into().unwrap()).unbox();let [sha_256_schedule_output_limb_1_col119]: [QM31; 1] = (*sha_256_schedule_output_limb_1_col119.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col120]: [QM31; 1] = (*triple_sum32_res_limb_0_col120.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col121]: [QM31; 1] = (*triple_sum32_res_limb_1_col121.try_into().unwrap()).unbox();let [triple_sum32_res_limb_0_col122]: [QM31; 1] = (*triple_sum32_res_limb_0_col122.try_into().unwrap()).unbox();let [triple_sum32_res_limb_1_col123]: [QM31; 1] = (*triple_sum32_res_limb_1_col123.try_into().unwrap()).unbox();let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();


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
        );let split_16_low_part_size_8_output_tmp_ce7d8_3_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_10_col10,
ms_8_bits_col54,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_5_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_11_col11,
ms_8_bits_col55,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_7_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_12_col12,
ms_8_bits_col56,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_9_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_13_col13,
ms_8_bits_col57,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_3_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_7_limb_0],
and_col58,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_2,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col54, ms_8_bits_col56],
and_col59,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_3,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_5_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_9_limb_0],
and_col60,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_4,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col55, ms_8_bits_col57],
and_col61,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_5,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_not_num_bits_16_evaluate(
            input_limb_10_col10,
not_a_col62,
self.verify_bitwise_not_16_lookup_elements,
ref verify_bitwise_not_16_sum_6,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_not_num_bits_16_evaluate(
            input_limb_11_col11,
not_a_col63,
self.verify_bitwise_not_16_lookup_elements,
ref verify_bitwise_not_16_sum_7,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_23_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            not_a_col62,
ms_8_bits_col64,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_25_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            not_a_col63,
ms_8_bits_col65,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_27_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_14_col14,
ms_8_bits_col66,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_29_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_15_col15,
ms_8_bits_col67,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_23_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_27_limb_0],
and_col68,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_8,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col64, ms_8_bits_col66],
and_col69,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_9,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_25_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_29_limb_0],
and_col70,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_10,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col65, ms_8_bits_col67],
and_col71,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_11,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col58, and_col68],
xor_col72,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_12,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col59, and_col69],
xor_col73,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_13,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col60, and_col70],
xor_col74,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_14,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col61, and_col71],
xor_col75,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_15,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        // Constraint - chl
        let constraint_quotient = ((chl_col76 - (xor_col72 + (xor_col73 * qm31_const::<256, 0, 0, 0>())))) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - chh
        let constraint_quotient = ((chh_col77 - (xor_col74 + (xor_col75 * qm31_const::<256, 0, 0, 0>())))) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        sha_256_k_table_sum_16 = self.sha_256_k_table_lookup_elements.combine_qm31(
            [
                input_limb_1_col1,
sha_256_k_table_output_limb_0_col80,
sha_256_k_table_output_limb_1_col81
            ],
        );triple_sum_32_evaluate(
            [input_limb_16_col16, input_limb_17_col17, sha_256_big_sigma_1_output_limb_0_col50, sha_256_big_sigma_1_output_limb_1_col51, ch_limb_0_col78, ch_limb_1_col79],
triple_sum32_res_limb_0_col82,
triple_sum32_res_limb_1_col83,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );triple_sum_32_evaluate(
            [triple_sum32_res_limb_0_col82, triple_sum32_res_limb_1_col83, sha_256_k_table_output_limb_0_col80, sha_256_k_table_output_limb_1_col81, input_limb_18_col18, input_limb_19_col19],
triple_sum32_res_limb_0_col84,
triple_sum32_res_limb_1_col85,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_57_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_2_col2,
ms_8_bits_col86,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_59_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_3_col3,
ms_8_bits_col87,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_61_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_4_col4,
ms_8_bits_col88,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_63_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_5_col5,
ms_8_bits_col89,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_65_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_6_col6,
ms_8_bits_col90,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );let split_16_low_part_size_8_output_tmp_ce7d8_67_limb_0: QM31 = split_16_low_part_size_8_evaluate(
            input_limb_7_col7,
ms_8_bits_col91,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_57_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_61_limb_0],
and_col92,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_17,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col86, ms_8_bits_col88],
and_col93,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_18,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_59_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_63_limb_0],
and_col94,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_19,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col87, ms_8_bits_col89],
and_col95,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_20,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_57_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_65_limb_0],
and_col96,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_21,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col86, ms_8_bits_col90],
and_col97,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_22,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_59_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_67_limb_0],
and_col98,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_23,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col87, ms_8_bits_col91],
and_col99,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_24,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_61_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_65_limb_0],
and_col100,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_25,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col88, ms_8_bits_col90],
and_col101,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_26,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [split_16_low_part_size_8_output_tmp_ce7d8_63_limb_0, split_16_low_part_size_8_output_tmp_ce7d8_67_limb_0],
and_col102,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_27,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_8_evaluate(
            [ms_8_bits_col89, ms_8_bits_col91],
and_col103,
self.verify_bitwise_and_8_lookup_elements,
ref verify_bitwise_and_8_sum_28,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col92, and_col96],
xor_col104,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_29,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col93, and_col97],
xor_col105,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_30,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col94, and_col98],
xor_col106,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_31,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [and_col95, and_col99],
xor_col107,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_32,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [xor_col104, and_col100],
xor_col108,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_33,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [xor_col105, and_col101],
xor_col109,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_34,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [xor_col106, and_col102],
xor_col110,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_35,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_8_evaluate(
            [xor_col107, and_col103],
xor_col111,
self.verify_bitwise_xor_8_lookup_elements,
ref verify_bitwise_xor_8_sum_36,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        // Constraint - majl
        let constraint_quotient = ((majl_col112 - (xor_col108 + (xor_col109 * qm31_const::<256, 0, 0, 0>())))) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - majh
        let constraint_quotient = ((majh_col113 - (xor_col110 + (xor_col111 * qm31_const::<256, 0, 0, 0>())))) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;triple_sum_32_evaluate(
            [sha_256_big_sigma_0_output_limb_0_col52, sha_256_big_sigma_0_output_limb_1_col53, maj_limb_0_col114, maj_limb_1_col115, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>()],
triple_sum32_res_limb_0_col116,
triple_sum32_res_limb_1_col117,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_schedule_sum_37 = self.sha_256_schedule_lookup_elements.combine_qm31(
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
sha_256_schedule_output_limb_0_col118,
sha_256_schedule_output_limb_1_col119
            ],
        );triple_sum_32_evaluate(
            [triple_sum32_res_limb_0_col84, triple_sum32_res_limb_1_col85, input_limb_8_col8, input_limb_9_col9, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>()],
triple_sum32_res_limb_0_col120,
triple_sum32_res_limb_1_col121,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );triple_sum_32_evaluate(
            [triple_sum32_res_limb_0_col84, triple_sum32_res_limb_1_col85, triple_sum32_res_limb_0_col116, triple_sum32_res_limb_1_col117, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>()],
triple_sum32_res_limb_0_col122,
triple_sum32_res_limb_1_col123,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_round_sum_38 = self.sha_256_round_lookup_elements.combine_qm31(
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

        sha_256_round_sum_39 = self.sha_256_round_lookup_elements.combine_qm31(
            [
                input_limb_0_col0,
(input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
triple_sum32_res_limb_0_col122,
triple_sum32_res_limb_1_col123,
input_limb_2_col2,
input_limb_3_col3,
input_limb_4_col4,
input_limb_5_col5,
input_limb_6_col6,
input_limb_7_col7,
triple_sum32_res_limb_0_col120,
triple_sum32_res_limb_1_col121,
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
sha_256_schedule_output_limb_0_col118,
sha_256_schedule_output_limb_1_col119
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
verify_bitwise_and_8_sum_2,
verify_bitwise_and_8_sum_3,
verify_bitwise_and_8_sum_4,
verify_bitwise_and_8_sum_5,
verify_bitwise_not_16_sum_6,
verify_bitwise_not_16_sum_7,
verify_bitwise_and_8_sum_8,
verify_bitwise_and_8_sum_9,
verify_bitwise_and_8_sum_10,
verify_bitwise_and_8_sum_11,
verify_bitwise_xor_8_sum_12,
verify_bitwise_xor_8_sum_13,
verify_bitwise_xor_8_sum_14,
verify_bitwise_xor_8_sum_15,
sha_256_k_table_sum_16,
verify_bitwise_and_8_sum_17,
verify_bitwise_and_8_sum_18,
verify_bitwise_and_8_sum_19,
verify_bitwise_and_8_sum_20,
verify_bitwise_and_8_sum_21,
verify_bitwise_and_8_sum_22,
verify_bitwise_and_8_sum_23,
verify_bitwise_and_8_sum_24,
verify_bitwise_and_8_sum_25,
verify_bitwise_and_8_sum_26,
verify_bitwise_and_8_sum_27,
verify_bitwise_and_8_sum_28,
verify_bitwise_xor_8_sum_29,
verify_bitwise_xor_8_sum_30,
verify_bitwise_xor_8_sum_31,
verify_bitwise_xor_8_sum_32,
verify_bitwise_xor_8_sum_33,
verify_bitwise_xor_8_sum_34,
verify_bitwise_xor_8_sum_35,
verify_bitwise_xor_8_sum_36,
sha_256_schedule_sum_37,
sha_256_round_sum_38,
sha_256_round_sum_39
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
verify_bitwise_and_8_sum_2: QM31,
verify_bitwise_and_8_sum_3: QM31,
verify_bitwise_and_8_sum_4: QM31,
verify_bitwise_and_8_sum_5: QM31,
verify_bitwise_not_16_sum_6: QM31,
verify_bitwise_not_16_sum_7: QM31,
verify_bitwise_and_8_sum_8: QM31,
verify_bitwise_and_8_sum_9: QM31,
verify_bitwise_and_8_sum_10: QM31,
verify_bitwise_and_8_sum_11: QM31,
verify_bitwise_xor_8_sum_12: QM31,
verify_bitwise_xor_8_sum_13: QM31,
verify_bitwise_xor_8_sum_14: QM31,
verify_bitwise_xor_8_sum_15: QM31,
sha_256_k_table_sum_16: QM31,
verify_bitwise_and_8_sum_17: QM31,
verify_bitwise_and_8_sum_18: QM31,
verify_bitwise_and_8_sum_19: QM31,
verify_bitwise_and_8_sum_20: QM31,
verify_bitwise_and_8_sum_21: QM31,
verify_bitwise_and_8_sum_22: QM31,
verify_bitwise_and_8_sum_23: QM31,
verify_bitwise_and_8_sum_24: QM31,
verify_bitwise_and_8_sum_25: QM31,
verify_bitwise_and_8_sum_26: QM31,
verify_bitwise_and_8_sum_27: QM31,
verify_bitwise_and_8_sum_28: QM31,
verify_bitwise_xor_8_sum_29: QM31,
verify_bitwise_xor_8_sum_30: QM31,
verify_bitwise_xor_8_sum_31: QM31,
verify_bitwise_xor_8_sum_32: QM31,
verify_bitwise_xor_8_sum_33: QM31,
verify_bitwise_xor_8_sum_34: QM31,
verify_bitwise_xor_8_sum_35: QM31,
verify_bitwise_xor_8_sum_36: QM31,
sha_256_schedule_sum_37: QM31,
sha_256_round_sum_38: QM31,
sha_256_round_sum_39: QM31
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3, trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7, trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11, trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15, trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19, trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23, trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27, trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31, trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35, trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39, trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43, trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47, trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51, trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55, trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59, trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63, trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67, trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71, trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75, trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79]: [Span<QM31>; 80]
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
let [trace_2_col44]: [QM31; 1] = (*trace_2_col44.try_into().unwrap()).unbox();
let [trace_2_col45]: [QM31; 1] = (*trace_2_col45.try_into().unwrap()).unbox();
let [trace_2_col46]: [QM31; 1] = (*trace_2_col46.try_into().unwrap()).unbox();
let [trace_2_col47]: [QM31; 1] = (*trace_2_col47.try_into().unwrap()).unbox();
let [trace_2_col48]: [QM31; 1] = (*trace_2_col48.try_into().unwrap()).unbox();
let [trace_2_col49]: [QM31; 1] = (*trace_2_col49.try_into().unwrap()).unbox();
let [trace_2_col50]: [QM31; 1] = (*trace_2_col50.try_into().unwrap()).unbox();
let [trace_2_col51]: [QM31; 1] = (*trace_2_col51.try_into().unwrap()).unbox();
let [trace_2_col52]: [QM31; 1] = (*trace_2_col52.try_into().unwrap()).unbox();
let [trace_2_col53]: [QM31; 1] = (*trace_2_col53.try_into().unwrap()).unbox();
let [trace_2_col54]: [QM31; 1] = (*trace_2_col54.try_into().unwrap()).unbox();
let [trace_2_col55]: [QM31; 1] = (*trace_2_col55.try_into().unwrap()).unbox();
let [trace_2_col56]: [QM31; 1] = (*trace_2_col56.try_into().unwrap()).unbox();
let [trace_2_col57]: [QM31; 1] = (*trace_2_col57.try_into().unwrap()).unbox();
let [trace_2_col58]: [QM31; 1] = (*trace_2_col58.try_into().unwrap()).unbox();
let [trace_2_col59]: [QM31; 1] = (*trace_2_col59.try_into().unwrap()).unbox();
let [trace_2_col60]: [QM31; 1] = (*trace_2_col60.try_into().unwrap()).unbox();
let [trace_2_col61]: [QM31; 1] = (*trace_2_col61.try_into().unwrap()).unbox();
let [trace_2_col62]: [QM31; 1] = (*trace_2_col62.try_into().unwrap()).unbox();
let [trace_2_col63]: [QM31; 1] = (*trace_2_col63.try_into().unwrap()).unbox();
let [trace_2_col64]: [QM31; 1] = (*trace_2_col64.try_into().unwrap()).unbox();
let [trace_2_col65]: [QM31; 1] = (*trace_2_col65.try_into().unwrap()).unbox();
let [trace_2_col66]: [QM31; 1] = (*trace_2_col66.try_into().unwrap()).unbox();
let [trace_2_col67]: [QM31; 1] = (*trace_2_col67.try_into().unwrap()).unbox();
let [trace_2_col68]: [QM31; 1] = (*trace_2_col68.try_into().unwrap()).unbox();
let [trace_2_col69]: [QM31; 1] = (*trace_2_col69.try_into().unwrap()).unbox();
let [trace_2_col70]: [QM31; 1] = (*trace_2_col70.try_into().unwrap()).unbox();
let [trace_2_col71]: [QM31; 1] = (*trace_2_col71.try_into().unwrap()).unbox();
let [trace_2_col72]: [QM31; 1] = (*trace_2_col72.try_into().unwrap()).unbox();
let [trace_2_col73]: [QM31; 1] = (*trace_2_col73.try_into().unwrap()).unbox();
let [trace_2_col74]: [QM31; 1] = (*trace_2_col74.try_into().unwrap()).unbox();
let [trace_2_col75]: [QM31; 1] = (*trace_2_col75.try_into().unwrap()).unbox();
let [trace_2_col76_neg1, trace_2_col76]: [QM31; 2] = (*trace_2_col76.try_into().unwrap()).unbox();
let [trace_2_col77_neg1, trace_2_col77]: [QM31; 2] = (*trace_2_col77.try_into().unwrap()).unbox();
let [trace_2_col78_neg1, trace_2_col78]: [QM31; 2] = (*trace_2_col78.try_into().unwrap()).unbox();
let [trace_2_col79_neg1, trace_2_col79]: [QM31; 2] = (*trace_2_col79.try_into().unwrap()).unbox();


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
            ) * verify_bitwise_and_8_sum_2 * verify_bitwise_and_8_sum_3
        ) - verify_bitwise_and_8_sum_2 - verify_bitwise_and_8_sum_3
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]) 
                - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
            ) * verify_bitwise_and_8_sum_4 * verify_bitwise_and_8_sum_5
        ) - verify_bitwise_and_8_sum_4 - verify_bitwise_and_8_sum_5
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15]) 
                - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
            ) * verify_bitwise_not_16_sum_6 * verify_bitwise_not_16_sum_7
        ) - verify_bitwise_not_16_sum_6 - verify_bitwise_not_16_sum_7
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19]) 
                - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
            ) * verify_bitwise_and_8_sum_8 * verify_bitwise_and_8_sum_9
        ) - verify_bitwise_and_8_sum_8 - verify_bitwise_and_8_sum_9
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23]) 
                - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19])
            ) * verify_bitwise_and_8_sum_10 * verify_bitwise_and_8_sum_11
        ) - verify_bitwise_and_8_sum_10 - verify_bitwise_and_8_sum_11
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27]) 
                - QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23])
            ) * verify_bitwise_xor_8_sum_12 * verify_bitwise_xor_8_sum_13
        ) - verify_bitwise_xor_8_sum_12 - verify_bitwise_xor_8_sum_13
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31]) 
                - QM31Impl::from_partial_evals([trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27])
            ) * verify_bitwise_xor_8_sum_14 * verify_bitwise_xor_8_sum_15
        ) - verify_bitwise_xor_8_sum_14 - verify_bitwise_xor_8_sum_15
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35]) 
                - QM31Impl::from_partial_evals([trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31])
            ) * sha_256_k_table_sum_16 * verify_bitwise_and_8_sum_17
        ) - sha_256_k_table_sum_16 - verify_bitwise_and_8_sum_17
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39]) 
                - QM31Impl::from_partial_evals([trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35])
            ) * verify_bitwise_and_8_sum_18 * verify_bitwise_and_8_sum_19
        ) - verify_bitwise_and_8_sum_18 - verify_bitwise_and_8_sum_19
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43]) 
                - QM31Impl::from_partial_evals([trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39])
            ) * verify_bitwise_and_8_sum_20 * verify_bitwise_and_8_sum_21
        ) - verify_bitwise_and_8_sum_20 - verify_bitwise_and_8_sum_21
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47]) 
                - QM31Impl::from_partial_evals([trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43])
            ) * verify_bitwise_and_8_sum_22 * verify_bitwise_and_8_sum_23
        ) - verify_bitwise_and_8_sum_22 - verify_bitwise_and_8_sum_23
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51]) 
                - QM31Impl::from_partial_evals([trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47])
            ) * verify_bitwise_and_8_sum_24 * verify_bitwise_and_8_sum_25
        ) - verify_bitwise_and_8_sum_24 - verify_bitwise_and_8_sum_25
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55]) 
                - QM31Impl::from_partial_evals([trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51])
            ) * verify_bitwise_and_8_sum_26 * verify_bitwise_and_8_sum_27
        ) - verify_bitwise_and_8_sum_26 - verify_bitwise_and_8_sum_27
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59]) 
                - QM31Impl::from_partial_evals([trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55])
            ) * verify_bitwise_and_8_sum_28 * verify_bitwise_xor_8_sum_29
        ) - verify_bitwise_and_8_sum_28 - verify_bitwise_xor_8_sum_29
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63]) 
                - QM31Impl::from_partial_evals([trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59])
            ) * verify_bitwise_xor_8_sum_30 * verify_bitwise_xor_8_sum_31
        ) - verify_bitwise_xor_8_sum_30 - verify_bitwise_xor_8_sum_31
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67]) 
                - QM31Impl::from_partial_evals([trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63])
            ) * verify_bitwise_xor_8_sum_32 * verify_bitwise_xor_8_sum_33
        ) - verify_bitwise_xor_8_sum_32 - verify_bitwise_xor_8_sum_33
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71]) 
                - QM31Impl::from_partial_evals([trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67])
            ) * verify_bitwise_xor_8_sum_34 * verify_bitwise_xor_8_sum_35
        ) - verify_bitwise_xor_8_sum_34 - verify_bitwise_xor_8_sum_35
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75]) 
                - QM31Impl::from_partial_evals([trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71])
            ) * verify_bitwise_xor_8_sum_36 * sha_256_schedule_sum_37
        ) - verify_bitwise_xor_8_sum_36 - sha_256_schedule_sum_37
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79]) 
                - QM31Impl::from_partial_evals([trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75]) 
                - QM31Impl::from_partial_evals([trace_2_col76_neg1, trace_2_col77_neg1, trace_2_col78_neg1, trace_2_col79_neg1])
                + (claimed_sum * (column_size.inverse().into()))
            ) * sha_256_round_sum_38 * sha_256_round_sum_39
        ) + (sha_256_round_sum_38 * enabler) - (sha_256_round_sum_39 * enabler)
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

}
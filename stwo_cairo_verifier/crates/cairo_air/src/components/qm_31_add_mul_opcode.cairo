// Constraints version: 9330aaaf

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
use crate::components::subroutines::decode_instruction_3802d::decode_instruction_3802d_evaluate;
use crate::components::subroutines::qm_31_read_reduced::qm_31_read_reduced_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 73;

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
        let interaction_log_sizes = ArrayImpl::new_repeated(24, log_size).span();
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
    pub range_check_4_4_4_4_lookup_elements: crate::RangeCheck_4_4_4_4Elements,
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
        let mut range_check_4_4_4_4_sum_3: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_4: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_5: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_6: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_7: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_8: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_9: QM31 = Zero::zero();
        let mut opcodes_sum_10: QM31 = Zero::zero();
        let mut opcodes_sum_11: QM31 = Zero::zero();

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            res_add_col10,
            ap_update_add_1_col11,
            mem_dst_base_col12,
            mem0_base_col13,
            mem1_base_col14,
            dst_id_col15,
            dst_limb_0_col16,
            dst_limb_1_col17,
            dst_limb_2_col18,
            dst_limb_3_col19,
            dst_limb_4_col20,
            dst_limb_5_col21,
            dst_limb_6_col22,
            dst_limb_7_col23,
            dst_limb_8_col24,
            dst_limb_9_col25,
            dst_limb_10_col26,
            dst_limb_11_col27,
            dst_limb_12_col28,
            dst_limb_13_col29,
            dst_limb_14_col30,
            dst_limb_15_col31,
            dst_delta_ab_inv_col32,
            dst_delta_cd_inv_col33,
            op0_id_col34,
            op0_limb_0_col35,
            op0_limb_1_col36,
            op0_limb_2_col37,
            op0_limb_3_col38,
            op0_limb_4_col39,
            op0_limb_5_col40,
            op0_limb_6_col41,
            op0_limb_7_col42,
            op0_limb_8_col43,
            op0_limb_9_col44,
            op0_limb_10_col45,
            op0_limb_11_col46,
            op0_limb_12_col47,
            op0_limb_13_col48,
            op0_limb_14_col49,
            op0_limb_15_col50,
            op0_delta_ab_inv_col51,
            op0_delta_cd_inv_col52,
            op1_id_col53,
            op1_limb_0_col54,
            op1_limb_1_col55,
            op1_limb_2_col56,
            op1_limb_3_col57,
            op1_limb_4_col58,
            op1_limb_5_col59,
            op1_limb_6_col60,
            op1_limb_7_col61,
            op1_limb_8_col62,
            op1_limb_9_col63,
            op1_limb_10_col64,
            op1_limb_11_col65,
            op1_limb_12_col66,
            op1_limb_13_col67,
            op1_limb_14_col68,
            op1_limb_15_col69,
            op1_delta_ab_inv_col70,
            op1_delta_cd_inv_col71,
            enabler,
        ]: [Span<QM31>; 73] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset0_col3]: [QM31; 1] = (*offset0_col3.try_into().unwrap()).unbox();
        let [offset1_col4]: [QM31; 1] = (*offset1_col4.try_into().unwrap()).unbox();
        let [offset2_col5]: [QM31; 1] = (*offset2_col5.try_into().unwrap()).unbox();
        let [dst_base_fp_col6]: [QM31; 1] = (*dst_base_fp_col6.try_into().unwrap()).unbox();
        let [op0_base_fp_col7]: [QM31; 1] = (*op0_base_fp_col7.try_into().unwrap()).unbox();
        let [op1_imm_col8]: [QM31; 1] = (*op1_imm_col8.try_into().unwrap()).unbox();
        let [op1_base_fp_col9]: [QM31; 1] = (*op1_base_fp_col9.try_into().unwrap()).unbox();
        let [res_add_col10]: [QM31; 1] = (*res_add_col10.try_into().unwrap()).unbox();
        let [ap_update_add_1_col11]: [QM31; 1] = (*ap_update_add_1_col11.try_into().unwrap())
            .unbox();
        let [mem_dst_base_col12]: [QM31; 1] = (*mem_dst_base_col12.try_into().unwrap()).unbox();
        let [mem0_base_col13]: [QM31; 1] = (*mem0_base_col13.try_into().unwrap()).unbox();
        let [mem1_base_col14]: [QM31; 1] = (*mem1_base_col14.try_into().unwrap()).unbox();
        let [dst_id_col15]: [QM31; 1] = (*dst_id_col15.try_into().unwrap()).unbox();
        let [dst_limb_0_col16]: [QM31; 1] = (*dst_limb_0_col16.try_into().unwrap()).unbox();
        let [dst_limb_1_col17]: [QM31; 1] = (*dst_limb_1_col17.try_into().unwrap()).unbox();
        let [dst_limb_2_col18]: [QM31; 1] = (*dst_limb_2_col18.try_into().unwrap()).unbox();
        let [dst_limb_3_col19]: [QM31; 1] = (*dst_limb_3_col19.try_into().unwrap()).unbox();
        let [dst_limb_4_col20]: [QM31; 1] = (*dst_limb_4_col20.try_into().unwrap()).unbox();
        let [dst_limb_5_col21]: [QM31; 1] = (*dst_limb_5_col21.try_into().unwrap()).unbox();
        let [dst_limb_6_col22]: [QM31; 1] = (*dst_limb_6_col22.try_into().unwrap()).unbox();
        let [dst_limb_7_col23]: [QM31; 1] = (*dst_limb_7_col23.try_into().unwrap()).unbox();
        let [dst_limb_8_col24]: [QM31; 1] = (*dst_limb_8_col24.try_into().unwrap()).unbox();
        let [dst_limb_9_col25]: [QM31; 1] = (*dst_limb_9_col25.try_into().unwrap()).unbox();
        let [dst_limb_10_col26]: [QM31; 1] = (*dst_limb_10_col26.try_into().unwrap()).unbox();
        let [dst_limb_11_col27]: [QM31; 1] = (*dst_limb_11_col27.try_into().unwrap()).unbox();
        let [dst_limb_12_col28]: [QM31; 1] = (*dst_limb_12_col28.try_into().unwrap()).unbox();
        let [dst_limb_13_col29]: [QM31; 1] = (*dst_limb_13_col29.try_into().unwrap()).unbox();
        let [dst_limb_14_col30]: [QM31; 1] = (*dst_limb_14_col30.try_into().unwrap()).unbox();
        let [dst_limb_15_col31]: [QM31; 1] = (*dst_limb_15_col31.try_into().unwrap()).unbox();
        let [dst_delta_ab_inv_col32]: [QM31; 1] = (*dst_delta_ab_inv_col32.try_into().unwrap())
            .unbox();
        let [dst_delta_cd_inv_col33]: [QM31; 1] = (*dst_delta_cd_inv_col33.try_into().unwrap())
            .unbox();
        let [op0_id_col34]: [QM31; 1] = (*op0_id_col34.try_into().unwrap()).unbox();
        let [op0_limb_0_col35]: [QM31; 1] = (*op0_limb_0_col35.try_into().unwrap()).unbox();
        let [op0_limb_1_col36]: [QM31; 1] = (*op0_limb_1_col36.try_into().unwrap()).unbox();
        let [op0_limb_2_col37]: [QM31; 1] = (*op0_limb_2_col37.try_into().unwrap()).unbox();
        let [op0_limb_3_col38]: [QM31; 1] = (*op0_limb_3_col38.try_into().unwrap()).unbox();
        let [op0_limb_4_col39]: [QM31; 1] = (*op0_limb_4_col39.try_into().unwrap()).unbox();
        let [op0_limb_5_col40]: [QM31; 1] = (*op0_limb_5_col40.try_into().unwrap()).unbox();
        let [op0_limb_6_col41]: [QM31; 1] = (*op0_limb_6_col41.try_into().unwrap()).unbox();
        let [op0_limb_7_col42]: [QM31; 1] = (*op0_limb_7_col42.try_into().unwrap()).unbox();
        let [op0_limb_8_col43]: [QM31; 1] = (*op0_limb_8_col43.try_into().unwrap()).unbox();
        let [op0_limb_9_col44]: [QM31; 1] = (*op0_limb_9_col44.try_into().unwrap()).unbox();
        let [op0_limb_10_col45]: [QM31; 1] = (*op0_limb_10_col45.try_into().unwrap()).unbox();
        let [op0_limb_11_col46]: [QM31; 1] = (*op0_limb_11_col46.try_into().unwrap()).unbox();
        let [op0_limb_12_col47]: [QM31; 1] = (*op0_limb_12_col47.try_into().unwrap()).unbox();
        let [op0_limb_13_col48]: [QM31; 1] = (*op0_limb_13_col48.try_into().unwrap()).unbox();
        let [op0_limb_14_col49]: [QM31; 1] = (*op0_limb_14_col49.try_into().unwrap()).unbox();
        let [op0_limb_15_col50]: [QM31; 1] = (*op0_limb_15_col50.try_into().unwrap()).unbox();
        let [op0_delta_ab_inv_col51]: [QM31; 1] = (*op0_delta_ab_inv_col51.try_into().unwrap())
            .unbox();
        let [op0_delta_cd_inv_col52]: [QM31; 1] = (*op0_delta_cd_inv_col52.try_into().unwrap())
            .unbox();
        let [op1_id_col53]: [QM31; 1] = (*op1_id_col53.try_into().unwrap()).unbox();
        let [op1_limb_0_col54]: [QM31; 1] = (*op1_limb_0_col54.try_into().unwrap()).unbox();
        let [op1_limb_1_col55]: [QM31; 1] = (*op1_limb_1_col55.try_into().unwrap()).unbox();
        let [op1_limb_2_col56]: [QM31; 1] = (*op1_limb_2_col56.try_into().unwrap()).unbox();
        let [op1_limb_3_col57]: [QM31; 1] = (*op1_limb_3_col57.try_into().unwrap()).unbox();
        let [op1_limb_4_col58]: [QM31; 1] = (*op1_limb_4_col58.try_into().unwrap()).unbox();
        let [op1_limb_5_col59]: [QM31; 1] = (*op1_limb_5_col59.try_into().unwrap()).unbox();
        let [op1_limb_6_col60]: [QM31; 1] = (*op1_limb_6_col60.try_into().unwrap()).unbox();
        let [op1_limb_7_col61]: [QM31; 1] = (*op1_limb_7_col61.try_into().unwrap()).unbox();
        let [op1_limb_8_col62]: [QM31; 1] = (*op1_limb_8_col62.try_into().unwrap()).unbox();
        let [op1_limb_9_col63]: [QM31; 1] = (*op1_limb_9_col63.try_into().unwrap()).unbox();
        let [op1_limb_10_col64]: [QM31; 1] = (*op1_limb_10_col64.try_into().unwrap()).unbox();
        let [op1_limb_11_col65]: [QM31; 1] = (*op1_limb_11_col65.try_into().unwrap()).unbox();
        let [op1_limb_12_col66]: [QM31; 1] = (*op1_limb_12_col66.try_into().unwrap()).unbox();
        let [op1_limb_13_col67]: [QM31; 1] = (*op1_limb_13_col67.try_into().unwrap()).unbox();
        let [op1_limb_14_col68]: [QM31; 1] = (*op1_limb_14_col68.try_into().unwrap()).unbox();
        let [op1_limb_15_col69]: [QM31; 1] = (*op1_limb_15_col69.try_into().unwrap()).unbox();
        let [op1_delta_ab_inv_col70]: [QM31; 1] = (*op1_delta_ab_inv_col70.try_into().unwrap())
            .unbox();
        let [op1_delta_cd_inv_col71]: [QM31; 1] = (*op1_delta_cd_inv_col71.try_into().unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 5] = decode_instruction_3802d_evaluate(
            [input_pc_col0],
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            res_add_col10,
            ap_update_add_1_col11,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            decode_instruction_3802d_output_tmp_fa85a_11_offset0,
            decode_instruction_3802d_output_tmp_fa85a_11_offset1,
            decode_instruction_3802d_output_tmp_fa85a_11_offset2,
            decode_instruction_3802d_output_tmp_fa85a_11_op1_base_ap,
            decode_instruction_3802d_output_tmp_fa85a_11_res_mul,
        ] =
            output;

        // Constraint - Either flag op1_imm is off or offset2 is equal to 1
        let constraint_quotient = ((op1_imm_col8
            * (decode_instruction_3802d_output_tmp_fa85a_11_offset2 - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem_dst_base
        let constraint_quotient = ((mem_dst_base_col12
            - ((dst_base_fp_col6 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col6) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem0_base
        let constraint_quotient = ((mem0_base_col13
            - ((op0_base_fp_col7 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col7) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem1_base
        let constraint_quotient = ((mem1_base_col14
            - (((op1_base_fp_col9 * input_fp_col2)
                + (decode_instruction_3802d_output_tmp_fa85a_11_op1_base_ap * input_ap_col1))
                + (op1_imm_col8 * input_pc_col0))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 4] = qm_31_read_reduced_evaluate(
            [(mem_dst_base_col12 + decode_instruction_3802d_output_tmp_fa85a_11_offset0)],
            dst_id_col15,
            dst_limb_0_col16,
            dst_limb_1_col17,
            dst_limb_2_col18,
            dst_limb_3_col19,
            dst_limb_4_col20,
            dst_limb_5_col21,
            dst_limb_6_col22,
            dst_limb_7_col23,
            dst_limb_8_col24,
            dst_limb_9_col25,
            dst_limb_10_col26,
            dst_limb_11_col27,
            dst_limb_12_col28,
            dst_limb_13_col29,
            dst_limb_14_col30,
            dst_limb_15_col31,
            dst_delta_ab_inv_col32,
            dst_delta_cd_inv_col33,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref range_check_4_4_4_4_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            qm_31_read_reduced_output_tmp_fa85a_15_limb_0,
            qm_31_read_reduced_output_tmp_fa85a_15_limb_1,
            qm_31_read_reduced_output_tmp_fa85a_15_limb_2,
            qm_31_read_reduced_output_tmp_fa85a_15_limb_3,
        ] =
            output;

        let output: [QM31; 4] = qm_31_read_reduced_evaluate(
            [(mem0_base_col13 + decode_instruction_3802d_output_tmp_fa85a_11_offset1)],
            op0_id_col34,
            op0_limb_0_col35,
            op0_limb_1_col36,
            op0_limb_2_col37,
            op0_limb_3_col38,
            op0_limb_4_col39,
            op0_limb_5_col40,
            op0_limb_6_col41,
            op0_limb_7_col42,
            op0_limb_8_col43,
            op0_limb_9_col44,
            op0_limb_10_col45,
            op0_limb_11_col46,
            op0_limb_12_col47,
            op0_limb_13_col48,
            op0_limb_14_col49,
            op0_limb_15_col50,
            op0_delta_ab_inv_col51,
            op0_delta_cd_inv_col52,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            ref memory_address_to_id_sum_4,
            ref memory_id_to_big_sum_5,
            ref range_check_4_4_4_4_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            qm_31_read_reduced_output_tmp_fa85a_19_limb_0,
            qm_31_read_reduced_output_tmp_fa85a_19_limb_1,
            qm_31_read_reduced_output_tmp_fa85a_19_limb_2,
            qm_31_read_reduced_output_tmp_fa85a_19_limb_3,
        ] =
            output;

        let output: [QM31; 4] = qm_31_read_reduced_evaluate(
            [(mem1_base_col14 + decode_instruction_3802d_output_tmp_fa85a_11_offset2)],
            op1_id_col53,
            op1_limb_0_col54,
            op1_limb_1_col55,
            op1_limb_2_col56,
            op1_limb_3_col57,
            op1_limb_4_col58,
            op1_limb_5_col59,
            op1_limb_6_col60,
            op1_limb_7_col61,
            op1_limb_8_col62,
            op1_limb_9_col63,
            op1_limb_10_col64,
            op1_limb_11_col65,
            op1_limb_12_col66,
            op1_limb_13_col67,
            op1_limb_14_col68,
            op1_limb_15_col69,
            op1_delta_ab_inv_col70,
            op1_delta_cd_inv_col71,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            ref memory_address_to_id_sum_7,
            ref memory_id_to_big_sum_8,
            ref range_check_4_4_4_4_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            qm_31_read_reduced_output_tmp_fa85a_23_limb_0,
            qm_31_read_reduced_output_tmp_fa85a_23_limb_1,
            qm_31_read_reduced_output_tmp_fa85a_23_limb_2,
            qm_31_read_reduced_output_tmp_fa85a_23_limb_3,
        ] =
            output;

        // Constraint - dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul)
        let constraint_quotient = (((qm_31_read_reduced_output_tmp_fa85a_15_limb_0
            - ((((((qm_31_read_reduced_output_tmp_fa85a_19_limb_0
                * qm_31_read_reduced_output_tmp_fa85a_23_limb_0)
                - (qm_31_read_reduced_output_tmp_fa85a_19_limb_1
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_1))
                + (qm31_const::<2, 0, 0, 0>()
                    * ((qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                        * qm_31_read_reduced_output_tmp_fa85a_23_limb_2)
                        - (qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                            * qm_31_read_reduced_output_tmp_fa85a_23_limb_3))))
                - (qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_3))
                - (qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_2))
                * decode_instruction_3802d_output_tmp_fa85a_11_res_mul))
            - ((qm_31_read_reduced_output_tmp_fa85a_19_limb_0
                + qm_31_read_reduced_output_tmp_fa85a_23_limb_0)
                * res_add_col10)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul)
        let constraint_quotient = (((qm_31_read_reduced_output_tmp_fa85a_15_limb_1
            - ((((((qm_31_read_reduced_output_tmp_fa85a_19_limb_0
                * qm_31_read_reduced_output_tmp_fa85a_23_limb_1)
                + (qm_31_read_reduced_output_tmp_fa85a_19_limb_1
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_0))
                + (qm31_const::<2, 0, 0, 0>()
                    * ((qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                        * qm_31_read_reduced_output_tmp_fa85a_23_limb_3)
                        + (qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                            * qm_31_read_reduced_output_tmp_fa85a_23_limb_2))))
                + (qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_2))
                - (qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_3))
                * decode_instruction_3802d_output_tmp_fa85a_11_res_mul))
            - ((qm_31_read_reduced_output_tmp_fa85a_19_limb_1
                + qm_31_read_reduced_output_tmp_fa85a_23_limb_1)
                * res_add_col10)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul)
        let constraint_quotient = (((qm_31_read_reduced_output_tmp_fa85a_15_limb_2
            - (((((qm_31_read_reduced_output_tmp_fa85a_19_limb_0
                * qm_31_read_reduced_output_tmp_fa85a_23_limb_2)
                - (qm_31_read_reduced_output_tmp_fa85a_19_limb_1
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_3))
                + (qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_0))
                - (qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_1))
                * decode_instruction_3802d_output_tmp_fa85a_11_res_mul))
            - ((qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                + qm_31_read_reduced_output_tmp_fa85a_23_limb_2)
                * res_add_col10)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - dst equals (op0 * op1)*flag_res_mul + (op0 + op1)*(1-flag_res_mul)
        let constraint_quotient = (((qm_31_read_reduced_output_tmp_fa85a_15_limb_3
            - (((((qm_31_read_reduced_output_tmp_fa85a_19_limb_0
                * qm_31_read_reduced_output_tmp_fa85a_23_limb_3)
                + (qm_31_read_reduced_output_tmp_fa85a_19_limb_1
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_2))
                + (qm_31_read_reduced_output_tmp_fa85a_19_limb_2
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_1))
                + (qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                    * qm_31_read_reduced_output_tmp_fa85a_23_limb_0))
                * decode_instruction_3802d_output_tmp_fa85a_11_res_mul))
            - ((qm_31_read_reduced_output_tmp_fa85a_19_limb_3
                + qm_31_read_reduced_output_tmp_fa85a_23_limb_3)
                * res_add_col10)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        opcodes_sum_10 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_11 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    ((input_pc_col0 + qm31_const::<1, 0, 0, 0>()) + op1_imm_col8),
                    (input_ap_col1 + ap_update_add_1_col11), input_fp_col2,
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
            range_check_4_4_4_4_sum_3,
            memory_address_to_id_sum_4,
            memory_id_to_big_sum_5,
            range_check_4_4_4_4_sum_6,
            memory_address_to_id_sum_7,
            memory_id_to_big_sum_8,
            range_check_4_4_4_4_sum_9,
            opcodes_sum_10,
            opcodes_sum_11,
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
    range_check_4_4_4_4_sum_3: QM31,
    memory_address_to_id_sum_4: QM31,
    memory_id_to_big_sum_5: QM31,
    range_check_4_4_4_4_sum_6: QM31,
    memory_address_to_id_sum_7: QM31,
    memory_id_to_big_sum_8: QM31,
    range_check_4_4_4_4_sum_9: QM31,
    opcodes_sum_10: QM31,
    opcodes_sum_11: QM31,
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
    ]: [Span<QM31>; 24] =
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
    let [trace_2_col20_neg1, trace_2_col20]: [QM31; 2] = (*trace_2_col20.try_into().unwrap())
        .unbox();
    let [trace_2_col21_neg1, trace_2_col21]: [QM31; 2] = (*trace_2_col21.try_into().unwrap())
        .unbox();
    let [trace_2_col22_neg1, trace_2_col22]: [QM31; 2] = (*trace_2_col22.try_into().unwrap())
        .unbox();
    let [trace_2_col23_neg1, trace_2_col23]: [QM31; 2] = (*trace_2_col23.try_into().unwrap())
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
        * range_check_4_4_4_4_sum_3)
        - memory_id_to_big_sum_2
        - range_check_4_4_4_4_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * memory_address_to_id_sum_4
        * memory_id_to_big_sum_5)
        - memory_address_to_id_sum_4
        - memory_id_to_big_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_4_4_4_4_sum_6
        * memory_address_to_id_sum_7)
        - range_check_4_4_4_4_sum_6
        - memory_address_to_id_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * memory_id_to_big_sum_8
        * range_check_4_4_4_4_sum_9)
        - memory_id_to_big_sum_8
        - range_check_4_4_4_4_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19])
        - QM31Impl::from_partial_evals(
            [trace_2_col20_neg1, trace_2_col21_neg1, trace_2_col22_neg1, trace_2_col23_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_10
        * opcodes_sum_11)
        + (opcodes_sum_10 * enabler)
        - (opcodes_sum_11 * enabler))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

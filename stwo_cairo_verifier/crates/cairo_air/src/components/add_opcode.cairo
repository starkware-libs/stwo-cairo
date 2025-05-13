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
use crate::components::subroutines::decode_instruction_bc3cd::decode_instruction_bc3cd_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::components::subroutines::verify_add_252::verify_add_252_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 103;

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
        let interaction_log_sizes = ArrayImpl::new_repeated(20, log_size).span();
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
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            ap_update_add_1_col10,
            mem_dst_base_col11,
            mem0_base_col12,
            mem1_base_col13,
            dst_id_col14,
            dst_limb_0_col15,
            dst_limb_1_col16,
            dst_limb_2_col17,
            dst_limb_3_col18,
            dst_limb_4_col19,
            dst_limb_5_col20,
            dst_limb_6_col21,
            dst_limb_7_col22,
            dst_limb_8_col23,
            dst_limb_9_col24,
            dst_limb_10_col25,
            dst_limb_11_col26,
            dst_limb_12_col27,
            dst_limb_13_col28,
            dst_limb_14_col29,
            dst_limb_15_col30,
            dst_limb_16_col31,
            dst_limb_17_col32,
            dst_limb_18_col33,
            dst_limb_19_col34,
            dst_limb_20_col35,
            dst_limb_21_col36,
            dst_limb_22_col37,
            dst_limb_23_col38,
            dst_limb_24_col39,
            dst_limb_25_col40,
            dst_limb_26_col41,
            dst_limb_27_col42,
            op0_id_col43,
            op0_limb_0_col44,
            op0_limb_1_col45,
            op0_limb_2_col46,
            op0_limb_3_col47,
            op0_limb_4_col48,
            op0_limb_5_col49,
            op0_limb_6_col50,
            op0_limb_7_col51,
            op0_limb_8_col52,
            op0_limb_9_col53,
            op0_limb_10_col54,
            op0_limb_11_col55,
            op0_limb_12_col56,
            op0_limb_13_col57,
            op0_limb_14_col58,
            op0_limb_15_col59,
            op0_limb_16_col60,
            op0_limb_17_col61,
            op0_limb_18_col62,
            op0_limb_19_col63,
            op0_limb_20_col64,
            op0_limb_21_col65,
            op0_limb_22_col66,
            op0_limb_23_col67,
            op0_limb_24_col68,
            op0_limb_25_col69,
            op0_limb_26_col70,
            op0_limb_27_col71,
            op1_id_col72,
            op1_limb_0_col73,
            op1_limb_1_col74,
            op1_limb_2_col75,
            op1_limb_3_col76,
            op1_limb_4_col77,
            op1_limb_5_col78,
            op1_limb_6_col79,
            op1_limb_7_col80,
            op1_limb_8_col81,
            op1_limb_9_col82,
            op1_limb_10_col83,
            op1_limb_11_col84,
            op1_limb_12_col85,
            op1_limb_13_col86,
            op1_limb_14_col87,
            op1_limb_15_col88,
            op1_limb_16_col89,
            op1_limb_17_col90,
            op1_limb_18_col91,
            op1_limb_19_col92,
            op1_limb_20_col93,
            op1_limb_21_col94,
            op1_limb_22_col95,
            op1_limb_23_col96,
            op1_limb_24_col97,
            op1_limb_25_col98,
            op1_limb_26_col99,
            op1_limb_27_col100,
            sub_p_bit_col101,
            enabler,
        ]: [Span<QM31>; 103] =
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
        let [ap_update_add_1_col10]: [QM31; 1] = (*ap_update_add_1_col10.try_into().unwrap())
            .unbox();
        let [mem_dst_base_col11]: [QM31; 1] = (*mem_dst_base_col11.try_into().unwrap()).unbox();
        let [mem0_base_col12]: [QM31; 1] = (*mem0_base_col12.try_into().unwrap()).unbox();
        let [mem1_base_col13]: [QM31; 1] = (*mem1_base_col13.try_into().unwrap()).unbox();
        let [dst_id_col14]: [QM31; 1] = (*dst_id_col14.try_into().unwrap()).unbox();
        let [dst_limb_0_col15]: [QM31; 1] = (*dst_limb_0_col15.try_into().unwrap()).unbox();
        let [dst_limb_1_col16]: [QM31; 1] = (*dst_limb_1_col16.try_into().unwrap()).unbox();
        let [dst_limb_2_col17]: [QM31; 1] = (*dst_limb_2_col17.try_into().unwrap()).unbox();
        let [dst_limb_3_col18]: [QM31; 1] = (*dst_limb_3_col18.try_into().unwrap()).unbox();
        let [dst_limb_4_col19]: [QM31; 1] = (*dst_limb_4_col19.try_into().unwrap()).unbox();
        let [dst_limb_5_col20]: [QM31; 1] = (*dst_limb_5_col20.try_into().unwrap()).unbox();
        let [dst_limb_6_col21]: [QM31; 1] = (*dst_limb_6_col21.try_into().unwrap()).unbox();
        let [dst_limb_7_col22]: [QM31; 1] = (*dst_limb_7_col22.try_into().unwrap()).unbox();
        let [dst_limb_8_col23]: [QM31; 1] = (*dst_limb_8_col23.try_into().unwrap()).unbox();
        let [dst_limb_9_col24]: [QM31; 1] = (*dst_limb_9_col24.try_into().unwrap()).unbox();
        let [dst_limb_10_col25]: [QM31; 1] = (*dst_limb_10_col25.try_into().unwrap()).unbox();
        let [dst_limb_11_col26]: [QM31; 1] = (*dst_limb_11_col26.try_into().unwrap()).unbox();
        let [dst_limb_12_col27]: [QM31; 1] = (*dst_limb_12_col27.try_into().unwrap()).unbox();
        let [dst_limb_13_col28]: [QM31; 1] = (*dst_limb_13_col28.try_into().unwrap()).unbox();
        let [dst_limb_14_col29]: [QM31; 1] = (*dst_limb_14_col29.try_into().unwrap()).unbox();
        let [dst_limb_15_col30]: [QM31; 1] = (*dst_limb_15_col30.try_into().unwrap()).unbox();
        let [dst_limb_16_col31]: [QM31; 1] = (*dst_limb_16_col31.try_into().unwrap()).unbox();
        let [dst_limb_17_col32]: [QM31; 1] = (*dst_limb_17_col32.try_into().unwrap()).unbox();
        let [dst_limb_18_col33]: [QM31; 1] = (*dst_limb_18_col33.try_into().unwrap()).unbox();
        let [dst_limb_19_col34]: [QM31; 1] = (*dst_limb_19_col34.try_into().unwrap()).unbox();
        let [dst_limb_20_col35]: [QM31; 1] = (*dst_limb_20_col35.try_into().unwrap()).unbox();
        let [dst_limb_21_col36]: [QM31; 1] = (*dst_limb_21_col36.try_into().unwrap()).unbox();
        let [dst_limb_22_col37]: [QM31; 1] = (*dst_limb_22_col37.try_into().unwrap()).unbox();
        let [dst_limb_23_col38]: [QM31; 1] = (*dst_limb_23_col38.try_into().unwrap()).unbox();
        let [dst_limb_24_col39]: [QM31; 1] = (*dst_limb_24_col39.try_into().unwrap()).unbox();
        let [dst_limb_25_col40]: [QM31; 1] = (*dst_limb_25_col40.try_into().unwrap()).unbox();
        let [dst_limb_26_col41]: [QM31; 1] = (*dst_limb_26_col41.try_into().unwrap()).unbox();
        let [dst_limb_27_col42]: [QM31; 1] = (*dst_limb_27_col42.try_into().unwrap()).unbox();
        let [op0_id_col43]: [QM31; 1] = (*op0_id_col43.try_into().unwrap()).unbox();
        let [op0_limb_0_col44]: [QM31; 1] = (*op0_limb_0_col44.try_into().unwrap()).unbox();
        let [op0_limb_1_col45]: [QM31; 1] = (*op0_limb_1_col45.try_into().unwrap()).unbox();
        let [op0_limb_2_col46]: [QM31; 1] = (*op0_limb_2_col46.try_into().unwrap()).unbox();
        let [op0_limb_3_col47]: [QM31; 1] = (*op0_limb_3_col47.try_into().unwrap()).unbox();
        let [op0_limb_4_col48]: [QM31; 1] = (*op0_limb_4_col48.try_into().unwrap()).unbox();
        let [op0_limb_5_col49]: [QM31; 1] = (*op0_limb_5_col49.try_into().unwrap()).unbox();
        let [op0_limb_6_col50]: [QM31; 1] = (*op0_limb_6_col50.try_into().unwrap()).unbox();
        let [op0_limb_7_col51]: [QM31; 1] = (*op0_limb_7_col51.try_into().unwrap()).unbox();
        let [op0_limb_8_col52]: [QM31; 1] = (*op0_limb_8_col52.try_into().unwrap()).unbox();
        let [op0_limb_9_col53]: [QM31; 1] = (*op0_limb_9_col53.try_into().unwrap()).unbox();
        let [op0_limb_10_col54]: [QM31; 1] = (*op0_limb_10_col54.try_into().unwrap()).unbox();
        let [op0_limb_11_col55]: [QM31; 1] = (*op0_limb_11_col55.try_into().unwrap()).unbox();
        let [op0_limb_12_col56]: [QM31; 1] = (*op0_limb_12_col56.try_into().unwrap()).unbox();
        let [op0_limb_13_col57]: [QM31; 1] = (*op0_limb_13_col57.try_into().unwrap()).unbox();
        let [op0_limb_14_col58]: [QM31; 1] = (*op0_limb_14_col58.try_into().unwrap()).unbox();
        let [op0_limb_15_col59]: [QM31; 1] = (*op0_limb_15_col59.try_into().unwrap()).unbox();
        let [op0_limb_16_col60]: [QM31; 1] = (*op0_limb_16_col60.try_into().unwrap()).unbox();
        let [op0_limb_17_col61]: [QM31; 1] = (*op0_limb_17_col61.try_into().unwrap()).unbox();
        let [op0_limb_18_col62]: [QM31; 1] = (*op0_limb_18_col62.try_into().unwrap()).unbox();
        let [op0_limb_19_col63]: [QM31; 1] = (*op0_limb_19_col63.try_into().unwrap()).unbox();
        let [op0_limb_20_col64]: [QM31; 1] = (*op0_limb_20_col64.try_into().unwrap()).unbox();
        let [op0_limb_21_col65]: [QM31; 1] = (*op0_limb_21_col65.try_into().unwrap()).unbox();
        let [op0_limb_22_col66]: [QM31; 1] = (*op0_limb_22_col66.try_into().unwrap()).unbox();
        let [op0_limb_23_col67]: [QM31; 1] = (*op0_limb_23_col67.try_into().unwrap()).unbox();
        let [op0_limb_24_col68]: [QM31; 1] = (*op0_limb_24_col68.try_into().unwrap()).unbox();
        let [op0_limb_25_col69]: [QM31; 1] = (*op0_limb_25_col69.try_into().unwrap()).unbox();
        let [op0_limb_26_col70]: [QM31; 1] = (*op0_limb_26_col70.try_into().unwrap()).unbox();
        let [op0_limb_27_col71]: [QM31; 1] = (*op0_limb_27_col71.try_into().unwrap()).unbox();
        let [op1_id_col72]: [QM31; 1] = (*op1_id_col72.try_into().unwrap()).unbox();
        let [op1_limb_0_col73]: [QM31; 1] = (*op1_limb_0_col73.try_into().unwrap()).unbox();
        let [op1_limb_1_col74]: [QM31; 1] = (*op1_limb_1_col74.try_into().unwrap()).unbox();
        let [op1_limb_2_col75]: [QM31; 1] = (*op1_limb_2_col75.try_into().unwrap()).unbox();
        let [op1_limb_3_col76]: [QM31; 1] = (*op1_limb_3_col76.try_into().unwrap()).unbox();
        let [op1_limb_4_col77]: [QM31; 1] = (*op1_limb_4_col77.try_into().unwrap()).unbox();
        let [op1_limb_5_col78]: [QM31; 1] = (*op1_limb_5_col78.try_into().unwrap()).unbox();
        let [op1_limb_6_col79]: [QM31; 1] = (*op1_limb_6_col79.try_into().unwrap()).unbox();
        let [op1_limb_7_col80]: [QM31; 1] = (*op1_limb_7_col80.try_into().unwrap()).unbox();
        let [op1_limb_8_col81]: [QM31; 1] = (*op1_limb_8_col81.try_into().unwrap()).unbox();
        let [op1_limb_9_col82]: [QM31; 1] = (*op1_limb_9_col82.try_into().unwrap()).unbox();
        let [op1_limb_10_col83]: [QM31; 1] = (*op1_limb_10_col83.try_into().unwrap()).unbox();
        let [op1_limb_11_col84]: [QM31; 1] = (*op1_limb_11_col84.try_into().unwrap()).unbox();
        let [op1_limb_12_col85]: [QM31; 1] = (*op1_limb_12_col85.try_into().unwrap()).unbox();
        let [op1_limb_13_col86]: [QM31; 1] = (*op1_limb_13_col86.try_into().unwrap()).unbox();
        let [op1_limb_14_col87]: [QM31; 1] = (*op1_limb_14_col87.try_into().unwrap()).unbox();
        let [op1_limb_15_col88]: [QM31; 1] = (*op1_limb_15_col88.try_into().unwrap()).unbox();
        let [op1_limb_16_col89]: [QM31; 1] = (*op1_limb_16_col89.try_into().unwrap()).unbox();
        let [op1_limb_17_col90]: [QM31; 1] = (*op1_limb_17_col90.try_into().unwrap()).unbox();
        let [op1_limb_18_col91]: [QM31; 1] = (*op1_limb_18_col91.try_into().unwrap()).unbox();
        let [op1_limb_19_col92]: [QM31; 1] = (*op1_limb_19_col92.try_into().unwrap()).unbox();
        let [op1_limb_20_col93]: [QM31; 1] = (*op1_limb_20_col93.try_into().unwrap()).unbox();
        let [op1_limb_21_col94]: [QM31; 1] = (*op1_limb_21_col94.try_into().unwrap()).unbox();
        let [op1_limb_22_col95]: [QM31; 1] = (*op1_limb_22_col95.try_into().unwrap()).unbox();
        let [op1_limb_23_col96]: [QM31; 1] = (*op1_limb_23_col96.try_into().unwrap()).unbox();
        let [op1_limb_24_col97]: [QM31; 1] = (*op1_limb_24_col97.try_into().unwrap()).unbox();
        let [op1_limb_25_col98]: [QM31; 1] = (*op1_limb_25_col98.try_into().unwrap()).unbox();
        let [op1_limb_26_col99]: [QM31; 1] = (*op1_limb_26_col99.try_into().unwrap()).unbox();
        let [op1_limb_27_col100]: [QM31; 1] = (*op1_limb_27_col100.try_into().unwrap()).unbox();
        let [sub_p_bit_col101]: [QM31; 1] = (*sub_p_bit_col101.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 4] = decode_instruction_bc3cd_evaluate(
            [input_pc_col0],
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            ap_update_add_1_col10,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            decode_instruction_bc3cd_output_tmp_3fa46_10_offset0,
            decode_instruction_bc3cd_output_tmp_3fa46_10_offset1,
            decode_instruction_bc3cd_output_tmp_3fa46_10_offset2,
            decode_instruction_bc3cd_output_tmp_3fa46_10_op1_base_ap,
        ] =
            output;

        // Constraint - if imm then offset2 is 1
        let constraint_quotient = ((op1_imm_col8
            * (qm31_const::<1, 0, 0, 0>() - decode_instruction_bc3cd_output_tmp_3fa46_10_offset2)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem_dst_base
        let constraint_quotient = ((mem_dst_base_col11
            - ((dst_base_fp_col6 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col6) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem0_base
        let constraint_quotient = ((mem0_base_col12
            - ((op0_base_fp_col7 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col7) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem1_base
        let constraint_quotient = ((mem1_base_col13
            - (((op1_imm_col8 * input_pc_col0) + (op1_base_fp_col9 * input_fp_col2))
                + (decode_instruction_bc3cd_output_tmp_3fa46_10_op1_base_ap * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        read_positive_num_bits_252_evaluate(
            [(mem_dst_base_col11 + decode_instruction_bc3cd_output_tmp_3fa46_10_offset0)],
            dst_id_col14,
            dst_limb_0_col15,
            dst_limb_1_col16,
            dst_limb_2_col17,
            dst_limb_3_col18,
            dst_limb_4_col19,
            dst_limb_5_col20,
            dst_limb_6_col21,
            dst_limb_7_col22,
            dst_limb_8_col23,
            dst_limb_9_col24,
            dst_limb_10_col25,
            dst_limb_11_col26,
            dst_limb_12_col27,
            dst_limb_13_col28,
            dst_limb_14_col29,
            dst_limb_15_col30,
            dst_limb_16_col31,
            dst_limb_17_col32,
            dst_limb_18_col33,
            dst_limb_19_col34,
            dst_limb_20_col35,
            dst_limb_21_col36,
            dst_limb_22_col37,
            dst_limb_23_col38,
            dst_limb_24_col39,
            dst_limb_25_col40,
            dst_limb_26_col41,
            dst_limb_27_col42,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_positive_num_bits_252_evaluate(
            [(mem0_base_col12 + decode_instruction_bc3cd_output_tmp_3fa46_10_offset1)],
            op0_id_col43,
            op0_limb_0_col44,
            op0_limb_1_col45,
            op0_limb_2_col46,
            op0_limb_3_col47,
            op0_limb_4_col48,
            op0_limb_5_col49,
            op0_limb_6_col50,
            op0_limb_7_col51,
            op0_limb_8_col52,
            op0_limb_9_col53,
            op0_limb_10_col54,
            op0_limb_11_col55,
            op0_limb_12_col56,
            op0_limb_13_col57,
            op0_limb_14_col58,
            op0_limb_15_col59,
            op0_limb_16_col60,
            op0_limb_17_col61,
            op0_limb_18_col62,
            op0_limb_19_col63,
            op0_limb_20_col64,
            op0_limb_21_col65,
            op0_limb_22_col66,
            op0_limb_23_col67,
            op0_limb_24_col68,
            op0_limb_25_col69,
            op0_limb_26_col70,
            op0_limb_27_col71,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_positive_num_bits_252_evaluate(
            [(mem1_base_col13 + decode_instruction_bc3cd_output_tmp_3fa46_10_offset2)],
            op1_id_col72,
            op1_limb_0_col73,
            op1_limb_1_col74,
            op1_limb_2_col75,
            op1_limb_3_col76,
            op1_limb_4_col77,
            op1_limb_5_col78,
            op1_limb_6_col79,
            op1_limb_7_col80,
            op1_limb_8_col81,
            op1_limb_9_col82,
            op1_limb_10_col83,
            op1_limb_11_col84,
            op1_limb_12_col85,
            op1_limb_13_col86,
            op1_limb_14_col87,
            op1_limb_15_col88,
            op1_limb_16_col89,
            op1_limb_17_col90,
            op1_limb_18_col91,
            op1_limb_19_col92,
            op1_limb_20_col93,
            op1_limb_21_col94,
            op1_limb_22_col95,
            op1_limb_23_col96,
            op1_limb_24_col97,
            op1_limb_25_col98,
            op1_limb_26_col99,
            op1_limb_27_col100,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        core::internal::revoke_ap_tracking();

        verify_add_252_evaluate(
            [
                op0_limb_0_col44, op0_limb_1_col45, op0_limb_2_col46, op0_limb_3_col47,
                op0_limb_4_col48, op0_limb_5_col49, op0_limb_6_col50, op0_limb_7_col51,
                op0_limb_8_col52, op0_limb_9_col53, op0_limb_10_col54, op0_limb_11_col55,
                op0_limb_12_col56, op0_limb_13_col57, op0_limb_14_col58, op0_limb_15_col59,
                op0_limb_16_col60, op0_limb_17_col61, op0_limb_18_col62, op0_limb_19_col63,
                op0_limb_20_col64, op0_limb_21_col65, op0_limb_22_col66, op0_limb_23_col67,
                op0_limb_24_col68, op0_limb_25_col69, op0_limb_26_col70, op0_limb_27_col71,
                op1_limb_0_col73, op1_limb_1_col74, op1_limb_2_col75, op1_limb_3_col76,
                op1_limb_4_col77, op1_limb_5_col78, op1_limb_6_col79, op1_limb_7_col80,
                op1_limb_8_col81, op1_limb_9_col82, op1_limb_10_col83, op1_limb_11_col84,
                op1_limb_12_col85, op1_limb_13_col86, op1_limb_14_col87, op1_limb_15_col88,
                op1_limb_16_col89, op1_limb_17_col90, op1_limb_18_col91, op1_limb_19_col92,
                op1_limb_20_col93, op1_limb_21_col94, op1_limb_22_col95, op1_limb_23_col96,
                op1_limb_24_col97, op1_limb_25_col98, op1_limb_26_col99, op1_limb_27_col100,
                dst_limb_0_col15, dst_limb_1_col16, dst_limb_2_col17, dst_limb_3_col18,
                dst_limb_4_col19, dst_limb_5_col20, dst_limb_6_col21, dst_limb_7_col22,
                dst_limb_8_col23, dst_limb_9_col24, dst_limb_10_col25, dst_limb_11_col26,
                dst_limb_12_col27, dst_limb_13_col28, dst_limb_14_col29, dst_limb_15_col30,
                dst_limb_16_col31, dst_limb_17_col32, dst_limb_18_col33, dst_limb_19_col34,
                dst_limb_20_col35, dst_limb_21_col36, dst_limb_22_col37, dst_limb_23_col38,
                dst_limb_24_col39, dst_limb_25_col40, dst_limb_26_col41, dst_limb_27_col42,
            ],
            sub_p_bit_col101,
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
                    ((input_pc_col0 + qm31_const::<1, 0, 0, 0>()) + op1_imm_col8),
                    (input_ap_col1 + ap_update_add_1_col10), input_fp_col2,
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

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
use crate::components::subroutines::bitwise_xor_num_bits_9::bitwise_xor_num_bits_9_evaluate;
use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 89;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub bitwise_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(76, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.bitwise_builtin_segment_start).into());
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
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub verify_bitwise_xor_9_lookup_elements: crate::VerifyBitwiseXor_9Elements,
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
        preprocessed_column_set.insert(PreprocessedColumn::Seq(*(self.claim.log_size)));
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
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
        let bitwise_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.bitwise_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_7: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_8: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_9: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_10: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_11: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_12: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_13: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_14: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_15: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_16: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_17: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_18: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_19: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_20: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_21: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_22: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_23: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_24: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_25: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_26: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_27: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_28: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_29: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_30: QM31 = Zero::zero();
        let mut verify_bitwise_xor_9_sum_31: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_32: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_33: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_34: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_35: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_36: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_37: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            op0_id_col0,
            op0_limb_0_col1,
            op0_limb_1_col2,
            op0_limb_2_col3,
            op0_limb_3_col4,
            op0_limb_4_col5,
            op0_limb_5_col6,
            op0_limb_6_col7,
            op0_limb_7_col8,
            op0_limb_8_col9,
            op0_limb_9_col10,
            op0_limb_10_col11,
            op0_limb_11_col12,
            op0_limb_12_col13,
            op0_limb_13_col14,
            op0_limb_14_col15,
            op0_limb_15_col16,
            op0_limb_16_col17,
            op0_limb_17_col18,
            op0_limb_18_col19,
            op0_limb_19_col20,
            op0_limb_20_col21,
            op0_limb_21_col22,
            op0_limb_22_col23,
            op0_limb_23_col24,
            op0_limb_24_col25,
            op0_limb_25_col26,
            op0_limb_26_col27,
            op0_limb_27_col28,
            op1_id_col29,
            op1_limb_0_col30,
            op1_limb_1_col31,
            op1_limb_2_col32,
            op1_limb_3_col33,
            op1_limb_4_col34,
            op1_limb_5_col35,
            op1_limb_6_col36,
            op1_limb_7_col37,
            op1_limb_8_col38,
            op1_limb_9_col39,
            op1_limb_10_col40,
            op1_limb_11_col41,
            op1_limb_12_col42,
            op1_limb_13_col43,
            op1_limb_14_col44,
            op1_limb_15_col45,
            op1_limb_16_col46,
            op1_limb_17_col47,
            op1_limb_18_col48,
            op1_limb_19_col49,
            op1_limb_20_col50,
            op1_limb_21_col51,
            op1_limb_22_col52,
            op1_limb_23_col53,
            op1_limb_24_col54,
            op1_limb_25_col55,
            op1_limb_26_col56,
            op1_limb_27_col57,
            xor_col58,
            xor_col59,
            xor_col60,
            xor_col61,
            xor_col62,
            xor_col63,
            xor_col64,
            xor_col65,
            xor_col66,
            xor_col67,
            xor_col68,
            xor_col69,
            xor_col70,
            xor_col71,
            xor_col72,
            xor_col73,
            xor_col74,
            xor_col75,
            xor_col76,
            xor_col77,
            xor_col78,
            xor_col79,
            xor_col80,
            xor_col81,
            xor_col82,
            xor_col83,
            xor_col84,
            xor_col85,
            and_id_col86,
            xor_id_col87,
            or_id_col88,
        ]: [Span<QM31>; 89] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [op0_id_col0]: [QM31; 1] = (*op0_id_col0.try_into().unwrap()).unbox();
        let [op0_limb_0_col1]: [QM31; 1] = (*op0_limb_0_col1.try_into().unwrap()).unbox();
        let [op0_limb_1_col2]: [QM31; 1] = (*op0_limb_1_col2.try_into().unwrap()).unbox();
        let [op0_limb_2_col3]: [QM31; 1] = (*op0_limb_2_col3.try_into().unwrap()).unbox();
        let [op0_limb_3_col4]: [QM31; 1] = (*op0_limb_3_col4.try_into().unwrap()).unbox();
        let [op0_limb_4_col5]: [QM31; 1] = (*op0_limb_4_col5.try_into().unwrap()).unbox();
        let [op0_limb_5_col6]: [QM31; 1] = (*op0_limb_5_col6.try_into().unwrap()).unbox();
        let [op0_limb_6_col7]: [QM31; 1] = (*op0_limb_6_col7.try_into().unwrap()).unbox();
        let [op0_limb_7_col8]: [QM31; 1] = (*op0_limb_7_col8.try_into().unwrap()).unbox();
        let [op0_limb_8_col9]: [QM31; 1] = (*op0_limb_8_col9.try_into().unwrap()).unbox();
        let [op0_limb_9_col10]: [QM31; 1] = (*op0_limb_9_col10.try_into().unwrap()).unbox();
        let [op0_limb_10_col11]: [QM31; 1] = (*op0_limb_10_col11.try_into().unwrap()).unbox();
        let [op0_limb_11_col12]: [QM31; 1] = (*op0_limb_11_col12.try_into().unwrap()).unbox();
        let [op0_limb_12_col13]: [QM31; 1] = (*op0_limb_12_col13.try_into().unwrap()).unbox();
        let [op0_limb_13_col14]: [QM31; 1] = (*op0_limb_13_col14.try_into().unwrap()).unbox();
        let [op0_limb_14_col15]: [QM31; 1] = (*op0_limb_14_col15.try_into().unwrap()).unbox();
        let [op0_limb_15_col16]: [QM31; 1] = (*op0_limb_15_col16.try_into().unwrap()).unbox();
        let [op0_limb_16_col17]: [QM31; 1] = (*op0_limb_16_col17.try_into().unwrap()).unbox();
        let [op0_limb_17_col18]: [QM31; 1] = (*op0_limb_17_col18.try_into().unwrap()).unbox();
        let [op0_limb_18_col19]: [QM31; 1] = (*op0_limb_18_col19.try_into().unwrap()).unbox();
        let [op0_limb_19_col20]: [QM31; 1] = (*op0_limb_19_col20.try_into().unwrap()).unbox();
        let [op0_limb_20_col21]: [QM31; 1] = (*op0_limb_20_col21.try_into().unwrap()).unbox();
        let [op0_limb_21_col22]: [QM31; 1] = (*op0_limb_21_col22.try_into().unwrap()).unbox();
        let [op0_limb_22_col23]: [QM31; 1] = (*op0_limb_22_col23.try_into().unwrap()).unbox();
        let [op0_limb_23_col24]: [QM31; 1] = (*op0_limb_23_col24.try_into().unwrap()).unbox();
        let [op0_limb_24_col25]: [QM31; 1] = (*op0_limb_24_col25.try_into().unwrap()).unbox();
        let [op0_limb_25_col26]: [QM31; 1] = (*op0_limb_25_col26.try_into().unwrap()).unbox();
        let [op0_limb_26_col27]: [QM31; 1] = (*op0_limb_26_col27.try_into().unwrap()).unbox();
        let [op0_limb_27_col28]: [QM31; 1] = (*op0_limb_27_col28.try_into().unwrap()).unbox();
        let [op1_id_col29]: [QM31; 1] = (*op1_id_col29.try_into().unwrap()).unbox();
        let [op1_limb_0_col30]: [QM31; 1] = (*op1_limb_0_col30.try_into().unwrap()).unbox();
        let [op1_limb_1_col31]: [QM31; 1] = (*op1_limb_1_col31.try_into().unwrap()).unbox();
        let [op1_limb_2_col32]: [QM31; 1] = (*op1_limb_2_col32.try_into().unwrap()).unbox();
        let [op1_limb_3_col33]: [QM31; 1] = (*op1_limb_3_col33.try_into().unwrap()).unbox();
        let [op1_limb_4_col34]: [QM31; 1] = (*op1_limb_4_col34.try_into().unwrap()).unbox();
        let [op1_limb_5_col35]: [QM31; 1] = (*op1_limb_5_col35.try_into().unwrap()).unbox();
        let [op1_limb_6_col36]: [QM31; 1] = (*op1_limb_6_col36.try_into().unwrap()).unbox();
        let [op1_limb_7_col37]: [QM31; 1] = (*op1_limb_7_col37.try_into().unwrap()).unbox();
        let [op1_limb_8_col38]: [QM31; 1] = (*op1_limb_8_col38.try_into().unwrap()).unbox();
        let [op1_limb_9_col39]: [QM31; 1] = (*op1_limb_9_col39.try_into().unwrap()).unbox();
        let [op1_limb_10_col40]: [QM31; 1] = (*op1_limb_10_col40.try_into().unwrap()).unbox();
        let [op1_limb_11_col41]: [QM31; 1] = (*op1_limb_11_col41.try_into().unwrap()).unbox();
        let [op1_limb_12_col42]: [QM31; 1] = (*op1_limb_12_col42.try_into().unwrap()).unbox();
        let [op1_limb_13_col43]: [QM31; 1] = (*op1_limb_13_col43.try_into().unwrap()).unbox();
        let [op1_limb_14_col44]: [QM31; 1] = (*op1_limb_14_col44.try_into().unwrap()).unbox();
        let [op1_limb_15_col45]: [QM31; 1] = (*op1_limb_15_col45.try_into().unwrap()).unbox();
        let [op1_limb_16_col46]: [QM31; 1] = (*op1_limb_16_col46.try_into().unwrap()).unbox();
        let [op1_limb_17_col47]: [QM31; 1] = (*op1_limb_17_col47.try_into().unwrap()).unbox();
        let [op1_limb_18_col48]: [QM31; 1] = (*op1_limb_18_col48.try_into().unwrap()).unbox();
        let [op1_limb_19_col49]: [QM31; 1] = (*op1_limb_19_col49.try_into().unwrap()).unbox();
        let [op1_limb_20_col50]: [QM31; 1] = (*op1_limb_20_col50.try_into().unwrap()).unbox();
        let [op1_limb_21_col51]: [QM31; 1] = (*op1_limb_21_col51.try_into().unwrap()).unbox();
        let [op1_limb_22_col52]: [QM31; 1] = (*op1_limb_22_col52.try_into().unwrap()).unbox();
        let [op1_limb_23_col53]: [QM31; 1] = (*op1_limb_23_col53.try_into().unwrap()).unbox();
        let [op1_limb_24_col54]: [QM31; 1] = (*op1_limb_24_col54.try_into().unwrap()).unbox();
        let [op1_limb_25_col55]: [QM31; 1] = (*op1_limb_25_col55.try_into().unwrap()).unbox();
        let [op1_limb_26_col56]: [QM31; 1] = (*op1_limb_26_col56.try_into().unwrap()).unbox();
        let [op1_limb_27_col57]: [QM31; 1] = (*op1_limb_27_col57.try_into().unwrap()).unbox();
        let [xor_col58]: [QM31; 1] = (*xor_col58.try_into().unwrap()).unbox();
        let [xor_col59]: [QM31; 1] = (*xor_col59.try_into().unwrap()).unbox();
        let [xor_col60]: [QM31; 1] = (*xor_col60.try_into().unwrap()).unbox();
        let [xor_col61]: [QM31; 1] = (*xor_col61.try_into().unwrap()).unbox();
        let [xor_col62]: [QM31; 1] = (*xor_col62.try_into().unwrap()).unbox();
        let [xor_col63]: [QM31; 1] = (*xor_col63.try_into().unwrap()).unbox();
        let [xor_col64]: [QM31; 1] = (*xor_col64.try_into().unwrap()).unbox();
        let [xor_col65]: [QM31; 1] = (*xor_col65.try_into().unwrap()).unbox();
        let [xor_col66]: [QM31; 1] = (*xor_col66.try_into().unwrap()).unbox();
        let [xor_col67]: [QM31; 1] = (*xor_col67.try_into().unwrap()).unbox();
        let [xor_col68]: [QM31; 1] = (*xor_col68.try_into().unwrap()).unbox();
        let [xor_col69]: [QM31; 1] = (*xor_col69.try_into().unwrap()).unbox();
        let [xor_col70]: [QM31; 1] = (*xor_col70.try_into().unwrap()).unbox();
        let [xor_col71]: [QM31; 1] = (*xor_col71.try_into().unwrap()).unbox();
        let [xor_col72]: [QM31; 1] = (*xor_col72.try_into().unwrap()).unbox();
        let [xor_col73]: [QM31; 1] = (*xor_col73.try_into().unwrap()).unbox();
        let [xor_col74]: [QM31; 1] = (*xor_col74.try_into().unwrap()).unbox();
        let [xor_col75]: [QM31; 1] = (*xor_col75.try_into().unwrap()).unbox();
        let [xor_col76]: [QM31; 1] = (*xor_col76.try_into().unwrap()).unbox();
        let [xor_col77]: [QM31; 1] = (*xor_col77.try_into().unwrap()).unbox();
        let [xor_col78]: [QM31; 1] = (*xor_col78.try_into().unwrap()).unbox();
        let [xor_col79]: [QM31; 1] = (*xor_col79.try_into().unwrap()).unbox();
        let [xor_col80]: [QM31; 1] = (*xor_col80.try_into().unwrap()).unbox();
        let [xor_col81]: [QM31; 1] = (*xor_col81.try_into().unwrap()).unbox();
        let [xor_col82]: [QM31; 1] = (*xor_col82.try_into().unwrap()).unbox();
        let [xor_col83]: [QM31; 1] = (*xor_col83.try_into().unwrap()).unbox();
        let [xor_col84]: [QM31; 1] = (*xor_col84.try_into().unwrap()).unbox();
        let [xor_col85]: [QM31; 1] = (*xor_col85.try_into().unwrap()).unbox();
        let [and_id_col86]: [QM31; 1] = (*and_id_col86.try_into().unwrap()).unbox();
        let [xor_id_col87]: [QM31; 1] = (*xor_id_col87.try_into().unwrap()).unbox();
        let [or_id_col88]: [QM31; 1] = (*or_id_col88.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        read_positive_num_bits_252_evaluate(
            [(bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))],
            op0_id_col0,
            op0_limb_0_col1,
            op0_limb_1_col2,
            op0_limb_2_col3,
            op0_limb_3_col4,
            op0_limb_4_col5,
            op0_limb_5_col6,
            op0_limb_6_col7,
            op0_limb_7_col8,
            op0_limb_8_col9,
            op0_limb_9_col10,
            op0_limb_10_col11,
            op0_limb_11_col12,
            op0_limb_12_col13,
            op0_limb_13_col14,
            op0_limb_14_col15,
            op0_limb_15_col16,
            op0_limb_16_col17,
            op0_limb_17_col18,
            op0_limb_18_col19,
            op0_limb_19_col20,
            op0_limb_20_col21,
            op0_limb_21_col22,
            op0_limb_22_col23,
            op0_limb_23_col24,
            op0_limb_24_col25,
            op0_limb_25_col26,
            op0_limb_26_col27,
            op0_limb_27_col28,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref memory_id_to_big_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_positive_num_bits_252_evaluate(
            [
                ((bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))
                    + qm31_const::<1, 0, 0, 0>())
            ],
            op1_id_col29,
            op1_limb_0_col30,
            op1_limb_1_col31,
            op1_limb_2_col32,
            op1_limb_3_col33,
            op1_limb_4_col34,
            op1_limb_5_col35,
            op1_limb_6_col36,
            op1_limb_7_col37,
            op1_limb_8_col38,
            op1_limb_9_col39,
            op1_limb_10_col40,
            op1_limb_11_col41,
            op1_limb_12_col42,
            op1_limb_13_col43,
            op1_limb_14_col44,
            op1_limb_15_col45,
            op1_limb_16_col46,
            op1_limb_17_col47,
            op1_limb_18_col48,
            op1_limb_19_col49,
            op1_limb_20_col50,
            op1_limb_21_col51,
            op1_limb_22_col52,
            op1_limb_23_col53,
            op1_limb_24_col54,
            op1_limb_25_col55,
            op1_limb_26_col56,
            op1_limb_27_col57,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_0_col1, op1_limb_0_col30],
            xor_col58,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_8: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_0_col1 + op1_limb_0_col30) - xor_col58));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_1_col2, op1_limb_1_col31],
            xor_col59,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_11: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_1_col2 + op1_limb_1_col31) - xor_col59));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_2_col3, op1_limb_2_col32],
            xor_col60,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_14: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_2_col3 + op1_limb_2_col32) - xor_col60));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_3_col4, op1_limb_3_col33],
            xor_col61,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_17: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_3_col4 + op1_limb_3_col33) - xor_col61));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_4_col5, op1_limb_4_col34],
            xor_col62,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_8,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_20: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_4_col5 + op1_limb_4_col34) - xor_col62));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_5_col6, op1_limb_5_col35],
            xor_col63,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_23: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_5_col6 + op1_limb_5_col35) - xor_col63));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_6_col7, op1_limb_6_col36],
            xor_col64,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_10,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_26: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_6_col7 + op1_limb_6_col36) - xor_col64));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_7_col8, op1_limb_7_col37],
            xor_col65,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_11,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_29: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_7_col8 + op1_limb_7_col37) - xor_col65));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_8_col9, op1_limb_8_col38],
            xor_col66,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_12,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_32: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_8_col9 + op1_limb_8_col38) - xor_col66));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_9_col10, op1_limb_9_col39],
            xor_col67,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_13,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_35: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_9_col10 + op1_limb_9_col39) - xor_col67));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_10_col11, op1_limb_10_col40],
            xor_col68,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_14,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_38: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_10_col11 + op1_limb_10_col40) - xor_col68));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_11_col12, op1_limb_11_col41],
            xor_col69,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_15,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_41: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_11_col12 + op1_limb_11_col41) - xor_col69));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_12_col13, op1_limb_12_col42],
            xor_col70,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_16,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_44: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_12_col13 + op1_limb_12_col42) - xor_col70));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_13_col14, op1_limb_13_col43],
            xor_col71,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_17,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_47: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_13_col14 + op1_limb_13_col43) - xor_col71));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_14_col15, op1_limb_14_col44],
            xor_col72,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_18,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_50: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_14_col15 + op1_limb_14_col44) - xor_col72));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_15_col16, op1_limb_15_col45],
            xor_col73,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_19,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_53: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_15_col16 + op1_limb_15_col45) - xor_col73));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_16_col17, op1_limb_16_col46],
            xor_col74,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_20,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_56: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_16_col17 + op1_limb_16_col46) - xor_col74));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_17_col18, op1_limb_17_col47],
            xor_col75,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_21,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_59: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_17_col18 + op1_limb_17_col47) - xor_col75));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_18_col19, op1_limb_18_col48],
            xor_col76,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_22,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_62: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_18_col19 + op1_limb_18_col48) - xor_col76));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_19_col20, op1_limb_19_col49],
            xor_col77,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_23,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_65: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_19_col20 + op1_limb_19_col49) - xor_col77));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_20_col21, op1_limb_20_col50],
            xor_col78,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_24,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_68: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_20_col21 + op1_limb_20_col50) - xor_col78));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_21_col22, op1_limb_21_col51],
            xor_col79,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_25,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_71: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_21_col22 + op1_limb_21_col51) - xor_col79));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_22_col23, op1_limb_22_col52],
            xor_col80,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_26,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_74: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_22_col23 + op1_limb_22_col52) - xor_col80));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_23_col24, op1_limb_23_col53],
            xor_col81,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_27,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_77: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_23_col24 + op1_limb_23_col53) - xor_col81));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_24_col25, op1_limb_24_col54],
            xor_col82,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_28,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_80: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_24_col25 + op1_limb_24_col54) - xor_col82));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_25_col26, op1_limb_25_col55],
            xor_col83,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_29,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_83: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_25_col26 + op1_limb_25_col55) - xor_col83));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_26_col27, op1_limb_26_col56],
            xor_col84,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_30,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_86: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_26_col27 + op1_limb_26_col56) - xor_col84));

        bitwise_xor_num_bits_9_evaluate(
            [op0_limb_27_col28, op1_limb_27_col57],
            xor_col85,
            self.verify_bitwise_xor_9_lookup_elements,
            ref verify_bitwise_xor_9_sum_31,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_89: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_27_col28 + op1_limb_27_col57) - xor_col85));

        mem_verify_evaluate(
            [
                ((bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))
                    + qm31_const::<2, 0, 0, 0>()),
                and_tmp_efb2a_8, and_tmp_efb2a_11, and_tmp_efb2a_14, and_tmp_efb2a_17,
                and_tmp_efb2a_20, and_tmp_efb2a_23, and_tmp_efb2a_26, and_tmp_efb2a_29,
                and_tmp_efb2a_32, and_tmp_efb2a_35, and_tmp_efb2a_38, and_tmp_efb2a_41,
                and_tmp_efb2a_44, and_tmp_efb2a_47, and_tmp_efb2a_50, and_tmp_efb2a_53,
                and_tmp_efb2a_56, and_tmp_efb2a_59, and_tmp_efb2a_62, and_tmp_efb2a_65,
                and_tmp_efb2a_68, and_tmp_efb2a_71, and_tmp_efb2a_74, and_tmp_efb2a_77,
                and_tmp_efb2a_80, and_tmp_efb2a_83, and_tmp_efb2a_86, and_tmp_efb2a_89,
            ],
            and_id_col86,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_32,
            ref memory_id_to_big_sum_33,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        mem_verify_evaluate(
            [
                ((bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))
                    + qm31_const::<3, 0, 0, 0>()),
                xor_col58, xor_col59, xor_col60, xor_col61, xor_col62, xor_col63, xor_col64,
                xor_col65, xor_col66, xor_col67, xor_col68, xor_col69, xor_col70, xor_col71,
                xor_col72, xor_col73, xor_col74, xor_col75, xor_col76, xor_col77, xor_col78,
                xor_col79, xor_col80, xor_col81, xor_col82, xor_col83, xor_col84, xor_col85,
            ],
            xor_id_col87,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_34,
            ref memory_id_to_big_sum_35,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        mem_verify_evaluate(
            [
                ((bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))
                    + qm31_const::<4, 0, 0, 0>()),
                (and_tmp_efb2a_8 + xor_col58), (and_tmp_efb2a_11 + xor_col59),
                (and_tmp_efb2a_14 + xor_col60), (and_tmp_efb2a_17 + xor_col61),
                (and_tmp_efb2a_20 + xor_col62), (and_tmp_efb2a_23 + xor_col63),
                (and_tmp_efb2a_26 + xor_col64), (and_tmp_efb2a_29 + xor_col65),
                (and_tmp_efb2a_32 + xor_col66), (and_tmp_efb2a_35 + xor_col67),
                (and_tmp_efb2a_38 + xor_col68), (and_tmp_efb2a_41 + xor_col69),
                (and_tmp_efb2a_44 + xor_col70), (and_tmp_efb2a_47 + xor_col71),
                (and_tmp_efb2a_50 + xor_col72), (and_tmp_efb2a_53 + xor_col73),
                (and_tmp_efb2a_56 + xor_col74), (and_tmp_efb2a_59 + xor_col75),
                (and_tmp_efb2a_62 + xor_col76), (and_tmp_efb2a_65 + xor_col77),
                (and_tmp_efb2a_68 + xor_col78), (and_tmp_efb2a_71 + xor_col79),
                (and_tmp_efb2a_74 + xor_col80), (and_tmp_efb2a_77 + xor_col81),
                (and_tmp_efb2a_80 + xor_col82), (and_tmp_efb2a_83 + xor_col83),
                (and_tmp_efb2a_86 + xor_col84), (and_tmp_efb2a_89 + xor_col85),
            ],
            or_id_col88,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_36,
            ref memory_id_to_big_sum_37,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_id_to_big_sum_1,
            memory_address_to_id_sum_2,
            memory_id_to_big_sum_3,
            verify_bitwise_xor_9_sum_4,
            verify_bitwise_xor_9_sum_5,
            verify_bitwise_xor_9_sum_6,
            verify_bitwise_xor_9_sum_7,
            verify_bitwise_xor_9_sum_8,
            verify_bitwise_xor_9_sum_9,
            verify_bitwise_xor_9_sum_10,
            verify_bitwise_xor_9_sum_11,
            verify_bitwise_xor_9_sum_12,
            verify_bitwise_xor_9_sum_13,
            verify_bitwise_xor_9_sum_14,
            verify_bitwise_xor_9_sum_15,
            verify_bitwise_xor_9_sum_16,
            verify_bitwise_xor_9_sum_17,
            verify_bitwise_xor_9_sum_18,
            verify_bitwise_xor_9_sum_19,
            verify_bitwise_xor_9_sum_20,
            verify_bitwise_xor_9_sum_21,
            verify_bitwise_xor_9_sum_22,
            verify_bitwise_xor_9_sum_23,
            verify_bitwise_xor_9_sum_24,
            verify_bitwise_xor_9_sum_25,
            verify_bitwise_xor_9_sum_26,
            verify_bitwise_xor_9_sum_27,
            verify_bitwise_xor_9_sum_28,
            verify_bitwise_xor_9_sum_29,
            verify_bitwise_xor_9_sum_30,
            verify_bitwise_xor_9_sum_31,
            memory_address_to_id_sum_32,
            memory_id_to_big_sum_33,
            memory_address_to_id_sum_34,
            memory_id_to_big_sum_35,
            memory_address_to_id_sum_36,
            memory_id_to_big_sum_37,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_address_to_id_sum_0: QM31,
    memory_id_to_big_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_id_to_big_sum_3: QM31,
    verify_bitwise_xor_9_sum_4: QM31,
    verify_bitwise_xor_9_sum_5: QM31,
    verify_bitwise_xor_9_sum_6: QM31,
    verify_bitwise_xor_9_sum_7: QM31,
    verify_bitwise_xor_9_sum_8: QM31,
    verify_bitwise_xor_9_sum_9: QM31,
    verify_bitwise_xor_9_sum_10: QM31,
    verify_bitwise_xor_9_sum_11: QM31,
    verify_bitwise_xor_9_sum_12: QM31,
    verify_bitwise_xor_9_sum_13: QM31,
    verify_bitwise_xor_9_sum_14: QM31,
    verify_bitwise_xor_9_sum_15: QM31,
    verify_bitwise_xor_9_sum_16: QM31,
    verify_bitwise_xor_9_sum_17: QM31,
    verify_bitwise_xor_9_sum_18: QM31,
    verify_bitwise_xor_9_sum_19: QM31,
    verify_bitwise_xor_9_sum_20: QM31,
    verify_bitwise_xor_9_sum_21: QM31,
    verify_bitwise_xor_9_sum_22: QM31,
    verify_bitwise_xor_9_sum_23: QM31,
    verify_bitwise_xor_9_sum_24: QM31,
    verify_bitwise_xor_9_sum_25: QM31,
    verify_bitwise_xor_9_sum_26: QM31,
    verify_bitwise_xor_9_sum_27: QM31,
    verify_bitwise_xor_9_sum_28: QM31,
    verify_bitwise_xor_9_sum_29: QM31,
    verify_bitwise_xor_9_sum_30: QM31,
    verify_bitwise_xor_9_sum_31: QM31,
    memory_address_to_id_sum_32: QM31,
    memory_id_to_big_sum_33: QM31,
    memory_address_to_id_sum_34: QM31,
    memory_id_to_big_sum_35: QM31,
    memory_address_to_id_sum_36: QM31,
    memory_id_to_big_sum_37: QM31,
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
        trace_2_col40,
        trace_2_col41,
        trace_2_col42,
        trace_2_col43,
        trace_2_col44,
        trace_2_col45,
        trace_2_col46,
        trace_2_col47,
        trace_2_col48,
        trace_2_col49,
        trace_2_col50,
        trace_2_col51,
        trace_2_col52,
        trace_2_col53,
        trace_2_col54,
        trace_2_col55,
        trace_2_col56,
        trace_2_col57,
        trace_2_col58,
        trace_2_col59,
        trace_2_col60,
        trace_2_col61,
        trace_2_col62,
        trace_2_col63,
        trace_2_col64,
        trace_2_col65,
        trace_2_col66,
        trace_2_col67,
        trace_2_col68,
        trace_2_col69,
        trace_2_col70,
        trace_2_col71,
        trace_2_col72,
        trace_2_col73,
        trace_2_col74,
        trace_2_col75,
    ]: [Span<QM31>; 76] =
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
    let [trace_2_col72_neg1, trace_2_col72]: [QM31; 2] = (*trace_2_col72.try_into().unwrap())
        .unbox();
    let [trace_2_col73_neg1, trace_2_col73]: [QM31; 2] = (*trace_2_col73.try_into().unwrap())
        .unbox();
    let [trace_2_col74_neg1, trace_2_col74]: [QM31; 2] = (*trace_2_col74.try_into().unwrap())
        .unbox();
    let [trace_2_col75_neg1, trace_2_col75]: [QM31; 2] = (*trace_2_col75.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * memory_address_to_id_sum_0
        * memory_id_to_big_sum_1)
        - memory_address_to_id_sum_0
        - memory_id_to_big_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_address_to_id_sum_2
        * memory_id_to_big_sum_3)
        - memory_address_to_id_sum_2
        - memory_id_to_big_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * verify_bitwise_xor_9_sum_4
        * verify_bitwise_xor_9_sum_5)
        - verify_bitwise_xor_9_sum_4
        - verify_bitwise_xor_9_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_xor_9_sum_6
        * verify_bitwise_xor_9_sum_7)
        - verify_bitwise_xor_9_sum_6
        - verify_bitwise_xor_9_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * verify_bitwise_xor_9_sum_8
        * verify_bitwise_xor_9_sum_9)
        - verify_bitwise_xor_9_sum_8
        - verify_bitwise_xor_9_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * verify_bitwise_xor_9_sum_10
        * verify_bitwise_xor_9_sum_11)
        - verify_bitwise_xor_9_sum_10
        - verify_bitwise_xor_9_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * verify_bitwise_xor_9_sum_12
        * verify_bitwise_xor_9_sum_13)
        - verify_bitwise_xor_9_sum_12
        - verify_bitwise_xor_9_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * verify_bitwise_xor_9_sum_14
        * verify_bitwise_xor_9_sum_15)
        - verify_bitwise_xor_9_sum_14
        - verify_bitwise_xor_9_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * verify_bitwise_xor_9_sum_16
        * verify_bitwise_xor_9_sum_17)
        - verify_bitwise_xor_9_sum_16
        - verify_bitwise_xor_9_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * verify_bitwise_xor_9_sum_18
        * verify_bitwise_xor_9_sum_19)
        - verify_bitwise_xor_9_sum_18
        - verify_bitwise_xor_9_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * verify_bitwise_xor_9_sum_20
        * verify_bitwise_xor_9_sum_21)
        - verify_bitwise_xor_9_sum_20
        - verify_bitwise_xor_9_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * verify_bitwise_xor_9_sum_22
        * verify_bitwise_xor_9_sum_23)
        - verify_bitwise_xor_9_sum_22
        - verify_bitwise_xor_9_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * verify_bitwise_xor_9_sum_24
        * verify_bitwise_xor_9_sum_25)
        - verify_bitwise_xor_9_sum_24
        - verify_bitwise_xor_9_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * verify_bitwise_xor_9_sum_26
        * verify_bitwise_xor_9_sum_27)
        - verify_bitwise_xor_9_sum_26
        - verify_bitwise_xor_9_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * verify_bitwise_xor_9_sum_28
        * verify_bitwise_xor_9_sum_29)
        - verify_bitwise_xor_9_sum_28
        - verify_bitwise_xor_9_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * verify_bitwise_xor_9_sum_30
        * verify_bitwise_xor_9_sum_31)
        - verify_bitwise_xor_9_sum_30
        - verify_bitwise_xor_9_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * memory_address_to_id_sum_32
        * memory_id_to_big_sum_33)
        - memory_address_to_id_sum_32
        - memory_id_to_big_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * memory_address_to_id_sum_34
        * memory_id_to_big_sum_35)
        - memory_address_to_id_sum_34
        - memory_id_to_big_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals([trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71])
        - QM31Impl::from_partial_evals(
            [trace_2_col72_neg1, trace_2_col73_neg1, trace_2_col74_neg1, trace_2_col75_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_address_to_id_sum_36
        * memory_id_to_big_sum_37)
        - memory_address_to_id_sum_36
        - memory_id_to_big_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

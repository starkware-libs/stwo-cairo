// This file was created by the AIR team.

use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::bitwise_xor_num_bits_9::bitwise_xor_num_bits_9_evaluate;
use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 89;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('MemoryAddressToId', 5), ('MemoryIdToBig', 5), ('VerifyBitwiseXor_9', 27),
    ('VerifyBitwiseXor_8', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub bitwise_builtin_segment_start: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 76].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.bitwise_builtin_segment_start).into());
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
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub verify_bitwise_xor_9_lookup_elements: crate::VerifyBitwiseXor_9Elements,
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
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
            memory_address_to_id_lookup_elements: interaction_elements.memory_address_to_id.clone(),
            memory_id_to_big_lookup_elements: interaction_elements.memory_id_to_value.clone(),
            verify_bitwise_xor_9_lookup_elements: interaction_elements.verify_bitwise_xor_9.clone(),
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
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
        let mut verify_bitwise_xor_8_sum_31: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_32: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_33: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_34: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_35: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_36: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_37: QM31 = Zero::zero();
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));

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
            (bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>())),
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
            ((bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))
                + qm31_const::<1, 0, 0, 0>()),
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
        let and_tmp_efb2a_12: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_15: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_18: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_21: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_24: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_27: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_30: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_33: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_36: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_39: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_42: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_45: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_48: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_51: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_54: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_57: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_60: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_63: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_66: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_69: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_72: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_75: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_78: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_81: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_84: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_87: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
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
        let and_tmp_efb2a_90: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_26_col27 + op1_limb_26_col56) - xor_col84));
        bitwise_xor_num_bits_8_evaluate(
            [op0_limb_27_col28, op1_limb_27_col57],
            xor_col85,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_31,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let and_tmp_efb2a_93: QM31 = (qm31_const::<1073741824, 0, 0, 0>()
            * ((op0_limb_27_col28 + op1_limb_27_col57) - xor_col85));
        mem_verify_evaluate(
            [
                ((bitwise_builtin_segment_start + (seq * qm31_const::<5, 0, 0, 0>()))
                    + qm31_const::<2, 0, 0, 0>()),
                and_tmp_efb2a_12, and_tmp_efb2a_15, and_tmp_efb2a_18, and_tmp_efb2a_21,
                and_tmp_efb2a_24, and_tmp_efb2a_27, and_tmp_efb2a_30, and_tmp_efb2a_33,
                and_tmp_efb2a_36, and_tmp_efb2a_39, and_tmp_efb2a_42, and_tmp_efb2a_45,
                and_tmp_efb2a_48, and_tmp_efb2a_51, and_tmp_efb2a_54, and_tmp_efb2a_57,
                and_tmp_efb2a_60, and_tmp_efb2a_63, and_tmp_efb2a_66, and_tmp_efb2a_69,
                and_tmp_efb2a_72, and_tmp_efb2a_75, and_tmp_efb2a_78, and_tmp_efb2a_81,
                and_tmp_efb2a_84, and_tmp_efb2a_87, and_tmp_efb2a_90, and_tmp_efb2a_93,
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
                (and_tmp_efb2a_12 + xor_col58), (and_tmp_efb2a_15 + xor_col59),
                (and_tmp_efb2a_18 + xor_col60), (and_tmp_efb2a_21 + xor_col61),
                (and_tmp_efb2a_24 + xor_col62), (and_tmp_efb2a_27 + xor_col63),
                (and_tmp_efb2a_30 + xor_col64), (and_tmp_efb2a_33 + xor_col65),
                (and_tmp_efb2a_36 + xor_col66), (and_tmp_efb2a_39 + xor_col67),
                (and_tmp_efb2a_42 + xor_col68), (and_tmp_efb2a_45 + xor_col69),
                (and_tmp_efb2a_48 + xor_col70), (and_tmp_efb2a_51 + xor_col71),
                (and_tmp_efb2a_54 + xor_col72), (and_tmp_efb2a_57 + xor_col73),
                (and_tmp_efb2a_60 + xor_col74), (and_tmp_efb2a_63 + xor_col75),
                (and_tmp_efb2a_66 + xor_col76), (and_tmp_efb2a_69 + xor_col77),
                (and_tmp_efb2a_72 + xor_col78), (and_tmp_efb2a_75 + xor_col79),
                (and_tmp_efb2a_78 + xor_col80), (and_tmp_efb2a_81 + xor_col81),
                (and_tmp_efb2a_84 + xor_col82), (and_tmp_efb2a_87 + xor_col83),
                (and_tmp_efb2a_90 + xor_col84), (and_tmp_efb2a_93 + xor_col85),
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
            verify_bitwise_xor_8_sum_31,
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
    verify_bitwise_xor_8_sum_31: QM31,
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
        * verify_bitwise_xor_8_sum_31)
        - verify_bitwise_xor_9_sum_30
        - verify_bitwise_xor_8_sum_31)
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
            claim: Claim { log_size: 15, bitwise_builtin_segment_start: 434121993 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            memory_address_to_id_lookup_elements: make_lookup_elements(
                qm31_const::<1842771211, 1960835386, 1582137647, 1333140033>(),
                qm31_const::<1360491305, 950648792, 556642685, 2096522554>(),
            ),
            memory_id_to_big_lookup_elements: make_lookup_elements(
                qm31_const::<844624398, 1166453613, 1247584074, 330174372>(),
                qm31_const::<1844105245, 1400976933, 1126903288, 1155460729>(),
            ),
            verify_bitwise_xor_8_lookup_elements: make_lookup_elements(
                qm31_const::<390097169, 1715941348, 958959293, 1227669969>(),
                qm31_const::<105167513, 476596518, 1027059816, 1879697407>(),
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
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            seq_column_idx(component.claim.log_size),
            qm31_const::<735272696, 1215403647, 795393303, 879304430>(),
        );

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
            [qm31_const::<375524249, 1987255153, 582840933, 343880041>()].span(),
            [qm31_const::<442633428, 2121472881, 649949797, 343880041>()].span(),
            [qm31_const::<509742607, 108206962, 717058662, 343880041>()].span(),
            [qm31_const::<576851786, 242424690, 784167526, 343880041>()].span(),
            [qm31_const::<643960965, 376642418, 851276390, 343880041>()].span(),
            [qm31_const::<711070144, 510860146, 918385254, 343880041>()].span(),
            [qm31_const::<778179323, 645077874, 985494118, 343880041>()].span(),
            [qm31_const::<845288502, 779295602, 1052602982, 343880041>()].span(),
            [qm31_const::<375831434, 1987255333, 582841113, 343880101>()].span(),
            [qm31_const::<308722255, 1853037605, 515732249, 343880101>()].span(),
            [qm31_const::<510049792, 108207142, 717058842, 343880101>()].span(),
            [qm31_const::<442940613, 2121473061, 649949977, 343880101>()].span(),
            [qm31_const::<644268150, 376642598, 851276570, 343880101>()].span(),
            [qm31_const::<577158971, 242424870, 784167706, 343880101>()].span(),
            [qm31_const::<778486508, 645078054, 985494298, 343880101>()].span(),
            [qm31_const::<711377329, 510860326, 918385434, 343880101>()].span(),
            [qm31_const::<912704866, 913513510, 1119712026, 343880101>()].span(),
            [qm31_const::<845595687, 779295782, 1052603162, 343880101>()].span(),
            [qm31_const::<1046820829, 1181948906, 1253929694, 343880081>()].span(),
            [qm31_const::<1113930008, 1316166634, 1321038558, 343880081>()].span(),
            [qm31_const::<912602471, 913513450, 1119711966, 343880081>()].span(),
            [qm31_const::<979711650, 1047731178, 1186820830, 343880081>()].span(),
            [qm31_const::<778384113, 645077994, 985494238, 343880081>()].span(),
            [qm31_const::<845493292, 779295722, 1052603102, 343880081>()].span(),
            [qm31_const::<644165755, 376642538, 851276510, 343880081>()].span(),
            [qm31_const::<711274934, 510860266, 918385374, 343880081>()].span(),
            [qm31_const::<1583694261, 108207083, 1790800607, 343880081>()].span(),
            [qm31_const::<1650803440, 242424811, 1857909471, 343880081>()].span(),
            [qm31_const::<108388425, 1450385012, 314406248, 343880298>()].span(),
            [qm31_const::<41279246, 1316167284, 247297384, 343880298>()].span(),
            [qm31_const::<2121653714, 1181949555, 180188520, 343880298>()].span(),
            [qm31_const::<2054544535, 1047731827, 113079656, 343880298>()].span(),
            [qm31_const::<1987435356, 913514099, 45970792, 343880298>()].span(),
            [qm31_const::<1920326177, 779296371, 2126345575, 343880297>()].span(),
            [qm31_const::<1853216998, 645078643, 2059236711, 343880297>()].span(),
            [qm31_const::<1786107819, 510860915, 1992127847, 343880297>()].span(),
            [qm31_const::<1718998640, 376643187, 1925018983, 343880297>()].span(),
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
            qm31_const::<1609150643, 1287940548, 304384230, 1941984120>(),
            qm31_const::<1577898798, 106101108, 1738096752, 1261630210>(),
            qm31_const::<1510789619, 2119367027, 1670987887, 1261630210>(),
            qm31_const::<1443680440, 1985149299, 1603879023, 1261630210>(),
            qm31_const::<1376571261, 1850931571, 1536770159, 1261630210>(),
            qm31_const::<1309462082, 1716713843, 1469661295, 1261630210>(),
            qm31_const::<1242352903, 1582496115, 1402552431, 1261630210>(),
            qm31_const::<1175243724, 1448278387, 1335443567, 1261630210>(),
            qm31_const::<1108134545, 1314060659, 1268334703, 1261630210>(),
            qm31_const::<2114772230, 1179842932, 127484017, 1261630211>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(BITWISE_BUILTIN_SAMPLE_EVAL_RESULT))
    }
}

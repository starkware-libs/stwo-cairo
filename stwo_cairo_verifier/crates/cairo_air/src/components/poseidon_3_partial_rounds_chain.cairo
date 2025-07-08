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
use crate::components::subroutines::poseidon_partial_round::poseidon_partial_round_evaluate;

pub const N_TRACE_COLUMNS: usize = 169;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 6] = [
    ('PoseidonRoundKeys', 1), ('Cube252', 3), ('RangeCheck_4_4_4_4', 6), ('RangeCheck_4_4', 3),
    ('RangeCheckFelt252Width27', 3), ('Poseidon3PartialRoundsChain', 1),
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
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(36, log_size).span();
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
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub range_check_4_4_4_4_lookup_elements: crate::RangeCheck_4_4_4_4Elements,
    pub range_check_4_4_lookup_elements: crate::RangeCheck_4_4Elements,
    pub range_check_felt_252_width_27_lookup_elements: crate::RangeCheckFelt252Width27Elements,
    pub poseidon_3_partial_rounds_chain_lookup_elements: crate::Poseidon3PartialRoundsChainElements,
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
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
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
        let mut poseidon_round_keys_sum_0: QM31 = Zero::zero();
        let mut cube_252_sum_1: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_2: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_3: QM31 = Zero::zero();
        let mut range_check_4_4_sum_4: QM31 = Zero::zero();
        let mut range_check_felt_252_width_27_sum_5: QM31 = Zero::zero();
        let mut cube_252_sum_6: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_7: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_8: QM31 = Zero::zero();
        let mut range_check_4_4_sum_9: QM31 = Zero::zero();
        let mut range_check_felt_252_width_27_sum_10: QM31 = Zero::zero();
        let mut cube_252_sum_11: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_12: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_13: QM31 = Zero::zero();
        let mut range_check_4_4_sum_14: QM31 = Zero::zero();
        let mut range_check_felt_252_width_27_sum_15: QM31 = Zero::zero();
        let mut poseidon_3_partial_rounds_chain_sum_16: QM31 = Zero::zero();
        let mut poseidon_3_partial_rounds_chain_sum_17: QM31 = Zero::zero();

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
            poseidon_round_keys_output_limb_0_col42,
            poseidon_round_keys_output_limb_1_col43,
            poseidon_round_keys_output_limb_2_col44,
            poseidon_round_keys_output_limb_3_col45,
            poseidon_round_keys_output_limb_4_col46,
            poseidon_round_keys_output_limb_5_col47,
            poseidon_round_keys_output_limb_6_col48,
            poseidon_round_keys_output_limb_7_col49,
            poseidon_round_keys_output_limb_8_col50,
            poseidon_round_keys_output_limb_9_col51,
            poseidon_round_keys_output_limb_10_col52,
            poseidon_round_keys_output_limb_11_col53,
            poseidon_round_keys_output_limb_12_col54,
            poseidon_round_keys_output_limb_13_col55,
            poseidon_round_keys_output_limb_14_col56,
            poseidon_round_keys_output_limb_15_col57,
            poseidon_round_keys_output_limb_16_col58,
            poseidon_round_keys_output_limb_17_col59,
            poseidon_round_keys_output_limb_18_col60,
            poseidon_round_keys_output_limb_19_col61,
            poseidon_round_keys_output_limb_20_col62,
            poseidon_round_keys_output_limb_21_col63,
            poseidon_round_keys_output_limb_22_col64,
            poseidon_round_keys_output_limb_23_col65,
            poseidon_round_keys_output_limb_24_col66,
            poseidon_round_keys_output_limb_25_col67,
            poseidon_round_keys_output_limb_26_col68,
            poseidon_round_keys_output_limb_27_col69,
            poseidon_round_keys_output_limb_28_col70,
            poseidon_round_keys_output_limb_29_col71,
            cube_252_output_limb_0_col72,
            cube_252_output_limb_1_col73,
            cube_252_output_limb_2_col74,
            cube_252_output_limb_3_col75,
            cube_252_output_limb_4_col76,
            cube_252_output_limb_5_col77,
            cube_252_output_limb_6_col78,
            cube_252_output_limb_7_col79,
            cube_252_output_limb_8_col80,
            cube_252_output_limb_9_col81,
            combination_limb_0_col82,
            combination_limb_1_col83,
            combination_limb_2_col84,
            combination_limb_3_col85,
            combination_limb_4_col86,
            combination_limb_5_col87,
            combination_limb_6_col88,
            combination_limb_7_col89,
            combination_limb_8_col90,
            combination_limb_9_col91,
            p_coef_col92,
            combination_limb_0_col93,
            combination_limb_1_col94,
            combination_limb_2_col95,
            combination_limb_3_col96,
            combination_limb_4_col97,
            combination_limb_5_col98,
            combination_limb_6_col99,
            combination_limb_7_col100,
            combination_limb_8_col101,
            combination_limb_9_col102,
            p_coef_col103,
            cube_252_output_limb_0_col104,
            cube_252_output_limb_1_col105,
            cube_252_output_limb_2_col106,
            cube_252_output_limb_3_col107,
            cube_252_output_limb_4_col108,
            cube_252_output_limb_5_col109,
            cube_252_output_limb_6_col110,
            cube_252_output_limb_7_col111,
            cube_252_output_limb_8_col112,
            cube_252_output_limb_9_col113,
            combination_limb_0_col114,
            combination_limb_1_col115,
            combination_limb_2_col116,
            combination_limb_3_col117,
            combination_limb_4_col118,
            combination_limb_5_col119,
            combination_limb_6_col120,
            combination_limb_7_col121,
            combination_limb_8_col122,
            combination_limb_9_col123,
            p_coef_col124,
            combination_limb_0_col125,
            combination_limb_1_col126,
            combination_limb_2_col127,
            combination_limb_3_col128,
            combination_limb_4_col129,
            combination_limb_5_col130,
            combination_limb_6_col131,
            combination_limb_7_col132,
            combination_limb_8_col133,
            combination_limb_9_col134,
            p_coef_col135,
            cube_252_output_limb_0_col136,
            cube_252_output_limb_1_col137,
            cube_252_output_limb_2_col138,
            cube_252_output_limb_3_col139,
            cube_252_output_limb_4_col140,
            cube_252_output_limb_5_col141,
            cube_252_output_limb_6_col142,
            cube_252_output_limb_7_col143,
            cube_252_output_limb_8_col144,
            cube_252_output_limb_9_col145,
            combination_limb_0_col146,
            combination_limb_1_col147,
            combination_limb_2_col148,
            combination_limb_3_col149,
            combination_limb_4_col150,
            combination_limb_5_col151,
            combination_limb_6_col152,
            combination_limb_7_col153,
            combination_limb_8_col154,
            combination_limb_9_col155,
            p_coef_col156,
            combination_limb_0_col157,
            combination_limb_1_col158,
            combination_limb_2_col159,
            combination_limb_3_col160,
            combination_limb_4_col161,
            combination_limb_5_col162,
            combination_limb_6_col163,
            combination_limb_7_col164,
            combination_limb_8_col165,
            combination_limb_9_col166,
            p_coef_col167,
            enabler,
        ]: [Span<QM31>; 169] =
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
        let [input_limb_12_col12]: [QM31; 1] = (*input_limb_12_col12.try_into().unwrap()).unbox();
        let [input_limb_13_col13]: [QM31; 1] = (*input_limb_13_col13.try_into().unwrap()).unbox();
        let [input_limb_14_col14]: [QM31; 1] = (*input_limb_14_col14.try_into().unwrap()).unbox();
        let [input_limb_15_col15]: [QM31; 1] = (*input_limb_15_col15.try_into().unwrap()).unbox();
        let [input_limb_16_col16]: [QM31; 1] = (*input_limb_16_col16.try_into().unwrap()).unbox();
        let [input_limb_17_col17]: [QM31; 1] = (*input_limb_17_col17.try_into().unwrap()).unbox();
        let [input_limb_18_col18]: [QM31; 1] = (*input_limb_18_col18.try_into().unwrap()).unbox();
        let [input_limb_19_col19]: [QM31; 1] = (*input_limb_19_col19.try_into().unwrap()).unbox();
        let [input_limb_20_col20]: [QM31; 1] = (*input_limb_20_col20.try_into().unwrap()).unbox();
        let [input_limb_21_col21]: [QM31; 1] = (*input_limb_21_col21.try_into().unwrap()).unbox();
        let [input_limb_22_col22]: [QM31; 1] = (*input_limb_22_col22.try_into().unwrap()).unbox();
        let [input_limb_23_col23]: [QM31; 1] = (*input_limb_23_col23.try_into().unwrap()).unbox();
        let [input_limb_24_col24]: [QM31; 1] = (*input_limb_24_col24.try_into().unwrap()).unbox();
        let [input_limb_25_col25]: [QM31; 1] = (*input_limb_25_col25.try_into().unwrap()).unbox();
        let [input_limb_26_col26]: [QM31; 1] = (*input_limb_26_col26.try_into().unwrap()).unbox();
        let [input_limb_27_col27]: [QM31; 1] = (*input_limb_27_col27.try_into().unwrap()).unbox();
        let [input_limb_28_col28]: [QM31; 1] = (*input_limb_28_col28.try_into().unwrap()).unbox();
        let [input_limb_29_col29]: [QM31; 1] = (*input_limb_29_col29.try_into().unwrap()).unbox();
        let [input_limb_30_col30]: [QM31; 1] = (*input_limb_30_col30.try_into().unwrap()).unbox();
        let [input_limb_31_col31]: [QM31; 1] = (*input_limb_31_col31.try_into().unwrap()).unbox();
        let [input_limb_32_col32]: [QM31; 1] = (*input_limb_32_col32.try_into().unwrap()).unbox();
        let [input_limb_33_col33]: [QM31; 1] = (*input_limb_33_col33.try_into().unwrap()).unbox();
        let [input_limb_34_col34]: [QM31; 1] = (*input_limb_34_col34.try_into().unwrap()).unbox();
        let [input_limb_35_col35]: [QM31; 1] = (*input_limb_35_col35.try_into().unwrap()).unbox();
        let [input_limb_36_col36]: [QM31; 1] = (*input_limb_36_col36.try_into().unwrap()).unbox();
        let [input_limb_37_col37]: [QM31; 1] = (*input_limb_37_col37.try_into().unwrap()).unbox();
        let [input_limb_38_col38]: [QM31; 1] = (*input_limb_38_col38.try_into().unwrap()).unbox();
        let [input_limb_39_col39]: [QM31; 1] = (*input_limb_39_col39.try_into().unwrap()).unbox();
        let [input_limb_40_col40]: [QM31; 1] = (*input_limb_40_col40.try_into().unwrap()).unbox();
        let [input_limb_41_col41]: [QM31; 1] = (*input_limb_41_col41.try_into().unwrap()).unbox();
        let [poseidon_round_keys_output_limb_0_col42]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_0_col42
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_1_col43]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_1_col43
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_2_col44]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_2_col44
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_3_col45]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_3_col45
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_4_col46]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_4_col46
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_5_col47]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_5_col47
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_6_col48]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_6_col48
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_7_col49]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_7_col49
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_8_col50]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_8_col50
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_9_col51]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_9_col51
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_10_col52]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_10_col52
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_11_col53]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_11_col53
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_12_col54]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_12_col54
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_13_col55]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_13_col55
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_14_col56]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_14_col56
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_15_col57]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_15_col57
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_16_col58]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_16_col58
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_17_col59]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_17_col59
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_18_col60]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_18_col60
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_19_col61]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_19_col61
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_20_col62]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_20_col62
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_21_col63]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_21_col63
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_22_col64]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_22_col64
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_23_col65]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_23_col65
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_24_col66]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_24_col66
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_25_col67]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_25_col67
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_26_col68]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_26_col68
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_27_col69]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_27_col69
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_28_col70]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_28_col70
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_29_col71]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_29_col71
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_0_col72]: [QM31; 1] = (*cube_252_output_limb_0_col72
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col73]: [QM31; 1] = (*cube_252_output_limb_1_col73
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col74]: [QM31; 1] = (*cube_252_output_limb_2_col74
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col75]: [QM31; 1] = (*cube_252_output_limb_3_col75
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col76]: [QM31; 1] = (*cube_252_output_limb_4_col76
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col77]: [QM31; 1] = (*cube_252_output_limb_5_col77
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col78]: [QM31; 1] = (*cube_252_output_limb_6_col78
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col79]: [QM31; 1] = (*cube_252_output_limb_7_col79
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col80]: [QM31; 1] = (*cube_252_output_limb_8_col80
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col81]: [QM31; 1] = (*cube_252_output_limb_9_col81
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col82]: [QM31; 1] = (*combination_limb_0_col82.try_into().unwrap())
            .unbox();
        let [combination_limb_1_col83]: [QM31; 1] = (*combination_limb_1_col83.try_into().unwrap())
            .unbox();
        let [combination_limb_2_col84]: [QM31; 1] = (*combination_limb_2_col84.try_into().unwrap())
            .unbox();
        let [combination_limb_3_col85]: [QM31; 1] = (*combination_limb_3_col85.try_into().unwrap())
            .unbox();
        let [combination_limb_4_col86]: [QM31; 1] = (*combination_limb_4_col86.try_into().unwrap())
            .unbox();
        let [combination_limb_5_col87]: [QM31; 1] = (*combination_limb_5_col87.try_into().unwrap())
            .unbox();
        let [combination_limb_6_col88]: [QM31; 1] = (*combination_limb_6_col88.try_into().unwrap())
            .unbox();
        let [combination_limb_7_col89]: [QM31; 1] = (*combination_limb_7_col89.try_into().unwrap())
            .unbox();
        let [combination_limb_8_col90]: [QM31; 1] = (*combination_limb_8_col90.try_into().unwrap())
            .unbox();
        let [combination_limb_9_col91]: [QM31; 1] = (*combination_limb_9_col91.try_into().unwrap())
            .unbox();
        let [p_coef_col92]: [QM31; 1] = (*p_coef_col92.try_into().unwrap()).unbox();
        let [combination_limb_0_col93]: [QM31; 1] = (*combination_limb_0_col93.try_into().unwrap())
            .unbox();
        let [combination_limb_1_col94]: [QM31; 1] = (*combination_limb_1_col94.try_into().unwrap())
            .unbox();
        let [combination_limb_2_col95]: [QM31; 1] = (*combination_limb_2_col95.try_into().unwrap())
            .unbox();
        let [combination_limb_3_col96]: [QM31; 1] = (*combination_limb_3_col96.try_into().unwrap())
            .unbox();
        let [combination_limb_4_col97]: [QM31; 1] = (*combination_limb_4_col97.try_into().unwrap())
            .unbox();
        let [combination_limb_5_col98]: [QM31; 1] = (*combination_limb_5_col98.try_into().unwrap())
            .unbox();
        let [combination_limb_6_col99]: [QM31; 1] = (*combination_limb_6_col99.try_into().unwrap())
            .unbox();
        let [combination_limb_7_col100]: [QM31; 1] = (*combination_limb_7_col100
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col101]: [QM31; 1] = (*combination_limb_8_col101
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col102]: [QM31; 1] = (*combination_limb_9_col102
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col103]: [QM31; 1] = (*p_coef_col103.try_into().unwrap()).unbox();
        let [cube_252_output_limb_0_col104]: [QM31; 1] = (*cube_252_output_limb_0_col104
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col105]: [QM31; 1] = (*cube_252_output_limb_1_col105
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col106]: [QM31; 1] = (*cube_252_output_limb_2_col106
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col107]: [QM31; 1] = (*cube_252_output_limb_3_col107
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col108]: [QM31; 1] = (*cube_252_output_limb_4_col108
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col109]: [QM31; 1] = (*cube_252_output_limb_5_col109
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col110]: [QM31; 1] = (*cube_252_output_limb_6_col110
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col111]: [QM31; 1] = (*cube_252_output_limb_7_col111
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col112]: [QM31; 1] = (*cube_252_output_limb_8_col112
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col113]: [QM31; 1] = (*cube_252_output_limb_9_col113
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col114]: [QM31; 1] = (*combination_limb_0_col114
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col115]: [QM31; 1] = (*combination_limb_1_col115
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col116]: [QM31; 1] = (*combination_limb_2_col116
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col117]: [QM31; 1] = (*combination_limb_3_col117
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col118]: [QM31; 1] = (*combination_limb_4_col118
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col119]: [QM31; 1] = (*combination_limb_5_col119
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col120]: [QM31; 1] = (*combination_limb_6_col120
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col121]: [QM31; 1] = (*combination_limb_7_col121
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col122]: [QM31; 1] = (*combination_limb_8_col122
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col123]: [QM31; 1] = (*combination_limb_9_col123
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col124]: [QM31; 1] = (*p_coef_col124.try_into().unwrap()).unbox();
        let [combination_limb_0_col125]: [QM31; 1] = (*combination_limb_0_col125
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col126]: [QM31; 1] = (*combination_limb_1_col126
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col127]: [QM31; 1] = (*combination_limb_2_col127
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col128]: [QM31; 1] = (*combination_limb_3_col128
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col129]: [QM31; 1] = (*combination_limb_4_col129
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col130]: [QM31; 1] = (*combination_limb_5_col130
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col131]: [QM31; 1] = (*combination_limb_6_col131
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col132]: [QM31; 1] = (*combination_limb_7_col132
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col133]: [QM31; 1] = (*combination_limb_8_col133
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col134]: [QM31; 1] = (*combination_limb_9_col134
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col135]: [QM31; 1] = (*p_coef_col135.try_into().unwrap()).unbox();
        let [cube_252_output_limb_0_col136]: [QM31; 1] = (*cube_252_output_limb_0_col136
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col137]: [QM31; 1] = (*cube_252_output_limb_1_col137
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col138]: [QM31; 1] = (*cube_252_output_limb_2_col138
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col139]: [QM31; 1] = (*cube_252_output_limb_3_col139
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col140]: [QM31; 1] = (*cube_252_output_limb_4_col140
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col141]: [QM31; 1] = (*cube_252_output_limb_5_col141
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col142]: [QM31; 1] = (*cube_252_output_limb_6_col142
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col143]: [QM31; 1] = (*cube_252_output_limb_7_col143
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col144]: [QM31; 1] = (*cube_252_output_limb_8_col144
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col145]: [QM31; 1] = (*cube_252_output_limb_9_col145
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col146]: [QM31; 1] = (*combination_limb_0_col146
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col147]: [QM31; 1] = (*combination_limb_1_col147
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col148]: [QM31; 1] = (*combination_limb_2_col148
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col149]: [QM31; 1] = (*combination_limb_3_col149
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col150]: [QM31; 1] = (*combination_limb_4_col150
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col151]: [QM31; 1] = (*combination_limb_5_col151
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col152]: [QM31; 1] = (*combination_limb_6_col152
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col153]: [QM31; 1] = (*combination_limb_7_col153
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col154]: [QM31; 1] = (*combination_limb_8_col154
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col155]: [QM31; 1] = (*combination_limb_9_col155
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col156]: [QM31; 1] = (*p_coef_col156.try_into().unwrap()).unbox();
        let [combination_limb_0_col157]: [QM31; 1] = (*combination_limb_0_col157
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col158]: [QM31; 1] = (*combination_limb_1_col158
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col159]: [QM31; 1] = (*combination_limb_2_col159
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col160]: [QM31; 1] = (*combination_limb_3_col160
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col161]: [QM31; 1] = (*combination_limb_4_col161
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col162]: [QM31; 1] = (*combination_limb_5_col162
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col163]: [QM31; 1] = (*combination_limb_6_col163
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col164]: [QM31; 1] = (*combination_limb_7_col164
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col165]: [QM31; 1] = (*combination_limb_8_col165
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col166]: [QM31; 1] = (*combination_limb_9_col166
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col167]: [QM31; 1] = (*p_coef_col167.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        poseidon_round_keys_sum_0 = self
            .poseidon_round_keys_lookup_elements
            .combine_qm31(
                [
                    input_limb_1_col1, poseidon_round_keys_output_limb_0_col42,
                    poseidon_round_keys_output_limb_1_col43,
                    poseidon_round_keys_output_limb_2_col44,
                    poseidon_round_keys_output_limb_3_col45,
                    poseidon_round_keys_output_limb_4_col46,
                    poseidon_round_keys_output_limb_5_col47,
                    poseidon_round_keys_output_limb_6_col48,
                    poseidon_round_keys_output_limb_7_col49,
                    poseidon_round_keys_output_limb_8_col50,
                    poseidon_round_keys_output_limb_9_col51,
                    poseidon_round_keys_output_limb_10_col52,
                    poseidon_round_keys_output_limb_11_col53,
                    poseidon_round_keys_output_limb_12_col54,
                    poseidon_round_keys_output_limb_13_col55,
                    poseidon_round_keys_output_limb_14_col56,
                    poseidon_round_keys_output_limb_15_col57,
                    poseidon_round_keys_output_limb_16_col58,
                    poseidon_round_keys_output_limb_17_col59,
                    poseidon_round_keys_output_limb_18_col60,
                    poseidon_round_keys_output_limb_19_col61,
                    poseidon_round_keys_output_limb_20_col62,
                    poseidon_round_keys_output_limb_21_col63,
                    poseidon_round_keys_output_limb_22_col64,
                    poseidon_round_keys_output_limb_23_col65,
                    poseidon_round_keys_output_limb_24_col66,
                    poseidon_round_keys_output_limb_25_col67,
                    poseidon_round_keys_output_limb_26_col68,
                    poseidon_round_keys_output_limb_27_col69,
                    poseidon_round_keys_output_limb_28_col70,
                    poseidon_round_keys_output_limb_29_col71,
                ],
            );

        let output: [QM31; 20] = poseidon_partial_round_evaluate(
            [
                input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5,
                input_limb_6_col6, input_limb_7_col7, input_limb_8_col8, input_limb_9_col9,
                input_limb_10_col10, input_limb_11_col11, input_limb_12_col12, input_limb_13_col13,
                input_limb_14_col14, input_limb_15_col15, input_limb_16_col16, input_limb_17_col17,
                input_limb_18_col18, input_limb_19_col19, input_limb_20_col20, input_limb_21_col21,
                input_limb_22_col22, input_limb_23_col23, input_limb_24_col24, input_limb_25_col25,
                input_limb_26_col26, input_limb_27_col27, input_limb_28_col28, input_limb_29_col29,
                input_limb_30_col30, input_limb_31_col31, input_limb_32_col32, input_limb_33_col33,
                input_limb_34_col34, input_limb_35_col35, input_limb_36_col36, input_limb_37_col37,
                input_limb_38_col38, input_limb_39_col39, input_limb_40_col40, input_limb_41_col41,
                poseidon_round_keys_output_limb_0_col42, poseidon_round_keys_output_limb_1_col43,
                poseidon_round_keys_output_limb_2_col44, poseidon_round_keys_output_limb_3_col45,
                poseidon_round_keys_output_limb_4_col46, poseidon_round_keys_output_limb_5_col47,
                poseidon_round_keys_output_limb_6_col48, poseidon_round_keys_output_limb_7_col49,
                poseidon_round_keys_output_limb_8_col50, poseidon_round_keys_output_limb_9_col51,
            ],
            cube_252_output_limb_0_col72,
            cube_252_output_limb_1_col73,
            cube_252_output_limb_2_col74,
            cube_252_output_limb_3_col75,
            cube_252_output_limb_4_col76,
            cube_252_output_limb_5_col77,
            cube_252_output_limb_6_col78,
            cube_252_output_limb_7_col79,
            cube_252_output_limb_8_col80,
            cube_252_output_limb_9_col81,
            combination_limb_0_col82,
            combination_limb_1_col83,
            combination_limb_2_col84,
            combination_limb_3_col85,
            combination_limb_4_col86,
            combination_limb_5_col87,
            combination_limb_6_col88,
            combination_limb_7_col89,
            combination_limb_8_col90,
            combination_limb_9_col91,
            p_coef_col92,
            combination_limb_0_col93,
            combination_limb_1_col94,
            combination_limb_2_col95,
            combination_limb_3_col96,
            combination_limb_4_col97,
            combination_limb_5_col98,
            combination_limb_6_col99,
            combination_limb_7_col100,
            combination_limb_8_col101,
            combination_limb_9_col102,
            p_coef_col103,
            self.cube_252_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            self.range_check_4_4_lookup_elements,
            self.range_check_felt_252_width_27_lookup_elements,
            ref cube_252_sum_1,
            ref range_check_4_4_4_4_sum_2,
            ref range_check_4_4_4_4_sum_3,
            ref range_check_4_4_sum_4,
            ref range_check_felt_252_width_27_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            poseidon_partial_round_output_tmp_44f04_36_limb_0,
            poseidon_partial_round_output_tmp_44f04_36_limb_1,
            poseidon_partial_round_output_tmp_44f04_36_limb_2,
            poseidon_partial_round_output_tmp_44f04_36_limb_3,
            poseidon_partial_round_output_tmp_44f04_36_limb_4,
            poseidon_partial_round_output_tmp_44f04_36_limb_5,
            poseidon_partial_round_output_tmp_44f04_36_limb_6,
            poseidon_partial_round_output_tmp_44f04_36_limb_7,
            poseidon_partial_round_output_tmp_44f04_36_limb_8,
            poseidon_partial_round_output_tmp_44f04_36_limb_9,
            poseidon_partial_round_output_tmp_44f04_36_limb_10,
            poseidon_partial_round_output_tmp_44f04_36_limb_11,
            poseidon_partial_round_output_tmp_44f04_36_limb_12,
            poseidon_partial_round_output_tmp_44f04_36_limb_13,
            poseidon_partial_round_output_tmp_44f04_36_limb_14,
            poseidon_partial_round_output_tmp_44f04_36_limb_15,
            poseidon_partial_round_output_tmp_44f04_36_limb_16,
            poseidon_partial_round_output_tmp_44f04_36_limb_17,
            poseidon_partial_round_output_tmp_44f04_36_limb_18,
            poseidon_partial_round_output_tmp_44f04_36_limb_19,
        ] =
            output;

        let output: [QM31; 20] = poseidon_partial_round_evaluate(
            [
                input_limb_22_col22, input_limb_23_col23, input_limb_24_col24, input_limb_25_col25,
                input_limb_26_col26, input_limb_27_col27, input_limb_28_col28, input_limb_29_col29,
                input_limb_30_col30, input_limb_31_col31, input_limb_32_col32, input_limb_33_col33,
                input_limb_34_col34, input_limb_35_col35, input_limb_36_col36, input_limb_37_col37,
                input_limb_38_col38, input_limb_39_col39, input_limb_40_col40, input_limb_41_col41,
                cube_252_output_limb_0_col72, cube_252_output_limb_1_col73,
                cube_252_output_limb_2_col74, cube_252_output_limb_3_col75,
                cube_252_output_limb_4_col76, cube_252_output_limb_5_col77,
                cube_252_output_limb_6_col78, cube_252_output_limb_7_col79,
                cube_252_output_limb_8_col80, cube_252_output_limb_9_col81,
                combination_limb_0_col93, combination_limb_1_col94, combination_limb_2_col95,
                combination_limb_3_col96, combination_limb_4_col97, combination_limb_5_col98,
                combination_limb_6_col99, combination_limb_7_col100, combination_limb_8_col101,
                combination_limb_9_col102, poseidon_round_keys_output_limb_10_col52,
                poseidon_round_keys_output_limb_11_col53, poseidon_round_keys_output_limb_12_col54,
                poseidon_round_keys_output_limb_13_col55, poseidon_round_keys_output_limb_14_col56,
                poseidon_round_keys_output_limb_15_col57, poseidon_round_keys_output_limb_16_col58,
                poseidon_round_keys_output_limb_17_col59, poseidon_round_keys_output_limb_18_col60,
                poseidon_round_keys_output_limb_19_col61,
            ],
            cube_252_output_limb_0_col104,
            cube_252_output_limb_1_col105,
            cube_252_output_limb_2_col106,
            cube_252_output_limb_3_col107,
            cube_252_output_limb_4_col108,
            cube_252_output_limb_5_col109,
            cube_252_output_limb_6_col110,
            cube_252_output_limb_7_col111,
            cube_252_output_limb_8_col112,
            cube_252_output_limb_9_col113,
            combination_limb_0_col114,
            combination_limb_1_col115,
            combination_limb_2_col116,
            combination_limb_3_col117,
            combination_limb_4_col118,
            combination_limb_5_col119,
            combination_limb_6_col120,
            combination_limb_7_col121,
            combination_limb_8_col122,
            combination_limb_9_col123,
            p_coef_col124,
            combination_limb_0_col125,
            combination_limb_1_col126,
            combination_limb_2_col127,
            combination_limb_3_col128,
            combination_limb_4_col129,
            combination_limb_5_col130,
            combination_limb_6_col131,
            combination_limb_7_col132,
            combination_limb_8_col133,
            combination_limb_9_col134,
            p_coef_col135,
            self.cube_252_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            self.range_check_4_4_lookup_elements,
            self.range_check_felt_252_width_27_lookup_elements,
            ref cube_252_sum_6,
            ref range_check_4_4_4_4_sum_7,
            ref range_check_4_4_4_4_sum_8,
            ref range_check_4_4_sum_9,
            ref range_check_felt_252_width_27_sum_10,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            poseidon_partial_round_output_tmp_44f04_72_limb_0,
            poseidon_partial_round_output_tmp_44f04_72_limb_1,
            poseidon_partial_round_output_tmp_44f04_72_limb_2,
            poseidon_partial_round_output_tmp_44f04_72_limb_3,
            poseidon_partial_round_output_tmp_44f04_72_limb_4,
            poseidon_partial_round_output_tmp_44f04_72_limb_5,
            poseidon_partial_round_output_tmp_44f04_72_limb_6,
            poseidon_partial_round_output_tmp_44f04_72_limb_7,
            poseidon_partial_round_output_tmp_44f04_72_limb_8,
            poseidon_partial_round_output_tmp_44f04_72_limb_9,
            poseidon_partial_round_output_tmp_44f04_72_limb_10,
            poseidon_partial_round_output_tmp_44f04_72_limb_11,
            poseidon_partial_round_output_tmp_44f04_72_limb_12,
            poseidon_partial_round_output_tmp_44f04_72_limb_13,
            poseidon_partial_round_output_tmp_44f04_72_limb_14,
            poseidon_partial_round_output_tmp_44f04_72_limb_15,
            poseidon_partial_round_output_tmp_44f04_72_limb_16,
            poseidon_partial_round_output_tmp_44f04_72_limb_17,
            poseidon_partial_round_output_tmp_44f04_72_limb_18,
            poseidon_partial_round_output_tmp_44f04_72_limb_19,
        ] =
            output;

        let output: [QM31; 20] = poseidon_partial_round_evaluate(
            [
                cube_252_output_limb_0_col72, cube_252_output_limb_1_col73,
                cube_252_output_limb_2_col74, cube_252_output_limb_3_col75,
                cube_252_output_limb_4_col76, cube_252_output_limb_5_col77,
                cube_252_output_limb_6_col78, cube_252_output_limb_7_col79,
                cube_252_output_limb_8_col80, cube_252_output_limb_9_col81,
                combination_limb_0_col93, combination_limb_1_col94, combination_limb_2_col95,
                combination_limb_3_col96, combination_limb_4_col97, combination_limb_5_col98,
                combination_limb_6_col99, combination_limb_7_col100, combination_limb_8_col101,
                combination_limb_9_col102, cube_252_output_limb_0_col104,
                cube_252_output_limb_1_col105, cube_252_output_limb_2_col106,
                cube_252_output_limb_3_col107, cube_252_output_limb_4_col108,
                cube_252_output_limb_5_col109, cube_252_output_limb_6_col110,
                cube_252_output_limb_7_col111, cube_252_output_limb_8_col112,
                cube_252_output_limb_9_col113, combination_limb_0_col125, combination_limb_1_col126,
                combination_limb_2_col127, combination_limb_3_col128, combination_limb_4_col129,
                combination_limb_5_col130, combination_limb_6_col131, combination_limb_7_col132,
                combination_limb_8_col133, combination_limb_9_col134,
                poseidon_round_keys_output_limb_20_col62, poseidon_round_keys_output_limb_21_col63,
                poseidon_round_keys_output_limb_22_col64, poseidon_round_keys_output_limb_23_col65,
                poseidon_round_keys_output_limb_24_col66, poseidon_round_keys_output_limb_25_col67,
                poseidon_round_keys_output_limb_26_col68, poseidon_round_keys_output_limb_27_col69,
                poseidon_round_keys_output_limb_28_col70, poseidon_round_keys_output_limb_29_col71,
            ],
            cube_252_output_limb_0_col136,
            cube_252_output_limb_1_col137,
            cube_252_output_limb_2_col138,
            cube_252_output_limb_3_col139,
            cube_252_output_limb_4_col140,
            cube_252_output_limb_5_col141,
            cube_252_output_limb_6_col142,
            cube_252_output_limb_7_col143,
            cube_252_output_limb_8_col144,
            cube_252_output_limb_9_col145,
            combination_limb_0_col146,
            combination_limb_1_col147,
            combination_limb_2_col148,
            combination_limb_3_col149,
            combination_limb_4_col150,
            combination_limb_5_col151,
            combination_limb_6_col152,
            combination_limb_7_col153,
            combination_limb_8_col154,
            combination_limb_9_col155,
            p_coef_col156,
            combination_limb_0_col157,
            combination_limb_1_col158,
            combination_limb_2_col159,
            combination_limb_3_col160,
            combination_limb_4_col161,
            combination_limb_5_col162,
            combination_limb_6_col163,
            combination_limb_7_col164,
            combination_limb_8_col165,
            combination_limb_9_col166,
            p_coef_col167,
            self.cube_252_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            self.range_check_4_4_lookup_elements,
            self.range_check_felt_252_width_27_lookup_elements,
            ref cube_252_sum_11,
            ref range_check_4_4_4_4_sum_12,
            ref range_check_4_4_4_4_sum_13,
            ref range_check_4_4_sum_14,
            ref range_check_felt_252_width_27_sum_15,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            poseidon_partial_round_output_tmp_44f04_108_limb_0,
            poseidon_partial_round_output_tmp_44f04_108_limb_1,
            poseidon_partial_round_output_tmp_44f04_108_limb_2,
            poseidon_partial_round_output_tmp_44f04_108_limb_3,
            poseidon_partial_round_output_tmp_44f04_108_limb_4,
            poseidon_partial_round_output_tmp_44f04_108_limb_5,
            poseidon_partial_round_output_tmp_44f04_108_limb_6,
            poseidon_partial_round_output_tmp_44f04_108_limb_7,
            poseidon_partial_round_output_tmp_44f04_108_limb_8,
            poseidon_partial_round_output_tmp_44f04_108_limb_9,
            poseidon_partial_round_output_tmp_44f04_108_limb_10,
            poseidon_partial_round_output_tmp_44f04_108_limb_11,
            poseidon_partial_round_output_tmp_44f04_108_limb_12,
            poseidon_partial_round_output_tmp_44f04_108_limb_13,
            poseidon_partial_round_output_tmp_44f04_108_limb_14,
            poseidon_partial_round_output_tmp_44f04_108_limb_15,
            poseidon_partial_round_output_tmp_44f04_108_limb_16,
            poseidon_partial_round_output_tmp_44f04_108_limb_17,
            poseidon_partial_round_output_tmp_44f04_108_limb_18,
            poseidon_partial_round_output_tmp_44f04_108_limb_19,
        ] =
            output;

        poseidon_3_partial_rounds_chain_sum_16 = self
            .poseidon_3_partial_rounds_chain_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                    input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11,
                    input_limb_12_col12, input_limb_13_col13, input_limb_14_col14,
                    input_limb_15_col15, input_limb_16_col16, input_limb_17_col17,
                    input_limb_18_col18, input_limb_19_col19, input_limb_20_col20,
                    input_limb_21_col21, input_limb_22_col22, input_limb_23_col23,
                    input_limb_24_col24, input_limb_25_col25, input_limb_26_col26,
                    input_limb_27_col27, input_limb_28_col28, input_limb_29_col29,
                    input_limb_30_col30, input_limb_31_col31, input_limb_32_col32,
                    input_limb_33_col33, input_limb_34_col34, input_limb_35_col35,
                    input_limb_36_col36, input_limb_37_col37, input_limb_38_col38,
                    input_limb_39_col39, input_limb_40_col40, input_limb_41_col41,
                ],
            );

        poseidon_3_partial_rounds_chain_sum_17 = self
            .poseidon_3_partial_rounds_chain_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, (input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
                    cube_252_output_limb_0_col104, cube_252_output_limb_1_col105,
                    cube_252_output_limb_2_col106, cube_252_output_limb_3_col107,
                    cube_252_output_limb_4_col108, cube_252_output_limb_5_col109,
                    cube_252_output_limb_6_col110, cube_252_output_limb_7_col111,
                    cube_252_output_limb_8_col112, cube_252_output_limb_9_col113,
                    combination_limb_0_col125, combination_limb_1_col126, combination_limb_2_col127,
                    combination_limb_3_col128, combination_limb_4_col129, combination_limb_5_col130,
                    combination_limb_6_col131, combination_limb_7_col132, combination_limb_8_col133,
                    combination_limb_9_col134, cube_252_output_limb_0_col136,
                    cube_252_output_limb_1_col137, cube_252_output_limb_2_col138,
                    cube_252_output_limb_3_col139, cube_252_output_limb_4_col140,
                    cube_252_output_limb_5_col141, cube_252_output_limb_6_col142,
                    cube_252_output_limb_7_col143, cube_252_output_limb_8_col144,
                    cube_252_output_limb_9_col145, combination_limb_0_col157,
                    combination_limb_1_col158, combination_limb_2_col159, combination_limb_3_col160,
                    combination_limb_4_col161, combination_limb_5_col162, combination_limb_6_col163,
                    combination_limb_7_col164, combination_limb_8_col165, combination_limb_9_col166,
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
            poseidon_round_keys_sum_0,
            cube_252_sum_1,
            range_check_4_4_4_4_sum_2,
            range_check_4_4_4_4_sum_3,
            range_check_4_4_sum_4,
            range_check_felt_252_width_27_sum_5,
            cube_252_sum_6,
            range_check_4_4_4_4_sum_7,
            range_check_4_4_4_4_sum_8,
            range_check_4_4_sum_9,
            range_check_felt_252_width_27_sum_10,
            cube_252_sum_11,
            range_check_4_4_4_4_sum_12,
            range_check_4_4_4_4_sum_13,
            range_check_4_4_sum_14,
            range_check_felt_252_width_27_sum_15,
            poseidon_3_partial_rounds_chain_sum_16,
            poseidon_3_partial_rounds_chain_sum_17,
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
    poseidon_round_keys_sum_0: QM31,
    cube_252_sum_1: QM31,
    range_check_4_4_4_4_sum_2: QM31,
    range_check_4_4_4_4_sum_3: QM31,
    range_check_4_4_sum_4: QM31,
    range_check_felt_252_width_27_sum_5: QM31,
    cube_252_sum_6: QM31,
    range_check_4_4_4_4_sum_7: QM31,
    range_check_4_4_4_4_sum_8: QM31,
    range_check_4_4_sum_9: QM31,
    range_check_felt_252_width_27_sum_10: QM31,
    cube_252_sum_11: QM31,
    range_check_4_4_4_4_sum_12: QM31,
    range_check_4_4_4_4_sum_13: QM31,
    range_check_4_4_sum_14: QM31,
    range_check_felt_252_width_27_sum_15: QM31,
    poseidon_3_partial_rounds_chain_sum_16: QM31,
    poseidon_3_partial_rounds_chain_sum_17: QM31,
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
        * poseidon_round_keys_sum_0
        * cube_252_sum_1)
        - poseidon_round_keys_sum_0
        - cube_252_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_4_4_4_4_sum_2
        * range_check_4_4_4_4_sum_3)
        - range_check_4_4_4_4_sum_2
        - range_check_4_4_4_4_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_4_4_sum_4
        * range_check_felt_252_width_27_sum_5)
        - range_check_4_4_sum_4
        - range_check_felt_252_width_27_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * cube_252_sum_6
        * range_check_4_4_4_4_sum_7)
        - cube_252_sum_6
        - range_check_4_4_4_4_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_4_4_4_4_sum_8
        * range_check_4_4_sum_9)
        - range_check_4_4_4_4_sum_8
        - range_check_4_4_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_felt_252_width_27_sum_10
        * cube_252_sum_11)
        - range_check_felt_252_width_27_sum_10
        - cube_252_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * range_check_4_4_4_4_sum_12
        * range_check_4_4_4_4_sum_13)
        - range_check_4_4_4_4_sum_12
        - range_check_4_4_4_4_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * range_check_4_4_sum_14
        * range_check_felt_252_width_27_sum_15)
        - range_check_4_4_sum_14
        - range_check_felt_252_width_27_sum_15)
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
        * poseidon_3_partial_rounds_chain_sum_16
        * poseidon_3_partial_rounds_chain_sum_17)
        + (poseidon_3_partial_rounds_chain_sum_16 * enabler)
        - (poseidon_3_partial_rounds_chain_sum_17 * enabler))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

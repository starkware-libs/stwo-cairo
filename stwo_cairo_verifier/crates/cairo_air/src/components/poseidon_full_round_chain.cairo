// Constraints version: bc855610

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
use crate::components::subroutines::linear_combination_n_4_coefs_1_1_m2_1::linear_combination_n_4_coefs_1_1_m2_1_evaluate;
use crate::components::subroutines::linear_combination_n_4_coefs_1_m1_1_1::linear_combination_n_4_coefs_1_m1_1_1_evaluate;
use crate::components::subroutines::linear_combination_n_4_coefs_3_1_1_1::linear_combination_n_4_coefs_3_1_1_1_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 126;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('Cube252', 3), ('PoseidonRoundKeys', 1), ('RangeCheck_3_3_3_3_3', 6),
    ('PoseidonFullRoundChain', 1),
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
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
    pub range_check_3_3_3_3_3_lookup_elements: crate::RangeCheck_3_3_3_3_3Elements,
    pub poseidon_full_round_chain_lookup_elements: crate::PoseidonFullRoundChainElements,
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
        let mut cube_252_sum_0: QM31 = Zero::zero();
        let mut cube_252_sum_1: QM31 = Zero::zero();
        let mut cube_252_sum_2: QM31 = Zero::zero();
        let mut poseidon_round_keys_sum_3: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_4: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_5: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_6: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_7: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_8: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_9: QM31 = Zero::zero();
        let mut poseidon_full_round_chain_sum_10: QM31 = Zero::zero();
        let mut poseidon_full_round_chain_sum_11: QM31 = Zero::zero();

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
            cube_252_output_limb_0_col32,
            cube_252_output_limb_1_col33,
            cube_252_output_limb_2_col34,
            cube_252_output_limb_3_col35,
            cube_252_output_limb_4_col36,
            cube_252_output_limb_5_col37,
            cube_252_output_limb_6_col38,
            cube_252_output_limb_7_col39,
            cube_252_output_limb_8_col40,
            cube_252_output_limb_9_col41,
            cube_252_output_limb_0_col42,
            cube_252_output_limb_1_col43,
            cube_252_output_limb_2_col44,
            cube_252_output_limb_3_col45,
            cube_252_output_limb_4_col46,
            cube_252_output_limb_5_col47,
            cube_252_output_limb_6_col48,
            cube_252_output_limb_7_col49,
            cube_252_output_limb_8_col50,
            cube_252_output_limb_9_col51,
            cube_252_output_limb_0_col52,
            cube_252_output_limb_1_col53,
            cube_252_output_limb_2_col54,
            cube_252_output_limb_3_col55,
            cube_252_output_limb_4_col56,
            cube_252_output_limb_5_col57,
            cube_252_output_limb_6_col58,
            cube_252_output_limb_7_col59,
            cube_252_output_limb_8_col60,
            cube_252_output_limb_9_col61,
            poseidon_round_keys_output_limb_0_col62,
            poseidon_round_keys_output_limb_1_col63,
            poseidon_round_keys_output_limb_2_col64,
            poseidon_round_keys_output_limb_3_col65,
            poseidon_round_keys_output_limb_4_col66,
            poseidon_round_keys_output_limb_5_col67,
            poseidon_round_keys_output_limb_6_col68,
            poseidon_round_keys_output_limb_7_col69,
            poseidon_round_keys_output_limb_8_col70,
            poseidon_round_keys_output_limb_9_col71,
            poseidon_round_keys_output_limb_10_col72,
            poseidon_round_keys_output_limb_11_col73,
            poseidon_round_keys_output_limb_12_col74,
            poseidon_round_keys_output_limb_13_col75,
            poseidon_round_keys_output_limb_14_col76,
            poseidon_round_keys_output_limb_15_col77,
            poseidon_round_keys_output_limb_16_col78,
            poseidon_round_keys_output_limb_17_col79,
            poseidon_round_keys_output_limb_18_col80,
            poseidon_round_keys_output_limb_19_col81,
            poseidon_round_keys_output_limb_20_col82,
            poseidon_round_keys_output_limb_21_col83,
            poseidon_round_keys_output_limb_22_col84,
            poseidon_round_keys_output_limb_23_col85,
            poseidon_round_keys_output_limb_24_col86,
            poseidon_round_keys_output_limb_25_col87,
            poseidon_round_keys_output_limb_26_col88,
            poseidon_round_keys_output_limb_27_col89,
            poseidon_round_keys_output_limb_28_col90,
            poseidon_round_keys_output_limb_29_col91,
            combination_limb_0_col92,
            combination_limb_1_col93,
            combination_limb_2_col94,
            combination_limb_3_col95,
            combination_limb_4_col96,
            combination_limb_5_col97,
            combination_limb_6_col98,
            combination_limb_7_col99,
            combination_limb_8_col100,
            combination_limb_9_col101,
            p_coef_col102,
            combination_limb_0_col103,
            combination_limb_1_col104,
            combination_limb_2_col105,
            combination_limb_3_col106,
            combination_limb_4_col107,
            combination_limb_5_col108,
            combination_limb_6_col109,
            combination_limb_7_col110,
            combination_limb_8_col111,
            combination_limb_9_col112,
            p_coef_col113,
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
            enabler,
        ]: [Span<QM31>; 126] =
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
        let [cube_252_output_limb_0_col32]: [QM31; 1] = (*cube_252_output_limb_0_col32
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col33]: [QM31; 1] = (*cube_252_output_limb_1_col33
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col34]: [QM31; 1] = (*cube_252_output_limb_2_col34
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col35]: [QM31; 1] = (*cube_252_output_limb_3_col35
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col36]: [QM31; 1] = (*cube_252_output_limb_4_col36
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col37]: [QM31; 1] = (*cube_252_output_limb_5_col37
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col38]: [QM31; 1] = (*cube_252_output_limb_6_col38
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col39]: [QM31; 1] = (*cube_252_output_limb_7_col39
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col40]: [QM31; 1] = (*cube_252_output_limb_8_col40
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col41]: [QM31; 1] = (*cube_252_output_limb_9_col41
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_0_col42]: [QM31; 1] = (*cube_252_output_limb_0_col42
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col43]: [QM31; 1] = (*cube_252_output_limb_1_col43
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col44]: [QM31; 1] = (*cube_252_output_limb_2_col44
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col45]: [QM31; 1] = (*cube_252_output_limb_3_col45
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col46]: [QM31; 1] = (*cube_252_output_limb_4_col46
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col47]: [QM31; 1] = (*cube_252_output_limb_5_col47
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col48]: [QM31; 1] = (*cube_252_output_limb_6_col48
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col49]: [QM31; 1] = (*cube_252_output_limb_7_col49
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col50]: [QM31; 1] = (*cube_252_output_limb_8_col50
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col51]: [QM31; 1] = (*cube_252_output_limb_9_col51
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_0_col52]: [QM31; 1] = (*cube_252_output_limb_0_col52
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col53]: [QM31; 1] = (*cube_252_output_limb_1_col53
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col54]: [QM31; 1] = (*cube_252_output_limb_2_col54
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col55]: [QM31; 1] = (*cube_252_output_limb_3_col55
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col56]: [QM31; 1] = (*cube_252_output_limb_4_col56
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col57]: [QM31; 1] = (*cube_252_output_limb_5_col57
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col58]: [QM31; 1] = (*cube_252_output_limb_6_col58
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col59]: [QM31; 1] = (*cube_252_output_limb_7_col59
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col60]: [QM31; 1] = (*cube_252_output_limb_8_col60
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col61]: [QM31; 1] = (*cube_252_output_limb_9_col61
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_0_col62]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_0_col62
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_1_col63]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_1_col63
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_2_col64]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_2_col64
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_3_col65]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_3_col65
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_4_col66]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_4_col66
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_5_col67]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_5_col67
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_6_col68]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_6_col68
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_7_col69]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_7_col69
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_8_col70]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_8_col70
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_9_col71]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_9_col71
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_10_col72]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_10_col72
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_11_col73]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_11_col73
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_12_col74]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_12_col74
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_13_col75]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_13_col75
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_14_col76]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_14_col76
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_15_col77]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_15_col77
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_16_col78]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_16_col78
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_17_col79]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_17_col79
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_18_col80]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_18_col80
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_19_col81]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_19_col81
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_20_col82]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_20_col82
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_21_col83]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_21_col83
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_22_col84]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_22_col84
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_23_col85]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_23_col85
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_24_col86]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_24_col86
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_25_col87]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_25_col87
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_26_col88]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_26_col88
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_27_col89]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_27_col89
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_28_col90]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_28_col90
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_output_limb_29_col91]: [QM31; 1] =
            (*poseidon_round_keys_output_limb_29_col91
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col92]: [QM31; 1] = (*combination_limb_0_col92.try_into().unwrap())
            .unbox();
        let [combination_limb_1_col93]: [QM31; 1] = (*combination_limb_1_col93.try_into().unwrap())
            .unbox();
        let [combination_limb_2_col94]: [QM31; 1] = (*combination_limb_2_col94.try_into().unwrap())
            .unbox();
        let [combination_limb_3_col95]: [QM31; 1] = (*combination_limb_3_col95.try_into().unwrap())
            .unbox();
        let [combination_limb_4_col96]: [QM31; 1] = (*combination_limb_4_col96.try_into().unwrap())
            .unbox();
        let [combination_limb_5_col97]: [QM31; 1] = (*combination_limb_5_col97.try_into().unwrap())
            .unbox();
        let [combination_limb_6_col98]: [QM31; 1] = (*combination_limb_6_col98.try_into().unwrap())
            .unbox();
        let [combination_limb_7_col99]: [QM31; 1] = (*combination_limb_7_col99.try_into().unwrap())
            .unbox();
        let [combination_limb_8_col100]: [QM31; 1] = (*combination_limb_8_col100
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col101]: [QM31; 1] = (*combination_limb_9_col101
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col102]: [QM31; 1] = (*p_coef_col102.try_into().unwrap()).unbox();
        let [combination_limb_0_col103]: [QM31; 1] = (*combination_limb_0_col103
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col104]: [QM31; 1] = (*combination_limb_1_col104
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col105]: [QM31; 1] = (*combination_limb_2_col105
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col106]: [QM31; 1] = (*combination_limb_3_col106
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col107]: [QM31; 1] = (*combination_limb_4_col107
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col108]: [QM31; 1] = (*combination_limb_5_col108
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col109]: [QM31; 1] = (*combination_limb_6_col109
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col110]: [QM31; 1] = (*combination_limb_7_col110
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col111]: [QM31; 1] = (*combination_limb_8_col111
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col112]: [QM31; 1] = (*combination_limb_9_col112
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col113]: [QM31; 1] = (*p_coef_col113.try_into().unwrap()).unbox();
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
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        cube_252_sum_0 = self
            .cube_252_lookup_elements
            .combine_qm31(
                [
                    input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5,
                    input_limb_6_col6, input_limb_7_col7, input_limb_8_col8, input_limb_9_col9,
                    input_limb_10_col10, input_limb_11_col11, cube_252_output_limb_0_col32,
                    cube_252_output_limb_1_col33, cube_252_output_limb_2_col34,
                    cube_252_output_limb_3_col35, cube_252_output_limb_4_col36,
                    cube_252_output_limb_5_col37, cube_252_output_limb_6_col38,
                    cube_252_output_limb_7_col39, cube_252_output_limb_8_col40,
                    cube_252_output_limb_9_col41,
                ],
            );

        cube_252_sum_1 = self
            .cube_252_lookup_elements
            .combine_qm31(
                [
                    input_limb_12_col12, input_limb_13_col13, input_limb_14_col14,
                    input_limb_15_col15, input_limb_16_col16, input_limb_17_col17,
                    input_limb_18_col18, input_limb_19_col19, input_limb_20_col20,
                    input_limb_21_col21, cube_252_output_limb_0_col42, cube_252_output_limb_1_col43,
                    cube_252_output_limb_2_col44, cube_252_output_limb_3_col45,
                    cube_252_output_limb_4_col46, cube_252_output_limb_5_col47,
                    cube_252_output_limb_6_col48, cube_252_output_limb_7_col49,
                    cube_252_output_limb_8_col50, cube_252_output_limb_9_col51,
                ],
            );

        cube_252_sum_2 = self
            .cube_252_lookup_elements
            .combine_qm31(
                [
                    input_limb_22_col22, input_limb_23_col23, input_limb_24_col24,
                    input_limb_25_col25, input_limb_26_col26, input_limb_27_col27,
                    input_limb_28_col28, input_limb_29_col29, input_limb_30_col30,
                    input_limb_31_col31, cube_252_output_limb_0_col52, cube_252_output_limb_1_col53,
                    cube_252_output_limb_2_col54, cube_252_output_limb_3_col55,
                    cube_252_output_limb_4_col56, cube_252_output_limb_5_col57,
                    cube_252_output_limb_6_col58, cube_252_output_limb_7_col59,
                    cube_252_output_limb_8_col60, cube_252_output_limb_9_col61,
                ],
            );

        poseidon_round_keys_sum_3 = self
            .poseidon_round_keys_lookup_elements
            .combine_qm31(
                [
                    input_limb_1_col1, poseidon_round_keys_output_limb_0_col62,
                    poseidon_round_keys_output_limb_1_col63,
                    poseidon_round_keys_output_limb_2_col64,
                    poseidon_round_keys_output_limb_3_col65,
                    poseidon_round_keys_output_limb_4_col66,
                    poseidon_round_keys_output_limb_5_col67,
                    poseidon_round_keys_output_limb_6_col68,
                    poseidon_round_keys_output_limb_7_col69,
                    poseidon_round_keys_output_limb_8_col70,
                    poseidon_round_keys_output_limb_9_col71,
                    poseidon_round_keys_output_limb_10_col72,
                    poseidon_round_keys_output_limb_11_col73,
                    poseidon_round_keys_output_limb_12_col74,
                    poseidon_round_keys_output_limb_13_col75,
                    poseidon_round_keys_output_limb_14_col76,
                    poseidon_round_keys_output_limb_15_col77,
                    poseidon_round_keys_output_limb_16_col78,
                    poseidon_round_keys_output_limb_17_col79,
                    poseidon_round_keys_output_limb_18_col80,
                    poseidon_round_keys_output_limb_19_col81,
                    poseidon_round_keys_output_limb_20_col82,
                    poseidon_round_keys_output_limb_21_col83,
                    poseidon_round_keys_output_limb_22_col84,
                    poseidon_round_keys_output_limb_23_col85,
                    poseidon_round_keys_output_limb_24_col86,
                    poseidon_round_keys_output_limb_25_col87,
                    poseidon_round_keys_output_limb_26_col88,
                    poseidon_round_keys_output_limb_27_col89,
                    poseidon_round_keys_output_limb_28_col90,
                    poseidon_round_keys_output_limb_29_col91,
                ],
            );

        linear_combination_n_4_coefs_3_1_1_1_evaluate(
            [
                cube_252_output_limb_0_col32, cube_252_output_limb_1_col33,
                cube_252_output_limb_2_col34, cube_252_output_limb_3_col35,
                cube_252_output_limb_4_col36, cube_252_output_limb_5_col37,
                cube_252_output_limb_6_col38, cube_252_output_limb_7_col39,
                cube_252_output_limb_8_col40, cube_252_output_limb_9_col41,
                cube_252_output_limb_0_col42, cube_252_output_limb_1_col43,
                cube_252_output_limb_2_col44, cube_252_output_limb_3_col45,
                cube_252_output_limb_4_col46, cube_252_output_limb_5_col47,
                cube_252_output_limb_6_col48, cube_252_output_limb_7_col49,
                cube_252_output_limb_8_col50, cube_252_output_limb_9_col51,
                cube_252_output_limb_0_col52, cube_252_output_limb_1_col53,
                cube_252_output_limb_2_col54, cube_252_output_limb_3_col55,
                cube_252_output_limb_4_col56, cube_252_output_limb_5_col57,
                cube_252_output_limb_6_col58, cube_252_output_limb_7_col59,
                cube_252_output_limb_8_col60, cube_252_output_limb_9_col61,
                poseidon_round_keys_output_limb_0_col62, poseidon_round_keys_output_limb_1_col63,
                poseidon_round_keys_output_limb_2_col64, poseidon_round_keys_output_limb_3_col65,
                poseidon_round_keys_output_limb_4_col66, poseidon_round_keys_output_limb_5_col67,
                poseidon_round_keys_output_limb_6_col68, poseidon_round_keys_output_limb_7_col69,
                poseidon_round_keys_output_limb_8_col70, poseidon_round_keys_output_limb_9_col71,
            ],
            combination_limb_0_col92,
            combination_limb_1_col93,
            combination_limb_2_col94,
            combination_limb_3_col95,
            combination_limb_4_col96,
            combination_limb_5_col97,
            combination_limb_6_col98,
            combination_limb_7_col99,
            combination_limb_8_col100,
            combination_limb_9_col101,
            p_coef_col102,
            self.range_check_3_3_3_3_3_lookup_elements,
            ref range_check_3_3_3_3_3_sum_4,
            ref range_check_3_3_3_3_3_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        linear_combination_n_4_coefs_1_m1_1_1_evaluate(
            [
                cube_252_output_limb_0_col32, cube_252_output_limb_1_col33,
                cube_252_output_limb_2_col34, cube_252_output_limb_3_col35,
                cube_252_output_limb_4_col36, cube_252_output_limb_5_col37,
                cube_252_output_limb_6_col38, cube_252_output_limb_7_col39,
                cube_252_output_limb_8_col40, cube_252_output_limb_9_col41,
                cube_252_output_limb_0_col42, cube_252_output_limb_1_col43,
                cube_252_output_limb_2_col44, cube_252_output_limb_3_col45,
                cube_252_output_limb_4_col46, cube_252_output_limb_5_col47,
                cube_252_output_limb_6_col48, cube_252_output_limb_7_col49,
                cube_252_output_limb_8_col50, cube_252_output_limb_9_col51,
                cube_252_output_limb_0_col52, cube_252_output_limb_1_col53,
                cube_252_output_limb_2_col54, cube_252_output_limb_3_col55,
                cube_252_output_limb_4_col56, cube_252_output_limb_5_col57,
                cube_252_output_limb_6_col58, cube_252_output_limb_7_col59,
                cube_252_output_limb_8_col60, cube_252_output_limb_9_col61,
                poseidon_round_keys_output_limb_10_col72, poseidon_round_keys_output_limb_11_col73,
                poseidon_round_keys_output_limb_12_col74, poseidon_round_keys_output_limb_13_col75,
                poseidon_round_keys_output_limb_14_col76, poseidon_round_keys_output_limb_15_col77,
                poseidon_round_keys_output_limb_16_col78, poseidon_round_keys_output_limb_17_col79,
                poseidon_round_keys_output_limb_18_col80, poseidon_round_keys_output_limb_19_col81,
            ],
            combination_limb_0_col103,
            combination_limb_1_col104,
            combination_limb_2_col105,
            combination_limb_3_col106,
            combination_limb_4_col107,
            combination_limb_5_col108,
            combination_limb_6_col109,
            combination_limb_7_col110,
            combination_limb_8_col111,
            combination_limb_9_col112,
            p_coef_col113,
            self.range_check_3_3_3_3_3_lookup_elements,
            ref range_check_3_3_3_3_3_sum_6,
            ref range_check_3_3_3_3_3_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        linear_combination_n_4_coefs_1_1_m2_1_evaluate(
            [
                cube_252_output_limb_0_col32, cube_252_output_limb_1_col33,
                cube_252_output_limb_2_col34, cube_252_output_limb_3_col35,
                cube_252_output_limb_4_col36, cube_252_output_limb_5_col37,
                cube_252_output_limb_6_col38, cube_252_output_limb_7_col39,
                cube_252_output_limb_8_col40, cube_252_output_limb_9_col41,
                cube_252_output_limb_0_col42, cube_252_output_limb_1_col43,
                cube_252_output_limb_2_col44, cube_252_output_limb_3_col45,
                cube_252_output_limb_4_col46, cube_252_output_limb_5_col47,
                cube_252_output_limb_6_col48, cube_252_output_limb_7_col49,
                cube_252_output_limb_8_col50, cube_252_output_limb_9_col51,
                cube_252_output_limb_0_col52, cube_252_output_limb_1_col53,
                cube_252_output_limb_2_col54, cube_252_output_limb_3_col55,
                cube_252_output_limb_4_col56, cube_252_output_limb_5_col57,
                cube_252_output_limb_6_col58, cube_252_output_limb_7_col59,
                cube_252_output_limb_8_col60, cube_252_output_limb_9_col61,
                poseidon_round_keys_output_limb_20_col82, poseidon_round_keys_output_limb_21_col83,
                poseidon_round_keys_output_limb_22_col84, poseidon_round_keys_output_limb_23_col85,
                poseidon_round_keys_output_limb_24_col86, poseidon_round_keys_output_limb_25_col87,
                poseidon_round_keys_output_limb_26_col88, poseidon_round_keys_output_limb_27_col89,
                poseidon_round_keys_output_limb_28_col90, poseidon_round_keys_output_limb_29_col91,
            ],
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
            self.range_check_3_3_3_3_3_lookup_elements,
            ref range_check_3_3_3_3_3_sum_8,
            ref range_check_3_3_3_3_3_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        poseidon_full_round_chain_sum_10 = self
            .poseidon_full_round_chain_lookup_elements
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
                    input_limb_30_col30, input_limb_31_col31,
                ],
            );

        poseidon_full_round_chain_sum_11 = self
            .poseidon_full_round_chain_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, (input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
                    combination_limb_0_col92, combination_limb_1_col93, combination_limb_2_col94,
                    combination_limb_3_col95, combination_limb_4_col96, combination_limb_5_col97,
                    combination_limb_6_col98, combination_limb_7_col99, combination_limb_8_col100,
                    combination_limb_9_col101, combination_limb_0_col103, combination_limb_1_col104,
                    combination_limb_2_col105, combination_limb_3_col106, combination_limb_4_col107,
                    combination_limb_5_col108, combination_limb_6_col109, combination_limb_7_col110,
                    combination_limb_8_col111, combination_limb_9_col112, combination_limb_0_col114,
                    combination_limb_1_col115, combination_limb_2_col116, combination_limb_3_col117,
                    combination_limb_4_col118, combination_limb_5_col119, combination_limb_6_col120,
                    combination_limb_7_col121, combination_limb_8_col122, combination_limb_9_col123,
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
            cube_252_sum_0,
            cube_252_sum_1,
            cube_252_sum_2,
            poseidon_round_keys_sum_3,
            range_check_3_3_3_3_3_sum_4,
            range_check_3_3_3_3_3_sum_5,
            range_check_3_3_3_3_3_sum_6,
            range_check_3_3_3_3_3_sum_7,
            range_check_3_3_3_3_3_sum_8,
            range_check_3_3_3_3_3_sum_9,
            poseidon_full_round_chain_sum_10,
            poseidon_full_round_chain_sum_11,
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
    cube_252_sum_0: QM31,
    cube_252_sum_1: QM31,
    cube_252_sum_2: QM31,
    poseidon_round_keys_sum_3: QM31,
    range_check_3_3_3_3_3_sum_4: QM31,
    range_check_3_3_3_3_3_sum_5: QM31,
    range_check_3_3_3_3_3_sum_6: QM31,
    range_check_3_3_3_3_3_sum_7: QM31,
    range_check_3_3_3_3_3_sum_8: QM31,
    range_check_3_3_3_3_3_sum_9: QM31,
    poseidon_full_round_chain_sum_10: QM31,
    poseidon_full_round_chain_sum_11: QM31,
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
        * cube_252_sum_0
        * cube_252_sum_1)
        - cube_252_sum_0
        - cube_252_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * cube_252_sum_2
        * poseidon_round_keys_sum_3)
        - cube_252_sum_2
        - poseidon_round_keys_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_3_3_3_3_3_sum_4
        * range_check_3_3_3_3_3_sum_5)
        - range_check_3_3_3_3_3_sum_4
        - range_check_3_3_3_3_3_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_3_3_3_3_3_sum_6
        * range_check_3_3_3_3_3_sum_7)
        - range_check_3_3_3_3_3_sum_6
        - range_check_3_3_3_3_3_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_3_3_3_3_3_sum_8
        * range_check_3_3_3_3_3_sum_9)
        - range_check_3_3_3_3_3_sum_8
        - range_check_3_3_3_3_3_sum_9)
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
        * poseidon_full_round_chain_sum_10
        * poseidon_full_round_chain_sum_11)
        + (poseidon_full_round_chain_sum_10 * enabler)
        - (poseidon_full_round_chain_sum_11 * enabler))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}

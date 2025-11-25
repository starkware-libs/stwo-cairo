// This file was created by the AIR team.

use crate::components::subroutines::linear_combination_n_4_coefs_1_1_m2_1::linear_combination_n_4_coefs_1_1_m2_1_evaluate;
use crate::components::subroutines::linear_combination_n_4_coefs_1_m1_1_1::linear_combination_n_4_coefs_1_m1_1_1_evaluate;
use crate::components::subroutines::linear_combination_n_4_coefs_3_1_1_1::linear_combination_n_4_coefs_3_1_1_1_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 126;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('Cube252', 3), ('PoseidonRoundKeys', 1), ('RangeCheck_3_3_3_3_3', 6),
    ('PoseidonFullRoundChain', 1),
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
        let interaction_log_sizes = [log_size; 24].span();
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
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
    pub range_check_3_3_3_3_3_lookup_elements: crate::RangeCheck_3_3_3_3_3Elements,
    pub poseidon_full_round_chain_lookup_elements: crate::PoseidonFullRoundChainElements,
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
            cube_252_lookup_elements: interaction_elements.cube_252.clone(),
            poseidon_round_keys_lookup_elements: interaction_elements.poseidon_round_keys.clone(),
            range_check_3_3_3_3_3_lookup_elements: interaction_elements
                .range_checks
                .rc_3_3_3_3_3
                .clone(),
            poseidon_full_round_chain_lookup_elements: interaction_elements
                .poseidon_full_round_chain
                .clone(),
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
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            cube_252_lookup_elements: make_lookup_elements(
                qm31_const::<1939233655, 1619044840, 261113095, 1630075268>(),
                qm31_const::<755723700, 1754586089, 2095994220, 802306310>(),
            ),
            poseidon_full_round_chain_lookup_elements: make_lookup_elements(
                qm31_const::<1798946566, 1261761511, 1965396494, 909844132>(),
                qm31_const::<1128133586, 1523205859, 844911885, 1669479084>(),
            ),
            poseidon_round_keys_lookup_elements: make_lookup_elements(
                qm31_const::<329128371, 1217552021, 2111282469, 775625911>(),
                qm31_const::<225442684, 1397510358, 1436331847, 1340164402>(),
            ),
            range_check_3_3_3_3_3_lookup_elements: make_lookup_elements(
                qm31_const::<1556236254, 464721654, 752948676, 101024730>(),
                qm31_const::<1064120346, 1019909923, 1735446893, 2115040738>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();
        let point = CirclePoint {
            x: qm31_const::<461666434, 38651694, 1083586041, 510305943>(),
            y: qm31_const::<817798294, 862569777, 2091320744, 1178484122>(),
        };

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };

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
            [qm31_const::<1651889461, 242425459, 1857910119, 343880297>()].span(),
            [qm31_const::<779367739, 645078582, 985494826, 343880277>()].span(),
            [qm31_const::<846476918, 779296310, 1052603690, 343880277>()].span(),
            [qm31_const::<913586097, 913514038, 1119712554, 343880277>()].span(),
            [qm31_const::<980695276, 1047731766, 1186821418, 343880277>()].span(),
            [qm31_const::<510931023, 108207670, 717059370, 343880277>()].span(),
            [qm31_const::<578040202, 242425398, 784168234, 343880277>()].span(),
            [qm31_const::<645149381, 376643126, 851277098, 343880277>()].span(),
            [qm31_const::<712258560, 510860854, 918385962, 343880277>()].span(),
            [qm31_const::<1316241171, 1718820406, 1522365738, 343880277>()].span(),
            [qm31_const::<1383350350, 1853038134, 1589474602, 343880277>()].span(),
            [qm31_const::<1340598866, 536394231, 1198633759, 502514173>()].span(),
            [qm31_const::<1407708045, 670611959, 1265742623, 502514173>()].span(),
            [qm31_const::<1474817224, 804829687, 1332851487, 502514173>()].span(),
            [qm31_const::<1541926403, 939047415, 1399960351, 502514173>()].span(),
            [qm31_const::<1072162150, 2147006966, 930198302, 502514173>()].span(),
            [qm31_const::<1139271329, 133741047, 997307167, 502514173>()].span(),
            [qm31_const::<1206380508, 267958775, 1064416031, 502514173>()].span(),
            [qm31_const::<1273489687, 402176503, 1131524895, 502514173>()].span(),
            [qm31_const::<1877472298, 1610136055, 1735504671, 502514173>()].span(),
            [qm31_const::<1944581477, 1744353783, 1802613535, 502514173>()].span(),
            [qm31_const::<669619552, 1341700661, 527545181, 502514194>()].span(),
            [qm31_const::<602510373, 1207482933, 460436317, 502514194>()].span(),
            [qm31_const::<535401194, 1073265205, 393327453, 502514194>()].span(),
            [qm31_const::<468292015, 939047477, 326218589, 502514194>()].span(),
            [qm31_const::<401182836, 804829749, 259109725, 502514194>()].span(),
            [qm31_const::<334073657, 670612021, 192000861, 502514194>()].span(),
            [qm31_const::<266964478, 536394293, 124891997, 502514194>()].span(),
            [qm31_const::<199855299, 402176565, 57783133, 502514194>()].span(),
            [qm31_const::<132746120, 267958837, 2138157916, 502514193>()].span(),
            [qm31_const::<65636941, 133741109, 2071049052, 502514193>()].span(),
            [qm31_const::<2146113804, 2147007087, 2003940247, 502514213>()].span(),
            [qm31_const::<65739336, 133741169, 2071049112, 502514213>()].span(),
            [qm31_const::<2011895446, 1878571631, 1869722519, 502514213>()].span(),
            [qm31_const::<2079004625, 2012789359, 1936831383, 502514213>()].span(),
            [qm31_const::<267066873, 536394353, 124892057, 502514214>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
            qm31_const::<803840495, 1824811459, 1646561508, 1941984119>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(POSEIDON_FULL_ROUND_CHAIN_SAMPLE_EVAL_RESULT))
    }
}

use crate::components::prelude::constraint_eval::*;

pub const N_TRACE_COLUMNS: usize = 169;

pub struct Eval {
    pub claim: Claim,
    pub cube_252_lookup_elements: relations::Cube252,
    pub poseidon_3_partial_rounds_chain_lookup_elements: relations::Poseidon3PartialRoundsChain,
    pub poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys,
    pub range_check_felt_252_width_27_lookup_elements: relations::RangeCheckFelt252Width27,
    pub range_check_4_4_lookup_elements: relations::RangeCheck_4_4,
    pub range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 9];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.claim.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_136 = E::F::from(M31::from(136));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_3 = E::F::from(M31::from(3));
        let M31_4 = E::F::from(M31::from(4));
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let input_limb_6_col6 = eval.next_trace_mask();
        let input_limb_7_col7 = eval.next_trace_mask();
        let input_limb_8_col8 = eval.next_trace_mask();
        let input_limb_9_col9 = eval.next_trace_mask();
        let input_limb_10_col10 = eval.next_trace_mask();
        let input_limb_11_col11 = eval.next_trace_mask();
        let input_limb_12_col12 = eval.next_trace_mask();
        let input_limb_13_col13 = eval.next_trace_mask();
        let input_limb_14_col14 = eval.next_trace_mask();
        let input_limb_15_col15 = eval.next_trace_mask();
        let input_limb_16_col16 = eval.next_trace_mask();
        let input_limb_17_col17 = eval.next_trace_mask();
        let input_limb_18_col18 = eval.next_trace_mask();
        let input_limb_19_col19 = eval.next_trace_mask();
        let input_limb_20_col20 = eval.next_trace_mask();
        let input_limb_21_col21 = eval.next_trace_mask();
        let input_limb_22_col22 = eval.next_trace_mask();
        let input_limb_23_col23 = eval.next_trace_mask();
        let input_limb_24_col24 = eval.next_trace_mask();
        let input_limb_25_col25 = eval.next_trace_mask();
        let input_limb_26_col26 = eval.next_trace_mask();
        let input_limb_27_col27 = eval.next_trace_mask();
        let input_limb_28_col28 = eval.next_trace_mask();
        let input_limb_29_col29 = eval.next_trace_mask();
        let input_limb_30_col30 = eval.next_trace_mask();
        let input_limb_31_col31 = eval.next_trace_mask();
        let input_limb_32_col32 = eval.next_trace_mask();
        let input_limb_33_col33 = eval.next_trace_mask();
        let input_limb_34_col34 = eval.next_trace_mask();
        let input_limb_35_col35 = eval.next_trace_mask();
        let input_limb_36_col36 = eval.next_trace_mask();
        let input_limb_37_col37 = eval.next_trace_mask();
        let input_limb_38_col38 = eval.next_trace_mask();
        let input_limb_39_col39 = eval.next_trace_mask();
        let input_limb_40_col40 = eval.next_trace_mask();
        let input_limb_41_col41 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_0_col42 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_1_col43 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_2_col44 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_3_col45 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_4_col46 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_5_col47 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_6_col48 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_7_col49 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_8_col50 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_9_col51 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_10_col52 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_11_col53 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_12_col54 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_13_col55 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_14_col56 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_15_col57 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_16_col58 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_17_col59 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_18_col60 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_19_col61 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_20_col62 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_21_col63 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_22_col64 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_23_col65 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_24_col66 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_25_col67 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_26_col68 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_27_col69 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_28_col70 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_29_col71 = eval.next_trace_mask();
        let cube_252_output_limb_0_col72 = eval.next_trace_mask();
        let cube_252_output_limb_1_col73 = eval.next_trace_mask();
        let cube_252_output_limb_2_col74 = eval.next_trace_mask();
        let cube_252_output_limb_3_col75 = eval.next_trace_mask();
        let cube_252_output_limb_4_col76 = eval.next_trace_mask();
        let cube_252_output_limb_5_col77 = eval.next_trace_mask();
        let cube_252_output_limb_6_col78 = eval.next_trace_mask();
        let cube_252_output_limb_7_col79 = eval.next_trace_mask();
        let cube_252_output_limb_8_col80 = eval.next_trace_mask();
        let cube_252_output_limb_9_col81 = eval.next_trace_mask();
        let combination_limb_0_col82 = eval.next_trace_mask();
        let combination_limb_1_col83 = eval.next_trace_mask();
        let combination_limb_2_col84 = eval.next_trace_mask();
        let combination_limb_3_col85 = eval.next_trace_mask();
        let combination_limb_4_col86 = eval.next_trace_mask();
        let combination_limb_5_col87 = eval.next_trace_mask();
        let combination_limb_6_col88 = eval.next_trace_mask();
        let combination_limb_7_col89 = eval.next_trace_mask();
        let combination_limb_8_col90 = eval.next_trace_mask();
        let combination_limb_9_col91 = eval.next_trace_mask();
        let p_coef_col92 = eval.next_trace_mask();
        let combination_limb_0_col93 = eval.next_trace_mask();
        let combination_limb_1_col94 = eval.next_trace_mask();
        let combination_limb_2_col95 = eval.next_trace_mask();
        let combination_limb_3_col96 = eval.next_trace_mask();
        let combination_limb_4_col97 = eval.next_trace_mask();
        let combination_limb_5_col98 = eval.next_trace_mask();
        let combination_limb_6_col99 = eval.next_trace_mask();
        let combination_limb_7_col100 = eval.next_trace_mask();
        let combination_limb_8_col101 = eval.next_trace_mask();
        let combination_limb_9_col102 = eval.next_trace_mask();
        let p_coef_col103 = eval.next_trace_mask();
        let cube_252_output_limb_0_col104 = eval.next_trace_mask();
        let cube_252_output_limb_1_col105 = eval.next_trace_mask();
        let cube_252_output_limb_2_col106 = eval.next_trace_mask();
        let cube_252_output_limb_3_col107 = eval.next_trace_mask();
        let cube_252_output_limb_4_col108 = eval.next_trace_mask();
        let cube_252_output_limb_5_col109 = eval.next_trace_mask();
        let cube_252_output_limb_6_col110 = eval.next_trace_mask();
        let cube_252_output_limb_7_col111 = eval.next_trace_mask();
        let cube_252_output_limb_8_col112 = eval.next_trace_mask();
        let cube_252_output_limb_9_col113 = eval.next_trace_mask();
        let combination_limb_0_col114 = eval.next_trace_mask();
        let combination_limb_1_col115 = eval.next_trace_mask();
        let combination_limb_2_col116 = eval.next_trace_mask();
        let combination_limb_3_col117 = eval.next_trace_mask();
        let combination_limb_4_col118 = eval.next_trace_mask();
        let combination_limb_5_col119 = eval.next_trace_mask();
        let combination_limb_6_col120 = eval.next_trace_mask();
        let combination_limb_7_col121 = eval.next_trace_mask();
        let combination_limb_8_col122 = eval.next_trace_mask();
        let combination_limb_9_col123 = eval.next_trace_mask();
        let p_coef_col124 = eval.next_trace_mask();
        let combination_limb_0_col125 = eval.next_trace_mask();
        let combination_limb_1_col126 = eval.next_trace_mask();
        let combination_limb_2_col127 = eval.next_trace_mask();
        let combination_limb_3_col128 = eval.next_trace_mask();
        let combination_limb_4_col129 = eval.next_trace_mask();
        let combination_limb_5_col130 = eval.next_trace_mask();
        let combination_limb_6_col131 = eval.next_trace_mask();
        let combination_limb_7_col132 = eval.next_trace_mask();
        let combination_limb_8_col133 = eval.next_trace_mask();
        let combination_limb_9_col134 = eval.next_trace_mask();
        let p_coef_col135 = eval.next_trace_mask();
        let cube_252_output_limb_0_col136 = eval.next_trace_mask();
        let cube_252_output_limb_1_col137 = eval.next_trace_mask();
        let cube_252_output_limb_2_col138 = eval.next_trace_mask();
        let cube_252_output_limb_3_col139 = eval.next_trace_mask();
        let cube_252_output_limb_4_col140 = eval.next_trace_mask();
        let cube_252_output_limb_5_col141 = eval.next_trace_mask();
        let cube_252_output_limb_6_col142 = eval.next_trace_mask();
        let cube_252_output_limb_7_col143 = eval.next_trace_mask();
        let cube_252_output_limb_8_col144 = eval.next_trace_mask();
        let cube_252_output_limb_9_col145 = eval.next_trace_mask();
        let combination_limb_0_col146 = eval.next_trace_mask();
        let combination_limb_1_col147 = eval.next_trace_mask();
        let combination_limb_2_col148 = eval.next_trace_mask();
        let combination_limb_3_col149 = eval.next_trace_mask();
        let combination_limb_4_col150 = eval.next_trace_mask();
        let combination_limb_5_col151 = eval.next_trace_mask();
        let combination_limb_6_col152 = eval.next_trace_mask();
        let combination_limb_7_col153 = eval.next_trace_mask();
        let combination_limb_8_col154 = eval.next_trace_mask();
        let combination_limb_9_col155 = eval.next_trace_mask();
        let p_coef_col156 = eval.next_trace_mask();
        let combination_limb_0_col157 = eval.next_trace_mask();
        let combination_limb_1_col158 = eval.next_trace_mask();
        let combination_limb_2_col159 = eval.next_trace_mask();
        let combination_limb_3_col160 = eval.next_trace_mask();
        let combination_limb_4_col161 = eval.next_trace_mask();
        let combination_limb_5_col162 = eval.next_trace_mask();
        let combination_limb_6_col163 = eval.next_trace_mask();
        let combination_limb_7_col164 = eval.next_trace_mask();
        let combination_limb_8_col165 = eval.next_trace_mask();
        let combination_limb_9_col166 = eval.next_trace_mask();
        let p_coef_col167 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());
        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_round_keys_lookup_elements,
            E::EF::one(),
            &[
                input_limb_1_col1.clone(),
                poseidon_round_keys_output_limb_0_col42.clone(),
                poseidon_round_keys_output_limb_1_col43.clone(),
                poseidon_round_keys_output_limb_2_col44.clone(),
                poseidon_round_keys_output_limb_3_col45.clone(),
                poseidon_round_keys_output_limb_4_col46.clone(),
                poseidon_round_keys_output_limb_5_col47.clone(),
                poseidon_round_keys_output_limb_6_col48.clone(),
                poseidon_round_keys_output_limb_7_col49.clone(),
                poseidon_round_keys_output_limb_8_col50.clone(),
                poseidon_round_keys_output_limb_9_col51.clone(),
                poseidon_round_keys_output_limb_10_col52.clone(),
                poseidon_round_keys_output_limb_11_col53.clone(),
                poseidon_round_keys_output_limb_12_col54.clone(),
                poseidon_round_keys_output_limb_13_col55.clone(),
                poseidon_round_keys_output_limb_14_col56.clone(),
                poseidon_round_keys_output_limb_15_col57.clone(),
                poseidon_round_keys_output_limb_16_col58.clone(),
                poseidon_round_keys_output_limb_17_col59.clone(),
                poseidon_round_keys_output_limb_18_col60.clone(),
                poseidon_round_keys_output_limb_19_col61.clone(),
                poseidon_round_keys_output_limb_20_col62.clone(),
                poseidon_round_keys_output_limb_21_col63.clone(),
                poseidon_round_keys_output_limb_22_col64.clone(),
                poseidon_round_keys_output_limb_23_col65.clone(),
                poseidon_round_keys_output_limb_24_col66.clone(),
                poseidon_round_keys_output_limb_25_col67.clone(),
                poseidon_round_keys_output_limb_26_col68.clone(),
                poseidon_round_keys_output_limb_27_col69.clone(),
                poseidon_round_keys_output_limb_28_col70.clone(),
                poseidon_round_keys_output_limb_29_col71.clone(),
            ],
        ));

        // Poseidon Partial Round.

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            E::EF::one(),
            &[
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
                cube_252_output_limb_0_col72.clone(),
                cube_252_output_limb_1_col73.clone(),
                cube_252_output_limb_2_col74.clone(),
                cube_252_output_limb_3_col75.clone(),
                cube_252_output_limb_4_col76.clone(),
                cube_252_output_limb_5_col77.clone(),
                cube_252_output_limb_6_col78.clone(),
                cube_252_output_limb_7_col79.clone(),
                cube_252_output_limb_8_col80.clone(),
                cube_252_output_limb_9_col81.clone(),
            ],
        ));

        // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

        let carry_0_tmp_44f04_4 = eval.add_intermediate(
            (((((((((M31_0.clone() - combination_limb_0_col82.clone())
                + (M31_4.clone() * input_limb_2_col2.clone()))
                + (M31_2.clone() * input_limb_12_col12.clone()))
                + (M31_3.clone() * input_limb_22_col22.clone()))
                + input_limb_32_col32.clone())
                - cube_252_output_limb_0_col72.clone())
                + poseidon_round_keys_output_limb_0_col42.clone())
                - p_coef_col92.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_44f04_5 = eval.add_intermediate(
            ((((((((carry_0_tmp_44f04_4.clone() - combination_limb_1_col83.clone())
                + (M31_4.clone() * input_limb_3_col3.clone()))
                + (M31_2.clone() * input_limb_13_col13.clone()))
                + (M31_3.clone() * input_limb_23_col23.clone()))
                + input_limb_33_col33.clone())
                - cube_252_output_limb_1_col73.clone())
                + poseidon_round_keys_output_limb_1_col43.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_44f04_6 = eval.add_intermediate(
            ((((((((carry_1_tmp_44f04_5.clone() - combination_limb_2_col84.clone())
                + (M31_4.clone() * input_limb_4_col4.clone()))
                + (M31_2.clone() * input_limb_14_col14.clone()))
                + (M31_3.clone() * input_limb_24_col24.clone()))
                + input_limb_34_col34.clone())
                - cube_252_output_limb_2_col74.clone())
                + poseidon_round_keys_output_limb_2_col44.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_44f04_7 = eval.add_intermediate(
            ((((((((carry_2_tmp_44f04_6.clone() - combination_limb_3_col85.clone())
                + (M31_4.clone() * input_limb_5_col5.clone()))
                + (M31_2.clone() * input_limb_15_col15.clone()))
                + (M31_3.clone() * input_limb_25_col25.clone()))
                + input_limb_35_col35.clone())
                - cube_252_output_limb_3_col75.clone())
                + poseidon_round_keys_output_limb_3_col45.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_44f04_8 = eval.add_intermediate(
            ((((((((carry_3_tmp_44f04_7.clone() - combination_limb_4_col86.clone())
                + (M31_4.clone() * input_limb_6_col6.clone()))
                + (M31_2.clone() * input_limb_16_col16.clone()))
                + (M31_3.clone() * input_limb_26_col26.clone()))
                + input_limb_36_col36.clone())
                - cube_252_output_limb_4_col76.clone())
                + poseidon_round_keys_output_limb_4_col46.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_44f04_9 = eval.add_intermediate(
            ((((((((carry_4_tmp_44f04_8.clone() - combination_limb_5_col87.clone())
                + (M31_4.clone() * input_limb_7_col7.clone()))
                + (M31_2.clone() * input_limb_17_col17.clone()))
                + (M31_3.clone() * input_limb_27_col27.clone()))
                + input_limb_37_col37.clone())
                - cube_252_output_limb_5_col77.clone())
                + poseidon_round_keys_output_limb_5_col47.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_44f04_10 = eval.add_intermediate(
            ((((((((carry_5_tmp_44f04_9.clone() - combination_limb_6_col88.clone())
                + (M31_4.clone() * input_limb_8_col8.clone()))
                + (M31_2.clone() * input_limb_18_col18.clone()))
                + (M31_3.clone() * input_limb_28_col28.clone()))
                + input_limb_38_col38.clone())
                - cube_252_output_limb_6_col78.clone())
                + poseidon_round_keys_output_limb_6_col48.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_44f04_11 = eval.add_intermediate(
            (((((((((carry_6_tmp_44f04_10.clone() - combination_limb_7_col89.clone())
                + (M31_4.clone() * input_limb_9_col9.clone()))
                + (M31_2.clone() * input_limb_19_col19.clone()))
                + (M31_3.clone() * input_limb_29_col29.clone()))
                + input_limb_39_col39.clone())
                - cube_252_output_limb_7_col79.clone())
                + poseidon_round_keys_output_limb_7_col49.clone())
                - (p_coef_col92.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_44f04_12 = eval.add_intermediate(
            ((((((((carry_7_tmp_44f04_11.clone() - combination_limb_8_col90.clone())
                + (M31_4.clone() * input_limb_10_col10.clone()))
                + (M31_2.clone() * input_limb_20_col20.clone()))
                + (M31_3.clone() * input_limb_30_col30.clone()))
                + input_limb_40_col40.clone())
                - cube_252_output_limb_8_col80.clone())
                + poseidon_round_keys_output_limb_8_col50.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((((carry_8_tmp_44f04_12.clone() - combination_limb_9_col91.clone())
                + (M31_4.clone() * input_limb_11_col11.clone()))
                + (M31_2.clone() * input_limb_21_col21.clone()))
                + (M31_3.clone() * input_limb_31_col31.clone()))
                + input_limb_41_col41.clone())
                - cube_252_output_limb_9_col81.clone())
                + poseidon_round_keys_output_limb_9_col51.clone())
                - (p_coef_col92.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col92.clone() + M31_2.clone()),
                (carry_0_tmp_44f04_4.clone() + M31_2.clone()),
                (carry_1_tmp_44f04_5.clone() + M31_2.clone()),
                (carry_2_tmp_44f04_6.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                (carry_3_tmp_44f04_7.clone() + M31_2.clone()),
                (carry_4_tmp_44f04_8.clone() + M31_2.clone()),
                (carry_5_tmp_44f04_9.clone() + M31_2.clone()),
                (carry_6_tmp_44f04_10.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_lookup_elements,
            E::EF::one(),
            &[
                (carry_7_tmp_44f04_11.clone() + M31_2.clone()),
                (carry_8_tmp_44f04_12.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_felt_252_width_27_lookup_elements,
            E::EF::one(),
            &[
                combination_limb_0_col82.clone(),
                combination_limb_1_col83.clone(),
                combination_limb_2_col84.clone(),
                combination_limb_3_col85.clone(),
                combination_limb_4_col86.clone(),
                combination_limb_5_col87.clone(),
                combination_limb_6_col88.clone(),
                combination_limb_7_col89.clone(),
                combination_limb_8_col90.clone(),
                combination_limb_9_col91.clone(),
            ],
        ));

        // Linear Combination N 1 Coefs 2.

        let carry_0_tmp_44f04_15 = eval.add_intermediate(
            ((((M31_0.clone() - combination_limb_0_col93.clone())
                + (M31_2.clone() * combination_limb_0_col82.clone()))
                - p_coef_col103.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_44f04_16 = eval.add_intermediate(
            (((carry_0_tmp_44f04_15.clone() - combination_limb_1_col94.clone())
                + (M31_2.clone() * combination_limb_1_col83.clone()))
                * M31_16.clone()),
        );
        let carry_2_tmp_44f04_17 = eval.add_intermediate(
            (((carry_1_tmp_44f04_16.clone() - combination_limb_2_col95.clone())
                + (M31_2.clone() * combination_limb_2_col84.clone()))
                * M31_16.clone()),
        );
        let carry_3_tmp_44f04_18 = eval.add_intermediate(
            (((carry_2_tmp_44f04_17.clone() - combination_limb_3_col96.clone())
                + (M31_2.clone() * combination_limb_3_col85.clone()))
                * M31_16.clone()),
        );
        let carry_4_tmp_44f04_19 = eval.add_intermediate(
            (((carry_3_tmp_44f04_18.clone() - combination_limb_4_col97.clone())
                + (M31_2.clone() * combination_limb_4_col86.clone()))
                * M31_16.clone()),
        );
        let carry_5_tmp_44f04_20 = eval.add_intermediate(
            (((carry_4_tmp_44f04_19.clone() - combination_limb_5_col98.clone())
                + (M31_2.clone() * combination_limb_5_col87.clone()))
                * M31_16.clone()),
        );
        let carry_6_tmp_44f04_21 = eval.add_intermediate(
            (((carry_5_tmp_44f04_20.clone() - combination_limb_6_col99.clone())
                + (M31_2.clone() * combination_limb_6_col88.clone()))
                * M31_16.clone()),
        );
        let carry_7_tmp_44f04_22 = eval.add_intermediate(
            ((((carry_6_tmp_44f04_21.clone() - combination_limb_7_col100.clone())
                + (M31_2.clone() * combination_limb_7_col89.clone()))
                - (p_coef_col103.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_44f04_23 = eval.add_intermediate(
            (((carry_7_tmp_44f04_22.clone() - combination_limb_8_col101.clone())
                + (M31_2.clone() * combination_limb_8_col90.clone()))
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_8_tmp_44f04_23.clone() - combination_limb_9_col102.clone())
                + (M31_2.clone() * combination_limb_9_col91.clone()))
                - (p_coef_col103.clone() * M31_256.clone())),
        );
        let biased_carry_0_tmp_44f04_24 =
            eval.add_intermediate(((p_coef_col103.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 0.
        eval.add_constraint(
            (((biased_carry_0_tmp_44f04_24.clone() * biased_carry_0_tmp_44f04_24.clone())
                * biased_carry_0_tmp_44f04_24.clone())
                - biased_carry_0_tmp_44f04_24.clone()),
        );
        let biased_carry_1_tmp_44f04_25 =
            eval.add_intermediate(((carry_0_tmp_44f04_15.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 1.
        eval.add_constraint(
            (((biased_carry_1_tmp_44f04_25.clone() * biased_carry_1_tmp_44f04_25.clone())
                * biased_carry_1_tmp_44f04_25.clone())
                - biased_carry_1_tmp_44f04_25.clone()),
        );
        let biased_carry_2_tmp_44f04_26 =
            eval.add_intermediate(((carry_1_tmp_44f04_16.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 2.
        eval.add_constraint(
            (((biased_carry_2_tmp_44f04_26.clone() * biased_carry_2_tmp_44f04_26.clone())
                * biased_carry_2_tmp_44f04_26.clone())
                - biased_carry_2_tmp_44f04_26.clone()),
        );
        let biased_carry_3_tmp_44f04_27 =
            eval.add_intermediate(((carry_2_tmp_44f04_17.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 3.
        eval.add_constraint(
            (((biased_carry_3_tmp_44f04_27.clone() * biased_carry_3_tmp_44f04_27.clone())
                * biased_carry_3_tmp_44f04_27.clone())
                - biased_carry_3_tmp_44f04_27.clone()),
        );
        let biased_carry_4_tmp_44f04_28 =
            eval.add_intermediate(((carry_3_tmp_44f04_18.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 4.
        eval.add_constraint(
            (((biased_carry_4_tmp_44f04_28.clone() * biased_carry_4_tmp_44f04_28.clone())
                * biased_carry_4_tmp_44f04_28.clone())
                - biased_carry_4_tmp_44f04_28.clone()),
        );
        let biased_carry_5_tmp_44f04_29 =
            eval.add_intermediate(((carry_4_tmp_44f04_19.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 5.
        eval.add_constraint(
            (((biased_carry_5_tmp_44f04_29.clone() * biased_carry_5_tmp_44f04_29.clone())
                * biased_carry_5_tmp_44f04_29.clone())
                - biased_carry_5_tmp_44f04_29.clone()),
        );
        let biased_carry_6_tmp_44f04_30 =
            eval.add_intermediate(((carry_5_tmp_44f04_20.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 6.
        eval.add_constraint(
            (((biased_carry_6_tmp_44f04_30.clone() * biased_carry_6_tmp_44f04_30.clone())
                * biased_carry_6_tmp_44f04_30.clone())
                - biased_carry_6_tmp_44f04_30.clone()),
        );
        let biased_carry_7_tmp_44f04_31 =
            eval.add_intermediate(((carry_6_tmp_44f04_21.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 7.
        eval.add_constraint(
            (((biased_carry_7_tmp_44f04_31.clone() * biased_carry_7_tmp_44f04_31.clone())
                * biased_carry_7_tmp_44f04_31.clone())
                - biased_carry_7_tmp_44f04_31.clone()),
        );
        let biased_carry_8_tmp_44f04_32 =
            eval.add_intermediate(((carry_7_tmp_44f04_22.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 8.
        eval.add_constraint(
            (((biased_carry_8_tmp_44f04_32.clone() * biased_carry_8_tmp_44f04_32.clone())
                * biased_carry_8_tmp_44f04_32.clone())
                - biased_carry_8_tmp_44f04_32.clone()),
        );
        let biased_carry_9_tmp_44f04_33 =
            eval.add_intermediate(((carry_8_tmp_44f04_23.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 9.
        eval.add_constraint(
            (((biased_carry_9_tmp_44f04_33.clone() * biased_carry_9_tmp_44f04_33.clone())
                * biased_carry_9_tmp_44f04_33.clone())
                - biased_carry_9_tmp_44f04_33.clone()),
        );

        // Poseidon Partial Round.

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            E::EF::one(),
            &[
                combination_limb_0_col93.clone(),
                combination_limb_1_col94.clone(),
                combination_limb_2_col95.clone(),
                combination_limb_3_col96.clone(),
                combination_limb_4_col97.clone(),
                combination_limb_5_col98.clone(),
                combination_limb_6_col99.clone(),
                combination_limb_7_col100.clone(),
                combination_limb_8_col101.clone(),
                combination_limb_9_col102.clone(),
                cube_252_output_limb_0_col104.clone(),
                cube_252_output_limb_1_col105.clone(),
                cube_252_output_limb_2_col106.clone(),
                cube_252_output_limb_3_col107.clone(),
                cube_252_output_limb_4_col108.clone(),
                cube_252_output_limb_5_col109.clone(),
                cube_252_output_limb_6_col110.clone(),
                cube_252_output_limb_7_col111.clone(),
                cube_252_output_limb_8_col112.clone(),
                cube_252_output_limb_9_col113.clone(),
            ],
        ));

        // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

        let carry_0_tmp_44f04_37 = eval.add_intermediate(
            (((((((((M31_0.clone() - combination_limb_0_col114.clone())
                + (M31_4.clone() * input_limb_22_col22.clone()))
                + (M31_2.clone() * input_limb_32_col32.clone()))
                + (M31_3.clone() * cube_252_output_limb_0_col72.clone()))
                + combination_limb_0_col93.clone())
                - cube_252_output_limb_0_col104.clone())
                + poseidon_round_keys_output_limb_10_col52.clone())
                - p_coef_col124.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_44f04_38 = eval.add_intermediate(
            ((((((((carry_0_tmp_44f04_37.clone() - combination_limb_1_col115.clone())
                + (M31_4.clone() * input_limb_23_col23.clone()))
                + (M31_2.clone() * input_limb_33_col33.clone()))
                + (M31_3.clone() * cube_252_output_limb_1_col73.clone()))
                + combination_limb_1_col94.clone())
                - cube_252_output_limb_1_col105.clone())
                + poseidon_round_keys_output_limb_11_col53.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_44f04_39 = eval.add_intermediate(
            ((((((((carry_1_tmp_44f04_38.clone() - combination_limb_2_col116.clone())
                + (M31_4.clone() * input_limb_24_col24.clone()))
                + (M31_2.clone() * input_limb_34_col34.clone()))
                + (M31_3.clone() * cube_252_output_limb_2_col74.clone()))
                + combination_limb_2_col95.clone())
                - cube_252_output_limb_2_col106.clone())
                + poseidon_round_keys_output_limb_12_col54.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_44f04_40 = eval.add_intermediate(
            ((((((((carry_2_tmp_44f04_39.clone() - combination_limb_3_col117.clone())
                + (M31_4.clone() * input_limb_25_col25.clone()))
                + (M31_2.clone() * input_limb_35_col35.clone()))
                + (M31_3.clone() * cube_252_output_limb_3_col75.clone()))
                + combination_limb_3_col96.clone())
                - cube_252_output_limb_3_col107.clone())
                + poseidon_round_keys_output_limb_13_col55.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_44f04_41 = eval.add_intermediate(
            ((((((((carry_3_tmp_44f04_40.clone() - combination_limb_4_col118.clone())
                + (M31_4.clone() * input_limb_26_col26.clone()))
                + (M31_2.clone() * input_limb_36_col36.clone()))
                + (M31_3.clone() * cube_252_output_limb_4_col76.clone()))
                + combination_limb_4_col97.clone())
                - cube_252_output_limb_4_col108.clone())
                + poseidon_round_keys_output_limb_14_col56.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_44f04_42 = eval.add_intermediate(
            ((((((((carry_4_tmp_44f04_41.clone() - combination_limb_5_col119.clone())
                + (M31_4.clone() * input_limb_27_col27.clone()))
                + (M31_2.clone() * input_limb_37_col37.clone()))
                + (M31_3.clone() * cube_252_output_limb_5_col77.clone()))
                + combination_limb_5_col98.clone())
                - cube_252_output_limb_5_col109.clone())
                + poseidon_round_keys_output_limb_15_col57.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_44f04_43 = eval.add_intermediate(
            ((((((((carry_5_tmp_44f04_42.clone() - combination_limb_6_col120.clone())
                + (M31_4.clone() * input_limb_28_col28.clone()))
                + (M31_2.clone() * input_limb_38_col38.clone()))
                + (M31_3.clone() * cube_252_output_limb_6_col78.clone()))
                + combination_limb_6_col99.clone())
                - cube_252_output_limb_6_col110.clone())
                + poseidon_round_keys_output_limb_16_col58.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_44f04_44 = eval.add_intermediate(
            (((((((((carry_6_tmp_44f04_43.clone() - combination_limb_7_col121.clone())
                + (M31_4.clone() * input_limb_29_col29.clone()))
                + (M31_2.clone() * input_limb_39_col39.clone()))
                + (M31_3.clone() * cube_252_output_limb_7_col79.clone()))
                + combination_limb_7_col100.clone())
                - cube_252_output_limb_7_col111.clone())
                + poseidon_round_keys_output_limb_17_col59.clone())
                - (p_coef_col124.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_44f04_45 = eval.add_intermediate(
            ((((((((carry_7_tmp_44f04_44.clone() - combination_limb_8_col122.clone())
                + (M31_4.clone() * input_limb_30_col30.clone()))
                + (M31_2.clone() * input_limb_40_col40.clone()))
                + (M31_3.clone() * cube_252_output_limb_8_col80.clone()))
                + combination_limb_8_col101.clone())
                - cube_252_output_limb_8_col112.clone())
                + poseidon_round_keys_output_limb_18_col60.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((((carry_8_tmp_44f04_45.clone() - combination_limb_9_col123.clone())
                + (M31_4.clone() * input_limb_31_col31.clone()))
                + (M31_2.clone() * input_limb_41_col41.clone()))
                + (M31_3.clone() * cube_252_output_limb_9_col81.clone()))
                + combination_limb_9_col102.clone())
                - cube_252_output_limb_9_col113.clone())
                + poseidon_round_keys_output_limb_19_col61.clone())
                - (p_coef_col124.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col124.clone() + M31_2.clone()),
                (carry_0_tmp_44f04_37.clone() + M31_2.clone()),
                (carry_1_tmp_44f04_38.clone() + M31_2.clone()),
                (carry_2_tmp_44f04_39.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                (carry_3_tmp_44f04_40.clone() + M31_2.clone()),
                (carry_4_tmp_44f04_41.clone() + M31_2.clone()),
                (carry_5_tmp_44f04_42.clone() + M31_2.clone()),
                (carry_6_tmp_44f04_43.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_lookup_elements,
            E::EF::one(),
            &[
                (carry_7_tmp_44f04_44.clone() + M31_2.clone()),
                (carry_8_tmp_44f04_45.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_felt_252_width_27_lookup_elements,
            E::EF::one(),
            &[
                combination_limb_0_col114.clone(),
                combination_limb_1_col115.clone(),
                combination_limb_2_col116.clone(),
                combination_limb_3_col117.clone(),
                combination_limb_4_col118.clone(),
                combination_limb_5_col119.clone(),
                combination_limb_6_col120.clone(),
                combination_limb_7_col121.clone(),
                combination_limb_8_col122.clone(),
                combination_limb_9_col123.clone(),
            ],
        ));

        // Linear Combination N 1 Coefs 2.

        let carry_0_tmp_44f04_48 = eval.add_intermediate(
            ((((M31_0.clone() - combination_limb_0_col125.clone())
                + (M31_2.clone() * combination_limb_0_col114.clone()))
                - p_coef_col135.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_44f04_49 = eval.add_intermediate(
            (((carry_0_tmp_44f04_48.clone() - combination_limb_1_col126.clone())
                + (M31_2.clone() * combination_limb_1_col115.clone()))
                * M31_16.clone()),
        );
        let carry_2_tmp_44f04_50 = eval.add_intermediate(
            (((carry_1_tmp_44f04_49.clone() - combination_limb_2_col127.clone())
                + (M31_2.clone() * combination_limb_2_col116.clone()))
                * M31_16.clone()),
        );
        let carry_3_tmp_44f04_51 = eval.add_intermediate(
            (((carry_2_tmp_44f04_50.clone() - combination_limb_3_col128.clone())
                + (M31_2.clone() * combination_limb_3_col117.clone()))
                * M31_16.clone()),
        );
        let carry_4_tmp_44f04_52 = eval.add_intermediate(
            (((carry_3_tmp_44f04_51.clone() - combination_limb_4_col129.clone())
                + (M31_2.clone() * combination_limb_4_col118.clone()))
                * M31_16.clone()),
        );
        let carry_5_tmp_44f04_53 = eval.add_intermediate(
            (((carry_4_tmp_44f04_52.clone() - combination_limb_5_col130.clone())
                + (M31_2.clone() * combination_limb_5_col119.clone()))
                * M31_16.clone()),
        );
        let carry_6_tmp_44f04_54 = eval.add_intermediate(
            (((carry_5_tmp_44f04_53.clone() - combination_limb_6_col131.clone())
                + (M31_2.clone() * combination_limb_6_col120.clone()))
                * M31_16.clone()),
        );
        let carry_7_tmp_44f04_55 = eval.add_intermediate(
            ((((carry_6_tmp_44f04_54.clone() - combination_limb_7_col132.clone())
                + (M31_2.clone() * combination_limb_7_col121.clone()))
                - (p_coef_col135.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_44f04_56 = eval.add_intermediate(
            (((carry_7_tmp_44f04_55.clone() - combination_limb_8_col133.clone())
                + (M31_2.clone() * combination_limb_8_col122.clone()))
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_8_tmp_44f04_56.clone() - combination_limb_9_col134.clone())
                + (M31_2.clone() * combination_limb_9_col123.clone()))
                - (p_coef_col135.clone() * M31_256.clone())),
        );
        let biased_carry_0_tmp_44f04_57 =
            eval.add_intermediate(((p_coef_col135.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 0.
        eval.add_constraint(
            (((biased_carry_0_tmp_44f04_57.clone() * biased_carry_0_tmp_44f04_57.clone())
                * biased_carry_0_tmp_44f04_57.clone())
                - biased_carry_0_tmp_44f04_57.clone()),
        );
        let biased_carry_1_tmp_44f04_58 =
            eval.add_intermediate(((carry_0_tmp_44f04_48.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 1.
        eval.add_constraint(
            (((biased_carry_1_tmp_44f04_58.clone() * biased_carry_1_tmp_44f04_58.clone())
                * biased_carry_1_tmp_44f04_58.clone())
                - biased_carry_1_tmp_44f04_58.clone()),
        );
        let biased_carry_2_tmp_44f04_59 =
            eval.add_intermediate(((carry_1_tmp_44f04_49.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 2.
        eval.add_constraint(
            (((biased_carry_2_tmp_44f04_59.clone() * biased_carry_2_tmp_44f04_59.clone())
                * biased_carry_2_tmp_44f04_59.clone())
                - biased_carry_2_tmp_44f04_59.clone()),
        );
        let biased_carry_3_tmp_44f04_60 =
            eval.add_intermediate(((carry_2_tmp_44f04_50.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 3.
        eval.add_constraint(
            (((biased_carry_3_tmp_44f04_60.clone() * biased_carry_3_tmp_44f04_60.clone())
                * biased_carry_3_tmp_44f04_60.clone())
                - biased_carry_3_tmp_44f04_60.clone()),
        );
        let biased_carry_4_tmp_44f04_61 =
            eval.add_intermediate(((carry_3_tmp_44f04_51.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 4.
        eval.add_constraint(
            (((biased_carry_4_tmp_44f04_61.clone() * biased_carry_4_tmp_44f04_61.clone())
                * biased_carry_4_tmp_44f04_61.clone())
                - biased_carry_4_tmp_44f04_61.clone()),
        );
        let biased_carry_5_tmp_44f04_62 =
            eval.add_intermediate(((carry_4_tmp_44f04_52.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 5.
        eval.add_constraint(
            (((biased_carry_5_tmp_44f04_62.clone() * biased_carry_5_tmp_44f04_62.clone())
                * biased_carry_5_tmp_44f04_62.clone())
                - biased_carry_5_tmp_44f04_62.clone()),
        );
        let biased_carry_6_tmp_44f04_63 =
            eval.add_intermediate(((carry_5_tmp_44f04_53.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 6.
        eval.add_constraint(
            (((biased_carry_6_tmp_44f04_63.clone() * biased_carry_6_tmp_44f04_63.clone())
                * biased_carry_6_tmp_44f04_63.clone())
                - biased_carry_6_tmp_44f04_63.clone()),
        );
        let biased_carry_7_tmp_44f04_64 =
            eval.add_intermediate(((carry_6_tmp_44f04_54.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 7.
        eval.add_constraint(
            (((biased_carry_7_tmp_44f04_64.clone() * biased_carry_7_tmp_44f04_64.clone())
                * biased_carry_7_tmp_44f04_64.clone())
                - biased_carry_7_tmp_44f04_64.clone()),
        );
        let biased_carry_8_tmp_44f04_65 =
            eval.add_intermediate(((carry_7_tmp_44f04_55.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 8.
        eval.add_constraint(
            (((biased_carry_8_tmp_44f04_65.clone() * biased_carry_8_tmp_44f04_65.clone())
                * biased_carry_8_tmp_44f04_65.clone())
                - biased_carry_8_tmp_44f04_65.clone()),
        );
        let biased_carry_9_tmp_44f04_66 =
            eval.add_intermediate(((carry_8_tmp_44f04_56.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 9.
        eval.add_constraint(
            (((biased_carry_9_tmp_44f04_66.clone() * biased_carry_9_tmp_44f04_66.clone())
                * biased_carry_9_tmp_44f04_66.clone())
                - biased_carry_9_tmp_44f04_66.clone()),
        );

        // Poseidon Partial Round.

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            E::EF::one(),
            &[
                combination_limb_0_col125.clone(),
                combination_limb_1_col126.clone(),
                combination_limb_2_col127.clone(),
                combination_limb_3_col128.clone(),
                combination_limb_4_col129.clone(),
                combination_limb_5_col130.clone(),
                combination_limb_6_col131.clone(),
                combination_limb_7_col132.clone(),
                combination_limb_8_col133.clone(),
                combination_limb_9_col134.clone(),
                cube_252_output_limb_0_col136.clone(),
                cube_252_output_limb_1_col137.clone(),
                cube_252_output_limb_2_col138.clone(),
                cube_252_output_limb_3_col139.clone(),
                cube_252_output_limb_4_col140.clone(),
                cube_252_output_limb_5_col141.clone(),
                cube_252_output_limb_6_col142.clone(),
                cube_252_output_limb_7_col143.clone(),
                cube_252_output_limb_8_col144.clone(),
                cube_252_output_limb_9_col145.clone(),
            ],
        ));

        // Linear Combination N 6 Coefs 4 2 3 1 M 1 1.

        let carry_0_tmp_44f04_70 = eval.add_intermediate(
            (((((((((M31_0.clone() - combination_limb_0_col146.clone())
                + (M31_4.clone() * cube_252_output_limb_0_col72.clone()))
                + (M31_2.clone() * combination_limb_0_col93.clone()))
                + (M31_3.clone() * cube_252_output_limb_0_col104.clone()))
                + combination_limb_0_col125.clone())
                - cube_252_output_limb_0_col136.clone())
                + poseidon_round_keys_output_limb_20_col62.clone())
                - p_coef_col156.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_44f04_71 = eval.add_intermediate(
            ((((((((carry_0_tmp_44f04_70.clone() - combination_limb_1_col147.clone())
                + (M31_4.clone() * cube_252_output_limb_1_col73.clone()))
                + (M31_2.clone() * combination_limb_1_col94.clone()))
                + (M31_3.clone() * cube_252_output_limb_1_col105.clone()))
                + combination_limb_1_col126.clone())
                - cube_252_output_limb_1_col137.clone())
                + poseidon_round_keys_output_limb_21_col63.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_44f04_72 = eval.add_intermediate(
            ((((((((carry_1_tmp_44f04_71.clone() - combination_limb_2_col148.clone())
                + (M31_4.clone() * cube_252_output_limb_2_col74.clone()))
                + (M31_2.clone() * combination_limb_2_col95.clone()))
                + (M31_3.clone() * cube_252_output_limb_2_col106.clone()))
                + combination_limb_2_col127.clone())
                - cube_252_output_limb_2_col138.clone())
                + poseidon_round_keys_output_limb_22_col64.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_44f04_73 = eval.add_intermediate(
            ((((((((carry_2_tmp_44f04_72.clone() - combination_limb_3_col149.clone())
                + (M31_4.clone() * cube_252_output_limb_3_col75.clone()))
                + (M31_2.clone() * combination_limb_3_col96.clone()))
                + (M31_3.clone() * cube_252_output_limb_3_col107.clone()))
                + combination_limb_3_col128.clone())
                - cube_252_output_limb_3_col139.clone())
                + poseidon_round_keys_output_limb_23_col65.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_44f04_74 = eval.add_intermediate(
            ((((((((carry_3_tmp_44f04_73.clone() - combination_limb_4_col150.clone())
                + (M31_4.clone() * cube_252_output_limb_4_col76.clone()))
                + (M31_2.clone() * combination_limb_4_col97.clone()))
                + (M31_3.clone() * cube_252_output_limb_4_col108.clone()))
                + combination_limb_4_col129.clone())
                - cube_252_output_limb_4_col140.clone())
                + poseidon_round_keys_output_limb_24_col66.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_44f04_75 = eval.add_intermediate(
            ((((((((carry_4_tmp_44f04_74.clone() - combination_limb_5_col151.clone())
                + (M31_4.clone() * cube_252_output_limb_5_col77.clone()))
                + (M31_2.clone() * combination_limb_5_col98.clone()))
                + (M31_3.clone() * cube_252_output_limb_5_col109.clone()))
                + combination_limb_5_col130.clone())
                - cube_252_output_limb_5_col141.clone())
                + poseidon_round_keys_output_limb_25_col67.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_44f04_76 = eval.add_intermediate(
            ((((((((carry_5_tmp_44f04_75.clone() - combination_limb_6_col152.clone())
                + (M31_4.clone() * cube_252_output_limb_6_col78.clone()))
                + (M31_2.clone() * combination_limb_6_col99.clone()))
                + (M31_3.clone() * cube_252_output_limb_6_col110.clone()))
                + combination_limb_6_col131.clone())
                - cube_252_output_limb_6_col142.clone())
                + poseidon_round_keys_output_limb_26_col68.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_44f04_77 = eval.add_intermediate(
            (((((((((carry_6_tmp_44f04_76.clone() - combination_limb_7_col153.clone())
                + (M31_4.clone() * cube_252_output_limb_7_col79.clone()))
                + (M31_2.clone() * combination_limb_7_col100.clone()))
                + (M31_3.clone() * cube_252_output_limb_7_col111.clone()))
                + combination_limb_7_col132.clone())
                - cube_252_output_limb_7_col143.clone())
                + poseidon_round_keys_output_limb_27_col69.clone())
                - (p_coef_col156.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_44f04_78 = eval.add_intermediate(
            ((((((((carry_7_tmp_44f04_77.clone() - combination_limb_8_col154.clone())
                + (M31_4.clone() * cube_252_output_limb_8_col80.clone()))
                + (M31_2.clone() * combination_limb_8_col101.clone()))
                + (M31_3.clone() * cube_252_output_limb_8_col112.clone()))
                + combination_limb_8_col133.clone())
                - cube_252_output_limb_8_col144.clone())
                + poseidon_round_keys_output_limb_28_col70.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((((carry_8_tmp_44f04_78.clone() - combination_limb_9_col155.clone())
                + (M31_4.clone() * cube_252_output_limb_9_col81.clone()))
                + (M31_2.clone() * combination_limb_9_col102.clone()))
                + (M31_3.clone() * cube_252_output_limb_9_col113.clone()))
                + combination_limb_9_col134.clone())
                - cube_252_output_limb_9_col145.clone())
                + poseidon_round_keys_output_limb_29_col71.clone())
                - (p_coef_col156.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col156.clone() + M31_2.clone()),
                (carry_0_tmp_44f04_70.clone() + M31_2.clone()),
                (carry_1_tmp_44f04_71.clone() + M31_2.clone()),
                (carry_2_tmp_44f04_72.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_4_4_lookup_elements,
            E::EF::one(),
            &[
                (carry_3_tmp_44f04_73.clone() + M31_2.clone()),
                (carry_4_tmp_44f04_74.clone() + M31_2.clone()),
                (carry_5_tmp_44f04_75.clone() + M31_2.clone()),
                (carry_6_tmp_44f04_76.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_4_4_lookup_elements,
            E::EF::one(),
            &[
                (carry_7_tmp_44f04_77.clone() + M31_2.clone()),
                (carry_8_tmp_44f04_78.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_felt_252_width_27_lookup_elements,
            E::EF::one(),
            &[
                combination_limb_0_col146.clone(),
                combination_limb_1_col147.clone(),
                combination_limb_2_col148.clone(),
                combination_limb_3_col149.clone(),
                combination_limb_4_col150.clone(),
                combination_limb_5_col151.clone(),
                combination_limb_6_col152.clone(),
                combination_limb_7_col153.clone(),
                combination_limb_8_col154.clone(),
                combination_limb_9_col155.clone(),
            ],
        ));

        // Linear Combination N 1 Coefs 2.

        let carry_0_tmp_44f04_81 = eval.add_intermediate(
            ((((M31_0.clone() - combination_limb_0_col157.clone())
                + (M31_2.clone() * combination_limb_0_col146.clone()))
                - p_coef_col167.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_44f04_82 = eval.add_intermediate(
            (((carry_0_tmp_44f04_81.clone() - combination_limb_1_col158.clone())
                + (M31_2.clone() * combination_limb_1_col147.clone()))
                * M31_16.clone()),
        );
        let carry_2_tmp_44f04_83 = eval.add_intermediate(
            (((carry_1_tmp_44f04_82.clone() - combination_limb_2_col159.clone())
                + (M31_2.clone() * combination_limb_2_col148.clone()))
                * M31_16.clone()),
        );
        let carry_3_tmp_44f04_84 = eval.add_intermediate(
            (((carry_2_tmp_44f04_83.clone() - combination_limb_3_col160.clone())
                + (M31_2.clone() * combination_limb_3_col149.clone()))
                * M31_16.clone()),
        );
        let carry_4_tmp_44f04_85 = eval.add_intermediate(
            (((carry_3_tmp_44f04_84.clone() - combination_limb_4_col161.clone())
                + (M31_2.clone() * combination_limb_4_col150.clone()))
                * M31_16.clone()),
        );
        let carry_5_tmp_44f04_86 = eval.add_intermediate(
            (((carry_4_tmp_44f04_85.clone() - combination_limb_5_col162.clone())
                + (M31_2.clone() * combination_limb_5_col151.clone()))
                * M31_16.clone()),
        );
        let carry_6_tmp_44f04_87 = eval.add_intermediate(
            (((carry_5_tmp_44f04_86.clone() - combination_limb_6_col163.clone())
                + (M31_2.clone() * combination_limb_6_col152.clone()))
                * M31_16.clone()),
        );
        let carry_7_tmp_44f04_88 = eval.add_intermediate(
            ((((carry_6_tmp_44f04_87.clone() - combination_limb_7_col164.clone())
                + (M31_2.clone() * combination_limb_7_col153.clone()))
                - (p_coef_col167.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_44f04_89 = eval.add_intermediate(
            (((carry_7_tmp_44f04_88.clone() - combination_limb_8_col165.clone())
                + (M31_2.clone() * combination_limb_8_col154.clone()))
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            (((carry_8_tmp_44f04_89.clone() - combination_limb_9_col166.clone())
                + (M31_2.clone() * combination_limb_9_col155.clone()))
                - (p_coef_col167.clone() * M31_256.clone())),
        );
        let biased_carry_0_tmp_44f04_90 =
            eval.add_intermediate(((p_coef_col167.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 0.
        eval.add_constraint(
            (((biased_carry_0_tmp_44f04_90.clone() * biased_carry_0_tmp_44f04_90.clone())
                * biased_carry_0_tmp_44f04_90.clone())
                - biased_carry_0_tmp_44f04_90.clone()),
        );
        let biased_carry_1_tmp_44f04_91 =
            eval.add_intermediate(((carry_0_tmp_44f04_81.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 1.
        eval.add_constraint(
            (((biased_carry_1_tmp_44f04_91.clone() * biased_carry_1_tmp_44f04_91.clone())
                * biased_carry_1_tmp_44f04_91.clone())
                - biased_carry_1_tmp_44f04_91.clone()),
        );
        let biased_carry_2_tmp_44f04_92 =
            eval.add_intermediate(((carry_1_tmp_44f04_82.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 2.
        eval.add_constraint(
            (((biased_carry_2_tmp_44f04_92.clone() * biased_carry_2_tmp_44f04_92.clone())
                * biased_carry_2_tmp_44f04_92.clone())
                - biased_carry_2_tmp_44f04_92.clone()),
        );
        let biased_carry_3_tmp_44f04_93 =
            eval.add_intermediate(((carry_2_tmp_44f04_83.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 3.
        eval.add_constraint(
            (((biased_carry_3_tmp_44f04_93.clone() * biased_carry_3_tmp_44f04_93.clone())
                * biased_carry_3_tmp_44f04_93.clone())
                - biased_carry_3_tmp_44f04_93.clone()),
        );
        let biased_carry_4_tmp_44f04_94 =
            eval.add_intermediate(((carry_3_tmp_44f04_84.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 4.
        eval.add_constraint(
            (((biased_carry_4_tmp_44f04_94.clone() * biased_carry_4_tmp_44f04_94.clone())
                * biased_carry_4_tmp_44f04_94.clone())
                - biased_carry_4_tmp_44f04_94.clone()),
        );
        let biased_carry_5_tmp_44f04_95 =
            eval.add_intermediate(((carry_4_tmp_44f04_85.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 5.
        eval.add_constraint(
            (((biased_carry_5_tmp_44f04_95.clone() * biased_carry_5_tmp_44f04_95.clone())
                * biased_carry_5_tmp_44f04_95.clone())
                - biased_carry_5_tmp_44f04_95.clone()),
        );
        let biased_carry_6_tmp_44f04_96 =
            eval.add_intermediate(((carry_5_tmp_44f04_86.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 6.
        eval.add_constraint(
            (((biased_carry_6_tmp_44f04_96.clone() * biased_carry_6_tmp_44f04_96.clone())
                * biased_carry_6_tmp_44f04_96.clone())
                - biased_carry_6_tmp_44f04_96.clone()),
        );
        let biased_carry_7_tmp_44f04_97 =
            eval.add_intermediate(((carry_6_tmp_44f04_87.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 7.
        eval.add_constraint(
            (((biased_carry_7_tmp_44f04_97.clone() * biased_carry_7_tmp_44f04_97.clone())
                * biased_carry_7_tmp_44f04_97.clone())
                - biased_carry_7_tmp_44f04_97.clone()),
        );
        let biased_carry_8_tmp_44f04_98 =
            eval.add_intermediate(((carry_7_tmp_44f04_88.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 8.
        eval.add_constraint(
            (((biased_carry_8_tmp_44f04_98.clone() * biased_carry_8_tmp_44f04_98.clone())
                * biased_carry_8_tmp_44f04_98.clone())
                - biased_carry_8_tmp_44f04_98.clone()),
        );
        let biased_carry_9_tmp_44f04_99 =
            eval.add_intermediate(((carry_8_tmp_44f04_89.clone() + M31_1.clone()) - M31_1.clone()));
        // carry constraint 9.
        eval.add_constraint(
            (((biased_carry_9_tmp_44f04_99.clone() * biased_carry_9_tmp_44f04_99.clone())
                * biased_carry_9_tmp_44f04_99.clone())
                - biased_carry_9_tmp_44f04_99.clone()),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_3_partial_rounds_chain_lookup_elements,
            E::EF::from(padding.clone()),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
                input_limb_6_col6.clone(),
                input_limb_7_col7.clone(),
                input_limb_8_col8.clone(),
                input_limb_9_col9.clone(),
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                input_limb_12_col12.clone(),
                input_limb_13_col13.clone(),
                input_limb_14_col14.clone(),
                input_limb_15_col15.clone(),
                input_limb_16_col16.clone(),
                input_limb_17_col17.clone(),
                input_limb_18_col18.clone(),
                input_limb_19_col19.clone(),
                input_limb_20_col20.clone(),
                input_limb_21_col21.clone(),
                input_limb_22_col22.clone(),
                input_limb_23_col23.clone(),
                input_limb_24_col24.clone(),
                input_limb_25_col25.clone(),
                input_limb_26_col26.clone(),
                input_limb_27_col27.clone(),
                input_limb_28_col28.clone(),
                input_limb_29_col29.clone(),
                input_limb_30_col30.clone(),
                input_limb_31_col31.clone(),
                input_limb_32_col32.clone(),
                input_limb_33_col33.clone(),
                input_limb_34_col34.clone(),
                input_limb_35_col35.clone(),
                input_limb_36_col36.clone(),
                input_limb_37_col37.clone(),
                input_limb_38_col38.clone(),
                input_limb_39_col39.clone(),
                input_limb_40_col40.clone(),
                input_limb_41_col41.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_3_partial_rounds_chain_lookup_elements,
            -E::EF::from(padding.clone()),
            &[
                input_limb_0_col0.clone(),
                (input_limb_1_col1.clone() + M31_1.clone()),
                cube_252_output_limb_0_col104.clone(),
                cube_252_output_limb_1_col105.clone(),
                cube_252_output_limb_2_col106.clone(),
                cube_252_output_limb_3_col107.clone(),
                cube_252_output_limb_4_col108.clone(),
                cube_252_output_limb_5_col109.clone(),
                cube_252_output_limb_6_col110.clone(),
                cube_252_output_limb_7_col111.clone(),
                cube_252_output_limb_8_col112.clone(),
                cube_252_output_limb_9_col113.clone(),
                combination_limb_0_col125.clone(),
                combination_limb_1_col126.clone(),
                combination_limb_2_col127.clone(),
                combination_limb_3_col128.clone(),
                combination_limb_4_col129.clone(),
                combination_limb_5_col130.clone(),
                combination_limb_6_col131.clone(),
                combination_limb_7_col132.clone(),
                combination_limb_8_col133.clone(),
                combination_limb_9_col134.clone(),
                cube_252_output_limb_0_col136.clone(),
                cube_252_output_limb_1_col137.clone(),
                cube_252_output_limb_2_col138.clone(),
                cube_252_output_limb_3_col139.clone(),
                cube_252_output_limb_4_col140.clone(),
                cube_252_output_limb_5_col141.clone(),
                cube_252_output_limb_6_col142.clone(),
                cube_252_output_limb_7_col143.clone(),
                cube_252_output_limb_8_col144.clone(),
                cube_252_output_limb_9_col145.clone(),
                combination_limb_0_col157.clone(),
                combination_limb_1_col158.clone(),
                combination_limb_2_col159.clone(),
                combination_limb_3_col160.clone(),
                combination_limb_4_col161.clone(),
                combination_limb_5_col162.clone(),
                combination_limb_6_col163.clone(),
                combination_limb_7_col164.clone(),
                combination_limb_8_col165.clone(),
                combination_limb_9_col166.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

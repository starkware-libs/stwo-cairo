use crate::prelude::constraint_eval::*;
pub const N_TRACE_COLUMNS: usize = 126;

pub struct Eval {
    pub claim: Claim,
    pub cube_252_lookup_elements: relations::Cube252,
    pub poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain,
    pub poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys,
    pub range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 6];
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
        let cube_252_output_limb_0_col32 = eval.next_trace_mask();
        let cube_252_output_limb_1_col33 = eval.next_trace_mask();
        let cube_252_output_limb_2_col34 = eval.next_trace_mask();
        let cube_252_output_limb_3_col35 = eval.next_trace_mask();
        let cube_252_output_limb_4_col36 = eval.next_trace_mask();
        let cube_252_output_limb_5_col37 = eval.next_trace_mask();
        let cube_252_output_limb_6_col38 = eval.next_trace_mask();
        let cube_252_output_limb_7_col39 = eval.next_trace_mask();
        let cube_252_output_limb_8_col40 = eval.next_trace_mask();
        let cube_252_output_limb_9_col41 = eval.next_trace_mask();
        let cube_252_output_limb_0_col42 = eval.next_trace_mask();
        let cube_252_output_limb_1_col43 = eval.next_trace_mask();
        let cube_252_output_limb_2_col44 = eval.next_trace_mask();
        let cube_252_output_limb_3_col45 = eval.next_trace_mask();
        let cube_252_output_limb_4_col46 = eval.next_trace_mask();
        let cube_252_output_limb_5_col47 = eval.next_trace_mask();
        let cube_252_output_limb_6_col48 = eval.next_trace_mask();
        let cube_252_output_limb_7_col49 = eval.next_trace_mask();
        let cube_252_output_limb_8_col50 = eval.next_trace_mask();
        let cube_252_output_limb_9_col51 = eval.next_trace_mask();
        let cube_252_output_limb_0_col52 = eval.next_trace_mask();
        let cube_252_output_limb_1_col53 = eval.next_trace_mask();
        let cube_252_output_limb_2_col54 = eval.next_trace_mask();
        let cube_252_output_limb_3_col55 = eval.next_trace_mask();
        let cube_252_output_limb_4_col56 = eval.next_trace_mask();
        let cube_252_output_limb_5_col57 = eval.next_trace_mask();
        let cube_252_output_limb_6_col58 = eval.next_trace_mask();
        let cube_252_output_limb_7_col59 = eval.next_trace_mask();
        let cube_252_output_limb_8_col60 = eval.next_trace_mask();
        let cube_252_output_limb_9_col61 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_0_col62 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_1_col63 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_2_col64 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_3_col65 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_4_col66 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_5_col67 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_6_col68 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_7_col69 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_8_col70 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_9_col71 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_10_col72 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_11_col73 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_12_col74 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_13_col75 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_14_col76 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_15_col77 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_16_col78 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_17_col79 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_18_col80 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_19_col81 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_20_col82 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_21_col83 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_22_col84 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_23_col85 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_24_col86 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_25_col87 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_26_col88 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_27_col89 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_28_col90 = eval.next_trace_mask();
        let poseidon_round_keys_output_limb_29_col91 = eval.next_trace_mask();
        let combination_limb_0_col92 = eval.next_trace_mask();
        let combination_limb_1_col93 = eval.next_trace_mask();
        let combination_limb_2_col94 = eval.next_trace_mask();
        let combination_limb_3_col95 = eval.next_trace_mask();
        let combination_limb_4_col96 = eval.next_trace_mask();
        let combination_limb_5_col97 = eval.next_trace_mask();
        let combination_limb_6_col98 = eval.next_trace_mask();
        let combination_limb_7_col99 = eval.next_trace_mask();
        let combination_limb_8_col100 = eval.next_trace_mask();
        let combination_limb_9_col101 = eval.next_trace_mask();
        let p_coef_col102 = eval.next_trace_mask();
        let combination_limb_0_col103 = eval.next_trace_mask();
        let combination_limb_1_col104 = eval.next_trace_mask();
        let combination_limb_2_col105 = eval.next_trace_mask();
        let combination_limb_3_col106 = eval.next_trace_mask();
        let combination_limb_4_col107 = eval.next_trace_mask();
        let combination_limb_5_col108 = eval.next_trace_mask();
        let combination_limb_6_col109 = eval.next_trace_mask();
        let combination_limb_7_col110 = eval.next_trace_mask();
        let combination_limb_8_col111 = eval.next_trace_mask();
        let combination_limb_9_col112 = eval.next_trace_mask();
        let p_coef_col113 = eval.next_trace_mask();
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
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            E::EF::one(),
            &[
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
                cube_252_output_limb_0_col32.clone(),
                cube_252_output_limb_1_col33.clone(),
                cube_252_output_limb_2_col34.clone(),
                cube_252_output_limb_3_col35.clone(),
                cube_252_output_limb_4_col36.clone(),
                cube_252_output_limb_5_col37.clone(),
                cube_252_output_limb_6_col38.clone(),
                cube_252_output_limb_7_col39.clone(),
                cube_252_output_limb_8_col40.clone(),
                cube_252_output_limb_9_col41.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            E::EF::one(),
            &[
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
                cube_252_output_limb_0_col42.clone(),
                cube_252_output_limb_1_col43.clone(),
                cube_252_output_limb_2_col44.clone(),
                cube_252_output_limb_3_col45.clone(),
                cube_252_output_limb_4_col46.clone(),
                cube_252_output_limb_5_col47.clone(),
                cube_252_output_limb_6_col48.clone(),
                cube_252_output_limb_7_col49.clone(),
                cube_252_output_limb_8_col50.clone(),
                cube_252_output_limb_9_col51.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            E::EF::one(),
            &[
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
                cube_252_output_limb_0_col52.clone(),
                cube_252_output_limb_1_col53.clone(),
                cube_252_output_limb_2_col54.clone(),
                cube_252_output_limb_3_col55.clone(),
                cube_252_output_limb_4_col56.clone(),
                cube_252_output_limb_5_col57.clone(),
                cube_252_output_limb_6_col58.clone(),
                cube_252_output_limb_7_col59.clone(),
                cube_252_output_limb_8_col60.clone(),
                cube_252_output_limb_9_col61.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_round_keys_lookup_elements,
            E::EF::one(),
            &[
                input_limb_1_col1.clone(),
                poseidon_round_keys_output_limb_0_col62.clone(),
                poseidon_round_keys_output_limb_1_col63.clone(),
                poseidon_round_keys_output_limb_2_col64.clone(),
                poseidon_round_keys_output_limb_3_col65.clone(),
                poseidon_round_keys_output_limb_4_col66.clone(),
                poseidon_round_keys_output_limb_5_col67.clone(),
                poseidon_round_keys_output_limb_6_col68.clone(),
                poseidon_round_keys_output_limb_7_col69.clone(),
                poseidon_round_keys_output_limb_8_col70.clone(),
                poseidon_round_keys_output_limb_9_col71.clone(),
                poseidon_round_keys_output_limb_10_col72.clone(),
                poseidon_round_keys_output_limb_11_col73.clone(),
                poseidon_round_keys_output_limb_12_col74.clone(),
                poseidon_round_keys_output_limb_13_col75.clone(),
                poseidon_round_keys_output_limb_14_col76.clone(),
                poseidon_round_keys_output_limb_15_col77.clone(),
                poseidon_round_keys_output_limb_16_col78.clone(),
                poseidon_round_keys_output_limb_17_col79.clone(),
                poseidon_round_keys_output_limb_18_col80.clone(),
                poseidon_round_keys_output_limb_19_col81.clone(),
                poseidon_round_keys_output_limb_20_col82.clone(),
                poseidon_round_keys_output_limb_21_col83.clone(),
                poseidon_round_keys_output_limb_22_col84.clone(),
                poseidon_round_keys_output_limb_23_col85.clone(),
                poseidon_round_keys_output_limb_24_col86.clone(),
                poseidon_round_keys_output_limb_25_col87.clone(),
                poseidon_round_keys_output_limb_26_col88.clone(),
                poseidon_round_keys_output_limb_27_col89.clone(),
                poseidon_round_keys_output_limb_28_col90.clone(),
                poseidon_round_keys_output_limb_29_col91.clone(),
            ],
        ));

        // Linear Combination N 4 Coefs 3 1 1 1.

        let carry_0_tmp_f9fbc_6 = eval.add_intermediate(
            (((((((M31_0.clone() - combination_limb_0_col92.clone())
                + (M31_3.clone() * cube_252_output_limb_0_col32.clone()))
                + cube_252_output_limb_0_col42.clone())
                + cube_252_output_limb_0_col52.clone())
                + poseidon_round_keys_output_limb_0_col62.clone())
                - p_coef_col102.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_f9fbc_7 = eval.add_intermediate(
            ((((((carry_0_tmp_f9fbc_6.clone() - combination_limb_1_col93.clone())
                + (M31_3.clone() * cube_252_output_limb_1_col33.clone()))
                + cube_252_output_limb_1_col43.clone())
                + cube_252_output_limb_1_col53.clone())
                + poseidon_round_keys_output_limb_1_col63.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_f9fbc_8 = eval.add_intermediate(
            ((((((carry_1_tmp_f9fbc_7.clone() - combination_limb_2_col94.clone())
                + (M31_3.clone() * cube_252_output_limb_2_col34.clone()))
                + cube_252_output_limb_2_col44.clone())
                + cube_252_output_limb_2_col54.clone())
                + poseidon_round_keys_output_limb_2_col64.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_f9fbc_9 = eval.add_intermediate(
            ((((((carry_2_tmp_f9fbc_8.clone() - combination_limb_3_col95.clone())
                + (M31_3.clone() * cube_252_output_limb_3_col35.clone()))
                + cube_252_output_limb_3_col45.clone())
                + cube_252_output_limb_3_col55.clone())
                + poseidon_round_keys_output_limb_3_col65.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_f9fbc_10 = eval.add_intermediate(
            ((((((carry_3_tmp_f9fbc_9.clone() - combination_limb_4_col96.clone())
                + (M31_3.clone() * cube_252_output_limb_4_col36.clone()))
                + cube_252_output_limb_4_col46.clone())
                + cube_252_output_limb_4_col56.clone())
                + poseidon_round_keys_output_limb_4_col66.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_f9fbc_11 = eval.add_intermediate(
            ((((((carry_4_tmp_f9fbc_10.clone() - combination_limb_5_col97.clone())
                + (M31_3.clone() * cube_252_output_limb_5_col37.clone()))
                + cube_252_output_limb_5_col47.clone())
                + cube_252_output_limb_5_col57.clone())
                + poseidon_round_keys_output_limb_5_col67.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_f9fbc_12 = eval.add_intermediate(
            ((((((carry_5_tmp_f9fbc_11.clone() - combination_limb_6_col98.clone())
                + (M31_3.clone() * cube_252_output_limb_6_col38.clone()))
                + cube_252_output_limb_6_col48.clone())
                + cube_252_output_limb_6_col58.clone())
                + poseidon_round_keys_output_limb_6_col68.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_f9fbc_13 = eval.add_intermediate(
            (((((((carry_6_tmp_f9fbc_12.clone() - combination_limb_7_col99.clone())
                + (M31_3.clone() * cube_252_output_limb_7_col39.clone()))
                + cube_252_output_limb_7_col49.clone())
                + cube_252_output_limb_7_col59.clone())
                + poseidon_round_keys_output_limb_7_col69.clone())
                - (p_coef_col102.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_f9fbc_14 = eval.add_intermediate(
            ((((((carry_7_tmp_f9fbc_13.clone() - combination_limb_8_col100.clone())
                + (M31_3.clone() * cube_252_output_limb_8_col40.clone()))
                + cube_252_output_limb_8_col50.clone())
                + cube_252_output_limb_8_col60.clone())
                + poseidon_round_keys_output_limb_8_col70.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((carry_8_tmp_f9fbc_14.clone() - combination_limb_9_col101.clone())
                + (M31_3.clone() * cube_252_output_limb_9_col41.clone()))
                + cube_252_output_limb_9_col51.clone())
                + cube_252_output_limb_9_col61.clone())
                + poseidon_round_keys_output_limb_9_col71.clone())
                - (p_coef_col102.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col102.clone() + M31_1.clone()),
                (carry_0_tmp_f9fbc_6.clone() + M31_1.clone()),
                (carry_1_tmp_f9fbc_7.clone() + M31_1.clone()),
                (carry_2_tmp_f9fbc_8.clone() + M31_1.clone()),
                (carry_3_tmp_f9fbc_9.clone() + M31_1.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (carry_4_tmp_f9fbc_10.clone() + M31_1.clone()),
                (carry_5_tmp_f9fbc_11.clone() + M31_1.clone()),
                (carry_6_tmp_f9fbc_12.clone() + M31_1.clone()),
                (carry_7_tmp_f9fbc_13.clone() + M31_1.clone()),
                (carry_8_tmp_f9fbc_14.clone() + M31_1.clone()),
            ],
        ));

        // Linear Combination N 4 Coefs 1 M 1 1 1.

        let carry_0_tmp_f9fbc_17 = eval.add_intermediate(
            (((((((M31_0.clone() - combination_limb_0_col103.clone())
                + cube_252_output_limb_0_col32.clone())
                - cube_252_output_limb_0_col42.clone())
                + cube_252_output_limb_0_col52.clone())
                + poseidon_round_keys_output_limb_10_col72.clone())
                - p_coef_col113.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_f9fbc_18 = eval.add_intermediate(
            ((((((carry_0_tmp_f9fbc_17.clone() - combination_limb_1_col104.clone())
                + cube_252_output_limb_1_col33.clone())
                - cube_252_output_limb_1_col43.clone())
                + cube_252_output_limb_1_col53.clone())
                + poseidon_round_keys_output_limb_11_col73.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_f9fbc_19 = eval.add_intermediate(
            ((((((carry_1_tmp_f9fbc_18.clone() - combination_limb_2_col105.clone())
                + cube_252_output_limb_2_col34.clone())
                - cube_252_output_limb_2_col44.clone())
                + cube_252_output_limb_2_col54.clone())
                + poseidon_round_keys_output_limb_12_col74.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_f9fbc_20 = eval.add_intermediate(
            ((((((carry_2_tmp_f9fbc_19.clone() - combination_limb_3_col106.clone())
                + cube_252_output_limb_3_col35.clone())
                - cube_252_output_limb_3_col45.clone())
                + cube_252_output_limb_3_col55.clone())
                + poseidon_round_keys_output_limb_13_col75.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_f9fbc_21 = eval.add_intermediate(
            ((((((carry_3_tmp_f9fbc_20.clone() - combination_limb_4_col107.clone())
                + cube_252_output_limb_4_col36.clone())
                - cube_252_output_limb_4_col46.clone())
                + cube_252_output_limb_4_col56.clone())
                + poseidon_round_keys_output_limb_14_col76.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_f9fbc_22 = eval.add_intermediate(
            ((((((carry_4_tmp_f9fbc_21.clone() - combination_limb_5_col108.clone())
                + cube_252_output_limb_5_col37.clone())
                - cube_252_output_limb_5_col47.clone())
                + cube_252_output_limb_5_col57.clone())
                + poseidon_round_keys_output_limb_15_col77.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_f9fbc_23 = eval.add_intermediate(
            ((((((carry_5_tmp_f9fbc_22.clone() - combination_limb_6_col109.clone())
                + cube_252_output_limb_6_col38.clone())
                - cube_252_output_limb_6_col48.clone())
                + cube_252_output_limb_6_col58.clone())
                + poseidon_round_keys_output_limb_16_col78.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_f9fbc_24 = eval.add_intermediate(
            (((((((carry_6_tmp_f9fbc_23.clone() - combination_limb_7_col110.clone())
                + cube_252_output_limb_7_col39.clone())
                - cube_252_output_limb_7_col49.clone())
                + cube_252_output_limb_7_col59.clone())
                + poseidon_round_keys_output_limb_17_col79.clone())
                - (p_coef_col113.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_f9fbc_25 = eval.add_intermediate(
            ((((((carry_7_tmp_f9fbc_24.clone() - combination_limb_8_col111.clone())
                + cube_252_output_limb_8_col40.clone())
                - cube_252_output_limb_8_col50.clone())
                + cube_252_output_limb_8_col60.clone())
                + poseidon_round_keys_output_limb_18_col80.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((carry_8_tmp_f9fbc_25.clone() - combination_limb_9_col112.clone())
                + cube_252_output_limb_9_col41.clone())
                - cube_252_output_limb_9_col51.clone())
                + cube_252_output_limb_9_col61.clone())
                + poseidon_round_keys_output_limb_19_col81.clone())
                - (p_coef_col113.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col113.clone() + M31_2.clone()),
                (carry_0_tmp_f9fbc_17.clone() + M31_2.clone()),
                (carry_1_tmp_f9fbc_18.clone() + M31_2.clone()),
                (carry_2_tmp_f9fbc_19.clone() + M31_2.clone()),
                (carry_3_tmp_f9fbc_20.clone() + M31_2.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (carry_4_tmp_f9fbc_21.clone() + M31_2.clone()),
                (carry_5_tmp_f9fbc_22.clone() + M31_2.clone()),
                (carry_6_tmp_f9fbc_23.clone() + M31_2.clone()),
                (carry_7_tmp_f9fbc_24.clone() + M31_2.clone()),
                (carry_8_tmp_f9fbc_25.clone() + M31_2.clone()),
            ],
        ));

        // Linear Combination N 4 Coefs 1 1 M 2 1.

        let carry_0_tmp_f9fbc_28 = eval.add_intermediate(
            (((((((M31_0.clone() - combination_limb_0_col114.clone())
                + cube_252_output_limb_0_col32.clone())
                + cube_252_output_limb_0_col42.clone())
                - (M31_2.clone() * cube_252_output_limb_0_col52.clone()))
                + poseidon_round_keys_output_limb_20_col82.clone())
                - p_coef_col124.clone())
                * M31_16.clone()),
        );
        let carry_1_tmp_f9fbc_29 = eval.add_intermediate(
            ((((((carry_0_tmp_f9fbc_28.clone() - combination_limb_1_col115.clone())
                + cube_252_output_limb_1_col33.clone())
                + cube_252_output_limb_1_col43.clone())
                - (M31_2.clone() * cube_252_output_limb_1_col53.clone()))
                + poseidon_round_keys_output_limb_21_col83.clone())
                * M31_16.clone()),
        );
        let carry_2_tmp_f9fbc_30 = eval.add_intermediate(
            ((((((carry_1_tmp_f9fbc_29.clone() - combination_limb_2_col116.clone())
                + cube_252_output_limb_2_col34.clone())
                + cube_252_output_limb_2_col44.clone())
                - (M31_2.clone() * cube_252_output_limb_2_col54.clone()))
                + poseidon_round_keys_output_limb_22_col84.clone())
                * M31_16.clone()),
        );
        let carry_3_tmp_f9fbc_31 = eval.add_intermediate(
            ((((((carry_2_tmp_f9fbc_30.clone() - combination_limb_3_col117.clone())
                + cube_252_output_limb_3_col35.clone())
                + cube_252_output_limb_3_col45.clone())
                - (M31_2.clone() * cube_252_output_limb_3_col55.clone()))
                + poseidon_round_keys_output_limb_23_col85.clone())
                * M31_16.clone()),
        );
        let carry_4_tmp_f9fbc_32 = eval.add_intermediate(
            ((((((carry_3_tmp_f9fbc_31.clone() - combination_limb_4_col118.clone())
                + cube_252_output_limb_4_col36.clone())
                + cube_252_output_limb_4_col46.clone())
                - (M31_2.clone() * cube_252_output_limb_4_col56.clone()))
                + poseidon_round_keys_output_limb_24_col86.clone())
                * M31_16.clone()),
        );
        let carry_5_tmp_f9fbc_33 = eval.add_intermediate(
            ((((((carry_4_tmp_f9fbc_32.clone() - combination_limb_5_col119.clone())
                + cube_252_output_limb_5_col37.clone())
                + cube_252_output_limb_5_col47.clone())
                - (M31_2.clone() * cube_252_output_limb_5_col57.clone()))
                + poseidon_round_keys_output_limb_25_col87.clone())
                * M31_16.clone()),
        );
        let carry_6_tmp_f9fbc_34 = eval.add_intermediate(
            ((((((carry_5_tmp_f9fbc_33.clone() - combination_limb_6_col120.clone())
                + cube_252_output_limb_6_col38.clone())
                + cube_252_output_limb_6_col48.clone())
                - (M31_2.clone() * cube_252_output_limb_6_col58.clone()))
                + poseidon_round_keys_output_limb_26_col88.clone())
                * M31_16.clone()),
        );
        let carry_7_tmp_f9fbc_35 = eval.add_intermediate(
            (((((((carry_6_tmp_f9fbc_34.clone() - combination_limb_7_col121.clone())
                + cube_252_output_limb_7_col39.clone())
                + cube_252_output_limb_7_col49.clone())
                - (M31_2.clone() * cube_252_output_limb_7_col59.clone()))
                + poseidon_round_keys_output_limb_27_col89.clone())
                - (p_coef_col124.clone() * M31_136.clone()))
                * M31_16.clone()),
        );
        let carry_8_tmp_f9fbc_36 = eval.add_intermediate(
            ((((((carry_7_tmp_f9fbc_35.clone() - combination_limb_8_col122.clone())
                + cube_252_output_limb_8_col40.clone())
                + cube_252_output_limb_8_col50.clone())
                - (M31_2.clone() * cube_252_output_limb_8_col60.clone()))
                + poseidon_round_keys_output_limb_28_col90.clone())
                * M31_16.clone()),
        );
        // final limb constraint.
        eval.add_constraint(
            ((((((carry_8_tmp_f9fbc_36.clone() - combination_limb_9_col123.clone())
                + cube_252_output_limb_9_col41.clone())
                + cube_252_output_limb_9_col51.clone())
                - (M31_2.clone() * cube_252_output_limb_9_col61.clone()))
                + poseidon_round_keys_output_limb_29_col91.clone())
                - (p_coef_col124.clone() * M31_256.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (p_coef_col124.clone() + M31_3.clone()),
                (carry_0_tmp_f9fbc_28.clone() + M31_3.clone()),
                (carry_1_tmp_f9fbc_29.clone() + M31_3.clone()),
                (carry_2_tmp_f9fbc_30.clone() + M31_3.clone()),
                (carry_3_tmp_f9fbc_31.clone() + M31_3.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_3_3_3_3_3_lookup_elements,
            E::EF::one(),
            &[
                (carry_4_tmp_f9fbc_32.clone() + M31_3.clone()),
                (carry_5_tmp_f9fbc_33.clone() + M31_3.clone()),
                (carry_6_tmp_f9fbc_34.clone() + M31_3.clone()),
                (carry_7_tmp_f9fbc_35.clone() + M31_3.clone()),
                (carry_8_tmp_f9fbc_36.clone() + M31_3.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_full_round_chain_lookup_elements,
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
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_full_round_chain_lookup_elements,
            -E::EF::from(padding.clone()),
            &[
                input_limb_0_col0.clone(),
                (input_limb_1_col1.clone() + M31_1.clone()),
                combination_limb_0_col92.clone(),
                combination_limb_1_col93.clone(),
                combination_limb_2_col94.clone(),
                combination_limb_3_col95.clone(),
                combination_limb_4_col96.clone(),
                combination_limb_5_col97.clone(),
                combination_limb_6_col98.clone(),
                combination_limb_7_col99.clone(),
                combination_limb_8_col100.clone(),
                combination_limb_9_col101.clone(),
                combination_limb_0_col103.clone(),
                combination_limb_1_col104.clone(),
                combination_limb_2_col105.clone(),
                combination_limb_3_col106.clone(),
                combination_limb_4_col107.clone(),
                combination_limb_5_col108.clone(),
                combination_limb_6_col109.clone(),
                combination_limb_7_col110.clone(),
                combination_limb_8_col111.clone(),
                combination_limb_9_col112.clone(),
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

        eval.finalize_logup_in_pairs();
        eval
    }
}

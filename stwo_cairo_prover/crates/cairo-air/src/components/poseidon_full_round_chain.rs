// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::linear_combination_n_4_coefs_1_1_m2_1::LinearCombinationN4Coefs11M21;
use crate::components::subroutines::linear_combination_n_4_coefs_1_m1_1_1::LinearCombinationN4Coefs1M111;
use crate::components::subroutines::linear_combination_n_4_coefs_3_1_1_1::LinearCombinationN4Coefs3111;

pub const N_TRACE_COLUMNS: usize = 126;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "Cube252",
        uses: 3,
    },
    RelationUse {
        relation_id: "PoseidonFullRoundChain",
        uses: 1,
    },
    RelationUse {
        relation_id: "PoseidonRoundKeys",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_3_3_3_3_3",
        uses: 6,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub cube_252_lookup_elements: relations::Cube252,
    pub poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys,
    pub range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3,
    pub poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        let M31_1 = E::F::from(M31::from(1));
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
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

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

        LinearCombinationN4Coefs3111::evaluate(
            [
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
            ],
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
            p_coef_col102.clone(),
            &self.range_check_3_3_3_3_3_lookup_elements,
            &mut eval,
        );
        LinearCombinationN4Coefs1M111::evaluate(
            [
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
            ],
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
            p_coef_col113.clone(),
            &self.range_check_3_3_3_3_3_lookup_elements,
            &mut eval,
        );
        LinearCombinationN4Coefs11M21::evaluate(
            [
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
            p_coef_col124.clone(),
            &self.range_check_3_3_3_3_3_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_full_round_chain_lookup_elements,
            E::EF::from(enabler.clone()),
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
            -E::EF::from(enabler.clone()),
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

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

    use super::*;
    use crate::components::constraints_regression_test_values::POSEIDON_FULL_ROUND_CHAIN;

    #[test]
    fn poseidon_full_round_chain_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            cube_252_lookup_elements: relations::Cube252::dummy(),
            poseidon_round_keys_lookup_elements: relations::PoseidonRoundKeys::dummy(),
            range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3::dummy(),
            poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, POSEIDON_FULL_ROUND_CHAIN);
    }
}

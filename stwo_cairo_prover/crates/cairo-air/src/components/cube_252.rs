// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::felt_252_unpack_from_27_range_check_output::Felt252UnpackFrom27RangeCheckOutput;
use crate::components::subroutines::mul_252::Mul252;

pub const N_TRACE_COLUMNS: usize = 141;
pub const RELATION_USES_PER_ROW: [RelationUse; 16] = [
    RelationUse {
        relation_id: "RangeCheck_20",
        uses: 8,
    },
    RelationUse {
        relation_id: "RangeCheck_20_B",
        uses: 8,
    },
    RelationUse {
        relation_id: "RangeCheck_20_C",
        uses: 8,
    },
    RelationUse {
        relation_id: "RangeCheck_20_D",
        uses: 8,
    },
    RelationUse {
        relation_id: "RangeCheck_20_E",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_20_F",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_20_G",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_20_H",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 6,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 3,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
    pub range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
    pub range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
    pub range_check_20_lookup_elements: relations::RangeCheck_20,
    pub range_check_20_b_lookup_elements: relations::RangeCheck_20_B,
    pub range_check_20_c_lookup_elements: relations::RangeCheck_20_C,
    pub range_check_20_d_lookup_elements: relations::RangeCheck_20_D,
    pub range_check_20_e_lookup_elements: relations::RangeCheck_20_E,
    pub range_check_20_f_lookup_elements: relations::RangeCheck_20_F,
    pub range_check_20_g_lookup_elements: relations::RangeCheck_20_G,
    pub range_check_20_h_lookup_elements: relations::RangeCheck_20_H,
    pub cube_252_lookup_elements: relations::Cube252,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 50];
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
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));
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
        let unpacked_limb_0_col10 = eval.next_trace_mask();
        let unpacked_limb_1_col11 = eval.next_trace_mask();
        let unpacked_limb_3_col12 = eval.next_trace_mask();
        let unpacked_limb_4_col13 = eval.next_trace_mask();
        let unpacked_limb_6_col14 = eval.next_trace_mask();
        let unpacked_limb_7_col15 = eval.next_trace_mask();
        let unpacked_limb_9_col16 = eval.next_trace_mask();
        let unpacked_limb_10_col17 = eval.next_trace_mask();
        let unpacked_limb_12_col18 = eval.next_trace_mask();
        let unpacked_limb_13_col19 = eval.next_trace_mask();
        let unpacked_limb_15_col20 = eval.next_trace_mask();
        let unpacked_limb_16_col21 = eval.next_trace_mask();
        let unpacked_limb_18_col22 = eval.next_trace_mask();
        let unpacked_limb_19_col23 = eval.next_trace_mask();
        let unpacked_limb_21_col24 = eval.next_trace_mask();
        let unpacked_limb_22_col25 = eval.next_trace_mask();
        let unpacked_limb_24_col26 = eval.next_trace_mask();
        let unpacked_limb_25_col27 = eval.next_trace_mask();
        let mul_res_limb_0_col28 = eval.next_trace_mask();
        let mul_res_limb_1_col29 = eval.next_trace_mask();
        let mul_res_limb_2_col30 = eval.next_trace_mask();
        let mul_res_limb_3_col31 = eval.next_trace_mask();
        let mul_res_limb_4_col32 = eval.next_trace_mask();
        let mul_res_limb_5_col33 = eval.next_trace_mask();
        let mul_res_limb_6_col34 = eval.next_trace_mask();
        let mul_res_limb_7_col35 = eval.next_trace_mask();
        let mul_res_limb_8_col36 = eval.next_trace_mask();
        let mul_res_limb_9_col37 = eval.next_trace_mask();
        let mul_res_limb_10_col38 = eval.next_trace_mask();
        let mul_res_limb_11_col39 = eval.next_trace_mask();
        let mul_res_limb_12_col40 = eval.next_trace_mask();
        let mul_res_limb_13_col41 = eval.next_trace_mask();
        let mul_res_limb_14_col42 = eval.next_trace_mask();
        let mul_res_limb_15_col43 = eval.next_trace_mask();
        let mul_res_limb_16_col44 = eval.next_trace_mask();
        let mul_res_limb_17_col45 = eval.next_trace_mask();
        let mul_res_limb_18_col46 = eval.next_trace_mask();
        let mul_res_limb_19_col47 = eval.next_trace_mask();
        let mul_res_limb_20_col48 = eval.next_trace_mask();
        let mul_res_limb_21_col49 = eval.next_trace_mask();
        let mul_res_limb_22_col50 = eval.next_trace_mask();
        let mul_res_limb_23_col51 = eval.next_trace_mask();
        let mul_res_limb_24_col52 = eval.next_trace_mask();
        let mul_res_limb_25_col53 = eval.next_trace_mask();
        let mul_res_limb_26_col54 = eval.next_trace_mask();
        let mul_res_limb_27_col55 = eval.next_trace_mask();
        let k_col56 = eval.next_trace_mask();
        let carry_0_col57 = eval.next_trace_mask();
        let carry_1_col58 = eval.next_trace_mask();
        let carry_2_col59 = eval.next_trace_mask();
        let carry_3_col60 = eval.next_trace_mask();
        let carry_4_col61 = eval.next_trace_mask();
        let carry_5_col62 = eval.next_trace_mask();
        let carry_6_col63 = eval.next_trace_mask();
        let carry_7_col64 = eval.next_trace_mask();
        let carry_8_col65 = eval.next_trace_mask();
        let carry_9_col66 = eval.next_trace_mask();
        let carry_10_col67 = eval.next_trace_mask();
        let carry_11_col68 = eval.next_trace_mask();
        let carry_12_col69 = eval.next_trace_mask();
        let carry_13_col70 = eval.next_trace_mask();
        let carry_14_col71 = eval.next_trace_mask();
        let carry_15_col72 = eval.next_trace_mask();
        let carry_16_col73 = eval.next_trace_mask();
        let carry_17_col74 = eval.next_trace_mask();
        let carry_18_col75 = eval.next_trace_mask();
        let carry_19_col76 = eval.next_trace_mask();
        let carry_20_col77 = eval.next_trace_mask();
        let carry_21_col78 = eval.next_trace_mask();
        let carry_22_col79 = eval.next_trace_mask();
        let carry_23_col80 = eval.next_trace_mask();
        let carry_24_col81 = eval.next_trace_mask();
        let carry_25_col82 = eval.next_trace_mask();
        let carry_26_col83 = eval.next_trace_mask();
        let mul_res_limb_0_col84 = eval.next_trace_mask();
        let mul_res_limb_1_col85 = eval.next_trace_mask();
        let mul_res_limb_2_col86 = eval.next_trace_mask();
        let mul_res_limb_3_col87 = eval.next_trace_mask();
        let mul_res_limb_4_col88 = eval.next_trace_mask();
        let mul_res_limb_5_col89 = eval.next_trace_mask();
        let mul_res_limb_6_col90 = eval.next_trace_mask();
        let mul_res_limb_7_col91 = eval.next_trace_mask();
        let mul_res_limb_8_col92 = eval.next_trace_mask();
        let mul_res_limb_9_col93 = eval.next_trace_mask();
        let mul_res_limb_10_col94 = eval.next_trace_mask();
        let mul_res_limb_11_col95 = eval.next_trace_mask();
        let mul_res_limb_12_col96 = eval.next_trace_mask();
        let mul_res_limb_13_col97 = eval.next_trace_mask();
        let mul_res_limb_14_col98 = eval.next_trace_mask();
        let mul_res_limb_15_col99 = eval.next_trace_mask();
        let mul_res_limb_16_col100 = eval.next_trace_mask();
        let mul_res_limb_17_col101 = eval.next_trace_mask();
        let mul_res_limb_18_col102 = eval.next_trace_mask();
        let mul_res_limb_19_col103 = eval.next_trace_mask();
        let mul_res_limb_20_col104 = eval.next_trace_mask();
        let mul_res_limb_21_col105 = eval.next_trace_mask();
        let mul_res_limb_22_col106 = eval.next_trace_mask();
        let mul_res_limb_23_col107 = eval.next_trace_mask();
        let mul_res_limb_24_col108 = eval.next_trace_mask();
        let mul_res_limb_25_col109 = eval.next_trace_mask();
        let mul_res_limb_26_col110 = eval.next_trace_mask();
        let mul_res_limb_27_col111 = eval.next_trace_mask();
        let k_col112 = eval.next_trace_mask();
        let carry_0_col113 = eval.next_trace_mask();
        let carry_1_col114 = eval.next_trace_mask();
        let carry_2_col115 = eval.next_trace_mask();
        let carry_3_col116 = eval.next_trace_mask();
        let carry_4_col117 = eval.next_trace_mask();
        let carry_5_col118 = eval.next_trace_mask();
        let carry_6_col119 = eval.next_trace_mask();
        let carry_7_col120 = eval.next_trace_mask();
        let carry_8_col121 = eval.next_trace_mask();
        let carry_9_col122 = eval.next_trace_mask();
        let carry_10_col123 = eval.next_trace_mask();
        let carry_11_col124 = eval.next_trace_mask();
        let carry_12_col125 = eval.next_trace_mask();
        let carry_13_col126 = eval.next_trace_mask();
        let carry_14_col127 = eval.next_trace_mask();
        let carry_15_col128 = eval.next_trace_mask();
        let carry_16_col129 = eval.next_trace_mask();
        let carry_17_col130 = eval.next_trace_mask();
        let carry_18_col131 = eval.next_trace_mask();
        let carry_19_col132 = eval.next_trace_mask();
        let carry_20_col133 = eval.next_trace_mask();
        let carry_21_col134 = eval.next_trace_mask();
        let carry_22_col135 = eval.next_trace_mask();
        let carry_23_col136 = eval.next_trace_mask();
        let carry_24_col137 = eval.next_trace_mask();
        let carry_25_col138 = eval.next_trace_mask();
        let carry_26_col139 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26, felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_27] =
            Felt252UnpackFrom27RangeCheckOutput::evaluate(
                [
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
                ],
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                &self.range_check_9_9_lookup_elements,
                &self.range_check_9_9_b_lookup_elements,
                &self.range_check_9_9_c_lookup_elements,
                &self.range_check_9_9_d_lookup_elements,
                &self.range_check_9_9_e_lookup_elements,
                &self.range_check_9_9_f_lookup_elements,
                &self.range_check_9_9_g_lookup_elements,
                &self.range_check_9_9_h_lookup_elements,
                &mut eval,
            );
        Mul252::evaluate(
            [
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26.clone(),
                input_limb_9_col9.clone(),
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26.clone(),
                input_limb_9_col9.clone(),
            ],
            mul_res_limb_0_col28.clone(),
            mul_res_limb_1_col29.clone(),
            mul_res_limb_2_col30.clone(),
            mul_res_limb_3_col31.clone(),
            mul_res_limb_4_col32.clone(),
            mul_res_limb_5_col33.clone(),
            mul_res_limb_6_col34.clone(),
            mul_res_limb_7_col35.clone(),
            mul_res_limb_8_col36.clone(),
            mul_res_limb_9_col37.clone(),
            mul_res_limb_10_col38.clone(),
            mul_res_limb_11_col39.clone(),
            mul_res_limb_12_col40.clone(),
            mul_res_limb_13_col41.clone(),
            mul_res_limb_14_col42.clone(),
            mul_res_limb_15_col43.clone(),
            mul_res_limb_16_col44.clone(),
            mul_res_limb_17_col45.clone(),
            mul_res_limb_18_col46.clone(),
            mul_res_limb_19_col47.clone(),
            mul_res_limb_20_col48.clone(),
            mul_res_limb_21_col49.clone(),
            mul_res_limb_22_col50.clone(),
            mul_res_limb_23_col51.clone(),
            mul_res_limb_24_col52.clone(),
            mul_res_limb_25_col53.clone(),
            mul_res_limb_26_col54.clone(),
            mul_res_limb_27_col55.clone(),
            k_col56.clone(),
            carry_0_col57.clone(),
            carry_1_col58.clone(),
            carry_2_col59.clone(),
            carry_3_col60.clone(),
            carry_4_col61.clone(),
            carry_5_col62.clone(),
            carry_6_col63.clone(),
            carry_7_col64.clone(),
            carry_8_col65.clone(),
            carry_9_col66.clone(),
            carry_10_col67.clone(),
            carry_11_col68.clone(),
            carry_12_col69.clone(),
            carry_13_col70.clone(),
            carry_14_col71.clone(),
            carry_15_col72.clone(),
            carry_16_col73.clone(),
            carry_17_col74.clone(),
            carry_18_col75.clone(),
            carry_19_col76.clone(),
            carry_20_col77.clone(),
            carry_21_col78.clone(),
            carry_22_col79.clone(),
            carry_23_col80.clone(),
            carry_24_col81.clone(),
            carry_25_col82.clone(),
            carry_26_col83.clone(),
            &self.range_check_9_9_lookup_elements,
            &self.range_check_9_9_b_lookup_elements,
            &self.range_check_9_9_c_lookup_elements,
            &self.range_check_9_9_d_lookup_elements,
            &self.range_check_9_9_e_lookup_elements,
            &self.range_check_9_9_f_lookup_elements,
            &self.range_check_9_9_g_lookup_elements,
            &self.range_check_9_9_h_lookup_elements,
            &self.range_check_20_lookup_elements,
            &self.range_check_20_b_lookup_elements,
            &self.range_check_20_c_lookup_elements,
            &self.range_check_20_d_lookup_elements,
            &self.range_check_20_e_lookup_elements,
            &self.range_check_20_f_lookup_elements,
            &self.range_check_20_g_lookup_elements,
            &self.range_check_20_h_lookup_elements,
            &mut eval,
        );
        Mul252::evaluate(
            [
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26.clone(),
                input_limb_9_col9.clone(),
                mul_res_limb_0_col28.clone(),
                mul_res_limb_1_col29.clone(),
                mul_res_limb_2_col30.clone(),
                mul_res_limb_3_col31.clone(),
                mul_res_limb_4_col32.clone(),
                mul_res_limb_5_col33.clone(),
                mul_res_limb_6_col34.clone(),
                mul_res_limb_7_col35.clone(),
                mul_res_limb_8_col36.clone(),
                mul_res_limb_9_col37.clone(),
                mul_res_limb_10_col38.clone(),
                mul_res_limb_11_col39.clone(),
                mul_res_limb_12_col40.clone(),
                mul_res_limb_13_col41.clone(),
                mul_res_limb_14_col42.clone(),
                mul_res_limb_15_col43.clone(),
                mul_res_limb_16_col44.clone(),
                mul_res_limb_17_col45.clone(),
                mul_res_limb_18_col46.clone(),
                mul_res_limb_19_col47.clone(),
                mul_res_limb_20_col48.clone(),
                mul_res_limb_21_col49.clone(),
                mul_res_limb_22_col50.clone(),
                mul_res_limb_23_col51.clone(),
                mul_res_limb_24_col52.clone(),
                mul_res_limb_25_col53.clone(),
                mul_res_limb_26_col54.clone(),
                mul_res_limb_27_col55.clone(),
            ],
            mul_res_limb_0_col84.clone(),
            mul_res_limb_1_col85.clone(),
            mul_res_limb_2_col86.clone(),
            mul_res_limb_3_col87.clone(),
            mul_res_limb_4_col88.clone(),
            mul_res_limb_5_col89.clone(),
            mul_res_limb_6_col90.clone(),
            mul_res_limb_7_col91.clone(),
            mul_res_limb_8_col92.clone(),
            mul_res_limb_9_col93.clone(),
            mul_res_limb_10_col94.clone(),
            mul_res_limb_11_col95.clone(),
            mul_res_limb_12_col96.clone(),
            mul_res_limb_13_col97.clone(),
            mul_res_limb_14_col98.clone(),
            mul_res_limb_15_col99.clone(),
            mul_res_limb_16_col100.clone(),
            mul_res_limb_17_col101.clone(),
            mul_res_limb_18_col102.clone(),
            mul_res_limb_19_col103.clone(),
            mul_res_limb_20_col104.clone(),
            mul_res_limb_21_col105.clone(),
            mul_res_limb_22_col106.clone(),
            mul_res_limb_23_col107.clone(),
            mul_res_limb_24_col108.clone(),
            mul_res_limb_25_col109.clone(),
            mul_res_limb_26_col110.clone(),
            mul_res_limb_27_col111.clone(),
            k_col112.clone(),
            carry_0_col113.clone(),
            carry_1_col114.clone(),
            carry_2_col115.clone(),
            carry_3_col116.clone(),
            carry_4_col117.clone(),
            carry_5_col118.clone(),
            carry_6_col119.clone(),
            carry_7_col120.clone(),
            carry_8_col121.clone(),
            carry_9_col122.clone(),
            carry_10_col123.clone(),
            carry_11_col124.clone(),
            carry_12_col125.clone(),
            carry_13_col126.clone(),
            carry_14_col127.clone(),
            carry_15_col128.clone(),
            carry_16_col129.clone(),
            carry_17_col130.clone(),
            carry_18_col131.clone(),
            carry_19_col132.clone(),
            carry_20_col133.clone(),
            carry_21_col134.clone(),
            carry_22_col135.clone(),
            carry_23_col136.clone(),
            carry_24_col137.clone(),
            carry_25_col138.clone(),
            carry_26_col139.clone(),
            &self.range_check_9_9_lookup_elements,
            &self.range_check_9_9_b_lookup_elements,
            &self.range_check_9_9_c_lookup_elements,
            &self.range_check_9_9_d_lookup_elements,
            &self.range_check_9_9_e_lookup_elements,
            &self.range_check_9_9_f_lookup_elements,
            &self.range_check_9_9_g_lookup_elements,
            &self.range_check_9_9_h_lookup_elements,
            &self.range_check_20_lookup_elements,
            &self.range_check_20_b_lookup_elements,
            &self.range_check_20_c_lookup_elements,
            &self.range_check_20_d_lookup_elements,
            &self.range_check_20_e_lookup_elements,
            &self.range_check_20_f_lookup_elements,
            &self.range_check_20_g_lookup_elements,
            &self.range_check_20_h_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            -E::EF::from(enabler.clone()),
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
                ((mul_res_limb_0_col84.clone() + (mul_res_limb_1_col85.clone() * M31_512.clone()))
                    + (mul_res_limb_2_col86.clone() * M31_262144.clone())),
                ((mul_res_limb_3_col87.clone() + (mul_res_limb_4_col88.clone() * M31_512.clone()))
                    + (mul_res_limb_5_col89.clone() * M31_262144.clone())),
                ((mul_res_limb_6_col90.clone() + (mul_res_limb_7_col91.clone() * M31_512.clone()))
                    + (mul_res_limb_8_col92.clone() * M31_262144.clone())),
                ((mul_res_limb_9_col93.clone()
                    + (mul_res_limb_10_col94.clone() * M31_512.clone()))
                    + (mul_res_limb_11_col95.clone() * M31_262144.clone())),
                ((mul_res_limb_12_col96.clone()
                    + (mul_res_limb_13_col97.clone() * M31_512.clone()))
                    + (mul_res_limb_14_col98.clone() * M31_262144.clone())),
                ((mul_res_limb_15_col99.clone()
                    + (mul_res_limb_16_col100.clone() * M31_512.clone()))
                    + (mul_res_limb_17_col101.clone() * M31_262144.clone())),
                ((mul_res_limb_18_col102.clone()
                    + (mul_res_limb_19_col103.clone() * M31_512.clone()))
                    + (mul_res_limb_20_col104.clone() * M31_262144.clone())),
                ((mul_res_limb_21_col105.clone()
                    + (mul_res_limb_22_col106.clone() * M31_512.clone()))
                    + (mul_res_limb_23_col107.clone() * M31_262144.clone())),
                ((mul_res_limb_24_col108.clone()
                    + (mul_res_limb_25_col109.clone() * M31_512.clone()))
                    + (mul_res_limb_26_col110.clone() * M31_262144.clone())),
                mul_res_limb_27_col111.clone(),
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
    use crate::components::constraints_regression_test_values::CUBE_252;

    #[test]
    fn cube_252_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F::dummy(),
            range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G::dummy(),
            range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H::dummy(),
            range_check_20_lookup_elements: relations::RangeCheck_20::dummy(),
            range_check_20_b_lookup_elements: relations::RangeCheck_20_B::dummy(),
            range_check_20_c_lookup_elements: relations::RangeCheck_20_C::dummy(),
            range_check_20_d_lookup_elements: relations::RangeCheck_20_D::dummy(),
            range_check_20_e_lookup_elements: relations::RangeCheck_20_E::dummy(),
            range_check_20_f_lookup_elements: relations::RangeCheck_20_F::dummy(),
            range_check_20_g_lookup_elements: relations::RangeCheck_20_G::dummy(),
            range_check_20_h_lookup_elements: relations::RangeCheck_20_H::dummy(),
            cube_252_lookup_elements: relations::Cube252::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, CUBE_252);
    }
}

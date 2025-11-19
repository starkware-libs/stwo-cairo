// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_split::ReadSplit;
use crate::components::subroutines::verify_reduced_252::VerifyReduced252;

pub const N_TRACE_COLUMNS: usize = 264;
pub const RELATION_USES_PER_ROW: [RelationUse; 5] = [
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 3,
    },
    RelationUse {
        relation_id: "PartialEcMul",
        uses: 2,
    },
    RelationUse {
        relation_id: "PedersenPointsTable",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_5_4",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_8",
        uses: 4,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub range_check_5_4_lookup_elements: relations::RangeCheck_5_4,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_8_lookup_elements: relations::RangeCheck_8,
    pub pedersen_points_table_lookup_elements: relations::PedersenPointsTable,
    pub partial_ec_mul_lookup_elements: relations::PartialEcMul,
    pub pedersen_aggregator_lookup_elements: relations::PedersenAggregator,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 8];
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
        let M31_0 = E::F::from(M31::from(0));
        let M31_1 = E::F::from(M31::from(1));
        let M31_14 = E::F::from(M31::from(14));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_28 = E::F::from(M31::from(28));
        let M31_512 = E::F::from(M31::from(512));
        let M31_7340032 = E::F::from(M31::from(7340032));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let value_limb_0_col3 = eval.next_trace_mask();
        let value_limb_1_col4 = eval.next_trace_mask();
        let value_limb_2_col5 = eval.next_trace_mask();
        let value_limb_3_col6 = eval.next_trace_mask();
        let value_limb_4_col7 = eval.next_trace_mask();
        let value_limb_5_col8 = eval.next_trace_mask();
        let value_limb_6_col9 = eval.next_trace_mask();
        let value_limb_7_col10 = eval.next_trace_mask();
        let value_limb_8_col11 = eval.next_trace_mask();
        let value_limb_9_col12 = eval.next_trace_mask();
        let value_limb_10_col13 = eval.next_trace_mask();
        let value_limb_11_col14 = eval.next_trace_mask();
        let value_limb_12_col15 = eval.next_trace_mask();
        let value_limb_13_col16 = eval.next_trace_mask();
        let value_limb_14_col17 = eval.next_trace_mask();
        let value_limb_15_col18 = eval.next_trace_mask();
        let value_limb_16_col19 = eval.next_trace_mask();
        let value_limb_17_col20 = eval.next_trace_mask();
        let value_limb_18_col21 = eval.next_trace_mask();
        let value_limb_19_col22 = eval.next_trace_mask();
        let value_limb_20_col23 = eval.next_trace_mask();
        let value_limb_21_col24 = eval.next_trace_mask();
        let value_limb_22_col25 = eval.next_trace_mask();
        let value_limb_23_col26 = eval.next_trace_mask();
        let value_limb_24_col27 = eval.next_trace_mask();
        let value_limb_25_col28 = eval.next_trace_mask();
        let value_limb_26_col29 = eval.next_trace_mask();
        let ms_limb_low_col30 = eval.next_trace_mask();
        let ms_limb_high_col31 = eval.next_trace_mask();
        let value_limb_0_col32 = eval.next_trace_mask();
        let value_limb_1_col33 = eval.next_trace_mask();
        let value_limb_2_col34 = eval.next_trace_mask();
        let value_limb_3_col35 = eval.next_trace_mask();
        let value_limb_4_col36 = eval.next_trace_mask();
        let value_limb_5_col37 = eval.next_trace_mask();
        let value_limb_6_col38 = eval.next_trace_mask();
        let value_limb_7_col39 = eval.next_trace_mask();
        let value_limb_8_col40 = eval.next_trace_mask();
        let value_limb_9_col41 = eval.next_trace_mask();
        let value_limb_10_col42 = eval.next_trace_mask();
        let value_limb_11_col43 = eval.next_trace_mask();
        let value_limb_12_col44 = eval.next_trace_mask();
        let value_limb_13_col45 = eval.next_trace_mask();
        let value_limb_14_col46 = eval.next_trace_mask();
        let value_limb_15_col47 = eval.next_trace_mask();
        let value_limb_16_col48 = eval.next_trace_mask();
        let value_limb_17_col49 = eval.next_trace_mask();
        let value_limb_18_col50 = eval.next_trace_mask();
        let value_limb_19_col51 = eval.next_trace_mask();
        let value_limb_20_col52 = eval.next_trace_mask();
        let value_limb_21_col53 = eval.next_trace_mask();
        let value_limb_22_col54 = eval.next_trace_mask();
        let value_limb_23_col55 = eval.next_trace_mask();
        let value_limb_24_col56 = eval.next_trace_mask();
        let value_limb_25_col57 = eval.next_trace_mask();
        let value_limb_26_col58 = eval.next_trace_mask();
        let ms_limb_low_col59 = eval.next_trace_mask();
        let ms_limb_high_col60 = eval.next_trace_mask();
        let ms_limb_is_max_col61 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col62 = eval.next_trace_mask();
        let rc_input_col63 = eval.next_trace_mask();
        let ms_limb_is_max_col64 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col65 = eval.next_trace_mask();
        let rc_input_col66 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_0_col67 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_1_col68 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_2_col69 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_3_col70 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_4_col71 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_5_col72 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_6_col73 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_7_col74 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_8_col75 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_9_col76 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_10_col77 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_11_col78 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_12_col79 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_13_col80 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_14_col81 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_15_col82 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_16_col83 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_17_col84 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_18_col85 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_19_col86 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_20_col87 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_21_col88 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_22_col89 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_23_col90 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_24_col91 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_25_col92 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_26_col93 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_27_col94 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_28_col95 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_29_col96 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_30_col97 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_31_col98 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_32_col99 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_33_col100 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_34_col101 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_35_col102 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_36_col103 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_37_col104 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_38_col105 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_39_col106 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_40_col107 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_41_col108 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_42_col109 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_43_col110 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_44_col111 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_45_col112 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_46_col113 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_47_col114 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_48_col115 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_49_col116 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_50_col117 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_51_col118 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_52_col119 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_53_col120 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_54_col121 = eval.next_trace_mask();
        let pedersen_points_table_output_limb_55_col122 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_0_col123 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_1_col124 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_2_col125 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_3_col126 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_4_col127 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_5_col128 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_6_col129 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_7_col130 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_8_col131 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_9_col132 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_10_col133 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_11_col134 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_12_col135 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_13_col136 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_14_col137 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_15_col138 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_16_col139 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_17_col140 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_18_col141 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_19_col142 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_20_col143 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_21_col144 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_22_col145 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_23_col146 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_24_col147 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_25_col148 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_26_col149 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_27_col150 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_28_col151 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_29_col152 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_30_col153 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_31_col154 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_32_col155 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_33_col156 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_34_col157 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_35_col158 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_36_col159 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_37_col160 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_38_col161 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_39_col162 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_40_col163 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_41_col164 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_42_col165 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_43_col166 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_44_col167 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_45_col168 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_46_col169 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_47_col170 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_48_col171 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_49_col172 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_50_col173 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_51_col174 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_52_col175 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_53_col176 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_54_col177 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_55_col178 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_56_col179 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_57_col180 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_58_col181 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_59_col182 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_60_col183 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_61_col184 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_62_col185 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_63_col186 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_64_col187 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_65_col188 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_66_col189 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_67_col190 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_68_col191 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_69_col192 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_0_col193 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_1_col194 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_2_col195 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_3_col196 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_4_col197 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_5_col198 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_6_col199 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_7_col200 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_8_col201 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_9_col202 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_10_col203 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_11_col204 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_12_col205 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_13_col206 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_14_col207 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_15_col208 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_16_col209 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_17_col210 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_18_col211 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_19_col212 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_20_col213 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_21_col214 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_22_col215 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_23_col216 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_24_col217 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_25_col218 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_26_col219 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_27_col220 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_28_col221 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_29_col222 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_30_col223 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_31_col224 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_32_col225 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_33_col226 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_34_col227 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_35_col228 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_36_col229 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_37_col230 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_38_col231 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_39_col232 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_40_col233 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_41_col234 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_42_col235 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_43_col236 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_44_col237 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_45_col238 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_46_col239 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_47_col240 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_48_col241 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_49_col242 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_50_col243 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_51_col244 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_52_col245 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_53_col246 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_54_col247 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_55_col248 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_56_col249 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_57_col250 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_58_col251 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_59_col252 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_60_col253 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_61_col254 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_62_col255 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_63_col256 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_64_col257 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_65_col258 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_66_col259 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_67_col260 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_68_col261 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_69_col262 = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_split_output_tmp_c48a1_4_original_limb_27] = ReadSplit::evaluate(
            [input_limb_0_col0.clone()],
            value_limb_0_col3.clone(),
            value_limb_1_col4.clone(),
            value_limb_2_col5.clone(),
            value_limb_3_col6.clone(),
            value_limb_4_col7.clone(),
            value_limb_5_col8.clone(),
            value_limb_6_col9.clone(),
            value_limb_7_col10.clone(),
            value_limb_8_col11.clone(),
            value_limb_9_col12.clone(),
            value_limb_10_col13.clone(),
            value_limb_11_col14.clone(),
            value_limb_12_col15.clone(),
            value_limb_13_col16.clone(),
            value_limb_14_col17.clone(),
            value_limb_15_col18.clone(),
            value_limb_16_col19.clone(),
            value_limb_17_col20.clone(),
            value_limb_18_col21.clone(),
            value_limb_19_col22.clone(),
            value_limb_20_col23.clone(),
            value_limb_21_col24.clone(),
            value_limb_22_col25.clone(),
            value_limb_23_col26.clone(),
            value_limb_24_col27.clone(),
            value_limb_25_col28.clone(),
            value_limb_26_col29.clone(),
            ms_limb_low_col30.clone(),
            ms_limb_high_col31.clone(),
            &self.range_check_5_4_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_split_output_tmp_c48a1_9_original_limb_27] = ReadSplit::evaluate(
            [input_limb_1_col1.clone()],
            value_limb_0_col32.clone(),
            value_limb_1_col33.clone(),
            value_limb_2_col34.clone(),
            value_limb_3_col35.clone(),
            value_limb_4_col36.clone(),
            value_limb_5_col37.clone(),
            value_limb_6_col38.clone(),
            value_limb_7_col39.clone(),
            value_limb_8_col40.clone(),
            value_limb_9_col41.clone(),
            value_limb_10_col42.clone(),
            value_limb_11_col43.clone(),
            value_limb_12_col44.clone(),
            value_limb_13_col45.clone(),
            value_limb_14_col46.clone(),
            value_limb_15_col47.clone(),
            value_limb_16_col48.clone(),
            value_limb_17_col49.clone(),
            value_limb_18_col50.clone(),
            value_limb_19_col51.clone(),
            value_limb_20_col52.clone(),
            value_limb_21_col53.clone(),
            value_limb_22_col54.clone(),
            value_limb_23_col55.clone(),
            value_limb_24_col56.clone(),
            value_limb_25_col57.clone(),
            value_limb_26_col58.clone(),
            ms_limb_low_col59.clone(),
            ms_limb_high_col60.clone(),
            &self.range_check_5_4_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        VerifyReduced252::evaluate(
            [
                value_limb_0_col3.clone(),
                value_limb_1_col4.clone(),
                value_limb_2_col5.clone(),
                value_limb_3_col6.clone(),
                value_limb_4_col7.clone(),
                value_limb_5_col8.clone(),
                value_limb_6_col9.clone(),
                value_limb_7_col10.clone(),
                value_limb_8_col11.clone(),
                value_limb_9_col12.clone(),
                value_limb_10_col13.clone(),
                value_limb_11_col14.clone(),
                value_limb_12_col15.clone(),
                value_limb_13_col16.clone(),
                value_limb_14_col17.clone(),
                value_limb_15_col18.clone(),
                value_limb_16_col19.clone(),
                value_limb_17_col20.clone(),
                value_limb_18_col21.clone(),
                value_limb_19_col22.clone(),
                value_limb_20_col23.clone(),
                value_limb_21_col24.clone(),
                value_limb_22_col25.clone(),
                value_limb_23_col26.clone(),
                value_limb_24_col27.clone(),
                value_limb_25_col28.clone(),
                value_limb_26_col29.clone(),
                read_split_output_tmp_c48a1_4_original_limb_27.clone(),
            ],
            ms_limb_is_max_col61.clone(),
            ms_and_mid_limbs_are_max_col62.clone(),
            rc_input_col63.clone(),
            &self.range_check_8_lookup_elements,
            &mut eval,
        );
        VerifyReduced252::evaluate(
            [
                value_limb_0_col32.clone(),
                value_limb_1_col33.clone(),
                value_limb_2_col34.clone(),
                value_limb_3_col35.clone(),
                value_limb_4_col36.clone(),
                value_limb_5_col37.clone(),
                value_limb_6_col38.clone(),
                value_limb_7_col39.clone(),
                value_limb_8_col40.clone(),
                value_limb_9_col41.clone(),
                value_limb_10_col42.clone(),
                value_limb_11_col43.clone(),
                value_limb_12_col44.clone(),
                value_limb_13_col45.clone(),
                value_limb_14_col46.clone(),
                value_limb_15_col47.clone(),
                value_limb_16_col48.clone(),
                value_limb_17_col49.clone(),
                value_limb_18_col50.clone(),
                value_limb_19_col51.clone(),
                value_limb_20_col52.clone(),
                value_limb_21_col53.clone(),
                value_limb_22_col54.clone(),
                value_limb_23_col55.clone(),
                value_limb_24_col56.clone(),
                value_limb_25_col57.clone(),
                value_limb_26_col58.clone(),
                read_split_output_tmp_c48a1_9_original_limb_27.clone(),
            ],
            ms_limb_is_max_col64.clone(),
            ms_and_mid_limbs_are_max_col65.clone(),
            rc_input_col66.clone(),
            &self.range_check_8_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            E::EF::one(),
            &[
                ((M31_7340032.clone() + (ms_limb_high_col60.clone() * M31_16.clone()))
                    + ms_limb_high_col31.clone()),
                pedersen_points_table_output_limb_0_col67.clone(),
                pedersen_points_table_output_limb_1_col68.clone(),
                pedersen_points_table_output_limb_2_col69.clone(),
                pedersen_points_table_output_limb_3_col70.clone(),
                pedersen_points_table_output_limb_4_col71.clone(),
                pedersen_points_table_output_limb_5_col72.clone(),
                pedersen_points_table_output_limb_6_col73.clone(),
                pedersen_points_table_output_limb_7_col74.clone(),
                pedersen_points_table_output_limb_8_col75.clone(),
                pedersen_points_table_output_limb_9_col76.clone(),
                pedersen_points_table_output_limb_10_col77.clone(),
                pedersen_points_table_output_limb_11_col78.clone(),
                pedersen_points_table_output_limb_12_col79.clone(),
                pedersen_points_table_output_limb_13_col80.clone(),
                pedersen_points_table_output_limb_14_col81.clone(),
                pedersen_points_table_output_limb_15_col82.clone(),
                pedersen_points_table_output_limb_16_col83.clone(),
                pedersen_points_table_output_limb_17_col84.clone(),
                pedersen_points_table_output_limb_18_col85.clone(),
                pedersen_points_table_output_limb_19_col86.clone(),
                pedersen_points_table_output_limb_20_col87.clone(),
                pedersen_points_table_output_limb_21_col88.clone(),
                pedersen_points_table_output_limb_22_col89.clone(),
                pedersen_points_table_output_limb_23_col90.clone(),
                pedersen_points_table_output_limb_24_col91.clone(),
                pedersen_points_table_output_limb_25_col92.clone(),
                pedersen_points_table_output_limb_26_col93.clone(),
                pedersen_points_table_output_limb_27_col94.clone(),
                pedersen_points_table_output_limb_28_col95.clone(),
                pedersen_points_table_output_limb_29_col96.clone(),
                pedersen_points_table_output_limb_30_col97.clone(),
                pedersen_points_table_output_limb_31_col98.clone(),
                pedersen_points_table_output_limb_32_col99.clone(),
                pedersen_points_table_output_limb_33_col100.clone(),
                pedersen_points_table_output_limb_34_col101.clone(),
                pedersen_points_table_output_limb_35_col102.clone(),
                pedersen_points_table_output_limb_36_col103.clone(),
                pedersen_points_table_output_limb_37_col104.clone(),
                pedersen_points_table_output_limb_38_col105.clone(),
                pedersen_points_table_output_limb_39_col106.clone(),
                pedersen_points_table_output_limb_40_col107.clone(),
                pedersen_points_table_output_limb_41_col108.clone(),
                pedersen_points_table_output_limb_42_col109.clone(),
                pedersen_points_table_output_limb_43_col110.clone(),
                pedersen_points_table_output_limb_44_col111.clone(),
                pedersen_points_table_output_limb_45_col112.clone(),
                pedersen_points_table_output_limb_46_col113.clone(),
                pedersen_points_table_output_limb_47_col114.clone(),
                pedersen_points_table_output_limb_48_col115.clone(),
                pedersen_points_table_output_limb_49_col116.clone(),
                pedersen_points_table_output_limb_50_col117.clone(),
                pedersen_points_table_output_limb_51_col118.clone(),
                pedersen_points_table_output_limb_52_col119.clone(),
                pedersen_points_table_output_limb_53_col120.clone(),
                pedersen_points_table_output_limb_54_col121.clone(),
                pedersen_points_table_output_limb_55_col122.clone(),
            ],
        ));

        let partial_ec_mul_chain_tmp_tmp_c48a1_15 =
            eval.add_intermediate((seq.clone() * M31_2.clone()));
        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::one(),
            &[
                partial_ec_mul_chain_tmp_tmp_c48a1_15.clone(),
                M31_0.clone(),
                (value_limb_0_col3.clone() + (value_limb_1_col4.clone() * M31_512.clone())),
                (value_limb_2_col5.clone() + (value_limb_3_col6.clone() * M31_512.clone())),
                (value_limb_4_col7.clone() + (value_limb_5_col8.clone() * M31_512.clone())),
                (value_limb_6_col9.clone() + (value_limb_7_col10.clone() * M31_512.clone())),
                (value_limb_8_col11.clone() + (value_limb_9_col12.clone() * M31_512.clone())),
                (value_limb_10_col13.clone() + (value_limb_11_col14.clone() * M31_512.clone())),
                (value_limb_12_col15.clone() + (value_limb_13_col16.clone() * M31_512.clone())),
                (value_limb_14_col17.clone() + (value_limb_15_col18.clone() * M31_512.clone())),
                (value_limb_16_col19.clone() + (value_limb_17_col20.clone() * M31_512.clone())),
                (value_limb_18_col21.clone() + (value_limb_19_col22.clone() * M31_512.clone())),
                (value_limb_20_col23.clone() + (value_limb_21_col24.clone() * M31_512.clone())),
                (value_limb_22_col25.clone() + (value_limb_23_col26.clone() * M31_512.clone())),
                (value_limb_24_col27.clone() + (value_limb_25_col28.clone() * M31_512.clone())),
                (value_limb_26_col29.clone() + (ms_limb_low_col30.clone() * M31_512.clone())),
                pedersen_points_table_output_limb_0_col67.clone(),
                pedersen_points_table_output_limb_1_col68.clone(),
                pedersen_points_table_output_limb_2_col69.clone(),
                pedersen_points_table_output_limb_3_col70.clone(),
                pedersen_points_table_output_limb_4_col71.clone(),
                pedersen_points_table_output_limb_5_col72.clone(),
                pedersen_points_table_output_limb_6_col73.clone(),
                pedersen_points_table_output_limb_7_col74.clone(),
                pedersen_points_table_output_limb_8_col75.clone(),
                pedersen_points_table_output_limb_9_col76.clone(),
                pedersen_points_table_output_limb_10_col77.clone(),
                pedersen_points_table_output_limb_11_col78.clone(),
                pedersen_points_table_output_limb_12_col79.clone(),
                pedersen_points_table_output_limb_13_col80.clone(),
                pedersen_points_table_output_limb_14_col81.clone(),
                pedersen_points_table_output_limb_15_col82.clone(),
                pedersen_points_table_output_limb_16_col83.clone(),
                pedersen_points_table_output_limb_17_col84.clone(),
                pedersen_points_table_output_limb_18_col85.clone(),
                pedersen_points_table_output_limb_19_col86.clone(),
                pedersen_points_table_output_limb_20_col87.clone(),
                pedersen_points_table_output_limb_21_col88.clone(),
                pedersen_points_table_output_limb_22_col89.clone(),
                pedersen_points_table_output_limb_23_col90.clone(),
                pedersen_points_table_output_limb_24_col91.clone(),
                pedersen_points_table_output_limb_25_col92.clone(),
                pedersen_points_table_output_limb_26_col93.clone(),
                pedersen_points_table_output_limb_27_col94.clone(),
                pedersen_points_table_output_limb_28_col95.clone(),
                pedersen_points_table_output_limb_29_col96.clone(),
                pedersen_points_table_output_limb_30_col97.clone(),
                pedersen_points_table_output_limb_31_col98.clone(),
                pedersen_points_table_output_limb_32_col99.clone(),
                pedersen_points_table_output_limb_33_col100.clone(),
                pedersen_points_table_output_limb_34_col101.clone(),
                pedersen_points_table_output_limb_35_col102.clone(),
                pedersen_points_table_output_limb_36_col103.clone(),
                pedersen_points_table_output_limb_37_col104.clone(),
                pedersen_points_table_output_limb_38_col105.clone(),
                pedersen_points_table_output_limb_39_col106.clone(),
                pedersen_points_table_output_limb_40_col107.clone(),
                pedersen_points_table_output_limb_41_col108.clone(),
                pedersen_points_table_output_limb_42_col109.clone(),
                pedersen_points_table_output_limb_43_col110.clone(),
                pedersen_points_table_output_limb_44_col111.clone(),
                pedersen_points_table_output_limb_45_col112.clone(),
                pedersen_points_table_output_limb_46_col113.clone(),
                pedersen_points_table_output_limb_47_col114.clone(),
                pedersen_points_table_output_limb_48_col115.clone(),
                pedersen_points_table_output_limb_49_col116.clone(),
                pedersen_points_table_output_limb_50_col117.clone(),
                pedersen_points_table_output_limb_51_col118.clone(),
                pedersen_points_table_output_limb_52_col119.clone(),
                pedersen_points_table_output_limb_53_col120.clone(),
                pedersen_points_table_output_limb_54_col121.clone(),
                pedersen_points_table_output_limb_55_col122.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::one(),
            &[
                partial_ec_mul_chain_tmp_tmp_c48a1_15.clone(),
                M31_14.clone(),
                partial_ec_mul_output_limb_0_col123.clone(),
                partial_ec_mul_output_limb_1_col124.clone(),
                partial_ec_mul_output_limb_2_col125.clone(),
                partial_ec_mul_output_limb_3_col126.clone(),
                partial_ec_mul_output_limb_4_col127.clone(),
                partial_ec_mul_output_limb_5_col128.clone(),
                partial_ec_mul_output_limb_6_col129.clone(),
                partial_ec_mul_output_limb_7_col130.clone(),
                partial_ec_mul_output_limb_8_col131.clone(),
                partial_ec_mul_output_limb_9_col132.clone(),
                partial_ec_mul_output_limb_10_col133.clone(),
                partial_ec_mul_output_limb_11_col134.clone(),
                partial_ec_mul_output_limb_12_col135.clone(),
                partial_ec_mul_output_limb_13_col136.clone(),
                partial_ec_mul_output_limb_14_col137.clone(),
                partial_ec_mul_output_limb_15_col138.clone(),
                partial_ec_mul_output_limb_16_col139.clone(),
                partial_ec_mul_output_limb_17_col140.clone(),
                partial_ec_mul_output_limb_18_col141.clone(),
                partial_ec_mul_output_limb_19_col142.clone(),
                partial_ec_mul_output_limb_20_col143.clone(),
                partial_ec_mul_output_limb_21_col144.clone(),
                partial_ec_mul_output_limb_22_col145.clone(),
                partial_ec_mul_output_limb_23_col146.clone(),
                partial_ec_mul_output_limb_24_col147.clone(),
                partial_ec_mul_output_limb_25_col148.clone(),
                partial_ec_mul_output_limb_26_col149.clone(),
                partial_ec_mul_output_limb_27_col150.clone(),
                partial_ec_mul_output_limb_28_col151.clone(),
                partial_ec_mul_output_limb_29_col152.clone(),
                partial_ec_mul_output_limb_30_col153.clone(),
                partial_ec_mul_output_limb_31_col154.clone(),
                partial_ec_mul_output_limb_32_col155.clone(),
                partial_ec_mul_output_limb_33_col156.clone(),
                partial_ec_mul_output_limb_34_col157.clone(),
                partial_ec_mul_output_limb_35_col158.clone(),
                partial_ec_mul_output_limb_36_col159.clone(),
                partial_ec_mul_output_limb_37_col160.clone(),
                partial_ec_mul_output_limb_38_col161.clone(),
                partial_ec_mul_output_limb_39_col162.clone(),
                partial_ec_mul_output_limb_40_col163.clone(),
                partial_ec_mul_output_limb_41_col164.clone(),
                partial_ec_mul_output_limb_42_col165.clone(),
                partial_ec_mul_output_limb_43_col166.clone(),
                partial_ec_mul_output_limb_44_col167.clone(),
                partial_ec_mul_output_limb_45_col168.clone(),
                partial_ec_mul_output_limb_46_col169.clone(),
                partial_ec_mul_output_limb_47_col170.clone(),
                partial_ec_mul_output_limb_48_col171.clone(),
                partial_ec_mul_output_limb_49_col172.clone(),
                partial_ec_mul_output_limb_50_col173.clone(),
                partial_ec_mul_output_limb_51_col174.clone(),
                partial_ec_mul_output_limb_52_col175.clone(),
                partial_ec_mul_output_limb_53_col176.clone(),
                partial_ec_mul_output_limb_54_col177.clone(),
                partial_ec_mul_output_limb_55_col178.clone(),
                partial_ec_mul_output_limb_56_col179.clone(),
                partial_ec_mul_output_limb_57_col180.clone(),
                partial_ec_mul_output_limb_58_col181.clone(),
                partial_ec_mul_output_limb_59_col182.clone(),
                partial_ec_mul_output_limb_60_col183.clone(),
                partial_ec_mul_output_limb_61_col184.clone(),
                partial_ec_mul_output_limb_62_col185.clone(),
                partial_ec_mul_output_limb_63_col186.clone(),
                partial_ec_mul_output_limb_64_col187.clone(),
                partial_ec_mul_output_limb_65_col188.clone(),
                partial_ec_mul_output_limb_66_col189.clone(),
                partial_ec_mul_output_limb_67_col190.clone(),
                partial_ec_mul_output_limb_68_col191.clone(),
                partial_ec_mul_output_limb_69_col192.clone(),
            ],
        ));

        let partial_ec_mul_chain_id_tmp_c48a1_30 =
            eval.add_intermediate((partial_ec_mul_chain_tmp_tmp_c48a1_15.clone() + M31_1.clone()));
        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::one(),
            &[
                partial_ec_mul_chain_id_tmp_c48a1_30.clone(),
                M31_14.clone(),
                (value_limb_0_col32.clone() + (value_limb_1_col33.clone() * M31_512.clone())),
                (value_limb_2_col34.clone() + (value_limb_3_col35.clone() * M31_512.clone())),
                (value_limb_4_col36.clone() + (value_limb_5_col37.clone() * M31_512.clone())),
                (value_limb_6_col38.clone() + (value_limb_7_col39.clone() * M31_512.clone())),
                (value_limb_8_col40.clone() + (value_limb_9_col41.clone() * M31_512.clone())),
                (value_limb_10_col42.clone() + (value_limb_11_col43.clone() * M31_512.clone())),
                (value_limb_12_col44.clone() + (value_limb_13_col45.clone() * M31_512.clone())),
                (value_limb_14_col46.clone() + (value_limb_15_col47.clone() * M31_512.clone())),
                (value_limb_16_col48.clone() + (value_limb_17_col49.clone() * M31_512.clone())),
                (value_limb_18_col50.clone() + (value_limb_19_col51.clone() * M31_512.clone())),
                (value_limb_20_col52.clone() + (value_limb_21_col53.clone() * M31_512.clone())),
                (value_limb_22_col54.clone() + (value_limb_23_col55.clone() * M31_512.clone())),
                (value_limb_24_col56.clone() + (value_limb_25_col57.clone() * M31_512.clone())),
                (value_limb_26_col58.clone() + (ms_limb_low_col59.clone() * M31_512.clone())),
                partial_ec_mul_output_limb_14_col137.clone(),
                partial_ec_mul_output_limb_15_col138.clone(),
                partial_ec_mul_output_limb_16_col139.clone(),
                partial_ec_mul_output_limb_17_col140.clone(),
                partial_ec_mul_output_limb_18_col141.clone(),
                partial_ec_mul_output_limb_19_col142.clone(),
                partial_ec_mul_output_limb_20_col143.clone(),
                partial_ec_mul_output_limb_21_col144.clone(),
                partial_ec_mul_output_limb_22_col145.clone(),
                partial_ec_mul_output_limb_23_col146.clone(),
                partial_ec_mul_output_limb_24_col147.clone(),
                partial_ec_mul_output_limb_25_col148.clone(),
                partial_ec_mul_output_limb_26_col149.clone(),
                partial_ec_mul_output_limb_27_col150.clone(),
                partial_ec_mul_output_limb_28_col151.clone(),
                partial_ec_mul_output_limb_29_col152.clone(),
                partial_ec_mul_output_limb_30_col153.clone(),
                partial_ec_mul_output_limb_31_col154.clone(),
                partial_ec_mul_output_limb_32_col155.clone(),
                partial_ec_mul_output_limb_33_col156.clone(),
                partial_ec_mul_output_limb_34_col157.clone(),
                partial_ec_mul_output_limb_35_col158.clone(),
                partial_ec_mul_output_limb_36_col159.clone(),
                partial_ec_mul_output_limb_37_col160.clone(),
                partial_ec_mul_output_limb_38_col161.clone(),
                partial_ec_mul_output_limb_39_col162.clone(),
                partial_ec_mul_output_limb_40_col163.clone(),
                partial_ec_mul_output_limb_41_col164.clone(),
                partial_ec_mul_output_limb_42_col165.clone(),
                partial_ec_mul_output_limb_43_col166.clone(),
                partial_ec_mul_output_limb_44_col167.clone(),
                partial_ec_mul_output_limb_45_col168.clone(),
                partial_ec_mul_output_limb_46_col169.clone(),
                partial_ec_mul_output_limb_47_col170.clone(),
                partial_ec_mul_output_limb_48_col171.clone(),
                partial_ec_mul_output_limb_49_col172.clone(),
                partial_ec_mul_output_limb_50_col173.clone(),
                partial_ec_mul_output_limb_51_col174.clone(),
                partial_ec_mul_output_limb_52_col175.clone(),
                partial_ec_mul_output_limb_53_col176.clone(),
                partial_ec_mul_output_limb_54_col177.clone(),
                partial_ec_mul_output_limb_55_col178.clone(),
                partial_ec_mul_output_limb_56_col179.clone(),
                partial_ec_mul_output_limb_57_col180.clone(),
                partial_ec_mul_output_limb_58_col181.clone(),
                partial_ec_mul_output_limb_59_col182.clone(),
                partial_ec_mul_output_limb_60_col183.clone(),
                partial_ec_mul_output_limb_61_col184.clone(),
                partial_ec_mul_output_limb_62_col185.clone(),
                partial_ec_mul_output_limb_63_col186.clone(),
                partial_ec_mul_output_limb_64_col187.clone(),
                partial_ec_mul_output_limb_65_col188.clone(),
                partial_ec_mul_output_limb_66_col189.clone(),
                partial_ec_mul_output_limb_67_col190.clone(),
                partial_ec_mul_output_limb_68_col191.clone(),
                partial_ec_mul_output_limb_69_col192.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::one(),
            &[
                partial_ec_mul_chain_id_tmp_c48a1_30.clone(),
                M31_28.clone(),
                partial_ec_mul_output_limb_0_col193.clone(),
                partial_ec_mul_output_limb_1_col194.clone(),
                partial_ec_mul_output_limb_2_col195.clone(),
                partial_ec_mul_output_limb_3_col196.clone(),
                partial_ec_mul_output_limb_4_col197.clone(),
                partial_ec_mul_output_limb_5_col198.clone(),
                partial_ec_mul_output_limb_6_col199.clone(),
                partial_ec_mul_output_limb_7_col200.clone(),
                partial_ec_mul_output_limb_8_col201.clone(),
                partial_ec_mul_output_limb_9_col202.clone(),
                partial_ec_mul_output_limb_10_col203.clone(),
                partial_ec_mul_output_limb_11_col204.clone(),
                partial_ec_mul_output_limb_12_col205.clone(),
                partial_ec_mul_output_limb_13_col206.clone(),
                partial_ec_mul_output_limb_14_col207.clone(),
                partial_ec_mul_output_limb_15_col208.clone(),
                partial_ec_mul_output_limb_16_col209.clone(),
                partial_ec_mul_output_limb_17_col210.clone(),
                partial_ec_mul_output_limb_18_col211.clone(),
                partial_ec_mul_output_limb_19_col212.clone(),
                partial_ec_mul_output_limb_20_col213.clone(),
                partial_ec_mul_output_limb_21_col214.clone(),
                partial_ec_mul_output_limb_22_col215.clone(),
                partial_ec_mul_output_limb_23_col216.clone(),
                partial_ec_mul_output_limb_24_col217.clone(),
                partial_ec_mul_output_limb_25_col218.clone(),
                partial_ec_mul_output_limb_26_col219.clone(),
                partial_ec_mul_output_limb_27_col220.clone(),
                partial_ec_mul_output_limb_28_col221.clone(),
                partial_ec_mul_output_limb_29_col222.clone(),
                partial_ec_mul_output_limb_30_col223.clone(),
                partial_ec_mul_output_limb_31_col224.clone(),
                partial_ec_mul_output_limb_32_col225.clone(),
                partial_ec_mul_output_limb_33_col226.clone(),
                partial_ec_mul_output_limb_34_col227.clone(),
                partial_ec_mul_output_limb_35_col228.clone(),
                partial_ec_mul_output_limb_36_col229.clone(),
                partial_ec_mul_output_limb_37_col230.clone(),
                partial_ec_mul_output_limb_38_col231.clone(),
                partial_ec_mul_output_limb_39_col232.clone(),
                partial_ec_mul_output_limb_40_col233.clone(),
                partial_ec_mul_output_limb_41_col234.clone(),
                partial_ec_mul_output_limb_42_col235.clone(),
                partial_ec_mul_output_limb_43_col236.clone(),
                partial_ec_mul_output_limb_44_col237.clone(),
                partial_ec_mul_output_limb_45_col238.clone(),
                partial_ec_mul_output_limb_46_col239.clone(),
                partial_ec_mul_output_limb_47_col240.clone(),
                partial_ec_mul_output_limb_48_col241.clone(),
                partial_ec_mul_output_limb_49_col242.clone(),
                partial_ec_mul_output_limb_50_col243.clone(),
                partial_ec_mul_output_limb_51_col244.clone(),
                partial_ec_mul_output_limb_52_col245.clone(),
                partial_ec_mul_output_limb_53_col246.clone(),
                partial_ec_mul_output_limb_54_col247.clone(),
                partial_ec_mul_output_limb_55_col248.clone(),
                partial_ec_mul_output_limb_56_col249.clone(),
                partial_ec_mul_output_limb_57_col250.clone(),
                partial_ec_mul_output_limb_58_col251.clone(),
                partial_ec_mul_output_limb_59_col252.clone(),
                partial_ec_mul_output_limb_60_col253.clone(),
                partial_ec_mul_output_limb_61_col254.clone(),
                partial_ec_mul_output_limb_62_col255.clone(),
                partial_ec_mul_output_limb_63_col256.clone(),
                partial_ec_mul_output_limb_64_col257.clone(),
                partial_ec_mul_output_limb_65_col258.clone(),
                partial_ec_mul_output_limb_66_col259.clone(),
                partial_ec_mul_output_limb_67_col260.clone(),
                partial_ec_mul_output_limb_68_col261.clone(),
                partial_ec_mul_output_limb_69_col262.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                input_limb_2_col2.clone(),
                partial_ec_mul_output_limb_14_col207.clone(),
                partial_ec_mul_output_limb_15_col208.clone(),
                partial_ec_mul_output_limb_16_col209.clone(),
                partial_ec_mul_output_limb_17_col210.clone(),
                partial_ec_mul_output_limb_18_col211.clone(),
                partial_ec_mul_output_limb_19_col212.clone(),
                partial_ec_mul_output_limb_20_col213.clone(),
                partial_ec_mul_output_limb_21_col214.clone(),
                partial_ec_mul_output_limb_22_col215.clone(),
                partial_ec_mul_output_limb_23_col216.clone(),
                partial_ec_mul_output_limb_24_col217.clone(),
                partial_ec_mul_output_limb_25_col218.clone(),
                partial_ec_mul_output_limb_26_col219.clone(),
                partial_ec_mul_output_limb_27_col220.clone(),
                partial_ec_mul_output_limb_28_col221.clone(),
                partial_ec_mul_output_limb_29_col222.clone(),
                partial_ec_mul_output_limb_30_col223.clone(),
                partial_ec_mul_output_limb_31_col224.clone(),
                partial_ec_mul_output_limb_32_col225.clone(),
                partial_ec_mul_output_limb_33_col226.clone(),
                partial_ec_mul_output_limb_34_col227.clone(),
                partial_ec_mul_output_limb_35_col228.clone(),
                partial_ec_mul_output_limb_36_col229.clone(),
                partial_ec_mul_output_limb_37_col230.clone(),
                partial_ec_mul_output_limb_38_col231.clone(),
                partial_ec_mul_output_limb_39_col232.clone(),
                partial_ec_mul_output_limb_40_col233.clone(),
                partial_ec_mul_output_limb_41_col234.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_aggregator_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
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
    use crate::components::constraints_regression_test_values::PEDERSEN_AGGREGATOR;

    #[test]
    fn pedersen_aggregator_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            range_check_5_4_lookup_elements: relations::RangeCheck_5_4::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_8_lookup_elements: relations::RangeCheck_8::dummy(),
            pedersen_points_table_lookup_elements: relations::PedersenPointsTable::dummy(),
            partial_ec_mul_lookup_elements: relations::PartialEcMul::dummy(),
            pedersen_aggregator_lookup_elements: relations::PedersenAggregator::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, PEDERSEN_AGGREGATOR);
    }
}

// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::read_positive_known_id_num_bits_252::ReadPositiveKnownIdNumBits252;
use crate::components::subroutines::verify_reduced_252::VerifyReduced252;

pub const N_TRACE_COLUMNS: usize = 206;
pub const RELATION_USES_PER_ROW: [RelationUse; 3] = [
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 3,
    },
    RelationUse {
        relation_id: "PartialEcMulWindowBits18",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 2,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub common_lookup_elements: relations::CommonLookupElements,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 5];
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
        let M31_101 = E::F::from(M31::from(101));
        let M31_108 = E::F::from(M31::from(108));
        let M31_115 = E::F::from(M31::from(115));
        let M31_120 = E::F::from(M31::from(120));
        let M31_124 = E::F::from(M31::from(124));
        let M31_135 = E::F::from(M31::from(135));
        let M31_14 = E::F::from(M31::from(14));
        let M31_140 = E::F::from(M31::from(140));
        let M31_141 = E::F::from(M31::from(141));
        let M31_155 = E::F::from(M31::from(155));
        let M31_156 = E::F::from(M31::from(156));
        let M31_160 = E::F::from(M31::from(160));
        let M31_162 = E::F::from(M31::from(162));
        let M31_1621226978 = E::F::from(M31::from(1621226978));
        let M31_1662111297 = E::F::from(M31::from(1662111297));
        let M31_169 = E::F::from(M31::from(169));
        let M31_191 = E::F::from(M31::from(191));
        let M31_199 = E::F::from(M31::from(199));
        let M31_2 = E::F::from(M31::from(2));
        let M31_20 = E::F::from(M31::from(20));
        let M31_202 = E::F::from(M31::from(202));
        let M31_208 = E::F::from(M31::from(208));
        let M31_213 = E::F::from(M31::from(213));
        let M31_22 = E::F::from(M31::from(22));
        let M31_222 = E::F::from(M31::from(222));
        let M31_223 = E::F::from(M31::from(223));
        let M31_225 = E::F::from(M31::from(225));
        let M31_26 = E::F::from(M31::from(26));
        let M31_27 = E::F::from(M31::from(27));
        let M31_28 = E::F::from(M31::from(28));
        let M31_297 = E::F::from(M31::from(297));
        let M31_303 = E::F::from(M31::from(303));
        let M31_314 = E::F::from(M31::from(314));
        let M31_315 = E::F::from(M31::from(315));
        let M31_325 = E::F::from(M31::from(325));
        let M31_334 = E::F::from(M31::from(334));
        let M31_373 = E::F::from(M31::from(373));
        let M31_377 = E::F::from(M31::from(377));
        let M31_379 = E::F::from(M31::from(379));
        let M31_389 = E::F::from(M31::from(389));
        let M31_418 = E::F::from(M31::from(418));
        let M31_420 = E::F::from(M31::from(420));
        let M31_428 = E::F::from(M31::from(428));
        let M31_449 = E::F::from(M31::from(449));
        let M31_464 = E::F::from(M31::from(464));
        let M31_466 = E::F::from(M31::from(466));
        let M31_473 = E::F::from(M31::from(473));
        let M31_480 = E::F::from(M31::from(480));
        let M31_484 = E::F::from(M31::from(484));
        let M31_49 = E::F::from(M31::from(49));
        let M31_497 = E::F::from(M31::from(497));
        let M31_498 = E::F::from(M31::from(498));
        let M31_510 = E::F::from(M31::from(510));
        let M31_512 = E::F::from(M31::from(512));
        let M31_520578465 = E::F::from(M31::from(520578465));
        let M31_54 = E::F::from(M31::from(54));
        let M31_64 = E::F::from(M31::from(64));
        let M31_68 = E::F::from(M31::from(68));
        let M31_72 = E::F::from(M31::from(72));
        let M31_79 = E::F::from(M31::from(79));
        let M31_97 = E::F::from(M31::from(97));
        let M31_98 = E::F::from(M31::from(98));
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
        let value_limb_27_col30 = eval.next_trace_mask();
        let value_limb_0_col31 = eval.next_trace_mask();
        let value_limb_1_col32 = eval.next_trace_mask();
        let value_limb_2_col33 = eval.next_trace_mask();
        let value_limb_3_col34 = eval.next_trace_mask();
        let value_limb_4_col35 = eval.next_trace_mask();
        let value_limb_5_col36 = eval.next_trace_mask();
        let value_limb_6_col37 = eval.next_trace_mask();
        let value_limb_7_col38 = eval.next_trace_mask();
        let value_limb_8_col39 = eval.next_trace_mask();
        let value_limb_9_col40 = eval.next_trace_mask();
        let value_limb_10_col41 = eval.next_trace_mask();
        let value_limb_11_col42 = eval.next_trace_mask();
        let value_limb_12_col43 = eval.next_trace_mask();
        let value_limb_13_col44 = eval.next_trace_mask();
        let value_limb_14_col45 = eval.next_trace_mask();
        let value_limb_15_col46 = eval.next_trace_mask();
        let value_limb_16_col47 = eval.next_trace_mask();
        let value_limb_17_col48 = eval.next_trace_mask();
        let value_limb_18_col49 = eval.next_trace_mask();
        let value_limb_19_col50 = eval.next_trace_mask();
        let value_limb_20_col51 = eval.next_trace_mask();
        let value_limb_21_col52 = eval.next_trace_mask();
        let value_limb_22_col53 = eval.next_trace_mask();
        let value_limb_23_col54 = eval.next_trace_mask();
        let value_limb_24_col55 = eval.next_trace_mask();
        let value_limb_25_col56 = eval.next_trace_mask();
        let value_limb_26_col57 = eval.next_trace_mask();
        let value_limb_27_col58 = eval.next_trace_mask();
        let ms_limb_is_max_col59 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col60 = eval.next_trace_mask();
        let rc_input_1_col61 = eval.next_trace_mask();
        let ms_limb_is_max_col62 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col63 = eval.next_trace_mask();
        let rc_input_1_col64 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_0_col65 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_1_col66 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_2_col67 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_3_col68 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_4_col69 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_5_col70 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_6_col71 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_7_col72 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_8_col73 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_9_col74 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_10_col75 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_11_col76 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_12_col77 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_13_col78 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_14_col79 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_15_col80 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_16_col81 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_17_col82 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_18_col83 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_19_col84 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_20_col85 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_21_col86 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_22_col87 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_23_col88 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_24_col89 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_25_col90 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_26_col91 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_27_col92 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_28_col93 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_29_col94 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_30_col95 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_31_col96 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_32_col97 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_33_col98 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_34_col99 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_35_col100 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_36_col101 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_37_col102 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_38_col103 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_39_col104 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_40_col105 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_41_col106 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_42_col107 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_43_col108 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_44_col109 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_45_col110 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_46_col111 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_47_col112 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_48_col113 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_49_col114 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_50_col115 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_51_col116 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_52_col117 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_53_col118 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_54_col119 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_55_col120 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_56_col121 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_57_col122 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_58_col123 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_59_col124 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_60_col125 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_61_col126 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_62_col127 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_63_col128 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_64_col129 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_65_col130 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_66_col131 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_67_col132 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_68_col133 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_69_col134 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_0_col135 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_1_col136 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_2_col137 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_3_col138 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_4_col139 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_5_col140 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_6_col141 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_7_col142 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_8_col143 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_9_col144 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_10_col145 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_11_col146 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_12_col147 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_13_col148 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_14_col149 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_15_col150 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_16_col151 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_17_col152 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_18_col153 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_19_col154 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_20_col155 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_21_col156 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_22_col157 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_23_col158 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_24_col159 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_25_col160 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_26_col161 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_27_col162 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_28_col163 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_29_col164 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_30_col165 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_31_col166 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_32_col167 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_33_col168 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_34_col169 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_35_col170 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_36_col171 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_37_col172 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_38_col173 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_39_col174 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_40_col175 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_41_col176 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_42_col177 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_43_col178 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_44_col179 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_45_col180 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_46_col181 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_47_col182 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_48_col183 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_49_col184 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_50_col185 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_51_col186 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_52_col187 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_53_col188 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_54_col189 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_55_col190 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_56_col191 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_57_col192 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_58_col193 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_59_col194 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_60_col195 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_61_col196 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_62_col197 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_63_col198 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_64_col199 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_65_col200 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_66_col201 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_67_col202 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_68_col203 = eval.next_trace_mask();
        let partial_ec_mul_window_bits_18_output_limb_69_col204 = eval.next_trace_mask();
        let multiplicity_0 = eval.next_trace_mask();

        ReadPositiveKnownIdNumBits252::evaluate(
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
            value_limb_27_col30.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadPositiveKnownIdNumBits252::evaluate(
            [input_limb_1_col1.clone()],
            value_limb_0_col31.clone(),
            value_limb_1_col32.clone(),
            value_limb_2_col33.clone(),
            value_limb_3_col34.clone(),
            value_limb_4_col35.clone(),
            value_limb_5_col36.clone(),
            value_limb_6_col37.clone(),
            value_limb_7_col38.clone(),
            value_limb_8_col39.clone(),
            value_limb_9_col40.clone(),
            value_limb_10_col41.clone(),
            value_limb_11_col42.clone(),
            value_limb_12_col43.clone(),
            value_limb_13_col44.clone(),
            value_limb_14_col45.clone(),
            value_limb_15_col46.clone(),
            value_limb_16_col47.clone(),
            value_limb_17_col48.clone(),
            value_limb_18_col49.clone(),
            value_limb_19_col50.clone(),
            value_limb_20_col51.clone(),
            value_limb_21_col52.clone(),
            value_limb_22_col53.clone(),
            value_limb_23_col54.clone(),
            value_limb_24_col55.clone(),
            value_limb_25_col56.clone(),
            value_limb_26_col57.clone(),
            value_limb_27_col58.clone(),
            &self.common_lookup_elements,
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
                value_limb_27_col30.clone(),
            ],
            ms_limb_is_max_col59.clone(),
            ms_and_mid_limbs_are_max_col60.clone(),
            rc_input_1_col61.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        VerifyReduced252::evaluate(
            [
                value_limb_0_col31.clone(),
                value_limb_1_col32.clone(),
                value_limb_2_col33.clone(),
                value_limb_3_col34.clone(),
                value_limb_4_col35.clone(),
                value_limb_5_col36.clone(),
                value_limb_6_col37.clone(),
                value_limb_7_col38.clone(),
                value_limb_8_col39.clone(),
                value_limb_9_col40.clone(),
                value_limb_10_col41.clone(),
                value_limb_11_col42.clone(),
                value_limb_12_col43.clone(),
                value_limb_13_col44.clone(),
                value_limb_14_col45.clone(),
                value_limb_15_col46.clone(),
                value_limb_16_col47.clone(),
                value_limb_17_col48.clone(),
                value_limb_18_col49.clone(),
                value_limb_19_col50.clone(),
                value_limb_20_col51.clone(),
                value_limb_21_col52.clone(),
                value_limb_22_col53.clone(),
                value_limb_23_col54.clone(),
                value_limb_24_col55.clone(),
                value_limb_25_col56.clone(),
                value_limb_26_col57.clone(),
                value_limb_27_col58.clone(),
            ],
            ms_limb_is_max_col62.clone(),
            ms_and_mid_limbs_are_max_col63.clone(),
            rc_input_1_col64.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        let partial_ec_mul_window_bits_18_chain_tmp_tmp_94cb4_8 =
            eval.add_intermediate((seq.clone() * M31_2.clone()));
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::one(),
            &[
                M31_1621226978.clone(),
                partial_ec_mul_window_bits_18_chain_tmp_tmp_94cb4_8.clone(),
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
                (value_limb_26_col29.clone() + (value_limb_27_col30.clone() * M31_512.clone())),
                M31_510.clone(),
                M31_315.clone(),
                M31_208.clone(),
                M31_480.clone(),
                M31_418.clone(),
                M31_115.clone(),
                M31_155.clone(),
                M31_54.clone(),
                M31_162.clone(),
                M31_449.clone(),
                M31_428.clone(),
                M31_466.clone(),
                M31_484.clone(),
                M31_169.clone(),
                M31_497.clone(),
                M31_373.clone(),
                M31_98.clone(),
                M31_64.clone(),
                M31_464.clone(),
                M31_498.clone(),
                M31_124.clone(),
                M31_68.clone(),
                M31_379.clone(),
                M31_140.clone(),
                M31_26.clone(),
                M31_22.clone(),
                M31_135.clone(),
                M31_202.clone(),
                M31_156.clone(),
                M31_120.clone(),
                M31_213.clone(),
                M31_389.clone(),
                M31_377.clone(),
                M31_20.clone(),
                M31_325.clone(),
                M31_303.clone(),
                M31_473.clone(),
                M31_334.clone(),
                M31_223.clone(),
                M31_160.clone(),
                M31_225.clone(),
                M31_297.clone(),
                M31_101.clone(),
                M31_420.clone(),
                M31_377.clone(),
                M31_72.clone(),
                M31_191.clone(),
                M31_49.clone(),
                M31_314.clone(),
                M31_27.clone(),
                M31_199.clone(),
                M31_222.clone(),
                M31_79.clone(),
                M31_97.clone(),
                M31_108.clone(),
                M31_141.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::one(),
            &[
                M31_1621226978.clone(),
                partial_ec_mul_window_bits_18_chain_tmp_tmp_94cb4_8.clone(),
                M31_14.clone(),
                partial_ec_mul_window_bits_18_output_limb_0_col65.clone(),
                partial_ec_mul_window_bits_18_output_limb_1_col66.clone(),
                partial_ec_mul_window_bits_18_output_limb_2_col67.clone(),
                partial_ec_mul_window_bits_18_output_limb_3_col68.clone(),
                partial_ec_mul_window_bits_18_output_limb_4_col69.clone(),
                partial_ec_mul_window_bits_18_output_limb_5_col70.clone(),
                partial_ec_mul_window_bits_18_output_limb_6_col71.clone(),
                partial_ec_mul_window_bits_18_output_limb_7_col72.clone(),
                partial_ec_mul_window_bits_18_output_limb_8_col73.clone(),
                partial_ec_mul_window_bits_18_output_limb_9_col74.clone(),
                partial_ec_mul_window_bits_18_output_limb_10_col75.clone(),
                partial_ec_mul_window_bits_18_output_limb_11_col76.clone(),
                partial_ec_mul_window_bits_18_output_limb_12_col77.clone(),
                partial_ec_mul_window_bits_18_output_limb_13_col78.clone(),
                partial_ec_mul_window_bits_18_output_limb_14_col79.clone(),
                partial_ec_mul_window_bits_18_output_limb_15_col80.clone(),
                partial_ec_mul_window_bits_18_output_limb_16_col81.clone(),
                partial_ec_mul_window_bits_18_output_limb_17_col82.clone(),
                partial_ec_mul_window_bits_18_output_limb_18_col83.clone(),
                partial_ec_mul_window_bits_18_output_limb_19_col84.clone(),
                partial_ec_mul_window_bits_18_output_limb_20_col85.clone(),
                partial_ec_mul_window_bits_18_output_limb_21_col86.clone(),
                partial_ec_mul_window_bits_18_output_limb_22_col87.clone(),
                partial_ec_mul_window_bits_18_output_limb_23_col88.clone(),
                partial_ec_mul_window_bits_18_output_limb_24_col89.clone(),
                partial_ec_mul_window_bits_18_output_limb_25_col90.clone(),
                partial_ec_mul_window_bits_18_output_limb_26_col91.clone(),
                partial_ec_mul_window_bits_18_output_limb_27_col92.clone(),
                partial_ec_mul_window_bits_18_output_limb_28_col93.clone(),
                partial_ec_mul_window_bits_18_output_limb_29_col94.clone(),
                partial_ec_mul_window_bits_18_output_limb_30_col95.clone(),
                partial_ec_mul_window_bits_18_output_limb_31_col96.clone(),
                partial_ec_mul_window_bits_18_output_limb_32_col97.clone(),
                partial_ec_mul_window_bits_18_output_limb_33_col98.clone(),
                partial_ec_mul_window_bits_18_output_limb_34_col99.clone(),
                partial_ec_mul_window_bits_18_output_limb_35_col100.clone(),
                partial_ec_mul_window_bits_18_output_limb_36_col101.clone(),
                partial_ec_mul_window_bits_18_output_limb_37_col102.clone(),
                partial_ec_mul_window_bits_18_output_limb_38_col103.clone(),
                partial_ec_mul_window_bits_18_output_limb_39_col104.clone(),
                partial_ec_mul_window_bits_18_output_limb_40_col105.clone(),
                partial_ec_mul_window_bits_18_output_limb_41_col106.clone(),
                partial_ec_mul_window_bits_18_output_limb_42_col107.clone(),
                partial_ec_mul_window_bits_18_output_limb_43_col108.clone(),
                partial_ec_mul_window_bits_18_output_limb_44_col109.clone(),
                partial_ec_mul_window_bits_18_output_limb_45_col110.clone(),
                partial_ec_mul_window_bits_18_output_limb_46_col111.clone(),
                partial_ec_mul_window_bits_18_output_limb_47_col112.clone(),
                partial_ec_mul_window_bits_18_output_limb_48_col113.clone(),
                partial_ec_mul_window_bits_18_output_limb_49_col114.clone(),
                partial_ec_mul_window_bits_18_output_limb_50_col115.clone(),
                partial_ec_mul_window_bits_18_output_limb_51_col116.clone(),
                partial_ec_mul_window_bits_18_output_limb_52_col117.clone(),
                partial_ec_mul_window_bits_18_output_limb_53_col118.clone(),
                partial_ec_mul_window_bits_18_output_limb_54_col119.clone(),
                partial_ec_mul_window_bits_18_output_limb_55_col120.clone(),
                partial_ec_mul_window_bits_18_output_limb_56_col121.clone(),
                partial_ec_mul_window_bits_18_output_limb_57_col122.clone(),
                partial_ec_mul_window_bits_18_output_limb_58_col123.clone(),
                partial_ec_mul_window_bits_18_output_limb_59_col124.clone(),
                partial_ec_mul_window_bits_18_output_limb_60_col125.clone(),
                partial_ec_mul_window_bits_18_output_limb_61_col126.clone(),
                partial_ec_mul_window_bits_18_output_limb_62_col127.clone(),
                partial_ec_mul_window_bits_18_output_limb_63_col128.clone(),
                partial_ec_mul_window_bits_18_output_limb_64_col129.clone(),
                partial_ec_mul_window_bits_18_output_limb_65_col130.clone(),
                partial_ec_mul_window_bits_18_output_limb_66_col131.clone(),
                partial_ec_mul_window_bits_18_output_limb_67_col132.clone(),
                partial_ec_mul_window_bits_18_output_limb_68_col133.clone(),
                partial_ec_mul_window_bits_18_output_limb_69_col134.clone(),
            ],
        ));

        let partial_ec_mul_window_bits_18_chain_id_tmp_94cb4_23 = eval.add_intermediate(
            (partial_ec_mul_window_bits_18_chain_tmp_tmp_94cb4_8.clone() + M31_1.clone()),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::one(),
            &[
                M31_1621226978.clone(),
                partial_ec_mul_window_bits_18_chain_id_tmp_94cb4_23.clone(),
                M31_14.clone(),
                (value_limb_0_col31.clone() + (value_limb_1_col32.clone() * M31_512.clone())),
                (value_limb_2_col33.clone() + (value_limb_3_col34.clone() * M31_512.clone())),
                (value_limb_4_col35.clone() + (value_limb_5_col36.clone() * M31_512.clone())),
                (value_limb_6_col37.clone() + (value_limb_7_col38.clone() * M31_512.clone())),
                (value_limb_8_col39.clone() + (value_limb_9_col40.clone() * M31_512.clone())),
                (value_limb_10_col41.clone() + (value_limb_11_col42.clone() * M31_512.clone())),
                (value_limb_12_col43.clone() + (value_limb_13_col44.clone() * M31_512.clone())),
                (value_limb_14_col45.clone() + (value_limb_15_col46.clone() * M31_512.clone())),
                (value_limb_16_col47.clone() + (value_limb_17_col48.clone() * M31_512.clone())),
                (value_limb_18_col49.clone() + (value_limb_19_col50.clone() * M31_512.clone())),
                (value_limb_20_col51.clone() + (value_limb_21_col52.clone() * M31_512.clone())),
                (value_limb_22_col53.clone() + (value_limb_23_col54.clone() * M31_512.clone())),
                (value_limb_24_col55.clone() + (value_limb_25_col56.clone() * M31_512.clone())),
                (value_limb_26_col57.clone() + (value_limb_27_col58.clone() * M31_512.clone())),
                partial_ec_mul_window_bits_18_output_limb_14_col79.clone(),
                partial_ec_mul_window_bits_18_output_limb_15_col80.clone(),
                partial_ec_mul_window_bits_18_output_limb_16_col81.clone(),
                partial_ec_mul_window_bits_18_output_limb_17_col82.clone(),
                partial_ec_mul_window_bits_18_output_limb_18_col83.clone(),
                partial_ec_mul_window_bits_18_output_limb_19_col84.clone(),
                partial_ec_mul_window_bits_18_output_limb_20_col85.clone(),
                partial_ec_mul_window_bits_18_output_limb_21_col86.clone(),
                partial_ec_mul_window_bits_18_output_limb_22_col87.clone(),
                partial_ec_mul_window_bits_18_output_limb_23_col88.clone(),
                partial_ec_mul_window_bits_18_output_limb_24_col89.clone(),
                partial_ec_mul_window_bits_18_output_limb_25_col90.clone(),
                partial_ec_mul_window_bits_18_output_limb_26_col91.clone(),
                partial_ec_mul_window_bits_18_output_limb_27_col92.clone(),
                partial_ec_mul_window_bits_18_output_limb_28_col93.clone(),
                partial_ec_mul_window_bits_18_output_limb_29_col94.clone(),
                partial_ec_mul_window_bits_18_output_limb_30_col95.clone(),
                partial_ec_mul_window_bits_18_output_limb_31_col96.clone(),
                partial_ec_mul_window_bits_18_output_limb_32_col97.clone(),
                partial_ec_mul_window_bits_18_output_limb_33_col98.clone(),
                partial_ec_mul_window_bits_18_output_limb_34_col99.clone(),
                partial_ec_mul_window_bits_18_output_limb_35_col100.clone(),
                partial_ec_mul_window_bits_18_output_limb_36_col101.clone(),
                partial_ec_mul_window_bits_18_output_limb_37_col102.clone(),
                partial_ec_mul_window_bits_18_output_limb_38_col103.clone(),
                partial_ec_mul_window_bits_18_output_limb_39_col104.clone(),
                partial_ec_mul_window_bits_18_output_limb_40_col105.clone(),
                partial_ec_mul_window_bits_18_output_limb_41_col106.clone(),
                partial_ec_mul_window_bits_18_output_limb_42_col107.clone(),
                partial_ec_mul_window_bits_18_output_limb_43_col108.clone(),
                partial_ec_mul_window_bits_18_output_limb_44_col109.clone(),
                partial_ec_mul_window_bits_18_output_limb_45_col110.clone(),
                partial_ec_mul_window_bits_18_output_limb_46_col111.clone(),
                partial_ec_mul_window_bits_18_output_limb_47_col112.clone(),
                partial_ec_mul_window_bits_18_output_limb_48_col113.clone(),
                partial_ec_mul_window_bits_18_output_limb_49_col114.clone(),
                partial_ec_mul_window_bits_18_output_limb_50_col115.clone(),
                partial_ec_mul_window_bits_18_output_limb_51_col116.clone(),
                partial_ec_mul_window_bits_18_output_limb_52_col117.clone(),
                partial_ec_mul_window_bits_18_output_limb_53_col118.clone(),
                partial_ec_mul_window_bits_18_output_limb_54_col119.clone(),
                partial_ec_mul_window_bits_18_output_limb_55_col120.clone(),
                partial_ec_mul_window_bits_18_output_limb_56_col121.clone(),
                partial_ec_mul_window_bits_18_output_limb_57_col122.clone(),
                partial_ec_mul_window_bits_18_output_limb_58_col123.clone(),
                partial_ec_mul_window_bits_18_output_limb_59_col124.clone(),
                partial_ec_mul_window_bits_18_output_limb_60_col125.clone(),
                partial_ec_mul_window_bits_18_output_limb_61_col126.clone(),
                partial_ec_mul_window_bits_18_output_limb_62_col127.clone(),
                partial_ec_mul_window_bits_18_output_limb_63_col128.clone(),
                partial_ec_mul_window_bits_18_output_limb_64_col129.clone(),
                partial_ec_mul_window_bits_18_output_limb_65_col130.clone(),
                partial_ec_mul_window_bits_18_output_limb_66_col131.clone(),
                partial_ec_mul_window_bits_18_output_limb_67_col132.clone(),
                partial_ec_mul_window_bits_18_output_limb_68_col133.clone(),
                partial_ec_mul_window_bits_18_output_limb_69_col134.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::one(),
            &[
                M31_1621226978.clone(),
                partial_ec_mul_window_bits_18_chain_id_tmp_94cb4_23.clone(),
                M31_28.clone(),
                partial_ec_mul_window_bits_18_output_limb_0_col135.clone(),
                partial_ec_mul_window_bits_18_output_limb_1_col136.clone(),
                partial_ec_mul_window_bits_18_output_limb_2_col137.clone(),
                partial_ec_mul_window_bits_18_output_limb_3_col138.clone(),
                partial_ec_mul_window_bits_18_output_limb_4_col139.clone(),
                partial_ec_mul_window_bits_18_output_limb_5_col140.clone(),
                partial_ec_mul_window_bits_18_output_limb_6_col141.clone(),
                partial_ec_mul_window_bits_18_output_limb_7_col142.clone(),
                partial_ec_mul_window_bits_18_output_limb_8_col143.clone(),
                partial_ec_mul_window_bits_18_output_limb_9_col144.clone(),
                partial_ec_mul_window_bits_18_output_limb_10_col145.clone(),
                partial_ec_mul_window_bits_18_output_limb_11_col146.clone(),
                partial_ec_mul_window_bits_18_output_limb_12_col147.clone(),
                partial_ec_mul_window_bits_18_output_limb_13_col148.clone(),
                partial_ec_mul_window_bits_18_output_limb_14_col149.clone(),
                partial_ec_mul_window_bits_18_output_limb_15_col150.clone(),
                partial_ec_mul_window_bits_18_output_limb_16_col151.clone(),
                partial_ec_mul_window_bits_18_output_limb_17_col152.clone(),
                partial_ec_mul_window_bits_18_output_limb_18_col153.clone(),
                partial_ec_mul_window_bits_18_output_limb_19_col154.clone(),
                partial_ec_mul_window_bits_18_output_limb_20_col155.clone(),
                partial_ec_mul_window_bits_18_output_limb_21_col156.clone(),
                partial_ec_mul_window_bits_18_output_limb_22_col157.clone(),
                partial_ec_mul_window_bits_18_output_limb_23_col158.clone(),
                partial_ec_mul_window_bits_18_output_limb_24_col159.clone(),
                partial_ec_mul_window_bits_18_output_limb_25_col160.clone(),
                partial_ec_mul_window_bits_18_output_limb_26_col161.clone(),
                partial_ec_mul_window_bits_18_output_limb_27_col162.clone(),
                partial_ec_mul_window_bits_18_output_limb_28_col163.clone(),
                partial_ec_mul_window_bits_18_output_limb_29_col164.clone(),
                partial_ec_mul_window_bits_18_output_limb_30_col165.clone(),
                partial_ec_mul_window_bits_18_output_limb_31_col166.clone(),
                partial_ec_mul_window_bits_18_output_limb_32_col167.clone(),
                partial_ec_mul_window_bits_18_output_limb_33_col168.clone(),
                partial_ec_mul_window_bits_18_output_limb_34_col169.clone(),
                partial_ec_mul_window_bits_18_output_limb_35_col170.clone(),
                partial_ec_mul_window_bits_18_output_limb_36_col171.clone(),
                partial_ec_mul_window_bits_18_output_limb_37_col172.clone(),
                partial_ec_mul_window_bits_18_output_limb_38_col173.clone(),
                partial_ec_mul_window_bits_18_output_limb_39_col174.clone(),
                partial_ec_mul_window_bits_18_output_limb_40_col175.clone(),
                partial_ec_mul_window_bits_18_output_limb_41_col176.clone(),
                partial_ec_mul_window_bits_18_output_limb_42_col177.clone(),
                partial_ec_mul_window_bits_18_output_limb_43_col178.clone(),
                partial_ec_mul_window_bits_18_output_limb_44_col179.clone(),
                partial_ec_mul_window_bits_18_output_limb_45_col180.clone(),
                partial_ec_mul_window_bits_18_output_limb_46_col181.clone(),
                partial_ec_mul_window_bits_18_output_limb_47_col182.clone(),
                partial_ec_mul_window_bits_18_output_limb_48_col183.clone(),
                partial_ec_mul_window_bits_18_output_limb_49_col184.clone(),
                partial_ec_mul_window_bits_18_output_limb_50_col185.clone(),
                partial_ec_mul_window_bits_18_output_limb_51_col186.clone(),
                partial_ec_mul_window_bits_18_output_limb_52_col187.clone(),
                partial_ec_mul_window_bits_18_output_limb_53_col188.clone(),
                partial_ec_mul_window_bits_18_output_limb_54_col189.clone(),
                partial_ec_mul_window_bits_18_output_limb_55_col190.clone(),
                partial_ec_mul_window_bits_18_output_limb_56_col191.clone(),
                partial_ec_mul_window_bits_18_output_limb_57_col192.clone(),
                partial_ec_mul_window_bits_18_output_limb_58_col193.clone(),
                partial_ec_mul_window_bits_18_output_limb_59_col194.clone(),
                partial_ec_mul_window_bits_18_output_limb_60_col195.clone(),
                partial_ec_mul_window_bits_18_output_limb_61_col196.clone(),
                partial_ec_mul_window_bits_18_output_limb_62_col197.clone(),
                partial_ec_mul_window_bits_18_output_limb_63_col198.clone(),
                partial_ec_mul_window_bits_18_output_limb_64_col199.clone(),
                partial_ec_mul_window_bits_18_output_limb_65_col200.clone(),
                partial_ec_mul_window_bits_18_output_limb_66_col201.clone(),
                partial_ec_mul_window_bits_18_output_limb_67_col202.clone(),
                partial_ec_mul_window_bits_18_output_limb_68_col203.clone(),
                partial_ec_mul_window_bits_18_output_limb_69_col204.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::one(),
            &[
                M31_1662111297.clone(),
                input_limb_2_col2.clone(),
                partial_ec_mul_window_bits_18_output_limb_14_col149.clone(),
                partial_ec_mul_window_bits_18_output_limb_15_col150.clone(),
                partial_ec_mul_window_bits_18_output_limb_16_col151.clone(),
                partial_ec_mul_window_bits_18_output_limb_17_col152.clone(),
                partial_ec_mul_window_bits_18_output_limb_18_col153.clone(),
                partial_ec_mul_window_bits_18_output_limb_19_col154.clone(),
                partial_ec_mul_window_bits_18_output_limb_20_col155.clone(),
                partial_ec_mul_window_bits_18_output_limb_21_col156.clone(),
                partial_ec_mul_window_bits_18_output_limb_22_col157.clone(),
                partial_ec_mul_window_bits_18_output_limb_23_col158.clone(),
                partial_ec_mul_window_bits_18_output_limb_24_col159.clone(),
                partial_ec_mul_window_bits_18_output_limb_25_col160.clone(),
                partial_ec_mul_window_bits_18_output_limb_26_col161.clone(),
                partial_ec_mul_window_bits_18_output_limb_27_col162.clone(),
                partial_ec_mul_window_bits_18_output_limb_28_col163.clone(),
                partial_ec_mul_window_bits_18_output_limb_29_col164.clone(),
                partial_ec_mul_window_bits_18_output_limb_30_col165.clone(),
                partial_ec_mul_window_bits_18_output_limb_31_col166.clone(),
                partial_ec_mul_window_bits_18_output_limb_32_col167.clone(),
                partial_ec_mul_window_bits_18_output_limb_33_col168.clone(),
                partial_ec_mul_window_bits_18_output_limb_34_col169.clone(),
                partial_ec_mul_window_bits_18_output_limb_35_col170.clone(),
                partial_ec_mul_window_bits_18_output_limb_36_col171.clone(),
                partial_ec_mul_window_bits_18_output_limb_37_col172.clone(),
                partial_ec_mul_window_bits_18_output_limb_38_col173.clone(),
                partial_ec_mul_window_bits_18_output_limb_39_col174.clone(),
                partial_ec_mul_window_bits_18_output_limb_40_col175.clone(),
                partial_ec_mul_window_bits_18_output_limb_41_col176.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity_0),
            &[
                M31_520578465.clone(),
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
    use crate::components::constraints_regression_test_values::PEDERSEN_AGGREGATOR_WINDOW_BITS_18;

    #[test]
    fn pedersen_aggregator_window_bits_18_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        PEDERSEN_AGGREGATOR_WINDOW_BITS_18.assert_debug_eq(&sum);
    }
}

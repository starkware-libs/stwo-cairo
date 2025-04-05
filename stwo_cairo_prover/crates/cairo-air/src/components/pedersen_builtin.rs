use crate::components::prelude::*;
use crate::components::subroutines::mem_verify::MemVerify;
use crate::components::subroutines::read_split::ReadSplit;
use crate::components::subroutines::verify_reduced_252::VerifyReduced252;

pub const N_TRACE_COLUMNS: usize = 359;

pub struct Eval {
    pub claim: Claim,
    pub range_check_5_4_lookup_elements: relations::RangeCheck_5_4,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub range_check_8_lookup_elements: relations::RangeCheck_8,
    pub partial_ec_mul_lookup_elements: relations::PartialEcMul,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
    pub pedersen_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 10];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.pedersen_builtin_segment_start as u64);
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
        let M31_102 = E::F::from(M31::from(102));
        let M31_118 = E::F::from(M31::from(118));
        let M31_125 = E::F::from(M31::from(125));
        let M31_130 = E::F::from(M31::from(130));
        let M31_145 = E::F::from(M31::from(145));
        let M31_15 = E::F::from(M31::from(15));
        let M31_191 = E::F::from(M31::from(191));
        let M31_2 = E::F::from(M31::from(2));
        let M31_202 = E::F::from(M31::from(202));
        let M31_212 = E::F::from(M31::from(212));
        let M31_213 = E::F::from(M31::from(213));
        let M31_221 = E::F::from(M31::from(221));
        let M31_222 = E::F::from(M31::from(222));
        let M31_226 = E::F::from(M31::from(226));
        let M31_227 = E::F::from(M31::from(227));
        let M31_228 = E::F::from(M31::from(228));
        let M31_24 = E::F::from(M31::from(24));
        let M31_251 = E::F::from(M31::from(251));
        let M31_252 = E::F::from(M31::from(252));
        let M31_253 = E::F::from(M31::from(253));
        let M31_259 = E::F::from(M31::from(259));
        let M31_264 = E::F::from(M31::from(264));
        let M31_269 = E::F::from(M31::from(269));
        let M31_27 = E::F::from(M31::from(27));
        let M31_276 = E::F::from(M31::from(276));
        let M31_281 = E::F::from(M31::from(281));
        let M31_3 = E::F::from(M31::from(3));
        let M31_301 = E::F::from(M31::from(301));
        let M31_308 = E::F::from(M31::from(308));
        let M31_31 = E::F::from(M31::from(31));
        let M31_319 = E::F::from(M31::from(319));
        let M31_321 = E::F::from(M31::from(321));
        let M31_330 = E::F::from(M31::from(330));
        let M31_334 = E::F::from(M31::from(334));
        let M31_354 = E::F::from(M31::from(354));
        let M31_3670016 = E::F::from(M31::from(3670016));
        let M31_3670032 = E::F::from(M31::from(3670032));
        let M31_377 = E::F::from(M31::from(377));
        let M31_383 = E::F::from(M31::from(383));
        let M31_385 = E::F::from(M31::from(385));
        let M31_4 = E::F::from(M31::from(4));
        let M31_413 = E::F::from(M31::from(413));
        let M31_419 = E::F::from(M31::from(419));
        let M31_422 = E::F::from(M31::from(422));
        let M31_435 = E::F::from(M31::from(435));
        let M31_458 = E::F::from(M31::from(458));
        let M31_461 = E::F::from(M31::from(461));
        let M31_464 = E::F::from(M31::from(464));
        let M31_471 = E::F::from(M31::from(471));
        let M31_472 = E::F::from(M31::from(472));
        let M31_483 = E::F::from(M31::from(483));
        let M31_50 = E::F::from(M31::from(50));
        let M31_508 = E::F::from(M31::from(508));
        let M31_512 = E::F::from(M31::from(512));
        let M31_7340048 = E::F::from(M31::from(7340048));
        let M31_83 = E::F::from(M31::from(83));
        let M31_92 = E::F::from(M31::from(92));
        let M31_96 = E::F::from(M31::from(96));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let value_limb_0_col0 = eval.next_trace_mask();
        let value_limb_1_col1 = eval.next_trace_mask();
        let value_limb_2_col2 = eval.next_trace_mask();
        let value_limb_3_col3 = eval.next_trace_mask();
        let value_limb_4_col4 = eval.next_trace_mask();
        let value_limb_5_col5 = eval.next_trace_mask();
        let value_limb_6_col6 = eval.next_trace_mask();
        let value_limb_7_col7 = eval.next_trace_mask();
        let value_limb_8_col8 = eval.next_trace_mask();
        let value_limb_9_col9 = eval.next_trace_mask();
        let value_limb_10_col10 = eval.next_trace_mask();
        let value_limb_11_col11 = eval.next_trace_mask();
        let value_limb_12_col12 = eval.next_trace_mask();
        let value_limb_13_col13 = eval.next_trace_mask();
        let value_limb_14_col14 = eval.next_trace_mask();
        let value_limb_15_col15 = eval.next_trace_mask();
        let value_limb_16_col16 = eval.next_trace_mask();
        let value_limb_17_col17 = eval.next_trace_mask();
        let value_limb_18_col18 = eval.next_trace_mask();
        let value_limb_19_col19 = eval.next_trace_mask();
        let value_limb_20_col20 = eval.next_trace_mask();
        let value_limb_21_col21 = eval.next_trace_mask();
        let value_limb_22_col22 = eval.next_trace_mask();
        let value_limb_23_col23 = eval.next_trace_mask();
        let value_limb_24_col24 = eval.next_trace_mask();
        let value_limb_25_col25 = eval.next_trace_mask();
        let value_limb_26_col26 = eval.next_trace_mask();
        let ms_limb_low_col27 = eval.next_trace_mask();
        let ms_limb_high_col28 = eval.next_trace_mask();
        let pedersen_a_id_col29 = eval.next_trace_mask();
        let value_limb_0_col30 = eval.next_trace_mask();
        let value_limb_1_col31 = eval.next_trace_mask();
        let value_limb_2_col32 = eval.next_trace_mask();
        let value_limb_3_col33 = eval.next_trace_mask();
        let value_limb_4_col34 = eval.next_trace_mask();
        let value_limb_5_col35 = eval.next_trace_mask();
        let value_limb_6_col36 = eval.next_trace_mask();
        let value_limb_7_col37 = eval.next_trace_mask();
        let value_limb_8_col38 = eval.next_trace_mask();
        let value_limb_9_col39 = eval.next_trace_mask();
        let value_limb_10_col40 = eval.next_trace_mask();
        let value_limb_11_col41 = eval.next_trace_mask();
        let value_limb_12_col42 = eval.next_trace_mask();
        let value_limb_13_col43 = eval.next_trace_mask();
        let value_limb_14_col44 = eval.next_trace_mask();
        let value_limb_15_col45 = eval.next_trace_mask();
        let value_limb_16_col46 = eval.next_trace_mask();
        let value_limb_17_col47 = eval.next_trace_mask();
        let value_limb_18_col48 = eval.next_trace_mask();
        let value_limb_19_col49 = eval.next_trace_mask();
        let value_limb_20_col50 = eval.next_trace_mask();
        let value_limb_21_col51 = eval.next_trace_mask();
        let value_limb_22_col52 = eval.next_trace_mask();
        let value_limb_23_col53 = eval.next_trace_mask();
        let value_limb_24_col54 = eval.next_trace_mask();
        let value_limb_25_col55 = eval.next_trace_mask();
        let value_limb_26_col56 = eval.next_trace_mask();
        let ms_limb_low_col57 = eval.next_trace_mask();
        let ms_limb_high_col58 = eval.next_trace_mask();
        let pedersen_b_id_col59 = eval.next_trace_mask();
        let ms_limb_is_max_col60 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col61 = eval.next_trace_mask();
        let rc_input_col62 = eval.next_trace_mask();
        let ms_limb_is_max_col63 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col64 = eval.next_trace_mask();
        let rc_input_col65 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_0_col66 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_1_col67 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_2_col68 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_3_col69 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_4_col70 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_5_col71 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_6_col72 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_7_col73 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_8_col74 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_9_col75 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_10_col76 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_11_col77 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_12_col78 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_13_col79 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_14_col80 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_15_col81 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_16_col82 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_17_col83 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_18_col84 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_19_col85 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_20_col86 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_21_col87 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_22_col88 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_23_col89 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_24_col90 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_25_col91 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_26_col92 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_27_col93 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_28_col94 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_29_col95 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_30_col96 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_31_col97 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_32_col98 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_33_col99 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_34_col100 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_35_col101 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_36_col102 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_37_col103 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_38_col104 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_39_col105 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_40_col106 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_41_col107 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_42_col108 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_43_col109 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_44_col110 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_45_col111 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_46_col112 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_47_col113 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_48_col114 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_49_col115 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_50_col116 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_51_col117 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_52_col118 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_53_col119 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_54_col120 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_55_col121 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_56_col122 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_57_col123 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_58_col124 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_59_col125 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_60_col126 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_61_col127 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_62_col128 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_63_col129 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_64_col130 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_65_col131 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_66_col132 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_67_col133 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_68_col134 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_69_col135 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_70_col136 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_71_col137 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_72_col138 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_0_col139 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_1_col140 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_2_col141 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_3_col142 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_4_col143 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_5_col144 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_6_col145 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_7_col146 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_8_col147 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_9_col148 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_10_col149 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_11_col150 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_12_col151 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_13_col152 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_14_col153 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_15_col154 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_16_col155 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_17_col156 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_18_col157 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_19_col158 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_20_col159 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_21_col160 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_22_col161 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_23_col162 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_24_col163 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_25_col164 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_26_col165 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_27_col166 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_28_col167 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_29_col168 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_30_col169 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_31_col170 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_32_col171 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_33_col172 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_34_col173 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_35_col174 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_36_col175 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_37_col176 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_38_col177 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_39_col178 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_40_col179 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_41_col180 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_42_col181 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_43_col182 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_44_col183 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_45_col184 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_46_col185 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_47_col186 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_48_col187 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_49_col188 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_50_col189 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_51_col190 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_52_col191 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_53_col192 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_54_col193 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_55_col194 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_56_col195 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_57_col196 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_58_col197 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_59_col198 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_60_col199 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_61_col200 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_62_col201 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_63_col202 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_64_col203 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_65_col204 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_66_col205 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_67_col206 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_68_col207 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_69_col208 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_70_col209 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_71_col210 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_72_col211 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_0_col212 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_1_col213 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_2_col214 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_3_col215 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_4_col216 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_5_col217 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_6_col218 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_7_col219 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_8_col220 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_9_col221 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_10_col222 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_11_col223 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_12_col224 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_13_col225 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_14_col226 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_15_col227 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_16_col228 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_17_col229 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_18_col230 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_19_col231 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_20_col232 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_21_col233 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_22_col234 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_23_col235 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_24_col236 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_25_col237 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_26_col238 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_27_col239 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_28_col240 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_29_col241 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_30_col242 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_31_col243 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_32_col244 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_33_col245 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_34_col246 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_35_col247 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_36_col248 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_37_col249 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_38_col250 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_39_col251 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_40_col252 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_41_col253 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_42_col254 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_43_col255 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_44_col256 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_45_col257 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_46_col258 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_47_col259 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_48_col260 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_49_col261 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_50_col262 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_51_col263 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_52_col264 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_53_col265 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_54_col266 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_55_col267 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_56_col268 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_57_col269 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_58_col270 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_59_col271 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_60_col272 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_61_col273 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_62_col274 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_63_col275 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_64_col276 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_65_col277 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_66_col278 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_67_col279 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_68_col280 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_69_col281 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_70_col282 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_71_col283 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_72_col284 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_0_col285 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_1_col286 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_2_col287 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_3_col288 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_4_col289 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_5_col290 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_6_col291 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_7_col292 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_8_col293 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_9_col294 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_10_col295 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_11_col296 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_12_col297 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_13_col298 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_14_col299 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_15_col300 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_16_col301 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_17_col302 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_18_col303 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_19_col304 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_20_col305 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_21_col306 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_22_col307 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_23_col308 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_24_col309 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_25_col310 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_26_col311 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_27_col312 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_28_col313 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_29_col314 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_30_col315 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_31_col316 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_32_col317 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_33_col318 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_34_col319 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_35_col320 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_36_col321 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_37_col322 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_38_col323 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_39_col324 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_40_col325 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_41_col326 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_42_col327 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_43_col328 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_44_col329 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_45_col330 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_46_col331 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_47_col332 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_48_col333 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_49_col334 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_50_col335 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_51_col336 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_52_col337 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_53_col338 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_54_col339 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_55_col340 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_56_col341 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_57_col342 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_58_col343 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_59_col344 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_60_col345 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_61_col346 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_62_col347 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_63_col348 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_64_col349 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_65_col350 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_66_col351 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_67_col352 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_68_col353 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_69_col354 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_70_col355 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_71_col356 = eval.next_trace_mask();
        let partial_ec_mul_output_limb_72_col357 = eval.next_trace_mask();
        let pedersen_result_id_col358 = eval.next_trace_mask();

        let instance_addr_tmp_d00c6_0 = eval.add_intermediate(
            ((seq.clone() * M31_3.clone())
                + E::F::from(M31::from(self.claim.pedersen_builtin_segment_start))),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_split_output_tmp_d00c6_6_limb_0, read_split_output_tmp_d00c6_6_limb_1, read_split_output_tmp_d00c6_6_limb_2, read_split_output_tmp_d00c6_6_limb_3, read_split_output_tmp_d00c6_6_limb_4, read_split_output_tmp_d00c6_6_limb_5, read_split_output_tmp_d00c6_6_limb_6, read_split_output_tmp_d00c6_6_limb_7, read_split_output_tmp_d00c6_6_limb_8, read_split_output_tmp_d00c6_6_limb_9, read_split_output_tmp_d00c6_6_limb_10, read_split_output_tmp_d00c6_6_limb_11, read_split_output_tmp_d00c6_6_limb_12, read_split_output_tmp_d00c6_6_limb_13, read_split_output_tmp_d00c6_6_limb_14, read_split_output_tmp_d00c6_6_limb_15, read_split_output_tmp_d00c6_6_limb_16, read_split_output_tmp_d00c6_6_limb_17, read_split_output_tmp_d00c6_6_limb_18, read_split_output_tmp_d00c6_6_limb_19, read_split_output_tmp_d00c6_6_limb_20, read_split_output_tmp_d00c6_6_limb_21, read_split_output_tmp_d00c6_6_limb_22, read_split_output_tmp_d00c6_6_limb_23, read_split_output_tmp_d00c6_6_limb_24, read_split_output_tmp_d00c6_6_limb_25, read_split_output_tmp_d00c6_6_limb_26, read_split_output_tmp_d00c6_6_limb_27, read_split_output_tmp_d00c6_6_limb_28, read_split_output_tmp_d00c6_6_limb_29, read_split_output_tmp_d00c6_6_limb_30, read_split_output_tmp_d00c6_6_limb_31, read_split_output_tmp_d00c6_6_limb_32, read_split_output_tmp_d00c6_6_limb_33, read_split_output_tmp_d00c6_6_limb_34, read_split_output_tmp_d00c6_6_limb_35, read_split_output_tmp_d00c6_6_limb_36, read_split_output_tmp_d00c6_6_limb_37, read_split_output_tmp_d00c6_6_limb_38, read_split_output_tmp_d00c6_6_limb_39, read_split_output_tmp_d00c6_6_limb_40, read_split_output_tmp_d00c6_6_limb_41, read_split_output_tmp_d00c6_6_limb_42, read_split_output_tmp_d00c6_6_limb_43, read_split_output_tmp_d00c6_6_limb_44, read_split_output_tmp_d00c6_6_limb_45, read_split_output_tmp_d00c6_6_limb_46, read_split_output_tmp_d00c6_6_limb_47, read_split_output_tmp_d00c6_6_limb_48, read_split_output_tmp_d00c6_6_limb_49, read_split_output_tmp_d00c6_6_limb_50, read_split_output_tmp_d00c6_6_limb_51, read_split_output_tmp_d00c6_6_limb_52, read_split_output_tmp_d00c6_6_limb_53, read_split_output_tmp_d00c6_6_limb_54, read_split_output_tmp_d00c6_6_limb_55, read_split_output_tmp_d00c6_6_limb_56, read_split_output_tmp_d00c6_6_limb_57, read_split_output_tmp_d00c6_6_limb_58, read_split_output_tmp_d00c6_6_limb_59, read_split_output_tmp_d00c6_6_limb_60, read_split_output_tmp_d00c6_6_limb_61, read_split_output_tmp_d00c6_6_limb_62, read_split_output_tmp_d00c6_6_limb_63, read_split_output_tmp_d00c6_6_limb_64, read_split_output_tmp_d00c6_6_limb_65, read_split_output_tmp_d00c6_6_limb_66, read_split_output_tmp_d00c6_6_limb_67, read_split_output_tmp_d00c6_6_limb_68, read_split_output_tmp_d00c6_6_limb_69, read_split_output_tmp_d00c6_6_limb_70, read_split_output_tmp_d00c6_6_limb_71, read_split_output_tmp_d00c6_6_limb_72, read_split_output_tmp_d00c6_6_limb_73, read_split_output_tmp_d00c6_6_limb_74, read_split_output_tmp_d00c6_6_limb_75, read_split_output_tmp_d00c6_6_limb_76, read_split_output_tmp_d00c6_6_limb_77, read_split_output_tmp_d00c6_6_limb_78, read_split_output_tmp_d00c6_6_limb_79, read_split_output_tmp_d00c6_6_limb_80, read_split_output_tmp_d00c6_6_limb_81, read_split_output_tmp_d00c6_6_limb_82, read_split_output_tmp_d00c6_6_limb_83] =
            ReadSplit::evaluate(
                instance_addr_tmp_d00c6_0.clone(),
                value_limb_0_col0.clone(),
                value_limb_1_col1.clone(),
                value_limb_2_col2.clone(),
                value_limb_3_col3.clone(),
                value_limb_4_col4.clone(),
                value_limb_5_col5.clone(),
                value_limb_6_col6.clone(),
                value_limb_7_col7.clone(),
                value_limb_8_col8.clone(),
                value_limb_9_col9.clone(),
                value_limb_10_col10.clone(),
                value_limb_11_col11.clone(),
                value_limb_12_col12.clone(),
                value_limb_13_col13.clone(),
                value_limb_14_col14.clone(),
                value_limb_15_col15.clone(),
                value_limb_16_col16.clone(),
                value_limb_17_col17.clone(),
                value_limb_18_col18.clone(),
                value_limb_19_col19.clone(),
                value_limb_20_col20.clone(),
                value_limb_21_col21.clone(),
                value_limb_22_col22.clone(),
                value_limb_23_col23.clone(),
                value_limb_24_col24.clone(),
                value_limb_25_col25.clone(),
                value_limb_26_col26.clone(),
                ms_limb_low_col27.clone(),
                ms_limb_high_col28.clone(),
                pedersen_a_id_col29.clone(),
                &mut eval,
                &self.range_check_5_4_lookup_elements,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_split_output_tmp_d00c6_12_limb_0, read_split_output_tmp_d00c6_12_limb_1, read_split_output_tmp_d00c6_12_limb_2, read_split_output_tmp_d00c6_12_limb_3, read_split_output_tmp_d00c6_12_limb_4, read_split_output_tmp_d00c6_12_limb_5, read_split_output_tmp_d00c6_12_limb_6, read_split_output_tmp_d00c6_12_limb_7, read_split_output_tmp_d00c6_12_limb_8, read_split_output_tmp_d00c6_12_limb_9, read_split_output_tmp_d00c6_12_limb_10, read_split_output_tmp_d00c6_12_limb_11, read_split_output_tmp_d00c6_12_limb_12, read_split_output_tmp_d00c6_12_limb_13, read_split_output_tmp_d00c6_12_limb_14, read_split_output_tmp_d00c6_12_limb_15, read_split_output_tmp_d00c6_12_limb_16, read_split_output_tmp_d00c6_12_limb_17, read_split_output_tmp_d00c6_12_limb_18, read_split_output_tmp_d00c6_12_limb_19, read_split_output_tmp_d00c6_12_limb_20, read_split_output_tmp_d00c6_12_limb_21, read_split_output_tmp_d00c6_12_limb_22, read_split_output_tmp_d00c6_12_limb_23, read_split_output_tmp_d00c6_12_limb_24, read_split_output_tmp_d00c6_12_limb_25, read_split_output_tmp_d00c6_12_limb_26, read_split_output_tmp_d00c6_12_limb_27, read_split_output_tmp_d00c6_12_limb_28, read_split_output_tmp_d00c6_12_limb_29, read_split_output_tmp_d00c6_12_limb_30, read_split_output_tmp_d00c6_12_limb_31, read_split_output_tmp_d00c6_12_limb_32, read_split_output_tmp_d00c6_12_limb_33, read_split_output_tmp_d00c6_12_limb_34, read_split_output_tmp_d00c6_12_limb_35, read_split_output_tmp_d00c6_12_limb_36, read_split_output_tmp_d00c6_12_limb_37, read_split_output_tmp_d00c6_12_limb_38, read_split_output_tmp_d00c6_12_limb_39, read_split_output_tmp_d00c6_12_limb_40, read_split_output_tmp_d00c6_12_limb_41, read_split_output_tmp_d00c6_12_limb_42, read_split_output_tmp_d00c6_12_limb_43, read_split_output_tmp_d00c6_12_limb_44, read_split_output_tmp_d00c6_12_limb_45, read_split_output_tmp_d00c6_12_limb_46, read_split_output_tmp_d00c6_12_limb_47, read_split_output_tmp_d00c6_12_limb_48, read_split_output_tmp_d00c6_12_limb_49, read_split_output_tmp_d00c6_12_limb_50, read_split_output_tmp_d00c6_12_limb_51, read_split_output_tmp_d00c6_12_limb_52, read_split_output_tmp_d00c6_12_limb_53, read_split_output_tmp_d00c6_12_limb_54, read_split_output_tmp_d00c6_12_limb_55, read_split_output_tmp_d00c6_12_limb_56, read_split_output_tmp_d00c6_12_limb_57, read_split_output_tmp_d00c6_12_limb_58, read_split_output_tmp_d00c6_12_limb_59, read_split_output_tmp_d00c6_12_limb_60, read_split_output_tmp_d00c6_12_limb_61, read_split_output_tmp_d00c6_12_limb_62, read_split_output_tmp_d00c6_12_limb_63, read_split_output_tmp_d00c6_12_limb_64, read_split_output_tmp_d00c6_12_limb_65, read_split_output_tmp_d00c6_12_limb_66, read_split_output_tmp_d00c6_12_limb_67, read_split_output_tmp_d00c6_12_limb_68, read_split_output_tmp_d00c6_12_limb_69, read_split_output_tmp_d00c6_12_limb_70, read_split_output_tmp_d00c6_12_limb_71, read_split_output_tmp_d00c6_12_limb_72, read_split_output_tmp_d00c6_12_limb_73, read_split_output_tmp_d00c6_12_limb_74, read_split_output_tmp_d00c6_12_limb_75, read_split_output_tmp_d00c6_12_limb_76, read_split_output_tmp_d00c6_12_limb_77, read_split_output_tmp_d00c6_12_limb_78, read_split_output_tmp_d00c6_12_limb_79, read_split_output_tmp_d00c6_12_limb_80, read_split_output_tmp_d00c6_12_limb_81, read_split_output_tmp_d00c6_12_limb_82, read_split_output_tmp_d00c6_12_limb_83] =
            ReadSplit::evaluate(
                (instance_addr_tmp_d00c6_0.clone() + M31_1.clone()),
                value_limb_0_col30.clone(),
                value_limb_1_col31.clone(),
                value_limb_2_col32.clone(),
                value_limb_3_col33.clone(),
                value_limb_4_col34.clone(),
                value_limb_5_col35.clone(),
                value_limb_6_col36.clone(),
                value_limb_7_col37.clone(),
                value_limb_8_col38.clone(),
                value_limb_9_col39.clone(),
                value_limb_10_col40.clone(),
                value_limb_11_col41.clone(),
                value_limb_12_col42.clone(),
                value_limb_13_col43.clone(),
                value_limb_14_col44.clone(),
                value_limb_15_col45.clone(),
                value_limb_16_col46.clone(),
                value_limb_17_col47.clone(),
                value_limb_18_col48.clone(),
                value_limb_19_col49.clone(),
                value_limb_20_col50.clone(),
                value_limb_21_col51.clone(),
                value_limb_22_col52.clone(),
                value_limb_23_col53.clone(),
                value_limb_24_col54.clone(),
                value_limb_25_col55.clone(),
                value_limb_26_col56.clone(),
                ms_limb_low_col57.clone(),
                ms_limb_high_col58.clone(),
                pedersen_b_id_col59.clone(),
                &mut eval,
                &self.range_check_5_4_lookup_elements,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        VerifyReduced252::evaluate(
            [
                value_limb_0_col0.clone(),
                value_limb_1_col1.clone(),
                value_limb_2_col2.clone(),
                value_limb_3_col3.clone(),
                value_limb_4_col4.clone(),
                value_limb_5_col5.clone(),
                value_limb_6_col6.clone(),
                value_limb_7_col7.clone(),
                value_limb_8_col8.clone(),
                value_limb_9_col9.clone(),
                value_limb_10_col10.clone(),
                value_limb_11_col11.clone(),
                value_limb_12_col12.clone(),
                value_limb_13_col13.clone(),
                value_limb_14_col14.clone(),
                value_limb_15_col15.clone(),
                value_limb_16_col16.clone(),
                value_limb_17_col17.clone(),
                value_limb_18_col18.clone(),
                value_limb_19_col19.clone(),
                value_limb_20_col20.clone(),
                value_limb_21_col21.clone(),
                value_limb_22_col22.clone(),
                value_limb_23_col23.clone(),
                value_limb_24_col24.clone(),
                value_limb_25_col25.clone(),
                value_limb_26_col26.clone(),
                read_split_output_tmp_d00c6_6_limb_83.clone(),
            ],
            ms_limb_is_max_col60.clone(),
            ms_and_mid_limbs_are_max_col61.clone(),
            rc_input_col62.clone(),
            &mut eval,
            &self.range_check_8_lookup_elements,
        );
        VerifyReduced252::evaluate(
            [
                value_limb_0_col30.clone(),
                value_limb_1_col31.clone(),
                value_limb_2_col32.clone(),
                value_limb_3_col33.clone(),
                value_limb_4_col34.clone(),
                value_limb_5_col35.clone(),
                value_limb_6_col36.clone(),
                value_limb_7_col37.clone(),
                value_limb_8_col38.clone(),
                value_limb_9_col39.clone(),
                value_limb_10_col40.clone(),
                value_limb_11_col41.clone(),
                value_limb_12_col42.clone(),
                value_limb_13_col43.clone(),
                value_limb_14_col44.clone(),
                value_limb_15_col45.clone(),
                value_limb_16_col46.clone(),
                value_limb_17_col47.clone(),
                value_limb_18_col48.clone(),
                value_limb_19_col49.clone(),
                value_limb_20_col50.clone(),
                value_limb_21_col51.clone(),
                value_limb_22_col52.clone(),
                value_limb_23_col53.clone(),
                value_limb_24_col54.clone(),
                value_limb_25_col55.clone(),
                value_limb_26_col56.clone(),
                read_split_output_tmp_d00c6_12_limb_83.clone(),
            ],
            ms_limb_is_max_col63.clone(),
            ms_and_mid_limbs_are_max_col64.clone(),
            rc_input_col65.clone(),
            &mut eval,
            &self.range_check_8_lookup_elements,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::one(),
            &[
                (seq.clone() * M31_4.clone()),
                M31_0.clone(),
                M31_0.clone(),
                (value_limb_0_col0.clone() + (value_limb_1_col1.clone() * M31_512.clone())),
                (value_limb_2_col2.clone() + (value_limb_3_col3.clone() * M31_512.clone())),
                (value_limb_4_col4.clone() + (value_limb_5_col5.clone() * M31_512.clone())),
                (value_limb_6_col6.clone() + (value_limb_7_col7.clone() * M31_512.clone())),
                (value_limb_8_col8.clone() + (value_limb_9_col9.clone() * M31_512.clone())),
                (value_limb_10_col10.clone() + (value_limb_11_col11.clone() * M31_512.clone())),
                (value_limb_12_col12.clone() + (value_limb_13_col13.clone() * M31_512.clone())),
                (value_limb_14_col14.clone() + (value_limb_15_col15.clone() * M31_512.clone())),
                (value_limb_16_col16.clone() + (value_limb_17_col17.clone() * M31_512.clone())),
                (value_limb_18_col18.clone() + (value_limb_19_col19.clone() * M31_512.clone())),
                (value_limb_20_col20.clone() + (value_limb_21_col21.clone() * M31_512.clone())),
                (value_limb_22_col22.clone() + (value_limb_23_col23.clone() * M31_512.clone())),
                (value_limb_24_col24.clone() + (value_limb_25_col25.clone() * M31_512.clone())),
                (value_limb_26_col26.clone() + (ms_limb_low_col27.clone() * M31_512.clone())),
                M31_435.clone(),
                M31_50.clone(),
                M31_508.clone(),
                M31_83.clone(),
                M31_221.clone(),
                M31_281.clone(),
                M31_377.clone(),
                M31_383.clone(),
                M31_212.clone(),
                M31_264.clone(),
                M31_301.clone(),
                M31_458.clone(),
                M31_130.clone(),
                M31_102.clone(),
                M31_385.clone(),
                M31_269.clone(),
                M31_145.clone(),
                M31_276.clone(),
                M31_483.clone(),
                M31_226.clone(),
                M31_422.clone(),
                M31_253.clone(),
                M31_308.clone(),
                M31_125.clone(),
                M31_472.clone(),
                M31_301.clone(),
                M31_227.clone(),
                M31_27.clone(),
                M31_92.clone(),
                M31_321.clone(),
                M31_252.clone(),
                M31_259.clone(),
                M31_252.clone(),
                M31_413.clone(),
                M31_228.clone(),
                M31_31.clone(),
                M31_24.clone(),
                M31_118.clone(),
                M31_301.clone(),
                M31_202.clone(),
                M31_15.clone(),
                M31_464.clone(),
                M31_334.clone(),
                M31_212.clone(),
                M31_471.clone(),
                M31_461.clone(),
                M31_419.clone(),
                M31_354.clone(),
                M31_96.clone(),
                M31_213.clone(),
                M31_319.clone(),
                M31_191.clone(),
                M31_251.clone(),
                M31_330.clone(),
                M31_15.clone(),
                M31_222.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::one(),
            &[
                partial_ec_mul_output_limb_0_col66.clone(),
                partial_ec_mul_output_limb_1_col67.clone(),
                partial_ec_mul_output_limb_2_col68.clone(),
                partial_ec_mul_output_limb_3_col69.clone(),
                partial_ec_mul_output_limb_4_col70.clone(),
                partial_ec_mul_output_limb_5_col71.clone(),
                partial_ec_mul_output_limb_6_col72.clone(),
                partial_ec_mul_output_limb_7_col73.clone(),
                partial_ec_mul_output_limb_8_col74.clone(),
                partial_ec_mul_output_limb_9_col75.clone(),
                partial_ec_mul_output_limb_10_col76.clone(),
                partial_ec_mul_output_limb_11_col77.clone(),
                partial_ec_mul_output_limb_12_col78.clone(),
                partial_ec_mul_output_limb_13_col79.clone(),
                partial_ec_mul_output_limb_14_col80.clone(),
                partial_ec_mul_output_limb_15_col81.clone(),
                partial_ec_mul_output_limb_16_col82.clone(),
                partial_ec_mul_output_limb_17_col83.clone(),
                partial_ec_mul_output_limb_18_col84.clone(),
                partial_ec_mul_output_limb_19_col85.clone(),
                partial_ec_mul_output_limb_20_col86.clone(),
                partial_ec_mul_output_limb_21_col87.clone(),
                partial_ec_mul_output_limb_22_col88.clone(),
                partial_ec_mul_output_limb_23_col89.clone(),
                partial_ec_mul_output_limb_24_col90.clone(),
                partial_ec_mul_output_limb_25_col91.clone(),
                partial_ec_mul_output_limb_26_col92.clone(),
                partial_ec_mul_output_limb_27_col93.clone(),
                partial_ec_mul_output_limb_28_col94.clone(),
                partial_ec_mul_output_limb_29_col95.clone(),
                partial_ec_mul_output_limb_30_col96.clone(),
                partial_ec_mul_output_limb_31_col97.clone(),
                partial_ec_mul_output_limb_32_col98.clone(),
                partial_ec_mul_output_limb_33_col99.clone(),
                partial_ec_mul_output_limb_34_col100.clone(),
                partial_ec_mul_output_limb_35_col101.clone(),
                partial_ec_mul_output_limb_36_col102.clone(),
                partial_ec_mul_output_limb_37_col103.clone(),
                partial_ec_mul_output_limb_38_col104.clone(),
                partial_ec_mul_output_limb_39_col105.clone(),
                partial_ec_mul_output_limb_40_col106.clone(),
                partial_ec_mul_output_limb_41_col107.clone(),
                partial_ec_mul_output_limb_42_col108.clone(),
                partial_ec_mul_output_limb_43_col109.clone(),
                partial_ec_mul_output_limb_44_col110.clone(),
                partial_ec_mul_output_limb_45_col111.clone(),
                partial_ec_mul_output_limb_46_col112.clone(),
                partial_ec_mul_output_limb_47_col113.clone(),
                partial_ec_mul_output_limb_48_col114.clone(),
                partial_ec_mul_output_limb_49_col115.clone(),
                partial_ec_mul_output_limb_50_col116.clone(),
                partial_ec_mul_output_limb_51_col117.clone(),
                partial_ec_mul_output_limb_52_col118.clone(),
                partial_ec_mul_output_limb_53_col119.clone(),
                partial_ec_mul_output_limb_54_col120.clone(),
                partial_ec_mul_output_limb_55_col121.clone(),
                partial_ec_mul_output_limb_56_col122.clone(),
                partial_ec_mul_output_limb_57_col123.clone(),
                partial_ec_mul_output_limb_58_col124.clone(),
                partial_ec_mul_output_limb_59_col125.clone(),
                partial_ec_mul_output_limb_60_col126.clone(),
                partial_ec_mul_output_limb_61_col127.clone(),
                partial_ec_mul_output_limb_62_col128.clone(),
                partial_ec_mul_output_limb_63_col129.clone(),
                partial_ec_mul_output_limb_64_col130.clone(),
                partial_ec_mul_output_limb_65_col131.clone(),
                partial_ec_mul_output_limb_66_col132.clone(),
                partial_ec_mul_output_limb_67_col133.clone(),
                partial_ec_mul_output_limb_68_col134.clone(),
                partial_ec_mul_output_limb_69_col135.clone(),
                partial_ec_mul_output_limb_70_col136.clone(),
                partial_ec_mul_output_limb_71_col137.clone(),
                partial_ec_mul_output_limb_72_col138.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::one(),
            &[
                ((seq.clone() * M31_4.clone()) + M31_1.clone()),
                M31_0.clone(),
                M31_3670016.clone(),
                ms_limb_high_col28.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                partial_ec_mul_output_limb_17_col83.clone(),
                partial_ec_mul_output_limb_18_col84.clone(),
                partial_ec_mul_output_limb_19_col85.clone(),
                partial_ec_mul_output_limb_20_col86.clone(),
                partial_ec_mul_output_limb_21_col87.clone(),
                partial_ec_mul_output_limb_22_col88.clone(),
                partial_ec_mul_output_limb_23_col89.clone(),
                partial_ec_mul_output_limb_24_col90.clone(),
                partial_ec_mul_output_limb_25_col91.clone(),
                partial_ec_mul_output_limb_26_col92.clone(),
                partial_ec_mul_output_limb_27_col93.clone(),
                partial_ec_mul_output_limb_28_col94.clone(),
                partial_ec_mul_output_limb_29_col95.clone(),
                partial_ec_mul_output_limb_30_col96.clone(),
                partial_ec_mul_output_limb_31_col97.clone(),
                partial_ec_mul_output_limb_32_col98.clone(),
                partial_ec_mul_output_limb_33_col99.clone(),
                partial_ec_mul_output_limb_34_col100.clone(),
                partial_ec_mul_output_limb_35_col101.clone(),
                partial_ec_mul_output_limb_36_col102.clone(),
                partial_ec_mul_output_limb_37_col103.clone(),
                partial_ec_mul_output_limb_38_col104.clone(),
                partial_ec_mul_output_limb_39_col105.clone(),
                partial_ec_mul_output_limb_40_col106.clone(),
                partial_ec_mul_output_limb_41_col107.clone(),
                partial_ec_mul_output_limb_42_col108.clone(),
                partial_ec_mul_output_limb_43_col109.clone(),
                partial_ec_mul_output_limb_44_col110.clone(),
                partial_ec_mul_output_limb_45_col111.clone(),
                partial_ec_mul_output_limb_46_col112.clone(),
                partial_ec_mul_output_limb_47_col113.clone(),
                partial_ec_mul_output_limb_48_col114.clone(),
                partial_ec_mul_output_limb_49_col115.clone(),
                partial_ec_mul_output_limb_50_col116.clone(),
                partial_ec_mul_output_limb_51_col117.clone(),
                partial_ec_mul_output_limb_52_col118.clone(),
                partial_ec_mul_output_limb_53_col119.clone(),
                partial_ec_mul_output_limb_54_col120.clone(),
                partial_ec_mul_output_limb_55_col121.clone(),
                partial_ec_mul_output_limb_56_col122.clone(),
                partial_ec_mul_output_limb_57_col123.clone(),
                partial_ec_mul_output_limb_58_col124.clone(),
                partial_ec_mul_output_limb_59_col125.clone(),
                partial_ec_mul_output_limb_60_col126.clone(),
                partial_ec_mul_output_limb_61_col127.clone(),
                partial_ec_mul_output_limb_62_col128.clone(),
                partial_ec_mul_output_limb_63_col129.clone(),
                partial_ec_mul_output_limb_64_col130.clone(),
                partial_ec_mul_output_limb_65_col131.clone(),
                partial_ec_mul_output_limb_66_col132.clone(),
                partial_ec_mul_output_limb_67_col133.clone(),
                partial_ec_mul_output_limb_68_col134.clone(),
                partial_ec_mul_output_limb_69_col135.clone(),
                partial_ec_mul_output_limb_70_col136.clone(),
                partial_ec_mul_output_limb_71_col137.clone(),
                partial_ec_mul_output_limb_72_col138.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::one(),
            &[
                partial_ec_mul_output_limb_0_col139.clone(),
                partial_ec_mul_output_limb_1_col140.clone(),
                partial_ec_mul_output_limb_2_col141.clone(),
                partial_ec_mul_output_limb_3_col142.clone(),
                partial_ec_mul_output_limb_4_col143.clone(),
                partial_ec_mul_output_limb_5_col144.clone(),
                partial_ec_mul_output_limb_6_col145.clone(),
                partial_ec_mul_output_limb_7_col146.clone(),
                partial_ec_mul_output_limb_8_col147.clone(),
                partial_ec_mul_output_limb_9_col148.clone(),
                partial_ec_mul_output_limb_10_col149.clone(),
                partial_ec_mul_output_limb_11_col150.clone(),
                partial_ec_mul_output_limb_12_col151.clone(),
                partial_ec_mul_output_limb_13_col152.clone(),
                partial_ec_mul_output_limb_14_col153.clone(),
                partial_ec_mul_output_limb_15_col154.clone(),
                partial_ec_mul_output_limb_16_col155.clone(),
                partial_ec_mul_output_limb_17_col156.clone(),
                partial_ec_mul_output_limb_18_col157.clone(),
                partial_ec_mul_output_limb_19_col158.clone(),
                partial_ec_mul_output_limb_20_col159.clone(),
                partial_ec_mul_output_limb_21_col160.clone(),
                partial_ec_mul_output_limb_22_col161.clone(),
                partial_ec_mul_output_limb_23_col162.clone(),
                partial_ec_mul_output_limb_24_col163.clone(),
                partial_ec_mul_output_limb_25_col164.clone(),
                partial_ec_mul_output_limb_26_col165.clone(),
                partial_ec_mul_output_limb_27_col166.clone(),
                partial_ec_mul_output_limb_28_col167.clone(),
                partial_ec_mul_output_limb_29_col168.clone(),
                partial_ec_mul_output_limb_30_col169.clone(),
                partial_ec_mul_output_limb_31_col170.clone(),
                partial_ec_mul_output_limb_32_col171.clone(),
                partial_ec_mul_output_limb_33_col172.clone(),
                partial_ec_mul_output_limb_34_col173.clone(),
                partial_ec_mul_output_limb_35_col174.clone(),
                partial_ec_mul_output_limb_36_col175.clone(),
                partial_ec_mul_output_limb_37_col176.clone(),
                partial_ec_mul_output_limb_38_col177.clone(),
                partial_ec_mul_output_limb_39_col178.clone(),
                partial_ec_mul_output_limb_40_col179.clone(),
                partial_ec_mul_output_limb_41_col180.clone(),
                partial_ec_mul_output_limb_42_col181.clone(),
                partial_ec_mul_output_limb_43_col182.clone(),
                partial_ec_mul_output_limb_44_col183.clone(),
                partial_ec_mul_output_limb_45_col184.clone(),
                partial_ec_mul_output_limb_46_col185.clone(),
                partial_ec_mul_output_limb_47_col186.clone(),
                partial_ec_mul_output_limb_48_col187.clone(),
                partial_ec_mul_output_limb_49_col188.clone(),
                partial_ec_mul_output_limb_50_col189.clone(),
                partial_ec_mul_output_limb_51_col190.clone(),
                partial_ec_mul_output_limb_52_col191.clone(),
                partial_ec_mul_output_limb_53_col192.clone(),
                partial_ec_mul_output_limb_54_col193.clone(),
                partial_ec_mul_output_limb_55_col194.clone(),
                partial_ec_mul_output_limb_56_col195.clone(),
                partial_ec_mul_output_limb_57_col196.clone(),
                partial_ec_mul_output_limb_58_col197.clone(),
                partial_ec_mul_output_limb_59_col198.clone(),
                partial_ec_mul_output_limb_60_col199.clone(),
                partial_ec_mul_output_limb_61_col200.clone(),
                partial_ec_mul_output_limb_62_col201.clone(),
                partial_ec_mul_output_limb_63_col202.clone(),
                partial_ec_mul_output_limb_64_col203.clone(),
                partial_ec_mul_output_limb_65_col204.clone(),
                partial_ec_mul_output_limb_66_col205.clone(),
                partial_ec_mul_output_limb_67_col206.clone(),
                partial_ec_mul_output_limb_68_col207.clone(),
                partial_ec_mul_output_limb_69_col208.clone(),
                partial_ec_mul_output_limb_70_col209.clone(),
                partial_ec_mul_output_limb_71_col210.clone(),
                partial_ec_mul_output_limb_72_col211.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::one(),
            &[
                ((seq.clone() * M31_4.clone()) + M31_2.clone()),
                M31_0.clone(),
                M31_3670032.clone(),
                (value_limb_0_col30.clone() + (value_limb_1_col31.clone() * M31_512.clone())),
                (value_limb_2_col32.clone() + (value_limb_3_col33.clone() * M31_512.clone())),
                (value_limb_4_col34.clone() + (value_limb_5_col35.clone() * M31_512.clone())),
                (value_limb_6_col36.clone() + (value_limb_7_col37.clone() * M31_512.clone())),
                (value_limb_8_col38.clone() + (value_limb_9_col39.clone() * M31_512.clone())),
                (value_limb_10_col40.clone() + (value_limb_11_col41.clone() * M31_512.clone())),
                (value_limb_12_col42.clone() + (value_limb_13_col43.clone() * M31_512.clone())),
                (value_limb_14_col44.clone() + (value_limb_15_col45.clone() * M31_512.clone())),
                (value_limb_16_col46.clone() + (value_limb_17_col47.clone() * M31_512.clone())),
                (value_limb_18_col48.clone() + (value_limb_19_col49.clone() * M31_512.clone())),
                (value_limb_20_col50.clone() + (value_limb_21_col51.clone() * M31_512.clone())),
                (value_limb_22_col52.clone() + (value_limb_23_col53.clone() * M31_512.clone())),
                (value_limb_24_col54.clone() + (value_limb_25_col55.clone() * M31_512.clone())),
                (value_limb_26_col56.clone() + (ms_limb_low_col57.clone() * M31_512.clone())),
                partial_ec_mul_output_limb_17_col156.clone(),
                partial_ec_mul_output_limb_18_col157.clone(),
                partial_ec_mul_output_limb_19_col158.clone(),
                partial_ec_mul_output_limb_20_col159.clone(),
                partial_ec_mul_output_limb_21_col160.clone(),
                partial_ec_mul_output_limb_22_col161.clone(),
                partial_ec_mul_output_limb_23_col162.clone(),
                partial_ec_mul_output_limb_24_col163.clone(),
                partial_ec_mul_output_limb_25_col164.clone(),
                partial_ec_mul_output_limb_26_col165.clone(),
                partial_ec_mul_output_limb_27_col166.clone(),
                partial_ec_mul_output_limb_28_col167.clone(),
                partial_ec_mul_output_limb_29_col168.clone(),
                partial_ec_mul_output_limb_30_col169.clone(),
                partial_ec_mul_output_limb_31_col170.clone(),
                partial_ec_mul_output_limb_32_col171.clone(),
                partial_ec_mul_output_limb_33_col172.clone(),
                partial_ec_mul_output_limb_34_col173.clone(),
                partial_ec_mul_output_limb_35_col174.clone(),
                partial_ec_mul_output_limb_36_col175.clone(),
                partial_ec_mul_output_limb_37_col176.clone(),
                partial_ec_mul_output_limb_38_col177.clone(),
                partial_ec_mul_output_limb_39_col178.clone(),
                partial_ec_mul_output_limb_40_col179.clone(),
                partial_ec_mul_output_limb_41_col180.clone(),
                partial_ec_mul_output_limb_42_col181.clone(),
                partial_ec_mul_output_limb_43_col182.clone(),
                partial_ec_mul_output_limb_44_col183.clone(),
                partial_ec_mul_output_limb_45_col184.clone(),
                partial_ec_mul_output_limb_46_col185.clone(),
                partial_ec_mul_output_limb_47_col186.clone(),
                partial_ec_mul_output_limb_48_col187.clone(),
                partial_ec_mul_output_limb_49_col188.clone(),
                partial_ec_mul_output_limb_50_col189.clone(),
                partial_ec_mul_output_limb_51_col190.clone(),
                partial_ec_mul_output_limb_52_col191.clone(),
                partial_ec_mul_output_limb_53_col192.clone(),
                partial_ec_mul_output_limb_54_col193.clone(),
                partial_ec_mul_output_limb_55_col194.clone(),
                partial_ec_mul_output_limb_56_col195.clone(),
                partial_ec_mul_output_limb_57_col196.clone(),
                partial_ec_mul_output_limb_58_col197.clone(),
                partial_ec_mul_output_limb_59_col198.clone(),
                partial_ec_mul_output_limb_60_col199.clone(),
                partial_ec_mul_output_limb_61_col200.clone(),
                partial_ec_mul_output_limb_62_col201.clone(),
                partial_ec_mul_output_limb_63_col202.clone(),
                partial_ec_mul_output_limb_64_col203.clone(),
                partial_ec_mul_output_limb_65_col204.clone(),
                partial_ec_mul_output_limb_66_col205.clone(),
                partial_ec_mul_output_limb_67_col206.clone(),
                partial_ec_mul_output_limb_68_col207.clone(),
                partial_ec_mul_output_limb_69_col208.clone(),
                partial_ec_mul_output_limb_70_col209.clone(),
                partial_ec_mul_output_limb_71_col210.clone(),
                partial_ec_mul_output_limb_72_col211.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::one(),
            &[
                partial_ec_mul_output_limb_0_col212.clone(),
                partial_ec_mul_output_limb_1_col213.clone(),
                partial_ec_mul_output_limb_2_col214.clone(),
                partial_ec_mul_output_limb_3_col215.clone(),
                partial_ec_mul_output_limb_4_col216.clone(),
                partial_ec_mul_output_limb_5_col217.clone(),
                partial_ec_mul_output_limb_6_col218.clone(),
                partial_ec_mul_output_limb_7_col219.clone(),
                partial_ec_mul_output_limb_8_col220.clone(),
                partial_ec_mul_output_limb_9_col221.clone(),
                partial_ec_mul_output_limb_10_col222.clone(),
                partial_ec_mul_output_limb_11_col223.clone(),
                partial_ec_mul_output_limb_12_col224.clone(),
                partial_ec_mul_output_limb_13_col225.clone(),
                partial_ec_mul_output_limb_14_col226.clone(),
                partial_ec_mul_output_limb_15_col227.clone(),
                partial_ec_mul_output_limb_16_col228.clone(),
                partial_ec_mul_output_limb_17_col229.clone(),
                partial_ec_mul_output_limb_18_col230.clone(),
                partial_ec_mul_output_limb_19_col231.clone(),
                partial_ec_mul_output_limb_20_col232.clone(),
                partial_ec_mul_output_limb_21_col233.clone(),
                partial_ec_mul_output_limb_22_col234.clone(),
                partial_ec_mul_output_limb_23_col235.clone(),
                partial_ec_mul_output_limb_24_col236.clone(),
                partial_ec_mul_output_limb_25_col237.clone(),
                partial_ec_mul_output_limb_26_col238.clone(),
                partial_ec_mul_output_limb_27_col239.clone(),
                partial_ec_mul_output_limb_28_col240.clone(),
                partial_ec_mul_output_limb_29_col241.clone(),
                partial_ec_mul_output_limb_30_col242.clone(),
                partial_ec_mul_output_limb_31_col243.clone(),
                partial_ec_mul_output_limb_32_col244.clone(),
                partial_ec_mul_output_limb_33_col245.clone(),
                partial_ec_mul_output_limb_34_col246.clone(),
                partial_ec_mul_output_limb_35_col247.clone(),
                partial_ec_mul_output_limb_36_col248.clone(),
                partial_ec_mul_output_limb_37_col249.clone(),
                partial_ec_mul_output_limb_38_col250.clone(),
                partial_ec_mul_output_limb_39_col251.clone(),
                partial_ec_mul_output_limb_40_col252.clone(),
                partial_ec_mul_output_limb_41_col253.clone(),
                partial_ec_mul_output_limb_42_col254.clone(),
                partial_ec_mul_output_limb_43_col255.clone(),
                partial_ec_mul_output_limb_44_col256.clone(),
                partial_ec_mul_output_limb_45_col257.clone(),
                partial_ec_mul_output_limb_46_col258.clone(),
                partial_ec_mul_output_limb_47_col259.clone(),
                partial_ec_mul_output_limb_48_col260.clone(),
                partial_ec_mul_output_limb_49_col261.clone(),
                partial_ec_mul_output_limb_50_col262.clone(),
                partial_ec_mul_output_limb_51_col263.clone(),
                partial_ec_mul_output_limb_52_col264.clone(),
                partial_ec_mul_output_limb_53_col265.clone(),
                partial_ec_mul_output_limb_54_col266.clone(),
                partial_ec_mul_output_limb_55_col267.clone(),
                partial_ec_mul_output_limb_56_col268.clone(),
                partial_ec_mul_output_limb_57_col269.clone(),
                partial_ec_mul_output_limb_58_col270.clone(),
                partial_ec_mul_output_limb_59_col271.clone(),
                partial_ec_mul_output_limb_60_col272.clone(),
                partial_ec_mul_output_limb_61_col273.clone(),
                partial_ec_mul_output_limb_62_col274.clone(),
                partial_ec_mul_output_limb_63_col275.clone(),
                partial_ec_mul_output_limb_64_col276.clone(),
                partial_ec_mul_output_limb_65_col277.clone(),
                partial_ec_mul_output_limb_66_col278.clone(),
                partial_ec_mul_output_limb_67_col279.clone(),
                partial_ec_mul_output_limb_68_col280.clone(),
                partial_ec_mul_output_limb_69_col281.clone(),
                partial_ec_mul_output_limb_70_col282.clone(),
                partial_ec_mul_output_limb_71_col283.clone(),
                partial_ec_mul_output_limb_72_col284.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            -E::EF::one(),
            &[
                ((seq.clone() * M31_4.clone()) + M31_3.clone()),
                M31_0.clone(),
                M31_7340048.clone(),
                ms_limb_high_col58.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                partial_ec_mul_output_limb_17_col229.clone(),
                partial_ec_mul_output_limb_18_col230.clone(),
                partial_ec_mul_output_limb_19_col231.clone(),
                partial_ec_mul_output_limb_20_col232.clone(),
                partial_ec_mul_output_limb_21_col233.clone(),
                partial_ec_mul_output_limb_22_col234.clone(),
                partial_ec_mul_output_limb_23_col235.clone(),
                partial_ec_mul_output_limb_24_col236.clone(),
                partial_ec_mul_output_limb_25_col237.clone(),
                partial_ec_mul_output_limb_26_col238.clone(),
                partial_ec_mul_output_limb_27_col239.clone(),
                partial_ec_mul_output_limb_28_col240.clone(),
                partial_ec_mul_output_limb_29_col241.clone(),
                partial_ec_mul_output_limb_30_col242.clone(),
                partial_ec_mul_output_limb_31_col243.clone(),
                partial_ec_mul_output_limb_32_col244.clone(),
                partial_ec_mul_output_limb_33_col245.clone(),
                partial_ec_mul_output_limb_34_col246.clone(),
                partial_ec_mul_output_limb_35_col247.clone(),
                partial_ec_mul_output_limb_36_col248.clone(),
                partial_ec_mul_output_limb_37_col249.clone(),
                partial_ec_mul_output_limb_38_col250.clone(),
                partial_ec_mul_output_limb_39_col251.clone(),
                partial_ec_mul_output_limb_40_col252.clone(),
                partial_ec_mul_output_limb_41_col253.clone(),
                partial_ec_mul_output_limb_42_col254.clone(),
                partial_ec_mul_output_limb_43_col255.clone(),
                partial_ec_mul_output_limb_44_col256.clone(),
                partial_ec_mul_output_limb_45_col257.clone(),
                partial_ec_mul_output_limb_46_col258.clone(),
                partial_ec_mul_output_limb_47_col259.clone(),
                partial_ec_mul_output_limb_48_col260.clone(),
                partial_ec_mul_output_limb_49_col261.clone(),
                partial_ec_mul_output_limb_50_col262.clone(),
                partial_ec_mul_output_limb_51_col263.clone(),
                partial_ec_mul_output_limb_52_col264.clone(),
                partial_ec_mul_output_limb_53_col265.clone(),
                partial_ec_mul_output_limb_54_col266.clone(),
                partial_ec_mul_output_limb_55_col267.clone(),
                partial_ec_mul_output_limb_56_col268.clone(),
                partial_ec_mul_output_limb_57_col269.clone(),
                partial_ec_mul_output_limb_58_col270.clone(),
                partial_ec_mul_output_limb_59_col271.clone(),
                partial_ec_mul_output_limb_60_col272.clone(),
                partial_ec_mul_output_limb_61_col273.clone(),
                partial_ec_mul_output_limb_62_col274.clone(),
                partial_ec_mul_output_limb_63_col275.clone(),
                partial_ec_mul_output_limb_64_col276.clone(),
                partial_ec_mul_output_limb_65_col277.clone(),
                partial_ec_mul_output_limb_66_col278.clone(),
                partial_ec_mul_output_limb_67_col279.clone(),
                partial_ec_mul_output_limb_68_col280.clone(),
                partial_ec_mul_output_limb_69_col281.clone(),
                partial_ec_mul_output_limb_70_col282.clone(),
                partial_ec_mul_output_limb_71_col283.clone(),
                partial_ec_mul_output_limb_72_col284.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.partial_ec_mul_lookup_elements,
            E::EF::one(),
            &[
                partial_ec_mul_output_limb_0_col285.clone(),
                partial_ec_mul_output_limb_1_col286.clone(),
                partial_ec_mul_output_limb_2_col287.clone(),
                partial_ec_mul_output_limb_3_col288.clone(),
                partial_ec_mul_output_limb_4_col289.clone(),
                partial_ec_mul_output_limb_5_col290.clone(),
                partial_ec_mul_output_limb_6_col291.clone(),
                partial_ec_mul_output_limb_7_col292.clone(),
                partial_ec_mul_output_limb_8_col293.clone(),
                partial_ec_mul_output_limb_9_col294.clone(),
                partial_ec_mul_output_limb_10_col295.clone(),
                partial_ec_mul_output_limb_11_col296.clone(),
                partial_ec_mul_output_limb_12_col297.clone(),
                partial_ec_mul_output_limb_13_col298.clone(),
                partial_ec_mul_output_limb_14_col299.clone(),
                partial_ec_mul_output_limb_15_col300.clone(),
                partial_ec_mul_output_limb_16_col301.clone(),
                partial_ec_mul_output_limb_17_col302.clone(),
                partial_ec_mul_output_limb_18_col303.clone(),
                partial_ec_mul_output_limb_19_col304.clone(),
                partial_ec_mul_output_limb_20_col305.clone(),
                partial_ec_mul_output_limb_21_col306.clone(),
                partial_ec_mul_output_limb_22_col307.clone(),
                partial_ec_mul_output_limb_23_col308.clone(),
                partial_ec_mul_output_limb_24_col309.clone(),
                partial_ec_mul_output_limb_25_col310.clone(),
                partial_ec_mul_output_limb_26_col311.clone(),
                partial_ec_mul_output_limb_27_col312.clone(),
                partial_ec_mul_output_limb_28_col313.clone(),
                partial_ec_mul_output_limb_29_col314.clone(),
                partial_ec_mul_output_limb_30_col315.clone(),
                partial_ec_mul_output_limb_31_col316.clone(),
                partial_ec_mul_output_limb_32_col317.clone(),
                partial_ec_mul_output_limb_33_col318.clone(),
                partial_ec_mul_output_limb_34_col319.clone(),
                partial_ec_mul_output_limb_35_col320.clone(),
                partial_ec_mul_output_limb_36_col321.clone(),
                partial_ec_mul_output_limb_37_col322.clone(),
                partial_ec_mul_output_limb_38_col323.clone(),
                partial_ec_mul_output_limb_39_col324.clone(),
                partial_ec_mul_output_limb_40_col325.clone(),
                partial_ec_mul_output_limb_41_col326.clone(),
                partial_ec_mul_output_limb_42_col327.clone(),
                partial_ec_mul_output_limb_43_col328.clone(),
                partial_ec_mul_output_limb_44_col329.clone(),
                partial_ec_mul_output_limb_45_col330.clone(),
                partial_ec_mul_output_limb_46_col331.clone(),
                partial_ec_mul_output_limb_47_col332.clone(),
                partial_ec_mul_output_limb_48_col333.clone(),
                partial_ec_mul_output_limb_49_col334.clone(),
                partial_ec_mul_output_limb_50_col335.clone(),
                partial_ec_mul_output_limb_51_col336.clone(),
                partial_ec_mul_output_limb_52_col337.clone(),
                partial_ec_mul_output_limb_53_col338.clone(),
                partial_ec_mul_output_limb_54_col339.clone(),
                partial_ec_mul_output_limb_55_col340.clone(),
                partial_ec_mul_output_limb_56_col341.clone(),
                partial_ec_mul_output_limb_57_col342.clone(),
                partial_ec_mul_output_limb_58_col343.clone(),
                partial_ec_mul_output_limb_59_col344.clone(),
                partial_ec_mul_output_limb_60_col345.clone(),
                partial_ec_mul_output_limb_61_col346.clone(),
                partial_ec_mul_output_limb_62_col347.clone(),
                partial_ec_mul_output_limb_63_col348.clone(),
                partial_ec_mul_output_limb_64_col349.clone(),
                partial_ec_mul_output_limb_65_col350.clone(),
                partial_ec_mul_output_limb_66_col351.clone(),
                partial_ec_mul_output_limb_67_col352.clone(),
                partial_ec_mul_output_limb_68_col353.clone(),
                partial_ec_mul_output_limb_69_col354.clone(),
                partial_ec_mul_output_limb_70_col355.clone(),
                partial_ec_mul_output_limb_71_col356.clone(),
                partial_ec_mul_output_limb_72_col357.clone(),
            ],
        ));

        MemVerify::evaluate(
            [
                (instance_addr_tmp_d00c6_0.clone() + M31_2.clone()),
                partial_ec_mul_output_limb_17_col302.clone(),
                partial_ec_mul_output_limb_18_col303.clone(),
                partial_ec_mul_output_limb_19_col304.clone(),
                partial_ec_mul_output_limb_20_col305.clone(),
                partial_ec_mul_output_limb_21_col306.clone(),
                partial_ec_mul_output_limb_22_col307.clone(),
                partial_ec_mul_output_limb_23_col308.clone(),
                partial_ec_mul_output_limb_24_col309.clone(),
                partial_ec_mul_output_limb_25_col310.clone(),
                partial_ec_mul_output_limb_26_col311.clone(),
                partial_ec_mul_output_limb_27_col312.clone(),
                partial_ec_mul_output_limb_28_col313.clone(),
                partial_ec_mul_output_limb_29_col314.clone(),
                partial_ec_mul_output_limb_30_col315.clone(),
                partial_ec_mul_output_limb_31_col316.clone(),
                partial_ec_mul_output_limb_32_col317.clone(),
                partial_ec_mul_output_limb_33_col318.clone(),
                partial_ec_mul_output_limb_34_col319.clone(),
                partial_ec_mul_output_limb_35_col320.clone(),
                partial_ec_mul_output_limb_36_col321.clone(),
                partial_ec_mul_output_limb_37_col322.clone(),
                partial_ec_mul_output_limb_38_col323.clone(),
                partial_ec_mul_output_limb_39_col324.clone(),
                partial_ec_mul_output_limb_40_col325.clone(),
                partial_ec_mul_output_limb_41_col326.clone(),
                partial_ec_mul_output_limb_42_col327.clone(),
                partial_ec_mul_output_limb_43_col328.clone(),
                partial_ec_mul_output_limb_44_col329.clone(),
            ],
            pedersen_result_id_col358.clone(),
            &mut eval,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
        );
        eval.finalize_logup_in_pairs();
        eval
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

    use super::*;
    use crate::components::constraints_regression_test_values::PEDERSEN_BUILTIN;

    #[test]
    fn pedersen_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                pedersen_builtin_segment_start: rng.gen::<u32>(),
            },
            range_check_5_4_lookup_elements: relations::RangeCheck_5_4::dummy(),
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_8_lookup_elements: relations::RangeCheck_8::dummy(),
            partial_ec_mul_lookup_elements: relations::PartialEcMul::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, PEDERSEN_BUILTIN);
    }
}

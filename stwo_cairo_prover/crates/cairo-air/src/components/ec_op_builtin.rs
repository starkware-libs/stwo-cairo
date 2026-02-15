// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::mem_verify::MemVerify;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;
use crate::components::subroutines::verify_reduced_252::VerifyReduced252;

pub const N_TRACE_COLUMNS: usize = 273;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 7,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 7,
    },
    RelationUse {
        relation_id: "PartialEcMulGeneric",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_8",
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
    pub ec_op_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 9];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.ec_op_builtin_segment_start as u64);
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
        let M31_183619546 = E::F::from(M31::from(183619546));
        let M31_2 = E::F::from(M31::from(2));
        let M31_252 = E::F::from(M31::from(252));
        let M31_26 = E::F::from(M31::from(26));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_3 = E::F::from(M31::from(3));
        let M31_4 = E::F::from(M31::from(4));
        let M31_5 = E::F::from(M31::from(5));
        let M31_512 = E::F::from(M31::from(512));
        let M31_6 = E::F::from(M31::from(6));
        let M31_7 = E::F::from(M31::from(7));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let p_x_id_col0 = eval.next_trace_mask();
        let p_x_limb_0_col1 = eval.next_trace_mask();
        let p_x_limb_1_col2 = eval.next_trace_mask();
        let p_x_limb_2_col3 = eval.next_trace_mask();
        let p_x_limb_3_col4 = eval.next_trace_mask();
        let p_x_limb_4_col5 = eval.next_trace_mask();
        let p_x_limb_5_col6 = eval.next_trace_mask();
        let p_x_limb_6_col7 = eval.next_trace_mask();
        let p_x_limb_7_col8 = eval.next_trace_mask();
        let p_x_limb_8_col9 = eval.next_trace_mask();
        let p_x_limb_9_col10 = eval.next_trace_mask();
        let p_x_limb_10_col11 = eval.next_trace_mask();
        let p_x_limb_11_col12 = eval.next_trace_mask();
        let p_x_limb_12_col13 = eval.next_trace_mask();
        let p_x_limb_13_col14 = eval.next_trace_mask();
        let p_x_limb_14_col15 = eval.next_trace_mask();
        let p_x_limb_15_col16 = eval.next_trace_mask();
        let p_x_limb_16_col17 = eval.next_trace_mask();
        let p_x_limb_17_col18 = eval.next_trace_mask();
        let p_x_limb_18_col19 = eval.next_trace_mask();
        let p_x_limb_19_col20 = eval.next_trace_mask();
        let p_x_limb_20_col21 = eval.next_trace_mask();
        let p_x_limb_21_col22 = eval.next_trace_mask();
        let p_x_limb_22_col23 = eval.next_trace_mask();
        let p_x_limb_23_col24 = eval.next_trace_mask();
        let p_x_limb_24_col25 = eval.next_trace_mask();
        let p_x_limb_25_col26 = eval.next_trace_mask();
        let p_x_limb_26_col27 = eval.next_trace_mask();
        let p_x_limb_27_col28 = eval.next_trace_mask();
        let p_y_id_col29 = eval.next_trace_mask();
        let p_y_limb_0_col30 = eval.next_trace_mask();
        let p_y_limb_1_col31 = eval.next_trace_mask();
        let p_y_limb_2_col32 = eval.next_trace_mask();
        let p_y_limb_3_col33 = eval.next_trace_mask();
        let p_y_limb_4_col34 = eval.next_trace_mask();
        let p_y_limb_5_col35 = eval.next_trace_mask();
        let p_y_limb_6_col36 = eval.next_trace_mask();
        let p_y_limb_7_col37 = eval.next_trace_mask();
        let p_y_limb_8_col38 = eval.next_trace_mask();
        let p_y_limb_9_col39 = eval.next_trace_mask();
        let p_y_limb_10_col40 = eval.next_trace_mask();
        let p_y_limb_11_col41 = eval.next_trace_mask();
        let p_y_limb_12_col42 = eval.next_trace_mask();
        let p_y_limb_13_col43 = eval.next_trace_mask();
        let p_y_limb_14_col44 = eval.next_trace_mask();
        let p_y_limb_15_col45 = eval.next_trace_mask();
        let p_y_limb_16_col46 = eval.next_trace_mask();
        let p_y_limb_17_col47 = eval.next_trace_mask();
        let p_y_limb_18_col48 = eval.next_trace_mask();
        let p_y_limb_19_col49 = eval.next_trace_mask();
        let p_y_limb_20_col50 = eval.next_trace_mask();
        let p_y_limb_21_col51 = eval.next_trace_mask();
        let p_y_limb_22_col52 = eval.next_trace_mask();
        let p_y_limb_23_col53 = eval.next_trace_mask();
        let p_y_limb_24_col54 = eval.next_trace_mask();
        let p_y_limb_25_col55 = eval.next_trace_mask();
        let p_y_limb_26_col56 = eval.next_trace_mask();
        let p_y_limb_27_col57 = eval.next_trace_mask();
        let q_x_id_col58 = eval.next_trace_mask();
        let q_x_limb_0_col59 = eval.next_trace_mask();
        let q_x_limb_1_col60 = eval.next_trace_mask();
        let q_x_limb_2_col61 = eval.next_trace_mask();
        let q_x_limb_3_col62 = eval.next_trace_mask();
        let q_x_limb_4_col63 = eval.next_trace_mask();
        let q_x_limb_5_col64 = eval.next_trace_mask();
        let q_x_limb_6_col65 = eval.next_trace_mask();
        let q_x_limb_7_col66 = eval.next_trace_mask();
        let q_x_limb_8_col67 = eval.next_trace_mask();
        let q_x_limb_9_col68 = eval.next_trace_mask();
        let q_x_limb_10_col69 = eval.next_trace_mask();
        let q_x_limb_11_col70 = eval.next_trace_mask();
        let q_x_limb_12_col71 = eval.next_trace_mask();
        let q_x_limb_13_col72 = eval.next_trace_mask();
        let q_x_limb_14_col73 = eval.next_trace_mask();
        let q_x_limb_15_col74 = eval.next_trace_mask();
        let q_x_limb_16_col75 = eval.next_trace_mask();
        let q_x_limb_17_col76 = eval.next_trace_mask();
        let q_x_limb_18_col77 = eval.next_trace_mask();
        let q_x_limb_19_col78 = eval.next_trace_mask();
        let q_x_limb_20_col79 = eval.next_trace_mask();
        let q_x_limb_21_col80 = eval.next_trace_mask();
        let q_x_limb_22_col81 = eval.next_trace_mask();
        let q_x_limb_23_col82 = eval.next_trace_mask();
        let q_x_limb_24_col83 = eval.next_trace_mask();
        let q_x_limb_25_col84 = eval.next_trace_mask();
        let q_x_limb_26_col85 = eval.next_trace_mask();
        let q_x_limb_27_col86 = eval.next_trace_mask();
        let q_y_id_col87 = eval.next_trace_mask();
        let q_y_limb_0_col88 = eval.next_trace_mask();
        let q_y_limb_1_col89 = eval.next_trace_mask();
        let q_y_limb_2_col90 = eval.next_trace_mask();
        let q_y_limb_3_col91 = eval.next_trace_mask();
        let q_y_limb_4_col92 = eval.next_trace_mask();
        let q_y_limb_5_col93 = eval.next_trace_mask();
        let q_y_limb_6_col94 = eval.next_trace_mask();
        let q_y_limb_7_col95 = eval.next_trace_mask();
        let q_y_limb_8_col96 = eval.next_trace_mask();
        let q_y_limb_9_col97 = eval.next_trace_mask();
        let q_y_limb_10_col98 = eval.next_trace_mask();
        let q_y_limb_11_col99 = eval.next_trace_mask();
        let q_y_limb_12_col100 = eval.next_trace_mask();
        let q_y_limb_13_col101 = eval.next_trace_mask();
        let q_y_limb_14_col102 = eval.next_trace_mask();
        let q_y_limb_15_col103 = eval.next_trace_mask();
        let q_y_limb_16_col104 = eval.next_trace_mask();
        let q_y_limb_17_col105 = eval.next_trace_mask();
        let q_y_limb_18_col106 = eval.next_trace_mask();
        let q_y_limb_19_col107 = eval.next_trace_mask();
        let q_y_limb_20_col108 = eval.next_trace_mask();
        let q_y_limb_21_col109 = eval.next_trace_mask();
        let q_y_limb_22_col110 = eval.next_trace_mask();
        let q_y_limb_23_col111 = eval.next_trace_mask();
        let q_y_limb_24_col112 = eval.next_trace_mask();
        let q_y_limb_25_col113 = eval.next_trace_mask();
        let q_y_limb_26_col114 = eval.next_trace_mask();
        let q_y_limb_27_col115 = eval.next_trace_mask();
        let m_id_col116 = eval.next_trace_mask();
        let m_limb_0_col117 = eval.next_trace_mask();
        let m_limb_1_col118 = eval.next_trace_mask();
        let m_limb_2_col119 = eval.next_trace_mask();
        let m_limb_3_col120 = eval.next_trace_mask();
        let m_limb_4_col121 = eval.next_trace_mask();
        let m_limb_5_col122 = eval.next_trace_mask();
        let m_limb_6_col123 = eval.next_trace_mask();
        let m_limb_7_col124 = eval.next_trace_mask();
        let m_limb_8_col125 = eval.next_trace_mask();
        let m_limb_9_col126 = eval.next_trace_mask();
        let m_limb_10_col127 = eval.next_trace_mask();
        let m_limb_11_col128 = eval.next_trace_mask();
        let m_limb_12_col129 = eval.next_trace_mask();
        let m_limb_13_col130 = eval.next_trace_mask();
        let m_limb_14_col131 = eval.next_trace_mask();
        let m_limb_15_col132 = eval.next_trace_mask();
        let m_limb_16_col133 = eval.next_trace_mask();
        let m_limb_17_col134 = eval.next_trace_mask();
        let m_limb_18_col135 = eval.next_trace_mask();
        let m_limb_19_col136 = eval.next_trace_mask();
        let m_limb_20_col137 = eval.next_trace_mask();
        let m_limb_21_col138 = eval.next_trace_mask();
        let m_limb_22_col139 = eval.next_trace_mask();
        let m_limb_23_col140 = eval.next_trace_mask();
        let m_limb_24_col141 = eval.next_trace_mask();
        let m_limb_25_col142 = eval.next_trace_mask();
        let m_limb_26_col143 = eval.next_trace_mask();
        let m_limb_27_col144 = eval.next_trace_mask();
        let ms_limb_is_max_col145 = eval.next_trace_mask();
        let ms_and_mid_limbs_are_max_col146 = eval.next_trace_mask();
        let rc_input_col147 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_0_col148 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_1_col149 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_2_col150 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_3_col151 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_4_col152 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_5_col153 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_6_col154 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_7_col155 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_8_col156 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_m_limb_9_col157 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_0_col158 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_1_col159 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_2_col160 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_3_col161 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_4_col162 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_5_col163 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_6_col164 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_7_col165 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_8_col166 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_9_col167 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_10_col168 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_11_col169 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_12_col170 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_13_col171 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_14_col172 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_15_col173 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_16_col174 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_17_col175 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_18_col176 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_19_col177 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_20_col178 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_21_col179 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_22_col180 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_23_col181 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_24_col182 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_25_col183 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_26_col184 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_x_limb_27_col185 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_0_col186 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_1_col187 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_2_col188 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_3_col189 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_4_col190 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_5_col191 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_6_col192 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_7_col193 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_8_col194 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_9_col195 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_10_col196 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_11_col197 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_12_col198 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_13_col199 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_14_col200 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_15_col201 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_16_col202 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_17_col203 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_18_col204 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_19_col205 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_20_col206 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_21_col207 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_22_col208 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_23_col209 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_24_col210 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_25_col211 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_26_col212 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_q_y_limb_27_col213 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_0_col214 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_1_col215 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_2_col216 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_3_col217 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_4_col218 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_5_col219 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_6_col220 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_7_col221 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_8_col222 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_9_col223 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_10_col224 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_11_col225 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_12_col226 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_13_col227 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_14_col228 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_15_col229 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_16_col230 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_17_col231 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_18_col232 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_19_col233 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_20_col234 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_21_col235 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_22_col236 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_23_col237 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_24_col238 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_25_col239 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_26_col240 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_x_limb_27_col241 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_0_col242 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_1_col243 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_2_col244 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_3_col245 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_4_col246 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_5_col247 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_6_col248 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_7_col249 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_8_col250 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_9_col251 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_10_col252 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_11_col253 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_12_col254 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_13_col255 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_14_col256 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_15_col257 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_16_col258 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_17_col259 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_18_col260 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_19_col261 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_20_col262 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_21_col263 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_22_col264 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_23_col265 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_24_col266 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_25_col267 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_26_col268 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_accumulator_y_limb_27_col269 = eval.next_trace_mask();
        let partial_ec_mul_generic_output_counter_col270 = eval.next_trace_mask();
        let res_x_id_col271 = eval.next_trace_mask();
        let res_y_id_col272 = eval.next_trace_mask();

        let instance_addr_tmp_45259_0 = eval.add_intermediate(
            ((seq.clone() * M31_7.clone())
                + E::F::from(M31::from(self.claim.ec_op_builtin_segment_start))),
        );
        ReadPositiveNumBits252::evaluate(
            [instance_addr_tmp_45259_0.clone()],
            p_x_id_col0.clone(),
            p_x_limb_0_col1.clone(),
            p_x_limb_1_col2.clone(),
            p_x_limb_2_col3.clone(),
            p_x_limb_3_col4.clone(),
            p_x_limb_4_col5.clone(),
            p_x_limb_5_col6.clone(),
            p_x_limb_6_col7.clone(),
            p_x_limb_7_col8.clone(),
            p_x_limb_8_col9.clone(),
            p_x_limb_9_col10.clone(),
            p_x_limb_10_col11.clone(),
            p_x_limb_11_col12.clone(),
            p_x_limb_12_col13.clone(),
            p_x_limb_13_col14.clone(),
            p_x_limb_14_col15.clone(),
            p_x_limb_15_col16.clone(),
            p_x_limb_16_col17.clone(),
            p_x_limb_17_col18.clone(),
            p_x_limb_18_col19.clone(),
            p_x_limb_19_col20.clone(),
            p_x_limb_20_col21.clone(),
            p_x_limb_21_col22.clone(),
            p_x_limb_22_col23.clone(),
            p_x_limb_23_col24.clone(),
            p_x_limb_24_col25.clone(),
            p_x_limb_25_col26.clone(),
            p_x_limb_26_col27.clone(),
            p_x_limb_27_col28.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadPositiveNumBits252::evaluate(
            [(instance_addr_tmp_45259_0.clone() + M31_1.clone())],
            p_y_id_col29.clone(),
            p_y_limb_0_col30.clone(),
            p_y_limb_1_col31.clone(),
            p_y_limb_2_col32.clone(),
            p_y_limb_3_col33.clone(),
            p_y_limb_4_col34.clone(),
            p_y_limb_5_col35.clone(),
            p_y_limb_6_col36.clone(),
            p_y_limb_7_col37.clone(),
            p_y_limb_8_col38.clone(),
            p_y_limb_9_col39.clone(),
            p_y_limb_10_col40.clone(),
            p_y_limb_11_col41.clone(),
            p_y_limb_12_col42.clone(),
            p_y_limb_13_col43.clone(),
            p_y_limb_14_col44.clone(),
            p_y_limb_15_col45.clone(),
            p_y_limb_16_col46.clone(),
            p_y_limb_17_col47.clone(),
            p_y_limb_18_col48.clone(),
            p_y_limb_19_col49.clone(),
            p_y_limb_20_col50.clone(),
            p_y_limb_21_col51.clone(),
            p_y_limb_22_col52.clone(),
            p_y_limb_23_col53.clone(),
            p_y_limb_24_col54.clone(),
            p_y_limb_25_col55.clone(),
            p_y_limb_26_col56.clone(),
            p_y_limb_27_col57.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadPositiveNumBits252::evaluate(
            [(instance_addr_tmp_45259_0.clone() + M31_2.clone())],
            q_x_id_col58.clone(),
            q_x_limb_0_col59.clone(),
            q_x_limb_1_col60.clone(),
            q_x_limb_2_col61.clone(),
            q_x_limb_3_col62.clone(),
            q_x_limb_4_col63.clone(),
            q_x_limb_5_col64.clone(),
            q_x_limb_6_col65.clone(),
            q_x_limb_7_col66.clone(),
            q_x_limb_8_col67.clone(),
            q_x_limb_9_col68.clone(),
            q_x_limb_10_col69.clone(),
            q_x_limb_11_col70.clone(),
            q_x_limb_12_col71.clone(),
            q_x_limb_13_col72.clone(),
            q_x_limb_14_col73.clone(),
            q_x_limb_15_col74.clone(),
            q_x_limb_16_col75.clone(),
            q_x_limb_17_col76.clone(),
            q_x_limb_18_col77.clone(),
            q_x_limb_19_col78.clone(),
            q_x_limb_20_col79.clone(),
            q_x_limb_21_col80.clone(),
            q_x_limb_22_col81.clone(),
            q_x_limb_23_col82.clone(),
            q_x_limb_24_col83.clone(),
            q_x_limb_25_col84.clone(),
            q_x_limb_26_col85.clone(),
            q_x_limb_27_col86.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadPositiveNumBits252::evaluate(
            [(instance_addr_tmp_45259_0.clone() + M31_3.clone())],
            q_y_id_col87.clone(),
            q_y_limb_0_col88.clone(),
            q_y_limb_1_col89.clone(),
            q_y_limb_2_col90.clone(),
            q_y_limb_3_col91.clone(),
            q_y_limb_4_col92.clone(),
            q_y_limb_5_col93.clone(),
            q_y_limb_6_col94.clone(),
            q_y_limb_7_col95.clone(),
            q_y_limb_8_col96.clone(),
            q_y_limb_9_col97.clone(),
            q_y_limb_10_col98.clone(),
            q_y_limb_11_col99.clone(),
            q_y_limb_12_col100.clone(),
            q_y_limb_13_col101.clone(),
            q_y_limb_14_col102.clone(),
            q_y_limb_15_col103.clone(),
            q_y_limb_16_col104.clone(),
            q_y_limb_17_col105.clone(),
            q_y_limb_18_col106.clone(),
            q_y_limb_19_col107.clone(),
            q_y_limb_20_col108.clone(),
            q_y_limb_21_col109.clone(),
            q_y_limb_22_col110.clone(),
            q_y_limb_23_col111.clone(),
            q_y_limb_24_col112.clone(),
            q_y_limb_25_col113.clone(),
            q_y_limb_26_col114.clone(),
            q_y_limb_27_col115.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        ReadPositiveNumBits252::evaluate(
            [(instance_addr_tmp_45259_0.clone() + M31_4.clone())],
            m_id_col116.clone(),
            m_limb_0_col117.clone(),
            m_limb_1_col118.clone(),
            m_limb_2_col119.clone(),
            m_limb_3_col120.clone(),
            m_limb_4_col121.clone(),
            m_limb_5_col122.clone(),
            m_limb_6_col123.clone(),
            m_limb_7_col124.clone(),
            m_limb_8_col125.clone(),
            m_limb_9_col126.clone(),
            m_limb_10_col127.clone(),
            m_limb_11_col128.clone(),
            m_limb_12_col129.clone(),
            m_limb_13_col130.clone(),
            m_limb_14_col131.clone(),
            m_limb_15_col132.clone(),
            m_limb_16_col133.clone(),
            m_limb_17_col134.clone(),
            m_limb_18_col135.clone(),
            m_limb_19_col136.clone(),
            m_limb_20_col137.clone(),
            m_limb_21_col138.clone(),
            m_limb_22_col139.clone(),
            m_limb_23_col140.clone(),
            m_limb_24_col141.clone(),
            m_limb_25_col142.clone(),
            m_limb_26_col143.clone(),
            m_limb_27_col144.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        VerifyReduced252::evaluate(
            [
                m_limb_0_col117.clone(),
                m_limb_1_col118.clone(),
                m_limb_2_col119.clone(),
                m_limb_3_col120.clone(),
                m_limb_4_col121.clone(),
                m_limb_5_col122.clone(),
                m_limb_6_col123.clone(),
                m_limb_7_col124.clone(),
                m_limb_8_col125.clone(),
                m_limb_9_col126.clone(),
                m_limb_10_col127.clone(),
                m_limb_11_col128.clone(),
                m_limb_12_col129.clone(),
                m_limb_13_col130.clone(),
                m_limb_14_col131.clone(),
                m_limb_15_col132.clone(),
                m_limb_16_col133.clone(),
                m_limb_17_col134.clone(),
                m_limb_18_col135.clone(),
                m_limb_19_col136.clone(),
                m_limb_20_col137.clone(),
                m_limb_21_col138.clone(),
                m_limb_22_col139.clone(),
                m_limb_23_col140.clone(),
                m_limb_24_col141.clone(),
                m_limb_25_col142.clone(),
                m_limb_26_col143.clone(),
                m_limb_27_col144.clone(),
            ],
            ms_limb_is_max_col145.clone(),
            ms_and_mid_limbs_are_max_col146.clone(),
            rc_input_col147.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(M31_1.clone()),
            &[
                M31_183619546.clone(),
                seq.clone(),
                M31_0.clone(),
                ((m_limb_0_col117.clone() + (m_limb_1_col118.clone() * M31_512.clone()))
                    + (m_limb_2_col119.clone() * M31_262144.clone())),
                ((m_limb_3_col120.clone() + (m_limb_4_col121.clone() * M31_512.clone()))
                    + (m_limb_5_col122.clone() * M31_262144.clone())),
                ((m_limb_6_col123.clone() + (m_limb_7_col124.clone() * M31_512.clone()))
                    + (m_limb_8_col125.clone() * M31_262144.clone())),
                ((m_limb_9_col126.clone() + (m_limb_10_col127.clone() * M31_512.clone()))
                    + (m_limb_11_col128.clone() * M31_262144.clone())),
                ((m_limb_12_col129.clone() + (m_limb_13_col130.clone() * M31_512.clone()))
                    + (m_limb_14_col131.clone() * M31_262144.clone())),
                ((m_limb_15_col132.clone() + (m_limb_16_col133.clone() * M31_512.clone()))
                    + (m_limb_17_col134.clone() * M31_262144.clone())),
                ((m_limb_18_col135.clone() + (m_limb_19_col136.clone() * M31_512.clone()))
                    + (m_limb_20_col137.clone() * M31_262144.clone())),
                ((m_limb_21_col138.clone() + (m_limb_22_col139.clone() * M31_512.clone()))
                    + (m_limb_23_col140.clone() * M31_262144.clone())),
                ((m_limb_24_col141.clone() + (m_limb_25_col142.clone() * M31_512.clone()))
                    + (m_limb_26_col143.clone() * M31_262144.clone())),
                m_limb_27_col144.clone(),
                q_x_limb_0_col59.clone(),
                q_x_limb_1_col60.clone(),
                q_x_limb_2_col61.clone(),
                q_x_limb_3_col62.clone(),
                q_x_limb_4_col63.clone(),
                q_x_limb_5_col64.clone(),
                q_x_limb_6_col65.clone(),
                q_x_limb_7_col66.clone(),
                q_x_limb_8_col67.clone(),
                q_x_limb_9_col68.clone(),
                q_x_limb_10_col69.clone(),
                q_x_limb_11_col70.clone(),
                q_x_limb_12_col71.clone(),
                q_x_limb_13_col72.clone(),
                q_x_limb_14_col73.clone(),
                q_x_limb_15_col74.clone(),
                q_x_limb_16_col75.clone(),
                q_x_limb_17_col76.clone(),
                q_x_limb_18_col77.clone(),
                q_x_limb_19_col78.clone(),
                q_x_limb_20_col79.clone(),
                q_x_limb_21_col80.clone(),
                q_x_limb_22_col81.clone(),
                q_x_limb_23_col82.clone(),
                q_x_limb_24_col83.clone(),
                q_x_limb_25_col84.clone(),
                q_x_limb_26_col85.clone(),
                q_x_limb_27_col86.clone(),
                q_y_limb_0_col88.clone(),
                q_y_limb_1_col89.clone(),
                q_y_limb_2_col90.clone(),
                q_y_limb_3_col91.clone(),
                q_y_limb_4_col92.clone(),
                q_y_limb_5_col93.clone(),
                q_y_limb_6_col94.clone(),
                q_y_limb_7_col95.clone(),
                q_y_limb_8_col96.clone(),
                q_y_limb_9_col97.clone(),
                q_y_limb_10_col98.clone(),
                q_y_limb_11_col99.clone(),
                q_y_limb_12_col100.clone(),
                q_y_limb_13_col101.clone(),
                q_y_limb_14_col102.clone(),
                q_y_limb_15_col103.clone(),
                q_y_limb_16_col104.clone(),
                q_y_limb_17_col105.clone(),
                q_y_limb_18_col106.clone(),
                q_y_limb_19_col107.clone(),
                q_y_limb_20_col108.clone(),
                q_y_limb_21_col109.clone(),
                q_y_limb_22_col110.clone(),
                q_y_limb_23_col111.clone(),
                q_y_limb_24_col112.clone(),
                q_y_limb_25_col113.clone(),
                q_y_limb_26_col114.clone(),
                q_y_limb_27_col115.clone(),
                p_x_limb_0_col1.clone(),
                p_x_limb_1_col2.clone(),
                p_x_limb_2_col3.clone(),
                p_x_limb_3_col4.clone(),
                p_x_limb_4_col5.clone(),
                p_x_limb_5_col6.clone(),
                p_x_limb_6_col7.clone(),
                p_x_limb_7_col8.clone(),
                p_x_limb_8_col9.clone(),
                p_x_limb_9_col10.clone(),
                p_x_limb_10_col11.clone(),
                p_x_limb_11_col12.clone(),
                p_x_limb_12_col13.clone(),
                p_x_limb_13_col14.clone(),
                p_x_limb_14_col15.clone(),
                p_x_limb_15_col16.clone(),
                p_x_limb_16_col17.clone(),
                p_x_limb_17_col18.clone(),
                p_x_limb_18_col19.clone(),
                p_x_limb_19_col20.clone(),
                p_x_limb_20_col21.clone(),
                p_x_limb_21_col22.clone(),
                p_x_limb_22_col23.clone(),
                p_x_limb_23_col24.clone(),
                p_x_limb_24_col25.clone(),
                p_x_limb_25_col26.clone(),
                p_x_limb_26_col27.clone(),
                p_x_limb_27_col28.clone(),
                p_y_limb_0_col30.clone(),
                p_y_limb_1_col31.clone(),
                p_y_limb_2_col32.clone(),
                p_y_limb_3_col33.clone(),
                p_y_limb_4_col34.clone(),
                p_y_limb_5_col35.clone(),
                p_y_limb_6_col36.clone(),
                p_y_limb_7_col37.clone(),
                p_y_limb_8_col38.clone(),
                p_y_limb_9_col39.clone(),
                p_y_limb_10_col40.clone(),
                p_y_limb_11_col41.clone(),
                p_y_limb_12_col42.clone(),
                p_y_limb_13_col43.clone(),
                p_y_limb_14_col44.clone(),
                p_y_limb_15_col45.clone(),
                p_y_limb_16_col46.clone(),
                p_y_limb_17_col47.clone(),
                p_y_limb_18_col48.clone(),
                p_y_limb_19_col49.clone(),
                p_y_limb_20_col50.clone(),
                p_y_limb_21_col51.clone(),
                p_y_limb_22_col52.clone(),
                p_y_limb_23_col53.clone(),
                p_y_limb_24_col54.clone(),
                p_y_limb_25_col55.clone(),
                p_y_limb_26_col56.clone(),
                p_y_limb_27_col57.clone(),
                M31_26.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            E::EF::from(M31_1.clone()),
            &[
                M31_183619546.clone(),
                seq.clone(),
                M31_252.clone(),
                partial_ec_mul_generic_output_m_limb_0_col148.clone(),
                partial_ec_mul_generic_output_m_limb_1_col149.clone(),
                partial_ec_mul_generic_output_m_limb_2_col150.clone(),
                partial_ec_mul_generic_output_m_limb_3_col151.clone(),
                partial_ec_mul_generic_output_m_limb_4_col152.clone(),
                partial_ec_mul_generic_output_m_limb_5_col153.clone(),
                partial_ec_mul_generic_output_m_limb_6_col154.clone(),
                partial_ec_mul_generic_output_m_limb_7_col155.clone(),
                partial_ec_mul_generic_output_m_limb_8_col156.clone(),
                partial_ec_mul_generic_output_m_limb_9_col157.clone(),
                partial_ec_mul_generic_output_q_x_limb_0_col158.clone(),
                partial_ec_mul_generic_output_q_x_limb_1_col159.clone(),
                partial_ec_mul_generic_output_q_x_limb_2_col160.clone(),
                partial_ec_mul_generic_output_q_x_limb_3_col161.clone(),
                partial_ec_mul_generic_output_q_x_limb_4_col162.clone(),
                partial_ec_mul_generic_output_q_x_limb_5_col163.clone(),
                partial_ec_mul_generic_output_q_x_limb_6_col164.clone(),
                partial_ec_mul_generic_output_q_x_limb_7_col165.clone(),
                partial_ec_mul_generic_output_q_x_limb_8_col166.clone(),
                partial_ec_mul_generic_output_q_x_limb_9_col167.clone(),
                partial_ec_mul_generic_output_q_x_limb_10_col168.clone(),
                partial_ec_mul_generic_output_q_x_limb_11_col169.clone(),
                partial_ec_mul_generic_output_q_x_limb_12_col170.clone(),
                partial_ec_mul_generic_output_q_x_limb_13_col171.clone(),
                partial_ec_mul_generic_output_q_x_limb_14_col172.clone(),
                partial_ec_mul_generic_output_q_x_limb_15_col173.clone(),
                partial_ec_mul_generic_output_q_x_limb_16_col174.clone(),
                partial_ec_mul_generic_output_q_x_limb_17_col175.clone(),
                partial_ec_mul_generic_output_q_x_limb_18_col176.clone(),
                partial_ec_mul_generic_output_q_x_limb_19_col177.clone(),
                partial_ec_mul_generic_output_q_x_limb_20_col178.clone(),
                partial_ec_mul_generic_output_q_x_limb_21_col179.clone(),
                partial_ec_mul_generic_output_q_x_limb_22_col180.clone(),
                partial_ec_mul_generic_output_q_x_limb_23_col181.clone(),
                partial_ec_mul_generic_output_q_x_limb_24_col182.clone(),
                partial_ec_mul_generic_output_q_x_limb_25_col183.clone(),
                partial_ec_mul_generic_output_q_x_limb_26_col184.clone(),
                partial_ec_mul_generic_output_q_x_limb_27_col185.clone(),
                partial_ec_mul_generic_output_q_y_limb_0_col186.clone(),
                partial_ec_mul_generic_output_q_y_limb_1_col187.clone(),
                partial_ec_mul_generic_output_q_y_limb_2_col188.clone(),
                partial_ec_mul_generic_output_q_y_limb_3_col189.clone(),
                partial_ec_mul_generic_output_q_y_limb_4_col190.clone(),
                partial_ec_mul_generic_output_q_y_limb_5_col191.clone(),
                partial_ec_mul_generic_output_q_y_limb_6_col192.clone(),
                partial_ec_mul_generic_output_q_y_limb_7_col193.clone(),
                partial_ec_mul_generic_output_q_y_limb_8_col194.clone(),
                partial_ec_mul_generic_output_q_y_limb_9_col195.clone(),
                partial_ec_mul_generic_output_q_y_limb_10_col196.clone(),
                partial_ec_mul_generic_output_q_y_limb_11_col197.clone(),
                partial_ec_mul_generic_output_q_y_limb_12_col198.clone(),
                partial_ec_mul_generic_output_q_y_limb_13_col199.clone(),
                partial_ec_mul_generic_output_q_y_limb_14_col200.clone(),
                partial_ec_mul_generic_output_q_y_limb_15_col201.clone(),
                partial_ec_mul_generic_output_q_y_limb_16_col202.clone(),
                partial_ec_mul_generic_output_q_y_limb_17_col203.clone(),
                partial_ec_mul_generic_output_q_y_limb_18_col204.clone(),
                partial_ec_mul_generic_output_q_y_limb_19_col205.clone(),
                partial_ec_mul_generic_output_q_y_limb_20_col206.clone(),
                partial_ec_mul_generic_output_q_y_limb_21_col207.clone(),
                partial_ec_mul_generic_output_q_y_limb_22_col208.clone(),
                partial_ec_mul_generic_output_q_y_limb_23_col209.clone(),
                partial_ec_mul_generic_output_q_y_limb_24_col210.clone(),
                partial_ec_mul_generic_output_q_y_limb_25_col211.clone(),
                partial_ec_mul_generic_output_q_y_limb_26_col212.clone(),
                partial_ec_mul_generic_output_q_y_limb_27_col213.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_0_col214.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_1_col215.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_2_col216.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_3_col217.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_4_col218.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_5_col219.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_6_col220.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_7_col221.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_8_col222.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_9_col223.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_10_col224.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_11_col225.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_12_col226.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_13_col227.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_14_col228.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_15_col229.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_16_col230.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_17_col231.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_18_col232.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_19_col233.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_20_col234.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_21_col235.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_22_col236.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_23_col237.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_24_col238.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_25_col239.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_26_col240.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_27_col241.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_0_col242.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_1_col243.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_2_col244.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_3_col245.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_4_col246.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_5_col247.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_6_col248.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_7_col249.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_8_col250.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_9_col251.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_10_col252.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_11_col253.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_12_col254.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_13_col255.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_14_col256.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_15_col257.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_16_col258.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_17_col259.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_18_col260.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_19_col261.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_20_col262.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_21_col263.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_22_col264.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_23_col265.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_24_col266.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_25_col267.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_26_col268.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_27_col269.clone(),
                partial_ec_mul_generic_output_counter_col270.clone(),
            ],
        ));

        // final m is zero.
        eval.add_constraint(partial_ec_mul_generic_output_m_limb_0_col148.clone());
        MemVerify::evaluate(
            [
                (instance_addr_tmp_45259_0.clone() + M31_5.clone()),
                partial_ec_mul_generic_output_accumulator_x_limb_0_col214.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_1_col215.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_2_col216.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_3_col217.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_4_col218.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_5_col219.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_6_col220.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_7_col221.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_8_col222.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_9_col223.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_10_col224.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_11_col225.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_12_col226.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_13_col227.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_14_col228.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_15_col229.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_16_col230.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_17_col231.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_18_col232.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_19_col233.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_20_col234.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_21_col235.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_22_col236.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_23_col237.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_24_col238.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_25_col239.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_26_col240.clone(),
                partial_ec_mul_generic_output_accumulator_x_limb_27_col241.clone(),
            ],
            res_x_id_col271.clone(),
            &self.common_lookup_elements,
            &mut eval,
        );
        MemVerify::evaluate(
            [
                (instance_addr_tmp_45259_0.clone() + M31_6.clone()),
                partial_ec_mul_generic_output_accumulator_y_limb_0_col242.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_1_col243.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_2_col244.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_3_col245.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_4_col246.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_5_col247.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_6_col248.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_7_col249.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_8_col250.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_9_col251.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_10_col252.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_11_col253.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_12_col254.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_13_col255.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_14_col256.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_15_col257.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_16_col258.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_17_col259.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_18_col260.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_19_col261.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_20_col262.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_21_col263.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_22_col264.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_23_col265.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_24_col266.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_25_col267.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_26_col268.clone(),
                partial_ec_mul_generic_output_accumulator_y_limb_27_col269.clone(),
            ],
            res_y_id_col272.clone(),
            &self.common_lookup_elements,
            &mut eval,
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
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

    use super::*;
    use crate::components::constraints_regression_test_values::EC_OP_BUILTIN;

    #[test]
    fn ec_op_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                ec_op_builtin_segment_start: rng.gen::<u32>(),
            },
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        EC_OP_BUILTIN.assert_debug_eq(&sum);
    }
}

// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::felt_252_unpack_from_27::Felt252UnpackFrom27;
use crate::components::subroutines::poseidon_hades_permutation::PoseidonHadesPermutation;
use crate::components::subroutines::read_positive_known_id_num_bits_252::ReadPositiveKnownIdNumBits252;

pub const N_TRACE_COLUMNS: usize = 342;
pub const RELATION_USES_PER_ROW: [RelationUse; 8] = [
    RelationUse {
        relation_id: "Cube252",
        uses: 2,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 6,
    },
    RelationUse {
        relation_id: "Poseidon3PartialRoundsChain",
        uses: 1,
    },
    RelationUse {
        relation_id: "PoseidonFullRoundChain",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck252Width27",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_3_3_3_3_3",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_4_4",
        uses: 3,
    },
    RelationUse {
        relation_id: "RangeCheck_4_4_4_4",
        uses: 6,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain,
    pub range_check_252_width_27_lookup_elements: relations::RangeCheck252Width27,
    pub cube_252_lookup_elements: relations::Cube252,
    pub range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3,
    pub range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4,
    pub range_check_4_4_lookup_elements: relations::RangeCheck_4_4,
    pub poseidon_3_partial_rounds_chain_lookup_elements: relations::Poseidon3PartialRoundsChain,
    pub poseidon_aggregator_lookup_elements: relations::PoseidonAggregator,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 14];
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
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let input_limb_0_col0 = eval.next_trace_mask();
        let input_limb_1_col1 = eval.next_trace_mask();
        let input_limb_2_col2 = eval.next_trace_mask();
        let input_limb_3_col3 = eval.next_trace_mask();
        let input_limb_4_col4 = eval.next_trace_mask();
        let input_limb_5_col5 = eval.next_trace_mask();
        let value_limb_0_col6 = eval.next_trace_mask();
        let value_limb_1_col7 = eval.next_trace_mask();
        let value_limb_2_col8 = eval.next_trace_mask();
        let value_limb_3_col9 = eval.next_trace_mask();
        let value_limb_4_col10 = eval.next_trace_mask();
        let value_limb_5_col11 = eval.next_trace_mask();
        let value_limb_6_col12 = eval.next_trace_mask();
        let value_limb_7_col13 = eval.next_trace_mask();
        let value_limb_8_col14 = eval.next_trace_mask();
        let value_limb_9_col15 = eval.next_trace_mask();
        let value_limb_10_col16 = eval.next_trace_mask();
        let value_limb_11_col17 = eval.next_trace_mask();
        let value_limb_12_col18 = eval.next_trace_mask();
        let value_limb_13_col19 = eval.next_trace_mask();
        let value_limb_14_col20 = eval.next_trace_mask();
        let value_limb_15_col21 = eval.next_trace_mask();
        let value_limb_16_col22 = eval.next_trace_mask();
        let value_limb_17_col23 = eval.next_trace_mask();
        let value_limb_18_col24 = eval.next_trace_mask();
        let value_limb_19_col25 = eval.next_trace_mask();
        let value_limb_20_col26 = eval.next_trace_mask();
        let value_limb_21_col27 = eval.next_trace_mask();
        let value_limb_22_col28 = eval.next_trace_mask();
        let value_limb_23_col29 = eval.next_trace_mask();
        let value_limb_24_col30 = eval.next_trace_mask();
        let value_limb_25_col31 = eval.next_trace_mask();
        let value_limb_26_col32 = eval.next_trace_mask();
        let value_limb_27_col33 = eval.next_trace_mask();
        let value_limb_0_col34 = eval.next_trace_mask();
        let value_limb_1_col35 = eval.next_trace_mask();
        let value_limb_2_col36 = eval.next_trace_mask();
        let value_limb_3_col37 = eval.next_trace_mask();
        let value_limb_4_col38 = eval.next_trace_mask();
        let value_limb_5_col39 = eval.next_trace_mask();
        let value_limb_6_col40 = eval.next_trace_mask();
        let value_limb_7_col41 = eval.next_trace_mask();
        let value_limb_8_col42 = eval.next_trace_mask();
        let value_limb_9_col43 = eval.next_trace_mask();
        let value_limb_10_col44 = eval.next_trace_mask();
        let value_limb_11_col45 = eval.next_trace_mask();
        let value_limb_12_col46 = eval.next_trace_mask();
        let value_limb_13_col47 = eval.next_trace_mask();
        let value_limb_14_col48 = eval.next_trace_mask();
        let value_limb_15_col49 = eval.next_trace_mask();
        let value_limb_16_col50 = eval.next_trace_mask();
        let value_limb_17_col51 = eval.next_trace_mask();
        let value_limb_18_col52 = eval.next_trace_mask();
        let value_limb_19_col53 = eval.next_trace_mask();
        let value_limb_20_col54 = eval.next_trace_mask();
        let value_limb_21_col55 = eval.next_trace_mask();
        let value_limb_22_col56 = eval.next_trace_mask();
        let value_limb_23_col57 = eval.next_trace_mask();
        let value_limb_24_col58 = eval.next_trace_mask();
        let value_limb_25_col59 = eval.next_trace_mask();
        let value_limb_26_col60 = eval.next_trace_mask();
        let value_limb_27_col61 = eval.next_trace_mask();
        let value_limb_0_col62 = eval.next_trace_mask();
        let value_limb_1_col63 = eval.next_trace_mask();
        let value_limb_2_col64 = eval.next_trace_mask();
        let value_limb_3_col65 = eval.next_trace_mask();
        let value_limb_4_col66 = eval.next_trace_mask();
        let value_limb_5_col67 = eval.next_trace_mask();
        let value_limb_6_col68 = eval.next_trace_mask();
        let value_limb_7_col69 = eval.next_trace_mask();
        let value_limb_8_col70 = eval.next_trace_mask();
        let value_limb_9_col71 = eval.next_trace_mask();
        let value_limb_10_col72 = eval.next_trace_mask();
        let value_limb_11_col73 = eval.next_trace_mask();
        let value_limb_12_col74 = eval.next_trace_mask();
        let value_limb_13_col75 = eval.next_trace_mask();
        let value_limb_14_col76 = eval.next_trace_mask();
        let value_limb_15_col77 = eval.next_trace_mask();
        let value_limb_16_col78 = eval.next_trace_mask();
        let value_limb_17_col79 = eval.next_trace_mask();
        let value_limb_18_col80 = eval.next_trace_mask();
        let value_limb_19_col81 = eval.next_trace_mask();
        let value_limb_20_col82 = eval.next_trace_mask();
        let value_limb_21_col83 = eval.next_trace_mask();
        let value_limb_22_col84 = eval.next_trace_mask();
        let value_limb_23_col85 = eval.next_trace_mask();
        let value_limb_24_col86 = eval.next_trace_mask();
        let value_limb_25_col87 = eval.next_trace_mask();
        let value_limb_26_col88 = eval.next_trace_mask();
        let value_limb_27_col89 = eval.next_trace_mask();
        let combination_limb_0_col90 = eval.next_trace_mask();
        let combination_limb_1_col91 = eval.next_trace_mask();
        let combination_limb_2_col92 = eval.next_trace_mask();
        let combination_limb_3_col93 = eval.next_trace_mask();
        let combination_limb_4_col94 = eval.next_trace_mask();
        let combination_limb_5_col95 = eval.next_trace_mask();
        let combination_limb_6_col96 = eval.next_trace_mask();
        let combination_limb_7_col97 = eval.next_trace_mask();
        let combination_limb_8_col98 = eval.next_trace_mask();
        let combination_limb_9_col99 = eval.next_trace_mask();
        let p_coef_col100 = eval.next_trace_mask();
        let combination_limb_0_col101 = eval.next_trace_mask();
        let combination_limb_1_col102 = eval.next_trace_mask();
        let combination_limb_2_col103 = eval.next_trace_mask();
        let combination_limb_3_col104 = eval.next_trace_mask();
        let combination_limb_4_col105 = eval.next_trace_mask();
        let combination_limb_5_col106 = eval.next_trace_mask();
        let combination_limb_6_col107 = eval.next_trace_mask();
        let combination_limb_7_col108 = eval.next_trace_mask();
        let combination_limb_8_col109 = eval.next_trace_mask();
        let combination_limb_9_col110 = eval.next_trace_mask();
        let p_coef_col111 = eval.next_trace_mask();
        let combination_limb_0_col112 = eval.next_trace_mask();
        let combination_limb_1_col113 = eval.next_trace_mask();
        let combination_limb_2_col114 = eval.next_trace_mask();
        let combination_limb_3_col115 = eval.next_trace_mask();
        let combination_limb_4_col116 = eval.next_trace_mask();
        let combination_limb_5_col117 = eval.next_trace_mask();
        let combination_limb_6_col118 = eval.next_trace_mask();
        let combination_limb_7_col119 = eval.next_trace_mask();
        let combination_limb_8_col120 = eval.next_trace_mask();
        let combination_limb_9_col121 = eval.next_trace_mask();
        let p_coef_col122 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_0_col123 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_1_col124 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_2_col125 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_3_col126 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_4_col127 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_5_col128 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_6_col129 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_7_col130 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_8_col131 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_9_col132 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_10_col133 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_11_col134 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_12_col135 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_13_col136 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_14_col137 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_15_col138 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_16_col139 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_17_col140 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_18_col141 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_19_col142 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_20_col143 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_21_col144 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_22_col145 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_23_col146 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_24_col147 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_25_col148 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_26_col149 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_27_col150 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_28_col151 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_29_col152 = eval.next_trace_mask();
        let cube_252_output_limb_0_col153 = eval.next_trace_mask();
        let cube_252_output_limb_1_col154 = eval.next_trace_mask();
        let cube_252_output_limb_2_col155 = eval.next_trace_mask();
        let cube_252_output_limb_3_col156 = eval.next_trace_mask();
        let cube_252_output_limb_4_col157 = eval.next_trace_mask();
        let cube_252_output_limb_5_col158 = eval.next_trace_mask();
        let cube_252_output_limb_6_col159 = eval.next_trace_mask();
        let cube_252_output_limb_7_col160 = eval.next_trace_mask();
        let cube_252_output_limb_8_col161 = eval.next_trace_mask();
        let cube_252_output_limb_9_col162 = eval.next_trace_mask();
        let combination_limb_0_col163 = eval.next_trace_mask();
        let combination_limb_1_col164 = eval.next_trace_mask();
        let combination_limb_2_col165 = eval.next_trace_mask();
        let combination_limb_3_col166 = eval.next_trace_mask();
        let combination_limb_4_col167 = eval.next_trace_mask();
        let combination_limb_5_col168 = eval.next_trace_mask();
        let combination_limb_6_col169 = eval.next_trace_mask();
        let combination_limb_7_col170 = eval.next_trace_mask();
        let combination_limb_8_col171 = eval.next_trace_mask();
        let combination_limb_9_col172 = eval.next_trace_mask();
        let p_coef_col173 = eval.next_trace_mask();
        let cube_252_output_limb_0_col174 = eval.next_trace_mask();
        let cube_252_output_limb_1_col175 = eval.next_trace_mask();
        let cube_252_output_limb_2_col176 = eval.next_trace_mask();
        let cube_252_output_limb_3_col177 = eval.next_trace_mask();
        let cube_252_output_limb_4_col178 = eval.next_trace_mask();
        let cube_252_output_limb_5_col179 = eval.next_trace_mask();
        let cube_252_output_limb_6_col180 = eval.next_trace_mask();
        let cube_252_output_limb_7_col181 = eval.next_trace_mask();
        let cube_252_output_limb_8_col182 = eval.next_trace_mask();
        let cube_252_output_limb_9_col183 = eval.next_trace_mask();
        let combination_limb_0_col184 = eval.next_trace_mask();
        let combination_limb_1_col185 = eval.next_trace_mask();
        let combination_limb_2_col186 = eval.next_trace_mask();
        let combination_limb_3_col187 = eval.next_trace_mask();
        let combination_limb_4_col188 = eval.next_trace_mask();
        let combination_limb_5_col189 = eval.next_trace_mask();
        let combination_limb_6_col190 = eval.next_trace_mask();
        let combination_limb_7_col191 = eval.next_trace_mask();
        let combination_limb_8_col192 = eval.next_trace_mask();
        let combination_limb_9_col193 = eval.next_trace_mask();
        let p_coef_col194 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_0_col195 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_1_col196 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_2_col197 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_3_col198 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_4_col199 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_5_col200 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_6_col201 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_7_col202 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_8_col203 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_9_col204 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_10_col205 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_11_col206 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_12_col207 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_13_col208 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_14_col209 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_15_col210 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_16_col211 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_17_col212 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_18_col213 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_19_col214 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_20_col215 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_21_col216 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_22_col217 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_23_col218 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_24_col219 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_25_col220 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_26_col221 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_27_col222 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_28_col223 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_29_col224 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_30_col225 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_31_col226 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_32_col227 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_33_col228 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_34_col229 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_35_col230 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_36_col231 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_37_col232 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_38_col233 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_39_col234 = eval.next_trace_mask();
        let combination_limb_0_col235 = eval.next_trace_mask();
        let combination_limb_1_col236 = eval.next_trace_mask();
        let combination_limb_2_col237 = eval.next_trace_mask();
        let combination_limb_3_col238 = eval.next_trace_mask();
        let combination_limb_4_col239 = eval.next_trace_mask();
        let combination_limb_5_col240 = eval.next_trace_mask();
        let combination_limb_6_col241 = eval.next_trace_mask();
        let combination_limb_7_col242 = eval.next_trace_mask();
        let combination_limb_8_col243 = eval.next_trace_mask();
        let combination_limb_9_col244 = eval.next_trace_mask();
        let p_coef_col245 = eval.next_trace_mask();
        let combination_limb_0_col246 = eval.next_trace_mask();
        let combination_limb_1_col247 = eval.next_trace_mask();
        let combination_limb_2_col248 = eval.next_trace_mask();
        let combination_limb_3_col249 = eval.next_trace_mask();
        let combination_limb_4_col250 = eval.next_trace_mask();
        let combination_limb_5_col251 = eval.next_trace_mask();
        let combination_limb_6_col252 = eval.next_trace_mask();
        let combination_limb_7_col253 = eval.next_trace_mask();
        let combination_limb_8_col254 = eval.next_trace_mask();
        let combination_limb_9_col255 = eval.next_trace_mask();
        let p_coef_col256 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_0_col257 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_1_col258 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_2_col259 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_3_col260 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_4_col261 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_5_col262 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_6_col263 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_7_col264 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_8_col265 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_9_col266 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_10_col267 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_11_col268 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_12_col269 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_13_col270 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_14_col271 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_15_col272 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_16_col273 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_17_col274 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_18_col275 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_19_col276 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_20_col277 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_21_col278 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_22_col279 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_23_col280 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_24_col281 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_25_col282 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_26_col283 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_27_col284 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_28_col285 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_29_col286 = eval.next_trace_mask();
        let unpacked_limb_0_col287 = eval.next_trace_mask();
        let unpacked_limb_1_col288 = eval.next_trace_mask();
        let unpacked_limb_3_col289 = eval.next_trace_mask();
        let unpacked_limb_4_col290 = eval.next_trace_mask();
        let unpacked_limb_6_col291 = eval.next_trace_mask();
        let unpacked_limb_7_col292 = eval.next_trace_mask();
        let unpacked_limb_9_col293 = eval.next_trace_mask();
        let unpacked_limb_10_col294 = eval.next_trace_mask();
        let unpacked_limb_12_col295 = eval.next_trace_mask();
        let unpacked_limb_13_col296 = eval.next_trace_mask();
        let unpacked_limb_15_col297 = eval.next_trace_mask();
        let unpacked_limb_16_col298 = eval.next_trace_mask();
        let unpacked_limb_18_col299 = eval.next_trace_mask();
        let unpacked_limb_19_col300 = eval.next_trace_mask();
        let unpacked_limb_21_col301 = eval.next_trace_mask();
        let unpacked_limb_22_col302 = eval.next_trace_mask();
        let unpacked_limb_24_col303 = eval.next_trace_mask();
        let unpacked_limb_25_col304 = eval.next_trace_mask();
        let unpacked_limb_0_col305 = eval.next_trace_mask();
        let unpacked_limb_1_col306 = eval.next_trace_mask();
        let unpacked_limb_3_col307 = eval.next_trace_mask();
        let unpacked_limb_4_col308 = eval.next_trace_mask();
        let unpacked_limb_6_col309 = eval.next_trace_mask();
        let unpacked_limb_7_col310 = eval.next_trace_mask();
        let unpacked_limb_9_col311 = eval.next_trace_mask();
        let unpacked_limb_10_col312 = eval.next_trace_mask();
        let unpacked_limb_12_col313 = eval.next_trace_mask();
        let unpacked_limb_13_col314 = eval.next_trace_mask();
        let unpacked_limb_15_col315 = eval.next_trace_mask();
        let unpacked_limb_16_col316 = eval.next_trace_mask();
        let unpacked_limb_18_col317 = eval.next_trace_mask();
        let unpacked_limb_19_col318 = eval.next_trace_mask();
        let unpacked_limb_21_col319 = eval.next_trace_mask();
        let unpacked_limb_22_col320 = eval.next_trace_mask();
        let unpacked_limb_24_col321 = eval.next_trace_mask();
        let unpacked_limb_25_col322 = eval.next_trace_mask();
        let unpacked_limb_0_col323 = eval.next_trace_mask();
        let unpacked_limb_1_col324 = eval.next_trace_mask();
        let unpacked_limb_3_col325 = eval.next_trace_mask();
        let unpacked_limb_4_col326 = eval.next_trace_mask();
        let unpacked_limb_6_col327 = eval.next_trace_mask();
        let unpacked_limb_7_col328 = eval.next_trace_mask();
        let unpacked_limb_9_col329 = eval.next_trace_mask();
        let unpacked_limb_10_col330 = eval.next_trace_mask();
        let unpacked_limb_12_col331 = eval.next_trace_mask();
        let unpacked_limb_13_col332 = eval.next_trace_mask();
        let unpacked_limb_15_col333 = eval.next_trace_mask();
        let unpacked_limb_16_col334 = eval.next_trace_mask();
        let unpacked_limb_18_col335 = eval.next_trace_mask();
        let unpacked_limb_19_col336 = eval.next_trace_mask();
        let unpacked_limb_21_col337 = eval.next_trace_mask();
        let unpacked_limb_22_col338 = eval.next_trace_mask();
        let unpacked_limb_24_col339 = eval.next_trace_mask();
        let unpacked_limb_25_col340 = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();

        ReadPositiveKnownIdNumBits252::evaluate(
            [input_limb_0_col0.clone()],
            value_limb_0_col6.clone(),
            value_limb_1_col7.clone(),
            value_limb_2_col8.clone(),
            value_limb_3_col9.clone(),
            value_limb_4_col10.clone(),
            value_limb_5_col11.clone(),
            value_limb_6_col12.clone(),
            value_limb_7_col13.clone(),
            value_limb_8_col14.clone(),
            value_limb_9_col15.clone(),
            value_limb_10_col16.clone(),
            value_limb_11_col17.clone(),
            value_limb_12_col18.clone(),
            value_limb_13_col19.clone(),
            value_limb_14_col20.clone(),
            value_limb_15_col21.clone(),
            value_limb_16_col22.clone(),
            value_limb_17_col23.clone(),
            value_limb_18_col24.clone(),
            value_limb_19_col25.clone(),
            value_limb_20_col26.clone(),
            value_limb_21_col27.clone(),
            value_limb_22_col28.clone(),
            value_limb_23_col29.clone(),
            value_limb_24_col30.clone(),
            value_limb_25_col31.clone(),
            value_limb_26_col32.clone(),
            value_limb_27_col33.clone(),
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        let packed_input_state_0_tmp_34cc4_2_limb_0 = eval.add_intermediate(
            ((value_limb_0_col6.clone() + (value_limb_1_col7.clone() * M31_512.clone()))
                + (value_limb_2_col8.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_1 = eval.add_intermediate(
            ((value_limb_3_col9.clone() + (value_limb_4_col10.clone() * M31_512.clone()))
                + (value_limb_5_col11.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_2 = eval.add_intermediate(
            ((value_limb_6_col12.clone() + (value_limb_7_col13.clone() * M31_512.clone()))
                + (value_limb_8_col14.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_3 = eval.add_intermediate(
            ((value_limb_9_col15.clone() + (value_limb_10_col16.clone() * M31_512.clone()))
                + (value_limb_11_col17.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_4 = eval.add_intermediate(
            ((value_limb_12_col18.clone() + (value_limb_13_col19.clone() * M31_512.clone()))
                + (value_limb_14_col20.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_5 = eval.add_intermediate(
            ((value_limb_15_col21.clone() + (value_limb_16_col22.clone() * M31_512.clone()))
                + (value_limb_17_col23.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_6 = eval.add_intermediate(
            ((value_limb_18_col24.clone() + (value_limb_19_col25.clone() * M31_512.clone()))
                + (value_limb_20_col26.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_7 = eval.add_intermediate(
            ((value_limb_21_col27.clone() + (value_limb_22_col28.clone() * M31_512.clone()))
                + (value_limb_23_col29.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_34cc4_2_limb_8 = eval.add_intermediate(
            ((value_limb_24_col30.clone() + (value_limb_25_col31.clone() * M31_512.clone()))
                + (value_limb_26_col32.clone() * M31_262144.clone())),
        );
        ReadPositiveKnownIdNumBits252::evaluate(
            [input_limb_1_col1.clone()],
            value_limb_0_col34.clone(),
            value_limb_1_col35.clone(),
            value_limb_2_col36.clone(),
            value_limb_3_col37.clone(),
            value_limb_4_col38.clone(),
            value_limb_5_col39.clone(),
            value_limb_6_col40.clone(),
            value_limb_7_col41.clone(),
            value_limb_8_col42.clone(),
            value_limb_9_col43.clone(),
            value_limb_10_col44.clone(),
            value_limb_11_col45.clone(),
            value_limb_12_col46.clone(),
            value_limb_13_col47.clone(),
            value_limb_14_col48.clone(),
            value_limb_15_col49.clone(),
            value_limb_16_col50.clone(),
            value_limb_17_col51.clone(),
            value_limb_18_col52.clone(),
            value_limb_19_col53.clone(),
            value_limb_20_col54.clone(),
            value_limb_21_col55.clone(),
            value_limb_22_col56.clone(),
            value_limb_23_col57.clone(),
            value_limb_24_col58.clone(),
            value_limb_25_col59.clone(),
            value_limb_26_col60.clone(),
            value_limb_27_col61.clone(),
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        let packed_input_state_1_tmp_34cc4_5_limb_0 = eval.add_intermediate(
            ((value_limb_0_col34.clone() + (value_limb_1_col35.clone() * M31_512.clone()))
                + (value_limb_2_col36.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_1 = eval.add_intermediate(
            ((value_limb_3_col37.clone() + (value_limb_4_col38.clone() * M31_512.clone()))
                + (value_limb_5_col39.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_2 = eval.add_intermediate(
            ((value_limb_6_col40.clone() + (value_limb_7_col41.clone() * M31_512.clone()))
                + (value_limb_8_col42.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_3 = eval.add_intermediate(
            ((value_limb_9_col43.clone() + (value_limb_10_col44.clone() * M31_512.clone()))
                + (value_limb_11_col45.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_4 = eval.add_intermediate(
            ((value_limb_12_col46.clone() + (value_limb_13_col47.clone() * M31_512.clone()))
                + (value_limb_14_col48.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_5 = eval.add_intermediate(
            ((value_limb_15_col49.clone() + (value_limb_16_col50.clone() * M31_512.clone()))
                + (value_limb_17_col51.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_6 = eval.add_intermediate(
            ((value_limb_18_col52.clone() + (value_limb_19_col53.clone() * M31_512.clone()))
                + (value_limb_20_col54.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_7 = eval.add_intermediate(
            ((value_limb_21_col55.clone() + (value_limb_22_col56.clone() * M31_512.clone()))
                + (value_limb_23_col57.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_34cc4_5_limb_8 = eval.add_intermediate(
            ((value_limb_24_col58.clone() + (value_limb_25_col59.clone() * M31_512.clone()))
                + (value_limb_26_col60.clone() * M31_262144.clone())),
        );
        ReadPositiveKnownIdNumBits252::evaluate(
            [input_limb_2_col2.clone()],
            value_limb_0_col62.clone(),
            value_limb_1_col63.clone(),
            value_limb_2_col64.clone(),
            value_limb_3_col65.clone(),
            value_limb_4_col66.clone(),
            value_limb_5_col67.clone(),
            value_limb_6_col68.clone(),
            value_limb_7_col69.clone(),
            value_limb_8_col70.clone(),
            value_limb_9_col71.clone(),
            value_limb_10_col72.clone(),
            value_limb_11_col73.clone(),
            value_limb_12_col74.clone(),
            value_limb_13_col75.clone(),
            value_limb_14_col76.clone(),
            value_limb_15_col77.clone(),
            value_limb_16_col78.clone(),
            value_limb_17_col79.clone(),
            value_limb_18_col80.clone(),
            value_limb_19_col81.clone(),
            value_limb_20_col82.clone(),
            value_limb_21_col83.clone(),
            value_limb_22_col84.clone(),
            value_limb_23_col85.clone(),
            value_limb_24_col86.clone(),
            value_limb_25_col87.clone(),
            value_limb_26_col88.clone(),
            value_limb_27_col89.clone(),
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        let packed_input_state_2_tmp_34cc4_8_limb_0 = eval.add_intermediate(
            ((value_limb_0_col62.clone() + (value_limb_1_col63.clone() * M31_512.clone()))
                + (value_limb_2_col64.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_1 = eval.add_intermediate(
            ((value_limb_3_col65.clone() + (value_limb_4_col66.clone() * M31_512.clone()))
                + (value_limb_5_col67.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_2 = eval.add_intermediate(
            ((value_limb_6_col68.clone() + (value_limb_7_col69.clone() * M31_512.clone()))
                + (value_limb_8_col70.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_3 = eval.add_intermediate(
            ((value_limb_9_col71.clone() + (value_limb_10_col72.clone() * M31_512.clone()))
                + (value_limb_11_col73.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_4 = eval.add_intermediate(
            ((value_limb_12_col74.clone() + (value_limb_13_col75.clone() * M31_512.clone()))
                + (value_limb_14_col76.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_5 = eval.add_intermediate(
            ((value_limb_15_col77.clone() + (value_limb_16_col78.clone() * M31_512.clone()))
                + (value_limb_17_col79.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_6 = eval.add_intermediate(
            ((value_limb_18_col80.clone() + (value_limb_19_col81.clone() * M31_512.clone()))
                + (value_limb_20_col82.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_7 = eval.add_intermediate(
            ((value_limb_21_col83.clone() + (value_limb_22_col84.clone() * M31_512.clone()))
                + (value_limb_23_col85.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_34cc4_8_limb_8 = eval.add_intermediate(
            ((value_limb_24_col86.clone() + (value_limb_25_col87.clone() * M31_512.clone()))
                + (value_limb_26_col88.clone() * M31_262144.clone())),
        );
        PoseidonHadesPermutation::evaluate(
            [
                packed_input_state_0_tmp_34cc4_2_limb_0.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_1.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_2.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_3.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_4.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_5.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_6.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_7.clone(),
                packed_input_state_0_tmp_34cc4_2_limb_8.clone(),
                value_limb_27_col33.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_0.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_1.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_2.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_3.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_4.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_5.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_6.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_7.clone(),
                packed_input_state_1_tmp_34cc4_5_limb_8.clone(),
                value_limb_27_col61.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_0.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_1.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_2.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_3.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_4.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_5.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_6.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_7.clone(),
                packed_input_state_2_tmp_34cc4_8_limb_8.clone(),
                value_limb_27_col89.clone(),
            ],
            combination_limb_0_col90.clone(),
            combination_limb_1_col91.clone(),
            combination_limb_2_col92.clone(),
            combination_limb_3_col93.clone(),
            combination_limb_4_col94.clone(),
            combination_limb_5_col95.clone(),
            combination_limb_6_col96.clone(),
            combination_limb_7_col97.clone(),
            combination_limb_8_col98.clone(),
            combination_limb_9_col99.clone(),
            p_coef_col100.clone(),
            combination_limb_0_col101.clone(),
            combination_limb_1_col102.clone(),
            combination_limb_2_col103.clone(),
            combination_limb_3_col104.clone(),
            combination_limb_4_col105.clone(),
            combination_limb_5_col106.clone(),
            combination_limb_6_col107.clone(),
            combination_limb_7_col108.clone(),
            combination_limb_8_col109.clone(),
            combination_limb_9_col110.clone(),
            p_coef_col111.clone(),
            combination_limb_0_col112.clone(),
            combination_limb_1_col113.clone(),
            combination_limb_2_col114.clone(),
            combination_limb_3_col115.clone(),
            combination_limb_4_col116.clone(),
            combination_limb_5_col117.clone(),
            combination_limb_6_col118.clone(),
            combination_limb_7_col119.clone(),
            combination_limb_8_col120.clone(),
            combination_limb_9_col121.clone(),
            p_coef_col122.clone(),
            poseidon_full_round_chain_output_limb_0_col123.clone(),
            poseidon_full_round_chain_output_limb_1_col124.clone(),
            poseidon_full_round_chain_output_limb_2_col125.clone(),
            poseidon_full_round_chain_output_limb_3_col126.clone(),
            poseidon_full_round_chain_output_limb_4_col127.clone(),
            poseidon_full_round_chain_output_limb_5_col128.clone(),
            poseidon_full_round_chain_output_limb_6_col129.clone(),
            poseidon_full_round_chain_output_limb_7_col130.clone(),
            poseidon_full_round_chain_output_limb_8_col131.clone(),
            poseidon_full_round_chain_output_limb_9_col132.clone(),
            poseidon_full_round_chain_output_limb_10_col133.clone(),
            poseidon_full_round_chain_output_limb_11_col134.clone(),
            poseidon_full_round_chain_output_limb_12_col135.clone(),
            poseidon_full_round_chain_output_limb_13_col136.clone(),
            poseidon_full_round_chain_output_limb_14_col137.clone(),
            poseidon_full_round_chain_output_limb_15_col138.clone(),
            poseidon_full_round_chain_output_limb_16_col139.clone(),
            poseidon_full_round_chain_output_limb_17_col140.clone(),
            poseidon_full_round_chain_output_limb_18_col141.clone(),
            poseidon_full_round_chain_output_limb_19_col142.clone(),
            poseidon_full_round_chain_output_limb_20_col143.clone(),
            poseidon_full_round_chain_output_limb_21_col144.clone(),
            poseidon_full_round_chain_output_limb_22_col145.clone(),
            poseidon_full_round_chain_output_limb_23_col146.clone(),
            poseidon_full_round_chain_output_limb_24_col147.clone(),
            poseidon_full_round_chain_output_limb_25_col148.clone(),
            poseidon_full_round_chain_output_limb_26_col149.clone(),
            poseidon_full_round_chain_output_limb_27_col150.clone(),
            poseidon_full_round_chain_output_limb_28_col151.clone(),
            poseidon_full_round_chain_output_limb_29_col152.clone(),
            cube_252_output_limb_0_col153.clone(),
            cube_252_output_limb_1_col154.clone(),
            cube_252_output_limb_2_col155.clone(),
            cube_252_output_limb_3_col156.clone(),
            cube_252_output_limb_4_col157.clone(),
            cube_252_output_limb_5_col158.clone(),
            cube_252_output_limb_6_col159.clone(),
            cube_252_output_limb_7_col160.clone(),
            cube_252_output_limb_8_col161.clone(),
            cube_252_output_limb_9_col162.clone(),
            combination_limb_0_col163.clone(),
            combination_limb_1_col164.clone(),
            combination_limb_2_col165.clone(),
            combination_limb_3_col166.clone(),
            combination_limb_4_col167.clone(),
            combination_limb_5_col168.clone(),
            combination_limb_6_col169.clone(),
            combination_limb_7_col170.clone(),
            combination_limb_8_col171.clone(),
            combination_limb_9_col172.clone(),
            p_coef_col173.clone(),
            cube_252_output_limb_0_col174.clone(),
            cube_252_output_limb_1_col175.clone(),
            cube_252_output_limb_2_col176.clone(),
            cube_252_output_limb_3_col177.clone(),
            cube_252_output_limb_4_col178.clone(),
            cube_252_output_limb_5_col179.clone(),
            cube_252_output_limb_6_col180.clone(),
            cube_252_output_limb_7_col181.clone(),
            cube_252_output_limb_8_col182.clone(),
            cube_252_output_limb_9_col183.clone(),
            combination_limb_0_col184.clone(),
            combination_limb_1_col185.clone(),
            combination_limb_2_col186.clone(),
            combination_limb_3_col187.clone(),
            combination_limb_4_col188.clone(),
            combination_limb_5_col189.clone(),
            combination_limb_6_col190.clone(),
            combination_limb_7_col191.clone(),
            combination_limb_8_col192.clone(),
            combination_limb_9_col193.clone(),
            p_coef_col194.clone(),
            poseidon_3_partial_rounds_chain_output_limb_0_col195.clone(),
            poseidon_3_partial_rounds_chain_output_limb_1_col196.clone(),
            poseidon_3_partial_rounds_chain_output_limb_2_col197.clone(),
            poseidon_3_partial_rounds_chain_output_limb_3_col198.clone(),
            poseidon_3_partial_rounds_chain_output_limb_4_col199.clone(),
            poseidon_3_partial_rounds_chain_output_limb_5_col200.clone(),
            poseidon_3_partial_rounds_chain_output_limb_6_col201.clone(),
            poseidon_3_partial_rounds_chain_output_limb_7_col202.clone(),
            poseidon_3_partial_rounds_chain_output_limb_8_col203.clone(),
            poseidon_3_partial_rounds_chain_output_limb_9_col204.clone(),
            poseidon_3_partial_rounds_chain_output_limb_10_col205.clone(),
            poseidon_3_partial_rounds_chain_output_limb_11_col206.clone(),
            poseidon_3_partial_rounds_chain_output_limb_12_col207.clone(),
            poseidon_3_partial_rounds_chain_output_limb_13_col208.clone(),
            poseidon_3_partial_rounds_chain_output_limb_14_col209.clone(),
            poseidon_3_partial_rounds_chain_output_limb_15_col210.clone(),
            poseidon_3_partial_rounds_chain_output_limb_16_col211.clone(),
            poseidon_3_partial_rounds_chain_output_limb_17_col212.clone(),
            poseidon_3_partial_rounds_chain_output_limb_18_col213.clone(),
            poseidon_3_partial_rounds_chain_output_limb_19_col214.clone(),
            poseidon_3_partial_rounds_chain_output_limb_20_col215.clone(),
            poseidon_3_partial_rounds_chain_output_limb_21_col216.clone(),
            poseidon_3_partial_rounds_chain_output_limb_22_col217.clone(),
            poseidon_3_partial_rounds_chain_output_limb_23_col218.clone(),
            poseidon_3_partial_rounds_chain_output_limb_24_col219.clone(),
            poseidon_3_partial_rounds_chain_output_limb_25_col220.clone(),
            poseidon_3_partial_rounds_chain_output_limb_26_col221.clone(),
            poseidon_3_partial_rounds_chain_output_limb_27_col222.clone(),
            poseidon_3_partial_rounds_chain_output_limb_28_col223.clone(),
            poseidon_3_partial_rounds_chain_output_limb_29_col224.clone(),
            poseidon_3_partial_rounds_chain_output_limb_30_col225.clone(),
            poseidon_3_partial_rounds_chain_output_limb_31_col226.clone(),
            poseidon_3_partial_rounds_chain_output_limb_32_col227.clone(),
            poseidon_3_partial_rounds_chain_output_limb_33_col228.clone(),
            poseidon_3_partial_rounds_chain_output_limb_34_col229.clone(),
            poseidon_3_partial_rounds_chain_output_limb_35_col230.clone(),
            poseidon_3_partial_rounds_chain_output_limb_36_col231.clone(),
            poseidon_3_partial_rounds_chain_output_limb_37_col232.clone(),
            poseidon_3_partial_rounds_chain_output_limb_38_col233.clone(),
            poseidon_3_partial_rounds_chain_output_limb_39_col234.clone(),
            combination_limb_0_col235.clone(),
            combination_limb_1_col236.clone(),
            combination_limb_2_col237.clone(),
            combination_limb_3_col238.clone(),
            combination_limb_4_col239.clone(),
            combination_limb_5_col240.clone(),
            combination_limb_6_col241.clone(),
            combination_limb_7_col242.clone(),
            combination_limb_8_col243.clone(),
            combination_limb_9_col244.clone(),
            p_coef_col245.clone(),
            combination_limb_0_col246.clone(),
            combination_limb_1_col247.clone(),
            combination_limb_2_col248.clone(),
            combination_limb_3_col249.clone(),
            combination_limb_4_col250.clone(),
            combination_limb_5_col251.clone(),
            combination_limb_6_col252.clone(),
            combination_limb_7_col253.clone(),
            combination_limb_8_col254.clone(),
            combination_limb_9_col255.clone(),
            p_coef_col256.clone(),
            poseidon_full_round_chain_output_limb_0_col257.clone(),
            poseidon_full_round_chain_output_limb_1_col258.clone(),
            poseidon_full_round_chain_output_limb_2_col259.clone(),
            poseidon_full_round_chain_output_limb_3_col260.clone(),
            poseidon_full_round_chain_output_limb_4_col261.clone(),
            poseidon_full_round_chain_output_limb_5_col262.clone(),
            poseidon_full_round_chain_output_limb_6_col263.clone(),
            poseidon_full_round_chain_output_limb_7_col264.clone(),
            poseidon_full_round_chain_output_limb_8_col265.clone(),
            poseidon_full_round_chain_output_limb_9_col266.clone(),
            poseidon_full_round_chain_output_limb_10_col267.clone(),
            poseidon_full_round_chain_output_limb_11_col268.clone(),
            poseidon_full_round_chain_output_limb_12_col269.clone(),
            poseidon_full_round_chain_output_limb_13_col270.clone(),
            poseidon_full_round_chain_output_limb_14_col271.clone(),
            poseidon_full_round_chain_output_limb_15_col272.clone(),
            poseidon_full_round_chain_output_limb_16_col273.clone(),
            poseidon_full_round_chain_output_limb_17_col274.clone(),
            poseidon_full_round_chain_output_limb_18_col275.clone(),
            poseidon_full_round_chain_output_limb_19_col276.clone(),
            poseidon_full_round_chain_output_limb_20_col277.clone(),
            poseidon_full_round_chain_output_limb_21_col278.clone(),
            poseidon_full_round_chain_output_limb_22_col279.clone(),
            poseidon_full_round_chain_output_limb_23_col280.clone(),
            poseidon_full_round_chain_output_limb_24_col281.clone(),
            poseidon_full_round_chain_output_limb_25_col282.clone(),
            poseidon_full_round_chain_output_limb_26_col283.clone(),
            poseidon_full_round_chain_output_limb_27_col284.clone(),
            poseidon_full_round_chain_output_limb_28_col285.clone(),
            poseidon_full_round_chain_output_limb_29_col286.clone(),
            &self.poseidon_full_round_chain_lookup_elements,
            &self.range_check_252_width_27_lookup_elements,
            &self.cube_252_lookup_elements,
            &self.range_check_3_3_3_3_3_lookup_elements,
            &self.range_check_4_4_4_4_lookup_elements,
            &self.range_check_4_4_lookup_elements,
            &self.poseidon_3_partial_rounds_chain_lookup_elements,
            seq.clone(),
            &mut eval,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_output_tmp_34cc4_162_limb_2, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_5, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_8, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_11, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_14, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_17, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_20, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_23, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_26, felt_252_unpack_from_27_output_tmp_34cc4_162_limb_27] =
            Felt252UnpackFrom27::evaluate(
                [
                    poseidon_full_round_chain_output_limb_0_col257.clone(),
                    poseidon_full_round_chain_output_limb_1_col258.clone(),
                    poseidon_full_round_chain_output_limb_2_col259.clone(),
                    poseidon_full_round_chain_output_limb_3_col260.clone(),
                    poseidon_full_round_chain_output_limb_4_col261.clone(),
                    poseidon_full_round_chain_output_limb_5_col262.clone(),
                    poseidon_full_round_chain_output_limb_6_col263.clone(),
                    poseidon_full_round_chain_output_limb_7_col264.clone(),
                    poseidon_full_round_chain_output_limb_8_col265.clone(),
                    poseidon_full_round_chain_output_limb_9_col266.clone(),
                ],
                unpacked_limb_0_col287.clone(),
                unpacked_limb_1_col288.clone(),
                unpacked_limb_3_col289.clone(),
                unpacked_limb_4_col290.clone(),
                unpacked_limb_6_col291.clone(),
                unpacked_limb_7_col292.clone(),
                unpacked_limb_9_col293.clone(),
                unpacked_limb_10_col294.clone(),
                unpacked_limb_12_col295.clone(),
                unpacked_limb_13_col296.clone(),
                unpacked_limb_15_col297.clone(),
                unpacked_limb_16_col298.clone(),
                unpacked_limb_18_col299.clone(),
                unpacked_limb_19_col300.clone(),
                unpacked_limb_21_col301.clone(),
                unpacked_limb_22_col302.clone(),
                unpacked_limb_24_col303.clone(),
                unpacked_limb_25_col304.clone(),
                &mut eval,
            );
        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                input_limb_3_col3.clone(),
                unpacked_limb_0_col287.clone(),
                unpacked_limb_1_col288.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_2.clone(),
                unpacked_limb_3_col289.clone(),
                unpacked_limb_4_col290.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_5.clone(),
                unpacked_limb_6_col291.clone(),
                unpacked_limb_7_col292.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_8.clone(),
                unpacked_limb_9_col293.clone(),
                unpacked_limb_10_col294.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_11.clone(),
                unpacked_limb_12_col295.clone(),
                unpacked_limb_13_col296.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_14.clone(),
                unpacked_limb_15_col297.clone(),
                unpacked_limb_16_col298.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_17.clone(),
                unpacked_limb_18_col299.clone(),
                unpacked_limb_19_col300.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_20.clone(),
                unpacked_limb_21_col301.clone(),
                unpacked_limb_22_col302.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_23.clone(),
                unpacked_limb_24_col303.clone(),
                unpacked_limb_25_col304.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_162_limb_26.clone(),
                poseidon_full_round_chain_output_limb_9_col266.clone(),
            ],
        ));

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_output_tmp_34cc4_164_limb_2, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_5, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_8, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_11, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_14, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_17, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_20, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_23, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_26, felt_252_unpack_from_27_output_tmp_34cc4_164_limb_27] =
            Felt252UnpackFrom27::evaluate(
                [
                    poseidon_full_round_chain_output_limb_10_col267.clone(),
                    poseidon_full_round_chain_output_limb_11_col268.clone(),
                    poseidon_full_round_chain_output_limb_12_col269.clone(),
                    poseidon_full_round_chain_output_limb_13_col270.clone(),
                    poseidon_full_round_chain_output_limb_14_col271.clone(),
                    poseidon_full_round_chain_output_limb_15_col272.clone(),
                    poseidon_full_round_chain_output_limb_16_col273.clone(),
                    poseidon_full_round_chain_output_limb_17_col274.clone(),
                    poseidon_full_round_chain_output_limb_18_col275.clone(),
                    poseidon_full_round_chain_output_limb_19_col276.clone(),
                ],
                unpacked_limb_0_col305.clone(),
                unpacked_limb_1_col306.clone(),
                unpacked_limb_3_col307.clone(),
                unpacked_limb_4_col308.clone(),
                unpacked_limb_6_col309.clone(),
                unpacked_limb_7_col310.clone(),
                unpacked_limb_9_col311.clone(),
                unpacked_limb_10_col312.clone(),
                unpacked_limb_12_col313.clone(),
                unpacked_limb_13_col314.clone(),
                unpacked_limb_15_col315.clone(),
                unpacked_limb_16_col316.clone(),
                unpacked_limb_18_col317.clone(),
                unpacked_limb_19_col318.clone(),
                unpacked_limb_21_col319.clone(),
                unpacked_limb_22_col320.clone(),
                unpacked_limb_24_col321.clone(),
                unpacked_limb_25_col322.clone(),
                &mut eval,
            );
        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                input_limb_4_col4.clone(),
                unpacked_limb_0_col305.clone(),
                unpacked_limb_1_col306.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_2.clone(),
                unpacked_limb_3_col307.clone(),
                unpacked_limb_4_col308.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_5.clone(),
                unpacked_limb_6_col309.clone(),
                unpacked_limb_7_col310.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_8.clone(),
                unpacked_limb_9_col311.clone(),
                unpacked_limb_10_col312.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_11.clone(),
                unpacked_limb_12_col313.clone(),
                unpacked_limb_13_col314.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_14.clone(),
                unpacked_limb_15_col315.clone(),
                unpacked_limb_16_col316.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_17.clone(),
                unpacked_limb_18_col317.clone(),
                unpacked_limb_19_col318.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_20.clone(),
                unpacked_limb_21_col319.clone(),
                unpacked_limb_22_col320.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_23.clone(),
                unpacked_limb_24_col321.clone(),
                unpacked_limb_25_col322.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_164_limb_26.clone(),
                poseidon_full_round_chain_output_limb_19_col276.clone(),
            ],
        ));

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_output_tmp_34cc4_166_limb_2, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_5, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_8, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_11, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_14, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_17, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_20, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_23, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_26, felt_252_unpack_from_27_output_tmp_34cc4_166_limb_27] =
            Felt252UnpackFrom27::evaluate(
                [
                    poseidon_full_round_chain_output_limb_20_col277.clone(),
                    poseidon_full_round_chain_output_limb_21_col278.clone(),
                    poseidon_full_round_chain_output_limb_22_col279.clone(),
                    poseidon_full_round_chain_output_limb_23_col280.clone(),
                    poseidon_full_round_chain_output_limb_24_col281.clone(),
                    poseidon_full_round_chain_output_limb_25_col282.clone(),
                    poseidon_full_round_chain_output_limb_26_col283.clone(),
                    poseidon_full_round_chain_output_limb_27_col284.clone(),
                    poseidon_full_round_chain_output_limb_28_col285.clone(),
                    poseidon_full_round_chain_output_limb_29_col286.clone(),
                ],
                unpacked_limb_0_col323.clone(),
                unpacked_limb_1_col324.clone(),
                unpacked_limb_3_col325.clone(),
                unpacked_limb_4_col326.clone(),
                unpacked_limb_6_col327.clone(),
                unpacked_limb_7_col328.clone(),
                unpacked_limb_9_col329.clone(),
                unpacked_limb_10_col330.clone(),
                unpacked_limb_12_col331.clone(),
                unpacked_limb_13_col332.clone(),
                unpacked_limb_15_col333.clone(),
                unpacked_limb_16_col334.clone(),
                unpacked_limb_18_col335.clone(),
                unpacked_limb_19_col336.clone(),
                unpacked_limb_21_col337.clone(),
                unpacked_limb_22_col338.clone(),
                unpacked_limb_24_col339.clone(),
                unpacked_limb_25_col340.clone(),
                &mut eval,
            );
        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                input_limb_5_col5.clone(),
                unpacked_limb_0_col323.clone(),
                unpacked_limb_1_col324.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_2.clone(),
                unpacked_limb_3_col325.clone(),
                unpacked_limb_4_col326.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_5.clone(),
                unpacked_limb_6_col327.clone(),
                unpacked_limb_7_col328.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_8.clone(),
                unpacked_limb_9_col329.clone(),
                unpacked_limb_10_col330.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_11.clone(),
                unpacked_limb_12_col331.clone(),
                unpacked_limb_13_col332.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_14.clone(),
                unpacked_limb_15_col333.clone(),
                unpacked_limb_16_col334.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_17.clone(),
                unpacked_limb_18_col335.clone(),
                unpacked_limb_19_col336.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_20.clone(),
                unpacked_limb_21_col337.clone(),
                unpacked_limb_22_col338.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_23.clone(),
                unpacked_limb_24_col339.clone(),
                unpacked_limb_25_col340.clone(),
                felt_252_unpack_from_27_output_tmp_34cc4_166_limb_26.clone(),
                poseidon_full_round_chain_output_limb_29_col286.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.poseidon_aggregator_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                input_limb_0_col0.clone(),
                input_limb_1_col1.clone(),
                input_limb_2_col2.clone(),
                input_limb_3_col3.clone(),
                input_limb_4_col4.clone(),
                input_limb_5_col5.clone(),
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
    use crate::components::constraints_regression_test_values::POSEIDON_AGGREGATOR;

    #[test]
    fn poseidon_aggregator_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim { log_size: 4 },
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain::dummy(),
            range_check_252_width_27_lookup_elements: relations::RangeCheck252Width27::dummy(),
            cube_252_lookup_elements: relations::Cube252::dummy(),
            range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3::dummy(),
            range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
            range_check_4_4_lookup_elements: relations::RangeCheck_4_4::dummy(),
            poseidon_3_partial_rounds_chain_lookup_elements:
                relations::Poseidon3PartialRoundsChain::dummy(),
            poseidon_aggregator_lookup_elements: relations::PoseidonAggregator::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, POSEIDON_AGGREGATOR);
    }
}

use crate::components::prelude::*;
use crate::components::subroutines::felt_252_unpack_from_27::Felt252UnpackFrom27;
use crate::components::subroutines::mem_verify::MemVerify;
use crate::components::subroutines::poseidon_hades_permutation::PoseidonHadesPermutation;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;

pub const N_TRACE_COLUMNS: usize = 347;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain,
    pub range_check_felt_252_width_27_lookup_elements: relations::RangeCheckFelt252Width27,
    pub cube_252_lookup_elements: relations::Cube252,
    pub range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3,
    pub range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4,
    pub range_check_4_4_lookup_elements: relations::RangeCheck_4_4,
    pub poseidon_3_partial_rounds_chain_lookup_elements: relations::Poseidon3PartialRoundsChain,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 17];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.poseidon_builtin_segment_start as u64);
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
        let M31_1 = E::F::from(M31::from(1));
        let M31_2 = E::F::from(M31::from(2));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_3 = E::F::from(M31::from(3));
        let M31_4 = E::F::from(M31::from(4));
        let M31_5 = E::F::from(M31::from(5));
        let M31_512 = E::F::from(M31::from(512));
        let M31_6 = E::F::from(M31::from(6));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let input_state_0_id_col0 = eval.next_trace_mask();
        let input_state_0_limb_0_col1 = eval.next_trace_mask();
        let input_state_0_limb_1_col2 = eval.next_trace_mask();
        let input_state_0_limb_2_col3 = eval.next_trace_mask();
        let input_state_0_limb_3_col4 = eval.next_trace_mask();
        let input_state_0_limb_4_col5 = eval.next_trace_mask();
        let input_state_0_limb_5_col6 = eval.next_trace_mask();
        let input_state_0_limb_6_col7 = eval.next_trace_mask();
        let input_state_0_limb_7_col8 = eval.next_trace_mask();
        let input_state_0_limb_8_col9 = eval.next_trace_mask();
        let input_state_0_limb_9_col10 = eval.next_trace_mask();
        let input_state_0_limb_10_col11 = eval.next_trace_mask();
        let input_state_0_limb_11_col12 = eval.next_trace_mask();
        let input_state_0_limb_12_col13 = eval.next_trace_mask();
        let input_state_0_limb_13_col14 = eval.next_trace_mask();
        let input_state_0_limb_14_col15 = eval.next_trace_mask();
        let input_state_0_limb_15_col16 = eval.next_trace_mask();
        let input_state_0_limb_16_col17 = eval.next_trace_mask();
        let input_state_0_limb_17_col18 = eval.next_trace_mask();
        let input_state_0_limb_18_col19 = eval.next_trace_mask();
        let input_state_0_limb_19_col20 = eval.next_trace_mask();
        let input_state_0_limb_20_col21 = eval.next_trace_mask();
        let input_state_0_limb_21_col22 = eval.next_trace_mask();
        let input_state_0_limb_22_col23 = eval.next_trace_mask();
        let input_state_0_limb_23_col24 = eval.next_trace_mask();
        let input_state_0_limb_24_col25 = eval.next_trace_mask();
        let input_state_0_limb_25_col26 = eval.next_trace_mask();
        let input_state_0_limb_26_col27 = eval.next_trace_mask();
        let input_state_0_limb_27_col28 = eval.next_trace_mask();
        let input_state_1_id_col29 = eval.next_trace_mask();
        let input_state_1_limb_0_col30 = eval.next_trace_mask();
        let input_state_1_limb_1_col31 = eval.next_trace_mask();
        let input_state_1_limb_2_col32 = eval.next_trace_mask();
        let input_state_1_limb_3_col33 = eval.next_trace_mask();
        let input_state_1_limb_4_col34 = eval.next_trace_mask();
        let input_state_1_limb_5_col35 = eval.next_trace_mask();
        let input_state_1_limb_6_col36 = eval.next_trace_mask();
        let input_state_1_limb_7_col37 = eval.next_trace_mask();
        let input_state_1_limb_8_col38 = eval.next_trace_mask();
        let input_state_1_limb_9_col39 = eval.next_trace_mask();
        let input_state_1_limb_10_col40 = eval.next_trace_mask();
        let input_state_1_limb_11_col41 = eval.next_trace_mask();
        let input_state_1_limb_12_col42 = eval.next_trace_mask();
        let input_state_1_limb_13_col43 = eval.next_trace_mask();
        let input_state_1_limb_14_col44 = eval.next_trace_mask();
        let input_state_1_limb_15_col45 = eval.next_trace_mask();
        let input_state_1_limb_16_col46 = eval.next_trace_mask();
        let input_state_1_limb_17_col47 = eval.next_trace_mask();
        let input_state_1_limb_18_col48 = eval.next_trace_mask();
        let input_state_1_limb_19_col49 = eval.next_trace_mask();
        let input_state_1_limb_20_col50 = eval.next_trace_mask();
        let input_state_1_limb_21_col51 = eval.next_trace_mask();
        let input_state_1_limb_22_col52 = eval.next_trace_mask();
        let input_state_1_limb_23_col53 = eval.next_trace_mask();
        let input_state_1_limb_24_col54 = eval.next_trace_mask();
        let input_state_1_limb_25_col55 = eval.next_trace_mask();
        let input_state_1_limb_26_col56 = eval.next_trace_mask();
        let input_state_1_limb_27_col57 = eval.next_trace_mask();
        let input_state_2_id_col58 = eval.next_trace_mask();
        let input_state_2_limb_0_col59 = eval.next_trace_mask();
        let input_state_2_limb_1_col60 = eval.next_trace_mask();
        let input_state_2_limb_2_col61 = eval.next_trace_mask();
        let input_state_2_limb_3_col62 = eval.next_trace_mask();
        let input_state_2_limb_4_col63 = eval.next_trace_mask();
        let input_state_2_limb_5_col64 = eval.next_trace_mask();
        let input_state_2_limb_6_col65 = eval.next_trace_mask();
        let input_state_2_limb_7_col66 = eval.next_trace_mask();
        let input_state_2_limb_8_col67 = eval.next_trace_mask();
        let input_state_2_limb_9_col68 = eval.next_trace_mask();
        let input_state_2_limb_10_col69 = eval.next_trace_mask();
        let input_state_2_limb_11_col70 = eval.next_trace_mask();
        let input_state_2_limb_12_col71 = eval.next_trace_mask();
        let input_state_2_limb_13_col72 = eval.next_trace_mask();
        let input_state_2_limb_14_col73 = eval.next_trace_mask();
        let input_state_2_limb_15_col74 = eval.next_trace_mask();
        let input_state_2_limb_16_col75 = eval.next_trace_mask();
        let input_state_2_limb_17_col76 = eval.next_trace_mask();
        let input_state_2_limb_18_col77 = eval.next_trace_mask();
        let input_state_2_limb_19_col78 = eval.next_trace_mask();
        let input_state_2_limb_20_col79 = eval.next_trace_mask();
        let input_state_2_limb_21_col80 = eval.next_trace_mask();
        let input_state_2_limb_22_col81 = eval.next_trace_mask();
        let input_state_2_limb_23_col82 = eval.next_trace_mask();
        let input_state_2_limb_24_col83 = eval.next_trace_mask();
        let input_state_2_limb_25_col84 = eval.next_trace_mask();
        let input_state_2_limb_26_col85 = eval.next_trace_mask();
        let input_state_2_limb_27_col86 = eval.next_trace_mask();
        let combination_limb_0_col87 = eval.next_trace_mask();
        let combination_limb_1_col88 = eval.next_trace_mask();
        let combination_limb_2_col89 = eval.next_trace_mask();
        let combination_limb_3_col90 = eval.next_trace_mask();
        let combination_limb_4_col91 = eval.next_trace_mask();
        let combination_limb_5_col92 = eval.next_trace_mask();
        let combination_limb_6_col93 = eval.next_trace_mask();
        let combination_limb_7_col94 = eval.next_trace_mask();
        let combination_limb_8_col95 = eval.next_trace_mask();
        let combination_limb_9_col96 = eval.next_trace_mask();
        let p_coef_col97 = eval.next_trace_mask();
        let combination_limb_0_col98 = eval.next_trace_mask();
        let combination_limb_1_col99 = eval.next_trace_mask();
        let combination_limb_2_col100 = eval.next_trace_mask();
        let combination_limb_3_col101 = eval.next_trace_mask();
        let combination_limb_4_col102 = eval.next_trace_mask();
        let combination_limb_5_col103 = eval.next_trace_mask();
        let combination_limb_6_col104 = eval.next_trace_mask();
        let combination_limb_7_col105 = eval.next_trace_mask();
        let combination_limb_8_col106 = eval.next_trace_mask();
        let combination_limb_9_col107 = eval.next_trace_mask();
        let p_coef_col108 = eval.next_trace_mask();
        let combination_limb_0_col109 = eval.next_trace_mask();
        let combination_limb_1_col110 = eval.next_trace_mask();
        let combination_limb_2_col111 = eval.next_trace_mask();
        let combination_limb_3_col112 = eval.next_trace_mask();
        let combination_limb_4_col113 = eval.next_trace_mask();
        let combination_limb_5_col114 = eval.next_trace_mask();
        let combination_limb_6_col115 = eval.next_trace_mask();
        let combination_limb_7_col116 = eval.next_trace_mask();
        let combination_limb_8_col117 = eval.next_trace_mask();
        let combination_limb_9_col118 = eval.next_trace_mask();
        let p_coef_col119 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_0_col120 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_1_col121 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_2_col122 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_3_col123 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_4_col124 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_5_col125 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_6_col126 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_7_col127 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_8_col128 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_9_col129 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_10_col130 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_11_col131 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_12_col132 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_13_col133 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_14_col134 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_15_col135 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_16_col136 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_17_col137 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_18_col138 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_19_col139 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_20_col140 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_21_col141 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_22_col142 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_23_col143 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_24_col144 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_25_col145 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_26_col146 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_27_col147 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_28_col148 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_29_col149 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_30_col150 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_31_col151 = eval.next_trace_mask();
        let cube_252_output_limb_0_col152 = eval.next_trace_mask();
        let cube_252_output_limb_1_col153 = eval.next_trace_mask();
        let cube_252_output_limb_2_col154 = eval.next_trace_mask();
        let cube_252_output_limb_3_col155 = eval.next_trace_mask();
        let cube_252_output_limb_4_col156 = eval.next_trace_mask();
        let cube_252_output_limb_5_col157 = eval.next_trace_mask();
        let cube_252_output_limb_6_col158 = eval.next_trace_mask();
        let cube_252_output_limb_7_col159 = eval.next_trace_mask();
        let cube_252_output_limb_8_col160 = eval.next_trace_mask();
        let cube_252_output_limb_9_col161 = eval.next_trace_mask();
        let combination_limb_0_col162 = eval.next_trace_mask();
        let combination_limb_1_col163 = eval.next_trace_mask();
        let combination_limb_2_col164 = eval.next_trace_mask();
        let combination_limb_3_col165 = eval.next_trace_mask();
        let combination_limb_4_col166 = eval.next_trace_mask();
        let combination_limb_5_col167 = eval.next_trace_mask();
        let combination_limb_6_col168 = eval.next_trace_mask();
        let combination_limb_7_col169 = eval.next_trace_mask();
        let combination_limb_8_col170 = eval.next_trace_mask();
        let combination_limb_9_col171 = eval.next_trace_mask();
        let p_coef_col172 = eval.next_trace_mask();
        let cube_252_output_limb_0_col173 = eval.next_trace_mask();
        let cube_252_output_limb_1_col174 = eval.next_trace_mask();
        let cube_252_output_limb_2_col175 = eval.next_trace_mask();
        let cube_252_output_limb_3_col176 = eval.next_trace_mask();
        let cube_252_output_limb_4_col177 = eval.next_trace_mask();
        let cube_252_output_limb_5_col178 = eval.next_trace_mask();
        let cube_252_output_limb_6_col179 = eval.next_trace_mask();
        let cube_252_output_limb_7_col180 = eval.next_trace_mask();
        let cube_252_output_limb_8_col181 = eval.next_trace_mask();
        let cube_252_output_limb_9_col182 = eval.next_trace_mask();
        let combination_limb_0_col183 = eval.next_trace_mask();
        let combination_limb_1_col184 = eval.next_trace_mask();
        let combination_limb_2_col185 = eval.next_trace_mask();
        let combination_limb_3_col186 = eval.next_trace_mask();
        let combination_limb_4_col187 = eval.next_trace_mask();
        let combination_limb_5_col188 = eval.next_trace_mask();
        let combination_limb_6_col189 = eval.next_trace_mask();
        let combination_limb_7_col190 = eval.next_trace_mask();
        let combination_limb_8_col191 = eval.next_trace_mask();
        let combination_limb_9_col192 = eval.next_trace_mask();
        let p_coef_col193 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_0_col194 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_1_col195 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_2_col196 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_3_col197 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_4_col198 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_5_col199 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_6_col200 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_7_col201 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_8_col202 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_9_col203 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_10_col204 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_11_col205 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_12_col206 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_13_col207 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_14_col208 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_15_col209 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_16_col210 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_17_col211 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_18_col212 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_19_col213 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_20_col214 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_21_col215 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_22_col216 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_23_col217 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_24_col218 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_25_col219 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_26_col220 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_27_col221 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_28_col222 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_29_col223 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_30_col224 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_31_col225 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_32_col226 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_33_col227 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_34_col228 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_35_col229 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_36_col230 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_37_col231 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_38_col232 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_39_col233 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_40_col234 = eval.next_trace_mask();
        let poseidon_3_partial_rounds_chain_output_limb_41_col235 = eval.next_trace_mask();
        let combination_limb_0_col236 = eval.next_trace_mask();
        let combination_limb_1_col237 = eval.next_trace_mask();
        let combination_limb_2_col238 = eval.next_trace_mask();
        let combination_limb_3_col239 = eval.next_trace_mask();
        let combination_limb_4_col240 = eval.next_trace_mask();
        let combination_limb_5_col241 = eval.next_trace_mask();
        let combination_limb_6_col242 = eval.next_trace_mask();
        let combination_limb_7_col243 = eval.next_trace_mask();
        let combination_limb_8_col244 = eval.next_trace_mask();
        let combination_limb_9_col245 = eval.next_trace_mask();
        let p_coef_col246 = eval.next_trace_mask();
        let combination_limb_0_col247 = eval.next_trace_mask();
        let combination_limb_1_col248 = eval.next_trace_mask();
        let combination_limb_2_col249 = eval.next_trace_mask();
        let combination_limb_3_col250 = eval.next_trace_mask();
        let combination_limb_4_col251 = eval.next_trace_mask();
        let combination_limb_5_col252 = eval.next_trace_mask();
        let combination_limb_6_col253 = eval.next_trace_mask();
        let combination_limb_7_col254 = eval.next_trace_mask();
        let combination_limb_8_col255 = eval.next_trace_mask();
        let combination_limb_9_col256 = eval.next_trace_mask();
        let p_coef_col257 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_0_col258 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_1_col259 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_2_col260 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_3_col261 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_4_col262 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_5_col263 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_6_col264 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_7_col265 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_8_col266 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_9_col267 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_10_col268 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_11_col269 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_12_col270 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_13_col271 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_14_col272 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_15_col273 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_16_col274 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_17_col275 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_18_col276 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_19_col277 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_20_col278 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_21_col279 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_22_col280 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_23_col281 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_24_col282 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_25_col283 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_26_col284 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_27_col285 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_28_col286 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_29_col287 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_30_col288 = eval.next_trace_mask();
        let poseidon_full_round_chain_output_limb_31_col289 = eval.next_trace_mask();
        let unpacked_limb_0_col290 = eval.next_trace_mask();
        let unpacked_limb_1_col291 = eval.next_trace_mask();
        let unpacked_limb_3_col292 = eval.next_trace_mask();
        let unpacked_limb_4_col293 = eval.next_trace_mask();
        let unpacked_limb_6_col294 = eval.next_trace_mask();
        let unpacked_limb_7_col295 = eval.next_trace_mask();
        let unpacked_limb_9_col296 = eval.next_trace_mask();
        let unpacked_limb_10_col297 = eval.next_trace_mask();
        let unpacked_limb_12_col298 = eval.next_trace_mask();
        let unpacked_limb_13_col299 = eval.next_trace_mask();
        let unpacked_limb_15_col300 = eval.next_trace_mask();
        let unpacked_limb_16_col301 = eval.next_trace_mask();
        let unpacked_limb_18_col302 = eval.next_trace_mask();
        let unpacked_limb_19_col303 = eval.next_trace_mask();
        let unpacked_limb_21_col304 = eval.next_trace_mask();
        let unpacked_limb_22_col305 = eval.next_trace_mask();
        let unpacked_limb_24_col306 = eval.next_trace_mask();
        let unpacked_limb_25_col307 = eval.next_trace_mask();
        let output_state_0_id_col308 = eval.next_trace_mask();
        let unpacked_limb_0_col309 = eval.next_trace_mask();
        let unpacked_limb_1_col310 = eval.next_trace_mask();
        let unpacked_limb_3_col311 = eval.next_trace_mask();
        let unpacked_limb_4_col312 = eval.next_trace_mask();
        let unpacked_limb_6_col313 = eval.next_trace_mask();
        let unpacked_limb_7_col314 = eval.next_trace_mask();
        let unpacked_limb_9_col315 = eval.next_trace_mask();
        let unpacked_limb_10_col316 = eval.next_trace_mask();
        let unpacked_limb_12_col317 = eval.next_trace_mask();
        let unpacked_limb_13_col318 = eval.next_trace_mask();
        let unpacked_limb_15_col319 = eval.next_trace_mask();
        let unpacked_limb_16_col320 = eval.next_trace_mask();
        let unpacked_limb_18_col321 = eval.next_trace_mask();
        let unpacked_limb_19_col322 = eval.next_trace_mask();
        let unpacked_limb_21_col323 = eval.next_trace_mask();
        let unpacked_limb_22_col324 = eval.next_trace_mask();
        let unpacked_limb_24_col325 = eval.next_trace_mask();
        let unpacked_limb_25_col326 = eval.next_trace_mask();
        let output_state_1_id_col327 = eval.next_trace_mask();
        let unpacked_limb_0_col328 = eval.next_trace_mask();
        let unpacked_limb_1_col329 = eval.next_trace_mask();
        let unpacked_limb_3_col330 = eval.next_trace_mask();
        let unpacked_limb_4_col331 = eval.next_trace_mask();
        let unpacked_limb_6_col332 = eval.next_trace_mask();
        let unpacked_limb_7_col333 = eval.next_trace_mask();
        let unpacked_limb_9_col334 = eval.next_trace_mask();
        let unpacked_limb_10_col335 = eval.next_trace_mask();
        let unpacked_limb_12_col336 = eval.next_trace_mask();
        let unpacked_limb_13_col337 = eval.next_trace_mask();
        let unpacked_limb_15_col338 = eval.next_trace_mask();
        let unpacked_limb_16_col339 = eval.next_trace_mask();
        let unpacked_limb_18_col340 = eval.next_trace_mask();
        let unpacked_limb_19_col341 = eval.next_trace_mask();
        let unpacked_limb_21_col342 = eval.next_trace_mask();
        let unpacked_limb_22_col343 = eval.next_trace_mask();
        let unpacked_limb_24_col344 = eval.next_trace_mask();
        let unpacked_limb_25_col345 = eval.next_trace_mask();
        let output_state_2_id_col346 = eval.next_trace_mask();

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_252_output_tmp_51986_2_limb_0, read_positive_num_bits_252_output_tmp_51986_2_limb_1, read_positive_num_bits_252_output_tmp_51986_2_limb_2, read_positive_num_bits_252_output_tmp_51986_2_limb_3, read_positive_num_bits_252_output_tmp_51986_2_limb_4, read_positive_num_bits_252_output_tmp_51986_2_limb_5, read_positive_num_bits_252_output_tmp_51986_2_limb_6, read_positive_num_bits_252_output_tmp_51986_2_limb_7, read_positive_num_bits_252_output_tmp_51986_2_limb_8, read_positive_num_bits_252_output_tmp_51986_2_limb_9, read_positive_num_bits_252_output_tmp_51986_2_limb_10, read_positive_num_bits_252_output_tmp_51986_2_limb_11, read_positive_num_bits_252_output_tmp_51986_2_limb_12, read_positive_num_bits_252_output_tmp_51986_2_limb_13, read_positive_num_bits_252_output_tmp_51986_2_limb_14, read_positive_num_bits_252_output_tmp_51986_2_limb_15, read_positive_num_bits_252_output_tmp_51986_2_limb_16, read_positive_num_bits_252_output_tmp_51986_2_limb_17, read_positive_num_bits_252_output_tmp_51986_2_limb_18, read_positive_num_bits_252_output_tmp_51986_2_limb_19, read_positive_num_bits_252_output_tmp_51986_2_limb_20, read_positive_num_bits_252_output_tmp_51986_2_limb_21, read_positive_num_bits_252_output_tmp_51986_2_limb_22, read_positive_num_bits_252_output_tmp_51986_2_limb_23, read_positive_num_bits_252_output_tmp_51986_2_limb_24, read_positive_num_bits_252_output_tmp_51986_2_limb_25, read_positive_num_bits_252_output_tmp_51986_2_limb_26, read_positive_num_bits_252_output_tmp_51986_2_limb_27, read_positive_num_bits_252_output_tmp_51986_2_limb_28] =
            ReadPositiveNumBits252::evaluate(
                (E::F::from(M31::from(self.claim.poseidon_builtin_segment_start))
                    + (seq.clone() * M31_6.clone())),
                input_state_0_id_col0.clone(),
                input_state_0_limb_0_col1.clone(),
                input_state_0_limb_1_col2.clone(),
                input_state_0_limb_2_col3.clone(),
                input_state_0_limb_3_col4.clone(),
                input_state_0_limb_4_col5.clone(),
                input_state_0_limb_5_col6.clone(),
                input_state_0_limb_6_col7.clone(),
                input_state_0_limb_7_col8.clone(),
                input_state_0_limb_8_col9.clone(),
                input_state_0_limb_9_col10.clone(),
                input_state_0_limb_10_col11.clone(),
                input_state_0_limb_11_col12.clone(),
                input_state_0_limb_12_col13.clone(),
                input_state_0_limb_13_col14.clone(),
                input_state_0_limb_14_col15.clone(),
                input_state_0_limb_15_col16.clone(),
                input_state_0_limb_16_col17.clone(),
                input_state_0_limb_17_col18.clone(),
                input_state_0_limb_18_col19.clone(),
                input_state_0_limb_19_col20.clone(),
                input_state_0_limb_20_col21.clone(),
                input_state_0_limb_21_col22.clone(),
                input_state_0_limb_22_col23.clone(),
                input_state_0_limb_23_col24.clone(),
                input_state_0_limb_24_col25.clone(),
                input_state_0_limb_25_col26.clone(),
                input_state_0_limb_26_col27.clone(),
                input_state_0_limb_27_col28.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        let packed_input_state_0_tmp_51986_3_limb_0 = eval.add_intermediate(
            ((input_state_0_limb_0_col1.clone()
                + (input_state_0_limb_1_col2.clone() * M31_512.clone()))
                + (input_state_0_limb_2_col3.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_1 = eval.add_intermediate(
            ((input_state_0_limb_3_col4.clone()
                + (input_state_0_limb_4_col5.clone() * M31_512.clone()))
                + (input_state_0_limb_5_col6.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_2 = eval.add_intermediate(
            ((input_state_0_limb_6_col7.clone()
                + (input_state_0_limb_7_col8.clone() * M31_512.clone()))
                + (input_state_0_limb_8_col9.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_3 = eval.add_intermediate(
            ((input_state_0_limb_9_col10.clone()
                + (input_state_0_limb_10_col11.clone() * M31_512.clone()))
                + (input_state_0_limb_11_col12.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_4 = eval.add_intermediate(
            ((input_state_0_limb_12_col13.clone()
                + (input_state_0_limb_13_col14.clone() * M31_512.clone()))
                + (input_state_0_limb_14_col15.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_5 = eval.add_intermediate(
            ((input_state_0_limb_15_col16.clone()
                + (input_state_0_limb_16_col17.clone() * M31_512.clone()))
                + (input_state_0_limb_17_col18.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_6 = eval.add_intermediate(
            ((input_state_0_limb_18_col19.clone()
                + (input_state_0_limb_19_col20.clone() * M31_512.clone()))
                + (input_state_0_limb_20_col21.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_7 = eval.add_intermediate(
            ((input_state_0_limb_21_col22.clone()
                + (input_state_0_limb_22_col23.clone() * M31_512.clone()))
                + (input_state_0_limb_23_col24.clone() * M31_262144.clone())),
        );
        let packed_input_state_0_tmp_51986_3_limb_8 = eval.add_intermediate(
            ((input_state_0_limb_24_col25.clone()
                + (input_state_0_limb_25_col26.clone() * M31_512.clone()))
                + (input_state_0_limb_26_col27.clone() * M31_262144.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_252_output_tmp_51986_6_limb_0, read_positive_num_bits_252_output_tmp_51986_6_limb_1, read_positive_num_bits_252_output_tmp_51986_6_limb_2, read_positive_num_bits_252_output_tmp_51986_6_limb_3, read_positive_num_bits_252_output_tmp_51986_6_limb_4, read_positive_num_bits_252_output_tmp_51986_6_limb_5, read_positive_num_bits_252_output_tmp_51986_6_limb_6, read_positive_num_bits_252_output_tmp_51986_6_limb_7, read_positive_num_bits_252_output_tmp_51986_6_limb_8, read_positive_num_bits_252_output_tmp_51986_6_limb_9, read_positive_num_bits_252_output_tmp_51986_6_limb_10, read_positive_num_bits_252_output_tmp_51986_6_limb_11, read_positive_num_bits_252_output_tmp_51986_6_limb_12, read_positive_num_bits_252_output_tmp_51986_6_limb_13, read_positive_num_bits_252_output_tmp_51986_6_limb_14, read_positive_num_bits_252_output_tmp_51986_6_limb_15, read_positive_num_bits_252_output_tmp_51986_6_limb_16, read_positive_num_bits_252_output_tmp_51986_6_limb_17, read_positive_num_bits_252_output_tmp_51986_6_limb_18, read_positive_num_bits_252_output_tmp_51986_6_limb_19, read_positive_num_bits_252_output_tmp_51986_6_limb_20, read_positive_num_bits_252_output_tmp_51986_6_limb_21, read_positive_num_bits_252_output_tmp_51986_6_limb_22, read_positive_num_bits_252_output_tmp_51986_6_limb_23, read_positive_num_bits_252_output_tmp_51986_6_limb_24, read_positive_num_bits_252_output_tmp_51986_6_limb_25, read_positive_num_bits_252_output_tmp_51986_6_limb_26, read_positive_num_bits_252_output_tmp_51986_6_limb_27, read_positive_num_bits_252_output_tmp_51986_6_limb_28] =
            ReadPositiveNumBits252::evaluate(
                ((E::F::from(M31::from(self.claim.poseidon_builtin_segment_start))
                    + (seq.clone() * M31_6.clone()))
                    + M31_1.clone()),
                input_state_1_id_col29.clone(),
                input_state_1_limb_0_col30.clone(),
                input_state_1_limb_1_col31.clone(),
                input_state_1_limb_2_col32.clone(),
                input_state_1_limb_3_col33.clone(),
                input_state_1_limb_4_col34.clone(),
                input_state_1_limb_5_col35.clone(),
                input_state_1_limb_6_col36.clone(),
                input_state_1_limb_7_col37.clone(),
                input_state_1_limb_8_col38.clone(),
                input_state_1_limb_9_col39.clone(),
                input_state_1_limb_10_col40.clone(),
                input_state_1_limb_11_col41.clone(),
                input_state_1_limb_12_col42.clone(),
                input_state_1_limb_13_col43.clone(),
                input_state_1_limb_14_col44.clone(),
                input_state_1_limb_15_col45.clone(),
                input_state_1_limb_16_col46.clone(),
                input_state_1_limb_17_col47.clone(),
                input_state_1_limb_18_col48.clone(),
                input_state_1_limb_19_col49.clone(),
                input_state_1_limb_20_col50.clone(),
                input_state_1_limb_21_col51.clone(),
                input_state_1_limb_22_col52.clone(),
                input_state_1_limb_23_col53.clone(),
                input_state_1_limb_24_col54.clone(),
                input_state_1_limb_25_col55.clone(),
                input_state_1_limb_26_col56.clone(),
                input_state_1_limb_27_col57.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        let packed_input_state_1_tmp_51986_7_limb_0 = eval.add_intermediate(
            ((input_state_1_limb_0_col30.clone()
                + (input_state_1_limb_1_col31.clone() * M31_512.clone()))
                + (input_state_1_limb_2_col32.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_1 = eval.add_intermediate(
            ((input_state_1_limb_3_col33.clone()
                + (input_state_1_limb_4_col34.clone() * M31_512.clone()))
                + (input_state_1_limb_5_col35.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_2 = eval.add_intermediate(
            ((input_state_1_limb_6_col36.clone()
                + (input_state_1_limb_7_col37.clone() * M31_512.clone()))
                + (input_state_1_limb_8_col38.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_3 = eval.add_intermediate(
            ((input_state_1_limb_9_col39.clone()
                + (input_state_1_limb_10_col40.clone() * M31_512.clone()))
                + (input_state_1_limb_11_col41.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_4 = eval.add_intermediate(
            ((input_state_1_limb_12_col42.clone()
                + (input_state_1_limb_13_col43.clone() * M31_512.clone()))
                + (input_state_1_limb_14_col44.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_5 = eval.add_intermediate(
            ((input_state_1_limb_15_col45.clone()
                + (input_state_1_limb_16_col46.clone() * M31_512.clone()))
                + (input_state_1_limb_17_col47.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_6 = eval.add_intermediate(
            ((input_state_1_limb_18_col48.clone()
                + (input_state_1_limb_19_col49.clone() * M31_512.clone()))
                + (input_state_1_limb_20_col50.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_7 = eval.add_intermediate(
            ((input_state_1_limb_21_col51.clone()
                + (input_state_1_limb_22_col52.clone() * M31_512.clone()))
                + (input_state_1_limb_23_col53.clone() * M31_262144.clone())),
        );
        let packed_input_state_1_tmp_51986_7_limb_8 = eval.add_intermediate(
            ((input_state_1_limb_24_col54.clone()
                + (input_state_1_limb_25_col55.clone() * M31_512.clone()))
                + (input_state_1_limb_26_col56.clone() * M31_262144.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_252_output_tmp_51986_10_limb_0, read_positive_num_bits_252_output_tmp_51986_10_limb_1, read_positive_num_bits_252_output_tmp_51986_10_limb_2, read_positive_num_bits_252_output_tmp_51986_10_limb_3, read_positive_num_bits_252_output_tmp_51986_10_limb_4, read_positive_num_bits_252_output_tmp_51986_10_limb_5, read_positive_num_bits_252_output_tmp_51986_10_limb_6, read_positive_num_bits_252_output_tmp_51986_10_limb_7, read_positive_num_bits_252_output_tmp_51986_10_limb_8, read_positive_num_bits_252_output_tmp_51986_10_limb_9, read_positive_num_bits_252_output_tmp_51986_10_limb_10, read_positive_num_bits_252_output_tmp_51986_10_limb_11, read_positive_num_bits_252_output_tmp_51986_10_limb_12, read_positive_num_bits_252_output_tmp_51986_10_limb_13, read_positive_num_bits_252_output_tmp_51986_10_limb_14, read_positive_num_bits_252_output_tmp_51986_10_limb_15, read_positive_num_bits_252_output_tmp_51986_10_limb_16, read_positive_num_bits_252_output_tmp_51986_10_limb_17, read_positive_num_bits_252_output_tmp_51986_10_limb_18, read_positive_num_bits_252_output_tmp_51986_10_limb_19, read_positive_num_bits_252_output_tmp_51986_10_limb_20, read_positive_num_bits_252_output_tmp_51986_10_limb_21, read_positive_num_bits_252_output_tmp_51986_10_limb_22, read_positive_num_bits_252_output_tmp_51986_10_limb_23, read_positive_num_bits_252_output_tmp_51986_10_limb_24, read_positive_num_bits_252_output_tmp_51986_10_limb_25, read_positive_num_bits_252_output_tmp_51986_10_limb_26, read_positive_num_bits_252_output_tmp_51986_10_limb_27, read_positive_num_bits_252_output_tmp_51986_10_limb_28] =
            ReadPositiveNumBits252::evaluate(
                ((E::F::from(M31::from(self.claim.poseidon_builtin_segment_start))
                    + (seq.clone() * M31_6.clone()))
                    + M31_2.clone()),
                input_state_2_id_col58.clone(),
                input_state_2_limb_0_col59.clone(),
                input_state_2_limb_1_col60.clone(),
                input_state_2_limb_2_col61.clone(),
                input_state_2_limb_3_col62.clone(),
                input_state_2_limb_4_col63.clone(),
                input_state_2_limb_5_col64.clone(),
                input_state_2_limb_6_col65.clone(),
                input_state_2_limb_7_col66.clone(),
                input_state_2_limb_8_col67.clone(),
                input_state_2_limb_9_col68.clone(),
                input_state_2_limb_10_col69.clone(),
                input_state_2_limb_11_col70.clone(),
                input_state_2_limb_12_col71.clone(),
                input_state_2_limb_13_col72.clone(),
                input_state_2_limb_14_col73.clone(),
                input_state_2_limb_15_col74.clone(),
                input_state_2_limb_16_col75.clone(),
                input_state_2_limb_17_col76.clone(),
                input_state_2_limb_18_col77.clone(),
                input_state_2_limb_19_col78.clone(),
                input_state_2_limb_20_col79.clone(),
                input_state_2_limb_21_col80.clone(),
                input_state_2_limb_22_col81.clone(),
                input_state_2_limb_23_col82.clone(),
                input_state_2_limb_24_col83.clone(),
                input_state_2_limb_25_col84.clone(),
                input_state_2_limb_26_col85.clone(),
                input_state_2_limb_27_col86.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        let packed_input_state_2_tmp_51986_11_limb_0 = eval.add_intermediate(
            ((input_state_2_limb_0_col59.clone()
                + (input_state_2_limb_1_col60.clone() * M31_512.clone()))
                + (input_state_2_limb_2_col61.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_1 = eval.add_intermediate(
            ((input_state_2_limb_3_col62.clone()
                + (input_state_2_limb_4_col63.clone() * M31_512.clone()))
                + (input_state_2_limb_5_col64.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_2 = eval.add_intermediate(
            ((input_state_2_limb_6_col65.clone()
                + (input_state_2_limb_7_col66.clone() * M31_512.clone()))
                + (input_state_2_limb_8_col67.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_3 = eval.add_intermediate(
            ((input_state_2_limb_9_col68.clone()
                + (input_state_2_limb_10_col69.clone() * M31_512.clone()))
                + (input_state_2_limb_11_col70.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_4 = eval.add_intermediate(
            ((input_state_2_limb_12_col71.clone()
                + (input_state_2_limb_13_col72.clone() * M31_512.clone()))
                + (input_state_2_limb_14_col73.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_5 = eval.add_intermediate(
            ((input_state_2_limb_15_col74.clone()
                + (input_state_2_limb_16_col75.clone() * M31_512.clone()))
                + (input_state_2_limb_17_col76.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_6 = eval.add_intermediate(
            ((input_state_2_limb_18_col77.clone()
                + (input_state_2_limb_19_col78.clone() * M31_512.clone()))
                + (input_state_2_limb_20_col79.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_7 = eval.add_intermediate(
            ((input_state_2_limb_21_col80.clone()
                + (input_state_2_limb_22_col81.clone() * M31_512.clone()))
                + (input_state_2_limb_23_col82.clone() * M31_262144.clone())),
        );
        let packed_input_state_2_tmp_51986_11_limb_8 = eval.add_intermediate(
            ((input_state_2_limb_24_col83.clone()
                + (input_state_2_limb_25_col84.clone() * M31_512.clone()))
                + (input_state_2_limb_26_col85.clone() * M31_262144.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [poseidon_hades_permutation_output_tmp_51986_163_limb_0, poseidon_hades_permutation_output_tmp_51986_163_limb_1, poseidon_hades_permutation_output_tmp_51986_163_limb_2, poseidon_hades_permutation_output_tmp_51986_163_limb_3, poseidon_hades_permutation_output_tmp_51986_163_limb_4, poseidon_hades_permutation_output_tmp_51986_163_limb_5, poseidon_hades_permutation_output_tmp_51986_163_limb_6, poseidon_hades_permutation_output_tmp_51986_163_limb_7, poseidon_hades_permutation_output_tmp_51986_163_limb_8, poseidon_hades_permutation_output_tmp_51986_163_limb_9, poseidon_hades_permutation_output_tmp_51986_163_limb_10, poseidon_hades_permutation_output_tmp_51986_163_limb_11, poseidon_hades_permutation_output_tmp_51986_163_limb_12, poseidon_hades_permutation_output_tmp_51986_163_limb_13, poseidon_hades_permutation_output_tmp_51986_163_limb_14, poseidon_hades_permutation_output_tmp_51986_163_limb_15, poseidon_hades_permutation_output_tmp_51986_163_limb_16, poseidon_hades_permutation_output_tmp_51986_163_limb_17, poseidon_hades_permutation_output_tmp_51986_163_limb_18, poseidon_hades_permutation_output_tmp_51986_163_limb_19, poseidon_hades_permutation_output_tmp_51986_163_limb_20, poseidon_hades_permutation_output_tmp_51986_163_limb_21, poseidon_hades_permutation_output_tmp_51986_163_limb_22, poseidon_hades_permutation_output_tmp_51986_163_limb_23, poseidon_hades_permutation_output_tmp_51986_163_limb_24, poseidon_hades_permutation_output_tmp_51986_163_limb_25, poseidon_hades_permutation_output_tmp_51986_163_limb_26, poseidon_hades_permutation_output_tmp_51986_163_limb_27, poseidon_hades_permutation_output_tmp_51986_163_limb_28, poseidon_hades_permutation_output_tmp_51986_163_limb_29] =
            PoseidonHadesPermutation::evaluate(
                [
                    packed_input_state_0_tmp_51986_3_limb_0.clone(),
                    packed_input_state_0_tmp_51986_3_limb_1.clone(),
                    packed_input_state_0_tmp_51986_3_limb_2.clone(),
                    packed_input_state_0_tmp_51986_3_limb_3.clone(),
                    packed_input_state_0_tmp_51986_3_limb_4.clone(),
                    packed_input_state_0_tmp_51986_3_limb_5.clone(),
                    packed_input_state_0_tmp_51986_3_limb_6.clone(),
                    packed_input_state_0_tmp_51986_3_limb_7.clone(),
                    packed_input_state_0_tmp_51986_3_limb_8.clone(),
                    input_state_0_limb_27_col28.clone(),
                    packed_input_state_1_tmp_51986_7_limb_0.clone(),
                    packed_input_state_1_tmp_51986_7_limb_1.clone(),
                    packed_input_state_1_tmp_51986_7_limb_2.clone(),
                    packed_input_state_1_tmp_51986_7_limb_3.clone(),
                    packed_input_state_1_tmp_51986_7_limb_4.clone(),
                    packed_input_state_1_tmp_51986_7_limb_5.clone(),
                    packed_input_state_1_tmp_51986_7_limb_6.clone(),
                    packed_input_state_1_tmp_51986_7_limb_7.clone(),
                    packed_input_state_1_tmp_51986_7_limb_8.clone(),
                    input_state_1_limb_27_col57.clone(),
                    packed_input_state_2_tmp_51986_11_limb_0.clone(),
                    packed_input_state_2_tmp_51986_11_limb_1.clone(),
                    packed_input_state_2_tmp_51986_11_limb_2.clone(),
                    packed_input_state_2_tmp_51986_11_limb_3.clone(),
                    packed_input_state_2_tmp_51986_11_limb_4.clone(),
                    packed_input_state_2_tmp_51986_11_limb_5.clone(),
                    packed_input_state_2_tmp_51986_11_limb_6.clone(),
                    packed_input_state_2_tmp_51986_11_limb_7.clone(),
                    packed_input_state_2_tmp_51986_11_limb_8.clone(),
                    input_state_2_limb_27_col86.clone(),
                ],
                combination_limb_0_col87.clone(),
                combination_limb_1_col88.clone(),
                combination_limb_2_col89.clone(),
                combination_limb_3_col90.clone(),
                combination_limb_4_col91.clone(),
                combination_limb_5_col92.clone(),
                combination_limb_6_col93.clone(),
                combination_limb_7_col94.clone(),
                combination_limb_8_col95.clone(),
                combination_limb_9_col96.clone(),
                p_coef_col97.clone(),
                combination_limb_0_col98.clone(),
                combination_limb_1_col99.clone(),
                combination_limb_2_col100.clone(),
                combination_limb_3_col101.clone(),
                combination_limb_4_col102.clone(),
                combination_limb_5_col103.clone(),
                combination_limb_6_col104.clone(),
                combination_limb_7_col105.clone(),
                combination_limb_8_col106.clone(),
                combination_limb_9_col107.clone(),
                p_coef_col108.clone(),
                combination_limb_0_col109.clone(),
                combination_limb_1_col110.clone(),
                combination_limb_2_col111.clone(),
                combination_limb_3_col112.clone(),
                combination_limb_4_col113.clone(),
                combination_limb_5_col114.clone(),
                combination_limb_6_col115.clone(),
                combination_limb_7_col116.clone(),
                combination_limb_8_col117.clone(),
                combination_limb_9_col118.clone(),
                p_coef_col119.clone(),
                poseidon_full_round_chain_output_limb_0_col120.clone(),
                poseidon_full_round_chain_output_limb_1_col121.clone(),
                poseidon_full_round_chain_output_limb_2_col122.clone(),
                poseidon_full_round_chain_output_limb_3_col123.clone(),
                poseidon_full_round_chain_output_limb_4_col124.clone(),
                poseidon_full_round_chain_output_limb_5_col125.clone(),
                poseidon_full_round_chain_output_limb_6_col126.clone(),
                poseidon_full_round_chain_output_limb_7_col127.clone(),
                poseidon_full_round_chain_output_limb_8_col128.clone(),
                poseidon_full_round_chain_output_limb_9_col129.clone(),
                poseidon_full_round_chain_output_limb_10_col130.clone(),
                poseidon_full_round_chain_output_limb_11_col131.clone(),
                poseidon_full_round_chain_output_limb_12_col132.clone(),
                poseidon_full_round_chain_output_limb_13_col133.clone(),
                poseidon_full_round_chain_output_limb_14_col134.clone(),
                poseidon_full_round_chain_output_limb_15_col135.clone(),
                poseidon_full_round_chain_output_limb_16_col136.clone(),
                poseidon_full_round_chain_output_limb_17_col137.clone(),
                poseidon_full_round_chain_output_limb_18_col138.clone(),
                poseidon_full_round_chain_output_limb_19_col139.clone(),
                poseidon_full_round_chain_output_limb_20_col140.clone(),
                poseidon_full_round_chain_output_limb_21_col141.clone(),
                poseidon_full_round_chain_output_limb_22_col142.clone(),
                poseidon_full_round_chain_output_limb_23_col143.clone(),
                poseidon_full_round_chain_output_limb_24_col144.clone(),
                poseidon_full_round_chain_output_limb_25_col145.clone(),
                poseidon_full_round_chain_output_limb_26_col146.clone(),
                poseidon_full_round_chain_output_limb_27_col147.clone(),
                poseidon_full_round_chain_output_limb_28_col148.clone(),
                poseidon_full_round_chain_output_limb_29_col149.clone(),
                poseidon_full_round_chain_output_limb_30_col150.clone(),
                poseidon_full_round_chain_output_limb_31_col151.clone(),
                cube_252_output_limb_0_col152.clone(),
                cube_252_output_limb_1_col153.clone(),
                cube_252_output_limb_2_col154.clone(),
                cube_252_output_limb_3_col155.clone(),
                cube_252_output_limb_4_col156.clone(),
                cube_252_output_limb_5_col157.clone(),
                cube_252_output_limb_6_col158.clone(),
                cube_252_output_limb_7_col159.clone(),
                cube_252_output_limb_8_col160.clone(),
                cube_252_output_limb_9_col161.clone(),
                combination_limb_0_col162.clone(),
                combination_limb_1_col163.clone(),
                combination_limb_2_col164.clone(),
                combination_limb_3_col165.clone(),
                combination_limb_4_col166.clone(),
                combination_limb_5_col167.clone(),
                combination_limb_6_col168.clone(),
                combination_limb_7_col169.clone(),
                combination_limb_8_col170.clone(),
                combination_limb_9_col171.clone(),
                p_coef_col172.clone(),
                cube_252_output_limb_0_col173.clone(),
                cube_252_output_limb_1_col174.clone(),
                cube_252_output_limb_2_col175.clone(),
                cube_252_output_limb_3_col176.clone(),
                cube_252_output_limb_4_col177.clone(),
                cube_252_output_limb_5_col178.clone(),
                cube_252_output_limb_6_col179.clone(),
                cube_252_output_limb_7_col180.clone(),
                cube_252_output_limb_8_col181.clone(),
                cube_252_output_limb_9_col182.clone(),
                combination_limb_0_col183.clone(),
                combination_limb_1_col184.clone(),
                combination_limb_2_col185.clone(),
                combination_limb_3_col186.clone(),
                combination_limb_4_col187.clone(),
                combination_limb_5_col188.clone(),
                combination_limb_6_col189.clone(),
                combination_limb_7_col190.clone(),
                combination_limb_8_col191.clone(),
                combination_limb_9_col192.clone(),
                p_coef_col193.clone(),
                poseidon_3_partial_rounds_chain_output_limb_0_col194.clone(),
                poseidon_3_partial_rounds_chain_output_limb_1_col195.clone(),
                poseidon_3_partial_rounds_chain_output_limb_2_col196.clone(),
                poseidon_3_partial_rounds_chain_output_limb_3_col197.clone(),
                poseidon_3_partial_rounds_chain_output_limb_4_col198.clone(),
                poseidon_3_partial_rounds_chain_output_limb_5_col199.clone(),
                poseidon_3_partial_rounds_chain_output_limb_6_col200.clone(),
                poseidon_3_partial_rounds_chain_output_limb_7_col201.clone(),
                poseidon_3_partial_rounds_chain_output_limb_8_col202.clone(),
                poseidon_3_partial_rounds_chain_output_limb_9_col203.clone(),
                poseidon_3_partial_rounds_chain_output_limb_10_col204.clone(),
                poseidon_3_partial_rounds_chain_output_limb_11_col205.clone(),
                poseidon_3_partial_rounds_chain_output_limb_12_col206.clone(),
                poseidon_3_partial_rounds_chain_output_limb_13_col207.clone(),
                poseidon_3_partial_rounds_chain_output_limb_14_col208.clone(),
                poseidon_3_partial_rounds_chain_output_limb_15_col209.clone(),
                poseidon_3_partial_rounds_chain_output_limb_16_col210.clone(),
                poseidon_3_partial_rounds_chain_output_limb_17_col211.clone(),
                poseidon_3_partial_rounds_chain_output_limb_18_col212.clone(),
                poseidon_3_partial_rounds_chain_output_limb_19_col213.clone(),
                poseidon_3_partial_rounds_chain_output_limb_20_col214.clone(),
                poseidon_3_partial_rounds_chain_output_limb_21_col215.clone(),
                poseidon_3_partial_rounds_chain_output_limb_22_col216.clone(),
                poseidon_3_partial_rounds_chain_output_limb_23_col217.clone(),
                poseidon_3_partial_rounds_chain_output_limb_24_col218.clone(),
                poseidon_3_partial_rounds_chain_output_limb_25_col219.clone(),
                poseidon_3_partial_rounds_chain_output_limb_26_col220.clone(),
                poseidon_3_partial_rounds_chain_output_limb_27_col221.clone(),
                poseidon_3_partial_rounds_chain_output_limb_28_col222.clone(),
                poseidon_3_partial_rounds_chain_output_limb_29_col223.clone(),
                poseidon_3_partial_rounds_chain_output_limb_30_col224.clone(),
                poseidon_3_partial_rounds_chain_output_limb_31_col225.clone(),
                poseidon_3_partial_rounds_chain_output_limb_32_col226.clone(),
                poseidon_3_partial_rounds_chain_output_limb_33_col227.clone(),
                poseidon_3_partial_rounds_chain_output_limb_34_col228.clone(),
                poseidon_3_partial_rounds_chain_output_limb_35_col229.clone(),
                poseidon_3_partial_rounds_chain_output_limb_36_col230.clone(),
                poseidon_3_partial_rounds_chain_output_limb_37_col231.clone(),
                poseidon_3_partial_rounds_chain_output_limb_38_col232.clone(),
                poseidon_3_partial_rounds_chain_output_limb_39_col233.clone(),
                poseidon_3_partial_rounds_chain_output_limb_40_col234.clone(),
                poseidon_3_partial_rounds_chain_output_limb_41_col235.clone(),
                combination_limb_0_col236.clone(),
                combination_limb_1_col237.clone(),
                combination_limb_2_col238.clone(),
                combination_limb_3_col239.clone(),
                combination_limb_4_col240.clone(),
                combination_limb_5_col241.clone(),
                combination_limb_6_col242.clone(),
                combination_limb_7_col243.clone(),
                combination_limb_8_col244.clone(),
                combination_limb_9_col245.clone(),
                p_coef_col246.clone(),
                combination_limb_0_col247.clone(),
                combination_limb_1_col248.clone(),
                combination_limb_2_col249.clone(),
                combination_limb_3_col250.clone(),
                combination_limb_4_col251.clone(),
                combination_limb_5_col252.clone(),
                combination_limb_6_col253.clone(),
                combination_limb_7_col254.clone(),
                combination_limb_8_col255.clone(),
                combination_limb_9_col256.clone(),
                p_coef_col257.clone(),
                poseidon_full_round_chain_output_limb_0_col258.clone(),
                poseidon_full_round_chain_output_limb_1_col259.clone(),
                poseidon_full_round_chain_output_limb_2_col260.clone(),
                poseidon_full_round_chain_output_limb_3_col261.clone(),
                poseidon_full_round_chain_output_limb_4_col262.clone(),
                poseidon_full_round_chain_output_limb_5_col263.clone(),
                poseidon_full_round_chain_output_limb_6_col264.clone(),
                poseidon_full_round_chain_output_limb_7_col265.clone(),
                poseidon_full_round_chain_output_limb_8_col266.clone(),
                poseidon_full_round_chain_output_limb_9_col267.clone(),
                poseidon_full_round_chain_output_limb_10_col268.clone(),
                poseidon_full_round_chain_output_limb_11_col269.clone(),
                poseidon_full_round_chain_output_limb_12_col270.clone(),
                poseidon_full_round_chain_output_limb_13_col271.clone(),
                poseidon_full_round_chain_output_limb_14_col272.clone(),
                poseidon_full_round_chain_output_limb_15_col273.clone(),
                poseidon_full_round_chain_output_limb_16_col274.clone(),
                poseidon_full_round_chain_output_limb_17_col275.clone(),
                poseidon_full_round_chain_output_limb_18_col276.clone(),
                poseidon_full_round_chain_output_limb_19_col277.clone(),
                poseidon_full_round_chain_output_limb_20_col278.clone(),
                poseidon_full_round_chain_output_limb_21_col279.clone(),
                poseidon_full_round_chain_output_limb_22_col280.clone(),
                poseidon_full_round_chain_output_limb_23_col281.clone(),
                poseidon_full_round_chain_output_limb_24_col282.clone(),
                poseidon_full_round_chain_output_limb_25_col283.clone(),
                poseidon_full_round_chain_output_limb_26_col284.clone(),
                poseidon_full_round_chain_output_limb_27_col285.clone(),
                poseidon_full_round_chain_output_limb_28_col286.clone(),
                poseidon_full_round_chain_output_limb_29_col287.clone(),
                poseidon_full_round_chain_output_limb_30_col288.clone(),
                poseidon_full_round_chain_output_limb_31_col289.clone(),
                &mut eval,
                &self.poseidon_full_round_chain_lookup_elements,
                &self.range_check_felt_252_width_27_lookup_elements,
                &self.cube_252_lookup_elements,
                &self.range_check_3_3_3_3_3_lookup_elements,
                &self.range_check_4_4_4_4_lookup_elements,
                &self.range_check_4_4_lookup_elements,
                &self.poseidon_3_partial_rounds_chain_lookup_elements,
                seq.clone(),
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_output_tmp_51986_165_limb_0, felt_252_unpack_from_27_output_tmp_51986_165_limb_1, felt_252_unpack_from_27_output_tmp_51986_165_limb_2, felt_252_unpack_from_27_output_tmp_51986_165_limb_3, felt_252_unpack_from_27_output_tmp_51986_165_limb_4, felt_252_unpack_from_27_output_tmp_51986_165_limb_5, felt_252_unpack_from_27_output_tmp_51986_165_limb_6, felt_252_unpack_from_27_output_tmp_51986_165_limb_7, felt_252_unpack_from_27_output_tmp_51986_165_limb_8, felt_252_unpack_from_27_output_tmp_51986_165_limb_9, felt_252_unpack_from_27_output_tmp_51986_165_limb_10, felt_252_unpack_from_27_output_tmp_51986_165_limb_11, felt_252_unpack_from_27_output_tmp_51986_165_limb_12, felt_252_unpack_from_27_output_tmp_51986_165_limb_13, felt_252_unpack_from_27_output_tmp_51986_165_limb_14, felt_252_unpack_from_27_output_tmp_51986_165_limb_15, felt_252_unpack_from_27_output_tmp_51986_165_limb_16, felt_252_unpack_from_27_output_tmp_51986_165_limb_17, felt_252_unpack_from_27_output_tmp_51986_165_limb_18, felt_252_unpack_from_27_output_tmp_51986_165_limb_19, felt_252_unpack_from_27_output_tmp_51986_165_limb_20, felt_252_unpack_from_27_output_tmp_51986_165_limb_21, felt_252_unpack_from_27_output_tmp_51986_165_limb_22, felt_252_unpack_from_27_output_tmp_51986_165_limb_23, felt_252_unpack_from_27_output_tmp_51986_165_limb_24, felt_252_unpack_from_27_output_tmp_51986_165_limb_25, felt_252_unpack_from_27_output_tmp_51986_165_limb_26, felt_252_unpack_from_27_output_tmp_51986_165_limb_27] =
            Felt252UnpackFrom27::evaluate(
                [
                    poseidon_full_round_chain_output_limb_2_col260.clone(),
                    poseidon_full_round_chain_output_limb_3_col261.clone(),
                    poseidon_full_round_chain_output_limb_4_col262.clone(),
                    poseidon_full_round_chain_output_limb_5_col263.clone(),
                    poseidon_full_round_chain_output_limb_6_col264.clone(),
                    poseidon_full_round_chain_output_limb_7_col265.clone(),
                    poseidon_full_round_chain_output_limb_8_col266.clone(),
                    poseidon_full_round_chain_output_limb_9_col267.clone(),
                    poseidon_full_round_chain_output_limb_10_col268.clone(),
                    poseidon_full_round_chain_output_limb_11_col269.clone(),
                ],
                unpacked_limb_0_col290.clone(),
                unpacked_limb_1_col291.clone(),
                unpacked_limb_3_col292.clone(),
                unpacked_limb_4_col293.clone(),
                unpacked_limb_6_col294.clone(),
                unpacked_limb_7_col295.clone(),
                unpacked_limb_9_col296.clone(),
                unpacked_limb_10_col297.clone(),
                unpacked_limb_12_col298.clone(),
                unpacked_limb_13_col299.clone(),
                unpacked_limb_15_col300.clone(),
                unpacked_limb_16_col301.clone(),
                unpacked_limb_18_col302.clone(),
                unpacked_limb_19_col303.clone(),
                unpacked_limb_21_col304.clone(),
                unpacked_limb_22_col305.clone(),
                unpacked_limb_24_col306.clone(),
                unpacked_limb_25_col307.clone(),
                &mut eval,
            );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.poseidon_builtin_segment_start))
                    + (seq.clone() * M31_6.clone()))
                    + M31_3.clone()),
                unpacked_limb_0_col290.clone(),
                unpacked_limb_1_col291.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_2.clone(),
                unpacked_limb_3_col292.clone(),
                unpacked_limb_4_col293.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_5.clone(),
                unpacked_limb_6_col294.clone(),
                unpacked_limb_7_col295.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_8.clone(),
                unpacked_limb_9_col296.clone(),
                unpacked_limb_10_col297.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_11.clone(),
                unpacked_limb_12_col298.clone(),
                unpacked_limb_13_col299.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_14.clone(),
                unpacked_limb_15_col300.clone(),
                unpacked_limb_16_col301.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_17.clone(),
                unpacked_limb_18_col302.clone(),
                unpacked_limb_19_col303.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_20.clone(),
                unpacked_limb_21_col304.clone(),
                unpacked_limb_22_col305.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_23.clone(),
                unpacked_limb_24_col306.clone(),
                unpacked_limb_25_col307.clone(),
                felt_252_unpack_from_27_output_tmp_51986_165_limb_26.clone(),
                poseidon_full_round_chain_output_limb_11_col269.clone(),
            ],
            output_state_0_id_col308.clone(),
            &mut eval,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_output_tmp_51986_168_limb_0, felt_252_unpack_from_27_output_tmp_51986_168_limb_1, felt_252_unpack_from_27_output_tmp_51986_168_limb_2, felt_252_unpack_from_27_output_tmp_51986_168_limb_3, felt_252_unpack_from_27_output_tmp_51986_168_limb_4, felt_252_unpack_from_27_output_tmp_51986_168_limb_5, felt_252_unpack_from_27_output_tmp_51986_168_limb_6, felt_252_unpack_from_27_output_tmp_51986_168_limb_7, felt_252_unpack_from_27_output_tmp_51986_168_limb_8, felt_252_unpack_from_27_output_tmp_51986_168_limb_9, felt_252_unpack_from_27_output_tmp_51986_168_limb_10, felt_252_unpack_from_27_output_tmp_51986_168_limb_11, felt_252_unpack_from_27_output_tmp_51986_168_limb_12, felt_252_unpack_from_27_output_tmp_51986_168_limb_13, felt_252_unpack_from_27_output_tmp_51986_168_limb_14, felt_252_unpack_from_27_output_tmp_51986_168_limb_15, felt_252_unpack_from_27_output_tmp_51986_168_limb_16, felt_252_unpack_from_27_output_tmp_51986_168_limb_17, felt_252_unpack_from_27_output_tmp_51986_168_limb_18, felt_252_unpack_from_27_output_tmp_51986_168_limb_19, felt_252_unpack_from_27_output_tmp_51986_168_limb_20, felt_252_unpack_from_27_output_tmp_51986_168_limb_21, felt_252_unpack_from_27_output_tmp_51986_168_limb_22, felt_252_unpack_from_27_output_tmp_51986_168_limb_23, felt_252_unpack_from_27_output_tmp_51986_168_limb_24, felt_252_unpack_from_27_output_tmp_51986_168_limb_25, felt_252_unpack_from_27_output_tmp_51986_168_limb_26, felt_252_unpack_from_27_output_tmp_51986_168_limb_27] =
            Felt252UnpackFrom27::evaluate(
                [
                    poseidon_full_round_chain_output_limb_12_col270.clone(),
                    poseidon_full_round_chain_output_limb_13_col271.clone(),
                    poseidon_full_round_chain_output_limb_14_col272.clone(),
                    poseidon_full_round_chain_output_limb_15_col273.clone(),
                    poseidon_full_round_chain_output_limb_16_col274.clone(),
                    poseidon_full_round_chain_output_limb_17_col275.clone(),
                    poseidon_full_round_chain_output_limb_18_col276.clone(),
                    poseidon_full_round_chain_output_limb_19_col277.clone(),
                    poseidon_full_round_chain_output_limb_20_col278.clone(),
                    poseidon_full_round_chain_output_limb_21_col279.clone(),
                ],
                unpacked_limb_0_col309.clone(),
                unpacked_limb_1_col310.clone(),
                unpacked_limb_3_col311.clone(),
                unpacked_limb_4_col312.clone(),
                unpacked_limb_6_col313.clone(),
                unpacked_limb_7_col314.clone(),
                unpacked_limb_9_col315.clone(),
                unpacked_limb_10_col316.clone(),
                unpacked_limb_12_col317.clone(),
                unpacked_limb_13_col318.clone(),
                unpacked_limb_15_col319.clone(),
                unpacked_limb_16_col320.clone(),
                unpacked_limb_18_col321.clone(),
                unpacked_limb_19_col322.clone(),
                unpacked_limb_21_col323.clone(),
                unpacked_limb_22_col324.clone(),
                unpacked_limb_24_col325.clone(),
                unpacked_limb_25_col326.clone(),
                &mut eval,
            );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.poseidon_builtin_segment_start))
                    + (seq.clone() * M31_6.clone()))
                    + M31_4.clone()),
                unpacked_limb_0_col309.clone(),
                unpacked_limb_1_col310.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_2.clone(),
                unpacked_limb_3_col311.clone(),
                unpacked_limb_4_col312.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_5.clone(),
                unpacked_limb_6_col313.clone(),
                unpacked_limb_7_col314.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_8.clone(),
                unpacked_limb_9_col315.clone(),
                unpacked_limb_10_col316.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_11.clone(),
                unpacked_limb_12_col317.clone(),
                unpacked_limb_13_col318.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_14.clone(),
                unpacked_limb_15_col319.clone(),
                unpacked_limb_16_col320.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_17.clone(),
                unpacked_limb_18_col321.clone(),
                unpacked_limb_19_col322.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_20.clone(),
                unpacked_limb_21_col323.clone(),
                unpacked_limb_22_col324.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_23.clone(),
                unpacked_limb_24_col325.clone(),
                unpacked_limb_25_col326.clone(),
                felt_252_unpack_from_27_output_tmp_51986_168_limb_26.clone(),
                poseidon_full_round_chain_output_limb_21_col279.clone(),
            ],
            output_state_1_id_col327.clone(),
            &mut eval,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [felt_252_unpack_from_27_output_tmp_51986_171_limb_0, felt_252_unpack_from_27_output_tmp_51986_171_limb_1, felt_252_unpack_from_27_output_tmp_51986_171_limb_2, felt_252_unpack_from_27_output_tmp_51986_171_limb_3, felt_252_unpack_from_27_output_tmp_51986_171_limb_4, felt_252_unpack_from_27_output_tmp_51986_171_limb_5, felt_252_unpack_from_27_output_tmp_51986_171_limb_6, felt_252_unpack_from_27_output_tmp_51986_171_limb_7, felt_252_unpack_from_27_output_tmp_51986_171_limb_8, felt_252_unpack_from_27_output_tmp_51986_171_limb_9, felt_252_unpack_from_27_output_tmp_51986_171_limb_10, felt_252_unpack_from_27_output_tmp_51986_171_limb_11, felt_252_unpack_from_27_output_tmp_51986_171_limb_12, felt_252_unpack_from_27_output_tmp_51986_171_limb_13, felt_252_unpack_from_27_output_tmp_51986_171_limb_14, felt_252_unpack_from_27_output_tmp_51986_171_limb_15, felt_252_unpack_from_27_output_tmp_51986_171_limb_16, felt_252_unpack_from_27_output_tmp_51986_171_limb_17, felt_252_unpack_from_27_output_tmp_51986_171_limb_18, felt_252_unpack_from_27_output_tmp_51986_171_limb_19, felt_252_unpack_from_27_output_tmp_51986_171_limb_20, felt_252_unpack_from_27_output_tmp_51986_171_limb_21, felt_252_unpack_from_27_output_tmp_51986_171_limb_22, felt_252_unpack_from_27_output_tmp_51986_171_limb_23, felt_252_unpack_from_27_output_tmp_51986_171_limb_24, felt_252_unpack_from_27_output_tmp_51986_171_limb_25, felt_252_unpack_from_27_output_tmp_51986_171_limb_26, felt_252_unpack_from_27_output_tmp_51986_171_limb_27] =
            Felt252UnpackFrom27::evaluate(
                [
                    poseidon_full_round_chain_output_limb_22_col280.clone(),
                    poseidon_full_round_chain_output_limb_23_col281.clone(),
                    poseidon_full_round_chain_output_limb_24_col282.clone(),
                    poseidon_full_round_chain_output_limb_25_col283.clone(),
                    poseidon_full_round_chain_output_limb_26_col284.clone(),
                    poseidon_full_round_chain_output_limb_27_col285.clone(),
                    poseidon_full_round_chain_output_limb_28_col286.clone(),
                    poseidon_full_round_chain_output_limb_29_col287.clone(),
                    poseidon_full_round_chain_output_limb_30_col288.clone(),
                    poseidon_full_round_chain_output_limb_31_col289.clone(),
                ],
                unpacked_limb_0_col328.clone(),
                unpacked_limb_1_col329.clone(),
                unpacked_limb_3_col330.clone(),
                unpacked_limb_4_col331.clone(),
                unpacked_limb_6_col332.clone(),
                unpacked_limb_7_col333.clone(),
                unpacked_limb_9_col334.clone(),
                unpacked_limb_10_col335.clone(),
                unpacked_limb_12_col336.clone(),
                unpacked_limb_13_col337.clone(),
                unpacked_limb_15_col338.clone(),
                unpacked_limb_16_col339.clone(),
                unpacked_limb_18_col340.clone(),
                unpacked_limb_19_col341.clone(),
                unpacked_limb_21_col342.clone(),
                unpacked_limb_22_col343.clone(),
                unpacked_limb_24_col344.clone(),
                unpacked_limb_25_col345.clone(),
                &mut eval,
            );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.poseidon_builtin_segment_start))
                    + (seq.clone() * M31_6.clone()))
                    + M31_5.clone()),
                unpacked_limb_0_col328.clone(),
                unpacked_limb_1_col329.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_2.clone(),
                unpacked_limb_3_col330.clone(),
                unpacked_limb_4_col331.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_5.clone(),
                unpacked_limb_6_col332.clone(),
                unpacked_limb_7_col333.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_8.clone(),
                unpacked_limb_9_col334.clone(),
                unpacked_limb_10_col335.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_11.clone(),
                unpacked_limb_12_col336.clone(),
                unpacked_limb_13_col337.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_14.clone(),
                unpacked_limb_15_col338.clone(),
                unpacked_limb_16_col339.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_17.clone(),
                unpacked_limb_18_col340.clone(),
                unpacked_limb_19_col341.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_20.clone(),
                unpacked_limb_21_col342.clone(),
                unpacked_limb_22_col343.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_23.clone(),
                unpacked_limb_24_col344.clone(),
                unpacked_limb_25_col345.clone(),
                felt_252_unpack_from_27_output_tmp_51986_171_limb_26.clone(),
                poseidon_full_round_chain_output_limb_31_col289.clone(),
            ],
            output_state_2_id_col346.clone(),
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
    use crate::components::constraints_regression_test_values::POSEIDON_BUILTIN;

    #[test]
    fn poseidon_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                poseidon_builtin_segment_start: rng.gen::<u32>(),
            },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            poseidon_full_round_chain_lookup_elements: relations::PoseidonFullRoundChain::dummy(),
            range_check_felt_252_width_27_lookup_elements:
                relations::RangeCheckFelt252Width27::dummy(),
            cube_252_lookup_elements: relations::Cube252::dummy(),
            range_check_3_3_3_3_3_lookup_elements: relations::RangeCheck_3_3_3_3_3::dummy(),
            range_check_4_4_4_4_lookup_elements: relations::RangeCheck_4_4_4_4::dummy(),
            range_check_4_4_lookup_elements: relations::RangeCheck_4_4::dummy(),
            poseidon_3_partial_rounds_chain_lookup_elements:
                relations::Poseidon3PartialRoundsChain::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, POSEIDON_BUILTIN);
    }
}

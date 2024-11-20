#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::ops::{Mul, Sub};

use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::{ClaimedPrefixSum, LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, INTERACTION_TRACE_IDX,
};
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use crate::components::memory::{addr_to_id, id_to_f252};
use crate::components::opcodes::VmRelationElements;
use crate::components::range_check_vector::{range_check_19, range_check_9_9};
use crate::components::{opcodes, pack_values, verifyinstruction};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationElements(LookupElements<4>);
impl RelationElements {
    pub fn draw(channel: &mut impl Channel) -> Self {
        Self(LookupElements::<4>::draw(channel))
    }
    pub fn combine<F: Clone, EF>(&self, values: &[F]) -> EF
    where
        EF: Clone + Zero + From<F> + From<SecureField> + Mul<F, Output = EF> + Sub<EF, Output = EF>,
    {
        self.0.combine(values)
    }
}

pub struct Eval {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub memoryaddresstoid_lookup_elements: addr_to_id::RelationElements,
    pub memoryidtobig_lookup_elements: id_to_f252::RelationElements,
    pub range_check_19_lookup_elements: range_check_19::RelationElements,
    pub range_check_9_9_lookup_elements: range_check_9_9::RelationElements,
    pub verifyinstruction_lookup_elements: verifyinstruction::RelationElements,
    pub opcodes_lookup_elements: VmRelationElements,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub n_calls: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let trace_log_sizes = vec![log_size; 229];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 63];
        let preprocessed_log_sizes = vec![log_size];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.n_calls as u64);
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InteractionClaim {
    pub total_sum: SecureField,
    pub claimed_sum: Option<ClaimedPrefixSum>,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.total_sum]);
        if let Some(claimed_sum) = self.claimed_sum {
            channel.mix_felts(&[claimed_sum.0]);
            channel.mix_u64(claimed_sum.1 as u64);
        }
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        std::cmp::max(self.claim.n_calls.next_power_of_two().ilog2(), LOG_N_LANES)
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
        let M31_131072 = E::F::from(M31::from(131072));
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4 = E::F::from(M31::from(4));
        let M31_4194304 = E::F::from(M31::from(4194304));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));
        let is_first = eval.get_preprocessed_column(PreprocessedColumn::IsFirst(self.log_size()));
        let mut logup = LogupAtRow::<E>::new(
            INTERACTION_TRACE_IDX,
            self.interaction_claim.total_sum,
            self.interaction_claim.claimed_sum,
            is_first,
        );
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let offset2_col5 = eval.next_trace_mask();
        let dst_base_fp_col6 = eval.next_trace_mask();
        let op0_base_fp_col7 = eval.next_trace_mask();
        let op1_imm_col8 = eval.next_trace_mask();
        let op1_base_fp_col9 = eval.next_trace_mask();
        let op1_base_ap_col10 = eval.next_trace_mask();
        let res_add_col11 = eval.next_trace_mask();
        let res_mul_col12 = eval.next_trace_mask();
        let pc_update_jump_col13 = eval.next_trace_mask();
        let pc_update_jump_rel_col14 = eval.next_trace_mask();
        let pc_update_jnz_col15 = eval.next_trace_mask();
        let ap_update_add_col16 = eval.next_trace_mask();
        let ap_update_add_1_col17 = eval.next_trace_mask();
        let opcode_call_col18 = eval.next_trace_mask();
        let opcode_ret_col19 = eval.next_trace_mask();
        let opcode_assert_eq_col20 = eval.next_trace_mask();
        let dst_id_col21 = eval.next_trace_mask();
        let dst_limb_0_col22 = eval.next_trace_mask();
        let dst_limb_1_col23 = eval.next_trace_mask();
        let dst_limb_2_col24 = eval.next_trace_mask();
        let dst_limb_3_col25 = eval.next_trace_mask();
        let dst_limb_4_col26 = eval.next_trace_mask();
        let dst_limb_5_col27 = eval.next_trace_mask();
        let dst_limb_6_col28 = eval.next_trace_mask();
        let dst_limb_7_col29 = eval.next_trace_mask();
        let dst_limb_8_col30 = eval.next_trace_mask();
        let dst_limb_9_col31 = eval.next_trace_mask();
        let dst_limb_10_col32 = eval.next_trace_mask();
        let dst_limb_11_col33 = eval.next_trace_mask();
        let dst_limb_12_col34 = eval.next_trace_mask();
        let dst_limb_13_col35 = eval.next_trace_mask();
        let dst_limb_14_col36 = eval.next_trace_mask();
        let dst_limb_15_col37 = eval.next_trace_mask();
        let dst_limb_16_col38 = eval.next_trace_mask();
        let dst_limb_17_col39 = eval.next_trace_mask();
        let dst_limb_18_col40 = eval.next_trace_mask();
        let dst_limb_19_col41 = eval.next_trace_mask();
        let dst_limb_20_col42 = eval.next_trace_mask();
        let dst_limb_21_col43 = eval.next_trace_mask();
        let dst_limb_22_col44 = eval.next_trace_mask();
        let dst_limb_23_col45 = eval.next_trace_mask();
        let dst_limb_24_col46 = eval.next_trace_mask();
        let dst_limb_25_col47 = eval.next_trace_mask();
        let dst_limb_26_col48 = eval.next_trace_mask();
        let dst_limb_27_col49 = eval.next_trace_mask();
        let op0_id_col50 = eval.next_trace_mask();
        let op0_limb_0_col51 = eval.next_trace_mask();
        let op0_limb_1_col52 = eval.next_trace_mask();
        let op0_limb_2_col53 = eval.next_trace_mask();
        let op0_limb_3_col54 = eval.next_trace_mask();
        let op0_limb_4_col55 = eval.next_trace_mask();
        let op0_limb_5_col56 = eval.next_trace_mask();
        let op0_limb_6_col57 = eval.next_trace_mask();
        let op0_limb_7_col58 = eval.next_trace_mask();
        let op0_limb_8_col59 = eval.next_trace_mask();
        let op0_limb_9_col60 = eval.next_trace_mask();
        let op0_limb_10_col61 = eval.next_trace_mask();
        let op0_limb_11_col62 = eval.next_trace_mask();
        let op0_limb_12_col63 = eval.next_trace_mask();
        let op0_limb_13_col64 = eval.next_trace_mask();
        let op0_limb_14_col65 = eval.next_trace_mask();
        let op0_limb_15_col66 = eval.next_trace_mask();
        let op0_limb_16_col67 = eval.next_trace_mask();
        let op0_limb_17_col68 = eval.next_trace_mask();
        let op0_limb_18_col69 = eval.next_trace_mask();
        let op0_limb_19_col70 = eval.next_trace_mask();
        let op0_limb_20_col71 = eval.next_trace_mask();
        let op0_limb_21_col72 = eval.next_trace_mask();
        let op0_limb_22_col73 = eval.next_trace_mask();
        let op0_limb_23_col74 = eval.next_trace_mask();
        let op0_limb_24_col75 = eval.next_trace_mask();
        let op0_limb_25_col76 = eval.next_trace_mask();
        let op0_limb_26_col77 = eval.next_trace_mask();
        let op0_limb_27_col78 = eval.next_trace_mask();
        let op1_id_col79 = eval.next_trace_mask();
        let op1_limb_0_col80 = eval.next_trace_mask();
        let op1_limb_1_col81 = eval.next_trace_mask();
        let op1_limb_2_col82 = eval.next_trace_mask();
        let op1_limb_3_col83 = eval.next_trace_mask();
        let op1_limb_4_col84 = eval.next_trace_mask();
        let op1_limb_5_col85 = eval.next_trace_mask();
        let op1_limb_6_col86 = eval.next_trace_mask();
        let op1_limb_7_col87 = eval.next_trace_mask();
        let op1_limb_8_col88 = eval.next_trace_mask();
        let op1_limb_9_col89 = eval.next_trace_mask();
        let op1_limb_10_col90 = eval.next_trace_mask();
        let op1_limb_11_col91 = eval.next_trace_mask();
        let op1_limb_12_col92 = eval.next_trace_mask();
        let op1_limb_13_col93 = eval.next_trace_mask();
        let op1_limb_14_col94 = eval.next_trace_mask();
        let op1_limb_15_col95 = eval.next_trace_mask();
        let op1_limb_16_col96 = eval.next_trace_mask();
        let op1_limb_17_col97 = eval.next_trace_mask();
        let op1_limb_18_col98 = eval.next_trace_mask();
        let op1_limb_19_col99 = eval.next_trace_mask();
        let op1_limb_20_col100 = eval.next_trace_mask();
        let op1_limb_21_col101 = eval.next_trace_mask();
        let op1_limb_22_col102 = eval.next_trace_mask();
        let op1_limb_23_col103 = eval.next_trace_mask();
        let op1_limb_24_col104 = eval.next_trace_mask();
        let op1_limb_25_col105 = eval.next_trace_mask();
        let op1_limb_26_col106 = eval.next_trace_mask();
        let op1_limb_27_col107 = eval.next_trace_mask();
        let add_res_limb_0_col108 = eval.next_trace_mask();
        let add_res_limb_1_col109 = eval.next_trace_mask();
        let add_res_limb_2_col110 = eval.next_trace_mask();
        let add_res_limb_3_col111 = eval.next_trace_mask();
        let add_res_limb_4_col112 = eval.next_trace_mask();
        let add_res_limb_5_col113 = eval.next_trace_mask();
        let add_res_limb_6_col114 = eval.next_trace_mask();
        let add_res_limb_7_col115 = eval.next_trace_mask();
        let add_res_limb_8_col116 = eval.next_trace_mask();
        let add_res_limb_9_col117 = eval.next_trace_mask();
        let add_res_limb_10_col118 = eval.next_trace_mask();
        let add_res_limb_11_col119 = eval.next_trace_mask();
        let add_res_limb_12_col120 = eval.next_trace_mask();
        let add_res_limb_13_col121 = eval.next_trace_mask();
        let add_res_limb_14_col122 = eval.next_trace_mask();
        let add_res_limb_15_col123 = eval.next_trace_mask();
        let add_res_limb_16_col124 = eval.next_trace_mask();
        let add_res_limb_17_col125 = eval.next_trace_mask();
        let add_res_limb_18_col126 = eval.next_trace_mask();
        let add_res_limb_19_col127 = eval.next_trace_mask();
        let add_res_limb_20_col128 = eval.next_trace_mask();
        let add_res_limb_21_col129 = eval.next_trace_mask();
        let add_res_limb_22_col130 = eval.next_trace_mask();
        let add_res_limb_23_col131 = eval.next_trace_mask();
        let add_res_limb_24_col132 = eval.next_trace_mask();
        let add_res_limb_25_col133 = eval.next_trace_mask();
        let add_res_limb_26_col134 = eval.next_trace_mask();
        let add_res_limb_27_col135 = eval.next_trace_mask();
        let sub_p_bit_col136 = eval.next_trace_mask();
        let mul_res_limb_0_col137 = eval.next_trace_mask();
        let mul_res_limb_1_col138 = eval.next_trace_mask();
        let mul_res_limb_2_col139 = eval.next_trace_mask();
        let mul_res_limb_3_col140 = eval.next_trace_mask();
        let mul_res_limb_4_col141 = eval.next_trace_mask();
        let mul_res_limb_5_col142 = eval.next_trace_mask();
        let mul_res_limb_6_col143 = eval.next_trace_mask();
        let mul_res_limb_7_col144 = eval.next_trace_mask();
        let mul_res_limb_8_col145 = eval.next_trace_mask();
        let mul_res_limb_9_col146 = eval.next_trace_mask();
        let mul_res_limb_10_col147 = eval.next_trace_mask();
        let mul_res_limb_11_col148 = eval.next_trace_mask();
        let mul_res_limb_12_col149 = eval.next_trace_mask();
        let mul_res_limb_13_col150 = eval.next_trace_mask();
        let mul_res_limb_14_col151 = eval.next_trace_mask();
        let mul_res_limb_15_col152 = eval.next_trace_mask();
        let mul_res_limb_16_col153 = eval.next_trace_mask();
        let mul_res_limb_17_col154 = eval.next_trace_mask();
        let mul_res_limb_18_col155 = eval.next_trace_mask();
        let mul_res_limb_19_col156 = eval.next_trace_mask();
        let mul_res_limb_20_col157 = eval.next_trace_mask();
        let mul_res_limb_21_col158 = eval.next_trace_mask();
        let mul_res_limb_22_col159 = eval.next_trace_mask();
        let mul_res_limb_23_col160 = eval.next_trace_mask();
        let mul_res_limb_24_col161 = eval.next_trace_mask();
        let mul_res_limb_25_col162 = eval.next_trace_mask();
        let mul_res_limb_26_col163 = eval.next_trace_mask();
        let mul_res_limb_27_col164 = eval.next_trace_mask();
        let k_col165 = eval.next_trace_mask();
        let carry_0_col166 = eval.next_trace_mask();
        let carry_1_col167 = eval.next_trace_mask();
        let carry_2_col168 = eval.next_trace_mask();
        let carry_3_col169 = eval.next_trace_mask();
        let carry_4_col170 = eval.next_trace_mask();
        let carry_5_col171 = eval.next_trace_mask();
        let carry_6_col172 = eval.next_trace_mask();
        let carry_7_col173 = eval.next_trace_mask();
        let carry_8_col174 = eval.next_trace_mask();
        let carry_9_col175 = eval.next_trace_mask();
        let carry_10_col176 = eval.next_trace_mask();
        let carry_11_col177 = eval.next_trace_mask();
        let carry_12_col178 = eval.next_trace_mask();
        let carry_13_col179 = eval.next_trace_mask();
        let carry_14_col180 = eval.next_trace_mask();
        let carry_15_col181 = eval.next_trace_mask();
        let carry_16_col182 = eval.next_trace_mask();
        let carry_17_col183 = eval.next_trace_mask();
        let carry_18_col184 = eval.next_trace_mask();
        let carry_19_col185 = eval.next_trace_mask();
        let carry_20_col186 = eval.next_trace_mask();
        let carry_21_col187 = eval.next_trace_mask();
        let carry_22_col188 = eval.next_trace_mask();
        let carry_23_col189 = eval.next_trace_mask();
        let carry_24_col190 = eval.next_trace_mask();
        let carry_25_col191 = eval.next_trace_mask();
        let carry_26_col192 = eval.next_trace_mask();
        let res_limb_0_col193 = eval.next_trace_mask();
        let res_limb_1_col194 = eval.next_trace_mask();
        let res_limb_2_col195 = eval.next_trace_mask();
        let res_limb_3_col196 = eval.next_trace_mask();
        let res_limb_4_col197 = eval.next_trace_mask();
        let res_limb_5_col198 = eval.next_trace_mask();
        let res_limb_6_col199 = eval.next_trace_mask();
        let res_limb_7_col200 = eval.next_trace_mask();
        let res_limb_8_col201 = eval.next_trace_mask();
        let res_limb_9_col202 = eval.next_trace_mask();
        let res_limb_10_col203 = eval.next_trace_mask();
        let res_limb_11_col204 = eval.next_trace_mask();
        let res_limb_12_col205 = eval.next_trace_mask();
        let res_limb_13_col206 = eval.next_trace_mask();
        let res_limb_14_col207 = eval.next_trace_mask();
        let res_limb_15_col208 = eval.next_trace_mask();
        let res_limb_16_col209 = eval.next_trace_mask();
        let res_limb_17_col210 = eval.next_trace_mask();
        let res_limb_18_col211 = eval.next_trace_mask();
        let res_limb_19_col212 = eval.next_trace_mask();
        let res_limb_20_col213 = eval.next_trace_mask();
        let res_limb_21_col214 = eval.next_trace_mask();
        let res_limb_22_col215 = eval.next_trace_mask();
        let res_limb_23_col216 = eval.next_trace_mask();
        let res_limb_24_col217 = eval.next_trace_mask();
        let res_limb_25_col218 = eval.next_trace_mask();
        let res_limb_26_col219 = eval.next_trace_mask();
        let res_limb_27_col220 = eval.next_trace_mask();
        let msb_col221 = eval.next_trace_mask();
        let mid_limbs_set_col222 = eval.next_trace_mask();
        let dst_sum_squares_inv_col223 = eval.next_trace_mask();
        let dst_sum_inv_col224 = eval.next_trace_mask();
        let op1_as_rel_imm_cond_col225 = eval.next_trace_mask();
        let msb_col226 = eval.next_trace_mask();
        let mid_limbs_set_col227 = eval.next_trace_mask();
        let next_pc_jnz_col228 = eval.next_trace_mask();

        // DecodeGenericInstruction.

        // DecodeInstruction_337193008ebaa578.

        let frac = Fraction::new(
            E::EF::one(),
            self.verifyinstruction_lookup_elements.combine(&[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                op1_imm_col8.clone(),
                op1_base_fp_col9.clone(),
                op1_base_ap_col10.clone(),
                res_add_col11.clone(),
                res_mul_col12.clone(),
                pc_update_jump_col13.clone(),
                pc_update_jump_rel_col14.clone(),
                pc_update_jnz_col15.clone(),
                ap_update_add_col16.clone(),
                ap_update_add_1_col17.clone(),
                opcode_call_col18.clone(),
                opcode_ret_col19.clone(),
                opcode_assert_eq_col20.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        let op1_base_op0_tmp_230 = (((M31_1.clone() - op1_imm_col8.clone())
            - op1_base_fp_col9.clone())
            - op1_base_ap_col10.clone());
        // op1_src is 0, 1, 2, or 4.
        eval.add_constraint(
            (op1_base_op0_tmp_230.clone() * (M31_1.clone() - op1_base_op0_tmp_230.clone())),
        );
        let res_op1_tmp_231 = (((M31_1.clone() - res_add_col11.clone()) - res_mul_col12.clone())
            - pc_update_jnz_col15.clone());
        // res_logic is 0, 1, or 2.
        eval.add_constraint((res_op1_tmp_231.clone() * (M31_1.clone() - res_op1_tmp_231.clone())));
        let pc_update_regular_tmp_232 = (((M31_1.clone() - pc_update_jump_col13.clone())
            - pc_update_jump_rel_col14.clone())
            - pc_update_jnz_col15.clone());
        // pc_update is 0, 1, 2, or 4.
        eval.add_constraint(
            (pc_update_regular_tmp_232.clone()
                * (M31_1.clone() - pc_update_regular_tmp_232.clone())),
        );
        let ap_update_regular_tmp_233 = (((M31_1.clone() - ap_update_add_col16.clone())
            - ap_update_add_1_col17.clone())
            - opcode_call_col18.clone());
        // ap_update is 0, 1, 2, or 4.
        eval.add_constraint(
            (ap_update_regular_tmp_233.clone()
                * (M31_1.clone() - ap_update_regular_tmp_233.clone())),
        );
        let fp_update_regular_tmp_234 =
            ((M31_1.clone() - opcode_call_col18.clone()) - opcode_ret_col19.clone());
        // opcode is 0, 1, 2, or 4.
        eval.add_constraint(
            (fp_update_regular_tmp_234.clone()
                * (M31_1.clone() - fp_update_regular_tmp_234.clone())),
        );

        // EvalOperands.

        // ReadPositive_num_bits_252.

        let frac = Fraction::new(
            E::EF::one(),
            self.memoryaddresstoid_lookup_elements.combine(&[
                (((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))
                    + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col21.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.memoryidtobig_lookup_elements.combine(&[
                dst_id_col21.clone(),
                dst_limb_0_col22.clone(),
                dst_limb_1_col23.clone(),
                dst_limb_2_col24.clone(),
                dst_limb_3_col25.clone(),
                dst_limb_4_col26.clone(),
                dst_limb_5_col27.clone(),
                dst_limb_6_col28.clone(),
                dst_limb_7_col29.clone(),
                dst_limb_8_col30.clone(),
                dst_limb_9_col31.clone(),
                dst_limb_10_col32.clone(),
                dst_limb_11_col33.clone(),
                dst_limb_12_col34.clone(),
                dst_limb_13_col35.clone(),
                dst_limb_14_col36.clone(),
                dst_limb_15_col37.clone(),
                dst_limb_16_col38.clone(),
                dst_limb_17_col39.clone(),
                dst_limb_18_col40.clone(),
                dst_limb_19_col41.clone(),
                dst_limb_20_col42.clone(),
                dst_limb_21_col43.clone(),
                dst_limb_22_col44.clone(),
                dst_limb_23_col45.clone(),
                dst_limb_24_col46.clone(),
                dst_limb_25_col47.clone(),
                dst_limb_26_col48.clone(),
                dst_limb_27_col49.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // ReadPositive_num_bits_252.

        let frac = Fraction::new(
            E::EF::one(),
            self.memoryaddresstoid_lookup_elements.combine(&[
                (((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))
                    + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col50.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.memoryidtobig_lookup_elements.combine(&[
                op0_id_col50.clone(),
                op0_limb_0_col51.clone(),
                op0_limb_1_col52.clone(),
                op0_limb_2_col53.clone(),
                op0_limb_3_col54.clone(),
                op0_limb_4_col55.clone(),
                op0_limb_5_col56.clone(),
                op0_limb_6_col57.clone(),
                op0_limb_7_col58.clone(),
                op0_limb_8_col59.clone(),
                op0_limb_9_col60.clone(),
                op0_limb_10_col61.clone(),
                op0_limb_11_col62.clone(),
                op0_limb_12_col63.clone(),
                op0_limb_13_col64.clone(),
                op0_limb_14_col65.clone(),
                op0_limb_15_col66.clone(),
                op0_limb_16_col67.clone(),
                op0_limb_17_col68.clone(),
                op0_limb_18_col69.clone(),
                op0_limb_19_col70.clone(),
                op0_limb_20_col71.clone(),
                op0_limb_21_col72.clone(),
                op0_limb_22_col73.clone(),
                op0_limb_23_col74.clone(),
                op0_limb_24_col75.clone(),
                op0_limb_25_col76.clone(),
                op0_limb_26_col77.clone(),
                op0_limb_27_col78.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // CondFelt252AsAddr.

        // Address limb 3 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_3_col54.clone()));
        // Address limb 4 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_4_col55.clone()));
        // Address limb 5 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_5_col56.clone()));
        // Address limb 6 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_6_col57.clone()));
        // Address limb 7 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_7_col58.clone()));
        // Address limb 8 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_8_col59.clone()));
        // Address limb 9 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_9_col60.clone()));
        // Address limb 10 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_10_col61.clone()));
        // Address limb 11 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_11_col62.clone()));
        // Address limb 12 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_12_col63.clone()));
        // Address limb 13 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_13_col64.clone()));
        // Address limb 14 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_14_col65.clone()));
        // Address limb 15 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_15_col66.clone()));
        // Address limb 16 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_16_col67.clone()));
        // Address limb 17 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_17_col68.clone()));
        // Address limb 18 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_18_col69.clone()));
        // Address limb 19 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_19_col70.clone()));
        // Address limb 20 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_20_col71.clone()));
        // Address limb 21 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_21_col72.clone()));
        // Address limb 22 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_22_col73.clone()));
        // Address limb 23 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_23_col74.clone()));
        // Address limb 24 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_24_col75.clone()));
        // Address limb 25 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_25_col76.clone()));
        // Address limb 26 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_26_col77.clone()));
        // Address limb 27 equals 0.
        eval.add_constraint((op1_base_op0_tmp_230.clone() * op0_limb_27_col78.clone()));

        // ReadPositive_num_bits_252.

        let frac = Fraction::new(
            E::EF::one(),
            self.memoryaddresstoid_lookup_elements.combine(&[
                (((((op1_base_fp_col9.clone() * input_fp_col2.clone())
                    + (op1_base_ap_col10.clone() * input_ap_col1.clone()))
                    + (op1_imm_col8.clone() * input_pc_col0.clone()))
                    + (op1_base_op0_tmp_230.clone()
                        * ((op0_limb_0_col51.clone()
                            + (op0_limb_1_col52.clone() * M31_512.clone()))
                            + (op0_limb_2_col53.clone() * M31_262144.clone()))))
                    + (offset2_col5.clone() - M31_32768.clone())),
                op1_id_col79.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.memoryidtobig_lookup_elements.combine(&[
                op1_id_col79.clone(),
                op1_limb_0_col80.clone(),
                op1_limb_1_col81.clone(),
                op1_limb_2_col82.clone(),
                op1_limb_3_col83.clone(),
                op1_limb_4_col84.clone(),
                op1_limb_5_col85.clone(),
                op1_limb_6_col86.clone(),
                op1_limb_7_col87.clone(),
                op1_limb_8_col88.clone(),
                op1_limb_9_col89.clone(),
                op1_limb_10_col90.clone(),
                op1_limb_11_col91.clone(),
                op1_limb_12_col92.clone(),
                op1_limb_13_col93.clone(),
                op1_limb_14_col94.clone(),
                op1_limb_15_col95.clone(),
                op1_limb_16_col96.clone(),
                op1_limb_17_col97.clone(),
                op1_limb_18_col98.clone(),
                op1_limb_19_col99.clone(),
                op1_limb_20_col100.clone(),
                op1_limb_21_col101.clone(),
                op1_limb_22_col102.clone(),
                op1_limb_23_col103.clone(),
                op1_limb_24_col104.clone(),
                op1_limb_25_col105.clone(),
                op1_limb_26_col106.clone(),
                op1_limb_27_col107.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // Add252.

        // RangeCheckBigValue.

        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[add_res_limb_0_col108.clone(), add_res_limb_1_col109.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[add_res_limb_2_col110.clone(), add_res_limb_3_col111.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[add_res_limb_4_col112.clone(), add_res_limb_5_col113.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[add_res_limb_6_col114.clone(), add_res_limb_7_col115.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[add_res_limb_8_col116.clone(), add_res_limb_9_col117.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_10_col118.clone(),
                add_res_limb_11_col119.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_12_col120.clone(),
                add_res_limb_13_col121.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_14_col122.clone(),
                add_res_limb_15_col123.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_16_col124.clone(),
                add_res_limb_17_col125.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_18_col126.clone(),
                add_res_limb_19_col127.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_20_col128.clone(),
                add_res_limb_21_col129.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_22_col130.clone(),
                add_res_limb_23_col131.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_24_col132.clone(),
                add_res_limb_25_col133.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                add_res_limb_26_col134.clone(),
                add_res_limb_27_col135.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // VerifyAdd252.

        // sub_p_bit is a bit.
        eval.add_constraint(
            (sub_p_bit_col136.clone() * (sub_p_bit_col136.clone() - M31_1.clone())),
        );
        let carry_tmp_745 = (((((op0_limb_0_col51.clone() + op1_limb_0_col80.clone())
            + M31_0.clone())
            - add_res_limb_0_col108.clone())
            - (M31_1.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_745.clone()
                * ((carry_tmp_745.clone() * carry_tmp_745.clone()) - M31_1.clone())),
        );
        let carry_tmp_746 = (((((op0_limb_1_col52.clone() + op1_limb_1_col81.clone())
            + carry_tmp_745.clone())
            - add_res_limb_1_col109.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_746.clone()
                * ((carry_tmp_746.clone() * carry_tmp_746.clone()) - M31_1.clone())),
        );
        let carry_tmp_747 = (((((op0_limb_2_col53.clone() + op1_limb_2_col82.clone())
            + carry_tmp_746.clone())
            - add_res_limb_2_col110.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_747.clone()
                * ((carry_tmp_747.clone() * carry_tmp_747.clone()) - M31_1.clone())),
        );
        let carry_tmp_748 = (((((op0_limb_3_col54.clone() + op1_limb_3_col83.clone())
            + carry_tmp_747.clone())
            - add_res_limb_3_col111.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_748.clone()
                * ((carry_tmp_748.clone() * carry_tmp_748.clone()) - M31_1.clone())),
        );
        let carry_tmp_749 = (((((op0_limb_4_col55.clone() + op1_limb_4_col84.clone())
            + carry_tmp_748.clone())
            - add_res_limb_4_col112.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_749.clone()
                * ((carry_tmp_749.clone() * carry_tmp_749.clone()) - M31_1.clone())),
        );
        let carry_tmp_750 = (((((op0_limb_5_col56.clone() + op1_limb_5_col85.clone())
            + carry_tmp_749.clone())
            - add_res_limb_5_col113.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_750.clone()
                * ((carry_tmp_750.clone() * carry_tmp_750.clone()) - M31_1.clone())),
        );
        let carry_tmp_751 = (((((op0_limb_6_col57.clone() + op1_limb_6_col86.clone())
            + carry_tmp_750.clone())
            - add_res_limb_6_col114.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_751.clone()
                * ((carry_tmp_751.clone() * carry_tmp_751.clone()) - M31_1.clone())),
        );
        let carry_tmp_752 = (((((op0_limb_7_col58.clone() + op1_limb_7_col87.clone())
            + carry_tmp_751.clone())
            - add_res_limb_7_col115.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_752.clone()
                * ((carry_tmp_752.clone() * carry_tmp_752.clone()) - M31_1.clone())),
        );
        let carry_tmp_753 = (((((op0_limb_8_col59.clone() + op1_limb_8_col88.clone())
            + carry_tmp_752.clone())
            - add_res_limb_8_col116.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_753.clone()
                * ((carry_tmp_753.clone() * carry_tmp_753.clone()) - M31_1.clone())),
        );
        let carry_tmp_754 = (((((op0_limb_9_col60.clone() + op1_limb_9_col89.clone())
            + carry_tmp_753.clone())
            - add_res_limb_9_col117.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_754.clone()
                * ((carry_tmp_754.clone() * carry_tmp_754.clone()) - M31_1.clone())),
        );
        let carry_tmp_755 = (((((op0_limb_10_col61.clone() + op1_limb_10_col90.clone())
            + carry_tmp_754.clone())
            - add_res_limb_10_col118.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_755.clone()
                * ((carry_tmp_755.clone() * carry_tmp_755.clone()) - M31_1.clone())),
        );
        let carry_tmp_756 = (((((op0_limb_11_col62.clone() + op1_limb_11_col91.clone())
            + carry_tmp_755.clone())
            - add_res_limb_11_col119.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_756.clone()
                * ((carry_tmp_756.clone() * carry_tmp_756.clone()) - M31_1.clone())),
        );
        let carry_tmp_757 = (((((op0_limb_12_col63.clone() + op1_limb_12_col92.clone())
            + carry_tmp_756.clone())
            - add_res_limb_12_col120.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_757.clone()
                * ((carry_tmp_757.clone() * carry_tmp_757.clone()) - M31_1.clone())),
        );
        let carry_tmp_758 = (((((op0_limb_13_col64.clone() + op1_limb_13_col93.clone())
            + carry_tmp_757.clone())
            - add_res_limb_13_col121.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_758.clone()
                * ((carry_tmp_758.clone() * carry_tmp_758.clone()) - M31_1.clone())),
        );
        let carry_tmp_759 = (((((op0_limb_14_col65.clone() + op1_limb_14_col94.clone())
            + carry_tmp_758.clone())
            - add_res_limb_14_col122.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_759.clone()
                * ((carry_tmp_759.clone() * carry_tmp_759.clone()) - M31_1.clone())),
        );
        let carry_tmp_760 = (((((op0_limb_15_col66.clone() + op1_limb_15_col95.clone())
            + carry_tmp_759.clone())
            - add_res_limb_15_col123.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_760.clone()
                * ((carry_tmp_760.clone() * carry_tmp_760.clone()) - M31_1.clone())),
        );
        let carry_tmp_761 = (((((op0_limb_16_col67.clone() + op1_limb_16_col96.clone())
            + carry_tmp_760.clone())
            - add_res_limb_16_col124.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_761.clone()
                * ((carry_tmp_761.clone() * carry_tmp_761.clone()) - M31_1.clone())),
        );
        let carry_tmp_762 = (((((op0_limb_17_col68.clone() + op1_limb_17_col97.clone())
            + carry_tmp_761.clone())
            - add_res_limb_17_col125.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_762.clone()
                * ((carry_tmp_762.clone() * carry_tmp_762.clone()) - M31_1.clone())),
        );
        let carry_tmp_763 = (((((op0_limb_18_col69.clone() + op1_limb_18_col98.clone())
            + carry_tmp_762.clone())
            - add_res_limb_18_col126.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_763.clone()
                * ((carry_tmp_763.clone() * carry_tmp_763.clone()) - M31_1.clone())),
        );
        let carry_tmp_764 = (((((op0_limb_19_col70.clone() + op1_limb_19_col99.clone())
            + carry_tmp_763.clone())
            - add_res_limb_19_col127.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_764.clone()
                * ((carry_tmp_764.clone() * carry_tmp_764.clone()) - M31_1.clone())),
        );
        let carry_tmp_765 = (((((op0_limb_20_col71.clone() + op1_limb_20_col100.clone())
            + carry_tmp_764.clone())
            - add_res_limb_20_col128.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_765.clone()
                * ((carry_tmp_765.clone() * carry_tmp_765.clone()) - M31_1.clone())),
        );
        let carry_tmp_766 = (((((op0_limb_21_col72.clone() + op1_limb_21_col101.clone())
            + carry_tmp_765.clone())
            - add_res_limb_21_col129.clone())
            - (M31_136.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_766.clone()
                * ((carry_tmp_766.clone() * carry_tmp_766.clone()) - M31_1.clone())),
        );
        let carry_tmp_767 = (((((op0_limb_22_col73.clone() + op1_limb_22_col102.clone())
            + carry_tmp_766.clone())
            - add_res_limb_22_col130.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_767.clone()
                * ((carry_tmp_767.clone() * carry_tmp_767.clone()) - M31_1.clone())),
        );
        let carry_tmp_768 = (((((op0_limb_23_col74.clone() + op1_limb_23_col103.clone())
            + carry_tmp_767.clone())
            - add_res_limb_23_col131.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_768.clone()
                * ((carry_tmp_768.clone() * carry_tmp_768.clone()) - M31_1.clone())),
        );
        let carry_tmp_769 = (((((op0_limb_24_col75.clone() + op1_limb_24_col104.clone())
            + carry_tmp_768.clone())
            - add_res_limb_24_col132.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_769.clone()
                * ((carry_tmp_769.clone() * carry_tmp_769.clone()) - M31_1.clone())),
        );
        let carry_tmp_770 = (((((op0_limb_25_col76.clone() + op1_limb_25_col105.clone())
            + carry_tmp_769.clone())
            - add_res_limb_25_col133.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_770.clone()
                * ((carry_tmp_770.clone() * carry_tmp_770.clone()) - M31_1.clone())),
        );
        let carry_tmp_771 = (((((op0_limb_26_col77.clone() + op1_limb_26_col106.clone())
            + carry_tmp_770.clone())
            - add_res_limb_26_col134.clone())
            - (M31_0.clone() * sub_p_bit_col136.clone()))
            * M31_4194304.clone());
        eval.add_constraint(
            (carry_tmp_771.clone()
                * ((carry_tmp_771.clone() * carry_tmp_771.clone()) - M31_1.clone())),
        );
        eval.add_constraint(
            ((((op0_limb_27_col78.clone() + op1_limb_27_col107.clone()) + carry_tmp_771.clone())
                - add_res_limb_27_col135.clone())
                - (M31_256.clone() * sub_p_bit_col136.clone())),
        );

        // Mul252.

        // RangeCheckBigValue.

        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[mul_res_limb_0_col137.clone(), mul_res_limb_1_col138.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[mul_res_limb_2_col139.clone(), mul_res_limb_3_col140.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[mul_res_limb_4_col141.clone(), mul_res_limb_5_col142.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[mul_res_limb_6_col143.clone(), mul_res_limb_7_col144.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements
                .combine(&[mul_res_limb_8_col145.clone(), mul_res_limb_9_col146.clone()]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_10_col147.clone(),
                mul_res_limb_11_col148.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_12_col149.clone(),
                mul_res_limb_13_col150.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_14_col151.clone(),
                mul_res_limb_15_col152.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_16_col153.clone(),
                mul_res_limb_17_col154.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_18_col155.clone(),
                mul_res_limb_19_col156.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_20_col157.clone(),
                mul_res_limb_21_col158.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_22_col159.clone(),
                mul_res_limb_23_col160.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_24_col161.clone(),
                mul_res_limb_25_col162.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_9_9_lookup_elements.combine(&[
                mul_res_limb_26_col163.clone(),
                mul_res_limb_27_col164.clone(),
            ]),
        );
        logup.write_frac(&mut eval, frac);

        // VerifyMul252.

        let conv_tmp_787 = ((M31_0.clone() - mul_res_limb_0_col137.clone())
            + (op0_limb_0_col51.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_788 = (((M31_0.clone() - mul_res_limb_1_col138.clone())
            + (op0_limb_0_col51.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_789 = ((((M31_0.clone() - mul_res_limb_2_col139.clone())
            + (op0_limb_0_col51.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_790 = (((((M31_0.clone() - mul_res_limb_3_col140.clone())
            + (op0_limb_0_col51.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_791 = ((((((M31_0.clone() - mul_res_limb_4_col141.clone())
            + (op0_limb_0_col51.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_792 = (((((((M31_0.clone() - mul_res_limb_5_col142.clone())
            + (op0_limb_0_col51.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_793 = ((((((((M31_0.clone() - mul_res_limb_6_col143.clone())
            + (op0_limb_0_col51.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_794 = (((((((((M31_0.clone() - mul_res_limb_7_col144.clone())
            + (op0_limb_0_col51.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_795 = ((((((((((M31_0.clone() - mul_res_limb_8_col145.clone())
            + (op0_limb_0_col51.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_796 = (((((((((((M31_0.clone() - mul_res_limb_9_col146.clone())
            + (op0_limb_0_col51.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_797 = ((((((((((((M31_0.clone() - mul_res_limb_10_col147.clone())
            + (op0_limb_0_col51.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_798 = (((((((((((((M31_0.clone() - mul_res_limb_11_col148.clone())
            + (op0_limb_0_col51.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_799 = ((((((((((((((M31_0.clone()
            - mul_res_limb_12_col149.clone())
            + (op0_limb_0_col51.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_800 = (((((((((((((((M31_0.clone()
            - mul_res_limb_13_col150.clone())
            + (op0_limb_0_col51.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_801 = ((((((((((((((((M31_0.clone()
            - mul_res_limb_14_col151.clone())
            + (op0_limb_0_col51.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_802 = (((((((((((((((((M31_0.clone()
            - mul_res_limb_15_col152.clone())
            + (op0_limb_0_col51.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_803 = ((((((((((((((((((M31_0.clone()
            - mul_res_limb_16_col153.clone())
            + (op0_limb_0_col51.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_804 = (((((((((((((((((((M31_0.clone()
            - mul_res_limb_17_col154.clone())
            + (op0_limb_0_col51.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_805 = ((((((((((((((((((((M31_0.clone()
            - mul_res_limb_18_col155.clone())
            + (op0_limb_0_col51.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_806 = (((((((((((((((((((((M31_0.clone()
            - mul_res_limb_19_col156.clone())
            + (op0_limb_0_col51.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_807 = ((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_20_col157.clone())
            + (op0_limb_0_col51.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_808 = (((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_21_col158.clone())
            + (op0_limb_0_col51.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_809 = ((((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_22_col159.clone())
            + (op0_limb_0_col51.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_810 = (((((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_23_col160.clone())
            + (op0_limb_0_col51.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_811 = ((((((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_24_col161.clone())
            + (op0_limb_0_col51.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_812 = (((((((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_25_col162.clone())
            + (op0_limb_0_col51.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_813 = ((((((((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_26_col163.clone())
            + (op0_limb_0_col51.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_814 = (((((((((((((((((((((((((((((M31_0.clone()
            - mul_res_limb_27_col164.clone())
            + (op0_limb_0_col51.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_1_col52.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_1_col81.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_0_col80.clone()));
        let conv_tmp_815 = (((((((((((((((((((((((((((M31_0.clone()
            + (op0_limb_1_col52.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_2_col53.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_2_col82.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_1_col81.clone()));
        let conv_tmp_816 = ((((((((((((((((((((((((((M31_0.clone()
            + (op0_limb_2_col53.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_3_col54.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_3_col83.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_2_col82.clone()));
        let conv_tmp_817 = (((((((((((((((((((((((((M31_0.clone()
            + (op0_limb_3_col54.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_4_col55.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_4_col84.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_3_col83.clone()));
        let conv_tmp_818 = ((((((((((((((((((((((((M31_0.clone()
            + (op0_limb_4_col55.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_5_col56.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_5_col85.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_4_col84.clone()));
        let conv_tmp_819 = (((((((((((((((((((((((M31_0.clone()
            + (op0_limb_5_col56.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_6_col57.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_6_col86.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_5_col85.clone()));
        let conv_tmp_820 = ((((((((((((((((((((((M31_0.clone()
            + (op0_limb_6_col57.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_7_col58.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_7_col87.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_6_col86.clone()));
        let conv_tmp_821 = (((((((((((((((((((((M31_0.clone()
            + (op0_limb_7_col58.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_8_col59.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_8_col88.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_7_col87.clone()));
        let conv_tmp_822 = ((((((((((((((((((((M31_0.clone()
            + (op0_limb_8_col59.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_9_col60.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_9_col89.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_8_col88.clone()));
        let conv_tmp_823 = (((((((((((((((((((M31_0.clone()
            + (op0_limb_9_col60.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_10_col61.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_10_col90.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_9_col89.clone()));
        let conv_tmp_824 = ((((((((((((((((((M31_0.clone()
            + (op0_limb_10_col61.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_11_col62.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_11_col91.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_10_col90.clone()));
        let conv_tmp_825 = (((((((((((((((((M31_0.clone()
            + (op0_limb_11_col62.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_12_col63.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_12_col92.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_11_col91.clone()));
        let conv_tmp_826 = ((((((((((((((((M31_0.clone()
            + (op0_limb_12_col63.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_13_col64.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_13_col93.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_12_col92.clone()));
        let conv_tmp_827 = (((((((((((((((M31_0.clone()
            + (op0_limb_13_col64.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_14_col65.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_14_col94.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_13_col93.clone()));
        let conv_tmp_828 = ((((((((((((((M31_0.clone()
            + (op0_limb_14_col65.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_15_col66.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_15_col95.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_14_col94.clone()));
        let conv_tmp_829 = (((((((((((((M31_0.clone()
            + (op0_limb_15_col66.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_16_col67.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_16_col96.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_15_col95.clone()));
        let conv_tmp_830 = ((((((((((((M31_0.clone()
            + (op0_limb_16_col67.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_17_col68.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_17_col97.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_16_col96.clone()));
        let conv_tmp_831 = (((((((((((M31_0.clone()
            + (op0_limb_17_col68.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_18_col69.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_18_col98.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_17_col97.clone()));
        let conv_tmp_832 = ((((((((((M31_0.clone()
            + (op0_limb_18_col69.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_19_col70.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_19_col99.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_18_col98.clone()));
        let conv_tmp_833 = (((((((((M31_0.clone()
            + (op0_limb_19_col70.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_20_col71.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_20_col100.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_19_col99.clone()));
        let conv_tmp_834 = ((((((((M31_0.clone()
            + (op0_limb_20_col71.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_21_col72.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_21_col101.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_20_col100.clone()));
        let conv_tmp_835 = (((((((M31_0.clone()
            + (op0_limb_21_col72.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_22_col73.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_22_col102.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_21_col101.clone()));
        let conv_tmp_836 = ((((((M31_0.clone()
            + (op0_limb_22_col73.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_23_col74.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_23_col103.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_22_col102.clone()));
        let conv_tmp_837 = (((((M31_0.clone()
            + (op0_limb_23_col74.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_24_col75.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_24_col104.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_23_col103.clone()));
        let conv_tmp_838 = ((((M31_0.clone()
            + (op0_limb_24_col75.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_25_col76.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_25_col105.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_24_col104.clone()));
        let conv_tmp_839 = (((M31_0.clone()
            + (op0_limb_25_col76.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_26_col77.clone() * op1_limb_26_col106.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_25_col105.clone()));
        let conv_tmp_840 = ((M31_0.clone()
            + (op0_limb_26_col77.clone() * op1_limb_27_col107.clone()))
            + (op0_limb_27_col78.clone() * op1_limb_26_col106.clone()));
        let conv_tmp_841 =
            (M31_0.clone() + (op0_limb_27_col78.clone() * op1_limb_27_col107.clone()));
        let conv_mod_tmp_842 = (((M31_0.clone() + (M31_32.clone() * conv_tmp_787.clone()))
            - (M31_4.clone() * conv_tmp_808.clone()))
            + (M31_8.clone() * conv_tmp_836.clone()));
        let conv_mod_tmp_843 = ((((M31_0.clone() + (M31_1.clone() * conv_tmp_787.clone()))
            + (M31_32.clone() * conv_tmp_788.clone()))
            - (M31_4.clone() * conv_tmp_809.clone()))
            + (M31_8.clone() * conv_tmp_837.clone()));
        let conv_mod_tmp_844 = ((((M31_0.clone() + (M31_1.clone() * conv_tmp_788.clone()))
            + (M31_32.clone() * conv_tmp_789.clone()))
            - (M31_4.clone() * conv_tmp_810.clone()))
            + (M31_8.clone() * conv_tmp_838.clone()));
        let conv_mod_tmp_845 = ((((M31_0.clone() + (M31_1.clone() * conv_tmp_789.clone()))
            + (M31_32.clone() * conv_tmp_790.clone()))
            - (M31_4.clone() * conv_tmp_811.clone()))
            + (M31_8.clone() * conv_tmp_839.clone()));
        let conv_mod_tmp_846 = ((((M31_0.clone() + (M31_1.clone() * conv_tmp_790.clone()))
            + (M31_32.clone() * conv_tmp_791.clone()))
            - (M31_4.clone() * conv_tmp_812.clone()))
            + (M31_8.clone() * conv_tmp_840.clone()));
        let conv_mod_tmp_847 = ((((M31_0.clone() + (M31_1.clone() * conv_tmp_791.clone()))
            + (M31_32.clone() * conv_tmp_792.clone()))
            - (M31_4.clone() * conv_tmp_813.clone()))
            + (M31_8.clone() * conv_tmp_841.clone()));
        let conv_mod_tmp_848 = (((M31_0.clone() + (M31_1.clone() * conv_tmp_792.clone()))
            + (M31_32.clone() * conv_tmp_793.clone()))
            - (M31_4.clone() * conv_tmp_814.clone()));
        let conv_mod_tmp_849 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_787.clone()))
            + (M31_1.clone() * conv_tmp_793.clone()))
            + (M31_32.clone() * conv_tmp_794.clone()))
            - (M31_4.clone() * conv_tmp_815.clone()));
        let conv_mod_tmp_850 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_788.clone()))
            + (M31_1.clone() * conv_tmp_794.clone()))
            + (M31_32.clone() * conv_tmp_795.clone()))
            - (M31_4.clone() * conv_tmp_816.clone()));
        let conv_mod_tmp_851 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_789.clone()))
            + (M31_1.clone() * conv_tmp_795.clone()))
            + (M31_32.clone() * conv_tmp_796.clone()))
            - (M31_4.clone() * conv_tmp_817.clone()));
        let conv_mod_tmp_852 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_790.clone()))
            + (M31_1.clone() * conv_tmp_796.clone()))
            + (M31_32.clone() * conv_tmp_797.clone()))
            - (M31_4.clone() * conv_tmp_818.clone()));
        let conv_mod_tmp_853 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_791.clone()))
            + (M31_1.clone() * conv_tmp_797.clone()))
            + (M31_32.clone() * conv_tmp_798.clone()))
            - (M31_4.clone() * conv_tmp_819.clone()));
        let conv_mod_tmp_854 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_792.clone()))
            + (M31_1.clone() * conv_tmp_798.clone()))
            + (M31_32.clone() * conv_tmp_799.clone()))
            - (M31_4.clone() * conv_tmp_820.clone()));
        let conv_mod_tmp_855 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_793.clone()))
            + (M31_1.clone() * conv_tmp_799.clone()))
            + (M31_32.clone() * conv_tmp_800.clone()))
            - (M31_4.clone() * conv_tmp_821.clone()));
        let conv_mod_tmp_856 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_794.clone()))
            + (M31_1.clone() * conv_tmp_800.clone()))
            + (M31_32.clone() * conv_tmp_801.clone()))
            - (M31_4.clone() * conv_tmp_822.clone()));
        let conv_mod_tmp_857 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_795.clone()))
            + (M31_1.clone() * conv_tmp_801.clone()))
            + (M31_32.clone() * conv_tmp_802.clone()))
            - (M31_4.clone() * conv_tmp_823.clone()));
        let conv_mod_tmp_858 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_796.clone()))
            + (M31_1.clone() * conv_tmp_802.clone()))
            + (M31_32.clone() * conv_tmp_803.clone()))
            - (M31_4.clone() * conv_tmp_824.clone()));
        let conv_mod_tmp_859 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_797.clone()))
            + (M31_1.clone() * conv_tmp_803.clone()))
            + (M31_32.clone() * conv_tmp_804.clone()))
            - (M31_4.clone() * conv_tmp_825.clone()));
        let conv_mod_tmp_860 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_798.clone()))
            + (M31_1.clone() * conv_tmp_804.clone()))
            + (M31_32.clone() * conv_tmp_805.clone()))
            - (M31_4.clone() * conv_tmp_826.clone()));
        let conv_mod_tmp_861 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_799.clone()))
            + (M31_1.clone() * conv_tmp_805.clone()))
            + (M31_32.clone() * conv_tmp_806.clone()))
            - (M31_4.clone() * conv_tmp_827.clone()));
        let conv_mod_tmp_862 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_800.clone()))
            + (M31_1.clone() * conv_tmp_806.clone()))
            + (M31_32.clone() * conv_tmp_807.clone()))
            - (M31_4.clone() * conv_tmp_828.clone()));
        let conv_mod_tmp_863 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_801.clone()))
            + (M31_1.clone() * conv_tmp_807.clone()))
            - (M31_4.clone() * conv_tmp_829.clone()))
            + (M31_64.clone() * conv_tmp_836.clone()));
        let conv_mod_tmp_864 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_802.clone()))
            - (M31_4.clone() * conv_tmp_830.clone()))
            + (M31_2.clone() * conv_tmp_836.clone()))
            + (M31_64.clone() * conv_tmp_837.clone()));
        let conv_mod_tmp_865 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_803.clone()))
            - (M31_4.clone() * conv_tmp_831.clone()))
            + (M31_2.clone() * conv_tmp_837.clone()))
            + (M31_64.clone() * conv_tmp_838.clone()));
        let conv_mod_tmp_866 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_804.clone()))
            - (M31_4.clone() * conv_tmp_832.clone()))
            + (M31_2.clone() * conv_tmp_838.clone()))
            + (M31_64.clone() * conv_tmp_839.clone()));
        let conv_mod_tmp_867 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_805.clone()))
            - (M31_4.clone() * conv_tmp_833.clone()))
            + (M31_2.clone() * conv_tmp_839.clone()))
            + (M31_64.clone() * conv_tmp_840.clone()));
        let conv_mod_tmp_868 = ((((M31_0.clone() + (M31_2.clone() * conv_tmp_806.clone()))
            - (M31_4.clone() * conv_tmp_834.clone()))
            + (M31_2.clone() * conv_tmp_840.clone()))
            + (M31_64.clone() * conv_tmp_841.clone()));
        let conv_mod_tmp_869 = (((M31_0.clone() + (M31_2.clone() * conv_tmp_807.clone()))
            - (M31_4.clone() * conv_tmp_835.clone()))
            + (M31_2.clone() * conv_tmp_841.clone()));
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(k_col165.clone() + M31_262144.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_0_col166.clone() * M31_512.clone())
                - ((conv_mod_tmp_842.clone() - (M31_1.clone() * k_col165.clone()))
                    + M31_0.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_0_col166.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_1_col167.clone() * M31_512.clone())
                - (conv_mod_tmp_843.clone() + carry_0_col166.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_1_col167.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_2_col168.clone() * M31_512.clone())
                - (conv_mod_tmp_844.clone() + carry_1_col167.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_2_col168.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_3_col169.clone() * M31_512.clone())
                - (conv_mod_tmp_845.clone() + carry_2_col168.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_3_col169.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_4_col170.clone() * M31_512.clone())
                - (conv_mod_tmp_846.clone() + carry_3_col169.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_4_col170.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_5_col171.clone() * M31_512.clone())
                - (conv_mod_tmp_847.clone() + carry_4_col170.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_5_col171.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_6_col172.clone() * M31_512.clone())
                - (conv_mod_tmp_848.clone() + carry_5_col171.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_6_col172.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_7_col173.clone() * M31_512.clone())
                - (conv_mod_tmp_849.clone() + carry_6_col172.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_7_col173.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_8_col174.clone() * M31_512.clone())
                - (conv_mod_tmp_850.clone() + carry_7_col173.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_8_col174.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_9_col175.clone() * M31_512.clone())
                - (conv_mod_tmp_851.clone() + carry_8_col174.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_9_col175.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_10_col176.clone() * M31_512.clone())
                - (conv_mod_tmp_852.clone() + carry_9_col175.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_10_col176.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_11_col177.clone() * M31_512.clone())
                - (conv_mod_tmp_853.clone() + carry_10_col176.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_11_col177.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_12_col178.clone() * M31_512.clone())
                - (conv_mod_tmp_854.clone() + carry_11_col177.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_12_col178.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_13_col179.clone() * M31_512.clone())
                - (conv_mod_tmp_855.clone() + carry_12_col178.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_13_col179.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_14_col180.clone() * M31_512.clone())
                - (conv_mod_tmp_856.clone() + carry_13_col179.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_14_col180.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_15_col181.clone() * M31_512.clone())
                - (conv_mod_tmp_857.clone() + carry_14_col180.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_15_col181.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_16_col182.clone() * M31_512.clone())
                - (conv_mod_tmp_858.clone() + carry_15_col181.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_16_col182.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_17_col183.clone() * M31_512.clone())
                - (conv_mod_tmp_859.clone() + carry_16_col182.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_17_col183.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_18_col184.clone() * M31_512.clone())
                - (conv_mod_tmp_860.clone() + carry_17_col183.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_18_col184.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_19_col185.clone() * M31_512.clone())
                - (conv_mod_tmp_861.clone() + carry_18_col184.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_19_col185.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_20_col186.clone() * M31_512.clone())
                - (conv_mod_tmp_862.clone() + carry_19_col185.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_20_col186.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_21_col187.clone() * M31_512.clone())
                - ((conv_mod_tmp_863.clone() - (M31_136.clone() * k_col165.clone()))
                    + carry_20_col186.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_21_col187.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_22_col188.clone() * M31_512.clone())
                - (conv_mod_tmp_864.clone() + carry_21_col187.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_22_col188.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_23_col189.clone() * M31_512.clone())
                - (conv_mod_tmp_865.clone() + carry_22_col188.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_23_col189.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_24_col190.clone() * M31_512.clone())
                - (conv_mod_tmp_866.clone() + carry_23_col189.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_24_col190.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_25_col191.clone() * M31_512.clone())
                - (conv_mod_tmp_867.clone() + carry_24_col190.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_25_col191.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((carry_26_col192.clone() * M31_512.clone())
                - (conv_mod_tmp_868.clone() + carry_25_col191.clone())),
        );
        let frac = Fraction::new(
            E::EF::one(),
            self.range_check_19_lookup_elements
                .combine(&[(carry_26_col192.clone() + M31_131072.clone())]),
        );
        logup.write_frac(&mut eval, frac);
        eval.add_constraint(
            ((conv_mod_tmp_869.clone() - (M31_256.clone() * k_col165.clone()))
                + carry_26_col192.clone()),
        );

        let res_constrained_tmp_900 = (M31_1.clone() - pc_update_jnz_col15.clone());
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_0_col193.clone() - op1_limb_0_col80.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_0_col193.clone() - add_res_limb_0_col108.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_0_col193.clone() - mul_res_limb_0_col137.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_1_col194.clone() - op1_limb_1_col81.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_1_col194.clone() - add_res_limb_1_col109.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_1_col194.clone() - mul_res_limb_1_col138.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_2_col195.clone() - op1_limb_2_col82.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_2_col195.clone() - add_res_limb_2_col110.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_2_col195.clone() - mul_res_limb_2_col139.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_3_col196.clone() - op1_limb_3_col83.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_3_col196.clone() - add_res_limb_3_col111.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_3_col196.clone() - mul_res_limb_3_col140.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_4_col197.clone() - op1_limb_4_col84.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_4_col197.clone() - add_res_limb_4_col112.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_4_col197.clone() - mul_res_limb_4_col141.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_5_col198.clone() - op1_limb_5_col85.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_5_col198.clone() - add_res_limb_5_col113.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_5_col198.clone() - mul_res_limb_5_col142.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_6_col199.clone() - op1_limb_6_col86.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_6_col199.clone() - add_res_limb_6_col114.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_6_col199.clone() - mul_res_limb_6_col143.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_7_col200.clone() - op1_limb_7_col87.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_7_col200.clone() - add_res_limb_7_col115.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_7_col200.clone() - mul_res_limb_7_col144.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_8_col201.clone() - op1_limb_8_col88.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_8_col201.clone() - add_res_limb_8_col116.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_8_col201.clone() - mul_res_limb_8_col145.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_9_col202.clone() - op1_limb_9_col89.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_9_col202.clone() - add_res_limb_9_col117.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_9_col202.clone() - mul_res_limb_9_col146.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_10_col203.clone() - op1_limb_10_col90.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_10_col203.clone() - add_res_limb_10_col118.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_10_col203.clone() - mul_res_limb_10_col147.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_11_col204.clone() - op1_limb_11_col91.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_11_col204.clone() - add_res_limb_11_col119.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_11_col204.clone() - mul_res_limb_11_col148.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_12_col205.clone() - op1_limb_12_col92.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_12_col205.clone() - add_res_limb_12_col120.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_12_col205.clone() - mul_res_limb_12_col149.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_13_col206.clone() - op1_limb_13_col93.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_13_col206.clone() - add_res_limb_13_col121.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_13_col206.clone() - mul_res_limb_13_col150.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_14_col207.clone() - op1_limb_14_col94.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_14_col207.clone() - add_res_limb_14_col122.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_14_col207.clone() - mul_res_limb_14_col151.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_15_col208.clone() - op1_limb_15_col95.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_15_col208.clone() - add_res_limb_15_col123.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_15_col208.clone() - mul_res_limb_15_col152.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_16_col209.clone() - op1_limb_16_col96.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_16_col209.clone() - add_res_limb_16_col124.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_16_col209.clone() - mul_res_limb_16_col153.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_17_col210.clone() - op1_limb_17_col97.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_17_col210.clone() - add_res_limb_17_col125.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_17_col210.clone() - mul_res_limb_17_col154.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_18_col211.clone() - op1_limb_18_col98.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_18_col211.clone() - add_res_limb_18_col126.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_18_col211.clone() - mul_res_limb_18_col155.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_19_col212.clone() - op1_limb_19_col99.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_19_col212.clone() - add_res_limb_19_col127.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_19_col212.clone() - mul_res_limb_19_col156.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_20_col213.clone() - op1_limb_20_col100.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_20_col213.clone() - add_res_limb_20_col128.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_20_col213.clone() - mul_res_limb_20_col157.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_21_col214.clone() - op1_limb_21_col101.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_21_col214.clone() - add_res_limb_21_col129.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_21_col214.clone() - mul_res_limb_21_col158.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_22_col215.clone() - op1_limb_22_col102.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_22_col215.clone() - add_res_limb_22_col130.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_22_col215.clone() - mul_res_limb_22_col159.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_23_col216.clone() - op1_limb_23_col103.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_23_col216.clone() - add_res_limb_23_col131.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_23_col216.clone() - mul_res_limb_23_col160.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_24_col217.clone() - op1_limb_24_col104.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_24_col217.clone() - add_res_limb_24_col132.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_24_col217.clone() - mul_res_limb_24_col161.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_25_col218.clone() - op1_limb_25_col105.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_25_col218.clone() - add_res_limb_25_col133.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_25_col218.clone() - mul_res_limb_25_col162.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_26_col219.clone() - op1_limb_26_col106.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_26_col219.clone() - add_res_limb_26_col134.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_26_col219.clone() - mul_res_limb_26_col163.clone())))),
        );
        eval.add_constraint(
            (res_constrained_tmp_900.clone()
                * (((res_op1_tmp_231.clone()
                    * (res_limb_27_col220.clone() - op1_limb_27_col107.clone()))
                    + (res_add_col11.clone()
                        * (res_limb_27_col220.clone() - add_res_limb_27_col135.clone())))
                    + (res_mul_col12.clone()
                        * (res_limb_27_col220.clone() - mul_res_limb_27_col164.clone())))),
        );

        // HandleOpcodes.

        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_0_col193.clone() - dst_limb_0_col22.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_1_col194.clone() - dst_limb_1_col23.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_2_col195.clone() - dst_limb_2_col24.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_3_col196.clone() - dst_limb_3_col25.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_4_col197.clone() - dst_limb_4_col26.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_5_col198.clone() - dst_limb_5_col27.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_6_col199.clone() - dst_limb_6_col28.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_7_col200.clone() - dst_limb_7_col29.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_8_col201.clone() - dst_limb_8_col30.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_9_col202.clone() - dst_limb_9_col31.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_10_col203.clone() - dst_limb_10_col32.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_11_col204.clone() - dst_limb_11_col33.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_12_col205.clone() - dst_limb_12_col34.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_13_col206.clone() - dst_limb_13_col35.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_14_col207.clone() - dst_limb_14_col36.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_15_col208.clone() - dst_limb_15_col37.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_16_col209.clone() - dst_limb_16_col38.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_17_col210.clone() - dst_limb_17_col39.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_18_col211.clone() - dst_limb_18_col40.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_19_col212.clone() - dst_limb_19_col41.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_20_col213.clone() - dst_limb_20_col42.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_21_col214.clone() - dst_limb_21_col43.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_22_col215.clone() - dst_limb_22_col44.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_23_col216.clone() - dst_limb_23_col45.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_24_col217.clone() - dst_limb_24_col46.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_25_col218.clone() - dst_limb_25_col47.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_26_col219.clone() - dst_limb_26_col48.clone())),
        );
        eval.add_constraint(
            (opcode_assert_eq_col20.clone()
                * (res_limb_27_col220.clone() - dst_limb_27_col49.clone())),
        );
        // ret opcode offset0 equals -2.
        eval.add_constraint(
            (opcode_ret_col19.clone()
                * ((offset0_col3.clone() - M31_32768.clone()) + M31_2.clone())),
        );
        // ret opcode offset2 equals -1.
        eval.add_constraint(
            (opcode_ret_col19.clone()
                * ((offset2_col5.clone() - M31_32768.clone()) + M31_1.clone())),
        );
        // ret opcode flags pc_update_jump and dst_base_fp and op1_base_fp_and_res_op1 are on.
        eval.add_constraint(
            (opcode_ret_col19.clone()
                * ((((M31_4.clone() - pc_update_jump_col13.clone()) - dst_base_fp_col6.clone())
                    - op1_base_fp_col9.clone())
                    - res_op1_tmp_231.clone())),
        );
        // call opcode offset0 equals 0.
        eval.add_constraint(
            (opcode_call_col18.clone() * (offset0_col3.clone() - M31_32768.clone())),
        );
        // call opcode offset1 equals 1.
        eval.add_constraint(
            (opcode_call_col18.clone()
                * (M31_1.clone() - (offset1_col4.clone() - M31_32768.clone()))),
        );
        // call opcode flags op0_base_fp and dst_base_fp are off.
        eval.add_constraint(
            (opcode_call_col18.clone() * (op0_base_fp_col7.clone() + dst_base_fp_col6.clone())),
        );

        // CondFelt252AsAddr.

        // Address limb 3 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_3_col25.clone()));
        // Address limb 4 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_4_col26.clone()));
        // Address limb 5 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_5_col27.clone()));
        // Address limb 6 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_6_col28.clone()));
        // Address limb 7 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_7_col29.clone()));
        // Address limb 8 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_8_col30.clone()));
        // Address limb 9 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_9_col31.clone()));
        // Address limb 10 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_10_col32.clone()));
        // Address limb 11 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_11_col33.clone()));
        // Address limb 12 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_12_col34.clone()));
        // Address limb 13 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_13_col35.clone()));
        // Address limb 14 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_14_col36.clone()));
        // Address limb 15 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_15_col37.clone()));
        // Address limb 16 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_16_col38.clone()));
        // Address limb 17 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_17_col39.clone()));
        // Address limb 18 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_18_col40.clone()));
        // Address limb 19 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_19_col41.clone()));
        // Address limb 20 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_20_col42.clone()));
        // Address limb 21 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_21_col43.clone()));
        // Address limb 22 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_22_col44.clone()));
        // Address limb 23 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_23_col45.clone()));
        // Address limb 24 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_24_col46.clone()));
        // Address limb 25 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_25_col47.clone()));
        // Address limb 26 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_26_col48.clone()));
        // Address limb 27 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * dst_limb_27_col49.clone()));

        eval.add_constraint(
            (opcode_call_col18.clone()
                * (((dst_limb_0_col22.clone() + (dst_limb_1_col23.clone() * M31_512.clone()))
                    + (dst_limb_2_col24.clone() * M31_262144.clone()))
                    - input_fp_col2.clone())),
        );

        // CondFelt252AsAddr.

        // Address limb 3 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_3_col54.clone()));
        // Address limb 4 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_4_col55.clone()));
        // Address limb 5 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_5_col56.clone()));
        // Address limb 6 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_6_col57.clone()));
        // Address limb 7 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_7_col58.clone()));
        // Address limb 8 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_8_col59.clone()));
        // Address limb 9 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_9_col60.clone()));
        // Address limb 10 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_10_col61.clone()));
        // Address limb 11 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_11_col62.clone()));
        // Address limb 12 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_12_col63.clone()));
        // Address limb 13 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_13_col64.clone()));
        // Address limb 14 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_14_col65.clone()));
        // Address limb 15 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_15_col66.clone()));
        // Address limb 16 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_16_col67.clone()));
        // Address limb 17 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_17_col68.clone()));
        // Address limb 18 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_18_col69.clone()));
        // Address limb 19 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_19_col70.clone()));
        // Address limb 20 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_20_col71.clone()));
        // Address limb 21 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_21_col72.clone()));
        // Address limb 22 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_22_col73.clone()));
        // Address limb 23 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_23_col74.clone()));
        // Address limb 24 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_24_col75.clone()));
        // Address limb 25 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_25_col76.clone()));
        // Address limb 26 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_26_col77.clone()));
        // Address limb 27 equals 0.
        eval.add_constraint((opcode_call_col18.clone() * op0_limb_27_col78.clone()));

        eval.add_constraint(
            (opcode_call_col18.clone()
                * (((op0_limb_0_col51.clone() + (op0_limb_1_col52.clone() * M31_512.clone()))
                    + (op0_limb_2_col53.clone() * M31_262144.clone()))
                    - (input_pc_col0.clone() + (M31_1.clone() + op1_imm_col8.clone())))),
        );

        // UpdateRegisters.

        // CondFelt252AsAddr.

        // Address limb 3 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_3_col196.clone()));
        // Address limb 4 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_4_col197.clone()));
        // Address limb 5 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_5_col198.clone()));
        // Address limb 6 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_6_col199.clone()));
        // Address limb 7 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_7_col200.clone()));
        // Address limb 8 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_8_col201.clone()));
        // Address limb 9 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_9_col202.clone()));
        // Address limb 10 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_10_col203.clone()));
        // Address limb 11 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_11_col204.clone()));
        // Address limb 12 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_12_col205.clone()));
        // Address limb 13 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_13_col206.clone()));
        // Address limb 14 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_14_col207.clone()));
        // Address limb 15 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_15_col208.clone()));
        // Address limb 16 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_16_col209.clone()));
        // Address limb 17 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_17_col210.clone()));
        // Address limb 18 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_18_col211.clone()));
        // Address limb 19 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_19_col212.clone()));
        // Address limb 20 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_20_col213.clone()));
        // Address limb 21 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_21_col214.clone()));
        // Address limb 22 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_22_col215.clone()));
        // Address limb 23 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_23_col216.clone()));
        // Address limb 24 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_24_col217.clone()));
        // Address limb 25 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_25_col218.clone()));
        // Address limb 26 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_26_col219.clone()));
        // Address limb 27 equals 0.
        eval.add_constraint((pc_update_jump_col13.clone() * res_limb_27_col220.clone()));

        // CondFelt252AsAddr.

        // Address limb 3 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_3_col25.clone()));
        // Address limb 4 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_4_col26.clone()));
        // Address limb 5 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_5_col27.clone()));
        // Address limb 6 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_6_col28.clone()));
        // Address limb 7 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_7_col29.clone()));
        // Address limb 8 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_8_col30.clone()));
        // Address limb 9 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_9_col31.clone()));
        // Address limb 10 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_10_col32.clone()));
        // Address limb 11 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_11_col33.clone()));
        // Address limb 12 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_12_col34.clone()));
        // Address limb 13 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_13_col35.clone()));
        // Address limb 14 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_14_col36.clone()));
        // Address limb 15 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_15_col37.clone()));
        // Address limb 16 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_16_col38.clone()));
        // Address limb 17 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_17_col39.clone()));
        // Address limb 18 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_18_col40.clone()));
        // Address limb 19 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_19_col41.clone()));
        // Address limb 20 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_20_col42.clone()));
        // Address limb 21 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_21_col43.clone()));
        // Address limb 22 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_22_col44.clone()));
        // Address limb 23 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_23_col45.clone()));
        // Address limb 24 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_24_col46.clone()));
        // Address limb 25 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_25_col47.clone()));
        // Address limb 26 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_26_col48.clone()));
        // Address limb 27 equals 0.
        eval.add_constraint((opcode_ret_col19.clone() * dst_limb_27_col49.clone()));

        // CondFelt252AsRelImm.

        // CondDecodeSmallSign.

        // msb is a bit.
        eval.add_constraint((msb_col221.clone() * (msb_col221.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col222.clone() * (mid_limbs_set_col222.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            (((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * mid_limbs_set_col222.clone())
                * (msb_col221.clone() - M31_1.clone())),
        );

        // rel_imm limb 3 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_3_col196.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 4 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_4_col197.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 5 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_5_col198.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 6 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_6_col199.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 7 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_7_col200.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 8 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_8_col201.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 9 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_9_col202.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 10 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_10_col203.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 11 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_11_col204.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 12 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_12_col205.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 13 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_13_col206.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 14 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_14_col207.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 15 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_15_col208.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 16 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_16_col209.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 17 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_17_col210.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 18 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_18_col211.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 19 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_19_col212.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 20 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_20_col213.clone() - (mid_limbs_set_col222.clone() * M31_511.clone()))),
        );
        // rel_imm limb 21 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_21_col214.clone()
                    - ((M31_136.clone() * msb_col221.clone()) - mid_limbs_set_col222.clone()))),
        );
        // rel_imm limb 22 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_22_col215.clone() - M31_0.clone())),
        );
        // rel_imm limb 23 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_23_col216.clone() - M31_0.clone())),
        );
        // rel_imm limb 24 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_24_col217.clone() - M31_0.clone())),
        );
        // rel_imm limb 25 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_25_col218.clone() - M31_0.clone())),
        );
        // rel_imm limb 26 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_26_col219.clone() - M31_0.clone())),
        );
        // rel_imm limb 27 is fixed.
        eval.add_constraint(
            ((pc_update_jump_rel_col14.clone() + ap_update_add_col16.clone())
                * (res_limb_27_col220.clone() - (msb_col221.clone() * M31_256.clone()))),
        );

        let diff_from_p_tmp_915 = (dst_limb_0_col22.clone() - M31_1.clone());
        let diff_from_p_tmp_916 = (dst_limb_21_col43.clone() - M31_136.clone());
        let diff_from_p_tmp_917 = (dst_limb_27_col49.clone() - M31_256.clone());
        // dst_not_p.
        eval.add_constraint(
            ((((((((((((((((((((((((((((((M31_0.clone()
                + (diff_from_p_tmp_915.clone()
                    * diff_from_p_tmp_915.clone()))
                + dst_limb_1_col23.clone())
                + dst_limb_2_col24.clone())
                + dst_limb_3_col25.clone())
                + dst_limb_4_col26.clone())
                + dst_limb_5_col27.clone())
                + dst_limb_6_col28.clone())
                + dst_limb_7_col29.clone())
                + dst_limb_8_col30.clone())
                + dst_limb_9_col31.clone())
                + dst_limb_10_col32.clone())
                + dst_limb_11_col33.clone())
                + dst_limb_12_col34.clone())
                + dst_limb_13_col35.clone())
                + dst_limb_14_col36.clone())
                + dst_limb_15_col37.clone())
                + dst_limb_16_col38.clone())
                + dst_limb_17_col39.clone())
                + dst_limb_18_col40.clone())
                + dst_limb_19_col41.clone())
                + dst_limb_20_col42.clone())
                + (diff_from_p_tmp_916.clone() * diff_from_p_tmp_916.clone()))
                + dst_limb_22_col44.clone())
                + dst_limb_23_col45.clone())
                + dst_limb_24_col46.clone())
                + dst_limb_25_col47.clone())
                + dst_limb_26_col48.clone())
                + (diff_from_p_tmp_917.clone() * diff_from_p_tmp_917.clone()))
                * dst_sum_squares_inv_col223.clone())
                - M31_1.clone()),
        );
        // op1_as_rel_imm_cond.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                - (pc_update_jnz_col15.clone()
                    * ((((((((((((((((((((((((((((M31_0.clone()
                        + dst_limb_0_col22.clone())
                        + dst_limb_1_col23.clone())
                        + dst_limb_2_col24.clone())
                        + dst_limb_3_col25.clone())
                        + dst_limb_4_col26.clone())
                        + dst_limb_5_col27.clone())
                        + dst_limb_6_col28.clone())
                        + dst_limb_7_col29.clone())
                        + dst_limb_8_col30.clone())
                        + dst_limb_9_col31.clone())
                        + dst_limb_10_col32.clone())
                        + dst_limb_11_col33.clone())
                        + dst_limb_12_col34.clone())
                        + dst_limb_13_col35.clone())
                        + dst_limb_14_col36.clone())
                        + dst_limb_15_col37.clone())
                        + dst_limb_16_col38.clone())
                        + dst_limb_17_col39.clone())
                        + dst_limb_18_col40.clone())
                        + dst_limb_19_col41.clone())
                        + dst_limb_20_col42.clone())
                        + dst_limb_21_col43.clone())
                        + dst_limb_22_col44.clone())
                        + dst_limb_23_col45.clone())
                        + dst_limb_24_col46.clone())
                        + dst_limb_25_col47.clone())
                        + dst_limb_26_col48.clone())
                        + dst_limb_27_col49.clone()))),
        );

        // CondFelt252AsRelImm.

        // CondDecodeSmallSign.

        // msb is a bit.
        eval.add_constraint((msb_col226.clone() * (msb_col226.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col227.clone() * (mid_limbs_set_col227.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((op1_as_rel_imm_cond_col225.clone() * mid_limbs_set_col227.clone())
                * (msb_col226.clone() - M31_1.clone())),
        );

        // rel_imm limb 3 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_3_col83.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 4 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_4_col84.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 5 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_5_col85.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 6 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_6_col86.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 7 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_7_col87.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 8 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_8_col88.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 9 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_9_col89.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 10 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_10_col90.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 11 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_11_col91.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 12 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_12_col92.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 13 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_13_col93.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 14 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_14_col94.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 15 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_15_col95.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 16 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_16_col96.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 17 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_17_col97.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 18 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_18_col98.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 19 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_19_col99.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 20 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_20_col100.clone() - (mid_limbs_set_col227.clone() * M31_511.clone()))),
        );
        // rel_imm limb 21 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_21_col101.clone()
                    - ((M31_136.clone() * msb_col226.clone()) - mid_limbs_set_col227.clone()))),
        );
        // rel_imm limb 22 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone() * (op1_limb_22_col102.clone() - M31_0.clone())),
        );
        // rel_imm limb 23 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone() * (op1_limb_23_col103.clone() - M31_0.clone())),
        );
        // rel_imm limb 24 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone() * (op1_limb_24_col104.clone() - M31_0.clone())),
        );
        // rel_imm limb 25 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone() * (op1_limb_25_col105.clone() - M31_0.clone())),
        );
        // rel_imm limb 26 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone() * (op1_limb_26_col106.clone() - M31_0.clone())),
        );
        // rel_imm limb 27 is fixed.
        eval.add_constraint(
            (op1_as_rel_imm_cond_col225.clone()
                * (op1_limb_27_col107.clone() - (msb_col226.clone() * M31_256.clone()))),
        );

        // Constraint1 for conditional jump.
        eval.add_constraint(
            ((next_pc_jnz_col228.clone()
                - (input_pc_col0.clone()
                    + ((((op1_limb_0_col80.clone()
                        + (op1_limb_1_col81.clone() * M31_512.clone()))
                        + (op1_limb_2_col82.clone() * M31_262144.clone()))
                        - msb_col226.clone())
                        - (M31_134217728.clone() * mid_limbs_set_col227.clone()))))
                * ((((((((((((((((((((((((((((M31_0.clone()
                    + dst_limb_0_col22.clone())
                    + dst_limb_1_col23.clone())
                    + dst_limb_2_col24.clone())
                    + dst_limb_3_col25.clone())
                    + dst_limb_4_col26.clone())
                    + dst_limb_5_col27.clone())
                    + dst_limb_6_col28.clone())
                    + dst_limb_7_col29.clone())
                    + dst_limb_8_col30.clone())
                    + dst_limb_9_col31.clone())
                    + dst_limb_10_col32.clone())
                    + dst_limb_11_col33.clone())
                    + dst_limb_12_col34.clone())
                    + dst_limb_13_col35.clone())
                    + dst_limb_14_col36.clone())
                    + dst_limb_15_col37.clone())
                    + dst_limb_16_col38.clone())
                    + dst_limb_17_col39.clone())
                    + dst_limb_18_col40.clone())
                    + dst_limb_19_col41.clone())
                    + dst_limb_20_col42.clone())
                    + dst_limb_21_col43.clone())
                    + dst_limb_22_col44.clone())
                    + dst_limb_23_col45.clone())
                    + dst_limb_24_col46.clone())
                    + dst_limb_25_col47.clone())
                    + dst_limb_26_col48.clone())
                    + dst_limb_27_col49.clone())),
        );
        // Constraint2 for conditional jump.
        eval.add_constraint(
            ((next_pc_jnz_col228.clone()
                - (input_pc_col0.clone() + (M31_1.clone() + op1_imm_col8.clone())))
                * ((((((((((((((((((((((((((((((M31_0.clone()
                    + dst_limb_0_col22.clone())
                    + dst_limb_1_col23.clone())
                    + dst_limb_2_col24.clone())
                    + dst_limb_3_col25.clone())
                    + dst_limb_4_col26.clone())
                    + dst_limb_5_col27.clone())
                    + dst_limb_6_col28.clone())
                    + dst_limb_7_col29.clone())
                    + dst_limb_8_col30.clone())
                    + dst_limb_9_col31.clone())
                    + dst_limb_10_col32.clone())
                    + dst_limb_11_col33.clone())
                    + dst_limb_12_col34.clone())
                    + dst_limb_13_col35.clone())
                    + dst_limb_14_col36.clone())
                    + dst_limb_15_col37.clone())
                    + dst_limb_16_col38.clone())
                    + dst_limb_17_col39.clone())
                    + dst_limb_18_col40.clone())
                    + dst_limb_19_col41.clone())
                    + dst_limb_20_col42.clone())
                    + dst_limb_21_col43.clone())
                    + dst_limb_22_col44.clone())
                    + dst_limb_23_col45.clone())
                    + dst_limb_24_col46.clone())
                    + dst_limb_25_col47.clone())
                    + dst_limb_26_col48.clone())
                    + dst_limb_27_col49.clone())
                    * dst_sum_inv_col224.clone())
                    - M31_1.clone())),
        );

        // let frac = Fraction::new(
        //     E::EF::one(),
        //     self.opcodes_lookup_elements.combine(&[
        //         input_pc_col0.clone(),
        //         input_ap_col1.clone(),
        //         input_fp_col2.clone(),
        //     ]),
        // );
        // logup.write_frac(&mut eval, frac);
        // let frac = Fraction::new(
        //     -E::EF::one(),
        //     self.opcodes_lookup_elements.combine(&[
        //         ((((pc_update_regular_tmp_232.clone()
        //             * (input_pc_col0.clone() + (M31_1.clone() + op1_imm_col8.clone())))
        //             + (pc_update_jump_col13.clone()
        //                 * ((res_limb_0_col193.clone()
        //                     + (res_limb_1_col194.clone() * M31_512.clone()))
        //                     + (res_limb_2_col195.clone() * M31_262144.clone()))))
        //             + (pc_update_jump_rel_col14.clone()
        //                 * (input_pc_col0.clone()
        //                     + ((((res_limb_0_col193.clone()
        //                         + (res_limb_1_col194.clone() * M31_512.clone()))
        //                         + (res_limb_2_col195.clone() * M31_262144.clone()))
        //                         - msb_col221.clone())
        //                         - (M31_134217728.clone() * mid_limbs_set_col222.clone())))))
        //             + (pc_update_jnz_col15.clone() * next_pc_jnz_col228.clone())),
        //         (((input_ap_col1.clone()
        //             + (ap_update_add_col16.clone()
        //                 * ((((res_limb_0_col193.clone()
        //                     + (res_limb_1_col194.clone() * M31_512.clone()))
        //                     + (res_limb_2_col195.clone() * M31_262144.clone()))
        //                     - msb_col221.clone())
        //                     - (M31_134217728.clone() * mid_limbs_set_col222.clone()))))
        //             + (ap_update_add_1_col17.clone() * M31_1.clone()))
        //             + (opcode_call_col18.clone() * M31_2.clone())),
        //         (((fp_update_regular_tmp_234.clone() * input_fp_col2.clone())
        //             + (opcode_ret_col19.clone()
        //                 * ((dst_limb_0_col22.clone()
        //                     + (dst_limb_1_col23.clone() * M31_512.clone()))
        //                     + (dst_limb_2_col24.clone() * M31_262144.clone()))))
        //             + (opcode_call_col18.clone() * (input_ap_col1.clone() + M31_2.clone()))),
        //     ]),
        // );
        // logup.write_frac(&mut eval, frac);
        logup.finalize(&mut eval);

        eval
    }
}

use crate::components::prelude::*;
use crate::components::subroutines::bitwise_xor_num_bits_9::BitwiseXorNumBits9;
use crate::components::subroutines::mem_verify::MemVerify;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;

pub const N_TRACE_COLUMNS: usize = 89;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
    pub bitwise_builtin_segment_start: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 19];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
        channel.mix_u64(self.bitwise_builtin_segment_start as u64);
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
        let M31_1073741824 = E::F::from(M31::from(1073741824));
        let M31_2 = E::F::from(M31::from(2));
        let M31_3 = E::F::from(M31::from(3));
        let M31_4 = E::F::from(M31::from(4));
        let M31_5 = E::F::from(M31::from(5));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let op0_id_col0 = eval.next_trace_mask();
        let op0_limb_0_col1 = eval.next_trace_mask();
        let op0_limb_1_col2 = eval.next_trace_mask();
        let op0_limb_2_col3 = eval.next_trace_mask();
        let op0_limb_3_col4 = eval.next_trace_mask();
        let op0_limb_4_col5 = eval.next_trace_mask();
        let op0_limb_5_col6 = eval.next_trace_mask();
        let op0_limb_6_col7 = eval.next_trace_mask();
        let op0_limb_7_col8 = eval.next_trace_mask();
        let op0_limb_8_col9 = eval.next_trace_mask();
        let op0_limb_9_col10 = eval.next_trace_mask();
        let op0_limb_10_col11 = eval.next_trace_mask();
        let op0_limb_11_col12 = eval.next_trace_mask();
        let op0_limb_12_col13 = eval.next_trace_mask();
        let op0_limb_13_col14 = eval.next_trace_mask();
        let op0_limb_14_col15 = eval.next_trace_mask();
        let op0_limb_15_col16 = eval.next_trace_mask();
        let op0_limb_16_col17 = eval.next_trace_mask();
        let op0_limb_17_col18 = eval.next_trace_mask();
        let op0_limb_18_col19 = eval.next_trace_mask();
        let op0_limb_19_col20 = eval.next_trace_mask();
        let op0_limb_20_col21 = eval.next_trace_mask();
        let op0_limb_21_col22 = eval.next_trace_mask();
        let op0_limb_22_col23 = eval.next_trace_mask();
        let op0_limb_23_col24 = eval.next_trace_mask();
        let op0_limb_24_col25 = eval.next_trace_mask();
        let op0_limb_25_col26 = eval.next_trace_mask();
        let op0_limb_26_col27 = eval.next_trace_mask();
        let op0_limb_27_col28 = eval.next_trace_mask();
        let op1_id_col29 = eval.next_trace_mask();
        let op1_limb_0_col30 = eval.next_trace_mask();
        let op1_limb_1_col31 = eval.next_trace_mask();
        let op1_limb_2_col32 = eval.next_trace_mask();
        let op1_limb_3_col33 = eval.next_trace_mask();
        let op1_limb_4_col34 = eval.next_trace_mask();
        let op1_limb_5_col35 = eval.next_trace_mask();
        let op1_limb_6_col36 = eval.next_trace_mask();
        let op1_limb_7_col37 = eval.next_trace_mask();
        let op1_limb_8_col38 = eval.next_trace_mask();
        let op1_limb_9_col39 = eval.next_trace_mask();
        let op1_limb_10_col40 = eval.next_trace_mask();
        let op1_limb_11_col41 = eval.next_trace_mask();
        let op1_limb_12_col42 = eval.next_trace_mask();
        let op1_limb_13_col43 = eval.next_trace_mask();
        let op1_limb_14_col44 = eval.next_trace_mask();
        let op1_limb_15_col45 = eval.next_trace_mask();
        let op1_limb_16_col46 = eval.next_trace_mask();
        let op1_limb_17_col47 = eval.next_trace_mask();
        let op1_limb_18_col48 = eval.next_trace_mask();
        let op1_limb_19_col49 = eval.next_trace_mask();
        let op1_limb_20_col50 = eval.next_trace_mask();
        let op1_limb_21_col51 = eval.next_trace_mask();
        let op1_limb_22_col52 = eval.next_trace_mask();
        let op1_limb_23_col53 = eval.next_trace_mask();
        let op1_limb_24_col54 = eval.next_trace_mask();
        let op1_limb_25_col55 = eval.next_trace_mask();
        let op1_limb_26_col56 = eval.next_trace_mask();
        let op1_limb_27_col57 = eval.next_trace_mask();
        let xor_col58 = eval.next_trace_mask();
        let xor_col59 = eval.next_trace_mask();
        let xor_col60 = eval.next_trace_mask();
        let xor_col61 = eval.next_trace_mask();
        let xor_col62 = eval.next_trace_mask();
        let xor_col63 = eval.next_trace_mask();
        let xor_col64 = eval.next_trace_mask();
        let xor_col65 = eval.next_trace_mask();
        let xor_col66 = eval.next_trace_mask();
        let xor_col67 = eval.next_trace_mask();
        let xor_col68 = eval.next_trace_mask();
        let xor_col69 = eval.next_trace_mask();
        let xor_col70 = eval.next_trace_mask();
        let xor_col71 = eval.next_trace_mask();
        let xor_col72 = eval.next_trace_mask();
        let xor_col73 = eval.next_trace_mask();
        let xor_col74 = eval.next_trace_mask();
        let xor_col75 = eval.next_trace_mask();
        let xor_col76 = eval.next_trace_mask();
        let xor_col77 = eval.next_trace_mask();
        let xor_col78 = eval.next_trace_mask();
        let xor_col79 = eval.next_trace_mask();
        let xor_col80 = eval.next_trace_mask();
        let xor_col81 = eval.next_trace_mask();
        let xor_col82 = eval.next_trace_mask();
        let xor_col83 = eval.next_trace_mask();
        let xor_col84 = eval.next_trace_mask();
        let xor_col85 = eval.next_trace_mask();
        let and_id_col86 = eval.next_trace_mask();
        let xor_id_col87 = eval.next_trace_mask();
        let or_id_col88 = eval.next_trace_mask();

        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_252_output_tmp_efb2a_2_limb_0, read_positive_num_bits_252_output_tmp_efb2a_2_limb_1, read_positive_num_bits_252_output_tmp_efb2a_2_limb_2, read_positive_num_bits_252_output_tmp_efb2a_2_limb_3, read_positive_num_bits_252_output_tmp_efb2a_2_limb_4, read_positive_num_bits_252_output_tmp_efb2a_2_limb_5, read_positive_num_bits_252_output_tmp_efb2a_2_limb_6, read_positive_num_bits_252_output_tmp_efb2a_2_limb_7, read_positive_num_bits_252_output_tmp_efb2a_2_limb_8, read_positive_num_bits_252_output_tmp_efb2a_2_limb_9, read_positive_num_bits_252_output_tmp_efb2a_2_limb_10, read_positive_num_bits_252_output_tmp_efb2a_2_limb_11, read_positive_num_bits_252_output_tmp_efb2a_2_limb_12, read_positive_num_bits_252_output_tmp_efb2a_2_limb_13, read_positive_num_bits_252_output_tmp_efb2a_2_limb_14, read_positive_num_bits_252_output_tmp_efb2a_2_limb_15, read_positive_num_bits_252_output_tmp_efb2a_2_limb_16, read_positive_num_bits_252_output_tmp_efb2a_2_limb_17, read_positive_num_bits_252_output_tmp_efb2a_2_limb_18, read_positive_num_bits_252_output_tmp_efb2a_2_limb_19, read_positive_num_bits_252_output_tmp_efb2a_2_limb_20, read_positive_num_bits_252_output_tmp_efb2a_2_limb_21, read_positive_num_bits_252_output_tmp_efb2a_2_limb_22, read_positive_num_bits_252_output_tmp_efb2a_2_limb_23, read_positive_num_bits_252_output_tmp_efb2a_2_limb_24, read_positive_num_bits_252_output_tmp_efb2a_2_limb_25, read_positive_num_bits_252_output_tmp_efb2a_2_limb_26, read_positive_num_bits_252_output_tmp_efb2a_2_limb_27, read_positive_num_bits_252_output_tmp_efb2a_2_limb_28] =
            ReadPositiveNumBits252::evaluate(
                (E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone())),
                op0_id_col0.clone(),
                op0_limb_0_col1.clone(),
                op0_limb_1_col2.clone(),
                op0_limb_2_col3.clone(),
                op0_limb_3_col4.clone(),
                op0_limb_4_col5.clone(),
                op0_limb_5_col6.clone(),
                op0_limb_6_col7.clone(),
                op0_limb_7_col8.clone(),
                op0_limb_8_col9.clone(),
                op0_limb_9_col10.clone(),
                op0_limb_10_col11.clone(),
                op0_limb_11_col12.clone(),
                op0_limb_12_col13.clone(),
                op0_limb_13_col14.clone(),
                op0_limb_14_col15.clone(),
                op0_limb_15_col16.clone(),
                op0_limb_16_col17.clone(),
                op0_limb_17_col18.clone(),
                op0_limb_18_col19.clone(),
                op0_limb_19_col20.clone(),
                op0_limb_20_col21.clone(),
                op0_limb_21_col22.clone(),
                op0_limb_22_col23.clone(),
                op0_limb_23_col24.clone(),
                op0_limb_24_col25.clone(),
                op0_limb_25_col26.clone(),
                op0_limb_26_col27.clone(),
                op0_limb_27_col28.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let [read_positive_num_bits_252_output_tmp_efb2a_5_limb_0, read_positive_num_bits_252_output_tmp_efb2a_5_limb_1, read_positive_num_bits_252_output_tmp_efb2a_5_limb_2, read_positive_num_bits_252_output_tmp_efb2a_5_limb_3, read_positive_num_bits_252_output_tmp_efb2a_5_limb_4, read_positive_num_bits_252_output_tmp_efb2a_5_limb_5, read_positive_num_bits_252_output_tmp_efb2a_5_limb_6, read_positive_num_bits_252_output_tmp_efb2a_5_limb_7, read_positive_num_bits_252_output_tmp_efb2a_5_limb_8, read_positive_num_bits_252_output_tmp_efb2a_5_limb_9, read_positive_num_bits_252_output_tmp_efb2a_5_limb_10, read_positive_num_bits_252_output_tmp_efb2a_5_limb_11, read_positive_num_bits_252_output_tmp_efb2a_5_limb_12, read_positive_num_bits_252_output_tmp_efb2a_5_limb_13, read_positive_num_bits_252_output_tmp_efb2a_5_limb_14, read_positive_num_bits_252_output_tmp_efb2a_5_limb_15, read_positive_num_bits_252_output_tmp_efb2a_5_limb_16, read_positive_num_bits_252_output_tmp_efb2a_5_limb_17, read_positive_num_bits_252_output_tmp_efb2a_5_limb_18, read_positive_num_bits_252_output_tmp_efb2a_5_limb_19, read_positive_num_bits_252_output_tmp_efb2a_5_limb_20, read_positive_num_bits_252_output_tmp_efb2a_5_limb_21, read_positive_num_bits_252_output_tmp_efb2a_5_limb_22, read_positive_num_bits_252_output_tmp_efb2a_5_limb_23, read_positive_num_bits_252_output_tmp_efb2a_5_limb_24, read_positive_num_bits_252_output_tmp_efb2a_5_limb_25, read_positive_num_bits_252_output_tmp_efb2a_5_limb_26, read_positive_num_bits_252_output_tmp_efb2a_5_limb_27, read_positive_num_bits_252_output_tmp_efb2a_5_limb_28] =
            ReadPositiveNumBits252::evaluate(
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_1.clone()),
                op1_id_col29.clone(),
                op1_limb_0_col30.clone(),
                op1_limb_1_col31.clone(),
                op1_limb_2_col32.clone(),
                op1_limb_3_col33.clone(),
                op1_limb_4_col34.clone(),
                op1_limb_5_col35.clone(),
                op1_limb_6_col36.clone(),
                op1_limb_7_col37.clone(),
                op1_limb_8_col38.clone(),
                op1_limb_9_col39.clone(),
                op1_limb_10_col40.clone(),
                op1_limb_11_col41.clone(),
                op1_limb_12_col42.clone(),
                op1_limb_13_col43.clone(),
                op1_limb_14_col44.clone(),
                op1_limb_15_col45.clone(),
                op1_limb_16_col46.clone(),
                op1_limb_17_col47.clone(),
                op1_limb_18_col48.clone(),
                op1_limb_19_col49.clone(),
                op1_limb_20_col50.clone(),
                op1_limb_21_col51.clone(),
                op1_limb_22_col52.clone(),
                op1_limb_23_col53.clone(),
                op1_limb_24_col54.clone(),
                op1_limb_25_col55.clone(),
                op1_limb_26_col56.clone(),
                op1_limb_27_col57.clone(),
                &mut eval,
                &self.memory_address_to_id_lookup_elements,
                &self.memory_id_to_big_lookup_elements,
            );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_7 = BitwiseXorNumBits9::evaluate(
            [op0_limb_0_col1.clone(), op1_limb_0_col30.clone()],
            xor_col58.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_8 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_0_col1.clone() + op1_limb_0_col30.clone()) - xor_col58.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_10 = BitwiseXorNumBits9::evaluate(
            [op0_limb_1_col2.clone(), op1_limb_1_col31.clone()],
            xor_col59.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_11 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_1_col2.clone() + op1_limb_1_col31.clone()) - xor_col59.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_13 = BitwiseXorNumBits9::evaluate(
            [op0_limb_2_col3.clone(), op1_limb_2_col32.clone()],
            xor_col60.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_14 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_2_col3.clone() + op1_limb_2_col32.clone()) - xor_col60.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_16 = BitwiseXorNumBits9::evaluate(
            [op0_limb_3_col4.clone(), op1_limb_3_col33.clone()],
            xor_col61.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_17 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_3_col4.clone() + op1_limb_3_col33.clone()) - xor_col61.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_19 = BitwiseXorNumBits9::evaluate(
            [op0_limb_4_col5.clone(), op1_limb_4_col34.clone()],
            xor_col62.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_20 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_4_col5.clone() + op1_limb_4_col34.clone()) - xor_col62.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_22 = BitwiseXorNumBits9::evaluate(
            [op0_limb_5_col6.clone(), op1_limb_5_col35.clone()],
            xor_col63.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_23 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_5_col6.clone() + op1_limb_5_col35.clone()) - xor_col63.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_25 = BitwiseXorNumBits9::evaluate(
            [op0_limb_6_col7.clone(), op1_limb_6_col36.clone()],
            xor_col64.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_26 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_6_col7.clone() + op1_limb_6_col36.clone()) - xor_col64.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_28 = BitwiseXorNumBits9::evaluate(
            [op0_limb_7_col8.clone(), op1_limb_7_col37.clone()],
            xor_col65.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_29 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_7_col8.clone() + op1_limb_7_col37.clone()) - xor_col65.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_31 = BitwiseXorNumBits9::evaluate(
            [op0_limb_8_col9.clone(), op1_limb_8_col38.clone()],
            xor_col66.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_32 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_8_col9.clone() + op1_limb_8_col38.clone()) - xor_col66.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_34 = BitwiseXorNumBits9::evaluate(
            [op0_limb_9_col10.clone(), op1_limb_9_col39.clone()],
            xor_col67.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_35 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_9_col10.clone() + op1_limb_9_col39.clone()) - xor_col67.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_37 = BitwiseXorNumBits9::evaluate(
            [op0_limb_10_col11.clone(), op1_limb_10_col40.clone()],
            xor_col68.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_38 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_10_col11.clone() + op1_limb_10_col40.clone()) - xor_col68.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_40 = BitwiseXorNumBits9::evaluate(
            [op0_limb_11_col12.clone(), op1_limb_11_col41.clone()],
            xor_col69.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_41 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_11_col12.clone() + op1_limb_11_col41.clone()) - xor_col69.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_43 = BitwiseXorNumBits9::evaluate(
            [op0_limb_12_col13.clone(), op1_limb_12_col42.clone()],
            xor_col70.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_44 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_12_col13.clone() + op1_limb_12_col42.clone()) - xor_col70.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_46 = BitwiseXorNumBits9::evaluate(
            [op0_limb_13_col14.clone(), op1_limb_13_col43.clone()],
            xor_col71.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_47 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_13_col14.clone() + op1_limb_13_col43.clone()) - xor_col71.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_49 = BitwiseXorNumBits9::evaluate(
            [op0_limb_14_col15.clone(), op1_limb_14_col44.clone()],
            xor_col72.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_50 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_14_col15.clone() + op1_limb_14_col44.clone()) - xor_col72.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_52 = BitwiseXorNumBits9::evaluate(
            [op0_limb_15_col16.clone(), op1_limb_15_col45.clone()],
            xor_col73.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_53 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_15_col16.clone() + op1_limb_15_col45.clone()) - xor_col73.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_55 = BitwiseXorNumBits9::evaluate(
            [op0_limb_16_col17.clone(), op1_limb_16_col46.clone()],
            xor_col74.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_56 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_16_col17.clone() + op1_limb_16_col46.clone()) - xor_col74.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_58 = BitwiseXorNumBits9::evaluate(
            [op0_limb_17_col18.clone(), op1_limb_17_col47.clone()],
            xor_col75.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_59 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_17_col18.clone() + op1_limb_17_col47.clone()) - xor_col75.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_61 = BitwiseXorNumBits9::evaluate(
            [op0_limb_18_col19.clone(), op1_limb_18_col48.clone()],
            xor_col76.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_62 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_18_col19.clone() + op1_limb_18_col48.clone()) - xor_col76.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_64 = BitwiseXorNumBits9::evaluate(
            [op0_limb_19_col20.clone(), op1_limb_19_col49.clone()],
            xor_col77.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_65 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_19_col20.clone() + op1_limb_19_col49.clone()) - xor_col77.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_67 = BitwiseXorNumBits9::evaluate(
            [op0_limb_20_col21.clone(), op1_limb_20_col50.clone()],
            xor_col78.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_68 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_20_col21.clone() + op1_limb_20_col50.clone()) - xor_col78.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_70 = BitwiseXorNumBits9::evaluate(
            [op0_limb_21_col22.clone(), op1_limb_21_col51.clone()],
            xor_col79.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_71 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_21_col22.clone() + op1_limb_21_col51.clone()) - xor_col79.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_73 = BitwiseXorNumBits9::evaluate(
            [op0_limb_22_col23.clone(), op1_limb_22_col52.clone()],
            xor_col80.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_74 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_22_col23.clone() + op1_limb_22_col52.clone()) - xor_col80.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_76 = BitwiseXorNumBits9::evaluate(
            [op0_limb_23_col24.clone(), op1_limb_23_col53.clone()],
            xor_col81.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_77 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_23_col24.clone() + op1_limb_23_col53.clone()) - xor_col81.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_79 = BitwiseXorNumBits9::evaluate(
            [op0_limb_24_col25.clone(), op1_limb_24_col54.clone()],
            xor_col82.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_80 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_24_col25.clone() + op1_limb_24_col54.clone()) - xor_col82.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_82 = BitwiseXorNumBits9::evaluate(
            [op0_limb_25_col26.clone(), op1_limb_25_col55.clone()],
            xor_col83.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_83 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_25_col26.clone() + op1_limb_25_col55.clone()) - xor_col83.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_85 = BitwiseXorNumBits9::evaluate(
            [op0_limb_26_col27.clone(), op1_limb_26_col56.clone()],
            xor_col84.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_86 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_26_col27.clone() + op1_limb_26_col56.clone()) - xor_col84.clone())),
        );
        #[allow(clippy::unused_unit)]
        #[allow(unused_variables)]
        let bitwise_xor_num_bits_9_output_tmp_efb2a_88 = BitwiseXorNumBits9::evaluate(
            [op0_limb_27_col28.clone(), op1_limb_27_col57.clone()],
            xor_col85.clone(),
            &mut eval,
            &self.verify_bitwise_xor_9_lookup_elements,
        );
        let and_tmp_efb2a_89 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_27_col28.clone() + op1_limb_27_col57.clone()) - xor_col85.clone())),
        );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_2.clone()),
                and_tmp_efb2a_8.clone(),
                and_tmp_efb2a_11.clone(),
                and_tmp_efb2a_14.clone(),
                and_tmp_efb2a_17.clone(),
                and_tmp_efb2a_20.clone(),
                and_tmp_efb2a_23.clone(),
                and_tmp_efb2a_26.clone(),
                and_tmp_efb2a_29.clone(),
                and_tmp_efb2a_32.clone(),
                and_tmp_efb2a_35.clone(),
                and_tmp_efb2a_38.clone(),
                and_tmp_efb2a_41.clone(),
                and_tmp_efb2a_44.clone(),
                and_tmp_efb2a_47.clone(),
                and_tmp_efb2a_50.clone(),
                and_tmp_efb2a_53.clone(),
                and_tmp_efb2a_56.clone(),
                and_tmp_efb2a_59.clone(),
                and_tmp_efb2a_62.clone(),
                and_tmp_efb2a_65.clone(),
                and_tmp_efb2a_68.clone(),
                and_tmp_efb2a_71.clone(),
                and_tmp_efb2a_74.clone(),
                and_tmp_efb2a_77.clone(),
                and_tmp_efb2a_80.clone(),
                and_tmp_efb2a_83.clone(),
                and_tmp_efb2a_86.clone(),
                and_tmp_efb2a_89.clone(),
            ],
            and_id_col86.clone(),
            &mut eval,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
        );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_3.clone()),
                xor_col58.clone(),
                xor_col59.clone(),
                xor_col60.clone(),
                xor_col61.clone(),
                xor_col62.clone(),
                xor_col63.clone(),
                xor_col64.clone(),
                xor_col65.clone(),
                xor_col66.clone(),
                xor_col67.clone(),
                xor_col68.clone(),
                xor_col69.clone(),
                xor_col70.clone(),
                xor_col71.clone(),
                xor_col72.clone(),
                xor_col73.clone(),
                xor_col74.clone(),
                xor_col75.clone(),
                xor_col76.clone(),
                xor_col77.clone(),
                xor_col78.clone(),
                xor_col79.clone(),
                xor_col80.clone(),
                xor_col81.clone(),
                xor_col82.clone(),
                xor_col83.clone(),
                xor_col84.clone(),
                xor_col85.clone(),
            ],
            xor_id_col87.clone(),
            &mut eval,
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
        );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_4.clone()),
                (and_tmp_efb2a_8.clone() + xor_col58.clone()),
                (and_tmp_efb2a_11.clone() + xor_col59.clone()),
                (and_tmp_efb2a_14.clone() + xor_col60.clone()),
                (and_tmp_efb2a_17.clone() + xor_col61.clone()),
                (and_tmp_efb2a_20.clone() + xor_col62.clone()),
                (and_tmp_efb2a_23.clone() + xor_col63.clone()),
                (and_tmp_efb2a_26.clone() + xor_col64.clone()),
                (and_tmp_efb2a_29.clone() + xor_col65.clone()),
                (and_tmp_efb2a_32.clone() + xor_col66.clone()),
                (and_tmp_efb2a_35.clone() + xor_col67.clone()),
                (and_tmp_efb2a_38.clone() + xor_col68.clone()),
                (and_tmp_efb2a_41.clone() + xor_col69.clone()),
                (and_tmp_efb2a_44.clone() + xor_col70.clone()),
                (and_tmp_efb2a_47.clone() + xor_col71.clone()),
                (and_tmp_efb2a_50.clone() + xor_col72.clone()),
                (and_tmp_efb2a_53.clone() + xor_col73.clone()),
                (and_tmp_efb2a_56.clone() + xor_col74.clone()),
                (and_tmp_efb2a_59.clone() + xor_col75.clone()),
                (and_tmp_efb2a_62.clone() + xor_col76.clone()),
                (and_tmp_efb2a_65.clone() + xor_col77.clone()),
                (and_tmp_efb2a_68.clone() + xor_col78.clone()),
                (and_tmp_efb2a_71.clone() + xor_col79.clone()),
                (and_tmp_efb2a_74.clone() + xor_col80.clone()),
                (and_tmp_efb2a_77.clone() + xor_col81.clone()),
                (and_tmp_efb2a_80.clone() + xor_col82.clone()),
                (and_tmp_efb2a_83.clone() + xor_col83.clone()),
                (and_tmp_efb2a_86.clone() + xor_col84.clone()),
                (and_tmp_efb2a_89.clone() + xor_col85.clone()),
            ],
            or_id_col88.clone(),
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
    use crate::components::constraints_regression_test_values::BITWISE_BUILTIN;

    #[test]
    fn bitwise_builtin_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            claim: Claim {
                log_size: 4,
                bitwise_builtin_segment_start: rng.gen::<u32>(),
            },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9::dummy(),
        };
        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, BITWISE_BUILTIN);
    }
}

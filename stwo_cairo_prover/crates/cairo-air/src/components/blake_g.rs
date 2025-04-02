use crate::components::prelude::*;

pub const N_TRACE_COLUMNS: usize = 53;

pub struct Eval {
    pub claim: Claim,
    pub blake_g_lookup_elements: relations::BlakeG,
    pub verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12,
    pub verify_bitwise_xor_4_lookup_elements: relations::VerifyBitwiseXor_4,
    pub verify_bitwise_xor_7_lookup_elements: relations::VerifyBitwiseXor_7,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 9];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
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
        let M31_128 = E::F::from(M31::from(128));
        let M31_16 = E::F::from(M31::from(16));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4096 = E::F::from(M31::from(4096));
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
        let input_limb_10_col10 = eval.next_trace_mask();
        let input_limb_11_col11 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col12 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col13 = eval.next_trace_mask();
        let ms_8_bits_col14 = eval.next_trace_mask();
        let ms_8_bits_col15 = eval.next_trace_mask();
        let ms_8_bits_col16 = eval.next_trace_mask();
        let ms_8_bits_col17 = eval.next_trace_mask();
        let xor_col18 = eval.next_trace_mask();
        let xor_col19 = eval.next_trace_mask();
        let xor_col20 = eval.next_trace_mask();
        let xor_col21 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col22 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col23 = eval.next_trace_mask();
        let ms_4_bits_col24 = eval.next_trace_mask();
        let ms_4_bits_col25 = eval.next_trace_mask();
        let ms_4_bits_col26 = eval.next_trace_mask();
        let ms_4_bits_col27 = eval.next_trace_mask();
        let xor_col28 = eval.next_trace_mask();
        let xor_col29 = eval.next_trace_mask();
        let xor_col30 = eval.next_trace_mask();
        let xor_col31 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col32 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col33 = eval.next_trace_mask();
        let ms_8_bits_col34 = eval.next_trace_mask();
        let ms_8_bits_col35 = eval.next_trace_mask();
        let ms_8_bits_col36 = eval.next_trace_mask();
        let ms_8_bits_col37 = eval.next_trace_mask();
        let xor_col38 = eval.next_trace_mask();
        let xor_col39 = eval.next_trace_mask();
        let xor_col40 = eval.next_trace_mask();
        let xor_col41 = eval.next_trace_mask();
        let triple_sum32_res_limb_0_col42 = eval.next_trace_mask();
        let triple_sum32_res_limb_1_col43 = eval.next_trace_mask();
        let ms_9_bits_col44 = eval.next_trace_mask();
        let ms_9_bits_col45 = eval.next_trace_mask();
        let ms_9_bits_col46 = eval.next_trace_mask();
        let ms_9_bits_col47 = eval.next_trace_mask();
        let xor_col48 = eval.next_trace_mask();
        let xor_col49 = eval.next_trace_mask();
        let xor_col50 = eval.next_trace_mask();
        let xor_col51 = eval.next_trace_mask();
        let enabler = eval.next_trace_mask();

        eval.add_constraint(enabler.clone() * enabler.clone() - enabler.clone());

        // Triple Sum 32.

        let carry_low_tmp_f72c8_1 = eval.add_intermediate(
            ((((input_limb_0_col0.clone() + input_limb_2_col2.clone())
                + input_limb_8_col8.clone())
                - triple_sum32_res_limb_0_col12.clone())
                * M31_32768.clone()),
        );
        // carry low is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_low_tmp_f72c8_1.clone() * (carry_low_tmp_f72c8_1.clone() - M31_1.clone()))
                * (carry_low_tmp_f72c8_1.clone() - M31_2.clone())),
        );
        let carry_high_tmp_f72c8_2 = eval.add_intermediate(
            (((((input_limb_1_col1.clone() + input_limb_3_col3.clone())
                + input_limb_9_col9.clone())
                + carry_low_tmp_f72c8_1.clone())
                - triple_sum32_res_limb_1_col13.clone())
                * M31_32768.clone()),
        );
        // carry high is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_high_tmp_f72c8_2.clone() * (carry_high_tmp_f72c8_2.clone() - M31_1.clone()))
                * (carry_high_tmp_f72c8_2.clone() - M31_2.clone())),
        );

        // Xor Rot 32 R 16.

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_5_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_0_col12.clone() - (ms_8_bits_col14.clone() * M31_256.clone())),
        );

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_7_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_1_col13.clone() - (ms_8_bits_col15.clone() * M31_256.clone())),
        );

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_9_limb_0 = eval.add_intermediate(
            (input_limb_6_col6.clone() - (ms_8_bits_col16.clone() * M31_256.clone())),
        );

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_11_limb_0 = eval.add_intermediate(
            (input_limb_7_col7.clone() - (ms_8_bits_col17.clone() * M31_256.clone())),
        );

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_8_output_tmp_f72c8_5_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_f72c8_9_limb_0.clone(),
                xor_col18.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                ms_8_bits_col14.clone(),
                ms_8_bits_col16.clone(),
                xor_col19.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_8_output_tmp_f72c8_7_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_f72c8_11_limb_0.clone(),
                xor_col20.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                ms_8_bits_col15.clone(),
                ms_8_bits_col17.clone(),
                xor_col21.clone(),
            ],
        ));

        let xor_rot_16_output_tmp_f72c8_20_limb_0 =
            eval.add_intermediate((xor_col20.clone() + (xor_col21.clone() * M31_256.clone())));
        let xor_rot_16_output_tmp_f72c8_20_limb_1 =
            eval.add_intermediate((xor_col18.clone() + (xor_col19.clone() * M31_256.clone())));
        let xor_rot_32_r_16_output_tmp_f72c8_21_limb_0 =
            eval.add_intermediate(xor_rot_16_output_tmp_f72c8_20_limb_0.clone());
        let xor_rot_32_r_16_output_tmp_f72c8_21_limb_1 =
            eval.add_intermediate(xor_rot_16_output_tmp_f72c8_20_limb_1.clone());

        // Triple Sum 32.

        let carry_low_tmp_f72c8_23 = eval.add_intermediate(
            (((input_limb_4_col4.clone() + xor_rot_32_r_16_output_tmp_f72c8_21_limb_0.clone())
                - triple_sum32_res_limb_0_col22.clone())
                * M31_32768.clone()),
        );
        // carry low is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_low_tmp_f72c8_23.clone() * (carry_low_tmp_f72c8_23.clone() - M31_1.clone()))
                * (carry_low_tmp_f72c8_23.clone() - M31_2.clone())),
        );
        let carry_high_tmp_f72c8_24 = eval.add_intermediate(
            ((((input_limb_5_col5.clone() + xor_rot_32_r_16_output_tmp_f72c8_21_limb_1.clone())
                + carry_low_tmp_f72c8_23.clone())
                - triple_sum32_res_limb_1_col23.clone())
                * M31_32768.clone()),
        );
        // carry high is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_high_tmp_f72c8_24.clone() * (carry_high_tmp_f72c8_24.clone() - M31_1.clone()))
                * (carry_high_tmp_f72c8_24.clone() - M31_2.clone())),
        );

        // Xor Rot 32 R 12.

        // Split 16 Low Part Size 12.

        let split_16_low_part_size_12_output_tmp_f72c8_27_limb_0 = eval.add_intermediate(
            (input_limb_2_col2.clone() - (ms_4_bits_col24.clone() * M31_4096.clone())),
        );

        // Split 16 Low Part Size 12.

        let split_16_low_part_size_12_output_tmp_f72c8_29_limb_0 = eval.add_intermediate(
            (input_limb_3_col3.clone() - (ms_4_bits_col25.clone() * M31_4096.clone())),
        );

        // Split 16 Low Part Size 12.

        let split_16_low_part_size_12_output_tmp_f72c8_31_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_0_col22.clone() - (ms_4_bits_col26.clone() * M31_4096.clone())),
        );

        // Split 16 Low Part Size 12.

        let split_16_low_part_size_12_output_tmp_f72c8_33_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_1_col23.clone() - (ms_4_bits_col27.clone() * M31_4096.clone())),
        );

        // Bitwise Xor Num Bits 12.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_12_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_12_output_tmp_f72c8_27_limb_0.clone(),
                split_16_low_part_size_12_output_tmp_f72c8_31_limb_0.clone(),
                xor_col28.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 4.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_4_lookup_elements,
            E::EF::one(),
            &[
                ms_4_bits_col24.clone(),
                ms_4_bits_col26.clone(),
                xor_col29.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 12.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_12_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_12_output_tmp_f72c8_29_limb_0.clone(),
                split_16_low_part_size_12_output_tmp_f72c8_33_limb_0.clone(),
                xor_col30.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 4.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_4_lookup_elements,
            E::EF::one(),
            &[
                ms_4_bits_col25.clone(),
                ms_4_bits_col27.clone(),
                xor_col31.clone(),
            ],
        ));

        let xor_rot_12_output_tmp_f72c8_42_limb_0 =
            eval.add_intermediate((xor_col29.clone() + (xor_col30.clone() * M31_16.clone())));
        let xor_rot_12_output_tmp_f72c8_42_limb_1 =
            eval.add_intermediate((xor_col31.clone() + (xor_col28.clone() * M31_16.clone())));
        let xor_rot_32_r_12_output_tmp_f72c8_43_limb_0 =
            eval.add_intermediate(xor_rot_12_output_tmp_f72c8_42_limb_0.clone());
        let xor_rot_32_r_12_output_tmp_f72c8_43_limb_1 =
            eval.add_intermediate(xor_rot_12_output_tmp_f72c8_42_limb_1.clone());

        // Triple Sum 32.

        let carry_low_tmp_f72c8_45 = eval.add_intermediate(
            ((((triple_sum32_res_limb_0_col12.clone()
                + xor_rot_32_r_12_output_tmp_f72c8_43_limb_0.clone())
                + input_limb_10_col10.clone())
                - triple_sum32_res_limb_0_col32.clone())
                * M31_32768.clone()),
        );
        // carry low is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_low_tmp_f72c8_45.clone() * (carry_low_tmp_f72c8_45.clone() - M31_1.clone()))
                * (carry_low_tmp_f72c8_45.clone() - M31_2.clone())),
        );
        let carry_high_tmp_f72c8_46 = eval.add_intermediate(
            (((((triple_sum32_res_limb_1_col13.clone()
                + xor_rot_32_r_12_output_tmp_f72c8_43_limb_1.clone())
                + input_limb_11_col11.clone())
                + carry_low_tmp_f72c8_45.clone())
                - triple_sum32_res_limb_1_col33.clone())
                * M31_32768.clone()),
        );
        // carry high is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_high_tmp_f72c8_46.clone() * (carry_high_tmp_f72c8_46.clone() - M31_1.clone()))
                * (carry_high_tmp_f72c8_46.clone() - M31_2.clone())),
        );

        // Xor Rot 32 R 8.

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_49_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_0_col32.clone() - (ms_8_bits_col34.clone() * M31_256.clone())),
        );

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_51_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_1_col33.clone() - (ms_8_bits_col35.clone() * M31_256.clone())),
        );

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_53_limb_0 = eval.add_intermediate(
            (xor_rot_32_r_16_output_tmp_f72c8_21_limb_0.clone()
                - (ms_8_bits_col36.clone() * M31_256.clone())),
        );

        // Split 16 Low Part Size 8.

        let split_16_low_part_size_8_output_tmp_f72c8_55_limb_0 = eval.add_intermediate(
            (xor_rot_32_r_16_output_tmp_f72c8_21_limb_1.clone()
                - (ms_8_bits_col37.clone() * M31_256.clone())),
        );

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_8_output_tmp_f72c8_49_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_f72c8_53_limb_0.clone(),
                xor_col38.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                ms_8_bits_col34.clone(),
                ms_8_bits_col36.clone(),
                xor_col39.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_8_output_tmp_f72c8_51_limb_0.clone(),
                split_16_low_part_size_8_output_tmp_f72c8_55_limb_0.clone(),
                xor_col40.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 8.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_8_lookup_elements,
            E::EF::one(),
            &[
                ms_8_bits_col35.clone(),
                ms_8_bits_col37.clone(),
                xor_col41.clone(),
            ],
        ));

        let xor_rot_8_output_tmp_f72c8_64_limb_0 =
            eval.add_intermediate((xor_col39.clone() + (xor_col40.clone() * M31_256.clone())));
        let xor_rot_8_output_tmp_f72c8_64_limb_1 =
            eval.add_intermediate((xor_col41.clone() + (xor_col38.clone() * M31_256.clone())));
        let xor_rot_32_r_8_output_tmp_f72c8_65_limb_0 =
            eval.add_intermediate(xor_rot_8_output_tmp_f72c8_64_limb_0.clone());
        let xor_rot_32_r_8_output_tmp_f72c8_65_limb_1 =
            eval.add_intermediate(xor_rot_8_output_tmp_f72c8_64_limb_1.clone());

        // Triple Sum 32.

        let carry_low_tmp_f72c8_67 = eval.add_intermediate(
            (((triple_sum32_res_limb_0_col22.clone()
                + xor_rot_32_r_8_output_tmp_f72c8_65_limb_0.clone())
                - triple_sum32_res_limb_0_col42.clone())
                * M31_32768.clone()),
        );
        // carry low is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_low_tmp_f72c8_67.clone() * (carry_low_tmp_f72c8_67.clone() - M31_1.clone()))
                * (carry_low_tmp_f72c8_67.clone() - M31_2.clone())),
        );
        let carry_high_tmp_f72c8_68 = eval.add_intermediate(
            ((((triple_sum32_res_limb_1_col23.clone()
                + xor_rot_32_r_8_output_tmp_f72c8_65_limb_1.clone())
                + carry_low_tmp_f72c8_67.clone())
                - triple_sum32_res_limb_1_col43.clone())
                * M31_32768.clone()),
        );
        // carry high is 0 or 1 or 2.
        eval.add_constraint(
            ((carry_high_tmp_f72c8_68.clone() * (carry_high_tmp_f72c8_68.clone() - M31_1.clone()))
                * (carry_high_tmp_f72c8_68.clone() - M31_2.clone())),
        );

        // Xor Rot 32 R 7.

        // Split 16 Low Part Size 7.

        let split_16_low_part_size_7_output_tmp_f72c8_71_limb_0 = eval.add_intermediate(
            (xor_rot_32_r_12_output_tmp_f72c8_43_limb_0.clone()
                - (ms_9_bits_col44.clone() * M31_128.clone())),
        );

        // Split 16 Low Part Size 7.

        let split_16_low_part_size_7_output_tmp_f72c8_73_limb_0 = eval.add_intermediate(
            (xor_rot_32_r_12_output_tmp_f72c8_43_limb_1.clone()
                - (ms_9_bits_col45.clone() * M31_128.clone())),
        );

        // Split 16 Low Part Size 7.

        let split_16_low_part_size_7_output_tmp_f72c8_75_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_0_col42.clone() - (ms_9_bits_col46.clone() * M31_128.clone())),
        );

        // Split 16 Low Part Size 7.

        let split_16_low_part_size_7_output_tmp_f72c8_77_limb_0 = eval.add_intermediate(
            (triple_sum32_res_limb_1_col43.clone() - (ms_9_bits_col47.clone() * M31_128.clone())),
        );

        // Bitwise Xor Num Bits 7.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_7_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_7_output_tmp_f72c8_71_limb_0.clone(),
                split_16_low_part_size_7_output_tmp_f72c8_75_limb_0.clone(),
                xor_col48.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 9.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_9_lookup_elements,
            E::EF::one(),
            &[
                ms_9_bits_col44.clone(),
                ms_9_bits_col46.clone(),
                xor_col49.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 7.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_7_lookup_elements,
            E::EF::one(),
            &[
                split_16_low_part_size_7_output_tmp_f72c8_73_limb_0.clone(),
                split_16_low_part_size_7_output_tmp_f72c8_77_limb_0.clone(),
                xor_col50.clone(),
            ],
        ));

        // Bitwise Xor Num Bits 9.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_bitwise_xor_9_lookup_elements,
            E::EF::one(),
            &[
                ms_9_bits_col45.clone(),
                ms_9_bits_col47.clone(),
                xor_col51.clone(),
            ],
        ));

        let xor_rot_7_output_tmp_f72c8_86_limb_0 =
            eval.add_intermediate((xor_col49.clone() + (xor_col50.clone() * M31_512.clone())));
        let xor_rot_7_output_tmp_f72c8_86_limb_1 =
            eval.add_intermediate((xor_col51.clone() + (xor_col48.clone() * M31_512.clone())));
        let xor_rot_32_r_7_output_tmp_f72c8_87_limb_0 =
            eval.add_intermediate(xor_rot_7_output_tmp_f72c8_86_limb_0.clone());
        let xor_rot_32_r_7_output_tmp_f72c8_87_limb_1 =
            eval.add_intermediate(xor_rot_7_output_tmp_f72c8_86_limb_1.clone());

        eval.add_to_relation(RelationEntry::new(
            &self.blake_g_lookup_elements,
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
                input_limb_10_col10.clone(),
                input_limb_11_col11.clone(),
                triple_sum32_res_limb_0_col32.clone(),
                triple_sum32_res_limb_1_col33.clone(),
                xor_rot_32_r_7_output_tmp_f72c8_87_limb_0.clone(),
                xor_rot_32_r_7_output_tmp_f72c8_87_limb_1.clone(),
                triple_sum32_res_limb_0_col42.clone(),
                triple_sum32_res_limb_1_col43.clone(),
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_0.clone(),
                xor_rot_32_r_8_output_tmp_f72c8_65_limb_1.clone(),
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
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

    use super::*;
    use crate::components::constraints_regression_test_values::BLAKE_G;

    #[test]
    fn blake_g_constraints_regression() {
        let eval = Eval {
            claim: Claim { log_size: 4 },
            blake_g_lookup_elements: relations::BlakeG::dummy(),
            verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12::dummy(),
            verify_bitwise_xor_4_lookup_elements: relations::VerifyBitwiseXor_4::dummy(),
            verify_bitwise_xor_7_lookup_elements: relations::VerifyBitwiseXor_7::dummy(),
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
            verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let mut rng = SmallRng::seed_from_u64(0);
        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.random_eval() * rng.gen::<QM31>();
        }

        assert_eq!(sum, BLAKE_G);
    }
}

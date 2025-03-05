use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub cube_252_lookup_elements: relations::Cube252,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub verify_mul_252_lookup_elements: relations::VerifyMul252,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 84];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE * 23];
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
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_512 = E::F::from(M31::from(512));
        let M31_8192 = E::F::from(M31::from(8192));
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
        let mul_res_limb_0_col56 = eval.next_trace_mask();
        let mul_res_limb_1_col57 = eval.next_trace_mask();
        let mul_res_limb_2_col58 = eval.next_trace_mask();
        let mul_res_limb_3_col59 = eval.next_trace_mask();
        let mul_res_limb_4_col60 = eval.next_trace_mask();
        let mul_res_limb_5_col61 = eval.next_trace_mask();
        let mul_res_limb_6_col62 = eval.next_trace_mask();
        let mul_res_limb_7_col63 = eval.next_trace_mask();
        let mul_res_limb_8_col64 = eval.next_trace_mask();
        let mul_res_limb_9_col65 = eval.next_trace_mask();
        let mul_res_limb_10_col66 = eval.next_trace_mask();
        let mul_res_limb_11_col67 = eval.next_trace_mask();
        let mul_res_limb_12_col68 = eval.next_trace_mask();
        let mul_res_limb_13_col69 = eval.next_trace_mask();
        let mul_res_limb_14_col70 = eval.next_trace_mask();
        let mul_res_limb_15_col71 = eval.next_trace_mask();
        let mul_res_limb_16_col72 = eval.next_trace_mask();
        let mul_res_limb_17_col73 = eval.next_trace_mask();
        let mul_res_limb_18_col74 = eval.next_trace_mask();
        let mul_res_limb_19_col75 = eval.next_trace_mask();
        let mul_res_limb_20_col76 = eval.next_trace_mask();
        let mul_res_limb_21_col77 = eval.next_trace_mask();
        let mul_res_limb_22_col78 = eval.next_trace_mask();
        let mul_res_limb_23_col79 = eval.next_trace_mask();
        let mul_res_limb_24_col80 = eval.next_trace_mask();
        let mul_res_limb_25_col81 = eval.next_trace_mask();
        let mul_res_limb_26_col82 = eval.next_trace_mask();
        let mul_res_limb_27_col83 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Felt 252 Unpack From 27 Range Check Output.

        // Range Check Mem Value N 28.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[unpacked_limb_0_col10.clone(), unpacked_limb_1_col11.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                (((input_limb_0_col0.clone() - unpacked_limb_0_col10.clone())
                    - (unpacked_limb_1_col11.clone() * M31_512.clone()))
                    * M31_8192.clone()),
                unpacked_limb_3_col12.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_4_col13.clone(),
                (((input_limb_1_col1.clone() - unpacked_limb_3_col12.clone())
                    - (unpacked_limb_4_col13.clone() * M31_512.clone()))
                    * M31_8192.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[unpacked_limb_6_col14.clone(), unpacked_limb_7_col15.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                (((input_limb_2_col2.clone() - unpacked_limb_6_col14.clone())
                    - (unpacked_limb_7_col15.clone() * M31_512.clone()))
                    * M31_8192.clone()),
                unpacked_limb_9_col16.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_10_col17.clone(),
                (((input_limb_3_col3.clone() - unpacked_limb_9_col16.clone())
                    - (unpacked_limb_10_col17.clone() * M31_512.clone()))
                    * M31_8192.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                (((input_limb_4_col4.clone() - unpacked_limb_12_col18.clone())
                    - (unpacked_limb_13_col19.clone() * M31_512.clone()))
                    * M31_8192.clone()),
                unpacked_limb_15_col20.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_16_col21.clone(),
                (((input_limb_5_col5.clone() - unpacked_limb_15_col20.clone())
                    - (unpacked_limb_16_col21.clone() * M31_512.clone()))
                    * M31_8192.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                (((input_limb_6_col6.clone() - unpacked_limb_18_col22.clone())
                    - (unpacked_limb_19_col23.clone() * M31_512.clone()))
                    * M31_8192.clone()),
                unpacked_limb_21_col24.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_22_col25.clone(),
                (((input_limb_7_col7.clone() - unpacked_limb_21_col24.clone())
                    - (unpacked_limb_22_col25.clone() * M31_512.clone()))
                    * M31_8192.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[
                (((input_limb_8_col8.clone() - unpacked_limb_24_col26.clone())
                    - (unpacked_limb_25_col27.clone() * M31_512.clone()))
                    * M31_8192.clone()),
                input_limb_9_col9.clone(),
            ],
        ));

        let a_tmp_fec87_2_limb_2 = eval.add_intermediate(
            (((input_limb_0_col0.clone() - unpacked_limb_0_col10.clone())
                - (unpacked_limb_1_col11.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_5 = eval.add_intermediate(
            (((input_limb_1_col1.clone() - unpacked_limb_3_col12.clone())
                - (unpacked_limb_4_col13.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_8 = eval.add_intermediate(
            (((input_limb_2_col2.clone() - unpacked_limb_6_col14.clone())
                - (unpacked_limb_7_col15.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_11 = eval.add_intermediate(
            (((input_limb_3_col3.clone() - unpacked_limb_9_col16.clone())
                - (unpacked_limb_10_col17.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_14 = eval.add_intermediate(
            (((input_limb_4_col4.clone() - unpacked_limb_12_col18.clone())
                - (unpacked_limb_13_col19.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_17 = eval.add_intermediate(
            (((input_limb_5_col5.clone() - unpacked_limb_15_col20.clone())
                - (unpacked_limb_16_col21.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_20 = eval.add_intermediate(
            (((input_limb_6_col6.clone() - unpacked_limb_18_col22.clone())
                - (unpacked_limb_19_col23.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_23 = eval.add_intermediate(
            (((input_limb_7_col7.clone() - unpacked_limb_21_col24.clone())
                - (unpacked_limb_22_col25.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );
        let a_tmp_fec87_2_limb_26 = eval.add_intermediate(
            (((input_limb_8_col8.clone() - unpacked_limb_24_col26.clone())
                - (unpacked_limb_25_col27.clone() * M31_512.clone()))
                * M31_8192.clone()),
        );

        // Mul 252.

        // Range Check Mem Value N 28.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_0_col28.clone(), mul_res_limb_1_col29.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_2_col30.clone(), mul_res_limb_3_col31.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_4_col32.clone(), mul_res_limb_5_col33.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_6_col34.clone(), mul_res_limb_7_col35.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_8_col36.clone(), mul_res_limb_9_col37.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_10_col38.clone(), mul_res_limb_11_col39.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_12_col40.clone(), mul_res_limb_13_col41.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_14_col42.clone(), mul_res_limb_15_col43.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_16_col44.clone(), mul_res_limb_17_col45.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_18_col46.clone(), mul_res_limb_19_col47.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_20_col48.clone(), mul_res_limb_21_col49.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_22_col50.clone(), mul_res_limb_23_col51.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_24_col52.clone(), mul_res_limb_25_col53.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_26_col54.clone(), mul_res_limb_27_col55.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.verify_mul_252_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                a_tmp_fec87_2_limb_2.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                a_tmp_fec87_2_limb_5.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                a_tmp_fec87_2_limb_8.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                a_tmp_fec87_2_limb_11.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                a_tmp_fec87_2_limb_14.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                a_tmp_fec87_2_limb_17.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                a_tmp_fec87_2_limb_20.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                a_tmp_fec87_2_limb_23.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                a_tmp_fec87_2_limb_26.clone(),
                input_limb_9_col9.clone(),
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                a_tmp_fec87_2_limb_2.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                a_tmp_fec87_2_limb_5.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                a_tmp_fec87_2_limb_8.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                a_tmp_fec87_2_limb_11.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                a_tmp_fec87_2_limb_14.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                a_tmp_fec87_2_limb_17.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                a_tmp_fec87_2_limb_20.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                a_tmp_fec87_2_limb_23.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                a_tmp_fec87_2_limb_26.clone(),
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
        ));

        // Mul 252.

        // Range Check Mem Value N 28.

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_0_col56.clone(), mul_res_limb_1_col57.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_2_col58.clone(), mul_res_limb_3_col59.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_4_col60.clone(), mul_res_limb_5_col61.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_6_col62.clone(), mul_res_limb_7_col63.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_8_col64.clone(), mul_res_limb_9_col65.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_10_col66.clone(), mul_res_limb_11_col67.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_12_col68.clone(), mul_res_limb_13_col69.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_14_col70.clone(), mul_res_limb_15_col71.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_16_col72.clone(), mul_res_limb_17_col73.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_18_col74.clone(), mul_res_limb_19_col75.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_20_col76.clone(), mul_res_limb_21_col77.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_22_col78.clone(), mul_res_limb_23_col79.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_24_col80.clone(), mul_res_limb_25_col81.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.range_check_9_9_lookup_elements,
            E::EF::one(),
            &[mul_res_limb_26_col82.clone(), mul_res_limb_27_col83.clone()],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.verify_mul_252_lookup_elements,
            E::EF::one(),
            &[
                unpacked_limb_0_col10.clone(),
                unpacked_limb_1_col11.clone(),
                a_tmp_fec87_2_limb_2.clone(),
                unpacked_limb_3_col12.clone(),
                unpacked_limb_4_col13.clone(),
                a_tmp_fec87_2_limb_5.clone(),
                unpacked_limb_6_col14.clone(),
                unpacked_limb_7_col15.clone(),
                a_tmp_fec87_2_limb_8.clone(),
                unpacked_limb_9_col16.clone(),
                unpacked_limb_10_col17.clone(),
                a_tmp_fec87_2_limb_11.clone(),
                unpacked_limb_12_col18.clone(),
                unpacked_limb_13_col19.clone(),
                a_tmp_fec87_2_limb_14.clone(),
                unpacked_limb_15_col20.clone(),
                unpacked_limb_16_col21.clone(),
                a_tmp_fec87_2_limb_17.clone(),
                unpacked_limb_18_col22.clone(),
                unpacked_limb_19_col23.clone(),
                a_tmp_fec87_2_limb_20.clone(),
                unpacked_limb_21_col24.clone(),
                unpacked_limb_22_col25.clone(),
                a_tmp_fec87_2_limb_23.clone(),
                unpacked_limb_24_col26.clone(),
                unpacked_limb_25_col27.clone(),
                a_tmp_fec87_2_limb_26.clone(),
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
                mul_res_limb_0_col56.clone(),
                mul_res_limb_1_col57.clone(),
                mul_res_limb_2_col58.clone(),
                mul_res_limb_3_col59.clone(),
                mul_res_limb_4_col60.clone(),
                mul_res_limb_5_col61.clone(),
                mul_res_limb_6_col62.clone(),
                mul_res_limb_7_col63.clone(),
                mul_res_limb_8_col64.clone(),
                mul_res_limb_9_col65.clone(),
                mul_res_limb_10_col66.clone(),
                mul_res_limb_11_col67.clone(),
                mul_res_limb_12_col68.clone(),
                mul_res_limb_13_col69.clone(),
                mul_res_limb_14_col70.clone(),
                mul_res_limb_15_col71.clone(),
                mul_res_limb_16_col72.clone(),
                mul_res_limb_17_col73.clone(),
                mul_res_limb_18_col74.clone(),
                mul_res_limb_19_col75.clone(),
                mul_res_limb_20_col76.clone(),
                mul_res_limb_21_col77.clone(),
                mul_res_limb_22_col78.clone(),
                mul_res_limb_23_col79.clone(),
                mul_res_limb_24_col80.clone(),
                mul_res_limb_25_col81.clone(),
                mul_res_limb_26_col82.clone(),
                mul_res_limb_27_col83.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.cube_252_lookup_elements,
            -E::EF::from(padding.clone()),
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
                ((mul_res_limb_0_col56.clone() + (mul_res_limb_1_col57.clone() * M31_512.clone()))
                    + (mul_res_limb_2_col58.clone() * M31_262144.clone())),
                ((mul_res_limb_3_col59.clone() + (mul_res_limb_4_col60.clone() * M31_512.clone()))
                    + (mul_res_limb_5_col61.clone() * M31_262144.clone())),
                ((mul_res_limb_6_col62.clone() + (mul_res_limb_7_col63.clone() * M31_512.clone()))
                    + (mul_res_limb_8_col64.clone() * M31_262144.clone())),
                ((mul_res_limb_9_col65.clone()
                    + (mul_res_limb_10_col66.clone() * M31_512.clone()))
                    + (mul_res_limb_11_col67.clone() * M31_262144.clone())),
                ((mul_res_limb_12_col68.clone()
                    + (mul_res_limb_13_col69.clone() * M31_512.clone()))
                    + (mul_res_limb_14_col70.clone() * M31_262144.clone())),
                ((mul_res_limb_15_col71.clone()
                    + (mul_res_limb_16_col72.clone() * M31_512.clone()))
                    + (mul_res_limb_17_col73.clone() * M31_262144.clone())),
                ((mul_res_limb_18_col74.clone()
                    + (mul_res_limb_19_col75.clone() * M31_512.clone()))
                    + (mul_res_limb_20_col76.clone() * M31_262144.clone())),
                ((mul_res_limb_21_col77.clone()
                    + (mul_res_limb_22_col78.clone() * M31_512.clone()))
                    + (mul_res_limb_23_col79.clone() * M31_262144.clone())),
                ((mul_res_limb_24_col80.clone()
                    + (mul_res_limb_25_col81.clone() * M31_512.clone()))
                    + (mul_res_limb_26_col82.clone() * M31_262144.clone())),
                mul_res_limb_27_col83.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

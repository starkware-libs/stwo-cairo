// This file was created by the AIR team.

use crate::components::prelude::*;
use crate::components::subroutines::bitwise_xor_num_bits_8::BitwiseXorNumBits8;
use crate::components::subroutines::bitwise_xor_num_bits_9::BitwiseXorNumBits9;
use crate::components::subroutines::mem_verify::MemVerify;
use crate::components::subroutines::read_positive_num_bits_252::ReadPositiveNumBits252;

pub const N_TRACE_COLUMNS: usize = 89;
pub const RELATION_USES_PER_ROW: [RelationUse; 4] = [
    RelationUse {
        relation_id: "MemoryAddressToId",
        uses: 5,
    },
    RelationUse {
        relation_id: "MemoryIdToBig",
        uses: 5,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_8",
        uses: 1,
    },
    RelationUse {
        relation_id: "VerifyBitwiseXor_9",
        uses: 27,
    },
];

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub verify_bitwise_xor_9_lookup_elements: relations::VerifyBitwiseXor_9,
    pub verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

        ReadPositiveNumBits252::evaluate(
            [
                (E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone())),
            ],
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
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        ReadPositiveNumBits252::evaluate(
            [
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_1.clone()),
            ],
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
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_0_col1.clone(), op1_limb_0_col30.clone()],
            xor_col58.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_12 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_0_col1.clone() + op1_limb_0_col30.clone()) - xor_col58.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_1_col2.clone(), op1_limb_1_col31.clone()],
            xor_col59.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_15 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_1_col2.clone() + op1_limb_1_col31.clone()) - xor_col59.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_2_col3.clone(), op1_limb_2_col32.clone()],
            xor_col60.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_18 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_2_col3.clone() + op1_limb_2_col32.clone()) - xor_col60.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_3_col4.clone(), op1_limb_3_col33.clone()],
            xor_col61.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_21 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_3_col4.clone() + op1_limb_3_col33.clone()) - xor_col61.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_4_col5.clone(), op1_limb_4_col34.clone()],
            xor_col62.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_24 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_4_col5.clone() + op1_limb_4_col34.clone()) - xor_col62.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_5_col6.clone(), op1_limb_5_col35.clone()],
            xor_col63.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_27 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_5_col6.clone() + op1_limb_5_col35.clone()) - xor_col63.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_6_col7.clone(), op1_limb_6_col36.clone()],
            xor_col64.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_30 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_6_col7.clone() + op1_limb_6_col36.clone()) - xor_col64.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_7_col8.clone(), op1_limb_7_col37.clone()],
            xor_col65.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_33 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_7_col8.clone() + op1_limb_7_col37.clone()) - xor_col65.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_8_col9.clone(), op1_limb_8_col38.clone()],
            xor_col66.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_36 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_8_col9.clone() + op1_limb_8_col38.clone()) - xor_col66.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_9_col10.clone(), op1_limb_9_col39.clone()],
            xor_col67.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_39 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_9_col10.clone() + op1_limb_9_col39.clone()) - xor_col67.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_10_col11.clone(), op1_limb_10_col40.clone()],
            xor_col68.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_42 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_10_col11.clone() + op1_limb_10_col40.clone()) - xor_col68.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_11_col12.clone(), op1_limb_11_col41.clone()],
            xor_col69.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_45 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_11_col12.clone() + op1_limb_11_col41.clone()) - xor_col69.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_12_col13.clone(), op1_limb_12_col42.clone()],
            xor_col70.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_48 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_12_col13.clone() + op1_limb_12_col42.clone()) - xor_col70.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_13_col14.clone(), op1_limb_13_col43.clone()],
            xor_col71.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_51 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_13_col14.clone() + op1_limb_13_col43.clone()) - xor_col71.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_14_col15.clone(), op1_limb_14_col44.clone()],
            xor_col72.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_54 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_14_col15.clone() + op1_limb_14_col44.clone()) - xor_col72.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_15_col16.clone(), op1_limb_15_col45.clone()],
            xor_col73.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_57 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_15_col16.clone() + op1_limb_15_col45.clone()) - xor_col73.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_16_col17.clone(), op1_limb_16_col46.clone()],
            xor_col74.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_60 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_16_col17.clone() + op1_limb_16_col46.clone()) - xor_col74.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_17_col18.clone(), op1_limb_17_col47.clone()],
            xor_col75.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_63 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_17_col18.clone() + op1_limb_17_col47.clone()) - xor_col75.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_18_col19.clone(), op1_limb_18_col48.clone()],
            xor_col76.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_66 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_18_col19.clone() + op1_limb_18_col48.clone()) - xor_col76.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_19_col20.clone(), op1_limb_19_col49.clone()],
            xor_col77.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_69 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_19_col20.clone() + op1_limb_19_col49.clone()) - xor_col77.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_20_col21.clone(), op1_limb_20_col50.clone()],
            xor_col78.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_72 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_20_col21.clone() + op1_limb_20_col50.clone()) - xor_col78.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_21_col22.clone(), op1_limb_21_col51.clone()],
            xor_col79.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_75 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_21_col22.clone() + op1_limb_21_col51.clone()) - xor_col79.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_22_col23.clone(), op1_limb_22_col52.clone()],
            xor_col80.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_78 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_22_col23.clone() + op1_limb_22_col52.clone()) - xor_col80.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_23_col24.clone(), op1_limb_23_col53.clone()],
            xor_col81.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_81 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_23_col24.clone() + op1_limb_23_col53.clone()) - xor_col81.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_24_col25.clone(), op1_limb_24_col54.clone()],
            xor_col82.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_84 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_24_col25.clone() + op1_limb_24_col54.clone()) - xor_col82.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_25_col26.clone(), op1_limb_25_col55.clone()],
            xor_col83.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_87 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_25_col26.clone() + op1_limb_25_col55.clone()) - xor_col83.clone())),
        );
        BitwiseXorNumBits9::evaluate(
            [op0_limb_26_col27.clone(), op1_limb_26_col56.clone()],
            xor_col84.clone(),
            &self.verify_bitwise_xor_9_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_90 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_26_col27.clone() + op1_limb_26_col56.clone()) - xor_col84.clone())),
        );
        BitwiseXorNumBits8::evaluate(
            [op0_limb_27_col28.clone(), op1_limb_27_col57.clone()],
            xor_col85.clone(),
            &self.verify_bitwise_xor_8_lookup_elements,
            &mut eval,
        );
        let and_tmp_efb2a_93 = eval.add_intermediate(
            (M31_1073741824.clone()
                * ((op0_limb_27_col28.clone() + op1_limb_27_col57.clone()) - xor_col85.clone())),
        );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_2.clone()),
                and_tmp_efb2a_12.clone(),
                and_tmp_efb2a_15.clone(),
                and_tmp_efb2a_18.clone(),
                and_tmp_efb2a_21.clone(),
                and_tmp_efb2a_24.clone(),
                and_tmp_efb2a_27.clone(),
                and_tmp_efb2a_30.clone(),
                and_tmp_efb2a_33.clone(),
                and_tmp_efb2a_36.clone(),
                and_tmp_efb2a_39.clone(),
                and_tmp_efb2a_42.clone(),
                and_tmp_efb2a_45.clone(),
                and_tmp_efb2a_48.clone(),
                and_tmp_efb2a_51.clone(),
                and_tmp_efb2a_54.clone(),
                and_tmp_efb2a_57.clone(),
                and_tmp_efb2a_60.clone(),
                and_tmp_efb2a_63.clone(),
                and_tmp_efb2a_66.clone(),
                and_tmp_efb2a_69.clone(),
                and_tmp_efb2a_72.clone(),
                and_tmp_efb2a_75.clone(),
                and_tmp_efb2a_78.clone(),
                and_tmp_efb2a_81.clone(),
                and_tmp_efb2a_84.clone(),
                and_tmp_efb2a_87.clone(),
                and_tmp_efb2a_90.clone(),
                and_tmp_efb2a_93.clone(),
            ],
            and_id_col86.clone(),
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
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
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
            &mut eval,
        );
        MemVerify::evaluate(
            [
                ((E::F::from(M31::from(self.claim.bitwise_builtin_segment_start))
                    + (seq.clone() * M31_5.clone()))
                    + M31_4.clone()),
                (and_tmp_efb2a_12.clone() + xor_col58.clone()),
                (and_tmp_efb2a_15.clone() + xor_col59.clone()),
                (and_tmp_efb2a_18.clone() + xor_col60.clone()),
                (and_tmp_efb2a_21.clone() + xor_col61.clone()),
                (and_tmp_efb2a_24.clone() + xor_col62.clone()),
                (and_tmp_efb2a_27.clone() + xor_col63.clone()),
                (and_tmp_efb2a_30.clone() + xor_col64.clone()),
                (and_tmp_efb2a_33.clone() + xor_col65.clone()),
                (and_tmp_efb2a_36.clone() + xor_col66.clone()),
                (and_tmp_efb2a_39.clone() + xor_col67.clone()),
                (and_tmp_efb2a_42.clone() + xor_col68.clone()),
                (and_tmp_efb2a_45.clone() + xor_col69.clone()),
                (and_tmp_efb2a_48.clone() + xor_col70.clone()),
                (and_tmp_efb2a_51.clone() + xor_col71.clone()),
                (and_tmp_efb2a_54.clone() + xor_col72.clone()),
                (and_tmp_efb2a_57.clone() + xor_col73.clone()),
                (and_tmp_efb2a_60.clone() + xor_col74.clone()),
                (and_tmp_efb2a_63.clone() + xor_col75.clone()),
                (and_tmp_efb2a_66.clone() + xor_col76.clone()),
                (and_tmp_efb2a_69.clone() + xor_col77.clone()),
                (and_tmp_efb2a_72.clone() + xor_col78.clone()),
                (and_tmp_efb2a_75.clone() + xor_col79.clone()),
                (and_tmp_efb2a_78.clone() + xor_col80.clone()),
                (and_tmp_efb2a_81.clone() + xor_col81.clone()),
                (and_tmp_efb2a_84.clone() + xor_col82.clone()),
                (and_tmp_efb2a_87.clone() + xor_col83.clone()),
                (and_tmp_efb2a_90.clone() + xor_col84.clone()),
                (and_tmp_efb2a_93.clone() + xor_col85.clone()),
            ],
            or_id_col88.clone(),
            &self.memory_address_to_id_lookup_elements,
            &self.memory_id_to_big_lookup_elements,
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
            verify_bitwise_xor_8_lookup_elements: relations::VerifyBitwiseXor_8::dummy(),
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

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LogupSums, LookupElements};
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::backend::simd::m31::LOG_N_LANES;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::relations;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub range_check_19_lookup_elements: relations::RangeCheck_19,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub n_calls: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let trace_log_sizes = vec![log_size; 128];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 19];
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

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct InteractionClaim {
    pub logup_sums: LogupSums,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        let (total_sum, claimed_sum) = self.logup_sums;
        channel.mix_felts(&[total_sum]);
        if let Some(claimed_sum) = claimed_sum {
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
        let M31_136 = E::F::from(M31::from(136));
        let M31_2 = E::F::from(M31::from(2));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32 = E::F::from(M31::from(32));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_4 = E::F::from(M31::from(4));
        let M31_512 = E::F::from(M31::from(512));
        let M31_64 = E::F::from(M31::from(64));
        let M31_8 = E::F::from(M31::from(8));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let offset1_col4 = eval.next_trace_mask();
        let offset2_col5 = eval.next_trace_mask();
        let dst_base_fp_col6 = eval.next_trace_mask();
        let op0_base_fp_col7 = eval.next_trace_mask();
        let op1_base_fp_col8 = eval.next_trace_mask();
        let op1_base_ap_col9 = eval.next_trace_mask();
        let ap_update_add_1_col10 = eval.next_trace_mask();
        let mem_dst_base_col11 = eval.next_trace_mask();
        let mem0_base_col12 = eval.next_trace_mask();
        let dst_id_col13 = eval.next_trace_mask();
        let dst_limb_0_col14 = eval.next_trace_mask();
        let dst_limb_1_col15 = eval.next_trace_mask();
        let dst_limb_2_col16 = eval.next_trace_mask();
        let dst_limb_3_col17 = eval.next_trace_mask();
        let dst_limb_4_col18 = eval.next_trace_mask();
        let dst_limb_5_col19 = eval.next_trace_mask();
        let dst_limb_6_col20 = eval.next_trace_mask();
        let dst_limb_7_col21 = eval.next_trace_mask();
        let dst_limb_8_col22 = eval.next_trace_mask();
        let dst_limb_9_col23 = eval.next_trace_mask();
        let dst_limb_10_col24 = eval.next_trace_mask();
        let dst_limb_11_col25 = eval.next_trace_mask();
        let dst_limb_12_col26 = eval.next_trace_mask();
        let dst_limb_13_col27 = eval.next_trace_mask();
        let dst_limb_14_col28 = eval.next_trace_mask();
        let dst_limb_15_col29 = eval.next_trace_mask();
        let dst_limb_16_col30 = eval.next_trace_mask();
        let dst_limb_17_col31 = eval.next_trace_mask();
        let dst_limb_18_col32 = eval.next_trace_mask();
        let dst_limb_19_col33 = eval.next_trace_mask();
        let dst_limb_20_col34 = eval.next_trace_mask();
        let dst_limb_21_col35 = eval.next_trace_mask();
        let dst_limb_22_col36 = eval.next_trace_mask();
        let dst_limb_23_col37 = eval.next_trace_mask();
        let dst_limb_24_col38 = eval.next_trace_mask();
        let dst_limb_25_col39 = eval.next_trace_mask();
        let dst_limb_26_col40 = eval.next_trace_mask();
        let dst_limb_27_col41 = eval.next_trace_mask();
        let op0_id_col42 = eval.next_trace_mask();
        let op0_limb_0_col43 = eval.next_trace_mask();
        let op0_limb_1_col44 = eval.next_trace_mask();
        let op0_limb_2_col45 = eval.next_trace_mask();
        let op0_limb_3_col46 = eval.next_trace_mask();
        let op0_limb_4_col47 = eval.next_trace_mask();
        let op0_limb_5_col48 = eval.next_trace_mask();
        let op0_limb_6_col49 = eval.next_trace_mask();
        let op0_limb_7_col50 = eval.next_trace_mask();
        let op0_limb_8_col51 = eval.next_trace_mask();
        let op0_limb_9_col52 = eval.next_trace_mask();
        let op0_limb_10_col53 = eval.next_trace_mask();
        let op0_limb_11_col54 = eval.next_trace_mask();
        let op0_limb_12_col55 = eval.next_trace_mask();
        let op0_limb_13_col56 = eval.next_trace_mask();
        let op0_limb_14_col57 = eval.next_trace_mask();
        let op0_limb_15_col58 = eval.next_trace_mask();
        let op0_limb_16_col59 = eval.next_trace_mask();
        let op0_limb_17_col60 = eval.next_trace_mask();
        let op0_limb_18_col61 = eval.next_trace_mask();
        let op0_limb_19_col62 = eval.next_trace_mask();
        let op0_limb_20_col63 = eval.next_trace_mask();
        let op0_limb_21_col64 = eval.next_trace_mask();
        let op0_limb_22_col65 = eval.next_trace_mask();
        let op0_limb_23_col66 = eval.next_trace_mask();
        let op0_limb_24_col67 = eval.next_trace_mask();
        let op0_limb_25_col68 = eval.next_trace_mask();
        let op0_limb_26_col69 = eval.next_trace_mask();
        let op0_limb_27_col70 = eval.next_trace_mask();
        let op1_id_col71 = eval.next_trace_mask();
        let op1_limb_0_col72 = eval.next_trace_mask();
        let op1_limb_1_col73 = eval.next_trace_mask();
        let op1_limb_2_col74 = eval.next_trace_mask();
        let op1_limb_3_col75 = eval.next_trace_mask();
        let op1_limb_4_col76 = eval.next_trace_mask();
        let op1_limb_5_col77 = eval.next_trace_mask();
        let op1_limb_6_col78 = eval.next_trace_mask();
        let op1_limb_7_col79 = eval.next_trace_mask();
        let op1_limb_8_col80 = eval.next_trace_mask();
        let op1_limb_9_col81 = eval.next_trace_mask();
        let op1_limb_10_col82 = eval.next_trace_mask();
        let op1_limb_11_col83 = eval.next_trace_mask();
        let op1_limb_12_col84 = eval.next_trace_mask();
        let op1_limb_13_col85 = eval.next_trace_mask();
        let op1_limb_14_col86 = eval.next_trace_mask();
        let op1_limb_15_col87 = eval.next_trace_mask();
        let op1_limb_16_col88 = eval.next_trace_mask();
        let op1_limb_17_col89 = eval.next_trace_mask();
        let op1_limb_18_col90 = eval.next_trace_mask();
        let op1_limb_19_col91 = eval.next_trace_mask();
        let op1_limb_20_col92 = eval.next_trace_mask();
        let op1_limb_21_col93 = eval.next_trace_mask();
        let op1_limb_22_col94 = eval.next_trace_mask();
        let op1_limb_23_col95 = eval.next_trace_mask();
        let op1_limb_24_col96 = eval.next_trace_mask();
        let op1_limb_25_col97 = eval.next_trace_mask();
        let op1_limb_26_col98 = eval.next_trace_mask();
        let op1_limb_27_col99 = eval.next_trace_mask();
        let k_col100 = eval.next_trace_mask();
        let carry_0_col101 = eval.next_trace_mask();
        let carry_1_col102 = eval.next_trace_mask();
        let carry_2_col103 = eval.next_trace_mask();
        let carry_3_col104 = eval.next_trace_mask();
        let carry_4_col105 = eval.next_trace_mask();
        let carry_5_col106 = eval.next_trace_mask();
        let carry_6_col107 = eval.next_trace_mask();
        let carry_7_col108 = eval.next_trace_mask();
        let carry_8_col109 = eval.next_trace_mask();
        let carry_9_col110 = eval.next_trace_mask();
        let carry_10_col111 = eval.next_trace_mask();
        let carry_11_col112 = eval.next_trace_mask();
        let carry_12_col113 = eval.next_trace_mask();
        let carry_13_col114 = eval.next_trace_mask();
        let carry_14_col115 = eval.next_trace_mask();
        let carry_15_col116 = eval.next_trace_mask();
        let carry_16_col117 = eval.next_trace_mask();
        let carry_17_col118 = eval.next_trace_mask();
        let carry_18_col119 = eval.next_trace_mask();
        let carry_19_col120 = eval.next_trace_mask();
        let carry_20_col121 = eval.next_trace_mask();
        let carry_21_col122 = eval.next_trace_mask();
        let carry_22_col123 = eval.next_trace_mask();
        let carry_23_col124 = eval.next_trace_mask();
        let carry_24_col125 = eval.next_trace_mask();
        let carry_25_col126 = eval.next_trace_mask();
        let carry_26_col127 = eval.next_trace_mask();

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                offset1_col4.clone(),
                offset2_col5.clone(),
                dst_base_fp_col6.clone(),
                op0_base_fp_col7.clone(),
                M31_0.clone(),
                op1_base_fp_col8.clone(),
                op1_base_ap_col9.clone(),
                M31_0.clone(),
                M31_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                ap_update_add_1_col10.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_1.clone(),
            ],
        ));

        // mem_dst_base.
        eval.add_constraint(
            (mem_dst_base_col11.clone()
                - ((dst_base_fp_col6.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - dst_base_fp_col6.clone()) * input_ap_col1.clone()))),
        );
        // mem0_base.
        eval.add_constraint(
            (mem0_base_col12.clone()
                - ((op0_base_fp_col7.clone() * input_fp_col2.clone())
                    + ((M31_1.clone() - op0_base_fp_col7.clone()) * input_ap_col1.clone()))),
        );
        // Either flag op1_base_fp is on or flag op1_base_ap is on.
        eval.add_constraint(
            ((op1_base_fp_col8.clone() + op1_base_ap_col9.clone()) - M31_1.clone()),
        );

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem_dst_base_col11.clone() + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col13.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                dst_id_col13.clone(),
                dst_limb_0_col14.clone(),
                dst_limb_1_col15.clone(),
                dst_limb_2_col16.clone(),
                dst_limb_3_col17.clone(),
                dst_limb_4_col18.clone(),
                dst_limb_5_col19.clone(),
                dst_limb_6_col20.clone(),
                dst_limb_7_col21.clone(),
                dst_limb_8_col22.clone(),
                dst_limb_9_col23.clone(),
                dst_limb_10_col24.clone(),
                dst_limb_11_col25.clone(),
                dst_limb_12_col26.clone(),
                dst_limb_13_col27.clone(),
                dst_limb_14_col28.clone(),
                dst_limb_15_col29.clone(),
                dst_limb_16_col30.clone(),
                dst_limb_17_col31.clone(),
                dst_limb_18_col32.clone(),
                dst_limb_19_col33.clone(),
                dst_limb_20_col34.clone(),
                dst_limb_21_col35.clone(),
                dst_limb_22_col36.clone(),
                dst_limb_23_col37.clone(),
                dst_limb_24_col38.clone(),
                dst_limb_25_col39.clone(),
                dst_limb_26_col40.clone(),
                dst_limb_27_col41.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (mem0_base_col12.clone() + (offset1_col4.clone() - M31_32768.clone())),
                op0_id_col42.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op0_id_col42.clone(),
                op0_limb_0_col43.clone(),
                op0_limb_1_col44.clone(),
                op0_limb_2_col45.clone(),
                op0_limb_3_col46.clone(),
                op0_limb_4_col47.clone(),
                op0_limb_5_col48.clone(),
                op0_limb_6_col49.clone(),
                op0_limb_7_col50.clone(),
                op0_limb_8_col51.clone(),
                op0_limb_9_col52.clone(),
                op0_limb_10_col53.clone(),
                op0_limb_11_col54.clone(),
                op0_limb_12_col55.clone(),
                op0_limb_13_col56.clone(),
                op0_limb_14_col57.clone(),
                op0_limb_15_col58.clone(),
                op0_limb_16_col59.clone(),
                op0_limb_17_col60.clone(),
                op0_limb_18_col61.clone(),
                op0_limb_19_col62.clone(),
                op0_limb_20_col63.clone(),
                op0_limb_21_col64.clone(),
                op0_limb_22_col65.clone(),
                op0_limb_23_col66.clone(),
                op0_limb_24_col67.clone(),
                op0_limb_25_col68.clone(),
                op0_limb_26_col69.clone(),
                op0_limb_27_col70.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (((op1_base_fp_col8.clone() * input_fp_col2.clone())
                    + (op1_base_ap_col9.clone() * input_ap_col1.clone()))
                    + (offset2_col5.clone() - M31_32768.clone())),
                op1_id_col71.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                op1_id_col71.clone(),
                op1_limb_0_col72.clone(),
                op1_limb_1_col73.clone(),
                op1_limb_2_col74.clone(),
                op1_limb_3_col75.clone(),
                op1_limb_4_col76.clone(),
                op1_limb_5_col77.clone(),
                op1_limb_6_col78.clone(),
                op1_limb_7_col79.clone(),
                op1_limb_8_col80.clone(),
                op1_limb_9_col81.clone(),
                op1_limb_10_col82.clone(),
                op1_limb_11_col83.clone(),
                op1_limb_12_col84.clone(),
                op1_limb_13_col85.clone(),
                op1_limb_14_col86.clone(),
                op1_limb_15_col87.clone(),
                op1_limb_16_col88.clone(),
                op1_limb_17_col89.clone(),
                op1_limb_18_col90.clone(),
                op1_limb_19_col91.clone(),
                op1_limb_20_col92.clone(),
                op1_limb_21_col93.clone(),
                op1_limb_22_col94.clone(),
                op1_limb_23_col95.clone(),
                op1_limb_24_col96.clone(),
                op1_limb_25_col97.clone(),
                op1_limb_26_col98.clone(),
                op1_limb_27_col99.clone(),
            ],
        ));

        // Verify Mul 252.

        let conv_tmp_5a14_17 = eval.add_intermediate(
            ((M31_0.clone() - dst_limb_0_col14.clone())
                + (op0_limb_0_col43.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_18 = eval.add_intermediate(
            (((M31_0.clone() - dst_limb_1_col15.clone())
                + (op0_limb_0_col43.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_19 = eval.add_intermediate(
            ((((M31_0.clone() - dst_limb_2_col16.clone())
                + (op0_limb_0_col43.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_20 = eval.add_intermediate(
            (((((M31_0.clone() - dst_limb_3_col17.clone())
                + (op0_limb_0_col43.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_21 = eval.add_intermediate(
            ((((((M31_0.clone() - dst_limb_4_col18.clone())
                + (op0_limb_0_col43.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_22 = eval.add_intermediate(
            (((((((M31_0.clone() - dst_limb_5_col19.clone())
                + (op0_limb_0_col43.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_23 = eval.add_intermediate(
            ((((((((M31_0.clone() - dst_limb_6_col20.clone())
                + (op0_limb_0_col43.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_24 = eval.add_intermediate(
            (((((((((M31_0.clone() - dst_limb_7_col21.clone())
                + (op0_limb_0_col43.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_25 = eval.add_intermediate(
            ((((((((((M31_0.clone() - dst_limb_8_col22.clone())
                + (op0_limb_0_col43.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_26 = eval.add_intermediate(
            (((((((((((M31_0.clone() - dst_limb_9_col23.clone())
                + (op0_limb_0_col43.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_27 = eval.add_intermediate(
            ((((((((((((M31_0.clone() - dst_limb_10_col24.clone())
                + (op0_limb_0_col43.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_28 = eval.add_intermediate(
            (((((((((((((M31_0.clone() - dst_limb_11_col25.clone())
                + (op0_limb_0_col43.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_29 = eval.add_intermediate(
            ((((((((((((((M31_0.clone() - dst_limb_12_col26.clone())
                + (op0_limb_0_col43.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_30 = eval.add_intermediate(
            (((((((((((((((M31_0.clone() - dst_limb_13_col27.clone())
                + (op0_limb_0_col43.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_31 = eval.add_intermediate(
            ((((((((((((((((M31_0.clone() - dst_limb_14_col28.clone())
                + (op0_limb_0_col43.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_32 = eval.add_intermediate(
            (((((((((((((((((M31_0.clone() - dst_limb_15_col29.clone())
                + (op0_limb_0_col43.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_33 = eval.add_intermediate(
            ((((((((((((((((((M31_0.clone() - dst_limb_16_col30.clone())
                + (op0_limb_0_col43.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_34 = eval.add_intermediate(
            (((((((((((((((((((M31_0.clone() - dst_limb_17_col31.clone())
                + (op0_limb_0_col43.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_35 = eval.add_intermediate(
            ((((((((((((((((((((M31_0.clone() - dst_limb_18_col32.clone())
                + (op0_limb_0_col43.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_36 = eval.add_intermediate(
            (((((((((((((((((((((M31_0.clone() - dst_limb_19_col33.clone())
                + (op0_limb_0_col43.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_37 = eval.add_intermediate(
            ((((((((((((((((((((((M31_0.clone() - dst_limb_20_col34.clone())
                + (op0_limb_0_col43.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_38 = eval.add_intermediate(
            (((((((((((((((((((((((M31_0.clone() - dst_limb_21_col35.clone())
                + (op0_limb_0_col43.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_39 = eval.add_intermediate(
            ((((((((((((((((((((((((M31_0.clone()
                - dst_limb_22_col36.clone())
                + (op0_limb_0_col43.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_40 = eval.add_intermediate(
            (((((((((((((((((((((((((M31_0.clone()
                - dst_limb_23_col37.clone())
                + (op0_limb_0_col43.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_41 = eval.add_intermediate(
            ((((((((((((((((((((((((((M31_0.clone()
                - dst_limb_24_col38.clone())
                + (op0_limb_0_col43.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_42 = eval.add_intermediate(
            (((((((((((((((((((((((((((M31_0.clone()
                - dst_limb_25_col39.clone())
                + (op0_limb_0_col43.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_43 = eval.add_intermediate(
            ((((((((((((((((((((((((((((M31_0.clone()
                - dst_limb_26_col40.clone())
                + (op0_limb_0_col43.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_44 = eval.add_intermediate(
            (((((((((((((((((((((((((((((M31_0.clone()
                - dst_limb_27_col41.clone())
                + (op0_limb_0_col43.clone()
                    * op1_limb_27_col99.clone()))
                + (op0_limb_1_col44.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_1_col73.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_0_col72.clone())),
        );
        let conv_tmp_5a14_45 = eval.add_intermediate(
            (((((((((((((((((((((((((((M31_0.clone()
                + (op0_limb_1_col44.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_2_col45.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_2_col74.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_1_col73.clone())),
        );
        let conv_tmp_5a14_46 = eval.add_intermediate(
            ((((((((((((((((((((((((((M31_0.clone()
                + (op0_limb_2_col45.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_3_col46.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_3_col75.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_2_col74.clone())),
        );
        let conv_tmp_5a14_47 = eval.add_intermediate(
            (((((((((((((((((((((((((M31_0.clone()
                + (op0_limb_3_col46.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_4_col47.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_4_col76.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_3_col75.clone())),
        );
        let conv_tmp_5a14_48 = eval.add_intermediate(
            ((((((((((((((((((((((((M31_0.clone()
                + (op0_limb_4_col47.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_5_col48.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_5_col77.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_4_col76.clone())),
        );
        let conv_tmp_5a14_49 = eval.add_intermediate(
            (((((((((((((((((((((((M31_0.clone()
                + (op0_limb_5_col48.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_6_col49.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_6_col78.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_5_col77.clone())),
        );
        let conv_tmp_5a14_50 = eval.add_intermediate(
            ((((((((((((((((((((((M31_0.clone()
                + (op0_limb_6_col49.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_7_col50.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_7_col79.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_6_col78.clone())),
        );
        let conv_tmp_5a14_51 = eval.add_intermediate(
            (((((((((((((((((((((M31_0.clone()
                + (op0_limb_7_col50.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_8_col51.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_8_col80.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_7_col79.clone())),
        );
        let conv_tmp_5a14_52 = eval.add_intermediate(
            ((((((((((((((((((((M31_0.clone()
                + (op0_limb_8_col51.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_9_col52.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_9_col81.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_8_col80.clone())),
        );
        let conv_tmp_5a14_53 = eval.add_intermediate(
            (((((((((((((((((((M31_0.clone()
                + (op0_limb_9_col52.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_10_col53.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_10_col82.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_9_col81.clone())),
        );
        let conv_tmp_5a14_54 = eval.add_intermediate(
            ((((((((((((((((((M31_0.clone()
                + (op0_limb_10_col53.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_11_col54.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_11_col83.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_10_col82.clone())),
        );
        let conv_tmp_5a14_55 = eval.add_intermediate(
            (((((((((((((((((M31_0.clone()
                + (op0_limb_11_col54.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_12_col55.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_12_col84.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_11_col83.clone())),
        );
        let conv_tmp_5a14_56 = eval.add_intermediate(
            ((((((((((((((((M31_0.clone()
                + (op0_limb_12_col55.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_13_col56.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_13_col85.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_12_col84.clone())),
        );
        let conv_tmp_5a14_57 = eval.add_intermediate(
            (((((((((((((((M31_0.clone()
                + (op0_limb_13_col56.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_14_col57.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_14_col86.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_13_col85.clone())),
        );
        let conv_tmp_5a14_58 = eval.add_intermediate(
            ((((((((((((((M31_0.clone()
                + (op0_limb_14_col57.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_15_col58.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_15_col87.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_14_col86.clone())),
        );
        let conv_tmp_5a14_59 = eval.add_intermediate(
            (((((((((((((M31_0.clone()
                + (op0_limb_15_col58.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_16_col59.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_16_col88.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_15_col87.clone())),
        );
        let conv_tmp_5a14_60 = eval.add_intermediate(
            ((((((((((((M31_0.clone()
                + (op0_limb_16_col59.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_17_col60.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_17_col89.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_16_col88.clone())),
        );
        let conv_tmp_5a14_61 = eval.add_intermediate(
            (((((((((((M31_0.clone()
                + (op0_limb_17_col60.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_18_col61.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_18_col90.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_17_col89.clone())),
        );
        let conv_tmp_5a14_62 = eval.add_intermediate(
            ((((((((((M31_0.clone()
                + (op0_limb_18_col61.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_19_col62.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_19_col91.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_18_col90.clone())),
        );
        let conv_tmp_5a14_63 = eval.add_intermediate(
            (((((((((M31_0.clone()
                + (op0_limb_19_col62.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_20_col63.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_20_col92.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_19_col91.clone())),
        );
        let conv_tmp_5a14_64 = eval.add_intermediate(
            ((((((((M31_0.clone() + (op0_limb_20_col63.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_21_col64.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_21_col93.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_20_col92.clone())),
        );
        let conv_tmp_5a14_65 = eval.add_intermediate(
            (((((((M31_0.clone() + (op0_limb_21_col64.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_22_col65.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_22_col94.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_21_col93.clone())),
        );
        let conv_tmp_5a14_66 = eval.add_intermediate(
            ((((((M31_0.clone() + (op0_limb_22_col65.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_23_col66.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_23_col95.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_22_col94.clone())),
        );
        let conv_tmp_5a14_67 = eval.add_intermediate(
            (((((M31_0.clone() + (op0_limb_23_col66.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_24_col67.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_24_col96.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_23_col95.clone())),
        );
        let conv_tmp_5a14_68 = eval.add_intermediate(
            ((((M31_0.clone() + (op0_limb_24_col67.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_25_col68.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_25_col97.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_24_col96.clone())),
        );
        let conv_tmp_5a14_69 = eval.add_intermediate(
            (((M31_0.clone() + (op0_limb_25_col68.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_26_col69.clone() * op1_limb_26_col98.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_25_col97.clone())),
        );
        let conv_tmp_5a14_70 = eval.add_intermediate(
            ((M31_0.clone() + (op0_limb_26_col69.clone() * op1_limb_27_col99.clone()))
                + (op0_limb_27_col70.clone() * op1_limb_26_col98.clone())),
        );
        let conv_tmp_5a14_71 = eval.add_intermediate(
            (M31_0.clone() + (op0_limb_27_col70.clone() * op1_limb_27_col99.clone())),
        );
        let conv_mod_tmp_5a14_72 = eval.add_intermediate(
            (((M31_0.clone() + (M31_32.clone() * conv_tmp_5a14_17.clone()))
                - (M31_4.clone() * conv_tmp_5a14_38.clone()))
                + (M31_8.clone() * conv_tmp_5a14_66.clone())),
        );
        let conv_mod_tmp_5a14_73 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_1.clone() * conv_tmp_5a14_17.clone()))
                + (M31_32.clone() * conv_tmp_5a14_18.clone()))
                - (M31_4.clone() * conv_tmp_5a14_39.clone()))
                + (M31_8.clone() * conv_tmp_5a14_67.clone())),
        );
        let conv_mod_tmp_5a14_74 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_1.clone() * conv_tmp_5a14_18.clone()))
                + (M31_32.clone() * conv_tmp_5a14_19.clone()))
                - (M31_4.clone() * conv_tmp_5a14_40.clone()))
                + (M31_8.clone() * conv_tmp_5a14_68.clone())),
        );
        let conv_mod_tmp_5a14_75 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_1.clone() * conv_tmp_5a14_19.clone()))
                + (M31_32.clone() * conv_tmp_5a14_20.clone()))
                - (M31_4.clone() * conv_tmp_5a14_41.clone()))
                + (M31_8.clone() * conv_tmp_5a14_69.clone())),
        );
        let conv_mod_tmp_5a14_76 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_1.clone() * conv_tmp_5a14_20.clone()))
                + (M31_32.clone() * conv_tmp_5a14_21.clone()))
                - (M31_4.clone() * conv_tmp_5a14_42.clone()))
                + (M31_8.clone() * conv_tmp_5a14_70.clone())),
        );
        let conv_mod_tmp_5a14_77 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_1.clone() * conv_tmp_5a14_21.clone()))
                + (M31_32.clone() * conv_tmp_5a14_22.clone()))
                - (M31_4.clone() * conv_tmp_5a14_43.clone()))
                + (M31_8.clone() * conv_tmp_5a14_71.clone())),
        );
        let conv_mod_tmp_5a14_78 = eval.add_intermediate(
            (((M31_0.clone() + (M31_1.clone() * conv_tmp_5a14_22.clone()))
                + (M31_32.clone() * conv_tmp_5a14_23.clone()))
                - (M31_4.clone() * conv_tmp_5a14_44.clone())),
        );
        let conv_mod_tmp_5a14_79 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_17.clone()))
                + (M31_1.clone() * conv_tmp_5a14_23.clone()))
                + (M31_32.clone() * conv_tmp_5a14_24.clone()))
                - (M31_4.clone() * conv_tmp_5a14_45.clone())),
        );
        let conv_mod_tmp_5a14_80 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_18.clone()))
                + (M31_1.clone() * conv_tmp_5a14_24.clone()))
                + (M31_32.clone() * conv_tmp_5a14_25.clone()))
                - (M31_4.clone() * conv_tmp_5a14_46.clone())),
        );
        let conv_mod_tmp_5a14_81 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_19.clone()))
                + (M31_1.clone() * conv_tmp_5a14_25.clone()))
                + (M31_32.clone() * conv_tmp_5a14_26.clone()))
                - (M31_4.clone() * conv_tmp_5a14_47.clone())),
        );
        let conv_mod_tmp_5a14_82 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_20.clone()))
                + (M31_1.clone() * conv_tmp_5a14_26.clone()))
                + (M31_32.clone() * conv_tmp_5a14_27.clone()))
                - (M31_4.clone() * conv_tmp_5a14_48.clone())),
        );
        let conv_mod_tmp_5a14_83 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_21.clone()))
                + (M31_1.clone() * conv_tmp_5a14_27.clone()))
                + (M31_32.clone() * conv_tmp_5a14_28.clone()))
                - (M31_4.clone() * conv_tmp_5a14_49.clone())),
        );
        let conv_mod_tmp_5a14_84 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_22.clone()))
                + (M31_1.clone() * conv_tmp_5a14_28.clone()))
                + (M31_32.clone() * conv_tmp_5a14_29.clone()))
                - (M31_4.clone() * conv_tmp_5a14_50.clone())),
        );
        let conv_mod_tmp_5a14_85 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_23.clone()))
                + (M31_1.clone() * conv_tmp_5a14_29.clone()))
                + (M31_32.clone() * conv_tmp_5a14_30.clone()))
                - (M31_4.clone() * conv_tmp_5a14_51.clone())),
        );
        let conv_mod_tmp_5a14_86 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_24.clone()))
                + (M31_1.clone() * conv_tmp_5a14_30.clone()))
                + (M31_32.clone() * conv_tmp_5a14_31.clone()))
                - (M31_4.clone() * conv_tmp_5a14_52.clone())),
        );
        let conv_mod_tmp_5a14_87 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_25.clone()))
                + (M31_1.clone() * conv_tmp_5a14_31.clone()))
                + (M31_32.clone() * conv_tmp_5a14_32.clone()))
                - (M31_4.clone() * conv_tmp_5a14_53.clone())),
        );
        let conv_mod_tmp_5a14_88 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_26.clone()))
                + (M31_1.clone() * conv_tmp_5a14_32.clone()))
                + (M31_32.clone() * conv_tmp_5a14_33.clone()))
                - (M31_4.clone() * conv_tmp_5a14_54.clone())),
        );
        let conv_mod_tmp_5a14_89 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_27.clone()))
                + (M31_1.clone() * conv_tmp_5a14_33.clone()))
                + (M31_32.clone() * conv_tmp_5a14_34.clone()))
                - (M31_4.clone() * conv_tmp_5a14_55.clone())),
        );
        let conv_mod_tmp_5a14_90 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_28.clone()))
                + (M31_1.clone() * conv_tmp_5a14_34.clone()))
                + (M31_32.clone() * conv_tmp_5a14_35.clone()))
                - (M31_4.clone() * conv_tmp_5a14_56.clone())),
        );
        let conv_mod_tmp_5a14_91 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_29.clone()))
                + (M31_1.clone() * conv_tmp_5a14_35.clone()))
                + (M31_32.clone() * conv_tmp_5a14_36.clone()))
                - (M31_4.clone() * conv_tmp_5a14_57.clone())),
        );
        let conv_mod_tmp_5a14_92 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_30.clone()))
                + (M31_1.clone() * conv_tmp_5a14_36.clone()))
                + (M31_32.clone() * conv_tmp_5a14_37.clone()))
                - (M31_4.clone() * conv_tmp_5a14_58.clone())),
        );
        let conv_mod_tmp_5a14_93 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_31.clone()))
                + (M31_1.clone() * conv_tmp_5a14_37.clone()))
                - (M31_4.clone() * conv_tmp_5a14_59.clone()))
                + (M31_64.clone() * conv_tmp_5a14_66.clone())),
        );
        let conv_mod_tmp_5a14_94 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_32.clone()))
                - (M31_4.clone() * conv_tmp_5a14_60.clone()))
                + (M31_2.clone() * conv_tmp_5a14_66.clone()))
                + (M31_64.clone() * conv_tmp_5a14_67.clone())),
        );
        let conv_mod_tmp_5a14_95 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_33.clone()))
                - (M31_4.clone() * conv_tmp_5a14_61.clone()))
                + (M31_2.clone() * conv_tmp_5a14_67.clone()))
                + (M31_64.clone() * conv_tmp_5a14_68.clone())),
        );
        let conv_mod_tmp_5a14_96 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_34.clone()))
                - (M31_4.clone() * conv_tmp_5a14_62.clone()))
                + (M31_2.clone() * conv_tmp_5a14_68.clone()))
                + (M31_64.clone() * conv_tmp_5a14_69.clone())),
        );
        let conv_mod_tmp_5a14_97 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_35.clone()))
                - (M31_4.clone() * conv_tmp_5a14_63.clone()))
                + (M31_2.clone() * conv_tmp_5a14_69.clone()))
                + (M31_64.clone() * conv_tmp_5a14_70.clone())),
        );
        let conv_mod_tmp_5a14_98 = eval.add_intermediate(
            ((((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_36.clone()))
                - (M31_4.clone() * conv_tmp_5a14_64.clone()))
                + (M31_2.clone() * conv_tmp_5a14_70.clone()))
                + (M31_64.clone() * conv_tmp_5a14_71.clone())),
        );
        let conv_mod_tmp_5a14_99 = eval.add_intermediate(
            (((M31_0.clone() + (M31_2.clone() * conv_tmp_5a14_37.clone()))
                - (M31_4.clone() * conv_tmp_5a14_65.clone()))
                + (M31_2.clone() * conv_tmp_5a14_71.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(k_col100.clone() + M31_262144.clone())],
        ));

        eval.add_constraint(
            ((carry_0_col101.clone() * M31_512.clone())
                - ((conv_mod_tmp_5a14_72.clone() - (M31_1.clone() * k_col100.clone()))
                    + M31_0.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_0_col101.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_1_col102.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_73.clone() + carry_0_col101.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_1_col102.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_2_col103.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_74.clone() + carry_1_col102.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_2_col103.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_3_col104.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_75.clone() + carry_2_col103.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_3_col104.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_4_col105.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_76.clone() + carry_3_col104.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_4_col105.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_5_col106.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_77.clone() + carry_4_col105.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_5_col106.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_6_col107.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_78.clone() + carry_5_col106.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_6_col107.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_7_col108.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_79.clone() + carry_6_col107.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_7_col108.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_8_col109.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_80.clone() + carry_7_col108.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_8_col109.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_9_col110.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_81.clone() + carry_8_col109.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_9_col110.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_10_col111.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_82.clone() + carry_9_col110.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_10_col111.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_11_col112.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_83.clone() + carry_10_col111.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_11_col112.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_12_col113.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_84.clone() + carry_11_col112.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_12_col113.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_13_col114.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_85.clone() + carry_12_col113.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_13_col114.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_14_col115.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_86.clone() + carry_13_col114.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_14_col115.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_15_col116.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_87.clone() + carry_14_col115.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_15_col116.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_16_col117.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_88.clone() + carry_15_col116.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_16_col117.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_17_col118.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_89.clone() + carry_16_col117.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_17_col118.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_18_col119.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_90.clone() + carry_17_col118.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_18_col119.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_19_col120.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_91.clone() + carry_18_col119.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_19_col120.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_20_col121.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_92.clone() + carry_19_col120.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_20_col121.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_21_col122.clone() * M31_512.clone())
                - ((conv_mod_tmp_5a14_93.clone() - (M31_136.clone() * k_col100.clone()))
                    + carry_20_col121.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_21_col122.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_22_col123.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_94.clone() + carry_21_col122.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_22_col123.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_23_col124.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_95.clone() + carry_22_col123.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_23_col124.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_24_col125.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_96.clone() + carry_23_col124.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_24_col125.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_25_col126.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_97.clone() + carry_24_col125.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_25_col126.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((carry_26_col127.clone() * M31_512.clone())
                - (conv_mod_tmp_5a14_98.clone() + carry_25_col126.clone())),
        );
        eval.add_to_relation(RelationEntry::new(
            &self.range_check_19_lookup_elements,
            E::EF::one(),
            &[(carry_26_col127.clone() + M31_131072.clone())],
        ));

        eval.add_constraint(
            ((conv_mod_tmp_5a14_99.clone() - (M31_256.clone() * k_col100.clone()))
                + carry_26_col127.clone()),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                (input_ap_col1.clone() + ap_update_add_1_col10.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

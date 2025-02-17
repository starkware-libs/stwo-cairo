#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::backend::simd::m31::LOG_N_LANES;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;

use crate::cairo_air::relations;

pub struct Eval {
    pub claim: Claim,
    pub memory_address_to_id_lookup_elements: relations::MemoryAddressToId,
    pub memory_id_to_big_lookup_elements: relations::MemoryIdToBig,
    pub opcodes_lookup_elements: relations::Opcodes,
    pub verify_instruction_lookup_elements: relations::VerifyInstruction,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub n_rows: usize,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let trace_log_sizes = vec![log_size; 43];
        let interaction_log_sizes = vec![log_size; SECURE_EXTENSION_DEGREE * 4];
        let preprocessed_log_sizes = vec![log_size];
        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.n_rows as u64);
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
        std::cmp::max(self.claim.n_rows.next_power_of_two().ilog2(), LOG_N_LANES)
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
        let M31_134217728 = E::F::from(M31::from(134217728));
        let M31_136 = E::F::from(M31::from(136));
        let M31_256 = E::F::from(M31::from(256));
        let M31_262144 = E::F::from(M31::from(262144));
        let M31_32767 = E::F::from(M31::from(32767));
        let M31_32768 = E::F::from(M31::from(32768));
        let M31_32769 = E::F::from(M31::from(32769));
        let M31_511 = E::F::from(M31::from(511));
        let M31_512 = E::F::from(M31::from(512));
        let input_pc_col0 = eval.next_trace_mask();
        let input_ap_col1 = eval.next_trace_mask();
        let input_fp_col2 = eval.next_trace_mask();
        let offset0_col3 = eval.next_trace_mask();
        let ap_update_add_1_col4 = eval.next_trace_mask();
        let dst_id_col5 = eval.next_trace_mask();
        let dst_limb_0_col6 = eval.next_trace_mask();
        let dst_limb_1_col7 = eval.next_trace_mask();
        let dst_limb_2_col8 = eval.next_trace_mask();
        let dst_limb_3_col9 = eval.next_trace_mask();
        let dst_limb_4_col10 = eval.next_trace_mask();
        let dst_limb_5_col11 = eval.next_trace_mask();
        let dst_limb_6_col12 = eval.next_trace_mask();
        let dst_limb_7_col13 = eval.next_trace_mask();
        let dst_limb_8_col14 = eval.next_trace_mask();
        let dst_limb_9_col15 = eval.next_trace_mask();
        let dst_limb_10_col16 = eval.next_trace_mask();
        let dst_limb_11_col17 = eval.next_trace_mask();
        let dst_limb_12_col18 = eval.next_trace_mask();
        let dst_limb_13_col19 = eval.next_trace_mask();
        let dst_limb_14_col20 = eval.next_trace_mask();
        let dst_limb_15_col21 = eval.next_trace_mask();
        let dst_limb_16_col22 = eval.next_trace_mask();
        let dst_limb_17_col23 = eval.next_trace_mask();
        let dst_limb_18_col24 = eval.next_trace_mask();
        let dst_limb_19_col25 = eval.next_trace_mask();
        let dst_limb_20_col26 = eval.next_trace_mask();
        let dst_limb_21_col27 = eval.next_trace_mask();
        let dst_limb_22_col28 = eval.next_trace_mask();
        let dst_limb_23_col29 = eval.next_trace_mask();
        let dst_limb_24_col30 = eval.next_trace_mask();
        let dst_limb_25_col31 = eval.next_trace_mask();
        let dst_limb_26_col32 = eval.next_trace_mask();
        let dst_limb_27_col33 = eval.next_trace_mask();
        let res_col34 = eval.next_trace_mask();
        let res_squares_col35 = eval.next_trace_mask();
        let next_pc_id_col36 = eval.next_trace_mask();
        let msb_col37 = eval.next_trace_mask();
        let mid_limbs_set_col38 = eval.next_trace_mask();
        let next_pc_limb_0_col39 = eval.next_trace_mask();
        let next_pc_limb_1_col40 = eval.next_trace_mask();
        let next_pc_limb_2_col41 = eval.next_trace_mask();
        let padding = eval.next_trace_mask();

        eval.add_constraint(padding.clone() * padding.clone() - padding.clone());

        // Decode Instruction.

        eval.add_to_relation(RelationEntry::new(
            &self.verify_instruction_lookup_elements,
            E::EF::one(),
            &[
                input_pc_col0.clone(),
                offset0_col3.clone(),
                M31_32767.clone(),
                M31_32769.clone(),
                M31_0.clone(),
                M31_1.clone(),
                M31_1.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_1.clone(),
                M31_0.clone(),
                ap_update_add_1_col4.clone(),
            ],
        ));

        // Read Positive Num Bits 252.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_ap_col1.clone() + (offset0_col3.clone() - M31_32768.clone())),
                dst_id_col5.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                dst_id_col5.clone(),
                dst_limb_0_col6.clone(),
                dst_limb_1_col7.clone(),
                dst_limb_2_col8.clone(),
                dst_limb_3_col9.clone(),
                dst_limb_4_col10.clone(),
                dst_limb_5_col11.clone(),
                dst_limb_6_col12.clone(),
                dst_limb_7_col13.clone(),
                dst_limb_8_col14.clone(),
                dst_limb_9_col15.clone(),
                dst_limb_10_col16.clone(),
                dst_limb_11_col17.clone(),
                dst_limb_12_col18.clone(),
                dst_limb_13_col19.clone(),
                dst_limb_14_col20.clone(),
                dst_limb_15_col21.clone(),
                dst_limb_16_col22.clone(),
                dst_limb_17_col23.clone(),
                dst_limb_18_col24.clone(),
                dst_limb_19_col25.clone(),
                dst_limb_20_col26.clone(),
                dst_limb_21_col27.clone(),
                dst_limb_22_col28.clone(),
                dst_limb_23_col29.clone(),
                dst_limb_24_col30.clone(),
                dst_limb_25_col31.clone(),
                dst_limb_26_col32.clone(),
                dst_limb_27_col33.clone(),
            ],
        ));

        // dst doesn't equal 0.
        eval.add_constraint(
            ((((((((((((((((((((((((((((((M31_0.clone()
                + dst_limb_0_col6.clone())
                + dst_limb_1_col7.clone())
                + dst_limb_2_col8.clone())
                + dst_limb_3_col9.clone())
                + dst_limb_4_col10.clone())
                + dst_limb_5_col11.clone())
                + dst_limb_6_col12.clone())
                + dst_limb_7_col13.clone())
                + dst_limb_8_col14.clone())
                + dst_limb_9_col15.clone())
                + dst_limb_10_col16.clone())
                + dst_limb_11_col17.clone())
                + dst_limb_12_col18.clone())
                + dst_limb_13_col19.clone())
                + dst_limb_14_col20.clone())
                + dst_limb_15_col21.clone())
                + dst_limb_16_col22.clone())
                + dst_limb_17_col23.clone())
                + dst_limb_18_col24.clone())
                + dst_limb_19_col25.clone())
                + dst_limb_20_col26.clone())
                + dst_limb_21_col27.clone())
                + dst_limb_22_col28.clone())
                + dst_limb_23_col29.clone())
                + dst_limb_24_col30.clone())
                + dst_limb_25_col31.clone())
                + dst_limb_26_col32.clone())
                + dst_limb_27_col33.clone())
                * res_col34.clone())
                - M31_1.clone()),
        );
        let diff_from_p_tmp_65b05_7 =
            eval.add_intermediate((dst_limb_0_col6.clone() - M31_1.clone()));
        let diff_from_p_tmp_65b05_8 =
            eval.add_intermediate((dst_limb_21_col27.clone() - M31_136.clone()));
        let diff_from_p_tmp_65b05_9 =
            eval.add_intermediate((dst_limb_27_col33.clone() - M31_256.clone()));
        // dst doesn't equal P.
        eval.add_constraint(
            ((((((((((((((((((((((((((((((M31_0.clone()
                + (diff_from_p_tmp_65b05_7.clone()
                    * diff_from_p_tmp_65b05_7.clone()))
                + dst_limb_1_col7.clone())
                + dst_limb_2_col8.clone())
                + dst_limb_3_col9.clone())
                + dst_limb_4_col10.clone())
                + dst_limb_5_col11.clone())
                + dst_limb_6_col12.clone())
                + dst_limb_7_col13.clone())
                + dst_limb_8_col14.clone())
                + dst_limb_9_col15.clone())
                + dst_limb_10_col16.clone())
                + dst_limb_11_col17.clone())
                + dst_limb_12_col18.clone())
                + dst_limb_13_col19.clone())
                + dst_limb_14_col20.clone())
                + dst_limb_15_col21.clone())
                + dst_limb_16_col22.clone())
                + dst_limb_17_col23.clone())
                + dst_limb_18_col24.clone())
                + dst_limb_19_col25.clone())
                + dst_limb_20_col26.clone())
                + (diff_from_p_tmp_65b05_8.clone() * diff_from_p_tmp_65b05_8.clone()))
                + dst_limb_22_col28.clone())
                + dst_limb_23_col29.clone())
                + dst_limb_24_col30.clone())
                + dst_limb_25_col31.clone())
                + dst_limb_26_col32.clone())
                + (diff_from_p_tmp_65b05_9.clone() * diff_from_p_tmp_65b05_9.clone()))
                * res_squares_col35.clone())
                - M31_1.clone()),
        );

        // Read Small.

        eval.add_to_relation(RelationEntry::new(
            &self.memory_address_to_id_lookup_elements,
            E::EF::one(),
            &[
                (input_pc_col0.clone() + M31_1.clone()),
                next_pc_id_col36.clone(),
            ],
        ));

        // Cond Decode Small Sign.

        // msb is a bit.
        eval.add_constraint((msb_col37.clone() * (msb_col37.clone() - M31_1.clone())));
        // mid_limbs_set is a bit.
        eval.add_constraint(
            (mid_limbs_set_col38.clone() * (mid_limbs_set_col38.clone() - M31_1.clone())),
        );
        // Cannot have msb equals 0 and mid_limbs_set equals 1.
        eval.add_constraint(
            ((M31_1.clone() * mid_limbs_set_col38.clone()) * (msb_col37.clone() - M31_1.clone())),
        );

        eval.add_to_relation(RelationEntry::new(
            &self.memory_id_to_big_lookup_elements,
            E::EF::one(),
            &[
                next_pc_id_col36.clone(),
                next_pc_limb_0_col39.clone(),
                next_pc_limb_1_col40.clone(),
                next_pc_limb_2_col41.clone(),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                (mid_limbs_set_col38.clone() * M31_511.clone()),
                ((M31_136.clone() * msb_col37.clone()) - mid_limbs_set_col38.clone()),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                M31_0.clone(),
                (msb_col37.clone() * M31_256.clone()),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            E::EF::from(padding.clone()),
            &[
                input_pc_col0.clone(),
                input_ap_col1.clone(),
                input_fp_col2.clone(),
            ],
        ));

        eval.add_to_relation(RelationEntry::new(
            &self.opcodes_lookup_elements,
            -E::EF::from(padding.clone()),
            &[
                (input_pc_col0.clone()
                    + ((((next_pc_limb_0_col39.clone()
                        + (next_pc_limb_1_col40.clone() * M31_512.clone()))
                        + (next_pc_limb_2_col41.clone() * M31_262144.clone()))
                        - msb_col37.clone())
                        - (M31_134217728.clone() * mid_limbs_set_col38.clone()))),
                (input_ap_col1.clone() + ap_update_add_1_col4.clone()),
                input_fp_col2.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

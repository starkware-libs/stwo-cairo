#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use air_structs_derive::SubComponentInputs;
use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col, Column};
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;

use super::component::{Claim, InteractionClaim};
use crate::components::{
    memory_address_to_id, memory_id_to_big, pack_values, range_check_19, verify_instruction,
};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 129;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_19_state: &range_check_19::ClaimGenerator,
        verify_instruction_state: &mut verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_calls = self.inputs.len();
        assert_ne!(n_calls, 0);
        let size = std::cmp::max(n_calls.next_power_of_two(), N_LANES);
        let need_padding = n_calls != size;

        if need_padding {
            self.inputs.resize(size, *self.inputs.first().unwrap());
            bit_reverse_coset_to_circle_domain_order(&mut self.inputs);
        }

        let packed_inputs = pack_values(&self.inputs);
        let (trace, mut sub_components_inputs, lookup_data) = write_trace_simd(
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs
            .memory_address_to_id_inputs
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .memory_id_to_big_inputs
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .range_check_19_inputs
            .iter()
            .for_each(|inputs| {
                range_check_19_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .verify_instruction_inputs
            .iter()
            .for_each(|inputs| {
                verify_instruction_state.add_inputs(&inputs[..n_calls]);
            });

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { n_calls },
            InteractionClaimGenerator {
                n_calls,
                lookup_data,
            },
        )
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        self.inputs.extend(inputs);
    }
}

#[derive(SubComponentInputs, Uninitialized, IterMut, ParIterMut)]
pub struct SubComponentInputs {
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 3],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 3],
    pub range_check_19_inputs: [Vec<range_check_19::InputType>; 28],
    pub verify_instruction_inputs: [Vec<verify_instruction::InputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    SubComponentInputs,
    LookupData,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_components_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_size),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    trace
        .par_iter_mut()
        .zip(inputs.par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(sub_components_inputs.par_iter_mut().chunks(N_LANES))
        .for_each(
            |(
                ((row, mul_opcode_is_small_f_is_imm_f_input), lookup_data),
                mut sub_components_inputs,
            )| {
                let input_tmp_5a147_0 = mul_opcode_is_small_f_is_imm_f_input;
                let input_pc_col0 = input_tmp_5a147_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_5a147_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_5a147_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_5a147_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_5a147_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a147_1);
                let offset0_tmp_5a147_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_5a147_3.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_5a147_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_5a147_4.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_5a147_5 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_5a147_5.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_5a147_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_5a147_6.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_5a147_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_5a147_7.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_base_fp_tmp_5a147_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col8 = op1_base_fp_tmp_5a147_8.as_m31();
                *row[8] = op1_base_fp_col8;
                let op1_base_ap_tmp_5a147_9 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_4))
                        & (UInt16_1));
                let op1_base_ap_col9 = op1_base_ap_tmp_5a147_9.as_m31();
                *row[9] = op1_base_ap_col9;
                let ap_update_add_1_tmp_5a147_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_5a147_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_5a147_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_5a147_10.as_m31();
                *row[10] = ap_update_add_1_col10;

                for (i, &input) in (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        dst_base_fp_col6,
                        op0_base_fp_col7,
                        M31_0,
                        op1_base_fp_col8,
                        op1_base_ap_col9,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col10,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                )
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].verify_instruction_inputs[0] = input;
                }
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    dst_base_fp_col6,
                    op0_base_fp_col7,
                    M31_0,
                    op1_base_fp_col8,
                    op1_base_ap_col9,
                    M31_0,
                    M31_1,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ap_update_add_1_col10,
                    M31_0,
                    M31_0,
                    M31_1,
                ];

                let mem_dst_base_col11 = (((dst_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
                *row[11] = mem_dst_base_col11;
                let mem0_base_col12 = (((op0_base_fp_col7) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
                *row[12] = mem0_base_col12;
                let mem1_base_col13 = (((op1_base_fp_col8) * (input_fp_col2))
                    + ((op1_base_ap_col9) * (input_ap_col1)));
                *row[13] = mem1_base_col13;

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_5a147_11 = memory_address_to_id_state
                    .deduce_output(((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_5a147_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a147_11);
                let dst_id_col14 = memory_address_to_id_value_tmp_5a147_11;
                *row[14] = dst_id_col14;
                for (i, &input) in ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768)))
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].memory_address_to_id_inputs[0] = input;
                }
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))),
                    dst_id_col14,
                ];
                let dst_limb_0_col15 = memory_id_to_big_value_tmp_5a147_12.get_m31(0);
                *row[15] = dst_limb_0_col15;
                let dst_limb_1_col16 = memory_id_to_big_value_tmp_5a147_12.get_m31(1);
                *row[16] = dst_limb_1_col16;
                let dst_limb_2_col17 = memory_id_to_big_value_tmp_5a147_12.get_m31(2);
                *row[17] = dst_limb_2_col17;
                let dst_limb_3_col18 = memory_id_to_big_value_tmp_5a147_12.get_m31(3);
                *row[18] = dst_limb_3_col18;
                let dst_limb_4_col19 = memory_id_to_big_value_tmp_5a147_12.get_m31(4);
                *row[19] = dst_limb_4_col19;
                let dst_limb_5_col20 = memory_id_to_big_value_tmp_5a147_12.get_m31(5);
                *row[20] = dst_limb_5_col20;
                let dst_limb_6_col21 = memory_id_to_big_value_tmp_5a147_12.get_m31(6);
                *row[21] = dst_limb_6_col21;
                let dst_limb_7_col22 = memory_id_to_big_value_tmp_5a147_12.get_m31(7);
                *row[22] = dst_limb_7_col22;
                let dst_limb_8_col23 = memory_id_to_big_value_tmp_5a147_12.get_m31(8);
                *row[23] = dst_limb_8_col23;
                let dst_limb_9_col24 = memory_id_to_big_value_tmp_5a147_12.get_m31(9);
                *row[24] = dst_limb_9_col24;
                let dst_limb_10_col25 = memory_id_to_big_value_tmp_5a147_12.get_m31(10);
                *row[25] = dst_limb_10_col25;
                let dst_limb_11_col26 = memory_id_to_big_value_tmp_5a147_12.get_m31(11);
                *row[26] = dst_limb_11_col26;
                let dst_limb_12_col27 = memory_id_to_big_value_tmp_5a147_12.get_m31(12);
                *row[27] = dst_limb_12_col27;
                let dst_limb_13_col28 = memory_id_to_big_value_tmp_5a147_12.get_m31(13);
                *row[28] = dst_limb_13_col28;
                let dst_limb_14_col29 = memory_id_to_big_value_tmp_5a147_12.get_m31(14);
                *row[29] = dst_limb_14_col29;
                let dst_limb_15_col30 = memory_id_to_big_value_tmp_5a147_12.get_m31(15);
                *row[30] = dst_limb_15_col30;
                let dst_limb_16_col31 = memory_id_to_big_value_tmp_5a147_12.get_m31(16);
                *row[31] = dst_limb_16_col31;
                let dst_limb_17_col32 = memory_id_to_big_value_tmp_5a147_12.get_m31(17);
                *row[32] = dst_limb_17_col32;
                let dst_limb_18_col33 = memory_id_to_big_value_tmp_5a147_12.get_m31(18);
                *row[33] = dst_limb_18_col33;
                let dst_limb_19_col34 = memory_id_to_big_value_tmp_5a147_12.get_m31(19);
                *row[34] = dst_limb_19_col34;
                let dst_limb_20_col35 = memory_id_to_big_value_tmp_5a147_12.get_m31(20);
                *row[35] = dst_limb_20_col35;
                let dst_limb_21_col36 = memory_id_to_big_value_tmp_5a147_12.get_m31(21);
                *row[36] = dst_limb_21_col36;
                let dst_limb_22_col37 = memory_id_to_big_value_tmp_5a147_12.get_m31(22);
                *row[37] = dst_limb_22_col37;
                let dst_limb_23_col38 = memory_id_to_big_value_tmp_5a147_12.get_m31(23);
                *row[38] = dst_limb_23_col38;
                let dst_limb_24_col39 = memory_id_to_big_value_tmp_5a147_12.get_m31(24);
                *row[39] = dst_limb_24_col39;
                let dst_limb_25_col40 = memory_id_to_big_value_tmp_5a147_12.get_m31(25);
                *row[40] = dst_limb_25_col40;
                let dst_limb_26_col41 = memory_id_to_big_value_tmp_5a147_12.get_m31(26);
                *row[41] = dst_limb_26_col41;
                let dst_limb_27_col42 = memory_id_to_big_value_tmp_5a147_12.get_m31(27);
                *row[42] = dst_limb_27_col42;
                for (i, &input) in dst_id_col14.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[0] = input;
                }
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col14,
                    dst_limb_0_col15,
                    dst_limb_1_col16,
                    dst_limb_2_col17,
                    dst_limb_3_col18,
                    dst_limb_4_col19,
                    dst_limb_5_col20,
                    dst_limb_6_col21,
                    dst_limb_7_col22,
                    dst_limb_8_col23,
                    dst_limb_9_col24,
                    dst_limb_10_col25,
                    dst_limb_11_col26,
                    dst_limb_12_col27,
                    dst_limb_13_col28,
                    dst_limb_14_col29,
                    dst_limb_15_col30,
                    dst_limb_16_col31,
                    dst_limb_17_col32,
                    dst_limb_18_col33,
                    dst_limb_19_col34,
                    dst_limb_20_col35,
                    dst_limb_21_col36,
                    dst_limb_22_col37,
                    dst_limb_23_col38,
                    dst_limb_24_col39,
                    dst_limb_25_col40,
                    dst_limb_26_col41,
                    dst_limb_27_col42,
                ];

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_5a147_13 = memory_address_to_id_state
                    .deduce_output(((mem0_base_col12) + ((offset1_col4) - (M31_32768))));
                let memory_id_to_big_value_tmp_5a147_14 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a147_13);
                let op0_id_col43 = memory_address_to_id_value_tmp_5a147_13;
                *row[43] = op0_id_col43;
                for (i, &input) in ((mem0_base_col12) + ((offset1_col4) - (M31_32768)))
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].memory_address_to_id_inputs[1] = input;
                }
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col12) + ((offset1_col4) - (M31_32768))),
                    op0_id_col43,
                ];
                let op0_limb_0_col44 = memory_id_to_big_value_tmp_5a147_14.get_m31(0);
                *row[44] = op0_limb_0_col44;
                let op0_limb_1_col45 = memory_id_to_big_value_tmp_5a147_14.get_m31(1);
                *row[45] = op0_limb_1_col45;
                let op0_limb_2_col46 = memory_id_to_big_value_tmp_5a147_14.get_m31(2);
                *row[46] = op0_limb_2_col46;
                let op0_limb_3_col47 = memory_id_to_big_value_tmp_5a147_14.get_m31(3);
                *row[47] = op0_limb_3_col47;
                let op0_limb_4_col48 = memory_id_to_big_value_tmp_5a147_14.get_m31(4);
                *row[48] = op0_limb_4_col48;
                let op0_limb_5_col49 = memory_id_to_big_value_tmp_5a147_14.get_m31(5);
                *row[49] = op0_limb_5_col49;
                let op0_limb_6_col50 = memory_id_to_big_value_tmp_5a147_14.get_m31(6);
                *row[50] = op0_limb_6_col50;
                let op0_limb_7_col51 = memory_id_to_big_value_tmp_5a147_14.get_m31(7);
                *row[51] = op0_limb_7_col51;
                let op0_limb_8_col52 = memory_id_to_big_value_tmp_5a147_14.get_m31(8);
                *row[52] = op0_limb_8_col52;
                let op0_limb_9_col53 = memory_id_to_big_value_tmp_5a147_14.get_m31(9);
                *row[53] = op0_limb_9_col53;
                let op0_limb_10_col54 = memory_id_to_big_value_tmp_5a147_14.get_m31(10);
                *row[54] = op0_limb_10_col54;
                let op0_limb_11_col55 = memory_id_to_big_value_tmp_5a147_14.get_m31(11);
                *row[55] = op0_limb_11_col55;
                let op0_limb_12_col56 = memory_id_to_big_value_tmp_5a147_14.get_m31(12);
                *row[56] = op0_limb_12_col56;
                let op0_limb_13_col57 = memory_id_to_big_value_tmp_5a147_14.get_m31(13);
                *row[57] = op0_limb_13_col57;
                let op0_limb_14_col58 = memory_id_to_big_value_tmp_5a147_14.get_m31(14);
                *row[58] = op0_limb_14_col58;
                let op0_limb_15_col59 = memory_id_to_big_value_tmp_5a147_14.get_m31(15);
                *row[59] = op0_limb_15_col59;
                let op0_limb_16_col60 = memory_id_to_big_value_tmp_5a147_14.get_m31(16);
                *row[60] = op0_limb_16_col60;
                let op0_limb_17_col61 = memory_id_to_big_value_tmp_5a147_14.get_m31(17);
                *row[61] = op0_limb_17_col61;
                let op0_limb_18_col62 = memory_id_to_big_value_tmp_5a147_14.get_m31(18);
                *row[62] = op0_limb_18_col62;
                let op0_limb_19_col63 = memory_id_to_big_value_tmp_5a147_14.get_m31(19);
                *row[63] = op0_limb_19_col63;
                let op0_limb_20_col64 = memory_id_to_big_value_tmp_5a147_14.get_m31(20);
                *row[64] = op0_limb_20_col64;
                let op0_limb_21_col65 = memory_id_to_big_value_tmp_5a147_14.get_m31(21);
                *row[65] = op0_limb_21_col65;
                let op0_limb_22_col66 = memory_id_to_big_value_tmp_5a147_14.get_m31(22);
                *row[66] = op0_limb_22_col66;
                let op0_limb_23_col67 = memory_id_to_big_value_tmp_5a147_14.get_m31(23);
                *row[67] = op0_limb_23_col67;
                let op0_limb_24_col68 = memory_id_to_big_value_tmp_5a147_14.get_m31(24);
                *row[68] = op0_limb_24_col68;
                let op0_limb_25_col69 = memory_id_to_big_value_tmp_5a147_14.get_m31(25);
                *row[69] = op0_limb_25_col69;
                let op0_limb_26_col70 = memory_id_to_big_value_tmp_5a147_14.get_m31(26);
                *row[70] = op0_limb_26_col70;
                let op0_limb_27_col71 = memory_id_to_big_value_tmp_5a147_14.get_m31(27);
                *row[71] = op0_limb_27_col71;
                for (i, &input) in op0_id_col43.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[1] = input;
                }
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col43,
                    op0_limb_0_col44,
                    op0_limb_1_col45,
                    op0_limb_2_col46,
                    op0_limb_3_col47,
                    op0_limb_4_col48,
                    op0_limb_5_col49,
                    op0_limb_6_col50,
                    op0_limb_7_col51,
                    op0_limb_8_col52,
                    op0_limb_9_col53,
                    op0_limb_10_col54,
                    op0_limb_11_col55,
                    op0_limb_12_col56,
                    op0_limb_13_col57,
                    op0_limb_14_col58,
                    op0_limb_15_col59,
                    op0_limb_16_col60,
                    op0_limb_17_col61,
                    op0_limb_18_col62,
                    op0_limb_19_col63,
                    op0_limb_20_col64,
                    op0_limb_21_col65,
                    op0_limb_22_col66,
                    op0_limb_23_col67,
                    op0_limb_24_col68,
                    op0_limb_25_col69,
                    op0_limb_26_col70,
                    op0_limb_27_col71,
                ];

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_5a147_15 = memory_address_to_id_state
                    .deduce_output(((mem1_base_col13) + ((offset2_col5) - (M31_32768))));
                let memory_id_to_big_value_tmp_5a147_16 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_5a147_15);
                let op1_id_col72 = memory_address_to_id_value_tmp_5a147_15;
                *row[72] = op1_id_col72;
                for (i, &input) in ((mem1_base_col13) + ((offset2_col5) - (M31_32768)))
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].memory_address_to_id_inputs[2] = input;
                }
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col13) + ((offset2_col5) - (M31_32768))),
                    op1_id_col72,
                ];
                let op1_limb_0_col73 = memory_id_to_big_value_tmp_5a147_16.get_m31(0);
                *row[73] = op1_limb_0_col73;
                let op1_limb_1_col74 = memory_id_to_big_value_tmp_5a147_16.get_m31(1);
                *row[74] = op1_limb_1_col74;
                let op1_limb_2_col75 = memory_id_to_big_value_tmp_5a147_16.get_m31(2);
                *row[75] = op1_limb_2_col75;
                let op1_limb_3_col76 = memory_id_to_big_value_tmp_5a147_16.get_m31(3);
                *row[76] = op1_limb_3_col76;
                let op1_limb_4_col77 = memory_id_to_big_value_tmp_5a147_16.get_m31(4);
                *row[77] = op1_limb_4_col77;
                let op1_limb_5_col78 = memory_id_to_big_value_tmp_5a147_16.get_m31(5);
                *row[78] = op1_limb_5_col78;
                let op1_limb_6_col79 = memory_id_to_big_value_tmp_5a147_16.get_m31(6);
                *row[79] = op1_limb_6_col79;
                let op1_limb_7_col80 = memory_id_to_big_value_tmp_5a147_16.get_m31(7);
                *row[80] = op1_limb_7_col80;
                let op1_limb_8_col81 = memory_id_to_big_value_tmp_5a147_16.get_m31(8);
                *row[81] = op1_limb_8_col81;
                let op1_limb_9_col82 = memory_id_to_big_value_tmp_5a147_16.get_m31(9);
                *row[82] = op1_limb_9_col82;
                let op1_limb_10_col83 = memory_id_to_big_value_tmp_5a147_16.get_m31(10);
                *row[83] = op1_limb_10_col83;
                let op1_limb_11_col84 = memory_id_to_big_value_tmp_5a147_16.get_m31(11);
                *row[84] = op1_limb_11_col84;
                let op1_limb_12_col85 = memory_id_to_big_value_tmp_5a147_16.get_m31(12);
                *row[85] = op1_limb_12_col85;
                let op1_limb_13_col86 = memory_id_to_big_value_tmp_5a147_16.get_m31(13);
                *row[86] = op1_limb_13_col86;
                let op1_limb_14_col87 = memory_id_to_big_value_tmp_5a147_16.get_m31(14);
                *row[87] = op1_limb_14_col87;
                let op1_limb_15_col88 = memory_id_to_big_value_tmp_5a147_16.get_m31(15);
                *row[88] = op1_limb_15_col88;
                let op1_limb_16_col89 = memory_id_to_big_value_tmp_5a147_16.get_m31(16);
                *row[89] = op1_limb_16_col89;
                let op1_limb_17_col90 = memory_id_to_big_value_tmp_5a147_16.get_m31(17);
                *row[90] = op1_limb_17_col90;
                let op1_limb_18_col91 = memory_id_to_big_value_tmp_5a147_16.get_m31(18);
                *row[91] = op1_limb_18_col91;
                let op1_limb_19_col92 = memory_id_to_big_value_tmp_5a147_16.get_m31(19);
                *row[92] = op1_limb_19_col92;
                let op1_limb_20_col93 = memory_id_to_big_value_tmp_5a147_16.get_m31(20);
                *row[93] = op1_limb_20_col93;
                let op1_limb_21_col94 = memory_id_to_big_value_tmp_5a147_16.get_m31(21);
                *row[94] = op1_limb_21_col94;
                let op1_limb_22_col95 = memory_id_to_big_value_tmp_5a147_16.get_m31(22);
                *row[95] = op1_limb_22_col95;
                let op1_limb_23_col96 = memory_id_to_big_value_tmp_5a147_16.get_m31(23);
                *row[96] = op1_limb_23_col96;
                let op1_limb_24_col97 = memory_id_to_big_value_tmp_5a147_16.get_m31(24);
                *row[97] = op1_limb_24_col97;
                let op1_limb_25_col98 = memory_id_to_big_value_tmp_5a147_16.get_m31(25);
                *row[98] = op1_limb_25_col98;
                let op1_limb_26_col99 = memory_id_to_big_value_tmp_5a147_16.get_m31(26);
                *row[99] = op1_limb_26_col99;
                let op1_limb_27_col100 = memory_id_to_big_value_tmp_5a147_16.get_m31(27);
                *row[100] = op1_limb_27_col100;
                for (i, &input) in op1_id_col72.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[2] = input;
                }
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col72,
                    op1_limb_0_col73,
                    op1_limb_1_col74,
                    op1_limb_2_col75,
                    op1_limb_3_col76,
                    op1_limb_4_col77,
                    op1_limb_5_col78,
                    op1_limb_6_col79,
                    op1_limb_7_col80,
                    op1_limb_8_col81,
                    op1_limb_9_col82,
                    op1_limb_10_col83,
                    op1_limb_11_col84,
                    op1_limb_12_col85,
                    op1_limb_13_col86,
                    op1_limb_14_col87,
                    op1_limb_15_col88,
                    op1_limb_16_col89,
                    op1_limb_17_col90,
                    op1_limb_18_col91,
                    op1_limb_19_col92,
                    op1_limb_20_col93,
                    op1_limb_21_col94,
                    op1_limb_22_col95,
                    op1_limb_23_col96,
                    op1_limb_24_col97,
                    op1_limb_25_col98,
                    op1_limb_26_col99,
                    op1_limb_27_col100,
                ];

                // Verify Mul 252.

                let conv_tmp_5a147_17 =
                    (((M31_0) - (dst_limb_0_col15)) + ((op0_limb_0_col44) * (op1_limb_0_col73)));
                let conv_tmp_5a147_18 = ((((M31_0) - (dst_limb_1_col16))
                    + ((op0_limb_0_col44) * (op1_limb_1_col74)))
                    + ((op0_limb_1_col45) * (op1_limb_0_col73)));
                let conv_tmp_5a147_19 = (((((M31_0) - (dst_limb_2_col17))
                    + ((op0_limb_0_col44) * (op1_limb_2_col75)))
                    + ((op0_limb_1_col45) * (op1_limb_1_col74)))
                    + ((op0_limb_2_col46) * (op1_limb_0_col73)));
                let conv_tmp_5a147_20 = ((((((M31_0) - (dst_limb_3_col18))
                    + ((op0_limb_0_col44) * (op1_limb_3_col76)))
                    + ((op0_limb_1_col45) * (op1_limb_2_col75)))
                    + ((op0_limb_2_col46) * (op1_limb_1_col74)))
                    + ((op0_limb_3_col47) * (op1_limb_0_col73)));
                let conv_tmp_5a147_21 = (((((((M31_0) - (dst_limb_4_col19))
                    + ((op0_limb_0_col44) * (op1_limb_4_col77)))
                    + ((op0_limb_1_col45) * (op1_limb_3_col76)))
                    + ((op0_limb_2_col46) * (op1_limb_2_col75)))
                    + ((op0_limb_3_col47) * (op1_limb_1_col74)))
                    + ((op0_limb_4_col48) * (op1_limb_0_col73)));
                let conv_tmp_5a147_22 = ((((((((M31_0) - (dst_limb_5_col20))
                    + ((op0_limb_0_col44) * (op1_limb_5_col78)))
                    + ((op0_limb_1_col45) * (op1_limb_4_col77)))
                    + ((op0_limb_2_col46) * (op1_limb_3_col76)))
                    + ((op0_limb_3_col47) * (op1_limb_2_col75)))
                    + ((op0_limb_4_col48) * (op1_limb_1_col74)))
                    + ((op0_limb_5_col49) * (op1_limb_0_col73)));
                let conv_tmp_5a147_23 = (((((((((M31_0) - (dst_limb_6_col21))
                    + ((op0_limb_0_col44) * (op1_limb_6_col79)))
                    + ((op0_limb_1_col45) * (op1_limb_5_col78)))
                    + ((op0_limb_2_col46) * (op1_limb_4_col77)))
                    + ((op0_limb_3_col47) * (op1_limb_3_col76)))
                    + ((op0_limb_4_col48) * (op1_limb_2_col75)))
                    + ((op0_limb_5_col49) * (op1_limb_1_col74)))
                    + ((op0_limb_6_col50) * (op1_limb_0_col73)));
                let conv_tmp_5a147_24 = ((((((((((M31_0) - (dst_limb_7_col22))
                    + ((op0_limb_0_col44) * (op1_limb_7_col80)))
                    + ((op0_limb_1_col45) * (op1_limb_6_col79)))
                    + ((op0_limb_2_col46) * (op1_limb_5_col78)))
                    + ((op0_limb_3_col47) * (op1_limb_4_col77)))
                    + ((op0_limb_4_col48) * (op1_limb_3_col76)))
                    + ((op0_limb_5_col49) * (op1_limb_2_col75)))
                    + ((op0_limb_6_col50) * (op1_limb_1_col74)))
                    + ((op0_limb_7_col51) * (op1_limb_0_col73)));
                let conv_tmp_5a147_25 = (((((((((((M31_0) - (dst_limb_8_col23))
                    + ((op0_limb_0_col44) * (op1_limb_8_col81)))
                    + ((op0_limb_1_col45) * (op1_limb_7_col80)))
                    + ((op0_limb_2_col46) * (op1_limb_6_col79)))
                    + ((op0_limb_3_col47) * (op1_limb_5_col78)))
                    + ((op0_limb_4_col48) * (op1_limb_4_col77)))
                    + ((op0_limb_5_col49) * (op1_limb_3_col76)))
                    + ((op0_limb_6_col50) * (op1_limb_2_col75)))
                    + ((op0_limb_7_col51) * (op1_limb_1_col74)))
                    + ((op0_limb_8_col52) * (op1_limb_0_col73)));
                let conv_tmp_5a147_26 = ((((((((((((M31_0) - (dst_limb_9_col24))
                    + ((op0_limb_0_col44) * (op1_limb_9_col82)))
                    + ((op0_limb_1_col45) * (op1_limb_8_col81)))
                    + ((op0_limb_2_col46) * (op1_limb_7_col80)))
                    + ((op0_limb_3_col47) * (op1_limb_6_col79)))
                    + ((op0_limb_4_col48) * (op1_limb_5_col78)))
                    + ((op0_limb_5_col49) * (op1_limb_4_col77)))
                    + ((op0_limb_6_col50) * (op1_limb_3_col76)))
                    + ((op0_limb_7_col51) * (op1_limb_2_col75)))
                    + ((op0_limb_8_col52) * (op1_limb_1_col74)))
                    + ((op0_limb_9_col53) * (op1_limb_0_col73)));
                let conv_tmp_5a147_27 = (((((((((((((M31_0) - (dst_limb_10_col25))
                    + ((op0_limb_0_col44) * (op1_limb_10_col83)))
                    + ((op0_limb_1_col45) * (op1_limb_9_col82)))
                    + ((op0_limb_2_col46) * (op1_limb_8_col81)))
                    + ((op0_limb_3_col47) * (op1_limb_7_col80)))
                    + ((op0_limb_4_col48) * (op1_limb_6_col79)))
                    + ((op0_limb_5_col49) * (op1_limb_5_col78)))
                    + ((op0_limb_6_col50) * (op1_limb_4_col77)))
                    + ((op0_limb_7_col51) * (op1_limb_3_col76)))
                    + ((op0_limb_8_col52) * (op1_limb_2_col75)))
                    + ((op0_limb_9_col53) * (op1_limb_1_col74)))
                    + ((op0_limb_10_col54) * (op1_limb_0_col73)));
                let conv_tmp_5a147_28 = ((((((((((((((M31_0) - (dst_limb_11_col26))
                    + ((op0_limb_0_col44) * (op1_limb_11_col84)))
                    + ((op0_limb_1_col45) * (op1_limb_10_col83)))
                    + ((op0_limb_2_col46) * (op1_limb_9_col82)))
                    + ((op0_limb_3_col47) * (op1_limb_8_col81)))
                    + ((op0_limb_4_col48) * (op1_limb_7_col80)))
                    + ((op0_limb_5_col49) * (op1_limb_6_col79)))
                    + ((op0_limb_6_col50) * (op1_limb_5_col78)))
                    + ((op0_limb_7_col51) * (op1_limb_4_col77)))
                    + ((op0_limb_8_col52) * (op1_limb_3_col76)))
                    + ((op0_limb_9_col53) * (op1_limb_2_col75)))
                    + ((op0_limb_10_col54) * (op1_limb_1_col74)))
                    + ((op0_limb_11_col55) * (op1_limb_0_col73)));
                let conv_tmp_5a147_29 = (((((((((((((((M31_0) - (dst_limb_12_col27))
                    + ((op0_limb_0_col44) * (op1_limb_12_col85)))
                    + ((op0_limb_1_col45) * (op1_limb_11_col84)))
                    + ((op0_limb_2_col46) * (op1_limb_10_col83)))
                    + ((op0_limb_3_col47) * (op1_limb_9_col82)))
                    + ((op0_limb_4_col48) * (op1_limb_8_col81)))
                    + ((op0_limb_5_col49) * (op1_limb_7_col80)))
                    + ((op0_limb_6_col50) * (op1_limb_6_col79)))
                    + ((op0_limb_7_col51) * (op1_limb_5_col78)))
                    + ((op0_limb_8_col52) * (op1_limb_4_col77)))
                    + ((op0_limb_9_col53) * (op1_limb_3_col76)))
                    + ((op0_limb_10_col54) * (op1_limb_2_col75)))
                    + ((op0_limb_11_col55) * (op1_limb_1_col74)))
                    + ((op0_limb_12_col56) * (op1_limb_0_col73)));
                let conv_tmp_5a147_30 = ((((((((((((((((M31_0) - (dst_limb_13_col28))
                    + ((op0_limb_0_col44) * (op1_limb_13_col86)))
                    + ((op0_limb_1_col45) * (op1_limb_12_col85)))
                    + ((op0_limb_2_col46) * (op1_limb_11_col84)))
                    + ((op0_limb_3_col47) * (op1_limb_10_col83)))
                    + ((op0_limb_4_col48) * (op1_limb_9_col82)))
                    + ((op0_limb_5_col49) * (op1_limb_8_col81)))
                    + ((op0_limb_6_col50) * (op1_limb_7_col80)))
                    + ((op0_limb_7_col51) * (op1_limb_6_col79)))
                    + ((op0_limb_8_col52) * (op1_limb_5_col78)))
                    + ((op0_limb_9_col53) * (op1_limb_4_col77)))
                    + ((op0_limb_10_col54) * (op1_limb_3_col76)))
                    + ((op0_limb_11_col55) * (op1_limb_2_col75)))
                    + ((op0_limb_12_col56) * (op1_limb_1_col74)))
                    + ((op0_limb_13_col57) * (op1_limb_0_col73)));
                let conv_tmp_5a147_31 = (((((((((((((((((M31_0)
                    - (dst_limb_14_col29))
                    + ((op0_limb_0_col44) * (op1_limb_14_col87)))
                    + ((op0_limb_1_col45) * (op1_limb_13_col86)))
                    + ((op0_limb_2_col46) * (op1_limb_12_col85)))
                    + ((op0_limb_3_col47) * (op1_limb_11_col84)))
                    + ((op0_limb_4_col48) * (op1_limb_10_col83)))
                    + ((op0_limb_5_col49) * (op1_limb_9_col82)))
                    + ((op0_limb_6_col50) * (op1_limb_8_col81)))
                    + ((op0_limb_7_col51) * (op1_limb_7_col80)))
                    + ((op0_limb_8_col52) * (op1_limb_6_col79)))
                    + ((op0_limb_9_col53) * (op1_limb_5_col78)))
                    + ((op0_limb_10_col54) * (op1_limb_4_col77)))
                    + ((op0_limb_11_col55) * (op1_limb_3_col76)))
                    + ((op0_limb_12_col56) * (op1_limb_2_col75)))
                    + ((op0_limb_13_col57) * (op1_limb_1_col74)))
                    + ((op0_limb_14_col58) * (op1_limb_0_col73)));
                let conv_tmp_5a147_32 = ((((((((((((((((((M31_0)
                    - (dst_limb_15_col30))
                    + ((op0_limb_0_col44) * (op1_limb_15_col88)))
                    + ((op0_limb_1_col45) * (op1_limb_14_col87)))
                    + ((op0_limb_2_col46) * (op1_limb_13_col86)))
                    + ((op0_limb_3_col47) * (op1_limb_12_col85)))
                    + ((op0_limb_4_col48) * (op1_limb_11_col84)))
                    + ((op0_limb_5_col49) * (op1_limb_10_col83)))
                    + ((op0_limb_6_col50) * (op1_limb_9_col82)))
                    + ((op0_limb_7_col51) * (op1_limb_8_col81)))
                    + ((op0_limb_8_col52) * (op1_limb_7_col80)))
                    + ((op0_limb_9_col53) * (op1_limb_6_col79)))
                    + ((op0_limb_10_col54) * (op1_limb_5_col78)))
                    + ((op0_limb_11_col55) * (op1_limb_4_col77)))
                    + ((op0_limb_12_col56) * (op1_limb_3_col76)))
                    + ((op0_limb_13_col57) * (op1_limb_2_col75)))
                    + ((op0_limb_14_col58) * (op1_limb_1_col74)))
                    + ((op0_limb_15_col59) * (op1_limb_0_col73)));
                let conv_tmp_5a147_33 = (((((((((((((((((((M31_0)
                    - (dst_limb_16_col31))
                    + ((op0_limb_0_col44) * (op1_limb_16_col89)))
                    + ((op0_limb_1_col45) * (op1_limb_15_col88)))
                    + ((op0_limb_2_col46) * (op1_limb_14_col87)))
                    + ((op0_limb_3_col47) * (op1_limb_13_col86)))
                    + ((op0_limb_4_col48) * (op1_limb_12_col85)))
                    + ((op0_limb_5_col49) * (op1_limb_11_col84)))
                    + ((op0_limb_6_col50) * (op1_limb_10_col83)))
                    + ((op0_limb_7_col51) * (op1_limb_9_col82)))
                    + ((op0_limb_8_col52) * (op1_limb_8_col81)))
                    + ((op0_limb_9_col53) * (op1_limb_7_col80)))
                    + ((op0_limb_10_col54) * (op1_limb_6_col79)))
                    + ((op0_limb_11_col55) * (op1_limb_5_col78)))
                    + ((op0_limb_12_col56) * (op1_limb_4_col77)))
                    + ((op0_limb_13_col57) * (op1_limb_3_col76)))
                    + ((op0_limb_14_col58) * (op1_limb_2_col75)))
                    + ((op0_limb_15_col59) * (op1_limb_1_col74)))
                    + ((op0_limb_16_col60) * (op1_limb_0_col73)));
                let conv_tmp_5a147_34 = ((((((((((((((((((((M31_0)
                    - (dst_limb_17_col32))
                    + ((op0_limb_0_col44) * (op1_limb_17_col90)))
                    + ((op0_limb_1_col45) * (op1_limb_16_col89)))
                    + ((op0_limb_2_col46) * (op1_limb_15_col88)))
                    + ((op0_limb_3_col47) * (op1_limb_14_col87)))
                    + ((op0_limb_4_col48) * (op1_limb_13_col86)))
                    + ((op0_limb_5_col49) * (op1_limb_12_col85)))
                    + ((op0_limb_6_col50) * (op1_limb_11_col84)))
                    + ((op0_limb_7_col51) * (op1_limb_10_col83)))
                    + ((op0_limb_8_col52) * (op1_limb_9_col82)))
                    + ((op0_limb_9_col53) * (op1_limb_8_col81)))
                    + ((op0_limb_10_col54) * (op1_limb_7_col80)))
                    + ((op0_limb_11_col55) * (op1_limb_6_col79)))
                    + ((op0_limb_12_col56) * (op1_limb_5_col78)))
                    + ((op0_limb_13_col57) * (op1_limb_4_col77)))
                    + ((op0_limb_14_col58) * (op1_limb_3_col76)))
                    + ((op0_limb_15_col59) * (op1_limb_2_col75)))
                    + ((op0_limb_16_col60) * (op1_limb_1_col74)))
                    + ((op0_limb_17_col61) * (op1_limb_0_col73)));
                let conv_tmp_5a147_35 = (((((((((((((((((((((M31_0)
                    - (dst_limb_18_col33))
                    + ((op0_limb_0_col44) * (op1_limb_18_col91)))
                    + ((op0_limb_1_col45) * (op1_limb_17_col90)))
                    + ((op0_limb_2_col46) * (op1_limb_16_col89)))
                    + ((op0_limb_3_col47) * (op1_limb_15_col88)))
                    + ((op0_limb_4_col48) * (op1_limb_14_col87)))
                    + ((op0_limb_5_col49) * (op1_limb_13_col86)))
                    + ((op0_limb_6_col50) * (op1_limb_12_col85)))
                    + ((op0_limb_7_col51) * (op1_limb_11_col84)))
                    + ((op0_limb_8_col52) * (op1_limb_10_col83)))
                    + ((op0_limb_9_col53) * (op1_limb_9_col82)))
                    + ((op0_limb_10_col54) * (op1_limb_8_col81)))
                    + ((op0_limb_11_col55) * (op1_limb_7_col80)))
                    + ((op0_limb_12_col56) * (op1_limb_6_col79)))
                    + ((op0_limb_13_col57) * (op1_limb_5_col78)))
                    + ((op0_limb_14_col58) * (op1_limb_4_col77)))
                    + ((op0_limb_15_col59) * (op1_limb_3_col76)))
                    + ((op0_limb_16_col60) * (op1_limb_2_col75)))
                    + ((op0_limb_17_col61) * (op1_limb_1_col74)))
                    + ((op0_limb_18_col62) * (op1_limb_0_col73)));
                let conv_tmp_5a147_36 = ((((((((((((((((((((((M31_0)
                    - (dst_limb_19_col34))
                    + ((op0_limb_0_col44) * (op1_limb_19_col92)))
                    + ((op0_limb_1_col45) * (op1_limb_18_col91)))
                    + ((op0_limb_2_col46) * (op1_limb_17_col90)))
                    + ((op0_limb_3_col47) * (op1_limb_16_col89)))
                    + ((op0_limb_4_col48) * (op1_limb_15_col88)))
                    + ((op0_limb_5_col49) * (op1_limb_14_col87)))
                    + ((op0_limb_6_col50) * (op1_limb_13_col86)))
                    + ((op0_limb_7_col51) * (op1_limb_12_col85)))
                    + ((op0_limb_8_col52) * (op1_limb_11_col84)))
                    + ((op0_limb_9_col53) * (op1_limb_10_col83)))
                    + ((op0_limb_10_col54) * (op1_limb_9_col82)))
                    + ((op0_limb_11_col55) * (op1_limb_8_col81)))
                    + ((op0_limb_12_col56) * (op1_limb_7_col80)))
                    + ((op0_limb_13_col57) * (op1_limb_6_col79)))
                    + ((op0_limb_14_col58) * (op1_limb_5_col78)))
                    + ((op0_limb_15_col59) * (op1_limb_4_col77)))
                    + ((op0_limb_16_col60) * (op1_limb_3_col76)))
                    + ((op0_limb_17_col61) * (op1_limb_2_col75)))
                    + ((op0_limb_18_col62) * (op1_limb_1_col74)))
                    + ((op0_limb_19_col63) * (op1_limb_0_col73)));
                let conv_tmp_5a147_37 = (((((((((((((((((((((((M31_0)
                    - (dst_limb_20_col35))
                    + ((op0_limb_0_col44) * (op1_limb_20_col93)))
                    + ((op0_limb_1_col45) * (op1_limb_19_col92)))
                    + ((op0_limb_2_col46) * (op1_limb_18_col91)))
                    + ((op0_limb_3_col47) * (op1_limb_17_col90)))
                    + ((op0_limb_4_col48) * (op1_limb_16_col89)))
                    + ((op0_limb_5_col49) * (op1_limb_15_col88)))
                    + ((op0_limb_6_col50) * (op1_limb_14_col87)))
                    + ((op0_limb_7_col51) * (op1_limb_13_col86)))
                    + ((op0_limb_8_col52) * (op1_limb_12_col85)))
                    + ((op0_limb_9_col53) * (op1_limb_11_col84)))
                    + ((op0_limb_10_col54) * (op1_limb_10_col83)))
                    + ((op0_limb_11_col55) * (op1_limb_9_col82)))
                    + ((op0_limb_12_col56) * (op1_limb_8_col81)))
                    + ((op0_limb_13_col57) * (op1_limb_7_col80)))
                    + ((op0_limb_14_col58) * (op1_limb_6_col79)))
                    + ((op0_limb_15_col59) * (op1_limb_5_col78)))
                    + ((op0_limb_16_col60) * (op1_limb_4_col77)))
                    + ((op0_limb_17_col61) * (op1_limb_3_col76)))
                    + ((op0_limb_18_col62) * (op1_limb_2_col75)))
                    + ((op0_limb_19_col63) * (op1_limb_1_col74)))
                    + ((op0_limb_20_col64) * (op1_limb_0_col73)));
                let conv_tmp_5a147_38 = ((((((((((((((((((((((((M31_0)
                    - (dst_limb_21_col36))
                    + ((op0_limb_0_col44) * (op1_limb_21_col94)))
                    + ((op0_limb_1_col45) * (op1_limb_20_col93)))
                    + ((op0_limb_2_col46) * (op1_limb_19_col92)))
                    + ((op0_limb_3_col47) * (op1_limb_18_col91)))
                    + ((op0_limb_4_col48) * (op1_limb_17_col90)))
                    + ((op0_limb_5_col49) * (op1_limb_16_col89)))
                    + ((op0_limb_6_col50) * (op1_limb_15_col88)))
                    + ((op0_limb_7_col51) * (op1_limb_14_col87)))
                    + ((op0_limb_8_col52) * (op1_limb_13_col86)))
                    + ((op0_limb_9_col53) * (op1_limb_12_col85)))
                    + ((op0_limb_10_col54) * (op1_limb_11_col84)))
                    + ((op0_limb_11_col55) * (op1_limb_10_col83)))
                    + ((op0_limb_12_col56) * (op1_limb_9_col82)))
                    + ((op0_limb_13_col57) * (op1_limb_8_col81)))
                    + ((op0_limb_14_col58) * (op1_limb_7_col80)))
                    + ((op0_limb_15_col59) * (op1_limb_6_col79)))
                    + ((op0_limb_16_col60) * (op1_limb_5_col78)))
                    + ((op0_limb_17_col61) * (op1_limb_4_col77)))
                    + ((op0_limb_18_col62) * (op1_limb_3_col76)))
                    + ((op0_limb_19_col63) * (op1_limb_2_col75)))
                    + ((op0_limb_20_col64) * (op1_limb_1_col74)))
                    + ((op0_limb_21_col65) * (op1_limb_0_col73)));
                let conv_tmp_5a147_39 = (((((((((((((((((((((((((M31_0)
                    - (dst_limb_22_col37))
                    + ((op0_limb_0_col44) * (op1_limb_22_col95)))
                    + ((op0_limb_1_col45) * (op1_limb_21_col94)))
                    + ((op0_limb_2_col46) * (op1_limb_20_col93)))
                    + ((op0_limb_3_col47) * (op1_limb_19_col92)))
                    + ((op0_limb_4_col48) * (op1_limb_18_col91)))
                    + ((op0_limb_5_col49) * (op1_limb_17_col90)))
                    + ((op0_limb_6_col50) * (op1_limb_16_col89)))
                    + ((op0_limb_7_col51) * (op1_limb_15_col88)))
                    + ((op0_limb_8_col52) * (op1_limb_14_col87)))
                    + ((op0_limb_9_col53) * (op1_limb_13_col86)))
                    + ((op0_limb_10_col54) * (op1_limb_12_col85)))
                    + ((op0_limb_11_col55) * (op1_limb_11_col84)))
                    + ((op0_limb_12_col56) * (op1_limb_10_col83)))
                    + ((op0_limb_13_col57) * (op1_limb_9_col82)))
                    + ((op0_limb_14_col58) * (op1_limb_8_col81)))
                    + ((op0_limb_15_col59) * (op1_limb_7_col80)))
                    + ((op0_limb_16_col60) * (op1_limb_6_col79)))
                    + ((op0_limb_17_col61) * (op1_limb_5_col78)))
                    + ((op0_limb_18_col62) * (op1_limb_4_col77)))
                    + ((op0_limb_19_col63) * (op1_limb_3_col76)))
                    + ((op0_limb_20_col64) * (op1_limb_2_col75)))
                    + ((op0_limb_21_col65) * (op1_limb_1_col74)))
                    + ((op0_limb_22_col66) * (op1_limb_0_col73)));
                let conv_tmp_5a147_40 = ((((((((((((((((((((((((((M31_0)
                    - (dst_limb_23_col38))
                    + ((op0_limb_0_col44) * (op1_limb_23_col96)))
                    + ((op0_limb_1_col45) * (op1_limb_22_col95)))
                    + ((op0_limb_2_col46) * (op1_limb_21_col94)))
                    + ((op0_limb_3_col47) * (op1_limb_20_col93)))
                    + ((op0_limb_4_col48) * (op1_limb_19_col92)))
                    + ((op0_limb_5_col49) * (op1_limb_18_col91)))
                    + ((op0_limb_6_col50) * (op1_limb_17_col90)))
                    + ((op0_limb_7_col51) * (op1_limb_16_col89)))
                    + ((op0_limb_8_col52) * (op1_limb_15_col88)))
                    + ((op0_limb_9_col53) * (op1_limb_14_col87)))
                    + ((op0_limb_10_col54) * (op1_limb_13_col86)))
                    + ((op0_limb_11_col55) * (op1_limb_12_col85)))
                    + ((op0_limb_12_col56) * (op1_limb_11_col84)))
                    + ((op0_limb_13_col57) * (op1_limb_10_col83)))
                    + ((op0_limb_14_col58) * (op1_limb_9_col82)))
                    + ((op0_limb_15_col59) * (op1_limb_8_col81)))
                    + ((op0_limb_16_col60) * (op1_limb_7_col80)))
                    + ((op0_limb_17_col61) * (op1_limb_6_col79)))
                    + ((op0_limb_18_col62) * (op1_limb_5_col78)))
                    + ((op0_limb_19_col63) * (op1_limb_4_col77)))
                    + ((op0_limb_20_col64) * (op1_limb_3_col76)))
                    + ((op0_limb_21_col65) * (op1_limb_2_col75)))
                    + ((op0_limb_22_col66) * (op1_limb_1_col74)))
                    + ((op0_limb_23_col67) * (op1_limb_0_col73)));
                let conv_tmp_5a147_41 = (((((((((((((((((((((((((((M31_0)
                    - (dst_limb_24_col39))
                    + ((op0_limb_0_col44) * (op1_limb_24_col97)))
                    + ((op0_limb_1_col45) * (op1_limb_23_col96)))
                    + ((op0_limb_2_col46) * (op1_limb_22_col95)))
                    + ((op0_limb_3_col47) * (op1_limb_21_col94)))
                    + ((op0_limb_4_col48) * (op1_limb_20_col93)))
                    + ((op0_limb_5_col49) * (op1_limb_19_col92)))
                    + ((op0_limb_6_col50) * (op1_limb_18_col91)))
                    + ((op0_limb_7_col51) * (op1_limb_17_col90)))
                    + ((op0_limb_8_col52) * (op1_limb_16_col89)))
                    + ((op0_limb_9_col53) * (op1_limb_15_col88)))
                    + ((op0_limb_10_col54) * (op1_limb_14_col87)))
                    + ((op0_limb_11_col55) * (op1_limb_13_col86)))
                    + ((op0_limb_12_col56) * (op1_limb_12_col85)))
                    + ((op0_limb_13_col57) * (op1_limb_11_col84)))
                    + ((op0_limb_14_col58) * (op1_limb_10_col83)))
                    + ((op0_limb_15_col59) * (op1_limb_9_col82)))
                    + ((op0_limb_16_col60) * (op1_limb_8_col81)))
                    + ((op0_limb_17_col61) * (op1_limb_7_col80)))
                    + ((op0_limb_18_col62) * (op1_limb_6_col79)))
                    + ((op0_limb_19_col63) * (op1_limb_5_col78)))
                    + ((op0_limb_20_col64) * (op1_limb_4_col77)))
                    + ((op0_limb_21_col65) * (op1_limb_3_col76)))
                    + ((op0_limb_22_col66) * (op1_limb_2_col75)))
                    + ((op0_limb_23_col67) * (op1_limb_1_col74)))
                    + ((op0_limb_24_col68) * (op1_limb_0_col73)));
                let conv_tmp_5a147_42 = ((((((((((((((((((((((((((((M31_0)
                    - (dst_limb_25_col40))
                    + ((op0_limb_0_col44) * (op1_limb_25_col98)))
                    + ((op0_limb_1_col45) * (op1_limb_24_col97)))
                    + ((op0_limb_2_col46) * (op1_limb_23_col96)))
                    + ((op0_limb_3_col47) * (op1_limb_22_col95)))
                    + ((op0_limb_4_col48) * (op1_limb_21_col94)))
                    + ((op0_limb_5_col49) * (op1_limb_20_col93)))
                    + ((op0_limb_6_col50) * (op1_limb_19_col92)))
                    + ((op0_limb_7_col51) * (op1_limb_18_col91)))
                    + ((op0_limb_8_col52) * (op1_limb_17_col90)))
                    + ((op0_limb_9_col53) * (op1_limb_16_col89)))
                    + ((op0_limb_10_col54) * (op1_limb_15_col88)))
                    + ((op0_limb_11_col55) * (op1_limb_14_col87)))
                    + ((op0_limb_12_col56) * (op1_limb_13_col86)))
                    + ((op0_limb_13_col57) * (op1_limb_12_col85)))
                    + ((op0_limb_14_col58) * (op1_limb_11_col84)))
                    + ((op0_limb_15_col59) * (op1_limb_10_col83)))
                    + ((op0_limb_16_col60) * (op1_limb_9_col82)))
                    + ((op0_limb_17_col61) * (op1_limb_8_col81)))
                    + ((op0_limb_18_col62) * (op1_limb_7_col80)))
                    + ((op0_limb_19_col63) * (op1_limb_6_col79)))
                    + ((op0_limb_20_col64) * (op1_limb_5_col78)))
                    + ((op0_limb_21_col65) * (op1_limb_4_col77)))
                    + ((op0_limb_22_col66) * (op1_limb_3_col76)))
                    + ((op0_limb_23_col67) * (op1_limb_2_col75)))
                    + ((op0_limb_24_col68) * (op1_limb_1_col74)))
                    + ((op0_limb_25_col69) * (op1_limb_0_col73)));
                let conv_tmp_5a147_43 = (((((((((((((((((((((((((((((M31_0)
                    - (dst_limb_26_col41))
                    + ((op0_limb_0_col44) * (op1_limb_26_col99)))
                    + ((op0_limb_1_col45) * (op1_limb_25_col98)))
                    + ((op0_limb_2_col46) * (op1_limb_24_col97)))
                    + ((op0_limb_3_col47) * (op1_limb_23_col96)))
                    + ((op0_limb_4_col48) * (op1_limb_22_col95)))
                    + ((op0_limb_5_col49) * (op1_limb_21_col94)))
                    + ((op0_limb_6_col50) * (op1_limb_20_col93)))
                    + ((op0_limb_7_col51) * (op1_limb_19_col92)))
                    + ((op0_limb_8_col52) * (op1_limb_18_col91)))
                    + ((op0_limb_9_col53) * (op1_limb_17_col90)))
                    + ((op0_limb_10_col54) * (op1_limb_16_col89)))
                    + ((op0_limb_11_col55) * (op1_limb_15_col88)))
                    + ((op0_limb_12_col56) * (op1_limb_14_col87)))
                    + ((op0_limb_13_col57) * (op1_limb_13_col86)))
                    + ((op0_limb_14_col58) * (op1_limb_12_col85)))
                    + ((op0_limb_15_col59) * (op1_limb_11_col84)))
                    + ((op0_limb_16_col60) * (op1_limb_10_col83)))
                    + ((op0_limb_17_col61) * (op1_limb_9_col82)))
                    + ((op0_limb_18_col62) * (op1_limb_8_col81)))
                    + ((op0_limb_19_col63) * (op1_limb_7_col80)))
                    + ((op0_limb_20_col64) * (op1_limb_6_col79)))
                    + ((op0_limb_21_col65) * (op1_limb_5_col78)))
                    + ((op0_limb_22_col66) * (op1_limb_4_col77)))
                    + ((op0_limb_23_col67) * (op1_limb_3_col76)))
                    + ((op0_limb_24_col68) * (op1_limb_2_col75)))
                    + ((op0_limb_25_col69) * (op1_limb_1_col74)))
                    + ((op0_limb_26_col70) * (op1_limb_0_col73)));
                let conv_tmp_5a147_44 = ((((((((((((((((((((((((((((((M31_0)
                    - (dst_limb_27_col42))
                    + ((op0_limb_0_col44) * (op1_limb_27_col100)))
                    + ((op0_limb_1_col45) * (op1_limb_26_col99)))
                    + ((op0_limb_2_col46) * (op1_limb_25_col98)))
                    + ((op0_limb_3_col47) * (op1_limb_24_col97)))
                    + ((op0_limb_4_col48) * (op1_limb_23_col96)))
                    + ((op0_limb_5_col49) * (op1_limb_22_col95)))
                    + ((op0_limb_6_col50) * (op1_limb_21_col94)))
                    + ((op0_limb_7_col51) * (op1_limb_20_col93)))
                    + ((op0_limb_8_col52) * (op1_limb_19_col92)))
                    + ((op0_limb_9_col53) * (op1_limb_18_col91)))
                    + ((op0_limb_10_col54) * (op1_limb_17_col90)))
                    + ((op0_limb_11_col55) * (op1_limb_16_col89)))
                    + ((op0_limb_12_col56) * (op1_limb_15_col88)))
                    + ((op0_limb_13_col57) * (op1_limb_14_col87)))
                    + ((op0_limb_14_col58) * (op1_limb_13_col86)))
                    + ((op0_limb_15_col59) * (op1_limb_12_col85)))
                    + ((op0_limb_16_col60) * (op1_limb_11_col84)))
                    + ((op0_limb_17_col61) * (op1_limb_10_col83)))
                    + ((op0_limb_18_col62) * (op1_limb_9_col82)))
                    + ((op0_limb_19_col63) * (op1_limb_8_col81)))
                    + ((op0_limb_20_col64) * (op1_limb_7_col80)))
                    + ((op0_limb_21_col65) * (op1_limb_6_col79)))
                    + ((op0_limb_22_col66) * (op1_limb_5_col78)))
                    + ((op0_limb_23_col67) * (op1_limb_4_col77)))
                    + ((op0_limb_24_col68) * (op1_limb_3_col76)))
                    + ((op0_limb_25_col69) * (op1_limb_2_col75)))
                    + ((op0_limb_26_col70) * (op1_limb_1_col74)))
                    + ((op0_limb_27_col71) * (op1_limb_0_col73)));
                let conv_tmp_5a147_45 = ((((((((((((((((((((((((((((M31_0)
                    + ((op0_limb_1_col45) * (op1_limb_27_col100)))
                    + ((op0_limb_2_col46) * (op1_limb_26_col99)))
                    + ((op0_limb_3_col47) * (op1_limb_25_col98)))
                    + ((op0_limb_4_col48) * (op1_limb_24_col97)))
                    + ((op0_limb_5_col49) * (op1_limb_23_col96)))
                    + ((op0_limb_6_col50) * (op1_limb_22_col95)))
                    + ((op0_limb_7_col51) * (op1_limb_21_col94)))
                    + ((op0_limb_8_col52) * (op1_limb_20_col93)))
                    + ((op0_limb_9_col53) * (op1_limb_19_col92)))
                    + ((op0_limb_10_col54) * (op1_limb_18_col91)))
                    + ((op0_limb_11_col55) * (op1_limb_17_col90)))
                    + ((op0_limb_12_col56) * (op1_limb_16_col89)))
                    + ((op0_limb_13_col57) * (op1_limb_15_col88)))
                    + ((op0_limb_14_col58) * (op1_limb_14_col87)))
                    + ((op0_limb_15_col59) * (op1_limb_13_col86)))
                    + ((op0_limb_16_col60) * (op1_limb_12_col85)))
                    + ((op0_limb_17_col61) * (op1_limb_11_col84)))
                    + ((op0_limb_18_col62) * (op1_limb_10_col83)))
                    + ((op0_limb_19_col63) * (op1_limb_9_col82)))
                    + ((op0_limb_20_col64) * (op1_limb_8_col81)))
                    + ((op0_limb_21_col65) * (op1_limb_7_col80)))
                    + ((op0_limb_22_col66) * (op1_limb_6_col79)))
                    + ((op0_limb_23_col67) * (op1_limb_5_col78)))
                    + ((op0_limb_24_col68) * (op1_limb_4_col77)))
                    + ((op0_limb_25_col69) * (op1_limb_3_col76)))
                    + ((op0_limb_26_col70) * (op1_limb_2_col75)))
                    + ((op0_limb_27_col71) * (op1_limb_1_col74)));
                let conv_tmp_5a147_46 = (((((((((((((((((((((((((((M31_0)
                    + ((op0_limb_2_col46) * (op1_limb_27_col100)))
                    + ((op0_limb_3_col47) * (op1_limb_26_col99)))
                    + ((op0_limb_4_col48) * (op1_limb_25_col98)))
                    + ((op0_limb_5_col49) * (op1_limb_24_col97)))
                    + ((op0_limb_6_col50) * (op1_limb_23_col96)))
                    + ((op0_limb_7_col51) * (op1_limb_22_col95)))
                    + ((op0_limb_8_col52) * (op1_limb_21_col94)))
                    + ((op0_limb_9_col53) * (op1_limb_20_col93)))
                    + ((op0_limb_10_col54) * (op1_limb_19_col92)))
                    + ((op0_limb_11_col55) * (op1_limb_18_col91)))
                    + ((op0_limb_12_col56) * (op1_limb_17_col90)))
                    + ((op0_limb_13_col57) * (op1_limb_16_col89)))
                    + ((op0_limb_14_col58) * (op1_limb_15_col88)))
                    + ((op0_limb_15_col59) * (op1_limb_14_col87)))
                    + ((op0_limb_16_col60) * (op1_limb_13_col86)))
                    + ((op0_limb_17_col61) * (op1_limb_12_col85)))
                    + ((op0_limb_18_col62) * (op1_limb_11_col84)))
                    + ((op0_limb_19_col63) * (op1_limb_10_col83)))
                    + ((op0_limb_20_col64) * (op1_limb_9_col82)))
                    + ((op0_limb_21_col65) * (op1_limb_8_col81)))
                    + ((op0_limb_22_col66) * (op1_limb_7_col80)))
                    + ((op0_limb_23_col67) * (op1_limb_6_col79)))
                    + ((op0_limb_24_col68) * (op1_limb_5_col78)))
                    + ((op0_limb_25_col69) * (op1_limb_4_col77)))
                    + ((op0_limb_26_col70) * (op1_limb_3_col76)))
                    + ((op0_limb_27_col71) * (op1_limb_2_col75)));
                let conv_tmp_5a147_47 = ((((((((((((((((((((((((((M31_0)
                    + ((op0_limb_3_col47) * (op1_limb_27_col100)))
                    + ((op0_limb_4_col48) * (op1_limb_26_col99)))
                    + ((op0_limb_5_col49) * (op1_limb_25_col98)))
                    + ((op0_limb_6_col50) * (op1_limb_24_col97)))
                    + ((op0_limb_7_col51) * (op1_limb_23_col96)))
                    + ((op0_limb_8_col52) * (op1_limb_22_col95)))
                    + ((op0_limb_9_col53) * (op1_limb_21_col94)))
                    + ((op0_limb_10_col54) * (op1_limb_20_col93)))
                    + ((op0_limb_11_col55) * (op1_limb_19_col92)))
                    + ((op0_limb_12_col56) * (op1_limb_18_col91)))
                    + ((op0_limb_13_col57) * (op1_limb_17_col90)))
                    + ((op0_limb_14_col58) * (op1_limb_16_col89)))
                    + ((op0_limb_15_col59) * (op1_limb_15_col88)))
                    + ((op0_limb_16_col60) * (op1_limb_14_col87)))
                    + ((op0_limb_17_col61) * (op1_limb_13_col86)))
                    + ((op0_limb_18_col62) * (op1_limb_12_col85)))
                    + ((op0_limb_19_col63) * (op1_limb_11_col84)))
                    + ((op0_limb_20_col64) * (op1_limb_10_col83)))
                    + ((op0_limb_21_col65) * (op1_limb_9_col82)))
                    + ((op0_limb_22_col66) * (op1_limb_8_col81)))
                    + ((op0_limb_23_col67) * (op1_limb_7_col80)))
                    + ((op0_limb_24_col68) * (op1_limb_6_col79)))
                    + ((op0_limb_25_col69) * (op1_limb_5_col78)))
                    + ((op0_limb_26_col70) * (op1_limb_4_col77)))
                    + ((op0_limb_27_col71) * (op1_limb_3_col76)));
                let conv_tmp_5a147_48 = (((((((((((((((((((((((((M31_0)
                    + ((op0_limb_4_col48) * (op1_limb_27_col100)))
                    + ((op0_limb_5_col49) * (op1_limb_26_col99)))
                    + ((op0_limb_6_col50) * (op1_limb_25_col98)))
                    + ((op0_limb_7_col51) * (op1_limb_24_col97)))
                    + ((op0_limb_8_col52) * (op1_limb_23_col96)))
                    + ((op0_limb_9_col53) * (op1_limb_22_col95)))
                    + ((op0_limb_10_col54) * (op1_limb_21_col94)))
                    + ((op0_limb_11_col55) * (op1_limb_20_col93)))
                    + ((op0_limb_12_col56) * (op1_limb_19_col92)))
                    + ((op0_limb_13_col57) * (op1_limb_18_col91)))
                    + ((op0_limb_14_col58) * (op1_limb_17_col90)))
                    + ((op0_limb_15_col59) * (op1_limb_16_col89)))
                    + ((op0_limb_16_col60) * (op1_limb_15_col88)))
                    + ((op0_limb_17_col61) * (op1_limb_14_col87)))
                    + ((op0_limb_18_col62) * (op1_limb_13_col86)))
                    + ((op0_limb_19_col63) * (op1_limb_12_col85)))
                    + ((op0_limb_20_col64) * (op1_limb_11_col84)))
                    + ((op0_limb_21_col65) * (op1_limb_10_col83)))
                    + ((op0_limb_22_col66) * (op1_limb_9_col82)))
                    + ((op0_limb_23_col67) * (op1_limb_8_col81)))
                    + ((op0_limb_24_col68) * (op1_limb_7_col80)))
                    + ((op0_limb_25_col69) * (op1_limb_6_col79)))
                    + ((op0_limb_26_col70) * (op1_limb_5_col78)))
                    + ((op0_limb_27_col71) * (op1_limb_4_col77)));
                let conv_tmp_5a147_49 = ((((((((((((((((((((((((M31_0)
                    + ((op0_limb_5_col49) * (op1_limb_27_col100)))
                    + ((op0_limb_6_col50) * (op1_limb_26_col99)))
                    + ((op0_limb_7_col51) * (op1_limb_25_col98)))
                    + ((op0_limb_8_col52) * (op1_limb_24_col97)))
                    + ((op0_limb_9_col53) * (op1_limb_23_col96)))
                    + ((op0_limb_10_col54) * (op1_limb_22_col95)))
                    + ((op0_limb_11_col55) * (op1_limb_21_col94)))
                    + ((op0_limb_12_col56) * (op1_limb_20_col93)))
                    + ((op0_limb_13_col57) * (op1_limb_19_col92)))
                    + ((op0_limb_14_col58) * (op1_limb_18_col91)))
                    + ((op0_limb_15_col59) * (op1_limb_17_col90)))
                    + ((op0_limb_16_col60) * (op1_limb_16_col89)))
                    + ((op0_limb_17_col61) * (op1_limb_15_col88)))
                    + ((op0_limb_18_col62) * (op1_limb_14_col87)))
                    + ((op0_limb_19_col63) * (op1_limb_13_col86)))
                    + ((op0_limb_20_col64) * (op1_limb_12_col85)))
                    + ((op0_limb_21_col65) * (op1_limb_11_col84)))
                    + ((op0_limb_22_col66) * (op1_limb_10_col83)))
                    + ((op0_limb_23_col67) * (op1_limb_9_col82)))
                    + ((op0_limb_24_col68) * (op1_limb_8_col81)))
                    + ((op0_limb_25_col69) * (op1_limb_7_col80)))
                    + ((op0_limb_26_col70) * (op1_limb_6_col79)))
                    + ((op0_limb_27_col71) * (op1_limb_5_col78)));
                let conv_tmp_5a147_50 = (((((((((((((((((((((((M31_0)
                    + ((op0_limb_6_col50) * (op1_limb_27_col100)))
                    + ((op0_limb_7_col51) * (op1_limb_26_col99)))
                    + ((op0_limb_8_col52) * (op1_limb_25_col98)))
                    + ((op0_limb_9_col53) * (op1_limb_24_col97)))
                    + ((op0_limb_10_col54) * (op1_limb_23_col96)))
                    + ((op0_limb_11_col55) * (op1_limb_22_col95)))
                    + ((op0_limb_12_col56) * (op1_limb_21_col94)))
                    + ((op0_limb_13_col57) * (op1_limb_20_col93)))
                    + ((op0_limb_14_col58) * (op1_limb_19_col92)))
                    + ((op0_limb_15_col59) * (op1_limb_18_col91)))
                    + ((op0_limb_16_col60) * (op1_limb_17_col90)))
                    + ((op0_limb_17_col61) * (op1_limb_16_col89)))
                    + ((op0_limb_18_col62) * (op1_limb_15_col88)))
                    + ((op0_limb_19_col63) * (op1_limb_14_col87)))
                    + ((op0_limb_20_col64) * (op1_limb_13_col86)))
                    + ((op0_limb_21_col65) * (op1_limb_12_col85)))
                    + ((op0_limb_22_col66) * (op1_limb_11_col84)))
                    + ((op0_limb_23_col67) * (op1_limb_10_col83)))
                    + ((op0_limb_24_col68) * (op1_limb_9_col82)))
                    + ((op0_limb_25_col69) * (op1_limb_8_col81)))
                    + ((op0_limb_26_col70) * (op1_limb_7_col80)))
                    + ((op0_limb_27_col71) * (op1_limb_6_col79)));
                let conv_tmp_5a147_51 = ((((((((((((((((((((((M31_0)
                    + ((op0_limb_7_col51) * (op1_limb_27_col100)))
                    + ((op0_limb_8_col52) * (op1_limb_26_col99)))
                    + ((op0_limb_9_col53) * (op1_limb_25_col98)))
                    + ((op0_limb_10_col54) * (op1_limb_24_col97)))
                    + ((op0_limb_11_col55) * (op1_limb_23_col96)))
                    + ((op0_limb_12_col56) * (op1_limb_22_col95)))
                    + ((op0_limb_13_col57) * (op1_limb_21_col94)))
                    + ((op0_limb_14_col58) * (op1_limb_20_col93)))
                    + ((op0_limb_15_col59) * (op1_limb_19_col92)))
                    + ((op0_limb_16_col60) * (op1_limb_18_col91)))
                    + ((op0_limb_17_col61) * (op1_limb_17_col90)))
                    + ((op0_limb_18_col62) * (op1_limb_16_col89)))
                    + ((op0_limb_19_col63) * (op1_limb_15_col88)))
                    + ((op0_limb_20_col64) * (op1_limb_14_col87)))
                    + ((op0_limb_21_col65) * (op1_limb_13_col86)))
                    + ((op0_limb_22_col66) * (op1_limb_12_col85)))
                    + ((op0_limb_23_col67) * (op1_limb_11_col84)))
                    + ((op0_limb_24_col68) * (op1_limb_10_col83)))
                    + ((op0_limb_25_col69) * (op1_limb_9_col82)))
                    + ((op0_limb_26_col70) * (op1_limb_8_col81)))
                    + ((op0_limb_27_col71) * (op1_limb_7_col80)));
                let conv_tmp_5a147_52 = (((((((((((((((((((((M31_0)
                    + ((op0_limb_8_col52) * (op1_limb_27_col100)))
                    + ((op0_limb_9_col53) * (op1_limb_26_col99)))
                    + ((op0_limb_10_col54) * (op1_limb_25_col98)))
                    + ((op0_limb_11_col55) * (op1_limb_24_col97)))
                    + ((op0_limb_12_col56) * (op1_limb_23_col96)))
                    + ((op0_limb_13_col57) * (op1_limb_22_col95)))
                    + ((op0_limb_14_col58) * (op1_limb_21_col94)))
                    + ((op0_limb_15_col59) * (op1_limb_20_col93)))
                    + ((op0_limb_16_col60) * (op1_limb_19_col92)))
                    + ((op0_limb_17_col61) * (op1_limb_18_col91)))
                    + ((op0_limb_18_col62) * (op1_limb_17_col90)))
                    + ((op0_limb_19_col63) * (op1_limb_16_col89)))
                    + ((op0_limb_20_col64) * (op1_limb_15_col88)))
                    + ((op0_limb_21_col65) * (op1_limb_14_col87)))
                    + ((op0_limb_22_col66) * (op1_limb_13_col86)))
                    + ((op0_limb_23_col67) * (op1_limb_12_col85)))
                    + ((op0_limb_24_col68) * (op1_limb_11_col84)))
                    + ((op0_limb_25_col69) * (op1_limb_10_col83)))
                    + ((op0_limb_26_col70) * (op1_limb_9_col82)))
                    + ((op0_limb_27_col71) * (op1_limb_8_col81)));
                let conv_tmp_5a147_53 = ((((((((((((((((((((M31_0)
                    + ((op0_limb_9_col53) * (op1_limb_27_col100)))
                    + ((op0_limb_10_col54) * (op1_limb_26_col99)))
                    + ((op0_limb_11_col55) * (op1_limb_25_col98)))
                    + ((op0_limb_12_col56) * (op1_limb_24_col97)))
                    + ((op0_limb_13_col57) * (op1_limb_23_col96)))
                    + ((op0_limb_14_col58) * (op1_limb_22_col95)))
                    + ((op0_limb_15_col59) * (op1_limb_21_col94)))
                    + ((op0_limb_16_col60) * (op1_limb_20_col93)))
                    + ((op0_limb_17_col61) * (op1_limb_19_col92)))
                    + ((op0_limb_18_col62) * (op1_limb_18_col91)))
                    + ((op0_limb_19_col63) * (op1_limb_17_col90)))
                    + ((op0_limb_20_col64) * (op1_limb_16_col89)))
                    + ((op0_limb_21_col65) * (op1_limb_15_col88)))
                    + ((op0_limb_22_col66) * (op1_limb_14_col87)))
                    + ((op0_limb_23_col67) * (op1_limb_13_col86)))
                    + ((op0_limb_24_col68) * (op1_limb_12_col85)))
                    + ((op0_limb_25_col69) * (op1_limb_11_col84)))
                    + ((op0_limb_26_col70) * (op1_limb_10_col83)))
                    + ((op0_limb_27_col71) * (op1_limb_9_col82)));
                let conv_tmp_5a147_54 = (((((((((((((((((((M31_0)
                    + ((op0_limb_10_col54) * (op1_limb_27_col100)))
                    + ((op0_limb_11_col55) * (op1_limb_26_col99)))
                    + ((op0_limb_12_col56) * (op1_limb_25_col98)))
                    + ((op0_limb_13_col57) * (op1_limb_24_col97)))
                    + ((op0_limb_14_col58) * (op1_limb_23_col96)))
                    + ((op0_limb_15_col59) * (op1_limb_22_col95)))
                    + ((op0_limb_16_col60) * (op1_limb_21_col94)))
                    + ((op0_limb_17_col61) * (op1_limb_20_col93)))
                    + ((op0_limb_18_col62) * (op1_limb_19_col92)))
                    + ((op0_limb_19_col63) * (op1_limb_18_col91)))
                    + ((op0_limb_20_col64) * (op1_limb_17_col90)))
                    + ((op0_limb_21_col65) * (op1_limb_16_col89)))
                    + ((op0_limb_22_col66) * (op1_limb_15_col88)))
                    + ((op0_limb_23_col67) * (op1_limb_14_col87)))
                    + ((op0_limb_24_col68) * (op1_limb_13_col86)))
                    + ((op0_limb_25_col69) * (op1_limb_12_col85)))
                    + ((op0_limb_26_col70) * (op1_limb_11_col84)))
                    + ((op0_limb_27_col71) * (op1_limb_10_col83)));
                let conv_tmp_5a147_55 = ((((((((((((((((((M31_0)
                    + ((op0_limb_11_col55) * (op1_limb_27_col100)))
                    + ((op0_limb_12_col56) * (op1_limb_26_col99)))
                    + ((op0_limb_13_col57) * (op1_limb_25_col98)))
                    + ((op0_limb_14_col58) * (op1_limb_24_col97)))
                    + ((op0_limb_15_col59) * (op1_limb_23_col96)))
                    + ((op0_limb_16_col60) * (op1_limb_22_col95)))
                    + ((op0_limb_17_col61) * (op1_limb_21_col94)))
                    + ((op0_limb_18_col62) * (op1_limb_20_col93)))
                    + ((op0_limb_19_col63) * (op1_limb_19_col92)))
                    + ((op0_limb_20_col64) * (op1_limb_18_col91)))
                    + ((op0_limb_21_col65) * (op1_limb_17_col90)))
                    + ((op0_limb_22_col66) * (op1_limb_16_col89)))
                    + ((op0_limb_23_col67) * (op1_limb_15_col88)))
                    + ((op0_limb_24_col68) * (op1_limb_14_col87)))
                    + ((op0_limb_25_col69) * (op1_limb_13_col86)))
                    + ((op0_limb_26_col70) * (op1_limb_12_col85)))
                    + ((op0_limb_27_col71) * (op1_limb_11_col84)));
                let conv_tmp_5a147_56 = (((((((((((((((((M31_0)
                    + ((op0_limb_12_col56) * (op1_limb_27_col100)))
                    + ((op0_limb_13_col57) * (op1_limb_26_col99)))
                    + ((op0_limb_14_col58) * (op1_limb_25_col98)))
                    + ((op0_limb_15_col59) * (op1_limb_24_col97)))
                    + ((op0_limb_16_col60) * (op1_limb_23_col96)))
                    + ((op0_limb_17_col61) * (op1_limb_22_col95)))
                    + ((op0_limb_18_col62) * (op1_limb_21_col94)))
                    + ((op0_limb_19_col63) * (op1_limb_20_col93)))
                    + ((op0_limb_20_col64) * (op1_limb_19_col92)))
                    + ((op0_limb_21_col65) * (op1_limb_18_col91)))
                    + ((op0_limb_22_col66) * (op1_limb_17_col90)))
                    + ((op0_limb_23_col67) * (op1_limb_16_col89)))
                    + ((op0_limb_24_col68) * (op1_limb_15_col88)))
                    + ((op0_limb_25_col69) * (op1_limb_14_col87)))
                    + ((op0_limb_26_col70) * (op1_limb_13_col86)))
                    + ((op0_limb_27_col71) * (op1_limb_12_col85)));
                let conv_tmp_5a147_57 = ((((((((((((((((M31_0)
                    + ((op0_limb_13_col57) * (op1_limb_27_col100)))
                    + ((op0_limb_14_col58) * (op1_limb_26_col99)))
                    + ((op0_limb_15_col59) * (op1_limb_25_col98)))
                    + ((op0_limb_16_col60) * (op1_limb_24_col97)))
                    + ((op0_limb_17_col61) * (op1_limb_23_col96)))
                    + ((op0_limb_18_col62) * (op1_limb_22_col95)))
                    + ((op0_limb_19_col63) * (op1_limb_21_col94)))
                    + ((op0_limb_20_col64) * (op1_limb_20_col93)))
                    + ((op0_limb_21_col65) * (op1_limb_19_col92)))
                    + ((op0_limb_22_col66) * (op1_limb_18_col91)))
                    + ((op0_limb_23_col67) * (op1_limb_17_col90)))
                    + ((op0_limb_24_col68) * (op1_limb_16_col89)))
                    + ((op0_limb_25_col69) * (op1_limb_15_col88)))
                    + ((op0_limb_26_col70) * (op1_limb_14_col87)))
                    + ((op0_limb_27_col71) * (op1_limb_13_col86)));
                let conv_tmp_5a147_58 = (((((((((((((((M31_0)
                    + ((op0_limb_14_col58) * (op1_limb_27_col100)))
                    + ((op0_limb_15_col59) * (op1_limb_26_col99)))
                    + ((op0_limb_16_col60) * (op1_limb_25_col98)))
                    + ((op0_limb_17_col61) * (op1_limb_24_col97)))
                    + ((op0_limb_18_col62) * (op1_limb_23_col96)))
                    + ((op0_limb_19_col63) * (op1_limb_22_col95)))
                    + ((op0_limb_20_col64) * (op1_limb_21_col94)))
                    + ((op0_limb_21_col65) * (op1_limb_20_col93)))
                    + ((op0_limb_22_col66) * (op1_limb_19_col92)))
                    + ((op0_limb_23_col67) * (op1_limb_18_col91)))
                    + ((op0_limb_24_col68) * (op1_limb_17_col90)))
                    + ((op0_limb_25_col69) * (op1_limb_16_col89)))
                    + ((op0_limb_26_col70) * (op1_limb_15_col88)))
                    + ((op0_limb_27_col71) * (op1_limb_14_col87)));
                let conv_tmp_5a147_59 = ((((((((((((((M31_0)
                    + ((op0_limb_15_col59) * (op1_limb_27_col100)))
                    + ((op0_limb_16_col60) * (op1_limb_26_col99)))
                    + ((op0_limb_17_col61) * (op1_limb_25_col98)))
                    + ((op0_limb_18_col62) * (op1_limb_24_col97)))
                    + ((op0_limb_19_col63) * (op1_limb_23_col96)))
                    + ((op0_limb_20_col64) * (op1_limb_22_col95)))
                    + ((op0_limb_21_col65) * (op1_limb_21_col94)))
                    + ((op0_limb_22_col66) * (op1_limb_20_col93)))
                    + ((op0_limb_23_col67) * (op1_limb_19_col92)))
                    + ((op0_limb_24_col68) * (op1_limb_18_col91)))
                    + ((op0_limb_25_col69) * (op1_limb_17_col90)))
                    + ((op0_limb_26_col70) * (op1_limb_16_col89)))
                    + ((op0_limb_27_col71) * (op1_limb_15_col88)));
                let conv_tmp_5a147_60 = (((((((((((((M31_0)
                    + ((op0_limb_16_col60) * (op1_limb_27_col100)))
                    + ((op0_limb_17_col61) * (op1_limb_26_col99)))
                    + ((op0_limb_18_col62) * (op1_limb_25_col98)))
                    + ((op0_limb_19_col63) * (op1_limb_24_col97)))
                    + ((op0_limb_20_col64) * (op1_limb_23_col96)))
                    + ((op0_limb_21_col65) * (op1_limb_22_col95)))
                    + ((op0_limb_22_col66) * (op1_limb_21_col94)))
                    + ((op0_limb_23_col67) * (op1_limb_20_col93)))
                    + ((op0_limb_24_col68) * (op1_limb_19_col92)))
                    + ((op0_limb_25_col69) * (op1_limb_18_col91)))
                    + ((op0_limb_26_col70) * (op1_limb_17_col90)))
                    + ((op0_limb_27_col71) * (op1_limb_16_col89)));
                let conv_tmp_5a147_61 = ((((((((((((M31_0)
                    + ((op0_limb_17_col61) * (op1_limb_27_col100)))
                    + ((op0_limb_18_col62) * (op1_limb_26_col99)))
                    + ((op0_limb_19_col63) * (op1_limb_25_col98)))
                    + ((op0_limb_20_col64) * (op1_limb_24_col97)))
                    + ((op0_limb_21_col65) * (op1_limb_23_col96)))
                    + ((op0_limb_22_col66) * (op1_limb_22_col95)))
                    + ((op0_limb_23_col67) * (op1_limb_21_col94)))
                    + ((op0_limb_24_col68) * (op1_limb_20_col93)))
                    + ((op0_limb_25_col69) * (op1_limb_19_col92)))
                    + ((op0_limb_26_col70) * (op1_limb_18_col91)))
                    + ((op0_limb_27_col71) * (op1_limb_17_col90)));
                let conv_tmp_5a147_62 = (((((((((((M31_0)
                    + ((op0_limb_18_col62) * (op1_limb_27_col100)))
                    + ((op0_limb_19_col63) * (op1_limb_26_col99)))
                    + ((op0_limb_20_col64) * (op1_limb_25_col98)))
                    + ((op0_limb_21_col65) * (op1_limb_24_col97)))
                    + ((op0_limb_22_col66) * (op1_limb_23_col96)))
                    + ((op0_limb_23_col67) * (op1_limb_22_col95)))
                    + ((op0_limb_24_col68) * (op1_limb_21_col94)))
                    + ((op0_limb_25_col69) * (op1_limb_20_col93)))
                    + ((op0_limb_26_col70) * (op1_limb_19_col92)))
                    + ((op0_limb_27_col71) * (op1_limb_18_col91)));
                let conv_tmp_5a147_63 = ((((((((((M31_0)
                    + ((op0_limb_19_col63) * (op1_limb_27_col100)))
                    + ((op0_limb_20_col64) * (op1_limb_26_col99)))
                    + ((op0_limb_21_col65) * (op1_limb_25_col98)))
                    + ((op0_limb_22_col66) * (op1_limb_24_col97)))
                    + ((op0_limb_23_col67) * (op1_limb_23_col96)))
                    + ((op0_limb_24_col68) * (op1_limb_22_col95)))
                    + ((op0_limb_25_col69) * (op1_limb_21_col94)))
                    + ((op0_limb_26_col70) * (op1_limb_20_col93)))
                    + ((op0_limb_27_col71) * (op1_limb_19_col92)));
                let conv_tmp_5a147_64 = (((((((((M31_0)
                    + ((op0_limb_20_col64) * (op1_limb_27_col100)))
                    + ((op0_limb_21_col65) * (op1_limb_26_col99)))
                    + ((op0_limb_22_col66) * (op1_limb_25_col98)))
                    + ((op0_limb_23_col67) * (op1_limb_24_col97)))
                    + ((op0_limb_24_col68) * (op1_limb_23_col96)))
                    + ((op0_limb_25_col69) * (op1_limb_22_col95)))
                    + ((op0_limb_26_col70) * (op1_limb_21_col94)))
                    + ((op0_limb_27_col71) * (op1_limb_20_col93)));
                let conv_tmp_5a147_65 = ((((((((M31_0)
                    + ((op0_limb_21_col65) * (op1_limb_27_col100)))
                    + ((op0_limb_22_col66) * (op1_limb_26_col99)))
                    + ((op0_limb_23_col67) * (op1_limb_25_col98)))
                    + ((op0_limb_24_col68) * (op1_limb_24_col97)))
                    + ((op0_limb_25_col69) * (op1_limb_23_col96)))
                    + ((op0_limb_26_col70) * (op1_limb_22_col95)))
                    + ((op0_limb_27_col71) * (op1_limb_21_col94)));
                let conv_tmp_5a147_66 = (((((((M31_0)
                    + ((op0_limb_22_col66) * (op1_limb_27_col100)))
                    + ((op0_limb_23_col67) * (op1_limb_26_col99)))
                    + ((op0_limb_24_col68) * (op1_limb_25_col98)))
                    + ((op0_limb_25_col69) * (op1_limb_24_col97)))
                    + ((op0_limb_26_col70) * (op1_limb_23_col96)))
                    + ((op0_limb_27_col71) * (op1_limb_22_col95)));
                let conv_tmp_5a147_67 = ((((((M31_0)
                    + ((op0_limb_23_col67) * (op1_limb_27_col100)))
                    + ((op0_limb_24_col68) * (op1_limb_26_col99)))
                    + ((op0_limb_25_col69) * (op1_limb_25_col98)))
                    + ((op0_limb_26_col70) * (op1_limb_24_col97)))
                    + ((op0_limb_27_col71) * (op1_limb_23_col96)));
                let conv_tmp_5a147_68 = (((((M31_0)
                    + ((op0_limb_24_col68) * (op1_limb_27_col100)))
                    + ((op0_limb_25_col69) * (op1_limb_26_col99)))
                    + ((op0_limb_26_col70) * (op1_limb_25_col98)))
                    + ((op0_limb_27_col71) * (op1_limb_24_col97)));
                let conv_tmp_5a147_69 = ((((M31_0)
                    + ((op0_limb_25_col69) * (op1_limb_27_col100)))
                    + ((op0_limb_26_col70) * (op1_limb_26_col99)))
                    + ((op0_limb_27_col71) * (op1_limb_25_col98)));
                let conv_tmp_5a147_70 = (((M31_0) + ((op0_limb_26_col70) * (op1_limb_27_col100)))
                    + ((op0_limb_27_col71) * (op1_limb_26_col99)));
                let conv_tmp_5a147_71 = ((M31_0) + ((op0_limb_27_col71) * (op1_limb_27_col100)));
                let conv_mod_tmp_5a147_72 = ((((M31_0) + ((M31_32) * (conv_tmp_5a147_17)))
                    - ((M31_4) * (conv_tmp_5a147_38)))
                    + ((M31_8) * (conv_tmp_5a147_66)));
                let conv_mod_tmp_5a147_73 = (((((M31_0) + ((M31_1) * (conv_tmp_5a147_17)))
                    + ((M31_32) * (conv_tmp_5a147_18)))
                    - ((M31_4) * (conv_tmp_5a147_39)))
                    + ((M31_8) * (conv_tmp_5a147_67)));
                let conv_mod_tmp_5a147_74 = (((((M31_0) + ((M31_1) * (conv_tmp_5a147_18)))
                    + ((M31_32) * (conv_tmp_5a147_19)))
                    - ((M31_4) * (conv_tmp_5a147_40)))
                    + ((M31_8) * (conv_tmp_5a147_68)));
                let conv_mod_tmp_5a147_75 = (((((M31_0) + ((M31_1) * (conv_tmp_5a147_19)))
                    + ((M31_32) * (conv_tmp_5a147_20)))
                    - ((M31_4) * (conv_tmp_5a147_41)))
                    + ((M31_8) * (conv_tmp_5a147_69)));
                let conv_mod_tmp_5a147_76 = (((((M31_0) + ((M31_1) * (conv_tmp_5a147_20)))
                    + ((M31_32) * (conv_tmp_5a147_21)))
                    - ((M31_4) * (conv_tmp_5a147_42)))
                    + ((M31_8) * (conv_tmp_5a147_70)));
                let conv_mod_tmp_5a147_77 = (((((M31_0) + ((M31_1) * (conv_tmp_5a147_21)))
                    + ((M31_32) * (conv_tmp_5a147_22)))
                    - ((M31_4) * (conv_tmp_5a147_43)))
                    + ((M31_8) * (conv_tmp_5a147_71)));
                let conv_mod_tmp_5a147_78 = ((((M31_0) + ((M31_1) * (conv_tmp_5a147_22)))
                    + ((M31_32) * (conv_tmp_5a147_23)))
                    - ((M31_4) * (conv_tmp_5a147_44)));
                let conv_mod_tmp_5a147_79 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_17)))
                    + ((M31_1) * (conv_tmp_5a147_23)))
                    + ((M31_32) * (conv_tmp_5a147_24)))
                    - ((M31_4) * (conv_tmp_5a147_45)));
                let conv_mod_tmp_5a147_80 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_18)))
                    + ((M31_1) * (conv_tmp_5a147_24)))
                    + ((M31_32) * (conv_tmp_5a147_25)))
                    - ((M31_4) * (conv_tmp_5a147_46)));
                let conv_mod_tmp_5a147_81 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_19)))
                    + ((M31_1) * (conv_tmp_5a147_25)))
                    + ((M31_32) * (conv_tmp_5a147_26)))
                    - ((M31_4) * (conv_tmp_5a147_47)));
                let conv_mod_tmp_5a147_82 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_20)))
                    + ((M31_1) * (conv_tmp_5a147_26)))
                    + ((M31_32) * (conv_tmp_5a147_27)))
                    - ((M31_4) * (conv_tmp_5a147_48)));
                let conv_mod_tmp_5a147_83 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_21)))
                    + ((M31_1) * (conv_tmp_5a147_27)))
                    + ((M31_32) * (conv_tmp_5a147_28)))
                    - ((M31_4) * (conv_tmp_5a147_49)));
                let conv_mod_tmp_5a147_84 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_22)))
                    + ((M31_1) * (conv_tmp_5a147_28)))
                    + ((M31_32) * (conv_tmp_5a147_29)))
                    - ((M31_4) * (conv_tmp_5a147_50)));
                let conv_mod_tmp_5a147_85 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_23)))
                    + ((M31_1) * (conv_tmp_5a147_29)))
                    + ((M31_32) * (conv_tmp_5a147_30)))
                    - ((M31_4) * (conv_tmp_5a147_51)));
                let conv_mod_tmp_5a147_86 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_24)))
                    + ((M31_1) * (conv_tmp_5a147_30)))
                    + ((M31_32) * (conv_tmp_5a147_31)))
                    - ((M31_4) * (conv_tmp_5a147_52)));
                let conv_mod_tmp_5a147_87 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_25)))
                    + ((M31_1) * (conv_tmp_5a147_31)))
                    + ((M31_32) * (conv_tmp_5a147_32)))
                    - ((M31_4) * (conv_tmp_5a147_53)));
                let conv_mod_tmp_5a147_88 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_26)))
                    + ((M31_1) * (conv_tmp_5a147_32)))
                    + ((M31_32) * (conv_tmp_5a147_33)))
                    - ((M31_4) * (conv_tmp_5a147_54)));
                let conv_mod_tmp_5a147_89 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_27)))
                    + ((M31_1) * (conv_tmp_5a147_33)))
                    + ((M31_32) * (conv_tmp_5a147_34)))
                    - ((M31_4) * (conv_tmp_5a147_55)));
                let conv_mod_tmp_5a147_90 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_28)))
                    + ((M31_1) * (conv_tmp_5a147_34)))
                    + ((M31_32) * (conv_tmp_5a147_35)))
                    - ((M31_4) * (conv_tmp_5a147_56)));
                let conv_mod_tmp_5a147_91 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_29)))
                    + ((M31_1) * (conv_tmp_5a147_35)))
                    + ((M31_32) * (conv_tmp_5a147_36)))
                    - ((M31_4) * (conv_tmp_5a147_57)));
                let conv_mod_tmp_5a147_92 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_30)))
                    + ((M31_1) * (conv_tmp_5a147_36)))
                    + ((M31_32) * (conv_tmp_5a147_37)))
                    - ((M31_4) * (conv_tmp_5a147_58)));
                let conv_mod_tmp_5a147_93 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_31)))
                    + ((M31_1) * (conv_tmp_5a147_37)))
                    - ((M31_4) * (conv_tmp_5a147_59)))
                    + ((M31_64) * (conv_tmp_5a147_66)));
                let conv_mod_tmp_5a147_94 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_32)))
                    - ((M31_4) * (conv_tmp_5a147_60)))
                    + ((M31_2) * (conv_tmp_5a147_66)))
                    + ((M31_64) * (conv_tmp_5a147_67)));
                let conv_mod_tmp_5a147_95 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_33)))
                    - ((M31_4) * (conv_tmp_5a147_61)))
                    + ((M31_2) * (conv_tmp_5a147_67)))
                    + ((M31_64) * (conv_tmp_5a147_68)));
                let conv_mod_tmp_5a147_96 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_34)))
                    - ((M31_4) * (conv_tmp_5a147_62)))
                    + ((M31_2) * (conv_tmp_5a147_68)))
                    + ((M31_64) * (conv_tmp_5a147_69)));
                let conv_mod_tmp_5a147_97 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_35)))
                    - ((M31_4) * (conv_tmp_5a147_63)))
                    + ((M31_2) * (conv_tmp_5a147_69)))
                    + ((M31_64) * (conv_tmp_5a147_70)));
                let conv_mod_tmp_5a147_98 = (((((M31_0) + ((M31_2) * (conv_tmp_5a147_36)))
                    - ((M31_4) * (conv_tmp_5a147_64)))
                    + ((M31_2) * (conv_tmp_5a147_70)))
                    + ((M31_64) * (conv_tmp_5a147_71)));
                let conv_mod_tmp_5a147_99 = ((((M31_0) + ((M31_2) * (conv_tmp_5a147_37)))
                    - ((M31_4) * (conv_tmp_5a147_65)))
                    + ((M31_2) * (conv_tmp_5a147_71)));
                let k_mod_2_18_biased_tmp_5a147_100 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_5a147_72) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_5a147_73) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col101 = ((k_mod_2_18_biased_tmp_5a147_100.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_5a147_100.high().as_m31()) - (M31_1))
                        * (M31_65536)));
                *row[101] = k_col101;

                for (i, &input) in [((k_col101) + (M31_262144))].unpack().iter().enumerate() {
                    *sub_components_inputs[i].range_check_19_inputs[0] = input;
                }
                *lookup_data.range_check_19_0 = [((k_col101) + (M31_262144))];
                let carry_0_col102 = ((((conv_mod_tmp_5a147_72) - ((M31_1) * (k_col101)))
                    + (M31_0))
                    * (M31_4194304));
                *row[102] = carry_0_col102;

                for (i, &input) in [((carry_0_col102) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[1] = input;
                }
                *lookup_data.range_check_19_1 = [((carry_0_col102) + (M31_131072))];
                let carry_1_col103 = (((conv_mod_tmp_5a147_73) + (carry_0_col102)) * (M31_4194304));
                *row[103] = carry_1_col103;

                for (i, &input) in [((carry_1_col103) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[2] = input;
                }
                *lookup_data.range_check_19_2 = [((carry_1_col103) + (M31_131072))];
                let carry_2_col104 = (((conv_mod_tmp_5a147_74) + (carry_1_col103)) * (M31_4194304));
                *row[104] = carry_2_col104;

                for (i, &input) in [((carry_2_col104) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[3] = input;
                }
                *lookup_data.range_check_19_3 = [((carry_2_col104) + (M31_131072))];
                let carry_3_col105 = (((conv_mod_tmp_5a147_75) + (carry_2_col104)) * (M31_4194304));
                *row[105] = carry_3_col105;

                for (i, &input) in [((carry_3_col105) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[4] = input;
                }
                *lookup_data.range_check_19_4 = [((carry_3_col105) + (M31_131072))];
                let carry_4_col106 = (((conv_mod_tmp_5a147_76) + (carry_3_col105)) * (M31_4194304));
                *row[106] = carry_4_col106;

                for (i, &input) in [((carry_4_col106) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[5] = input;
                }
                *lookup_data.range_check_19_5 = [((carry_4_col106) + (M31_131072))];
                let carry_5_col107 = (((conv_mod_tmp_5a147_77) + (carry_4_col106)) * (M31_4194304));
                *row[107] = carry_5_col107;

                for (i, &input) in [((carry_5_col107) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[6] = input;
                }
                *lookup_data.range_check_19_6 = [((carry_5_col107) + (M31_131072))];
                let carry_6_col108 = (((conv_mod_tmp_5a147_78) + (carry_5_col107)) * (M31_4194304));
                *row[108] = carry_6_col108;

                for (i, &input) in [((carry_6_col108) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[7] = input;
                }
                *lookup_data.range_check_19_7 = [((carry_6_col108) + (M31_131072))];
                let carry_7_col109 = (((conv_mod_tmp_5a147_79) + (carry_6_col108)) * (M31_4194304));
                *row[109] = carry_7_col109;

                for (i, &input) in [((carry_7_col109) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[8] = input;
                }
                *lookup_data.range_check_19_8 = [((carry_7_col109) + (M31_131072))];
                let carry_8_col110 = (((conv_mod_tmp_5a147_80) + (carry_7_col109)) * (M31_4194304));
                *row[110] = carry_8_col110;

                for (i, &input) in [((carry_8_col110) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[9] = input;
                }
                *lookup_data.range_check_19_9 = [((carry_8_col110) + (M31_131072))];
                let carry_9_col111 = (((conv_mod_tmp_5a147_81) + (carry_8_col110)) * (M31_4194304));
                *row[111] = carry_9_col111;

                for (i, &input) in [((carry_9_col111) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[10] = input;
                }
                *lookup_data.range_check_19_10 = [((carry_9_col111) + (M31_131072))];
                let carry_10_col112 =
                    (((conv_mod_tmp_5a147_82) + (carry_9_col111)) * (M31_4194304));
                *row[112] = carry_10_col112;

                for (i, &input) in [((carry_10_col112) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[11] = input;
                }
                *lookup_data.range_check_19_11 = [((carry_10_col112) + (M31_131072))];
                let carry_11_col113 =
                    (((conv_mod_tmp_5a147_83) + (carry_10_col112)) * (M31_4194304));
                *row[113] = carry_11_col113;

                for (i, &input) in [((carry_11_col113) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[12] = input;
                }
                *lookup_data.range_check_19_12 = [((carry_11_col113) + (M31_131072))];
                let carry_12_col114 =
                    (((conv_mod_tmp_5a147_84) + (carry_11_col113)) * (M31_4194304));
                *row[114] = carry_12_col114;

                for (i, &input) in [((carry_12_col114) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[13] = input;
                }
                *lookup_data.range_check_19_13 = [((carry_12_col114) + (M31_131072))];
                let carry_13_col115 =
                    (((conv_mod_tmp_5a147_85) + (carry_12_col114)) * (M31_4194304));
                *row[115] = carry_13_col115;

                for (i, &input) in [((carry_13_col115) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[14] = input;
                }
                *lookup_data.range_check_19_14 = [((carry_13_col115) + (M31_131072))];
                let carry_14_col116 =
                    (((conv_mod_tmp_5a147_86) + (carry_13_col115)) * (M31_4194304));
                *row[116] = carry_14_col116;

                for (i, &input) in [((carry_14_col116) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[15] = input;
                }
                *lookup_data.range_check_19_15 = [((carry_14_col116) + (M31_131072))];
                let carry_15_col117 =
                    (((conv_mod_tmp_5a147_87) + (carry_14_col116)) * (M31_4194304));
                *row[117] = carry_15_col117;

                for (i, &input) in [((carry_15_col117) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[16] = input;
                }
                *lookup_data.range_check_19_16 = [((carry_15_col117) + (M31_131072))];
                let carry_16_col118 =
                    (((conv_mod_tmp_5a147_88) + (carry_15_col117)) * (M31_4194304));
                *row[118] = carry_16_col118;

                for (i, &input) in [((carry_16_col118) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[17] = input;
                }
                *lookup_data.range_check_19_17 = [((carry_16_col118) + (M31_131072))];
                let carry_17_col119 =
                    (((conv_mod_tmp_5a147_89) + (carry_16_col118)) * (M31_4194304));
                *row[119] = carry_17_col119;

                for (i, &input) in [((carry_17_col119) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[18] = input;
                }
                *lookup_data.range_check_19_18 = [((carry_17_col119) + (M31_131072))];
                let carry_18_col120 =
                    (((conv_mod_tmp_5a147_90) + (carry_17_col119)) * (M31_4194304));
                *row[120] = carry_18_col120;

                for (i, &input) in [((carry_18_col120) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[19] = input;
                }
                *lookup_data.range_check_19_19 = [((carry_18_col120) + (M31_131072))];
                let carry_19_col121 =
                    (((conv_mod_tmp_5a147_91) + (carry_18_col120)) * (M31_4194304));
                *row[121] = carry_19_col121;

                for (i, &input) in [((carry_19_col121) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[20] = input;
                }
                *lookup_data.range_check_19_20 = [((carry_19_col121) + (M31_131072))];
                let carry_20_col122 =
                    (((conv_mod_tmp_5a147_92) + (carry_19_col121)) * (M31_4194304));
                *row[122] = carry_20_col122;

                for (i, &input) in [((carry_20_col122) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[21] = input;
                }
                *lookup_data.range_check_19_21 = [((carry_20_col122) + (M31_131072))];
                let carry_21_col123 = ((((conv_mod_tmp_5a147_93) - ((M31_136) * (k_col101)))
                    + (carry_20_col122))
                    * (M31_4194304));
                *row[123] = carry_21_col123;

                for (i, &input) in [((carry_21_col123) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[22] = input;
                }
                *lookup_data.range_check_19_22 = [((carry_21_col123) + (M31_131072))];
                let carry_22_col124 =
                    (((conv_mod_tmp_5a147_94) + (carry_21_col123)) * (M31_4194304));
                *row[124] = carry_22_col124;

                for (i, &input) in [((carry_22_col124) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[23] = input;
                }
                *lookup_data.range_check_19_23 = [((carry_22_col124) + (M31_131072))];
                let carry_23_col125 =
                    (((conv_mod_tmp_5a147_95) + (carry_22_col124)) * (M31_4194304));
                *row[125] = carry_23_col125;

                for (i, &input) in [((carry_23_col125) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[24] = input;
                }
                *lookup_data.range_check_19_24 = [((carry_23_col125) + (M31_131072))];
                let carry_24_col126 =
                    (((conv_mod_tmp_5a147_96) + (carry_23_col125)) * (M31_4194304));
                *row[126] = carry_24_col126;

                for (i, &input) in [((carry_24_col126) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[25] = input;
                }
                *lookup_data.range_check_19_25 = [((carry_24_col126) + (M31_131072))];
                let carry_25_col127 =
                    (((conv_mod_tmp_5a147_97) + (carry_24_col126)) * (M31_4194304));
                *row[127] = carry_25_col127;

                for (i, &input) in [((carry_25_col127) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[26] = input;
                }
                *lookup_data.range_check_19_26 = [((carry_25_col127) + (M31_131072))];
                let carry_26_col128 =
                    (((conv_mod_tmp_5a147_98) + (carry_25_col127)) * (M31_4194304));
                *row[128] = carry_26_col128;

                for (i, &input) in [((carry_26_col128) + (M31_131072))]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_19_inputs[27] = input;
                }
                *lookup_data.range_check_19_27 = [((carry_26_col128) + (M31_131072))];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_1)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
            },
        );

    (trace, sub_components_inputs, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_19_1: Vec<[PackedM31; 1]>,
    range_check_19_2: Vec<[PackedM31; 1]>,
    range_check_19_3: Vec<[PackedM31; 1]>,
    range_check_19_4: Vec<[PackedM31; 1]>,
    range_check_19_5: Vec<[PackedM31; 1]>,
    range_check_19_6: Vec<[PackedM31; 1]>,
    range_check_19_7: Vec<[PackedM31; 1]>,
    range_check_19_8: Vec<[PackedM31; 1]>,
    range_check_19_9: Vec<[PackedM31; 1]>,
    range_check_19_10: Vec<[PackedM31; 1]>,
    range_check_19_11: Vec<[PackedM31; 1]>,
    range_check_19_12: Vec<[PackedM31; 1]>,
    range_check_19_13: Vec<[PackedM31; 1]>,
    range_check_19_14: Vec<[PackedM31; 1]>,
    range_check_19_15: Vec<[PackedM31; 1]>,
    range_check_19_16: Vec<[PackedM31; 1]>,
    range_check_19_17: Vec<[PackedM31; 1]>,
    range_check_19_18: Vec<[PackedM31; 1]>,
    range_check_19_19: Vec<[PackedM31; 1]>,
    range_check_19_20: Vec<[PackedM31; 1]>,
    range_check_19_21: Vec<[PackedM31; 1]>,
    range_check_19_22: Vec<[PackedM31; 1]>,
    range_check_19_23: Vec<[PackedM31; 1]>,
    range_check_19_24: Vec<[PackedM31; 1]>,
    range_check_19_25: Vec<[PackedM31; 1]>,
    range_check_19_26: Vec<[PackedM31; 1]>,
    range_check_19_27: Vec<[PackedM31; 1]>,
    verify_instruction_0: Vec<[PackedM31; 19]>,
}

pub struct InteractionClaimGenerator {
    n_calls: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        range_check_19: &relations::RangeCheck_19,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_instruction.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_address_to_id_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.memory_address_to_id_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.range_check_19_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_1,
            &self.lookup_data.range_check_19_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_3,
            &self.lookup_data.range_check_19_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_5,
            &self.lookup_data.range_check_19_6,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_7,
            &self.lookup_data.range_check_19_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_9,
            &self.lookup_data.range_check_19_10,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_11,
            &self.lookup_data.range_check_19_12,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_13,
            &self.lookup_data.range_check_19_14,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_15,
            &self.lookup_data.range_check_19_16,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_17,
            &self.lookup_data.range_check_19_18,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_19,
            &self.lookup_data.range_check_19_20,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_21,
            &self.lookup_data.range_check_19_22,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_23,
            &self.lookup_data.range_check_19_24,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_25,
            &self.lookup_data.range_check_19_26,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = range_check_19.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_19_27,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_19.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.opcodes_1.iter().enumerate() {
            let denom = opcodes.combine(values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, total_sum, claimed_sum) = if self.n_calls == 1 << log_size {
            let (trace, claimed_sum) = logup_gen.finalize_last();
            (trace, claimed_sum, None)
        } else {
            let (trace, [total_sum, claimed_sum]) =
                logup_gen.finalize_at([(1 << log_size) - 1, self.n_calls - 1]);
            (trace, total_sum, Some((claimed_sum, self.n_calls - 1)))
        };
        tree_builder.extend_evals(trace);

        InteractionClaim {
            logup_sums: (total_sum, claimed_sum),
        }
    }
}

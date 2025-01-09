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
use crate::components::{memory_address_to_id, memory_id_to_big, pack_values, verify_instruction};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 42;

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
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 2],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 2],
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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    trace
        .par_iter_mut()
        .zip(inputs.par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(sub_components_inputs.par_iter_mut().chunks(N_LANES))
        .for_each(
            |(
                ((row, jnz_opcode_is_taken_t_dst_base_fp_t_input), lookup_data),
                mut sub_components_inputs,
            )| {
                let input_tmp_10786_0 = jnz_opcode_is_taken_t_dst_base_fp_t_input;
                let input_pc_col0 = input_tmp_10786_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_10786_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_10786_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_10786_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_10786_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_10786_1);
                let offset0_tmp_10786_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_10786_2.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_10786_2.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_10786_3.as_m31();
                *row[3] = offset0_col3;
                let ap_update_add_1_tmp_10786_4 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_10786_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_10786_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col4 = ap_update_add_1_tmp_10786_4.as_m31();
                *row[4] = ap_update_add_1_col4;

                for (i, &input) in (
                    input_pc_col0,
                    [offset0_col3, M31_32767, M31_32769],
                    [
                        M31_1,
                        M31_1,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        ap_update_add_1_col4,
                        M31_0,
                        M31_0,
                        M31_0,
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
                    M31_32767,
                    M31_32769,
                    M31_1,
                    M31_1,
                    M31_1,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_1,
                    M31_0,
                    ap_update_add_1_col4,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                // Read Positive Num Bits 252.

                let memory_address_to_id_value_tmp_10786_5 = memory_address_to_id_state
                    .deduce_output(((input_fp_col2) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_10786_6 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_10786_5);
                let dst_id_col5 = memory_address_to_id_value_tmp_10786_5;
                *row[5] = dst_id_col5;
                for (i, &input) in ((input_fp_col2) + ((offset0_col3) - (M31_32768)))
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].memory_address_to_id_inputs[0] = input;
                }
                *lookup_data.memory_address_to_id_0 = [
                    ((input_fp_col2) + ((offset0_col3) - (M31_32768))),
                    dst_id_col5,
                ];
                let dst_limb_0_col6 = memory_id_to_big_value_tmp_10786_6.get_m31(0);
                *row[6] = dst_limb_0_col6;
                let dst_limb_1_col7 = memory_id_to_big_value_tmp_10786_6.get_m31(1);
                *row[7] = dst_limb_1_col7;
                let dst_limb_2_col8 = memory_id_to_big_value_tmp_10786_6.get_m31(2);
                *row[8] = dst_limb_2_col8;
                let dst_limb_3_col9 = memory_id_to_big_value_tmp_10786_6.get_m31(3);
                *row[9] = dst_limb_3_col9;
                let dst_limb_4_col10 = memory_id_to_big_value_tmp_10786_6.get_m31(4);
                *row[10] = dst_limb_4_col10;
                let dst_limb_5_col11 = memory_id_to_big_value_tmp_10786_6.get_m31(5);
                *row[11] = dst_limb_5_col11;
                let dst_limb_6_col12 = memory_id_to_big_value_tmp_10786_6.get_m31(6);
                *row[12] = dst_limb_6_col12;
                let dst_limb_7_col13 = memory_id_to_big_value_tmp_10786_6.get_m31(7);
                *row[13] = dst_limb_7_col13;
                let dst_limb_8_col14 = memory_id_to_big_value_tmp_10786_6.get_m31(8);
                *row[14] = dst_limb_8_col14;
                let dst_limb_9_col15 = memory_id_to_big_value_tmp_10786_6.get_m31(9);
                *row[15] = dst_limb_9_col15;
                let dst_limb_10_col16 = memory_id_to_big_value_tmp_10786_6.get_m31(10);
                *row[16] = dst_limb_10_col16;
                let dst_limb_11_col17 = memory_id_to_big_value_tmp_10786_6.get_m31(11);
                *row[17] = dst_limb_11_col17;
                let dst_limb_12_col18 = memory_id_to_big_value_tmp_10786_6.get_m31(12);
                *row[18] = dst_limb_12_col18;
                let dst_limb_13_col19 = memory_id_to_big_value_tmp_10786_6.get_m31(13);
                *row[19] = dst_limb_13_col19;
                let dst_limb_14_col20 = memory_id_to_big_value_tmp_10786_6.get_m31(14);
                *row[20] = dst_limb_14_col20;
                let dst_limb_15_col21 = memory_id_to_big_value_tmp_10786_6.get_m31(15);
                *row[21] = dst_limb_15_col21;
                let dst_limb_16_col22 = memory_id_to_big_value_tmp_10786_6.get_m31(16);
                *row[22] = dst_limb_16_col22;
                let dst_limb_17_col23 = memory_id_to_big_value_tmp_10786_6.get_m31(17);
                *row[23] = dst_limb_17_col23;
                let dst_limb_18_col24 = memory_id_to_big_value_tmp_10786_6.get_m31(18);
                *row[24] = dst_limb_18_col24;
                let dst_limb_19_col25 = memory_id_to_big_value_tmp_10786_6.get_m31(19);
                *row[25] = dst_limb_19_col25;
                let dst_limb_20_col26 = memory_id_to_big_value_tmp_10786_6.get_m31(20);
                *row[26] = dst_limb_20_col26;
                let dst_limb_21_col27 = memory_id_to_big_value_tmp_10786_6.get_m31(21);
                *row[27] = dst_limb_21_col27;
                let dst_limb_22_col28 = memory_id_to_big_value_tmp_10786_6.get_m31(22);
                *row[28] = dst_limb_22_col28;
                let dst_limb_23_col29 = memory_id_to_big_value_tmp_10786_6.get_m31(23);
                *row[29] = dst_limb_23_col29;
                let dst_limb_24_col30 = memory_id_to_big_value_tmp_10786_6.get_m31(24);
                *row[30] = dst_limb_24_col30;
                let dst_limb_25_col31 = memory_id_to_big_value_tmp_10786_6.get_m31(25);
                *row[31] = dst_limb_25_col31;
                let dst_limb_26_col32 = memory_id_to_big_value_tmp_10786_6.get_m31(26);
                *row[32] = dst_limb_26_col32;
                let dst_limb_27_col33 = memory_id_to_big_value_tmp_10786_6.get_m31(27);
                *row[33] = dst_limb_27_col33;
                for (i, &input) in dst_id_col5.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[0] = input;
                }
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col5,
                    dst_limb_0_col6,
                    dst_limb_1_col7,
                    dst_limb_2_col8,
                    dst_limb_3_col9,
                    dst_limb_4_col10,
                    dst_limb_5_col11,
                    dst_limb_6_col12,
                    dst_limb_7_col13,
                    dst_limb_8_col14,
                    dst_limb_9_col15,
                    dst_limb_10_col16,
                    dst_limb_11_col17,
                    dst_limb_12_col18,
                    dst_limb_13_col19,
                    dst_limb_14_col20,
                    dst_limb_15_col21,
                    dst_limb_16_col22,
                    dst_limb_17_col23,
                    dst_limb_18_col24,
                    dst_limb_19_col25,
                    dst_limb_20_col26,
                    dst_limb_21_col27,
                    dst_limb_22_col28,
                    dst_limb_23_col29,
                    dst_limb_24_col30,
                    dst_limb_25_col31,
                    dst_limb_26_col32,
                    dst_limb_27_col33,
                ];

                let res_col34 = ((M31_1).div(
                    (((((((((((((((((((((((((((((M31_0)
                        + (dst_limb_0_col6))
                        + (dst_limb_1_col7))
                        + (dst_limb_2_col8))
                        + (dst_limb_3_col9))
                        + (dst_limb_4_col10))
                        + (dst_limb_5_col11))
                        + (dst_limb_6_col12))
                        + (dst_limb_7_col13))
                        + (dst_limb_8_col14))
                        + (dst_limb_9_col15))
                        + (dst_limb_10_col16))
                        + (dst_limb_11_col17))
                        + (dst_limb_12_col18))
                        + (dst_limb_13_col19))
                        + (dst_limb_14_col20))
                        + (dst_limb_15_col21))
                        + (dst_limb_16_col22))
                        + (dst_limb_17_col23))
                        + (dst_limb_18_col24))
                        + (dst_limb_19_col25))
                        + (dst_limb_20_col26))
                        + (dst_limb_21_col27))
                        + (dst_limb_22_col28))
                        + (dst_limb_23_col29))
                        + (dst_limb_24_col30))
                        + (dst_limb_25_col31))
                        + (dst_limb_26_col32))
                        + (dst_limb_27_col33)),
                ));
                *row[34] = res_col34;
                let diff_from_p_tmp_10786_7 = ((dst_limb_0_col6) - (M31_1));
                let diff_from_p_tmp_10786_8 = ((dst_limb_21_col27) - (M31_136));
                let diff_from_p_tmp_10786_9 = ((dst_limb_27_col33) - (M31_256));
                let res_squares_col35 = ((M31_1).div(
                    (((((((((((((((((((((((((((((M31_0)
                        + ((diff_from_p_tmp_10786_7)
                            * (diff_from_p_tmp_10786_7)))
                        + (dst_limb_1_col7))
                        + (dst_limb_2_col8))
                        + (dst_limb_3_col9))
                        + (dst_limb_4_col10))
                        + (dst_limb_5_col11))
                        + (dst_limb_6_col12))
                        + (dst_limb_7_col13))
                        + (dst_limb_8_col14))
                        + (dst_limb_9_col15))
                        + (dst_limb_10_col16))
                        + (dst_limb_11_col17))
                        + (dst_limb_12_col18))
                        + (dst_limb_13_col19))
                        + (dst_limb_14_col20))
                        + (dst_limb_15_col21))
                        + (dst_limb_16_col22))
                        + (dst_limb_17_col23))
                        + (dst_limb_18_col24))
                        + (dst_limb_19_col25))
                        + (dst_limb_20_col26))
                        + ((diff_from_p_tmp_10786_8) * (diff_from_p_tmp_10786_8)))
                        + (dst_limb_22_col28))
                        + (dst_limb_23_col29))
                        + (dst_limb_24_col30))
                        + (dst_limb_25_col31))
                        + (dst_limb_26_col32))
                        + ((diff_from_p_tmp_10786_9) * (diff_from_p_tmp_10786_9))),
                ));
                *row[35] = res_squares_col35;

                // Read Small.

                let memory_address_to_id_value_tmp_10786_10 =
                    memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
                let memory_id_to_big_value_tmp_10786_11 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_10786_10);
                let next_pc_id_col36 = memory_address_to_id_value_tmp_10786_10;
                *row[36] = next_pc_id_col36;
                for (i, &input) in ((input_pc_col0) + (M31_1)).unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_address_to_id_inputs[1] = input;
                }
                *lookup_data.memory_address_to_id_1 =
                    [((input_pc_col0) + (M31_1)), next_pc_id_col36];

                // Cond Decode Small Sign.

                let msb_tmp_10786_12 = memory_id_to_big_value_tmp_10786_11.get_m31(27).eq(M31_256);
                let msb_col37 = msb_tmp_10786_12.as_m31();
                *row[37] = msb_col37;
                let mid_limbs_set_tmp_10786_13 =
                    memory_id_to_big_value_tmp_10786_11.get_m31(20).eq(M31_511);
                let mid_limbs_set_col38 = mid_limbs_set_tmp_10786_13.as_m31();
                *row[38] = mid_limbs_set_col38;

                let next_pc_limb_0_col39 = memory_id_to_big_value_tmp_10786_11.get_m31(0);
                *row[39] = next_pc_limb_0_col39;
                let next_pc_limb_1_col40 = memory_id_to_big_value_tmp_10786_11.get_m31(1);
                *row[40] = next_pc_limb_1_col40;
                let next_pc_limb_2_col41 = memory_id_to_big_value_tmp_10786_11.get_m31(2);
                *row[41] = next_pc_limb_2_col41;
                for (i, &input) in next_pc_id_col36.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[1] = input;
                }
                *lookup_data.memory_id_to_big_1 = [
                    next_pc_id_col36,
                    next_pc_limb_0_col39,
                    next_pc_limb_1_col40,
                    next_pc_limb_2_col41,
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    ((mid_limbs_set_col38) * (M31_511)),
                    (((M31_136) * (msb_col37)) - (mid_limbs_set_col38)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col37) * (M31_256)),
                ];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0)
                        + (((((next_pc_limb_0_col39) + ((next_pc_limb_1_col40) * (M31_512)))
                            + ((next_pc_limb_2_col41) * (M31_262144)))
                            - (msb_col37))
                            - ((M31_134217728) * (mid_limbs_set_col38)))),
                    ((input_ap_col1) + (ap_update_add_1_col4)),
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
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
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
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
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

// AIR version 97774321-dirty
#![allow(unused_parens)]
use cairo_air::components::eval_operands::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_19, range_check_19_b, range_check_19_c,
    range_check_19_d, range_check_19_e, range_check_19_f, range_check_19_g, range_check_19_h,
    range_check_9_9, range_check_9_9_b, range_check_9_9_c, range_check_9_9_d, range_check_9_9_e,
    range_check_9_9_f, range_check_9_9_g, range_check_9_9_h,
};
use crate::witness::prelude::*;

pub type InputType = (CasmState, [M31; 20], [M31; 3]);
pub type PackedInputType = (PackedCasmState, [PackedM31; 20], [PackedM31; 3]);

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_19_state: &range_check_19::ClaimGenerator,
        range_check_19_b_state: &range_check_19_b::ClaimGenerator,
        range_check_19_c_state: &range_check_19_c::ClaimGenerator,
        range_check_19_d_state: &range_check_19_d::ClaimGenerator,
        range_check_19_e_state: &range_check_19_e::ClaimGenerator,
        range_check_19_f_state: &range_check_19_f::ClaimGenerator,
        range_check_19_g_state: &range_check_19_g::ClaimGenerator,
        range_check_19_h_state: &range_check_19_h::ClaimGenerator,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
        range_check_9_9_b_state: &range_check_9_9_b::ClaimGenerator,
        range_check_9_9_c_state: &range_check_9_9_c::ClaimGenerator,
        range_check_9_9_d_state: &range_check_9_9_d::ClaimGenerator,
        range_check_9_9_e_state: &range_check_9_9_e::ClaimGenerator,
        range_check_9_9_f_state: &range_check_9_9_f::ClaimGenerator,
        range_check_9_9_g_state: &range_check_9_9_g::ClaimGenerator,
        range_check_9_9_h_state: &range_check_9_9_h::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_19_state,
            range_check_19_b_state,
            range_check_19_c_state,
            range_check_19_d_state,
            range_check_19_e_state,
            range_check_19_f_state,
            range_check_19_g_state,
            range_check_19_h_state,
            range_check_9_9_state,
            range_check_9_9_b_state,
            range_check_9_9_c_state,
            range_check_9_9_d_state,
            range_check_9_9_e_state,
            range_check_9_9_f_state,
            range_check_9_9_g_state,
            range_check_9_9_h_state,
        );
        sub_component_inputs
            .memory_address_to_id
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_id_to_big
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_b
            .iter()
            .for_each(|inputs| {
                range_check_9_9_b_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_c
            .iter()
            .for_each(|inputs| {
                range_check_9_9_c_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_d
            .iter()
            .for_each(|inputs| {
                range_check_9_9_d_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_e
            .iter()
            .for_each(|inputs| {
                range_check_9_9_e_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_f
            .iter()
            .for_each(|inputs| {
                range_check_9_9_f_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_g
            .iter()
            .for_each(|inputs| {
                range_check_9_9_g_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_9_9_h
            .iter()
            .for_each(|inputs| {
                range_check_9_9_h_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_h
            .iter()
            .for_each(|inputs| {
                range_check_19_h_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19
            .iter()
            .for_each(|inputs| {
                range_check_19_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_b
            .iter()
            .for_each(|inputs| {
                range_check_19_b_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_c
            .iter()
            .for_each(|inputs| {
                range_check_19_c_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_d
            .iter()
            .for_each(|inputs| {
                range_check_19_d_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_e
            .iter()
            .for_each(|inputs| {
                range_check_19_e_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_f
            .iter()
            .for_each(|inputs| {
                range_check_19_f_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_19_g
            .iter()
            .for_each(|inputs| {
                range_check_19_g_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 3],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 3],
    range_check_9_9: [Vec<range_check_9_9::PackedInputType>; 4],
    range_check_9_9_b: [Vec<range_check_9_9_b::PackedInputType>; 4],
    range_check_9_9_c: [Vec<range_check_9_9_c::PackedInputType>; 4],
    range_check_9_9_d: [Vec<range_check_9_9_d::PackedInputType>; 4],
    range_check_9_9_e: [Vec<range_check_9_9_e::PackedInputType>; 4],
    range_check_9_9_f: [Vec<range_check_9_9_f::PackedInputType>; 4],
    range_check_9_9_g: [Vec<range_check_9_9_g::PackedInputType>; 2],
    range_check_9_9_h: [Vec<range_check_9_9_h::PackedInputType>; 2],
    range_check_19_h: [Vec<range_check_19_h::PackedInputType>; 4],
    range_check_19: [Vec<range_check_19::PackedInputType>; 4],
    range_check_19_b: [Vec<range_check_19_b::PackedInputType>; 4],
    range_check_19_c: [Vec<range_check_19_c::PackedInputType>; 4],
    range_check_19_d: [Vec<range_check_19_d::PackedInputType>; 3],
    range_check_19_e: [Vec<range_check_19_e::PackedInputType>; 3],
    range_check_19_f: [Vec<range_check_19_f::PackedInputType>; 3],
    range_check_19_g: [Vec<range_check_19_g::PackedInputType>; 3],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_19_state: &range_check_19::ClaimGenerator,
    range_check_19_b_state: &range_check_19_b::ClaimGenerator,
    range_check_19_c_state: &range_check_19_c::ClaimGenerator,
    range_check_19_d_state: &range_check_19_d::ClaimGenerator,
    range_check_19_e_state: &range_check_19_e::ClaimGenerator,
    range_check_19_f_state: &range_check_19_f::ClaimGenerator,
    range_check_19_g_state: &range_check_19_g::ClaimGenerator,
    range_check_19_h_state: &range_check_19_h::ClaimGenerator,
    range_check_9_9_state: &range_check_9_9::ClaimGenerator,
    range_check_9_9_b_state: &range_check_9_9_b::ClaimGenerator,
    range_check_9_9_c_state: &range_check_9_9_c::ClaimGenerator,
    range_check_9_9_d_state: &range_check_9_9_d::ClaimGenerator,
    range_check_9_9_e_state: &range_check_9_9_e::ClaimGenerator,
    range_check_9_9_f_state: &range_check_9_9_f::ClaimGenerator,
    range_check_9_9_g_state: &range_check_9_9_g::ClaimGenerator,
    range_check_9_9_h_state: &range_check_9_9_h::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_131072 = PackedM31::broadcast(M31::from(131072));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_65536 = PackedM31::broadcast(M31::from(65536));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt32_262143 = PackedUInt32::broadcast(UInt32::from(262143));
    let UInt32_511 = PackedUInt32::broadcast(UInt32::from(511));
    let UInt32_65536 = PackedUInt32::broadcast(UInt32::from(65536));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs, eval_operands_input))| {
                let dst_src_col0 = (((eval_operands_input.1[0]) * (eval_operands_input.0.fp))
                    + (((M31_1) - (eval_operands_input.1[0])) * (eval_operands_input.0.ap)));
                *row[0] = dst_src_col0;

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_3172c_0 = memory_address_to_id_state
                    .deduce_output(((dst_src_col0) + (eval_operands_input.2[0])));
                let dst_id_col1 = memory_address_to_id_value_tmp_3172c_0;
                *row[1] = dst_id_col1;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((dst_src_col0) + (eval_operands_input.2[0]));
                *lookup_data.memory_address_to_id_0 =
                    [((dst_src_col0) + (eval_operands_input.2[0])), dst_id_col1];

                // Read Positive known Id Num Bits 252.

                let memory_id_to_big_value_tmp_3172c_2 =
                    memory_id_to_big_state.deduce_output(dst_id_col1);
                let dst_limb_0_col2 = memory_id_to_big_value_tmp_3172c_2.get_m31(0);
                *row[2] = dst_limb_0_col2;
                let dst_limb_1_col3 = memory_id_to_big_value_tmp_3172c_2.get_m31(1);
                *row[3] = dst_limb_1_col3;
                let dst_limb_2_col4 = memory_id_to_big_value_tmp_3172c_2.get_m31(2);
                *row[4] = dst_limb_2_col4;
                let dst_limb_3_col5 = memory_id_to_big_value_tmp_3172c_2.get_m31(3);
                *row[5] = dst_limb_3_col5;
                let dst_limb_4_col6 = memory_id_to_big_value_tmp_3172c_2.get_m31(4);
                *row[6] = dst_limb_4_col6;
                let dst_limb_5_col7 = memory_id_to_big_value_tmp_3172c_2.get_m31(5);
                *row[7] = dst_limb_5_col7;
                let dst_limb_6_col8 = memory_id_to_big_value_tmp_3172c_2.get_m31(6);
                *row[8] = dst_limb_6_col8;
                let dst_limb_7_col9 = memory_id_to_big_value_tmp_3172c_2.get_m31(7);
                *row[9] = dst_limb_7_col9;
                let dst_limb_8_col10 = memory_id_to_big_value_tmp_3172c_2.get_m31(8);
                *row[10] = dst_limb_8_col10;
                let dst_limb_9_col11 = memory_id_to_big_value_tmp_3172c_2.get_m31(9);
                *row[11] = dst_limb_9_col11;
                let dst_limb_10_col12 = memory_id_to_big_value_tmp_3172c_2.get_m31(10);
                *row[12] = dst_limb_10_col12;
                let dst_limb_11_col13 = memory_id_to_big_value_tmp_3172c_2.get_m31(11);
                *row[13] = dst_limb_11_col13;
                let dst_limb_12_col14 = memory_id_to_big_value_tmp_3172c_2.get_m31(12);
                *row[14] = dst_limb_12_col14;
                let dst_limb_13_col15 = memory_id_to_big_value_tmp_3172c_2.get_m31(13);
                *row[15] = dst_limb_13_col15;
                let dst_limb_14_col16 = memory_id_to_big_value_tmp_3172c_2.get_m31(14);
                *row[16] = dst_limb_14_col16;
                let dst_limb_15_col17 = memory_id_to_big_value_tmp_3172c_2.get_m31(15);
                *row[17] = dst_limb_15_col17;
                let dst_limb_16_col18 = memory_id_to_big_value_tmp_3172c_2.get_m31(16);
                *row[18] = dst_limb_16_col18;
                let dst_limb_17_col19 = memory_id_to_big_value_tmp_3172c_2.get_m31(17);
                *row[19] = dst_limb_17_col19;
                let dst_limb_18_col20 = memory_id_to_big_value_tmp_3172c_2.get_m31(18);
                *row[20] = dst_limb_18_col20;
                let dst_limb_19_col21 = memory_id_to_big_value_tmp_3172c_2.get_m31(19);
                *row[21] = dst_limb_19_col21;
                let dst_limb_20_col22 = memory_id_to_big_value_tmp_3172c_2.get_m31(20);
                *row[22] = dst_limb_20_col22;
                let dst_limb_21_col23 = memory_id_to_big_value_tmp_3172c_2.get_m31(21);
                *row[23] = dst_limb_21_col23;
                let dst_limb_22_col24 = memory_id_to_big_value_tmp_3172c_2.get_m31(22);
                *row[24] = dst_limb_22_col24;
                let dst_limb_23_col25 = memory_id_to_big_value_tmp_3172c_2.get_m31(23);
                *row[25] = dst_limb_23_col25;
                let dst_limb_24_col26 = memory_id_to_big_value_tmp_3172c_2.get_m31(24);
                *row[26] = dst_limb_24_col26;
                let dst_limb_25_col27 = memory_id_to_big_value_tmp_3172c_2.get_m31(25);
                *row[27] = dst_limb_25_col27;
                let dst_limb_26_col28 = memory_id_to_big_value_tmp_3172c_2.get_m31(26);
                *row[28] = dst_limb_26_col28;
                let dst_limb_27_col29 = memory_id_to_big_value_tmp_3172c_2.get_m31(27);
                *row[29] = dst_limb_27_col29;
                *sub_component_inputs.memory_id_to_big[0] = dst_id_col1;
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col1,
                    dst_limb_0_col2,
                    dst_limb_1_col3,
                    dst_limb_2_col4,
                    dst_limb_3_col5,
                    dst_limb_4_col6,
                    dst_limb_5_col7,
                    dst_limb_6_col8,
                    dst_limb_7_col9,
                    dst_limb_8_col10,
                    dst_limb_9_col11,
                    dst_limb_10_col12,
                    dst_limb_11_col13,
                    dst_limb_12_col14,
                    dst_limb_13_col15,
                    dst_limb_14_col16,
                    dst_limb_15_col17,
                    dst_limb_16_col18,
                    dst_limb_17_col19,
                    dst_limb_18_col20,
                    dst_limb_19_col21,
                    dst_limb_20_col22,
                    dst_limb_21_col23,
                    dst_limb_22_col24,
                    dst_limb_23_col25,
                    dst_limb_24_col26,
                    dst_limb_25_col27,
                    dst_limb_26_col28,
                    dst_limb_27_col29,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_3172c_3 =
                    PackedFelt252::from_limbs([
                        dst_limb_0_col2,
                        dst_limb_1_col3,
                        dst_limb_2_col4,
                        dst_limb_3_col5,
                        dst_limb_4_col6,
                        dst_limb_5_col7,
                        dst_limb_6_col8,
                        dst_limb_7_col9,
                        dst_limb_8_col10,
                        dst_limb_9_col11,
                        dst_limb_10_col12,
                        dst_limb_11_col13,
                        dst_limb_12_col14,
                        dst_limb_13_col15,
                        dst_limb_14_col16,
                        dst_limb_15_col17,
                        dst_limb_16_col18,
                        dst_limb_17_col19,
                        dst_limb_18_col20,
                        dst_limb_19_col21,
                        dst_limb_20_col22,
                        dst_limb_21_col23,
                        dst_limb_22_col24,
                        dst_limb_23_col25,
                        dst_limb_24_col26,
                        dst_limb_25_col27,
                        dst_limb_26_col28,
                        dst_limb_27_col29,
                    ]);

                let read_positive_num_bits_252_output_tmp_3172c_4 = (
                    read_positive_known_id_num_bits_252_output_tmp_3172c_3,
                    dst_id_col1,
                );

                let op0_src_col30 = (((eval_operands_input.1[1]) * (eval_operands_input.0.fp))
                    + (((M31_1) - (eval_operands_input.1[1])) * (eval_operands_input.0.ap)));
                *row[30] = op0_src_col30;

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_3172c_5 = memory_address_to_id_state
                    .deduce_output(((op0_src_col30) + (eval_operands_input.2[1])));
                let op0_id_col31 = memory_address_to_id_value_tmp_3172c_5;
                *row[31] = op0_id_col31;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((op0_src_col30) + (eval_operands_input.2[1]));
                *lookup_data.memory_address_to_id_1 =
                    [((op0_src_col30) + (eval_operands_input.2[1])), op0_id_col31];

                // Read Positive known Id Num Bits 252.

                let memory_id_to_big_value_tmp_3172c_7 =
                    memory_id_to_big_state.deduce_output(op0_id_col31);
                let op0_limb_0_col32 = memory_id_to_big_value_tmp_3172c_7.get_m31(0);
                *row[32] = op0_limb_0_col32;
                let op0_limb_1_col33 = memory_id_to_big_value_tmp_3172c_7.get_m31(1);
                *row[33] = op0_limb_1_col33;
                let op0_limb_2_col34 = memory_id_to_big_value_tmp_3172c_7.get_m31(2);
                *row[34] = op0_limb_2_col34;
                let op0_limb_3_col35 = memory_id_to_big_value_tmp_3172c_7.get_m31(3);
                *row[35] = op0_limb_3_col35;
                let op0_limb_4_col36 = memory_id_to_big_value_tmp_3172c_7.get_m31(4);
                *row[36] = op0_limb_4_col36;
                let op0_limb_5_col37 = memory_id_to_big_value_tmp_3172c_7.get_m31(5);
                *row[37] = op0_limb_5_col37;
                let op0_limb_6_col38 = memory_id_to_big_value_tmp_3172c_7.get_m31(6);
                *row[38] = op0_limb_6_col38;
                let op0_limb_7_col39 = memory_id_to_big_value_tmp_3172c_7.get_m31(7);
                *row[39] = op0_limb_7_col39;
                let op0_limb_8_col40 = memory_id_to_big_value_tmp_3172c_7.get_m31(8);
                *row[40] = op0_limb_8_col40;
                let op0_limb_9_col41 = memory_id_to_big_value_tmp_3172c_7.get_m31(9);
                *row[41] = op0_limb_9_col41;
                let op0_limb_10_col42 = memory_id_to_big_value_tmp_3172c_7.get_m31(10);
                *row[42] = op0_limb_10_col42;
                let op0_limb_11_col43 = memory_id_to_big_value_tmp_3172c_7.get_m31(11);
                *row[43] = op0_limb_11_col43;
                let op0_limb_12_col44 = memory_id_to_big_value_tmp_3172c_7.get_m31(12);
                *row[44] = op0_limb_12_col44;
                let op0_limb_13_col45 = memory_id_to_big_value_tmp_3172c_7.get_m31(13);
                *row[45] = op0_limb_13_col45;
                let op0_limb_14_col46 = memory_id_to_big_value_tmp_3172c_7.get_m31(14);
                *row[46] = op0_limb_14_col46;
                let op0_limb_15_col47 = memory_id_to_big_value_tmp_3172c_7.get_m31(15);
                *row[47] = op0_limb_15_col47;
                let op0_limb_16_col48 = memory_id_to_big_value_tmp_3172c_7.get_m31(16);
                *row[48] = op0_limb_16_col48;
                let op0_limb_17_col49 = memory_id_to_big_value_tmp_3172c_7.get_m31(17);
                *row[49] = op0_limb_17_col49;
                let op0_limb_18_col50 = memory_id_to_big_value_tmp_3172c_7.get_m31(18);
                *row[50] = op0_limb_18_col50;
                let op0_limb_19_col51 = memory_id_to_big_value_tmp_3172c_7.get_m31(19);
                *row[51] = op0_limb_19_col51;
                let op0_limb_20_col52 = memory_id_to_big_value_tmp_3172c_7.get_m31(20);
                *row[52] = op0_limb_20_col52;
                let op0_limb_21_col53 = memory_id_to_big_value_tmp_3172c_7.get_m31(21);
                *row[53] = op0_limb_21_col53;
                let op0_limb_22_col54 = memory_id_to_big_value_tmp_3172c_7.get_m31(22);
                *row[54] = op0_limb_22_col54;
                let op0_limb_23_col55 = memory_id_to_big_value_tmp_3172c_7.get_m31(23);
                *row[55] = op0_limb_23_col55;
                let op0_limb_24_col56 = memory_id_to_big_value_tmp_3172c_7.get_m31(24);
                *row[56] = op0_limb_24_col56;
                let op0_limb_25_col57 = memory_id_to_big_value_tmp_3172c_7.get_m31(25);
                *row[57] = op0_limb_25_col57;
                let op0_limb_26_col58 = memory_id_to_big_value_tmp_3172c_7.get_m31(26);
                *row[58] = op0_limb_26_col58;
                let op0_limb_27_col59 = memory_id_to_big_value_tmp_3172c_7.get_m31(27);
                *row[59] = op0_limb_27_col59;
                *sub_component_inputs.memory_id_to_big[1] = op0_id_col31;
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col31,
                    op0_limb_0_col32,
                    op0_limb_1_col33,
                    op0_limb_2_col34,
                    op0_limb_3_col35,
                    op0_limb_4_col36,
                    op0_limb_5_col37,
                    op0_limb_6_col38,
                    op0_limb_7_col39,
                    op0_limb_8_col40,
                    op0_limb_9_col41,
                    op0_limb_10_col42,
                    op0_limb_11_col43,
                    op0_limb_12_col44,
                    op0_limb_13_col45,
                    op0_limb_14_col46,
                    op0_limb_15_col47,
                    op0_limb_16_col48,
                    op0_limb_17_col49,
                    op0_limb_18_col50,
                    op0_limb_19_col51,
                    op0_limb_20_col52,
                    op0_limb_21_col53,
                    op0_limb_22_col54,
                    op0_limb_23_col55,
                    op0_limb_24_col56,
                    op0_limb_25_col57,
                    op0_limb_26_col58,
                    op0_limb_27_col59,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_3172c_8 =
                    PackedFelt252::from_limbs([
                        op0_limb_0_col32,
                        op0_limb_1_col33,
                        op0_limb_2_col34,
                        op0_limb_3_col35,
                        op0_limb_4_col36,
                        op0_limb_5_col37,
                        op0_limb_6_col38,
                        op0_limb_7_col39,
                        op0_limb_8_col40,
                        op0_limb_9_col41,
                        op0_limb_10_col42,
                        op0_limb_11_col43,
                        op0_limb_12_col44,
                        op0_limb_13_col45,
                        op0_limb_14_col46,
                        op0_limb_15_col47,
                        op0_limb_16_col48,
                        op0_limb_17_col49,
                        op0_limb_18_col50,
                        op0_limb_19_col51,
                        op0_limb_20_col52,
                        op0_limb_21_col53,
                        op0_limb_22_col54,
                        op0_limb_23_col55,
                        op0_limb_24_col56,
                        op0_limb_25_col57,
                        op0_limb_26_col58,
                        op0_limb_27_col59,
                    ]);

                let read_positive_num_bits_252_output_tmp_3172c_9 = (
                    read_positive_known_id_num_bits_252_output_tmp_3172c_8,
                    op0_id_col31,
                );

                // Cond Felt 252 As Addr.

                let cond_felt_252_as_addr_output_tmp_3172c_10 = (((op0_limb_0_col32)
                    + ((op0_limb_1_col33) * (M31_512)))
                    + ((op0_limb_2_col34) * (M31_262144)));

                let op1_src_col60 = (((((eval_operands_input.1[3]) * (eval_operands_input.0.fp))
                    + ((eval_operands_input.1[4]) * (eval_operands_input.0.ap)))
                    + ((eval_operands_input.1[2]) * (eval_operands_input.0.pc)))
                    + ((eval_operands_input.1[15]) * (cond_felt_252_as_addr_output_tmp_3172c_10)));
                *row[60] = op1_src_col60;

                // Read Positive Num Bits 252.

                // Read Id.

                let memory_address_to_id_value_tmp_3172c_11 = memory_address_to_id_state
                    .deduce_output(((op1_src_col60) + (eval_operands_input.2[2])));
                let op1_id_col61 = memory_address_to_id_value_tmp_3172c_11;
                *row[61] = op1_id_col61;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((op1_src_col60) + (eval_operands_input.2[2]));
                *lookup_data.memory_address_to_id_2 =
                    [((op1_src_col60) + (eval_operands_input.2[2])), op1_id_col61];

                // Read Positive known Id Num Bits 252.

                let memory_id_to_big_value_tmp_3172c_13 =
                    memory_id_to_big_state.deduce_output(op1_id_col61);
                let op1_limb_0_col62 = memory_id_to_big_value_tmp_3172c_13.get_m31(0);
                *row[62] = op1_limb_0_col62;
                let op1_limb_1_col63 = memory_id_to_big_value_tmp_3172c_13.get_m31(1);
                *row[63] = op1_limb_1_col63;
                let op1_limb_2_col64 = memory_id_to_big_value_tmp_3172c_13.get_m31(2);
                *row[64] = op1_limb_2_col64;
                let op1_limb_3_col65 = memory_id_to_big_value_tmp_3172c_13.get_m31(3);
                *row[65] = op1_limb_3_col65;
                let op1_limb_4_col66 = memory_id_to_big_value_tmp_3172c_13.get_m31(4);
                *row[66] = op1_limb_4_col66;
                let op1_limb_5_col67 = memory_id_to_big_value_tmp_3172c_13.get_m31(5);
                *row[67] = op1_limb_5_col67;
                let op1_limb_6_col68 = memory_id_to_big_value_tmp_3172c_13.get_m31(6);
                *row[68] = op1_limb_6_col68;
                let op1_limb_7_col69 = memory_id_to_big_value_tmp_3172c_13.get_m31(7);
                *row[69] = op1_limb_7_col69;
                let op1_limb_8_col70 = memory_id_to_big_value_tmp_3172c_13.get_m31(8);
                *row[70] = op1_limb_8_col70;
                let op1_limb_9_col71 = memory_id_to_big_value_tmp_3172c_13.get_m31(9);
                *row[71] = op1_limb_9_col71;
                let op1_limb_10_col72 = memory_id_to_big_value_tmp_3172c_13.get_m31(10);
                *row[72] = op1_limb_10_col72;
                let op1_limb_11_col73 = memory_id_to_big_value_tmp_3172c_13.get_m31(11);
                *row[73] = op1_limb_11_col73;
                let op1_limb_12_col74 = memory_id_to_big_value_tmp_3172c_13.get_m31(12);
                *row[74] = op1_limb_12_col74;
                let op1_limb_13_col75 = memory_id_to_big_value_tmp_3172c_13.get_m31(13);
                *row[75] = op1_limb_13_col75;
                let op1_limb_14_col76 = memory_id_to_big_value_tmp_3172c_13.get_m31(14);
                *row[76] = op1_limb_14_col76;
                let op1_limb_15_col77 = memory_id_to_big_value_tmp_3172c_13.get_m31(15);
                *row[77] = op1_limb_15_col77;
                let op1_limb_16_col78 = memory_id_to_big_value_tmp_3172c_13.get_m31(16);
                *row[78] = op1_limb_16_col78;
                let op1_limb_17_col79 = memory_id_to_big_value_tmp_3172c_13.get_m31(17);
                *row[79] = op1_limb_17_col79;
                let op1_limb_18_col80 = memory_id_to_big_value_tmp_3172c_13.get_m31(18);
                *row[80] = op1_limb_18_col80;
                let op1_limb_19_col81 = memory_id_to_big_value_tmp_3172c_13.get_m31(19);
                *row[81] = op1_limb_19_col81;
                let op1_limb_20_col82 = memory_id_to_big_value_tmp_3172c_13.get_m31(20);
                *row[82] = op1_limb_20_col82;
                let op1_limb_21_col83 = memory_id_to_big_value_tmp_3172c_13.get_m31(21);
                *row[83] = op1_limb_21_col83;
                let op1_limb_22_col84 = memory_id_to_big_value_tmp_3172c_13.get_m31(22);
                *row[84] = op1_limb_22_col84;
                let op1_limb_23_col85 = memory_id_to_big_value_tmp_3172c_13.get_m31(23);
                *row[85] = op1_limb_23_col85;
                let op1_limb_24_col86 = memory_id_to_big_value_tmp_3172c_13.get_m31(24);
                *row[86] = op1_limb_24_col86;
                let op1_limb_25_col87 = memory_id_to_big_value_tmp_3172c_13.get_m31(25);
                *row[87] = op1_limb_25_col87;
                let op1_limb_26_col88 = memory_id_to_big_value_tmp_3172c_13.get_m31(26);
                *row[88] = op1_limb_26_col88;
                let op1_limb_27_col89 = memory_id_to_big_value_tmp_3172c_13.get_m31(27);
                *row[89] = op1_limb_27_col89;
                *sub_component_inputs.memory_id_to_big[2] = op1_id_col61;
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col61,
                    op1_limb_0_col62,
                    op1_limb_1_col63,
                    op1_limb_2_col64,
                    op1_limb_3_col65,
                    op1_limb_4_col66,
                    op1_limb_5_col67,
                    op1_limb_6_col68,
                    op1_limb_7_col69,
                    op1_limb_8_col70,
                    op1_limb_9_col71,
                    op1_limb_10_col72,
                    op1_limb_11_col73,
                    op1_limb_12_col74,
                    op1_limb_13_col75,
                    op1_limb_14_col76,
                    op1_limb_15_col77,
                    op1_limb_16_col78,
                    op1_limb_17_col79,
                    op1_limb_18_col80,
                    op1_limb_19_col81,
                    op1_limb_20_col82,
                    op1_limb_21_col83,
                    op1_limb_22_col84,
                    op1_limb_23_col85,
                    op1_limb_24_col86,
                    op1_limb_25_col87,
                    op1_limb_26_col88,
                    op1_limb_27_col89,
                ];
                let read_positive_known_id_num_bits_252_output_tmp_3172c_14 =
                    PackedFelt252::from_limbs([
                        op1_limb_0_col62,
                        op1_limb_1_col63,
                        op1_limb_2_col64,
                        op1_limb_3_col65,
                        op1_limb_4_col66,
                        op1_limb_5_col67,
                        op1_limb_6_col68,
                        op1_limb_7_col69,
                        op1_limb_8_col70,
                        op1_limb_9_col71,
                        op1_limb_10_col72,
                        op1_limb_11_col73,
                        op1_limb_12_col74,
                        op1_limb_13_col75,
                        op1_limb_14_col76,
                        op1_limb_15_col77,
                        op1_limb_16_col78,
                        op1_limb_17_col79,
                        op1_limb_18_col80,
                        op1_limb_19_col81,
                        op1_limb_20_col82,
                        op1_limb_21_col83,
                        op1_limb_22_col84,
                        op1_limb_23_col85,
                        op1_limb_24_col86,
                        op1_limb_25_col87,
                        op1_limb_26_col88,
                        op1_limb_27_col89,
                    ]);

                let read_positive_num_bits_252_output_tmp_3172c_15 = (
                    read_positive_known_id_num_bits_252_output_tmp_3172c_14,
                    op1_id_col61,
                );

                // Add 252.

                let add_res_tmp_3172c_16 = ((read_positive_num_bits_252_output_tmp_3172c_9.0)
                    + (read_positive_num_bits_252_output_tmp_3172c_15.0));
                let add_res_limb_0_col90 = add_res_tmp_3172c_16.get_m31(0);
                *row[90] = add_res_limb_0_col90;
                let add_res_limb_1_col91 = add_res_tmp_3172c_16.get_m31(1);
                *row[91] = add_res_limb_1_col91;
                let add_res_limb_2_col92 = add_res_tmp_3172c_16.get_m31(2);
                *row[92] = add_res_limb_2_col92;
                let add_res_limb_3_col93 = add_res_tmp_3172c_16.get_m31(3);
                *row[93] = add_res_limb_3_col93;
                let add_res_limb_4_col94 = add_res_tmp_3172c_16.get_m31(4);
                *row[94] = add_res_limb_4_col94;
                let add_res_limb_5_col95 = add_res_tmp_3172c_16.get_m31(5);
                *row[95] = add_res_limb_5_col95;
                let add_res_limb_6_col96 = add_res_tmp_3172c_16.get_m31(6);
                *row[96] = add_res_limb_6_col96;
                let add_res_limb_7_col97 = add_res_tmp_3172c_16.get_m31(7);
                *row[97] = add_res_limb_7_col97;
                let add_res_limb_8_col98 = add_res_tmp_3172c_16.get_m31(8);
                *row[98] = add_res_limb_8_col98;
                let add_res_limb_9_col99 = add_res_tmp_3172c_16.get_m31(9);
                *row[99] = add_res_limb_9_col99;
                let add_res_limb_10_col100 = add_res_tmp_3172c_16.get_m31(10);
                *row[100] = add_res_limb_10_col100;
                let add_res_limb_11_col101 = add_res_tmp_3172c_16.get_m31(11);
                *row[101] = add_res_limb_11_col101;
                let add_res_limb_12_col102 = add_res_tmp_3172c_16.get_m31(12);
                *row[102] = add_res_limb_12_col102;
                let add_res_limb_13_col103 = add_res_tmp_3172c_16.get_m31(13);
                *row[103] = add_res_limb_13_col103;
                let add_res_limb_14_col104 = add_res_tmp_3172c_16.get_m31(14);
                *row[104] = add_res_limb_14_col104;
                let add_res_limb_15_col105 = add_res_tmp_3172c_16.get_m31(15);
                *row[105] = add_res_limb_15_col105;
                let add_res_limb_16_col106 = add_res_tmp_3172c_16.get_m31(16);
                *row[106] = add_res_limb_16_col106;
                let add_res_limb_17_col107 = add_res_tmp_3172c_16.get_m31(17);
                *row[107] = add_res_limb_17_col107;
                let add_res_limb_18_col108 = add_res_tmp_3172c_16.get_m31(18);
                *row[108] = add_res_limb_18_col108;
                let add_res_limb_19_col109 = add_res_tmp_3172c_16.get_m31(19);
                *row[109] = add_res_limb_19_col109;
                let add_res_limb_20_col110 = add_res_tmp_3172c_16.get_m31(20);
                *row[110] = add_res_limb_20_col110;
                let add_res_limb_21_col111 = add_res_tmp_3172c_16.get_m31(21);
                *row[111] = add_res_limb_21_col111;
                let add_res_limb_22_col112 = add_res_tmp_3172c_16.get_m31(22);
                *row[112] = add_res_limb_22_col112;
                let add_res_limb_23_col113 = add_res_tmp_3172c_16.get_m31(23);
                *row[113] = add_res_limb_23_col113;
                let add_res_limb_24_col114 = add_res_tmp_3172c_16.get_m31(24);
                *row[114] = add_res_limb_24_col114;
                let add_res_limb_25_col115 = add_res_tmp_3172c_16.get_m31(25);
                *row[115] = add_res_limb_25_col115;
                let add_res_limb_26_col116 = add_res_tmp_3172c_16.get_m31(26);
                *row[116] = add_res_limb_26_col116;
                let add_res_limb_27_col117 = add_res_tmp_3172c_16.get_m31(27);
                *row[117] = add_res_limb_27_col117;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[0] =
                    [add_res_limb_0_col90, add_res_limb_1_col91];
                *lookup_data.range_check_9_9_0 = [add_res_limb_0_col90, add_res_limb_1_col91];
                *sub_component_inputs.range_check_9_9_b[0] =
                    [add_res_limb_2_col92, add_res_limb_3_col93];
                *lookup_data.range_check_9_9_b_0 = [add_res_limb_2_col92, add_res_limb_3_col93];
                *sub_component_inputs.range_check_9_9_c[0] =
                    [add_res_limb_4_col94, add_res_limb_5_col95];
                *lookup_data.range_check_9_9_c_0 = [add_res_limb_4_col94, add_res_limb_5_col95];
                *sub_component_inputs.range_check_9_9_d[0] =
                    [add_res_limb_6_col96, add_res_limb_7_col97];
                *lookup_data.range_check_9_9_d_0 = [add_res_limb_6_col96, add_res_limb_7_col97];
                *sub_component_inputs.range_check_9_9_e[0] =
                    [add_res_limb_8_col98, add_res_limb_9_col99];
                *lookup_data.range_check_9_9_e_0 = [add_res_limb_8_col98, add_res_limb_9_col99];
                *sub_component_inputs.range_check_9_9_f[0] =
                    [add_res_limb_10_col100, add_res_limb_11_col101];
                *lookup_data.range_check_9_9_f_0 = [add_res_limb_10_col100, add_res_limb_11_col101];
                *sub_component_inputs.range_check_9_9_g[0] =
                    [add_res_limb_12_col102, add_res_limb_13_col103];
                *lookup_data.range_check_9_9_g_0 = [add_res_limb_12_col102, add_res_limb_13_col103];
                *sub_component_inputs.range_check_9_9_h[0] =
                    [add_res_limb_14_col104, add_res_limb_15_col105];
                *lookup_data.range_check_9_9_h_0 = [add_res_limb_14_col104, add_res_limb_15_col105];
                *sub_component_inputs.range_check_9_9[1] =
                    [add_res_limb_16_col106, add_res_limb_17_col107];
                *lookup_data.range_check_9_9_1 = [add_res_limb_16_col106, add_res_limb_17_col107];
                *sub_component_inputs.range_check_9_9_b[1] =
                    [add_res_limb_18_col108, add_res_limb_19_col109];
                *lookup_data.range_check_9_9_b_1 = [add_res_limb_18_col108, add_res_limb_19_col109];
                *sub_component_inputs.range_check_9_9_c[1] =
                    [add_res_limb_20_col110, add_res_limb_21_col111];
                *lookup_data.range_check_9_9_c_1 = [add_res_limb_20_col110, add_res_limb_21_col111];
                *sub_component_inputs.range_check_9_9_d[1] =
                    [add_res_limb_22_col112, add_res_limb_23_col113];
                *lookup_data.range_check_9_9_d_1 = [add_res_limb_22_col112, add_res_limb_23_col113];
                *sub_component_inputs.range_check_9_9_e[1] =
                    [add_res_limb_24_col114, add_res_limb_25_col115];
                *lookup_data.range_check_9_9_e_1 = [add_res_limb_24_col114, add_res_limb_25_col115];
                *sub_component_inputs.range_check_9_9_f[1] =
                    [add_res_limb_26_col116, add_res_limb_27_col117];
                *lookup_data.range_check_9_9_f_1 = [add_res_limb_26_col116, add_res_limb_27_col117];

                // Verify Add 252.

                let sub_p_bit_tmp_3172c_17 = ((UInt16_1)
                    & (((PackedUInt16::from_m31(op0_limb_0_col32))
                        ^ (PackedUInt16::from_m31(op1_limb_0_col62)))
                        ^ (PackedUInt16::from_m31(add_res_limb_0_col90))));
                let sub_p_bit_col118 = sub_p_bit_tmp_3172c_17.as_m31();
                *row[118] = sub_p_bit_col118;

                let add_252_output_tmp_3172c_45 = add_res_tmp_3172c_16;

                // Mul 252.

                let mul_res_tmp_3172c_46 = ((read_positive_num_bits_252_output_tmp_3172c_9.0)
                    * (read_positive_num_bits_252_output_tmp_3172c_15.0));
                let mul_res_limb_0_col119 = mul_res_tmp_3172c_46.get_m31(0);
                *row[119] = mul_res_limb_0_col119;
                let mul_res_limb_1_col120 = mul_res_tmp_3172c_46.get_m31(1);
                *row[120] = mul_res_limb_1_col120;
                let mul_res_limb_2_col121 = mul_res_tmp_3172c_46.get_m31(2);
                *row[121] = mul_res_limb_2_col121;
                let mul_res_limb_3_col122 = mul_res_tmp_3172c_46.get_m31(3);
                *row[122] = mul_res_limb_3_col122;
                let mul_res_limb_4_col123 = mul_res_tmp_3172c_46.get_m31(4);
                *row[123] = mul_res_limb_4_col123;
                let mul_res_limb_5_col124 = mul_res_tmp_3172c_46.get_m31(5);
                *row[124] = mul_res_limb_5_col124;
                let mul_res_limb_6_col125 = mul_res_tmp_3172c_46.get_m31(6);
                *row[125] = mul_res_limb_6_col125;
                let mul_res_limb_7_col126 = mul_res_tmp_3172c_46.get_m31(7);
                *row[126] = mul_res_limb_7_col126;
                let mul_res_limb_8_col127 = mul_res_tmp_3172c_46.get_m31(8);
                *row[127] = mul_res_limb_8_col127;
                let mul_res_limb_9_col128 = mul_res_tmp_3172c_46.get_m31(9);
                *row[128] = mul_res_limb_9_col128;
                let mul_res_limb_10_col129 = mul_res_tmp_3172c_46.get_m31(10);
                *row[129] = mul_res_limb_10_col129;
                let mul_res_limb_11_col130 = mul_res_tmp_3172c_46.get_m31(11);
                *row[130] = mul_res_limb_11_col130;
                let mul_res_limb_12_col131 = mul_res_tmp_3172c_46.get_m31(12);
                *row[131] = mul_res_limb_12_col131;
                let mul_res_limb_13_col132 = mul_res_tmp_3172c_46.get_m31(13);
                *row[132] = mul_res_limb_13_col132;
                let mul_res_limb_14_col133 = mul_res_tmp_3172c_46.get_m31(14);
                *row[133] = mul_res_limb_14_col133;
                let mul_res_limb_15_col134 = mul_res_tmp_3172c_46.get_m31(15);
                *row[134] = mul_res_limb_15_col134;
                let mul_res_limb_16_col135 = mul_res_tmp_3172c_46.get_m31(16);
                *row[135] = mul_res_limb_16_col135;
                let mul_res_limb_17_col136 = mul_res_tmp_3172c_46.get_m31(17);
                *row[136] = mul_res_limb_17_col136;
                let mul_res_limb_18_col137 = mul_res_tmp_3172c_46.get_m31(18);
                *row[137] = mul_res_limb_18_col137;
                let mul_res_limb_19_col138 = mul_res_tmp_3172c_46.get_m31(19);
                *row[138] = mul_res_limb_19_col138;
                let mul_res_limb_20_col139 = mul_res_tmp_3172c_46.get_m31(20);
                *row[139] = mul_res_limb_20_col139;
                let mul_res_limb_21_col140 = mul_res_tmp_3172c_46.get_m31(21);
                *row[140] = mul_res_limb_21_col140;
                let mul_res_limb_22_col141 = mul_res_tmp_3172c_46.get_m31(22);
                *row[141] = mul_res_limb_22_col141;
                let mul_res_limb_23_col142 = mul_res_tmp_3172c_46.get_m31(23);
                *row[142] = mul_res_limb_23_col142;
                let mul_res_limb_24_col143 = mul_res_tmp_3172c_46.get_m31(24);
                *row[143] = mul_res_limb_24_col143;
                let mul_res_limb_25_col144 = mul_res_tmp_3172c_46.get_m31(25);
                *row[144] = mul_res_limb_25_col144;
                let mul_res_limb_26_col145 = mul_res_tmp_3172c_46.get_m31(26);
                *row[145] = mul_res_limb_26_col145;
                let mul_res_limb_27_col146 = mul_res_tmp_3172c_46.get_m31(27);
                *row[146] = mul_res_limb_27_col146;

                // Range Check Mem Value N 28.

                *sub_component_inputs.range_check_9_9[2] =
                    [mul_res_limb_0_col119, mul_res_limb_1_col120];
                *lookup_data.range_check_9_9_2 = [mul_res_limb_0_col119, mul_res_limb_1_col120];
                *sub_component_inputs.range_check_9_9_b[2] =
                    [mul_res_limb_2_col121, mul_res_limb_3_col122];
                *lookup_data.range_check_9_9_b_2 = [mul_res_limb_2_col121, mul_res_limb_3_col122];
                *sub_component_inputs.range_check_9_9_c[2] =
                    [mul_res_limb_4_col123, mul_res_limb_5_col124];
                *lookup_data.range_check_9_9_c_2 = [mul_res_limb_4_col123, mul_res_limb_5_col124];
                *sub_component_inputs.range_check_9_9_d[2] =
                    [mul_res_limb_6_col125, mul_res_limb_7_col126];
                *lookup_data.range_check_9_9_d_2 = [mul_res_limb_6_col125, mul_res_limb_7_col126];
                *sub_component_inputs.range_check_9_9_e[2] =
                    [mul_res_limb_8_col127, mul_res_limb_9_col128];
                *lookup_data.range_check_9_9_e_2 = [mul_res_limb_8_col127, mul_res_limb_9_col128];
                *sub_component_inputs.range_check_9_9_f[2] =
                    [mul_res_limb_10_col129, mul_res_limb_11_col130];
                *lookup_data.range_check_9_9_f_2 = [mul_res_limb_10_col129, mul_res_limb_11_col130];
                *sub_component_inputs.range_check_9_9_g[1] =
                    [mul_res_limb_12_col131, mul_res_limb_13_col132];
                *lookup_data.range_check_9_9_g_1 = [mul_res_limb_12_col131, mul_res_limb_13_col132];
                *sub_component_inputs.range_check_9_9_h[1] =
                    [mul_res_limb_14_col133, mul_res_limb_15_col134];
                *lookup_data.range_check_9_9_h_1 = [mul_res_limb_14_col133, mul_res_limb_15_col134];
                *sub_component_inputs.range_check_9_9[3] =
                    [mul_res_limb_16_col135, mul_res_limb_17_col136];
                *lookup_data.range_check_9_9_3 = [mul_res_limb_16_col135, mul_res_limb_17_col136];
                *sub_component_inputs.range_check_9_9_b[3] =
                    [mul_res_limb_18_col137, mul_res_limb_19_col138];
                *lookup_data.range_check_9_9_b_3 = [mul_res_limb_18_col137, mul_res_limb_19_col138];
                *sub_component_inputs.range_check_9_9_c[3] =
                    [mul_res_limb_20_col139, mul_res_limb_21_col140];
                *lookup_data.range_check_9_9_c_3 = [mul_res_limb_20_col139, mul_res_limb_21_col140];
                *sub_component_inputs.range_check_9_9_d[3] =
                    [mul_res_limb_22_col141, mul_res_limb_23_col142];
                *lookup_data.range_check_9_9_d_3 = [mul_res_limb_22_col141, mul_res_limb_23_col142];
                *sub_component_inputs.range_check_9_9_e[3] =
                    [mul_res_limb_24_col143, mul_res_limb_25_col144];
                *lookup_data.range_check_9_9_e_3 = [mul_res_limb_24_col143, mul_res_limb_25_col144];
                *sub_component_inputs.range_check_9_9_f[3] =
                    [mul_res_limb_26_col145, mul_res_limb_27_col146];
                *lookup_data.range_check_9_9_f_3 = [mul_res_limb_26_col145, mul_res_limb_27_col146];

                // Verify Mul 252.

                // Double Karatsuba N 7 Limb Max Bound 511.

                // Single Karatsuba N 7.

                let z0_tmp_3172c_47 = [
                    ((op0_limb_0_col32) * (op1_limb_0_col62)),
                    (((op0_limb_0_col32) * (op1_limb_1_col63))
                        + ((op0_limb_1_col33) * (op1_limb_0_col62))),
                    ((((op0_limb_0_col32) * (op1_limb_2_col64))
                        + ((op0_limb_1_col33) * (op1_limb_1_col63)))
                        + ((op0_limb_2_col34) * (op1_limb_0_col62))),
                    (((((op0_limb_0_col32) * (op1_limb_3_col65))
                        + ((op0_limb_1_col33) * (op1_limb_2_col64)))
                        + ((op0_limb_2_col34) * (op1_limb_1_col63)))
                        + ((op0_limb_3_col35) * (op1_limb_0_col62))),
                    ((((((op0_limb_0_col32) * (op1_limb_4_col66))
                        + ((op0_limb_1_col33) * (op1_limb_3_col65)))
                        + ((op0_limb_2_col34) * (op1_limb_2_col64)))
                        + ((op0_limb_3_col35) * (op1_limb_1_col63)))
                        + ((op0_limb_4_col36) * (op1_limb_0_col62))),
                    (((((((op0_limb_0_col32) * (op1_limb_5_col67))
                        + ((op0_limb_1_col33) * (op1_limb_4_col66)))
                        + ((op0_limb_2_col34) * (op1_limb_3_col65)))
                        + ((op0_limb_3_col35) * (op1_limb_2_col64)))
                        + ((op0_limb_4_col36) * (op1_limb_1_col63)))
                        + ((op0_limb_5_col37) * (op1_limb_0_col62))),
                    ((((((((op0_limb_0_col32) * (op1_limb_6_col68))
                        + ((op0_limb_1_col33) * (op1_limb_5_col67)))
                        + ((op0_limb_2_col34) * (op1_limb_4_col66)))
                        + ((op0_limb_3_col35) * (op1_limb_3_col65)))
                        + ((op0_limb_4_col36) * (op1_limb_2_col64)))
                        + ((op0_limb_5_col37) * (op1_limb_1_col63)))
                        + ((op0_limb_6_col38) * (op1_limb_0_col62))),
                    (((((((op0_limb_1_col33) * (op1_limb_6_col68))
                        + ((op0_limb_2_col34) * (op1_limb_5_col67)))
                        + ((op0_limb_3_col35) * (op1_limb_4_col66)))
                        + ((op0_limb_4_col36) * (op1_limb_3_col65)))
                        + ((op0_limb_5_col37) * (op1_limb_2_col64)))
                        + ((op0_limb_6_col38) * (op1_limb_1_col63))),
                    ((((((op0_limb_2_col34) * (op1_limb_6_col68))
                        + ((op0_limb_3_col35) * (op1_limb_5_col67)))
                        + ((op0_limb_4_col36) * (op1_limb_4_col66)))
                        + ((op0_limb_5_col37) * (op1_limb_3_col65)))
                        + ((op0_limb_6_col38) * (op1_limb_2_col64))),
                    (((((op0_limb_3_col35) * (op1_limb_6_col68))
                        + ((op0_limb_4_col36) * (op1_limb_5_col67)))
                        + ((op0_limb_5_col37) * (op1_limb_4_col66)))
                        + ((op0_limb_6_col38) * (op1_limb_3_col65))),
                    ((((op0_limb_4_col36) * (op1_limb_6_col68))
                        + ((op0_limb_5_col37) * (op1_limb_5_col67)))
                        + ((op0_limb_6_col38) * (op1_limb_4_col66))),
                    (((op0_limb_5_col37) * (op1_limb_6_col68))
                        + ((op0_limb_6_col38) * (op1_limb_5_col67))),
                    ((op0_limb_6_col38) * (op1_limb_6_col68)),
                ];
                let z2_tmp_3172c_48 = [
                    ((op0_limb_7_col39) * (op1_limb_7_col69)),
                    (((op0_limb_7_col39) * (op1_limb_8_col70))
                        + ((op0_limb_8_col40) * (op1_limb_7_col69))),
                    ((((op0_limb_7_col39) * (op1_limb_9_col71))
                        + ((op0_limb_8_col40) * (op1_limb_8_col70)))
                        + ((op0_limb_9_col41) * (op1_limb_7_col69))),
                    (((((op0_limb_7_col39) * (op1_limb_10_col72))
                        + ((op0_limb_8_col40) * (op1_limb_9_col71)))
                        + ((op0_limb_9_col41) * (op1_limb_8_col70)))
                        + ((op0_limb_10_col42) * (op1_limb_7_col69))),
                    ((((((op0_limb_7_col39) * (op1_limb_11_col73))
                        + ((op0_limb_8_col40) * (op1_limb_10_col72)))
                        + ((op0_limb_9_col41) * (op1_limb_9_col71)))
                        + ((op0_limb_10_col42) * (op1_limb_8_col70)))
                        + ((op0_limb_11_col43) * (op1_limb_7_col69))),
                    (((((((op0_limb_7_col39) * (op1_limb_12_col74))
                        + ((op0_limb_8_col40) * (op1_limb_11_col73)))
                        + ((op0_limb_9_col41) * (op1_limb_10_col72)))
                        + ((op0_limb_10_col42) * (op1_limb_9_col71)))
                        + ((op0_limb_11_col43) * (op1_limb_8_col70)))
                        + ((op0_limb_12_col44) * (op1_limb_7_col69))),
                    ((((((((op0_limb_7_col39) * (op1_limb_13_col75))
                        + ((op0_limb_8_col40) * (op1_limb_12_col74)))
                        + ((op0_limb_9_col41) * (op1_limb_11_col73)))
                        + ((op0_limb_10_col42) * (op1_limb_10_col72)))
                        + ((op0_limb_11_col43) * (op1_limb_9_col71)))
                        + ((op0_limb_12_col44) * (op1_limb_8_col70)))
                        + ((op0_limb_13_col45) * (op1_limb_7_col69))),
                    (((((((op0_limb_8_col40) * (op1_limb_13_col75))
                        + ((op0_limb_9_col41) * (op1_limb_12_col74)))
                        + ((op0_limb_10_col42) * (op1_limb_11_col73)))
                        + ((op0_limb_11_col43) * (op1_limb_10_col72)))
                        + ((op0_limb_12_col44) * (op1_limb_9_col71)))
                        + ((op0_limb_13_col45) * (op1_limb_8_col70))),
                    ((((((op0_limb_9_col41) * (op1_limb_13_col75))
                        + ((op0_limb_10_col42) * (op1_limb_12_col74)))
                        + ((op0_limb_11_col43) * (op1_limb_11_col73)))
                        + ((op0_limb_12_col44) * (op1_limb_10_col72)))
                        + ((op0_limb_13_col45) * (op1_limb_9_col71))),
                    (((((op0_limb_10_col42) * (op1_limb_13_col75))
                        + ((op0_limb_11_col43) * (op1_limb_12_col74)))
                        + ((op0_limb_12_col44) * (op1_limb_11_col73)))
                        + ((op0_limb_13_col45) * (op1_limb_10_col72))),
                    ((((op0_limb_11_col43) * (op1_limb_13_col75))
                        + ((op0_limb_12_col44) * (op1_limb_12_col74)))
                        + ((op0_limb_13_col45) * (op1_limb_11_col73))),
                    (((op0_limb_12_col44) * (op1_limb_13_col75))
                        + ((op0_limb_13_col45) * (op1_limb_12_col74))),
                    ((op0_limb_13_col45) * (op1_limb_13_col75)),
                ];
                let x_sum_tmp_3172c_49 = [
                    ((op0_limb_0_col32) + (op0_limb_7_col39)),
                    ((op0_limb_1_col33) + (op0_limb_8_col40)),
                    ((op0_limb_2_col34) + (op0_limb_9_col41)),
                    ((op0_limb_3_col35) + (op0_limb_10_col42)),
                    ((op0_limb_4_col36) + (op0_limb_11_col43)),
                    ((op0_limb_5_col37) + (op0_limb_12_col44)),
                    ((op0_limb_6_col38) + (op0_limb_13_col45)),
                ];
                let y_sum_tmp_3172c_50 = [
                    ((op1_limb_0_col62) + (op1_limb_7_col69)),
                    ((op1_limb_1_col63) + (op1_limb_8_col70)),
                    ((op1_limb_2_col64) + (op1_limb_9_col71)),
                    ((op1_limb_3_col65) + (op1_limb_10_col72)),
                    ((op1_limb_4_col66) + (op1_limb_11_col73)),
                    ((op1_limb_5_col67) + (op1_limb_12_col74)),
                    ((op1_limb_6_col68) + (op1_limb_13_col75)),
                ];
                let single_karatsuba_n_7_output_tmp_3172c_51 = [
                    z0_tmp_3172c_47[0],
                    z0_tmp_3172c_47[1],
                    z0_tmp_3172c_47[2],
                    z0_tmp_3172c_47[3],
                    z0_tmp_3172c_47[4],
                    z0_tmp_3172c_47[5],
                    z0_tmp_3172c_47[6],
                    ((z0_tmp_3172c_47[7])
                        + ((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[0]))
                            - (z0_tmp_3172c_47[0]))
                            - (z2_tmp_3172c_48[0]))),
                    ((z0_tmp_3172c_47[8])
                        + (((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[1]))
                            + ((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[0])))
                            - (z0_tmp_3172c_47[1]))
                            - (z2_tmp_3172c_48[1]))),
                    ((z0_tmp_3172c_47[9])
                        + ((((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[2]))
                            + ((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[1])))
                            + ((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[0])))
                            - (z0_tmp_3172c_47[2]))
                            - (z2_tmp_3172c_48[2]))),
                    ((z0_tmp_3172c_47[10])
                        + (((((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[3]))
                            + ((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[2])))
                            + ((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[1])))
                            + ((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[0])))
                            - (z0_tmp_3172c_47[3]))
                            - (z2_tmp_3172c_48[3]))),
                    ((z0_tmp_3172c_47[11])
                        + ((((((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[4]))
                            + ((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[3])))
                            + ((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[2])))
                            + ((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[1])))
                            + ((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[0])))
                            - (z0_tmp_3172c_47[4]))
                            - (z2_tmp_3172c_48[4]))),
                    ((z0_tmp_3172c_47[12])
                        + (((((((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[5]))
                            + ((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[4])))
                            + ((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[3])))
                            + ((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[2])))
                            + ((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[1])))
                            + ((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[0])))
                            - (z0_tmp_3172c_47[5]))
                            - (z2_tmp_3172c_48[5]))),
                    ((((((((((x_sum_tmp_3172c_49[0]) * (y_sum_tmp_3172c_50[6]))
                        + ((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[5])))
                        + ((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[4])))
                        + ((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[3])))
                        + ((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[2])))
                        + ((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[1])))
                        + ((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[0])))
                        - (z0_tmp_3172c_47[6]))
                        - (z2_tmp_3172c_48[6])),
                    ((z2_tmp_3172c_48[0])
                        + (((((((((x_sum_tmp_3172c_49[1]) * (y_sum_tmp_3172c_50[6]))
                            + ((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[5])))
                            + ((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[4])))
                            + ((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[3])))
                            + ((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[2])))
                            + ((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[1])))
                            - (z0_tmp_3172c_47[7]))
                            - (z2_tmp_3172c_48[7]))),
                    ((z2_tmp_3172c_48[1])
                        + ((((((((x_sum_tmp_3172c_49[2]) * (y_sum_tmp_3172c_50[6]))
                            + ((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[5])))
                            + ((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[4])))
                            + ((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[3])))
                            + ((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[2])))
                            - (z0_tmp_3172c_47[8]))
                            - (z2_tmp_3172c_48[8]))),
                    ((z2_tmp_3172c_48[2])
                        + (((((((x_sum_tmp_3172c_49[3]) * (y_sum_tmp_3172c_50[6]))
                            + ((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[5])))
                            + ((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[4])))
                            + ((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[3])))
                            - (z0_tmp_3172c_47[9]))
                            - (z2_tmp_3172c_48[9]))),
                    ((z2_tmp_3172c_48[3])
                        + ((((((x_sum_tmp_3172c_49[4]) * (y_sum_tmp_3172c_50[6]))
                            + ((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[5])))
                            + ((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[4])))
                            - (z0_tmp_3172c_47[10]))
                            - (z2_tmp_3172c_48[10]))),
                    ((z2_tmp_3172c_48[4])
                        + (((((x_sum_tmp_3172c_49[5]) * (y_sum_tmp_3172c_50[6]))
                            + ((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[5])))
                            - (z0_tmp_3172c_47[11]))
                            - (z2_tmp_3172c_48[11]))),
                    ((z2_tmp_3172c_48[5])
                        + ((((x_sum_tmp_3172c_49[6]) * (y_sum_tmp_3172c_50[6]))
                            - (z0_tmp_3172c_47[12]))
                            - (z2_tmp_3172c_48[12]))),
                    z2_tmp_3172c_48[6],
                    z2_tmp_3172c_48[7],
                    z2_tmp_3172c_48[8],
                    z2_tmp_3172c_48[9],
                    z2_tmp_3172c_48[10],
                    z2_tmp_3172c_48[11],
                    z2_tmp_3172c_48[12],
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3172c_52 = [
                    ((op0_limb_14_col46) * (op1_limb_14_col76)),
                    (((op0_limb_14_col46) * (op1_limb_15_col77))
                        + ((op0_limb_15_col47) * (op1_limb_14_col76))),
                    ((((op0_limb_14_col46) * (op1_limb_16_col78))
                        + ((op0_limb_15_col47) * (op1_limb_15_col77)))
                        + ((op0_limb_16_col48) * (op1_limb_14_col76))),
                    (((((op0_limb_14_col46) * (op1_limb_17_col79))
                        + ((op0_limb_15_col47) * (op1_limb_16_col78)))
                        + ((op0_limb_16_col48) * (op1_limb_15_col77)))
                        + ((op0_limb_17_col49) * (op1_limb_14_col76))),
                    ((((((op0_limb_14_col46) * (op1_limb_18_col80))
                        + ((op0_limb_15_col47) * (op1_limb_17_col79)))
                        + ((op0_limb_16_col48) * (op1_limb_16_col78)))
                        + ((op0_limb_17_col49) * (op1_limb_15_col77)))
                        + ((op0_limb_18_col50) * (op1_limb_14_col76))),
                    (((((((op0_limb_14_col46) * (op1_limb_19_col81))
                        + ((op0_limb_15_col47) * (op1_limb_18_col80)))
                        + ((op0_limb_16_col48) * (op1_limb_17_col79)))
                        + ((op0_limb_17_col49) * (op1_limb_16_col78)))
                        + ((op0_limb_18_col50) * (op1_limb_15_col77)))
                        + ((op0_limb_19_col51) * (op1_limb_14_col76))),
                    ((((((((op0_limb_14_col46) * (op1_limb_20_col82))
                        + ((op0_limb_15_col47) * (op1_limb_19_col81)))
                        + ((op0_limb_16_col48) * (op1_limb_18_col80)))
                        + ((op0_limb_17_col49) * (op1_limb_17_col79)))
                        + ((op0_limb_18_col50) * (op1_limb_16_col78)))
                        + ((op0_limb_19_col51) * (op1_limb_15_col77)))
                        + ((op0_limb_20_col52) * (op1_limb_14_col76))),
                    (((((((op0_limb_15_col47) * (op1_limb_20_col82))
                        + ((op0_limb_16_col48) * (op1_limb_19_col81)))
                        + ((op0_limb_17_col49) * (op1_limb_18_col80)))
                        + ((op0_limb_18_col50) * (op1_limb_17_col79)))
                        + ((op0_limb_19_col51) * (op1_limb_16_col78)))
                        + ((op0_limb_20_col52) * (op1_limb_15_col77))),
                    ((((((op0_limb_16_col48) * (op1_limb_20_col82))
                        + ((op0_limb_17_col49) * (op1_limb_19_col81)))
                        + ((op0_limb_18_col50) * (op1_limb_18_col80)))
                        + ((op0_limb_19_col51) * (op1_limb_17_col79)))
                        + ((op0_limb_20_col52) * (op1_limb_16_col78))),
                    (((((op0_limb_17_col49) * (op1_limb_20_col82))
                        + ((op0_limb_18_col50) * (op1_limb_19_col81)))
                        + ((op0_limb_19_col51) * (op1_limb_18_col80)))
                        + ((op0_limb_20_col52) * (op1_limb_17_col79))),
                    ((((op0_limb_18_col50) * (op1_limb_20_col82))
                        + ((op0_limb_19_col51) * (op1_limb_19_col81)))
                        + ((op0_limb_20_col52) * (op1_limb_18_col80))),
                    (((op0_limb_19_col51) * (op1_limb_20_col82))
                        + ((op0_limb_20_col52) * (op1_limb_19_col81))),
                    ((op0_limb_20_col52) * (op1_limb_20_col82)),
                ];
                let z2_tmp_3172c_53 = [
                    ((op0_limb_21_col53) * (op1_limb_21_col83)),
                    (((op0_limb_21_col53) * (op1_limb_22_col84))
                        + ((op0_limb_22_col54) * (op1_limb_21_col83))),
                    ((((op0_limb_21_col53) * (op1_limb_23_col85))
                        + ((op0_limb_22_col54) * (op1_limb_22_col84)))
                        + ((op0_limb_23_col55) * (op1_limb_21_col83))),
                    (((((op0_limb_21_col53) * (op1_limb_24_col86))
                        + ((op0_limb_22_col54) * (op1_limb_23_col85)))
                        + ((op0_limb_23_col55) * (op1_limb_22_col84)))
                        + ((op0_limb_24_col56) * (op1_limb_21_col83))),
                    ((((((op0_limb_21_col53) * (op1_limb_25_col87))
                        + ((op0_limb_22_col54) * (op1_limb_24_col86)))
                        + ((op0_limb_23_col55) * (op1_limb_23_col85)))
                        + ((op0_limb_24_col56) * (op1_limb_22_col84)))
                        + ((op0_limb_25_col57) * (op1_limb_21_col83))),
                    (((((((op0_limb_21_col53) * (op1_limb_26_col88))
                        + ((op0_limb_22_col54) * (op1_limb_25_col87)))
                        + ((op0_limb_23_col55) * (op1_limb_24_col86)))
                        + ((op0_limb_24_col56) * (op1_limb_23_col85)))
                        + ((op0_limb_25_col57) * (op1_limb_22_col84)))
                        + ((op0_limb_26_col58) * (op1_limb_21_col83))),
                    ((((((((op0_limb_21_col53) * (op1_limb_27_col89))
                        + ((op0_limb_22_col54) * (op1_limb_26_col88)))
                        + ((op0_limb_23_col55) * (op1_limb_25_col87)))
                        + ((op0_limb_24_col56) * (op1_limb_24_col86)))
                        + ((op0_limb_25_col57) * (op1_limb_23_col85)))
                        + ((op0_limb_26_col58) * (op1_limb_22_col84)))
                        + ((op0_limb_27_col59) * (op1_limb_21_col83))),
                    (((((((op0_limb_22_col54) * (op1_limb_27_col89))
                        + ((op0_limb_23_col55) * (op1_limb_26_col88)))
                        + ((op0_limb_24_col56) * (op1_limb_25_col87)))
                        + ((op0_limb_25_col57) * (op1_limb_24_col86)))
                        + ((op0_limb_26_col58) * (op1_limb_23_col85)))
                        + ((op0_limb_27_col59) * (op1_limb_22_col84))),
                    ((((((op0_limb_23_col55) * (op1_limb_27_col89))
                        + ((op0_limb_24_col56) * (op1_limb_26_col88)))
                        + ((op0_limb_25_col57) * (op1_limb_25_col87)))
                        + ((op0_limb_26_col58) * (op1_limb_24_col86)))
                        + ((op0_limb_27_col59) * (op1_limb_23_col85))),
                    (((((op0_limb_24_col56) * (op1_limb_27_col89))
                        + ((op0_limb_25_col57) * (op1_limb_26_col88)))
                        + ((op0_limb_26_col58) * (op1_limb_25_col87)))
                        + ((op0_limb_27_col59) * (op1_limb_24_col86))),
                    ((((op0_limb_25_col57) * (op1_limb_27_col89))
                        + ((op0_limb_26_col58) * (op1_limb_26_col88)))
                        + ((op0_limb_27_col59) * (op1_limb_25_col87))),
                    (((op0_limb_26_col58) * (op1_limb_27_col89))
                        + ((op0_limb_27_col59) * (op1_limb_26_col88))),
                    ((op0_limb_27_col59) * (op1_limb_27_col89)),
                ];
                let x_sum_tmp_3172c_54 = [
                    ((op0_limb_14_col46) + (op0_limb_21_col53)),
                    ((op0_limb_15_col47) + (op0_limb_22_col54)),
                    ((op0_limb_16_col48) + (op0_limb_23_col55)),
                    ((op0_limb_17_col49) + (op0_limb_24_col56)),
                    ((op0_limb_18_col50) + (op0_limb_25_col57)),
                    ((op0_limb_19_col51) + (op0_limb_26_col58)),
                    ((op0_limb_20_col52) + (op0_limb_27_col59)),
                ];
                let y_sum_tmp_3172c_55 = [
                    ((op1_limb_14_col76) + (op1_limb_21_col83)),
                    ((op1_limb_15_col77) + (op1_limb_22_col84)),
                    ((op1_limb_16_col78) + (op1_limb_23_col85)),
                    ((op1_limb_17_col79) + (op1_limb_24_col86)),
                    ((op1_limb_18_col80) + (op1_limb_25_col87)),
                    ((op1_limb_19_col81) + (op1_limb_26_col88)),
                    ((op1_limb_20_col82) + (op1_limb_27_col89)),
                ];
                let single_karatsuba_n_7_output_tmp_3172c_56 = [
                    z0_tmp_3172c_52[0],
                    z0_tmp_3172c_52[1],
                    z0_tmp_3172c_52[2],
                    z0_tmp_3172c_52[3],
                    z0_tmp_3172c_52[4],
                    z0_tmp_3172c_52[5],
                    z0_tmp_3172c_52[6],
                    ((z0_tmp_3172c_52[7])
                        + ((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[0]))
                            - (z0_tmp_3172c_52[0]))
                            - (z2_tmp_3172c_53[0]))),
                    ((z0_tmp_3172c_52[8])
                        + (((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[1]))
                            + ((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[0])))
                            - (z0_tmp_3172c_52[1]))
                            - (z2_tmp_3172c_53[1]))),
                    ((z0_tmp_3172c_52[9])
                        + ((((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[2]))
                            + ((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[1])))
                            + ((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[0])))
                            - (z0_tmp_3172c_52[2]))
                            - (z2_tmp_3172c_53[2]))),
                    ((z0_tmp_3172c_52[10])
                        + (((((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[3]))
                            + ((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[2])))
                            + ((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[1])))
                            + ((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[0])))
                            - (z0_tmp_3172c_52[3]))
                            - (z2_tmp_3172c_53[3]))),
                    ((z0_tmp_3172c_52[11])
                        + ((((((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[4]))
                            + ((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[3])))
                            + ((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[2])))
                            + ((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[1])))
                            + ((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[0])))
                            - (z0_tmp_3172c_52[4]))
                            - (z2_tmp_3172c_53[4]))),
                    ((z0_tmp_3172c_52[12])
                        + (((((((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[5]))
                            + ((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[4])))
                            + ((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[3])))
                            + ((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[2])))
                            + ((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[1])))
                            + ((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[0])))
                            - (z0_tmp_3172c_52[5]))
                            - (z2_tmp_3172c_53[5]))),
                    ((((((((((x_sum_tmp_3172c_54[0]) * (y_sum_tmp_3172c_55[6]))
                        + ((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[5])))
                        + ((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[4])))
                        + ((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[3])))
                        + ((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[2])))
                        + ((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[1])))
                        + ((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[0])))
                        - (z0_tmp_3172c_52[6]))
                        - (z2_tmp_3172c_53[6])),
                    ((z2_tmp_3172c_53[0])
                        + (((((((((x_sum_tmp_3172c_54[1]) * (y_sum_tmp_3172c_55[6]))
                            + ((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[5])))
                            + ((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[4])))
                            + ((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[3])))
                            + ((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[2])))
                            + ((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[1])))
                            - (z0_tmp_3172c_52[7]))
                            - (z2_tmp_3172c_53[7]))),
                    ((z2_tmp_3172c_53[1])
                        + ((((((((x_sum_tmp_3172c_54[2]) * (y_sum_tmp_3172c_55[6]))
                            + ((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[5])))
                            + ((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[4])))
                            + ((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[3])))
                            + ((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[2])))
                            - (z0_tmp_3172c_52[8]))
                            - (z2_tmp_3172c_53[8]))),
                    ((z2_tmp_3172c_53[2])
                        + (((((((x_sum_tmp_3172c_54[3]) * (y_sum_tmp_3172c_55[6]))
                            + ((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[5])))
                            + ((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[4])))
                            + ((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[3])))
                            - (z0_tmp_3172c_52[9]))
                            - (z2_tmp_3172c_53[9]))),
                    ((z2_tmp_3172c_53[3])
                        + ((((((x_sum_tmp_3172c_54[4]) * (y_sum_tmp_3172c_55[6]))
                            + ((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[5])))
                            + ((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[4])))
                            - (z0_tmp_3172c_52[10]))
                            - (z2_tmp_3172c_53[10]))),
                    ((z2_tmp_3172c_53[4])
                        + (((((x_sum_tmp_3172c_54[5]) * (y_sum_tmp_3172c_55[6]))
                            + ((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[5])))
                            - (z0_tmp_3172c_52[11]))
                            - (z2_tmp_3172c_53[11]))),
                    ((z2_tmp_3172c_53[5])
                        + ((((x_sum_tmp_3172c_54[6]) * (y_sum_tmp_3172c_55[6]))
                            - (z0_tmp_3172c_52[12]))
                            - (z2_tmp_3172c_53[12]))),
                    z2_tmp_3172c_53[6],
                    z2_tmp_3172c_53[7],
                    z2_tmp_3172c_53[8],
                    z2_tmp_3172c_53[9],
                    z2_tmp_3172c_53[10],
                    z2_tmp_3172c_53[11],
                    z2_tmp_3172c_53[12],
                ];

                let x_sum_tmp_3172c_57 = [
                    ((op0_limb_0_col32) + (op0_limb_14_col46)),
                    ((op0_limb_1_col33) + (op0_limb_15_col47)),
                    ((op0_limb_2_col34) + (op0_limb_16_col48)),
                    ((op0_limb_3_col35) + (op0_limb_17_col49)),
                    ((op0_limb_4_col36) + (op0_limb_18_col50)),
                    ((op0_limb_5_col37) + (op0_limb_19_col51)),
                    ((op0_limb_6_col38) + (op0_limb_20_col52)),
                    ((op0_limb_7_col39) + (op0_limb_21_col53)),
                    ((op0_limb_8_col40) + (op0_limb_22_col54)),
                    ((op0_limb_9_col41) + (op0_limb_23_col55)),
                    ((op0_limb_10_col42) + (op0_limb_24_col56)),
                    ((op0_limb_11_col43) + (op0_limb_25_col57)),
                    ((op0_limb_12_col44) + (op0_limb_26_col58)),
                    ((op0_limb_13_col45) + (op0_limb_27_col59)),
                ];
                let y_sum_tmp_3172c_58 = [
                    ((op1_limb_0_col62) + (op1_limb_14_col76)),
                    ((op1_limb_1_col63) + (op1_limb_15_col77)),
                    ((op1_limb_2_col64) + (op1_limb_16_col78)),
                    ((op1_limb_3_col65) + (op1_limb_17_col79)),
                    ((op1_limb_4_col66) + (op1_limb_18_col80)),
                    ((op1_limb_5_col67) + (op1_limb_19_col81)),
                    ((op1_limb_6_col68) + (op1_limb_20_col82)),
                    ((op1_limb_7_col69) + (op1_limb_21_col83)),
                    ((op1_limb_8_col70) + (op1_limb_22_col84)),
                    ((op1_limb_9_col71) + (op1_limb_23_col85)),
                    ((op1_limb_10_col72) + (op1_limb_24_col86)),
                    ((op1_limb_11_col73) + (op1_limb_25_col87)),
                    ((op1_limb_12_col74) + (op1_limb_26_col88)),
                    ((op1_limb_13_col75) + (op1_limb_27_col89)),
                ];

                // Single Karatsuba N 7.

                let z0_tmp_3172c_59 = [
                    ((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[0])),
                    (((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[1]))
                        + ((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[0]))),
                    ((((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[2]))
                        + ((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[1])))
                        + ((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[0]))),
                    (((((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[3]))
                        + ((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[2])))
                        + ((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[1])))
                        + ((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[0]))),
                    ((((((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[4]))
                        + ((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[3])))
                        + ((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[2])))
                        + ((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[1])))
                        + ((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[0]))),
                    (((((((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[5]))
                        + ((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[4])))
                        + ((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[3])))
                        + ((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[2])))
                        + ((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[1])))
                        + ((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[0]))),
                    ((((((((x_sum_tmp_3172c_57[0]) * (y_sum_tmp_3172c_58[6]))
                        + ((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[5])))
                        + ((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[4])))
                        + ((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[3])))
                        + ((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[2])))
                        + ((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[1])))
                        + ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[0]))),
                    (((((((x_sum_tmp_3172c_57[1]) * (y_sum_tmp_3172c_58[6]))
                        + ((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[5])))
                        + ((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[4])))
                        + ((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[3])))
                        + ((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[2])))
                        + ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[1]))),
                    ((((((x_sum_tmp_3172c_57[2]) * (y_sum_tmp_3172c_58[6]))
                        + ((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[5])))
                        + ((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[4])))
                        + ((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[3])))
                        + ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[2]))),
                    (((((x_sum_tmp_3172c_57[3]) * (y_sum_tmp_3172c_58[6]))
                        + ((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[5])))
                        + ((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[4])))
                        + ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[3]))),
                    ((((x_sum_tmp_3172c_57[4]) * (y_sum_tmp_3172c_58[6]))
                        + ((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[5])))
                        + ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[4]))),
                    (((x_sum_tmp_3172c_57[5]) * (y_sum_tmp_3172c_58[6]))
                        + ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[5]))),
                    ((x_sum_tmp_3172c_57[6]) * (y_sum_tmp_3172c_58[6])),
                ];
                let z2_tmp_3172c_60 = [
                    ((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[7])),
                    (((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[8]))
                        + ((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[7]))),
                    ((((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[9]))
                        + ((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[8])))
                        + ((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[7]))),
                    (((((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[10]))
                        + ((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[9])))
                        + ((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[8])))
                        + ((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[7]))),
                    ((((((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[11]))
                        + ((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[10])))
                        + ((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[9])))
                        + ((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[8])))
                        + ((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[7]))),
                    (((((((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[12]))
                        + ((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[11])))
                        + ((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[10])))
                        + ((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[9])))
                        + ((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[8])))
                        + ((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[7]))),
                    ((((((((x_sum_tmp_3172c_57[7]) * (y_sum_tmp_3172c_58[13]))
                        + ((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[12])))
                        + ((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[11])))
                        + ((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[10])))
                        + ((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[9])))
                        + ((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[8])))
                        + ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[7]))),
                    (((((((x_sum_tmp_3172c_57[8]) * (y_sum_tmp_3172c_58[13]))
                        + ((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[12])))
                        + ((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[11])))
                        + ((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[10])))
                        + ((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[9])))
                        + ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[8]))),
                    ((((((x_sum_tmp_3172c_57[9]) * (y_sum_tmp_3172c_58[13]))
                        + ((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[12])))
                        + ((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[11])))
                        + ((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[10])))
                        + ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[9]))),
                    (((((x_sum_tmp_3172c_57[10]) * (y_sum_tmp_3172c_58[13]))
                        + ((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[12])))
                        + ((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[11])))
                        + ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[10]))),
                    ((((x_sum_tmp_3172c_57[11]) * (y_sum_tmp_3172c_58[13]))
                        + ((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[12])))
                        + ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[11]))),
                    (((x_sum_tmp_3172c_57[12]) * (y_sum_tmp_3172c_58[13]))
                        + ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[12]))),
                    ((x_sum_tmp_3172c_57[13]) * (y_sum_tmp_3172c_58[13])),
                ];
                let x_sum_tmp_3172c_61 = [
                    ((x_sum_tmp_3172c_57[0]) + (x_sum_tmp_3172c_57[7])),
                    ((x_sum_tmp_3172c_57[1]) + (x_sum_tmp_3172c_57[8])),
                    ((x_sum_tmp_3172c_57[2]) + (x_sum_tmp_3172c_57[9])),
                    ((x_sum_tmp_3172c_57[3]) + (x_sum_tmp_3172c_57[10])),
                    ((x_sum_tmp_3172c_57[4]) + (x_sum_tmp_3172c_57[11])),
                    ((x_sum_tmp_3172c_57[5]) + (x_sum_tmp_3172c_57[12])),
                    ((x_sum_tmp_3172c_57[6]) + (x_sum_tmp_3172c_57[13])),
                ];
                let y_sum_tmp_3172c_62 = [
                    ((y_sum_tmp_3172c_58[0]) + (y_sum_tmp_3172c_58[7])),
                    ((y_sum_tmp_3172c_58[1]) + (y_sum_tmp_3172c_58[8])),
                    ((y_sum_tmp_3172c_58[2]) + (y_sum_tmp_3172c_58[9])),
                    ((y_sum_tmp_3172c_58[3]) + (y_sum_tmp_3172c_58[10])),
                    ((y_sum_tmp_3172c_58[4]) + (y_sum_tmp_3172c_58[11])),
                    ((y_sum_tmp_3172c_58[5]) + (y_sum_tmp_3172c_58[12])),
                    ((y_sum_tmp_3172c_58[6]) + (y_sum_tmp_3172c_58[13])),
                ];
                let single_karatsuba_n_7_output_tmp_3172c_63 = [
                    z0_tmp_3172c_59[0],
                    z0_tmp_3172c_59[1],
                    z0_tmp_3172c_59[2],
                    z0_tmp_3172c_59[3],
                    z0_tmp_3172c_59[4],
                    z0_tmp_3172c_59[5],
                    z0_tmp_3172c_59[6],
                    ((z0_tmp_3172c_59[7])
                        + ((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[0]))
                            - (z0_tmp_3172c_59[0]))
                            - (z2_tmp_3172c_60[0]))),
                    ((z0_tmp_3172c_59[8])
                        + (((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[1]))
                            + ((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[0])))
                            - (z0_tmp_3172c_59[1]))
                            - (z2_tmp_3172c_60[1]))),
                    ((z0_tmp_3172c_59[9])
                        + ((((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[2]))
                            + ((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[1])))
                            + ((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[0])))
                            - (z0_tmp_3172c_59[2]))
                            - (z2_tmp_3172c_60[2]))),
                    ((z0_tmp_3172c_59[10])
                        + (((((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[3]))
                            + ((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[2])))
                            + ((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[1])))
                            + ((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[0])))
                            - (z0_tmp_3172c_59[3]))
                            - (z2_tmp_3172c_60[3]))),
                    ((z0_tmp_3172c_59[11])
                        + ((((((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[4]))
                            + ((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[3])))
                            + ((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[2])))
                            + ((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[1])))
                            + ((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[0])))
                            - (z0_tmp_3172c_59[4]))
                            - (z2_tmp_3172c_60[4]))),
                    ((z0_tmp_3172c_59[12])
                        + (((((((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[5]))
                            + ((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[4])))
                            + ((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[3])))
                            + ((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[2])))
                            + ((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[1])))
                            + ((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[0])))
                            - (z0_tmp_3172c_59[5]))
                            - (z2_tmp_3172c_60[5]))),
                    ((((((((((x_sum_tmp_3172c_61[0]) * (y_sum_tmp_3172c_62[6]))
                        + ((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[5])))
                        + ((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[4])))
                        + ((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[3])))
                        + ((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[2])))
                        + ((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[1])))
                        + ((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[0])))
                        - (z0_tmp_3172c_59[6]))
                        - (z2_tmp_3172c_60[6])),
                    ((z2_tmp_3172c_60[0])
                        + (((((((((x_sum_tmp_3172c_61[1]) * (y_sum_tmp_3172c_62[6]))
                            + ((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[5])))
                            + ((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[4])))
                            + ((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[3])))
                            + ((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[2])))
                            + ((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[1])))
                            - (z0_tmp_3172c_59[7]))
                            - (z2_tmp_3172c_60[7]))),
                    ((z2_tmp_3172c_60[1])
                        + ((((((((x_sum_tmp_3172c_61[2]) * (y_sum_tmp_3172c_62[6]))
                            + ((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[5])))
                            + ((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[4])))
                            + ((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[3])))
                            + ((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[2])))
                            - (z0_tmp_3172c_59[8]))
                            - (z2_tmp_3172c_60[8]))),
                    ((z2_tmp_3172c_60[2])
                        + (((((((x_sum_tmp_3172c_61[3]) * (y_sum_tmp_3172c_62[6]))
                            + ((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[5])))
                            + ((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[4])))
                            + ((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[3])))
                            - (z0_tmp_3172c_59[9]))
                            - (z2_tmp_3172c_60[9]))),
                    ((z2_tmp_3172c_60[3])
                        + ((((((x_sum_tmp_3172c_61[4]) * (y_sum_tmp_3172c_62[6]))
                            + ((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[5])))
                            + ((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[4])))
                            - (z0_tmp_3172c_59[10]))
                            - (z2_tmp_3172c_60[10]))),
                    ((z2_tmp_3172c_60[4])
                        + (((((x_sum_tmp_3172c_61[5]) * (y_sum_tmp_3172c_62[6]))
                            + ((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[5])))
                            - (z0_tmp_3172c_59[11]))
                            - (z2_tmp_3172c_60[11]))),
                    ((z2_tmp_3172c_60[5])
                        + ((((x_sum_tmp_3172c_61[6]) * (y_sum_tmp_3172c_62[6]))
                            - (z0_tmp_3172c_59[12]))
                            - (z2_tmp_3172c_60[12]))),
                    z2_tmp_3172c_60[6],
                    z2_tmp_3172c_60[7],
                    z2_tmp_3172c_60[8],
                    z2_tmp_3172c_60[9],
                    z2_tmp_3172c_60[10],
                    z2_tmp_3172c_60[11],
                    z2_tmp_3172c_60[12],
                ];

                let double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64 = [
                    single_karatsuba_n_7_output_tmp_3172c_51[0],
                    single_karatsuba_n_7_output_tmp_3172c_51[1],
                    single_karatsuba_n_7_output_tmp_3172c_51[2],
                    single_karatsuba_n_7_output_tmp_3172c_51[3],
                    single_karatsuba_n_7_output_tmp_3172c_51[4],
                    single_karatsuba_n_7_output_tmp_3172c_51[5],
                    single_karatsuba_n_7_output_tmp_3172c_51[6],
                    single_karatsuba_n_7_output_tmp_3172c_51[7],
                    single_karatsuba_n_7_output_tmp_3172c_51[8],
                    single_karatsuba_n_7_output_tmp_3172c_51[9],
                    single_karatsuba_n_7_output_tmp_3172c_51[10],
                    single_karatsuba_n_7_output_tmp_3172c_51[11],
                    single_karatsuba_n_7_output_tmp_3172c_51[12],
                    single_karatsuba_n_7_output_tmp_3172c_51[13],
                    ((single_karatsuba_n_7_output_tmp_3172c_51[14])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[0])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[0]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[0]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[15])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[1])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[1]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[1]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[16])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[2])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[2]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[2]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[17])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[3])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[3]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[3]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[18])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[4])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[4]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[4]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[19])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[5])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[5]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[5]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[20])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[6])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[6]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[6]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[21])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[7])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[7]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[7]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[22])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[8])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[8]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[8]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[23])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[9])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[9]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[9]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[24])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[10])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[10]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[10]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[25])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[11])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[11]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[11]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_51[26])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[12])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[12]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[12]))),
                    (((single_karatsuba_n_7_output_tmp_3172c_63[13])
                        - (single_karatsuba_n_7_output_tmp_3172c_51[13]))
                        - (single_karatsuba_n_7_output_tmp_3172c_56[13])),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[0])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[14])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[14]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[14]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[1])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[15])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[15]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[15]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[2])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[16])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[16]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[16]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[3])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[17])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[17]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[17]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[4])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[18])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[18]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[18]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[5])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[19])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[19]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[19]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[6])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[20])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[20]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[20]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[7])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[21])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[21]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[21]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[8])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[22])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[22]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[22]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[9])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[23])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[23]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[23]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[10])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[24])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[24]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[24]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[11])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[25])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[25]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[25]))),
                    ((single_karatsuba_n_7_output_tmp_3172c_56[12])
                        + (((single_karatsuba_n_7_output_tmp_3172c_63[26])
                            - (single_karatsuba_n_7_output_tmp_3172c_51[26]))
                            - (single_karatsuba_n_7_output_tmp_3172c_56[26]))),
                    single_karatsuba_n_7_output_tmp_3172c_56[13],
                    single_karatsuba_n_7_output_tmp_3172c_56[14],
                    single_karatsuba_n_7_output_tmp_3172c_56[15],
                    single_karatsuba_n_7_output_tmp_3172c_56[16],
                    single_karatsuba_n_7_output_tmp_3172c_56[17],
                    single_karatsuba_n_7_output_tmp_3172c_56[18],
                    single_karatsuba_n_7_output_tmp_3172c_56[19],
                    single_karatsuba_n_7_output_tmp_3172c_56[20],
                    single_karatsuba_n_7_output_tmp_3172c_56[21],
                    single_karatsuba_n_7_output_tmp_3172c_56[22],
                    single_karatsuba_n_7_output_tmp_3172c_56[23],
                    single_karatsuba_n_7_output_tmp_3172c_56[24],
                    single_karatsuba_n_7_output_tmp_3172c_56[25],
                    single_karatsuba_n_7_output_tmp_3172c_56[26],
                ];

                let conv_tmp_3172c_65 = [
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[0])
                        - (mul_res_limb_0_col119)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[1])
                        - (mul_res_limb_1_col120)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[2])
                        - (mul_res_limb_2_col121)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[3])
                        - (mul_res_limb_3_col122)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[4])
                        - (mul_res_limb_4_col123)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[5])
                        - (mul_res_limb_5_col124)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[6])
                        - (mul_res_limb_6_col125)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[7])
                        - (mul_res_limb_7_col126)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[8])
                        - (mul_res_limb_8_col127)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[9])
                        - (mul_res_limb_9_col128)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[10])
                        - (mul_res_limb_10_col129)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[11])
                        - (mul_res_limb_11_col130)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[12])
                        - (mul_res_limb_12_col131)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[13])
                        - (mul_res_limb_13_col132)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[14])
                        - (mul_res_limb_14_col133)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[15])
                        - (mul_res_limb_15_col134)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[16])
                        - (mul_res_limb_16_col135)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[17])
                        - (mul_res_limb_17_col136)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[18])
                        - (mul_res_limb_18_col137)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[19])
                        - (mul_res_limb_19_col138)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[20])
                        - (mul_res_limb_20_col139)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[21])
                        - (mul_res_limb_21_col140)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[22])
                        - (mul_res_limb_22_col141)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[23])
                        - (mul_res_limb_23_col142)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[24])
                        - (mul_res_limb_24_col143)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[25])
                        - (mul_res_limb_25_col144)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[26])
                        - (mul_res_limb_26_col145)),
                    ((double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[27])
                        - (mul_res_limb_27_col146)),
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[28],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[29],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[30],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[31],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[32],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[33],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[34],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[35],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[36],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[37],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[38],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[39],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[40],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[41],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[42],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[43],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[44],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[45],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[46],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[47],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[48],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[49],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[50],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[51],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[52],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[53],
                    double_karatsuba_n_7_limb_max_bound_511_output_tmp_3172c_64[54],
                ];
                let conv_mod_tmp_3172c_66 = [
                    ((((M31_32) * (conv_tmp_3172c_65[0])) - ((M31_4) * (conv_tmp_3172c_65[21])))
                        + ((M31_8) * (conv_tmp_3172c_65[49]))),
                    ((((conv_tmp_3172c_65[0]) + ((M31_32) * (conv_tmp_3172c_65[1])))
                        - ((M31_4) * (conv_tmp_3172c_65[22])))
                        + ((M31_8) * (conv_tmp_3172c_65[50]))),
                    ((((conv_tmp_3172c_65[1]) + ((M31_32) * (conv_tmp_3172c_65[2])))
                        - ((M31_4) * (conv_tmp_3172c_65[23])))
                        + ((M31_8) * (conv_tmp_3172c_65[51]))),
                    ((((conv_tmp_3172c_65[2]) + ((M31_32) * (conv_tmp_3172c_65[3])))
                        - ((M31_4) * (conv_tmp_3172c_65[24])))
                        + ((M31_8) * (conv_tmp_3172c_65[52]))),
                    ((((conv_tmp_3172c_65[3]) + ((M31_32) * (conv_tmp_3172c_65[4])))
                        - ((M31_4) * (conv_tmp_3172c_65[25])))
                        + ((M31_8) * (conv_tmp_3172c_65[53]))),
                    ((((conv_tmp_3172c_65[4]) + ((M31_32) * (conv_tmp_3172c_65[5])))
                        - ((M31_4) * (conv_tmp_3172c_65[26])))
                        + ((M31_8) * (conv_tmp_3172c_65[54]))),
                    (((conv_tmp_3172c_65[5]) + ((M31_32) * (conv_tmp_3172c_65[6])))
                        - ((M31_4) * (conv_tmp_3172c_65[27]))),
                    (((((M31_2) * (conv_tmp_3172c_65[0])) + (conv_tmp_3172c_65[6]))
                        + ((M31_32) * (conv_tmp_3172c_65[7])))
                        - ((M31_4) * (conv_tmp_3172c_65[28]))),
                    (((((M31_2) * (conv_tmp_3172c_65[1])) + (conv_tmp_3172c_65[7]))
                        + ((M31_32) * (conv_tmp_3172c_65[8])))
                        - ((M31_4) * (conv_tmp_3172c_65[29]))),
                    (((((M31_2) * (conv_tmp_3172c_65[2])) + (conv_tmp_3172c_65[8]))
                        + ((M31_32) * (conv_tmp_3172c_65[9])))
                        - ((M31_4) * (conv_tmp_3172c_65[30]))),
                    (((((M31_2) * (conv_tmp_3172c_65[3])) + (conv_tmp_3172c_65[9]))
                        + ((M31_32) * (conv_tmp_3172c_65[10])))
                        - ((M31_4) * (conv_tmp_3172c_65[31]))),
                    (((((M31_2) * (conv_tmp_3172c_65[4])) + (conv_tmp_3172c_65[10]))
                        + ((M31_32) * (conv_tmp_3172c_65[11])))
                        - ((M31_4) * (conv_tmp_3172c_65[32]))),
                    (((((M31_2) * (conv_tmp_3172c_65[5])) + (conv_tmp_3172c_65[11]))
                        + ((M31_32) * (conv_tmp_3172c_65[12])))
                        - ((M31_4) * (conv_tmp_3172c_65[33]))),
                    (((((M31_2) * (conv_tmp_3172c_65[6])) + (conv_tmp_3172c_65[12]))
                        + ((M31_32) * (conv_tmp_3172c_65[13])))
                        - ((M31_4) * (conv_tmp_3172c_65[34]))),
                    (((((M31_2) * (conv_tmp_3172c_65[7])) + (conv_tmp_3172c_65[13]))
                        + ((M31_32) * (conv_tmp_3172c_65[14])))
                        - ((M31_4) * (conv_tmp_3172c_65[35]))),
                    (((((M31_2) * (conv_tmp_3172c_65[8])) + (conv_tmp_3172c_65[14]))
                        + ((M31_32) * (conv_tmp_3172c_65[15])))
                        - ((M31_4) * (conv_tmp_3172c_65[36]))),
                    (((((M31_2) * (conv_tmp_3172c_65[9])) + (conv_tmp_3172c_65[15]))
                        + ((M31_32) * (conv_tmp_3172c_65[16])))
                        - ((M31_4) * (conv_tmp_3172c_65[37]))),
                    (((((M31_2) * (conv_tmp_3172c_65[10])) + (conv_tmp_3172c_65[16]))
                        + ((M31_32) * (conv_tmp_3172c_65[17])))
                        - ((M31_4) * (conv_tmp_3172c_65[38]))),
                    (((((M31_2) * (conv_tmp_3172c_65[11])) + (conv_tmp_3172c_65[17]))
                        + ((M31_32) * (conv_tmp_3172c_65[18])))
                        - ((M31_4) * (conv_tmp_3172c_65[39]))),
                    (((((M31_2) * (conv_tmp_3172c_65[12])) + (conv_tmp_3172c_65[18]))
                        + ((M31_32) * (conv_tmp_3172c_65[19])))
                        - ((M31_4) * (conv_tmp_3172c_65[40]))),
                    (((((M31_2) * (conv_tmp_3172c_65[13])) + (conv_tmp_3172c_65[19]))
                        + ((M31_32) * (conv_tmp_3172c_65[20])))
                        - ((M31_4) * (conv_tmp_3172c_65[41]))),
                    (((((M31_2) * (conv_tmp_3172c_65[14])) + (conv_tmp_3172c_65[20]))
                        - ((M31_4) * (conv_tmp_3172c_65[42])))
                        + ((M31_64) * (conv_tmp_3172c_65[49]))),
                    (((((M31_2) * (conv_tmp_3172c_65[15])) - ((M31_4) * (conv_tmp_3172c_65[43])))
                        + ((M31_2) * (conv_tmp_3172c_65[49])))
                        + ((M31_64) * (conv_tmp_3172c_65[50]))),
                    (((((M31_2) * (conv_tmp_3172c_65[16])) - ((M31_4) * (conv_tmp_3172c_65[44])))
                        + ((M31_2) * (conv_tmp_3172c_65[50])))
                        + ((M31_64) * (conv_tmp_3172c_65[51]))),
                    (((((M31_2) * (conv_tmp_3172c_65[17])) - ((M31_4) * (conv_tmp_3172c_65[45])))
                        + ((M31_2) * (conv_tmp_3172c_65[51])))
                        + ((M31_64) * (conv_tmp_3172c_65[52]))),
                    (((((M31_2) * (conv_tmp_3172c_65[18])) - ((M31_4) * (conv_tmp_3172c_65[46])))
                        + ((M31_2) * (conv_tmp_3172c_65[52])))
                        + ((M31_64) * (conv_tmp_3172c_65[53]))),
                    (((((M31_2) * (conv_tmp_3172c_65[19])) - ((M31_4) * (conv_tmp_3172c_65[47])))
                        + ((M31_2) * (conv_tmp_3172c_65[53])))
                        + ((M31_64) * (conv_tmp_3172c_65[54]))),
                    ((((M31_2) * (conv_tmp_3172c_65[20])) - ((M31_4) * (conv_tmp_3172c_65[48])))
                        + ((M31_2) * (conv_tmp_3172c_65[54]))),
                ];
                let k_mod_2_18_biased_tmp_3172c_67 =
                    ((((PackedUInt32::from_m31(((conv_mod_tmp_3172c_66[0]) + (M31_134217728))))
                        + (((PackedUInt32::from_m31(
                            ((conv_mod_tmp_3172c_66[1]) + (M31_134217728)),
                        )) & (UInt32_511))
                            << (UInt32_9)))
                        + (UInt32_65536))
                        & (UInt32_262143));
                let k_col147 = ((k_mod_2_18_biased_tmp_3172c_67.low().as_m31())
                    + (((k_mod_2_18_biased_tmp_3172c_67.high().as_m31()) - (M31_1)) * (M31_65536)));
                *row[147] = k_col147;
                *sub_component_inputs.range_check_19_h[0] = [((k_col147) + (M31_262144))];
                *lookup_data.range_check_19_h_0 = [((k_col147) + (M31_262144))];
                let carry_0_col148 = (((conv_mod_tmp_3172c_66[0]) - (k_col147)) * (M31_4194304));
                *row[148] = carry_0_col148;
                *sub_component_inputs.range_check_19[0] = [((carry_0_col148) + (M31_131072))];
                *lookup_data.range_check_19_0 = [((carry_0_col148) + (M31_131072))];
                let carry_1_col149 =
                    (((conv_mod_tmp_3172c_66[1]) + (carry_0_col148)) * (M31_4194304));
                *row[149] = carry_1_col149;
                *sub_component_inputs.range_check_19_b[0] = [((carry_1_col149) + (M31_131072))];
                *lookup_data.range_check_19_b_0 = [((carry_1_col149) + (M31_131072))];
                let carry_2_col150 =
                    (((conv_mod_tmp_3172c_66[2]) + (carry_1_col149)) * (M31_4194304));
                *row[150] = carry_2_col150;
                *sub_component_inputs.range_check_19_c[0] = [((carry_2_col150) + (M31_131072))];
                *lookup_data.range_check_19_c_0 = [((carry_2_col150) + (M31_131072))];
                let carry_3_col151 =
                    (((conv_mod_tmp_3172c_66[3]) + (carry_2_col150)) * (M31_4194304));
                *row[151] = carry_3_col151;
                *sub_component_inputs.range_check_19_d[0] = [((carry_3_col151) + (M31_131072))];
                *lookup_data.range_check_19_d_0 = [((carry_3_col151) + (M31_131072))];
                let carry_4_col152 =
                    (((conv_mod_tmp_3172c_66[4]) + (carry_3_col151)) * (M31_4194304));
                *row[152] = carry_4_col152;
                *sub_component_inputs.range_check_19_e[0] = [((carry_4_col152) + (M31_131072))];
                *lookup_data.range_check_19_e_0 = [((carry_4_col152) + (M31_131072))];
                let carry_5_col153 =
                    (((conv_mod_tmp_3172c_66[5]) + (carry_4_col152)) * (M31_4194304));
                *row[153] = carry_5_col153;
                *sub_component_inputs.range_check_19_f[0] = [((carry_5_col153) + (M31_131072))];
                *lookup_data.range_check_19_f_0 = [((carry_5_col153) + (M31_131072))];
                let carry_6_col154 =
                    (((conv_mod_tmp_3172c_66[6]) + (carry_5_col153)) * (M31_4194304));
                *row[154] = carry_6_col154;
                *sub_component_inputs.range_check_19_g[0] = [((carry_6_col154) + (M31_131072))];
                *lookup_data.range_check_19_g_0 = [((carry_6_col154) + (M31_131072))];
                let carry_7_col155 =
                    (((conv_mod_tmp_3172c_66[7]) + (carry_6_col154)) * (M31_4194304));
                *row[155] = carry_7_col155;
                *sub_component_inputs.range_check_19_h[1] = [((carry_7_col155) + (M31_131072))];
                *lookup_data.range_check_19_h_1 = [((carry_7_col155) + (M31_131072))];
                let carry_8_col156 =
                    (((conv_mod_tmp_3172c_66[8]) + (carry_7_col155)) * (M31_4194304));
                *row[156] = carry_8_col156;
                *sub_component_inputs.range_check_19[1] = [((carry_8_col156) + (M31_131072))];
                *lookup_data.range_check_19_1 = [((carry_8_col156) + (M31_131072))];
                let carry_9_col157 =
                    (((conv_mod_tmp_3172c_66[9]) + (carry_8_col156)) * (M31_4194304));
                *row[157] = carry_9_col157;
                *sub_component_inputs.range_check_19_b[1] = [((carry_9_col157) + (M31_131072))];
                *lookup_data.range_check_19_b_1 = [((carry_9_col157) + (M31_131072))];
                let carry_10_col158 =
                    (((conv_mod_tmp_3172c_66[10]) + (carry_9_col157)) * (M31_4194304));
                *row[158] = carry_10_col158;
                *sub_component_inputs.range_check_19_c[1] = [((carry_10_col158) + (M31_131072))];
                *lookup_data.range_check_19_c_1 = [((carry_10_col158) + (M31_131072))];
                let carry_11_col159 =
                    (((conv_mod_tmp_3172c_66[11]) + (carry_10_col158)) * (M31_4194304));
                *row[159] = carry_11_col159;
                *sub_component_inputs.range_check_19_d[1] = [((carry_11_col159) + (M31_131072))];
                *lookup_data.range_check_19_d_1 = [((carry_11_col159) + (M31_131072))];
                let carry_12_col160 =
                    (((conv_mod_tmp_3172c_66[12]) + (carry_11_col159)) * (M31_4194304));
                *row[160] = carry_12_col160;
                *sub_component_inputs.range_check_19_e[1] = [((carry_12_col160) + (M31_131072))];
                *lookup_data.range_check_19_e_1 = [((carry_12_col160) + (M31_131072))];
                let carry_13_col161 =
                    (((conv_mod_tmp_3172c_66[13]) + (carry_12_col160)) * (M31_4194304));
                *row[161] = carry_13_col161;
                *sub_component_inputs.range_check_19_f[1] = [((carry_13_col161) + (M31_131072))];
                *lookup_data.range_check_19_f_1 = [((carry_13_col161) + (M31_131072))];
                let carry_14_col162 =
                    (((conv_mod_tmp_3172c_66[14]) + (carry_13_col161)) * (M31_4194304));
                *row[162] = carry_14_col162;
                *sub_component_inputs.range_check_19_g[1] = [((carry_14_col162) + (M31_131072))];
                *lookup_data.range_check_19_g_1 = [((carry_14_col162) + (M31_131072))];
                let carry_15_col163 =
                    (((conv_mod_tmp_3172c_66[15]) + (carry_14_col162)) * (M31_4194304));
                *row[163] = carry_15_col163;
                *sub_component_inputs.range_check_19_h[2] = [((carry_15_col163) + (M31_131072))];
                *lookup_data.range_check_19_h_2 = [((carry_15_col163) + (M31_131072))];
                let carry_16_col164 =
                    (((conv_mod_tmp_3172c_66[16]) + (carry_15_col163)) * (M31_4194304));
                *row[164] = carry_16_col164;
                *sub_component_inputs.range_check_19[2] = [((carry_16_col164) + (M31_131072))];
                *lookup_data.range_check_19_2 = [((carry_16_col164) + (M31_131072))];
                let carry_17_col165 =
                    (((conv_mod_tmp_3172c_66[17]) + (carry_16_col164)) * (M31_4194304));
                *row[165] = carry_17_col165;
                *sub_component_inputs.range_check_19_b[2] = [((carry_17_col165) + (M31_131072))];
                *lookup_data.range_check_19_b_2 = [((carry_17_col165) + (M31_131072))];
                let carry_18_col166 =
                    (((conv_mod_tmp_3172c_66[18]) + (carry_17_col165)) * (M31_4194304));
                *row[166] = carry_18_col166;
                *sub_component_inputs.range_check_19_c[2] = [((carry_18_col166) + (M31_131072))];
                *lookup_data.range_check_19_c_2 = [((carry_18_col166) + (M31_131072))];
                let carry_19_col167 =
                    (((conv_mod_tmp_3172c_66[19]) + (carry_18_col166)) * (M31_4194304));
                *row[167] = carry_19_col167;
                *sub_component_inputs.range_check_19_d[2] = [((carry_19_col167) + (M31_131072))];
                *lookup_data.range_check_19_d_2 = [((carry_19_col167) + (M31_131072))];
                let carry_20_col168 =
                    (((conv_mod_tmp_3172c_66[20]) + (carry_19_col167)) * (M31_4194304));
                *row[168] = carry_20_col168;
                *sub_component_inputs.range_check_19_e[2] = [((carry_20_col168) + (M31_131072))];
                *lookup_data.range_check_19_e_2 = [((carry_20_col168) + (M31_131072))];
                let carry_21_col169 = ((((conv_mod_tmp_3172c_66[21]) - ((M31_136) * (k_col147)))
                    + (carry_20_col168))
                    * (M31_4194304));
                *row[169] = carry_21_col169;
                *sub_component_inputs.range_check_19_f[2] = [((carry_21_col169) + (M31_131072))];
                *lookup_data.range_check_19_f_2 = [((carry_21_col169) + (M31_131072))];
                let carry_22_col170 =
                    (((conv_mod_tmp_3172c_66[22]) + (carry_21_col169)) * (M31_4194304));
                *row[170] = carry_22_col170;
                *sub_component_inputs.range_check_19_g[2] = [((carry_22_col170) + (M31_131072))];
                *lookup_data.range_check_19_g_2 = [((carry_22_col170) + (M31_131072))];
                let carry_23_col171 =
                    (((conv_mod_tmp_3172c_66[23]) + (carry_22_col170)) * (M31_4194304));
                *row[171] = carry_23_col171;
                *sub_component_inputs.range_check_19_h[3] = [((carry_23_col171) + (M31_131072))];
                *lookup_data.range_check_19_h_3 = [((carry_23_col171) + (M31_131072))];
                let carry_24_col172 =
                    (((conv_mod_tmp_3172c_66[24]) + (carry_23_col171)) * (M31_4194304));
                *row[172] = carry_24_col172;
                *sub_component_inputs.range_check_19[3] = [((carry_24_col172) + (M31_131072))];
                *lookup_data.range_check_19_3 = [((carry_24_col172) + (M31_131072))];
                let carry_25_col173 =
                    (((conv_mod_tmp_3172c_66[25]) + (carry_24_col172)) * (M31_4194304));
                *row[173] = carry_25_col173;
                *sub_component_inputs.range_check_19_b[3] = [((carry_25_col173) + (M31_131072))];
                *lookup_data.range_check_19_b_3 = [((carry_25_col173) + (M31_131072))];
                let carry_26_col174 =
                    (((conv_mod_tmp_3172c_66[26]) + (carry_25_col173)) * (M31_4194304));
                *row[174] = carry_26_col174;
                *sub_component_inputs.range_check_19_c[3] = [((carry_26_col174) + (M31_131072))];
                *lookup_data.range_check_19_c_3 = [((carry_26_col174) + (M31_131072))];

                let mul_252_output_tmp_3172c_68 = mul_res_tmp_3172c_46;

                let res_tmp_3172c_69 = ((((PackedFelt252::from_m31(eval_operands_input.1[16]))
                    * (read_positive_num_bits_252_output_tmp_3172c_15.0))
                    + ((PackedFelt252::from_m31(eval_operands_input.1[6]))
                        * (mul_252_output_tmp_3172c_68)))
                    + ((PackedFelt252::from_m31(eval_operands_input.1[5]))
                        * (add_252_output_tmp_3172c_45)));
                let res_limb_0_col175 = res_tmp_3172c_69.get_m31(0);
                *row[175] = res_limb_0_col175;
                let res_limb_1_col176 = res_tmp_3172c_69.get_m31(1);
                *row[176] = res_limb_1_col176;
                let res_limb_2_col177 = res_tmp_3172c_69.get_m31(2);
                *row[177] = res_limb_2_col177;
                let res_limb_3_col178 = res_tmp_3172c_69.get_m31(3);
                *row[178] = res_limb_3_col178;
                let res_limb_4_col179 = res_tmp_3172c_69.get_m31(4);
                *row[179] = res_limb_4_col179;
                let res_limb_5_col180 = res_tmp_3172c_69.get_m31(5);
                *row[180] = res_limb_5_col180;
                let res_limb_6_col181 = res_tmp_3172c_69.get_m31(6);
                *row[181] = res_limb_6_col181;
                let res_limb_7_col182 = res_tmp_3172c_69.get_m31(7);
                *row[182] = res_limb_7_col182;
                let res_limb_8_col183 = res_tmp_3172c_69.get_m31(8);
                *row[183] = res_limb_8_col183;
                let res_limb_9_col184 = res_tmp_3172c_69.get_m31(9);
                *row[184] = res_limb_9_col184;
                let res_limb_10_col185 = res_tmp_3172c_69.get_m31(10);
                *row[185] = res_limb_10_col185;
                let res_limb_11_col186 = res_tmp_3172c_69.get_m31(11);
                *row[186] = res_limb_11_col186;
                let res_limb_12_col187 = res_tmp_3172c_69.get_m31(12);
                *row[187] = res_limb_12_col187;
                let res_limb_13_col188 = res_tmp_3172c_69.get_m31(13);
                *row[188] = res_limb_13_col188;
                let res_limb_14_col189 = res_tmp_3172c_69.get_m31(14);
                *row[189] = res_limb_14_col189;
                let res_limb_15_col190 = res_tmp_3172c_69.get_m31(15);
                *row[190] = res_limb_15_col190;
                let res_limb_16_col191 = res_tmp_3172c_69.get_m31(16);
                *row[191] = res_limb_16_col191;
                let res_limb_17_col192 = res_tmp_3172c_69.get_m31(17);
                *row[192] = res_limb_17_col192;
                let res_limb_18_col193 = res_tmp_3172c_69.get_m31(18);
                *row[193] = res_limb_18_col193;
                let res_limb_19_col194 = res_tmp_3172c_69.get_m31(19);
                *row[194] = res_limb_19_col194;
                let res_limb_20_col195 = res_tmp_3172c_69.get_m31(20);
                *row[195] = res_limb_20_col195;
                let res_limb_21_col196 = res_tmp_3172c_69.get_m31(21);
                *row[196] = res_limb_21_col196;
                let res_limb_22_col197 = res_tmp_3172c_69.get_m31(22);
                *row[197] = res_limb_22_col197;
                let res_limb_23_col198 = res_tmp_3172c_69.get_m31(23);
                *row[198] = res_limb_23_col198;
                let res_limb_24_col199 = res_tmp_3172c_69.get_m31(24);
                *row[199] = res_limb_24_col199;
                let res_limb_25_col200 = res_tmp_3172c_69.get_m31(25);
                *row[200] = res_limb_25_col200;
                let res_limb_26_col201 = res_tmp_3172c_69.get_m31(26);
                *row[201] = res_limb_26_col201;
                let res_limb_27_col202 = res_tmp_3172c_69.get_m31(27);
                *row[202] = res_limb_27_col202;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    range_check_19_0: Vec<[PackedM31; 1]>,
    range_check_19_1: Vec<[PackedM31; 1]>,
    range_check_19_2: Vec<[PackedM31; 1]>,
    range_check_19_3: Vec<[PackedM31; 1]>,
    range_check_19_b_0: Vec<[PackedM31; 1]>,
    range_check_19_b_1: Vec<[PackedM31; 1]>,
    range_check_19_b_2: Vec<[PackedM31; 1]>,
    range_check_19_b_3: Vec<[PackedM31; 1]>,
    range_check_19_c_0: Vec<[PackedM31; 1]>,
    range_check_19_c_1: Vec<[PackedM31; 1]>,
    range_check_19_c_2: Vec<[PackedM31; 1]>,
    range_check_19_c_3: Vec<[PackedM31; 1]>,
    range_check_19_d_0: Vec<[PackedM31; 1]>,
    range_check_19_d_1: Vec<[PackedM31; 1]>,
    range_check_19_d_2: Vec<[PackedM31; 1]>,
    range_check_19_e_0: Vec<[PackedM31; 1]>,
    range_check_19_e_1: Vec<[PackedM31; 1]>,
    range_check_19_e_2: Vec<[PackedM31; 1]>,
    range_check_19_f_0: Vec<[PackedM31; 1]>,
    range_check_19_f_1: Vec<[PackedM31; 1]>,
    range_check_19_f_2: Vec<[PackedM31; 1]>,
    range_check_19_g_0: Vec<[PackedM31; 1]>,
    range_check_19_g_1: Vec<[PackedM31; 1]>,
    range_check_19_g_2: Vec<[PackedM31; 1]>,
    range_check_19_h_0: Vec<[PackedM31; 1]>,
    range_check_19_h_1: Vec<[PackedM31; 1]>,
    range_check_19_h_2: Vec<[PackedM31; 1]>,
    range_check_19_h_3: Vec<[PackedM31; 1]>,
    range_check_9_9_0: Vec<[PackedM31; 2]>,
    range_check_9_9_1: Vec<[PackedM31; 2]>,
    range_check_9_9_2: Vec<[PackedM31; 2]>,
    range_check_9_9_3: Vec<[PackedM31; 2]>,
    range_check_9_9_b_0: Vec<[PackedM31; 2]>,
    range_check_9_9_b_1: Vec<[PackedM31; 2]>,
    range_check_9_9_b_2: Vec<[PackedM31; 2]>,
    range_check_9_9_b_3: Vec<[PackedM31; 2]>,
    range_check_9_9_c_0: Vec<[PackedM31; 2]>,
    range_check_9_9_c_1: Vec<[PackedM31; 2]>,
    range_check_9_9_c_2: Vec<[PackedM31; 2]>,
    range_check_9_9_c_3: Vec<[PackedM31; 2]>,
    range_check_9_9_d_0: Vec<[PackedM31; 2]>,
    range_check_9_9_d_1: Vec<[PackedM31; 2]>,
    range_check_9_9_d_2: Vec<[PackedM31; 2]>,
    range_check_9_9_d_3: Vec<[PackedM31; 2]>,
    range_check_9_9_e_0: Vec<[PackedM31; 2]>,
    range_check_9_9_e_1: Vec<[PackedM31; 2]>,
    range_check_9_9_e_2: Vec<[PackedM31; 2]>,
    range_check_9_9_e_3: Vec<[PackedM31; 2]>,
    range_check_9_9_f_0: Vec<[PackedM31; 2]>,
    range_check_9_9_f_1: Vec<[PackedM31; 2]>,
    range_check_9_9_f_2: Vec<[PackedM31; 2]>,
    range_check_9_9_f_3: Vec<[PackedM31; 2]>,
    range_check_9_9_g_0: Vec<[PackedM31; 2]>,
    range_check_9_9_g_1: Vec<[PackedM31; 2]>,
    range_check_9_9_h_0: Vec<[PackedM31; 2]>,
    range_check_9_9_h_1: Vec<[PackedM31; 2]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_9_9: &relations::RangeCheck_9_9,
        range_check_9_9_b: &relations::RangeCheck_9_9_B,
        range_check_9_9_c: &relations::RangeCheck_9_9_C,
        range_check_9_9_d: &relations::RangeCheck_9_9_D,
        range_check_9_9_e: &relations::RangeCheck_9_9_E,
        range_check_9_9_f: &relations::RangeCheck_9_9_F,
        range_check_9_9_g: &relations::RangeCheck_9_9_G,
        range_check_9_9_h: &relations::RangeCheck_9_9_H,
        range_check_19_h: &relations::RangeCheck_19_H,
        range_check_19: &relations::RangeCheck_19,
        range_check_19_b: &relations::RangeCheck_19_B,
        range_check_19_c: &relations::RangeCheck_19_C,
        range_check_19_d: &relations::RangeCheck_19_D,
        range_check_19_e: &relations::RangeCheck_19_E,
        range_check_19_f: &relations::RangeCheck_19_F,
        range_check_19_g: &relations::RangeCheck_19_G,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_1,
            &self.lookup_data.memory_id_to_big_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.memory_id_to_big_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_0,
            &self.lookup_data.range_check_9_9_b_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_0,
            &self.lookup_data.range_check_9_9_d_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_0,
            &self.lookup_data.range_check_9_9_f_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_e.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_f.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_g_0,
            &self.lookup_data.range_check_9_9_h_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_g.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_h.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_1,
            &self.lookup_data.range_check_9_9_b_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_1,
            &self.lookup_data.range_check_9_9_d_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_1,
            &self.lookup_data.range_check_9_9_f_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_e.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_f.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_2,
            &self.lookup_data.range_check_9_9_b_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_2,
            &self.lookup_data.range_check_9_9_d_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_2,
            &self.lookup_data.range_check_9_9_f_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_e.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_f.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_g_1,
            &self.lookup_data.range_check_9_9_h_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_g.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_h.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_3,
            &self.lookup_data.range_check_9_9_b_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_c_3,
            &self.lookup_data.range_check_9_9_d_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_c.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_d.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_e_3,
            &self.lookup_data.range_check_9_9_f_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_9_9_e.combine(values0);
                let denom1: PackedQM31 = range_check_9_9_f.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_h_0,
            &self.lookup_data.range_check_19_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_h.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_0,
            &self.lookup_data.range_check_19_c_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_d_0,
            &self.lookup_data.range_check_19_e_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_d.combine(values0);
                let denom1: PackedQM31 = range_check_19_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_f_0,
            &self.lookup_data.range_check_19_g_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_f.combine(values0);
                let denom1: PackedQM31 = range_check_19_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_h_1,
            &self.lookup_data.range_check_19_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_h.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_1,
            &self.lookup_data.range_check_19_c_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_d_1,
            &self.lookup_data.range_check_19_e_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_d.combine(values0);
                let denom1: PackedQM31 = range_check_19_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_f_1,
            &self.lookup_data.range_check_19_g_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_f.combine(values0);
                let denom1: PackedQM31 = range_check_19_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_h_2,
            &self.lookup_data.range_check_19_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_h.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_2,
            &self.lookup_data.range_check_19_c_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_d_2,
            &self.lookup_data.range_check_19_e_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_d.combine(values0);
                let denom1: PackedQM31 = range_check_19_e.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_f_2,
            &self.lookup_data.range_check_19_g_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_f.combine(values0);
                let denom1: PackedQM31 = range_check_19_g.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_h_3,
            &self.lookup_data.range_check_19_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_h.combine(values0);
                let denom1: PackedQM31 = range_check_19.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_19_b_3,
            &self.lookup_data.range_check_19_c_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_19_b.combine(values0);
                let denom1: PackedQM31 = range_check_19_c.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

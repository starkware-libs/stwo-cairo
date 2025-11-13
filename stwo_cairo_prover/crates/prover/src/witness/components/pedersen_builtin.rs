// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::pedersen_builtin::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, partial_ec_mul, pedersen_points_table, range_check_5_4,
    range_check_8,
};
use crate::witness::prelude::*;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub pedersen_builtin_segment_start: u32,
}

impl ClaimGenerator {
    pub fn new(log_size: u32, pedersen_builtin_segment_start: u32) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            pedersen_builtin_segment_start,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_5_4_state: &range_check_5_4::ClaimGenerator,
        range_check_8_state: &range_check_8::ClaimGenerator,
        pedersen_points_table_state: &pedersen_points_table::ClaimGenerator,
        partial_ec_mul_state: &mut partial_ec_mul::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let log_size = self.log_size;

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            log_size,
            self.pedersen_builtin_segment_start,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_5_4_state,
            range_check_8_state,
            pedersen_points_table_state,
            partial_ec_mul_state,
        );
        sub_component_inputs
            .range_check_5_4
            .iter()
            .for_each(|inputs| {
                range_check_5_4_state.add_packed_inputs(inputs);
            });
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
            .range_check_8
            .iter()
            .for_each(|inputs| {
                range_check_8_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .pedersen_points_table
            .iter()
            .for_each(|inputs| {
                pedersen_points_table_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .partial_ec_mul
            .iter()
            .for_each(|inputs| {
                partial_ec_mul_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim {
                log_size,
                pedersen_builtin_segment_start: self.pedersen_builtin_segment_start,
            },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    range_check_5_4: [Vec<range_check_5_4::PackedInputType>; 2],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 3],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 3],
    range_check_8: [Vec<range_check_8::PackedInputType>; 4],
    pedersen_points_table: [Vec<pedersen_points_table::PackedInputType>; 1],
    partial_ec_mul: [Vec<partial_ec_mul::PackedInputType>; 28],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    pedersen_builtin_segment_start: u32,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_5_4_state: &range_check_5_4::ClaimGenerator,
    range_check_8_state: &range_check_8::ClaimGenerator,
    pedersen_points_table_state: &pedersen_points_table::ClaimGenerator,
    partial_ec_mul_state: &mut partial_ec_mul::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_10 = PackedM31::broadcast(M31::from(10));
    let M31_11 = PackedM31::broadcast(M31::from(11));
    let M31_12 = PackedM31::broadcast(M31::from(12));
    let M31_120 = PackedM31::broadcast(M31::from(120));
    let M31_13 = PackedM31::broadcast(M31::from(13));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_14 = PackedM31::broadcast(M31::from(14));
    let M31_15 = PackedM31::broadcast(M31::from(15));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_17 = PackedM31::broadcast(M31::from(17));
    let M31_18 = PackedM31::broadcast(M31::from(18));
    let M31_19 = PackedM31::broadcast(M31::from(19));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_20 = PackedM31::broadcast(M31::from(20));
    let M31_21 = PackedM31::broadcast(M31::from(21));
    let M31_22 = PackedM31::broadcast(M31::from(22));
    let M31_23 = PackedM31::broadcast(M31::from(23));
    let M31_24 = PackedM31::broadcast(M31::from(24));
    let M31_25 = PackedM31::broadcast(M31::from(25));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_26 = PackedM31::broadcast(M31::from(26));
    let M31_27 = PackedM31::broadcast(M31::from(27));
    let M31_28 = PackedM31::broadcast(M31::from(28));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_5 = PackedM31::broadcast(M31::from(5));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_6 = PackedM31::broadcast(M31::from(6));
    let M31_7 = PackedM31::broadcast(M31::from(7));
    let M31_7340032 = PackedM31::broadcast(M31::from(7340032));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_9 = PackedM31::broadcast(M31::from(9));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let seq = Seq::new(log_size);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs))| {
                let seq = seq.packed_at(row_index);
                let instance_addr_tmp_d00c6_0 = (((seq) * (M31_3))
                    + (PackedM31::broadcast(M31::from(pedersen_builtin_segment_start))));

                // Read Split.

                let memory_address_to_id_value_tmp_d00c6_1 =
                    memory_address_to_id_state.deduce_output(instance_addr_tmp_d00c6_0);
                let memory_id_to_big_value_tmp_d00c6_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_d00c6_1);
                let value_limb_0_col0 = memory_id_to_big_value_tmp_d00c6_2.get_m31(0);
                *row[0] = value_limb_0_col0;
                let value_limb_1_col1 = memory_id_to_big_value_tmp_d00c6_2.get_m31(1);
                *row[1] = value_limb_1_col1;
                let value_limb_2_col2 = memory_id_to_big_value_tmp_d00c6_2.get_m31(2);
                *row[2] = value_limb_2_col2;
                let value_limb_3_col3 = memory_id_to_big_value_tmp_d00c6_2.get_m31(3);
                *row[3] = value_limb_3_col3;
                let value_limb_4_col4 = memory_id_to_big_value_tmp_d00c6_2.get_m31(4);
                *row[4] = value_limb_4_col4;
                let value_limb_5_col5 = memory_id_to_big_value_tmp_d00c6_2.get_m31(5);
                *row[5] = value_limb_5_col5;
                let value_limb_6_col6 = memory_id_to_big_value_tmp_d00c6_2.get_m31(6);
                *row[6] = value_limb_6_col6;
                let value_limb_7_col7 = memory_id_to_big_value_tmp_d00c6_2.get_m31(7);
                *row[7] = value_limb_7_col7;
                let value_limb_8_col8 = memory_id_to_big_value_tmp_d00c6_2.get_m31(8);
                *row[8] = value_limb_8_col8;
                let value_limb_9_col9 = memory_id_to_big_value_tmp_d00c6_2.get_m31(9);
                *row[9] = value_limb_9_col9;
                let value_limb_10_col10 = memory_id_to_big_value_tmp_d00c6_2.get_m31(10);
                *row[10] = value_limb_10_col10;
                let value_limb_11_col11 = memory_id_to_big_value_tmp_d00c6_2.get_m31(11);
                *row[11] = value_limb_11_col11;
                let value_limb_12_col12 = memory_id_to_big_value_tmp_d00c6_2.get_m31(12);
                *row[12] = value_limb_12_col12;
                let value_limb_13_col13 = memory_id_to_big_value_tmp_d00c6_2.get_m31(13);
                *row[13] = value_limb_13_col13;
                let value_limb_14_col14 = memory_id_to_big_value_tmp_d00c6_2.get_m31(14);
                *row[14] = value_limb_14_col14;
                let value_limb_15_col15 = memory_id_to_big_value_tmp_d00c6_2.get_m31(15);
                *row[15] = value_limb_15_col15;
                let value_limb_16_col16 = memory_id_to_big_value_tmp_d00c6_2.get_m31(16);
                *row[16] = value_limb_16_col16;
                let value_limb_17_col17 = memory_id_to_big_value_tmp_d00c6_2.get_m31(17);
                *row[17] = value_limb_17_col17;
                let value_limb_18_col18 = memory_id_to_big_value_tmp_d00c6_2.get_m31(18);
                *row[18] = value_limb_18_col18;
                let value_limb_19_col19 = memory_id_to_big_value_tmp_d00c6_2.get_m31(19);
                *row[19] = value_limb_19_col19;
                let value_limb_20_col20 = memory_id_to_big_value_tmp_d00c6_2.get_m31(20);
                *row[20] = value_limb_20_col20;
                let value_limb_21_col21 = memory_id_to_big_value_tmp_d00c6_2.get_m31(21);
                *row[21] = value_limb_21_col21;
                let value_limb_22_col22 = memory_id_to_big_value_tmp_d00c6_2.get_m31(22);
                *row[22] = value_limb_22_col22;
                let value_limb_23_col23 = memory_id_to_big_value_tmp_d00c6_2.get_m31(23);
                *row[23] = value_limb_23_col23;
                let value_limb_24_col24 = memory_id_to_big_value_tmp_d00c6_2.get_m31(24);
                *row[24] = value_limb_24_col24;
                let value_limb_25_col25 = memory_id_to_big_value_tmp_d00c6_2.get_m31(25);
                *row[25] = value_limb_25_col25;
                let value_limb_26_col26 = memory_id_to_big_value_tmp_d00c6_2.get_m31(26);
                *row[26] = value_limb_26_col26;
                let ms_limb_low_tmp_d00c6_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d00c6_2.get_m31(27)))
                        & (UInt16_31));
                let ms_limb_low_col27 = ms_limb_low_tmp_d00c6_3.as_m31();
                *row[27] = ms_limb_low_col27;
                let ms_limb_high_tmp_d00c6_4 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d00c6_2.get_m31(27)))
                        >> (UInt16_5));
                let ms_limb_high_col28 = ms_limb_high_tmp_d00c6_4.as_m31();
                *row[28] = ms_limb_high_col28;
                *sub_component_inputs.range_check_5_4[0] = [ms_limb_low_col27, ms_limb_high_col28];
                *lookup_data.range_check_5_4_0 = [ms_limb_low_col27, ms_limb_high_col28];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_d00c6_5 =
                    memory_address_to_id_state.deduce_output(instance_addr_tmp_d00c6_0);
                let pedersen_a_id_col29 = memory_address_to_id_value_tmp_d00c6_5;
                *row[29] = pedersen_a_id_col29;
                *sub_component_inputs.memory_address_to_id[0] = instance_addr_tmp_d00c6_0;
                *lookup_data.memory_address_to_id_0 =
                    [instance_addr_tmp_d00c6_0, pedersen_a_id_col29];

                *sub_component_inputs.memory_id_to_big[0] = pedersen_a_id_col29;
                *lookup_data.memory_id_to_big_0 = [
                    pedersen_a_id_col29,
                    value_limb_0_col0,
                    value_limb_1_col1,
                    value_limb_2_col2,
                    value_limb_3_col3,
                    value_limb_4_col4,
                    value_limb_5_col5,
                    value_limb_6_col6,
                    value_limb_7_col7,
                    value_limb_8_col8,
                    value_limb_9_col9,
                    value_limb_10_col10,
                    value_limb_11_col11,
                    value_limb_12_col12,
                    value_limb_13_col13,
                    value_limb_14_col14,
                    value_limb_15_col15,
                    value_limb_16_col16,
                    value_limb_17_col17,
                    value_limb_18_col18,
                    value_limb_19_col19,
                    value_limb_20_col20,
                    value_limb_21_col21,
                    value_limb_22_col22,
                    value_limb_23_col23,
                    value_limb_24_col24,
                    value_limb_25_col25,
                    value_limb_26_col26,
                    (((ms_limb_high_col28) * (M31_32)) + (ms_limb_low_col27)),
                ];

                let read_split_output_tmp_d00c6_7 = (
                    ms_limb_high_col28,
                    [
                        PackedFelt252::from_limbs([
                            value_limb_0_col0,
                            value_limb_1_col1,
                            value_limb_2_col2,
                            value_limb_3_col3,
                            value_limb_4_col4,
                            value_limb_5_col5,
                            value_limb_6_col6,
                            value_limb_7_col7,
                            value_limb_8_col8,
                            value_limb_9_col9,
                            value_limb_10_col10,
                            value_limb_11_col11,
                            value_limb_12_col12,
                            value_limb_13_col13,
                            value_limb_14_col14,
                            value_limb_15_col15,
                            value_limb_16_col16,
                            value_limb_17_col17,
                            value_limb_18_col18,
                            value_limb_19_col19,
                            value_limb_20_col20,
                            value_limb_21_col21,
                            value_limb_22_col22,
                            value_limb_23_col23,
                            value_limb_24_col24,
                            value_limb_25_col25,
                            value_limb_26_col26,
                            ms_limb_low_col27,
                        ]),
                        PackedFelt252::from_limbs([
                            value_limb_0_col0,
                            value_limb_1_col1,
                            value_limb_2_col2,
                            value_limb_3_col3,
                            value_limb_4_col4,
                            value_limb_5_col5,
                            value_limb_6_col6,
                            value_limb_7_col7,
                            value_limb_8_col8,
                            value_limb_9_col9,
                            value_limb_10_col10,
                            value_limb_11_col11,
                            value_limb_12_col12,
                            value_limb_13_col13,
                            value_limb_14_col14,
                            value_limb_15_col15,
                            value_limb_16_col16,
                            value_limb_17_col17,
                            value_limb_18_col18,
                            value_limb_19_col19,
                            value_limb_20_col20,
                            value_limb_21_col21,
                            value_limb_22_col22,
                            value_limb_23_col23,
                            value_limb_24_col24,
                            value_limb_25_col25,
                            value_limb_26_col26,
                            (((ms_limb_high_col28) * (M31_32)) + (ms_limb_low_col27)),
                        ]),
                    ],
                );

                // Read Split.

                let memory_address_to_id_value_tmp_d00c6_8 = memory_address_to_id_state
                    .deduce_output(((instance_addr_tmp_d00c6_0) + (M31_1)));
                let memory_id_to_big_value_tmp_d00c6_9 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_d00c6_8);
                let value_limb_0_col30 = memory_id_to_big_value_tmp_d00c6_9.get_m31(0);
                *row[30] = value_limb_0_col30;
                let value_limb_1_col31 = memory_id_to_big_value_tmp_d00c6_9.get_m31(1);
                *row[31] = value_limb_1_col31;
                let value_limb_2_col32 = memory_id_to_big_value_tmp_d00c6_9.get_m31(2);
                *row[32] = value_limb_2_col32;
                let value_limb_3_col33 = memory_id_to_big_value_tmp_d00c6_9.get_m31(3);
                *row[33] = value_limb_3_col33;
                let value_limb_4_col34 = memory_id_to_big_value_tmp_d00c6_9.get_m31(4);
                *row[34] = value_limb_4_col34;
                let value_limb_5_col35 = memory_id_to_big_value_tmp_d00c6_9.get_m31(5);
                *row[35] = value_limb_5_col35;
                let value_limb_6_col36 = memory_id_to_big_value_tmp_d00c6_9.get_m31(6);
                *row[36] = value_limb_6_col36;
                let value_limb_7_col37 = memory_id_to_big_value_tmp_d00c6_9.get_m31(7);
                *row[37] = value_limb_7_col37;
                let value_limb_8_col38 = memory_id_to_big_value_tmp_d00c6_9.get_m31(8);
                *row[38] = value_limb_8_col38;
                let value_limb_9_col39 = memory_id_to_big_value_tmp_d00c6_9.get_m31(9);
                *row[39] = value_limb_9_col39;
                let value_limb_10_col40 = memory_id_to_big_value_tmp_d00c6_9.get_m31(10);
                *row[40] = value_limb_10_col40;
                let value_limb_11_col41 = memory_id_to_big_value_tmp_d00c6_9.get_m31(11);
                *row[41] = value_limb_11_col41;
                let value_limb_12_col42 = memory_id_to_big_value_tmp_d00c6_9.get_m31(12);
                *row[42] = value_limb_12_col42;
                let value_limb_13_col43 = memory_id_to_big_value_tmp_d00c6_9.get_m31(13);
                *row[43] = value_limb_13_col43;
                let value_limb_14_col44 = memory_id_to_big_value_tmp_d00c6_9.get_m31(14);
                *row[44] = value_limb_14_col44;
                let value_limb_15_col45 = memory_id_to_big_value_tmp_d00c6_9.get_m31(15);
                *row[45] = value_limb_15_col45;
                let value_limb_16_col46 = memory_id_to_big_value_tmp_d00c6_9.get_m31(16);
                *row[46] = value_limb_16_col46;
                let value_limb_17_col47 = memory_id_to_big_value_tmp_d00c6_9.get_m31(17);
                *row[47] = value_limb_17_col47;
                let value_limb_18_col48 = memory_id_to_big_value_tmp_d00c6_9.get_m31(18);
                *row[48] = value_limb_18_col48;
                let value_limb_19_col49 = memory_id_to_big_value_tmp_d00c6_9.get_m31(19);
                *row[49] = value_limb_19_col49;
                let value_limb_20_col50 = memory_id_to_big_value_tmp_d00c6_9.get_m31(20);
                *row[50] = value_limb_20_col50;
                let value_limb_21_col51 = memory_id_to_big_value_tmp_d00c6_9.get_m31(21);
                *row[51] = value_limb_21_col51;
                let value_limb_22_col52 = memory_id_to_big_value_tmp_d00c6_9.get_m31(22);
                *row[52] = value_limb_22_col52;
                let value_limb_23_col53 = memory_id_to_big_value_tmp_d00c6_9.get_m31(23);
                *row[53] = value_limb_23_col53;
                let value_limb_24_col54 = memory_id_to_big_value_tmp_d00c6_9.get_m31(24);
                *row[54] = value_limb_24_col54;
                let value_limb_25_col55 = memory_id_to_big_value_tmp_d00c6_9.get_m31(25);
                *row[55] = value_limb_25_col55;
                let value_limb_26_col56 = memory_id_to_big_value_tmp_d00c6_9.get_m31(26);
                *row[56] = value_limb_26_col56;
                let ms_limb_low_tmp_d00c6_10 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d00c6_9.get_m31(27)))
                        & (UInt16_31));
                let ms_limb_low_col57 = ms_limb_low_tmp_d00c6_10.as_m31();
                *row[57] = ms_limb_low_col57;
                let ms_limb_high_tmp_d00c6_11 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d00c6_9.get_m31(27)))
                        >> (UInt16_5));
                let ms_limb_high_col58 = ms_limb_high_tmp_d00c6_11.as_m31();
                *row[58] = ms_limb_high_col58;
                *sub_component_inputs.range_check_5_4[1] = [ms_limb_low_col57, ms_limb_high_col58];
                *lookup_data.range_check_5_4_1 = [ms_limb_low_col57, ms_limb_high_col58];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_d00c6_12 = memory_address_to_id_state
                    .deduce_output(((instance_addr_tmp_d00c6_0) + (M31_1)));
                let pedersen_b_id_col59 = memory_address_to_id_value_tmp_d00c6_12;
                *row[59] = pedersen_b_id_col59;
                *sub_component_inputs.memory_address_to_id[1] =
                    ((instance_addr_tmp_d00c6_0) + (M31_1));
                *lookup_data.memory_address_to_id_1 =
                    [((instance_addr_tmp_d00c6_0) + (M31_1)), pedersen_b_id_col59];

                *sub_component_inputs.memory_id_to_big[1] = pedersen_b_id_col59;
                *lookup_data.memory_id_to_big_1 = [
                    pedersen_b_id_col59,
                    value_limb_0_col30,
                    value_limb_1_col31,
                    value_limb_2_col32,
                    value_limb_3_col33,
                    value_limb_4_col34,
                    value_limb_5_col35,
                    value_limb_6_col36,
                    value_limb_7_col37,
                    value_limb_8_col38,
                    value_limb_9_col39,
                    value_limb_10_col40,
                    value_limb_11_col41,
                    value_limb_12_col42,
                    value_limb_13_col43,
                    value_limb_14_col44,
                    value_limb_15_col45,
                    value_limb_16_col46,
                    value_limb_17_col47,
                    value_limb_18_col48,
                    value_limb_19_col49,
                    value_limb_20_col50,
                    value_limb_21_col51,
                    value_limb_22_col52,
                    value_limb_23_col53,
                    value_limb_24_col54,
                    value_limb_25_col55,
                    value_limb_26_col56,
                    (((ms_limb_high_col58) * (M31_32)) + (ms_limb_low_col57)),
                ];

                let read_split_output_tmp_d00c6_14 = (
                    ms_limb_high_col58,
                    [
                        PackedFelt252::from_limbs([
                            value_limb_0_col30,
                            value_limb_1_col31,
                            value_limb_2_col32,
                            value_limb_3_col33,
                            value_limb_4_col34,
                            value_limb_5_col35,
                            value_limb_6_col36,
                            value_limb_7_col37,
                            value_limb_8_col38,
                            value_limb_9_col39,
                            value_limb_10_col40,
                            value_limb_11_col41,
                            value_limb_12_col42,
                            value_limb_13_col43,
                            value_limb_14_col44,
                            value_limb_15_col45,
                            value_limb_16_col46,
                            value_limb_17_col47,
                            value_limb_18_col48,
                            value_limb_19_col49,
                            value_limb_20_col50,
                            value_limb_21_col51,
                            value_limb_22_col52,
                            value_limb_23_col53,
                            value_limb_24_col54,
                            value_limb_25_col55,
                            value_limb_26_col56,
                            ms_limb_low_col57,
                        ]),
                        PackedFelt252::from_limbs([
                            value_limb_0_col30,
                            value_limb_1_col31,
                            value_limb_2_col32,
                            value_limb_3_col33,
                            value_limb_4_col34,
                            value_limb_5_col35,
                            value_limb_6_col36,
                            value_limb_7_col37,
                            value_limb_8_col38,
                            value_limb_9_col39,
                            value_limb_10_col40,
                            value_limb_11_col41,
                            value_limb_12_col42,
                            value_limb_13_col43,
                            value_limb_14_col44,
                            value_limb_15_col45,
                            value_limb_16_col46,
                            value_limb_17_col47,
                            value_limb_18_col48,
                            value_limb_19_col49,
                            value_limb_20_col50,
                            value_limb_21_col51,
                            value_limb_22_col52,
                            value_limb_23_col53,
                            value_limb_24_col54,
                            value_limb_25_col55,
                            value_limb_26_col56,
                            (((ms_limb_high_col58) * (M31_32)) + (ms_limb_low_col57)),
                        ]),
                    ],
                );

                // Verify Reduced 252.

                let ms_limb_is_max_tmp_d00c6_15 =
                    read_split_output_tmp_d00c6_7.1[1].get_m31(27).eq(M31_256);
                let ms_limb_is_max_col60 = ms_limb_is_max_tmp_d00c6_15.as_m31();
                *row[60] = ms_limb_is_max_col60;
                let ms_and_mid_limbs_are_max_tmp_d00c6_16 =
                    ((read_split_output_tmp_d00c6_7.1[1].get_m31(27).eq(M31_256))
                        & (value_limb_21_col21.eq(M31_136)));
                let ms_and_mid_limbs_are_max_col61 = ms_and_mid_limbs_are_max_tmp_d00c6_16.as_m31();
                *row[61] = ms_and_mid_limbs_are_max_col61;
                *sub_component_inputs.range_check_8[0] =
                    [((read_split_output_tmp_d00c6_7.1[1].get_m31(27)) - (ms_limb_is_max_col60))];
                *lookup_data.range_check_8_0 =
                    [((read_split_output_tmp_d00c6_7.1[1].get_m31(27)) - (ms_limb_is_max_col60))];
                let rc_input_col62 = ((ms_limb_is_max_col60)
                    * (((M31_120) + (value_limb_21_col21)) - (ms_and_mid_limbs_are_max_col61)));
                *row[62] = rc_input_col62;
                *sub_component_inputs.range_check_8[1] = [rc_input_col62];
                *lookup_data.range_check_8_1 = [rc_input_col62];

                // Verify Reduced 252.

                let ms_limb_is_max_tmp_d00c6_17 =
                    read_split_output_tmp_d00c6_14.1[1].get_m31(27).eq(M31_256);
                let ms_limb_is_max_col63 = ms_limb_is_max_tmp_d00c6_17.as_m31();
                *row[63] = ms_limb_is_max_col63;
                let ms_and_mid_limbs_are_max_tmp_d00c6_18 =
                    ((read_split_output_tmp_d00c6_14.1[1].get_m31(27).eq(M31_256))
                        & (value_limb_21_col51.eq(M31_136)));
                let ms_and_mid_limbs_are_max_col64 = ms_and_mid_limbs_are_max_tmp_d00c6_18.as_m31();
                *row[64] = ms_and_mid_limbs_are_max_col64;
                *sub_component_inputs.range_check_8[2] = [((read_split_output_tmp_d00c6_14.1[1]
                    .get_m31(27))
                    - (ms_limb_is_max_col63))];
                *lookup_data.range_check_8_2 = [((read_split_output_tmp_d00c6_14.1[1]
                    .get_m31(27))
                    - (ms_limb_is_max_col63))];
                let rc_input_col65 = ((ms_limb_is_max_col63)
                    * (((M31_120) + (value_limb_21_col51)) - (ms_and_mid_limbs_are_max_col64)));
                *row[65] = rc_input_col65;
                *sub_component_inputs.range_check_8[3] = [rc_input_col65];
                *lookup_data.range_check_8_3 = [rc_input_col65];

                *sub_component_inputs.pedersen_points_table[0] = [(((M31_7340032)
                    + ((ms_limb_high_col58) * (M31_16)))
                    + (ms_limb_high_col28))];
                let pedersen_points_table_output_tmp_d00c6_19 =
                    PackedPedersenPointsTable::deduce_output([(((M31_7340032)
                        + ((ms_limb_high_col58) * (M31_16)))
                        + (ms_limb_high_col28))]);
                let pedersen_points_table_output_limb_0_col66 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(0);
                *row[66] = pedersen_points_table_output_limb_0_col66;
                let pedersen_points_table_output_limb_1_col67 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(1);
                *row[67] = pedersen_points_table_output_limb_1_col67;
                let pedersen_points_table_output_limb_2_col68 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(2);
                *row[68] = pedersen_points_table_output_limb_2_col68;
                let pedersen_points_table_output_limb_3_col69 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(3);
                *row[69] = pedersen_points_table_output_limb_3_col69;
                let pedersen_points_table_output_limb_4_col70 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(4);
                *row[70] = pedersen_points_table_output_limb_4_col70;
                let pedersen_points_table_output_limb_5_col71 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(5);
                *row[71] = pedersen_points_table_output_limb_5_col71;
                let pedersen_points_table_output_limb_6_col72 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(6);
                *row[72] = pedersen_points_table_output_limb_6_col72;
                let pedersen_points_table_output_limb_7_col73 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(7);
                *row[73] = pedersen_points_table_output_limb_7_col73;
                let pedersen_points_table_output_limb_8_col74 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(8);
                *row[74] = pedersen_points_table_output_limb_8_col74;
                let pedersen_points_table_output_limb_9_col75 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(9);
                *row[75] = pedersen_points_table_output_limb_9_col75;
                let pedersen_points_table_output_limb_10_col76 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(10);
                *row[76] = pedersen_points_table_output_limb_10_col76;
                let pedersen_points_table_output_limb_11_col77 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(11);
                *row[77] = pedersen_points_table_output_limb_11_col77;
                let pedersen_points_table_output_limb_12_col78 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(12);
                *row[78] = pedersen_points_table_output_limb_12_col78;
                let pedersen_points_table_output_limb_13_col79 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(13);
                *row[79] = pedersen_points_table_output_limb_13_col79;
                let pedersen_points_table_output_limb_14_col80 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(14);
                *row[80] = pedersen_points_table_output_limb_14_col80;
                let pedersen_points_table_output_limb_15_col81 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(15);
                *row[81] = pedersen_points_table_output_limb_15_col81;
                let pedersen_points_table_output_limb_16_col82 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(16);
                *row[82] = pedersen_points_table_output_limb_16_col82;
                let pedersen_points_table_output_limb_17_col83 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(17);
                *row[83] = pedersen_points_table_output_limb_17_col83;
                let pedersen_points_table_output_limb_18_col84 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(18);
                *row[84] = pedersen_points_table_output_limb_18_col84;
                let pedersen_points_table_output_limb_19_col85 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(19);
                *row[85] = pedersen_points_table_output_limb_19_col85;
                let pedersen_points_table_output_limb_20_col86 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(20);
                *row[86] = pedersen_points_table_output_limb_20_col86;
                let pedersen_points_table_output_limb_21_col87 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(21);
                *row[87] = pedersen_points_table_output_limb_21_col87;
                let pedersen_points_table_output_limb_22_col88 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(22);
                *row[88] = pedersen_points_table_output_limb_22_col88;
                let pedersen_points_table_output_limb_23_col89 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(23);
                *row[89] = pedersen_points_table_output_limb_23_col89;
                let pedersen_points_table_output_limb_24_col90 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(24);
                *row[90] = pedersen_points_table_output_limb_24_col90;
                let pedersen_points_table_output_limb_25_col91 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(25);
                *row[91] = pedersen_points_table_output_limb_25_col91;
                let pedersen_points_table_output_limb_26_col92 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(26);
                *row[92] = pedersen_points_table_output_limb_26_col92;
                let pedersen_points_table_output_limb_27_col93 =
                    pedersen_points_table_output_tmp_d00c6_19[0].get_m31(27);
                *row[93] = pedersen_points_table_output_limb_27_col93;
                let pedersen_points_table_output_limb_28_col94 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(0);
                *row[94] = pedersen_points_table_output_limb_28_col94;
                let pedersen_points_table_output_limb_29_col95 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(1);
                *row[95] = pedersen_points_table_output_limb_29_col95;
                let pedersen_points_table_output_limb_30_col96 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(2);
                *row[96] = pedersen_points_table_output_limb_30_col96;
                let pedersen_points_table_output_limb_31_col97 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(3);
                *row[97] = pedersen_points_table_output_limb_31_col97;
                let pedersen_points_table_output_limb_32_col98 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(4);
                *row[98] = pedersen_points_table_output_limb_32_col98;
                let pedersen_points_table_output_limb_33_col99 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(5);
                *row[99] = pedersen_points_table_output_limb_33_col99;
                let pedersen_points_table_output_limb_34_col100 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(6);
                *row[100] = pedersen_points_table_output_limb_34_col100;
                let pedersen_points_table_output_limb_35_col101 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(7);
                *row[101] = pedersen_points_table_output_limb_35_col101;
                let pedersen_points_table_output_limb_36_col102 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(8);
                *row[102] = pedersen_points_table_output_limb_36_col102;
                let pedersen_points_table_output_limb_37_col103 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(9);
                *row[103] = pedersen_points_table_output_limb_37_col103;
                let pedersen_points_table_output_limb_38_col104 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(10);
                *row[104] = pedersen_points_table_output_limb_38_col104;
                let pedersen_points_table_output_limb_39_col105 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(11);
                *row[105] = pedersen_points_table_output_limb_39_col105;
                let pedersen_points_table_output_limb_40_col106 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(12);
                *row[106] = pedersen_points_table_output_limb_40_col106;
                let pedersen_points_table_output_limb_41_col107 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(13);
                *row[107] = pedersen_points_table_output_limb_41_col107;
                let pedersen_points_table_output_limb_42_col108 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(14);
                *row[108] = pedersen_points_table_output_limb_42_col108;
                let pedersen_points_table_output_limb_43_col109 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(15);
                *row[109] = pedersen_points_table_output_limb_43_col109;
                let pedersen_points_table_output_limb_44_col110 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(16);
                *row[110] = pedersen_points_table_output_limb_44_col110;
                let pedersen_points_table_output_limb_45_col111 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(17);
                *row[111] = pedersen_points_table_output_limb_45_col111;
                let pedersen_points_table_output_limb_46_col112 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(18);
                *row[112] = pedersen_points_table_output_limb_46_col112;
                let pedersen_points_table_output_limb_47_col113 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(19);
                *row[113] = pedersen_points_table_output_limb_47_col113;
                let pedersen_points_table_output_limb_48_col114 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(20);
                *row[114] = pedersen_points_table_output_limb_48_col114;
                let pedersen_points_table_output_limb_49_col115 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(21);
                *row[115] = pedersen_points_table_output_limb_49_col115;
                let pedersen_points_table_output_limb_50_col116 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(22);
                *row[116] = pedersen_points_table_output_limb_50_col116;
                let pedersen_points_table_output_limb_51_col117 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(23);
                *row[117] = pedersen_points_table_output_limb_51_col117;
                let pedersen_points_table_output_limb_52_col118 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(24);
                *row[118] = pedersen_points_table_output_limb_52_col118;
                let pedersen_points_table_output_limb_53_col119 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(25);
                *row[119] = pedersen_points_table_output_limb_53_col119;
                let pedersen_points_table_output_limb_54_col120 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(26);
                *row[120] = pedersen_points_table_output_limb_54_col120;
                let pedersen_points_table_output_limb_55_col121 =
                    pedersen_points_table_output_tmp_d00c6_19[1].get_m31(27);
                *row[121] = pedersen_points_table_output_limb_55_col121;
                *lookup_data.pedersen_points_table_0 = [
                    (((M31_7340032) + ((ms_limb_high_col58) * (M31_16))) + (ms_limb_high_col28)),
                    pedersen_points_table_output_limb_0_col66,
                    pedersen_points_table_output_limb_1_col67,
                    pedersen_points_table_output_limb_2_col68,
                    pedersen_points_table_output_limb_3_col69,
                    pedersen_points_table_output_limb_4_col70,
                    pedersen_points_table_output_limb_5_col71,
                    pedersen_points_table_output_limb_6_col72,
                    pedersen_points_table_output_limb_7_col73,
                    pedersen_points_table_output_limb_8_col74,
                    pedersen_points_table_output_limb_9_col75,
                    pedersen_points_table_output_limb_10_col76,
                    pedersen_points_table_output_limb_11_col77,
                    pedersen_points_table_output_limb_12_col78,
                    pedersen_points_table_output_limb_13_col79,
                    pedersen_points_table_output_limb_14_col80,
                    pedersen_points_table_output_limb_15_col81,
                    pedersen_points_table_output_limb_16_col82,
                    pedersen_points_table_output_limb_17_col83,
                    pedersen_points_table_output_limb_18_col84,
                    pedersen_points_table_output_limb_19_col85,
                    pedersen_points_table_output_limb_20_col86,
                    pedersen_points_table_output_limb_21_col87,
                    pedersen_points_table_output_limb_22_col88,
                    pedersen_points_table_output_limb_23_col89,
                    pedersen_points_table_output_limb_24_col90,
                    pedersen_points_table_output_limb_25_col91,
                    pedersen_points_table_output_limb_26_col92,
                    pedersen_points_table_output_limb_27_col93,
                    pedersen_points_table_output_limb_28_col94,
                    pedersen_points_table_output_limb_29_col95,
                    pedersen_points_table_output_limb_30_col96,
                    pedersen_points_table_output_limb_31_col97,
                    pedersen_points_table_output_limb_32_col98,
                    pedersen_points_table_output_limb_33_col99,
                    pedersen_points_table_output_limb_34_col100,
                    pedersen_points_table_output_limb_35_col101,
                    pedersen_points_table_output_limb_36_col102,
                    pedersen_points_table_output_limb_37_col103,
                    pedersen_points_table_output_limb_38_col104,
                    pedersen_points_table_output_limb_39_col105,
                    pedersen_points_table_output_limb_40_col106,
                    pedersen_points_table_output_limb_41_col107,
                    pedersen_points_table_output_limb_42_col108,
                    pedersen_points_table_output_limb_43_col109,
                    pedersen_points_table_output_limb_44_col110,
                    pedersen_points_table_output_limb_45_col111,
                    pedersen_points_table_output_limb_46_col112,
                    pedersen_points_table_output_limb_47_col113,
                    pedersen_points_table_output_limb_48_col114,
                    pedersen_points_table_output_limb_49_col115,
                    pedersen_points_table_output_limb_50_col116,
                    pedersen_points_table_output_limb_51_col117,
                    pedersen_points_table_output_limb_52_col118,
                    pedersen_points_table_output_limb_53_col119,
                    pedersen_points_table_output_limb_54_col120,
                    pedersen_points_table_output_limb_55_col121,
                ];
                let partial_ec_mul_chain_tmp_tmp_d00c6_20 = ((seq) * (M31_2));
                *lookup_data.partial_ec_mul_0 = [
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_0,
                    ((value_limb_0_col0) + ((value_limb_1_col1) * (M31_512))),
                    ((value_limb_2_col2) + ((value_limb_3_col3) * (M31_512))),
                    ((value_limb_4_col4) + ((value_limb_5_col5) * (M31_512))),
                    ((value_limb_6_col6) + ((value_limb_7_col7) * (M31_512))),
                    ((value_limb_8_col8) + ((value_limb_9_col9) * (M31_512))),
                    ((value_limb_10_col10) + ((value_limb_11_col11) * (M31_512))),
                    ((value_limb_12_col12) + ((value_limb_13_col13) * (M31_512))),
                    ((value_limb_14_col14) + ((value_limb_15_col15) * (M31_512))),
                    ((value_limb_16_col16) + ((value_limb_17_col17) * (M31_512))),
                    ((value_limb_18_col18) + ((value_limb_19_col19) * (M31_512))),
                    ((value_limb_20_col20) + ((value_limb_21_col21) * (M31_512))),
                    ((value_limb_22_col22) + ((value_limb_23_col23) * (M31_512))),
                    ((value_limb_24_col24) + ((value_limb_25_col25) * (M31_512))),
                    ((value_limb_26_col26) + ((ms_limb_low_col27) * (M31_512))),
                    pedersen_points_table_output_limb_0_col66,
                    pedersen_points_table_output_limb_1_col67,
                    pedersen_points_table_output_limb_2_col68,
                    pedersen_points_table_output_limb_3_col69,
                    pedersen_points_table_output_limb_4_col70,
                    pedersen_points_table_output_limb_5_col71,
                    pedersen_points_table_output_limb_6_col72,
                    pedersen_points_table_output_limb_7_col73,
                    pedersen_points_table_output_limb_8_col74,
                    pedersen_points_table_output_limb_9_col75,
                    pedersen_points_table_output_limb_10_col76,
                    pedersen_points_table_output_limb_11_col77,
                    pedersen_points_table_output_limb_12_col78,
                    pedersen_points_table_output_limb_13_col79,
                    pedersen_points_table_output_limb_14_col80,
                    pedersen_points_table_output_limb_15_col81,
                    pedersen_points_table_output_limb_16_col82,
                    pedersen_points_table_output_limb_17_col83,
                    pedersen_points_table_output_limb_18_col84,
                    pedersen_points_table_output_limb_19_col85,
                    pedersen_points_table_output_limb_20_col86,
                    pedersen_points_table_output_limb_21_col87,
                    pedersen_points_table_output_limb_22_col88,
                    pedersen_points_table_output_limb_23_col89,
                    pedersen_points_table_output_limb_24_col90,
                    pedersen_points_table_output_limb_25_col91,
                    pedersen_points_table_output_limb_26_col92,
                    pedersen_points_table_output_limb_27_col93,
                    pedersen_points_table_output_limb_28_col94,
                    pedersen_points_table_output_limb_29_col95,
                    pedersen_points_table_output_limb_30_col96,
                    pedersen_points_table_output_limb_31_col97,
                    pedersen_points_table_output_limb_32_col98,
                    pedersen_points_table_output_limb_33_col99,
                    pedersen_points_table_output_limb_34_col100,
                    pedersen_points_table_output_limb_35_col101,
                    pedersen_points_table_output_limb_36_col102,
                    pedersen_points_table_output_limb_37_col103,
                    pedersen_points_table_output_limb_38_col104,
                    pedersen_points_table_output_limb_39_col105,
                    pedersen_points_table_output_limb_40_col106,
                    pedersen_points_table_output_limb_41_col107,
                    pedersen_points_table_output_limb_42_col108,
                    pedersen_points_table_output_limb_43_col109,
                    pedersen_points_table_output_limb_44_col110,
                    pedersen_points_table_output_limb_45_col111,
                    pedersen_points_table_output_limb_46_col112,
                    pedersen_points_table_output_limb_47_col113,
                    pedersen_points_table_output_limb_48_col114,
                    pedersen_points_table_output_limb_49_col115,
                    pedersen_points_table_output_limb_50_col116,
                    pedersen_points_table_output_limb_51_col117,
                    pedersen_points_table_output_limb_52_col118,
                    pedersen_points_table_output_limb_53_col119,
                    pedersen_points_table_output_limb_54_col120,
                    pedersen_points_table_output_limb_55_col121,
                ];
                *sub_component_inputs.partial_ec_mul[0] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_0,
                    (
                        [
                            ((value_limb_0_col0) + ((value_limb_1_col1) * (M31_512))),
                            ((value_limb_2_col2) + ((value_limb_3_col3) * (M31_512))),
                            ((value_limb_4_col4) + ((value_limb_5_col5) * (M31_512))),
                            ((value_limb_6_col6) + ((value_limb_7_col7) * (M31_512))),
                            ((value_limb_8_col8) + ((value_limb_9_col9) * (M31_512))),
                            ((value_limb_10_col10) + ((value_limb_11_col11) * (M31_512))),
                            ((value_limb_12_col12) + ((value_limb_13_col13) * (M31_512))),
                            ((value_limb_14_col14) + ((value_limb_15_col15) * (M31_512))),
                            ((value_limb_16_col16) + ((value_limb_17_col17) * (M31_512))),
                            ((value_limb_18_col18) + ((value_limb_19_col19) * (M31_512))),
                            ((value_limb_20_col20) + ((value_limb_21_col21) * (M31_512))),
                            ((value_limb_22_col22) + ((value_limb_23_col23) * (M31_512))),
                            ((value_limb_24_col24) + ((value_limb_25_col25) * (M31_512))),
                            ((value_limb_26_col26) + ((ms_limb_low_col27) * (M31_512))),
                        ],
                        [
                            pedersen_points_table_output_tmp_d00c6_19[0],
                            pedersen_points_table_output_tmp_d00c6_19[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_0_tmp_d00c6_21 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_0,
                        (
                            [
                                ((value_limb_0_col0) + ((value_limb_1_col1) * (M31_512))),
                                ((value_limb_2_col2) + ((value_limb_3_col3) * (M31_512))),
                                ((value_limb_4_col4) + ((value_limb_5_col5) * (M31_512))),
                                ((value_limb_6_col6) + ((value_limb_7_col7) * (M31_512))),
                                ((value_limb_8_col8) + ((value_limb_9_col9) * (M31_512))),
                                ((value_limb_10_col10) + ((value_limb_11_col11) * (M31_512))),
                                ((value_limb_12_col12) + ((value_limb_13_col13) * (M31_512))),
                                ((value_limb_14_col14) + ((value_limb_15_col15) * (M31_512))),
                                ((value_limb_16_col16) + ((value_limb_17_col17) * (M31_512))),
                                ((value_limb_18_col18) + ((value_limb_19_col19) * (M31_512))),
                                ((value_limb_20_col20) + ((value_limb_21_col21) * (M31_512))),
                                ((value_limb_22_col22) + ((value_limb_23_col23) * (M31_512))),
                                ((value_limb_24_col24) + ((value_limb_25_col25) * (M31_512))),
                                ((value_limb_26_col26) + ((ms_limb_low_col27) * (M31_512))),
                            ],
                            [
                                pedersen_points_table_output_tmp_d00c6_19[0],
                                pedersen_points_table_output_tmp_d00c6_19[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[1] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_1,
                    (
                        [
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[0],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[1],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[2],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[3],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[4],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[5],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[6],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[7],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[8],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[9],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[10],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[11],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[12],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .1[0],
                            partial_ec_mul_output_round_0_tmp_d00c6_21.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_1_tmp_d00c6_22 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_1,
                        (
                            [
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[0],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[1],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[2],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[3],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[4],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[5],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[6],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[7],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[8],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[9],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[10],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[11],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[12],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .1[0],
                                partial_ec_mul_output_round_0_tmp_d00c6_21.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[2] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_2,
                    (
                        [
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[0],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[1],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[2],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[3],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[4],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[5],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[6],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[7],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[8],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[9],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[10],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[11],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[12],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .1[0],
                            partial_ec_mul_output_round_1_tmp_d00c6_22.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_2_tmp_d00c6_23 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_2,
                        (
                            [
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[0],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[1],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[2],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[3],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[4],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[5],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[6],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[7],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[8],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[9],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[10],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[11],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[12],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .1[0],
                                partial_ec_mul_output_round_1_tmp_d00c6_22.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[3] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_3,
                    (
                        [
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[0],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[1],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[2],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[3],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[4],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[5],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[6],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[7],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[8],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[9],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[10],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[11],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[12],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .1[0],
                            partial_ec_mul_output_round_2_tmp_d00c6_23.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_3_tmp_d00c6_24 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_3,
                        (
                            [
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[0],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[1],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[2],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[3],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[4],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[5],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[6],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[7],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[8],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[9],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[10],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[11],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[12],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .1[0],
                                partial_ec_mul_output_round_2_tmp_d00c6_23.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[4] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_4,
                    (
                        [
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[0],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[1],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[2],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[3],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[4],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[5],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[6],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[7],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[8],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[9],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[10],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[11],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[12],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .1[0],
                            partial_ec_mul_output_round_3_tmp_d00c6_24.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_4_tmp_d00c6_25 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_4,
                        (
                            [
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[0],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[1],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[2],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[3],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[4],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[5],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[6],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[7],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[8],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[9],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[10],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[11],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[12],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .1[0],
                                partial_ec_mul_output_round_3_tmp_d00c6_24.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[5] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_5,
                    (
                        [
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[0],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[1],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[2],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[3],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[4],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[5],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[6],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[7],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[8],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[9],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[10],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[11],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[12],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .1[0],
                            partial_ec_mul_output_round_4_tmp_d00c6_25.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_5_tmp_d00c6_26 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_5,
                        (
                            [
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[0],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[1],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[2],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[3],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[4],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[5],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[6],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[7],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[8],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[9],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[10],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[11],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[12],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .1[0],
                                partial_ec_mul_output_round_4_tmp_d00c6_25.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[6] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_6,
                    (
                        [
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[0],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[1],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[2],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[3],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[4],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[5],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[6],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[7],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[8],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[9],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[10],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[11],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[12],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .1[0],
                            partial_ec_mul_output_round_5_tmp_d00c6_26.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_6_tmp_d00c6_27 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_6,
                        (
                            [
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[0],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[1],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[2],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[3],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[4],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[5],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[6],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[7],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[8],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[9],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[10],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[11],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[12],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .1[0],
                                partial_ec_mul_output_round_5_tmp_d00c6_26.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[7] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_7,
                    (
                        [
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[0],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[1],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[2],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[3],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[4],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[5],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[6],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[7],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[8],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[9],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[10],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[11],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[12],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .1[0],
                            partial_ec_mul_output_round_6_tmp_d00c6_27.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_7_tmp_d00c6_28 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_7,
                        (
                            [
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[0],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[1],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[2],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[3],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[4],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[5],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[6],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[7],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[8],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[9],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[10],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[11],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[12],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .1[0],
                                partial_ec_mul_output_round_6_tmp_d00c6_27.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[8] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_8,
                    (
                        [
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[0],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[1],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[2],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[3],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[4],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[5],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[6],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[7],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[8],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[9],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[10],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[11],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[12],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .1[0],
                            partial_ec_mul_output_round_7_tmp_d00c6_28.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_8_tmp_d00c6_29 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_8,
                        (
                            [
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[0],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[1],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[2],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[3],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[4],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[5],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[6],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[7],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[8],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[9],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[10],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[11],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[12],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .1[0],
                                partial_ec_mul_output_round_7_tmp_d00c6_28.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[9] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_9,
                    (
                        [
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[0],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[1],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[2],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[3],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[4],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[5],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[6],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[7],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[8],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[9],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[10],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[11],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[12],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .1[0],
                            partial_ec_mul_output_round_8_tmp_d00c6_29.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_9_tmp_d00c6_30 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_9,
                        (
                            [
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[0],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[1],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[2],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[3],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[4],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[5],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[6],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[7],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[8],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[9],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[10],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[11],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[12],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .1[0],
                                partial_ec_mul_output_round_8_tmp_d00c6_29.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[10] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_10,
                    (
                        [
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[0],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[1],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[2],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[3],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[4],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[5],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[6],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[7],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[8],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[9],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[10],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[11],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[12],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .1[0],
                            partial_ec_mul_output_round_9_tmp_d00c6_30.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_10_tmp_d00c6_31 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_10,
                        (
                            [
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[0],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[1],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[2],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[3],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[4],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[5],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[6],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[7],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[8],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[9],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[10],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[11],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[12],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .1[0],
                                partial_ec_mul_output_round_9_tmp_d00c6_30.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[11] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_11,
                    (
                        [
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[0],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[1],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[2],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[3],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[4],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[5],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[6],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[7],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[8],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[9],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[10],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[11],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[12],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .1[0],
                            partial_ec_mul_output_round_10_tmp_d00c6_31.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_11_tmp_d00c6_32 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_11,
                        (
                            [
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[0],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[1],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[2],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[3],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[4],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[5],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[6],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[7],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[8],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[9],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[10],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[11],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[12],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .1[0],
                                partial_ec_mul_output_round_10_tmp_d00c6_31.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[12] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_12,
                    (
                        [
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[0],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[1],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[2],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[3],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[4],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[5],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[6],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[7],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[8],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[9],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[10],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[11],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[12],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .1[0],
                            partial_ec_mul_output_round_11_tmp_d00c6_32.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_12_tmp_d00c6_33 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_12,
                        (
                            [
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[0],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[1],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[2],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[3],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[4],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[5],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[6],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[7],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[8],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[9],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[10],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[11],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[12],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .1[0],
                                partial_ec_mul_output_round_11_tmp_d00c6_32.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[13] = (
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_13,
                    (
                        [
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[0],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[1],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[2],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[3],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[4],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[5],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[6],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[7],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[8],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[9],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[10],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[11],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[12],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .1[0],
                            partial_ec_mul_output_round_12_tmp_d00c6_33.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_13_tmp_d00c6_34 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_d00c6_20,
                        M31_13,
                        (
                            [
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[0],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[1],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[2],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[3],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[4],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[5],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[6],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[7],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[8],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[9],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[10],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[11],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[12],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .1[0],
                                partial_ec_mul_output_round_12_tmp_d00c6_33.2 .1[1],
                            ],
                        ),
                    ));
                let partial_ec_mul_output_limb_0_col122 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[0];
                *row[122] = partial_ec_mul_output_limb_0_col122;
                let partial_ec_mul_output_limb_1_col123 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[1];
                *row[123] = partial_ec_mul_output_limb_1_col123;
                let partial_ec_mul_output_limb_2_col124 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[2];
                *row[124] = partial_ec_mul_output_limb_2_col124;
                let partial_ec_mul_output_limb_3_col125 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[3];
                *row[125] = partial_ec_mul_output_limb_3_col125;
                let partial_ec_mul_output_limb_4_col126 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[4];
                *row[126] = partial_ec_mul_output_limb_4_col126;
                let partial_ec_mul_output_limb_5_col127 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[5];
                *row[127] = partial_ec_mul_output_limb_5_col127;
                let partial_ec_mul_output_limb_6_col128 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[6];
                *row[128] = partial_ec_mul_output_limb_6_col128;
                let partial_ec_mul_output_limb_7_col129 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[7];
                *row[129] = partial_ec_mul_output_limb_7_col129;
                let partial_ec_mul_output_limb_8_col130 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[8];
                *row[130] = partial_ec_mul_output_limb_8_col130;
                let partial_ec_mul_output_limb_9_col131 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[9];
                *row[131] = partial_ec_mul_output_limb_9_col131;
                let partial_ec_mul_output_limb_10_col132 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[10];
                *row[132] = partial_ec_mul_output_limb_10_col132;
                let partial_ec_mul_output_limb_11_col133 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[11];
                *row[133] = partial_ec_mul_output_limb_11_col133;
                let partial_ec_mul_output_limb_12_col134 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[12];
                *row[134] = partial_ec_mul_output_limb_12_col134;
                let partial_ec_mul_output_limb_13_col135 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .0[13];
                *row[135] = partial_ec_mul_output_limb_13_col135;
                let partial_ec_mul_output_limb_14_col136 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(0);
                *row[136] = partial_ec_mul_output_limb_14_col136;
                let partial_ec_mul_output_limb_15_col137 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(1);
                *row[137] = partial_ec_mul_output_limb_15_col137;
                let partial_ec_mul_output_limb_16_col138 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(2);
                *row[138] = partial_ec_mul_output_limb_16_col138;
                let partial_ec_mul_output_limb_17_col139 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(3);
                *row[139] = partial_ec_mul_output_limb_17_col139;
                let partial_ec_mul_output_limb_18_col140 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(4);
                *row[140] = partial_ec_mul_output_limb_18_col140;
                let partial_ec_mul_output_limb_19_col141 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(5);
                *row[141] = partial_ec_mul_output_limb_19_col141;
                let partial_ec_mul_output_limb_20_col142 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(6);
                *row[142] = partial_ec_mul_output_limb_20_col142;
                let partial_ec_mul_output_limb_21_col143 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(7);
                *row[143] = partial_ec_mul_output_limb_21_col143;
                let partial_ec_mul_output_limb_22_col144 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(8);
                *row[144] = partial_ec_mul_output_limb_22_col144;
                let partial_ec_mul_output_limb_23_col145 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(9);
                *row[145] = partial_ec_mul_output_limb_23_col145;
                let partial_ec_mul_output_limb_24_col146 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(10);
                *row[146] = partial_ec_mul_output_limb_24_col146;
                let partial_ec_mul_output_limb_25_col147 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(11);
                *row[147] = partial_ec_mul_output_limb_25_col147;
                let partial_ec_mul_output_limb_26_col148 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(12);
                *row[148] = partial_ec_mul_output_limb_26_col148;
                let partial_ec_mul_output_limb_27_col149 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(13);
                *row[149] = partial_ec_mul_output_limb_27_col149;
                let partial_ec_mul_output_limb_28_col150 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(14);
                *row[150] = partial_ec_mul_output_limb_28_col150;
                let partial_ec_mul_output_limb_29_col151 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(15);
                *row[151] = partial_ec_mul_output_limb_29_col151;
                let partial_ec_mul_output_limb_30_col152 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(16);
                *row[152] = partial_ec_mul_output_limb_30_col152;
                let partial_ec_mul_output_limb_31_col153 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(17);
                *row[153] = partial_ec_mul_output_limb_31_col153;
                let partial_ec_mul_output_limb_32_col154 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(18);
                *row[154] = partial_ec_mul_output_limb_32_col154;
                let partial_ec_mul_output_limb_33_col155 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(19);
                *row[155] = partial_ec_mul_output_limb_33_col155;
                let partial_ec_mul_output_limb_34_col156 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(20);
                *row[156] = partial_ec_mul_output_limb_34_col156;
                let partial_ec_mul_output_limb_35_col157 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(21);
                *row[157] = partial_ec_mul_output_limb_35_col157;
                let partial_ec_mul_output_limb_36_col158 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(22);
                *row[158] = partial_ec_mul_output_limb_36_col158;
                let partial_ec_mul_output_limb_37_col159 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(23);
                *row[159] = partial_ec_mul_output_limb_37_col159;
                let partial_ec_mul_output_limb_38_col160 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(24);
                *row[160] = partial_ec_mul_output_limb_38_col160;
                let partial_ec_mul_output_limb_39_col161 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(25);
                *row[161] = partial_ec_mul_output_limb_39_col161;
                let partial_ec_mul_output_limb_40_col162 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(26);
                *row[162] = partial_ec_mul_output_limb_40_col162;
                let partial_ec_mul_output_limb_41_col163 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0].get_m31(27);
                *row[163] = partial_ec_mul_output_limb_41_col163;
                let partial_ec_mul_output_limb_42_col164 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(0);
                *row[164] = partial_ec_mul_output_limb_42_col164;
                let partial_ec_mul_output_limb_43_col165 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(1);
                *row[165] = partial_ec_mul_output_limb_43_col165;
                let partial_ec_mul_output_limb_44_col166 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(2);
                *row[166] = partial_ec_mul_output_limb_44_col166;
                let partial_ec_mul_output_limb_45_col167 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(3);
                *row[167] = partial_ec_mul_output_limb_45_col167;
                let partial_ec_mul_output_limb_46_col168 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(4);
                *row[168] = partial_ec_mul_output_limb_46_col168;
                let partial_ec_mul_output_limb_47_col169 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(5);
                *row[169] = partial_ec_mul_output_limb_47_col169;
                let partial_ec_mul_output_limb_48_col170 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(6);
                *row[170] = partial_ec_mul_output_limb_48_col170;
                let partial_ec_mul_output_limb_49_col171 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(7);
                *row[171] = partial_ec_mul_output_limb_49_col171;
                let partial_ec_mul_output_limb_50_col172 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(8);
                *row[172] = partial_ec_mul_output_limb_50_col172;
                let partial_ec_mul_output_limb_51_col173 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(9);
                *row[173] = partial_ec_mul_output_limb_51_col173;
                let partial_ec_mul_output_limb_52_col174 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(10);
                *row[174] = partial_ec_mul_output_limb_52_col174;
                let partial_ec_mul_output_limb_53_col175 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(11);
                *row[175] = partial_ec_mul_output_limb_53_col175;
                let partial_ec_mul_output_limb_54_col176 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(12);
                *row[176] = partial_ec_mul_output_limb_54_col176;
                let partial_ec_mul_output_limb_55_col177 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(13);
                *row[177] = partial_ec_mul_output_limb_55_col177;
                let partial_ec_mul_output_limb_56_col178 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(14);
                *row[178] = partial_ec_mul_output_limb_56_col178;
                let partial_ec_mul_output_limb_57_col179 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(15);
                *row[179] = partial_ec_mul_output_limb_57_col179;
                let partial_ec_mul_output_limb_58_col180 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(16);
                *row[180] = partial_ec_mul_output_limb_58_col180;
                let partial_ec_mul_output_limb_59_col181 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(17);
                *row[181] = partial_ec_mul_output_limb_59_col181;
                let partial_ec_mul_output_limb_60_col182 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(18);
                *row[182] = partial_ec_mul_output_limb_60_col182;
                let partial_ec_mul_output_limb_61_col183 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(19);
                *row[183] = partial_ec_mul_output_limb_61_col183;
                let partial_ec_mul_output_limb_62_col184 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(20);
                *row[184] = partial_ec_mul_output_limb_62_col184;
                let partial_ec_mul_output_limb_63_col185 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(21);
                *row[185] = partial_ec_mul_output_limb_63_col185;
                let partial_ec_mul_output_limb_64_col186 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(22);
                *row[186] = partial_ec_mul_output_limb_64_col186;
                let partial_ec_mul_output_limb_65_col187 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(23);
                *row[187] = partial_ec_mul_output_limb_65_col187;
                let partial_ec_mul_output_limb_66_col188 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(24);
                *row[188] = partial_ec_mul_output_limb_66_col188;
                let partial_ec_mul_output_limb_67_col189 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(25);
                *row[189] = partial_ec_mul_output_limb_67_col189;
                let partial_ec_mul_output_limb_68_col190 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(26);
                *row[190] = partial_ec_mul_output_limb_68_col190;
                let partial_ec_mul_output_limb_69_col191 =
                    partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1].get_m31(27);
                *row[191] = partial_ec_mul_output_limb_69_col191;
                *lookup_data.partial_ec_mul_1 = [
                    partial_ec_mul_chain_tmp_tmp_d00c6_20,
                    M31_14,
                    partial_ec_mul_output_limb_0_col122,
                    partial_ec_mul_output_limb_1_col123,
                    partial_ec_mul_output_limb_2_col124,
                    partial_ec_mul_output_limb_3_col125,
                    partial_ec_mul_output_limb_4_col126,
                    partial_ec_mul_output_limb_5_col127,
                    partial_ec_mul_output_limb_6_col128,
                    partial_ec_mul_output_limb_7_col129,
                    partial_ec_mul_output_limb_8_col130,
                    partial_ec_mul_output_limb_9_col131,
                    partial_ec_mul_output_limb_10_col132,
                    partial_ec_mul_output_limb_11_col133,
                    partial_ec_mul_output_limb_12_col134,
                    partial_ec_mul_output_limb_13_col135,
                    partial_ec_mul_output_limb_14_col136,
                    partial_ec_mul_output_limb_15_col137,
                    partial_ec_mul_output_limb_16_col138,
                    partial_ec_mul_output_limb_17_col139,
                    partial_ec_mul_output_limb_18_col140,
                    partial_ec_mul_output_limb_19_col141,
                    partial_ec_mul_output_limb_20_col142,
                    partial_ec_mul_output_limb_21_col143,
                    partial_ec_mul_output_limb_22_col144,
                    partial_ec_mul_output_limb_23_col145,
                    partial_ec_mul_output_limb_24_col146,
                    partial_ec_mul_output_limb_25_col147,
                    partial_ec_mul_output_limb_26_col148,
                    partial_ec_mul_output_limb_27_col149,
                    partial_ec_mul_output_limb_28_col150,
                    partial_ec_mul_output_limb_29_col151,
                    partial_ec_mul_output_limb_30_col152,
                    partial_ec_mul_output_limb_31_col153,
                    partial_ec_mul_output_limb_32_col154,
                    partial_ec_mul_output_limb_33_col155,
                    partial_ec_mul_output_limb_34_col156,
                    partial_ec_mul_output_limb_35_col157,
                    partial_ec_mul_output_limb_36_col158,
                    partial_ec_mul_output_limb_37_col159,
                    partial_ec_mul_output_limb_38_col160,
                    partial_ec_mul_output_limb_39_col161,
                    partial_ec_mul_output_limb_40_col162,
                    partial_ec_mul_output_limb_41_col163,
                    partial_ec_mul_output_limb_42_col164,
                    partial_ec_mul_output_limb_43_col165,
                    partial_ec_mul_output_limb_44_col166,
                    partial_ec_mul_output_limb_45_col167,
                    partial_ec_mul_output_limb_46_col168,
                    partial_ec_mul_output_limb_47_col169,
                    partial_ec_mul_output_limb_48_col170,
                    partial_ec_mul_output_limb_49_col171,
                    partial_ec_mul_output_limb_50_col172,
                    partial_ec_mul_output_limb_51_col173,
                    partial_ec_mul_output_limb_52_col174,
                    partial_ec_mul_output_limb_53_col175,
                    partial_ec_mul_output_limb_54_col176,
                    partial_ec_mul_output_limb_55_col177,
                    partial_ec_mul_output_limb_56_col178,
                    partial_ec_mul_output_limb_57_col179,
                    partial_ec_mul_output_limb_58_col180,
                    partial_ec_mul_output_limb_59_col181,
                    partial_ec_mul_output_limb_60_col182,
                    partial_ec_mul_output_limb_61_col183,
                    partial_ec_mul_output_limb_62_col184,
                    partial_ec_mul_output_limb_63_col185,
                    partial_ec_mul_output_limb_64_col186,
                    partial_ec_mul_output_limb_65_col187,
                    partial_ec_mul_output_limb_66_col188,
                    partial_ec_mul_output_limb_67_col189,
                    partial_ec_mul_output_limb_68_col190,
                    partial_ec_mul_output_limb_69_col191,
                ];
                let partial_ec_mul_chain_id_tmp_d00c6_35 =
                    ((partial_ec_mul_chain_tmp_tmp_d00c6_20) + (M31_1));
                *lookup_data.partial_ec_mul_2 = [
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_14,
                    ((value_limb_0_col30) + ((value_limb_1_col31) * (M31_512))),
                    ((value_limb_2_col32) + ((value_limb_3_col33) * (M31_512))),
                    ((value_limb_4_col34) + ((value_limb_5_col35) * (M31_512))),
                    ((value_limb_6_col36) + ((value_limb_7_col37) * (M31_512))),
                    ((value_limb_8_col38) + ((value_limb_9_col39) * (M31_512))),
                    ((value_limb_10_col40) + ((value_limb_11_col41) * (M31_512))),
                    ((value_limb_12_col42) + ((value_limb_13_col43) * (M31_512))),
                    ((value_limb_14_col44) + ((value_limb_15_col45) * (M31_512))),
                    ((value_limb_16_col46) + ((value_limb_17_col47) * (M31_512))),
                    ((value_limb_18_col48) + ((value_limb_19_col49) * (M31_512))),
                    ((value_limb_20_col50) + ((value_limb_21_col51) * (M31_512))),
                    ((value_limb_22_col52) + ((value_limb_23_col53) * (M31_512))),
                    ((value_limb_24_col54) + ((value_limb_25_col55) * (M31_512))),
                    ((value_limb_26_col56) + ((ms_limb_low_col57) * (M31_512))),
                    partial_ec_mul_output_limb_14_col136,
                    partial_ec_mul_output_limb_15_col137,
                    partial_ec_mul_output_limb_16_col138,
                    partial_ec_mul_output_limb_17_col139,
                    partial_ec_mul_output_limb_18_col140,
                    partial_ec_mul_output_limb_19_col141,
                    partial_ec_mul_output_limb_20_col142,
                    partial_ec_mul_output_limb_21_col143,
                    partial_ec_mul_output_limb_22_col144,
                    partial_ec_mul_output_limb_23_col145,
                    partial_ec_mul_output_limb_24_col146,
                    partial_ec_mul_output_limb_25_col147,
                    partial_ec_mul_output_limb_26_col148,
                    partial_ec_mul_output_limb_27_col149,
                    partial_ec_mul_output_limb_28_col150,
                    partial_ec_mul_output_limb_29_col151,
                    partial_ec_mul_output_limb_30_col152,
                    partial_ec_mul_output_limb_31_col153,
                    partial_ec_mul_output_limb_32_col154,
                    partial_ec_mul_output_limb_33_col155,
                    partial_ec_mul_output_limb_34_col156,
                    partial_ec_mul_output_limb_35_col157,
                    partial_ec_mul_output_limb_36_col158,
                    partial_ec_mul_output_limb_37_col159,
                    partial_ec_mul_output_limb_38_col160,
                    partial_ec_mul_output_limb_39_col161,
                    partial_ec_mul_output_limb_40_col162,
                    partial_ec_mul_output_limb_41_col163,
                    partial_ec_mul_output_limb_42_col164,
                    partial_ec_mul_output_limb_43_col165,
                    partial_ec_mul_output_limb_44_col166,
                    partial_ec_mul_output_limb_45_col167,
                    partial_ec_mul_output_limb_46_col168,
                    partial_ec_mul_output_limb_47_col169,
                    partial_ec_mul_output_limb_48_col170,
                    partial_ec_mul_output_limb_49_col171,
                    partial_ec_mul_output_limb_50_col172,
                    partial_ec_mul_output_limb_51_col173,
                    partial_ec_mul_output_limb_52_col174,
                    partial_ec_mul_output_limb_53_col175,
                    partial_ec_mul_output_limb_54_col176,
                    partial_ec_mul_output_limb_55_col177,
                    partial_ec_mul_output_limb_56_col178,
                    partial_ec_mul_output_limb_57_col179,
                    partial_ec_mul_output_limb_58_col180,
                    partial_ec_mul_output_limb_59_col181,
                    partial_ec_mul_output_limb_60_col182,
                    partial_ec_mul_output_limb_61_col183,
                    partial_ec_mul_output_limb_62_col184,
                    partial_ec_mul_output_limb_63_col185,
                    partial_ec_mul_output_limb_64_col186,
                    partial_ec_mul_output_limb_65_col187,
                    partial_ec_mul_output_limb_66_col188,
                    partial_ec_mul_output_limb_67_col189,
                    partial_ec_mul_output_limb_68_col190,
                    partial_ec_mul_output_limb_69_col191,
                ];
                *sub_component_inputs.partial_ec_mul[14] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_14,
                    (
                        [
                            ((value_limb_0_col30) + ((value_limb_1_col31) * (M31_512))),
                            ((value_limb_2_col32) + ((value_limb_3_col33) * (M31_512))),
                            ((value_limb_4_col34) + ((value_limb_5_col35) * (M31_512))),
                            ((value_limb_6_col36) + ((value_limb_7_col37) * (M31_512))),
                            ((value_limb_8_col38) + ((value_limb_9_col39) * (M31_512))),
                            ((value_limb_10_col40) + ((value_limb_11_col41) * (M31_512))),
                            ((value_limb_12_col42) + ((value_limb_13_col43) * (M31_512))),
                            ((value_limb_14_col44) + ((value_limb_15_col45) * (M31_512))),
                            ((value_limb_16_col46) + ((value_limb_17_col47) * (M31_512))),
                            ((value_limb_18_col48) + ((value_limb_19_col49) * (M31_512))),
                            ((value_limb_20_col50) + ((value_limb_21_col51) * (M31_512))),
                            ((value_limb_22_col52) + ((value_limb_23_col53) * (M31_512))),
                            ((value_limb_24_col54) + ((value_limb_25_col55) * (M31_512))),
                            ((value_limb_26_col56) + ((ms_limb_low_col57) * (M31_512))),
                        ],
                        [
                            partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0],
                            partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_14_tmp_d00c6_36 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_14,
                        (
                            [
                                ((value_limb_0_col30) + ((value_limb_1_col31) * (M31_512))),
                                ((value_limb_2_col32) + ((value_limb_3_col33) * (M31_512))),
                                ((value_limb_4_col34) + ((value_limb_5_col35) * (M31_512))),
                                ((value_limb_6_col36) + ((value_limb_7_col37) * (M31_512))),
                                ((value_limb_8_col38) + ((value_limb_9_col39) * (M31_512))),
                                ((value_limb_10_col40) + ((value_limb_11_col41) * (M31_512))),
                                ((value_limb_12_col42) + ((value_limb_13_col43) * (M31_512))),
                                ((value_limb_14_col44) + ((value_limb_15_col45) * (M31_512))),
                                ((value_limb_16_col46) + ((value_limb_17_col47) * (M31_512))),
                                ((value_limb_18_col48) + ((value_limb_19_col49) * (M31_512))),
                                ((value_limb_20_col50) + ((value_limb_21_col51) * (M31_512))),
                                ((value_limb_22_col52) + ((value_limb_23_col53) * (M31_512))),
                                ((value_limb_24_col54) + ((value_limb_25_col55) * (M31_512))),
                                ((value_limb_26_col56) + ((ms_limb_low_col57) * (M31_512))),
                            ],
                            [
                                partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[0],
                                partial_ec_mul_output_round_13_tmp_d00c6_34.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[15] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_15,
                    (
                        [
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[0],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[1],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[2],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[3],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[4],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[5],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[6],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[7],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[8],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[9],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[10],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[11],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[12],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .1[0],
                            partial_ec_mul_output_round_14_tmp_d00c6_36.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_15_tmp_d00c6_37 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_15,
                        (
                            [
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[0],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[1],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[2],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[3],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[4],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[5],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[6],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[7],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[8],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[9],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[10],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[11],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[12],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .1[0],
                                partial_ec_mul_output_round_14_tmp_d00c6_36.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[16] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_16,
                    (
                        [
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[0],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[1],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[2],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[3],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[4],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[5],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[6],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[7],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[8],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[9],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[10],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[11],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[12],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .1[0],
                            partial_ec_mul_output_round_15_tmp_d00c6_37.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_16_tmp_d00c6_38 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_16,
                        (
                            [
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[0],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[1],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[2],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[3],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[4],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[5],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[6],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[7],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[8],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[9],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[10],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[11],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[12],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .1[0],
                                partial_ec_mul_output_round_15_tmp_d00c6_37.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[17] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_17,
                    (
                        [
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[0],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[1],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[2],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[3],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[4],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[5],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[6],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[7],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[8],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[9],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[10],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[11],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[12],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .1[0],
                            partial_ec_mul_output_round_16_tmp_d00c6_38.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_17_tmp_d00c6_39 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_17,
                        (
                            [
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[0],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[1],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[2],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[3],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[4],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[5],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[6],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[7],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[8],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[9],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[10],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[11],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[12],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .1[0],
                                partial_ec_mul_output_round_16_tmp_d00c6_38.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[18] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_18,
                    (
                        [
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[0],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[1],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[2],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[3],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[4],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[5],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[6],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[7],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[8],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[9],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[10],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[11],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[12],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .1[0],
                            partial_ec_mul_output_round_17_tmp_d00c6_39.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_18_tmp_d00c6_40 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_18,
                        (
                            [
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[0],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[1],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[2],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[3],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[4],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[5],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[6],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[7],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[8],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[9],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[10],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[11],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[12],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .1[0],
                                partial_ec_mul_output_round_17_tmp_d00c6_39.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[19] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_19,
                    (
                        [
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[0],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[1],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[2],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[3],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[4],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[5],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[6],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[7],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[8],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[9],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[10],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[11],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[12],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .1[0],
                            partial_ec_mul_output_round_18_tmp_d00c6_40.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_19_tmp_d00c6_41 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_19,
                        (
                            [
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[0],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[1],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[2],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[3],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[4],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[5],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[6],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[7],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[8],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[9],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[10],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[11],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[12],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .1[0],
                                partial_ec_mul_output_round_18_tmp_d00c6_40.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[20] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_20,
                    (
                        [
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[0],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[1],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[2],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[3],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[4],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[5],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[6],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[7],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[8],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[9],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[10],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[11],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[12],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .1[0],
                            partial_ec_mul_output_round_19_tmp_d00c6_41.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_20_tmp_d00c6_42 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_20,
                        (
                            [
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[0],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[1],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[2],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[3],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[4],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[5],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[6],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[7],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[8],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[9],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[10],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[11],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[12],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .1[0],
                                partial_ec_mul_output_round_19_tmp_d00c6_41.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[21] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_21,
                    (
                        [
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[0],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[1],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[2],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[3],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[4],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[5],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[6],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[7],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[8],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[9],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[10],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[11],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[12],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .1[0],
                            partial_ec_mul_output_round_20_tmp_d00c6_42.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_21_tmp_d00c6_43 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_21,
                        (
                            [
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[0],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[1],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[2],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[3],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[4],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[5],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[6],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[7],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[8],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[9],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[10],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[11],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[12],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .1[0],
                                partial_ec_mul_output_round_20_tmp_d00c6_42.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[22] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_22,
                    (
                        [
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[0],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[1],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[2],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[3],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[4],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[5],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[6],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[7],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[8],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[9],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[10],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[11],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[12],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .1[0],
                            partial_ec_mul_output_round_21_tmp_d00c6_43.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_22_tmp_d00c6_44 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_22,
                        (
                            [
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[0],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[1],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[2],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[3],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[4],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[5],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[6],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[7],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[8],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[9],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[10],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[11],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[12],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .1[0],
                                partial_ec_mul_output_round_21_tmp_d00c6_43.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[23] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_23,
                    (
                        [
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[0],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[1],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[2],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[3],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[4],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[5],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[6],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[7],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[8],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[9],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[10],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[11],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[12],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .1[0],
                            partial_ec_mul_output_round_22_tmp_d00c6_44.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_23_tmp_d00c6_45 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_23,
                        (
                            [
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[0],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[1],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[2],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[3],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[4],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[5],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[6],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[7],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[8],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[9],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[10],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[11],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[12],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .1[0],
                                partial_ec_mul_output_round_22_tmp_d00c6_44.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[24] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_24,
                    (
                        [
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[0],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[1],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[2],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[3],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[4],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[5],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[6],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[7],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[8],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[9],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[10],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[11],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[12],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .1[0],
                            partial_ec_mul_output_round_23_tmp_d00c6_45.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_24_tmp_d00c6_46 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_24,
                        (
                            [
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[0],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[1],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[2],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[3],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[4],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[5],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[6],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[7],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[8],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[9],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[10],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[11],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[12],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .1[0],
                                partial_ec_mul_output_round_23_tmp_d00c6_45.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[25] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_25,
                    (
                        [
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[0],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[1],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[2],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[3],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[4],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[5],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[6],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[7],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[8],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[9],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[10],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[11],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[12],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .1[0],
                            partial_ec_mul_output_round_24_tmp_d00c6_46.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_25_tmp_d00c6_47 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_25,
                        (
                            [
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[0],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[1],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[2],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[3],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[4],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[5],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[6],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[7],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[8],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[9],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[10],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[11],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[12],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .1[0],
                                partial_ec_mul_output_round_24_tmp_d00c6_46.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[26] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_26,
                    (
                        [
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[0],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[1],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[2],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[3],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[4],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[5],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[6],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[7],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[8],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[9],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[10],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[11],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[12],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .1[0],
                            partial_ec_mul_output_round_25_tmp_d00c6_47.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_26_tmp_d00c6_48 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_26,
                        (
                            [
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[0],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[1],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[2],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[3],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[4],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[5],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[6],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[7],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[8],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[9],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[10],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[11],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[12],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .1[0],
                                partial_ec_mul_output_round_25_tmp_d00c6_47.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[27] = (
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_27,
                    (
                        [
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[0],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[1],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[2],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[3],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[4],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[5],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[6],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[7],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[8],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[9],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[10],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[11],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[12],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .1[0],
                            partial_ec_mul_output_round_26_tmp_d00c6_48.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_27_tmp_d00c6_49 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_d00c6_35,
                        M31_27,
                        (
                            [
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[0],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[1],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[2],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[3],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[4],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[5],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[6],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[7],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[8],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[9],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[10],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[11],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[12],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .1[0],
                                partial_ec_mul_output_round_26_tmp_d00c6_48.2 .1[1],
                            ],
                        ),
                    ));
                let partial_ec_mul_output_limb_0_col192 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[0];
                *row[192] = partial_ec_mul_output_limb_0_col192;
                let partial_ec_mul_output_limb_1_col193 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[1];
                *row[193] = partial_ec_mul_output_limb_1_col193;
                let partial_ec_mul_output_limb_2_col194 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[2];
                *row[194] = partial_ec_mul_output_limb_2_col194;
                let partial_ec_mul_output_limb_3_col195 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[3];
                *row[195] = partial_ec_mul_output_limb_3_col195;
                let partial_ec_mul_output_limb_4_col196 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[4];
                *row[196] = partial_ec_mul_output_limb_4_col196;
                let partial_ec_mul_output_limb_5_col197 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[5];
                *row[197] = partial_ec_mul_output_limb_5_col197;
                let partial_ec_mul_output_limb_6_col198 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[6];
                *row[198] = partial_ec_mul_output_limb_6_col198;
                let partial_ec_mul_output_limb_7_col199 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[7];
                *row[199] = partial_ec_mul_output_limb_7_col199;
                let partial_ec_mul_output_limb_8_col200 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[8];
                *row[200] = partial_ec_mul_output_limb_8_col200;
                let partial_ec_mul_output_limb_9_col201 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[9];
                *row[201] = partial_ec_mul_output_limb_9_col201;
                let partial_ec_mul_output_limb_10_col202 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[10];
                *row[202] = partial_ec_mul_output_limb_10_col202;
                let partial_ec_mul_output_limb_11_col203 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[11];
                *row[203] = partial_ec_mul_output_limb_11_col203;
                let partial_ec_mul_output_limb_12_col204 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[12];
                *row[204] = partial_ec_mul_output_limb_12_col204;
                let partial_ec_mul_output_limb_13_col205 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .0[13];
                *row[205] = partial_ec_mul_output_limb_13_col205;
                let partial_ec_mul_output_limb_14_col206 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(0);
                *row[206] = partial_ec_mul_output_limb_14_col206;
                let partial_ec_mul_output_limb_15_col207 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(1);
                *row[207] = partial_ec_mul_output_limb_15_col207;
                let partial_ec_mul_output_limb_16_col208 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(2);
                *row[208] = partial_ec_mul_output_limb_16_col208;
                let partial_ec_mul_output_limb_17_col209 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(3);
                *row[209] = partial_ec_mul_output_limb_17_col209;
                let partial_ec_mul_output_limb_18_col210 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(4);
                *row[210] = partial_ec_mul_output_limb_18_col210;
                let partial_ec_mul_output_limb_19_col211 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(5);
                *row[211] = partial_ec_mul_output_limb_19_col211;
                let partial_ec_mul_output_limb_20_col212 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(6);
                *row[212] = partial_ec_mul_output_limb_20_col212;
                let partial_ec_mul_output_limb_21_col213 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(7);
                *row[213] = partial_ec_mul_output_limb_21_col213;
                let partial_ec_mul_output_limb_22_col214 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(8);
                *row[214] = partial_ec_mul_output_limb_22_col214;
                let partial_ec_mul_output_limb_23_col215 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(9);
                *row[215] = partial_ec_mul_output_limb_23_col215;
                let partial_ec_mul_output_limb_24_col216 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(10);
                *row[216] = partial_ec_mul_output_limb_24_col216;
                let partial_ec_mul_output_limb_25_col217 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(11);
                *row[217] = partial_ec_mul_output_limb_25_col217;
                let partial_ec_mul_output_limb_26_col218 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(12);
                *row[218] = partial_ec_mul_output_limb_26_col218;
                let partial_ec_mul_output_limb_27_col219 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(13);
                *row[219] = partial_ec_mul_output_limb_27_col219;
                let partial_ec_mul_output_limb_28_col220 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(14);
                *row[220] = partial_ec_mul_output_limb_28_col220;
                let partial_ec_mul_output_limb_29_col221 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(15);
                *row[221] = partial_ec_mul_output_limb_29_col221;
                let partial_ec_mul_output_limb_30_col222 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(16);
                *row[222] = partial_ec_mul_output_limb_30_col222;
                let partial_ec_mul_output_limb_31_col223 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(17);
                *row[223] = partial_ec_mul_output_limb_31_col223;
                let partial_ec_mul_output_limb_32_col224 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(18);
                *row[224] = partial_ec_mul_output_limb_32_col224;
                let partial_ec_mul_output_limb_33_col225 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(19);
                *row[225] = partial_ec_mul_output_limb_33_col225;
                let partial_ec_mul_output_limb_34_col226 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(20);
                *row[226] = partial_ec_mul_output_limb_34_col226;
                let partial_ec_mul_output_limb_35_col227 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(21);
                *row[227] = partial_ec_mul_output_limb_35_col227;
                let partial_ec_mul_output_limb_36_col228 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(22);
                *row[228] = partial_ec_mul_output_limb_36_col228;
                let partial_ec_mul_output_limb_37_col229 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(23);
                *row[229] = partial_ec_mul_output_limb_37_col229;
                let partial_ec_mul_output_limb_38_col230 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(24);
                *row[230] = partial_ec_mul_output_limb_38_col230;
                let partial_ec_mul_output_limb_39_col231 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(25);
                *row[231] = partial_ec_mul_output_limb_39_col231;
                let partial_ec_mul_output_limb_40_col232 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(26);
                *row[232] = partial_ec_mul_output_limb_40_col232;
                let partial_ec_mul_output_limb_41_col233 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[0].get_m31(27);
                *row[233] = partial_ec_mul_output_limb_41_col233;
                let partial_ec_mul_output_limb_42_col234 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(0);
                *row[234] = partial_ec_mul_output_limb_42_col234;
                let partial_ec_mul_output_limb_43_col235 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(1);
                *row[235] = partial_ec_mul_output_limb_43_col235;
                let partial_ec_mul_output_limb_44_col236 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(2);
                *row[236] = partial_ec_mul_output_limb_44_col236;
                let partial_ec_mul_output_limb_45_col237 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(3);
                *row[237] = partial_ec_mul_output_limb_45_col237;
                let partial_ec_mul_output_limb_46_col238 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(4);
                *row[238] = partial_ec_mul_output_limb_46_col238;
                let partial_ec_mul_output_limb_47_col239 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(5);
                *row[239] = partial_ec_mul_output_limb_47_col239;
                let partial_ec_mul_output_limb_48_col240 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(6);
                *row[240] = partial_ec_mul_output_limb_48_col240;
                let partial_ec_mul_output_limb_49_col241 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(7);
                *row[241] = partial_ec_mul_output_limb_49_col241;
                let partial_ec_mul_output_limb_50_col242 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(8);
                *row[242] = partial_ec_mul_output_limb_50_col242;
                let partial_ec_mul_output_limb_51_col243 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(9);
                *row[243] = partial_ec_mul_output_limb_51_col243;
                let partial_ec_mul_output_limb_52_col244 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(10);
                *row[244] = partial_ec_mul_output_limb_52_col244;
                let partial_ec_mul_output_limb_53_col245 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(11);
                *row[245] = partial_ec_mul_output_limb_53_col245;
                let partial_ec_mul_output_limb_54_col246 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(12);
                *row[246] = partial_ec_mul_output_limb_54_col246;
                let partial_ec_mul_output_limb_55_col247 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(13);
                *row[247] = partial_ec_mul_output_limb_55_col247;
                let partial_ec_mul_output_limb_56_col248 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(14);
                *row[248] = partial_ec_mul_output_limb_56_col248;
                let partial_ec_mul_output_limb_57_col249 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(15);
                *row[249] = partial_ec_mul_output_limb_57_col249;
                let partial_ec_mul_output_limb_58_col250 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(16);
                *row[250] = partial_ec_mul_output_limb_58_col250;
                let partial_ec_mul_output_limb_59_col251 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(17);
                *row[251] = partial_ec_mul_output_limb_59_col251;
                let partial_ec_mul_output_limb_60_col252 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(18);
                *row[252] = partial_ec_mul_output_limb_60_col252;
                let partial_ec_mul_output_limb_61_col253 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(19);
                *row[253] = partial_ec_mul_output_limb_61_col253;
                let partial_ec_mul_output_limb_62_col254 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(20);
                *row[254] = partial_ec_mul_output_limb_62_col254;
                let partial_ec_mul_output_limb_63_col255 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(21);
                *row[255] = partial_ec_mul_output_limb_63_col255;
                let partial_ec_mul_output_limb_64_col256 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(22);
                *row[256] = partial_ec_mul_output_limb_64_col256;
                let partial_ec_mul_output_limb_65_col257 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(23);
                *row[257] = partial_ec_mul_output_limb_65_col257;
                let partial_ec_mul_output_limb_66_col258 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(24);
                *row[258] = partial_ec_mul_output_limb_66_col258;
                let partial_ec_mul_output_limb_67_col259 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(25);
                *row[259] = partial_ec_mul_output_limb_67_col259;
                let partial_ec_mul_output_limb_68_col260 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(26);
                *row[260] = partial_ec_mul_output_limb_68_col260;
                let partial_ec_mul_output_limb_69_col261 =
                    partial_ec_mul_output_round_27_tmp_d00c6_49.2 .1[1].get_m31(27);
                *row[261] = partial_ec_mul_output_limb_69_col261;
                *lookup_data.partial_ec_mul_3 = [
                    partial_ec_mul_chain_id_tmp_d00c6_35,
                    M31_28,
                    partial_ec_mul_output_limb_0_col192,
                    partial_ec_mul_output_limb_1_col193,
                    partial_ec_mul_output_limb_2_col194,
                    partial_ec_mul_output_limb_3_col195,
                    partial_ec_mul_output_limb_4_col196,
                    partial_ec_mul_output_limb_5_col197,
                    partial_ec_mul_output_limb_6_col198,
                    partial_ec_mul_output_limb_7_col199,
                    partial_ec_mul_output_limb_8_col200,
                    partial_ec_mul_output_limb_9_col201,
                    partial_ec_mul_output_limb_10_col202,
                    partial_ec_mul_output_limb_11_col203,
                    partial_ec_mul_output_limb_12_col204,
                    partial_ec_mul_output_limb_13_col205,
                    partial_ec_mul_output_limb_14_col206,
                    partial_ec_mul_output_limb_15_col207,
                    partial_ec_mul_output_limb_16_col208,
                    partial_ec_mul_output_limb_17_col209,
                    partial_ec_mul_output_limb_18_col210,
                    partial_ec_mul_output_limb_19_col211,
                    partial_ec_mul_output_limb_20_col212,
                    partial_ec_mul_output_limb_21_col213,
                    partial_ec_mul_output_limb_22_col214,
                    partial_ec_mul_output_limb_23_col215,
                    partial_ec_mul_output_limb_24_col216,
                    partial_ec_mul_output_limb_25_col217,
                    partial_ec_mul_output_limb_26_col218,
                    partial_ec_mul_output_limb_27_col219,
                    partial_ec_mul_output_limb_28_col220,
                    partial_ec_mul_output_limb_29_col221,
                    partial_ec_mul_output_limb_30_col222,
                    partial_ec_mul_output_limb_31_col223,
                    partial_ec_mul_output_limb_32_col224,
                    partial_ec_mul_output_limb_33_col225,
                    partial_ec_mul_output_limb_34_col226,
                    partial_ec_mul_output_limb_35_col227,
                    partial_ec_mul_output_limb_36_col228,
                    partial_ec_mul_output_limb_37_col229,
                    partial_ec_mul_output_limb_38_col230,
                    partial_ec_mul_output_limb_39_col231,
                    partial_ec_mul_output_limb_40_col232,
                    partial_ec_mul_output_limb_41_col233,
                    partial_ec_mul_output_limb_42_col234,
                    partial_ec_mul_output_limb_43_col235,
                    partial_ec_mul_output_limb_44_col236,
                    partial_ec_mul_output_limb_45_col237,
                    partial_ec_mul_output_limb_46_col238,
                    partial_ec_mul_output_limb_47_col239,
                    partial_ec_mul_output_limb_48_col240,
                    partial_ec_mul_output_limb_49_col241,
                    partial_ec_mul_output_limb_50_col242,
                    partial_ec_mul_output_limb_51_col243,
                    partial_ec_mul_output_limb_52_col244,
                    partial_ec_mul_output_limb_53_col245,
                    partial_ec_mul_output_limb_54_col246,
                    partial_ec_mul_output_limb_55_col247,
                    partial_ec_mul_output_limb_56_col248,
                    partial_ec_mul_output_limb_57_col249,
                    partial_ec_mul_output_limb_58_col250,
                    partial_ec_mul_output_limb_59_col251,
                    partial_ec_mul_output_limb_60_col252,
                    partial_ec_mul_output_limb_61_col253,
                    partial_ec_mul_output_limb_62_col254,
                    partial_ec_mul_output_limb_63_col255,
                    partial_ec_mul_output_limb_64_col256,
                    partial_ec_mul_output_limb_65_col257,
                    partial_ec_mul_output_limb_66_col258,
                    partial_ec_mul_output_limb_67_col259,
                    partial_ec_mul_output_limb_68_col260,
                    partial_ec_mul_output_limb_69_col261,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_d00c6_50 = memory_address_to_id_state
                    .deduce_output(((instance_addr_tmp_d00c6_0) + (M31_2)));
                let pedersen_result_id_col262 = memory_address_to_id_value_tmp_d00c6_50;
                *row[262] = pedersen_result_id_col262;
                *sub_component_inputs.memory_address_to_id[2] =
                    ((instance_addr_tmp_d00c6_0) + (M31_2));
                *lookup_data.memory_address_to_id_2 = [
                    ((instance_addr_tmp_d00c6_0) + (M31_2)),
                    pedersen_result_id_col262,
                ];

                *sub_component_inputs.memory_id_to_big[2] = pedersen_result_id_col262;
                *lookup_data.memory_id_to_big_2 = [
                    pedersen_result_id_col262,
                    partial_ec_mul_output_limb_14_col206,
                    partial_ec_mul_output_limb_15_col207,
                    partial_ec_mul_output_limb_16_col208,
                    partial_ec_mul_output_limb_17_col209,
                    partial_ec_mul_output_limb_18_col210,
                    partial_ec_mul_output_limb_19_col211,
                    partial_ec_mul_output_limb_20_col212,
                    partial_ec_mul_output_limb_21_col213,
                    partial_ec_mul_output_limb_22_col214,
                    partial_ec_mul_output_limb_23_col215,
                    partial_ec_mul_output_limb_24_col216,
                    partial_ec_mul_output_limb_25_col217,
                    partial_ec_mul_output_limb_26_col218,
                    partial_ec_mul_output_limb_27_col219,
                    partial_ec_mul_output_limb_28_col220,
                    partial_ec_mul_output_limb_29_col221,
                    partial_ec_mul_output_limb_30_col222,
                    partial_ec_mul_output_limb_31_col223,
                    partial_ec_mul_output_limb_32_col224,
                    partial_ec_mul_output_limb_33_col225,
                    partial_ec_mul_output_limb_34_col226,
                    partial_ec_mul_output_limb_35_col227,
                    partial_ec_mul_output_limb_36_col228,
                    partial_ec_mul_output_limb_37_col229,
                    partial_ec_mul_output_limb_38_col230,
                    partial_ec_mul_output_limb_39_col231,
                    partial_ec_mul_output_limb_40_col232,
                    partial_ec_mul_output_limb_41_col233,
                ];
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
    partial_ec_mul_0: Vec<[PackedM31; 72]>,
    partial_ec_mul_1: Vec<[PackedM31; 72]>,
    partial_ec_mul_2: Vec<[PackedM31; 72]>,
    partial_ec_mul_3: Vec<[PackedM31; 72]>,
    pedersen_points_table_0: Vec<[PackedM31; 57]>,
    range_check_5_4_0: Vec<[PackedM31; 2]>,
    range_check_5_4_1: Vec<[PackedM31; 2]>,
    range_check_8_0: Vec<[PackedM31; 1]>,
    range_check_8_1: Vec<[PackedM31; 1]>,
    range_check_8_2: Vec<[PackedM31; 1]>,
    range_check_8_3: Vec<[PackedM31; 1]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_5_4: &relations::RangeCheck_5_4,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_8: &relations::RangeCheck_8,
        pedersen_points_table: &relations::PedersenPointsTable,
        partial_ec_mul: &relations::PartialEcMul,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_5_4_0,
            &self.lookup_data.memory_address_to_id_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_5_4.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.range_check_5_4_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = range_check_5_4.combine(values1);
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
            &self.lookup_data.range_check_8_0,
            &self.lookup_data.range_check_8_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_8.combine(values0);
                let denom1: PackedQM31 = range_check_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_8_2,
            &self.lookup_data.range_check_8_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_8.combine(values0);
                let denom1: PackedQM31 = range_check_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.pedersen_points_table_0,
            &self.lookup_data.partial_ec_mul_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = pedersen_points_table.combine(values0);
                let denom1: PackedQM31 = partial_ec_mul.combine(values1);
                writer.write_frac(denom1 - denom0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.partial_ec_mul_1,
            &self.lookup_data.partial_ec_mul_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = partial_ec_mul.combine(values0);
                let denom1: PackedQM31 = partial_ec_mul.combine(values1);
                writer.write_frac(denom1 - denom0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.partial_ec_mul_3,
            &self.lookup_data.memory_address_to_id_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = partial_ec_mul.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.memory_id_to_big_2)
            .into_par_iter()
            .for_each(|(writer, values)| {
                let denom = memory_id_to_big.combine(values);
                writer.write_frac(PackedQM31::one(), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

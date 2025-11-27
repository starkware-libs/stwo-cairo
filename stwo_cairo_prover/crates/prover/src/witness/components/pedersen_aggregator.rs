// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::pedersen_aggregator::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_id_to_big, partial_ec_mul, pedersen_points_table, range_check_5_4, range_check_8,
};
use crate::witness::prelude::*;

pub type InputType = ([M31; 2], M31);
pub type PackedInputType = ([PackedM31; 2], PackedM31);

#[derive(Default)]
pub struct ClaimGenerator {
    pub mults: DashMap<InputType, AtomicU32>,
}

impl ClaimGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.mults.is_empty()
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_5_4_state: &range_check_5_4::ClaimGenerator,
        range_check_8_state: &range_check_8::ClaimGenerator,
        pedersen_points_table_state: &pedersen_points_table::ClaimGenerator,
        partial_ec_mul_state: &mut partial_ec_mul::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let mut inputs_mults = self
            .mults
            .iter()
            .map(|entry| (*entry.key(), M31(entry.value().load(Ordering::Relaxed))))
            .collect::<Vec<_>>();

        inputs_mults.sort_by_key(|(input, _)| input.0);

        let (mut inputs, mut mults) = inputs_mults.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        let n_rows = inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();

        inputs.resize(size, *inputs.first().unwrap());
        mults.resize(size, M31::zero());

        let packed_inputs = pack_values(&inputs);
        let packed_mults = pack_values(&mults);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            packed_mults,
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
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults
            .entry(*input)
            .or_insert_with(|| AtomicU32::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    range_check_5_4: [Vec<range_check_5_4::PackedInputType>; 2],
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
    inputs: Vec<PackedInputType>,
    mults: Vec<PackedM31>,
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
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
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
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (row, lookup_data, sub_component_inputs, pedersen_aggregator_input))| {
                let seq = seq.packed_at(row_index);
                let input_limb_0_col0 = pedersen_aggregator_input.0[0];
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = pedersen_aggregator_input.0[1];
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = pedersen_aggregator_input.1;
                *row[2] = input_limb_2_col2;

                // Read Split.

                let memory_id_to_big_value_tmp_c48a1_0 =
                    memory_id_to_big_state.deduce_output(input_limb_0_col0);
                let value_limb_0_col3 = memory_id_to_big_value_tmp_c48a1_0.get_m31(0);
                *row[3] = value_limb_0_col3;
                let value_limb_1_col4 = memory_id_to_big_value_tmp_c48a1_0.get_m31(1);
                *row[4] = value_limb_1_col4;
                let value_limb_2_col5 = memory_id_to_big_value_tmp_c48a1_0.get_m31(2);
                *row[5] = value_limb_2_col5;
                let value_limb_3_col6 = memory_id_to_big_value_tmp_c48a1_0.get_m31(3);
                *row[6] = value_limb_3_col6;
                let value_limb_4_col7 = memory_id_to_big_value_tmp_c48a1_0.get_m31(4);
                *row[7] = value_limb_4_col7;
                let value_limb_5_col8 = memory_id_to_big_value_tmp_c48a1_0.get_m31(5);
                *row[8] = value_limb_5_col8;
                let value_limb_6_col9 = memory_id_to_big_value_tmp_c48a1_0.get_m31(6);
                *row[9] = value_limb_6_col9;
                let value_limb_7_col10 = memory_id_to_big_value_tmp_c48a1_0.get_m31(7);
                *row[10] = value_limb_7_col10;
                let value_limb_8_col11 = memory_id_to_big_value_tmp_c48a1_0.get_m31(8);
                *row[11] = value_limb_8_col11;
                let value_limb_9_col12 = memory_id_to_big_value_tmp_c48a1_0.get_m31(9);
                *row[12] = value_limb_9_col12;
                let value_limb_10_col13 = memory_id_to_big_value_tmp_c48a1_0.get_m31(10);
                *row[13] = value_limb_10_col13;
                let value_limb_11_col14 = memory_id_to_big_value_tmp_c48a1_0.get_m31(11);
                *row[14] = value_limb_11_col14;
                let value_limb_12_col15 = memory_id_to_big_value_tmp_c48a1_0.get_m31(12);
                *row[15] = value_limb_12_col15;
                let value_limb_13_col16 = memory_id_to_big_value_tmp_c48a1_0.get_m31(13);
                *row[16] = value_limb_13_col16;
                let value_limb_14_col17 = memory_id_to_big_value_tmp_c48a1_0.get_m31(14);
                *row[17] = value_limb_14_col17;
                let value_limb_15_col18 = memory_id_to_big_value_tmp_c48a1_0.get_m31(15);
                *row[18] = value_limb_15_col18;
                let value_limb_16_col19 = memory_id_to_big_value_tmp_c48a1_0.get_m31(16);
                *row[19] = value_limb_16_col19;
                let value_limb_17_col20 = memory_id_to_big_value_tmp_c48a1_0.get_m31(17);
                *row[20] = value_limb_17_col20;
                let value_limb_18_col21 = memory_id_to_big_value_tmp_c48a1_0.get_m31(18);
                *row[21] = value_limb_18_col21;
                let value_limb_19_col22 = memory_id_to_big_value_tmp_c48a1_0.get_m31(19);
                *row[22] = value_limb_19_col22;
                let value_limb_20_col23 = memory_id_to_big_value_tmp_c48a1_0.get_m31(20);
                *row[23] = value_limb_20_col23;
                let value_limb_21_col24 = memory_id_to_big_value_tmp_c48a1_0.get_m31(21);
                *row[24] = value_limb_21_col24;
                let value_limb_22_col25 = memory_id_to_big_value_tmp_c48a1_0.get_m31(22);
                *row[25] = value_limb_22_col25;
                let value_limb_23_col26 = memory_id_to_big_value_tmp_c48a1_0.get_m31(23);
                *row[26] = value_limb_23_col26;
                let value_limb_24_col27 = memory_id_to_big_value_tmp_c48a1_0.get_m31(24);
                *row[27] = value_limb_24_col27;
                let value_limb_25_col28 = memory_id_to_big_value_tmp_c48a1_0.get_m31(25);
                *row[28] = value_limb_25_col28;
                let value_limb_26_col29 = memory_id_to_big_value_tmp_c48a1_0.get_m31(26);
                *row[29] = value_limb_26_col29;
                let ms_limb_tmp_c48a1_1 =
                    PackedUInt16::from_m31(memory_id_to_big_value_tmp_c48a1_0.get_m31(27));
                let ms_limb_low_tmp_c48a1_2 = ((ms_limb_tmp_c48a1_1) & (UInt16_31));
                let ms_limb_low_col30 = ms_limb_low_tmp_c48a1_2.as_m31();
                *row[30] = ms_limb_low_col30;
                let ms_limb_high_tmp_c48a1_3 = ((ms_limb_tmp_c48a1_1) >> (UInt16_5));
                let ms_limb_high_col31 = ms_limb_high_tmp_c48a1_3.as_m31();
                *row[31] = ms_limb_high_col31;
                *sub_component_inputs.range_check_5_4[0] = [ms_limb_low_col30, ms_limb_high_col31];
                *lookup_data.range_check_5_4_0 = [ms_limb_low_col30, ms_limb_high_col31];
                *sub_component_inputs.memory_id_to_big[0] = input_limb_0_col0;
                *lookup_data.memory_id_to_big_0 = [
                    input_limb_0_col0,
                    value_limb_0_col3,
                    value_limb_1_col4,
                    value_limb_2_col5,
                    value_limb_3_col6,
                    value_limb_4_col7,
                    value_limb_5_col8,
                    value_limb_6_col9,
                    value_limb_7_col10,
                    value_limb_8_col11,
                    value_limb_9_col12,
                    value_limb_10_col13,
                    value_limb_11_col14,
                    value_limb_12_col15,
                    value_limb_13_col16,
                    value_limb_14_col17,
                    value_limb_15_col18,
                    value_limb_16_col19,
                    value_limb_17_col20,
                    value_limb_18_col21,
                    value_limb_19_col22,
                    value_limb_20_col23,
                    value_limb_21_col24,
                    value_limb_22_col25,
                    value_limb_23_col26,
                    value_limb_24_col27,
                    value_limb_25_col28,
                    value_limb_26_col29,
                    (((ms_limb_high_col31) * (M31_32)) + (ms_limb_low_col30)),
                ];
                let read_split_output_tmp_c48a1_4 = (
                    ms_limb_high_col31,
                    [
                        PackedFelt252::from_limbs([
                            value_limb_0_col3,
                            value_limb_1_col4,
                            value_limb_2_col5,
                            value_limb_3_col6,
                            value_limb_4_col7,
                            value_limb_5_col8,
                            value_limb_6_col9,
                            value_limb_7_col10,
                            value_limb_8_col11,
                            value_limb_9_col12,
                            value_limb_10_col13,
                            value_limb_11_col14,
                            value_limb_12_col15,
                            value_limb_13_col16,
                            value_limb_14_col17,
                            value_limb_15_col18,
                            value_limb_16_col19,
                            value_limb_17_col20,
                            value_limb_18_col21,
                            value_limb_19_col22,
                            value_limb_20_col23,
                            value_limb_21_col24,
                            value_limb_22_col25,
                            value_limb_23_col26,
                            value_limb_24_col27,
                            value_limb_25_col28,
                            value_limb_26_col29,
                            ms_limb_low_col30,
                        ]),
                        PackedFelt252::from_limbs([
                            value_limb_0_col3,
                            value_limb_1_col4,
                            value_limb_2_col5,
                            value_limb_3_col6,
                            value_limb_4_col7,
                            value_limb_5_col8,
                            value_limb_6_col9,
                            value_limb_7_col10,
                            value_limb_8_col11,
                            value_limb_9_col12,
                            value_limb_10_col13,
                            value_limb_11_col14,
                            value_limb_12_col15,
                            value_limb_13_col16,
                            value_limb_14_col17,
                            value_limb_15_col18,
                            value_limb_16_col19,
                            value_limb_17_col20,
                            value_limb_18_col21,
                            value_limb_19_col22,
                            value_limb_20_col23,
                            value_limb_21_col24,
                            value_limb_22_col25,
                            value_limb_23_col26,
                            value_limb_24_col27,
                            value_limb_25_col28,
                            value_limb_26_col29,
                            (((ms_limb_high_col31) * (M31_32)) + (ms_limb_low_col30)),
                        ]),
                    ],
                );

                // Read Split.

                let memory_id_to_big_value_tmp_c48a1_5 =
                    memory_id_to_big_state.deduce_output(input_limb_1_col1);
                let value_limb_0_col32 = memory_id_to_big_value_tmp_c48a1_5.get_m31(0);
                *row[32] = value_limb_0_col32;
                let value_limb_1_col33 = memory_id_to_big_value_tmp_c48a1_5.get_m31(1);
                *row[33] = value_limb_1_col33;
                let value_limb_2_col34 = memory_id_to_big_value_tmp_c48a1_5.get_m31(2);
                *row[34] = value_limb_2_col34;
                let value_limb_3_col35 = memory_id_to_big_value_tmp_c48a1_5.get_m31(3);
                *row[35] = value_limb_3_col35;
                let value_limb_4_col36 = memory_id_to_big_value_tmp_c48a1_5.get_m31(4);
                *row[36] = value_limb_4_col36;
                let value_limb_5_col37 = memory_id_to_big_value_tmp_c48a1_5.get_m31(5);
                *row[37] = value_limb_5_col37;
                let value_limb_6_col38 = memory_id_to_big_value_tmp_c48a1_5.get_m31(6);
                *row[38] = value_limb_6_col38;
                let value_limb_7_col39 = memory_id_to_big_value_tmp_c48a1_5.get_m31(7);
                *row[39] = value_limb_7_col39;
                let value_limb_8_col40 = memory_id_to_big_value_tmp_c48a1_5.get_m31(8);
                *row[40] = value_limb_8_col40;
                let value_limb_9_col41 = memory_id_to_big_value_tmp_c48a1_5.get_m31(9);
                *row[41] = value_limb_9_col41;
                let value_limb_10_col42 = memory_id_to_big_value_tmp_c48a1_5.get_m31(10);
                *row[42] = value_limb_10_col42;
                let value_limb_11_col43 = memory_id_to_big_value_tmp_c48a1_5.get_m31(11);
                *row[43] = value_limb_11_col43;
                let value_limb_12_col44 = memory_id_to_big_value_tmp_c48a1_5.get_m31(12);
                *row[44] = value_limb_12_col44;
                let value_limb_13_col45 = memory_id_to_big_value_tmp_c48a1_5.get_m31(13);
                *row[45] = value_limb_13_col45;
                let value_limb_14_col46 = memory_id_to_big_value_tmp_c48a1_5.get_m31(14);
                *row[46] = value_limb_14_col46;
                let value_limb_15_col47 = memory_id_to_big_value_tmp_c48a1_5.get_m31(15);
                *row[47] = value_limb_15_col47;
                let value_limb_16_col48 = memory_id_to_big_value_tmp_c48a1_5.get_m31(16);
                *row[48] = value_limb_16_col48;
                let value_limb_17_col49 = memory_id_to_big_value_tmp_c48a1_5.get_m31(17);
                *row[49] = value_limb_17_col49;
                let value_limb_18_col50 = memory_id_to_big_value_tmp_c48a1_5.get_m31(18);
                *row[50] = value_limb_18_col50;
                let value_limb_19_col51 = memory_id_to_big_value_tmp_c48a1_5.get_m31(19);
                *row[51] = value_limb_19_col51;
                let value_limb_20_col52 = memory_id_to_big_value_tmp_c48a1_5.get_m31(20);
                *row[52] = value_limb_20_col52;
                let value_limb_21_col53 = memory_id_to_big_value_tmp_c48a1_5.get_m31(21);
                *row[53] = value_limb_21_col53;
                let value_limb_22_col54 = memory_id_to_big_value_tmp_c48a1_5.get_m31(22);
                *row[54] = value_limb_22_col54;
                let value_limb_23_col55 = memory_id_to_big_value_tmp_c48a1_5.get_m31(23);
                *row[55] = value_limb_23_col55;
                let value_limb_24_col56 = memory_id_to_big_value_tmp_c48a1_5.get_m31(24);
                *row[56] = value_limb_24_col56;
                let value_limb_25_col57 = memory_id_to_big_value_tmp_c48a1_5.get_m31(25);
                *row[57] = value_limb_25_col57;
                let value_limb_26_col58 = memory_id_to_big_value_tmp_c48a1_5.get_m31(26);
                *row[58] = value_limb_26_col58;
                let ms_limb_tmp_c48a1_6 =
                    PackedUInt16::from_m31(memory_id_to_big_value_tmp_c48a1_5.get_m31(27));
                let ms_limb_low_tmp_c48a1_7 = ((ms_limb_tmp_c48a1_6) & (UInt16_31));
                let ms_limb_low_col59 = ms_limb_low_tmp_c48a1_7.as_m31();
                *row[59] = ms_limb_low_col59;
                let ms_limb_high_tmp_c48a1_8 = ((ms_limb_tmp_c48a1_6) >> (UInt16_5));
                let ms_limb_high_col60 = ms_limb_high_tmp_c48a1_8.as_m31();
                *row[60] = ms_limb_high_col60;
                *sub_component_inputs.range_check_5_4[1] = [ms_limb_low_col59, ms_limb_high_col60];
                *lookup_data.range_check_5_4_1 = [ms_limb_low_col59, ms_limb_high_col60];
                *sub_component_inputs.memory_id_to_big[1] = input_limb_1_col1;
                *lookup_data.memory_id_to_big_1 = [
                    input_limb_1_col1,
                    value_limb_0_col32,
                    value_limb_1_col33,
                    value_limb_2_col34,
                    value_limb_3_col35,
                    value_limb_4_col36,
                    value_limb_5_col37,
                    value_limb_6_col38,
                    value_limb_7_col39,
                    value_limb_8_col40,
                    value_limb_9_col41,
                    value_limb_10_col42,
                    value_limb_11_col43,
                    value_limb_12_col44,
                    value_limb_13_col45,
                    value_limb_14_col46,
                    value_limb_15_col47,
                    value_limb_16_col48,
                    value_limb_17_col49,
                    value_limb_18_col50,
                    value_limb_19_col51,
                    value_limb_20_col52,
                    value_limb_21_col53,
                    value_limb_22_col54,
                    value_limb_23_col55,
                    value_limb_24_col56,
                    value_limb_25_col57,
                    value_limb_26_col58,
                    (((ms_limb_high_col60) * (M31_32)) + (ms_limb_low_col59)),
                ];
                let read_split_output_tmp_c48a1_9 = (
                    ms_limb_high_col60,
                    [
                        PackedFelt252::from_limbs([
                            value_limb_0_col32,
                            value_limb_1_col33,
                            value_limb_2_col34,
                            value_limb_3_col35,
                            value_limb_4_col36,
                            value_limb_5_col37,
                            value_limb_6_col38,
                            value_limb_7_col39,
                            value_limb_8_col40,
                            value_limb_9_col41,
                            value_limb_10_col42,
                            value_limb_11_col43,
                            value_limb_12_col44,
                            value_limb_13_col45,
                            value_limb_14_col46,
                            value_limb_15_col47,
                            value_limb_16_col48,
                            value_limb_17_col49,
                            value_limb_18_col50,
                            value_limb_19_col51,
                            value_limb_20_col52,
                            value_limb_21_col53,
                            value_limb_22_col54,
                            value_limb_23_col55,
                            value_limb_24_col56,
                            value_limb_25_col57,
                            value_limb_26_col58,
                            ms_limb_low_col59,
                        ]),
                        PackedFelt252::from_limbs([
                            value_limb_0_col32,
                            value_limb_1_col33,
                            value_limb_2_col34,
                            value_limb_3_col35,
                            value_limb_4_col36,
                            value_limb_5_col37,
                            value_limb_6_col38,
                            value_limb_7_col39,
                            value_limb_8_col40,
                            value_limb_9_col41,
                            value_limb_10_col42,
                            value_limb_11_col43,
                            value_limb_12_col44,
                            value_limb_13_col45,
                            value_limb_14_col46,
                            value_limb_15_col47,
                            value_limb_16_col48,
                            value_limb_17_col49,
                            value_limb_18_col50,
                            value_limb_19_col51,
                            value_limb_20_col52,
                            value_limb_21_col53,
                            value_limb_22_col54,
                            value_limb_23_col55,
                            value_limb_24_col56,
                            value_limb_25_col57,
                            value_limb_26_col58,
                            (((ms_limb_high_col60) * (M31_32)) + (ms_limb_low_col59)),
                        ]),
                    ],
                );

                // Verify Reduced 252.

                let ms_limb_is_max_tmp_c48a1_10 =
                    read_split_output_tmp_c48a1_4.1[1].get_m31(27).eq(M31_256);
                let ms_limb_is_max_col61 = ms_limb_is_max_tmp_c48a1_10.as_m31();
                *row[61] = ms_limb_is_max_col61;
                let ms_and_mid_limbs_are_max_tmp_c48a1_11 =
                    ((read_split_output_tmp_c48a1_4.1[1].get_m31(27).eq(M31_256))
                        & (value_limb_21_col24.eq(M31_136)));
                let ms_and_mid_limbs_are_max_col62 = ms_and_mid_limbs_are_max_tmp_c48a1_11.as_m31();
                *row[62] = ms_and_mid_limbs_are_max_col62;
                *sub_component_inputs.range_check_8[0] =
                    [((read_split_output_tmp_c48a1_4.1[1].get_m31(27)) - (ms_limb_is_max_col61))];
                *lookup_data.range_check_8_0 =
                    [((read_split_output_tmp_c48a1_4.1[1].get_m31(27)) - (ms_limb_is_max_col61))];
                let rc_input_col63 = ((ms_limb_is_max_col61)
                    * (((M31_120) + (value_limb_21_col24)) - (ms_and_mid_limbs_are_max_col62)));
                *row[63] = rc_input_col63;
                *sub_component_inputs.range_check_8[1] = [rc_input_col63];
                *lookup_data.range_check_8_1 = [rc_input_col63];

                // Verify Reduced 252.

                let ms_limb_is_max_tmp_c48a1_12 =
                    read_split_output_tmp_c48a1_9.1[1].get_m31(27).eq(M31_256);
                let ms_limb_is_max_col64 = ms_limb_is_max_tmp_c48a1_12.as_m31();
                *row[64] = ms_limb_is_max_col64;
                let ms_and_mid_limbs_are_max_tmp_c48a1_13 =
                    ((read_split_output_tmp_c48a1_9.1[1].get_m31(27).eq(M31_256))
                        & (value_limb_21_col53.eq(M31_136)));
                let ms_and_mid_limbs_are_max_col65 = ms_and_mid_limbs_are_max_tmp_c48a1_13.as_m31();
                *row[65] = ms_and_mid_limbs_are_max_col65;
                *sub_component_inputs.range_check_8[2] =
                    [((read_split_output_tmp_c48a1_9.1[1].get_m31(27)) - (ms_limb_is_max_col64))];
                *lookup_data.range_check_8_2 =
                    [((read_split_output_tmp_c48a1_9.1[1].get_m31(27)) - (ms_limb_is_max_col64))];
                let rc_input_col66 = ((ms_limb_is_max_col64)
                    * (((M31_120) + (value_limb_21_col53)) - (ms_and_mid_limbs_are_max_col65)));
                *row[66] = rc_input_col66;
                *sub_component_inputs.range_check_8[3] = [rc_input_col66];
                *lookup_data.range_check_8_3 = [rc_input_col66];

                *sub_component_inputs.pedersen_points_table[0] = [(((M31_7340032)
                    + ((ms_limb_high_col60) * (M31_16)))
                    + (ms_limb_high_col31))];
                let pedersen_points_table_output_tmp_c48a1_14 =
                    PackedPedersenPointsTable::deduce_output([(((M31_7340032)
                        + ((ms_limb_high_col60) * (M31_16)))
                        + (ms_limb_high_col31))]);
                let pedersen_points_table_output_limb_0_col67 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(0);
                *row[67] = pedersen_points_table_output_limb_0_col67;
                let pedersen_points_table_output_limb_1_col68 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(1);
                *row[68] = pedersen_points_table_output_limb_1_col68;
                let pedersen_points_table_output_limb_2_col69 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(2);
                *row[69] = pedersen_points_table_output_limb_2_col69;
                let pedersen_points_table_output_limb_3_col70 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(3);
                *row[70] = pedersen_points_table_output_limb_3_col70;
                let pedersen_points_table_output_limb_4_col71 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(4);
                *row[71] = pedersen_points_table_output_limb_4_col71;
                let pedersen_points_table_output_limb_5_col72 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(5);
                *row[72] = pedersen_points_table_output_limb_5_col72;
                let pedersen_points_table_output_limb_6_col73 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(6);
                *row[73] = pedersen_points_table_output_limb_6_col73;
                let pedersen_points_table_output_limb_7_col74 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(7);
                *row[74] = pedersen_points_table_output_limb_7_col74;
                let pedersen_points_table_output_limb_8_col75 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(8);
                *row[75] = pedersen_points_table_output_limb_8_col75;
                let pedersen_points_table_output_limb_9_col76 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(9);
                *row[76] = pedersen_points_table_output_limb_9_col76;
                let pedersen_points_table_output_limb_10_col77 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(10);
                *row[77] = pedersen_points_table_output_limb_10_col77;
                let pedersen_points_table_output_limb_11_col78 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(11);
                *row[78] = pedersen_points_table_output_limb_11_col78;
                let pedersen_points_table_output_limb_12_col79 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(12);
                *row[79] = pedersen_points_table_output_limb_12_col79;
                let pedersen_points_table_output_limb_13_col80 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(13);
                *row[80] = pedersen_points_table_output_limb_13_col80;
                let pedersen_points_table_output_limb_14_col81 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(14);
                *row[81] = pedersen_points_table_output_limb_14_col81;
                let pedersen_points_table_output_limb_15_col82 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(15);
                *row[82] = pedersen_points_table_output_limb_15_col82;
                let pedersen_points_table_output_limb_16_col83 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(16);
                *row[83] = pedersen_points_table_output_limb_16_col83;
                let pedersen_points_table_output_limb_17_col84 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(17);
                *row[84] = pedersen_points_table_output_limb_17_col84;
                let pedersen_points_table_output_limb_18_col85 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(18);
                *row[85] = pedersen_points_table_output_limb_18_col85;
                let pedersen_points_table_output_limb_19_col86 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(19);
                *row[86] = pedersen_points_table_output_limb_19_col86;
                let pedersen_points_table_output_limb_20_col87 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(20);
                *row[87] = pedersen_points_table_output_limb_20_col87;
                let pedersen_points_table_output_limb_21_col88 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(21);
                *row[88] = pedersen_points_table_output_limb_21_col88;
                let pedersen_points_table_output_limb_22_col89 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(22);
                *row[89] = pedersen_points_table_output_limb_22_col89;
                let pedersen_points_table_output_limb_23_col90 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(23);
                *row[90] = pedersen_points_table_output_limb_23_col90;
                let pedersen_points_table_output_limb_24_col91 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(24);
                *row[91] = pedersen_points_table_output_limb_24_col91;
                let pedersen_points_table_output_limb_25_col92 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(25);
                *row[92] = pedersen_points_table_output_limb_25_col92;
                let pedersen_points_table_output_limb_26_col93 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(26);
                *row[93] = pedersen_points_table_output_limb_26_col93;
                let pedersen_points_table_output_limb_27_col94 =
                    pedersen_points_table_output_tmp_c48a1_14[0].get_m31(27);
                *row[94] = pedersen_points_table_output_limb_27_col94;
                let pedersen_points_table_output_limb_28_col95 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(0);
                *row[95] = pedersen_points_table_output_limb_28_col95;
                let pedersen_points_table_output_limb_29_col96 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(1);
                *row[96] = pedersen_points_table_output_limb_29_col96;
                let pedersen_points_table_output_limb_30_col97 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(2);
                *row[97] = pedersen_points_table_output_limb_30_col97;
                let pedersen_points_table_output_limb_31_col98 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(3);
                *row[98] = pedersen_points_table_output_limb_31_col98;
                let pedersen_points_table_output_limb_32_col99 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(4);
                *row[99] = pedersen_points_table_output_limb_32_col99;
                let pedersen_points_table_output_limb_33_col100 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(5);
                *row[100] = pedersen_points_table_output_limb_33_col100;
                let pedersen_points_table_output_limb_34_col101 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(6);
                *row[101] = pedersen_points_table_output_limb_34_col101;
                let pedersen_points_table_output_limb_35_col102 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(7);
                *row[102] = pedersen_points_table_output_limb_35_col102;
                let pedersen_points_table_output_limb_36_col103 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(8);
                *row[103] = pedersen_points_table_output_limb_36_col103;
                let pedersen_points_table_output_limb_37_col104 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(9);
                *row[104] = pedersen_points_table_output_limb_37_col104;
                let pedersen_points_table_output_limb_38_col105 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(10);
                *row[105] = pedersen_points_table_output_limb_38_col105;
                let pedersen_points_table_output_limb_39_col106 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(11);
                *row[106] = pedersen_points_table_output_limb_39_col106;
                let pedersen_points_table_output_limb_40_col107 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(12);
                *row[107] = pedersen_points_table_output_limb_40_col107;
                let pedersen_points_table_output_limb_41_col108 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(13);
                *row[108] = pedersen_points_table_output_limb_41_col108;
                let pedersen_points_table_output_limb_42_col109 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(14);
                *row[109] = pedersen_points_table_output_limb_42_col109;
                let pedersen_points_table_output_limb_43_col110 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(15);
                *row[110] = pedersen_points_table_output_limb_43_col110;
                let pedersen_points_table_output_limb_44_col111 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(16);
                *row[111] = pedersen_points_table_output_limb_44_col111;
                let pedersen_points_table_output_limb_45_col112 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(17);
                *row[112] = pedersen_points_table_output_limb_45_col112;
                let pedersen_points_table_output_limb_46_col113 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(18);
                *row[113] = pedersen_points_table_output_limb_46_col113;
                let pedersen_points_table_output_limb_47_col114 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(19);
                *row[114] = pedersen_points_table_output_limb_47_col114;
                let pedersen_points_table_output_limb_48_col115 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(20);
                *row[115] = pedersen_points_table_output_limb_48_col115;
                let pedersen_points_table_output_limb_49_col116 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(21);
                *row[116] = pedersen_points_table_output_limb_49_col116;
                let pedersen_points_table_output_limb_50_col117 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(22);
                *row[117] = pedersen_points_table_output_limb_50_col117;
                let pedersen_points_table_output_limb_51_col118 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(23);
                *row[118] = pedersen_points_table_output_limb_51_col118;
                let pedersen_points_table_output_limb_52_col119 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(24);
                *row[119] = pedersen_points_table_output_limb_52_col119;
                let pedersen_points_table_output_limb_53_col120 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(25);
                *row[120] = pedersen_points_table_output_limb_53_col120;
                let pedersen_points_table_output_limb_54_col121 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(26);
                *row[121] = pedersen_points_table_output_limb_54_col121;
                let pedersen_points_table_output_limb_55_col122 =
                    pedersen_points_table_output_tmp_c48a1_14[1].get_m31(27);
                *row[122] = pedersen_points_table_output_limb_55_col122;
                *lookup_data.pedersen_points_table_0 = [
                    (((M31_7340032) + ((ms_limb_high_col60) * (M31_16))) + (ms_limb_high_col31)),
                    pedersen_points_table_output_limb_0_col67,
                    pedersen_points_table_output_limb_1_col68,
                    pedersen_points_table_output_limb_2_col69,
                    pedersen_points_table_output_limb_3_col70,
                    pedersen_points_table_output_limb_4_col71,
                    pedersen_points_table_output_limb_5_col72,
                    pedersen_points_table_output_limb_6_col73,
                    pedersen_points_table_output_limb_7_col74,
                    pedersen_points_table_output_limb_8_col75,
                    pedersen_points_table_output_limb_9_col76,
                    pedersen_points_table_output_limb_10_col77,
                    pedersen_points_table_output_limb_11_col78,
                    pedersen_points_table_output_limb_12_col79,
                    pedersen_points_table_output_limb_13_col80,
                    pedersen_points_table_output_limb_14_col81,
                    pedersen_points_table_output_limb_15_col82,
                    pedersen_points_table_output_limb_16_col83,
                    pedersen_points_table_output_limb_17_col84,
                    pedersen_points_table_output_limb_18_col85,
                    pedersen_points_table_output_limb_19_col86,
                    pedersen_points_table_output_limb_20_col87,
                    pedersen_points_table_output_limb_21_col88,
                    pedersen_points_table_output_limb_22_col89,
                    pedersen_points_table_output_limb_23_col90,
                    pedersen_points_table_output_limb_24_col91,
                    pedersen_points_table_output_limb_25_col92,
                    pedersen_points_table_output_limb_26_col93,
                    pedersen_points_table_output_limb_27_col94,
                    pedersen_points_table_output_limb_28_col95,
                    pedersen_points_table_output_limb_29_col96,
                    pedersen_points_table_output_limb_30_col97,
                    pedersen_points_table_output_limb_31_col98,
                    pedersen_points_table_output_limb_32_col99,
                    pedersen_points_table_output_limb_33_col100,
                    pedersen_points_table_output_limb_34_col101,
                    pedersen_points_table_output_limb_35_col102,
                    pedersen_points_table_output_limb_36_col103,
                    pedersen_points_table_output_limb_37_col104,
                    pedersen_points_table_output_limb_38_col105,
                    pedersen_points_table_output_limb_39_col106,
                    pedersen_points_table_output_limb_40_col107,
                    pedersen_points_table_output_limb_41_col108,
                    pedersen_points_table_output_limb_42_col109,
                    pedersen_points_table_output_limb_43_col110,
                    pedersen_points_table_output_limb_44_col111,
                    pedersen_points_table_output_limb_45_col112,
                    pedersen_points_table_output_limb_46_col113,
                    pedersen_points_table_output_limb_47_col114,
                    pedersen_points_table_output_limb_48_col115,
                    pedersen_points_table_output_limb_49_col116,
                    pedersen_points_table_output_limb_50_col117,
                    pedersen_points_table_output_limb_51_col118,
                    pedersen_points_table_output_limb_52_col119,
                    pedersen_points_table_output_limb_53_col120,
                    pedersen_points_table_output_limb_54_col121,
                    pedersen_points_table_output_limb_55_col122,
                ];
                let partial_ec_mul_chain_tmp_tmp_c48a1_15 = ((seq) * (M31_2));
                *lookup_data.partial_ec_mul_0 = [
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_0,
                    ((value_limb_0_col3) + ((value_limb_1_col4) * (M31_512))),
                    ((value_limb_2_col5) + ((value_limb_3_col6) * (M31_512))),
                    ((value_limb_4_col7) + ((value_limb_5_col8) * (M31_512))),
                    ((value_limb_6_col9) + ((value_limb_7_col10) * (M31_512))),
                    ((value_limb_8_col11) + ((value_limb_9_col12) * (M31_512))),
                    ((value_limb_10_col13) + ((value_limb_11_col14) * (M31_512))),
                    ((value_limb_12_col15) + ((value_limb_13_col16) * (M31_512))),
                    ((value_limb_14_col17) + ((value_limb_15_col18) * (M31_512))),
                    ((value_limb_16_col19) + ((value_limb_17_col20) * (M31_512))),
                    ((value_limb_18_col21) + ((value_limb_19_col22) * (M31_512))),
                    ((value_limb_20_col23) + ((value_limb_21_col24) * (M31_512))),
                    ((value_limb_22_col25) + ((value_limb_23_col26) * (M31_512))),
                    ((value_limb_24_col27) + ((value_limb_25_col28) * (M31_512))),
                    ((value_limb_26_col29) + ((ms_limb_low_col30) * (M31_512))),
                    pedersen_points_table_output_limb_0_col67,
                    pedersen_points_table_output_limb_1_col68,
                    pedersen_points_table_output_limb_2_col69,
                    pedersen_points_table_output_limb_3_col70,
                    pedersen_points_table_output_limb_4_col71,
                    pedersen_points_table_output_limb_5_col72,
                    pedersen_points_table_output_limb_6_col73,
                    pedersen_points_table_output_limb_7_col74,
                    pedersen_points_table_output_limb_8_col75,
                    pedersen_points_table_output_limb_9_col76,
                    pedersen_points_table_output_limb_10_col77,
                    pedersen_points_table_output_limb_11_col78,
                    pedersen_points_table_output_limb_12_col79,
                    pedersen_points_table_output_limb_13_col80,
                    pedersen_points_table_output_limb_14_col81,
                    pedersen_points_table_output_limb_15_col82,
                    pedersen_points_table_output_limb_16_col83,
                    pedersen_points_table_output_limb_17_col84,
                    pedersen_points_table_output_limb_18_col85,
                    pedersen_points_table_output_limb_19_col86,
                    pedersen_points_table_output_limb_20_col87,
                    pedersen_points_table_output_limb_21_col88,
                    pedersen_points_table_output_limb_22_col89,
                    pedersen_points_table_output_limb_23_col90,
                    pedersen_points_table_output_limb_24_col91,
                    pedersen_points_table_output_limb_25_col92,
                    pedersen_points_table_output_limb_26_col93,
                    pedersen_points_table_output_limb_27_col94,
                    pedersen_points_table_output_limb_28_col95,
                    pedersen_points_table_output_limb_29_col96,
                    pedersen_points_table_output_limb_30_col97,
                    pedersen_points_table_output_limb_31_col98,
                    pedersen_points_table_output_limb_32_col99,
                    pedersen_points_table_output_limb_33_col100,
                    pedersen_points_table_output_limb_34_col101,
                    pedersen_points_table_output_limb_35_col102,
                    pedersen_points_table_output_limb_36_col103,
                    pedersen_points_table_output_limb_37_col104,
                    pedersen_points_table_output_limb_38_col105,
                    pedersen_points_table_output_limb_39_col106,
                    pedersen_points_table_output_limb_40_col107,
                    pedersen_points_table_output_limb_41_col108,
                    pedersen_points_table_output_limb_42_col109,
                    pedersen_points_table_output_limb_43_col110,
                    pedersen_points_table_output_limb_44_col111,
                    pedersen_points_table_output_limb_45_col112,
                    pedersen_points_table_output_limb_46_col113,
                    pedersen_points_table_output_limb_47_col114,
                    pedersen_points_table_output_limb_48_col115,
                    pedersen_points_table_output_limb_49_col116,
                    pedersen_points_table_output_limb_50_col117,
                    pedersen_points_table_output_limb_51_col118,
                    pedersen_points_table_output_limb_52_col119,
                    pedersen_points_table_output_limb_53_col120,
                    pedersen_points_table_output_limb_54_col121,
                    pedersen_points_table_output_limb_55_col122,
                ];
                *sub_component_inputs.partial_ec_mul[0] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_0,
                    (
                        [
                            ((value_limb_0_col3) + ((value_limb_1_col4) * (M31_512))),
                            ((value_limb_2_col5) + ((value_limb_3_col6) * (M31_512))),
                            ((value_limb_4_col7) + ((value_limb_5_col8) * (M31_512))),
                            ((value_limb_6_col9) + ((value_limb_7_col10) * (M31_512))),
                            ((value_limb_8_col11) + ((value_limb_9_col12) * (M31_512))),
                            ((value_limb_10_col13) + ((value_limb_11_col14) * (M31_512))),
                            ((value_limb_12_col15) + ((value_limb_13_col16) * (M31_512))),
                            ((value_limb_14_col17) + ((value_limb_15_col18) * (M31_512))),
                            ((value_limb_16_col19) + ((value_limb_17_col20) * (M31_512))),
                            ((value_limb_18_col21) + ((value_limb_19_col22) * (M31_512))),
                            ((value_limb_20_col23) + ((value_limb_21_col24) * (M31_512))),
                            ((value_limb_22_col25) + ((value_limb_23_col26) * (M31_512))),
                            ((value_limb_24_col27) + ((value_limb_25_col28) * (M31_512))),
                            ((value_limb_26_col29) + ((ms_limb_low_col30) * (M31_512))),
                        ],
                        [
                            pedersen_points_table_output_tmp_c48a1_14[0],
                            pedersen_points_table_output_tmp_c48a1_14[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_0_tmp_c48a1_16 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_0,
                        (
                            [
                                ((value_limb_0_col3) + ((value_limb_1_col4) * (M31_512))),
                                ((value_limb_2_col5) + ((value_limb_3_col6) * (M31_512))),
                                ((value_limb_4_col7) + ((value_limb_5_col8) * (M31_512))),
                                ((value_limb_6_col9) + ((value_limb_7_col10) * (M31_512))),
                                ((value_limb_8_col11) + ((value_limb_9_col12) * (M31_512))),
                                ((value_limb_10_col13) + ((value_limb_11_col14) * (M31_512))),
                                ((value_limb_12_col15) + ((value_limb_13_col16) * (M31_512))),
                                ((value_limb_14_col17) + ((value_limb_15_col18) * (M31_512))),
                                ((value_limb_16_col19) + ((value_limb_17_col20) * (M31_512))),
                                ((value_limb_18_col21) + ((value_limb_19_col22) * (M31_512))),
                                ((value_limb_20_col23) + ((value_limb_21_col24) * (M31_512))),
                                ((value_limb_22_col25) + ((value_limb_23_col26) * (M31_512))),
                                ((value_limb_24_col27) + ((value_limb_25_col28) * (M31_512))),
                                ((value_limb_26_col29) + ((ms_limb_low_col30) * (M31_512))),
                            ],
                            [
                                pedersen_points_table_output_tmp_c48a1_14[0],
                                pedersen_points_table_output_tmp_c48a1_14[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[1] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_1,
                    (
                        [
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[0],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[1],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[2],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[3],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[4],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[5],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[6],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[7],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[8],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[9],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[10],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[11],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[12],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .1[0],
                            partial_ec_mul_output_round_0_tmp_c48a1_16.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_1_tmp_c48a1_17 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_1,
                        (
                            [
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[0],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[1],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[2],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[3],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[4],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[5],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[6],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[7],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[8],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[9],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[10],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[11],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[12],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .1[0],
                                partial_ec_mul_output_round_0_tmp_c48a1_16.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[2] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_2,
                    (
                        [
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[0],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[1],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[2],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[3],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[4],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[5],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[6],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[7],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[8],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[9],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[10],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[11],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[12],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .1[0],
                            partial_ec_mul_output_round_1_tmp_c48a1_17.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_2_tmp_c48a1_18 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_2,
                        (
                            [
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[0],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[1],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[2],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[3],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[4],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[5],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[6],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[7],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[8],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[9],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[10],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[11],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[12],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .1[0],
                                partial_ec_mul_output_round_1_tmp_c48a1_17.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[3] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_3,
                    (
                        [
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[0],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[1],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[2],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[3],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[4],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[5],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[6],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[7],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[8],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[9],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[10],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[11],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[12],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .1[0],
                            partial_ec_mul_output_round_2_tmp_c48a1_18.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_3_tmp_c48a1_19 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_3,
                        (
                            [
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[0],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[1],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[2],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[3],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[4],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[5],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[6],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[7],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[8],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[9],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[10],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[11],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[12],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .1[0],
                                partial_ec_mul_output_round_2_tmp_c48a1_18.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[4] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_4,
                    (
                        [
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[0],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[1],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[2],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[3],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[4],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[5],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[6],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[7],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[8],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[9],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[10],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[11],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[12],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .1[0],
                            partial_ec_mul_output_round_3_tmp_c48a1_19.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_4_tmp_c48a1_20 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_4,
                        (
                            [
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[0],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[1],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[2],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[3],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[4],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[5],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[6],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[7],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[8],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[9],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[10],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[11],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[12],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .1[0],
                                partial_ec_mul_output_round_3_tmp_c48a1_19.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[5] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_5,
                    (
                        [
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[0],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[1],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[2],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[3],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[4],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[5],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[6],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[7],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[8],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[9],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[10],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[11],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[12],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .1[0],
                            partial_ec_mul_output_round_4_tmp_c48a1_20.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_5_tmp_c48a1_21 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_5,
                        (
                            [
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[0],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[1],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[2],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[3],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[4],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[5],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[6],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[7],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[8],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[9],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[10],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[11],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[12],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .1[0],
                                partial_ec_mul_output_round_4_tmp_c48a1_20.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[6] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_6,
                    (
                        [
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[0],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[1],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[2],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[3],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[4],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[5],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[6],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[7],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[8],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[9],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[10],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[11],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[12],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .1[0],
                            partial_ec_mul_output_round_5_tmp_c48a1_21.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_6_tmp_c48a1_22 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_6,
                        (
                            [
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[0],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[1],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[2],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[3],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[4],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[5],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[6],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[7],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[8],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[9],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[10],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[11],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[12],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .1[0],
                                partial_ec_mul_output_round_5_tmp_c48a1_21.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[7] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_7,
                    (
                        [
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[0],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[1],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[2],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[3],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[4],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[5],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[6],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[7],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[8],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[9],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[10],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[11],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[12],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .1[0],
                            partial_ec_mul_output_round_6_tmp_c48a1_22.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_7_tmp_c48a1_23 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_7,
                        (
                            [
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[0],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[1],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[2],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[3],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[4],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[5],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[6],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[7],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[8],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[9],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[10],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[11],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[12],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .1[0],
                                partial_ec_mul_output_round_6_tmp_c48a1_22.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[8] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_8,
                    (
                        [
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[0],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[1],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[2],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[3],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[4],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[5],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[6],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[7],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[8],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[9],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[10],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[11],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[12],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .1[0],
                            partial_ec_mul_output_round_7_tmp_c48a1_23.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_8_tmp_c48a1_24 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_8,
                        (
                            [
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[0],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[1],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[2],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[3],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[4],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[5],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[6],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[7],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[8],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[9],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[10],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[11],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[12],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .1[0],
                                partial_ec_mul_output_round_7_tmp_c48a1_23.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[9] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_9,
                    (
                        [
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[0],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[1],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[2],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[3],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[4],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[5],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[6],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[7],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[8],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[9],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[10],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[11],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[12],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .1[0],
                            partial_ec_mul_output_round_8_tmp_c48a1_24.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_9_tmp_c48a1_25 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_9,
                        (
                            [
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[0],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[1],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[2],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[3],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[4],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[5],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[6],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[7],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[8],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[9],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[10],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[11],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[12],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .1[0],
                                partial_ec_mul_output_round_8_tmp_c48a1_24.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[10] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_10,
                    (
                        [
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[0],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[1],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[2],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[3],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[4],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[5],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[6],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[7],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[8],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[9],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[10],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[11],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[12],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .1[0],
                            partial_ec_mul_output_round_9_tmp_c48a1_25.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_10_tmp_c48a1_26 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_10,
                        (
                            [
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[0],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[1],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[2],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[3],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[4],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[5],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[6],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[7],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[8],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[9],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[10],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[11],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[12],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .1[0],
                                partial_ec_mul_output_round_9_tmp_c48a1_25.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[11] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_11,
                    (
                        [
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[0],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[1],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[2],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[3],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[4],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[5],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[6],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[7],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[8],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[9],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[10],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[11],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[12],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .1[0],
                            partial_ec_mul_output_round_10_tmp_c48a1_26.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_11_tmp_c48a1_27 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_11,
                        (
                            [
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[0],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[1],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[2],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[3],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[4],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[5],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[6],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[7],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[8],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[9],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[10],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[11],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[12],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .1[0],
                                partial_ec_mul_output_round_10_tmp_c48a1_26.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[12] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_12,
                    (
                        [
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[0],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[1],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[2],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[3],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[4],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[5],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[6],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[7],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[8],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[9],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[10],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[11],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[12],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .1[0],
                            partial_ec_mul_output_round_11_tmp_c48a1_27.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_12_tmp_c48a1_28 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_12,
                        (
                            [
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[0],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[1],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[2],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[3],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[4],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[5],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[6],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[7],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[8],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[9],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[10],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[11],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[12],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .1[0],
                                partial_ec_mul_output_round_11_tmp_c48a1_27.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[13] = (
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_13,
                    (
                        [
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[0],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[1],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[2],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[3],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[4],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[5],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[6],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[7],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[8],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[9],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[10],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[11],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[12],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .1[0],
                            partial_ec_mul_output_round_12_tmp_c48a1_28.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_13_tmp_c48a1_29 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_tmp_tmp_c48a1_15,
                        M31_13,
                        (
                            [
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[0],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[1],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[2],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[3],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[4],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[5],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[6],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[7],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[8],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[9],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[10],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[11],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[12],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .1[0],
                                partial_ec_mul_output_round_12_tmp_c48a1_28.2 .1[1],
                            ],
                        ),
                    ));
                let partial_ec_mul_output_limb_0_col123 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[0];
                *row[123] = partial_ec_mul_output_limb_0_col123;
                let partial_ec_mul_output_limb_1_col124 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[1];
                *row[124] = partial_ec_mul_output_limb_1_col124;
                let partial_ec_mul_output_limb_2_col125 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[2];
                *row[125] = partial_ec_mul_output_limb_2_col125;
                let partial_ec_mul_output_limb_3_col126 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[3];
                *row[126] = partial_ec_mul_output_limb_3_col126;
                let partial_ec_mul_output_limb_4_col127 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[4];
                *row[127] = partial_ec_mul_output_limb_4_col127;
                let partial_ec_mul_output_limb_5_col128 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[5];
                *row[128] = partial_ec_mul_output_limb_5_col128;
                let partial_ec_mul_output_limb_6_col129 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[6];
                *row[129] = partial_ec_mul_output_limb_6_col129;
                let partial_ec_mul_output_limb_7_col130 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[7];
                *row[130] = partial_ec_mul_output_limb_7_col130;
                let partial_ec_mul_output_limb_8_col131 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[8];
                *row[131] = partial_ec_mul_output_limb_8_col131;
                let partial_ec_mul_output_limb_9_col132 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[9];
                *row[132] = partial_ec_mul_output_limb_9_col132;
                let partial_ec_mul_output_limb_10_col133 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[10];
                *row[133] = partial_ec_mul_output_limb_10_col133;
                let partial_ec_mul_output_limb_11_col134 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[11];
                *row[134] = partial_ec_mul_output_limb_11_col134;
                let partial_ec_mul_output_limb_12_col135 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[12];
                *row[135] = partial_ec_mul_output_limb_12_col135;
                let partial_ec_mul_output_limb_13_col136 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .0[13];
                *row[136] = partial_ec_mul_output_limb_13_col136;
                let partial_ec_mul_output_limb_14_col137 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(0);
                *row[137] = partial_ec_mul_output_limb_14_col137;
                let partial_ec_mul_output_limb_15_col138 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(1);
                *row[138] = partial_ec_mul_output_limb_15_col138;
                let partial_ec_mul_output_limb_16_col139 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(2);
                *row[139] = partial_ec_mul_output_limb_16_col139;
                let partial_ec_mul_output_limb_17_col140 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(3);
                *row[140] = partial_ec_mul_output_limb_17_col140;
                let partial_ec_mul_output_limb_18_col141 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(4);
                *row[141] = partial_ec_mul_output_limb_18_col141;
                let partial_ec_mul_output_limb_19_col142 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(5);
                *row[142] = partial_ec_mul_output_limb_19_col142;
                let partial_ec_mul_output_limb_20_col143 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(6);
                *row[143] = partial_ec_mul_output_limb_20_col143;
                let partial_ec_mul_output_limb_21_col144 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(7);
                *row[144] = partial_ec_mul_output_limb_21_col144;
                let partial_ec_mul_output_limb_22_col145 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(8);
                *row[145] = partial_ec_mul_output_limb_22_col145;
                let partial_ec_mul_output_limb_23_col146 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(9);
                *row[146] = partial_ec_mul_output_limb_23_col146;
                let partial_ec_mul_output_limb_24_col147 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(10);
                *row[147] = partial_ec_mul_output_limb_24_col147;
                let partial_ec_mul_output_limb_25_col148 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(11);
                *row[148] = partial_ec_mul_output_limb_25_col148;
                let partial_ec_mul_output_limb_26_col149 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(12);
                *row[149] = partial_ec_mul_output_limb_26_col149;
                let partial_ec_mul_output_limb_27_col150 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(13);
                *row[150] = partial_ec_mul_output_limb_27_col150;
                let partial_ec_mul_output_limb_28_col151 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(14);
                *row[151] = partial_ec_mul_output_limb_28_col151;
                let partial_ec_mul_output_limb_29_col152 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(15);
                *row[152] = partial_ec_mul_output_limb_29_col152;
                let partial_ec_mul_output_limb_30_col153 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(16);
                *row[153] = partial_ec_mul_output_limb_30_col153;
                let partial_ec_mul_output_limb_31_col154 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(17);
                *row[154] = partial_ec_mul_output_limb_31_col154;
                let partial_ec_mul_output_limb_32_col155 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(18);
                *row[155] = partial_ec_mul_output_limb_32_col155;
                let partial_ec_mul_output_limb_33_col156 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(19);
                *row[156] = partial_ec_mul_output_limb_33_col156;
                let partial_ec_mul_output_limb_34_col157 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(20);
                *row[157] = partial_ec_mul_output_limb_34_col157;
                let partial_ec_mul_output_limb_35_col158 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(21);
                *row[158] = partial_ec_mul_output_limb_35_col158;
                let partial_ec_mul_output_limb_36_col159 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(22);
                *row[159] = partial_ec_mul_output_limb_36_col159;
                let partial_ec_mul_output_limb_37_col160 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(23);
                *row[160] = partial_ec_mul_output_limb_37_col160;
                let partial_ec_mul_output_limb_38_col161 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(24);
                *row[161] = partial_ec_mul_output_limb_38_col161;
                let partial_ec_mul_output_limb_39_col162 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(25);
                *row[162] = partial_ec_mul_output_limb_39_col162;
                let partial_ec_mul_output_limb_40_col163 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(26);
                *row[163] = partial_ec_mul_output_limb_40_col163;
                let partial_ec_mul_output_limb_41_col164 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0].get_m31(27);
                *row[164] = partial_ec_mul_output_limb_41_col164;
                let partial_ec_mul_output_limb_42_col165 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(0);
                *row[165] = partial_ec_mul_output_limb_42_col165;
                let partial_ec_mul_output_limb_43_col166 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(1);
                *row[166] = partial_ec_mul_output_limb_43_col166;
                let partial_ec_mul_output_limb_44_col167 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(2);
                *row[167] = partial_ec_mul_output_limb_44_col167;
                let partial_ec_mul_output_limb_45_col168 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(3);
                *row[168] = partial_ec_mul_output_limb_45_col168;
                let partial_ec_mul_output_limb_46_col169 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(4);
                *row[169] = partial_ec_mul_output_limb_46_col169;
                let partial_ec_mul_output_limb_47_col170 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(5);
                *row[170] = partial_ec_mul_output_limb_47_col170;
                let partial_ec_mul_output_limb_48_col171 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(6);
                *row[171] = partial_ec_mul_output_limb_48_col171;
                let partial_ec_mul_output_limb_49_col172 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(7);
                *row[172] = partial_ec_mul_output_limb_49_col172;
                let partial_ec_mul_output_limb_50_col173 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(8);
                *row[173] = partial_ec_mul_output_limb_50_col173;
                let partial_ec_mul_output_limb_51_col174 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(9);
                *row[174] = partial_ec_mul_output_limb_51_col174;
                let partial_ec_mul_output_limb_52_col175 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(10);
                *row[175] = partial_ec_mul_output_limb_52_col175;
                let partial_ec_mul_output_limb_53_col176 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(11);
                *row[176] = partial_ec_mul_output_limb_53_col176;
                let partial_ec_mul_output_limb_54_col177 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(12);
                *row[177] = partial_ec_mul_output_limb_54_col177;
                let partial_ec_mul_output_limb_55_col178 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(13);
                *row[178] = partial_ec_mul_output_limb_55_col178;
                let partial_ec_mul_output_limb_56_col179 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(14);
                *row[179] = partial_ec_mul_output_limb_56_col179;
                let partial_ec_mul_output_limb_57_col180 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(15);
                *row[180] = partial_ec_mul_output_limb_57_col180;
                let partial_ec_mul_output_limb_58_col181 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(16);
                *row[181] = partial_ec_mul_output_limb_58_col181;
                let partial_ec_mul_output_limb_59_col182 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(17);
                *row[182] = partial_ec_mul_output_limb_59_col182;
                let partial_ec_mul_output_limb_60_col183 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(18);
                *row[183] = partial_ec_mul_output_limb_60_col183;
                let partial_ec_mul_output_limb_61_col184 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(19);
                *row[184] = partial_ec_mul_output_limb_61_col184;
                let partial_ec_mul_output_limb_62_col185 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(20);
                *row[185] = partial_ec_mul_output_limb_62_col185;
                let partial_ec_mul_output_limb_63_col186 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(21);
                *row[186] = partial_ec_mul_output_limb_63_col186;
                let partial_ec_mul_output_limb_64_col187 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(22);
                *row[187] = partial_ec_mul_output_limb_64_col187;
                let partial_ec_mul_output_limb_65_col188 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(23);
                *row[188] = partial_ec_mul_output_limb_65_col188;
                let partial_ec_mul_output_limb_66_col189 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(24);
                *row[189] = partial_ec_mul_output_limb_66_col189;
                let partial_ec_mul_output_limb_67_col190 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(25);
                *row[190] = partial_ec_mul_output_limb_67_col190;
                let partial_ec_mul_output_limb_68_col191 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(26);
                *row[191] = partial_ec_mul_output_limb_68_col191;
                let partial_ec_mul_output_limb_69_col192 =
                    partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1].get_m31(27);
                *row[192] = partial_ec_mul_output_limb_69_col192;
                *lookup_data.partial_ec_mul_1 = [
                    partial_ec_mul_chain_tmp_tmp_c48a1_15,
                    M31_14,
                    partial_ec_mul_output_limb_0_col123,
                    partial_ec_mul_output_limb_1_col124,
                    partial_ec_mul_output_limb_2_col125,
                    partial_ec_mul_output_limb_3_col126,
                    partial_ec_mul_output_limb_4_col127,
                    partial_ec_mul_output_limb_5_col128,
                    partial_ec_mul_output_limb_6_col129,
                    partial_ec_mul_output_limb_7_col130,
                    partial_ec_mul_output_limb_8_col131,
                    partial_ec_mul_output_limb_9_col132,
                    partial_ec_mul_output_limb_10_col133,
                    partial_ec_mul_output_limb_11_col134,
                    partial_ec_mul_output_limb_12_col135,
                    partial_ec_mul_output_limb_13_col136,
                    partial_ec_mul_output_limb_14_col137,
                    partial_ec_mul_output_limb_15_col138,
                    partial_ec_mul_output_limb_16_col139,
                    partial_ec_mul_output_limb_17_col140,
                    partial_ec_mul_output_limb_18_col141,
                    partial_ec_mul_output_limb_19_col142,
                    partial_ec_mul_output_limb_20_col143,
                    partial_ec_mul_output_limb_21_col144,
                    partial_ec_mul_output_limb_22_col145,
                    partial_ec_mul_output_limb_23_col146,
                    partial_ec_mul_output_limb_24_col147,
                    partial_ec_mul_output_limb_25_col148,
                    partial_ec_mul_output_limb_26_col149,
                    partial_ec_mul_output_limb_27_col150,
                    partial_ec_mul_output_limb_28_col151,
                    partial_ec_mul_output_limb_29_col152,
                    partial_ec_mul_output_limb_30_col153,
                    partial_ec_mul_output_limb_31_col154,
                    partial_ec_mul_output_limb_32_col155,
                    partial_ec_mul_output_limb_33_col156,
                    partial_ec_mul_output_limb_34_col157,
                    partial_ec_mul_output_limb_35_col158,
                    partial_ec_mul_output_limb_36_col159,
                    partial_ec_mul_output_limb_37_col160,
                    partial_ec_mul_output_limb_38_col161,
                    partial_ec_mul_output_limb_39_col162,
                    partial_ec_mul_output_limb_40_col163,
                    partial_ec_mul_output_limb_41_col164,
                    partial_ec_mul_output_limb_42_col165,
                    partial_ec_mul_output_limb_43_col166,
                    partial_ec_mul_output_limb_44_col167,
                    partial_ec_mul_output_limb_45_col168,
                    partial_ec_mul_output_limb_46_col169,
                    partial_ec_mul_output_limb_47_col170,
                    partial_ec_mul_output_limb_48_col171,
                    partial_ec_mul_output_limb_49_col172,
                    partial_ec_mul_output_limb_50_col173,
                    partial_ec_mul_output_limb_51_col174,
                    partial_ec_mul_output_limb_52_col175,
                    partial_ec_mul_output_limb_53_col176,
                    partial_ec_mul_output_limb_54_col177,
                    partial_ec_mul_output_limb_55_col178,
                    partial_ec_mul_output_limb_56_col179,
                    partial_ec_mul_output_limb_57_col180,
                    partial_ec_mul_output_limb_58_col181,
                    partial_ec_mul_output_limb_59_col182,
                    partial_ec_mul_output_limb_60_col183,
                    partial_ec_mul_output_limb_61_col184,
                    partial_ec_mul_output_limb_62_col185,
                    partial_ec_mul_output_limb_63_col186,
                    partial_ec_mul_output_limb_64_col187,
                    partial_ec_mul_output_limb_65_col188,
                    partial_ec_mul_output_limb_66_col189,
                    partial_ec_mul_output_limb_67_col190,
                    partial_ec_mul_output_limb_68_col191,
                    partial_ec_mul_output_limb_69_col192,
                ];
                let partial_ec_mul_chain_id_tmp_c48a1_30 =
                    ((partial_ec_mul_chain_tmp_tmp_c48a1_15) + (M31_1));
                *lookup_data.partial_ec_mul_2 = [
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_14,
                    ((value_limb_0_col32) + ((value_limb_1_col33) * (M31_512))),
                    ((value_limb_2_col34) + ((value_limb_3_col35) * (M31_512))),
                    ((value_limb_4_col36) + ((value_limb_5_col37) * (M31_512))),
                    ((value_limb_6_col38) + ((value_limb_7_col39) * (M31_512))),
                    ((value_limb_8_col40) + ((value_limb_9_col41) * (M31_512))),
                    ((value_limb_10_col42) + ((value_limb_11_col43) * (M31_512))),
                    ((value_limb_12_col44) + ((value_limb_13_col45) * (M31_512))),
                    ((value_limb_14_col46) + ((value_limb_15_col47) * (M31_512))),
                    ((value_limb_16_col48) + ((value_limb_17_col49) * (M31_512))),
                    ((value_limb_18_col50) + ((value_limb_19_col51) * (M31_512))),
                    ((value_limb_20_col52) + ((value_limb_21_col53) * (M31_512))),
                    ((value_limb_22_col54) + ((value_limb_23_col55) * (M31_512))),
                    ((value_limb_24_col56) + ((value_limb_25_col57) * (M31_512))),
                    ((value_limb_26_col58) + ((ms_limb_low_col59) * (M31_512))),
                    partial_ec_mul_output_limb_14_col137,
                    partial_ec_mul_output_limb_15_col138,
                    partial_ec_mul_output_limb_16_col139,
                    partial_ec_mul_output_limb_17_col140,
                    partial_ec_mul_output_limb_18_col141,
                    partial_ec_mul_output_limb_19_col142,
                    partial_ec_mul_output_limb_20_col143,
                    partial_ec_mul_output_limb_21_col144,
                    partial_ec_mul_output_limb_22_col145,
                    partial_ec_mul_output_limb_23_col146,
                    partial_ec_mul_output_limb_24_col147,
                    partial_ec_mul_output_limb_25_col148,
                    partial_ec_mul_output_limb_26_col149,
                    partial_ec_mul_output_limb_27_col150,
                    partial_ec_mul_output_limb_28_col151,
                    partial_ec_mul_output_limb_29_col152,
                    partial_ec_mul_output_limb_30_col153,
                    partial_ec_mul_output_limb_31_col154,
                    partial_ec_mul_output_limb_32_col155,
                    partial_ec_mul_output_limb_33_col156,
                    partial_ec_mul_output_limb_34_col157,
                    partial_ec_mul_output_limb_35_col158,
                    partial_ec_mul_output_limb_36_col159,
                    partial_ec_mul_output_limb_37_col160,
                    partial_ec_mul_output_limb_38_col161,
                    partial_ec_mul_output_limb_39_col162,
                    partial_ec_mul_output_limb_40_col163,
                    partial_ec_mul_output_limb_41_col164,
                    partial_ec_mul_output_limb_42_col165,
                    partial_ec_mul_output_limb_43_col166,
                    partial_ec_mul_output_limb_44_col167,
                    partial_ec_mul_output_limb_45_col168,
                    partial_ec_mul_output_limb_46_col169,
                    partial_ec_mul_output_limb_47_col170,
                    partial_ec_mul_output_limb_48_col171,
                    partial_ec_mul_output_limb_49_col172,
                    partial_ec_mul_output_limb_50_col173,
                    partial_ec_mul_output_limb_51_col174,
                    partial_ec_mul_output_limb_52_col175,
                    partial_ec_mul_output_limb_53_col176,
                    partial_ec_mul_output_limb_54_col177,
                    partial_ec_mul_output_limb_55_col178,
                    partial_ec_mul_output_limb_56_col179,
                    partial_ec_mul_output_limb_57_col180,
                    partial_ec_mul_output_limb_58_col181,
                    partial_ec_mul_output_limb_59_col182,
                    partial_ec_mul_output_limb_60_col183,
                    partial_ec_mul_output_limb_61_col184,
                    partial_ec_mul_output_limb_62_col185,
                    partial_ec_mul_output_limb_63_col186,
                    partial_ec_mul_output_limb_64_col187,
                    partial_ec_mul_output_limb_65_col188,
                    partial_ec_mul_output_limb_66_col189,
                    partial_ec_mul_output_limb_67_col190,
                    partial_ec_mul_output_limb_68_col191,
                    partial_ec_mul_output_limb_69_col192,
                ];
                *sub_component_inputs.partial_ec_mul[14] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_14,
                    (
                        [
                            ((value_limb_0_col32) + ((value_limb_1_col33) * (M31_512))),
                            ((value_limb_2_col34) + ((value_limb_3_col35) * (M31_512))),
                            ((value_limb_4_col36) + ((value_limb_5_col37) * (M31_512))),
                            ((value_limb_6_col38) + ((value_limb_7_col39) * (M31_512))),
                            ((value_limb_8_col40) + ((value_limb_9_col41) * (M31_512))),
                            ((value_limb_10_col42) + ((value_limb_11_col43) * (M31_512))),
                            ((value_limb_12_col44) + ((value_limb_13_col45) * (M31_512))),
                            ((value_limb_14_col46) + ((value_limb_15_col47) * (M31_512))),
                            ((value_limb_16_col48) + ((value_limb_17_col49) * (M31_512))),
                            ((value_limb_18_col50) + ((value_limb_19_col51) * (M31_512))),
                            ((value_limb_20_col52) + ((value_limb_21_col53) * (M31_512))),
                            ((value_limb_22_col54) + ((value_limb_23_col55) * (M31_512))),
                            ((value_limb_24_col56) + ((value_limb_25_col57) * (M31_512))),
                            ((value_limb_26_col58) + ((ms_limb_low_col59) * (M31_512))),
                        ],
                        [
                            partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0],
                            partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_14_tmp_c48a1_31 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_14,
                        (
                            [
                                ((value_limb_0_col32) + ((value_limb_1_col33) * (M31_512))),
                                ((value_limb_2_col34) + ((value_limb_3_col35) * (M31_512))),
                                ((value_limb_4_col36) + ((value_limb_5_col37) * (M31_512))),
                                ((value_limb_6_col38) + ((value_limb_7_col39) * (M31_512))),
                                ((value_limb_8_col40) + ((value_limb_9_col41) * (M31_512))),
                                ((value_limb_10_col42) + ((value_limb_11_col43) * (M31_512))),
                                ((value_limb_12_col44) + ((value_limb_13_col45) * (M31_512))),
                                ((value_limb_14_col46) + ((value_limb_15_col47) * (M31_512))),
                                ((value_limb_16_col48) + ((value_limb_17_col49) * (M31_512))),
                                ((value_limb_18_col50) + ((value_limb_19_col51) * (M31_512))),
                                ((value_limb_20_col52) + ((value_limb_21_col53) * (M31_512))),
                                ((value_limb_22_col54) + ((value_limb_23_col55) * (M31_512))),
                                ((value_limb_24_col56) + ((value_limb_25_col57) * (M31_512))),
                                ((value_limb_26_col58) + ((ms_limb_low_col59) * (M31_512))),
                            ],
                            [
                                partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[0],
                                partial_ec_mul_output_round_13_tmp_c48a1_29.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[15] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_15,
                    (
                        [
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[0],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[1],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[2],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[3],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[4],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[5],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[6],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[7],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[8],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[9],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[10],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[11],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[12],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .1[0],
                            partial_ec_mul_output_round_14_tmp_c48a1_31.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_15_tmp_c48a1_32 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_15,
                        (
                            [
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[0],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[1],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[2],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[3],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[4],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[5],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[6],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[7],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[8],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[9],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[10],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[11],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[12],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .1[0],
                                partial_ec_mul_output_round_14_tmp_c48a1_31.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[16] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_16,
                    (
                        [
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[0],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[1],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[2],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[3],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[4],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[5],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[6],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[7],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[8],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[9],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[10],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[11],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[12],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .1[0],
                            partial_ec_mul_output_round_15_tmp_c48a1_32.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_16_tmp_c48a1_33 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_16,
                        (
                            [
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[0],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[1],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[2],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[3],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[4],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[5],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[6],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[7],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[8],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[9],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[10],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[11],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[12],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .1[0],
                                partial_ec_mul_output_round_15_tmp_c48a1_32.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[17] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_17,
                    (
                        [
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[0],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[1],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[2],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[3],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[4],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[5],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[6],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[7],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[8],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[9],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[10],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[11],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[12],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .1[0],
                            partial_ec_mul_output_round_16_tmp_c48a1_33.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_17_tmp_c48a1_34 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_17,
                        (
                            [
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[0],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[1],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[2],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[3],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[4],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[5],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[6],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[7],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[8],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[9],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[10],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[11],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[12],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .1[0],
                                partial_ec_mul_output_round_16_tmp_c48a1_33.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[18] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_18,
                    (
                        [
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[0],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[1],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[2],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[3],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[4],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[5],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[6],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[7],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[8],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[9],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[10],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[11],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[12],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .1[0],
                            partial_ec_mul_output_round_17_tmp_c48a1_34.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_18_tmp_c48a1_35 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_18,
                        (
                            [
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[0],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[1],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[2],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[3],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[4],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[5],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[6],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[7],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[8],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[9],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[10],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[11],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[12],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .1[0],
                                partial_ec_mul_output_round_17_tmp_c48a1_34.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[19] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_19,
                    (
                        [
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[0],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[1],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[2],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[3],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[4],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[5],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[6],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[7],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[8],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[9],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[10],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[11],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[12],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .1[0],
                            partial_ec_mul_output_round_18_tmp_c48a1_35.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_19_tmp_c48a1_36 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_19,
                        (
                            [
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[0],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[1],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[2],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[3],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[4],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[5],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[6],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[7],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[8],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[9],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[10],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[11],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[12],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .1[0],
                                partial_ec_mul_output_round_18_tmp_c48a1_35.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[20] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_20,
                    (
                        [
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[0],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[1],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[2],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[3],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[4],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[5],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[6],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[7],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[8],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[9],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[10],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[11],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[12],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .1[0],
                            partial_ec_mul_output_round_19_tmp_c48a1_36.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_20_tmp_c48a1_37 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_20,
                        (
                            [
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[0],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[1],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[2],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[3],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[4],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[5],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[6],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[7],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[8],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[9],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[10],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[11],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[12],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .1[0],
                                partial_ec_mul_output_round_19_tmp_c48a1_36.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[21] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_21,
                    (
                        [
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[0],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[1],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[2],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[3],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[4],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[5],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[6],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[7],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[8],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[9],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[10],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[11],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[12],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .1[0],
                            partial_ec_mul_output_round_20_tmp_c48a1_37.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_21_tmp_c48a1_38 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_21,
                        (
                            [
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[0],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[1],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[2],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[3],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[4],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[5],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[6],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[7],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[8],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[9],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[10],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[11],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[12],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .1[0],
                                partial_ec_mul_output_round_20_tmp_c48a1_37.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[22] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_22,
                    (
                        [
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[0],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[1],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[2],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[3],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[4],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[5],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[6],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[7],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[8],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[9],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[10],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[11],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[12],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .1[0],
                            partial_ec_mul_output_round_21_tmp_c48a1_38.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_22_tmp_c48a1_39 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_22,
                        (
                            [
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[0],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[1],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[2],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[3],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[4],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[5],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[6],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[7],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[8],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[9],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[10],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[11],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[12],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .1[0],
                                partial_ec_mul_output_round_21_tmp_c48a1_38.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[23] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_23,
                    (
                        [
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[0],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[1],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[2],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[3],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[4],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[5],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[6],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[7],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[8],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[9],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[10],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[11],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[12],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .1[0],
                            partial_ec_mul_output_round_22_tmp_c48a1_39.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_23_tmp_c48a1_40 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_23,
                        (
                            [
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[0],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[1],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[2],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[3],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[4],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[5],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[6],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[7],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[8],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[9],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[10],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[11],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[12],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .1[0],
                                partial_ec_mul_output_round_22_tmp_c48a1_39.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[24] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_24,
                    (
                        [
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[0],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[1],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[2],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[3],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[4],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[5],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[6],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[7],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[8],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[9],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[10],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[11],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[12],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .1[0],
                            partial_ec_mul_output_round_23_tmp_c48a1_40.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_24_tmp_c48a1_41 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_24,
                        (
                            [
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[0],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[1],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[2],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[3],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[4],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[5],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[6],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[7],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[8],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[9],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[10],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[11],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[12],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .1[0],
                                partial_ec_mul_output_round_23_tmp_c48a1_40.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[25] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_25,
                    (
                        [
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[0],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[1],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[2],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[3],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[4],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[5],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[6],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[7],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[8],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[9],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[10],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[11],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[12],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .1[0],
                            partial_ec_mul_output_round_24_tmp_c48a1_41.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_25_tmp_c48a1_42 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_25,
                        (
                            [
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[0],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[1],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[2],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[3],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[4],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[5],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[6],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[7],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[8],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[9],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[10],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[11],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[12],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .1[0],
                                partial_ec_mul_output_round_24_tmp_c48a1_41.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[26] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_26,
                    (
                        [
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[0],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[1],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[2],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[3],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[4],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[5],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[6],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[7],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[8],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[9],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[10],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[11],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[12],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .1[0],
                            partial_ec_mul_output_round_25_tmp_c48a1_42.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_26_tmp_c48a1_43 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_26,
                        (
                            [
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[0],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[1],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[2],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[3],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[4],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[5],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[6],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[7],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[8],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[9],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[10],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[11],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[12],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .1[0],
                                partial_ec_mul_output_round_25_tmp_c48a1_42.2 .1[1],
                            ],
                        ),
                    ));
                *sub_component_inputs.partial_ec_mul[27] = (
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_27,
                    (
                        [
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[0],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[1],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[2],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[3],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[4],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[5],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[6],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[7],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[8],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[9],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[10],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[11],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[12],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[13],
                        ],
                        [
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .1[0],
                            partial_ec_mul_output_round_26_tmp_c48a1_43.2 .1[1],
                        ],
                    ),
                );
                let partial_ec_mul_output_round_27_tmp_c48a1_44 =
                    PackedPartialEcMul::deduce_output((
                        partial_ec_mul_chain_id_tmp_c48a1_30,
                        M31_27,
                        (
                            [
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[0],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[1],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[2],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[3],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[4],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[5],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[6],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[7],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[8],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[9],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[10],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[11],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[12],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .0[13],
                            ],
                            [
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .1[0],
                                partial_ec_mul_output_round_26_tmp_c48a1_43.2 .1[1],
                            ],
                        ),
                    ));
                let partial_ec_mul_output_limb_0_col193 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[0];
                *row[193] = partial_ec_mul_output_limb_0_col193;
                let partial_ec_mul_output_limb_1_col194 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[1];
                *row[194] = partial_ec_mul_output_limb_1_col194;
                let partial_ec_mul_output_limb_2_col195 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[2];
                *row[195] = partial_ec_mul_output_limb_2_col195;
                let partial_ec_mul_output_limb_3_col196 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[3];
                *row[196] = partial_ec_mul_output_limb_3_col196;
                let partial_ec_mul_output_limb_4_col197 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[4];
                *row[197] = partial_ec_mul_output_limb_4_col197;
                let partial_ec_mul_output_limb_5_col198 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[5];
                *row[198] = partial_ec_mul_output_limb_5_col198;
                let partial_ec_mul_output_limb_6_col199 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[6];
                *row[199] = partial_ec_mul_output_limb_6_col199;
                let partial_ec_mul_output_limb_7_col200 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[7];
                *row[200] = partial_ec_mul_output_limb_7_col200;
                let partial_ec_mul_output_limb_8_col201 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[8];
                *row[201] = partial_ec_mul_output_limb_8_col201;
                let partial_ec_mul_output_limb_9_col202 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[9];
                *row[202] = partial_ec_mul_output_limb_9_col202;
                let partial_ec_mul_output_limb_10_col203 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[10];
                *row[203] = partial_ec_mul_output_limb_10_col203;
                let partial_ec_mul_output_limb_11_col204 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[11];
                *row[204] = partial_ec_mul_output_limb_11_col204;
                let partial_ec_mul_output_limb_12_col205 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[12];
                *row[205] = partial_ec_mul_output_limb_12_col205;
                let partial_ec_mul_output_limb_13_col206 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .0[13];
                *row[206] = partial_ec_mul_output_limb_13_col206;
                let partial_ec_mul_output_limb_14_col207 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(0);
                *row[207] = partial_ec_mul_output_limb_14_col207;
                let partial_ec_mul_output_limb_15_col208 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(1);
                *row[208] = partial_ec_mul_output_limb_15_col208;
                let partial_ec_mul_output_limb_16_col209 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(2);
                *row[209] = partial_ec_mul_output_limb_16_col209;
                let partial_ec_mul_output_limb_17_col210 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(3);
                *row[210] = partial_ec_mul_output_limb_17_col210;
                let partial_ec_mul_output_limb_18_col211 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(4);
                *row[211] = partial_ec_mul_output_limb_18_col211;
                let partial_ec_mul_output_limb_19_col212 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(5);
                *row[212] = partial_ec_mul_output_limb_19_col212;
                let partial_ec_mul_output_limb_20_col213 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(6);
                *row[213] = partial_ec_mul_output_limb_20_col213;
                let partial_ec_mul_output_limb_21_col214 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(7);
                *row[214] = partial_ec_mul_output_limb_21_col214;
                let partial_ec_mul_output_limb_22_col215 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(8);
                *row[215] = partial_ec_mul_output_limb_22_col215;
                let partial_ec_mul_output_limb_23_col216 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(9);
                *row[216] = partial_ec_mul_output_limb_23_col216;
                let partial_ec_mul_output_limb_24_col217 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(10);
                *row[217] = partial_ec_mul_output_limb_24_col217;
                let partial_ec_mul_output_limb_25_col218 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(11);
                *row[218] = partial_ec_mul_output_limb_25_col218;
                let partial_ec_mul_output_limb_26_col219 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(12);
                *row[219] = partial_ec_mul_output_limb_26_col219;
                let partial_ec_mul_output_limb_27_col220 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(13);
                *row[220] = partial_ec_mul_output_limb_27_col220;
                let partial_ec_mul_output_limb_28_col221 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(14);
                *row[221] = partial_ec_mul_output_limb_28_col221;
                let partial_ec_mul_output_limb_29_col222 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(15);
                *row[222] = partial_ec_mul_output_limb_29_col222;
                let partial_ec_mul_output_limb_30_col223 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(16);
                *row[223] = partial_ec_mul_output_limb_30_col223;
                let partial_ec_mul_output_limb_31_col224 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(17);
                *row[224] = partial_ec_mul_output_limb_31_col224;
                let partial_ec_mul_output_limb_32_col225 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(18);
                *row[225] = partial_ec_mul_output_limb_32_col225;
                let partial_ec_mul_output_limb_33_col226 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(19);
                *row[226] = partial_ec_mul_output_limb_33_col226;
                let partial_ec_mul_output_limb_34_col227 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(20);
                *row[227] = partial_ec_mul_output_limb_34_col227;
                let partial_ec_mul_output_limb_35_col228 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(21);
                *row[228] = partial_ec_mul_output_limb_35_col228;
                let partial_ec_mul_output_limb_36_col229 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(22);
                *row[229] = partial_ec_mul_output_limb_36_col229;
                let partial_ec_mul_output_limb_37_col230 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(23);
                *row[230] = partial_ec_mul_output_limb_37_col230;
                let partial_ec_mul_output_limb_38_col231 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(24);
                *row[231] = partial_ec_mul_output_limb_38_col231;
                let partial_ec_mul_output_limb_39_col232 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(25);
                *row[232] = partial_ec_mul_output_limb_39_col232;
                let partial_ec_mul_output_limb_40_col233 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(26);
                *row[233] = partial_ec_mul_output_limb_40_col233;
                let partial_ec_mul_output_limb_41_col234 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[0].get_m31(27);
                *row[234] = partial_ec_mul_output_limb_41_col234;
                let partial_ec_mul_output_limb_42_col235 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(0);
                *row[235] = partial_ec_mul_output_limb_42_col235;
                let partial_ec_mul_output_limb_43_col236 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(1);
                *row[236] = partial_ec_mul_output_limb_43_col236;
                let partial_ec_mul_output_limb_44_col237 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(2);
                *row[237] = partial_ec_mul_output_limb_44_col237;
                let partial_ec_mul_output_limb_45_col238 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(3);
                *row[238] = partial_ec_mul_output_limb_45_col238;
                let partial_ec_mul_output_limb_46_col239 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(4);
                *row[239] = partial_ec_mul_output_limb_46_col239;
                let partial_ec_mul_output_limb_47_col240 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(5);
                *row[240] = partial_ec_mul_output_limb_47_col240;
                let partial_ec_mul_output_limb_48_col241 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(6);
                *row[241] = partial_ec_mul_output_limb_48_col241;
                let partial_ec_mul_output_limb_49_col242 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(7);
                *row[242] = partial_ec_mul_output_limb_49_col242;
                let partial_ec_mul_output_limb_50_col243 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(8);
                *row[243] = partial_ec_mul_output_limb_50_col243;
                let partial_ec_mul_output_limb_51_col244 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(9);
                *row[244] = partial_ec_mul_output_limb_51_col244;
                let partial_ec_mul_output_limb_52_col245 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(10);
                *row[245] = partial_ec_mul_output_limb_52_col245;
                let partial_ec_mul_output_limb_53_col246 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(11);
                *row[246] = partial_ec_mul_output_limb_53_col246;
                let partial_ec_mul_output_limb_54_col247 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(12);
                *row[247] = partial_ec_mul_output_limb_54_col247;
                let partial_ec_mul_output_limb_55_col248 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(13);
                *row[248] = partial_ec_mul_output_limb_55_col248;
                let partial_ec_mul_output_limb_56_col249 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(14);
                *row[249] = partial_ec_mul_output_limb_56_col249;
                let partial_ec_mul_output_limb_57_col250 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(15);
                *row[250] = partial_ec_mul_output_limb_57_col250;
                let partial_ec_mul_output_limb_58_col251 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(16);
                *row[251] = partial_ec_mul_output_limb_58_col251;
                let partial_ec_mul_output_limb_59_col252 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(17);
                *row[252] = partial_ec_mul_output_limb_59_col252;
                let partial_ec_mul_output_limb_60_col253 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(18);
                *row[253] = partial_ec_mul_output_limb_60_col253;
                let partial_ec_mul_output_limb_61_col254 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(19);
                *row[254] = partial_ec_mul_output_limb_61_col254;
                let partial_ec_mul_output_limb_62_col255 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(20);
                *row[255] = partial_ec_mul_output_limb_62_col255;
                let partial_ec_mul_output_limb_63_col256 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(21);
                *row[256] = partial_ec_mul_output_limb_63_col256;
                let partial_ec_mul_output_limb_64_col257 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(22);
                *row[257] = partial_ec_mul_output_limb_64_col257;
                let partial_ec_mul_output_limb_65_col258 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(23);
                *row[258] = partial_ec_mul_output_limb_65_col258;
                let partial_ec_mul_output_limb_66_col259 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(24);
                *row[259] = partial_ec_mul_output_limb_66_col259;
                let partial_ec_mul_output_limb_67_col260 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(25);
                *row[260] = partial_ec_mul_output_limb_67_col260;
                let partial_ec_mul_output_limb_68_col261 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(26);
                *row[261] = partial_ec_mul_output_limb_68_col261;
                let partial_ec_mul_output_limb_69_col262 =
                    partial_ec_mul_output_round_27_tmp_c48a1_44.2 .1[1].get_m31(27);
                *row[262] = partial_ec_mul_output_limb_69_col262;
                *lookup_data.partial_ec_mul_3 = [
                    partial_ec_mul_chain_id_tmp_c48a1_30,
                    M31_28,
                    partial_ec_mul_output_limb_0_col193,
                    partial_ec_mul_output_limb_1_col194,
                    partial_ec_mul_output_limb_2_col195,
                    partial_ec_mul_output_limb_3_col196,
                    partial_ec_mul_output_limb_4_col197,
                    partial_ec_mul_output_limb_5_col198,
                    partial_ec_mul_output_limb_6_col199,
                    partial_ec_mul_output_limb_7_col200,
                    partial_ec_mul_output_limb_8_col201,
                    partial_ec_mul_output_limb_9_col202,
                    partial_ec_mul_output_limb_10_col203,
                    partial_ec_mul_output_limb_11_col204,
                    partial_ec_mul_output_limb_12_col205,
                    partial_ec_mul_output_limb_13_col206,
                    partial_ec_mul_output_limb_14_col207,
                    partial_ec_mul_output_limb_15_col208,
                    partial_ec_mul_output_limb_16_col209,
                    partial_ec_mul_output_limb_17_col210,
                    partial_ec_mul_output_limb_18_col211,
                    partial_ec_mul_output_limb_19_col212,
                    partial_ec_mul_output_limb_20_col213,
                    partial_ec_mul_output_limb_21_col214,
                    partial_ec_mul_output_limb_22_col215,
                    partial_ec_mul_output_limb_23_col216,
                    partial_ec_mul_output_limb_24_col217,
                    partial_ec_mul_output_limb_25_col218,
                    partial_ec_mul_output_limb_26_col219,
                    partial_ec_mul_output_limb_27_col220,
                    partial_ec_mul_output_limb_28_col221,
                    partial_ec_mul_output_limb_29_col222,
                    partial_ec_mul_output_limb_30_col223,
                    partial_ec_mul_output_limb_31_col224,
                    partial_ec_mul_output_limb_32_col225,
                    partial_ec_mul_output_limb_33_col226,
                    partial_ec_mul_output_limb_34_col227,
                    partial_ec_mul_output_limb_35_col228,
                    partial_ec_mul_output_limb_36_col229,
                    partial_ec_mul_output_limb_37_col230,
                    partial_ec_mul_output_limb_38_col231,
                    partial_ec_mul_output_limb_39_col232,
                    partial_ec_mul_output_limb_40_col233,
                    partial_ec_mul_output_limb_41_col234,
                    partial_ec_mul_output_limb_42_col235,
                    partial_ec_mul_output_limb_43_col236,
                    partial_ec_mul_output_limb_44_col237,
                    partial_ec_mul_output_limb_45_col238,
                    partial_ec_mul_output_limb_46_col239,
                    partial_ec_mul_output_limb_47_col240,
                    partial_ec_mul_output_limb_48_col241,
                    partial_ec_mul_output_limb_49_col242,
                    partial_ec_mul_output_limb_50_col243,
                    partial_ec_mul_output_limb_51_col244,
                    partial_ec_mul_output_limb_52_col245,
                    partial_ec_mul_output_limb_53_col246,
                    partial_ec_mul_output_limb_54_col247,
                    partial_ec_mul_output_limb_55_col248,
                    partial_ec_mul_output_limb_56_col249,
                    partial_ec_mul_output_limb_57_col250,
                    partial_ec_mul_output_limb_58_col251,
                    partial_ec_mul_output_limb_59_col252,
                    partial_ec_mul_output_limb_60_col253,
                    partial_ec_mul_output_limb_61_col254,
                    partial_ec_mul_output_limb_62_col255,
                    partial_ec_mul_output_limb_63_col256,
                    partial_ec_mul_output_limb_64_col257,
                    partial_ec_mul_output_limb_65_col258,
                    partial_ec_mul_output_limb_66_col259,
                    partial_ec_mul_output_limb_67_col260,
                    partial_ec_mul_output_limb_68_col261,
                    partial_ec_mul_output_limb_69_col262,
                ];
                *sub_component_inputs.memory_id_to_big[2] = input_limb_2_col2;
                *lookup_data.memory_id_to_big_2 = [
                    input_limb_2_col2,
                    partial_ec_mul_output_limb_14_col207,
                    partial_ec_mul_output_limb_15_col208,
                    partial_ec_mul_output_limb_16_col209,
                    partial_ec_mul_output_limb_17_col210,
                    partial_ec_mul_output_limb_18_col211,
                    partial_ec_mul_output_limb_19_col212,
                    partial_ec_mul_output_limb_20_col213,
                    partial_ec_mul_output_limb_21_col214,
                    partial_ec_mul_output_limb_22_col215,
                    partial_ec_mul_output_limb_23_col216,
                    partial_ec_mul_output_limb_24_col217,
                    partial_ec_mul_output_limb_25_col218,
                    partial_ec_mul_output_limb_26_col219,
                    partial_ec_mul_output_limb_27_col220,
                    partial_ec_mul_output_limb_28_col221,
                    partial_ec_mul_output_limb_29_col222,
                    partial_ec_mul_output_limb_30_col223,
                    partial_ec_mul_output_limb_31_col224,
                    partial_ec_mul_output_limb_32_col225,
                    partial_ec_mul_output_limb_33_col226,
                    partial_ec_mul_output_limb_34_col227,
                    partial_ec_mul_output_limb_35_col228,
                    partial_ec_mul_output_limb_36_col229,
                    partial_ec_mul_output_limb_37_col230,
                    partial_ec_mul_output_limb_38_col231,
                    partial_ec_mul_output_limb_39_col232,
                    partial_ec_mul_output_limb_40_col233,
                    partial_ec_mul_output_limb_41_col234,
                ];
                *lookup_data.pedersen_aggregator_0 =
                    [input_limb_0_col0, input_limb_1_col1, input_limb_2_col2];
                let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
                *row[263] = mult_at_row;
                *lookup_data.mults = mult_at_row;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    partial_ec_mul_0: Vec<[PackedM31; 72]>,
    partial_ec_mul_1: Vec<[PackedM31; 72]>,
    partial_ec_mul_2: Vec<[PackedM31; 72]>,
    partial_ec_mul_3: Vec<[PackedM31; 72]>,
    pedersen_aggregator_0: Vec<[PackedM31; 3]>,
    pedersen_points_table_0: Vec<[PackedM31; 57]>,
    range_check_5_4_0: Vec<[PackedM31; 2]>,
    range_check_5_4_1: Vec<[PackedM31; 2]>,
    range_check_8_0: Vec<[PackedM31; 1]>,
    range_check_8_1: Vec<[PackedM31; 1]>,
    range_check_8_2: Vec<[PackedM31; 1]>,
    range_check_8_3: Vec<[PackedM31; 1]>,
    mults: Vec<PackedM31>,
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
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_8: &relations::RangeCheck_8,
        pedersen_points_table: &relations::PedersenPointsTable,
        partial_ec_mul: &relations::PartialEcMul,
        pedersen_aggregator: &relations::PedersenAggregator,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_5_4_0,
            &self.lookup_data.memory_id_to_big_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_5_4.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_5_4_1,
            &self.lookup_data.memory_id_to_big_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_5_4.combine(values0);
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
            &self.lookup_data.memory_id_to_big_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = partial_ec_mul.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.pedersen_aggregator_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = pedersen_aggregator.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

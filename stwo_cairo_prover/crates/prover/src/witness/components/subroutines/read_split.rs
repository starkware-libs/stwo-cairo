// AIR version 97774321-dirty
#![allow(unused_parens)]
use cairo_air::components::read_split::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big, range_check_5_4};
use crate::witness::prelude::*;

pub type InputType = M31;
pub type PackedInputType = PackedM31;

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
        range_check_5_4_state: &range_check_5_4::ClaimGenerator,
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
            range_check_5_4_state,
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
    range_check_5_4: [Vec<range_check_5_4::PackedInputType>; 1],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_5_4_state: &range_check_5_4::ClaimGenerator,
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

    let M31_32 = PackedM31::broadcast(M31::from(32));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs, read_split_input))| {
                let memory_address_to_id_value_tmp_61c4d_0 =
                    memory_address_to_id_state.deduce_output(read_split_input);
                let memory_id_to_big_value_tmp_61c4d_1 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_61c4d_0);
                let value_limb_0_col0 = memory_id_to_big_value_tmp_61c4d_1.get_m31(0);
                *row[0] = value_limb_0_col0;
                let value_limb_1_col1 = memory_id_to_big_value_tmp_61c4d_1.get_m31(1);
                *row[1] = value_limb_1_col1;
                let value_limb_2_col2 = memory_id_to_big_value_tmp_61c4d_1.get_m31(2);
                *row[2] = value_limb_2_col2;
                let value_limb_3_col3 = memory_id_to_big_value_tmp_61c4d_1.get_m31(3);
                *row[3] = value_limb_3_col3;
                let value_limb_4_col4 = memory_id_to_big_value_tmp_61c4d_1.get_m31(4);
                *row[4] = value_limb_4_col4;
                let value_limb_5_col5 = memory_id_to_big_value_tmp_61c4d_1.get_m31(5);
                *row[5] = value_limb_5_col5;
                let value_limb_6_col6 = memory_id_to_big_value_tmp_61c4d_1.get_m31(6);
                *row[6] = value_limb_6_col6;
                let value_limb_7_col7 = memory_id_to_big_value_tmp_61c4d_1.get_m31(7);
                *row[7] = value_limb_7_col7;
                let value_limb_8_col8 = memory_id_to_big_value_tmp_61c4d_1.get_m31(8);
                *row[8] = value_limb_8_col8;
                let value_limb_9_col9 = memory_id_to_big_value_tmp_61c4d_1.get_m31(9);
                *row[9] = value_limb_9_col9;
                let value_limb_10_col10 = memory_id_to_big_value_tmp_61c4d_1.get_m31(10);
                *row[10] = value_limb_10_col10;
                let value_limb_11_col11 = memory_id_to_big_value_tmp_61c4d_1.get_m31(11);
                *row[11] = value_limb_11_col11;
                let value_limb_12_col12 = memory_id_to_big_value_tmp_61c4d_1.get_m31(12);
                *row[12] = value_limb_12_col12;
                let value_limb_13_col13 = memory_id_to_big_value_tmp_61c4d_1.get_m31(13);
                *row[13] = value_limb_13_col13;
                let value_limb_14_col14 = memory_id_to_big_value_tmp_61c4d_1.get_m31(14);
                *row[14] = value_limb_14_col14;
                let value_limb_15_col15 = memory_id_to_big_value_tmp_61c4d_1.get_m31(15);
                *row[15] = value_limb_15_col15;
                let value_limb_16_col16 = memory_id_to_big_value_tmp_61c4d_1.get_m31(16);
                *row[16] = value_limb_16_col16;
                let value_limb_17_col17 = memory_id_to_big_value_tmp_61c4d_1.get_m31(17);
                *row[17] = value_limb_17_col17;
                let value_limb_18_col18 = memory_id_to_big_value_tmp_61c4d_1.get_m31(18);
                *row[18] = value_limb_18_col18;
                let value_limb_19_col19 = memory_id_to_big_value_tmp_61c4d_1.get_m31(19);
                *row[19] = value_limb_19_col19;
                let value_limb_20_col20 = memory_id_to_big_value_tmp_61c4d_1.get_m31(20);
                *row[20] = value_limb_20_col20;
                let value_limb_21_col21 = memory_id_to_big_value_tmp_61c4d_1.get_m31(21);
                *row[21] = value_limb_21_col21;
                let value_limb_22_col22 = memory_id_to_big_value_tmp_61c4d_1.get_m31(22);
                *row[22] = value_limb_22_col22;
                let value_limb_23_col23 = memory_id_to_big_value_tmp_61c4d_1.get_m31(23);
                *row[23] = value_limb_23_col23;
                let value_limb_24_col24 = memory_id_to_big_value_tmp_61c4d_1.get_m31(24);
                *row[24] = value_limb_24_col24;
                let value_limb_25_col25 = memory_id_to_big_value_tmp_61c4d_1.get_m31(25);
                *row[25] = value_limb_25_col25;
                let value_limb_26_col26 = memory_id_to_big_value_tmp_61c4d_1.get_m31(26);
                *row[26] = value_limb_26_col26;
                let ms_limb_low_tmp_61c4d_2 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_61c4d_1.get_m31(27)))
                        & (UInt16_31));
                let ms_limb_low_col27 = ms_limb_low_tmp_61c4d_2.as_m31();
                *row[27] = ms_limb_low_col27;
                let ms_limb_high_tmp_61c4d_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_61c4d_1.get_m31(27)))
                        >> (UInt16_5));
                let ms_limb_high_col28 = ms_limb_high_tmp_61c4d_3.as_m31();
                *row[28] = ms_limb_high_col28;
                *sub_component_inputs.range_check_5_4[0] = [ms_limb_low_col27, ms_limb_high_col28];
                *lookup_data.range_check_5_4_0 = [ms_limb_low_col27, ms_limb_high_col28];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_61c4d_4 =
                    memory_address_to_id_state.deduce_output(read_split_input);
                let id_col29 = memory_address_to_id_value_tmp_61c4d_4;
                *row[29] = id_col29;
                *sub_component_inputs.memory_address_to_id[0] = read_split_input;
                *lookup_data.memory_address_to_id_0 = [read_split_input, id_col29];

                *sub_component_inputs.memory_id_to_big[0] = id_col29;
                *lookup_data.memory_id_to_big_0 = [
                    id_col29,
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
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    range_check_5_4_0: Vec<[PackedM31; 2]>,
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

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.memory_id_to_big_0)
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

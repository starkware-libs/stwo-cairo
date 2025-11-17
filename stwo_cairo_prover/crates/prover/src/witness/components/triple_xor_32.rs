// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::triple_xor_32::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{verify_bitwise_xor_8, verify_bitwise_xor_8_b};
use crate::witness::prelude::*;

pub type PackedInputType = [PackedUInt32; 3];

#[derive(Default)]
pub struct ClaimGenerator {
    pub packed_inputs: Vec<PackedInputType>,
}

impl ClaimGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.packed_inputs.is_empty()
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
        verify_bitwise_xor_8_b_state: &verify_bitwise_xor_8_b::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        assert!(!self.packed_inputs.is_empty());
        let n_vec_rows = self.packed_inputs.len();
        let n_rows = n_vec_rows * N_LANES;
        let packed_size = n_vec_rows.next_power_of_two();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        self.packed_inputs
            .resize(packed_size, *self.packed_inputs.first().unwrap());

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            self.packed_inputs,
            n_rows,
            verify_bitwise_xor_8_state,
            verify_bitwise_xor_8_b_state,
        );
        sub_component_inputs
            .verify_bitwise_xor_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_8_b
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_b_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                n_rows,
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_packed_inputs(&mut self, inputs: &[PackedInputType]) {
        self.packed_inputs.extend(inputs);
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    verify_bitwise_xor_8: [Vec<verify_bitwise_xor_8::PackedInputType>; 4],
    verify_bitwise_xor_8_b: [Vec<verify_bitwise_xor_8_b::PackedInputType>; 4],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    verify_bitwise_xor_8_b_state: &verify_bitwise_xor_8_b::ClaimGenerator,
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

    let M31_256 = PackedM31::broadcast(M31::from(256));
    let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));
    let enabler_col = Enabler::new(n_rows);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (row, lookup_data, sub_component_inputs, triple_xor_32_input))| {
                let input_limb_0_col0 = triple_xor_32_input[0].low().as_m31();
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = triple_xor_32_input[0].high().as_m31();
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = triple_xor_32_input[1].low().as_m31();
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = triple_xor_32_input[1].high().as_m31();
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = triple_xor_32_input[2].low().as_m31();
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = triple_xor_32_input[2].high().as_m31();
                *row[5] = input_limb_5_col5;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_298db_0 = ((triple_xor_32_input[0].low()) >> (UInt16_8));
                let ms_8_bits_col6 = ms_8_bits_tmp_298db_0.as_m31();
                *row[6] = ms_8_bits_col6;
                let split_16_low_part_size_8_output_tmp_298db_1 = [
                    ((input_limb_0_col0) - ((ms_8_bits_col6) * (M31_256))),
                    ms_8_bits_col6,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_298db_2 = ((triple_xor_32_input[0].high()) >> (UInt16_8));
                let ms_8_bits_col7 = ms_8_bits_tmp_298db_2.as_m31();
                *row[7] = ms_8_bits_col7;
                let split_16_low_part_size_8_output_tmp_298db_3 = [
                    ((input_limb_1_col1) - ((ms_8_bits_col7) * (M31_256))),
                    ms_8_bits_col7,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_298db_4 = ((triple_xor_32_input[1].low()) >> (UInt16_8));
                let ms_8_bits_col8 = ms_8_bits_tmp_298db_4.as_m31();
                *row[8] = ms_8_bits_col8;
                let split_16_low_part_size_8_output_tmp_298db_5 = [
                    ((input_limb_2_col2) - ((ms_8_bits_col8) * (M31_256))),
                    ms_8_bits_col8,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_298db_6 = ((triple_xor_32_input[1].high()) >> (UInt16_8));
                let ms_8_bits_col9 = ms_8_bits_tmp_298db_6.as_m31();
                *row[9] = ms_8_bits_col9;
                let split_16_low_part_size_8_output_tmp_298db_7 = [
                    ((input_limb_3_col3) - ((ms_8_bits_col9) * (M31_256))),
                    ms_8_bits_col9,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_298db_8 = ((triple_xor_32_input[2].low()) >> (UInt16_8));
                let ms_8_bits_col10 = ms_8_bits_tmp_298db_8.as_m31();
                *row[10] = ms_8_bits_col10;
                let split_16_low_part_size_8_output_tmp_298db_9 = [
                    ((input_limb_4_col4) - ((ms_8_bits_col10) * (M31_256))),
                    ms_8_bits_col10,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_298db_10 = ((triple_xor_32_input[2].high()) >> (UInt16_8));
                let ms_8_bits_col11 = ms_8_bits_tmp_298db_10.as_m31();
                *row[11] = ms_8_bits_col11;
                let split_16_low_part_size_8_output_tmp_298db_11 = [
                    ((input_limb_5_col5) - ((ms_8_bits_col11) * (M31_256))),
                    ms_8_bits_col11,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_298db_12 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_298db_1[0]))
                        ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_298db_5[0])));
                let xor_col12 = xor_tmp_298db_12.as_m31();
                *row[12] = xor_col12;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [
                    split_16_low_part_size_8_output_tmp_298db_1[0],
                    split_16_low_part_size_8_output_tmp_298db_5[0],
                    xor_col12,
                ];
                *lookup_data.verify_bitwise_xor_8_0 = [
                    split_16_low_part_size_8_output_tmp_298db_1[0],
                    split_16_low_part_size_8_output_tmp_298db_5[0],
                    xor_col12,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_298db_14 = ((PackedUInt16::from_m31(xor_col12))
                    ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_298db_9[0])));
                let xor_col13 = xor_tmp_298db_14.as_m31();
                *row[13] = xor_col13;
                *sub_component_inputs.verify_bitwise_xor_8[1] = [
                    xor_col12,
                    split_16_low_part_size_8_output_tmp_298db_9[0],
                    xor_col13,
                ];
                *lookup_data.verify_bitwise_xor_8_1 = [
                    xor_col12,
                    split_16_low_part_size_8_output_tmp_298db_9[0],
                    xor_col13,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_298db_16 = ((PackedUInt16::from_m31(ms_8_bits_col6))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col8)));
                let xor_col14 = xor_tmp_298db_16.as_m31();
                *row[14] = xor_col14;
                *sub_component_inputs.verify_bitwise_xor_8[2] =
                    [ms_8_bits_col6, ms_8_bits_col8, xor_col14];
                *lookup_data.verify_bitwise_xor_8_2 = [ms_8_bits_col6, ms_8_bits_col8, xor_col14];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_298db_18 = ((PackedUInt16::from_m31(xor_col14))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col10)));
                let xor_col15 = xor_tmp_298db_18.as_m31();
                *row[15] = xor_col15;
                *sub_component_inputs.verify_bitwise_xor_8[3] =
                    [xor_col14, ms_8_bits_col10, xor_col15];
                *lookup_data.verify_bitwise_xor_8_3 = [xor_col14, ms_8_bits_col10, xor_col15];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_298db_20 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_298db_3[0]))
                        ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_298db_7[0])));
                let xor_col16 = xor_tmp_298db_20.as_m31();
                *row[16] = xor_col16;
                *sub_component_inputs.verify_bitwise_xor_8_b[0] = [
                    split_16_low_part_size_8_output_tmp_298db_3[0],
                    split_16_low_part_size_8_output_tmp_298db_7[0],
                    xor_col16,
                ];
                *lookup_data.verify_bitwise_xor_8_b_0 = [
                    split_16_low_part_size_8_output_tmp_298db_3[0],
                    split_16_low_part_size_8_output_tmp_298db_7[0],
                    xor_col16,
                ];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_298db_22 = ((PackedUInt16::from_m31(xor_col16))
                    ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_298db_11[0])));
                let xor_col17 = xor_tmp_298db_22.as_m31();
                *row[17] = xor_col17;
                *sub_component_inputs.verify_bitwise_xor_8_b[1] = [
                    xor_col16,
                    split_16_low_part_size_8_output_tmp_298db_11[0],
                    xor_col17,
                ];
                *lookup_data.verify_bitwise_xor_8_b_1 = [
                    xor_col16,
                    split_16_low_part_size_8_output_tmp_298db_11[0],
                    xor_col17,
                ];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_298db_24 = ((PackedUInt16::from_m31(ms_8_bits_col7))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col9)));
                let xor_col18 = xor_tmp_298db_24.as_m31();
                *row[18] = xor_col18;
                *sub_component_inputs.verify_bitwise_xor_8_b[2] =
                    [ms_8_bits_col7, ms_8_bits_col9, xor_col18];
                *lookup_data.verify_bitwise_xor_8_b_2 = [ms_8_bits_col7, ms_8_bits_col9, xor_col18];

                // Bitwise Xor Num Bits 8 B.

                let xor_tmp_298db_26 = ((PackedUInt16::from_m31(xor_col18))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col11)));
                let xor_col19 = xor_tmp_298db_26.as_m31();
                *row[19] = xor_col19;
                *sub_component_inputs.verify_bitwise_xor_8_b[3] =
                    [xor_col18, ms_8_bits_col11, xor_col19];
                *lookup_data.verify_bitwise_xor_8_b_3 = [xor_col18, ms_8_bits_col11, xor_col19];

                let triple_xor32_output_tmp_298db_28 = PackedUInt32::from_limbs([
                    ((xor_col13) + ((xor_col15) * (M31_256))),
                    ((xor_col17) + ((xor_col19) * (M31_256))),
                ]);
                *lookup_data.triple_xor_32_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    triple_xor32_output_tmp_298db_28.low().as_m31(),
                    triple_xor32_output_tmp_298db_28.high().as_m31(),
                ];
                *row[20] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    triple_xor_32_0: Vec<[PackedM31; 8]>,
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_b_3: Vec<[PackedM31; 3]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
        verify_bitwise_xor_8_b: &relations::VerifyBitwiseXor_8_B,
        triple_xor_32: &relations::TripleXor32,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_0,
            &self.lookup_data.verify_bitwise_xor_8_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_2,
            &self.lookup_data.verify_bitwise_xor_8_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_b_0,
            &self.lookup_data.verify_bitwise_xor_8_b_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8_b.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_b_2,
            &self.lookup_data.verify_bitwise_xor_8_b_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_8_b.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_8_b.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.triple_xor_32_0)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = triple_xor_32.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

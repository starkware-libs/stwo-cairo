// AIR version e1943601-dirty
#![allow(unused_parens)]
use cairo_air::components::triple_xor_16::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::verify_bitwise_xor_8;
use crate::witness::prelude::*;

pub type PackedInputType = [PackedUInt16; 3];

#[derive(Default)]
pub struct ClaimGenerator {
    pub packed_inputs: Vec<PackedInputType>,
}
impl ClaimGenerator {
    pub fn new() -> Self {
        Self {
            packed_inputs: vec![],
        }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        assert!(!self.packed_inputs.is_empty());
        let n_vec_rows = self.packed_inputs.len();
        let n_rows = n_vec_rows * N_LANES;
        let packed_size = n_vec_rows.next_power_of_two();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        self.packed_inputs
            .resize(packed_size, *self.packed_inputs.first().unwrap());

        let (trace, lookup_data, sub_component_inputs) =
            write_trace_simd(self.packed_inputs, n_rows, verify_bitwise_xor_8_state);
        sub_component_inputs
            .verify_bitwise_xor_8
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_8_state.add_packed_inputs(inputs);
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
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
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
            |(row_index, (mut row, lookup_data, sub_component_inputs, triple_xor_16_input))| {
                let input_limb_0_col0 = triple_xor_16_input[0].as_m31();
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = triple_xor_16_input[1].as_m31();
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = triple_xor_16_input[2].as_m31();
                *row[2] = input_limb_2_col2;

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_3c0e0_0 = ((triple_xor_16_input[0]) >> (UInt16_8));
                let ms_8_bits_col3 = ms_8_bits_tmp_3c0e0_0.as_m31();
                *row[3] = ms_8_bits_col3;
                let split_16_low_part_size_8_output_tmp_3c0e0_1 = [
                    ((input_limb_0_col0) - ((ms_8_bits_col3) * (M31_256))),
                    ms_8_bits_col3,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_3c0e0_2 = ((triple_xor_16_input[1]) >> (UInt16_8));
                let ms_8_bits_col4 = ms_8_bits_tmp_3c0e0_2.as_m31();
                *row[4] = ms_8_bits_col4;
                let split_16_low_part_size_8_output_tmp_3c0e0_3 = [
                    ((input_limb_1_col1) - ((ms_8_bits_col4) * (M31_256))),
                    ms_8_bits_col4,
                ];

                // Split 16 Low Part Size 8.

                let ms_8_bits_tmp_3c0e0_4 = ((triple_xor_16_input[2]) >> (UInt16_8));
                let ms_8_bits_col5 = ms_8_bits_tmp_3c0e0_4.as_m31();
                *row[5] = ms_8_bits_col5;
                let split_16_low_part_size_8_output_tmp_3c0e0_5 = [
                    ((input_limb_2_col2) - ((ms_8_bits_col5) * (M31_256))),
                    ms_8_bits_col5,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_3c0e0_6 =
                    ((PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_3c0e0_1[0]))
                        ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_3c0e0_3[0])));
                let xor_col6 = xor_tmp_3c0e0_6.as_m31();
                *row[6] = xor_col6;
                *sub_component_inputs.verify_bitwise_xor_8[0] = [
                    split_16_low_part_size_8_output_tmp_3c0e0_1[0],
                    split_16_low_part_size_8_output_tmp_3c0e0_3[0],
                    xor_col6,
                ];
                *lookup_data.verify_bitwise_xor_8_0 = [
                    split_16_low_part_size_8_output_tmp_3c0e0_1[0],
                    split_16_low_part_size_8_output_tmp_3c0e0_3[0],
                    xor_col6,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_3c0e0_8 = ((PackedUInt16::from_m31(xor_col6))
                    ^ (PackedUInt16::from_m31(split_16_low_part_size_8_output_tmp_3c0e0_5[0])));
                let xor_col7 = xor_tmp_3c0e0_8.as_m31();
                *row[7] = xor_col7;
                *sub_component_inputs.verify_bitwise_xor_8[1] = [
                    xor_col6,
                    split_16_low_part_size_8_output_tmp_3c0e0_5[0],
                    xor_col7,
                ];
                *lookup_data.verify_bitwise_xor_8_1 = [
                    xor_col6,
                    split_16_low_part_size_8_output_tmp_3c0e0_5[0],
                    xor_col7,
                ];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_3c0e0_10 = ((PackedUInt16::from_m31(ms_8_bits_col3))
                    ^ (PackedUInt16::from_m31(ms_8_bits_col4)));
                let xor_col8 = xor_tmp_3c0e0_10.as_m31();
                *row[8] = xor_col8;
                *sub_component_inputs.verify_bitwise_xor_8[2] =
                    [ms_8_bits_col3, ms_8_bits_col4, xor_col8];
                *lookup_data.verify_bitwise_xor_8_2 = [ms_8_bits_col3, ms_8_bits_col4, xor_col8];

                // Bitwise Xor Num Bits 8.

                let xor_tmp_3c0e0_12 =
                    ((PackedUInt16::from_m31(xor_col8)) ^ (PackedUInt16::from_m31(ms_8_bits_col5)));
                let xor_col9 = xor_tmp_3c0e0_12.as_m31();
                *row[9] = xor_col9;
                *sub_component_inputs.verify_bitwise_xor_8[3] =
                    [xor_col8, ms_8_bits_col5, xor_col9];
                *lookup_data.verify_bitwise_xor_8_3 = [xor_col8, ms_8_bits_col5, xor_col9];

                *lookup_data.triple_xor_16_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    input_limb_2_col2,
                    ((xor_col7) + ((xor_col9) * (M31_256))),
                ];
                *row[10] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    triple_xor_16_0: Vec<[PackedM31; 4]>,
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
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
        triple_xor_16: &relations::TripleXor16,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
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

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.triple_xor_16_0)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = triple_xor_16.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

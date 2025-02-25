#![allow(unused_parens)]
#![allow(unused_imports)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::verify_bitwise_xor_8;

pub type InputType = [UInt32; 3];
pub type PackedInputType = [PackedUInt32; 3];
const N_TRACE_COLUMNS: usize = 20;

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
        verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) =
            write_trace_simd(n_rows, packed_inputs, verify_bitwise_xor_8_state);

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
        unimplemented!("Implement manually");
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    verify_bitwise_xor_8_state: &verify_bitwise_xor_8::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_256 = PackedM31::broadcast(M31::from(256));
    let UInt16_8 = PackedUInt16::broadcast(UInt16::from(8));

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(|(((row_index, row), triple_xor_32_input), lookup_data)| {
            let input_tmp_298db_0 = [
                triple_xor_32_input[0],
                triple_xor_32_input[1],
                triple_xor_32_input[2],
            ];
            let input_limb_0_col0 = input_tmp_298db_0[0].low().as_m31();
            *row[0] = input_limb_0_col0;
            let input_limb_1_col1 = input_tmp_298db_0[0].high().as_m31();
            *row[1] = input_limb_1_col1;
            let input_limb_2_col2 = input_tmp_298db_0[1].low().as_m31();
            *row[2] = input_limb_2_col2;
            let input_limb_3_col3 = input_tmp_298db_0[1].high().as_m31();
            *row[3] = input_limb_3_col3;
            let input_limb_4_col4 = input_tmp_298db_0[2].low().as_m31();
            *row[4] = input_limb_4_col4;
            let input_limb_5_col5 = input_tmp_298db_0[2].high().as_m31();
            *row[5] = input_limb_5_col5;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_298db_1 = ((input_tmp_298db_0[0].low()) >> (UInt16_8));
            let ms_8_bits_col6 = ms_8_bits_tmp_298db_1.as_m31();
            *row[6] = ms_8_bits_col6;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_298db_2 = ((input_tmp_298db_0[0].high()) >> (UInt16_8));
            let ms_8_bits_col7 = ms_8_bits_tmp_298db_2.as_m31();
            *row[7] = ms_8_bits_col7;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_298db_3 = ((input_tmp_298db_0[1].low()) >> (UInt16_8));
            let ms_8_bits_col8 = ms_8_bits_tmp_298db_3.as_m31();
            *row[8] = ms_8_bits_col8;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_298db_4 = ((input_tmp_298db_0[1].high()) >> (UInt16_8));
            let ms_8_bits_col9 = ms_8_bits_tmp_298db_4.as_m31();
            *row[9] = ms_8_bits_col9;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_298db_5 = ((input_tmp_298db_0[2].low()) >> (UInt16_8));
            let ms_8_bits_col10 = ms_8_bits_tmp_298db_5.as_m31();
            *row[10] = ms_8_bits_col10;

            // Split 16 Low Part Size 8.

            let ms_8_bits_tmp_298db_6 = ((input_tmp_298db_0[2].high()) >> (UInt16_8));
            let ms_8_bits_col11 = ms_8_bits_tmp_298db_6.as_m31();
            *row[11] = ms_8_bits_col11;

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_7 =
                ((UInt16::from_m31(((input_limb_0_col0) - ((ms_8_bits_col6) * (M31_256)))))
                    ^ (UInt16::from_m31(((input_limb_2_col2) - ((ms_8_bits_col8) * (M31_256))))));
            let xor_col12 = xor_tmp_298db_7.as_m31();
            *row[12] = xor_col12;
            let verify_bitwise_xor_8_inputs_0 = [
                ((input_limb_0_col0) - ((ms_8_bits_col6) * (M31_256))),
                ((input_limb_2_col2) - ((ms_8_bits_col8) * (M31_256))),
                xor_col12,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_0 = [
                ((input_limb_0_col0) - ((ms_8_bits_col6) * (M31_256))),
                ((input_limb_2_col2) - ((ms_8_bits_col8) * (M31_256))),
                xor_col12,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_8 = ((UInt16::from_m31(xor_col12))
                ^ (UInt16::from_m31(((input_limb_4_col4) - ((ms_8_bits_col10) * (M31_256))))));
            let xor_col13 = xor_tmp_298db_8.as_m31();
            *row[13] = xor_col13;
            let verify_bitwise_xor_8_inputs_1 = [
                xor_col12,
                ((input_limb_4_col4) - ((ms_8_bits_col10) * (M31_256))),
                xor_col13,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_1 = [
                xor_col12,
                ((input_limb_4_col4) - ((ms_8_bits_col10) * (M31_256))),
                xor_col13,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_9 =
                ((UInt16::from_m31(ms_8_bits_col6)) ^ (UInt16::from_m31(ms_8_bits_col8)));
            let xor_col14 = xor_tmp_298db_9.as_m31();
            *row[14] = xor_col14;
            let verify_bitwise_xor_8_inputs_2 =
                [ms_8_bits_col6, ms_8_bits_col8, xor_col14].unpack();
            *lookup_data.verify_bitwise_xor_8_2 = [ms_8_bits_col6, ms_8_bits_col8, xor_col14];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_10 =
                ((UInt16::from_m31(xor_col14)) ^ (UInt16::from_m31(ms_8_bits_col10)));
            let xor_col15 = xor_tmp_298db_10.as_m31();
            *row[15] = xor_col15;
            let verify_bitwise_xor_8_inputs_3 = [xor_col14, ms_8_bits_col10, xor_col15].unpack();
            *lookup_data.verify_bitwise_xor_8_3 = [xor_col14, ms_8_bits_col10, xor_col15];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_11 =
                ((UInt16::from_m31(((input_limb_1_col1) - ((ms_8_bits_col7) * (M31_256)))))
                    ^ (UInt16::from_m31(((input_limb_3_col3) - ((ms_8_bits_col9) * (M31_256))))));
            let xor_col16 = xor_tmp_298db_11.as_m31();
            *row[16] = xor_col16;
            let verify_bitwise_xor_8_inputs_4 = [
                ((input_limb_1_col1) - ((ms_8_bits_col7) * (M31_256))),
                ((input_limb_3_col3) - ((ms_8_bits_col9) * (M31_256))),
                xor_col16,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_4 = [
                ((input_limb_1_col1) - ((ms_8_bits_col7) * (M31_256))),
                ((input_limb_3_col3) - ((ms_8_bits_col9) * (M31_256))),
                xor_col16,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_12 = ((UInt16::from_m31(xor_col16))
                ^ (UInt16::from_m31(((input_limb_5_col5) - ((ms_8_bits_col11) * (M31_256))))));
            let xor_col17 = xor_tmp_298db_12.as_m31();
            *row[17] = xor_col17;
            let verify_bitwise_xor_8_inputs_5 = [
                xor_col16,
                ((input_limb_5_col5) - ((ms_8_bits_col11) * (M31_256))),
                xor_col17,
            ]
            .unpack();
            *lookup_data.verify_bitwise_xor_8_5 = [
                xor_col16,
                ((input_limb_5_col5) - ((ms_8_bits_col11) * (M31_256))),
                xor_col17,
            ];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_13 =
                ((UInt16::from_m31(ms_8_bits_col7)) ^ (UInt16::from_m31(ms_8_bits_col9)));
            let xor_col18 = xor_tmp_298db_13.as_m31();
            *row[18] = xor_col18;
            let verify_bitwise_xor_8_inputs_6 =
                [ms_8_bits_col7, ms_8_bits_col9, xor_col18].unpack();
            *lookup_data.verify_bitwise_xor_8_6 = [ms_8_bits_col7, ms_8_bits_col9, xor_col18];

            // Bitwise Xor Num Bits 8.

            let xor_tmp_298db_14 =
                ((UInt16::from_m31(xor_col18)) ^ (UInt16::from_m31(ms_8_bits_col11)));
            let xor_col19 = xor_tmp_298db_14.as_m31();
            *row[19] = xor_col19;
            let verify_bitwise_xor_8_inputs_7 = [xor_col18, ms_8_bits_col11, xor_col19].unpack();
            *lookup_data.verify_bitwise_xor_8_7 = [xor_col18, ms_8_bits_col11, xor_col19];

            let triple_xor32_output_tmp_298db_15 = UInt32::from_limbs(
                ((xor_col13) + ((xor_col15) * (M31_256))),
                ((xor_col17) + ((xor_col19) * (M31_256))),
            );
            let triple_xor32_output_tmp_298db_15_limb_0 = ((xor_col13) + ((xor_col15) * (M31_256)));
            let triple_xor32_output_tmp_298db_15_limb_1 = ((xor_col17) + ((xor_col19) * (M31_256)));
            *lookup_data.triple_xor_32_0 = [
                input_limb_0_col0,
                input_limb_1_col1,
                input_limb_2_col2,
                input_limb_3_col3,
                input_limb_4_col4,
                input_limb_5_col5,
                triple_xor32_output_tmp_298db_15_limb_0,
                triple_xor32_output_tmp_298db_15_limb_1,
            ];

            // Add sub-components inputs.
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_0);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_1);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_2);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_3);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_4);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_5);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_6);
            verify_bitwise_xor_8_state.add_inputs(&verify_bitwise_xor_8_inputs_7);
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    triple_xor_32_0: Vec<[PackedM31; 8]>,
    verify_bitwise_xor_8_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_4: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_5: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_6: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_8_7: Vec<[PackedM31; 3]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        triple_xor_32: &relations::TripleXor32,
        verify_bitwise_xor_8: &relations::VerifyBitwiseXor_8,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_0,
            &self.lookup_data.verify_bitwise_xor_8_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_2,
            &self.lookup_data.verify_bitwise_xor_8_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_4,
            &self.lookup_data.verify_bitwise_xor_8_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_8_6,
            &self.lookup_data.verify_bitwise_xor_8_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_8.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_8.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.triple_xor_32_0.iter().enumerate() {
            let denom = triple_xor_32.combine(values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

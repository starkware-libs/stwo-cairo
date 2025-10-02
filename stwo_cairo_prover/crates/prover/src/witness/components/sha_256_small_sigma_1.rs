// AIR version 98896da1
#![allow(unused_parens)]
use cairo_air::components::sha_256_small_sigma_1::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    sha_256_small_sigma_1_o_0, sha_256_small_sigma_1_o_1, verify_bitwise_and_16,
    verify_bitwise_xor_16,
};
use crate::witness::prelude::*;

pub type PackedInputType = PackedUInt32;

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

    pub fn is_empty(&self) -> bool {
        self.packed_inputs.is_empty()
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        sha_256_small_sigma_1_o_0_state: &sha_256_small_sigma_1_o_0::ClaimGenerator,
        sha_256_small_sigma_1_o_1_state: &sha_256_small_sigma_1_o_1::ClaimGenerator,
        verify_bitwise_and_16_state: &verify_bitwise_and_16::ClaimGenerator,
        verify_bitwise_xor_16_state: &verify_bitwise_xor_16::ClaimGenerator,
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
            sha_256_small_sigma_1_o_0_state,
            sha_256_small_sigma_1_o_1_state,
            verify_bitwise_and_16_state,
            verify_bitwise_xor_16_state,
        );
        sub_component_inputs
            .verify_bitwise_and_16
            .iter()
            .for_each(|inputs| {
                verify_bitwise_and_16_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_small_sigma_1_o_0
            .iter()
            .for_each(|inputs| {
                sha_256_small_sigma_1_o_0_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .sha_256_small_sigma_1_o_1
            .iter()
            .for_each(|inputs| {
                sha_256_small_sigma_1_o_1_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .verify_bitwise_xor_16
            .iter()
            .for_each(|inputs| {
                verify_bitwise_xor_16_state.add_packed_inputs(inputs);
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
    verify_bitwise_and_16: [Vec<verify_bitwise_and_16::PackedInputType>; 6],
    sha_256_small_sigma_1_o_0: [Vec<sha_256_small_sigma_1_o_0::PackedInputType>; 1],
    sha_256_small_sigma_1_o_1: [Vec<sha_256_small_sigma_1_o_1::PackedInputType>; 1],
    verify_bitwise_xor_16: [Vec<verify_bitwise_xor_16::PackedInputType>; 2],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    n_rows: usize,
    sha_256_small_sigma_1_o_0_state: &sha_256_small_sigma_1_o_0::ClaimGenerator,
    sha_256_small_sigma_1_o_1_state: &sha_256_small_sigma_1_o_1::ClaimGenerator,
    verify_bitwise_and_16_state: &verify_bitwise_and_16::ClaimGenerator,
    verify_bitwise_xor_16_state: &verify_bitwise_xor_16::ClaimGenerator,
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

    let M31_122 = PackedM31::broadcast(M31::from(122));
    let M31_17029 = PackedM31::broadcast(M31::from(17029));
    let M31_19109 = PackedM31::broadcast(M31::from(19109));
    let M31_346 = PackedM31::broadcast(M31::from(346));
    let M31_46080 = PackedM31::broadcast(M31::from(46080));
    let M31_48384 = PackedM31::broadcast(M31::from(48384));
    let UInt16_122 = PackedUInt16::broadcast(UInt16::from(122));
    let UInt16_17029 = PackedUInt16::broadcast(UInt16::from(17029));
    let UInt16_19109 = PackedUInt16::broadcast(UInt16::from(19109));
    let UInt16_21161 = PackedUInt16::broadcast(UInt16::from(21161));
    let UInt16_22189 = PackedUInt16::broadcast(UInt16::from(22189));
    let UInt16_2322 = PackedUInt16::broadcast(UInt16::from(2322));
    let UInt16_3078 = PackedUInt16::broadcast(UInt16::from(3078));
    let UInt16_346 = PackedUInt16::broadcast(UInt16::from(346));
    let UInt16_41024 = PackedUInt16::broadcast(UInt16::from(41024));
    let UInt16_41296 = PackedUInt16::broadcast(UInt16::from(41296));
    let UInt16_46080 = PackedUInt16::broadcast(UInt16::from(46080));
    let UInt16_48384 = PackedUInt16::broadcast(UInt16::from(48384));
    let UInt32_10 = PackedUInt32::broadcast(UInt32::from(10));
    let UInt32_13 = PackedUInt32::broadcast(UInt32::from(13));
    let UInt32_15 = PackedUInt32::broadcast(UInt32::from(15));
    let UInt32_17 = PackedUInt32::broadcast(UInt32::from(17));
    let UInt32_19 = PackedUInt32::broadcast(UInt32::from(19));
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
            |(
                row_index,
                (mut row, lookup_data, sub_component_inputs, sha_256_small_sigma_1_input),
            )| {
                let input_limb_0_col0 = sha_256_small_sigma_1_input.low().as_m31();
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = sha_256_small_sigma_1_input.high().as_m31();
                *row[1] = input_limb_1_col1;

                // Bitwise And Num Bits 16.

                let and_tmp_2ae59_0 =
                    ((PackedUInt16::from_m31(input_limb_0_col0)) & (UInt16_17029));
                let and_col2 = and_tmp_2ae59_0.as_m31();
                *row[2] = and_col2;
                *sub_component_inputs.verify_bitwise_and_16[0] =
                    [input_limb_0_col0, M31_17029, and_col2];
                *lookup_data.verify_bitwise_and_16_0 = [input_limb_0_col0, M31_17029, and_col2];

                // Bitwise And Num Bits 16.

                let and_tmp_2ae59_2 = ((PackedUInt16::from_m31(input_limb_0_col0)) & (UInt16_122));
                let and_col3 = and_tmp_2ae59_2.as_m31();
                *row[3] = and_col3;
                *sub_component_inputs.verify_bitwise_and_16[1] =
                    [input_limb_0_col0, M31_122, and_col3];
                *lookup_data.verify_bitwise_and_16_1 = [input_limb_0_col0, M31_122, and_col3];

                // Bitwise And Num Bits 16.

                let and_tmp_2ae59_4 =
                    ((PackedUInt16::from_m31(input_limb_0_col0)) & (UInt16_48384));
                let and_col4 = and_tmp_2ae59_4.as_m31();
                *row[4] = and_col4;
                *sub_component_inputs.verify_bitwise_and_16[2] =
                    [input_limb_0_col0, M31_48384, and_col4];
                *lookup_data.verify_bitwise_and_16_2 = [input_limb_0_col0, M31_48384, and_col4];

                // Bitwise And Num Bits 16.

                let and_tmp_2ae59_6 =
                    ((PackedUInt16::from_m31(input_limb_1_col1)) & (UInt16_19109));
                let and_col5 = and_tmp_2ae59_6.as_m31();
                *row[5] = and_col5;
                *sub_component_inputs.verify_bitwise_and_16[3] =
                    [input_limb_1_col1, M31_19109, and_col5];
                *lookup_data.verify_bitwise_and_16_3 = [input_limb_1_col1, M31_19109, and_col5];

                // Bitwise And Num Bits 16.

                let and_tmp_2ae59_8 = ((PackedUInt16::from_m31(input_limb_1_col1)) & (UInt16_346));
                let and_col6 = and_tmp_2ae59_8.as_m31();
                *row[6] = and_col6;
                *sub_component_inputs.verify_bitwise_and_16[4] =
                    [input_limb_1_col1, M31_346, and_col6];
                *lookup_data.verify_bitwise_and_16_4 = [input_limb_1_col1, M31_346, and_col6];

                // Bitwise And Num Bits 16.

                let and_tmp_2ae59_10 =
                    ((PackedUInt16::from_m31(input_limb_1_col1)) & (UInt16_46080));
                let and_col7 = and_tmp_2ae59_10.as_m31();
                *row[7] = and_col7;
                *sub_component_inputs.verify_bitwise_and_16[5] =
                    [input_limb_1_col1, M31_46080, and_col7];
                *lookup_data.verify_bitwise_and_16_5 = [input_limb_1_col1, M31_46080, and_col7];

                let sigma_rotated0_tmp_2ae59_12 =
                    (((((PackedUInt32::from_limbs([and_col2, and_col5])) >> (UInt32_17))
                        | ((PackedUInt32::from_limbs([and_col2, and_col5])) << (UInt32_15)))
                        ^ (((PackedUInt32::from_limbs([and_col2, and_col5])) >> (UInt32_19))
                            | ((PackedUInt32::from_limbs([and_col2, and_col5])) << (UInt32_13))))
                        ^ ((PackedUInt32::from_limbs([and_col2, and_col5])) >> (UInt32_10)));
                let sigma_rotated1_tmp_2ae59_13 = (((((PackedUInt32::from_limbs([
                    ((and_col3) + (and_col4)),
                    ((and_col6) + (and_col7)),
                ])) >> (UInt32_17))
                    | ((PackedUInt32::from_limbs([
                        ((and_col3) + (and_col4)),
                        ((and_col6) + (and_col7)),
                    ])) << (UInt32_15)))
                    ^ (((PackedUInt32::from_limbs([
                        ((and_col3) + (and_col4)),
                        ((and_col6) + (and_col7)),
                    ])) >> (UInt32_19))
                        | ((PackedUInt32::from_limbs([
                            ((and_col3) + (and_col4)),
                            ((and_col6) + (and_col7)),
                        ])) << (UInt32_13))))
                    ^ ((PackedUInt32::from_limbs([
                        ((and_col3) + (and_col4)),
                        ((and_col6) + (and_col7)),
                    ])) >> (UInt32_10)));
                let sigma_O0_L_tmp_2ae59_14 =
                    ((sigma_rotated0_tmp_2ae59_12.low()) & (UInt16_41296));
                let sigma_O0_L_col8 = sigma_O0_L_tmp_2ae59_14.as_m31();
                *row[8] = sigma_O0_L_col8;
                let sigma_O0_H_tmp_2ae59_15 =
                    ((sigma_rotated0_tmp_2ae59_12.high()) & (UInt16_41024));
                let sigma_O0_H_col9 = sigma_O0_H_tmp_2ae59_15.as_m31();
                *row[9] = sigma_O0_H_col9;
                let sigma_O1_L_tmp_2ae59_16 =
                    ((sigma_rotated1_tmp_2ae59_13.low()) & (UInt16_21161));
                let sigma_O1_L_col10 = sigma_O1_L_tmp_2ae59_16.as_m31();
                *row[10] = sigma_O1_L_col10;
                let sigma_O1_H_tmp_2ae59_17 =
                    ((sigma_rotated1_tmp_2ae59_13.high()) & (UInt16_22189));
                let sigma_O1_H_col11 = sigma_O1_H_tmp_2ae59_17.as_m31();
                *row[11] = sigma_O1_H_col11;
                let sigma_O2_L_tmp_2ae59_18 = ((sigma_rotated0_tmp_2ae59_12.low()) & (UInt16_3078));
                let sigma_O2_L_col12 = sigma_O2_L_tmp_2ae59_18.as_m31();
                *row[12] = sigma_O2_L_col12;
                let sigma_O2_H_tmp_2ae59_19 =
                    ((sigma_rotated0_tmp_2ae59_12.high()) & (UInt16_2322));
                let sigma_O2_H_col13 = sigma_O2_H_tmp_2ae59_19.as_m31();
                *row[13] = sigma_O2_H_col13;
                let sigma_O2_prime_L_tmp_2ae59_20 =
                    ((sigma_rotated1_tmp_2ae59_13.low()) & (UInt16_3078));
                let sigma_O2_prime_L_col14 = sigma_O2_prime_L_tmp_2ae59_20.as_m31();
                *row[14] = sigma_O2_prime_L_col14;
                let sigma_O2_prime_H_tmp_2ae59_21 =
                    ((sigma_rotated1_tmp_2ae59_13.high()) & (UInt16_2322));
                let sigma_O2_prime_H_col15 = sigma_O2_prime_H_tmp_2ae59_21.as_m31();
                *row[15] = sigma_O2_prime_H_col15;
                *sub_component_inputs.sha_256_small_sigma_1_o_0[0] = [
                    and_col2,
                    and_col5,
                    sigma_O0_L_col8,
                    sigma_O0_H_col9,
                    sigma_O2_L_col12,
                    sigma_O2_H_col13,
                ];
                *lookup_data.sha_256_small_sigma_1_o_0_0 = [
                    and_col2,
                    and_col5,
                    sigma_O0_L_col8,
                    sigma_O0_H_col9,
                    sigma_O2_L_col12,
                    sigma_O2_H_col13,
                ];
                *sub_component_inputs.sha_256_small_sigma_1_o_1[0] = [
                    ((and_col3) + (and_col4)),
                    ((and_col6) + (and_col7)),
                    sigma_O1_L_col10,
                    sigma_O1_H_col11,
                    sigma_O2_prime_L_col14,
                    sigma_O2_prime_H_col15,
                ];
                *lookup_data.sha_256_small_sigma_1_o_1_0 = [
                    ((and_col3) + (and_col4)),
                    ((and_col6) + (and_col7)),
                    sigma_O1_L_col10,
                    sigma_O1_H_col11,
                    sigma_O2_prime_L_col14,
                    sigma_O2_prime_H_col15,
                ];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_2ae59_22 = ((PackedUInt16::from_m31(sigma_O2_prime_L_col14))
                    ^ (PackedUInt16::from_m31(sigma_O2_L_col12)));
                let xor_col16 = xor_tmp_2ae59_22.as_m31();
                *row[16] = xor_col16;
                *sub_component_inputs.verify_bitwise_xor_16[0] =
                    [sigma_O2_prime_L_col14, sigma_O2_L_col12, xor_col16];
                *lookup_data.verify_bitwise_xor_16_0 =
                    [sigma_O2_prime_L_col14, sigma_O2_L_col12, xor_col16];

                // Bitwise Xor Num Bits 16.

                let xor_tmp_2ae59_24 = ((PackedUInt16::from_m31(sigma_O2_prime_H_col15))
                    ^ (PackedUInt16::from_m31(sigma_O2_H_col13)));
                let xor_col17 = xor_tmp_2ae59_24.as_m31();
                *row[17] = xor_col17;
                *sub_component_inputs.verify_bitwise_xor_16[1] =
                    [sigma_O2_prime_H_col15, sigma_O2_H_col13, xor_col17];
                *lookup_data.verify_bitwise_xor_16_1 =
                    [sigma_O2_prime_H_col15, sigma_O2_H_col13, xor_col17];

                let output_low_tmp_2ae59_26 = (((sigma_O0_L_tmp_2ae59_14)
                    + (sigma_O1_L_tmp_2ae59_16))
                    + (PackedUInt16::from_m31(xor_col16)));
                let output_low_col18 = output_low_tmp_2ae59_26.as_m31();
                *row[18] = output_low_col18;
                let output_high_tmp_2ae59_27 = (((sigma_O0_H_tmp_2ae59_15)
                    + (sigma_O1_H_tmp_2ae59_17))
                    + (PackedUInt16::from_m31(xor_col17)));
                let output_high_col19 = output_high_tmp_2ae59_27.as_m31();
                *row[19] = output_high_col19;
                *lookup_data.sha_256_small_sigma_1_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    output_low_col18,
                    output_high_col19,
                ];
                *row[20] = enabler_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    sha_256_small_sigma_1_0: Vec<[PackedM31; 4]>,
    sha_256_small_sigma_1_o_0_0: Vec<[PackedM31; 6]>,
    sha_256_small_sigma_1_o_1_0: Vec<[PackedM31; 6]>,
    verify_bitwise_and_16_0: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_1: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_2: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_3: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_4: Vec<[PackedM31; 3]>,
    verify_bitwise_and_16_5: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_16_1: Vec<[PackedM31; 3]>,
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
        verify_bitwise_and_16: &relations::VerifyBitwiseAnd_16,
        sha_256_small_sigma_1_o_0: &relations::Sha256SmallSigma1O0,
        sha_256_small_sigma_1_o_1: &relations::Sha256SmallSigma1O1,
        verify_bitwise_xor_16: &relations::VerifyBitwiseXor_16,
        sha_256_small_sigma_1: &relations::Sha256SmallSigma1,
    ) -> InteractionClaim {
        let enabler_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_0,
            &self.lookup_data.verify_bitwise_and_16_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_2,
            &self.lookup_data.verify_bitwise_and_16_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_and_16_4,
            &self.lookup_data.verify_bitwise_and_16_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_and_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_and_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_small_sigma_1_o_0_0,
            &self.lookup_data.sha_256_small_sigma_1_o_1_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = sha_256_small_sigma_1_o_0.combine(values0);
                let denom1: PackedQM31 = sha_256_small_sigma_1_o_1.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_16_0,
            &self.lookup_data.verify_bitwise_xor_16_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = verify_bitwise_xor_16.combine(values0);
                let denom1: PackedQM31 = verify_bitwise_xor_16.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.sha_256_small_sigma_1_0,
        )
            .into_par_iter()
            .enumerate()
            .for_each(|(i, (writer, values))| {
                let denom = sha_256_small_sigma_1.combine(values);
                writer.write_frac(-PackedQM31::one() * enabler_col.packed_at(i), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

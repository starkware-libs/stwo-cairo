// AIR version 97774321-dirty
#![allow(unused_parens)]
use cairo_air::components::create_blake_output::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::triple_xor_32;
use crate::witness::prelude::*;

pub type InputType = ([UInt32; 8], [UInt32; 16]);
pub type PackedInputType = ([PackedUInt32; 8], [PackedUInt32; 16]);

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
        triple_xor_32_state: &triple_xor_32::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) =
            write_trace_simd(packed_inputs, triple_xor_32_state);
        sub_component_inputs
            .triple_xor_32
            .iter()
            .for_each(|inputs| {
                triple_xor_32_state.add_packed_inputs(inputs);
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
    triple_xor_32: [Vec<triple_xor_32::PackedInputType>; 8],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    triple_xor_32_state: &triple_xor_32::ClaimGenerator,
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
                (mut row, lookup_data, sub_component_inputs, create_blake_output_input),
            )| {
                *sub_component_inputs.triple_xor_32[0] = [
                    create_blake_output_input.1[0],
                    create_blake_output_input.1[8],
                    create_blake_output_input.0[0],
                ];
                let triple_xor_32_output_tmp_c84e6_0 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[0],
                    create_blake_output_input.1[8],
                    create_blake_output_input.0[0],
                ]);
                let triple_xor_32_output_limb_0_col0 =
                    triple_xor_32_output_tmp_c84e6_0.low().as_m31();
                *row[0] = triple_xor_32_output_limb_0_col0;
                let triple_xor_32_output_limb_1_col1 =
                    triple_xor_32_output_tmp_c84e6_0.high().as_m31();
                *row[1] = triple_xor_32_output_limb_1_col1;
                *lookup_data.triple_xor_32_0 = [
                    create_blake_output_input.1[0].low().as_m31(),
                    create_blake_output_input.1[0].high().as_m31(),
                    create_blake_output_input.1[8].low().as_m31(),
                    create_blake_output_input.1[8].high().as_m31(),
                    create_blake_output_input.0[0].low().as_m31(),
                    create_blake_output_input.0[0].high().as_m31(),
                    triple_xor_32_output_limb_0_col0,
                    triple_xor_32_output_limb_1_col1,
                ];
                *sub_component_inputs.triple_xor_32[1] = [
                    create_blake_output_input.1[1],
                    create_blake_output_input.1[9],
                    create_blake_output_input.0[1],
                ];
                let triple_xor_32_output_tmp_c84e6_1 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[1],
                    create_blake_output_input.1[9],
                    create_blake_output_input.0[1],
                ]);
                let triple_xor_32_output_limb_0_col2 =
                    triple_xor_32_output_tmp_c84e6_1.low().as_m31();
                *row[2] = triple_xor_32_output_limb_0_col2;
                let triple_xor_32_output_limb_1_col3 =
                    triple_xor_32_output_tmp_c84e6_1.high().as_m31();
                *row[3] = triple_xor_32_output_limb_1_col3;
                *lookup_data.triple_xor_32_1 = [
                    create_blake_output_input.1[1].low().as_m31(),
                    create_blake_output_input.1[1].high().as_m31(),
                    create_blake_output_input.1[9].low().as_m31(),
                    create_blake_output_input.1[9].high().as_m31(),
                    create_blake_output_input.0[1].low().as_m31(),
                    create_blake_output_input.0[1].high().as_m31(),
                    triple_xor_32_output_limb_0_col2,
                    triple_xor_32_output_limb_1_col3,
                ];
                *sub_component_inputs.triple_xor_32[2] = [
                    create_blake_output_input.1[2],
                    create_blake_output_input.1[10],
                    create_blake_output_input.0[2],
                ];
                let triple_xor_32_output_tmp_c84e6_2 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[2],
                    create_blake_output_input.1[10],
                    create_blake_output_input.0[2],
                ]);
                let triple_xor_32_output_limb_0_col4 =
                    triple_xor_32_output_tmp_c84e6_2.low().as_m31();
                *row[4] = triple_xor_32_output_limb_0_col4;
                let triple_xor_32_output_limb_1_col5 =
                    triple_xor_32_output_tmp_c84e6_2.high().as_m31();
                *row[5] = triple_xor_32_output_limb_1_col5;
                *lookup_data.triple_xor_32_2 = [
                    create_blake_output_input.1[2].low().as_m31(),
                    create_blake_output_input.1[2].high().as_m31(),
                    create_blake_output_input.1[10].low().as_m31(),
                    create_blake_output_input.1[10].high().as_m31(),
                    create_blake_output_input.0[2].low().as_m31(),
                    create_blake_output_input.0[2].high().as_m31(),
                    triple_xor_32_output_limb_0_col4,
                    triple_xor_32_output_limb_1_col5,
                ];
                *sub_component_inputs.triple_xor_32[3] = [
                    create_blake_output_input.1[3],
                    create_blake_output_input.1[11],
                    create_blake_output_input.0[3],
                ];
                let triple_xor_32_output_tmp_c84e6_3 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[3],
                    create_blake_output_input.1[11],
                    create_blake_output_input.0[3],
                ]);
                let triple_xor_32_output_limb_0_col6 =
                    triple_xor_32_output_tmp_c84e6_3.low().as_m31();
                *row[6] = triple_xor_32_output_limb_0_col6;
                let triple_xor_32_output_limb_1_col7 =
                    triple_xor_32_output_tmp_c84e6_3.high().as_m31();
                *row[7] = triple_xor_32_output_limb_1_col7;
                *lookup_data.triple_xor_32_3 = [
                    create_blake_output_input.1[3].low().as_m31(),
                    create_blake_output_input.1[3].high().as_m31(),
                    create_blake_output_input.1[11].low().as_m31(),
                    create_blake_output_input.1[11].high().as_m31(),
                    create_blake_output_input.0[3].low().as_m31(),
                    create_blake_output_input.0[3].high().as_m31(),
                    triple_xor_32_output_limb_0_col6,
                    triple_xor_32_output_limb_1_col7,
                ];
                *sub_component_inputs.triple_xor_32[4] = [
                    create_blake_output_input.1[4],
                    create_blake_output_input.1[12],
                    create_blake_output_input.0[4],
                ];
                let triple_xor_32_output_tmp_c84e6_4 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[4],
                    create_blake_output_input.1[12],
                    create_blake_output_input.0[4],
                ]);
                let triple_xor_32_output_limb_0_col8 =
                    triple_xor_32_output_tmp_c84e6_4.low().as_m31();
                *row[8] = triple_xor_32_output_limb_0_col8;
                let triple_xor_32_output_limb_1_col9 =
                    triple_xor_32_output_tmp_c84e6_4.high().as_m31();
                *row[9] = triple_xor_32_output_limb_1_col9;
                *lookup_data.triple_xor_32_4 = [
                    create_blake_output_input.1[4].low().as_m31(),
                    create_blake_output_input.1[4].high().as_m31(),
                    create_blake_output_input.1[12].low().as_m31(),
                    create_blake_output_input.1[12].high().as_m31(),
                    create_blake_output_input.0[4].low().as_m31(),
                    create_blake_output_input.0[4].high().as_m31(),
                    triple_xor_32_output_limb_0_col8,
                    triple_xor_32_output_limb_1_col9,
                ];
                *sub_component_inputs.triple_xor_32[5] = [
                    create_blake_output_input.1[5],
                    create_blake_output_input.1[13],
                    create_blake_output_input.0[5],
                ];
                let triple_xor_32_output_tmp_c84e6_5 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[5],
                    create_blake_output_input.1[13],
                    create_blake_output_input.0[5],
                ]);
                let triple_xor_32_output_limb_0_col10 =
                    triple_xor_32_output_tmp_c84e6_5.low().as_m31();
                *row[10] = triple_xor_32_output_limb_0_col10;
                let triple_xor_32_output_limb_1_col11 =
                    triple_xor_32_output_tmp_c84e6_5.high().as_m31();
                *row[11] = triple_xor_32_output_limb_1_col11;
                *lookup_data.triple_xor_32_5 = [
                    create_blake_output_input.1[5].low().as_m31(),
                    create_blake_output_input.1[5].high().as_m31(),
                    create_blake_output_input.1[13].low().as_m31(),
                    create_blake_output_input.1[13].high().as_m31(),
                    create_blake_output_input.0[5].low().as_m31(),
                    create_blake_output_input.0[5].high().as_m31(),
                    triple_xor_32_output_limb_0_col10,
                    triple_xor_32_output_limb_1_col11,
                ];
                *sub_component_inputs.triple_xor_32[6] = [
                    create_blake_output_input.1[6],
                    create_blake_output_input.1[14],
                    create_blake_output_input.0[6],
                ];
                let triple_xor_32_output_tmp_c84e6_6 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[6],
                    create_blake_output_input.1[14],
                    create_blake_output_input.0[6],
                ]);
                let triple_xor_32_output_limb_0_col12 =
                    triple_xor_32_output_tmp_c84e6_6.low().as_m31();
                *row[12] = triple_xor_32_output_limb_0_col12;
                let triple_xor_32_output_limb_1_col13 =
                    triple_xor_32_output_tmp_c84e6_6.high().as_m31();
                *row[13] = triple_xor_32_output_limb_1_col13;
                *lookup_data.triple_xor_32_6 = [
                    create_blake_output_input.1[6].low().as_m31(),
                    create_blake_output_input.1[6].high().as_m31(),
                    create_blake_output_input.1[14].low().as_m31(),
                    create_blake_output_input.1[14].high().as_m31(),
                    create_blake_output_input.0[6].low().as_m31(),
                    create_blake_output_input.0[6].high().as_m31(),
                    triple_xor_32_output_limb_0_col12,
                    triple_xor_32_output_limb_1_col13,
                ];
                *sub_component_inputs.triple_xor_32[7] = [
                    create_blake_output_input.1[7],
                    create_blake_output_input.1[15],
                    create_blake_output_input.0[7],
                ];
                let triple_xor_32_output_tmp_c84e6_7 = PackedTripleXor32::deduce_output([
                    create_blake_output_input.1[7],
                    create_blake_output_input.1[15],
                    create_blake_output_input.0[7],
                ]);
                let triple_xor_32_output_limb_0_col14 =
                    triple_xor_32_output_tmp_c84e6_7.low().as_m31();
                *row[14] = triple_xor_32_output_limb_0_col14;
                let triple_xor_32_output_limb_1_col15 =
                    triple_xor_32_output_tmp_c84e6_7.high().as_m31();
                *row[15] = triple_xor_32_output_limb_1_col15;
                *lookup_data.triple_xor_32_7 = [
                    create_blake_output_input.1[7].low().as_m31(),
                    create_blake_output_input.1[7].high().as_m31(),
                    create_blake_output_input.1[15].low().as_m31(),
                    create_blake_output_input.1[15].high().as_m31(),
                    create_blake_output_input.0[7].low().as_m31(),
                    create_blake_output_input.0[7].high().as_m31(),
                    triple_xor_32_output_limb_0_col14,
                    triple_xor_32_output_limb_1_col15,
                ];
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    triple_xor_32_0: Vec<[PackedM31; 8]>,
    triple_xor_32_1: Vec<[PackedM31; 8]>,
    triple_xor_32_2: Vec<[PackedM31; 8]>,
    triple_xor_32_3: Vec<[PackedM31; 8]>,
    triple_xor_32_4: Vec<[PackedM31; 8]>,
    triple_xor_32_5: Vec<[PackedM31; 8]>,
    triple_xor_32_6: Vec<[PackedM31; 8]>,
    triple_xor_32_7: Vec<[PackedM31; 8]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        triple_xor_32: &relations::TripleXor32,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_0,
            &self.lookup_data.triple_xor_32_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_2,
            &self.lookup_data.triple_xor_32_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_4,
            &self.lookup_data.triple_xor_32_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.triple_xor_32_6,
            &self.lookup_data.triple_xor_32_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = triple_xor_32.combine(values0);
                let denom1: PackedQM31 = triple_xor_32.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

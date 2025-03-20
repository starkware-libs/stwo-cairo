#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim, N_TRACE_COLUMNS};
use crate::components::prelude::proving::*;
use crate::components::range_check_vector::{range_check_18, range_check_9_9};

pub type InputType = Felt252Width27;
pub type PackedInputType = PackedFelt252Width27;

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

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_18_state: &range_check_18::ClaimGenerator,
        range_check_9_9_state: &range_check_9_9::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        assert!(!self.packed_inputs.is_empty());
        let n_vec_rows = self.packed_inputs.len();
        let n_rows = n_vec_rows * N_LANES;
        let packed_size = n_vec_rows.next_power_of_two();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        self.packed_inputs
            .resize(packed_size, *self.packed_inputs.first().unwrap());

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            n_rows,
            self.packed_inputs,
            range_check_18_state,
            range_check_9_9_state,
        );
        sub_component_inputs
            .range_check_9_9
            .iter()
            .for_each(|inputs| {
                range_check_9_9_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_18
            .iter()
            .for_each(|inputs| {
                range_check_18_state.add_packed_inputs(inputs);
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
    range_check_9_9: [Vec<range_check_9_9::PackedInputType>; 5],
    range_check_18: [Vec<range_check_18::PackedInputType>; 9],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    range_check_18_state: &range_check_18::ClaimGenerator,
    range_check_9_9_state: &range_check_9_9::ClaimGenerator,
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

    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_4194304 = PackedM31::broadcast(M31::from(4194304));
    let padding_col = Enabler::new(n_rows);

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
                (mut row, lookup_data, sub_component_inputs, range_check_felt_252_width_27_input),
            )| {
                let input_limb_0_col0 = range_check_felt_252_width_27_input.get_m31(0);
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = range_check_felt_252_width_27_input.get_m31(1);
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = range_check_felt_252_width_27_input.get_m31(2);
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = range_check_felt_252_width_27_input.get_m31(3);
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = range_check_felt_252_width_27_input.get_m31(4);
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = range_check_felt_252_width_27_input.get_m31(5);
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = range_check_felt_252_width_27_input.get_m31(6);
                *row[6] = input_limb_6_col6;
                let input_limb_7_col7 = range_check_felt_252_width_27_input.get_m31(7);
                *row[7] = input_limb_7_col7;
                let input_limb_8_col8 = range_check_felt_252_width_27_input.get_m31(8);
                *row[8] = input_limb_8_col8;
                let input_limb_9_col9 = range_check_felt_252_width_27_input.get_m31(9);
                *row[9] = input_limb_9_col9;
                let input_as_felt252_tmp_2405e_0 =
                    PackedFelt252::from_packed_felt252width27(range_check_felt_252_width_27_input);
                let limb_0_high_part_col10 = input_as_felt252_tmp_2405e_0.get_m31(2);
                *row[10] = limb_0_high_part_col10;
                let limb_1_low_part_col11 = input_as_felt252_tmp_2405e_0.get_m31(3);
                *row[11] = limb_1_low_part_col11;
                *sub_component_inputs.range_check_9_9[0] =
                    [limb_0_high_part_col10, limb_1_low_part_col11];
                *lookup_data.range_check_9_9_0 = [limb_0_high_part_col10, limb_1_low_part_col11];
                *sub_component_inputs.range_check_18[0] =
                    [((input_limb_0_col0) - ((limb_0_high_part_col10) * (M31_262144)))];
                *lookup_data.range_check_18_0 =
                    [((input_limb_0_col0) - ((limb_0_high_part_col10) * (M31_262144)))];
                *sub_component_inputs.range_check_18[1] =
                    [(((input_limb_1_col1) - (limb_1_low_part_col11)) * (M31_4194304))];
                *lookup_data.range_check_18_1 =
                    [(((input_limb_1_col1) - (limb_1_low_part_col11)) * (M31_4194304))];
                let limb_2_high_part_col12 = input_as_felt252_tmp_2405e_0.get_m31(8);
                *row[12] = limb_2_high_part_col12;
                let limb_3_low_part_col13 = input_as_felt252_tmp_2405e_0.get_m31(9);
                *row[13] = limb_3_low_part_col13;
                *sub_component_inputs.range_check_9_9[1] =
                    [limb_2_high_part_col12, limb_3_low_part_col13];
                *lookup_data.range_check_9_9_1 = [limb_2_high_part_col12, limb_3_low_part_col13];
                *sub_component_inputs.range_check_18[2] =
                    [((input_limb_2_col2) - ((limb_2_high_part_col12) * (M31_262144)))];
                *lookup_data.range_check_18_2 =
                    [((input_limb_2_col2) - ((limb_2_high_part_col12) * (M31_262144)))];
                *sub_component_inputs.range_check_18[3] =
                    [(((input_limb_3_col3) - (limb_3_low_part_col13)) * (M31_4194304))];
                *lookup_data.range_check_18_3 =
                    [(((input_limb_3_col3) - (limb_3_low_part_col13)) * (M31_4194304))];
                let limb_4_high_part_col14 = input_as_felt252_tmp_2405e_0.get_m31(14);
                *row[14] = limb_4_high_part_col14;
                let limb_5_low_part_col15 = input_as_felt252_tmp_2405e_0.get_m31(15);
                *row[15] = limb_5_low_part_col15;
                *sub_component_inputs.range_check_9_9[2] =
                    [limb_4_high_part_col14, limb_5_low_part_col15];
                *lookup_data.range_check_9_9_2 = [limb_4_high_part_col14, limb_5_low_part_col15];
                *sub_component_inputs.range_check_18[4] =
                    [((input_limb_4_col4) - ((limb_4_high_part_col14) * (M31_262144)))];
                *lookup_data.range_check_18_4 =
                    [((input_limb_4_col4) - ((limb_4_high_part_col14) * (M31_262144)))];
                *sub_component_inputs.range_check_18[5] =
                    [(((input_limb_5_col5) - (limb_5_low_part_col15)) * (M31_4194304))];
                *lookup_data.range_check_18_5 =
                    [(((input_limb_5_col5) - (limb_5_low_part_col15)) * (M31_4194304))];
                let limb_6_high_part_col16 = input_as_felt252_tmp_2405e_0.get_m31(20);
                *row[16] = limb_6_high_part_col16;
                let limb_7_low_part_col17 = input_as_felt252_tmp_2405e_0.get_m31(21);
                *row[17] = limb_7_low_part_col17;
                *sub_component_inputs.range_check_9_9[3] =
                    [limb_6_high_part_col16, limb_7_low_part_col17];
                *lookup_data.range_check_9_9_3 = [limb_6_high_part_col16, limb_7_low_part_col17];
                *sub_component_inputs.range_check_18[6] =
                    [((input_limb_6_col6) - ((limb_6_high_part_col16) * (M31_262144)))];
                *lookup_data.range_check_18_6 =
                    [((input_limb_6_col6) - ((limb_6_high_part_col16) * (M31_262144)))];
                *sub_component_inputs.range_check_18[7] =
                    [(((input_limb_7_col7) - (limb_7_low_part_col17)) * (M31_4194304))];
                *lookup_data.range_check_18_7 =
                    [(((input_limb_7_col7) - (limb_7_low_part_col17)) * (M31_4194304))];
                let limb_8_high_part_col18 = input_as_felt252_tmp_2405e_0.get_m31(26);
                *row[18] = limb_8_high_part_col18;
                *sub_component_inputs.range_check_9_9[4] =
                    [limb_8_high_part_col18, input_limb_9_col9];
                *lookup_data.range_check_9_9_4 = [limb_8_high_part_col18, input_limb_9_col9];
                *sub_component_inputs.range_check_18[8] =
                    [((input_limb_8_col8) - ((limb_8_high_part_col18) * (M31_262144)))];
                *lookup_data.range_check_18_8 =
                    [((input_limb_8_col8) - ((limb_8_high_part_col18) * (M31_262144)))];
                *lookup_data.range_check_felt_252_width_27_0 = [
                    input_limb_0_col0,
                    input_limb_1_col1,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_6_col6,
                    input_limb_7_col7,
                    input_limb_8_col8,
                    input_limb_9_col9,
                ];
                *row[19] = padding_col.packed_at(row_index);
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_felt_252_width_27_0: Vec<[PackedM31; 10]>,
    range_check_18_0: Vec<[PackedM31; 1]>,
    range_check_18_1: Vec<[PackedM31; 1]>,
    range_check_18_2: Vec<[PackedM31; 1]>,
    range_check_18_3: Vec<[PackedM31; 1]>,
    range_check_18_4: Vec<[PackedM31; 1]>,
    range_check_18_5: Vec<[PackedM31; 1]>,
    range_check_18_6: Vec<[PackedM31; 1]>,
    range_check_18_7: Vec<[PackedM31; 1]>,
    range_check_18_8: Vec<[PackedM31; 1]>,
    range_check_9_9_0: Vec<[PackedM31; 2]>,
    range_check_9_9_1: Vec<[PackedM31; 2]>,
    range_check_9_9_2: Vec<[PackedM31; 2]>,
    range_check_9_9_3: Vec<[PackedM31; 2]>,
    range_check_9_9_4: Vec<[PackedM31; 2]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_felt_252_width_27: &relations::RangeCheckFelt252Width27,
        range_check_18: &relations::RangeCheck_18,
        range_check_9_9: &relations::RangeCheck_9_9,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_0,
            &self.lookup_data.range_check_18_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_18.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_18_1,
            &self.lookup_data.range_check_9_9_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_18.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_18_2,
            &self.lookup_data.range_check_18_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_18.combine(values0);
            let denom1: PackedQM31 = range_check_18.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_2,
            &self.lookup_data.range_check_18_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_18.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_18_5,
            &self.lookup_data.range_check_9_9_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_18.combine(values0);
            let denom1: PackedQM31 = range_check_9_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_18_6,
            &self.lookup_data.range_check_18_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_18.combine(values0);
            let denom1: PackedQM31 = range_check_18.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_9_9_4,
            &self.lookup_data.range_check_18_8,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_9_9.combine(values0);
            let denom1: PackedQM31 = range_check_18.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self
            .lookup_data
            .range_check_felt_252_width_27_0
            .iter()
            .enumerate()
        {
            let denom = range_check_felt_252_width_27.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * padding_col.packed_at(i), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

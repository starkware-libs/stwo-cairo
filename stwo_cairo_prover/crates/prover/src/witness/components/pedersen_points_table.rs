#![allow(unused_parens)]
#![allow(dead_code)]
use cairo_air::components::pedersen_points_table::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};
use stwo_cairo_common::preprocessed_consts::pedersen::PEDERSEN_TABLE_N_ROWS;

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: AtomicMultiplicityColumn::new(PEDERSEN_TABLE_N_ROWS),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self.mults.into_simd_vec();

        let (trace, lookup_data) = write_trace_simd(mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults.increase_at(input[0].0);
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(mults: Vec<PackedM31>) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = LOG_SIZE - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(LOG_SIZE),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let pedersenpoints_0 = PedersenPoints::new(0);
    let pedersenpoints_1 = PedersenPoints::new(1);
    let pedersenpoints_2 = PedersenPoints::new(2);
    let pedersenpoints_3 = PedersenPoints::new(3);
    let pedersenpoints_4 = PedersenPoints::new(4);
    let pedersenpoints_5 = PedersenPoints::new(5);
    let pedersenpoints_6 = PedersenPoints::new(6);
    let pedersenpoints_7 = PedersenPoints::new(7);
    let pedersenpoints_8 = PedersenPoints::new(8);
    let pedersenpoints_9 = PedersenPoints::new(9);
    let pedersenpoints_10 = PedersenPoints::new(10);
    let pedersenpoints_11 = PedersenPoints::new(11);
    let pedersenpoints_12 = PedersenPoints::new(12);
    let pedersenpoints_13 = PedersenPoints::new(13);
    let pedersenpoints_14 = PedersenPoints::new(14);
    let pedersenpoints_15 = PedersenPoints::new(15);
    let pedersenpoints_16 = PedersenPoints::new(16);
    let pedersenpoints_17 = PedersenPoints::new(17);
    let pedersenpoints_18 = PedersenPoints::new(18);
    let pedersenpoints_19 = PedersenPoints::new(19);
    let pedersenpoints_20 = PedersenPoints::new(20);
    let pedersenpoints_21 = PedersenPoints::new(21);
    let pedersenpoints_22 = PedersenPoints::new(22);
    let pedersenpoints_23 = PedersenPoints::new(23);
    let pedersenpoints_24 = PedersenPoints::new(24);
    let pedersenpoints_25 = PedersenPoints::new(25);
    let pedersenpoints_26 = PedersenPoints::new(26);
    let pedersenpoints_27 = PedersenPoints::new(27);
    let pedersenpoints_28 = PedersenPoints::new(28);
    let pedersenpoints_29 = PedersenPoints::new(29);
    let pedersenpoints_30 = PedersenPoints::new(30);
    let pedersenpoints_31 = PedersenPoints::new(31);
    let pedersenpoints_32 = PedersenPoints::new(32);
    let pedersenpoints_33 = PedersenPoints::new(33);
    let pedersenpoints_34 = PedersenPoints::new(34);
    let pedersenpoints_35 = PedersenPoints::new(35);
    let pedersenpoints_36 = PedersenPoints::new(36);
    let pedersenpoints_37 = PedersenPoints::new(37);
    let pedersenpoints_38 = PedersenPoints::new(38);
    let pedersenpoints_39 = PedersenPoints::new(39);
    let pedersenpoints_40 = PedersenPoints::new(40);
    let pedersenpoints_41 = PedersenPoints::new(41);
    let pedersenpoints_42 = PedersenPoints::new(42);
    let pedersenpoints_43 = PedersenPoints::new(43);
    let pedersenpoints_44 = PedersenPoints::new(44);
    let pedersenpoints_45 = PedersenPoints::new(45);
    let pedersenpoints_46 = PedersenPoints::new(46);
    let pedersenpoints_47 = PedersenPoints::new(47);
    let pedersenpoints_48 = PedersenPoints::new(48);
    let pedersenpoints_49 = PedersenPoints::new(49);
    let pedersenpoints_50 = PedersenPoints::new(50);
    let pedersenpoints_51 = PedersenPoints::new(51);
    let pedersenpoints_52 = PedersenPoints::new(52);
    let pedersenpoints_53 = PedersenPoints::new(53);
    let pedersenpoints_54 = PedersenPoints::new(54);
    let pedersenpoints_55 = PedersenPoints::new(55);
    let seq = Seq::new(LOG_SIZE);

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (mut row, lookup_data))| {
            let pedersenpoints_0 = pedersenpoints_0.packed_at(row_index);
            let pedersenpoints_1 = pedersenpoints_1.packed_at(row_index);
            let pedersenpoints_2 = pedersenpoints_2.packed_at(row_index);
            let pedersenpoints_3 = pedersenpoints_3.packed_at(row_index);
            let pedersenpoints_4 = pedersenpoints_4.packed_at(row_index);
            let pedersenpoints_5 = pedersenpoints_5.packed_at(row_index);
            let pedersenpoints_6 = pedersenpoints_6.packed_at(row_index);
            let pedersenpoints_7 = pedersenpoints_7.packed_at(row_index);
            let pedersenpoints_8 = pedersenpoints_8.packed_at(row_index);
            let pedersenpoints_9 = pedersenpoints_9.packed_at(row_index);
            let pedersenpoints_10 = pedersenpoints_10.packed_at(row_index);
            let pedersenpoints_11 = pedersenpoints_11.packed_at(row_index);
            let pedersenpoints_12 = pedersenpoints_12.packed_at(row_index);
            let pedersenpoints_13 = pedersenpoints_13.packed_at(row_index);
            let pedersenpoints_14 = pedersenpoints_14.packed_at(row_index);
            let pedersenpoints_15 = pedersenpoints_15.packed_at(row_index);
            let pedersenpoints_16 = pedersenpoints_16.packed_at(row_index);
            let pedersenpoints_17 = pedersenpoints_17.packed_at(row_index);
            let pedersenpoints_18 = pedersenpoints_18.packed_at(row_index);
            let pedersenpoints_19 = pedersenpoints_19.packed_at(row_index);
            let pedersenpoints_20 = pedersenpoints_20.packed_at(row_index);
            let pedersenpoints_21 = pedersenpoints_21.packed_at(row_index);
            let pedersenpoints_22 = pedersenpoints_22.packed_at(row_index);
            let pedersenpoints_23 = pedersenpoints_23.packed_at(row_index);
            let pedersenpoints_24 = pedersenpoints_24.packed_at(row_index);
            let pedersenpoints_25 = pedersenpoints_25.packed_at(row_index);
            let pedersenpoints_26 = pedersenpoints_26.packed_at(row_index);
            let pedersenpoints_27 = pedersenpoints_27.packed_at(row_index);
            let pedersenpoints_28 = pedersenpoints_28.packed_at(row_index);
            let pedersenpoints_29 = pedersenpoints_29.packed_at(row_index);
            let pedersenpoints_30 = pedersenpoints_30.packed_at(row_index);
            let pedersenpoints_31 = pedersenpoints_31.packed_at(row_index);
            let pedersenpoints_32 = pedersenpoints_32.packed_at(row_index);
            let pedersenpoints_33 = pedersenpoints_33.packed_at(row_index);
            let pedersenpoints_34 = pedersenpoints_34.packed_at(row_index);
            let pedersenpoints_35 = pedersenpoints_35.packed_at(row_index);
            let pedersenpoints_36 = pedersenpoints_36.packed_at(row_index);
            let pedersenpoints_37 = pedersenpoints_37.packed_at(row_index);
            let pedersenpoints_38 = pedersenpoints_38.packed_at(row_index);
            let pedersenpoints_39 = pedersenpoints_39.packed_at(row_index);
            let pedersenpoints_40 = pedersenpoints_40.packed_at(row_index);
            let pedersenpoints_41 = pedersenpoints_41.packed_at(row_index);
            let pedersenpoints_42 = pedersenpoints_42.packed_at(row_index);
            let pedersenpoints_43 = pedersenpoints_43.packed_at(row_index);
            let pedersenpoints_44 = pedersenpoints_44.packed_at(row_index);
            let pedersenpoints_45 = pedersenpoints_45.packed_at(row_index);
            let pedersenpoints_46 = pedersenpoints_46.packed_at(row_index);
            let pedersenpoints_47 = pedersenpoints_47.packed_at(row_index);
            let pedersenpoints_48 = pedersenpoints_48.packed_at(row_index);
            let pedersenpoints_49 = pedersenpoints_49.packed_at(row_index);
            let pedersenpoints_50 = pedersenpoints_50.packed_at(row_index);
            let pedersenpoints_51 = pedersenpoints_51.packed_at(row_index);
            let pedersenpoints_52 = pedersenpoints_52.packed_at(row_index);
            let pedersenpoints_53 = pedersenpoints_53.packed_at(row_index);
            let pedersenpoints_54 = pedersenpoints_54.packed_at(row_index);
            let pedersenpoints_55 = pedersenpoints_55.packed_at(row_index);
            let seq = seq.packed_at(row_index);
            *lookup_data.pedersen_points_table_0 = [
                seq,
                pedersenpoints_0,
                pedersenpoints_1,
                pedersenpoints_2,
                pedersenpoints_3,
                pedersenpoints_4,
                pedersenpoints_5,
                pedersenpoints_6,
                pedersenpoints_7,
                pedersenpoints_8,
                pedersenpoints_9,
                pedersenpoints_10,
                pedersenpoints_11,
                pedersenpoints_12,
                pedersenpoints_13,
                pedersenpoints_14,
                pedersenpoints_15,
                pedersenpoints_16,
                pedersenpoints_17,
                pedersenpoints_18,
                pedersenpoints_19,
                pedersenpoints_20,
                pedersenpoints_21,
                pedersenpoints_22,
                pedersenpoints_23,
                pedersenpoints_24,
                pedersenpoints_25,
                pedersenpoints_26,
                pedersenpoints_27,
                pedersenpoints_28,
                pedersenpoints_29,
                pedersenpoints_30,
                pedersenpoints_31,
                pedersenpoints_32,
                pedersenpoints_33,
                pedersenpoints_34,
                pedersenpoints_35,
                pedersenpoints_36,
                pedersenpoints_37,
                pedersenpoints_38,
                pedersenpoints_39,
                pedersenpoints_40,
                pedersenpoints_41,
                pedersenpoints_42,
                pedersenpoints_43,
                pedersenpoints_44,
                pedersenpoints_45,
                pedersenpoints_46,
                pedersenpoints_47,
                pedersenpoints_48,
                pedersenpoints_49,
                pedersenpoints_50,
                pedersenpoints_51,
                pedersenpoints_52,
                pedersenpoints_53,
                pedersenpoints_54,
                pedersenpoints_55,
            ];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    pedersen_points_table_0: Vec<[PackedM31; 57]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        pedersen_points_table: &relations::PedersenPointsTable,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.pedersen_points_table_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = pedersen_points_table.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::pedersen_points_table::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
    input_to_row: HashMap<[M31; 1], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let column_ids = [PreProcessedColumnId {
            id: "seq_23".to_owned(),
        }];
        Self {
            mults: AtomicMultiplicityColumn::new(1 << LOG_SIZE),
            input_to_row: make_input_to_row(&preprocessed_trace, column_ids),
            preprocessed_trace,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self.mults.into_simd_vec();

        let (trace, lookup_data) = write_trace_simd(&self.preprocessed_trace, mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults
            .increase_at((*self.input_to_row.get(input).unwrap()).try_into().unwrap());
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
fn write_trace_simd(
    preprocessed_trace: &PreProcessedTrace,
    mults: Vec<PackedM31>,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = LOG_SIZE - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(LOG_SIZE),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let seq_23 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_23".to_owned(),
    });
    let pedersen_points_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_0".to_owned(),
    });
    let pedersen_points_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_1".to_owned(),
    });
    let pedersen_points_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_2".to_owned(),
    });
    let pedersen_points_3 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_3".to_owned(),
    });
    let pedersen_points_4 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_4".to_owned(),
    });
    let pedersen_points_5 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_5".to_owned(),
    });
    let pedersen_points_6 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_6".to_owned(),
    });
    let pedersen_points_7 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_7".to_owned(),
    });
    let pedersen_points_8 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_8".to_owned(),
    });
    let pedersen_points_9 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_9".to_owned(),
    });
    let pedersen_points_10 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_10".to_owned(),
    });
    let pedersen_points_11 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_11".to_owned(),
    });
    let pedersen_points_12 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_12".to_owned(),
    });
    let pedersen_points_13 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_13".to_owned(),
    });
    let pedersen_points_14 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_14".to_owned(),
    });
    let pedersen_points_15 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_15".to_owned(),
    });
    let pedersen_points_16 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_16".to_owned(),
    });
    let pedersen_points_17 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_17".to_owned(),
    });
    let pedersen_points_18 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_18".to_owned(),
    });
    let pedersen_points_19 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_19".to_owned(),
    });
    let pedersen_points_20 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_20".to_owned(),
    });
    let pedersen_points_21 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_21".to_owned(),
    });
    let pedersen_points_22 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_22".to_owned(),
    });
    let pedersen_points_23 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_23".to_owned(),
    });
    let pedersen_points_24 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_24".to_owned(),
    });
    let pedersen_points_25 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_25".to_owned(),
    });
    let pedersen_points_26 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_26".to_owned(),
    });
    let pedersen_points_27 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_27".to_owned(),
    });
    let pedersen_points_28 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_28".to_owned(),
    });
    let pedersen_points_29 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_29".to_owned(),
    });
    let pedersen_points_30 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_30".to_owned(),
    });
    let pedersen_points_31 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_31".to_owned(),
    });
    let pedersen_points_32 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_32".to_owned(),
    });
    let pedersen_points_33 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_33".to_owned(),
    });
    let pedersen_points_34 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_34".to_owned(),
    });
    let pedersen_points_35 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_35".to_owned(),
    });
    let pedersen_points_36 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_36".to_owned(),
    });
    let pedersen_points_37 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_37".to_owned(),
    });
    let pedersen_points_38 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_38".to_owned(),
    });
    let pedersen_points_39 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_39".to_owned(),
    });
    let pedersen_points_40 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_40".to_owned(),
    });
    let pedersen_points_41 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_41".to_owned(),
    });
    let pedersen_points_42 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_42".to_owned(),
    });
    let pedersen_points_43 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_43".to_owned(),
    });
    let pedersen_points_44 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_44".to_owned(),
    });
    let pedersen_points_45 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_45".to_owned(),
    });
    let pedersen_points_46 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_46".to_owned(),
    });
    let pedersen_points_47 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_47".to_owned(),
    });
    let pedersen_points_48 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_48".to_owned(),
    });
    let pedersen_points_49 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_49".to_owned(),
    });
    let pedersen_points_50 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_50".to_owned(),
    });
    let pedersen_points_51 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_51".to_owned(),
    });
    let pedersen_points_52 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_52".to_owned(),
    });
    let pedersen_points_53 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_53".to_owned(),
    });
    let pedersen_points_54 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_54".to_owned(),
    });
    let pedersen_points_55 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_55".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_23 = seq_23.packed_at(row_index);
            let pedersen_points_0 = pedersen_points_0.packed_at(row_index);
            let pedersen_points_1 = pedersen_points_1.packed_at(row_index);
            let pedersen_points_2 = pedersen_points_2.packed_at(row_index);
            let pedersen_points_3 = pedersen_points_3.packed_at(row_index);
            let pedersen_points_4 = pedersen_points_4.packed_at(row_index);
            let pedersen_points_5 = pedersen_points_5.packed_at(row_index);
            let pedersen_points_6 = pedersen_points_6.packed_at(row_index);
            let pedersen_points_7 = pedersen_points_7.packed_at(row_index);
            let pedersen_points_8 = pedersen_points_8.packed_at(row_index);
            let pedersen_points_9 = pedersen_points_9.packed_at(row_index);
            let pedersen_points_10 = pedersen_points_10.packed_at(row_index);
            let pedersen_points_11 = pedersen_points_11.packed_at(row_index);
            let pedersen_points_12 = pedersen_points_12.packed_at(row_index);
            let pedersen_points_13 = pedersen_points_13.packed_at(row_index);
            let pedersen_points_14 = pedersen_points_14.packed_at(row_index);
            let pedersen_points_15 = pedersen_points_15.packed_at(row_index);
            let pedersen_points_16 = pedersen_points_16.packed_at(row_index);
            let pedersen_points_17 = pedersen_points_17.packed_at(row_index);
            let pedersen_points_18 = pedersen_points_18.packed_at(row_index);
            let pedersen_points_19 = pedersen_points_19.packed_at(row_index);
            let pedersen_points_20 = pedersen_points_20.packed_at(row_index);
            let pedersen_points_21 = pedersen_points_21.packed_at(row_index);
            let pedersen_points_22 = pedersen_points_22.packed_at(row_index);
            let pedersen_points_23 = pedersen_points_23.packed_at(row_index);
            let pedersen_points_24 = pedersen_points_24.packed_at(row_index);
            let pedersen_points_25 = pedersen_points_25.packed_at(row_index);
            let pedersen_points_26 = pedersen_points_26.packed_at(row_index);
            let pedersen_points_27 = pedersen_points_27.packed_at(row_index);
            let pedersen_points_28 = pedersen_points_28.packed_at(row_index);
            let pedersen_points_29 = pedersen_points_29.packed_at(row_index);
            let pedersen_points_30 = pedersen_points_30.packed_at(row_index);
            let pedersen_points_31 = pedersen_points_31.packed_at(row_index);
            let pedersen_points_32 = pedersen_points_32.packed_at(row_index);
            let pedersen_points_33 = pedersen_points_33.packed_at(row_index);
            let pedersen_points_34 = pedersen_points_34.packed_at(row_index);
            let pedersen_points_35 = pedersen_points_35.packed_at(row_index);
            let pedersen_points_36 = pedersen_points_36.packed_at(row_index);
            let pedersen_points_37 = pedersen_points_37.packed_at(row_index);
            let pedersen_points_38 = pedersen_points_38.packed_at(row_index);
            let pedersen_points_39 = pedersen_points_39.packed_at(row_index);
            let pedersen_points_40 = pedersen_points_40.packed_at(row_index);
            let pedersen_points_41 = pedersen_points_41.packed_at(row_index);
            let pedersen_points_42 = pedersen_points_42.packed_at(row_index);
            let pedersen_points_43 = pedersen_points_43.packed_at(row_index);
            let pedersen_points_44 = pedersen_points_44.packed_at(row_index);
            let pedersen_points_45 = pedersen_points_45.packed_at(row_index);
            let pedersen_points_46 = pedersen_points_46.packed_at(row_index);
            let pedersen_points_47 = pedersen_points_47.packed_at(row_index);
            let pedersen_points_48 = pedersen_points_48.packed_at(row_index);
            let pedersen_points_49 = pedersen_points_49.packed_at(row_index);
            let pedersen_points_50 = pedersen_points_50.packed_at(row_index);
            let pedersen_points_51 = pedersen_points_51.packed_at(row_index);
            let pedersen_points_52 = pedersen_points_52.packed_at(row_index);
            let pedersen_points_53 = pedersen_points_53.packed_at(row_index);
            let pedersen_points_54 = pedersen_points_54.packed_at(row_index);
            let pedersen_points_55 = pedersen_points_55.packed_at(row_index);
            *lookup_data.pedersen_points_table_0 = [
                seq_23,
                pedersen_points_0,
                pedersen_points_1,
                pedersen_points_2,
                pedersen_points_3,
                pedersen_points_4,
                pedersen_points_5,
                pedersen_points_6,
                pedersen_points_7,
                pedersen_points_8,
                pedersen_points_9,
                pedersen_points_10,
                pedersen_points_11,
                pedersen_points_12,
                pedersen_points_13,
                pedersen_points_14,
                pedersen_points_15,
                pedersen_points_16,
                pedersen_points_17,
                pedersen_points_18,
                pedersen_points_19,
                pedersen_points_20,
                pedersen_points_21,
                pedersen_points_22,
                pedersen_points_23,
                pedersen_points_24,
                pedersen_points_25,
                pedersen_points_26,
                pedersen_points_27,
                pedersen_points_28,
                pedersen_points_29,
                pedersen_points_30,
                pedersen_points_31,
                pedersen_points_32,
                pedersen_points_33,
                pedersen_points_34,
                pedersen_points_35,
                pedersen_points_36,
                pedersen_points_37,
                pedersen_points_38,
                pedersen_points_39,
                pedersen_points_40,
                pedersen_points_41,
                pedersen_points_42,
                pedersen_points_43,
                pedersen_points_44,
                pedersen_points_45,
                pedersen_points_46,
                pedersen_points_47,
                pedersen_points_48,
                pedersen_points_49,
                pedersen_points_50,
                pedersen_points_51,
                pedersen_points_52,
                pedersen_points_53,
                pedersen_points_54,
                pedersen_points_55,
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

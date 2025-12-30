// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::pedersen_points_table_window_bits_9::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 1],
    input_to_row: HashMap<[M31; 1], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        let column_ids = [PreProcessedColumnId {
            id: "seq_15".to_owned(),
        }];

        Self {
            mults,
            input_to_row: make_input_to_row(&preprocessed_trace, column_ids),
            preprocessed_trace,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self
            .mults
            .into_iter()
            .map(|v| v.into_simd_vec())
            .collect::<Vec<_>>();

        let (trace, lookup_data) = write_trace_simd(&self.preprocessed_trace, mults);
        tree_builder.extend_evals(trace.to_evals());

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, input: &InputType, relation_index: usize) {
        self.mults[relation_index]
            .increase_at((*self.input_to_row.get(input).unwrap()).try_into().unwrap());
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType], relation_index: usize) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input, relation_index);
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
    mults: Vec<Vec<PackedM31>>,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = LOG_SIZE - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(LOG_SIZE),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_1791500038 = PackedM31::broadcast(M31::from(1791500038));
    let seq_15 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_15".to_owned(),
    });
    let pedersen_points_small_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_0".to_owned(),
    });
    let pedersen_points_small_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_1".to_owned(),
    });
    let pedersen_points_small_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_2".to_owned(),
    });
    let pedersen_points_small_3 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_3".to_owned(),
    });
    let pedersen_points_small_4 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_4".to_owned(),
    });
    let pedersen_points_small_5 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_5".to_owned(),
    });
    let pedersen_points_small_6 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_6".to_owned(),
    });
    let pedersen_points_small_7 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_7".to_owned(),
    });
    let pedersen_points_small_8 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_8".to_owned(),
    });
    let pedersen_points_small_9 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_9".to_owned(),
    });
    let pedersen_points_small_10 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_10".to_owned(),
    });
    let pedersen_points_small_11 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_11".to_owned(),
    });
    let pedersen_points_small_12 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_12".to_owned(),
    });
    let pedersen_points_small_13 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_13".to_owned(),
    });
    let pedersen_points_small_14 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_14".to_owned(),
    });
    let pedersen_points_small_15 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_15".to_owned(),
    });
    let pedersen_points_small_16 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_16".to_owned(),
    });
    let pedersen_points_small_17 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_17".to_owned(),
    });
    let pedersen_points_small_18 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_18".to_owned(),
    });
    let pedersen_points_small_19 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_19".to_owned(),
    });
    let pedersen_points_small_20 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_20".to_owned(),
    });
    let pedersen_points_small_21 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_21".to_owned(),
    });
    let pedersen_points_small_22 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_22".to_owned(),
    });
    let pedersen_points_small_23 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_23".to_owned(),
    });
    let pedersen_points_small_24 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_24".to_owned(),
    });
    let pedersen_points_small_25 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_25".to_owned(),
    });
    let pedersen_points_small_26 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_26".to_owned(),
    });
    let pedersen_points_small_27 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_27".to_owned(),
    });
    let pedersen_points_small_28 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_28".to_owned(),
    });
    let pedersen_points_small_29 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_29".to_owned(),
    });
    let pedersen_points_small_30 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_30".to_owned(),
    });
    let pedersen_points_small_31 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_31".to_owned(),
    });
    let pedersen_points_small_32 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_32".to_owned(),
    });
    let pedersen_points_small_33 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_33".to_owned(),
    });
    let pedersen_points_small_34 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_34".to_owned(),
    });
    let pedersen_points_small_35 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_35".to_owned(),
    });
    let pedersen_points_small_36 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_36".to_owned(),
    });
    let pedersen_points_small_37 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_37".to_owned(),
    });
    let pedersen_points_small_38 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_38".to_owned(),
    });
    let pedersen_points_small_39 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_39".to_owned(),
    });
    let pedersen_points_small_40 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_40".to_owned(),
    });
    let pedersen_points_small_41 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_41".to_owned(),
    });
    let pedersen_points_small_42 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_42".to_owned(),
    });
    let pedersen_points_small_43 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_43".to_owned(),
    });
    let pedersen_points_small_44 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_44".to_owned(),
    });
    let pedersen_points_small_45 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_45".to_owned(),
    });
    let pedersen_points_small_46 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_46".to_owned(),
    });
    let pedersen_points_small_47 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_47".to_owned(),
    });
    let pedersen_points_small_48 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_48".to_owned(),
    });
    let pedersen_points_small_49 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_49".to_owned(),
    });
    let pedersen_points_small_50 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_50".to_owned(),
    });
    let pedersen_points_small_51 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_51".to_owned(),
    });
    let pedersen_points_small_52 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_52".to_owned(),
    });
    let pedersen_points_small_53 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_53".to_owned(),
    });
    let pedersen_points_small_54 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_54".to_owned(),
    });
    let pedersen_points_small_55 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "pedersen_points_small_55".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_15 = seq_15.packed_at(row_index);
            let pedersen_points_small_0 = pedersen_points_small_0.packed_at(row_index);
            let pedersen_points_small_1 = pedersen_points_small_1.packed_at(row_index);
            let pedersen_points_small_2 = pedersen_points_small_2.packed_at(row_index);
            let pedersen_points_small_3 = pedersen_points_small_3.packed_at(row_index);
            let pedersen_points_small_4 = pedersen_points_small_4.packed_at(row_index);
            let pedersen_points_small_5 = pedersen_points_small_5.packed_at(row_index);
            let pedersen_points_small_6 = pedersen_points_small_6.packed_at(row_index);
            let pedersen_points_small_7 = pedersen_points_small_7.packed_at(row_index);
            let pedersen_points_small_8 = pedersen_points_small_8.packed_at(row_index);
            let pedersen_points_small_9 = pedersen_points_small_9.packed_at(row_index);
            let pedersen_points_small_10 = pedersen_points_small_10.packed_at(row_index);
            let pedersen_points_small_11 = pedersen_points_small_11.packed_at(row_index);
            let pedersen_points_small_12 = pedersen_points_small_12.packed_at(row_index);
            let pedersen_points_small_13 = pedersen_points_small_13.packed_at(row_index);
            let pedersen_points_small_14 = pedersen_points_small_14.packed_at(row_index);
            let pedersen_points_small_15 = pedersen_points_small_15.packed_at(row_index);
            let pedersen_points_small_16 = pedersen_points_small_16.packed_at(row_index);
            let pedersen_points_small_17 = pedersen_points_small_17.packed_at(row_index);
            let pedersen_points_small_18 = pedersen_points_small_18.packed_at(row_index);
            let pedersen_points_small_19 = pedersen_points_small_19.packed_at(row_index);
            let pedersen_points_small_20 = pedersen_points_small_20.packed_at(row_index);
            let pedersen_points_small_21 = pedersen_points_small_21.packed_at(row_index);
            let pedersen_points_small_22 = pedersen_points_small_22.packed_at(row_index);
            let pedersen_points_small_23 = pedersen_points_small_23.packed_at(row_index);
            let pedersen_points_small_24 = pedersen_points_small_24.packed_at(row_index);
            let pedersen_points_small_25 = pedersen_points_small_25.packed_at(row_index);
            let pedersen_points_small_26 = pedersen_points_small_26.packed_at(row_index);
            let pedersen_points_small_27 = pedersen_points_small_27.packed_at(row_index);
            let pedersen_points_small_28 = pedersen_points_small_28.packed_at(row_index);
            let pedersen_points_small_29 = pedersen_points_small_29.packed_at(row_index);
            let pedersen_points_small_30 = pedersen_points_small_30.packed_at(row_index);
            let pedersen_points_small_31 = pedersen_points_small_31.packed_at(row_index);
            let pedersen_points_small_32 = pedersen_points_small_32.packed_at(row_index);
            let pedersen_points_small_33 = pedersen_points_small_33.packed_at(row_index);
            let pedersen_points_small_34 = pedersen_points_small_34.packed_at(row_index);
            let pedersen_points_small_35 = pedersen_points_small_35.packed_at(row_index);
            let pedersen_points_small_36 = pedersen_points_small_36.packed_at(row_index);
            let pedersen_points_small_37 = pedersen_points_small_37.packed_at(row_index);
            let pedersen_points_small_38 = pedersen_points_small_38.packed_at(row_index);
            let pedersen_points_small_39 = pedersen_points_small_39.packed_at(row_index);
            let pedersen_points_small_40 = pedersen_points_small_40.packed_at(row_index);
            let pedersen_points_small_41 = pedersen_points_small_41.packed_at(row_index);
            let pedersen_points_small_42 = pedersen_points_small_42.packed_at(row_index);
            let pedersen_points_small_43 = pedersen_points_small_43.packed_at(row_index);
            let pedersen_points_small_44 = pedersen_points_small_44.packed_at(row_index);
            let pedersen_points_small_45 = pedersen_points_small_45.packed_at(row_index);
            let pedersen_points_small_46 = pedersen_points_small_46.packed_at(row_index);
            let pedersen_points_small_47 = pedersen_points_small_47.packed_at(row_index);
            let pedersen_points_small_48 = pedersen_points_small_48.packed_at(row_index);
            let pedersen_points_small_49 = pedersen_points_small_49.packed_at(row_index);
            let pedersen_points_small_50 = pedersen_points_small_50.packed_at(row_index);
            let pedersen_points_small_51 = pedersen_points_small_51.packed_at(row_index);
            let pedersen_points_small_52 = pedersen_points_small_52.packed_at(row_index);
            let pedersen_points_small_53 = pedersen_points_small_53.packed_at(row_index);
            let pedersen_points_small_54 = pedersen_points_small_54.packed_at(row_index);
            let pedersen_points_small_55 = pedersen_points_small_55.packed_at(row_index);
            *lookup_data.pedersen_points_table_window_bits_9_0 = [
                M31_1791500038,
                seq_15,
                pedersen_points_small_0,
                pedersen_points_small_1,
                pedersen_points_small_2,
                pedersen_points_small_3,
                pedersen_points_small_4,
                pedersen_points_small_5,
                pedersen_points_small_6,
                pedersen_points_small_7,
                pedersen_points_small_8,
                pedersen_points_small_9,
                pedersen_points_small_10,
                pedersen_points_small_11,
                pedersen_points_small_12,
                pedersen_points_small_13,
                pedersen_points_small_14,
                pedersen_points_small_15,
                pedersen_points_small_16,
                pedersen_points_small_17,
                pedersen_points_small_18,
                pedersen_points_small_19,
                pedersen_points_small_20,
                pedersen_points_small_21,
                pedersen_points_small_22,
                pedersen_points_small_23,
                pedersen_points_small_24,
                pedersen_points_small_25,
                pedersen_points_small_26,
                pedersen_points_small_27,
                pedersen_points_small_28,
                pedersen_points_small_29,
                pedersen_points_small_30,
                pedersen_points_small_31,
                pedersen_points_small_32,
                pedersen_points_small_33,
                pedersen_points_small_34,
                pedersen_points_small_35,
                pedersen_points_small_36,
                pedersen_points_small_37,
                pedersen_points_small_38,
                pedersen_points_small_39,
                pedersen_points_small_40,
                pedersen_points_small_41,
                pedersen_points_small_42,
                pedersen_points_small_43,
                pedersen_points_small_44,
                pedersen_points_small_45,
                pedersen_points_small_46,
                pedersen_points_small_47,
                pedersen_points_small_48,
                pedersen_points_small_49,
                pedersen_points_small_50,
                pedersen_points_small_51,
                pedersen_points_small_52,
                pedersen_points_small_53,
                pedersen_points_small_54,
                pedersen_points_small_55,
            ];
            let mult = &mults[0];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults_0 = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    pedersen_points_table_window_bits_9_0: Vec<[PackedM31; 58]>,
    mults_0: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        common_lookup_elements: &relations::CommonLookupElements,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.pedersen_points_table_window_bits_9_0,
            self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults_0)| {
                let denom = common_lookup_elements.combine(values);
                writer.write_frac(-PackedQM31::one() * mults_0, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

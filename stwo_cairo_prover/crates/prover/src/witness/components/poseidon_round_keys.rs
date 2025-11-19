// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_round_keys::{
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
            id: "seq_6".to_owned(),
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

    let seq_6 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_6".to_owned(),
    });
    let poseidon_round_keys_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_0".to_owned(),
    });
    let poseidon_round_keys_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_1".to_owned(),
    });
    let poseidon_round_keys_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_2".to_owned(),
    });
    let poseidon_round_keys_3 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_3".to_owned(),
    });
    let poseidon_round_keys_4 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_4".to_owned(),
    });
    let poseidon_round_keys_5 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_5".to_owned(),
    });
    let poseidon_round_keys_6 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_6".to_owned(),
    });
    let poseidon_round_keys_7 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_7".to_owned(),
    });
    let poseidon_round_keys_8 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_8".to_owned(),
    });
    let poseidon_round_keys_9 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_9".to_owned(),
    });
    let poseidon_round_keys_10 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_10".to_owned(),
    });
    let poseidon_round_keys_11 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_11".to_owned(),
    });
    let poseidon_round_keys_12 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_12".to_owned(),
    });
    let poseidon_round_keys_13 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_13".to_owned(),
    });
    let poseidon_round_keys_14 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_14".to_owned(),
    });
    let poseidon_round_keys_15 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_15".to_owned(),
    });
    let poseidon_round_keys_16 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_16".to_owned(),
    });
    let poseidon_round_keys_17 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_17".to_owned(),
    });
    let poseidon_round_keys_18 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_18".to_owned(),
    });
    let poseidon_round_keys_19 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_19".to_owned(),
    });
    let poseidon_round_keys_20 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_20".to_owned(),
    });
    let poseidon_round_keys_21 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_21".to_owned(),
    });
    let poseidon_round_keys_22 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_22".to_owned(),
    });
    let poseidon_round_keys_23 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_23".to_owned(),
    });
    let poseidon_round_keys_24 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_24".to_owned(),
    });
    let poseidon_round_keys_25 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_25".to_owned(),
    });
    let poseidon_round_keys_26 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_26".to_owned(),
    });
    let poseidon_round_keys_27 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_27".to_owned(),
    });
    let poseidon_round_keys_28 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_28".to_owned(),
    });
    let poseidon_round_keys_29 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "poseidon_round_keys_29".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_6 = seq_6.packed_at(row_index);
            let poseidon_round_keys_0 = poseidon_round_keys_0.packed_at(row_index);
            let poseidon_round_keys_1 = poseidon_round_keys_1.packed_at(row_index);
            let poseidon_round_keys_2 = poseidon_round_keys_2.packed_at(row_index);
            let poseidon_round_keys_3 = poseidon_round_keys_3.packed_at(row_index);
            let poseidon_round_keys_4 = poseidon_round_keys_4.packed_at(row_index);
            let poseidon_round_keys_5 = poseidon_round_keys_5.packed_at(row_index);
            let poseidon_round_keys_6 = poseidon_round_keys_6.packed_at(row_index);
            let poseidon_round_keys_7 = poseidon_round_keys_7.packed_at(row_index);
            let poseidon_round_keys_8 = poseidon_round_keys_8.packed_at(row_index);
            let poseidon_round_keys_9 = poseidon_round_keys_9.packed_at(row_index);
            let poseidon_round_keys_10 = poseidon_round_keys_10.packed_at(row_index);
            let poseidon_round_keys_11 = poseidon_round_keys_11.packed_at(row_index);
            let poseidon_round_keys_12 = poseidon_round_keys_12.packed_at(row_index);
            let poseidon_round_keys_13 = poseidon_round_keys_13.packed_at(row_index);
            let poseidon_round_keys_14 = poseidon_round_keys_14.packed_at(row_index);
            let poseidon_round_keys_15 = poseidon_round_keys_15.packed_at(row_index);
            let poseidon_round_keys_16 = poseidon_round_keys_16.packed_at(row_index);
            let poseidon_round_keys_17 = poseidon_round_keys_17.packed_at(row_index);
            let poseidon_round_keys_18 = poseidon_round_keys_18.packed_at(row_index);
            let poseidon_round_keys_19 = poseidon_round_keys_19.packed_at(row_index);
            let poseidon_round_keys_20 = poseidon_round_keys_20.packed_at(row_index);
            let poseidon_round_keys_21 = poseidon_round_keys_21.packed_at(row_index);
            let poseidon_round_keys_22 = poseidon_round_keys_22.packed_at(row_index);
            let poseidon_round_keys_23 = poseidon_round_keys_23.packed_at(row_index);
            let poseidon_round_keys_24 = poseidon_round_keys_24.packed_at(row_index);
            let poseidon_round_keys_25 = poseidon_round_keys_25.packed_at(row_index);
            let poseidon_round_keys_26 = poseidon_round_keys_26.packed_at(row_index);
            let poseidon_round_keys_27 = poseidon_round_keys_27.packed_at(row_index);
            let poseidon_round_keys_28 = poseidon_round_keys_28.packed_at(row_index);
            let poseidon_round_keys_29 = poseidon_round_keys_29.packed_at(row_index);
            *lookup_data.poseidon_round_keys_0 = [
                seq_6,
                poseidon_round_keys_0,
                poseidon_round_keys_1,
                poseidon_round_keys_2,
                poseidon_round_keys_3,
                poseidon_round_keys_4,
                poseidon_round_keys_5,
                poseidon_round_keys_6,
                poseidon_round_keys_7,
                poseidon_round_keys_8,
                poseidon_round_keys_9,
                poseidon_round_keys_10,
                poseidon_round_keys_11,
                poseidon_round_keys_12,
                poseidon_round_keys_13,
                poseidon_round_keys_14,
                poseidon_round_keys_15,
                poseidon_round_keys_16,
                poseidon_round_keys_17,
                poseidon_round_keys_18,
                poseidon_round_keys_19,
                poseidon_round_keys_20,
                poseidon_round_keys_21,
                poseidon_round_keys_22,
                poseidon_round_keys_23,
                poseidon_round_keys_24,
                poseidon_round_keys_25,
                poseidon_round_keys_26,
                poseidon_round_keys_27,
                poseidon_round_keys_28,
                poseidon_round_keys_29,
            ];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    poseidon_round_keys_0: Vec<[PackedM31; 31]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        poseidon_round_keys: &relations::PoseidonRoundKeys,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_round_keys_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = poseidon_round_keys.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::blake_round_sigma::{
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
            id: "seq_4".to_owned(),
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

    let seq_4 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_4".to_owned(),
    });
    let blake_sigma_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_0".to_owned(),
    });
    let blake_sigma_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_1".to_owned(),
    });
    let blake_sigma_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_2".to_owned(),
    });
    let blake_sigma_3 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_3".to_owned(),
    });
    let blake_sigma_4 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_4".to_owned(),
    });
    let blake_sigma_5 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_5".to_owned(),
    });
    let blake_sigma_6 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_6".to_owned(),
    });
    let blake_sigma_7 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_7".to_owned(),
    });
    let blake_sigma_8 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_8".to_owned(),
    });
    let blake_sigma_9 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_9".to_owned(),
    });
    let blake_sigma_10 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_10".to_owned(),
    });
    let blake_sigma_11 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_11".to_owned(),
    });
    let blake_sigma_12 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_12".to_owned(),
    });
    let blake_sigma_13 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_13".to_owned(),
    });
    let blake_sigma_14 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_14".to_owned(),
    });
    let blake_sigma_15 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "blake_sigma_15".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_4 = seq_4.packed_at(row_index);
            let blake_sigma_0 = blake_sigma_0.packed_at(row_index);
            let blake_sigma_1 = blake_sigma_1.packed_at(row_index);
            let blake_sigma_2 = blake_sigma_2.packed_at(row_index);
            let blake_sigma_3 = blake_sigma_3.packed_at(row_index);
            let blake_sigma_4 = blake_sigma_4.packed_at(row_index);
            let blake_sigma_5 = blake_sigma_5.packed_at(row_index);
            let blake_sigma_6 = blake_sigma_6.packed_at(row_index);
            let blake_sigma_7 = blake_sigma_7.packed_at(row_index);
            let blake_sigma_8 = blake_sigma_8.packed_at(row_index);
            let blake_sigma_9 = blake_sigma_9.packed_at(row_index);
            let blake_sigma_10 = blake_sigma_10.packed_at(row_index);
            let blake_sigma_11 = blake_sigma_11.packed_at(row_index);
            let blake_sigma_12 = blake_sigma_12.packed_at(row_index);
            let blake_sigma_13 = blake_sigma_13.packed_at(row_index);
            let blake_sigma_14 = blake_sigma_14.packed_at(row_index);
            let blake_sigma_15 = blake_sigma_15.packed_at(row_index);
            *lookup_data.blake_round_sigma_0 = [
                seq_4,
                blake_sigma_0,
                blake_sigma_1,
                blake_sigma_2,
                blake_sigma_3,
                blake_sigma_4,
                blake_sigma_5,
                blake_sigma_6,
                blake_sigma_7,
                blake_sigma_8,
                blake_sigma_9,
                blake_sigma_10,
                blake_sigma_11,
                blake_sigma_12,
                blake_sigma_13,
                blake_sigma_14,
                blake_sigma_15,
            ];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    blake_round_sigma_0: Vec<[PackedM31; 17]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        blake_round_sigma: &relations::BlakeRoundSigma,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.blake_round_sigma_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = blake_round_sigma.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

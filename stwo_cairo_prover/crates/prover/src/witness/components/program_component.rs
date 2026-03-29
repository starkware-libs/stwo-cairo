// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::program_component::{
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
            id: "seq_12".to_owned(),
        }];

        Self {
            mults,
            input_to_row: make_input_to_row(&preprocessed_trace, column_ids),
            preprocessed_trace,
        }
    }

    pub fn write_trace(
        self,
    ) -> (
        ComponentTrace<N_TRACE_COLUMNS>,
        Claim,
        InteractionClaimGenerator,
    ) {
        let mults = self
            .mults
            .into_iter()
            .map(|v| v.into_simd_vec())
            .collect::<Vec<_>>();

        let (trace, lookup_data) = write_trace_simd(&self.preprocessed_trace, mults);

        (trace, Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType], relation_index: usize) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            for input in packed_input.unpack() {
                self.mults[relation_index].increase_at(
                    (*self.input_to_row.get(&input).unwrap())
                        .try_into()
                        .unwrap(),
                );
            }
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

    let M31_1942035206 = PackedM31::broadcast(M31::from(1942035206));
    let seq_12 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_12".to_owned(),
    });
    let curr_program_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_0".to_owned(),
    });
    let curr_program_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_1".to_owned(),
    });
    let curr_program_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_2".to_owned(),
    });
    let curr_program_3 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_3".to_owned(),
    });
    let curr_program_4 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_4".to_owned(),
    });
    let curr_program_5 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_5".to_owned(),
    });
    let curr_program_6 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_6".to_owned(),
    });
    let curr_program_7 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_7".to_owned(),
    });
    let curr_program_8 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_8".to_owned(),
    });
    let curr_program_9 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_9".to_owned(),
    });
    let curr_program_10 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_10".to_owned(),
    });
    let curr_program_11 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_11".to_owned(),
    });
    let curr_program_12 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_12".to_owned(),
    });
    let curr_program_13 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_13".to_owned(),
    });
    let curr_program_14 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_14".to_owned(),
    });
    let curr_program_15 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_15".to_owned(),
    });
    let curr_program_16 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_16".to_owned(),
    });
    let curr_program_17 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_17".to_owned(),
    });
    let curr_program_18 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_18".to_owned(),
    });
    let curr_program_19 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_19".to_owned(),
    });
    let curr_program_20 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_20".to_owned(),
    });
    let curr_program_21 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_21".to_owned(),
    });
    let curr_program_22 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_22".to_owned(),
    });
    let curr_program_23 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_23".to_owned(),
    });
    let curr_program_24 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_24".to_owned(),
    });
    let curr_program_25 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_25".to_owned(),
    });
    let curr_program_26 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_26".to_owned(),
    });
    let curr_program_27 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "curr_program_27".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_12 = seq_12.packed_at(row_index);
            let curr_program_0 = curr_program_0.packed_at(row_index);
            let curr_program_1 = curr_program_1.packed_at(row_index);
            let curr_program_2 = curr_program_2.packed_at(row_index);
            let curr_program_3 = curr_program_3.packed_at(row_index);
            let curr_program_4 = curr_program_4.packed_at(row_index);
            let curr_program_5 = curr_program_5.packed_at(row_index);
            let curr_program_6 = curr_program_6.packed_at(row_index);
            let curr_program_7 = curr_program_7.packed_at(row_index);
            let curr_program_8 = curr_program_8.packed_at(row_index);
            let curr_program_9 = curr_program_9.packed_at(row_index);
            let curr_program_10 = curr_program_10.packed_at(row_index);
            let curr_program_11 = curr_program_11.packed_at(row_index);
            let curr_program_12 = curr_program_12.packed_at(row_index);
            let curr_program_13 = curr_program_13.packed_at(row_index);
            let curr_program_14 = curr_program_14.packed_at(row_index);
            let curr_program_15 = curr_program_15.packed_at(row_index);
            let curr_program_16 = curr_program_16.packed_at(row_index);
            let curr_program_17 = curr_program_17.packed_at(row_index);
            let curr_program_18 = curr_program_18.packed_at(row_index);
            let curr_program_19 = curr_program_19.packed_at(row_index);
            let curr_program_20 = curr_program_20.packed_at(row_index);
            let curr_program_21 = curr_program_21.packed_at(row_index);
            let curr_program_22 = curr_program_22.packed_at(row_index);
            let curr_program_23 = curr_program_23.packed_at(row_index);
            let curr_program_24 = curr_program_24.packed_at(row_index);
            let curr_program_25 = curr_program_25.packed_at(row_index);
            let curr_program_26 = curr_program_26.packed_at(row_index);
            let curr_program_27 = curr_program_27.packed_at(row_index);
            *lookup_data.program_component_0 = [
                M31_1942035206,
                seq_12,
                curr_program_0,
                curr_program_1,
                curr_program_2,
                curr_program_3,
                curr_program_4,
                curr_program_5,
                curr_program_6,
                curr_program_7,
                curr_program_8,
                curr_program_9,
                curr_program_10,
                curr_program_11,
                curr_program_12,
                curr_program_13,
                curr_program_14,
                curr_program_15,
                curr_program_16,
                curr_program_17,
                curr_program_18,
                curr_program_19,
                curr_program_20,
                curr_program_21,
                curr_program_22,
                curr_program_23,
                curr_program_24,
                curr_program_25,
                curr_program_26,
                curr_program_27,
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
    program_component_0: Vec<[PackedM31; 30]>,
    mults_0: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        common_lookup_elements: &relations::CommonLookupElements,
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        InteractionClaim,
    ) {
        let mut logup_gen = unsafe { LogupTraceGenerator::uninitialized(LOG_SIZE) };

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.program_component_0,
            self.lookup_data.mults_0,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults_0)| {
                let denom = common_lookup_elements.combine(values);
                writer.write_frac(-PackedQM31::one() * mults_0, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}

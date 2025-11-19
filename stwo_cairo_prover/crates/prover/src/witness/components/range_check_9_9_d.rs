// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::range_check_9_9_d::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};

use crate::witness::prelude::*;

pub type InputType = [M31; 2];
pub type PackedInputType = [PackedM31; 2];

pub struct ClaimGenerator {
    pub mults: AtomicMultiplicityColumn,
    input_to_row: HashMap<[M31; 2], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let column_ids = [
            PreProcessedColumnId {
                id: "range_check_9_9_column_0".to_owned(),
            },
            PreProcessedColumnId {
                id: "range_check_9_9_column_1".to_owned(),
            },
        ];
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

    let range_check_9_9_column_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "range_check_9_9_column_0".to_owned(),
    });
    let range_check_9_9_column_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "range_check_9_9_column_1".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let range_check_9_9_column_0 = range_check_9_9_column_0.packed_at(row_index);
            let range_check_9_9_column_1 = range_check_9_9_column_1.packed_at(row_index);
            *lookup_data.range_check_9_9_d_0 = [range_check_9_9_column_0, range_check_9_9_column_1];
            let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_9_9_d_0: Vec<[PackedM31; 2]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_9_9_d: &relations::RangeCheck_9_9_D,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_9_9_d_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = range_check_9_9_d.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::range_check_18::{Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS};

use crate::witness::prelude::*;

pub type InputType = [M31; 1];
pub type PackedInputType = [PackedM31; 1];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 2],
    input_to_row: HashMap<[M31; 1], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        let column_ids = [PreProcessedColumnId {
            id: "seq_18".to_owned(),
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

    let M31_1109051422 = PackedM31::broadcast(M31::from(1109051422));
    let M31_1424798916 = PackedM31::broadcast(M31::from(1424798916));
    let seq_18 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "seq_18".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let seq_18 = seq_18.packed_at(row_index);
            *lookup_data.range_check_18_0 = [M31_1109051422, seq_18];
            *lookup_data.range_check_18_b_0 = [M31_1424798916, seq_18];
            let mult = &mults[0];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = mult_at_row;
            *lookup_data.mults_0 = mult_at_row;
            let mult = &mults[1];
            let mult_at_row = *mult.get(row_index).unwrap_or(&PackedM31::zero());
            *row[1] = mult_at_row;
            *lookup_data.mults_1 = mult_at_row;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    range_check_18_0: Vec<[PackedM31; 2]>,
    range_check_18_b_0: Vec<[PackedM31; 2]>,
    mults_0: Vec<PackedM31>,
    mults_1: Vec<PackedM31>,
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
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_18_0,
            &self.lookup_data.range_check_18_b_0,
            self.lookup_data.mults_0,
            self.lookup_data.mults_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mults_0, mults_1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(-(denom0 * mults_1 + denom1 * mults_0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}

// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::verify_bitwise_xor_8::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};

use crate::witness::prelude::*;

pub type InputType = [M31; 3];
pub type PackedInputType = [PackedM31; 3];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 2],
    input_to_row: HashMap<[M31; 3], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        let column_ids = [
            PreProcessedColumnId {
                id: "bitwise_xor_8_0".to_owned(),
            },
            PreProcessedColumnId {
                id: "bitwise_xor_8_1".to_owned(),
            },
            PreProcessedColumnId {
                id: "bitwise_xor_8_2".to_owned(),
            },
        ];

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
}

impl AddInputs for ClaimGenerator {
    type PackedInputType = PackedInputType;
    type InputType = InputType;

    fn add_packed_inputs(&self, packed_inputs: &[PackedInputType], relation_index: usize) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            for input in packed_input.unpack() {
                self.add_input(&input, relation_index);
            }
        });
    }
    fn add_input(&self, input: &InputType, relation_index: usize) {
        self.mults[relation_index]
            .increase_at((*self.input_to_row.get(input).unwrap()).try_into().unwrap());
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

    let M31_112558620 = PackedM31::broadcast(M31::from(112558620));
    let M31_521092554 = PackedM31::broadcast(M31::from(521092554));
    let bitwise_xor_8_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "bitwise_xor_8_0".to_owned(),
    });
    let bitwise_xor_8_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "bitwise_xor_8_1".to_owned(),
    });
    let bitwise_xor_8_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "bitwise_xor_8_2".to_owned(),
    });

    (trace.par_iter_mut(), lookup_data.par_iter_mut())
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let bitwise_xor_8_0 = bitwise_xor_8_0.packed_at(row_index);
            let bitwise_xor_8_1 = bitwise_xor_8_1.packed_at(row_index);
            let bitwise_xor_8_2 = bitwise_xor_8_2.packed_at(row_index);
            let multiplicity_0_col0 = *mults[0].get(row_index).unwrap_or(&PackedM31::zero());
            *row[0] = multiplicity_0_col0;
            let multiplicity_1_col1 = *mults[1].get(row_index).unwrap_or(&PackedM31::zero());
            *row[1] = multiplicity_1_col1;
            *lookup_data.verify_bitwise_xor_8_0 = [
                M31_112558620,
                bitwise_xor_8_0,
                bitwise_xor_8_1,
                bitwise_xor_8_2,
            ];
            *lookup_data.verify_bitwise_xor_8_b_1 = [
                M31_521092554,
                bitwise_xor_8_0,
                bitwise_xor_8_1,
                bitwise_xor_8_2,
            ];
            *lookup_data.mults_0 = multiplicity_0_col0;
            *lookup_data.mults_1 = multiplicity_1_col1;
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    verify_bitwise_xor_8_0: Vec<[PackedM31; 4]>,
    verify_bitwise_xor_8_b_1: Vec<[PackedM31; 4]>,
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
        let mut logup_gen = unsafe { LogupTraceGenerator::uninitialized(LOG_SIZE) };

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_bitwise_xor_8_0,
            &self.lookup_data.verify_bitwise_xor_8_b_1,
            &self.lookup_data.mults_0,
            &self.lookup_data.mults_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mult0, mult1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(-(denom0 * *mult1 + denom1 * *mult0), denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}

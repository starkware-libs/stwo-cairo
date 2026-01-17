// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::verify_bitwise_xor_4::{
    Claim, InteractionClaim, LOG_SIZE, N_TRACE_COLUMNS,
};

use itertools::izip;

use crate::witness::prelude::*;

pub type InputType = [M31; 3];
pub type PackedInputType = [PackedM31; 3];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; 1],
    input_to_row: HashMap<[M31; 3], usize>,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(preprocessed_trace: Arc<PreProcessedTrace>) -> Self {
        let mults = from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE));
        let column_ids = [
            PreProcessedColumnId {
                id: "bitwise_xor_4_0".to_owned(),
            },
            PreProcessedColumnId {
                id: "bitwise_xor_4_1".to_owned(),
            },
            PreProcessedColumnId {
                id: "bitwise_xor_4_2".to_owned(),
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
        packed_inputs.into_iter().for_each(|packed_input| {
            packed_input.unpack().into_iter().for_each(|input| {
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

    let M31_45448144 = PackedM31::broadcast(M31::from(45448144));
    let bitwise_xor_4_0 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "bitwise_xor_4_0".to_owned(),
    });
    let bitwise_xor_4_1 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "bitwise_xor_4_1".to_owned(),
    });
    let bitwise_xor_4_2 = preprocessed_trace.get_column(&PreProcessedColumnId {
        id: "bitwise_xor_4_2".to_owned(),
    });

    izip!(trace.iter_mut(), lookup_data.iter_mut())
        .enumerate()
        .for_each(|(row_index, (row, lookup_data))| {
            let bitwise_xor_4_0 = bitwise_xor_4_0.packed_at(row_index);
            let bitwise_xor_4_1 = bitwise_xor_4_1.packed_at(row_index);
            let bitwise_xor_4_2 = bitwise_xor_4_2.packed_at(row_index);
            *lookup_data.verify_bitwise_xor_4_0 = [
                M31_45448144,
                bitwise_xor_4_0,
                bitwise_xor_4_1,
                bitwise_xor_4_2,
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
    verify_bitwise_xor_4_0: Vec<[PackedM31; 4]>,
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
        izip!(
            col_gen.iter_mut(),
            &self.lookup_data.verify_bitwise_xor_4_0,
            self.lookup_data.mults_0,
        )
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

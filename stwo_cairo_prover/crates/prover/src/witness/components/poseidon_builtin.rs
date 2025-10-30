// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_builtin::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, poseidon_aggregator};
use crate::witness::prelude::*;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}
impl ClaimGenerator {
    pub fn new(log_size: u32, poseidon_builtin_segment_start: u32) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            poseidon_builtin_segment_start,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        poseidon_aggregator_state: &mut poseidon_aggregator::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let log_size = self.log_size;

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            log_size,
            self.poseidon_builtin_segment_start,
            memory_address_to_id_state,
            poseidon_aggregator_state,
        );
        sub_component_inputs
            .memory_address_to_id
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .poseidon_aggregator
            .iter()
            .for_each(|inputs| {
                poseidon_aggregator_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim {
                log_size,
                poseidon_builtin_segment_start: self.poseidon_builtin_segment_start,
            },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 6],
    poseidon_aggregator: [Vec<poseidon_aggregator::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    poseidon_builtin_segment_start: u32,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    poseidon_aggregator_state: &mut poseidon_aggregator::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_5 = PackedM31::broadcast(M31::from(5));
    let M31_6 = PackedM31::broadcast(M31::from(6));
    let seq = Seq::new(log_size);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs))| {
                let seq = seq.packed_at(row_index);

                // Read Id.

                let memory_address_to_id_value_tmp_51986_0 = memory_address_to_id_state
                    .deduce_output(
                        ((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                            + ((seq) * (M31_6))),
                    );
                let input_state_0_id_col0 = memory_address_to_id_value_tmp_51986_0;
                *row[0] = input_state_0_id_col0;
                *sub_component_inputs.memory_address_to_id[0] =
                    ((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)));
                *lookup_data.memory_address_to_id_0 = [
                    ((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6))),
                    input_state_0_id_col0,
                ];

                // Read Id.

                let memory_address_to_id_value_tmp_51986_2 = memory_address_to_id_state
                    .deduce_output(
                        (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                            + ((seq) * (M31_6)))
                            + (M31_1)),
                    );
                let input_state_1_id_col1 = memory_address_to_id_value_tmp_51986_2;
                *row[1] = input_state_1_id_col1;
                *sub_component_inputs.memory_address_to_id[1] =
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_1));
                *lookup_data.memory_address_to_id_1 = [
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_1)),
                    input_state_1_id_col1,
                ];

                // Read Id.

                let memory_address_to_id_value_tmp_51986_4 = memory_address_to_id_state
                    .deduce_output(
                        (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                            + ((seq) * (M31_6)))
                            + (M31_2)),
                    );
                let input_state_2_id_col2 = memory_address_to_id_value_tmp_51986_4;
                *row[2] = input_state_2_id_col2;
                *sub_component_inputs.memory_address_to_id[2] =
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_2));
                *lookup_data.memory_address_to_id_2 = [
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_2)),
                    input_state_2_id_col2,
                ];

                // Read Id.

                let memory_address_to_id_value_tmp_51986_6 = memory_address_to_id_state
                    .deduce_output(
                        (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                            + ((seq) * (M31_6)))
                            + (M31_3)),
                    );
                let output_state_0_id_col3 = memory_address_to_id_value_tmp_51986_6;
                *row[3] = output_state_0_id_col3;
                *sub_component_inputs.memory_address_to_id[3] =
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_3));
                *lookup_data.memory_address_to_id_3 = [
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_3)),
                    output_state_0_id_col3,
                ];

                // Read Id.

                let memory_address_to_id_value_tmp_51986_8 = memory_address_to_id_state
                    .deduce_output(
                        (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                            + ((seq) * (M31_6)))
                            + (M31_4)),
                    );
                let output_state_1_id_col4 = memory_address_to_id_value_tmp_51986_8;
                *row[4] = output_state_1_id_col4;
                *sub_component_inputs.memory_address_to_id[4] =
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_4));
                *lookup_data.memory_address_to_id_4 = [
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_4)),
                    output_state_1_id_col4,
                ];

                // Read Id.

                let memory_address_to_id_value_tmp_51986_10 = memory_address_to_id_state
                    .deduce_output(
                        (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                            + ((seq) * (M31_6)))
                            + (M31_5)),
                    );
                let output_state_2_id_col5 = memory_address_to_id_value_tmp_51986_10;
                *row[5] = output_state_2_id_col5;
                *sub_component_inputs.memory_address_to_id[5] =
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_5));
                *lookup_data.memory_address_to_id_5 = [
                    (((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start)))
                        + ((seq) * (M31_6)))
                        + (M31_5)),
                    output_state_2_id_col5,
                ];

                *sub_component_inputs.poseidon_aggregator[0] = (
                    [
                        input_state_0_id_col0,
                        input_state_1_id_col1,
                        input_state_2_id_col2,
                    ],
                    [
                        output_state_0_id_col3,
                        output_state_1_id_col4,
                        output_state_2_id_col5,
                    ],
                );
                *lookup_data.poseidon_aggregator_0 = [
                    input_state_0_id_col0,
                    input_state_1_id_col1,
                    input_state_2_id_col2,
                    output_state_0_id_col3,
                    output_state_1_id_col4,
                    output_state_2_id_col5,
                ];
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_address_to_id_3: Vec<[PackedM31; 2]>,
    memory_address_to_id_4: Vec<[PackedM31; 2]>,
    memory_address_to_id_5: Vec<[PackedM31; 2]>,
    poseidon_aggregator_0: Vec<[PackedM31; 6]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id: &relations::MemoryAddressToId,
        poseidon_aggregator: &relations::PoseidonAggregator,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_address_to_id_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.memory_address_to_id_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_4,
            &self.lookup_data.memory_address_to_id_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_aggregator_0,
        )
            .into_par_iter()
            .for_each(|(writer, values)| {
                let denom = poseidon_aggregator.combine(values);
                writer.write_frac(PackedQM31::one(), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

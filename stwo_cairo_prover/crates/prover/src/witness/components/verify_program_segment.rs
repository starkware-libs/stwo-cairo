// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::verify_program_segment::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big};
use crate::witness::prelude::*;

pub struct ClaimGenerator {
    pub log_size: u32,
    pub verify_program_segment_start: u32,
    preprocessed_trace: Arc<PreProcessedTrace>,
}

impl ClaimGenerator {
    pub fn new(
        log_size: u32,
        verify_program_segment_start: u32,
        preprocessed_trace: Arc<PreProcessedTrace>,
    ) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            verify_program_segment_start,
            preprocessed_trace,
        }
    }

    pub fn write_trace(
        self,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    ) -> (
        ComponentTrace<N_TRACE_COLUMNS>,
        Claim,
        InteractionClaimGenerator,
    ) {
        let log_size = self.log_size;

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            &self.preprocessed_trace,
            log_size,
            self.verify_program_segment_start,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );
        for inputs in sub_component_inputs.memory_address_to_id {
            memory_address_to_id_state.add_packed_inputs(&inputs, 0);
        }
        for inputs in sub_component_inputs.memory_id_to_big {
            memory_id_to_big_state.add_packed_inputs(&inputs, 0);
        }

        (
            trace,
            Claim {
                log_size,
                verify_program_segment_start: self.verify_program_segment_start,
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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    preprocessed_trace: &PreProcessedTrace,
    log_size: u32,
    verify_program_segment_start: u32,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
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

    let M31_1444891767 = PackedM31::broadcast(M31::from(1444891767));
    let M31_1662111297 = PackedM31::broadcast(M31::from(1662111297));
    let seq = Seq::new(log_size);
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

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data, sub_component_inputs))| {
            let seq = seq.packed_at(row_index);
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

            // Mem Verify.

            // Read Id.

            let memory_address_to_id_value_tmp_9b2ce_0 = memory_address_to_id_state.deduce_output(
                ((PackedM31::broadcast(M31::from(verify_program_segment_start))) + (seq)),
            );
            let address_id_col0 = memory_address_to_id_value_tmp_9b2ce_0;
            *row[0] = address_id_col0;
            *sub_component_inputs.memory_address_to_id[0] =
                ((PackedM31::broadcast(M31::from(verify_program_segment_start))) + (seq));
            *lookup_data.memory_address_to_id_0 = [
                M31_1444891767,
                ((PackedM31::broadcast(M31::from(verify_program_segment_start))) + (seq)),
                address_id_col0,
            ];

            *sub_component_inputs.memory_id_to_big[0] = address_id_col0;
            *lookup_data.memory_id_to_big_0 = [
                M31_1662111297,
                address_id_col0,
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
        });

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 3]>,
    memory_id_to_big_0: Vec<[PackedM31; 30]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
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
        let mut logup_gen = unsafe { LogupTraceGenerator::uninitialized(self.log_size) };

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}

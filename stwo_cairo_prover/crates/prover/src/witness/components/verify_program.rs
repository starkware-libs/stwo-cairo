// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::verify_program::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big};
use crate::witness::prelude::*;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub verify_program_segment_start: u32,
}

impl ClaimGenerator {
    pub fn new(log_size: u32, verify_program_segment_start: u32) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            verify_program_segment_start,
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
            log_size,
            self.verify_program_segment_start,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );
        // Memory sub-component inputs are conditional on cond (is_active).
        // Only add inputs for lanes where cond=1 (actual program entries, not padding).
        let cond_values: Vec<PackedM31> = lookup_data
            .memory_address_to_id_0
            .iter()
            .map(|v| v[0])
            .collect();
        for (inputs, cond) in sub_component_inputs
            .memory_address_to_id
            .iter()
            .flat_map(|v| v.iter())
            .zip(cond_values.iter())
        {
            for (input, c) in inputs.to_array().iter().zip(cond.to_array().iter()) {
                if c.0 != 0 {
                    memory_address_to_id_state.add_input(input);
                }
            }
        }
        for (inputs, cond) in sub_component_inputs
            .memory_id_to_big
            .iter()
            .flat_map(|v| v.iter())
            .zip(cond_values.iter())
        {
            for (input, c) in inputs.to_array().iter().zip(cond.to_array().iter()) {
                if c.0 != 0 {
                    memory_id_to_big_state.add_input(input);
                }
            }
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
    let curr_program_cols: [ProgramColumn; 29] = std::array::from_fn(ProgramColumn::new);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data, sub_component_inputs))| {
            let seq = seq.packed_at(row_index);
            let limbs: [PackedM31; 28] =
                std::array::from_fn(|i| curr_program_cols[i].packed_at(row_index));
            let cond = curr_program_cols[28].packed_at(row_index);

            // Mem Verify Cond.
            // For padding rows (cond=0), use a safe address to avoid out-of-bounds access.
            let safe_addr = PackedM31::broadcast(M31::from(verify_program_segment_start));
            let addr = safe_addr + seq * cond;
            let address_id_col0 = memory_address_to_id_state.deduce_output(addr);
            *row[0] = address_id_col0;

            let real_addr = PackedM31::broadcast(M31::from(verify_program_segment_start)) + seq;
            *sub_component_inputs.memory_address_to_id[0] = real_addr;
            *lookup_data.memory_address_to_id_0 =
                [cond, M31_1444891767, real_addr, address_id_col0];
            *sub_component_inputs.memory_id_to_big[0] = address_id_col0;
            *lookup_data.memory_id_to_big_0 = [
                M31_1662111297,
                address_id_col0,
                limbs[0],
                limbs[1],
                limbs[2],
                limbs[3],
                limbs[4],
                limbs[5],
                limbs[6],
                limbs[7],
                limbs[8],
                limbs[9],
                limbs[10],
                limbs[11],
                limbs[12],
                limbs[13],
                limbs[14],
                limbs[15],
                limbs[16],
                limbs[17],
                limbs[18],
                limbs[19],
                limbs[20],
                limbs[21],
                limbs[22],
                limbs[23],
                limbs[24],
                limbs[25],
                limbs[26],
                limbs[27],
            ];
        });

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    // [cond, relation_id, address, id]
    memory_address_to_id_0: Vec<[PackedM31; 4]>,
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

        // Sum the two memory logup terms in pairs, both with numerator=cond.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let cond: PackedQM31 = values0[0].into();
                // values0 = [cond, relation_id, address, id] -> combine over [1..4].
                let denom0: PackedQM31 =
                    common_lookup_elements.combine(&[values0[1], values0[2], values0[3]]);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                // cond/denom0 + cond/denom1 = cond * (denom0 + denom1) / (denom0 * denom1)
                writer.write_frac(cond * (denom0 + denom1), denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}

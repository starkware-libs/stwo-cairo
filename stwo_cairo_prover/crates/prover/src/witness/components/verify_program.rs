// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::verify_program::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big, program_component};
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
        program_component_state: &program_component::ClaimGenerator,
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
            program_component_state,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );
        for inputs in sub_component_inputs.program_component {
            program_component_state.add_packed_inputs(&inputs, 0);
        }
        // Memory sub-component inputs are conditional on cond (is_active).
        // Only add inputs for lanes where cond=1 (actual program entries, not padding).
        let cond_values: Vec<PackedM31> = lookup_data
            .program_component_0
            .iter()
            .map(|v| v[30])
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
    program_component: [Vec<program_component::PackedInputType>; 1],
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
    program_component_state: &program_component::ClaimGenerator,
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
    let M31_1942035206 = PackedM31::broadcast(M31::from(1942035206));
    let seq = Seq::new(log_size);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(|(row_index, (row, lookup_data, sub_component_inputs))| {
            let seq = seq.packed_at(row_index);
            *sub_component_inputs.program_component[0] = [seq];
            let program_component_output_tmp_49bd6_0 = PackedProgramComponent::deduce_output([seq]);
            let program_component_output_limb_0_col0 =
                program_component_output_tmp_49bd6_0.0.get_m31(0);
            *row[0] = program_component_output_limb_0_col0;
            let program_component_output_limb_1_col1 =
                program_component_output_tmp_49bd6_0.0.get_m31(1);
            *row[1] = program_component_output_limb_1_col1;
            let program_component_output_limb_2_col2 =
                program_component_output_tmp_49bd6_0.0.get_m31(2);
            *row[2] = program_component_output_limb_2_col2;
            let program_component_output_limb_3_col3 =
                program_component_output_tmp_49bd6_0.0.get_m31(3);
            *row[3] = program_component_output_limb_3_col3;
            let program_component_output_limb_4_col4 =
                program_component_output_tmp_49bd6_0.0.get_m31(4);
            *row[4] = program_component_output_limb_4_col4;
            let program_component_output_limb_5_col5 =
                program_component_output_tmp_49bd6_0.0.get_m31(5);
            *row[5] = program_component_output_limb_5_col5;
            let program_component_output_limb_6_col6 =
                program_component_output_tmp_49bd6_0.0.get_m31(6);
            *row[6] = program_component_output_limb_6_col6;
            let program_component_output_limb_7_col7 =
                program_component_output_tmp_49bd6_0.0.get_m31(7);
            *row[7] = program_component_output_limb_7_col7;
            let program_component_output_limb_8_col8 =
                program_component_output_tmp_49bd6_0.0.get_m31(8);
            *row[8] = program_component_output_limb_8_col8;
            let program_component_output_limb_9_col9 =
                program_component_output_tmp_49bd6_0.0.get_m31(9);
            *row[9] = program_component_output_limb_9_col9;
            let program_component_output_limb_10_col10 =
                program_component_output_tmp_49bd6_0.0.get_m31(10);
            *row[10] = program_component_output_limb_10_col10;
            let program_component_output_limb_11_col11 =
                program_component_output_tmp_49bd6_0.0.get_m31(11);
            *row[11] = program_component_output_limb_11_col11;
            let program_component_output_limb_12_col12 =
                program_component_output_tmp_49bd6_0.0.get_m31(12);
            *row[12] = program_component_output_limb_12_col12;
            let program_component_output_limb_13_col13 =
                program_component_output_tmp_49bd6_0.0.get_m31(13);
            *row[13] = program_component_output_limb_13_col13;
            let program_component_output_limb_14_col14 =
                program_component_output_tmp_49bd6_0.0.get_m31(14);
            *row[14] = program_component_output_limb_14_col14;
            let program_component_output_limb_15_col15 =
                program_component_output_tmp_49bd6_0.0.get_m31(15);
            *row[15] = program_component_output_limb_15_col15;
            let program_component_output_limb_16_col16 =
                program_component_output_tmp_49bd6_0.0.get_m31(16);
            *row[16] = program_component_output_limb_16_col16;
            let program_component_output_limb_17_col17 =
                program_component_output_tmp_49bd6_0.0.get_m31(17);
            *row[17] = program_component_output_limb_17_col17;
            let program_component_output_limb_18_col18 =
                program_component_output_tmp_49bd6_0.0.get_m31(18);
            *row[18] = program_component_output_limb_18_col18;
            let program_component_output_limb_19_col19 =
                program_component_output_tmp_49bd6_0.0.get_m31(19);
            *row[19] = program_component_output_limb_19_col19;
            let program_component_output_limb_20_col20 =
                program_component_output_tmp_49bd6_0.0.get_m31(20);
            *row[20] = program_component_output_limb_20_col20;
            let program_component_output_limb_21_col21 =
                program_component_output_tmp_49bd6_0.0.get_m31(21);
            *row[21] = program_component_output_limb_21_col21;
            let program_component_output_limb_22_col22 =
                program_component_output_tmp_49bd6_0.0.get_m31(22);
            *row[22] = program_component_output_limb_22_col22;
            let program_component_output_limb_23_col23 =
                program_component_output_tmp_49bd6_0.0.get_m31(23);
            *row[23] = program_component_output_limb_23_col23;
            let program_component_output_limb_24_col24 =
                program_component_output_tmp_49bd6_0.0.get_m31(24);
            *row[24] = program_component_output_limb_24_col24;
            let program_component_output_limb_25_col25 =
                program_component_output_tmp_49bd6_0.0.get_m31(25);
            *row[25] = program_component_output_limb_25_col25;
            let program_component_output_limb_26_col26 =
                program_component_output_tmp_49bd6_0.0.get_m31(26);
            *row[26] = program_component_output_limb_26_col26;
            let program_component_output_limb_27_col27 =
                program_component_output_tmp_49bd6_0.0.get_m31(27);
            *row[27] = program_component_output_limb_27_col27;
            let program_component_output_limb_28_col28 = program_component_output_tmp_49bd6_0.1;
            *row[28] = program_component_output_limb_28_col28;
            *lookup_data.program_component_0 = [
                M31_1942035206,
                seq,
                program_component_output_limb_0_col0,
                program_component_output_limb_1_col1,
                program_component_output_limb_2_col2,
                program_component_output_limb_3_col3,
                program_component_output_limb_4_col4,
                program_component_output_limb_5_col5,
                program_component_output_limb_6_col6,
                program_component_output_limb_7_col7,
                program_component_output_limb_8_col8,
                program_component_output_limb_9_col9,
                program_component_output_limb_10_col10,
                program_component_output_limb_11_col11,
                program_component_output_limb_12_col12,
                program_component_output_limb_13_col13,
                program_component_output_limb_14_col14,
                program_component_output_limb_15_col15,
                program_component_output_limb_16_col16,
                program_component_output_limb_17_col17,
                program_component_output_limb_18_col18,
                program_component_output_limb_19_col19,
                program_component_output_limb_20_col20,
                program_component_output_limb_21_col21,
                program_component_output_limb_22_col22,
                program_component_output_limb_23_col23,
                program_component_output_limb_24_col24,
                program_component_output_limb_25_col25,
                program_component_output_limb_26_col26,
                program_component_output_limb_27_col27,
                program_component_output_limb_28_col28,
            ];

            // Mem Verify Cond.
            // For padding rows (cond=0), use a safe address to avoid out-of-bounds access.
            let cond = program_component_output_limb_28_col28;
            let safe_addr = PackedM31::broadcast(M31::from(verify_program_segment_start));
            let addr = safe_addr + seq * cond;
            let memory_address_to_id_value_tmp_49bd6_1 =
                memory_address_to_id_state.deduce_output(addr);
            let address_id_col29 = memory_address_to_id_value_tmp_49bd6_1;
            *row[29] = address_id_col29;
            *sub_component_inputs.memory_address_to_id[0] =
                ((PackedM31::broadcast(M31::from(verify_program_segment_start))) + (seq));
            *lookup_data.memory_address_to_id_0 = [
                M31_1444891767,
                ((PackedM31::broadcast(M31::from(verify_program_segment_start))) + (seq)),
                address_id_col29,
            ];
            *sub_component_inputs.memory_id_to_big[0] = address_id_col29;
            *lookup_data.memory_id_to_big_0 = [
                M31_1662111297,
                address_id_col29,
                program_component_output_limb_0_col0,
                program_component_output_limb_1_col1,
                program_component_output_limb_2_col2,
                program_component_output_limb_3_col3,
                program_component_output_limb_4_col4,
                program_component_output_limb_5_col5,
                program_component_output_limb_6_col6,
                program_component_output_limb_7_col7,
                program_component_output_limb_8_col8,
                program_component_output_limb_9_col9,
                program_component_output_limb_10_col10,
                program_component_output_limb_11_col11,
                program_component_output_limb_12_col12,
                program_component_output_limb_13_col13,
                program_component_output_limb_14_col14,
                program_component_output_limb_15_col15,
                program_component_output_limb_16_col16,
                program_component_output_limb_17_col17,
                program_component_output_limb_18_col18,
                program_component_output_limb_19_col19,
                program_component_output_limb_20_col20,
                program_component_output_limb_21_col21,
                program_component_output_limb_22_col22,
                program_component_output_limb_23_col23,
                program_component_output_limb_24_col24,
                program_component_output_limb_25_col25,
                program_component_output_limb_26_col26,
                program_component_output_limb_27_col27,
            ];
        });

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 3]>,
    memory_id_to_big_0: Vec<[PackedM31; 30]>,
    program_component_0: Vec<[PackedM31; 31]>,
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
        // program_component has numerator=1, memory_address_to_id has numerator=cond.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.program_component_0,
            &self.lookup_data.memory_address_to_id_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let cond: PackedQM31 = values0[30].into();
                let denom0: PackedQM31 = common_lookup_elements.combine(values0);
                let denom1: PackedQM31 = common_lookup_elements.combine(values1);
                // 1/denom0 + cond/denom1 = (cond * denom0 + denom1) / (denom0 * denom1)
                writer.write_frac(cond * denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term with numerator=cond.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.program_component_0,
        )
            .into_par_iter()
            .for_each(|(writer, values, pc_values)| {
                let cond: PackedQM31 = pc_values[30].into();
                let denom = common_lookup_elements.combine(values);
                writer.write_frac(cond, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        (trace, InteractionClaim { claimed_sum })
    }
}

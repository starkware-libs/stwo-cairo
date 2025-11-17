// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::range_check_builtin_bits_96::{
    Claim, InteractionClaim, N_TRACE_COLUMNS,
};

use crate::witness::components::{memory_address_to_id, memory_id_to_big, range_check_6};
use crate::witness::prelude::*;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub range_check96_builtin_segment_start: u32,
}

impl ClaimGenerator {
    pub fn new(log_size: u32, range_check96_builtin_segment_start: u32) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            range_check96_builtin_segment_start,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_6_state: &range_check_6::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let log_size = self.log_size;

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            log_size,
            self.range_check96_builtin_segment_start,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_6_state,
        );
        sub_component_inputs
            .memory_address_to_id
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_6
            .iter()
            .for_each(|inputs| {
                range_check_6_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_id_to_big
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim {
                log_size,
                range_check96_builtin_segment_start: self.range_check96_builtin_segment_start,
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
    range_check_6: [Vec<range_check_6::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    range_check96_builtin_segment_start: u32,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_6_state: &range_check_6::ClaimGenerator,
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

    let M31_0 = PackedM31::broadcast(M31::from(0));
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

            // Read Positive Num Bits 96.

            // Read Id.

            let memory_address_to_id_value_tmp_6e07e_0 = memory_address_to_id_state.deduce_output(
                ((PackedM31::broadcast(M31::from(range_check96_builtin_segment_start))) + (seq)),
            );
            let value_id_col0 = memory_address_to_id_value_tmp_6e07e_0;
            *row[0] = value_id_col0;
            *sub_component_inputs.memory_address_to_id[0] =
                ((PackedM31::broadcast(M31::from(range_check96_builtin_segment_start))) + (seq));
            *lookup_data.memory_address_to_id_0 = [
                ((PackedM31::broadcast(M31::from(range_check96_builtin_segment_start))) + (seq)),
                value_id_col0,
            ];

            // Read Positive Known Id Num Bits 96.

            let memory_id_to_big_value_tmp_6e07e_2 =
                memory_id_to_big_state.deduce_output(value_id_col0);
            let value_limb_0_col1 = memory_id_to_big_value_tmp_6e07e_2.get_m31(0);
            *row[1] = value_limb_0_col1;
            let value_limb_1_col2 = memory_id_to_big_value_tmp_6e07e_2.get_m31(1);
            *row[2] = value_limb_1_col2;
            let value_limb_2_col3 = memory_id_to_big_value_tmp_6e07e_2.get_m31(2);
            *row[3] = value_limb_2_col3;
            let value_limb_3_col4 = memory_id_to_big_value_tmp_6e07e_2.get_m31(3);
            *row[4] = value_limb_3_col4;
            let value_limb_4_col5 = memory_id_to_big_value_tmp_6e07e_2.get_m31(4);
            *row[5] = value_limb_4_col5;
            let value_limb_5_col6 = memory_id_to_big_value_tmp_6e07e_2.get_m31(5);
            *row[6] = value_limb_5_col6;
            let value_limb_6_col7 = memory_id_to_big_value_tmp_6e07e_2.get_m31(6);
            *row[7] = value_limb_6_col7;
            let value_limb_7_col8 = memory_id_to_big_value_tmp_6e07e_2.get_m31(7);
            *row[8] = value_limb_7_col8;
            let value_limb_8_col9 = memory_id_to_big_value_tmp_6e07e_2.get_m31(8);
            *row[9] = value_limb_8_col9;
            let value_limb_9_col10 = memory_id_to_big_value_tmp_6e07e_2.get_m31(9);
            *row[10] = value_limb_9_col10;
            let value_limb_10_col11 = memory_id_to_big_value_tmp_6e07e_2.get_m31(10);
            *row[11] = value_limb_10_col11;

            // Range Check Last Limb Bits In Ms Limb 6.

            *sub_component_inputs.range_check_6[0] = [value_limb_10_col11];
            *lookup_data.range_check_6_0 = [value_limb_10_col11];

            *sub_component_inputs.memory_id_to_big[0] = value_id_col0;
            *lookup_data.memory_id_to_big_0 = [
                value_id_col0,
                value_limb_0_col1,
                value_limb_1_col2,
                value_limb_2_col3,
                value_limb_3_col4,
                value_limb_4_col5,
                value_limb_5_col6,
                value_limb_6_col7,
                value_limb_7_col8,
                value_limb_8_col9,
                value_limb_9_col10,
                value_limb_10_col11,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
            ];
            let read_positive_known_id_num_bits_96_output_tmp_6e07e_3 =
                PackedFelt252::from_limbs([
                    value_limb_0_col1,
                    value_limb_1_col2,
                    value_limb_2_col3,
                    value_limb_3_col4,
                    value_limb_4_col5,
                    value_limb_5_col6,
                    value_limb_6_col7,
                    value_limb_7_col8,
                    value_limb_8_col9,
                    value_limb_9_col10,
                    value_limb_10_col11,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ]);

            let read_positive_num_bits_96_output_tmp_6e07e_4 = (
                read_positive_known_id_num_bits_96_output_tmp_6e07e_3,
                value_id_col0,
            );
        });

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    range_check_6_0: Vec<[PackedM31; 1]>,
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
        range_check_6: &relations::RangeCheck_6,
        memory_id_to_big: &relations::MemoryIdToBig,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.range_check_6_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = range_check_6.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (col_gen.par_iter_mut(), &self.lookup_data.memory_id_to_big_0)
            .into_par_iter()
            .for_each(|(writer, values)| {
                let denom = memory_id_to_big.combine(values);
                writer.write_frac(PackedQM31::one(), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

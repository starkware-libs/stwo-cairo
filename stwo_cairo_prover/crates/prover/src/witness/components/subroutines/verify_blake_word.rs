// AIR version 97774321-dirty
#![allow(unused_parens)]
use cairo_air::components::verify_blake_word::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big, range_check_7_2_5};
use crate::witness::prelude::*;

pub type InputType = (M31, UInt32);
pub type PackedInputType = (PackedM31, PackedUInt32);

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_7_2_5_state,
        );
        sub_component_inputs
            .range_check_7_2_5
            .iter()
            .for_each(|inputs| {
                range_check_7_2_5_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_address_to_id
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_id_to_big
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    range_check_7_2_5: [Vec<range_check_7_2_5::PackedInputType>; 1],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs, verify_blake_word_input))| {
                let low_7_ms_bits_tmp_d476b_0 = ((verify_blake_word_input.1.low()) >> (UInt16_9));
                let low_7_ms_bits_col0 = low_7_ms_bits_tmp_d476b_0.as_m31();
                *row[0] = low_7_ms_bits_col0;
                let high_14_ms_bits_tmp_d476b_1 =
                    ((verify_blake_word_input.1.high()) >> (UInt16_2));
                let high_14_ms_bits_col1 = high_14_ms_bits_tmp_d476b_1.as_m31();
                *row[1] = high_14_ms_bits_col1;
                let high_5_ms_bits_tmp_d476b_2 = ((high_14_ms_bits_tmp_d476b_1) >> (UInt16_9));
                let high_5_ms_bits_col2 = high_5_ms_bits_tmp_d476b_2.as_m31();
                *row[2] = high_5_ms_bits_col2;
                *sub_component_inputs.range_check_7_2_5[0] = [
                    low_7_ms_bits_col0,
                    ((verify_blake_word_input.1.high().as_m31())
                        - ((high_14_ms_bits_col1) * (M31_4))),
                    high_5_ms_bits_col2,
                ];
                *lookup_data.range_check_7_2_5_0 = [
                    low_7_ms_bits_col0,
                    ((verify_blake_word_input.1.high().as_m31())
                        - ((high_14_ms_bits_col1) * (M31_4))),
                    high_5_ms_bits_col2,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_d476b_3 =
                    memory_address_to_id_state.deduce_output(verify_blake_word_input.0);
                let id_col3 = memory_address_to_id_value_tmp_d476b_3;
                *row[3] = id_col3;
                *sub_component_inputs.memory_address_to_id[0] = verify_blake_word_input.0;
                *lookup_data.memory_address_to_id_0 = [verify_blake_word_input.0, id_col3];

                *sub_component_inputs.memory_id_to_big[0] = id_col3;
                *lookup_data.memory_id_to_big_0 = [
                    id_col3,
                    ((verify_blake_word_input.1.low().as_m31())
                        - ((low_7_ms_bits_col0) * (M31_512))),
                    ((low_7_ms_bits_col0)
                        + (((verify_blake_word_input.1.high().as_m31())
                            - ((high_14_ms_bits_col1) * (M31_4)))
                            * (M31_128))),
                    ((high_14_ms_bits_col1) - ((high_5_ms_bits_col2) * (M31_512))),
                    high_5_ms_bits_col2,
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
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    range_check_7_2_5_0: Vec<[PackedM31; 3]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_7_2_5: &relations::RangeCheck_7_2_5,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_0,
            &self.lookup_data.memory_address_to_id_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
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

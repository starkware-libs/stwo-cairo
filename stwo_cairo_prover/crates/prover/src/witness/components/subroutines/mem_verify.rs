// AIR version 97774321-dirty
#![allow(unused_parens)]
use cairo_air::components::mem_verify::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big};
use crate::witness::prelude::*;

pub type InputType = (M31, Felt252);
pub type PackedInputType = (PackedM31, PackedFelt252);

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
        );
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

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs, mem_verify_input))| {
                // Read Id.

                let memory_address_to_id_value_tmp_5789d_0 =
                    memory_address_to_id_state.deduce_output(mem_verify_input.0);
                let id_col0 = memory_address_to_id_value_tmp_5789d_0;
                *row[0] = id_col0;
                *sub_component_inputs.memory_address_to_id[0] = mem_verify_input.0;
                *lookup_data.memory_address_to_id_0 = [mem_verify_input.0, id_col0];

                *sub_component_inputs.memory_id_to_big[0] = id_col0;
                *lookup_data.memory_id_to_big_0 = [
                    id_col0,
                    mem_verify_input.1.get_m31(0),
                    mem_verify_input.1.get_m31(1),
                    mem_verify_input.1.get_m31(2),
                    mem_verify_input.1.get_m31(3),
                    mem_verify_input.1.get_m31(4),
                    mem_verify_input.1.get_m31(5),
                    mem_verify_input.1.get_m31(6),
                    mem_verify_input.1.get_m31(7),
                    mem_verify_input.1.get_m31(8),
                    mem_verify_input.1.get_m31(9),
                    mem_verify_input.1.get_m31(10),
                    mem_verify_input.1.get_m31(11),
                    mem_verify_input.1.get_m31(12),
                    mem_verify_input.1.get_m31(13),
                    mem_verify_input.1.get_m31(14),
                    mem_verify_input.1.get_m31(15),
                    mem_verify_input.1.get_m31(16),
                    mem_verify_input.1.get_m31(17),
                    mem_verify_input.1.get_m31(18),
                    mem_verify_input.1.get_m31(19),
                    mem_verify_input.1.get_m31(20),
                    mem_verify_input.1.get_m31(21),
                    mem_verify_input.1.get_m31(22),
                    mem_verify_input.1.get_m31(23),
                    mem_verify_input.1.get_m31(24),
                    mem_verify_input.1.get_m31(25),
                    mem_verify_input.1.get_m31(26),
                    mem_verify_input.1.get_m31(27),
                ];
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
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
        memory_id_to_big: &relations::MemoryIdToBig,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

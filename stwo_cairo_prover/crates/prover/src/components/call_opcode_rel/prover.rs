#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col, Column};
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{
    bit_reverse_coset_to_circle_domain_order, bit_reverse_index, coset_index_to_circle_domain_index,
};

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::preprocessed::PreProcessedColumn;
use crate::components::utils::{pack_values, Enabler};
use crate::components::{memory_address_to_id, memory_id_to_big, verify_instruction};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 18;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        verify_instruction_state: &verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);

        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);
        let (trace, lookup_data) = write_trace_simd(
            n_rows,
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
            verify_instruction_state,
        );

        tree_builder.extend_evals(trace.to_evals());
        (
            Claim { n_rows },
            InteractionClaimGenerator {
                n_rows,
                lookup_data,
            },
        )
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    verify_instruction_state: &verify_instruction::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(|(((row_index, row), call_opcode_rel_input), lookup_data)| {
            let input_tmp_4997f_0 = call_opcode_rel_input;
            let input_pc_col0 = input_tmp_4997f_0.pc;
            *row[0] = input_pc_col0;
            let input_ap_col1 = input_tmp_4997f_0.ap;
            *row[1] = input_ap_col1;
            let input_fp_col2 = input_tmp_4997f_0.fp;
            *row[2] = input_fp_col2;

            // Decode Instruction.

            let memory_address_to_id_value_tmp_4997f_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_4997f_2 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_4997f_1);
            let verify_instruction_inputs_0 = (
                input_pc_col0,
                [M31_32768, M31_32769, M31_32769],
                [
                    M31_0, M31_0, M31_1, M31_0, M31_0, M31_0, M31_0, M31_0, M31_1, M31_0, M31_0,
                    M31_0, M31_1, M31_0, M31_0,
                ],
            )
                .unpack();
            *lookup_data.verify_instruction_0 = [
                input_pc_col0,
                M31_32768,
                M31_32769,
                M31_32769,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
            ];

            // Read Positive Num Bits 27.

            let memory_address_to_id_value_tmp_4997f_3 =
                memory_address_to_id_state.deduce_output(input_ap_col1);
            let memory_id_to_big_value_tmp_4997f_4 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_4997f_3);
            let stored_fp_id_col3 = memory_address_to_id_value_tmp_4997f_3;
            *row[3] = stored_fp_id_col3;
            let memory_address_to_id_inputs_0 = input_ap_col1.unpack();
            *lookup_data.memory_address_to_id_0 = [input_ap_col1, stored_fp_id_col3];
            let stored_fp_limb_0_col4 = memory_id_to_big_value_tmp_4997f_4.get_m31(0);
            *row[4] = stored_fp_limb_0_col4;
            let stored_fp_limb_1_col5 = memory_id_to_big_value_tmp_4997f_4.get_m31(1);
            *row[5] = stored_fp_limb_1_col5;
            let stored_fp_limb_2_col6 = memory_id_to_big_value_tmp_4997f_4.get_m31(2);
            *row[6] = stored_fp_limb_2_col6;
            let memory_id_to_big_inputs_0 = stored_fp_id_col3.unpack();
            *lookup_data.memory_id_to_big_0 = [
                stored_fp_id_col3,
                stored_fp_limb_0_col4,
                stored_fp_limb_1_col5,
                stored_fp_limb_2_col6,
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
                M31_0,
            ];

            // Read Positive Num Bits 27.

            let memory_address_to_id_value_tmp_4997f_5 =
                memory_address_to_id_state.deduce_output(((input_ap_col1) + (M31_1)));
            let memory_id_to_big_value_tmp_4997f_6 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_4997f_5);
            let stored_ret_pc_id_col7 = memory_address_to_id_value_tmp_4997f_5;
            *row[7] = stored_ret_pc_id_col7;
            let memory_address_to_id_inputs_1 = ((input_ap_col1) + (M31_1)).unpack();
            *lookup_data.memory_address_to_id_1 =
                [((input_ap_col1) + (M31_1)), stored_ret_pc_id_col7];
            let stored_ret_pc_limb_0_col8 = memory_id_to_big_value_tmp_4997f_6.get_m31(0);
            *row[8] = stored_ret_pc_limb_0_col8;
            let stored_ret_pc_limb_1_col9 = memory_id_to_big_value_tmp_4997f_6.get_m31(1);
            *row[9] = stored_ret_pc_limb_1_col9;
            let stored_ret_pc_limb_2_col10 = memory_id_to_big_value_tmp_4997f_6.get_m31(2);
            *row[10] = stored_ret_pc_limb_2_col10;
            let memory_id_to_big_inputs_1 = stored_ret_pc_id_col7.unpack();
            *lookup_data.memory_id_to_big_1 = [
                stored_ret_pc_id_col7,
                stored_ret_pc_limb_0_col8,
                stored_ret_pc_limb_1_col9,
                stored_ret_pc_limb_2_col10,
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
                M31_0,
            ];

            // Read Small.

            let memory_address_to_id_value_tmp_4997f_7 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memory_id_to_big_value_tmp_4997f_8 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_4997f_7);
            let distance_to_next_pc_id_col11 = memory_address_to_id_value_tmp_4997f_7;
            *row[11] = distance_to_next_pc_id_col11;
            let memory_address_to_id_inputs_2 = ((input_pc_col0) + (M31_1)).unpack();
            *lookup_data.memory_address_to_id_2 =
                [((input_pc_col0) + (M31_1)), distance_to_next_pc_id_col11];

            // Cond Decode Small Sign.

            let msb_tmp_4997f_9 = memory_id_to_big_value_tmp_4997f_8.get_m31(27).eq(M31_256);
            let msb_col12 = msb_tmp_4997f_9.as_m31();
            *row[12] = msb_col12;
            let mid_limbs_set_tmp_4997f_10 =
                memory_id_to_big_value_tmp_4997f_8.get_m31(20).eq(M31_511);
            let mid_limbs_set_col13 = mid_limbs_set_tmp_4997f_10.as_m31();
            *row[13] = mid_limbs_set_col13;

            let distance_to_next_pc_limb_0_col14 = memory_id_to_big_value_tmp_4997f_8.get_m31(0);
            *row[14] = distance_to_next_pc_limb_0_col14;
            let distance_to_next_pc_limb_1_col15 = memory_id_to_big_value_tmp_4997f_8.get_m31(1);
            *row[15] = distance_to_next_pc_limb_1_col15;
            let distance_to_next_pc_limb_2_col16 = memory_id_to_big_value_tmp_4997f_8.get_m31(2);
            *row[16] = distance_to_next_pc_limb_2_col16;
            let memory_id_to_big_inputs_2 = distance_to_next_pc_id_col11.unpack();
            *lookup_data.memory_id_to_big_2 = [
                distance_to_next_pc_id_col11,
                distance_to_next_pc_limb_0_col14,
                distance_to_next_pc_limb_1_col15,
                distance_to_next_pc_limb_2_col16,
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                ((mid_limbs_set_col13) * (M31_511)),
                (((M31_136) * (msb_col12)) - (mid_limbs_set_col13)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col12) * (M31_256)),
            ];

            *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
            *lookup_data.opcodes_1 = [
                ((input_pc_col0)
                    + (((((distance_to_next_pc_limb_0_col14)
                        + ((distance_to_next_pc_limb_1_col15) * (M31_512)))
                        + ((distance_to_next_pc_limb_2_col16) * (M31_262144)))
                        - (msb_col12))
                        - ((M31_134217728) * (mid_limbs_set_col13)))),
                ((input_ap_col1) + (M31_2)),
                ((input_ap_col1) + (M31_2)),
            ];
            *row[17] = Enabler::new(n_rows).packed_at(row_index);

            // Add sub-components inputs.
            verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 19]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_instruction.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_address_to_id_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.memory_address_to_id_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(
                i,
                denom0 * Enabler::new(self.n_rows).packed_at(i) + denom1,
                denom0 * denom1,
            );
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.opcodes_1.iter().enumerate() {
            let denom = opcodes.combine(values);
            col_gen.write_frac(
                i,
                -PackedQM31::one() * Enabler::new(self.n_rows).packed_at(i),
                denom,
            );
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim {
            logup_sums: (claimed_sum, None),
        }
    }
}

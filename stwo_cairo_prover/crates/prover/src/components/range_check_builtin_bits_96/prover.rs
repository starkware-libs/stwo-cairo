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
use crate::cairo_air::preprocessed::{PreProcessedColumn, Seq};
use crate::components::utils::AtomicMultiplicityColumn;
use crate::components::{memory_address_to_id, memory_id_to_big, range_check_6};
use crate::relations;

const N_TRACE_COLUMNS: usize = 12;

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

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_6_state: &range_check_6::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = self.log_size;

        let (trace, lookup_data) = write_trace_simd(
            log_size,
            self.range_check96_builtin_segment_start,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_6_state,
        );

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
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, row), lookup_data)| {
            let seq = Seq::new(log_size).packed_at(row_index);

            // Read Positive Num Bits 96.

            let memory_address_to_id_value_tmp_fd7ee_0 = memory_address_to_id_state.deduce_output(
                ((PackedM31::broadcast(M31::from(range_check96_builtin_segment_start))) + (seq)),
            );
            let memory_id_to_big_value_tmp_fd7ee_1 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_fd7ee_0);
            let value_id_col0 = memory_address_to_id_value_tmp_fd7ee_0;
            *row[0] = value_id_col0;
            let memory_address_to_id_inputs_0 =
                ((PackedM31::broadcast(M31::from(range_check96_builtin_segment_start))) + (seq))
                    .unpack();
            *lookup_data.memory_address_to_id_0 = [
                ((PackedM31::broadcast(M31::from(range_check96_builtin_segment_start))) + (seq)),
                value_id_col0,
            ];
            let value_limb_0_col1 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(0);
            *row[1] = value_limb_0_col1;
            let value_limb_1_col2 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(1);
            *row[2] = value_limb_1_col2;
            let value_limb_2_col3 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(2);
            *row[3] = value_limb_2_col3;
            let value_limb_3_col4 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(3);
            *row[4] = value_limb_3_col4;
            let value_limb_4_col5 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(4);
            *row[5] = value_limb_4_col5;
            let value_limb_5_col6 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(5);
            *row[6] = value_limb_5_col6;
            let value_limb_6_col7 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(6);
            *row[7] = value_limb_6_col7;
            let value_limb_7_col8 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(7);
            *row[8] = value_limb_7_col8;
            let value_limb_8_col9 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(8);
            *row[9] = value_limb_8_col9;
            let value_limb_9_col10 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(9);
            *row[10] = value_limb_9_col10;
            let value_limb_10_col11 = memory_id_to_big_value_tmp_fd7ee_1.get_m31(10);
            *row[11] = value_limb_10_col11;

            // Range Check Last Limb Bits In Ms Limb 6.

            let range_check_6_inputs_0 = [value_limb_10_col11].unpack();
            *lookup_data.range_check_6_0 = [value_limb_10_col11];

            let memory_id_to_big_inputs_0 = value_id_col0.unpack();
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

            // Add sub-components inputs.
            #[allow(clippy::needless_range_loop)]
            for i in 0..N_LANES {
                memory_address_to_id_state.add_input(&memory_address_to_id_inputs_0[i]);
                range_check_6_state.add_input(&range_check_6_inputs_0[i]);
                memory_id_to_big_state.add_input(&memory_id_to_big_inputs_0[i]);
            }
        });

    (trace, lookup_data)
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
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_6: &relations::RangeCheck_6,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.range_check_6_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = range_check_6.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.memory_id_to_big_0.iter().enumerate() {
            let denom = memory_id_to_big.combine(values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

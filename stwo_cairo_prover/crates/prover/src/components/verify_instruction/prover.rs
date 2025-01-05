#![allow(unused_parens)]
#![allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet};
use std::iter::zip;
use std::vec;

use air_structs_derive::SubComponentInputs;
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
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::bit_reverse_coset_to_circle_domain_order;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};

use super::component::{Claim, InteractionClaim};
use crate::components::{
    memory_address_to_id, memory_id_to_big, pack_values, range_check_4_3, range_check_7_2_5,
};
use crate::relations;

pub type InputType = (M31, [M31; 3], [M31; 15]);
pub type PackedInputType = (PackedM31, [PackedM31; 3], [PackedM31; 15]);
const N_MULTIPLICITY_COLUMNS: usize = 1;
const N_TRACE_COLUMNS: usize = 28 + N_MULTIPLICITY_COLUMNS;

#[derive(Default)]
pub struct ClaimGenerator {
    /// A map from input to multiplicity.
    inputs: BTreeMap<InputType, u32>,
}
impl ClaimGenerator {
    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_4_3_state: &range_check_4_3::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (mut inputs, mut mults) = self
            .inputs
            .into_iter()
            .map(|(input, mult)| (input, M31(mult)))
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let n_calls = inputs.len();
        assert_ne!(n_calls, 0);
        let size = std::cmp::max(n_calls.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        let need_padding = n_calls != size;

        if need_padding {
            inputs.resize(size, *inputs.first().unwrap());
            mults.resize(size, M31::zero());
        }

        let packed_inputs = pack_values(&inputs);
        let packed_mults = pack_values(&mults);
        let (trace, sub_components_inputs, lookup_data) =
            write_trace_simd(packed_inputs, packed_mults, memory_address_to_id_state);

        sub_components_inputs
            .range_check_4_3_inputs
            .iter()
            .for_each(|inputs| {
                range_check_4_3_state.add_inputs(&inputs[..]);
            });
        sub_components_inputs
            .range_check_7_2_5_inputs
            .iter()
            .for_each(|inputs| {
                range_check_7_2_5_state.add_inputs(&inputs[..]);
            });
        sub_components_inputs
            .memory_address_to_id_inputs
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_inputs(&inputs[..]);
            });
        sub_components_inputs
            .memory_id_to_big_inputs
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_inputs(&inputs[..]);
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

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        for input in inputs {
            *self.inputs.entry(*input).or_default() += 1;
        }
    }
}

#[derive(SubComponentInputs, ParIterMut, IterMut, Uninitialized)]
pub struct SubComponentInputs {
    pub range_check_4_3_inputs: [Vec<range_check_4_3::InputType>; 1],
    pub range_check_7_2_5_inputs: [Vec<range_check_7_2_5::InputType>; 1],
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 1],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 1],
}

#[derive(ParIterMut, IterMut, Uninitialized)]
struct LookupData {
    pub memory_address_to_id: [Vec<[PackedM31; 2]>; 1],
    pub memory_id_to_big: [Vec<[PackedM31; 29]>; 1],
    pub range_check_4_3: [Vec<[PackedM31; 2]>; 1],
    pub range_check_7_2_5: [Vec<[PackedM31; 3]>; 1],
    pub verify_instruction: [Vec<[PackedM31; 19]>; 1],
    pub mults: Vec<PackedM31>,
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    mults: Vec<PackedM31>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    SubComponentInputs,
    LookupData,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_components_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_size),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_15 = PackedUInt16::broadcast(UInt16::from(15));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_511 = PackedUInt16::broadcast(UInt16::from(511));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    trace
        .par_iter_mut()
        .zip(inputs.par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(sub_components_inputs.par_iter_mut().chunks(N_LANES))
        .zip(mults.into_par_iter())
        .for_each(
            |(
                (((row, verify_instruction_input), lookup_data), mut sub_components_inputs),
                multiplicity,
            )| {
                let input_tmp_16a4f_0 = (
                    verify_instruction_input.0,
                    [
                        verify_instruction_input.1[0],
                        verify_instruction_input.1[1],
                        verify_instruction_input.1[2],
                    ],
                    [
                        verify_instruction_input.2[0],
                        verify_instruction_input.2[1],
                        verify_instruction_input.2[2],
                        verify_instruction_input.2[3],
                        verify_instruction_input.2[4],
                        verify_instruction_input.2[5],
                        verify_instruction_input.2[6],
                        verify_instruction_input.2[7],
                        verify_instruction_input.2[8],
                        verify_instruction_input.2[9],
                        verify_instruction_input.2[10],
                        verify_instruction_input.2[11],
                        verify_instruction_input.2[12],
                        verify_instruction_input.2[13],
                        verify_instruction_input.2[14],
                    ],
                );
                let input_col0 = input_tmp_16a4f_0.0;
                *row[0] = input_col0;
                let input_col1 = input_tmp_16a4f_0.1[0];
                *row[1] = input_col1;
                let input_col2 = input_tmp_16a4f_0.1[1];
                *row[2] = input_col2;
                let input_col3 = input_tmp_16a4f_0.1[2];
                *row[3] = input_col3;
                let input_col4 = input_tmp_16a4f_0.2[0];
                *row[4] = input_col4;
                let input_col5 = input_tmp_16a4f_0.2[1];
                *row[5] = input_col5;
                let input_col6 = input_tmp_16a4f_0.2[2];
                *row[6] = input_col6;
                let input_col7 = input_tmp_16a4f_0.2[3];
                *row[7] = input_col7;
                let input_col8 = input_tmp_16a4f_0.2[4];
                *row[8] = input_col8;
                let input_col9 = input_tmp_16a4f_0.2[5];
                *row[9] = input_col9;
                let input_col10 = input_tmp_16a4f_0.2[6];
                *row[10] = input_col10;
                let input_col11 = input_tmp_16a4f_0.2[7];
                *row[11] = input_col11;
                let input_col12 = input_tmp_16a4f_0.2[8];
                *row[12] = input_col12;
                let input_col13 = input_tmp_16a4f_0.2[9];
                *row[13] = input_col13;
                let input_col14 = input_tmp_16a4f_0.2[10];
                *row[14] = input_col14;
                let input_col15 = input_tmp_16a4f_0.2[11];
                *row[15] = input_col15;
                let input_col16 = input_tmp_16a4f_0.2[12];
                *row[16] = input_col16;
                let input_col17 = input_tmp_16a4f_0.2[13];
                *row[17] = input_col17;
                let input_col18 = input_tmp_16a4f_0.2[14];
                *row[18] = input_col18;

                // Encode Offsets.

                let offset0_low_tmp_16a4f_1 = ((PackedUInt16::from_m31(input_col1)) & (UInt16_511));
                let offset0_low_col19 = offset0_low_tmp_16a4f_1.as_m31();
                *row[19] = offset0_low_col19;
                let offset0_mid_tmp_16a4f_2 = ((PackedUInt16::from_m31(input_col1)) >> (UInt16_9));
                let offset0_mid_col20 = offset0_mid_tmp_16a4f_2.as_m31();
                *row[20] = offset0_mid_col20;
                let offset1_low_tmp_16a4f_3 = ((PackedUInt16::from_m31(input_col2)) & (UInt16_3));
                let offset1_low_col21 = offset1_low_tmp_16a4f_3.as_m31();
                *row[21] = offset1_low_col21;
                let offset1_mid_tmp_16a4f_4 =
                    (((PackedUInt16::from_m31(input_col2)) >> (UInt16_2)) & (UInt16_511));
                let offset1_mid_col22 = offset1_mid_tmp_16a4f_4.as_m31();
                *row[22] = offset1_mid_col22;
                let offset1_high_tmp_16a4f_5 =
                    ((PackedUInt16::from_m31(input_col2)) >> (UInt16_11));
                let offset1_high_col23 = offset1_high_tmp_16a4f_5.as_m31();
                *row[23] = offset1_high_col23;
                let offset2_low_tmp_16a4f_6 = ((PackedUInt16::from_m31(input_col3)) & (UInt16_15));
                let offset2_low_col24 = offset2_low_tmp_16a4f_6.as_m31();
                *row[24] = offset2_low_col24;
                let offset2_mid_tmp_16a4f_7 =
                    (((PackedUInt16::from_m31(input_col3)) >> (UInt16_4)) & (UInt16_511));
                let offset2_mid_col25 = offset2_mid_tmp_16a4f_7.as_m31();
                *row[25] = offset2_mid_col25;
                let offset2_high_tmp_16a4f_8 =
                    ((PackedUInt16::from_m31(input_col3)) >> (UInt16_13));
                let offset2_high_col26 = offset2_high_tmp_16a4f_8.as_m31();
                *row[26] = offset2_high_col26;

                for (i, &input) in [offset0_mid_col20, offset1_low_col21, offset1_high_col23]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_7_2_5_inputs[0] = input;
                }
                *lookup_data.range_check_7_2_5[0] =
                    [offset0_mid_col20, offset1_low_col21, offset1_high_col23];

                for (i, &input) in [offset2_low_col24, offset2_high_col26]
                    .unpack()
                    .iter()
                    .enumerate()
                {
                    *sub_components_inputs[i].range_check_4_3_inputs[0] = input;
                }
                *lookup_data.range_check_4_3[0] = [offset2_low_col24, offset2_high_col26];

                // Mem Verify.

                let memory_address_to_id_value_tmp_16a4f_9 =
                    memory_address_to_id_state.deduce_output(input_col0);
                let instruction_id_col27 = memory_address_to_id_value_tmp_16a4f_9;
                *row[27] = instruction_id_col27;
                for (i, &input) in input_col0.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_address_to_id_inputs[0] = input;
                }
                *lookup_data.memory_address_to_id[0] = [input_col0, instruction_id_col27];
                for (i, &input) in instruction_id_col27.unpack().iter().enumerate() {
                    *sub_components_inputs[i].memory_id_to_big_inputs[0] = input;
                }
                *lookup_data.memory_id_to_big[0] = [
                    instruction_id_col27,
                    offset0_low_col19,
                    ((offset0_mid_col20) + ((offset1_low_col21) * (M31_128))),
                    offset1_mid_col22,
                    ((offset1_high_col23) + ((offset2_low_col24) * (M31_32))),
                    offset2_mid_col25,
                    ((offset2_high_col26)
                        + (((((((M31_0) + ((input_col4) * (M31_8)))
                            + ((input_col5) * (M31_16)))
                            + ((input_col6) * (M31_32)))
                            + ((input_col7) * (M31_64)))
                            + ((input_col8) * (M31_128)))
                            + ((input_col9) * (M31_256)))),
                    ((((((((((M31_0) + ((input_col10) * (M31_1)))
                        + ((input_col11) * (M31_2)))
                        + ((input_col12) * (M31_4)))
                        + ((input_col13) * (M31_8)))
                        + ((input_col14) * (M31_16)))
                        + ((input_col15) * (M31_32)))
                        + ((input_col16) * (M31_64)))
                        + ((input_col17) * (M31_128)))
                        + ((input_col18) * (M31_256))),
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

                *lookup_data.verify_instruction[0] = [
                    input_col0,
                    input_col1,
                    input_col2,
                    input_col3,
                    input_col4,
                    input_col5,
                    input_col6,
                    input_col7,
                    input_col8,
                    input_col9,
                    input_col10,
                    input_col11,
                    input_col12,
                    input_col13,
                    input_col14,
                    input_col15,
                    input_col16,
                    input_col17,
                    input_col18,
                ];
                *row[28] = multiplicity;
                *lookup_data.mults = multiplicity;
            },
        );

    (trace, sub_components_inputs, lookup_data)
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_lookup_elements: &relations::MemoryAddressToId,
        memory_id_to_big_lookup_elements: &relations::MemoryIdToBig,
        range_check_4_3_lookup_elements: &relations::RangeCheck_4_3,
        range_check_7_2_5_lookup_elements: &relations::RangeCheck_7_2_5,
        verify_instruction_lookup_elements: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.range_check_7_2_5[0],
            &self.lookup_data.range_check_4_3[0],
        )
        .enumerate()
        {
            let p0: PackedQM31 = range_check_7_2_5_lookup_elements.combine(v0);
            let p1: PackedQM31 = range_check_4_3_lookup_elements.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (v0, v1)) in zip(
            &self.lookup_data.memory_address_to_id[0],
            &self.lookup_data.memory_id_to_big[0],
        )
        .enumerate()
        {
            let p0: PackedQM31 = memory_address_to_id_lookup_elements.combine(v0);
            let p1: PackedQM31 = memory_id_to_big_lookup_elements.combine(v1);
            col_gen.write_frac(i, p0 + p1, p0 * p1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.verify_instruction[0];
        let mults = &self.lookup_data.mults;
        for (i, (lookup_values, &mults)) in zip(lookup_row, mults).enumerate() {
            let denom = verify_instruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, (-mults).into(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

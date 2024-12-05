#![allow(unused_parens)]
#![allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet};
use std::iter::zip;
use std::vec;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
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
const N_TRACE_COLUMNS: usize = 28 + 1;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: BTreeMap<InputType, u32>,
}
impl ClaimGenerator {
    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
        range_check_4_3_state: &mut range_check_4_3::ClaimGenerator,
        range_check_7_2_5_state: &mut range_check_7_2_5::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let (mut inputs, mut mults) = self.inputs.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        let n_calls = inputs.len();
        assert_ne!(n_calls, 0);
        let size = std::cmp::max(n_calls.next_power_of_two(), N_LANES);
        let need_padding = n_calls != size;

        if need_padding {
            inputs.resize(size, *inputs.first().unwrap());
            mults.resize(size, 0);
        }

        let packed_inputs = pack_values(&inputs);
        let (trace, sub_components_inputs, lookup_data) =
            write_trace_simd(packed_inputs, mults, memory_address_to_id_state);

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

        tree_builder.extend_evals(
            trace
                .into_iter()
                .map(|eval| {
                    let domain = CanonicCoset::new(
                        eval.len()
                            .checked_ilog2()
                            .expect("Input is not a power of 2!"),
                    )
                    .circle_domain();
                    CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, eval)
                })
                .collect_vec(),
        );

        (
            Claim { n_calls },
            InteractionClaimGenerator {
                n_calls,
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

pub struct SubComponentInputs {
    pub range_check_4_3_inputs: [Vec<range_check_4_3::InputType>; 1],
    pub range_check_7_2_5_inputs: [Vec<range_check_7_2_5::InputType>; 1],
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 1],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            range_check_4_3_inputs: [Vec::with_capacity(capacity)],
            range_check_7_2_5_inputs: [Vec::with_capacity(capacity)],
            memory_address_to_id_inputs: [Vec::with_capacity(capacity)],
            memory_id_to_big_inputs: [Vec::with_capacity(capacity)],
        }
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
pub fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    mults: Vec<u32>,
    memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
) -> (
    [BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData,
) {
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

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

    inputs
        .into_iter()
        .enumerate()
        .for_each(|(row_index, verify_instruction_input)| {
            let input_tmp_155 = (
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
            let input_col0 = input_tmp_155.0;
            trace[0].data[row_index] = input_col0;
            let input_col1 = input_tmp_155.1[0];
            trace[1].data[row_index] = input_col1;
            let input_col2 = input_tmp_155.1[1];
            trace[2].data[row_index] = input_col2;
            let input_col3 = input_tmp_155.1[2];
            trace[3].data[row_index] = input_col3;
            let input_col4 = input_tmp_155.2[0];
            trace[4].data[row_index] = input_col4;
            let input_col5 = input_tmp_155.2[1];
            trace[5].data[row_index] = input_col5;
            let input_col6 = input_tmp_155.2[2];
            trace[6].data[row_index] = input_col6;
            let input_col7 = input_tmp_155.2[3];
            trace[7].data[row_index] = input_col7;
            let input_col8 = input_tmp_155.2[4];
            trace[8].data[row_index] = input_col8;
            let input_col9 = input_tmp_155.2[5];
            trace[9].data[row_index] = input_col9;
            let input_col10 = input_tmp_155.2[6];
            trace[10].data[row_index] = input_col10;
            let input_col11 = input_tmp_155.2[7];
            trace[11].data[row_index] = input_col11;
            let input_col12 = input_tmp_155.2[8];
            trace[12].data[row_index] = input_col12;
            let input_col13 = input_tmp_155.2[9];
            trace[13].data[row_index] = input_col13;
            let input_col14 = input_tmp_155.2[10];
            trace[14].data[row_index] = input_col14;
            let input_col15 = input_tmp_155.2[11];
            trace[15].data[row_index] = input_col15;
            let input_col16 = input_tmp_155.2[12];
            trace[16].data[row_index] = input_col16;
            let input_col17 = input_tmp_155.2[13];
            trace[17].data[row_index] = input_col17;
            let input_col18 = input_tmp_155.2[14];
            trace[18].data[row_index] = input_col18;

            // encode_offsets.

            let offset0_low_tmp_166 = ((PackedUInt16::from_m31(input_col1)) & (UInt16_511));
            let offset0_low_col19 = offset0_low_tmp_166.as_m31();
            trace[19].data[row_index] = offset0_low_col19;
            let offset0_mid_tmp_167 = ((PackedUInt16::from_m31(input_col1)) >> (UInt16_9));
            let offset0_mid_col20 = offset0_mid_tmp_167.as_m31();
            trace[20].data[row_index] = offset0_mid_col20;
            let offset1_low_tmp_168 = ((PackedUInt16::from_m31(input_col2)) & (UInt16_3));
            let offset1_low_col21 = offset1_low_tmp_168.as_m31();
            trace[21].data[row_index] = offset1_low_col21;
            let offset1_mid_tmp_169 =
                (((PackedUInt16::from_m31(input_col2)) >> (UInt16_2)) & (UInt16_511));
            let offset1_mid_col22 = offset1_mid_tmp_169.as_m31();
            trace[22].data[row_index] = offset1_mid_col22;
            let offset1_high_tmp_170 = ((PackedUInt16::from_m31(input_col2)) >> (UInt16_11));
            let offset1_high_col23 = offset1_high_tmp_170.as_m31();
            trace[23].data[row_index] = offset1_high_col23;
            let offset2_low_tmp_171 = ((PackedUInt16::from_m31(input_col3)) & (UInt16_15));
            let offset2_low_col24 = offset2_low_tmp_171.as_m31();
            trace[24].data[row_index] = offset2_low_col24;
            let offset2_mid_tmp_172 =
                (((PackedUInt16::from_m31(input_col3)) >> (UInt16_4)) & (UInt16_511));
            let offset2_mid_col25 = offset2_mid_tmp_172.as_m31();
            trace[25].data[row_index] = offset2_mid_col25;
            let offset2_high_tmp_173 = ((PackedUInt16::from_m31(input_col3)) >> (UInt16_13));
            let offset2_high_col26 = offset2_high_tmp_173.as_m31();
            trace[26].data[row_index] = offset2_high_col26;

            sub_components_inputs.range_check_7_2_5_inputs[0]
                .extend([offset0_mid_col20, offset1_low_col21, offset1_high_col23].unpack());

            lookup_data.rangecheck_7_2_5[0].push([
                offset0_mid_col20,
                offset1_low_col21,
                offset1_high_col23,
            ]);

            sub_components_inputs.range_check_4_3_inputs[0]
                .extend([offset2_low_col24, offset2_high_col26].unpack());

            lookup_data.rangecheck_4_3[0].push([offset2_low_col24, offset2_high_col26]);

            // encode_flags.

            // mem_verify.

            let memory_address_to_id_value_tmp_176 =
                memory_address_to_id_state.deduce_output(input_col0);
            let instruction_id_col27 = memory_address_to_id_value_tmp_176;
            trace[27].data[row_index] = instruction_id_col27;
            sub_components_inputs.memory_address_to_id_inputs[0].extend(input_col0.unpack());

            lookup_data.memoryaddresstoid[0].push([input_col0, instruction_id_col27]);
            sub_components_inputs.memory_id_to_big_inputs[0].extend(instruction_id_col27.unpack());

            lookup_data.memoryidtobig[0].push([
                instruction_id_col27,
                offset0_low_col19,
                ((offset0_mid_col20) + ((offset1_low_col21) * (M31_128))),
                offset1_mid_col22,
                ((offset1_high_col23) + ((offset2_low_col24) * (M31_32))),
                offset2_mid_col25,
                ((offset2_high_col26)
                    + (((((((M31_0) + ((input_col4) * (M31_8))) + ((input_col5) * (M31_16)))
                        + ((input_col6) * (M31_32)))
                        + ((input_col7) * (M31_64)))
                        + ((input_col8) * (M31_128)))
                        + ((input_col9) * (M31_256)))),
                ((((((((((M31_0) + ((input_col10) * (M31_1))) + ((input_col11) * (M31_2)))
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
            ]);

            lookup_data.verifyinstruction[0].push([
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
            ]);
        });

    let multplicity_column =
        BaseColumn::from_iter(mults.clone().into_iter().map(M31::from_u32_unchecked));
    lookup_data.mults = multplicity_column.data.clone();
    trace[28] = multplicity_column;

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 1],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 1],
    pub rangecheck_4_3: [Vec<[PackedM31; 2]>; 1],
    pub rangecheck_7_2_5: [Vec<[PackedM31; 3]>; 1],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
    pub mults: Vec<PackedM31>,
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid: [Vec::with_capacity(capacity)],
            memoryidtobig: [Vec::with_capacity(capacity)],
            rangecheck_4_3: [Vec::with_capacity(capacity)],
            rangecheck_7_2_5: [Vec::with_capacity(capacity)],
            verifyinstruction: [Vec::with_capacity(capacity)],
            mults: vec![],
        }
    }
}

pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_lookup_elements: &relations::MemoryAddressToId,
        memoryidtobig_lookup_elements: &relations::MemoryIdToBig,
        rangecheck_4_3_lookup_elements: &relations::RangeCheck_4_3,
        rangecheck_7_2_5_lookup_elements: &relations::RangeCheck_7_2_5,
        verifyinstruction_lookup_elements: &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_7_2_5[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_7_2_5_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.rangecheck_4_3[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = rangecheck_4_3_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryaddresstoid[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryidtobig[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.verifyinstruction[0];
        let mults = &self.lookup_data.mults;
        for (i, (lookup_values, &mults)) in zip(lookup_row, mults).enumerate() {
            let denom = verifyinstruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, (-mults).into(), denom);
        }
        col_gen.finalize_col();

        let (trace, total_sum, claimed_sum) = {
            let (trace, claimed_sum) = logup_gen.finalize_last();
            (trace, claimed_sum, None)
        };
        tree_builder.extend_evals(trace);

        InteractionClaim {
            logup_sums: (total_sum, claimed_sum),
        }
    }
}

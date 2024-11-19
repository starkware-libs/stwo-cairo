#![allow(unused_parens)]
#![allow(unused_imports)]
use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::{PackedQM31, PackedSecureField};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{bit_reverse, bit_reverse_coset_to_circle_domain_order};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};

use super::component::{Claim, InteractionClaim, RelationElements};
use crate::components::range_check_vector::{range_check_4_3, range_check_7_2_5};
use crate::components::{memory, pack_values, verifyinstruction};

pub type PackedInputType = (PackedM31, [PackedM31; 3], [PackedM31; 15]);
pub type InputType = (M31, [M31; 3], [M31; 15]);

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn write_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_state: &mut memory::addr_to_id::ClaimGenerator,
        rangecheck_4_3_state: &mut range_check_4_3::ClaimGenerator,
        range_check_7_2_5_state: &mut range_check_7_2_5::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_calls = self.inputs.len();
        let size = if n_calls == 0 {
            n_calls
        } else {
            std::cmp::max(n_calls.next_power_of_two(), N_LANES)
        };
        let need_padding = n_calls != size;

        if need_padding {
            self.inputs.resize(size, *self.inputs.first().unwrap());
            bit_reverse_coset_to_circle_domain_order(&mut self.inputs);
        }

        let simd_inputs = pack_values(&self.inputs);
        let (trace, mut sub_components_inputs, lookup_data) =
            write_trace_simd(simd_inputs, memoryaddresstoid_state);

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }

        rangecheck_4_3_state.add_inputs(&sub_components_inputs.rangecheck_4_3_inputs[..n_calls]);
        range_check_7_2_5_state
            .add_inputs(&sub_components_inputs.range_check_7_2_5_inputs[..n_calls]);

        let trace = trace
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
            .collect_vec();

        tree_builder.extend_evals(trace);

        (
            Claim { n_calls },
            InteractionClaimGenerator {
                n_calls,
                lookup_data,
            },
        )
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        inputs.iter().for_each(|input| self.inputs.push(*input));
    }

    pub fn add_single_input(&mut self, input: InputType) {
        self.inputs.push(input);
    }
}

pub struct SubComponentInputs {
    pub memoryaddresstoid_inputs: Vec<memory::addr_to_id::InputType>,
    pub rangecheck_4_3_inputs: Vec<range_check_4_3::InputType>,
    pub range_check_7_2_5_inputs: Vec<range_check_7_2_5::InputType>,
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid_inputs: Vec::with_capacity(capacity),
            rangecheck_4_3_inputs: Vec::with_capacity(capacity),
            range_check_7_2_5_inputs: Vec::with_capacity(capacity),
        }
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        bit_reverse_coset_to_circle_domain_order(&mut self.memoryaddresstoid_inputs);
        bit_reverse_coset_to_circle_domain_order(&mut self.rangecheck_4_3_inputs);
        bit_reverse_coset_to_circle_domain_order(&mut self.range_check_7_2_5_inputs);
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
pub fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memoryaddresstoid_state: &mut memory::addr_to_id::ClaimGenerator,
) -> (Vec<BaseColumn>, SubComponentInputs, LookupData) {
    const N_TRACE_COLUMNS: usize = 28;
    let mut trace_values: [_; N_TRACE_COLUMNS] =
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
        .for_each(|(row_index, verifyinstruction_input)| {
            let tmp_0 = (
                verifyinstruction_input.0,
                [
                    verifyinstruction_input.1[0],
                    verifyinstruction_input.1[1],
                    verifyinstruction_input.1[2],
                ],
                [
                    verifyinstruction_input.2[0],
                    verifyinstruction_input.2[1],
                    verifyinstruction_input.2[2],
                    verifyinstruction_input.2[3],
                    verifyinstruction_input.2[4],
                    verifyinstruction_input.2[5],
                    verifyinstruction_input.2[6],
                    verifyinstruction_input.2[7],
                    verifyinstruction_input.2[8],
                    verifyinstruction_input.2[9],
                    verifyinstruction_input.2[10],
                    verifyinstruction_input.2[11],
                    verifyinstruction_input.2[12],
                    verifyinstruction_input.2[13],
                    verifyinstruction_input.2[14],
                ],
            );
            let input_col0 = tmp_0.0;
            trace_values[0].data[row_index] = input_col0;
            let input_col1 = tmp_0.1[0];
            trace_values[1].data[row_index] = input_col1;
            let input_col2 = tmp_0.1[1];
            trace_values[2].data[row_index] = input_col2;
            let input_col3 = tmp_0.1[2];
            trace_values[3].data[row_index] = input_col3;
            let input_col4 = tmp_0.2[0];
            trace_values[4].data[row_index] = input_col4;
            let input_col5 = tmp_0.2[1];
            trace_values[5].data[row_index] = input_col5;
            let input_col6 = tmp_0.2[2];
            trace_values[6].data[row_index] = input_col6;
            let input_col7 = tmp_0.2[3];
            trace_values[7].data[row_index] = input_col7;
            let input_col8 = tmp_0.2[4];
            trace_values[8].data[row_index] = input_col8;
            let input_col9 = tmp_0.2[5];
            trace_values[9].data[row_index] = input_col9;
            let input_col10 = tmp_0.2[6];
            trace_values[10].data[row_index] = input_col10;
            let input_col11 = tmp_0.2[7];
            trace_values[11].data[row_index] = input_col11;
            let input_col12 = tmp_0.2[8];
            trace_values[12].data[row_index] = input_col12;
            let input_col13 = tmp_0.2[9];
            trace_values[13].data[row_index] = input_col13;
            let input_col14 = tmp_0.2[10];
            trace_values[14].data[row_index] = input_col14;
            let input_col15 = tmp_0.2[11];
            trace_values[15].data[row_index] = input_col15;
            let input_col16 = tmp_0.2[12];
            trace_values[16].data[row_index] = input_col16;
            let input_col17 = tmp_0.2[13];
            trace_values[17].data[row_index] = input_col17;
            let input_col18 = tmp_0.2[14];
            trace_values[18].data[row_index] = input_col18;
            let tmp_11 = ((PackedUInt16::from_m31(input_col1)) & (UInt16_511));
            let offset0_low_col19 = tmp_11.as_m31();
            trace_values[19].data[row_index] = offset0_low_col19;
            let tmp_12 = ((PackedUInt16::from_m31(input_col1)) >> (UInt16_9));
            let offset0_mid_col20 = tmp_12.as_m31();
            trace_values[20].data[row_index] = offset0_mid_col20;
            let tmp_13 = ((PackedUInt16::from_m31(input_col2)) & (UInt16_3));
            let offset1_low_col21 = tmp_13.as_m31();
            trace_values[21].data[row_index] = offset1_low_col21;
            let tmp_14 = (((PackedUInt16::from_m31(input_col2)) >> (UInt16_2)) & (UInt16_511));
            let offset1_mid_col22 = tmp_14.as_m31();
            trace_values[22].data[row_index] = offset1_mid_col22;
            let tmp_15 = ((PackedUInt16::from_m31(input_col2)) >> (UInt16_11));
            let offset1_high_col23 = tmp_15.as_m31();
            trace_values[23].data[row_index] = offset1_high_col23;
            let tmp_16 = ((PackedUInt16::from_m31(input_col3)) & (UInt16_15));
            let offset2_low_col24 = tmp_16.as_m31();
            trace_values[24].data[row_index] = offset2_low_col24;
            let tmp_17 = (((PackedUInt16::from_m31(input_col3)) >> (UInt16_4)) & (UInt16_511));
            let offset2_mid_col25 = tmp_17.as_m31();
            trace_values[25].data[row_index] = offset2_mid_col25;
            let tmp_18 = ((PackedUInt16::from_m31(input_col3)) >> (UInt16_13));
            let offset2_high_col26 = tmp_18.as_m31();
            trace_values[26].data[row_index] = offset2_high_col26;
            sub_components_inputs
                .range_check_7_2_5_inputs
                .extend([offset0_mid_col20, offset1_low_col21, offset1_high_col23].unpack());
            lookup_data.range_check_7_2_5[0].push([
                offset0_mid_col20,
                offset1_low_col21,
                offset1_high_col23,
            ]);
            sub_components_inputs
                .rangecheck_4_3_inputs
                .extend([offset2_low_col24, offset2_high_col26].unpack());
            lookup_data.rangecheck_4_3[0].push([offset2_low_col24, offset2_high_col26]);
            sub_components_inputs
                .memoryaddresstoid_inputs
                .extend(input_col0.unpack());
            let tmp_50 = memoryaddresstoid_state.deduce_output(input_col0);
            let instruction_id_col27 = tmp_50;
            trace_values[27].data[row_index] = instruction_id_col27;
            lookup_data.memoryaddresstoid[0].push([input_col0, instruction_id_col27]);
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

    (trace_values.to_vec(), sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 1],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 1],
    pub rangecheck_4_3: [Vec<[PackedM31; 2]>; 1],
    pub range_check_7_2_5: [Vec<[PackedM31; 3]>; 1],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid: [Vec::with_capacity(capacity)],
            memoryidtobig: [Vec::with_capacity(capacity)],
            rangecheck_4_3: [Vec::with_capacity(capacity)],
            range_check_7_2_5: [Vec::with_capacity(capacity)],
            verifyinstruction: [Vec::with_capacity(capacity)],
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
        memoryaddresstoid_lookup_elements: &memory::addr_to_id::RelationElements,
        memoryidtobig_lookup_elements: &memory::id_to_f252::RelationElements,
        rangecheck_4_3_lookup_elements: &range_check_4_3::RelationElements,
        range_check_7_2_5_lookup_elements: &range_check_7_2_5::RelationElements,
        verifyinstruction_lookup_elements: &verifyinstruction::RelationElements,
    ) -> InteractionClaim {
        let log_size = self.n_calls.next_power_of_two().ilog2();
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.range_check_7_2_5[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = range_check_7_2_5_lookup_elements.combine(lookup_values);
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
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = verifyinstruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, [total_sum, claimed_sum]) =
            logup_gen.finalize_at([(1 << log_size) - 1, self.n_calls - 1]);
        tree_builder.extend_evals(trace);

        InteractionClaim {
            total_sum,
            claimed_sum: Some((claimed_sum, self.n_calls - 1)),
        }
    }
}

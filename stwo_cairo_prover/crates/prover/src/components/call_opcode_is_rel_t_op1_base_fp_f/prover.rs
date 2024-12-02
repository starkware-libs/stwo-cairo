#![allow(unused_parens)]
#![allow(unused_imports)]
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
use crate::components::{memory_address_to_id, memory_id_to_big, pack_values, verify_instruction};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 11;

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
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
        verify_instruction_state: &mut verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let n_calls = self.inputs.len();
        assert_ne!(n_calls, 0);
        let size = std::cmp::max(n_calls.next_power_of_two(), N_LANES);
        let need_padding = n_calls != size;

        if need_padding {
            self.inputs.resize(size, *self.inputs.first().unwrap());
            bit_reverse_coset_to_circle_domain_order(&mut self.inputs);
        }

        let packed_inputs = pack_values(&self.inputs);
        let (trace, mut sub_components_inputs, lookup_data) = write_trace_simd(
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs
            .memory_address_to_id_inputs
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .memory_id_to_big_inputs
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .verify_instruction_inputs
            .iter()
            .for_each(|inputs| {
                verify_instruction_state.add_inputs(&inputs[..n_calls]);
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
        self.inputs.extend(inputs);
    }
}

pub struct SubComponentInputs {
    pub memory_address_to_id_inputs: [Vec<memory_address_to_id::InputType>; 3],
    pub memory_id_to_big_inputs: [Vec<memory_id_to_big::InputType>; 3],
    pub verify_instruction_inputs: [Vec<verify_instruction::InputType>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memory_address_to_id_inputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            memory_id_to_big_inputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            verify_instruction_inputs: [Vec::with_capacity(capacity)],
        }
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        self.memory_address_to_id_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.memory_id_to_big_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.verify_instruction_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
pub fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &mut memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &mut memory_id_to_big::ClaimGenerator,
) -> (
    [BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData,
) {
    const N_TRACE_COLUMNS: usize = 11;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

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

    inputs.into_iter().enumerate().for_each(
        |(row_index, call_opcode_is_rel_t_op1_base_fp_f_input)| {
            let input_tmp_1182 = call_opcode_is_rel_t_op1_base_fp_f_input;
            let input_pc_col0 = input_tmp_1182.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_1182.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_1182.fp;
            trace[2].data[row_index] = input_fp_col2;

            // decode_instruction_553df0b64b2e381f.

            let memory_address_to_id_value_tmp_1186 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_1187 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_1186);

            sub_components_inputs.verify_instruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [M31_32768, M31_32769, M31_32769],
                    [
                        M31_0, M31_0, M31_1, M31_0, M31_0, M31_0, M31_0, M31_0, M31_1, M31_0,
                        M31_0, M31_0, M31_1, M31_0, M31_0,
                    ],
                )
                    .unpack(),
            );

            lookup_data.verifyinstruction[0].push([
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
            ]);

            // mem_verify.

            let memory_address_to_id_value_tmp_1189 =
                memory_address_to_id_state.deduce_output(input_ap_col1);
            let ap_id_col3 = memory_address_to_id_value_tmp_1189;
            trace[3].data[row_index] = ap_id_col3;
            sub_components_inputs.memory_address_to_id_inputs[0].extend(input_ap_col1.unpack());

            lookup_data.memoryaddresstoid[0].push([input_ap_col1, ap_id_col3]);
            sub_components_inputs.memory_id_to_big_inputs[0].extend(ap_id_col3.unpack());

            lookup_data.memoryidtobig[0].push([
                ap_id_col3,
                input_fp_col2,
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
                M31_0,
                M31_0,
            ]);

            // mem_verify.

            let memory_address_to_id_value_tmp_1190 =
                memory_address_to_id_state.deduce_output(((input_ap_col1) + (M31_1)));
            let ap_plus_one_id_col4 = memory_address_to_id_value_tmp_1190;
            trace[4].data[row_index] = ap_plus_one_id_col4;
            sub_components_inputs.memory_address_to_id_inputs[1]
                .extend(((input_ap_col1) + (M31_1)).unpack());

            lookup_data.memoryaddresstoid[1]
                .push([((input_ap_col1) + (M31_1)), ap_plus_one_id_col4]);
            sub_components_inputs.memory_id_to_big_inputs[1].extend(ap_plus_one_id_col4.unpack());

            lookup_data.memoryidtobig[1].push([
                ap_plus_one_id_col4,
                ((input_pc_col0) + (M31_2)),
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
                M31_0,
                M31_0,
            ]);

            // read_small.

            let memory_address_to_id_value_tmp_1191 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memory_id_to_big_value_tmp_1192 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_1191);
            let next_pc_id_col5 = memory_address_to_id_value_tmp_1191;
            trace[5].data[row_index] = next_pc_id_col5;
            sub_components_inputs.memory_address_to_id_inputs[2]
                .extend(((input_pc_col0) + (M31_1)).unpack());

            lookup_data.memoryaddresstoid[2].push([((input_pc_col0) + (M31_1)), next_pc_id_col5]);

            // cond_decode_small_sign.

            let msb_tmp_1193 = memory_id_to_big_value_tmp_1192.get_m31(27).eq(M31_256);
            let msb_col6 = msb_tmp_1193.as_m31();
            trace[6].data[row_index] = msb_col6;
            let mid_limbs_set_tmp_1194 = memory_id_to_big_value_tmp_1192.get_m31(20).eq(M31_511);
            let mid_limbs_set_col7 = mid_limbs_set_tmp_1194.as_m31();
            trace[7].data[row_index] = mid_limbs_set_col7;

            let next_pc_limb_0_col8 = memory_id_to_big_value_tmp_1192.get_m31(0);
            trace[8].data[row_index] = next_pc_limb_0_col8;
            let next_pc_limb_1_col9 = memory_id_to_big_value_tmp_1192.get_m31(1);
            trace[9].data[row_index] = next_pc_limb_1_col9;
            let next_pc_limb_2_col10 = memory_id_to_big_value_tmp_1192.get_m31(2);
            trace[10].data[row_index] = next_pc_limb_2_col10;
            sub_components_inputs.memory_id_to_big_inputs[2].extend(next_pc_id_col5.unpack());

            lookup_data.memoryidtobig[2].push([
                next_pc_id_col5,
                next_pc_limb_0_col8,
                next_pc_limb_1_col9,
                next_pc_limb_2_col10,
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                ((mid_limbs_set_col7) * (M31_511)),
                (((M31_136) * (msb_col6)) - (mid_limbs_set_col7)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col6) * (M31_256)),
            ]);

            lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes[1].push([
                ((input_pc_col0)
                    + (((((next_pc_limb_0_col8) + ((next_pc_limb_1_col9) * (M31_512)))
                        + ((next_pc_limb_2_col10) * (M31_262144)))
                        - (msb_col6))
                        - ((M31_134217728) * (mid_limbs_set_col7)))),
                ((input_ap_col1) + (M31_2)),
                ((input_ap_col1) + (M31_2)),
            ]);
        },
    );

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 3],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 3],
    pub opcodes: [Vec<[PackedM31; 3]>; 2],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            memoryidtobig: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            opcodes: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
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
        memoryaddresstoid_lookup_elements: &relations::MemoryAddressToId,
        memoryidtobig_lookup_elements: &relations::MemoryIdToBig,
        opcodes_lookup_elements: &relations::Opcodes,
        verifyinstruction_lookup_elements: &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.verifyinstruction[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = verifyinstruction_lookup_elements.combine(lookup_values);
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
        let lookup_row = &self.lookup_data.memoryaddresstoid[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryidtobig[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryaddresstoid[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.memoryidtobig[2];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.opcodes[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = opcodes_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.opcodes[1];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = opcodes_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, -PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, total_sum, claimed_sum) = if self.n_calls == 1 << log_size {
            let (trace, claimed_sum) = logup_gen.finalize_last();
            (trace, claimed_sum, None)
        } else {
            let (trace, [total_sum, claimed_sum]) =
                logup_gen.finalize_at([(1 << log_size) - 1, self.n_calls - 1]);
            (trace, total_sum, Some((claimed_sum, self.n_calls - 1)))
        };
        tree_builder.extend_evals(trace);

        InteractionClaim {
            logup_sums: (total_sum, claimed_sum),
        }
    }
}

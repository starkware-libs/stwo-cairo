#![allow(unused_parens)]
#![allow(unused_imports)]
use std::cell::UnsafeCell;
use std::rc::Rc;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
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
    memory_address_to_id, memory_id_to_big, pack_values, verify_instruction, ThreadSafeTrace,
};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 16;

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
            .iter_mut()
            .for_each(|inputs| {
                memory_address_to_id_state.add_inputs(&inputs.get_mut()[..n_calls]);
            });
        sub_components_inputs
            .memory_id_to_big_inputs
            .iter_mut()
            .for_each(|inputs| {
                memory_id_to_big_state.add_inputs(&inputs.get_mut()[..n_calls]);
            });
        sub_components_inputs
            .verify_instruction_inputs
            .iter_mut()
            .for_each(|inputs| {
                verify_instruction_state.add_inputs(&inputs.get_mut()[..n_calls]);
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
    pub memory_address_to_id_inputs: [UnsafeCell<Vec<memory_address_to_id::InputType>>; 3],
    pub memory_id_to_big_inputs: [UnsafeCell<Vec<memory_id_to_big::InputType>>; 3],
    pub verify_instruction_inputs: [UnsafeCell<Vec<verify_instruction::InputType>>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    unsafe fn uninitialized(len: usize) -> Self {
        let sub_component_inputs = Self {
            memory_address_to_id_inputs: [
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
            ],
            memory_id_to_big_inputs: [
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
            ],
            verify_instruction_inputs: [Vec::with_capacity(len).into()],
        };
        sub_component_inputs
            .memory_address_to_id_inputs
            .iter()
            .for_each(|vec| {
                (*vec.get()).set_len(len);
            });
        sub_component_inputs
            .memory_id_to_big_inputs
            .iter()
            .for_each(|vec| {
                (*vec.get()).set_len(len);
            });
        sub_component_inputs
            .verify_instruction_inputs
            .iter()
            .for_each(|vec| {
                (*vec.get()).set_len(len);
            });
        sub_component_inputs
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        self.memory_address_to_id_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
        self.memory_id_to_big_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
        self.verify_instruction_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec.get_mut()));
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
    let len = inputs.len();
    let cpu_len = len * N_LANES;
    let trace = Rc::new(ThreadSafeTrace::<N_TRACE_COLUMNS>::new(cpu_len));
    let lookup_data = Rc::new(unsafe { LookupData::uninitialized(len) });
    let sub_components_inputs = Rc::new(unsafe { SubComponentInputs::uninitialized(cpu_len) });

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));

    #[cfg(not(feature = "parallel"))]
    let iter = inputs.into_iter().enumerate();

    #[cfg(feature = "parallel")]
    let iter = inputs.into_par_iter().enumerate();

    iter.for_each(|(row_index, call_opcode_is_rel_f_op1_base_fp_f_input)| {
        unsafe {
            let input_tmp_eefa_0 = call_opcode_is_rel_f_op1_base_fp_f_input;
            let input_pc_col0 = input_tmp_eefa_0.pc;
            (*trace.data[0].get()).data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_eefa_0.ap;
            (*trace.data[1].get()).data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_eefa_0.fp;
            (*trace.data[2].get()).data[row_index] = input_fp_col2;

            // DecodeInstruction_d06cc8770ddfafbc.

            let memoryaddresstoid_value_tmp_eefa_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memoryidtobig_value_tmp_eefa_2 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_eefa_1);
            let offset2_tmp_eefa_3 =
                ((((PackedUInt16::from_m31(memoryidtobig_value_tmp_eefa_2.get_m31(3)))
                    >> (UInt16_5))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_eefa_2.get_m31(4)))
                        << (UInt16_4)))
                    + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_eefa_2.get_m31(5)))
                        & (UInt16_7))
                        << (UInt16_13)));
            let offset2_col3 = offset2_tmp_eefa_3.as_m31();
            (*trace.data[3].get()).data[row_index] = offset2_col3;

            (*sub_components_inputs.verify_instruction_inputs[0].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(
                    &(
                        input_pc_col0,
                        [M31_32768, M31_32769, offset2_col3],
                        [
                            M31_0, M31_0, M31_0, M31_0, M31_1, M31_0, M31_0, M31_1, M31_0, M31_0,
                            M31_0, M31_0, M31_1, M31_0, M31_0,
                        ],
                    )
                        .unpack(),
                );

            (*lookup_data.verifyinstruction[0].get())[row_index] = [
                input_pc_col0,
                M31_32768,
                M31_32769,
                offset2_col3,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
            ];

            // ReadPositive_num_bits_27.

            let memoryaddresstoid_value_tmp_eefa_4 =
                memory_address_to_id_state.deduce_output(input_ap_col1);
            let memoryidtobig_value_tmp_eefa_5 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_eefa_4);
            let stored_fp_id_col4 = memoryaddresstoid_value_tmp_eefa_4;
            (*trace.data[4].get()).data[row_index] = stored_fp_id_col4;
            (*sub_components_inputs.memory_address_to_id_inputs[0].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&input_ap_col1.unpack());

            (*lookup_data.memoryaddresstoid[0].get())[row_index] =
                [input_ap_col1, stored_fp_id_col4];
            let stored_fp_limb_0_col5 = memoryidtobig_value_tmp_eefa_5.get_m31(0);
            (*trace.data[5].get()).data[row_index] = stored_fp_limb_0_col5;
            let stored_fp_limb_1_col6 = memoryidtobig_value_tmp_eefa_5.get_m31(1);
            (*trace.data[6].get()).data[row_index] = stored_fp_limb_1_col6;
            let stored_fp_limb_2_col7 = memoryidtobig_value_tmp_eefa_5.get_m31(2);
            (*trace.data[7].get()).data[row_index] = stored_fp_limb_2_col7;
            (*sub_components_inputs.memory_id_to_big_inputs[0].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&stored_fp_id_col4.unpack());

            (*lookup_data.memoryidtobig[0].get())[row_index] = [
                stored_fp_id_col4,
                stored_fp_limb_0_col5,
                stored_fp_limb_1_col6,
                stored_fp_limb_2_col7,
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

            // ReadPositive_num_bits_27.

            let memoryaddresstoid_value_tmp_eefa_6 =
                memory_address_to_id_state.deduce_output(((input_ap_col1) + (M31_1)));
            let memoryidtobig_value_tmp_eefa_7 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_eefa_6);
            let stored_ret_pc_id_col8 = memoryaddresstoid_value_tmp_eefa_6;
            (*trace.data[8].get()).data[row_index] = stored_ret_pc_id_col8;
            (*sub_components_inputs.memory_address_to_id_inputs[1].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&((input_ap_col1) + (M31_1)).unpack());

            (*lookup_data.memoryaddresstoid[1].get())[row_index] =
                [((input_ap_col1) + (M31_1)), stored_ret_pc_id_col8];
            let stored_ret_pc_limb_0_col9 = memoryidtobig_value_tmp_eefa_7.get_m31(0);
            (*trace.data[9].get()).data[row_index] = stored_ret_pc_limb_0_col9;
            let stored_ret_pc_limb_1_col10 = memoryidtobig_value_tmp_eefa_7.get_m31(1);
            (*trace.data[10].get()).data[row_index] = stored_ret_pc_limb_1_col10;
            let stored_ret_pc_limb_2_col11 = memoryidtobig_value_tmp_eefa_7.get_m31(2);
            (*trace.data[11].get()).data[row_index] = stored_ret_pc_limb_2_col11;
            (*sub_components_inputs.memory_id_to_big_inputs[1].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&stored_ret_pc_id_col8.unpack());

            (*lookup_data.memoryidtobig[1].get())[row_index] = [
                stored_ret_pc_id_col8,
                stored_ret_pc_limb_0_col9,
                stored_ret_pc_limb_1_col10,
                stored_ret_pc_limb_2_col11,
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

            // ReadPositive_num_bits_27.

            let memoryaddresstoid_value_tmp_eefa_8 = memory_address_to_id_state
                .deduce_output(((input_ap_col1) + ((offset2_col3) - (M31_32768))));
            let memoryidtobig_value_tmp_eefa_9 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_eefa_8);
            let next_pc_id_col12 = memoryaddresstoid_value_tmp_eefa_8;
            (*trace.data[12].get()).data[row_index] = next_pc_id_col12;
            (*sub_components_inputs.memory_address_to_id_inputs[2].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&((input_ap_col1) + ((offset2_col3) - (M31_32768))).unpack());

            (*lookup_data.memoryaddresstoid[2].get())[row_index] = [
                ((input_ap_col1) + ((offset2_col3) - (M31_32768))),
                next_pc_id_col12,
            ];
            let next_pc_limb_0_col13 = memoryidtobig_value_tmp_eefa_9.get_m31(0);
            (*trace.data[13].get()).data[row_index] = next_pc_limb_0_col13;
            let next_pc_limb_1_col14 = memoryidtobig_value_tmp_eefa_9.get_m31(1);
            (*trace.data[14].get()).data[row_index] = next_pc_limb_1_col14;
            let next_pc_limb_2_col15 = memoryidtobig_value_tmp_eefa_9.get_m31(2);
            (*trace.data[15].get()).data[row_index] = next_pc_limb_2_col15;
            (*sub_components_inputs.memory_id_to_big_inputs[2].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&next_pc_id_col12.unpack());

            (*lookup_data.memoryidtobig[2].get())[row_index] = [
                next_pc_id_col12,
                next_pc_limb_0_col13,
                next_pc_limb_1_col14,
                next_pc_limb_2_col15,
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

            (*lookup_data.opcodes[0].get())[row_index] =
                [input_pc_col0, input_ap_col1, input_fp_col2];
            (*lookup_data.opcodes[1].get())[row_index] = [
                (((next_pc_limb_0_col13) + ((next_pc_limb_1_col14) * (M31_512)))
                    + ((next_pc_limb_2_col15) * (M31_262144))),
                ((input_ap_col1) + (M31_2)),
                ((input_ap_col1) + (M31_2)),
            ];
        }
    });

    let trace = match Rc::try_unwrap(trace) {
        Ok(trace) => trace.inner(),
        Err(_) => panic!("could not unwrap trace"),
    };
    let sub_components_inputs = match Rc::try_unwrap(sub_components_inputs) {
        Ok(sub_components_inputs) => sub_components_inputs,
        Err(_) => panic!("could not unwrap sub_components_inputs"),
    };
    let lookup_data = match Rc::try_unwrap(lookup_data) {
        Ok(lookup_data) => lookup_data,
        Err(_) => panic!("could not unwrap lookup_data"),
    };
    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [UnsafeCell<Vec<[PackedM31; 2]>>; 3],
    pub memoryidtobig: [UnsafeCell<Vec<[PackedM31; 29]>>; 3],
    pub opcodes: [UnsafeCell<Vec<[PackedM31; 3]>>; 2],
    pub verifyinstruction: [UnsafeCell<Vec<[PackedM31; 19]>>; 1],
}
impl LookupData {
    unsafe fn uninitialized(len: usize) -> Self {
        let lookup_data = Self {
            memoryaddresstoid: [
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
            ],
            memoryidtobig: [
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
            ],
            opcodes: [
                Vec::with_capacity(len).into(),
                Vec::with_capacity(len).into(),
            ],
            verifyinstruction: [Vec::with_capacity(len).into()],
        };
        lookup_data.memoryaddresstoid.iter().for_each(|vec| {
            (*vec.get()).set_len(len);
        });
        lookup_data.memoryidtobig.iter().for_each(|vec| {
            (*vec.get()).set_len(len);
        });
        lookup_data.opcodes.iter().for_each(|vec| {
            (*vec.get()).set_len(len);
        });
        lookup_data.verifyinstruction.iter().for_each(|vec| {
            (*vec.get()).set_len(len);
        });
        lookup_data
    }
}

pub struct InteractionClaimGenerator {
    pub n_calls: usize,
    pub lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_lookup_elements: &relations::MemoryAddressToId,
        memoryidtobig_lookup_elements: &relations::MemoryIdToBig,
        opcodes_lookup_elements: &relations::Opcodes,
        verifyinstruction_lookup_elements: &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let log_size = std::cmp::max(self.n_calls.next_power_of_two().ilog2(), LOG_N_LANES);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.verifyinstruction[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = verifyinstruction_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.memoryaddresstoid[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.memoryidtobig[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.memoryaddresstoid[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.memoryidtobig[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.memoryaddresstoid[2];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = memoryaddresstoid_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.memoryidtobig[2];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = memoryidtobig_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.opcodes[0];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
            let denom = opcodes_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &mut self.lookup_data.opcodes[1];
        for (i, lookup_values) in lookup_row.get_mut().iter().enumerate() {
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

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
const N_TRACE_COLUMNS: usize = 34;

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
    pub memory_address_to_id_inputs: [UnsafeCell<Vec<memory_address_to_id::InputType>>; 1],
    pub memory_id_to_big_inputs: [UnsafeCell<Vec<memory_id_to_big::InputType>>; 1],
    pub verify_instruction_inputs: [UnsafeCell<Vec<verify_instruction::InputType>>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    unsafe fn uninitialized(len: usize) -> Self {
        let sub_component_inputs = Self {
            memory_address_to_id_inputs: [Vec::with_capacity(len).into()],
            memory_id_to_big_inputs: [Vec::with_capacity(len).into()],
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
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    #[cfg(not(feature = "parallel"))]
    let iter = inputs.into_iter().enumerate();

    #[cfg(feature = "parallel")]
    let iter = inputs.into_par_iter().enumerate();

    iter.for_each(|(row_index, jnz_opcode_is_taken_f_dst_base_fp_t_input)| {
        unsafe {
            let input_tmp_1c37_0 = jnz_opcode_is_taken_f_dst_base_fp_t_input;
            let input_pc_col0 = input_tmp_1c37_0.pc;
            (*trace.data[0].get()).data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_1c37_0.ap;
            (*trace.data[1].get()).data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_1c37_0.fp;
            (*trace.data[2].get()).data[row_index] = input_fp_col2;

            // DecodeInstruction_113648125c3c3f56.

            let memoryaddresstoid_value_tmp_1c37_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memoryidtobig_value_tmp_1c37_2 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_1c37_1);
            let offset0_tmp_1c37_3 =
                ((PackedUInt16::from_m31(memoryidtobig_value_tmp_1c37_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memoryidtobig_value_tmp_1c37_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_1c37_3.as_m31();
            (*trace.data[3].get()).data[row_index] = offset0_col3;
            let ap_update_add_1_tmp_1c37_4 =
                (((((PackedUInt16::from_m31(memoryidtobig_value_tmp_1c37_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memoryidtobig_value_tmp_1c37_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col4 = ap_update_add_1_tmp_1c37_4.as_m31();
            (*trace.data[4].get()).data[row_index] = ap_update_add_1_col4;

            (*sub_components_inputs.verify_instruction_inputs[0].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(
                    &(
                        input_pc_col0,
                        [offset0_col3, M31_32767, M31_32769],
                        [
                            M31_1,
                            M31_1,
                            M31_1,
                            M31_0,
                            M31_0,
                            M31_0,
                            M31_0,
                            M31_0,
                            M31_0,
                            M31_1,
                            M31_0,
                            ap_update_add_1_col4,
                            M31_0,
                            M31_0,
                            M31_0,
                        ],
                    )
                        .unpack(),
                );

            (*lookup_data.verifyinstruction[0].get())[row_index] = [
                input_pc_col0,
                offset0_col3,
                M31_32767,
                M31_32769,
                M31_1,
                M31_1,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                ap_update_add_1_col4,
                M31_0,
                M31_0,
                M31_0,
            ];

            // ReadPositive_num_bits_252.

            let memoryaddresstoid_value_tmp_1c37_5 = memory_address_to_id_state
                .deduce_output(((input_fp_col2) + ((offset0_col3) - (M31_32768))));
            let memoryidtobig_value_tmp_1c37_6 =
                memory_id_to_big_state.deduce_output(memoryaddresstoid_value_tmp_1c37_5);
            let dst_id_col5 = memoryaddresstoid_value_tmp_1c37_5;
            (*trace.data[5].get()).data[row_index] = dst_id_col5;
            (*sub_components_inputs.memory_address_to_id_inputs[0].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&((input_fp_col2) + ((offset0_col3) - (M31_32768))).unpack());

            (*lookup_data.memoryaddresstoid[0].get())[row_index] = [
                ((input_fp_col2) + ((offset0_col3) - (M31_32768))),
                dst_id_col5,
            ];
            let dst_limb_0_col6 = memoryidtobig_value_tmp_1c37_6.get_m31(0);
            (*trace.data[6].get()).data[row_index] = dst_limb_0_col6;
            let dst_limb_1_col7 = memoryidtobig_value_tmp_1c37_6.get_m31(1);
            (*trace.data[7].get()).data[row_index] = dst_limb_1_col7;
            let dst_limb_2_col8 = memoryidtobig_value_tmp_1c37_6.get_m31(2);
            (*trace.data[8].get()).data[row_index] = dst_limb_2_col8;
            let dst_limb_3_col9 = memoryidtobig_value_tmp_1c37_6.get_m31(3);
            (*trace.data[9].get()).data[row_index] = dst_limb_3_col9;
            let dst_limb_4_col10 = memoryidtobig_value_tmp_1c37_6.get_m31(4);
            (*trace.data[10].get()).data[row_index] = dst_limb_4_col10;
            let dst_limb_5_col11 = memoryidtobig_value_tmp_1c37_6.get_m31(5);
            (*trace.data[11].get()).data[row_index] = dst_limb_5_col11;
            let dst_limb_6_col12 = memoryidtobig_value_tmp_1c37_6.get_m31(6);
            (*trace.data[12].get()).data[row_index] = dst_limb_6_col12;
            let dst_limb_7_col13 = memoryidtobig_value_tmp_1c37_6.get_m31(7);
            (*trace.data[13].get()).data[row_index] = dst_limb_7_col13;
            let dst_limb_8_col14 = memoryidtobig_value_tmp_1c37_6.get_m31(8);
            (*trace.data[14].get()).data[row_index] = dst_limb_8_col14;
            let dst_limb_9_col15 = memoryidtobig_value_tmp_1c37_6.get_m31(9);
            (*trace.data[15].get()).data[row_index] = dst_limb_9_col15;
            let dst_limb_10_col16 = memoryidtobig_value_tmp_1c37_6.get_m31(10);
            (*trace.data[16].get()).data[row_index] = dst_limb_10_col16;
            let dst_limb_11_col17 = memoryidtobig_value_tmp_1c37_6.get_m31(11);
            (*trace.data[17].get()).data[row_index] = dst_limb_11_col17;
            let dst_limb_12_col18 = memoryidtobig_value_tmp_1c37_6.get_m31(12);
            (*trace.data[18].get()).data[row_index] = dst_limb_12_col18;
            let dst_limb_13_col19 = memoryidtobig_value_tmp_1c37_6.get_m31(13);
            (*trace.data[19].get()).data[row_index] = dst_limb_13_col19;
            let dst_limb_14_col20 = memoryidtobig_value_tmp_1c37_6.get_m31(14);
            (*trace.data[20].get()).data[row_index] = dst_limb_14_col20;
            let dst_limb_15_col21 = memoryidtobig_value_tmp_1c37_6.get_m31(15);
            (*trace.data[21].get()).data[row_index] = dst_limb_15_col21;
            let dst_limb_16_col22 = memoryidtobig_value_tmp_1c37_6.get_m31(16);
            (*trace.data[22].get()).data[row_index] = dst_limb_16_col22;
            let dst_limb_17_col23 = memoryidtobig_value_tmp_1c37_6.get_m31(17);
            (*trace.data[23].get()).data[row_index] = dst_limb_17_col23;
            let dst_limb_18_col24 = memoryidtobig_value_tmp_1c37_6.get_m31(18);
            (*trace.data[24].get()).data[row_index] = dst_limb_18_col24;
            let dst_limb_19_col25 = memoryidtobig_value_tmp_1c37_6.get_m31(19);
            (*trace.data[25].get()).data[row_index] = dst_limb_19_col25;
            let dst_limb_20_col26 = memoryidtobig_value_tmp_1c37_6.get_m31(20);
            (*trace.data[26].get()).data[row_index] = dst_limb_20_col26;
            let dst_limb_21_col27 = memoryidtobig_value_tmp_1c37_6.get_m31(21);
            (*trace.data[27].get()).data[row_index] = dst_limb_21_col27;
            let dst_limb_22_col28 = memoryidtobig_value_tmp_1c37_6.get_m31(22);
            (*trace.data[28].get()).data[row_index] = dst_limb_22_col28;
            let dst_limb_23_col29 = memoryidtobig_value_tmp_1c37_6.get_m31(23);
            (*trace.data[29].get()).data[row_index] = dst_limb_23_col29;
            let dst_limb_24_col30 = memoryidtobig_value_tmp_1c37_6.get_m31(24);
            (*trace.data[30].get()).data[row_index] = dst_limb_24_col30;
            let dst_limb_25_col31 = memoryidtobig_value_tmp_1c37_6.get_m31(25);
            (*trace.data[31].get()).data[row_index] = dst_limb_25_col31;
            let dst_limb_26_col32 = memoryidtobig_value_tmp_1c37_6.get_m31(26);
            (*trace.data[32].get()).data[row_index] = dst_limb_26_col32;
            let dst_limb_27_col33 = memoryidtobig_value_tmp_1c37_6.get_m31(27);
            (*trace.data[33].get()).data[row_index] = dst_limb_27_col33;
            (*sub_components_inputs.memory_id_to_big_inputs[0].get())
                [row_index * N_LANES..(row_index + 1) * N_LANES]
                .copy_from_slice(&dst_id_col5.unpack());

            (*lookup_data.memoryidtobig[0].get())[row_index] = [
                dst_id_col5,
                dst_limb_0_col6,
                dst_limb_1_col7,
                dst_limb_2_col8,
                dst_limb_3_col9,
                dst_limb_4_col10,
                dst_limb_5_col11,
                dst_limb_6_col12,
                dst_limb_7_col13,
                dst_limb_8_col14,
                dst_limb_9_col15,
                dst_limb_10_col16,
                dst_limb_11_col17,
                dst_limb_12_col18,
                dst_limb_13_col19,
                dst_limb_14_col20,
                dst_limb_15_col21,
                dst_limb_16_col22,
                dst_limb_17_col23,
                dst_limb_18_col24,
                dst_limb_19_col25,
                dst_limb_20_col26,
                dst_limb_21_col27,
                dst_limb_22_col28,
                dst_limb_23_col29,
                dst_limb_24_col30,
                dst_limb_25_col31,
                dst_limb_26_col32,
                dst_limb_27_col33,
            ];

            (*lookup_data.opcodes[0].get())[row_index] =
                [input_pc_col0, input_ap_col1, input_fp_col2];
            (*lookup_data.opcodes[1].get())[row_index] = [
                ((input_pc_col0) + (M31_2)),
                ((input_ap_col1) + (ap_update_add_1_col4)),
                input_fp_col2,
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
    pub memoryaddresstoid: [UnsafeCell<Vec<[PackedM31; 2]>>; 1],
    pub memoryidtobig: [UnsafeCell<Vec<[PackedM31; 29]>>; 1],
    pub opcodes: [UnsafeCell<Vec<[PackedM31; 3]>>; 2],
    pub verifyinstruction: [UnsafeCell<Vec<[PackedM31; 19]>>; 1],
}
impl LookupData {
    unsafe fn uninitialized(len: usize) -> Self {
        let lookup_data = Self {
            memoryaddresstoid: [Vec::with_capacity(len).into()],
            memoryidtobig: [Vec::with_capacity(len).into()],
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

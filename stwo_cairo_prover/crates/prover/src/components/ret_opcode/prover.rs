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
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{bit_reverse, bit_reverse_coset_to_circle_domain_order};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};

use super::component::{Claim, InteractionClaim, RelationElements};
use crate::components::{memory, opcodes, pack_values, verifyinstruction};

pub type PackedInputType = PackedCasmState;
pub type InputType = CasmState;

const N_TRACE_COLUMNS: usize = 11;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(cpu_inputs: Vec<InputType>) -> Self {
        Self { inputs: cpu_inputs }
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memoryaddresstoid_state: &mut memory::addr_to_id::ClaimGenerator,
        memoryidtobig_state: &mut memory::id_to_f252::ClaimGenerator,
        verifyinstruction_state: &mut verifyinstruction::ClaimGenerator,
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
            write_trace_simd(simd_inputs, memoryaddresstoid_state, memoryidtobig_state);

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }

        // Add inputs.
        memoryaddresstoid_state
            .add_inputs(&sub_components_inputs.memoryaddresstoid_inputs[0][..n_calls]);
        memoryaddresstoid_state
            .add_inputs(&sub_components_inputs.memoryaddresstoid_inputs[1][..n_calls]);
        memoryaddresstoid_state
            .add_inputs(&sub_components_inputs.memoryaddresstoid_inputs[2][..n_calls]);
        memoryidtobig_state.add_inputs(&sub_components_inputs.memoryidtobig_inputs[0][..n_calls]);
        memoryidtobig_state.add_inputs(&sub_components_inputs.memoryidtobig_inputs[1][..n_calls]);
        memoryidtobig_state.add_inputs(&sub_components_inputs.memoryidtobig_inputs[2][..n_calls]);
        verifyinstruction_state
            .add_inputs(&sub_components_inputs.verifyinstruction_inputs[0][..n_calls]);

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
}

pub struct SubComponentInputs {
    pub memoryaddresstoid_inputs: [Vec<memory::addr_to_id::CpuInputType>; 3],
    pub memoryidtobig_inputs: [Vec<memory::id_to_f252::CpuInputType>; 3],
    pub verifyinstruction_inputs: [Vec<verifyinstruction::InputType>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid_inputs: std::array::from_fn(|_| Vec::with_capacity(capacity)),
            memoryidtobig_inputs: std::array::from_fn(|_| Vec::with_capacity(capacity)),
            verifyinstruction_inputs: std::array::from_fn(|_| Vec::with_capacity(capacity)),
        }
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        for vec in self.memoryaddresstoid_inputs.iter_mut() {
            bit_reverse_coset_to_circle_domain_order(vec);
        }
        for vec in self.memoryidtobig_inputs.iter_mut() {
            bit_reverse_coset_to_circle_domain_order(vec);
        }
        for vec in self.verifyinstruction_inputs.iter_mut() {
            bit_reverse_coset_to_circle_domain_order(vec);
        }
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
pub fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    memoryaddresstoid_state: &mut memory::addr_to_id::ClaimGenerator,
    memoryidtobig_state: &mut memory::id_to_f252::ClaimGenerator,
) -> (
    [BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData,
) {
    let capacity = (inputs.len() * N_LANES).next_power_of_two();
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32766 = PackedM31::broadcast(M31::from(32766));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_512 = PackedM31::broadcast(M31::from(512));

    inputs
        .into_iter()
        .enumerate()
        .for_each(|(row_index, retopcode_input)| {
            let tmp_0 = retopcode_input;
            let input_pc_col0 = tmp_0.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = tmp_0.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = tmp_0.fp;
            trace[2].data[row_index] = input_fp_col2;
            sub_components_inputs.memoryaddresstoid_inputs[0].extend(input_pc_col0.to_array());
            let tmp_55 = memoryaddresstoid_state.deduce_output(input_pc_col0);
            sub_components_inputs.memoryidtobig_inputs[0].extend(tmp_55.to_array());
            let tmp_56 = memoryidtobig_state.deduce_output(tmp_55);
            sub_components_inputs.verifyinstruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [
                        PackedM31::broadcast(M31(32766)),
                        PackedM31::broadcast(M31(32767)),
                        PackedM31::broadcast(M31(32767)),
                    ],
                    [
                        PackedM31::broadcast(M31(1)),
                        PackedM31::broadcast(M31(1)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(1)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(1)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(0)),
                        PackedM31::broadcast(M31(1)),
                        PackedM31::broadcast(M31(0)),
                    ],
                )
                    .unpack(),
            );
            lookup_data.verifyinstruction[0].push([
                input_pc_col0,
                M31_32766,
                M31_32767,
                M31_32767,
                M31_1,
                M31_1,
                M31_0,
                M31_1,
                M31_0,
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
            ]);
            sub_components_inputs.memoryaddresstoid_inputs[1]
                .extend(((input_fp_col2) - (M31_1)).to_array());
            let tmp_60 = memoryaddresstoid_state.deduce_output(((input_fp_col2) - (M31_1)));
            let next_pc_id_col3 = tmp_60;
            trace[3].data[row_index] = next_pc_id_col3;
            lookup_data.memoryaddresstoid[0].push([((input_fp_col2) - (M31_1)), next_pc_id_col3]);
            sub_components_inputs.memoryidtobig_inputs[1].extend(next_pc_id_col3.to_array());
            let tmp_61 = memoryidtobig_state.deduce_output(next_pc_id_col3);
            let next_pc_limb_0_col4 = tmp_61.get_m31(0);
            trace[4].data[row_index] = next_pc_limb_0_col4;
            let next_pc_limb_1_col5 = tmp_61.get_m31(1);
            trace[5].data[row_index] = next_pc_limb_1_col5;
            let next_pc_limb_2_col6 = tmp_61.get_m31(2);
            trace[6].data[row_index] = next_pc_limb_2_col6;
            lookup_data.memoryidtobig[0].push([
                next_pc_id_col3,
                next_pc_limb_0_col4,
                next_pc_limb_1_col5,
                next_pc_limb_2_col6,
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
            sub_components_inputs.memoryaddresstoid_inputs[2]
                .extend(((input_fp_col2) - (M31_2)).to_array());
            let tmp_62 = memoryaddresstoid_state.deduce_output(((input_fp_col2) - (M31_2)));
            let next_fp_id_col7 = tmp_62;
            trace[7].data[row_index] = next_fp_id_col7;
            lookup_data.memoryaddresstoid[1].push([((input_fp_col2) - (M31_2)), next_fp_id_col7]);
            sub_components_inputs.memoryidtobig_inputs[2].extend(next_fp_id_col7.to_array());
            let tmp_63 = memoryidtobig_state.deduce_output(next_fp_id_col7);
            let next_fp_limb_0_col8 = tmp_63.get_m31(0);
            trace[8].data[row_index] = next_fp_limb_0_col8;
            let next_fp_limb_1_col9 = tmp_63.get_m31(1);
            trace[9].data[row_index] = next_fp_limb_1_col9;
            let next_fp_limb_2_col10 = tmp_63.get_m31(2);
            trace[10].data[row_index] = next_fp_limb_2_col10;
            lookup_data.memoryidtobig[1].push([
                next_fp_id_col7,
                next_fp_limb_0_col8,
                next_fp_limb_1_col9,
                next_fp_limb_2_col10,
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
            lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes[1].push([
                (((next_pc_limb_0_col4) + ((next_pc_limb_1_col5) * (M31_512)))
                    + ((next_pc_limb_2_col6) * (M31_262144))),
                input_ap_col1,
                (((next_fp_limb_0_col8) + ((next_fp_limb_1_col9) * (M31_512)))
                    + ((next_fp_limb_2_col10) * (M31_262144))),
            ]);
        });

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub memoryaddresstoid: [Vec<[PackedM31; 2]>; 2],
    pub memoryidtobig: [Vec<[PackedM31; 29]>; 2],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
    pub opcodes: [Vec<[PackedM31; 3]>; 2],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memoryaddresstoid: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            memoryidtobig: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            verifyinstruction: [Vec::with_capacity(capacity)],
            opcodes: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
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
        verifyinstruction_lookup_elements: &verifyinstruction::RelationElements,
        _opcodes_lookup_elements: &opcodes::RelationElements,
    ) -> InteractionClaim {
        let log_size = self.n_calls.next_power_of_two().ilog2();
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

        // VM Constraint.

        // let mut col_gen = logup_gen.new_col();
        // let lookup_row = &self.lookup_data.opcodes[0];
        // for (i, lookup_values) in lookup_row.iter().enumerate() {
        //     let denom = opcodes_lookup_elements.combine(lookup_values);
        //     col_gen.write_frac(i, PackedQM31::one(), denom);
        //     println!("used {:?}, to opcodes",lookup_values);
        // }
        // col_gen.finalize_col();

        // let mut col_gen = logup_gen.new_col();
        // let lookup_row = &self.lookup_data.opcodes[1];
        // for (i, lookup_values) in lookup_row.iter().enumerate() {
        //     let denom = opcodes_lookup_elements.combine(lookup_values);
        //     col_gen.write_frac(i, -PackedQM31::one(), denom);
        // }
        // col_gen.finalize_col();

        let (trace, [total_sum, claimed_sum]) =
            logup_gen.finalize_at([(1 << log_size) - 1, self.n_calls - 1]);
        tree_builder.extend_evals(trace);

        InteractionClaim {
            total_sum,
            claimed_sum: (claimed_sum, self.n_calls - 1),
        }
    }
}

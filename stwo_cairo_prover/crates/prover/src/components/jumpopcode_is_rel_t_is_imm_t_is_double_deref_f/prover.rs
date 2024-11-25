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

use super::component::{Claim, InteractionClaim, RelationElements};
use crate::components::memory::{addr_to_id, id_to_f252};
use crate::components::{pack_values, verifyinstruction};
use crate::input::instructions::VmState;
use crate::relations::*;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 10;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(cpu_inputs: Vec<VmState>) -> Self {
        let cpu_inputs = cpu_inputs
            .into_iter()
            .map(|VmState { pc, ap, fp }| CasmState {
                pc: M31(pc),
                ap: M31(ap),
                fp: M31(fp),
            })
            .collect();
        Self { inputs: cpu_inputs }
    }
    
    pub fn write_trace(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        addr_to_id_state: &mut addr_to_id::ClaimGenerator,
        id_to_f252_state: &mut id_to_f252::ClaimGenerator,
        verifyinstruction_state: &mut verifyinstruction::ClaimGenerator,
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
        let (trace, mut sub_components_inputs, lookup_data) =
            write_trace_simd(packed_inputs, addr_to_id_state, id_to_f252_state);

        if need_padding {
            sub_components_inputs.bit_reverse_coset_to_circle_domain_order();
        }
        sub_components_inputs
            .addr_to_id_inputs
            .iter()
            .for_each(|inputs| {
                addr_to_id_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .id_to_f252_inputs
            .iter()
            .for_each(|inputs| {
                id_to_f252_state.add_inputs(&inputs[..n_calls]);
            });
        sub_components_inputs
            .verifyinstruction_inputs
            .iter()
            .for_each(|inputs| {
                verifyinstruction_state.add_inputs(&inputs[..n_calls]);
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
    pub addr_to_id_inputs: [Vec<addr_to_id::InputType>; 2],
    pub id_to_f252_inputs: [Vec<id_to_f252::InputType>; 2],
    pub verifyinstruction_inputs: [Vec<verifyinstruction::InputType>; 1],
}
impl SubComponentInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            addr_to_id_inputs: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            id_to_f252_inputs: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            verifyinstruction_inputs: [Vec::with_capacity(capacity)],
        }
    }

    fn bit_reverse_coset_to_circle_domain_order(&mut self) {
        self.addr_to_id_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.id_to_f252_inputs
            .iter_mut()
            .for_each(|vec| bit_reverse_coset_to_circle_domain_order(vec));
        self.verifyinstruction_inputs
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
    addr_to_id_state: &mut addr_to_id::ClaimGenerator,
    id_to_f252_state: &mut id_to_f252::ClaimGenerator,
) -> (
    [BaseColumn; N_TRACE_COLUMNS],
    SubComponentInputs,
    LookupData,
) {
    const N_TRACE_COLUMNS: usize = 10;
    let mut trace: [_; N_TRACE_COLUMNS] =
        std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES));

    let mut lookup_data = LookupData::with_capacity(inputs.len());
    #[allow(unused_mut)]
    let mut sub_components_inputs = SubComponentInputs::with_capacity(inputs.len());

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));

    inputs.into_iter().enumerate().for_each(
        |(row_index, jumpopcode_is_rel_t_is_imm_t_is_double_deref_f_input)| {
            let input_tmp_1264 = jumpopcode_is_rel_t_is_imm_t_is_double_deref_f_input;
            let input_pc_col0 = input_tmp_1264.pc;
            trace[0].data[row_index] = input_pc_col0;
            let input_ap_col1 = input_tmp_1264.ap;
            trace[1].data[row_index] = input_ap_col1;
            let input_fp_col2 = input_tmp_1264.fp;
            trace[2].data[row_index] = input_fp_col2;

            // DecodeInstruction_ccd4c4cd993af638.

            sub_components_inputs.addr_to_id_inputs[0].extend(input_pc_col0.unpack());
            let addr_to_id_value_tmp_1269 = addr_to_id_state.deduce_output(input_pc_col0);
            sub_components_inputs.id_to_f252_inputs[0].extend(addr_to_id_value_tmp_1269.unpack());
            let id_to_f252_value_tmp_1270 =
                id_to_f252_state.deduce_output(addr_to_id_value_tmp_1269);
            let ap_update_add_1_tmp_1271 =
                (((((PackedUInt16::from_m31(id_to_f252_value_tmp_1270.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(id_to_f252_value_tmp_1270.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col3 = ap_update_add_1_tmp_1271.as_m31();
            trace[3].data[row_index] = ap_update_add_1_col3;
            sub_components_inputs.verifyinstruction_inputs[0].extend(
                (
                    input_pc_col0,
                    [M31_32767, M31_32767, M31_32769],
                    [
                        M31_1,
                        M31_1,
                        M31_1,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_1,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col3,
                        M31_0,
                        M31_0,
                        M31_0,
                    ],
                )
                    .unpack(),
            );

            lookup_data.verifyinstruction[0].push([
                input_pc_col0,
                M31_32767,
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
                M31_1,
                M31_0,
                M31_0,
                ap_update_add_1_col3,
                M31_0,
                M31_0,
                M31_0,
            ]);

            // ReadSmall.

            sub_components_inputs.addr_to_id_inputs[1].extend(((input_pc_col0) + (M31_1)).unpack());
            let addr_to_id_value_tmp_1273 =
                addr_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            sub_components_inputs.id_to_f252_inputs[1].extend(addr_to_id_value_tmp_1273.unpack());
            let id_to_f252_value_tmp_1274 =
                id_to_f252_state.deduce_output(addr_to_id_value_tmp_1273);
            let next_pc_id_col4 = addr_to_id_value_tmp_1273;
            trace[4].data[row_index] = next_pc_id_col4;

            lookup_data.addr_to_id[0].push([((input_pc_col0) + (M31_1)), next_pc_id_col4]);

            // CondDecodeSmallSign.

            let msb_tmp_1275 = id_to_f252_value_tmp_1274.get_m31(27).eq(M31_256);
            let msb_col5 = msb_tmp_1275.as_m31();
            trace[5].data[row_index] = msb_col5;
            let mid_limbs_set_tmp_1276 = id_to_f252_value_tmp_1274.get_m31(20).eq(M31_511);
            let mid_limbs_set_col6 = mid_limbs_set_tmp_1276.as_m31();
            trace[6].data[row_index] = mid_limbs_set_col6;

            let next_pc_limb_0_col7 = id_to_f252_value_tmp_1274.get_m31(0);
            trace[7].data[row_index] = next_pc_limb_0_col7;
            let next_pc_limb_1_col8 = id_to_f252_value_tmp_1274.get_m31(1);
            trace[8].data[row_index] = next_pc_limb_1_col8;
            let next_pc_limb_2_col9 = id_to_f252_value_tmp_1274.get_m31(2);
            trace[9].data[row_index] = next_pc_limb_2_col9;

            lookup_data.id_to_f252[0].push([
                next_pc_id_col4,
                next_pc_limb_0_col7,
                next_pc_limb_1_col8,
                next_pc_limb_2_col9,
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                ((mid_limbs_set_col6) * (M31_511)),
                (((M31_136) * (msb_col5)) - (mid_limbs_set_col6)),
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ((msb_col5) * (M31_256)),
            ]);

            lookup_data.opcodes[0].push([input_pc_col0, input_ap_col1, input_fp_col2]);
            lookup_data.opcodes[1].push([
                ((input_pc_col0)
                    + (((((next_pc_limb_0_col7) + ((next_pc_limb_1_col8) * (M31_512)))
                        + ((next_pc_limb_2_col9) * (M31_262144)))
                        - (msb_col5))
                        - ((M31_134217728) * (mid_limbs_set_col6)))),
                ((input_ap_col1) + (ap_update_add_1_col3)),
                input_fp_col2,
            ]);
        },
    );

    (trace, sub_components_inputs, lookup_data)
}

pub struct LookupData {
    pub addr_to_id: [Vec<[PackedM31; 2]>; 1],
    pub id_to_f252: [Vec<[PackedM31; 29]>; 1],
    pub verifyinstruction: [Vec<[PackedM31; 19]>; 1],
    pub opcodes: [Vec<[PackedM31; 3]>; 2],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            addr_to_id: [Vec::with_capacity(capacity)],
            id_to_f252: [Vec::with_capacity(capacity)],
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
        addr_to_id_lookup_elements: &AddrToIdRelation,
        id_to_f252_lookup_elements: &IdToValueRelation,
        verifyinstruction_lookup_elements: &VerifyInstructionRelation,
        opcodes_lookup_elements: &VmRelation,
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
        let lookup_row = &self.lookup_data.addr_to_id[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = addr_to_id_lookup_elements.combine(lookup_values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        let lookup_row = &self.lookup_data.id_to_f252[0];
        for (i, lookup_values) in lookup_row.iter().enumerate() {
            let denom = id_to_f252_lookup_elements.combine(lookup_values);
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

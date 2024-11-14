use itertools::{chain, zip_eq, Itertools};
use num_traits::One;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{Claim, InteractionClaim, RET_INSTRUCTION};
use crate::components::memory::id_to_f252::component::N_M31_IN_FELT252;
use crate::components::memory::{self, id_to_f252};
use crate::input::instructions::VmState;

const N_MEMORY_CALLS: usize = 3;

// TODO(Ohad): take from prover_types and remove.
pub struct PackedCasmState {
    pub pc: PackedM31,
    pub ap: PackedM31,
    pub fp: PackedM31,
}

pub struct ClaimGenerator {
    pub inputs: Vec<PackedCasmState>,
}
impl ClaimGenerator {
    pub fn new(mut inputs: Vec<VmState>) -> Self {
        assert!(!inputs.is_empty());

        // TODO(spapini): Split to multiple components.
        let size = inputs.len().next_power_of_two();
        inputs.resize(size, inputs[0].clone());

        let inputs = inputs
            .into_iter()
            .array_chunks::<N_LANES>()
            .map(|chunk| PackedCasmState {
                pc: PackedM31::from_array(std::array::from_fn(|i| {
                    M31::from_u32_unchecked(chunk[i].pc)
                })),
                ap: PackedM31::from_array(std::array::from_fn(|i| {
                    M31::from_u32_unchecked(chunk[i].ap)
                })),
                fp: PackedM31::from_array(std::array::from_fn(|i| {
                    M31::from_u32_unchecked(chunk[i].fp)
                })),
            })
            .collect_vec();
        Self { inputs }
    }
    pub fn write_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_addr_to_id_state: &mut memory::addr_to_id::ClaimGenerator,
        memory_trace_generator: &mut id_to_f252::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let (trace, interaction_prover) = write_trace_simd(
            &self.inputs,
            memory_addr_to_id_state,
            memory_trace_generator,
        );
        // interaction_prover.memory_inputs.iter().for_each(|c| {
        //     c.iter()
        //         .for_each(|v| memory_trace_generator.add_packed_m31(v))
        // });
        tree_builder.extend_evals(trace);
        let claim = Claim {
            n_rets: self.inputs.len() * N_LANES,
        };
        (claim, interaction_prover)
    }
}

pub struct InteractionClaimGenerator {
    pub memory_inputs: [Vec<PackedM31>; N_MEMORY_CALLS],
    pub memory_outputs: [Vec<[PackedM31; N_M31_IN_FELT252]>; N_MEMORY_CALLS],
    // Callee data.
    // pc: Vec<PackedM31>,
    // fp: Vec<PackedM31>,
    // instr: Vec<PackedM31>,
    // new_pc: Vec<PackedM31>,
    // new_fp: Vec<PackedM31>,
}
impl InteractionClaimGenerator {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory_inputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
            memory_outputs: [
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
                Vec::with_capacity(capacity),
            ],
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &id_to_f252::RelationElements,
    ) -> InteractionClaim {
        let log_size = self.memory_inputs[0].len().ilog2() + LOG_N_LANES;
        let mut logup_gen = LogupTraceGenerator::new(log_size);
        for col_index in 0..N_MEMORY_CALLS {
            let mut col_gen = logup_gen.new_col();
            for (i, (addr, output)) in zip_eq(
                &self.memory_inputs[col_index],
                &self.memory_outputs[col_index],
            )
            .enumerate()
            {
                let address_and_value = chain!([addr], output).copied().collect_vec();
                let denom = lookup_elements.combine(&address_and_value);
                col_gen.write_frac(i, PackedQM31::one(), denom);
            }
            col_gen.finalize_col();
        }
        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim {
            log_size,
            claimed_sum,
        }
    }
}

fn write_trace_simd(
    inputs: &[PackedCasmState],
    memory_addr_to_id_state: &memory::addr_to_id::ClaimGenerator,
    memory_trace_generator: &id_to_f252::ClaimGenerator,
) -> (
    Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
    InteractionClaimGenerator,
) {
    let n_trace_columns = 7;
    let mut trace_values = (0..n_trace_columns)
        .map(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES))
        .collect_vec();
    let mut sub_components_inputs = InteractionClaimGenerator::with_capacity(inputs.len());
    inputs.iter().enumerate().for_each(|(i, input)| {
        write_trace_row(
            &mut trace_values,
            input,
            i,
            &mut sub_components_inputs,
            memory_addr_to_id_state,
            memory_trace_generator,
        );
    });

    let trace = trace_values
        .into_iter()
        .map(|eval| {
            // TODO(Ohad): Support non-power of 2 inputs.
            let domain = CanonicCoset::new(
                eval.len()
                    .checked_ilog2()
                    .expect("Input is not a power of 2!"),
            )
            .circle_domain();
            CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, eval)
        })
        .collect_vec();

    (trace, sub_components_inputs)
}

// Ret trace row:
// | pc | ap | fp | [fp-1].0 | [fp-1].1 | [fp-2].0 | [fp-2].1 |
// TODO(Ohad): redo when air team decides how it should look.
fn write_trace_row(
    dst: &mut [Col<SimdBackend, M31>],
    ret_opcode_input: &PackedCasmState,
    row_index: usize,
    lookup_data: &mut InteractionClaimGenerator,
    memory_addr_to_id_state: &memory::addr_to_id::ClaimGenerator,
    memory_trace_generator: &id_to_f252::ClaimGenerator,
) {
    let col0 = ret_opcode_input.pc;
    dst[0].data[row_index] = col0;
    let col1 = ret_opcode_input.ap;
    dst[1].data[row_index] = col1;
    let col2 = ret_opcode_input.fp;
    dst[2].data[row_index] = col2;
    lookup_data.memory_inputs[0].push(col0);
    lookup_data.memory_inputs[1].push((col2) - (PackedM31::broadcast(M31::one())));
    lookup_data.memory_outputs[0].push(RET_INSTRUCTION.map(|v| PackedM31::broadcast(M31::from(v))));
    let id = memory_addr_to_id_state.deduce_output((col2) - (PackedM31::broadcast(M31::from(1))));
    let mem_fp_minus_one = memory_trace_generator.deduce_output(id).value;
    lookup_data.memory_outputs[1].push(mem_fp_minus_one);

    let col3 = mem_fp_minus_one[0];
    dst[3].data[row_index] = col3;
    let col4 = mem_fp_minus_one[1];
    dst[4].data[row_index] = col4;
    lookup_data.memory_inputs[2].push((col2) - (PackedM31::broadcast(M31::from(2))));
    let id = memory_addr_to_id_state.deduce_output((col2) - (PackedM31::broadcast(M31::from(2))));
    let mem_fp_minus_two = memory_trace_generator.deduce_output(id).value;
    lookup_data.memory_outputs[2].push(mem_fp_minus_two);
    let col5 = mem_fp_minus_two[0];
    dst[5].data[row_index] = col5;
    let col6 = mem_fp_minus_two[1];
    dst[6].data[row_index] = col6;
}

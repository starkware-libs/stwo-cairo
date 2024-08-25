use std::simd::Simd;

use itertools::Itertools;
use stwo_prover::core::backend::simd::m31::{LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::OpcodeClaim;
use super::{Opcode, OpcodeGenContext};
use crate::components::utils::to_evals;
use crate::components::StandardInteractionProver;
use crate::input::instructions::VmState;

pub const N_REPETITIONS: usize = 4;

pub struct PackedVmState {
    pub pc: Simd<u32, N_LANES>,
    pub ap: Simd<u32, N_LANES>,
    pub fp: Simd<u32, N_LANES>,
}

pub struct OpcodeProver<O: Opcode> {
    pub inputs: Vec<PackedVmState>,
    n_instances: usize,
    phantom: std::marker::PhantomData<O>,
}
impl<O: Opcode> OpcodeProver<O> {
    pub fn new(mut inputs: Vec<VmState>) -> Self {
        assert!(!inputs.is_empty());

        // TODO(spapini): Split to multiple components.
        let n_instances = inputs.len();
        let size = n_instances.next_power_of_two().max(64 * N_REPETITIONS);

        // Pad the input vector to a power of 2.
        inputs.resize(size, inputs[0].clone());

        // Transpose the input vectors to arrays of SIMD lanes.
        let inputs = inputs
            .into_iter()
            .array_chunks::<N_LANES>()
            .map(|chunk| PackedVmState {
                pc: Simd::from_array(std::array::from_fn(|i| chunk[i].pc)),
                ap: Simd::from_array(std::array::from_fn(|i| chunk[i].ap)),
                fp: Simd::from_array(std::array::from_fn(|i| chunk[i].fp)),
            })
            .collect_vec();
        Self {
            inputs,
            n_instances,
            phantom: std::marker::PhantomData,
        }
    }
    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        ctx: &mut OpcodeGenContext<'_>,
    ) -> (OpcodeClaim<O>, StandardInteractionProver<O::LookupData>) {
        let inputs = self.inputs;
        let n_cols = O::n_columns();
        let mut trace_values = (0..(n_cols * N_REPETITIONS))
            .map(|_| Col::<SimdBackend, M31>::zeros(inputs.len() / N_REPETITIONS * N_LANES))
            .collect_vec();

        let log_size = inputs.len().ilog2() + LOG_N_LANES;
        let mut lookup_data = [log_size; N_REPETITIONS].map(O::new_lookup_data);

        for (row_index, states) in inputs.array_chunks::<N_REPETITIONS>().enumerate() {
            for i in 0..N_REPETITIONS {
                let offset = n_cols * i;
                // Write opcode columns.
                O::write_trace_row(
                    &mut trace_values[offset..(offset + n_cols)],
                    &states[i],
                    row_index,
                    ctx,
                    &mut lookup_data[i],
                );
            }
        }

        tree_builder.extend_evals(to_evals(trace_values));
        let claim = OpcodeClaim {
            log_size: self.inputs.len().ilog2() + LOG_N_LANES,
            n_instances: self.n_instances,
            phantom: std::marker::PhantomData,
        };

        (
            claim,
            StandardInteractionProver {
                log_size,
                lookup_data: lookup_data.into_iter().collect_vec(),
            },
        )
    }
}

use std::simd::Simd;

use itertools::Itertools;
use num_traits::Zero;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{OpcodeClaim, OpcodeInteractionClaim};
use super::{Opcode, OpcodeElements, OpcodeGenContext};
use crate::components::utils::to_evals;
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
        ctx: OpcodeGenContext<'_>,
    ) -> OpcodeClaimProver<O> {
        let inputs = self.inputs;
        let n_cols = O::n_columns();
        let mut trace_values = (0..(n_cols * N_REPETITIONS))
            .map(|_| Col::<SimdBackend, M31>::zeros(inputs.len() / N_REPETITIONS * N_LANES))
            .collect_vec();

        let log_size = inputs.len().ilog2() + LOG_N_LANES;
        let mut lookup_data = [O::new_lookup_data(log_size); N_REPETITIONS];

        for (row_index, states) in inputs.array_chunks::<N_REPETITIONS>().enumerate() {
            for i in 0..N_REPETITIONS {
                let offset = n_cols * i;
                // Write opcode columns.
                O::write_trace_row(
                    &mut trace_values[offset..(offset + n_cols)],
                    &states[i],
                    row_index,
                    ctx,
                    &mut lookup_data,
                );
            }
        }

        tree_builder.extend_evals(to_evals(trace_values));
        let claim = OpcodeClaim {
            log_size: self.inputs.len().ilog2() + LOG_N_LANES,
            n_instances: self.n_instances,
            phantom: std::marker::PhantomData,
        };

        OpcodeClaimProver {
            log_size,
            claim,
            lookup_data,
        }
    }
}

pub struct OpcodeClaimProver<O: Opcode> {
    log_size: u32,
    pub claim: OpcodeClaim<O>,
    lookup_data: [O::LookupData; N_REPETITIONS],
}
impl<O: Opcode> OpcodeClaimProver<O> {
    pub fn claim(&self) -> &OpcodeClaim<O> {
        &self.claim
    }
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        opcode_elements: &OpcodeElements,
    ) -> OpcodeInteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);
        let mut lookups = self
            .lookup_data
            .into_iter
            .flat_map(|ld| O::lookups(&ld, opcode_elements));
        let it = lookups.array_chunks();

        // TODO: Do this in chunks.
        let mut fracs0 = vec![
            Fraction::new(PackedM31::zero(), PackedQM31::zero());
            1 << (self.log_size - LOG_N_LANES)
        ];
        let mut fracs1 = vec![
            Fraction::new(PackedM31::zero(), PackedQM31::zero());
            1 << (self.log_size - LOG_N_LANES)
        ];

        for [l0, l1] in it {
            let mut col_gen = logup_gen.new_col();
            l0(0, &mut fracs0);
            l1(1, &mut fracs1);
            for vec_row in 0..(1 << (self.log_size - LOG_N_LANES)) {
                let numerator = fracs0[vec_row].numerator * fracs1[vec_row].denominator
                    + fracs0[vec_row].denominator * fracs1[vec_row].numerator;
                let denom = fracs0[vec_row].denominator * fracs1[vec_row].denominator;
                col_gen.push_frac(vec_row, numerator, denom);
            }
            col_gen.finalize_col();
        }
        // TODO: Remainder.
        assert!(it.remainder().is_empty());

        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        OpcodeInteractionClaim {
            log_size: self.log_size,
            claimed_sum,
        }
    }
}

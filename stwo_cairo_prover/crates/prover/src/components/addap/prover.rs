#![allow(unused_parens)]
#![allow(clippy::too_many_arguments)]
#![allow(unused_imports)]
use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::VmStateLookupElements;
use crate::components::memory::component::N_M31_IN_FELT252;
use crate::components::memory::prover::MemoryClaimGenerator;
use crate::components::memory::MemoryLookupElements;
use crate::input::instructions::VmState;
use crate::prover_types::{EqExtend, PackedCasmState, PackedFelt252};

pub type InputType = PackedCasmState;
pub type OutputType = PackedCasmState;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(mut inputs: Vec<VmState>) -> Self {
        assert!(!inputs.is_empty());

        // TODO(spapini): Split to multiple components.
        let size = inputs.len().next_power_of_two().max(64);
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
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_state: &mut MemoryClaimGenerator,
    ) -> ClaimProver {
        let len = self.inputs.len();
        let (trace, lookup_data) = write_trace_simd(self.inputs, memory_state);
        lookup_data
            .memory_inputs
            .iter()
            .for_each(|c| c.iter().for_each(|v| memory_state.add_inputs_simd(v)));

        tree_builder.extend_evals(trace);
        let claim = Claim {
            log_size: len.ilog2() + LOG_N_LANES,
            n_calls: len * N_LANES,
        };

        ClaimProver { claim, lookup_data }
    }
}

pub fn write_trace_simd(
    inputs: Vec<InputType>,
    memory_state: &mut MemoryClaimGenerator,
) -> (
    Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
    LookupData,
) {
    let n_trace_columns = 8;
    let mut trace_values = (0..n_trace_columns)
        .map(|_| Col::<SimdBackend, M31>::zeros(inputs.len() * N_LANES))
        .collect_vec();
    let mut sub_components_inputs = LookupData::with_capacity(inputs.len());
    inputs.into_iter().enumerate().for_each(|(i, input)| {
        write_trace_row(
            &mut trace_values,
            input,
            i,
            &mut sub_components_inputs,
            memory_state,
        );
    });

    let trace = trace_values
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
    (trace, sub_components_inputs)
}
#[allow(clippy::useless_conversion)]
#[allow(clippy::let_unit_value)]
fn write_trace_row(
    #[allow(unused_variables)] dst: &mut [Col<SimdBackend, M31>],
    addap_debc7bc8d7fbc47e_input: InputType,
    row_index: usize,
    lookup_data: &mut LookupData,
    memory_state: &mut MemoryClaimGenerator,
) {
    let tmp_0 = [
        addap_debc7bc8d7fbc47e_input.pc.into(),
        addap_debc7bc8d7fbc47e_input.ap.into(),
        addap_debc7bc8d7fbc47e_input.fp.into(),
    ];
    let col0 = tmp_0[0];
    dst[0].data[row_index] = col0;
    let col1 = tmp_0[1];
    dst[1].data[row_index] = col1;
    let col2 = tmp_0[2];
    dst[2].data[row_index] = col2;
    lookup_data.memory_inputs[0].push(col0.into());
    let tmp_3 = memory_state.deduce_output(col0.into());
    lookup_data.memory_outputs[0].push(tmp_3.into());
    let _col3 = tmp_3;
    lookup_data.memory_inputs[1]
        .push(((col0) + (PackedM31::broadcast(M31::from(1).into()))).into());
    let tmp_9 =
        memory_state.deduce_output(((col0) + (PackedM31::broadcast(M31::from(1).into()))).into());
    lookup_data.memory_outputs[1].push(tmp_9.into());
    let tmp_11 = tmp_9
        .get_m31(27)
        .eq(PackedM31::broadcast(M31::from(256).into()));
    let col5 = tmp_11.as_m31();
    dst[3].data[row_index] = col5;
    let tmp_12 = tmp_9
        .get_m31(20)
        .eq(PackedM31::broadcast(M31::from(511).into()));
    let col6 = tmp_12.as_m31();
    dst[4].data[row_index] = col6;
    let col7 = tmp_9.get_m31(0);
    dst[5].data[row_index] = col7;
    let col8 = tmp_9.get_m31(1);
    dst[6].data[row_index] = col8;
    let col9 = tmp_9.get_m31(2);
    dst[7].data[row_index] = col9;

    lookup_data.self_inputs.push(addap_debc7bc8d7fbc47e_input);
    lookup_data.self_outputs.push(
        [
            ((col0) + (PackedM31::broadcast(M31::from(2).into()))),
            ((col1)
                + (((((col9) * (PackedM31::broadcast(M31::from(262144).into())))
                    + (((col8) * (PackedM31::broadcast(M31::from(512).into()))) + (col7)))
                    - (col5))
                    - ((PackedM31::broadcast(M31::from(134217728).into())) * (col6)))),
            col2,
        ]
        .into(),
    );
}

#[allow(non_snake_case)]
pub struct LookupData {
    pub self_inputs: Vec<InputType>,
    pub self_outputs: Vec<OutputType>,
    pub memory_inputs: [Vec<PackedM31>; 2],
    pub memory_outputs: [Vec<PackedFelt252>; 2],
}
impl LookupData {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            self_inputs: Vec::with_capacity(capacity),
            self_outputs: Vec::with_capacity(capacity),
            memory_inputs: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
            memory_outputs: [Vec::with_capacity(capacity), Vec::with_capacity(capacity)],
        }
    }
}

pub struct ClaimProver {
    pub claim: Claim,
    pub lookup_data: LookupData,
}
impl ClaimProver {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        _vm_lookup_elements: &VmStateLookupElements,
        memory_lookup_elements: &MemoryLookupElements,
    ) -> InteractionClaim {
        let log_size = self.claim.log_size;
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        for (inputs, outputs) in zip_eq(
            self.lookup_data.memory_inputs,
            self.lookup_data.memory_outputs,
        ) {
            let mut col_gen = logup_gen.new_col();
            for (i, (input, output)) in zip_eq(inputs, outputs).enumerate() {
                let lookup_values = chain!([input], output.value).collect_vec();
                let denom = memory_lookup_elements.combine(lookup_values.as_ref());
                col_gen.write_frac(i, PackedQM31::one(), denom);
            }
            col_gen.finalize_col();
        }

        // VM state.
        // let mut col_gen = logup_gen.new_col();
        // for (vec_row, (input, output)) in
        //     zip_eq(self.lookup_data.self_inputs, self.lookup_data.self_outputs).enumerate()
        // {
        //     let lookup_values = input.concat(&output);
        //     let denom = self_lookup_elements.combine(lookup_values.as_ref());
        //     col_gen.write_frac(vec_row, PackedQM31::one(), denom);
        // }
        // col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

use itertools::{chain, Itertools};
use num_traits::One;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo_prover::core::ColumnVec;

use super::component::{
    RangeCheckBuiltinClaim, RangeCheckBuiltinInteractionClaim, N_RANGE_CHECK_COLUMNS,
    N_VALUES_FELTS,
};
use crate::components::memory::prover::MemoryClaimProver;
use crate::components::memory::MemoryLookupElements;
use crate::input::SegmentAddrs;

// Memory addresses for the RangeCheckBuiltin segment.
pub type RangeCheckBuiltinInput = PackedM31;

pub struct RangeCheckBuiltinClaimProver {
    pub memory_segment: SegmentAddrs,
}

impl RangeCheckBuiltinClaimProver {
    pub fn new(input: SegmentAddrs) -> Self {
        Self {
            memory_segment: input,
        }
    }

    pub fn write_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_trace_generator: &mut MemoryClaimProver,
    ) -> (RangeCheckBuiltinClaim, RangeCheckBuiltinInteractionProver) {
        let mut addresses = self.memory_segment.addresses();
        // TODO(spapini): Split to multiple components.
        let size = addresses.len().next_power_of_two();
        // TODO(AlonH): Addresses should be increasing.
        addresses.resize(size, addresses[0]);

        let inputs = addresses
            .into_iter()
            .array_chunks::<N_LANES>()
            .map(|chunk| {
                PackedM31::from_array(std::array::from_fn(|i| M31::from_u32_unchecked(chunk[i])))
            })
            .collect_vec();
        let (trace, interaction_prover) = write_trace_simd(&inputs, memory_trace_generator);
        interaction_prover
            .memory_addresses
            .iter()
            .for_each(|v| memory_trace_generator.add_inputs_simd(v));
        tree_builder.extend_evals(trace);
        let claim = RangeCheckBuiltinClaim {
            memory_segment: self.memory_segment.clone(),
        };
        (claim, interaction_prover)
    }
}

pub struct RangeCheckBuiltinInteractionProver {
    pub memory_addresses: Vec<PackedM31>,
    pub memory_values: Vec<[PackedM31; N_VALUES_FELTS]>,
}

impl RangeCheckBuiltinInteractionProver {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            memory_addresses: Vec::with_capacity(capacity),
            memory_values: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        memory_lookup_elements: &MemoryLookupElements,
    ) -> RangeCheckBuiltinInteractionClaim {
        let log_size = self.memory_addresses.len().ilog2() + LOG_N_LANES;
        let (trace, claimed_sum) = gen_interaction_trace(self, log_size, memory_lookup_elements);
        tree_builder.extend_evals(trace);

        RangeCheckBuiltinInteractionClaim {
            log_size,
            claimed_sum,
        }
    }
}

pub fn write_trace_simd(
    inputs: &[RangeCheckBuiltinInput],
    memory_trace_generator: &MemoryClaimProver,
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    RangeCheckBuiltinInteractionProver,
) {
    let log_size = inputs.len().ilog2() + LOG_N_LANES;
    let mut interaction_prover = RangeCheckBuiltinInteractionProver::with_capacity(inputs.len());
    let mut trace = (0..N_RANGE_CHECK_COLUMNS)
        .map(|_| unsafe { Col::<SimdBackend, M31>::uninitialized(1 << log_size) })
        .collect_vec();

    let address_initial_offset = PackedM31::broadcast(BaseField::from(inputs[0].into_simd()[0]));
    #[allow(clippy::needless_range_loop)]
    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let row_input = *inputs.get(vec_row).unwrap();
        // TODO: remove address from the trace.
        let split_values: [PackedM31; N_VALUES_FELTS] = memory_trace_generator
            .deduce_output(row_input)[..N_VALUES_FELTS]
            .try_into()
            .unwrap();
        let address = row_input - address_initial_offset;
        trace[0].data[vec_row] = address;
        for (i, v) in split_values.iter().enumerate() {
            trace[i + 1].data[vec_row] = *v;
        }
        let last_value_felt = split_values[N_VALUES_FELTS - 1];
        trace[N_VALUES_FELTS + 1].data[vec_row] =
            last_value_felt * (last_value_felt - PackedM31::one());

        interaction_prover.memory_addresses.push(row_input);
        interaction_prover.memory_values.push(split_values);
    }

    let domain = CanonicCoset::new(log_size).circle_domain();
    (
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec(),
        interaction_prover,
    )
}

pub fn gen_interaction_trace(
    interaction_prover: &RangeCheckBuiltinInteractionProver,
    log_size: u32,
    memory_lookup_elements: &MemoryLookupElements,
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    SecureField,
) {
    let mut logup_gen = LogupTraceGenerator::new(log_size);
    let mut col_gen = logup_gen.new_col();
    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let p_mem: PackedQM31 = memory_lookup_elements.combine(
            &chain!(
                [interaction_prover.memory_addresses[vec_row]],
                interaction_prover.memory_values[vec_row]
            )
            .collect_vec(),
        );
        col_gen.write_frac(vec_row, PackedQM31::one(), p_mem);
    }
    col_gen.finalize_col();

    logup_gen.finalize()
}

#[cfg(test)]
mod tests {
    use std::array;
    use std::simd::Simd;

    use itertools::zip_eq;
    use rand::Rng;
    use stwo_prover::constraint_framework::FrameworkEval;
    use stwo_prover::core::backend::simd::m31::N_LANES;
    use stwo_prover::core::channel::Blake2sChannel;
    use stwo_prover::core::pcs::TreeVec;

    use super::*;
    use crate::components::memory::{MemoryLookupElements, N_ADDRESS_FELTS, N_BITS_PER_FELT};
    use crate::components::range_check_builtin::component::RangeCheckBuiltinEval;
    use crate::felt::split_f252;
    use crate::prover_types::PackedUInt32;

    #[test]
    fn test_generate_trace() {
        use super::*;

        let mut rng = rand::thread_rng();
        let log_size = 8;
        let inputs = (0..1 << (log_size - LOG_N_LANES))
            .map(|i| {
                PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32)
                }))
            })
            .collect_vec();

        let values = (0..1 << (log_size - LOG_N_LANES))
            .map(|_| {
                array::from_fn(|i| {
                    if i < 4 {
                        Simd::from_array(rng.gen())
                    } else {
                        Simd::splat(0)
                    }
                })
            })
            .collect_vec();
        let memory_trace_generator = MemoryClaimProver {
            values: values.clone(),
            multiplicities: vec![PackedUInt32::broadcast(0); 1 << (log_size - LOG_N_LANES)],
        };
        let (trace, interaction_prover) = write_trace_simd(&inputs, &memory_trace_generator);

        assert_eq!(trace.len(), N_RANGE_CHECK_COLUMNS);
        assert_eq!(
            trace[0].values.clone().into_cpu_vec(),
            (0..1 << log_size).map(M31::from).collect_vec()
        );

        // Assert that the trace values are correct.
        #[allow(clippy::needless_range_loop)]
        for row_offset in 0..1 << (log_size - LOG_N_LANES) {
            let input = values[row_offset];

            let mut inputs_u128 = [0_u128; 16];
            for (index, simd) in input.iter().enumerate() {
                for (i, val) in simd.to_array().into_iter().enumerate() {
                    if index >= 4 {
                        assert_eq!(val, 0);
                        continue;
                    }
                    let val_u128 = val as u128;
                    inputs_u128[i] += val_u128 << (32 * index);
                }
            }

            let mask = ((1 << N_BITS_PER_FELT) - 1) as u128;
            for col in trace.iter().skip(N_ADDRESS_FELTS).take(N_VALUES_FELTS) {
                for j in 0..N_LANES {
                    let val = col.values.at((row_offset << LOG_N_LANES) + j);
                    assert_eq!(val.0, (inputs_u128[j] & mask) as u32);
                    inputs_u128[j] >>= N_BITS_PER_FELT;
                }
            }

            let last_value_felts = trace[N_ADDRESS_FELTS + N_VALUES_FELTS - 1].values.clone();
            let intermediate_values = trace[N_ADDRESS_FELTS + N_VALUES_FELTS].values.clone();
            for (last_value_felt, intermediate_value) in
                zip_eq(last_value_felts.data.clone(), intermediate_values.data)
            {
                assert_eq!(
                    intermediate_value.to_array(),
                    (last_value_felt * (last_value_felt - PackedM31::one())).to_array()
                );
            }
            // Assert that the high values are in range [0, 4).
            last_value_felts.into_cpu_vec().iter().all(|&x| x.0 < 4);
        }

        // Assert memory addresses lookup are sequential, offset by `address_initial_offset`.
        assert_eq!(
            (1 + interaction_prover.memory_values[0].len()) * N_LANES,
            1 << log_size
        );
        for (i, addresses) in interaction_prover.memory_addresses.iter().enumerate() {
            assert_eq!(
                addresses.to_array(),
                array::from_fn(|j| { M31::from_u32_unchecked((i * N_LANES + j) as u32) })
            );
        }
    }

    #[test]
    fn test_generate_interaction_trace() {
        let mut rng = rand::thread_rng();
        let log_size = 8;
        let mem_log_size = log_size + 1;
        let address_initial_offset = 256;
        let inputs = (0..1 << (log_size - LOG_N_LANES))
            .map(|i| {
                PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32 + address_initial_offset)
                }))
            })
            .collect_vec();

        let values = (0..1 << (mem_log_size - LOG_N_LANES))
            .map(|_| {
                array::from_fn(|i| {
                    if i < 4 {
                        Simd::from_array(rng.gen())
                    } else {
                        Simd::splat(0)
                    }
                })
            })
            .collect_vec();
        let memory_trace_generator = MemoryClaimProver {
            values: values.clone(),
            multiplicities: vec![PackedUInt32::broadcast(0); 1 << (mem_log_size - LOG_N_LANES)],
        };
        let (trace, interaction_prover) = write_trace_simd(&inputs, &memory_trace_generator);

        let channel = &mut Blake2sChannel::default();
        let memory_lookup_elements = MemoryLookupElements::draw(channel);

        let (interaction_trace, claimed_sum) =
            gen_interaction_trace(&interaction_prover, log_size, &memory_lookup_elements);

        let trace = TreeVec::new(vec![trace, interaction_trace]);
        let trace_polys = trace.map_cols(|c| c.interpolate());

        let component = RangeCheckBuiltinEval {
            log_size,
            initial_memory_address: M31::from(address_initial_offset),
            memory_lookup_elements,
            claimed_sum,
        };

        stwo_prover::constraint_framework::assert_constraints(
            &trace_polys,
            CanonicCoset::new(log_size),
            |eval| {
                component.evaluate(eval);
            },
        )
    }

    #[test]
    fn test_split() {
        let x: [u32; 8] = [
            0x12345678, 0x9abcdef0, 0x13579bdf, 0x2468ace0, 0x12345678, 0x9abcdef0, 0x13579bdf, 0,
        ];
        let res = split_f252(x);
        assert_eq!(
            res,
            [
                120, 43, 141, 2, 495, 486, 106, 447, 411, 427, 4, 412, 138, 291, 480, 172, 52, 9,
                444, 411, 427, 252, 111, 175, 19, 0, 0, 0
            ]
            .map(M31::from)
        );
    }
}

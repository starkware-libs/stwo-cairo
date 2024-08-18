use std::simd::u32x16;

use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use super::component::{LAST_VALUE_OFFSET, N_VALUES_FELTS};
use crate::components::memory::{MemoryLookupElements, N_ADDRESS_FELTS, N_BITS_PER_FELT};
use crate::components::range_check_unit::RangeElements;
use crate::felt::split_u128_simd;

// Memory addresses and the corresponding values, for the RangeCheck128Builtin segment.
pub struct RangeCheckInput {
    pub address: PackedM31,
    pub values: [u32x16; 4],
}

impl Default for RangeCheckInput {
    fn default() -> Self {
        Self {
            address: PackedM31::zero(),
            values: [u32x16::splat(0); 4],
        }
    }
}

pub struct RangeCheck128BuiltinLookupData {
    pub memory_lookups: [BaseColumn; N_ADDRESS_FELTS + N_VALUES_FELTS],
}
impl RangeCheck128BuiltinLookupData {
    fn new(log_size: u32) -> Self {
        Self {
            memory_lookups: std::array::from_fn(|_| unsafe {
                BaseColumn::uninitialized(1 << log_size)
            }),
        }
    }
}

pub fn generate_trace(
    log_size: u32,
    address_initial_offset: M31,
    inputs: &[RangeCheckInput],
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    RangeCheck128BuiltinLookupData,
) {
    let mut lookup_data = RangeCheck128BuiltinLookupData::new(log_size);
    let mut trace = (0..N_ADDRESS_FELTS + N_VALUES_FELTS)
        .map(|_| unsafe { Col::<SimdBackend, M31>::uninitialized(1 << log_size) })
        .collect_vec();

    #[allow(clippy::needless_range_loop)]
    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let row_input = inputs.get(vec_row).unwrap();
        // TODO: remove address from the trace.
        let address = row_input.address - PackedM31::broadcast(address_initial_offset);
        let split_values = split_u128_simd(row_input.values);
        trace[0].data[vec_row] = address;
        for (i, v) in split_values.iter().enumerate() {
            trace[i + 1].data[vec_row] = *v;
        }

        chain![vec![row_input.address], split_values]
            .enumerate()
            .for_each(|(i, val)| lookup_data.memory_lookups[i].data[vec_row] = val);
    }

    let domain = CanonicCoset::new(log_size).circle_domain();
    (
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec(),
        lookup_data,
    )
}

pub fn gen_interaction_trace(
    log_size: u32,
    lookup_data: RangeCheck128BuiltinLookupData,
    memory_lookup_elements: &MemoryLookupElements,
    range2_lookup_elements: &RangeCheckElements,
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    SecureField,
) {
    let mut logup_gen = LogupTraceGenerator::new(log_size);
    let mut col_gen = logup_gen.new_col();
    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let p_mem: PackedQM31 = memory_lookup_elements.combine(
            &lookup_data
                .memory_lookups
                .each_ref()
                .map(|l| l.data[vec_row]),
        );
        let p_rc2: PackedQM31 = range2_lookup_elements
            .combine(&[lookup_data.memory_lookups[LAST_VALUE_OFFSET].data[vec_row]]);
        col_gen.write_frac(vec_row, p_mem + p_rc2, p_mem * p_rc2);
    }
    col_gen.finalize_col();
    logup_gen.finalize()
}

#[cfg(test)]
mod tests {
    use std::array;
    use std::simd::Simd;

    use rand::Rng;
    use stwo_prover::constraint_framework::constant_columns::gen_is_first;
    use stwo_prover::constraint_framework::FrameworkComponent;
    use stwo_prover::core::backend::simd::m31::N_LANES;
    use stwo_prover::core::channel::{Blake2sChannel, Channel};
    use stwo_prover::core::fields::IntoSlice;
    use stwo_prover::core::pcs::TreeVec;
    use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;

    use super::*;
    use crate::components::memory::{MemoryLookupElements, N_BITS_PER_FELT};
    use crate::components::range_check_builtin::component::RangeCheck128BuiltinComponent;
    use crate::felt::split_f252;

    #[test]
    fn test_generate_trace() {
        use super::*;

        let mut rng = rand::thread_rng();
        let log_size = 8;
        let address_initial_offset = 1000;
        let inputs = (0..1 << (log_size - LOG_N_LANES))
            .map(|i| RangeCheckInput {
                address: PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32 + address_initial_offset)
                })),
                values: array::from_fn(|_| Simd::from_array(rng.gen())),
            })
            .collect_vec();

        let (trace, lookup_data) =
            generate_trace(log_size, M31::from(address_initial_offset), &inputs);

        assert_eq!(trace.len(), N_ADDRESS_FELTS + N_VALUES_FELTS);
        assert_eq!(
            trace[0].values.clone().into_cpu_vec(),
            (0..1 << log_size).map(M31::from).collect_vec()
        );

        // Assert that the trace values are correct.
        #[allow(clippy::needless_range_loop)]
        for row_offset in 0..1 << (log_size - LOG_N_LANES) {
            let input = inputs[row_offset].values;

            let mut inputs_u128 = [0_u128; 16];
            for (index, simd) in input.iter().enumerate() {
                for (i, val) in simd.to_array().into_iter().enumerate() {
                    let val_u128 = val as u128;
                    inputs_u128[i] += val_u128 << (32 * index);
                }
            }

            let mask = ((1 << N_BITS_PER_FELT) - 1) as u128;
            for col in trace.iter().skip(1) {
                for j in 0..N_LANES {
                    let val = col.values.at((row_offset << LOG_N_LANES) + j);
                    assert_eq!(val.0, (inputs_u128[j] & mask) as u32);
                    inputs_u128[j] >>= N_BITS_PER_FELT;
                }
            }
        }

        // Assert that the high values are in range [0, 4).
        let high_values_col = &lookup_data.memory_lookups[LAST_VALUE_OFFSET];
        for packed_row in high_values_col.data.iter() {
            assert!(packed_row.to_array().iter().all(|&x| x.0 < 4));
        }
        // Assert memory addresses lookup are sequential, offseted by `address_initial_offset`.
        assert_eq!(
            lookup_data.memory_lookups[0].data.len() * N_LANES,
            1 << log_size
        );
        for (i, addresses) in lookup_data.memory_lookups[0].data.iter().enumerate() {
            assert_eq!(
                addresses.to_array(),
                array::from_fn(|j| {
                    M31::from_u32_unchecked((i * N_LANES + j) as u32 + address_initial_offset)
                })
            );
        }
    }

    #[test]
    fn test_generate_interaction_trace() {
        let mut rng = rand::thread_rng();
        let log_size = 8;
        let address_initial_offset = 10032;
        let inputs = (0..32)
            .map(|i| RangeCheckInput {
                address: PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32 + address_initial_offset)
                })),
                values: array::from_fn(|_| Simd::from_array(rng.gen())),
            })
            .collect_vec();

        let (trace, lookup_data) =
            generate_trace(log_size, M31::from(address_initial_offset), &inputs);

        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let memory_lookup_elements = MemoryLookupElements::draw(channel);
        let range2_lookup_elements = RangeCheckElements::draw(channel);

        let (interaction_trace, claimed_sum) = gen_interaction_trace(
            log_size,
            lookup_data,
            &memory_lookup_elements,
            &range2_lookup_elements,
        );

        let trace = TreeVec::new(vec![trace, interaction_trace, vec![gen_is_first(log_size)]]);
        let trace_polys = trace.map_cols(|c| c.interpolate());

        let component = RangeCheck128BuiltinComponent {
            log_size,
            initial_memory_address: M31::from(address_initial_offset),
            memory_lookup_elements,
            range2_lookup_elements,
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

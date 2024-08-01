use std::simd::{u32x16, Simd};

use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use super::component::{N_ADDRESS_FELTS, N_BITS_PER_FELT, N_VALUES_FELTS};

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
        let split_values = split_u128(row_input.values);
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

// Given 16 128-bit values, stored as 4 32-bit values, split them into 9-bit values elements.
fn split_u128(x: [u32x16; 4]) -> [PackedM31; N_VALUES_FELTS] {
    const MASK: Simd<u32, N_LANES> = u32x16::from_array([(1 << N_BITS_PER_FELT) - 1; N_LANES]);

    let mut res = [Simd::splat(0); N_VALUES_FELTS];
    let mut n_bits_in_word = 32;
    let mut word_i = 0;
    let mut word = x[word_i];
    for e in res.iter_mut() {
        // If current word has more bits than needed, chop it.
        if n_bits_in_word > N_BITS_PER_FELT {
            *e = word & MASK;
            word >>= N_BITS_PER_FELT as u32;
            n_bits_in_word -= N_BITS_PER_FELT;
            continue;
        }

        *e = word;
        // Fetch next word.
        word_i += 1;
        word = x.get(word_i).copied().unwrap_or_default();

        // If we need more bits to fill, take from next word.
        if n_bits_in_word < N_BITS_PER_FELT {
            *e |= (word << n_bits_in_word as u32) & MASK;
            word >>= (N_BITS_PER_FELT - n_bits_in_word) as u32;
        }

        n_bits_in_word += 32 - N_BITS_PER_FELT;
    }

    res.map(|x| PackedM31::from(x.to_array().map(M31::from)))
}

#[cfg(test)]
mod tests {
    use std::array;

    use rand::Rng;

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
        let high_values_col = &lookup_data.memory_lookups[N_ADDRESS_FELTS + N_VALUES_FELTS - 1];
        for packed_row in high_values_col.data.iter() {
            assert!(packed_row.to_array().iter().all(|&x| x.0 < 4));
        }

        // Assert memory addresses are sequential, offseted by `address_initial_offset`.
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
}

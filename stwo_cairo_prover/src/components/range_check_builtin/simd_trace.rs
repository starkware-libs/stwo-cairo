use std::simd::{u32x16, Simd};

use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
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
    let mut curr_bit = 0;
    let mut curr_x = 0;
    let full_mask: u32x16 = u32x16::splat((1 << N_BITS_PER_FELT) - 1);
    let mut res = [PackedM31::zero(); N_VALUES_FELTS];
    for e in res.iter_mut().take(N_VALUES_FELTS) {
        // Check if the element bits overlap two 32-bit values.
        match curr_bit + N_BITS_PER_FELT <= 32 {
            true => {
                let val = (x.get(curr_x).unwrap() >> curr_bit as u32) & full_mask;
                *e = unsafe { PackedM31::from_simd_unchecked(val) };
            }
            false => {
                let val_l = x.get(curr_x).unwrap() >> curr_bit as u32;
                let mask_h = u32x16::splat(1 << (N_BITS_PER_FELT + curr_bit - 32));
                // In the case of the last element, we don't have high bits.
                let val_h = x.get(curr_x + 1).unwrap_or(&Simd::splat(0)) & mask_h;
                *e = unsafe {
                    PackedM31::from_simd_unchecked(val_h << (32 - curr_bit as u32) | val_l)
                };
                curr_x += 1;
            }
        }
        curr_bit = (curr_bit + N_BITS_PER_FELT) % 32;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::array;

    use rand::Rng;
    use stwo_prover::core::backend::simd::m31::N_LANES;

    #[test]
    fn test_generate_trace() {
        use super::*;

        let log_size = 8;
        let address_initial_offset = 1000;
        let mut rng = rand::thread_rng();
        let inputs = (0..32)
            .map(|i| RangeCheckInput {
                address: PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32 + address_initial_offset)
                })),
                values: array::from_fn(|_| Simd::splat(rng.gen())),
            })
            .collect_vec();

        let (trace, lookup_data) =
            generate_trace(log_size, M31::from(address_initial_offset), &inputs);

        assert_eq!(trace.len(), N_ADDRESS_FELTS + N_VALUES_FELTS);
        assert_eq!(
            trace[0].values.clone().into_cpu_vec(),
            (0..1 << log_size).map(M31::from).collect_vec()
        );

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

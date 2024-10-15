use std::simd::Simd;

use itertools::Itertools;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::fields::m31::{M31, MODULUS_BITS};

const SIMD_ENUMERATION_IV: Simd<u32, N_LANES> =
    Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
const SIMD_N_LANES: Simd<u32, N_LANES> = Simd::from_array([N_LANES as u32; N_LANES]);

/// Computes the i'th vector of a [N_LANES]-packed enumeration.
pub fn simd_by_index(packed_idx: u32) -> Simd<u32, N_LANES> {
    SIMD_ENUMERATION_IV + Simd::splat(packed_idx) * SIMD_N_LANES
}

/// Computes exclusive suffix sum.
pub fn exclusive_suffix_sum<const N: usize>(values: [u32; N]) -> [u32; N] {
    let mut res = [0; N];
    for (i, log_range) in values[1..].iter().enumerate().rev() {
        res[i] = log_range + res[i + 1];
    }
    res
}

/// Splits a 32-bit integer into parts based on an array of split sizes, each part is represented
/// as a Mersenne31 field element where the extracted bits are placed in the least significant
/// positions.
///
/// # Arguments
///
/// * `value` - A 32-bit integer from which the bits will be split.
/// * `splits` - split sizes (in number of bits), from MSB to LSB.
///
/// # Returns
///
/// A `[M31; N]` where each element corresponds to a chunk of bits extracted from `value`,
///
/// NOTE: not efficient.
pub fn sparse_m31_representation<const N: usize>(splits: [u32; N], value: u32) -> [M31; N] {
    let n_bits = splits.iter().sum::<u32>();
    assert!(n_bits < MODULUS_BITS);
    assert!(
        value < 1 << n_bits,
        "Value {} is too large for {} bits",
        value,
        n_bits
    );

    let suffix_sum = exclusive_suffix_sum(splits);
    let masks = masks(splits);
    std::array::from_fn(|i| M31((value & masks[i]) >> suffix_sum[i]))
}

// Generates masks for each split size.
// EXAMPLE: [3, 4, 2] -> [0b111000000, 0b000111100, 0b000000011]
fn masks<const N: usize>(split: [u32; N]) -> [u32; N] {
    let suffix_sum = exclusive_suffix_sum(split);
    std::array::from_fn(|i| ((1 << split[i]) - 1) << suffix_sum[i])
}

/// The inverse of `sparse_m31_representation`.
pub fn dense_m31_representation<const N: usize>(trailing_sum: [u32; N], values: [M31; N]) -> u32 {
    trailing_sum
        .iter()
        .zip_eq(values)
        .fold(0, |dense, (&trailing, val)| dense + (val.0 << trailing))
}

pub fn packed_sparse_representation<const N: usize>(
    suffix_sum: [u32; N],
    masks: [Simd<u32, N_LANES>; N],
    dense_idx: u32,
) -> [PackedM31; N] {
    let simd: [Simd<u32, N_LANES>; N] =
        std::array::from_fn(|i| (simd_by_index(dense_idx) & masks[i]) >> suffix_sum[i]);
    std::array::from_fn(|i| unsafe { PackedM31::from_simd_unchecked(simd[i]) })
}

pub fn simd_masks<const N: usize>(split: [u32; N]) -> [Simd<u32, N_LANES>; N] {
    let suffix_sum = exclusive_suffix_sum(split);
    std::array::from_fn(|i| Simd::splat((1 << split[i]) - 1) << suffix_sum[i])
}

/// Generates the map from 0..2^(split.sum()) to the corresponding sparse representation.
pub fn generate_sparse_enumeration<const N: usize>(split: [u32; N]) -> [Vec<PackedM31>; N] {
    let n_bits = split.iter().sum::<u32>();
    assert!(n_bits < MODULUS_BITS);

    let masks = simd_masks(split);
    let suffix_sum = exclusive_suffix_sum(split);
    let mut res = std::array::from_fn(|_| vec![]);

    // Enumerate over 0..2^bits in packed chunks, computing the sparse representation of `i`.
    for vec_row in 0..1 << (n_bits - LOG_N_LANES) {
        let sparse = packed_sparse_representation(suffix_sum, masks, vec_row);
        for i in 0..N {
            res[i].push(sparse[i]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::fields::m31::M31;

    use super::{exclusive_suffix_sum, generate_sparse_enumeration};
    use crate::components::utils::{dense_m31_representation, sparse_m31_representation};
    #[test]
    fn test_trailing_sum() {
        let log_ranges = [3, 4, 2];
        let expected_trailing_sum = [6, 2, 0];

        let result = exclusive_suffix_sum(log_ranges);

        assert_eq!(expected_trailing_sum, result)
    }

    #[test]
    fn test_dense_representation() {
        let log_ranges = [8, 4, 3];
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10 {
            let rand = [
                rng.gen::<u32>() % (1 << 8),
                rng.gen::<u32>() % (1 << 4),
                rng.gen::<u32>() % (1 << 3),
            ];
            let trailing_sum = exclusive_suffix_sum(log_ranges);
            assert_eq!(
                dense_m31_representation(trailing_sum, [M31(rand[0]), M31(rand[1]), M31(rand[2])]),
                rand[0] << 7 | rand[1] << 3 | rand[2]
            )
        }
    }

    #[test]
    fn test_sparse_representation() {
        let log_ranges = [8, 4, 3];
        let mut rng = SmallRng::seed_from_u64(1);
        for _ in 0..10 {
            let rand = [
                rng.gen::<u32>() % (1 << 8),
                rng.gen::<u32>() % (1 << 4),
                rng.gen::<u32>() % (1 << 3),
            ];
            let dense = rand[0] << 7 | rand[1] << 3 | rand[2];
            assert_eq!(
                sparse_m31_representation(log_ranges, dense),
                [M31(rand[0]), M31(rand[1]), M31(rand[2])]
            );
        }
    }

    #[test]
    fn test_packed_sparse_enumerate() {
        let log_ranges = [5, 3, 3];
        let log_size = log_ranges.iter().sum::<u32>();
        let trailing_bits = exclusive_suffix_sum(log_ranges);

        let mut result = generate_sparse_enumeration(log_ranges).into_iter();
        let result: [Vec<M31>; 3] = std::array::from_fn(|_| {
            BaseColumn {
                data: result.next().unwrap(),
                length: 1 << log_size,
            }
            .into_cpu_vec()
        });

        for i in 0..1 << log_size {
            let vals = [result[0][i], result[1][i], result[2][i]];
            assert_eq!(dense_m31_representation(trailing_bits, vals), i as u32);
        }
    }
}

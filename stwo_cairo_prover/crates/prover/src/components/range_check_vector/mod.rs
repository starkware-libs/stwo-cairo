use std::simd::Simd;

use stwo_prover::constraint_framework::logup::LookupElements;
pub mod component;
pub mod component_prover;

// TODO(Ohad): figure out n_alpha_powers.
pub type RangeCheckLookupElements = LookupElements<3>;

use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::fields::m31::MODULUS_BITS;

const SIMD_ENUMERATION_0: Simd<u32, N_LANES> =
    Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

/// Computes exclusive suffix sum.
pub fn exclusive_suffix_sum<const N: usize>(values: [u32; N]) -> [u32; N] {
    let mut res = [0; N];
    for (i, value) in values[1..].iter().enumerate().rev() {
        res[i] = value + res[i + 1];
    }
    res
}

/// Partitions a number into 'N' bit segments according to the given split sizes.
/// EXAMPLE: [3, 4, 2], 0b110101010 -> [0b110, 0b1010, 0b10]
///
/// Each segment is returned as a `M31` Field element with the segmant value in the least
/// significant bits.
///
/// # Arguments
pub fn partition_into_bit_segments<const N: usize>(
    suffix_sum: [u32; N],
    masks: [Simd<u32, N_LANES>; N],
    num: u32,
) -> [PackedM31; N] {
    let simd: [Simd<u32, N_LANES>; N] = std::array::from_fn(|i| {
        ((SIMD_ENUMERATION_0 + Simd::splat(num << LOG_N_LANES)) & masks[i]) >> suffix_sum[i]
    });
    std::array::from_fn(|i| unsafe { PackedM31::from_simd_unchecked(simd[i]) })
}

// Generates masks for each split size.
// EXAMPLE: [3, 4, 2] -> [0b111000000, 0b000111100, 0b000000011]
fn simd_masks<const N: usize>(split: [u32; N]) -> [Simd<u32, N_LANES>; N] {
    let suffix_sum = exclusive_suffix_sum(split);
    std::array::from_fn(|i| Simd::splat((1 << split[i]) - 1) << suffix_sum[i])
}

/// Generates the map from 0..2^(split.sum()) to the corresponding value's partition segments.
pub fn generate_partitioned_enumeration<const N: usize>(split: [u32; N]) -> [Vec<PackedM31>; N] {
    let n_bits = split.iter().sum::<u32>();
    assert!(n_bits < MODULUS_BITS);

    let masks = simd_masks(split);
    let suffix_sum = exclusive_suffix_sum(split);
    let mut res = std::array::from_fn(|_| vec![]);

    for vec_row in 0..1 << (n_bits - LOG_N_LANES) {
        let segments = partition_into_bit_segments(suffix_sum, masks, vec_row);
        for i in 0..N {
            res[i].push(segments[i]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::fields::m31::{M31, MODULUS_BITS};

    use super::{exclusive_suffix_sum, generate_partitioned_enumeration};

    pub fn split_bits<const N: usize>(split_sizes: [u32; N], value: u32) -> [M31; N] {
        let n_bits = split_sizes.iter().sum::<u32>();
        assert!(n_bits < MODULUS_BITS);
        assert!(value < 1 << n_bits,);

        let suffix_sum = exclusive_suffix_sum(split_sizes);
        let masks: [_; N] = std::array::from_fn(|i| ((1 << split_sizes[i]) - 1) << suffix_sum[i]);
        std::array::from_fn(|i| M31((value & masks[i]) >> suffix_sum[i]))
    }

    #[test]
    fn test_suffix_sum() {
        let log_ranges = [3, 4, 2];
        let expected_suffix_sum = [6, 2, 0];

        let result = exclusive_suffix_sum(log_ranges);

        assert_eq!(expected_suffix_sum, result)
    }

    #[test]
    fn test_partition_to_bit_segments() {
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
                split_bits(log_ranges, dense),
                [M31(rand[0]), M31(rand[1]), M31(rand[2])]
            );
        }
    }

    #[test]
    fn test_packed_partition_enumerate() {
        let log_ranges = [5, 3, 3];
        let log_size = log_ranges.iter().sum::<u32>();
        let mut expected = [vec![], vec![], vec![]];
        for i in 0..1 << 5 {
            for j in 0..1 << 3 {
                for k in 0..1 << 3 {
                    expected[0].push(M31(i));
                    expected[1].push(M31(j));
                    expected[2].push(M31(k));
                }
            }
        }

        let mut result = generate_partitioned_enumeration(log_ranges).into_iter();
        let result: [Vec<M31>; 3] = std::array::from_fn(|_| {
            BaseColumn {
                data: result.next().unwrap(),
                length: 1 << log_size,
            }
            .into_cpu_vec()
        });

        assert_eq!(result, expected)
    }
}

use std::cmp::max;
use std::simd::Simd;
use std::usize;

use itertools::Itertools;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::M31;

/// Computes cumulative sum right -> left.
///
/// # Example
///  Given [1, 2, 3, 4] returns [9, 7, 4, 0].
///
/// NOTE: The first (from left) element is excluded.
pub fn excluded_trailing_sum<const N: usize>(values: [u32; N]) -> [u32; N] {
    let mut res = [0; N];
    for (i, log_range) in values.iter().skip(1).enumerate().rev() {
        res[i] = log_range + res[i + 1];
    }
    res
}

/// Computes cumulative sum right -> left.
///
/// # Example
///  Given [1, 2, 3, 4] returns [0, 1, 3, 6].
///
/// NOTE: The last (from left) element is excluded.
pub fn excluded_leading_sum<const N: usize>(values: [u32; N]) -> [u32; N] {
    let mut res = [0; N];
    for (i, log_range) in values[..N - 1].iter().enumerate() {
        res[i + 1] = log_range + res[i];
    }
    res
}

/// Enumerates over 'bits' values, each with 1 << 'trailing' repetitions, and repeats the whole
/// process 1 << 'log_reps' times.
///
/// # Example
///
/// Given log_reps = 1, bits = 2, trailing = 1, returns:
/// [0,0,1,1,2,2,3,3,0,0,1,1,2,2,3,3]
pub fn generate_expanded_enumeration(log_reps: u32, bits: u32, trailing: u32) -> Vec<PackedM31> {
    let log_size = bits + trailing + log_reps;
    let mut res = unsafe { Col::<SimdBackend, M31>::uninitialized(1 << log_size).data };
    let (iv, step) = compute_iv_and_step(bits, trailing);
    let chunk_size = 1 << (max(bits + trailing, LOG_N_LANES) - LOG_N_LANES);
    res.chunks_mut(chunk_size)
        .for_each(|chunk| write_rep(chunk, iv, step, trailing));
    res
}
fn write_rep(dst: &mut [PackedM31], iv: PackedM31, step: PackedM31, trailing: u32) {
    let log_step_size = max(trailing as i64 - LOG_N_LANES as i64, 0);
    let mut current = iv;
    for step_chunk in dst.chunks_mut(1 << log_step_size) {
        step_chunk.iter_mut().for_each(|v| {
            *v = current;
        });
        current += step;
    }
}

pub fn compute_iv_and_step(bits: u32, trailing_bits: u32) -> (PackedM31, PackedM31) {
    let (simd_iv, simd_step) = if trailing_bits >= LOG_N_LANES {
        (
            Simd::<u32, N_LANES>::splat(0),
            Simd::<u32, N_LANES>::splat(1),
        )
    } else {
        let log_m = LOG_N_LANES - trailing_bits;
        let iv = Simd::<u32, N_LANES>::from_array(
            (0..(1 << log_m))
                .map(|x| vec![x; 1 << (LOG_N_LANES - log_m)])
                .concat()
                .try_into()
                .unwrap(),
        );
        let step = Simd::<u32, N_LANES>::splat(1 << log_m);
        let mask = Simd::<u32, N_LANES>::splat((1 << bits) - 1);
        (iv & mask, step & mask)
    };
    (unsafe { PackedM31::from_simd_unchecked(simd_iv) }, unsafe {
        PackedM31::from_simd_unchecked(simd_step)
    })
}

pub fn dense_representation<const N: usize>(trailing_sum: [u32; N], values: [M31; N]) -> u32 {
    trailing_sum
        .iter()
        .zip_eq(values)
        .fold(0, |dense, (&trailing, val)| dense + (val.0 << trailing))
}

pub fn sparse_representation<const N: usize>(trailing_sum: [u32; N], mut dense: u32) -> [M31; N] {
    trailing_sum
        .iter()
        .map(|&trailing_bits| {
            let value = (!((1 << trailing_bits) - 1) & dense) >> trailing_bits;
            dense &= (1 << trailing_bits) - 1;
            value.into()
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::fields::m31::M31;

    use super::{excluded_trailing_sum, generate_expanded_enumeration};
    use crate::components::utils::{
        dense_representation, excluded_leading_sum, sparse_representation,
    };
    #[test]
    fn test_trailing_sum() {
        let log_ranges = [3, 4, 2];
        let expected_trailing_sum = [6, 2, 0];

        let result = excluded_trailing_sum(log_ranges);

        assert_eq!(expected_trailing_sum, result)
    }

    #[test]
    fn test_leading_sum() {
        let log_ranges = [3, 4, 2];
        let expected_leading_sum = [0, 3, 7];

        let result = excluded_leading_sum(log_ranges);

        assert_eq!(expected_leading_sum, result)
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
            let trailing_sum = excluded_trailing_sum(log_ranges);
            assert_eq!(
                dense_representation(trailing_sum, [M31(rand[0]), M31(rand[1]), M31(rand[2])]),
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
            let trailing_sum = excluded_trailing_sum(log_ranges);
            assert_eq!(
                sparse_representation(trailing_sum, dense),
                [M31(rand[0]), M31(rand[1]), M31(rand[2])]
            );
        }
    }

    #[test]
    fn test_enumerate_expand() {
        let log_ranges = [5, 3, 3];
        let log_size = 5 + 3 + 3;
        let trailing_bits = excluded_trailing_sum(log_ranges);
        let leading_bits = excluded_leading_sum(log_ranges);

        let result: [Vec<M31>; 3] = std::array::from_fn(|i| {
            let (leading, n_bits, trailing) = (leading_bits[i], log_ranges[i], trailing_bits[i]);
            BaseColumn {
                data: generate_expanded_enumeration(leading, n_bits, trailing),
                length: 1 << log_size,
            }
            .into_cpu_vec()
        });

        for i in 0..1 << log_size {
            let vals = [result[0][i], result[1][i], result[2][i]];
            assert_eq!(dense_representation(trailing_bits, vals), i as u32);
        }
    }
}

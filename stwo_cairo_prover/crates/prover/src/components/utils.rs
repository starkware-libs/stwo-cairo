use std::cmp::max;
use std::mem::transmute;
use std::simd::u32x16;
use std::sync::atomic::{AtomicU32, Ordering};

use num_traits::{One, Zero};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Pack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::fields::m31::{M31, MODULUS_BITS};

// When padding is needed, the inputs must be arranged in the order defined by the neighbor
// function. This order allows using the partial sum mechanism to sum only the first n_call inputs.
// After getting the `SubComponentInputs` we apply the permutation again to ignore padded values at
// the tail of the vector.
// TODO(Ohad): generalize the padding logic, and move above doc to the relevant function.

pub fn pack_values<T: Pack>(values: &[T]) -> Vec<T::SimdType> {
    values
        .array_chunks::<N_LANES>()
        .map(|c| T::pack(*c))
        .collect()
}

/// A column of multiplicities for lookup arguments. Allows increasing the multiplicity at a given
/// index. This version uses atomic operations to increase the multiplicity, and is `Send`.
pub struct AtomicMultiplicityColumn {
    data: Vec<AtomicU32>,
}
impl AtomicMultiplicityColumn {
    /// Creates a new `AtomicMultiplicityColumn` with the given size. The elements are initialized
    /// to 0.
    pub fn new(size: usize) -> Self {
        Self {
            data: (0..size as u32).map(|_| AtomicU32::new(0)).collect(),
        }
    }

    pub fn increase_at(&self, address: u32) {
        self.data[address as usize].fetch_add(1, Ordering::Relaxed);
    }

    /// Returns the internal data as a Vec<PackedM31>. The last element of the vector is padded with
    /// zeros if needed. This function performs a copy on the inner data, If atomics are not
    /// necessary, use [`MultiplicityColumn`] instead.
    pub fn into_simd_vec(self) -> Vec<PackedM31> {
        // Safe because the data is aligned to the size of PackedM31 and the size of the data is a
        // multiple of N_LANES.
        BaseColumn::from_iter(
            self.data
                .into_iter()
                .map(|a| M31(a.load(Ordering::Relaxed))),
        )
        .data
    }
}

/// The enabler column is a column of length `padding_offset.next_power_of_two()` where
/// 1. The first `padding_offset` elements are set to 1;
/// 2. The rest are set to 0.
#[derive(Debug, Clone)]
pub struct Enabler {
    pub padding_offset: usize,
}
impl Enabler {
    pub const fn new(padding_offset: usize) -> Self {
        Self { padding_offset }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let row_offset = vec_row * N_LANES;
        if self.padding_offset <= row_offset {
            return PackedM31::zero();
        }
        if self.padding_offset >= row_offset + N_LANES {
            return PackedM31::one();
        }

        // The row is partially enabled.
        let mut res = [M31::zero(); N_LANES];
        for v in res.iter_mut().take(self.padding_offset - row_offset) {
            *v = M31::one();
        }
        PackedM31::from_array(res)
    }
}

/// Returns Vector of PackedM31 values representing an enumeration pattern.
///
/// The pattern repeats a sequence of numbers where:
/// - Each number takes 'bits' bits
/// - Each value is repeated 2^'trailing' times
/// - The whole sequence is repeated 2^'log_reps' times
///
/// # Arguments
/// * `log_reps` - Log2 of number of repetitions of the whole sequence
/// * `bits` - Number of bits per value
/// * `trailing` - Log2 of repetitions per value
///
/// # Example
///
/// With `log_reps`=1, `bits`=2, `trailing`=1, generates:
/// `[0,0,1,1,2,2,3,3,0,0,1,1,2,2,3,3]`
pub fn generate_expanded_enumeration(log_reps: u32, bits: u32, trailing: u32) -> Vec<PackedM31> {
    assert!(bits < MODULUS_BITS, "bits must be less than MODULUS_BITS");

    let log_size = bits + trailing + log_reps;
    let total_size = 1 << log_size;
    let chunk_size = 1 << (max(bits + trailing, LOG_N_LANES) - LOG_N_LANES);

    let mut result = vec![u32x16::splat(0); total_size >> LOG_N_LANES];

    // Generate the base pattern in the first chunk.
    let (first_chunk, remaining) = result.split_at_mut(chunk_size);
    write_base_pattern(first_chunk, bits, trailing);

    // Copy the pattern to remaining chunks.
    remaining
        .chunks_mut(chunk_size)
        .for_each(|chunk| chunk.copy_from_slice(first_chunk));

    unsafe { transmute(result) }
}

/// Generates the base pattern for a single chunk.
fn write_base_pattern(dst: &mut [u32x16], bits: u32, trailing: u32) {
    let log_step_size = max(trailing, LOG_N_LANES) - LOG_N_LANES;
    let (initial_value, step) = iv_and_step(bits, trailing);

    let mut current = initial_value;
    for step_chunk in dst.chunks_mut(1 << log_step_size) {
        step_chunk.iter_mut().for_each(|v| *v = current);
        current += step;
    }
}

fn iv_and_step(bits: u32, trailing_bits: u32) -> (u32x16, u32x16) {
    if trailing_bits >= LOG_N_LANES {
        return (u32x16::splat(0), u32x16::splat(1));
    }

    let pattern = match LOG_N_LANES - trailing_bits {
        1 => ([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1], 2),
        2 => ([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3], 4),
        3 => ([0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7], 8),
        4 => ([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], 16),
        _ => unreachable!("Invalid trailing_bits value"),
    };

    let mask = (1 << bits) - 1;
    (
        u32x16::from_array(pattern.0.map(|n| n & mask)),
        u32x16::splat(pattern.1),
    )
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use stwo_prover::core::backend::simd::m31::N_LANES;
    use stwo_prover::core::fields::m31::M31;

    use super::Enabler;

    #[test]
    fn test_atomic_multiplicities_column() {
        let size = N_LANES;
        let n_loops = 10;
        let col = super::AtomicMultiplicityColumn::new(size);
        let n_threads = 32;

        (0..n_threads).into_par_iter().for_each(|_| {
            (0..size * n_loops).for_each(|i| col.increase_at((i % size) as u32));
        });
        let result = col
            .into_simd_vec()
            .into_iter()
            .flat_map(|p| p.to_array().map(|v| v.0));

        for value in result {
            assert_eq!(value, n_threads * n_loops as u32);
        }
    }

    #[test]
    fn test_multiplicities_column_into_simd() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let expected_length = 6;
        let cpu_length = expected_length * N_LANES - 2;

        let multiplicity_column = super::AtomicMultiplicityColumn::new(cpu_length);
        (0..10 * N_LANES).for_each(|_| {
            let addr = rng.gen_range(0..cpu_length as u32);
            multiplicity_column.increase_at(addr);
        });
        let actual_length = multiplicity_column.into_simd_vec().len();

        assert_eq!(actual_length, expected_length);
    }

    #[test]
    fn test_enabler_packed_at_single_row() {
        let n_calls = 1;

        let enabler_column = Enabler::new(n_calls);

        assert_eq!(
            enabler_column.packed_at(0).to_array(),
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(M31::from)
        );
    }

    #[test]
    fn test_enabler_packed_row_n_lanes() {
        let enabler_column = Enabler::new(N_LANES);

        assert_eq!(
            enabler_column.packed_at(0).to_array(),
            [1; N_LANES].map(M31::from)
        );
        assert_eq!(
            enabler_column.packed_at(1).to_array(),
            [0; N_LANES].map(M31::from)
        );
    }

    #[test]
    fn test_enabler_packed_at() {
        let n_calls = 30;

        let enabler_column = Enabler::new(n_calls);

        assert_eq!(
            enabler_column.packed_at(0).to_array(),
            [1; N_LANES].map(M31::from)
        );
        assert_eq!(
            enabler_column.packed_at(1).to_array(),
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0].map(M31::from)
        );
        assert_eq!(
            enabler_column.packed_at(2).to_array(),
            [0; N_LANES].map(M31::from)
        );
    }

    #[test]
    fn test_expanded_enumeration() {
        let mut expected = vec![];
        for _ in 0..1 << 5 {
            for j in 0..1 << 3 {
                for _ in 0..1 << 3 {
                    expected.push(j as u32);
                }
            }
        }

        let result = super::generate_expanded_enumeration(5, 3, 3)
            .into_iter()
            .flat_map(|p| p.to_array().map(|v| v.0))
            .collect_vec();

        assert_eq!(result, expected);
    }
}

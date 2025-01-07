use std::mem::transmute;
use std::simd::Simd;
use std::sync::atomic::{AtomicU32, Ordering};

use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Pack;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::fields::m31::M31;

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

// TODO(Gali): Move to stwo-air-utils.
/// A column of multiplicities for lookup arguments. Allows increasing the multiplicity at a given
/// index.
pub struct MultiplicityColumn {
    data: Vec<Simd<u32, N_LANES>>,
}
impl MultiplicityColumn {
    /// Creates a new `MultiplicityColumn` with the given size. The elements are initialized to 0.
    pub fn new(size: usize) -> Self {
        let vec_size = size.div_ceil(N_LANES);
        Self {
            data: vec![unsafe { std::mem::zeroed() }; vec_size],
        }
    }

    pub fn increase_at(&mut self, address: u32) {
        self.data[address as usize / N_LANES][address as usize % N_LANES] += 1;
    }

    /// Returns the internal data as a Vec<PackedM31>. The last element of the vector is padded with
    /// zeros if needed.
    pub fn into_simd_vec(self) -> Vec<PackedM31> {
        // Safe because the data is aligned to the size of PackedM31 and the size of the data is a
        // multiple of N_LANES.
        unsafe { transmute(self.data) }
    }
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

/// Generates a padding column for the given padding offset.
/// The padding column is a MultiplicityColumn of length next_power_of_two(padding_offset) where
/// 1. The first `padding_offset` elements set to 1;
/// 2. Otherwise set to 0.
pub fn gen_padding_column(padding_offset: usize) -> MultiplicityColumn {
    let log_size = padding_offset.next_power_of_two().ilog2();
    let mut padding_column = MultiplicityColumn::new(1 << log_size);
    (0..padding_offset).for_each(|i| padding_column.increase_at(i as u32));
    padding_column
}

#[cfg(test)]
mod tests {
    use num_traits::{One, Zero};
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
    use stwo_prover::core::fields::m31::M31;

    #[test]
    fn test_multiplicities_column() {
        let mut rng = SmallRng::seed_from_u64(0u64);
        let mut multiplicity_column = super::MultiplicityColumn::new(6 * N_LANES - 2);
        let mut expected = vec![M31::zero(); 6 * N_LANES];

        (0..10 * N_LANES).for_each(|_| {
            let addr = rng.gen_range(0..N_LANES * 6);
            multiplicity_column.increase_at(addr as u32);
            expected[addr] += M31::one();
        });
        let res = multiplicity_column.into_simd_vec();

        assert!(res.len() == 6);
        for (res_chunk, expected_chunk) in res.iter().zip(expected.chunks(N_LANES)) {
            assert!(res_chunk.to_array() == expected_chunk);
        }
    }

    #[test]
    fn test_gen_padding_column() {
        let n_calls = 30;

        let enabler_column = super::gen_padding_column(n_calls);

        for (i, val) in enabler_column
            .into_simd_vec()
            .into_iter()
            .flat_map(PackedM31::to_array)
            .enumerate()
        {
            if i < n_calls {
                assert_eq!(val, M31::one());
            } else {
                assert_eq!(val, M31::zero());
            }
        }
    }
}

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use cairo_air::air::CairoClaim;
use cairo_air::PreProcessedTraceVariant;
use itertools::Itertools;
use num_traits::{One, Zero};
use stwo::core::channel::MerkleChannel;
use stwo::core::fields::m31::M31;
use stwo::core::pcs::{TreeSubspan, TreeVec};
use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo::core::vcs::MerkleHasher;
use stwo::prover::backend::simd::column::BaseColumn;
use stwo::prover::backend::simd::conversion::Pack;
use stwo::prover::backend::simd::m31::{PackedM31, N_LANES};
use stwo::prover::backend::{Backend, BackendForChannel};
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
use stwo_cairo_common::prover_types::simd::LOG_N_LANES;
use stwo_constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_constraint_framework::PREPROCESSED_TRACE_IDX;

use crate::witness::preprocessed_trace::generate_preprocessed_commitment_root;

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

/// Extenders of a commitment-tree with evaluations.
pub trait TreeBuilder<B: Backend> {
    fn extend_evals(
        &mut self,
        columns: impl IntoIterator<Item = CircleEvaluation<B, M31, BitReversedOrder>>,
    ) -> TreeSubspan;
}

impl<B: BackendForChannel<MC>, MC: MerkleChannel> TreeBuilder<B>
    for stwo::prover::TreeBuilder<'_, '_, B, MC>
{
    fn extend_evals(
        &mut self,
        columns: impl IntoIterator<Item = CircleEvaluation<B, M31, BitReversedOrder>>,
    ) -> TreeSubspan {
        self.extend_evals(columns)
    }
}

fn tree_trace_cells(tree_log_sizes: TreeVec<Vec<u32>>) -> Vec<u64> {
    tree_log_sizes
        .iter()
        .map(|tree| {
            tree.iter()
                .map(|col_log_size| 1 << col_log_size)
                .sum::<u64>()
        })
        .collect()
}

/// Returns the number of cells in each trace interaction of the witness.
/// Preprocess trace is determined by the `pp_trace` parameter (and not by the claim).
pub fn witness_trace_cells(claim: &CairoClaim, pp_trace: &PreProcessedTrace) -> Vec<u64> {
    let mut log_sizes = claim.log_sizes();
    log_sizes[PREPROCESSED_TRACE_IDX] = pp_trace.log_sizes();

    tree_trace_cells(log_sizes)
}

fn get_preprocessed_roots<MC: MerkleChannel>(
    max_log_blowup_factor: u32,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Vec<<MC::H as MerkleHasher>::Hash>
where
    stwo::prover::backend::simd::SimdBackend: BackendForChannel<MC>,
{
    (1..=max_log_blowup_factor)
        .map(|i| generate_preprocessed_commitment_root::<MC>(i, preprocessed_trace))
        .collect_vec()
}

/// Exports the preprocessed roots for both Blake2s and Poseidon252 channels.
/// Note: This function is very slow and is intended for generating the preprocessed roots when
/// needed.
pub fn export_preprocessed_roots() {
    let max_log_blowup_factor = 2;

    // Blake2s roots.
    let blake_roots = get_preprocessed_roots::<Blake2sMerkleChannel>(
        max_log_blowup_factor,
        PreProcessedTraceVariant::Canonical,
    );
    blake_roots.iter().enumerate().for_each(|(i, root)| {
        let root_bytes = root.0;
        let u32s_hex = root_bytes
            .array_chunks::<4>()
            .map(|&bytes| format!("{:#010x}", u32::from_le_bytes(bytes)))
            .collect_vec()
            .join(", ");

        println!("log_blowup_factor: {}, blake root: [{}]", i + 1, u32s_hex);
    });

    // Poseidon252 roots.
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    {
        use stwo::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
        // Poseidon252 roots.
        get_preprocessed_roots::<Poseidon252MerkleChannel>(
            max_log_blowup_factor,
            PreProcessedTraceVariant::CanonicalWithoutPedersen,
        )
        .into_iter()
        .enumerate()
        .for_each(|(i, root)| {
            println!(
                "log_blowup_factor: {}, poseidon root: [{:#010x}]",
                i + 1,
                root
            );
        });
    }
}

/// Create the input_to_row map used in const-size components.
///
/// `preprocessed_trace` - The preprocessed trace.
/// `column_ids` - PreProcessedColumnId for each input column of the component.
///
/// Returns a mapping from input tuple to its row number. Used to find
/// out which multiplicity value to update for a given input.
pub fn make_input_to_row<const N: usize>(
    preprocessed_trace: &PreProcessedTrace,
    column_ids: [PreProcessedColumnId; N],
) -> HashMap<[M31; N], usize> {
    let mut result: HashMap<[M31; N], usize> = HashMap::new();

    let columns = column_ids
        .iter()
        .map(|id| preprocessed_trace.get_column(id))
        .collect_vec();
    let log_size = columns[0].log_size();
    assert!(
        columns.iter().all(|c| c.log_size() == log_size),
        "input_to_row columns of different sizes"
    );

    for packed_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let packed_values = columns
            .iter()
            .map(|c| c.packed_at(packed_row).to_array())
            .collect_vec();
        for i in 0..N_LANES {
            let key: [M31; N] = packed_values
                .iter()
                .map(|pv| pv[i])
                .collect_vec()
                .try_into()
                .expect("Unexpected number of column values");
            result.insert(key, packed_row * N_LANES + i);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use stwo::core::fields::m31::M31;
    use stwo::core::pcs::TreeVec;
    use stwo::prover::backend::simd::m31::N_LANES;

    use super::Enabler;
    use crate::witness::utils::tree_trace_cells;

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
    fn test_tree_trace_cells() {
        let tree_sizes = TreeVec::new(vec![vec![1, 2], vec![3, 4], vec![5]]);

        let result = tree_trace_cells(tree_sizes);

        assert_eq!(result, vec![6, 24, 32]);
    }
}

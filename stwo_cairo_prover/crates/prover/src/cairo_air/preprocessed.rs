use std::simd::{u32x16, Simd};

use itertools::Itertools;
use stwo_cairo_common::prover_types::simd::LOG_N_LANES;
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col};
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::CommitmentTreeProver;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation, PolyOps};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::ops::MerkleHasher;

use super::LOG_MAX_ROWS;
use crate::components::range_check_vector::{
    generate_partitioned_enumeration, partition_into_bit_segments, SIMD_ENUMERATION_0,
};

// Size to initialize the preprocessed trace with for `PreprocessedColumn::BitwiseXor`.
const XOR_N_BITS: u32 = 9;

pub trait PreProcessedColumn {
    fn log_size(&self) -> u32;
    fn id(&self) -> PreProcessedColumnId;
    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>;
}

pub struct PreProcessedTrace {
    seq_columns: Vec<Seq>,
    bitwise_xor_columns: Vec<BitwiseXor>,
    range_check_columns: Vec<Box<dyn PreProcessedColumn>>,
}
impl PreProcessedTrace {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let seq_columns = (LOG_N_LANES..=LOG_MAX_ROWS).map(Seq::new).collect_vec();
        let bitwise_xor_columns = (0..3)
            .map(move |col_index| BitwiseXor::new(XOR_N_BITS, col_index))
            .collect_vec();
        let range_check_columns = gen_range_check_columns();

        Self {
            seq_columns,
            bitwise_xor_columns,
            range_check_columns,
        }
    }

    fn columns(&self) -> Vec<&dyn PreProcessedColumn> {
        let mut columns: Vec<&dyn PreProcessedColumn> = vec![];
        columns.extend(
            self.seq_columns
                .iter()
                .map(|c| c as &dyn PreProcessedColumn),
        );
        columns.extend(
            self.bitwise_xor_columns
                .iter()
                .map(|c| c as &dyn PreProcessedColumn),
        );
        columns.extend(self.range_check_columns.iter().map(|c| c.as_ref()));

        // Sort columns by descending log size.
        columns
            .into_iter()
            .sorted_by_key(|column| std::cmp::Reverse(column.log_size()))
            .collect_vec()
    }

    pub fn log_sizes(&self) -> Vec<u32> {
        self.columns().iter().map(|c| c.log_size()).collect()
    }

    pub fn gen_trace(&self) -> Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
        self.columns().iter().map(|c| c.gen_column_simd()).collect()
    }

    pub fn ids(&self) -> Vec<PreProcessedColumnId> {
        self.columns().iter().map(|c| c.id()).collect()
    }
}

fn gen_range_check_columns() -> Vec<Box<dyn PreProcessedColumn>> {
    // RangeCheck_6.
    let range_check_6 = RangeCheck::new([6], 0);
    // RangeCheck_11.
    let range_check_11 = RangeCheck::new([11], 0);
    // RangeCheck_12.
    let range_check_12 = RangeCheck::new([12], 0);
    // RangeCheck_18.
    let range_check_18 = RangeCheck::new([18], 0);
    // RangeCheck_19.
    let range_check_19 = RangeCheck::new([19], 0);
    // RangeCheck_3_6.
    let range_check_3_6_col_0 = RangeCheck::new([3, 6], 0);
    let range_check_3_6_col_1 = RangeCheck::new([3, 6], 1);
    // RangeCheck_4_3.
    let range_check_4_3_col_0 = RangeCheck::new([4, 3], 0);
    let range_check_4_3_col_1 = RangeCheck::new([4, 3], 1);
    // RangeCheck_9_9.
    let range_check_9_9_col_0 = RangeCheck::new([9, 9], 0);
    let range_check_9_9_col_1 = RangeCheck::new([9, 9], 1);
    // RangeCheck_7_2_5.
    let range_check_7_2_5_col_0 = RangeCheck::new([7, 2, 5], 0);
    let range_check_7_2_5_col_1 = RangeCheck::new([7, 2, 5], 1);
    let range_check_7_2_5_col_2 = RangeCheck::new([7, 2, 5], 2);
    // RangeCheck_3_6_6_3.
    let range_check_3_6_6_3_col_0 = RangeCheck::new([3, 6, 6, 3], 0);
    let range_check_3_6_6_3_col_1 = RangeCheck::new([3, 6, 6, 3], 1);
    let range_check_3_6_6_3_col_2 = RangeCheck::new([3, 6, 6, 3], 2);
    let range_check_3_6_6_3_col_3 = RangeCheck::new([3, 6, 6, 3], 3);

    vec![
        Box::new(range_check_6),
        Box::new(range_check_11),
        Box::new(range_check_12),
        Box::new(range_check_18),
        Box::new(range_check_19),
        Box::new(range_check_3_6_col_0),
        Box::new(range_check_3_6_col_1),
        Box::new(range_check_4_3_col_0),
        Box::new(range_check_4_3_col_1),
        Box::new(range_check_9_9_col_0),
        Box::new(range_check_9_9_col_1),
        Box::new(range_check_7_2_5_col_0),
        Box::new(range_check_7_2_5_col_1),
        Box::new(range_check_7_2_5_col_2),
        Box::new(range_check_3_6_6_3_col_0),
        Box::new(range_check_3_6_6_3_col_1),
        Box::new(range_check_3_6_6_3_col_2),
        Box::new(range_check_3_6_6_3_col_3),
    ]
}

/// A column with the numbers [0..(2^log_size)-1].
#[derive(Debug, Clone)]
pub struct Seq {
    pub log_size: u32,
}
impl Seq {
    pub const fn new(log_size: u32) -> Self {
        Self { log_size }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        PackedM31::broadcast(M31::from(vec_row * N_LANES))
            + unsafe { PackedM31::from_simd_unchecked(SIMD_ENUMERATION_0) }
    }
}
impl PreProcessedColumn for Seq {
    fn log_size(&self) -> u32 {
        self.log_size
    }
    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let col = Col::<SimdBackend, BaseField>::from_iter(
            (0..(1 << self.log_size)).map(BaseField::from),
        );
        CircleEvaluation::new(CanonicCoset::new(self.log_size).circle_domain(), col)
    }
    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("seq_{}", self.log_size).to_string(),
        }
    }
}

/// A table of a,b,c, where a,b,c are integers and a ^ b = c.
///
/// # Attributes
///
/// - `n_bits`: The number of bits in each integer.
/// - `col_index`: The column index in the preprocessed table.
#[derive(Debug)]
pub struct BitwiseXor {
    n_bits: u32,
    col_index: usize,
}
impl BitwiseXor {
    pub const fn new(n_bits: u32, col_index: usize) -> Self {
        assert!(col_index < 3, "col_index must be in range 0..=2");
        Self { n_bits, col_index }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let lhs = || -> u32x16 {
            (SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32)) >> self.n_bits
        };
        let rhs = || -> u32x16 {
            (SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32))
                & Simd::splat((1 << self.n_bits) - 1)
        };
        let simd = match self.col_index {
            0 => lhs(),
            1 => rhs(),
            2 => lhs() ^ rhs(),
            _ => unreachable!(),
        };
        unsafe { PackedM31::from_simd_unchecked(simd) }
    }
}
impl PreProcessedColumn for BitwiseXor {
    fn log_size(&self) -> u32 {
        2 * self.n_bits
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_simd(
                (0..(1 << (self.log_size() - LOG_N_LANES)))
                    .map(|i| self.packed_at(i))
                    .collect(),
            ),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: format!("bitwise_xor_{}_{}", self.n_bits, self.col_index),
        }
    }
}

pub struct RangeCheck<const N: usize> {
    ranges: [u32; N],
    column_idx: usize,
}
impl<const N: usize> RangeCheck<N> {
    pub fn new(ranges: [u32; N], column_idx: usize) -> Self {
        // TODO(Ohad): consider asserting height is lower than some bound.
        assert!(ranges.iter().all(|&r| r > 0));
        assert!(column_idx < N);
        Self { ranges, column_idx }
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let n = SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32);

        unsafe {
            PackedM31::from_simd_unchecked(
                partition_into_bit_segments(n, self.ranges)[self.column_idx],
            )
        }
    }
}
impl<const N: usize> PreProcessedColumn for RangeCheck<N> {
    fn log_size(&self) -> u32 {
        self.ranges.iter().sum()
    }

    fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        let partitions = generate_partitioned_enumeration(self.ranges);
        let column = partitions.into_iter().nth(self.column_idx).unwrap();
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_simd(column),
        )
    }

    fn id(&self) -> PreProcessedColumnId {
        let ranges = self.ranges.iter().join("_");
        PreProcessedColumnId {
            id: format!("range_check_{}_column_{}", ranges, self.column_idx).to_string(),
        }
    }
}

/// Generates the root of the preprocessed trace commitment tree for a given `log_blowup_factor`.
pub fn generate_preprocessed_commitment_root<MC: MerkleChannel>(
    log_blowup_factor: u32,
) -> <<MC as MerkleChannel>::H as MerkleHasher>::Hash
where
    SimdBackend: BackendForChannel<MC>,
{
    let preprocessed_trace = PreProcessedTrace::new();

    // Precompute twiddles for the commitment scheme.
    let max_log_size = preprocessed_trace.log_sizes().into_iter().max().unwrap();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(max_log_size + log_blowup_factor)
            .circle_domain()
            .half_coset,
    );

    // Generate the commitment tree.
    let polys = SimdBackend::interpolate_columns(preprocessed_trace.gen_trace(), &twiddles);
    let commitment_scheme = CommitmentTreeProver::<SimdBackend, MC>::new(
        polys,
        log_blowup_factor,
        &mut MC::C::default(),
        &twiddles,
    );

    commitment_scheme.commitment.root()
}

#[cfg(test)]
mod tests {
    use super::*;
    const LOG_SIZE: u32 = 8;
    use stwo_prover::core::backend::Column;
    use stwo_prover::core::vcs::blake2_hash::Blake2sHash;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    #[test]
    fn test_columns_are_in_decending_order() {
        let preprocessed_trace = PreProcessedTrace::new();

        let columns = preprocessed_trace.columns();

        assert!(columns
            .windows(2)
            .all(|w| w[0].log_size() >= w[1].log_size()));
    }

    #[test]
    fn test_gen_seq() {
        let seq = Seq::new(LOG_SIZE).gen_column_simd();
        for i in 0..(1 << LOG_SIZE) {
            assert_eq!(seq.at(i), BaseField::from_u32_unchecked(i as u32));
        }
    }

    #[test]
    fn test_packed_at_seq() {
        let seq = Seq::new(LOG_SIZE);
        let expected_seq: [_; 1 << LOG_SIZE] = std::array::from_fn(|i| M31::from(i as u32));
        let packed_seq = std::array::from_fn::<_, { (1 << LOG_SIZE) / N_LANES }, _>(|i| {
            seq.packed_at(i).to_array()
        })
        .concat();
        assert_eq!(packed_seq, expected_seq);
    }

    #[test]
    fn test_packed_at_bitwise_xor() {
        let bitwise_a = BitwiseXor::new(LOG_SIZE, 0);
        let bitwise_b = BitwiseXor::new(LOG_SIZE, 1);
        let bitwise_xor = BitwiseXor::new(LOG_SIZE, 2);
        let index: usize = 1000;
        let a = index / (1 << LOG_SIZE);
        let b = index % (1 << LOG_SIZE);
        let expected_xor = a ^ b;

        let res_a = bitwise_a.packed_at(index / N_LANES).to_array()[index % N_LANES];
        let res_b = bitwise_b.packed_at(index / N_LANES).to_array()[index % N_LANES];
        let res_xor = bitwise_xor.packed_at(index / N_LANES).to_array()[index % N_LANES];

        assert_eq!(res_a.0, a as u32);
        assert_eq!(res_b.0, b as u32);
        assert_eq!(res_xor.0, expected_xor as u32);
    }

    #[test]
    fn test_range_check_packed_at() {
        let ranges = [1, 2, 3, 4];
        let range_check = RangeCheck::new(ranges, 2);
        let index: usize = 500;
        let expected = ((index & ((1 << (3 + 4)) - 1)) >> 4) as u32;

        let actual = range_check.packed_at(index / N_LANES).to_array()[index % N_LANES].0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_range_check_gen_column_simd() {
        let ranges = [3, 1];
        let expected_0 = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7].map(M31);
        let expected_1 = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1].map(M31);

        let col_0 = RangeCheck::new(ranges, 0);
        let col_1 = RangeCheck::new(ranges, 1);
        let col_0_simd = col_0.gen_column_simd().to_cpu().to_vec();
        let col_1_simd = col_1.gen_column_simd().to_cpu().to_vec();

        assert_eq!(col_0_simd, expected_0);
        assert_eq!(col_1_simd, expected_1);
    }

    #[test]
    fn test_range_check_id() {
        let ranges = [1, 2, 3, 4];
        let range_check = RangeCheck::new(ranges, 2);

        let id = range_check.id();

        assert_eq!(id.id, "range_check_1_2_3_4_column_2");
    }

    #[test]
    fn test_preprocessed_root_regression() {
        let log_blowup_factor = 1;
        let expected = Blake2sHash::from(
            hex::decode("1b15a11b1a7cbb7a2120361f80ae300bce567c28498f1cfe28accbaade41a16e")
                .expect("Invalid hex string"),
        );

        let root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(log_blowup_factor);

        assert_eq!(root, expected);
    }
}

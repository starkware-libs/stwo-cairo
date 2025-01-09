use itertools::Itertools;
use prover_types::simd::LOG_N_LANES;
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;

use super::LOG_MAX_ROWS;

const N_PREPROCESSED_COLUMN_SIZES: usize = (LOG_MAX_ROWS - LOG_N_LANES) as usize + 1;

// List of sizes to initialize the preprocessed trace with for `PreprocessedColumn::IsFirst`.
const IS_FIRST_LOG_SIZES: [u32; N_PREPROCESSED_COLUMN_SIZES] = preprocessed_log_sizes();

// List of sizes to initialize the preprocessed trace with for `PreprocessedColumn::Seq`.
const SEQ_LOG_SIZES: [u32; N_PREPROCESSED_COLUMN_SIZES] = preprocessed_log_sizes();

/// [LOG_MAX_ROWS, LOG_MAX_ROWS - 1, ..., LOG_N_LANES]
const fn preprocessed_log_sizes() -> [u32; N_PREPROCESSED_COLUMN_SIZES] {
    let mut arr = [0; N_PREPROCESSED_COLUMN_SIZES];
    let mut i = 0;
    while i < N_PREPROCESSED_COLUMN_SIZES {
        arr[i] = LOG_MAX_ROWS - i as u32;
        i += 1;
    }
    arr
}

/// Returns column info for the preprocessed trace.
pub fn preprocessed_trace_columns() -> Vec<PreprocessedColumn> {
    let is_first_columns = IS_FIRST_LOG_SIZES.map(PreprocessedColumn::IsFirst);
    let seq_columns = SEQ_LOG_SIZES.map(PreprocessedColumn::Seq);
    [is_first_columns, seq_columns]
        .into_iter()
        .flat_map(|columns| columns.into_iter())
        .sorted_by_key(|column| std::cmp::Reverse(column.log_size()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_are_in_decending_order() {
        let columns = preprocessed_trace_columns();

        assert!(columns
            .windows(2)
            .all(|w| w[0].log_size() >= w[1].log_size()));
    }
}

use itertools::Itertools;
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;

// List of sizes to initialize the preprocessed trace with for `PreprocessedColumn::IsFirst`.
const IS_FIRST_LOG_SIZES: [u32; 19] = [
    22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
];

// List of sizes to initialize the preprocessed trace with for `PreprocessedColumn::Seq`.
const SEQ_LOG_SIZES: [u32; 19] = [
    22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
];

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

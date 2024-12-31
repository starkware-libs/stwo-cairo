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

// Returns the preprocessed columns info for the preprocessed trace.
pub fn preprocessed_trace_columns() -> Vec<PreprocessedColumn> {
    let mut res = Vec::new();
    res.extend(
        IS_FIRST_LOG_SIZES
            .iter()
            .cloned()
            .map(PreprocessedColumn::IsFirst)
            .collect_vec(),
    );
    res.extend(
        SEQ_LOG_SIZES
            .iter()
            .cloned()
            .map(PreprocessedColumn::Seq)
            .collect_vec(),
    );

    // Columns should be in descending order of log size.
    res.sort_by_key(|b| std::cmp::Reverse(b.log_size()));
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessed_trace_columns() {
        let columns = preprocessed_trace_columns();
        assert!(!columns.is_empty());
        let mut size = columns[0].log_size();

        for column in columns {
            assert!(column.log_size() <= size);
            size = column.log_size();
        }
    }
}

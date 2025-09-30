use crate::{PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl};

#[test]
fn test_preprocessed_column_set() {
    let mut set: PreprocessedColumnSet = Default::default();
    let seq_16_column = PreprocessedColumn::Seq(16);
    let seq_10_column = PreprocessedColumn::Seq(10);

    set.insert(seq_16_column);
    set.insert(seq_16_column);

    assert!(set.contains(seq_16_column));
    assert!(!set.contains(seq_10_column));
    assert_eq!(set.values, array![seq_16_column]);
}

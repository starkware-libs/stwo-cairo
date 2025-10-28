use crate::{PreprocessedColumnSet, PreprocessedColumnSetImpl};

#[test]
fn test_preprocessed_column_set() {
    let mut set: PreprocessedColumnSet = Default::default();
    let first_column = 16;
    let second_column = 10;

    set.insert(first_column);
    set.insert(first_column);

    assert!(set.contains(first_column));
    assert!(!set.contains(second_column));
}

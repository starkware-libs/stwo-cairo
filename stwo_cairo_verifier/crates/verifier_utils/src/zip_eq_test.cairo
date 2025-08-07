use crate::zip_eq::zip_eq;


#[test]
fn test_iterator_zip_eq() {
    let mut iter = zip_eq(array![1, 2, 3], array![4, 5, 6]);

    assert_eq!(iter.next(), Some((1, 4)));
    assert_eq!(iter.next(), Some((2, 5)));
    assert_eq!(iter.next(), Some((3, 6)));
    assert_eq!(iter.next(), None);

    // Test with nested zip_eq
    let mut iter = zip_eq(zip_eq(array![1, 2, 3], array![4, 5, 6]), array![7, 8, 9]);

    assert_eq!(iter.next(), Some(((1, 4), 7)));
    assert_eq!(iter.next(), Some(((2, 5), 8)));
    assert_eq!(iter.next(), Some(((3, 6), 9)));
    assert_eq!(iter.next(), None);
}


#[test]
#[should_panic(expected: "ZipEq: iterators have different lengths")]
fn test_iterator_zip_eq_a_longer_than_b() {
    let mut iter = zip_eq(array![1, 2, 3], array![4, 5]);

    for _ in iter {}
}


#[test]
#[should_panic(expected: "ZipEq: iterators have different lengths")]
fn test_iterator_zip_eq_b_longer_than_a() {
    let mut iter = zip_eq(array![1], array![4, 5]);

    for _ in iter {}
}

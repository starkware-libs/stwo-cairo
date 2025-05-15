use crate::fields::m31::{M31, m31};
use crate::fields::{BatchInvertible, Invertible};

#[test]
fn test_batch_inverse() {
    let arr = array![m31(2), m31(3), m31(5), m31(7)];
    let mut arr_inv = array![];
    for v in arr.span() {
        arr_inv.append((*v).inverse());
    }

    let res = BatchInvertible::batch_inverse(arr);

    assert_eq!(res, arr_inv);
}

#[test]
fn test_batch_inverse_with_empty_array() {
    let arr: Array<M31> = array![];

    let res = BatchInvertible::batch_inverse(arr);

    assert_eq!(res, array![]);
}

#[test]
fn test_batch_inverse_with_single_value() {
    let two = m31(2);
    let two_inv = two.inverse();
    let arr = array![two];

    let res = BatchInvertible::batch_inverse(arr);

    assert_eq!(res, array![two_inv]);
}

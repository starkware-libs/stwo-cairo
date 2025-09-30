use crate::utils::{ArrayImpl, bit_reverse_index, pow2_u64};

#[test]
fn test_bit_reverse() {
    // 1 bit
    assert_eq!(0, bit_reverse_index(0, 1));
    assert_eq!(1, bit_reverse_index(1, 1));

    // 2 bits
    assert_eq!(0, bit_reverse_index(0, 2));
    assert_eq!(2, bit_reverse_index(1, 2));
    assert_eq!(1, bit_reverse_index(2, 2));
    assert_eq!(3, bit_reverse_index(3, 2));

    // 3 bits
    assert_eq!(0, bit_reverse_index(0, 3));
    assert_eq!(4, bit_reverse_index(1, 3));
    assert_eq!(2, bit_reverse_index(2, 3));
    assert_eq!(6, bit_reverse_index(3, 3));

    // 16 bits
    assert_eq!(24415, bit_reverse_index(64250, 16));

    // 31 bits
    assert_eq!(16448250, bit_reverse_index(800042880, 31));
}

#[test]
fn test_array_new_repeated() {
    assert_eq!(ArrayImpl::new_repeated(n: 5, v: 3_usize), array![3, 3, 3, 3, 3]);
}

#[test]
fn test_pow2_u64() {
    assert_eq!(pow2_u64(0), 1);
    assert_eq!(pow2_u64(1), 2);
    assert_eq!(pow2_u64(31), 0x80000000);
    assert_eq!(pow2_u64(32), 0x100000000);
    assert_eq!(pow2_u64(63), 0x8000000000000000);
}

#[test]
#[should_panic]
fn test_pow2_u64_panic() {
    pow2_u64(64);
}

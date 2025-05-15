use crate::utils::{ArrayImpl, bit_reverse_index, gen_bit_mask};

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
fn test_sort_ascending() {
    assert_eq!(array![6_usize, 5, 1, 4, 2, 3].sort_ascending(), array![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_dedup() {
    assert_eq!(array![1_usize, 1, 1, 2, 2, 3, 4, 5, 5, 5].dedup(), array![1, 2, 3, 4, 5]);
}

#[test]
fn test_array_new_repeated() {
    assert_eq!(ArrayImpl::new_repeated(n: 5, v: 3_usize), array![3, 3, 3, 3, 3]);
}

#[test]
fn test_gen_bit_mask_with_0_bits() {
    assert_eq!(gen_bit_mask(0), 0);
}

#[test]
fn test_gen_bit_mask_with_8_bits() {
    assert_eq!(gen_bit_mask(8), 0b11111111);
}

#[test]
fn test_gen_bit_mask_with_128_bits() {
    assert_eq!(gen_bit_mask(128), 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
}

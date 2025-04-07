use core::array::ToSpanTrait;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::WrappingMul;
use core::traits::DivRem;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::utils::pow2;
use super::components::memory_id_to_big;


#[generate_trait]
pub impl U32Impl of U32ExTrait {
    /// Returns the smallest power of two greater than or equal to self.
    ///
    /// Panics if the next power of two is greater than the type’s maximum value.
    fn next_power_of_two(self: u32) -> u32 {
        let mut next_power_of_two = 1;
        while next_power_of_two < self {
            next_power_of_two *= 2;
        }
        next_power_of_two
    }

    /// Returns the base 2 logarithm of the number, rounded down.
    fn ilog2(self: u32) -> u32 {
        let self: u64 = self.into();
        let mut res = 0;
        let mut next_power_of_two = 1;
        while next_power_of_two < self {
            next_power_of_two *= 2;
            res += 1;
        }
        res
    }
}

#[generate_trait]
pub impl UsizeImpl of UsizeExTrait {
    /// Calculates the quotient of `self` and `other`, rounding the result towards positive
    /// infinity.
    ///
    /// # Panics
    ///
    /// This function will panic if `other` is zero.
    fn div_ceil(self: usize, other: usize) -> usize {
        let (d, r) = DivRem::div_rem(self, other.try_into().unwrap());
        if r > 0 {
            d + 1
        } else {
            d
        }
    }
}

pub fn tree_array_concat_cols(tree_array: Array<TreeArray<Span<u32>>>) -> TreeArray<Span<u32>> {
    let mut tree0 = array![];
    let mut tree1 = array![];
    let mut tree2 = array![];

    for curr_tree in tree_array.span() {
        // TODO: Instead of changing this to make it generic just refactor so the
        // whole function can be removed.
        assert!(curr_tree.len() <= 3);

        if curr_tree.len() > 0 {
            tree0.append_span(*curr_tree[0]);
        }
        if curr_tree.len() > 1 {
            tree1.append_span(*curr_tree[1]);
        }
        if curr_tree.len() > 2 {
            tree2.append_span(*curr_tree[2]);
        }
    }

    array![tree0.span(), tree1.span(), tree2.span()]
}

/// Splits a 252 bit dense representation into felts, each with `N_BITS_PER_FELT` bits.
pub fn split_f252(x: [u32; 8]) -> [M31; memory_id_to_big::N_M31_IN_FELT252] {
    let mask = pow2(memory_id_to_big::N_BITS_PER_FELT) - 1;
    let segments: [u32; memory_id_to_big::N_M31_IN_FELT252] = split(x, mask);
    let mut m31_segments = array![];

    for segment in segments.span() {
        m31_segments.append((*segment).try_into().unwrap());
    }

    (*m31_segments.span().try_into().unwrap()).unbox()
}

/// Constructs a `felt252` from 8 u32 limbs.
/// Doesn't check for overflow, i.e, the result is in fact a u256 modulo p252.
pub fn reconstitute_f252(x: [u32; 8]) -> felt252 {
    let x = x.span();
    let mut res: felt252 = 0;
    for i in 0..8_usize {
        let limb: felt252 = (*x[7 - i]).into();
        res = res * 0x100000000 + limb;
    }
    res
}


/// Splits a 32N bit dense representation into felts, each with N_BITS_PER_FELT bits.
///
/// Parameters:
/// - `N`: the number of 32-bit words in the input.
/// - `M`: the number of felts in the output.
/// - `x`: the input dense representation.
/// - `mask`: (1 << N_BITS_PER_FELT) - 1.
// TODO: Why is the mask passed?
fn split<
    const N: usize,
    const M: usize,
    impl FixedArrayToSpan: ToSpanTrait<[u32; N], u32>,
    impl SpanTryIntoFixedArray: TryInto<Span<u32>, @Box<[u32; M]>>,
>(
    x: [u32; N], mask: u32,
) -> [u32; M] {
    let mut res = array![];
    let mut n_bits_in_word = 32;
    let mut word_iter = FixedArrayToSpan::span(@x).into_iter();
    let mut word = *word_iter.next().unwrap_or(@0);

    for _ in 0..M {
        if n_bits_in_word > memory_id_to_big::N_BITS_PER_FELT {
            res.append(word & mask);
            word /= pow2(memory_id_to_big::N_BITS_PER_FELT);
            n_bits_in_word -= memory_id_to_big::N_BITS_PER_FELT;
            continue;
        }

        let mut segment = word;
        // Fetch next word.
        word = *word_iter.next().unwrap_or(@0);

        // If we need more bits to fill, take from next word.
        if n_bits_in_word < memory_id_to_big::N_BITS_PER_FELT {
            segment = segment | ((WrappingMul::wrapping_mul(word, pow2(n_bits_in_word))) & mask);
            word /= pow2(memory_id_to_big::N_BITS_PER_FELT - n_bits_in_word);
        }

        res.append(segment);

        n_bits_in_word += 32 - memory_id_to_big::N_BITS_PER_FELT;
    }

    (*SpanTryIntoFixedArray::try_into(res.span()).unwrap()).unbox()
}

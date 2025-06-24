use core::array::ToSpanTrait;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::WrappingMul;
use core::traits::DivRem;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::utils::pow2;
use super::components::memory_id_to_big;


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
    let segments: [u32; memory_id_to_big::N_M31_IN_FELT252] = split(x);
    let mut m31_segments = array![];

    for segment in segments.span() {
        m31_segments.append((*segment).try_into().unwrap());
    }

    (*m31_segments.span().try_into().unwrap()).unbox()
}

/// Constructs a `felt252` from 8 u32 little-endian limbs.
/// Doesn't check for overflow, i.e, the result is in fact a u256 modulo p252.
pub fn construct_f252(x: Box<[u32; 8]>) -> felt252 {
    let [l0, l1, l2, l3, l4, l5, l6, l7] = x.unbox();
    let offset = 0x100000000;
    let result: felt252 = l7.into();
    let result = result * offset + l6.into();
    let result = result * offset + l5.into();
    let result = result * offset + l4.into();
    let result = result * offset + l3.into();
    let result = result * offset + l2.into();
    let result = result * offset + l1.into();
    result * offset + l0.into()
}

/// Deconstructs a `felt252` to 8 u32 little-endian limbs.
pub fn deconstruct_f252(x: felt252) -> Box<[u32; 8]> {
    let offset = 0x100000000;
    let cur: u256 = x.into();
    let (cur, l0) = DivRem::div_rem(cur, offset);
    let (cur, l1) = DivRem::div_rem(cur, offset);
    let (cur, l2) = DivRem::div_rem(cur, offset);
    let (cur, l3) = DivRem::div_rem(cur, offset);
    let (cur, l4) = DivRem::div_rem(cur, offset);
    let (cur, l5) = DivRem::div_rem(cur, offset);
    let (cur, l6) = DivRem::div_rem(cur, offset);
    let (_, l7) = DivRem::div_rem(cur, offset);
    BoxTrait::new(
        [
            l0.try_into().unwrap(), l1.try_into().unwrap(), l2.try_into().unwrap(),
            l3.try_into().unwrap(), l4.try_into().unwrap(), l5.try_into().unwrap(),
            l6.try_into().unwrap(), l7.try_into().unwrap(),
        ],
    )
}


/// Splits a 32N bit dense representation into felts, each with N_BITS_PER_FELT bits.
///
/// Parameters:
/// - `N`: the number of 32-bit words in the input.
/// - `M`: the number of felts in the output.
/// - `x`: the input dense representation.
fn split<
    const N: usize,
    const M: usize,
    impl FixedArrayToSpan: ToSpanTrait<[u32; N], u32>,
    impl SpanTryIntoFixedArray: TryInto<Span<u32>, @Box<[u32; M]>>,
>(
    x: [u32; N],
) -> [u32; M] {
    let mut res = array![];
    let mut n_bits_in_word = 32;
    let mut word_iter = FixedArrayToSpan::span(@x).into_iter();
    let mut word = *word_iter.next().unwrap_or(@0);

    let shift = pow2(memory_id_to_big::N_BITS_PER_FELT).try_into().unwrap();
    for _ in 0..M {
        if n_bits_in_word > memory_id_to_big::N_BITS_PER_FELT {
            let (high, low) = DivRem::div_rem(word, shift);
            res.append(low);
            word = high;
            n_bits_in_word -= memory_id_to_big::N_BITS_PER_FELT;
            continue;
        }

        let mut segment = word;
        // Fetch next word.
        word = *word_iter.next().unwrap_or(@0);

        // If we need more bits to fill, take from next word.
        if n_bits_in_word < memory_id_to_big::N_BITS_PER_FELT {
            let (_high, low) = DivRem::div_rem(
                WrappingMul::wrapping_mul(word, pow2(n_bits_in_word)), shift,
            );
            segment = segment + low;
            word /= pow2(memory_id_to_big::N_BITS_PER_FELT - n_bits_in_word);
        }

        res.append(segment);

        n_bits_in_word += 32 - memory_id_to_big::N_BITS_PER_FELT;
    }

    (*SpanTryIntoFixedArray::try_into(res.span()).unwrap()).unbox()
}

#[cfg(test)]
mod tests {
    use super::{construct_f252, deconstruct_f252};

    #[test]
    fn test_construct_felt() {
        assert_eq!(
            construct_f252(BoxTrait::new([1_u32, 2, 3, 4, 5, 6, 7, 8])),
            0x800000007000000060000000500000004000000030000000200000001,
        );
    }

    #[test]
    fn test_deconstruct_felt() {
        assert_eq!(
            deconstruct_f252(0x800000007000000060000000500000004000000030000000200000001).unbox(),
            [1_u32, 2, 3, 4, 5, 6, 7, 8],
        );
    }
}

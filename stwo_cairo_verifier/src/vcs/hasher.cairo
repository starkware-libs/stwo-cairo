use core::array::ArrayTrait;
use core::option::OptionTrait;
use core::poseidon::poseidon_hash_span;
use stwo_cairo_verifier::BaseField;

// A Merkle node hash is a hash of:
//   [left_child_hash, right_child_hash], column0_value, column1_value, ...
// "[]" denotes optional values.
// The largest Merkle layer has no left and right child hashes. The rest of the layers have
// children hashes.
// At each layer, the tree may have multiple columns of the same length as the layer.
// Each node in that layer contains one value from each column.
pub trait MerkleHasher {
    type Hash;
    // Hashes a single Merkle node.
    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, column_values: Array<BaseField>,
    ) -> Self::Hash;
}

// 8 M31 elements fit in a hash, since 31*8 = 242 < 252.
const M31_ELEMENETS_IN_HASH: usize = 8;
const M31_ELEMENETS_IN_HASH_MINUS1: usize = M31_ELEMENETS_IN_HASH - 1;
const M31_IN_HASH_SHIFT: felt252 = 0x80000000; // 2**31.
pub impl PoseidonMerkleHasher of MerkleHasher {
    type Hash = felt252;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Array<BaseField>,
    ) -> Self::Hash {
        let mut hash_array: Array<felt252> = Default::default();
        if let Option::Some((x, y)) = children_hashes {
            hash_array.append(x);
            hash_array.append(y);
        }

        // Pad column_values to a multiple of 8.
        let mut pad_len = M31_ELEMENETS_IN_HASH_MINUS1
            - ((column_values.len() + M31_ELEMENETS_IN_HASH_MINUS1) % M31_ELEMENETS_IN_HASH);
        while pad_len > 0 {
            column_values.append(core::num::traits::Zero::zero());
            pad_len = M31_ELEMENETS_IN_HASH_MINUS1
                - ((column_values.len() + M31_ELEMENETS_IN_HASH_MINUS1) % M31_ELEMENETS_IN_HASH);
        };

        while !column_values.is_empty() {
            let mut word = 0;
            // Hash M31_ELEMENETS_IN_HASH = 8 M31 elements into a single field element.
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            hash_array.append(word);
        };

        poseidon_hash_span(hash_array.span())
    }
}

#[cfg(test)]
mod tests {
    use stwo_cairo_verifier::fields::m31::{m31};
    use super::PoseidonMerkleHasher;

    #[test]
    fn test_m31() {
        assert_eq!(
            PoseidonMerkleHasher::hash_node(Option::None, array![m31(0), m31(1)]),
            2552053700073128806553921687214114320458351061521275103654266875084493044716
        );

        assert_eq!(
            PoseidonMerkleHasher::hash_node(Option::Some((1, 2)), array![m31(3)]),
            159358216886023795422515519110998391754567506678525778721401012606792642769
        );
    }
}

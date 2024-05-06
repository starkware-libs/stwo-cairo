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
const M31_ELS_IN_HASH: usize = 8;
const M31_ELS_IN_HASH_MINUS1: usize = 7;
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
        let mut pad_len = M31_ELS_IN_HASH_MINUS1
            - ((column_values.len() + M31_ELS_IN_HASH_MINUS1) % M31_ELS_IN_HASH);
        while pad_len > 0 {
            column_values.append(core::num::traits::Zero::zero());
            pad_len = M31_ELS_IN_HASH_MINUS1
                - ((column_values.len() + M31_ELS_IN_HASH_MINUS1) % M31_ELS_IN_HASH);
        };

        while !column_values.is_empty() {
            let mut word = 0;
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
            word = word * M31_IN_HASH_SHIFT + column_values.pop_front().unwrap().inner.into();
        };

        poseidon_hash_span(hash_array.span())
    }
}

#[cfg(test)]
mod tests {
    use super::PoseidonMerkleHasher;
    use stwo_cairo_verifier::fields::m31::{m31};

    #[test]
    fn test_m31() {
        assert_eq!(
            PoseidonMerkleHasher::hash_node(Option::None, array![m31(0), m31(1)]),
            973835572668429495915136902981656666590582180872133591629269551720657739196
        );
    }
}

use core::array::ArrayTrait;
use core::hash::HashStateTrait;
use core::poseidon::{poseidon_hash_span, hades_permutation, HashState};
use stwo_cairo_verifier::BaseField;

/// 8 M31 elements fit in a hash, since 31*8 = 242 < 252.
const M31_ELEMENETS_IN_HASH: usize = 8;

const M31_ELEMENETS_IN_HASH_MINUS1: usize = M31_ELEMENETS_IN_HASH - 1;

/// Equals `2^31`.
const M31_IN_HASH_SHIFT: felt252 = 0x80000000;

/// Equals `(2^31)^4`.
const M31_IN_HASH_SHIFT_POW_4: felt252 = 0x10000000000000000000000000000000;

/// A Merkle node hash is a hash of:
///   [left_child_hash, right_child_hash], column0_value, column1_value, ...
/// "[]" denotes optional values.
/// The largest Merkle layer has no left and right child hashes. The rest of the layers have
/// children hashes.
/// At each layer, the tree may have multiple columns of the same length as the layer.
/// Each node in that layer contains one value from each column.
pub trait MerkleHasher {
    type Hash;

    /// Hashes a single Merkle node.
    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, column_values: Array<BaseField>,
    ) -> Self::Hash;
}

pub impl PoseidonMerkleHasher of MerkleHasher {
    type Hash = felt252;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Array<BaseField>,
    ) -> Self::Hash {
        let mut hash_array: Array<felt252> = Default::default();
        if let Option::Some((x, y)) = children_hashes {
            // Most often a node has no column values.
            if column_values.len() == 0 {
                // Inlined and simplified `poseidon_hash_span(...)` for better performance.
                // TODO: Posiedon2 here { == hades_permutation(x, y, 2); }
                let (s0, s1, s2) = hades_permutation(x, y, 0);
                let hash_state = HashState { s0, s1, s2, odd: false };
                return hash_state.finalize();
            }
            hash_array.append(x);
            hash_array.append(y);
        } else {
            // Most often a single QM31 column commitment due to FRI.
            // TODO(andrew): Implement non-mixed degree merkle for FRI decommitments.
            if let Option::Some(values) = column_values.span().try_into() {
                // Inlined and simplified `poseidon_hash_span(...)` for better performance.
                let [v0, v1, v2, v3]: [BaseField; 4] = (*values).unbox();
                let mut word = v0.inner.into();
                word = word * M31_IN_HASH_SHIFT + v1.inner.into();
                word = word * M31_IN_HASH_SHIFT + v2.inner.into();
                word = word * M31_IN_HASH_SHIFT + v3.inner.into();
                word = word * M31_IN_HASH_SHIFT_POW_4;
                let (hash, _, _) = hades_permutation(word, 1, 0);
                return hash;
            }
        }

        // Pad column_values to a multiple of 8.
        let mut pad_len = M31_ELEMENETS_IN_HASH_MINUS1
            - ((column_values.len() + M31_ELEMENETS_IN_HASH_MINUS1) % M31_ELEMENETS_IN_HASH);
        let padding = core::num::traits::Zero::zero();
        for _ in 0..pad_len {
            column_values.append(padding);
        };

        let mut column_values = column_values.span();

        // TODO(andrew): Measure performance diff and consider inlining `poseidon_hash_span(..)`
        // functionality here to do all packing and hashing in a single pass.
        while let Option::Some(values) = column_values.multi_pop_front::<8>() {
            let [v0, v1, v2, v3, v4, v5, v6, v7] = (*values).unbox();
            let mut word = v0.inner.into();
            word = word * M31_IN_HASH_SHIFT + v1.inner.into();
            word = word * M31_IN_HASH_SHIFT + v2.inner.into();
            word = word * M31_IN_HASH_SHIFT + v3.inner.into();
            word = word * M31_IN_HASH_SHIFT + v4.inner.into();
            word = word * M31_IN_HASH_SHIFT + v5.inner.into();
            word = word * M31_IN_HASH_SHIFT + v6.inner.into();
            word = word * M31_IN_HASH_SHIFT + v7.inner.into();
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

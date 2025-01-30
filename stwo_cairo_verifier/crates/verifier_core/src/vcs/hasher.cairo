use core::array::ArrayTrait;
use core::blake::blake2s_compress;
use core::hash::HashStateTrait;
use core::poseidon::{HashState, hades_permutation, poseidon_hash_span};
use crate::BaseField;

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
        children_hashes: Option<(Self::Hash, Self::Hash)>, column_values: Span<BaseField>,
    ) -> Self::Hash;
}

pub impl PoseidonMerkleHasher of MerkleHasher {
    type Hash = felt252;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Span<BaseField>,
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
            if let Option::Some(values) = column_values.try_into() {
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

        if !column_values.is_empty() {
            let mut word = (*column_values.pop_front().unwrap_or(@BaseField { inner: 0 }))
                .inner
                .into();

            for _ in 1..M31_ELEMENETS_IN_HASH {
                let v = (*column_values.pop_front().unwrap_or(@BaseField { inner: 0 }))
                    .inner
                    .into();
                word = word * M31_IN_HASH_SHIFT + v;
            };

            hash_array.append(word);
        }

        poseidon_hash_span(hash_array.span())
    }
}


fn concat_blakes(l: Box<[u32; 8]>, r: Box<[u32; 8]>) -> Box<[u32; 16]> {
    let [l0, l1, l2, l3, l4, l5, l6, l7] = l.unbox();
    let [r0, r1, r2, r3, r4, r5, r6, r7] = r.unbox();
    BoxTrait::new([l0, l1, l2, l3, l4, l5, l6, l7, r0, r1, r2, r3, r4, r5, r6, r7])
}


pub impl Blake2sMerkleHasher of MerkleHasher {
    type Hash = Box<[u32; 8]>;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Span<BaseField>,
    ) -> Self::Hash {
        let mut state = BoxTrait::new([0_u32; 8]);
        let byte_count = 0;

        if let Option::Some((x, y)) = children_hashes {
            state = blake2s_compress(state, byte_count, concat_blakes(x, y));
            // Most often a node has no column values.
            if column_values.len() == 0 {
                return state;
            }
        } else {
            // Most often a single QM31 column commitment due to FRI.
            // TODO(andrew): Implement non-mixed degree merkle for FRI decommitments.
            if let Option::Some(values) = column_values.try_into() {
                let [v0, v1, v2, v3]: [BaseField; 4] = (*values).unbox();
                let msg: Box::<[u32; 16]> = BoxTrait::new(
                    [
                        v0.into(), v1.into(), v2.into(), v3.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0,
                    ],
                );
                return blake2s_compress(state, byte_count, msg);
            }
        }

        // functionality here to do all packing and hashing in a single pass.
        while let Option::Some(values) = column_values.multi_pop_front::<16>() {
            let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15] = (*values)
                .unbox();
            let msg: Box<[u32; 16]> = BoxTrait::new(
                [
                    v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(),
                    v7.into(), v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), v13.into(),
                    v14.into(), v15.into(),
                ],
            );
            state = blake2s_compress(state, byte_count, msg);
        };

        if !column_values.is_empty() {
            let mut arr: Array<u32> = array![];
            for value in column_values {
                arr.append((*value).into());
            };

            for _ in arr.len()..16 {
                arr.append(0);
            };

            state = blake2s_compress(state, byte_count, *arr.span().try_into().unwrap());
        }

        state
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::m31::{m31};
    use super::PoseidonMerkleHasher;

    #[test]
    fn test_m31() {
        assert_eq!(
            PoseidonMerkleHasher::hash_node(Option::None, array![m31(0), m31(1)].span()),
            2552053700073128806553921687214114320458351061521275103654266875084493044716,
        );

        assert_eq!(
            PoseidonMerkleHasher::hash_node(Option::Some((1, 2)), array![m31(3)].span()),
            159358216886023795422515519110998391754567506678525778721401012606792642769,
        );
    }
}

use core::array::ArrayTrait;
use core::blake::blake2s_compress;
use core::box::BoxImpl;
use core::hash::HashStateTrait;
use crate::fields::m31::{M31, M31Zero};
use crate::vcs::hasher::MerkleHasher;
use crate::BaseField;

const M31_ELEMENETS_IN_MSG: usize = 16;

/// State for Blake2s hash.
type Blake2sState = Box<[u32; 8]>;

pub impl Blake2sMerkleHasher of MerkleHasher {
    type Hash = Blake2sHash;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Span<BaseField>,
    ) -> Self::Hash {
        let mut state = BoxImpl::new([0, 0, 0, 0, 0, 0, 0, 0]);
        let mut hash_array: Array<felt252> = Default::default();
        if let Some((x, y)) = children_hashes {
            let [x0, x1, x2, x3, x4, x5, x6, x7] = x.hash.unbox();
            let [y0, y1, y2, y3, y4, y5, y6, y7] = y.hash.unbox();
            let msg = BoxImpl::new(
                [x0, x1, x2, x3, x4, x5, x6, x7, y0, y1, y2, y3, y4, y5, y6, y7],
            );
            state = blake2s_compress(:state, byte_count: 0, :msg);

            // Most often a node has no column values.
            if column_values.len() == 0 {
                return Blake2sHash { hash: state };
            }
        } else { // TODO(andrew): Consider handling single column case (used lots due to FRI).
        }

        // This loop doesn't handle padding.
        // TODO(andrew): Measure performance diff and consider inlining `poseidon_hash_span(..)`
        // functionality here to do all packing and hashing in a single pass.
        while let Some(values) = column_values.multi_pop_front::<M31_ELEMENETS_IN_MSG>() {
            let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15] = (*values)
                .unbox();
            let msg = BoxImpl::new(
                [
                    v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(),
                    v7.into(), v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), v13.into(),
                    v14.into(), v15.into(),
                ],
            );
            // TODO: Determine if byte_count=0 and unfinalized hashes for the leaves is ok.
            state = blake2s_compress(:state, byte_count: 0, :msg);
        }

        // Padding last column_values with zeros.
        if !column_values.is_empty() {
            let mut padded_column_values = array![];

            for v in column_values {
                padded_column_values.append((*v).into());
            }

            for _ in column_values.len()..M31_ELEMENETS_IN_MSG {
                padded_column_values.append(0);
            }

            let msg = *padded_column_values.span().try_into().unwrap();
            state = blake2s_compress(:state, byte_count: 0, :msg);
        }

        Blake2sHash { hash: state }
    }
}

#[derive(Drop, Clone, Debug)]
pub struct Blake2sHash {
    pub hash: Blake2sState,
}

impl Blake2sHashPartialEq of PartialEq<Blake2sHash> {
    fn eq(lhs: @Blake2sHash, rhs: @Blake2sHash) -> bool {
        lhs.hash.unbox() == rhs.hash.unbox()
    }
}

impl Blake2sHashSerde of Serde<Blake2sHash> {
    fn serialize(self: @Blake2sHash, ref output: Array<felt252>) {
        let [w0, w1, w2, w3, w4, w5, w6, w7] = self.hash.unbox();
        output.append(w0.into());
        output.append(w1.into());
        output.append(w2.into());
        output.append(w3.into());
        output.append(w4.into());
        output.append(w5.into());
        output.append(w6.into());
        output.append(w7.into());
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<Blake2sHash> {
        let [w0, w1, w2, w3, w4, w5, w6, w7] = (*serialized.multi_pop_front()?).unbox();
        Some(
            Blake2sHash {
                hash: BoxImpl::new(
                    [
                        w0.try_into().unwrap(), w1.try_into().unwrap(), w2.try_into().unwrap(),
                        w3.try_into().unwrap(), w4.try_into().unwrap(), w5.try_into().unwrap(),
                        w6.try_into().unwrap(), w7.try_into().unwrap(),
                    ],
                ),
            },
        )
    }
}


#[cfg(test)]
mod tests {
    use core::box::BoxImpl;
    use crate::fields::m31::m31;
    use super::{Blake2sHash, Blake2sMerkleHasher};

    #[test]
    fn test_hash_node_with_no_children() {
        assert_eq!(
            Blake2sMerkleHasher::hash_node(None, array![m31(0), m31(1)].span()).hash.unbox(),
            [
                3326510057, 2699391079, 499129371, 1615300198, 152557944, 2105250166, 920231055,
                3607089427,
            ],
        );
    }

    #[test]
    fn test_hash_node_with_children() {
        let l_node = Blake2sHash {
            hash: BoxImpl::new(
                [
                    3326510057, 2699391079, 499129371, 1615300198, 152557944, 2105250166, 920231055,
                    3607089427,
                ],
            ),
        };
        let r_node = Blake2sHash {
            hash: BoxImpl::new(
                [
                    707328262, 1356420215, 727373128, 3551204147, 4019359644, 4016890851,
                    1375080809, 3547510316,
                ],
            ),
        };
        assert_eq!(
            Blake2sMerkleHasher::hash_node(Some((l_node, r_node)), array![m31(3)].span())
                .hash
                .unbox(),
            [
                4291656322, 3451476936, 1663868538, 3400868049, 1858355141, 3484943437, 3592219053,
                2464289423,
            ],
        );
    }

    #[test]
    fn test_hash_node_with_many_values() {
        let l_node = Blake2sHash {
            hash: BoxImpl::new(
                [
                    3326510057, 2699391079, 499129371, 1615300198, 152557944, 2105250166, 920231055,
                    3607089427,
                ],
            ),
        };
        let r_node = Blake2sHash {
            hash: BoxImpl::new(
                [
                    707328262, 1356420215, 727373128, 3551204147, 4019359644, 4016890851,
                    1375080809, 3547510316,
                ],
            ),
        };
        let values = array![
            m31(0),
            m31(1),
            m31(2),
            m31(3),
            m31(4),
            m31(5),
            m31(6),
            m31(7),
            m31(8),
            m31(9),
            m31(10),
            m31(11),
            m31(12),
            m31(13),
            m31(14),
            m31(15),
            m31(16),
            m31(17),
            m31(18),
            m31(19),
            m31(20),
            m31(21),
        ];
        assert_eq!(
            Blake2sMerkleHasher::hash_node(Some((l_node, r_node)), values.span()).hash.unbox(),
            [
                1386089130, 164151351, 3113069523, 2362825950, 1134782659, 2525373360, 1754076912,
                4040834212,
            ],
        );
    }
}

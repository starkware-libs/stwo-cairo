use core::array::ArrayTrait;
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use stwo_verifier_utils::BLAKE2S_256_INITIAL_STATE;
use crate::BaseField;
use crate::fields::m31::{M31, M31Zero};
use crate::vcs::hasher::MerkleHasher;

const M31_ELEMENTS_IN_MSG: usize = 16;

/// State for Blake2s hash.
type Blake2sState = Box<[u32; 8]>;

pub impl Blake2sMerkleHasher of MerkleHasher {
    type Hash = Blake2sHash;

    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, mut column_values: Span<BaseField>,
    ) -> Self::Hash {
        let mut state = BoxImpl::new(BLAKE2S_256_INITIAL_STATE);

        // No column values.
        if column_values.is_empty() {
            let (msg, byte_count) = match children_hashes {
                Some((
                    x, y,
                )) => {
                    let [x0, x1, x2, x3, x4, x5, x6, x7] = x.hash.unbox();
                    let [y0, y1, y2, y3, y4, y5, y6, y7] = y.hash.unbox();
                    (
                        BoxImpl::new(
                            [x0, x1, x2, x3, x4, x5, x6, x7, y0, y1, y2, y3, y4, y5, y6, y7],
                        ),
                        64,
                    )
                },
                None => panic!("Empty nodes are not supported"),
            };
            return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
        }

        let mut byte_count = 0_u32;
        if let Some((x, y)) = children_hashes {
            let [x0, x1, x2, x3, x4, x5, x6, x7] = x.hash.unbox();
            let [y0, y1, y2, y3, y4, y5, y6, y7] = y.hash.unbox();
            let msg = BoxImpl::new(
                [x0, x1, x2, x3, x4, x5, x6, x7, y0, y1, y2, y3, y4, y5, y6, y7],
            );
            byte_count = 64;
            state = blake2s_compress(:state, :byte_count, :msg);
        }

        // Special case #1: Single QM31 column (split into 4 M31 coordinates), inner FRI layer
        // decommitment phase.
        if let Some(quadruplet_box) = column_values.try_into() {
            let [v0, v1, v2, v3]: [M31; 4] = (*quadruplet_box).unbox();
            let msg = BoxImpl::new(
                [v0.into(), v1.into(), v2.into(), v3.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            );
            byte_count += 16;
            return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
        }

        // Special case #2: Single M31 column, queried value decommitment phase, common for PP tree.
        if let Some(singleton_box) = column_values.try_into() {
            let [v0]: [M31; 1] = (*singleton_box).unbox();
            let msg = BoxImpl::new([v0.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            byte_count += 4;
            return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
        }

        // Compress full 16-element blocks except the last one.
        if let Some(mut full_block) = column_values.multi_pop_front::<M31_ELEMENTS_IN_MSG>() {
            // Compress intermediate blocks.
            while let Some(next_block) = column_values.multi_pop_front::<M31_ELEMENTS_IN_MSG>() {
                let msg = m31x16_to_u32x16_boxed(full_block);
                byte_count += 64;
                state = blake2s_compress(:state, :byte_count, :msg);
                full_block = next_block;
            }

            // Handle the last full block.
            let msg = m31x16_to_u32x16_boxed(full_block);
            byte_count += 64;
            if column_values.is_empty() {
                return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
            } else {
                state = blake2s_compress(:state, :byte_count, :msg);
            }
        }

        // Handle the last partial block.
        let mut padded_values: Array<u32> = column_values
            .into_iter()
            .map(|v| (*v).into())
            .collect();
        let last_block_length = padded_values.len();
        for _ in last_block_length..M31_ELEMENTS_IN_MSG {
            padded_values.append(0);
        }
        let msg = *padded_values.span().try_into().unwrap();
        byte_count += last_block_length * 4;
        Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) }
    }
}

/// Converts a full 16-element block of M31 values to a 16-element block of u32 values.
#[inline]
fn m31x16_to_u32x16_boxed(full_block: @Box<[M31; 16]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15] = full_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), v13.into(), v14.into(),
            v15.into(),
        ],
    )
}

#[derive(Drop, Copy, Debug)]
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

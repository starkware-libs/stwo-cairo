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

        // No column values (most common case).
        if column_values.is_empty() {
            let (msg, byte_count) = match children_hashes {
                Some((x, y)) => (combine_u32_block(x.hash, y.hash), 64),
                None => panic!("Empty nodes are not supported"),
            };
            return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
        }

        let mut byte_count = 0_u32;
        if let Some((x, y)) = children_hashes {
            let msg = combine_u32_block(x.hash, y.hash);
            byte_count = 64;
            state = blake2s_compress(:state, :byte_count, :msg);
        }

        // Special case #1: Single QM31 column (split into 4 M31 coordinates), inner FRI layer
        // decommitment phase.
        if let Some(quadruplet_box) = column_values.try_into() {
            let msg = as_u32_block_pad_12(quadruplet_box);
            byte_count += 16;
            return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
        }

        // Special case #2: Single M31 column, queried value decommitment phase, common for PP tree.
        if let Some(singleton_box) = column_values.try_into() {
            let msg = as_u32_block_pad_15(singleton_box);
            byte_count += 4;
            return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
        }

        // Compress full 16-element blocks except the last one.
        if let Some(mut full_block) = column_values.multi_pop_front::<M31_ELEMENTS_IN_MSG>() {
            // Compress intermediate blocks.
            while let Some(next_block) = column_values.multi_pop_front::<M31_ELEMENTS_IN_MSG>() {
                let msg = as_u32_block(full_block);
                byte_count += 64;
                state = blake2s_compress(:state, :byte_count, :msg);
                full_block = next_block;
            }

            // Handle the last full block.
            let msg = as_u32_block(full_block);
            byte_count += 64;
            if column_values.is_empty() {
                // The last block is a full 16-element block; finalize the hash using this block as
                // the final input.
                return Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) };
            }

            state = blake2s_compress(:state, :byte_count, :msg);
        }

        // Handle the last partial block.
        let (msg, last_block_length) = pad_partial_block(column_values);
        byte_count += last_block_length;
        Blake2sHash { hash: blake2s_finalize(:state, :byte_count, :msg) }
    }
}

/// Combines two 8-element M31 blocks into a 16-element u32 block.
#[inline]
fn combine_u32_block(left: Box<[u32; 8]>, right: Box<[u32; 8]>) -> Box<[u32; 16]> {
    let [l0, l1, l2, l3, l4, l5, l6, l7] = left.unbox();
    let [r0, r1, r2, r3, r4, r5, r6, r7] = right.unbox();
    BoxImpl::new([l0, l1, l2, l3, l4, l5, l6, l7, r0, r1, r2, r3, r4, r5, r6, r7])
}

/// Converts a full 16-element block of M31 values to a 16-element block of u32 values.
#[inline]
fn as_u32_block(full_block: @Box<[M31; 16]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15] = full_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), v13.into(), v14.into(),
            v15.into(),
        ],
    )
}

/// Converts a partial block of M31 values to a 16-element block of u32 values and the number of
/// bytes in the partial block (before padding).
fn pad_partial_block(values: Span<M31>) -> (Box<[u32; 16]>, u32) {
    if let Some(box8) = values.try_into() {
        return (as_u32_block_pad_8(box8), 32);
    }
    if let Some(box12) = values.try_into() {
        return (as_u32_block_pad_4(box12), 48);
    }
    if let Some(box6) = values.try_into() {
        return (as_u32_block_pad_10(box6), 24);
    }
    if let Some(box2) = values.try_into() {
        return (as_u32_block_pad_14(box2), 8);
    }
    if let Some(box4) = values.try_into() {
        return (as_u32_block_pad_12(box4), 16);
    }
    if let Some(box1) = values.try_into() {
        return (as_u32_block_pad_15(box1), 4);
    }
    if let Some(box3) = values.try_into() {
        return (as_u32_block_pad_13(box3), 12);
    }
    if let Some(box5) = values.try_into() {
        return (as_u32_block_pad_11(box5), 20);
    }
    if let Some(box7) = values.try_into() {
        return (as_u32_block_pad_9(box7), 28);
    }
    if let Some(box9) = values.try_into() {
        return (as_u32_block_pad_7(box9), 36);
    }
    if let Some(box10) = values.try_into() {
        return (as_u32_block_pad_6(box10), 40);
    }
    if let Some(box11) = values.try_into() {
        return (as_u32_block_pad_5(box11), 44);
    }
    if let Some(box13) = values.try_into() {
        return (as_u32_block_pad_3(box13), 52);
    }
    if let Some(box14) = values.try_into() {
        return (as_u32_block_pad_2(box14), 56);
    }
    if let Some(box15) = values.try_into() {
        return (as_u32_block_pad_1(box15), 60);
    }
    panic!("Invalid number of M31 values");
}

#[inline]
fn as_u32_block_pad_15(partial_block: @Box<[M31; 1]>) -> Box<[u32; 16]> {
    let [v0] = partial_block.unbox();
    BoxImpl::new([v0.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

#[inline]
fn as_u32_block_pad_14(partial_block: @Box<[M31; 2]>) -> Box<[u32; 16]> {
    let [v0, v1] = partial_block.unbox();
    BoxImpl::new([v0.into(), v1.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

#[inline]
fn as_u32_block_pad_13(partial_block: @Box<[M31; 3]>) -> Box<[u32; 16]> {
    let [v0, v1, v2] = partial_block.unbox();
    BoxImpl::new([v0.into(), v1.into(), v2.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

#[inline]
fn as_u32_block_pad_12(partial_block: @Box<[M31; 4]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3] = partial_block.unbox();
    BoxImpl::new([v0.into(), v1.into(), v2.into(), v3.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
}

#[inline]
fn as_u32_block_pad_11(partial_block: @Box<[M31; 5]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4] = partial_block.unbox();
    BoxImpl::new(
        [v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )
}

#[inline]
fn as_u32_block_pad_10(partial_block: @Box<[M31; 6]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_9(partial_block: @Box<[M31; 7]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_8(partial_block: @Box<[M31; 8]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            0, 0, 0, 0, 0, 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_7(partial_block: @Box<[M31; 9]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), 0, 0, 0, 0, 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_6(partial_block: @Box<[M31; 10]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), 0, 0, 0, 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_5(partial_block: @Box<[M31; 11]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), 0, 0, 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_4(partial_block: @Box<[M31; 12]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), v11.into(), 0, 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_3(partial_block: @Box<[M31; 13]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), 0, 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_2(partial_block: @Box<[M31; 14]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), v13.into(), 0, 0,
        ],
    )
}

#[inline]
fn as_u32_block_pad_1(partial_block: @Box<[M31; 15]>) -> Box<[u32; 16]> {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14] = partial_block.unbox();
    BoxImpl::new(
        [
            v0.into(), v1.into(), v2.into(), v3.into(), v4.into(), v5.into(), v6.into(), v7.into(),
            v8.into(), v9.into(), v10.into(), v11.into(), v12.into(), v13.into(), v14.into(), 0,
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

use core::array::ArrayTrait;
use core::blake::{blake2s_compress, blake2s_finalize};
use core::box::BoxImpl;
use crate::BaseField;
use crate::channel::blake2s::BLAKE2S_256_INITIAL_STATE;
use crate::fields::m31::M31Zero;
use crate::utils::SpanExTrait;
use crate::vcs::hasher::MerkleHasher;

const M31_ELEMENETS_IN_MSG: usize = 16;

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

        // This loop doesn't handle padding.
        // TODO(andrew): Measure performance diff and consider inlining `poseidon_hash_span(..)`
        // functionality here to do all packing and hashing in a single pass.
        // TODO(andrew): Consider handling single column case (used lots due to FRI).
        let rem = column_values.len() % M31_ELEMENETS_IN_MSG;
        let last_block_length = match rem {
            0 => M31_ELEMENETS_IN_MSG,
            _ => rem,
        };

        let (mut column_values, last_block) = column_values
            .split_at(column_values.len() - last_block_length);

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
            byte_count += 64;
            state = blake2s_compress(:state, :byte_count, :msg);
        }

        // Padding last column_values with zeros.
        let mut padded_column_values = last_block.into_iter().map(|x| (*x).into()).collect();
        append_padding(ref padded_column_values, M31_ELEMENETS_IN_MSG - last_block_length, 0);

        byte_count += last_block_length * 4;
        let msg = *padded_column_values.span().try_into().unwrap();
        state = blake2s_finalize(:state, :byte_count, :msg);

        Blake2sHash { hash: state }
    }
}

/// Appends `count ∈ [0, 16)` padding values to the array.
fn append_padding(ref arr: Array<u32>, count: u32, padding_value: u32) {
    // Avoid AP alignment of this function (prevents memory holes).
    core::internal::revoke_ap_tracking();
    if count == 0 {
        return;
    }
    arr.append(padding_value);
    if count == 1 {
        return;
    }
    arr.append(padding_value);
    if count == 2 {
        return;
    }
    arr.append(padding_value);
    if count == 3 {
        return;
    }
    arr.append(padding_value);
    if count == 4 {
        return;
    }
    arr.append(padding_value);
    if count == 5 {
        return;
    }
    arr.append(padding_value);
    if count == 6 {
        return;
    }
    arr.append(padding_value);
    if count == 7 {
        return;
    }
    arr.append(padding_value);
    if count == 8 {
        return;
    }
    arr.append(padding_value);
    if count == 9 {
        return;
    }
    arr.append(padding_value);
    if count == 10 {
        return;
    }
    arr.append(padding_value);
    if count == 11 {
        return;
    }
    arr.append(padding_value);
    if count == 12 {
        return;
    }
    arr.append(padding_value);
    if count == 13 {
        return;
    }
    arr.append(padding_value);
    if count == 14 {
        return;
    }
    arr.append(padding_value);
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

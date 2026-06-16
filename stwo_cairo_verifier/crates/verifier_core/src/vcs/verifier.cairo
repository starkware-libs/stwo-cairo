use core::array::{ArrayTrait, SpanTrait, ToSpanTrait};
use core::dict::{Felt252Dict, Felt252DictEntryTrait, SquashedFelt252DictTrait};
use core::fmt::{Debug, Error, Formatter};
use core::nullable::{FromNullableResult, match_nullable};
use core::num::traits::DivRem;
use core::option::OptionTrait;
use crate::BaseField;
use crate::utils::{SpanExTrait, pow2};
use crate::vcs::hasher::MerkleHasher;

pub struct MerkleVerifier<impl H: MerkleHasher> {
    /// The root of the Merkle tree being verified
    pub root: H::Hash,
    // The height of the Merkle tree.
    //
    // The height can be computed as log_blowup_factor + column_log_deg_bounds.max().
    pub tree_height: u32,
    // The log degree bounds of the committed columns.
    pub column_log_deg_bounds: Span<u32>,
}
impl MerkleVerifierDrop<impl H: MerkleHasher, +Drop<H::Hash>> of Drop<MerkleVerifier<H>>;

pub struct MerkleDecommitment<impl H: MerkleHasher> {
    /// Hash values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed.
    pub hash_witness: Span<H::Hash>,
}

impl MerkleDecommitmentDrop<impl H: MerkleHasher, +Drop<H::Hash>> of Drop<MerkleDecommitment<H>>;

impl MerkleDecommitmentDebug<
    impl H: MerkleHasher, +Debug<H::Hash>,
> of Debug<MerkleDecommitment<H>> {
    fn fmt(self: @MerkleDecommitment<H>, ref f: Formatter) -> Result<(), Error> {
        Ok(())
    }
}

impl MerkleDecommitmentClone<
    impl H: MerkleHasher, +Clone<Array<H::Hash>>, +Drop<Array<H::Hash>>,
> of Clone<MerkleDecommitment<H>> {
    fn clone(self: @MerkleDecommitment<H>) -> MerkleDecommitment<H> {
        MerkleDecommitment::<H> { hash_witness: *self.hash_witness }
    }
}

impl MerkleDecommitmentSerde<
    impl H: MerkleHasher, +Serde<Span<H::Hash>>,
> of Serde<MerkleDecommitment<H>> {
    fn serialize(self: @MerkleDecommitment<H>, ref output: Array<felt252>) {
        self.hash_witness.serialize(ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<MerkleDecommitment<H>> {
        Some(MerkleDecommitment { hash_witness: Serde::deserialize(ref serialized)? })
    }
}

pub trait MerkleVerifierTrait<impl H: MerkleHasher> {
    /// Verifies the decommitment of the columns at some query positions.
    /// .
    ///
    /// # Arguments
    ///
    /// * `query_positions` - Vector of query positions with respect to the largest layer. Note that
    /// `query_positions` is not necessarily sorted and may contain duplicates.
    /// * `queried_values` - An array of queried values, of length (number of columns) x (number of
    /// queries). For each query position, there is a row of values, one for each column, at the
    /// index corresponding to the query position.
    /// * `decommitment` - The decommitment object containing the hash witness.
    fn verify(
        self: @MerkleVerifier<H>,
        query_positions: Span<u32>,
        queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    );
}

impl MerkleVerifierImpl<
    impl H: MerkleHasher, +Clone<H::Hash>, +Drop<H::Hash>, +PartialEq<H::Hash>,
> of MerkleVerifierTrait<H> {
    /// Verifies openings of a Merkle commitment at positions given by `query_positions`. Expects
    /// a Merkle commitment to a collection of column vectors of the same length, whose leaves
    /// are the hashes of the rows.
    fn verify(
        self: @MerkleVerifier<H>,
        query_positions: Span<u32>,
        mut queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    ) {
        let n_columns = self.column_log_deg_bounds.len();
        let mut positions_and_values: Felt252Dict<Nullable<Span<BaseField>>> = Default::default();

        for pos in query_positions {
            let column_values = queried_values.pop_front_n(n_columns);
            let (entry, maybe_present_values) = positions_and_values.entry((*pos).into());
            // Check that queried values corresponding to the same position have the same
            // values.
            match match_nullable(maybe_present_values) {
                FromNullableResult::Null => (),
                FromNullableResult::NotNull(present_values) => assert!(
                    present_values.unbox() == column_values,
                    "Queried values at same positions are inconsistent.",
                ),
            }
            positions_and_values = entry.finalize(NullableTrait::new(column_values));
        }
        assert!(queried_values.is_empty());

        // This buffer is filled with the sorted+deduplicated positions and the hash
        // of the corresponding queried_values. The first element of each tuple encodes
        // in its MSB the height of the associated layer in the Merkle tree. For example,
        // a query in position k in the leaf layer is encoded as (2^tree_height + k).
        let mut positions_and_hashes: Array<(usize, H::Hash)> = array![];
        let layer_idx = pow2(*self.tree_height);

        // A squashed dict's entries are sorted by key in ascending order.
        for (pos, _, queried_values) in positions_and_values.squash().into_entries() {
            let position_with_msb = pos.try_into().unwrap() + layer_idx;
            positions_and_hashes
                .append((position_with_msb, H::hash_node(None, queried_values.deref())));
        }

        // At this point `positions_and_hashes` contains the query positions and their
        // values, for the bottom layer of the Merkle tree (i.e. the leaves).
        let MerkleDecommitment { mut hash_witness } = decommitment;
        let computed_root = loop {
            let (child_position, child_hash) = positions_and_hashes.pop_front().unwrap();
            let (parent_position, parity) = child_position.div_rem(2);

            if parent_position == 0 {
                break child_hash;
            }

            if parity == 1 {
                // If `child_position` is odd, we know that the sibling was not calculated before.
                // Consume the hash_witness.
                let parent_hash = H::hash_node(
                    Some((hash_witness.pop_front().unwrap().clone(), child_hash.clone())),
                    array![].span(),
                );
                positions_and_hashes.append((parent_position, parent_hash));
                continue;
            }

            // If `child_position` is even, we need to check if the sibling is in the
            // previous layer. This happens if: 1. `positions_and_hashes` is non-empty 2.
            // the first element of `positions_and_hashes` is indeed the sibling. If any of these
            // conditions is not satisfied, we need to consume the witness.
            let sibling_hash = if let Some((maybe_sibling_position, maybe_sibling_hash)) =
                positions_and_hashes
                .span()
                .first() {
                if *maybe_sibling_position == child_position + 1 {
                    // Consume the buffer.
                    let _ = positions_and_hashes.pop_front();
                    maybe_sibling_hash
                } else {
                    // Consume the hash_witness.
                    hash_witness.pop_front().unwrap()
                }
            } else {
                // Consume the hash_witness.
                hash_witness.pop_front().unwrap()
            };

            let parent_hash = H::hash_node(
                Some((child_hash.clone(), sibling_hash.clone())), array![].span(),
            );
            positions_and_hashes.append((parent_position, parent_hash));
        };

        // Check that the witness has been consumed.
        assert!(hash_witness.is_empty(), "{}", MerkleVerificationError::WitnessTooLong);
        assert!(@computed_root == self.root, "{}", MerkleVerificationError::RootMismatch);
    }
}

#[derive(Drop, Debug)]
pub enum MerkleVerificationError {
    WitnessTooShort,
    WitnessTooLong,
    RootMismatch,
}

impl MerkleVerificationErrorDisplay of core::fmt::Display<MerkleVerificationError> {
    fn fmt(
        self: @MerkleVerificationError, ref f: core::fmt::Formatter,
    ) -> Result<(), core::fmt::Error> {
        match self {
            MerkleVerificationError::WitnessTooShort => write!(
                f, "Merkle Verification Error: Witness Too Short",
            ),
            MerkleVerificationError::WitnessTooLong => write!(
                f, "Merkle Verification Error: Witness Too Long",
            ),
            MerkleVerificationError::RootMismatch => write!(
                f, "Merkle Verification Error: Root Mismatch",
            ),
        }
    }
}

use core::array::{ArrayTrait, SpanTrait, ToSpanTrait};
use core::fmt::{Debug, Error, Formatter};
use core::num::traits::DivRem;
use core::option::OptionTrait;
use crate::BaseField;
use crate::utils::SpanExTrait;
use crate::vcs::hasher::MerkleHasher;
use crate::vcs::verifier::MerkleVerifier;


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

pub trait MerkleVerifierLiftedTrait<impl H: MerkleHasher> {
    /// Verifies the decommitment of the columns at some query positions.
    /// .
    ///
    /// # Arguments
    ///
    /// * `query_positions` - Vector of query positions with respect to the largest layer, in
    /// non-decreasing order (in particular, there may be duplicates).
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
> of MerkleVerifierLiftedTrait<H> {
    /// Verifies openings of a Merkle commitment at positions given by `query_positions`. Expects
    /// a Merkle commitment to a collection of column vectors of the same length, whose leaves
    /// are the hashes of the rows.
    fn verify(
        self: @MerkleVerifier<H>,
        query_positions: Span<u32>,
        mut queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    ) {
        let n_columns = self
            .column_indices_by_log_deg_bound
            .into_iter()
            .fold(0, |acc, c| acc + c.len());
        // These two buffers are first filled with the deduplicated query_positions and the hash
        // of the corresponding queried_values.
        let mut prev_layer_queries: Array<usize> = array![];
        let mut prev_layer_hashes: Array<H::Hash> = array![];

        let mut query_positions_iter = query_positions.into_iter();
        // We deduplicate the queries: first, we get the first query and the first queried values;
        // then, we deduplicate the remaining positions and values in a loop.
        let mut prev_pos = query_positions_iter.next().unwrap();
        let mut prev_queried_values = queried_values.pop_front_n(n_columns);
        prev_layer_queries.append(*prev_pos);
        prev_layer_hashes.append(H::hash_node(None, prev_queried_values));

        for pos in query_positions_iter {
            let column_values = queried_values.pop_front_n(n_columns);
            if prev_pos == pos {
                // Check that queried values corresponding to the same position have the same
                // values.
                assert!(
                    prev_queried_values == column_values,
                    "Queried values at same positions are inconsistent.",
                );
            } else {
                prev_layer_queries.append(*pos);
                prev_layer_hashes.append(H::hash_node(None, column_values));
            }
            prev_pos = pos;
            prev_queried_values = column_values;
        }
        assert!(queried_values.is_empty());

        // At this point `prev_layer_queries` and `prev_layer_hashea` contain, respectively, the
        // query positions and their values, for the bottom layer of the Merkle tree (i.e. the
        // leaves).
        let mut prev_layer_queries: Span<usize> = prev_layer_queries.span();
        let mut prev_layer_hashes: Span<H::Hash> = prev_layer_hashes.span();
        let MerkleDecommitment { mut hash_witness } = decommitment;
        for _ in 0..*self.tree_height {
            let mut curr_layer_queries: Array<u32> = array![];
            let mut curr_layer_hashes: Array<H::Hash> = array![];
            // The arrays `prev_layer_queries` and `prev_layer_hashes` are consumed in lock step.
            while let Some(idx) = prev_layer_queries.pop_front() {
                let child_hash = prev_layer_hashes.pop_front().unwrap();
                // Check if the `prev_layer_hashes` contains the sibling of child_hash, else take it
                // from the witness.
                let sibling_hash: @H::Hash = if prev_layer_queries
                    .next_if_eq(@(*idx ^ 1))
                    .is_some() {
                    prev_layer_hashes.pop_front().unwrap()
                } else {
                    hash_witness.pop_front().unwrap()
                };

                // Check in which order we need to hash the parents.
                let (_, parity) = idx.div_rem(2);
                let parent_hash = if parity == 0 {
                    H::hash_node(Some((child_hash.clone(), sibling_hash.clone())), array![].span())
                } else {
                    H::hash_node(Some((sibling_hash.clone(), child_hash.clone())), array![].span())
                };

                curr_layer_queries.append(*idx / 2);
                curr_layer_hashes.append(parent_hash);
            }
            prev_layer_queries = curr_layer_queries.span();
            prev_layer_hashes = curr_layer_hashes.span();
        }

        // Check that the witness has been consumed.
        assert!(hash_witness.is_empty(), "{}", MerkleVerificationError::WitnessTooLong);
        let computed_root = prev_layer_hashes.pop_front().unwrap();
        assert!(prev_layer_hashes.is_empty());
        assert!(computed_root == self.root, "{}", MerkleVerificationError::RootMismatch);
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

use core::array::{ArrayTrait, SpanTrait, ToSpanTrait};
use core::cmp::min;
use core::dict::{Felt252Dict, Felt252DictEntryTrait, Felt252DictTrait};
use core::fmt::{Debug, Error, Formatter};
use core::nullable::NullableTrait;
use core::option::OptionTrait;
use crate::BaseField;
use crate::utils::{ArrayExTrait, DictTrait, SpanExTrait};
use crate::vcs::hasher::MerkleHasher;

pub struct MerkleDecommitment<impl H: MerkleHasher> {
    /// Hash values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed.
    pub hash_witness: Span<H::Hash>,
    /// Column values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed.
    /// This complements the column values that were queried. These must be supplied directly to
    /// the verifier.
    pub column_witness: Span<BaseField>,
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
        MerkleDecommitment::<
            H,
        > { hash_witness: self.hash_witness.clone(), column_witness: self.column_witness.clone() }
    }
}

impl MerkleDecommitmentSerde<
    impl H: MerkleHasher, +Serde<Span<H::Hash>>,
> of Serde<MerkleDecommitment<H>> {
    fn serialize(self: @MerkleDecommitment<H>, ref output: Array<felt252>) {
        self.hash_witness.serialize(ref output);
        self.column_witness.serialize(ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<MerkleDecommitment<H>> {
        Some(
            MerkleDecommitment {
                hash_witness: Serde::deserialize(ref serialized)?,
                column_witness: Serde::deserialize(ref serialized)?,
            },
        )
    }
}

pub struct MerkleVerifier<impl H: MerkleHasher> {
    pub root: H::Hash,
    pub column_log_sizes: Array<u32>,
    pub columns_by_log_size: Span<Span<usize>>,
}
impl MerkleVerifierDrop<impl H: MerkleHasher, +Drop<H::Hash>> of Drop<MerkleVerifier<H>>;

pub trait MerkleVerifierTrait<impl H: MerkleHasher> {
    /// Verifies the decommitment of the columns.
    ///
    /// # Arguments
    ///
    /// * `queries_per_log_size` - A map from log_size to a vector of queries for columns of that
    ///  log_size.
    /// * `queried_values` - An array of spans of queried values. For each column, there is a
    /// span of queried values to that column.
    /// * `decommitment` - The decommitment object containing the witness and column values.
    ///
    /// # Panics
    ///
    /// Panics if any of the following conditions are met:
    ///
    /// * The witness is too long (not fully consumed).
    /// * The witness is too short (missing values).
    /// * The column values are too long (not fully consumed).
    /// * The column values are too short (missing values).
    /// * The computed root does not match the expected root.
    fn verify(
        self: @MerkleVerifier<H>,
        queries_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
        queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    );
}

impl MerkleVerifierImpl<
    impl H: MerkleHasher, +Clone<H::Hash>, +Drop<H::Hash>, +PartialEq<H::Hash>,
> of MerkleVerifierTrait<H> {
    fn verify(
        self: @MerkleVerifier<H>,
        mut queries_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
        mut queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    ) {
        let MerkleDecommitment { mut hash_witness, mut column_witness } = decommitment;

        let mut columns_by_log_size = self.columns_by_log_size.clone();
        let mut layer_log_size: felt252 = columns_by_log_size.len().into();
        let mut prev_layer_hashes: Array<(usize, H::Hash)> = array![];
        let mut is_first_layer = true;

        while let Some(layer_cols) = columns_by_log_size.pop_back() {
            let n_columns_in_layer = layer_cols.len();
            layer_log_size -= 1;

            // Prepare write buffer for queries to the current layer. This will propagate to the
            // next layer.
            let mut layer_total_queries = array![];

            // Prepare read buffer for queried values to the current layer.

            // Extract the requested queries to the current layer.
            let mut layer_column_queries = queries_per_log_size
                .get(layer_log_size)
                .deref_or(array![].span());

            // Merge previous layer queries and column queries.
            while let Some(current_query) =
                next_decommitment_node(layer_column_queries, prev_layer_hashes.span()) {
                let node_hashes = if is_first_layer {
                    None
                } else {
                    let left_hash =
                        match fetch_prev_node_hash(
                            ref prev_layer_hashes, ref hash_witness, current_query * 2,
                        ) {
                        Some(val) => val,
                        None => panic!("{}", MerkleVerificationError::WitnessTooShort),
                    };

                    let right_hash =
                        match fetch_prev_node_hash(
                            ref prev_layer_hashes, ref hash_witness, current_query * 2 + 1,
                        ) {
                        Some(val) => val,
                        None => panic!("{}", MerkleVerificationError::WitnessTooShort),
                    };

                    Some((left_hash.clone(), right_hash.clone()))
                };

                // If the column values were queried, read them from `queried_value`.
                let column_values = if layer_column_queries.next_if_eq(@current_query).is_some() {
                    queried_values.pop_front_n(n_columns_in_layer)
                } else {
                    column_witness.pop_front_n(n_columns_in_layer)
                };

                assert!(
                    column_values.len() == n_columns_in_layer,
                    "{}",
                    MerkleVerificationError::WitnessTooShort,
                );

                layer_total_queries
                    .append((current_query, H::hash_node(node_hashes, column_values)));
            }

            prev_layer_hashes = layer_total_queries;
            is_first_layer = false;
        }

        // Check that all witnesses and values have been consumed.
        assert!(hash_witness.is_empty(), "{}", MerkleVerificationError::WitnessTooLong);
        assert!(column_witness.is_empty(), "{}", MerkleVerificationError::WitnessTooLong);

        let (_, computed_root) = prev_layer_hashes.pop_front().unwrap();

        if @computed_root != self.root {
            panic!("{}", MerkleVerificationError::RootMismatch);
        }
    }
}

fn next_decommitment_node<H>(
    layer_queries: Span<u32>, prev_queries: Span<(u32, H)>,
) -> Option<usize> {
    // Fetch the next query.
    let layer_query_head = layer_queries.first();
    let prev_query_head = if let Some((prev_query, _)) = prev_queries.first() {
        Some(*prev_query / 2)
    } else {
        None
    };

    match (layer_query_head, prev_query_head) {
        (None, None) => { None },
        (Some(column_query), None) => { Some(*column_query) },
        (None, Some(prev_query)) => { Some(prev_query) },
        (Some(column_query), Some(prev_query)) => { Some(min(*column_query, prev_query)) },
    }
}

/// Fetches the hash of the next node from the previous layer in the Merkle tree.
/// The hash is fetched either from the computed values or from the witness.
#[inline]
fn fetch_prev_node_hash<H, +Clone<H>, +Drop<H>>(
    ref prev_layer_hashes: Array<(u32, H)>, ref hash_witness: Span<H>, expected_query: u32,
) -> Option<@H> {
    // If the child was computed, use that value.
    let mut prev_layer_hashes_span = prev_layer_hashes.span();
    if let Some((q, h)) = prev_layer_hashes_span.pop_front() {
        if *q == expected_query {
            let _ = prev_layer_hashes.pop_front();
            return Some(h);
        }
    }
    // If the child was not computed, read it from the witness.
    hash_witness.pop_front()
}

#[derive(Drop, Debug)]
pub enum MerkleVerificationError {
    WitnessTooShort,
    WitnessTooLong,
    ColumnValuesTooLong,
    ColumnValuesTooShort,
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
            MerkleVerificationError::ColumnValuesTooLong => write!(
                f, "Merkle Verification Error: Column Values Too Long",
            ),
            MerkleVerificationError::ColumnValuesTooShort => write!(
                f, "Merkle Verification Error: Column Values Too Short",
            ),
            MerkleVerificationError::RootMismatch => write!(
                f, "Merkle Verification Error: Root Mismatch",
            ),
        }
    }
}

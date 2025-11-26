use core::array::{ArrayTrait, SpanTrait, ToSpanTrait};
use core::dict::{Felt252Dict, Felt252DictTrait};
use core::fmt::{Debug, Error, Formatter};
use core::nullable::NullableTrait;
use core::option::OptionTrait;
use crate::BaseField;
use crate::utils::{ColumnsIndicesByLogDegreeBound, SpanExTrait};
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
        > { hash_witness: *self.hash_witness, column_witness: *self.column_witness }
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
    /// The root of the Merkle tree being verified
    pub root: H::Hash,
    // The height of the Merkle tree.
    //
    // The height can be computed as log_blowup_factor + column_indices_by_log_deg_bound.len() - 1.
    pub tree_height: u32,
    /// Indices of columns, grouped by their associated degree bound.
    ///
    /// While the MerkleVerifier itself only needs to know the number of columns for each degree
    /// bound, we store the full list of indices here because the Polynomial Commitment Scheme (PCS)
    /// verifier requires access to the actual indices. Keeping this information here avoids the
    /// need to save us computing 'n_column_by_log_deg_bound' when creating the MerkleVerifier.
    pub column_indices_by_log_deg_bound: ColumnsIndicesByLogDegreeBound,
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
        ref queries_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
        queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    );
}

impl MerkleVerifierImpl<
    impl H: MerkleHasher, +Clone<H::Hash>, +Drop<H::Hash>, +PartialEq<H::Hash>,
> of MerkleVerifierTrait<H> {
    /// Verifies openings of a Merkle commitment at positions given by `queries_per_log_size`.
    ///
    /// The current implementation only supports verification of query positions such that
    /// the set of query indices in a given column contains at least all the foldings of
    /// query indices in longer columns.
    /// This assumption implies that the `column_witness` in `decommitment` is empty.
    fn verify(
        self: @MerkleVerifier<H>,
        ref queries_per_log_size: Felt252Dict<Nullable<Span<usize>>>,
        mut queried_values: Span<BaseField>,
        decommitment: MerkleDecommitment<H>,
    ) {
        let MerkleDecommitment { mut hash_witness, mut column_witness } = decommitment;

        let mut column_indices_by_log_deg_bound = *self.column_indices_by_log_deg_bound;
        let mut layer_log_size: felt252 = (*self.tree_height).into();
        let mut prev_layer_hashes: Array<(usize, H::Hash)> = array![];

        let layer_cols = column_indices_by_log_deg_bound.pop_back().unwrap();
        let layer_column_queries = queries_per_log_size
            .get(layer_log_size)
            .deref_or(array![].span());

        let n_columns_in_layer = layer_cols.len();
        assert!(n_columns_in_layer != 0);
        for current_query in layer_column_queries {
            let column_values = queried_values.pop_front_n(n_columns_in_layer);

            prev_layer_hashes.append((*current_query, H::hash_node(None, column_values)));
        }

        while layer_log_size != 0 {
            layer_log_size -= 1;
            // `None` happens only in the last `log_blowup_factor` layers.
            let n_columns_in_layer = match column_indices_by_log_deg_bound.pop_back() {
                Some(layer_cols) => layer_cols.len(),
                None => 0,
            };

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
                let left_child_index = current_query * 2;
                let left_hash = fetch_prev_node_hash(
                    ref prev_layer_hashes, ref hash_witness, left_child_index,
                );
                let right_hash = fetch_prev_node_hash(
                    ref prev_layer_hashes, ref hash_witness, left_child_index + 1,
                );
                let node_hashes = Some((left_hash.clone(), right_hash.clone()));

                let column_values = if layer_column_queries.next_if_eq(@current_query).is_some() {
                    queried_values.pop_front_n(n_columns_in_layer)
                } else {
                    column_witness.pop_front_n(n_columns_in_layer)
                };

                layer_total_queries
                    .append((current_query, H::hash_node(node_hashes, column_values)));
            }

            prev_layer_hashes = layer_total_queries;
        }

        // Check that all witnesses and values have been consumed.
        assert!(hash_witness.is_empty(), "{}", MerkleVerificationError::WitnessTooLong);
        assert!(column_witness.is_empty(), "{}", MerkleVerificationError::WitnessTooLong);
        let (_, computed_root) = prev_layer_hashes.pop_front().unwrap();
        assert!(prev_layer_hashes.is_empty());
        assert!(queried_values.is_empty());

        assert!(@computed_root == self.root, "{}", MerkleVerificationError::RootMismatch);
    }
}

fn next_decommitment_node<H>(
    layer_queries: Span<u32>, prev_queries: Span<(u32, H)>,
) -> Option<usize> {
    let Some((prev_query, _)) = prev_queries.first() else {
        return layer_queries.first().map(|v| *v);
    };

    let next_query = *prev_query / 2;
    let Some(layer_query_head) = layer_queries.first() else {
        return Some(next_query);
    };

    if *layer_query_head < next_query {
        Some(*layer_query_head)
    } else {
        Some(next_query)
    }
}

/// Fetches the hash of the next node from the previous layer in the Merkle tree.
/// The hash is fetched either from the computed values or from the witness.
///
/// # Panics
///
/// Panics if the required hash should be fetched from `hash_witness` but `hash_witness`
/// is empty.
#[inline]
fn fetch_prev_node_hash<H, +Clone<H>, +Drop<H>>(
    ref prev_layer_hashes: Array<(u32, H)>, ref hash_witness: Span<H>, expected_query: u32,
) -> @H {
    // If the child was computed, use that value.
    if let Some((q, h)) = prev_layer_hashes.span().first() && *q == expected_query {
        let _ = prev_layer_hashes.pop_front();
        return h;
    }
    // If the child was not computed, read it from the witness. Panics if the witness
    // is already empty.
    hash_witness
        .pop_front()
        .unwrap_or_else(|| panic!("{}", MerkleVerificationError::WitnessTooShort))
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

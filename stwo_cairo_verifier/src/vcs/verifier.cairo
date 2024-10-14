use core::dict::Felt252DictTrait;
use core::result::ResultTrait;
use stwo_cairo_verifier::utils::SpanExTrait;
use core::option::OptionTrait;
use core::array::ArrayTrait;
use core::array::SpanTrait;
use core::array::ToSpanTrait;
use core::dict::Felt252Dict;
use core::dict::Felt252DictEntryTrait;
use core::nullable::NullableTrait;
use core::cmp::min;
use core::fmt::{Debug, Formatter, Error};
use stwo_cairo_verifier::BaseField;
use stwo_cairo_verifier::fields::m31::m31;
use stwo_cairo_verifier::utils::{ArrayExTrait, DictTrait, OptBoxTrait};
use stwo_cairo_verifier::vcs::hasher::MerkleHasher;

pub struct MerkleDecommitment<impl H: MerkleHasher> {
    /// Hash values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed.
    pub hash_witness: Array<H::Hash>,
    /// Column values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed.
    /// This complements the column values that were queried. These must be supplied directly to
    /// the verifier.
    pub column_witness: Array<BaseField>,
}
impl MerkleDecommitmentDrop<impl H: MerkleHasher, +Drop<H::Hash>> of Drop<MerkleDecommitment<H>>;

impl MerkleDecommitmentDebug<
    impl H: MerkleHasher, +Debug<H::Hash>
> of Debug<MerkleDecommitment<H>> {
    fn fmt(self: @MerkleDecommitment<H>, ref f: Formatter) -> Result<(), Error> {
        Result::Ok(())
    }
}

impl MerkleDecommitmentClone<
    impl H: MerkleHasher, +Clone<Array<H::Hash>>, +Drop<Array<H::Hash>>
> of Clone<MerkleDecommitment<H>> {
    fn clone(self: @MerkleDecommitment<H>) -> MerkleDecommitment<H> {
        MerkleDecommitment::<
            H
        > { hash_witness: self.hash_witness.clone(), column_witness: self.column_witness.clone() }
    }
}

pub struct MerkleVerifier<impl H: MerkleHasher> {
    pub root: H::Hash,
    pub column_log_sizes: Array<u32>,
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
    /// # Errors
    ///
    /// Returns an error if any of the following conditions are met:
    ///
    /// * The witness is too long (not fully consumed).
    /// * The witness is too short (missing values).
    /// * The column values are too long (not fully consumed).
    /// * The column values are too short (missing values).
    /// * The computed root does not match the expected root.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the decommitment is successfully verified.
    fn verify(
        self: @MerkleVerifier<H>,
        queries_per_log_size: Felt252Dict<Nullable<Array<usize>>>,
        queried_values: Array<Span<BaseField>>,
        decommitment: MerkleDecommitment<H>,
    ) -> Result<(), MerkleVerificationError>;
    fn cols_by_size(self: @MerkleVerifier<H>) -> Felt252Dict<Nullable<Array<u32>>>;
}

impl MerkleVerifierImpl<
    impl H: MerkleHasher, +Copy<H::Hash>, +Drop<H::Hash>, +PartialEq<H::Hash>
> of MerkleVerifierTrait<H> {
    fn verify(
        self: @MerkleVerifier<H>,
        mut queries_per_log_size: Felt252Dict<Nullable<Array<usize>>>,
        queried_values: Array<Span<BaseField>>,
        mut decommitment: MerkleDecommitment<H>,
    ) -> Result<(), MerkleVerificationError> {
        let MerkleDecommitment::<
            H
        > { hash_witness: mut hash_witness, column_witness: mut column_witness, } =
            decommitment;

        let queried_values = @queried_values;
        let mut layer_log_size = *self.column_log_sizes.max().unwrap();
        let mut cols_by_size = Self::cols_by_size(self);

        let mut prev_layer_hashes: Array<(usize, H::Hash)> = array![];
        let mut is_first_layer = true;
        loop {
            // Prepare write buffer for queries to the current layer. This will propagate to the
            // next layer.
            let mut layer_total_queries = array![];

            // Prepare read buffer for queried values to the current layer.
            let mut layer_cols = cols_by_size
                .replace(layer_log_size.into(), Default::default())
                .deref_or(array![]);
            let n_columns_in_layer = layer_cols.len();

            let mut layer_queried_values = array![];

            while let Option::Some(column_index) = layer_cols.pop_front() {
                layer_queried_values.append(*queried_values[column_index]);
            };

            let layer_queried_values = @layer_queried_values;

            // Extract the requested queries to the current layer.
            let mut col_query_index: u32 = 0;
            let mut layer_column_queries = queries_per_log_size
                .replace(layer_log_size.into(), Default::default(),)
                .deref_or(array![])
                .span();

            // Merge previous layer queries and column queries.
            let res = loop {
                // Fetch the next query.
                let current_query = if let Option::Some(current_query) =
                    next_decommitment_node(layer_column_queries, @prev_layer_hashes) {
                    current_query
                } else {
                    break Result::Ok(());
                };

                let node_hashes = if is_first_layer {
                    Option::None
                } else {
                    let left_hash = if let Option::Some(val) =
                        fetch_prev_node_hash(
                            ref prev_layer_hashes, ref hash_witness, current_query * 2
                        ) {
                        val
                    } else {
                        break Result::Err(MerkleVerificationError::WitnessTooShort);
                    };

                    let right_hash = if let Option::Some(val) =
                        fetch_prev_node_hash(
                            ref prev_layer_hashes, ref hash_witness, current_query * 2 + 1
                        ) {
                        val
                    } else {
                        break Result::Err(MerkleVerificationError::WitnessTooShort);
                    };
                    Option::Some((left_hash, right_hash))
                };

                // If the column values were queried, read them from `queried_value`.
                let column_values = if layer_column_queries.next_if_eq(@current_query).is_some() {
                    let mut res = array![];
                    let mut i = 0;
                    while i != n_columns_in_layer {
                        let queried_column = layer_queried_values[i];
                        res.append(*queried_column[col_query_index]);
                        i += 1;
                    };
                    col_query_index += 1;
                    res
                } else {
                    column_witness.pop_n(n_columns_in_layer)
                };

                if column_values.len() != n_columns_in_layer {
                    break Result::Err(MerkleVerificationError::WitnessTooShort);
                }

                layer_total_queries
                    .append((current_query, H::hash_node(node_hashes, column_values)));
            };
            if let Result::Err(err) = res {
                break Result::Err(err);
            }

            prev_layer_hashes = layer_total_queries;
            if layer_log_size == 0 {
                break Result::Ok(());
            }
            is_first_layer = false;
            layer_log_size -= 1;
        }?;

        // Check that all witnesses and values have been consumed.
        if !hash_witness.is_empty() {
            return Result::Err(MerkleVerificationError::WitnessTooLong);
        }
        if !column_witness.is_empty() {
            return Result::Err(MerkleVerificationError::WitnessTooLong);
        }

        let (_, computed_root) = prev_layer_hashes.pop_front().unwrap();

        if @computed_root != self.root {
            return Result::Err(MerkleVerificationError::RootMismatch);
        }

        Result::Ok(())
    }

    fn cols_by_size(self: @MerkleVerifier<H>) -> Felt252Dict<Nullable<Array<u32>>> {
        let mut column_log_sizes = self.column_log_sizes.span();
        let mut res_dict = Default::default();
        let mut col_index = 0;
        let mut max_size = 0;
        while !column_log_sizes.is_empty() {
            let col_size = *column_log_sizes.pop_front().unwrap();
            if col_size > max_size {
                max_size = col_size;
            }
            let (res_dict_entry, value) = res_dict.entry(col_size.into());
            let mut value = value.deref_or(array![]);
            value.append(col_index);
            res_dict = res_dict_entry.finalize(NullableTrait::new(value));
            col_index += 1;
        };

        res_dict
    }
}

fn next_decommitment_node<H>(
    layer_queries: Span<u32>, prev_queries: @Array<(u32, H)>,
) -> Option<usize> {
    // Fetch the next query.
    let layer_query_head = layer_queries.get(0).as_unboxed();
    let prev_query_head = if let Option::Some((prev_query, _)) = prev_queries.get(0).as_unboxed() {
        Option::Some(*prev_query / 2)
    } else {
        Option::None
    };

    match (layer_query_head, prev_query_head) {
        (Option::None, Option::None) => { Option::None },
        (Option::Some(column_query), Option::None) => { Option::Some(*column_query) },
        (Option::None, Option::Some(prev_query)) => { Option::Some(prev_query) },
        (
            Option::Some(column_query), Option::Some(prev_query)
        ) => { Option::Some(min(*column_query, prev_query)) },
    }
}

/// Fetches the hash of the next node from the previous layer in the Merkle tree.
/// The hash is fetched either from the computed values or from the witness.
fn fetch_prev_node_hash<H, +Copy<H>, +Drop<H>>(
    ref prev_layer_hashes: Array<(u32, H)>, ref hash_witness: Array<H>, expected_query: u32
) -> Option<H> {
    // If the child was computed, use that value.
    if let Option::Some((q, h)) = prev_layer_hashes.get(0).as_unboxed() {
        if *q == expected_query {
            prev_layer_hashes.pop_front().unwrap();
            return Option::Some(*h);
        }
    }
    // If the child was not computed, read it from the witness.
    if let Option::Some(h) = hash_witness.pop_front() {
        return Option::Some(h);
    }
    Option::None
}

#[derive(Copy, Drop, Debug)]
pub enum MerkleVerificationError {
    WitnessTooShort,
    WitnessTooLong,
    ColumnValuesTooLong,
    ColumnValuesTooShort,
    RootMismatch,
}


#[test]
fn test_verifier() {
    let root = 0x06e3a2499c5ee8a2a66f536f30640b9b67cb50092642003b64a60c401e280214;
    let column_log_sizes = array![4, 3, 4, 3, 3, 3, 4, 4, 3, 3];
    let decommitment = MerkleDecommitment {
        hash_witness: array![
            0x037056abc40b9e8c2a67826f54a8c379b0b3ef46629e6a19609e1144bf230f36,
            0x068708ce1c3fc019a43494bd262e87fc70e5c1f68f42881f120fe90ea2bf2201,
            0x01270a97c943188a4aa8a839687ff6d2681b070d1d1627466b93843ad26f4cb2,
            0x06be4322e37fe02371c14436674765da25109e9bc3af4a683c9afea63eb3bdc3,
            0x0360c78816d1d60758c67c011dcd82396a2ccf85fe49ea45667e3cb9feca3f40,
            0x01b4e5f9533e652324ab6b5747edc3343db8f1b9432cdcf2e5ea54fa156ba483,
            0x04a389ddc8e37da68b73c185460f372a5ed8a09eab0f51c63578776db8d1b5ae,
            0x03adfd255329a9a3d49792362f34630fd6b04cc7efdb3a6a175c70b988915cdc,
        ],
        column_witness: array![
            m31(885772305),
            m31(94648313),
            m31(604384470),
            m31(957953858),
            m31(608524802),
            m31(428382412),
        ]
    };
    let mut queries_per_log_size = Default::default();
    queries_per_log_size.insert(3, NullableTrait::new(array![2, 5, 7]));
    queries_per_log_size.insert(4, NullableTrait::new(array![7, 11, 14]));
    let queried_values = array![
        array![m31(720125469), m31(997644238), m31(194302184)].span(),
        array![m31(122725140), m31(840979908), m31(658446453)].span(),
        array![m31(968171809), m31(100529415), m31(1057594968)].span(),
        array![m31(1012109813), m31(428994537), m31(992269493)].span(),
        array![m31(766295003), m31(28706943), m31(967997322)].span(),
        array![m31(552345729), m31(696999129), m31(287489501)].span(),
        array![m31(364669117), m31(933029034), m31(285391207)].span(),
        array![m31(996158769), m31(69309287), m31(420798739)].span(),
        array![m31(650584843), m31(942699537), m31(310081088)].span(),
        array![m31(71167745), m31(330264928), m31(409791388)].span()
    ];
    MerkleVerifier { root, column_log_sizes, }
        .verify(queries_per_log_size, queried_values, decommitment,)
        .expect('verification failed');
}

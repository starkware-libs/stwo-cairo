use core::array::SpanIter;
use core::dict::Felt252Dict;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::{CheckedSub, Zero};
use crate::channel::{Channel, ChannelImpl};
use crate::circle::CosetImpl;
use crate::fields::BatchInvertible;
use crate::fields::qm31::{QM31, QM31Serde, QM31Trait, QM31_EXTENSION_DEGREE};
use crate::poly::circle::{CanonicCosetImpl, CircleDomain, CircleDomainImpl};
use crate::poly::line::{LineDomain, LineDomainImpl, LineEvaluationImpl, LinePoly, LinePolyImpl};
use crate::poly::utils::ibutterfly;
use crate::queries::{Queries, QueriesImpl};
use crate::utils::{ArrayImpl, OptionImpl, SpanExTrait, bit_reverse_index, pow2};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use crate::{ColumnArray, Hash};

/// Fold step size for circle polynomials.
pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;

/// Equals `2^CIRCLE_TO_LINE_FOLD_STEP`.
pub const CIRCLE_TO_LINE_FOLD_FACTOR: usize = 2;

/// Fold step size for univariate polynomials.
pub const FOLD_STEP: u32 = 1;

/// Equals `2^FOLD_STEP`.
pub const FOLD_FACTOR: usize = 2;

#[derive(Drop, Serde)]
pub struct FriConfig {
    pub log_blowup_factor: u32,
    pub log_last_layer_degree_bound: u32,
    pub n_queries: usize,
}

#[derive(Drop)]
pub struct FriVerifier {
    config: FriConfig,
    first_layer: FriFirstLayerVerifier,
    inner_layers: Array<FriInnerLayerVerifier>,
    last_layer_domain: LineDomain,
    last_layer_poly: LinePoly,
    queries: Option<Queries>,
}

#[generate_trait]
pub impl FriVerifierImpl of FriVerifierTrait {
    /// Verifies the commitment stage of FRI.
    ///
    /// `column_log_bounds` should be the committed circle polynomial log
    /// degree bounds in descending order.
    fn commit(
        ref channel: Channel, config: FriConfig, proof: FriProof, column_log_bounds: Span<u32>,
    ) -> Result<FriVerifier, FriVerificationError> {
        let FriProof {
            first_layer: first_layer_proof, inner_layers: mut inner_layer_proofs, last_layer_poly,
        } = proof;

        channel.mix_root(first_layer_proof.commitment.clone());

        let mut column_commitment_domains = array![];

        for column_log_bound in column_log_bounds {
            let commitment_domain_log_size = *column_log_bound + config.log_blowup_factor;
            let commitment_domain = CanonicCosetImpl::new(commitment_domain_log_size)
                .circle_domain();
            column_commitment_domains.append(commitment_domain);
        }

        let first_layer = FriFirstLayerVerifier {
            column_log_bounds,
            column_commitment_domains: column_commitment_domains.span(),
            proof: first_layer_proof,
            folding_alpha: channel.draw_felt(),
        };

        // Bounds are stored in descending order.
        // TODO(andrew): There is no check that it's sorted. Add check.
        let max_column_log_bound = *column_log_bounds.first().unwrap();

        let mut layer_index = 0;
        let mut inner_layers = array![];
        let mut layer_log_bound = max_column_log_bound - CIRCLE_TO_LINE_FOLD_STEP;
        let mut layer_domain = LineDomainImpl::new_unchecked(
            CosetImpl::half_odds(layer_log_bound + config.log_blowup_factor),
        );

        loop {
            let proof = match inner_layer_proofs.pop_front() {
                Some(proof) => proof,
                None => { break Ok(()); },
            };

            channel.mix_root(proof.commitment.clone());

            inner_layers
                .append(
                    FriInnerLayerVerifier {
                        log_degree_bound: layer_log_bound,
                        domain: layer_domain,
                        folding_alpha: channel.draw_felt(),
                        layer_index,
                        proof,
                    },
                );

            layer_log_bound = match layer_log_bound.checked_sub(FOLD_STEP) {
                Some(layer_log_bound) => layer_log_bound,
                None => { break Err(FriVerificationError::InvalidNumFriLayers); },
            };

            layer_index += 1;
            layer_domain = layer_domain.double();
        }?;

        if layer_log_bound != config.log_last_layer_degree_bound {
            return Err(FriVerificationError::InvalidNumFriLayers);
        }

        if last_layer_poly.len() != pow2(config.log_last_layer_degree_bound) {
            return Err(FriVerificationError::LastLayerDegreeInvalid);
        }

        channel.mix_felts(last_layer_poly.coeffs.span());

        Ok(
            FriVerifier {
                config,
                first_layer,
                inner_layers,
                last_layer_domain: layer_domain,
                last_layer_poly,
                queries: None,
            },
        )
    }

    /// Verifies the decommitment stage of FRI.
    ///
    /// The query evals need to be provided in the same order as their commitment.
    fn decommit(
        self: FriVerifier, first_layer_query_evals: ColumnArray<Span<QM31>>,
    ) -> Result<(), FriVerificationError> {
        let queries = self.queries.expect('queries not sampled');
        self.decommit_on_queries(queries, first_layer_query_evals)
    }

    fn decommit_on_queries(
        self: FriVerifier, queries: Queries, first_layer_query_evals: ColumnArray<Span<QM31>>,
    ) -> Result<(), FriVerificationError> {
        let first_layer_sparse_evals = decommit_first_layer(
            @self, queries, first_layer_query_evals,
        )?;

        let inner_layer_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);

        let (last_layer_queries, last_layer_query_evals) = decommit_inner_layers(
            @self, inner_layer_queries, first_layer_sparse_evals,
        )?;

        decommit_last_layer(self, last_layer_queries, last_layer_query_evals)
    }

    /// Samples and returns query positions mapped by column log size.
    ///
    /// Output is of the form `(unique_log_sizes, queries_by_log_size)`.
    fn sample_query_positions(
        ref self: FriVerifier, ref channel: Channel,
    ) -> (Span<u32>, Felt252Dict<Nullable<Span<usize>>>) {
        // The sizes of input circle polynomial commitment domains.
        let mut column_log_sizes = array![];

        for column_commitment_domain in self.first_layer.column_commitment_domains {
            column_log_sizes.append(column_commitment_domain.log_size());
        }

        let unique_column_log_sizes = column_log_sizes.dedup().span();

        // Column are sorted in descending order by size.
        let max_column_log_size = *unique_column_log_sizes.first().unwrap();
        let n_queries = self.config.n_queries;
        let queries = QueriesImpl::generate(ref channel, max_column_log_size, n_queries);
        self.queries = Some(queries);

        (unique_column_log_sizes, get_query_positions_by_log_size(queries, unique_column_log_sizes))
    }
}

/// Verifies the first layer decommitment.
///
/// Returns the queries and first layer folded column evaluations needed for
/// verifying the remaining layers.
fn decommit_first_layer(
    verifier: @FriVerifier, queries: Queries, first_layer_query_evals: ColumnArray<Span<QM31>>,
) -> Result<ColumnArray<SparseEvaluation>, FriVerificationError> {
    verifier.first_layer.verify(queries, first_layer_query_evals)
}

/// Verifies all inner layer decommitments.
///
/// Returns the queries and query evaluations needed for verifying the last FRI layer.
fn decommit_inner_layers(
    verifier: @FriVerifier,
    queries: Queries,
    mut first_layer_sparse_evals: ColumnArray<SparseEvaluation>,
) -> Result<(Queries, Array<QM31>), FriVerificationError> {
    let mut inner_layers = verifier.inner_layers.span();
    let mut layer_queries = queries;
    let mut layer_query_evals = ArrayImpl::new_repeated(n: layer_queries.len(), v: Zero::zero());
    let mut first_layer_sparse_evals = first_layer_sparse_evals.span();
    let mut first_layer_column_bounds = *verifier.first_layer.column_log_bounds;
    let mut first_layer_column_domains = *verifier.first_layer.column_commitment_domains;
    let mut prev_fold_alpha = *verifier.first_layer.folding_alpha;

    loop {
        let layer = match inner_layers.pop_front() {
            Some(layer) => layer,
            None => { break Ok(()); },
        };

        let circle_poly_degree_bound = *layer.log_degree_bound + CIRCLE_TO_LINE_FOLD_STEP;

        // Check for evals committed in the first layer that need to be folded into this layer.
        while let Some(_) = first_layer_column_bounds.next_if_eq(@circle_poly_degree_bound) {
            let column_domain = *first_layer_column_domains.pop_front().unwrap();
            let first_layer_sparse_eval = first_layer_sparse_evals.pop_front().unwrap();
            let mut folded_column_evals = first_layer_sparse_eval
                .fold_circle(prev_fold_alpha, column_domain);
            let mut updated_layer_query_evals = array![];
            let prev_fold_alpha_pow_fold_factor = prev_fold_alpha * prev_fold_alpha;

            while let (Some(curr_layer_eval), Some(folded_column_eval)) =
                (layer_query_evals.pop_front(), folded_column_evals.pop_front()) {
                updated_layer_query_evals
                    .append(curr_layer_eval * prev_fold_alpha_pow_fold_factor + folded_column_eval);
            }

            layer_query_evals = updated_layer_query_evals;
        }

        match layer
            .verify_and_fold(queries: layer_queries, evals_at_queries: layer_query_evals.span()) {
            Ok((
                next_layer_queries, next_layer_query_evals,
            )) => {
                layer_queries = next_layer_queries;
                layer_query_evals = next_layer_query_evals;
                prev_fold_alpha = *layer.folding_alpha;
            },
            Err(error) => { break Err(error); },
        };
    }?;

    // Check all values have been consumed.
    assert!(first_layer_column_bounds.is_empty());
    assert!(first_layer_column_domains.is_empty());
    assert!(first_layer_sparse_evals.is_empty());

    Ok((layer_queries, layer_query_evals))
}

/// Verifies the last layer.
fn decommit_last_layer(
    verifier: FriVerifier, mut queries: Queries, mut query_evals: Array<QM31>,
) -> Result<(), FriVerificationError> {
    let FriVerifier { last_layer_domain, last_layer_poly, .. } = verifier;

    // TODO(andrew): Note depending on the proof parameters, doing FFT on the last layer poly vs
    // pointwize evaluation is less efficient.
    let last_layer_evals = last_layer_poly.evaluate(last_layer_domain).values;
    let domain_log_size = last_layer_domain.log_size();

    loop {
        let (query, query_eval) = match (queries.positions.pop_front(), query_evals.pop_front()) {
            (Some(query), Some(query_eval)) => (query, query_eval),
            _ => { break Ok(()); },
        };

        // TODO(andrew): Makes more sense for the proof to provide coeffs in natural order and
        // the FFT return evals in bit-reversed order to prevent this unnessesary bit-reverse.
        let last_layer_eval_i = bit_reverse_index(*query, domain_log_size);

        if query_eval != *last_layer_evals[last_layer_eval_i] {
            break Err(FriVerificationError::LastLayerEvaluationsInvalid);
        }
    }
}

/// Returns the column query positions needed for verification.
///
/// The column log sizes must be unique and in descending order.
/// Returned column query positions are mapped by their log size.
fn get_query_positions_by_log_size(
    mut queries: Queries, mut unique_column_log_sizes: Span<u32>,
) -> Felt252Dict<Nullable<Span<usize>>> {
    let mut query_positions_by_log_size: Felt252Dict<Nullable<Span<usize>>> = Default::default();

    for column_log_size in unique_column_log_sizes {
        let n_folds = queries.log_domain_size - *column_log_size;

        if n_folds != 0 {
            queries = queries.fold(n_folds);
        }

        query_positions_by_log_size
            .insert((*column_log_size).into(), NullableTrait::new(queries.positions));
    }

    query_positions_by_log_size
}

/// A FRI proof.
#[derive(Drop, Serde)]
pub struct FriProof {
    pub first_layer: FriLayerProof,
    pub inner_layers: Span<FriLayerProof>,
    pub last_layer_poly: LinePoly,
}

#[derive(Drop)]
struct FriFirstLayerVerifier {
    /// The list of degree bounds of all circle polynomials commited in the first layer.
    column_log_bounds: Span<u32>,
    /// The commitment domain all the circle polynomials in the first layer.
    column_commitment_domains: Span<CircleDomain>,
    folding_alpha: QM31,
    proof: FriLayerProof,
}

#[generate_trait]
impl FriFirstLayerVerifierImpl of FriFirstLayerVerifierTrait {
    /// Verifies the first layer's merkle decommitment, and returns the evaluations needed for
    /// folding the columns to their corresponding layer.
    ///
    /// # Errors
    ///
    /// An `Err` will be returned if:
    /// * The proof doesn't store enough evaluations.
    /// * The merkle decommitment is invalid.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// * The queries are sampled on the wrong domain.
    /// * There are an invalid number of provided column evals.
    fn verify(
        self: @FriFirstLayerVerifier,
        queries: Queries,
        mut query_evals_by_column: ColumnArray<Span<QM31>>,
    ) -> Result<ColumnArray<SparseEvaluation>, FriVerificationError> {
        // Columns are provided in descending order by size.
        let max_column_log_size = (*self.column_commitment_domains).first().unwrap().log_size();
        assert!(queries.log_domain_size == max_column_log_size);

        let mut column_queries = queries;
        let mut column_commitment_domains = *self.column_commitment_domains;
        let mut fri_witness = (*self.proof.fri_witness).into_iter();
        let mut decommitment_positions_by_log_size: Felt252Dict = Default::default();
        // For decommitment, each QM31 col must be split into its constituent M31 coordinate cols.
        let mut decommitment_coordinate_column_log_sizes = array![];
        let mut sparse_evals_by_column = array![];
        let mut decommitmented_values = array![];

        loop {
            let (column_domain, column_query_evals) =
                match (column_commitment_domains.pop_front(), query_evals_by_column.pop_front()) {
                (Some(domain), Some(evals)) => (domain, evals),
                (None, None) => { break Ok(()); },
                _ => { panic!() },
            };

            let column_domain_log_size = column_domain.log_size();
            let n_folds = column_queries.log_domain_size - column_domain_log_size;

            if n_folds != 0 {
                column_queries = column_queries.fold(n_folds);
            }

            let (column_decommitment_positions, sparse_evaluation) =
                match compute_decommitment_positions_and_rebuild_evals(
                    column_queries, column_query_evals, ref fri_witness, CIRCLE_TO_LINE_FOLD_STEP,
                ) {
                Ok(res) => res,
                Err(_) => { break Err(FriVerificationError::FirstLayerEvaluationsInvalid); },
            };

            // Columns of the same size have the same decommitment positions.
            // TODO(andrew): Do without nullable.
            decommitment_positions_by_log_size
                .insert(
                    column_domain_log_size.into(),
                    NullableTrait::new(column_decommitment_positions),
                );

            for subset_eval in sparse_evaluation.subset_evals.span() {
                for eval in subset_eval.span() {
                    // Split the QM31 into its M31 coordinate values.
                    let [v0, v1, v2, v3] = (*eval).to_array();
                    decommitmented_values.append(v0.into());
                    decommitmented_values.append(v1.into());
                    decommitmented_values.append(v2.into());
                    decommitmented_values.append(v3.into());
                };
            }

            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);
            decommitment_coordinate_column_log_sizes.append(column_domain_log_size);

            sparse_evals_by_column.append(sparse_evaluation);
        }?;

        // Check all proof evals have been consumed.
        if fri_witness.next().is_some() {
            return Err(FriVerificationError::FirstLayerEvaluationsInvalid);
        }

        let merkle_verifier = MerkleVerifier {
            root: self.proof.commitment.clone(),
            column_log_sizes: decommitment_coordinate_column_log_sizes,
        };

        if let Err(_) = merkle_verifier
            .verify(
                decommitment_positions_by_log_size,
                decommitmented_values.span(),
                self.proof.decommitment.clone(),
            ) {
            return Err(FriVerificationError::FirstLayerCommitmentInvalid);
        }

        Ok(sparse_evals_by_column)
    }
}

#[derive(Drop, Debug)]
struct FriInnerLayerVerifier {
    log_degree_bound: u32,
    domain: LineDomain,
    folding_alpha: QM31,
    layer_index: usize,
    proof: @FriLayerProof,
}

#[generate_trait]
impl FriInnerLayerVerifierImpl of FriInnerLayerVerifierTrait {
    /// Verifies the layer's merkle decommitment and returns the the folded queries and query evals.
    fn verify_and_fold(
        self: @FriInnerLayerVerifier, queries: Queries, evals_at_queries: Span<QM31>,
    ) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        assert!(queries.log_domain_size == self.domain.log_size());

        let mut fri_witness = (**self.proof.fri_witness).into_iter();

        let (decommitment_positions, sparse_evaluation) =
            match compute_decommitment_positions_and_rebuild_evals(
                queries, evals_at_queries, ref fri_witness, FOLD_STEP,
            ) {
            Ok(res) => res,
            Err(_) => { return Err(FriVerificationError::InnerLayerEvaluationsInvalid); },
        };

        // Check all proof evals have been consumed.
        if fri_witness.next().is_some() {
            return Err(FriVerificationError::InnerLayerEvaluationsInvalid);
        }

        let mut decommitmented_values = array![];
        for subset_eval in sparse_evaluation.subset_evals.span() {
            for eval in subset_eval.span() {
                // Split the QM31 into its M31 coordinate values.
                let [v0, v1, v2, v3] = (*eval).to_array();
                decommitmented_values.append(v0.into());
                decommitmented_values.append(v1.into());
                decommitmented_values.append(v2.into());
                decommitmented_values.append(v3.into());
            };
        }

        let column_log_size = self.domain.log_size();
        let merkle_verifier = MerkleVerifier {
            root: (*self.proof.commitment).clone(),
            column_log_sizes: ArrayImpl::new_repeated(n: QM31_EXTENSION_DEGREE, v: column_log_size),
        };

        let mut decommitment_positions_dict: Felt252Dict<Nullable<Span<usize>>> =
            Default::default();
        decommitment_positions_dict
            .insert(self.domain.log_size().into(), NullableTrait::new(decommitment_positions));

        if let Err(_) = merkle_verifier
            .verify(
                decommitment_positions_dict,
                decommitmented_values.span(),
                (*self.proof.decommitment).clone(),
            ) {
            return Err(FriVerificationError::InnerLayerCommitmentInvalid);
        }

        let folded_queries = queries.fold(FOLD_STEP);
        let folded_evals = sparse_evaluation.fold_line(*self.folding_alpha, *self.domain);

        Ok((folded_queries, folded_evals))
    }
}

/// Returns a column's merkle tree decommitment positions and re-builds the evaluations needed by
/// the verifier for folding and decommitment.
///
/// # Panics
///
/// Panics if the number of queries doesn't match the number of query evals.
fn compute_decommitment_positions_and_rebuild_evals(
    mut queries: Queries,
    mut query_evals: Span<QM31>,
    ref witness_evals_iter: SpanIter<QM31>,
    fold_step: u32,
) -> Result<(Span<usize>, SparseEvaluation), InsufficientWitnessError> {
    let fold_factor = pow2(fold_step);
    let mut query_evals_iter = query_evals.into_iter();

    let mut decommitment_positions = array![];
    let mut subset_evals = array![];
    let mut subset_domain_index_initials = array![];

    let mut query_positions = queries.positions;
    let mut folded_query_positions = queries.fold(fold_step).positions;

    loop {
        let folded_query_position = match folded_query_positions.pop_front() {
            Some(position) => *position,
            None => { break Ok(()); },
        };

        let subset_start = folded_query_position * fold_factor;
        let subset_end = subset_start + fold_factor;
        let mut subset_decommitment_positions = (subset_start..subset_end).into_iter();
        let mut subset_eval = array![];

        // Extract the subset eval and decommitment positions.
        // TODO(andrew): Handle the error.
        let loop_res = loop {
            let decommitment_position = match subset_decommitment_positions.next() {
                Some(position) => position,
                None => { break Ok(()); },
            };

            decommitment_positions.append(decommitment_position);

            // If the decommitment position is a query position: take the value from `query_evals`,
            // else: take the value from `witness_evals`.
            subset_eval
                .append(
                    *match query_positions.next_if_eq(@decommitment_position) {
                        Some(_) => {
                            let res = query_evals_iter.next().unwrap();
                            res
                        },
                        None => match witness_evals_iter.next() {
                            Some(witness_eval) => { witness_eval },
                            None => { break Err(InsufficientWitnessError {}); },
                        },
                    },
                );
        };

        // TODO(andrew): Is there an easier way to break if an inner loop returns an error i.e. like
        // `loop { .. }?;` but for breaking a loop?.
        if let Err(error) = loop_res {
            break Err(error);
        }

        subset_evals.append(subset_eval);

        subset_domain_index_initials
            .append(bit_reverse_index(subset_start, queries.log_domain_size));
    }?;

    // Sanity check all the values have been consumed.
    assert!(query_positions.is_empty());
    assert!(query_evals_iter.next().is_none());

    let sparse_evaluation = SparseEvaluationImpl::new(
        subset_evals, subset_domain_index_initials.span(),
    );

    Ok((decommitment_positions.span(), sparse_evaluation))
}

#[derive(Drop)]
struct InsufficientWitnessError {}

/// Foldable subsets of evaluations on a circle polynomial or univariate polynomial.
#[derive(Drop)]
struct SparseEvaluation {
    // TODO(andrew): Perhaps subset isn't the right word. Coset, Subgroup?
    subset_evals: Array<Array<QM31>>,
    subset_domain_initial_indexes: Span<usize>,
}

#[generate_trait]
impl SparseEvaluationImpl of SparseEvaluationTrait {
    fn new(
        subset_evals: Array<Array<QM31>>, subset_domain_initial_indexes: Span<usize>,
    ) -> SparseEvaluation {
        assert!(subset_evals.len() == subset_domain_initial_indexes.len());
        SparseEvaluation { subset_evals, subset_domain_initial_indexes }
    }

    /// Folds evaluations of a degree `d` univariate polynomial into evaluations of a degree `d/2`
    /// univariate polynomial.
    fn fold_line(
        self: @SparseEvaluation, fold_alpha: QM31, source_domain: LineDomain,
    ) -> Array<QM31> {
        let mut domain_initials = array![];

        for subset_domain_initial_index in *self.subset_domain_initial_indexes {
            domain_initials.append(source_domain.at(*subset_domain_initial_index));
        }

        let mut domain_initials_inv = BatchInvertible::batch_inverse(domain_initials);
        let mut res = array![];

        for subset_eval in self.subset_evals.span() {
            let x_inv = domain_initials_inv.pop_front().unwrap();
            let values: Box<[QM31; FOLD_FACTOR]> = *subset_eval.span().try_into().unwrap();
            let [f_at_x, f_at_neg_x] = values.unbox();
            let (f0, f1) = ibutterfly(f_at_x, f_at_neg_x, x_inv);
            res.append(f0 + fold_alpha * f1);
        }

        res
    }

    /// Folds evaluations of a degree `d` circle polynomial into evaluations of a
    /// degree `d/2` univariate polynomial.
    fn fold_circle(
        self: @SparseEvaluation, fold_alpha: QM31, source_domain: CircleDomain,
    ) -> Array<QM31> {
        let mut domain_initial_ys = array![];

        for subset_domain_initial_index in *self.subset_domain_initial_indexes {
            domain_initial_ys.append(source_domain.at(*subset_domain_initial_index).y);
        }

        let mut domain_initial_ys_inv = BatchInvertible::batch_inverse(domain_initial_ys);
        let mut res = array![];

        for subset_eval in self.subset_evals.span() {
            let y_inv = domain_initial_ys_inv.pop_front().unwrap();
            let values: Box<[QM31; CIRCLE_TO_LINE_FOLD_FACTOR]> = *subset_eval
                .span()
                .try_into()
                .unwrap();
            let [f_at_p, f_at_neg_p] = values.unbox();
            let (f0, f1) = ibutterfly(f_at_p, f_at_neg_p, y_inv);
            res.append(f0 + fold_alpha * f1);
        }

        res
    }
}

/// Proof of an individual FRI layer.
#[derive(Drop, Clone, Debug, Serde)]
pub struct FriLayerProof {
    /// Values that the verifier needs but cannot deduce from previous computations, in the
    /// order they are needed. This complements the values that were queried. These must be
    /// supplied directly to the verifier.
    pub fri_witness: Span<QM31>,
    pub decommitment: MerkleDecommitment<MerkleHasher>,
    pub commitment: Hash,
}

#[derive(Debug, Drop)]
pub enum FriVerificationError {
    InvalidNumFriLayers,
    FirstLayerEvaluationsInvalid,
    FirstLayerCommitmentInvalid,
    InnerLayerEvaluationsInvalid,
    InnerLayerCommitmentInvalid,
    LastLayerDegreeInvalid,
    LastLayerEvaluationsInvalid,
}

#[cfg(test)]
mod test {
    use crate::channel::{Channel, ChannelTrait};
    use crate::circle::{CirclePointIndexImpl, CosetImpl};
    use crate::fields::qm31::qm31_const;
    use crate::poly::circle::CircleEvaluationImpl;
    use crate::poly::line::LineDomainImpl;
    use crate::queries::{Queries, QueriesImpl};
    use super::{FriConfig, FriVerificationError, FriVerifierImpl};

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn valid_proof_passes_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 1, log_blowup_factor: 2, n_queries: 1,
        };
        let column_log_bound = 4;
        let column_log_size = column_log_bound + config.log_blowup_factor;
        let column_log_bounds = array![column_log_bound].span();
        let queries = Queries { positions: array![5].span(), log_domain_size: column_log_size };
        let query_evals = array![array![qm31_const::<1242514872, 0, 0, 0>()].span()];
        let mut proof_data = array![
            1, 1381744584, 0, 0, 0, 5,
            248237920038552760115173130164174203123353934176490790804028298292638637730,
            565795211414946469157204288974583864677896605339666153201617364626219012076,
            1429241829308718590475118459587065761194070023556886281836694442445078550767,
            112736985517415102864396627290325395865021582833422635609580236825399909887,
            1734751601212898474594489984934132832871090066333664002442834978729904575960, 0,
            2062842123560885137372604875777049649036399148319124894222012291287950136814, 2, 1,
            99537030, 598645364, 524399856, 124126124, 4,
            1291319665378567694814885574810831136881893448439564944229389185828650376462,
            2249116528260766262796149521495905607529121395331559366029961601830532988670,
            1126517304007438099452497759147854255367428269270252391844854672474276877173,
            1617597733625675487228439398588025564892271376782836689059755775897834180556, 0,
            1983648368674016433209849425227719906723907079145392260477967429097269024313, 1,
            1840773062, 1902846149, 205939217, 1982803611, 3,
            1843248255397560487620908119891030826762716801182907092596379579688005655434,
            1082215532944925120606470745042487858168767126081955019128805741045675298724,
            1887153286672469747773859320549426761925735961972312365901204557604613601081, 0,
            3023413240775455596710093522929806976598221825938778905395287716531056287773, 2,
            591023774, 1634460577, 1933849409, 1639637777, 591023774, 1634460577, 1933849409,
            1639637777, 1,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();

        verifier.decommit_on_queries(queries, query_evals).unwrap();
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn valid_proof_with_constant_last_layer_passes_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 0, log_blowup_factor: 2, n_queries: 1,
        };
        let column_log_bound = 3;
        let column_log_size = column_log_bound + config.log_blowup_factor;
        let column_log_bounds = array![column_log_bound].span();
        let queries = Queries { positions: array![5].span(), log_domain_size: column_log_size };
        let query_evals = array![array![qm31_const::<1059056252, 0, 0, 0>()].span()];
        let mut proof_data = array![
            1, 520313910, 0, 0, 0, 4,
            2511859689460956062754798212736603971289039205668758416916441010091827960967,
            3116447051172487019914330687758294186696708622072747727020227859480276128456,
            721698868812272649654951509597754221353628194583368094837414807957240128187,
            2322781146978847693424724097972151857550932350708357827088012624999661465745, 0,
            748174431875540733373318683157905446208709800770031351438402873811128343902, 2, 1,
            822799490, 762931150, 1156162632, 2122921032, 3,
            2682605036744925837528395024963644910241648916416714246021123266777932169829,
            2897190808213678224361292969431925141623148330898699376650599616824548437798,
            1696260486512635605141370500725283204493308222655042278770397536250642854003, 0,
            2532324905130335387885988086730858190821881679074630420916383382420280214544, 1,
            1262514944, 1312656862, 371859941, 1063882049, 2,
            3370021968363855570672925386839858344850086742008104500695244207452648849439,
            769840047821584466049489766531807204023233788256576686626168200236190476002, 0,
            3299797690816392573386697551286401640236146406244365528122431092098000056783, 1,
            1897124133, 864171056, 872860971, 139972076, 0,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();

        verifier.decommit_on_queries(queries, query_evals).unwrap();
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn valid_mixed_degree_proof_passes_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 2, log_blowup_factor: 2, n_queries: 2,
        };
        let column_log_bounds = array![6, 5, 4].span();
        let max_column_log_size = *column_log_bounds[0] + config.log_blowup_factor;
        let queries = Queries {
            positions: array![7, 70].span(), log_domain_size: max_column_log_size,
        };
        let query_evals_by_column = array![
            array![qm31_const::<1086480365, 0, 0, 0>(), qm31_const::<1641851833, 0, 0, 0>()].span(),
            array![qm31_const::<455377974, 0, 0, 0>(), qm31_const::<734901538, 0, 0, 0>()].span(),
            array![qm31_const::<127986842, 0, 0, 0>(), qm31_const::<1809783851, 0, 0, 0>()].span(),
        ];
        let mut proof_data = array![
            6, 526587957, 0, 0, 0, 1437150104, 0, 0, 0, 1829392430, 0, 0, 0, 537130739, 0, 0, 0,
            466407290, 0, 0, 0, 57598558, 0, 0, 0, 15,
            3421324018055699479496104615050761807609805326584520775327079053702207326683,
            2330443576228719381632164764556367362586482387226832541100606007323985940494,
            887461958902773443898246353903830803582977392502641659935582370155836337335,
            1042142015464500480184248823464893036715138273826821461316876289730089667491,
            2008379838341947588860067279261308170323485773456710358719827408251658694576,
            448590630519198450202628760734512534696449934811836393688344556748155207089,
            3219389286842873950904053384074541207099675646523024274488094441621215282104,
            260868260226152002210395313184016285020480537321966550619831826552955245469,
            2506020012326569502696314581077800626488371208680295494281508851965097439385,
            3540027825632145272201110936622715638959073850783481667375168474797345543588,
            1636055863497721599502597754618727152515947225111829343743541919436660795256,
            3528613168809509450017421250157984372326630216029035025827291365248806069657,
            167525632150548163893942209752970156590121577705462960369036894818323866109,
            565298324760267740526869033802555657924955517654710479282581639258488240793,
            3295022416540753404585716324113082086794958115799524768012594202270050792498, 0,
            2445089585988445976839018166677734368458871763768371606514145595508593107280, 3, 2,
            846942708, 1316713975, 1953115736, 1210460432, 889376038, 1762207776, 89420610,
            1632236435, 9,
            2067220454524038244428504197923630442590620916172088910094312597993114174910,
            983564698554398749898716443897676520726618432407716601995953674629961915198,
            1942499659156005049843276794149414877389862675199669225605310851211330576231,
            1312830079214797255425291440682267391606241995344195319836556922549285481101,
            2930718454070247990267379386862887139141284124718178871470991118359643049048,
            369784786568146698895275692375877483349264484649907239380514740999020085390,
            146108019810989720845887526122018346914684286910495653184631168618380573647,
            3564579070868287706328499126369789116076984185119293652405758857973603647074,
            2507697581821306366504882095585363206314662661007412833498910881472851727681, 0,
            380279795793516015327686081082577349613118967174225832006193046074288193451, 2,
            810422993, 549597255, 742728049, 1420495095, 1895377213, 1497246528, 1233007332,
            2142176098, 7,
            1282816230210098734030995560127090397685239081104726636044591253493370307429,
            1406511017939057441302642939991849495972545773167811392589253876471671973660,
            338124601418460142984980871087198677649847880420334694593891783955215527044,
            2100587184826875173250822555681724547200951205061104890378058643426227179059,
            1013778858613323352830925648746910056539874114776963582551317986204267411482,
            577981265423866299126390712985402151548270728972661182480159517199777272947,
            1937946099960709864705068752874828455041161150893298566587508511390281402007, 0,
            2427721132030678761597508719260487862745032268909838166334853008563605603044, 2,
            908222780, 43974163, 1956189523, 1020601450, 1798101537, 1440100863, 611704414,
            553423607, 5,
            812365784490476235508684971934544962955163888812446798530946859960605738503,
            764984335886451990341159270512305360476044654579646368333265166381050846118,
            1077758081676939549244030557898591743511791784984358848623942779243730674077,
            2122062998200095003439260133157994075767023076499281614880396264997220931502,
            2278074802674035751127226922682361508316169543102033913446484131381896508207, 0,
            1370391678871254279717913562379765266526987774724491259716767700180864026511, 4,
            1263113205, 1843822163, 531396913, 1520711933, 1263113205, 1843822163, 531396913,
            1520711933, 1263113205, 1843822163, 531396913, 1520711933, 1263113205, 1843822163,
            531396913, 1520711933, 2,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();

        verifier.decommit_on_queries(queries, query_evals_by_column).unwrap();
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn mixed_degree_proof_with_queries_sampled_from_channel_passes_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 2, log_blowup_factor: 2, n_queries: 3,
        };
        let column_log_bounds = array![6, 5, 4].span();
        let query_evals_by_column = array![
            array![
                qm31_const::<1622179054, 0, 0, 0>(), qm31_const::<1104239013, 0, 0, 0>(),
                qm31_const::<231172335, 0, 0, 0>(),
            ]
                .span(),
            array![
                qm31_const::<962456622, 0, 0, 0>(), qm31_const::<205390949, 0, 0, 0>(),
                qm31_const::<1061027972, 0, 0, 0>(),
            ]
                .span(),
            array![
                qm31_const::<619773471, 0, 0, 0>(), qm31_const::<465540164, 0, 0, 0>(),
                qm31_const::<1073797249, 0, 0, 0>(),
            ]
                .span(),
        ];
        let mut proof_data = array![
            9, 1231945271, 0, 0, 0, 56055233, 0, 0, 0, 727227966, 0, 0, 0, 1494326383, 0, 0, 0,
            1095409103, 0, 0, 0, 958231711, 0, 0, 0, 1927599636, 0, 0, 0, 604224095, 0, 0, 0,
            1097851807, 0, 0, 0, 20,
            1255613725489791209650105440601537764712000862519033119307981233550137661155,
            3465998622438987482422615396445198513344646449106044816456344464840673608917,
            1579103844314527294074482131877085614657187993047649387254625983511038632478,
            1836040142909366886991373161699847646973812894978176107452736037819680876234,
            2318943299146733382762380074259936475451406316127165056047017953597903131727,
            896210176879223377564263006142804405455334459379303373557303462869569027849,
            27118476866256629122219345560562955560593812014328258608734601741751685131,
            1194836845261553250234096726341000934055925346559900450493065289974467697276,
            1057162759696202590309892452823570247623356188198969888292420441946781156084,
            1085368966290914331227680030064338585116034078326551064404623112215534517616,
            558090923078539102834498673726360830935013409512313085985301147426902942401,
            58945168431054466129809621008820285637190722960529255861940103936445468094,
            2386008196348860605200312334803241485569422150918218869712230870011365036692,
            1110466417021587456300594335445215919250942244290829430633741584184624461956,
            3343124230216319067695947682431205444637173644730169341308489214495400301171,
            3255113767235620041424889121061289922329814349949230141670516524123494311349,
            565298324760267740526869033802555657924955517654710479282581639258488240793,
            169764250527681324164221639816854881982334265983093810147856666033812340989,
            3277860936667567130905205573680315538202961663243197247355824385344200592605,
            1331312300678708686852617807060225524879052786102492336699248194945061029220, 0,
            2445089585988445976839018166677734368458871763768371606514145595508593107280, 3, 3,
            979442419, 587110564, 1328520057, 1192240617, 381069592, 2030985727, 1315333223,
            94156578, 494072332, 1362601376, 1861632162, 199804464, 11,
            2175070916862691478523725168861351549957421247402141186548036591876374252274,
            1906868958332591867369860228574914862173698540322210216529024484841521055065,
            2348236351204812774518841051011842334845718249721684576184852063090332400700,
            3074151298964845942483510861669316185843060283119366352268851170467640870391,
            1991100447620090764811037541248872238605228869987459596104404654376943598689,
            2433675357104482601675845877657860262661496101883240727691195150827358495629,
            751991765080010646211185440482926662126241506295062338288745189707168146963,
            3564579070868287706328499126369789116076984185119293652405758857973603647074,
            999516884653847522846292682840981939847232242758891787911937848108488593763,
            1998956941538782164394267811926388611936689556172683745867734036694428348159,
            1109369034223326111295790834214269541941830920151396576331733976632609437893, 0,
            380279795793516015327686081082577349613118967174225832006193046074288193451, 3,
            1041751807, 273018, 355477115, 27304098, 1422937491, 474998460, 1392550012, 1550676711,
            259126404, 753270265, 654810118, 690812765, 8,
            1425745501459517281488876967594938263690391456911736669566797663132757347396,
            1389919918651071666802840231769027234092809896384753736357938988480699389811,
            3326779193698307499499428119363749020021491564580745459030155352552543623309,
            1893996990564850410749809718371670376463910524585132544024687412693553528367,
            577981265423866299126390712985402151548270728972661182480159517199777272947,
            2992573147603016154664414985315152480454878366766871166909907493377999504211,
            408021228821750588489991450073022637958850423325235083232871355985975989250,
            1233550027792761656301285896564316665465713625848925824963157744570519703058, 0,
            2427721132030678761597508719260487862745032268909838166334853008563605603044, 3,
            910122642, 1488583011, 591415629, 332226537, 1390471863, 1941461608, 1144997196,
            627928469, 119028977, 1636386447, 240997492, 457607964, 5,
            1874963451684128317276363705195009906704051987761953070781687612663397260592,
            2122062998200095003439260133157994075767023076499281614880396264997220931502,
            513946138044385089941378111212482566421929032178332853573746012768136464056,
            152520003676965715125194580945302801203310284687061966572764134846913784628,
            1383132424767704872238977503590219244609602505821057501862400154258457587816, 0,
            1370391678871254279717913562379765266526987774724491259716767700180864026511, 4,
            1263113205, 1843822163, 531396913, 1520711933, 1263113205, 1843822163, 531396913,
            1520711933, 1263113205, 1843822163, 531396913, 1520711933, 1263113205, 1843822163,
            531396913, 1520711933, 2,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let mut verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();
        let _query_positions_per_log_size = verifier.sample_query_positions(ref channel);

        verifier.decommit(query_evals_by_column).unwrap();
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn proof_with_invalid_inner_layer_evaluation_fails_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 2, log_blowup_factor: 2, n_queries: 1,
        };
        let column_log_bound = 6;
        let column_log_size = column_log_bound + config.log_blowup_factor;
        let column_log_bounds = array![column_log_bound].span();
        let queries = Queries { positions: array![5].span(), log_domain_size: column_log_size };
        let query_evals = array![array![qm31_const::<511282811, 0, 0, 0>()].span()];
        let mut proof_data = array![
            1, 297706028, 0, 0, 0, 7,
            3493776687527140828368497288821880537037420561562730934871129213640366059514,
            1302201433386659022646953262844862660100213350984814381128106468273445814027,
            2779751751011164856195488077852838228707262348063408173412536809957725682863,
            770189832466915170827672871303060560096281402158850180038788106465085477483,
            3069976756114957526059192199103487785556171329209053545239438274409921785053,
            680223131569150133967126151154770375037918917679072010383451469433376284715,
            771087638539779794875017397672832958116409883486068782457639684521745902298, 0,
            2906774104231402115125849245886411734063363715652850229575637428545214370658, 3, 1,
            538440537, 763926843, 297835090, 675450448, 6,
            2323489267184389648904940334524128319335728562719478000757257061595699724409,
            2518355010803493215236742676989937047358819576038995155770467812049853780302,
            1052099319509182016215622541805215392785322937638345954453047023790985619336,
            134724124743202229553669564186632778246758284203279875388625817255285523351,
            747228874504072031948012038628776919639938882961609484616335923189916306269,
            986490867278298294475423346828038671538855090104725084441125913874228017567, 0,
            995524627752196735661894212183970503203430507166601836968508900279938945086, 0, 5,
            3391302796018267639958936865244251298561915156822537900630723309754711190171,
            97439209108437137515782522757217317923887919870294821783405862016101191265,
            1948531947904000762393255748180154743732564980365748228979683960050239935500,
            629549320318792956712638764191845454995553725249231776669667909539065755958,
            2434751759245803082581349704195895480432168147867347790967923582277723748886, 0,
            2149735103824111869949414413764166503122090836900441970959186989416467221143, 1,
            294882200, 730761129, 578956600, 36719417, 4,
            479514794246235700821866475623346973502891730541037606725184106895626377158,
            297309643017702014371545487431545507680516868776245181237311089465722014591,
            2127502746971593028564482961924309171918348094827433337314131210427373210306,
            2206301781526548311677458232607775332450862347745916668552172879217421510381, 0,
            2168899312563977570176790236694005793684040578094317937516080629846982916156, 4,
            92255467, 1174963615, 663113792, 2066585349, 92255467, 1174963615, 663113792,
            2066585349, 92255467, 1174963615, 663113792, 2066585349, 92255467, 1174963615,
            663113792, 2066585349, 2,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();

        let verification_result = verifier.decommit_on_queries(queries, query_evals);

        // TODO(andrew): Something like Rust's `assert_matches!()` would be nice.
        match verification_result {
            Err(error) => match error {
                FriVerificationError::InnerLayerEvaluationsInvalid => {},
                _ => panic!("wrong error"),
            },
            _ => panic!("should error"),
        }
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn proof_with_added_layer_fails_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 3, log_blowup_factor: 2, n_queries: 1,
        };
        let column_log_bound = 6;
        let column_log_bounds = array![column_log_bound].span();
        let mut proof_data = array![
            1, 13485189, 0, 0, 0, 7,
            1018772866886356793591132853402474585086454566523929550724544407378948823808,
            2649771523025999405113769711824400532328865192514853420237355944984638515692,
            2779751751011164856195488077852838228707262348063408173412536809957725682863,
            770189832466915170827672871303060560096281402158850180038788106465085477483,
            3069976756114957526059192199103487785556171329209053545239438274409921785053,
            680223131569150133967126151154770375037918917679072010383451469433376284715,
            771087638539779794875017397672832958116409883486068782457639684521745902298, 0,
            2906774104231402115125849245886411734063363715652850229575637428545214370658, 3, 1,
            1984232051, 1301620570, 1751255046, 2066154700, 6,
            930909046950338848675671179033947720344577278394252394520228351960094703457,
            2518355010803493215236742676989937047358819576038995155770467812049853780302,
            1052099319509182016215622541805215392785322937638345954453047023790985619336,
            134724124743202229553669564186632778246758284203279875388625817255285523351,
            747228874504072031948012038628776919639938882961609484616335923189916306269,
            986490867278298294475423346828038671538855090104725084441125913874228017567, 0,
            995524627752196735661894212183970503203430507166601836968508900279938945086, 1,
            1458405795, 491359394, 695332103, 1156919555, 5,
            3391302796018267639958936865244251298561915156822537900630723309754711190171,
            97439209108437137515782522757217317923887919870294821783405862016101191265,
            1948531947904000762393255748180154743732564980365748228979683960050239935500,
            629549320318792956712638764191845454995553725249231776669667909539065755958,
            2434751759245803082581349704195895480432168147867347790967923582277723748886, 0,
            2149735103824111869949414413764166503122090836900441970959186989416467221143, 1,
            294882200, 730761129, 578956600, 36719417, 4,
            479514794246235700821866475623346973502891730541037606725184106895626377158,
            297309643017702014371545487431545507680516868776245181237311089465722014591,
            2127502746971593028564482961924309171918348094827433337314131210427373210306,
            2206301781526548311677458232607775332450862347745916668552172879217421510381, 0,
            2168899312563977570176790236694005793684040578094317937516080629846982916156, 4,
            92255467, 1174963615, 663113792, 2066585349, 92255467, 1174963615, 663113792,
            2066585349, 92255467, 1174963615, 663113792, 2066585349, 92255467, 1174963615,
            663113792, 2066585349, 2,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();

        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds);

        match verifier {
            Err(error) => match error {
                FriVerificationError::InvalidNumFriLayers => {},
                _ => panic!("wrong error"),
            },
            _ => panic!("should error"),
        }
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn proof_with_removed_layer_fails_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 1, log_blowup_factor: 2, n_queries: 1,
        };
        let column_log_bound = 6;
        let column_log_bounds = array![column_log_bound].span();
        let mut proof_data = array![
            1, 13485189, 0, 0, 0, 7,
            1018772866886356793591132853402474585086454566523929550724544407378948823808,
            2649771523025999405113769711824400532328865192514853420237355944984638515692,
            2779751751011164856195488077852838228707262348063408173412536809957725682863,
            770189832466915170827672871303060560096281402158850180038788106465085477483,
            3069976756114957526059192199103487785556171329209053545239438274409921785053,
            680223131569150133967126151154770375037918917679072010383451469433376284715,
            771087638539779794875017397672832958116409883486068782457639684521745902298, 0,
            2906774104231402115125849245886411734063363715652850229575637428545214370658, 3, 1,
            1984232051, 1301620570, 1751255046, 2066154700, 6,
            930909046950338848675671179033947720344577278394252394520228351960094703457,
            2518355010803493215236742676989937047358819576038995155770467812049853780302,
            1052099319509182016215622541805215392785322937638345954453047023790985619336,
            134724124743202229553669564186632778246758284203279875388625817255285523351,
            747228874504072031948012038628776919639938882961609484616335923189916306269,
            986490867278298294475423346828038671538855090104725084441125913874228017567, 0,
            995524627752196735661894212183970503203430507166601836968508900279938945086, 1,
            1458405795, 491359394, 695332103, 1156919555, 5,
            3391302796018267639958936865244251298561915156822537900630723309754711190171,
            97439209108437137515782522757217317923887919870294821783405862016101191265,
            1948531947904000762393255748180154743732564980365748228979683960050239935500,
            629549320318792956712638764191845454995553725249231776669667909539065755958,
            2434751759245803082581349704195895480432168147867347790967923582277723748886, 0,
            2149735103824111869949414413764166503122090836900441970959186989416467221143, 1,
            294882200, 730761129, 578956600, 36719417, 4,
            479514794246235700821866475623346973502891730541037606725184106895626377158,
            297309643017702014371545487431545507680516868776245181237311089465722014591,
            2127502746971593028564482961924309171918348094827433337314131210427373210306,
            2206301781526548311677458232607775332450862347745916668552172879217421510381, 0,
            2168899312563977570176790236694005793684040578094317937516080629846982916156, 4,
            92255467, 1174963615, 663113792, 2066585349, 92255467, 1174963615, 663113792,
            2066585349, 92255467, 1174963615, 663113792, 2066585349, 92255467, 1174963615,
            663113792, 2066585349, 2,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();

        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds);

        match verifier {
            Err(error) => match error {
                FriVerificationError::InvalidNumFriLayers => {},
                _ => panic!("wrong error"),
            },
            _ => panic!("should error"),
        }
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn proof_with_invalid_last_layer_degree_fails_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 2, log_blowup_factor: 2, n_queries: 3,
        };
        let column_log_bound = 4;
        let column_log_bounds = array![column_log_bound].span();
        let mut proof_data = array![
            3, 466407290, 0, 0, 0, 1657247602, 0, 0, 0, 183082621, 0, 0, 0, 6,
            2191983833872455433114432226011293834134487075743191836614597135553794517005,
            3464840636653936594197539435512969951836467536729265308502051557386128572008,
            222796975636755964353674243152293642953167138800260344804970874742438880851,
            1798533664940731406819278501050858084865711142717794910914770138137877766030,
            112736985517415102864396627290325395865021582833422635609580236825399909887,
            1734751601212898474594489984934132832871090066333664002442834978729904575960, 0,
            2062842123560885137372604875777049649036399148319124894222012291287950136814, 1, 3,
            141033582, 2113260302, 1238696787, 1899978666, 868401741, 698045836, 214810831,
            1288148988, 917728605, 296711650, 1163674366, 578376068, 3,
            1296391960212195064330229477331179473327220696707113566862317562405116506156,
            1126517304007438099452497759147854255367428269270252391844854672474276877173,
            1617597733625675487228439398588025564892271376782836689059755775897834180556, 0,
            1983648368674016433209849425227719906723907079145392260477967429097269024313, 8, 1, 0,
            0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0,
            0, 3,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();

        let verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds);

        match verifier {
            // TODO(andrew): Support `Err(FriVerificationError::LastLayerDegreeInvalid)`
            // to be consistent with Rust.
            Err(err) => match err {
                FriVerificationError::LastLayerDegreeInvalid => {},
                _ => panic!("wrong error"),
            },
            _ => panic!("should error"),
        }
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[ignore]
    fn proof_with_invalid_last_layer_fails_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 2, log_blowup_factor: 2, n_queries: 3,
        };
        let column_log_bound = 4;
        let column_log_size = column_log_bound + config.log_blowup_factor;
        let column_log_bounds = array![column_log_bound].span();
        let queries = Queries {
            positions: array![1, 7, 8].span(), log_domain_size: column_log_size,
        };
        let query_evals = array![
            array![
                qm31_const::<127986842, 0, 0, 0>(), qm31_const::<1816542136, 0, 0, 0>(),
                qm31_const::<18610701, 0, 0, 0>(),
            ]
                .span(),
        ];
        let mut proof_data = array![
            3, 466407290, 0, 0, 0, 1657247602, 0, 0, 0, 183082621, 0, 0, 0, 6,
            2191983833872455433114432226011293834134487075743191836614597135553794517005,
            3464840636653936594197539435512969951836467536729265308502051557386128572008,
            222796975636755964353674243152293642953167138800260344804970874742438880851,
            1798533664940731406819278501050858084865711142717794910914770138137877766030,
            112736985517415102864396627290325395865021582833422635609580236825399909887,
            1734751601212898474594489984934132832871090066333664002442834978729904575960, 0,
            2062842123560885137372604875777049649036399148319124894222012291287950136814, 1, 3,
            141033582, 2113260302, 1238696787, 1899978666, 868401741, 698045836, 214810831,
            1288148988, 917728605, 296711650, 1163674366, 578376068, 3,
            1296391960212195064330229477331179473327220696707113566862317562405116506156,
            1126517304007438099452497759147854255367428269270252391844854672474276877173,
            1617597733625675487228439398588025564892271376782836689059755775897834180556, 0,
            1983648368674016433209849425227719906723907079145392260477967429097269024313, 4,
            129445544, 2099793146, 401612806, 286217047, 129445543, 2099793146, 401612806,
            286217047, 129445543, 2099793146, 401612806, 286217047, 129445543, 2099793146,
            401612806, 286217047, 2,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let mut verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();

        let verification_result = verifier.decommit_on_queries(queries, query_evals);

        match verification_result {
            Err(err) => match err {
                FriVerificationError::LastLayerEvaluationsInvalid => {},
                _ => panic!("wrong error"),
            },
            _ => panic!("should error"),
        }
    }

    // TODO(andrew): Add back in with new proof data.
    #[test]
    #[should_panic]
    #[ignore]
    fn decommit_queries_on_invalid_domain_fails_verification() {
        let config = FriConfig {
            log_last_layer_degree_bound: 1, log_blowup_factor: 2, n_queries: 1,
        };
        let column_log_bound = 3;
        let column_log_size = column_log_bound + config.log_blowup_factor;
        let column_log_bounds = array![column_log_bound].span();
        let invalid_column_log_size = column_log_size - 1;
        let invalid_queries = Queries {
            positions: array![5].span(), log_domain_size: invalid_column_log_size,
        };
        let query_evals = array![array![qm31_const::<1059056252, 0, 0, 0>()].span()];
        let mut proof_data = array![
            1, 520313910, 0, 0, 0, 4,
            2511859689460956062754798212736603971289039205668758416916441010091827960967,
            3116447051172487019914330687758294186696708622072747727020227859480276128456,
            721698868812272649654951509597754221353628194583368094837414807957240128187,
            2322781146978847693424724097972151857550932350708357827088012624999661465745, 0,
            748174431875540733373318683157905446208709800770031351438402873811128343902, 1, 1,
            822799490, 762931150, 1156162632, 2122921032, 3,
            2682605036744925837528395024963644910241648916416714246021123266777932169829,
            2897190808213678224361292969431925141623148330898699376650599616824548437798,
            1696260486512635605141370500725283204493308222655042278770397536250642854003, 0,
            2532324905130335387885988086730858190821881679074630420916383382420280214544, 2,
            1901985872, 2023841224, 1529482466, 2072637482, 1901985872, 2023841224, 1529482466,
            2072637482, 1,
        ]
            .span();
        let proof = Serde::deserialize(ref proof_data).unwrap();
        let mut channel: Channel = Default::default();
        let mut verifier = FriVerifierImpl::commit(ref channel, config, proof, column_log_bounds)
            .unwrap();

        verifier.decommit_on_queries(invalid_queries, query_evals).unwrap();
    }
}

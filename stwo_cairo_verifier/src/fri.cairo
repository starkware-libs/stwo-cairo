use stwo_cairo_verifier::fields::m31::M31Trait;
use stwo_cairo_verifier::circle::{Coset, CosetImpl};
use stwo_cairo_verifier::poly::line::{LineDomain, LineDomainImpl};
use stwo_cairo_verifier::fields::qm31::{QM31, qm31, QM31Trait};
use stwo_cairo_verifier::queries::SparseSubCircleDomain;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::utils::{bit_reverse_index, pow, pow_qm31, qm31_zero_array, find};
use stwo_cairo_verifier::poly::line::{
    LineEvaluation, LineEvaluationImpl, SparseLineEvaluation, SparseLineEvaluationImpl
};
use stwo_cairo_verifier::poly::circle::{
    CircleEvaluation, SparseCircleEvaluation, SparseCircleEvaluationImpl
};
use stwo_cairo_verifier::poly::circle::{CircleDomain, CircleDomainImpl};
use stwo_cairo_verifier::queries::{Queries, QueriesImpl};
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use stwo_cairo_verifier::vcs::hasher::PoseidonMerkleHasher;
use stwo_cairo_verifier::poly::line::{LinePoly, LinePolyImpl};
use stwo_cairo_verifier::channel::{Channel, ChannelTrait};

pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;
pub const FOLD_STEP: u32 = 1;

#[derive(Debug, Drop, PartialEq)]
pub enum FriVerificationError {
    InvalidNumFriLayers,
    LastLayerDegreeInvalid,
    LastLayerEvaluationsInvalid,
    InnerLayerEvaluationsInvalid,
    InnerLayerCommitmentInvalid
}

#[derive(Drop, Debug)]
struct FriLayerVerifier {
    degree_bound: u32,
    domain: LineDomain,
    folding_alpha: QM31,
    layer_index: usize,
    proof: @FriLayerProof,
}

#[generate_trait]
impl FriLayerVerifierImpl of FriLayerVerifierTrait {
    /// Verifies the layer's merkle decommitment and returns the the folded queries and query evals.
    fn verify_and_fold(
        self: @FriLayerVerifier, queries: @Queries, evals_at_queries: @Array<QM31>
    ) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        let commitment = self.proof.commitment;

        let sparse_evaluation = @self.extract_evaluation(queries, evals_at_queries)?;
        let mut column_0: Array<M31> = array![];
        let mut column_1: Array<M31> = array![];
        let mut column_2: Array<M31> = array![];
        let mut column_3: Array<M31> = array![];
        let mut i = 0;
        while i < (sparse_evaluation).subline_evals.len() {
            let mut j = 0;
            let subline_eval = sparse_evaluation.subline_evals[i];
            while j < (sparse_evaluation.subline_evals[i]).values.len() {
                let [x0, x1, x2, x3] = (*subline_eval.values[j]).to_array();

                column_0.append(x0);
                column_1.append(x1);
                column_2.append(x2);
                column_3.append(x3);
                j += 1;
            };
            i += 1;
        };
        let actual_decommitment_array = array![
            column_0.span(), column_1.span(), column_2.span(), column_3.span()
        ];

        let folded_queries = queries.fold(FOLD_STEP);
        let folded_queries_snapshot = @folded_queries;

        let mut decommitment_positions = array![];
        let mut i = 0;
        while i < folded_queries_snapshot.len() {
            let start = *folded_queries_snapshot.positions[i] * pow(2, FOLD_STEP);
            let end = start + pow(2, FOLD_STEP);
            let mut j = start;
            while j < end {
                decommitment_positions.append(j);
                j += 1;
            };
            i += 1;
        };

        let merkle_verifier = MerkleVerifier {
            root: *commitment.clone(),
            column_log_sizes: array![
                // TODO: adapt to handle other secure_extension_degree
                self.domain.log_size(),
                self.domain.log_size(),
                self.domain.log_size(),
                self.domain.log_size()
            ]
        };

        let mut queries_per_log_size: Felt252Dict<Nullable<Array<usize>>> = Default::default();
        queries_per_log_size
            .insert(self.domain.log_size().into(), NullableTrait::new(decommitment_positions));

        let decommitment = self.proof.decommitment.clone();
        let result = merkle_verifier
            .verify(queries_per_log_size, actual_decommitment_array, decommitment.clone());

        let evals_at_folded_queries = sparse_evaluation.fold(*self.folding_alpha);
        match result {
            Result::Ok(()) => Result::Ok((folded_queries, evals_at_folded_queries)),
            Result::Err(_) => Result::Err(FriVerificationError::InnerLayerCommitmentInvalid)
        }
    }

    /// Returns the evaluations needed for decommitment.
    fn extract_evaluation(
        self: @FriLayerVerifier, queries: @Queries, evals_at_queries: @Array<QM31>
    ) -> Result<SparseLineEvaluation, FriVerificationError> {
        let mut all_subline_evals: Array<LineEvaluation> = array![];

        // Index of the evals provided by the verifier.
        let mut evals_at_queries_index = 0;

        // Index of the evals stored in the proof.
        let mut proof_evals_index = 0;

        // Group queries by the subline they reside in.
        let mut error = false;
        let mut i = 0;
        while i < queries.positions.len() {
            // In this step we'll work over queries.positions between
            // start_subline_index and end_subline_index, which group
            // the queries.
            let start_subline_index = i;
            i += 1;
            while i < queries.positions.len()
                && (*queries.positions[i
                    - 1] / pow(2, FOLD_STEP)) == (*queries.positions[i] / pow(2, FOLD_STEP)) {
                i = i + 1;
            };
            let end_subline_index = i;

            // These are the values whose evaluations are required.
            let subline_start = (*queries.positions[start_subline_index] / pow(2, FOLD_STEP))
                * pow(2, FOLD_STEP);
            let subline_end = subline_start + pow(2, FOLD_STEP);

            let mut subline_evals: Array<QM31> = array![];

            let mut j = start_subline_index;
            let mut eval_position = subline_start;

            while eval_position < subline_end {
                if (j < end_subline_index) && (*queries.positions[j] == eval_position) {
                    subline_evals.append(evals_at_queries[evals_at_queries_index].clone());

                    evals_at_queries_index += 1;
                    j += 1;
                } else {
                    if (proof_evals_index < (*self.proof.evals_subset).len()) {
                        subline_evals.append((*self.proof.evals_subset)[proof_evals_index].clone());
                        proof_evals_index += 1;
                    } else {
                        error = true;
                        break;
                    }
                }
                eval_position += 1;
            };

            if error {
                break;
            }

            let subline_initial_index = bit_reverse_index(subline_start, self.domain.log_size());
            let subline_initial = self.domain.coset.index_at(subline_initial_index);
            let subline_domain = LineDomainImpl::new(CosetImpl::new(subline_initial, FOLD_STEP));

            all_subline_evals.append(LineEvaluationImpl::new(subline_domain, subline_evals));
        };

        if error || proof_evals_index != (*self.proof.evals_subset).len() {
            return Result::Err(FriVerificationError::InnerLayerEvaluationsInvalid);
        }

        Result::Ok(SparseLineEvaluation { subline_evals: all_subline_evals })
    }
}

#[derive(Clone, Copy, Drop, Debug)]
pub struct FriConfig {
    pub log_blowup_factor: u32,
    pub log_last_layer_degree_bound: usize,
    pub n_queries: usize,
}


/// Stores a subset of evaluations in a fri layer with their corresponding merkle decommitments.
///
/// The subset corresponds to the set of evaluations needed by a FRI verifier.
#[derive(Drop, Clone, Debug)]
pub struct FriLayerProof {
    pub evals_subset: Array<QM31>,
    pub decommitment: MerkleDecommitment::<PoseidonMerkleHasher>,
    pub commitment: felt252,
}

#[derive(Drop)]
pub struct FriProof {
    pub inner_layers: Array<FriLayerProof>,
    pub last_layer_poly: LinePoly,
}

#[derive(Drop, Debug)]
pub struct FriVerifier {
    config: FriConfig,
    circle_poly_alpha: QM31,
    expected_query_log_domain_size: u32,
    column_bounds: Array<u32>,
    inner_layers: Array<FriLayerVerifier>,
    last_layer_domain: LineDomain,
    last_layer_poly: LinePoly,
    queries: Option<Queries>,
}

#[generate_trait]
pub impl FriVerifierImpl of FriVerifierTrait {
    /// Verifies the commitment stage of FRI.
    ///
    /// `column_bounds` should be the committed circle polynomial degree bounds in descending order.
    fn commit(
        ref channel: Channel, config: FriConfig, proof: FriProof, column_bounds: Array<u32>
    ) -> Result<FriVerifier, FriVerificationError> {
        let max_column_bound = column_bounds[0];
        let expected_query_log_domain_size = *max_column_bound + config.log_blowup_factor;
        let circle_poly_alpha = channel.draw_felt();

        let mut inner_layers = array![];
        let mut layer_bound = *max_column_bound - CIRCLE_TO_LINE_FOLD_STEP;
        let mut layer_domain = LineDomain {
            coset: CosetImpl::new(
                pow(2, 31 - layer_bound - config.log_blowup_factor - 2),
                layer_bound + config.log_blowup_factor
            )
        };

        let mut layer_index = 0;
        let mut invalid_fri_layers_number = false;
        while layer_index < proof.inner_layers.len() {
            let proof = proof.inner_layers[layer_index];
            channel.mix_digest(*proof.commitment);
            let new_folding_alpha = channel.draw_felt();
            inner_layers
                .append(
                    FriLayerVerifier {
                        degree_bound: layer_bound,
                        domain: layer_domain,
                        folding_alpha: new_folding_alpha,
                        layer_index,
                        proof,
                    }
                );

            if layer_bound >= FOLD_STEP {
                layer_bound = layer_bound - FOLD_STEP;
            } else {
                invalid_fri_layers_number = true;
                break;
            }
            layer_domain = layer_domain.double();
            layer_index += 1;
        };

        if invalid_fri_layers_number {
            return Result::Err(FriVerificationError::InvalidNumFriLayers);
        }

        if layer_bound != config.log_last_layer_degree_bound {
            return Result::Err(FriVerificationError::InvalidNumFriLayers);
        }

        if proof.last_layer_poly.len() != pow(2, config.log_last_layer_degree_bound) {
            return Result::Err(FriVerificationError::LastLayerDegreeInvalid);
        }

        channel.mix_felts(proof.last_layer_poly.coeffs.span());

        Result::Ok(
            FriVerifier {
                config,
                circle_poly_alpha,
                column_bounds,
                expected_query_log_domain_size,
                inner_layers,
                last_layer_domain: layer_domain,
                last_layer_poly: proof.last_layer_poly,
                queries: Option::None
            }
        )
    }

    /// Verifies the decommitment stage of FRI.
    ///
    /// The decommitment values need to be provided in the same order as their commitment.
    fn decommit(
        self: @FriVerifier, decommitted_values: Array<SparseCircleEvaluation>,
    ) -> Result<(), FriVerificationError> {
        let queries = if let Option::Some(queries_snapshot) = self.queries {
            Option::Some(queries_snapshot)
        } else {
            Option::None
        }
            .expect('queries not sampled');
        self.decommit_on_queries(queries, decommitted_values)
    }

    fn decommit_on_queries(
        self: @FriVerifier, queries: @Queries, decommitted_values: Array<SparseCircleEvaluation>
    ) -> Result<(), FriVerificationError> {
        assert_eq!(queries.log_domain_size, self.expected_query_log_domain_size);

        let (last_layer_queries, last_layer_query_evals) = self
            .decommit_inner_layers(queries, @decommitted_values)?;
        self.decommit_last_layer(last_layer_queries, last_layer_query_evals)
    }

    /// Verifies all inner layer decommitments.
    ///
    /// Returns the queries and query evaluations needed for verifying the last FRI layer.
    fn decommit_inner_layers(
        self: @FriVerifier, queries: @Queries, decommitted_values: @Array<SparseCircleEvaluation>
    ) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        let circle_poly_alpha = self.circle_poly_alpha;
        let circle_poly_alpha_sq = *circle_poly_alpha * *circle_poly_alpha;

        let mut layer_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);
        let mut layer_query_evals = qm31_zero_array(layer_queries.len());

        let mut inner_layers_index = 0;
        let mut column_bound_index = 0;
        loop {
            if inner_layers_index == self.inner_layers.len() {
                // TODO: remove clones
                break Result::Ok((layer_queries.clone(), layer_query_evals.clone()));
            }

            let current_layer = self.inner_layers[inner_layers_index];
            if column_bound_index < self.column_bounds.len()
                && *self.column_bounds[column_bound_index]
                - CIRCLE_TO_LINE_FOLD_STEP == *current_layer.degree_bound {
                let mut n_columns_in_layer = 1;
                // TODO: remove clone?
                let mut combined_sparse_evals = decommitted_values[column_bound_index].clone();

                column_bound_index += 1;

                while column_bound_index < self.column_bounds.len()
                    && *self.column_bounds[column_bound_index]
                    - CIRCLE_TO_LINE_FOLD_STEP == *current_layer.degree_bound {
                    combined_sparse_evals = combined_sparse_evals
                        .accumulate(decommitted_values[column_bound_index], circle_poly_alpha_sq);
                    column_bound_index += 1;
                    n_columns_in_layer += 1;
                };

                let folded_evals = combined_sparse_evals.fold(*circle_poly_alpha);
                let prev_layer_combination_factor = pow_qm31(
                    circle_poly_alpha_sq, n_columns_in_layer
                );

                let mut k = 0;
                let mut new_layer_query_evals: Array<QM31> = array![];
                assert!(folded_evals.len() == layer_query_evals.len());
                while k < folded_evals.len() {
                    new_layer_query_evals
                        .append(
                            *layer_query_evals[k] * prev_layer_combination_factor + *folded_evals[k]
                        );
                    k += 1;
                };
                layer_query_evals = new_layer_query_evals;
            }

            let result = current_layer.verify_and_fold(@layer_queries, @layer_query_evals);
            if result.is_err() {
                break result;
            } else {
                let (new_layer_queries, new_layer_query_evals) = result.unwrap();
                layer_queries = new_layer_queries;
                layer_query_evals = new_layer_query_evals;
            }
            inner_layers_index += 1;
        }
    }

    /// Verifies the last layer.
    fn decommit_last_layer(
        self: @FriVerifier, queries: Queries, query_evals: Array<QM31>,
    ) -> Result<(), FriVerificationError> {
        let mut failed = false;
        let mut i = 0;
        while i < queries.positions.len() {
            let query = queries.positions[i];
            let domain = self.last_layer_domain;
            let x = self.last_layer_domain.at(bit_reverse_index(*query, domain.log_size()));

            if *query_evals[i] != self.last_layer_poly.eval_at_point(x.into()) {
                failed = true;
                break;
            }
            i += 1;
        };
        if failed {
            return Result::Err(FriVerificationError::LastLayerEvaluationsInvalid);
        } else {
            Result::Ok(())
        }
    }

    /// Samples queries and returns the opening positions for each unique column size.
    ///
    /// The order of the opening positions corresponds to the order of the column commitment.
    fn column_query_positions(
        ref self: FriVerifier, ref channel: Channel
    ) -> (Felt252Dict<Nullable<SparseSubCircleDomain>>, Span<u32>) {
        let queries = QueriesImpl::generate(
            ref channel,
            *self.column_bounds[0] + self.config.log_blowup_factor,
            self.config.n_queries
        );
        self.queries = Option::Some(queries.clone());

        let mut column_log_sizes = array![];
        let mut i = 0;
        let column_bounds_snap = @self.column_bounds;

        while i < column_bounds_snap.len() {
            let v = *(column_bounds_snap.at(i)) + self.config.log_blowup_factor;
            if (!find(v, column_log_sizes.span())) {
                column_log_sizes.append(v);
            }
            i = i + 1;
        };

        (get_opening_positions(@queries, column_log_sizes.span()), column_log_sizes.span())
    }
}

/// Returns the column opening positions needed for verification.
///
/// The column log sizes must be unique and in descending order. Returned
/// column opening positions are mapped by their log size.
fn get_opening_positions(
    queries: @Queries, column_log_sizes: Span<u32>,
) -> Felt252Dict<Nullable<SparseSubCircleDomain>> {
    let mut prev_log_size = column_log_sizes[0];
    assert!(prev_log_size == queries.log_domain_size);
    let mut prev_queries = queries.clone();
    let mut positions: Felt252Dict<Nullable<SparseSubCircleDomain>> = Default::default();
    let felt_prev: core::felt252 = (*prev_log_size).into();
    positions.insert(felt_prev, NullableTrait::new(prev_queries.opening_positions(FOLD_STEP)));

    let mut i = 1;
    while i < column_log_sizes.len() {
        let n_folds = *prev_log_size - *column_log_sizes.at(i);
        let queries = prev_queries.fold(n_folds);
        let felt_column_log_sizes: core::felt252 = (*column_log_sizes.at(i)).into();
        positions
            .insert(
                felt_column_log_sizes, NullableTrait::new(queries.opening_positions(FOLD_STEP))
            );
        prev_log_size = column_log_sizes.at(i);
        prev_queries = queries;
        i = i + 1;
    };
    positions
}

/// Folds a degree `d` polynomial into a degree `d/2` polynomial.
///
/// Let `eval` be a polynomial evaluated on line domain `E`, `alpha` be a random field
/// element and `pi(x) = 2x^2 - 1` be the circle's x-coordinate doubling map. This function
/// returns `f' = f0 + alpha * f1` evaluated on `pi(E)` such that `2f(x) = f0(pi(x)) + x *
/// f1(pi(x))`.
pub fn fold_line(eval: @LineEvaluation, alpha: QM31) -> LineEvaluation {
    let domain = eval.domain;
    let mut values = array![];
    let mut i = 0;
    while i < eval.values.len() / 2 {
        let x = domain.at(bit_reverse_index(i * pow(2, FOLD_STEP), domain.log_size()));
        let f_x = eval.values[2 * i];
        let f_neg_x = eval.values[2 * i + 1];
        let (f0, f1) = ibutterfly(*f_x, *f_neg_x, x.inverse());
        values.append(f0 + alpha * f1);
        i += 1;
    };
    LineEvaluationImpl::new(domain.double(), values)
}

/// Folds and accumulates a degree `d` circle polynomial into a degree `d/2` univariate
/// polynomial.
///
/// Let `src` be the evaluation of a circle polynomial `f` on a
/// [`CircleDomain`] `E`. This function computes evaluations of `f' = f0
/// + alpha * f1` on the x-coordinates of `E` such that `2f(p) = f0(px) + py * f1(px)`. The
/// evaluations of `f'` are accumulated into `dst` by the formula `dst = dst * alpha^2 +
/// f'`.
pub fn fold_circle_into_line(eval: @CircleEvaluation, alpha: QM31) -> LineEvaluation {
    let domain = eval.domain;
    let mut values = array![];
    let mut i = 0;
    while i < eval.values.len() / 2 {
        let p = domain
            .at(bit_reverse_index(i * pow(2, CIRCLE_TO_LINE_FOLD_STEP), domain.log_size()));
        let f_p = eval.values[2 * i];
        let f_neg_p = eval.values[2 * i + 1];
        let (f0, f1) = ibutterfly(*f_p, *f_neg_p, p.y.inverse());
        values.append(f0 + alpha * f1);
        i += 1;
    };
    LineEvaluation { values, domain: LineDomainImpl::new(*domain.half_coset) }
}

pub fn ibutterfly(v0: QM31, v1: QM31, itwid: M31) -> (QM31, QM31) {
    (v0 + v1, (v0 - v1).mul_m31(itwid))
}

#[cfg(test)]
mod test {
    use stwo_cairo_verifier::poly::line::{
        LineEvaluation, SparseLineEvaluation, SparseLineEvaluationImpl
    };
    use stwo_cairo_verifier::fields::m31::M31Trait;
    use stwo_cairo_verifier::circle::{Coset, CosetImpl};
    use stwo_cairo_verifier::poly::line::{LineDomain, LineDomainImpl};
    use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
    use stwo_cairo_verifier::fields::m31::M31;
    use stwo_cairo_verifier::utils::{bit_reverse_index, pow};
    use stwo_cairo_verifier::poly::circle::{
        CircleEvaluation, CircleEvaluationTrait, CircleDomain, CircleEvaluationImpl,
        SparseCircleEvaluation, SparseCircleEvaluationImpl
    };
    use stwo_cairo_verifier::queries::{Queries, QueriesImpl};
    use stwo_cairo_verifier::channel::ChannelTrait;
    use stwo_cairo_verifier::poly::line::LinePoly;
    use stwo_cairo_verifier::vcs::verifier::MerkleDecommitment;
    use super::{
        FOLD_STEP, CIRCLE_TO_LINE_FOLD_STEP, FriConfig, FriProof, FriVerifierImpl,
        FriVerificationError, FriLayerProof
    };

    #[test]
    fn test_fold_line_1() {
        let domain = LineDomainImpl::new(CosetImpl::new(67108864, 1));
        let values = array![
            qm31(440443058, 1252794525, 1129773609, 1309365757),
            qm31(974589897, 1592795796, 649052897, 863407657)
        ];
        let sparse_line_evaluation = SparseLineEvaluation {
            subline_evals: array![LineEvaluation { values, domain }]
        };
        let alpha = qm31(1047716961, 506143067, 1065078409, 990356067);

        let result = sparse_line_evaluation.fold(alpha);

        assert_eq!(result, array![qm31(515899232, 1030391528, 1006544539, 11142505)]);
    }

    #[test]
    fn test_fold_line_2() {
        let domain = LineDomainImpl::new(CosetImpl::new(553648128, 1));
        let values = array![
            qm31(730692421, 1363821003, 2146256633, 106012305),
            qm31(1387266930, 149259209, 1148988082, 1930518101)
        ];
        let sparse_line_evaluation = SparseLineEvaluation {
            subline_evals: array![LineEvaluation { values, domain }]
        };
        let alpha = qm31(2084521793, 1326105747, 548635876, 1532708504);

        let result = sparse_line_evaluation.fold(alpha);

        assert_eq!(result, array![qm31(1379727866, 1083096056, 1409020369, 1977903500)]);
    }

    #[test]
    fn test_fold_circle_into_line_1() {
        let domain = CircleDomain { half_coset: CosetImpl::new(553648128, 0) };
        let values = array![qm31(807167738, 0, 0, 0), qm31(1359821401, 0, 0, 0)];
        let sparse_circle_evaluation: SparseCircleEvaluation = SparseCircleEvaluation {
            subcircle_evals: array![CircleEvaluationImpl::new(domain, values)]
        };
        let alpha = qm31(260773061, 362745443, 1347591543, 1084609991);
        let result = sparse_circle_evaluation.fold(alpha);
        let expected_result = array![qm31(730692421, 1363821003, 2146256633, 106012305)];
        assert_eq!(expected_result, result);
    }

    type ProofValues = (FriConfig, FriProof, Array<u32>, Queries, Array<SparseCircleEvaluation>);

    #[test]
    fn valid_proof_with_constant_last_layer_passes_verification() {
        let (config, proof, bounds, queries, decommitted_values) = proof_with_constant_last_layer();

        let mut channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

        verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
    }

    #[test]
    fn valid_proof_passes_verification() {
        let (config, proof, bounds, queries, decommitted_values) = proof_with_linear_last_layer();

        let mut channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

        verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
    }

    #[test]
    fn valid_mixed_degree_proof_passes_verification() {
        let (config, proof, bounds, queries, decommitted_values) = proof_with_mixed_degree_1();

        let mut channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

        verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
    }

    #[test]
    fn valid_mixed_degree_end_to_end_proof_passes_verification() {
        let (config, proof, bounds, decommitted_values) = proof_with_mixed_degree_2();
        let mut channel = ChannelTrait::new(0x00);
        let mut verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds).unwrap();

        let mut channel = ChannelTrait::new(0x00);
        let (_, _) = verifier.column_query_positions(ref channel);

        verifier.decommit(decommitted_values).unwrap();
    }

    #[test]
    fn proof_with_removed_layer_fails_verification() {
        let (config, proof, bounds, _queries, _decommitted_values) = proof_with_mixed_degree_1();

        let mut invalid_config = config;
        invalid_config.log_last_layer_degree_bound -= 1;

        let mut channel = ChannelTrait::new(0x00);
        let result = FriVerifierImpl::commit(ref channel, invalid_config, proof, bounds);

        match result {
            Result::Ok(_) => { panic!("Verifier should return InvalidNumFriLayers"); },
            Result::Err(error) => { assert!(error == FriVerificationError::InvalidNumFriLayers); }
        }
    }

    #[test]
    fn proof_with_added_layer_fails_verification() {
        let (config, proof, bounds, _queries, _decommitted_values) = proof_with_mixed_degree_1();

        let mut invalid_config = config;
        invalid_config.log_last_layer_degree_bound += 1;

        let mut channel = ChannelTrait::new(0x00);
        let result = FriVerifierImpl::commit(ref channel, invalid_config, proof, bounds);

        match result {
            Result::Ok(_) => { panic!("Verifier should return InvalidNumFriLayers"); },
            Result::Err(error) => { assert!(error == FriVerificationError::InvalidNumFriLayers); }
        }
    }

    #[test]
    fn proof_with_invalid_inner_layer_evaluation_fails_verification() {
        let (config, proof, bounds, queries, decommitted_values) =
            proof_with_last_layer_of_degree_four();

        // Create an invalid proof by removing an evaluation from the second layer's proof
        let inner_layers = @proof.inner_layers;
        let invalid_proof = {
            let mut invalid_inner_layers = array![];
            // Keep the first layer as it is
            invalid_inner_layers.append(proof.inner_layers[0].clone());

            // Modify the second layer
            let mut invalid_evals_subset = array![];
            let mut i = 1;
            while i < inner_layers[1].evals_subset.len() {
                invalid_evals_subset.append(inner_layers[1].evals_subset[i].clone());
                i += 1;
            };
            invalid_inner_layers
                .append(
                    FriLayerProof {
                        evals_subset: invalid_evals_subset,
                        decommitment: inner_layers[1].decommitment.clone(),
                        commitment: *inner_layers[1].commitment
                    }
                );

            // Keep the rest of the layers as they are
            let mut i = 2;
            while i < proof.inner_layers.len() {
                invalid_inner_layers.append(proof.inner_layers[i].clone());
                i += 1;
            };

            FriProof {
                inner_layers: invalid_inner_layers, last_layer_poly: proof.last_layer_poly.clone()
            }
        };

        let mut channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(ref channel, config, invalid_proof, bounds.clone())
            .unwrap();
        let verification_result = verifier.decommit_on_queries(@queries, decommitted_values);

        match verification_result {
            Result::Ok(_) => { panic!("Verifier should return InnerLayerEvaluationsInvalid"); },
            Result::Err(error) => {
                assert!(error == FriVerificationError::InnerLayerEvaluationsInvalid);
            }
        }
    }

    #[test]
    fn proof_with_invalid_inner_layer_decommitment_fails_verification() {
        let (config, proof, bounds, queries, decommitted_values) =
            proof_with_last_layer_of_degree_four();

        // Create an invalid proof by modifying the committed values in the second layer.
        let inner_layers = @proof.inner_layers;
        let invalid_proof = {
            let mut invalid_inner_layers = array![];
            // Keep the first layer as it is
            invalid_inner_layers.append(proof.inner_layers[0].clone());

            // Modify the second layer
            let mut invalid_evals_subset = array![
                *inner_layers[1].evals_subset[0] + qm31(1, 0, 0, 0)
            ];
            let mut i = 1;
            while i < inner_layers[1].evals_subset.len() {
                invalid_evals_subset.append(inner_layers[1].evals_subset[i].clone());
                i += 1;
            };
            invalid_inner_layers
                .append(
                    FriLayerProof {
                        evals_subset: invalid_evals_subset,
                        decommitment: inner_layers[1].decommitment.clone(),
                        commitment: *inner_layers[1].commitment
                    }
                );

            // Keep the rest of the layers as they are
            let mut i = 2;
            while i < proof.inner_layers.len() {
                invalid_inner_layers.append(proof.inner_layers[i].clone());
                i += 1;
            };

            FriProof {
                inner_layers: invalid_inner_layers, last_layer_poly: proof.last_layer_poly.clone()
            }
        };
        let mut channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(ref channel, config, invalid_proof, bounds).unwrap();

        let verification_result = verifier.decommit_on_queries(@queries, decommitted_values);

        match verification_result {
            Result::Ok(_) => { panic!("Verifier should return InnerLayerCommitmentInvalid"); },
            Result::Err(error) => {
                assert!(error == FriVerificationError::InnerLayerCommitmentInvalid);
            }
        }
    }

    #[test]
    fn proof_with_invalid_last_layer_degree_fails_verification() {
        let (config, mut proof, bounds, _, _) = proof_with_last_layer_of_degree_four();

        proof
            .last_layer_poly =
                LinePoly {
                    coeffs: array![qm31(1, 0, 0, 0), qm31(1, 0, 0, 0)], ..proof.last_layer_poly
                };

        let mut channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(ref channel, config, proof, bounds);
        match verifier {
            Result::Ok(_) => { panic!("Verifier should return LastLayerDegreeInvalid"); },
            Result::Err(error) => {
                assert!(error == FriVerificationError::LastLayerDegreeInvalid);
            }
        }
    }

    #[should_panic]
    #[test]
    fn decommit_queries_on_invalid_domain_fails_verification() {
        let (config, proof, bounds, queries, decommitted_values) =
            proof_with_last_layer_of_degree_four();

        let mut invalid_queries = queries;
        invalid_queries.log_domain_size -= 1;

        let mut channel = ChannelTrait::new(0x00);
        let verifier_result = FriVerifierImpl::commit(ref channel, config, proof, bounds);
        match verifier_result {
            Result::Ok(verifier) => {
                verifier.decommit_on_queries(@invalid_queries, decommitted_values).unwrap();
            },
            Result::Err(_) => {}
        }
    }

    //////////////////////////////////////////////////////
    // Proofs extracted from Stwo's rust implementation //
    //////////////////////////////////////////////////////

    fn proof_with_constant_last_layer() -> ProofValues {
        let config = FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 0, n_queries: 1
        };

        let proof = FriProof {
            inner_layers: array![
                FriLayerProof {
                    evals_subset: array![qm31(1654551922, 1975507039, 724492960, 302041406)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x02894fb64f5b5ad74ad6868ded445416d52840c2c4a36499f0eb37a03841bfc8,
                            0x05d3f79e2cfd15b605e1e8eb759aa79e775e89df7c4ae5966efe3b96d3554003
                        ],
                        column_witness: array![]
                    },
                    commitment: 0x03e5bad5822d062c05ff947d282dc2d56a6a420d14f2f74972bb5b01287731a7
                },
                FriLayerProof {
                    evals_subset: array![qm31(1396531676, 750161390, 1275165237, 1824394799)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x0539eb6bd5d99019f938130703ddfd97aaa9f46dea9714f9ed75528babb4db55
                        ],
                        column_witness: array![]
                    },
                    commitment: 0x078189f0ad5c044994f4b3100183203ed10545891f2459770dde4af4b9c2def7
                }
            ],
            last_layer_poly: LinePoly {
                coeffs: array![qm31(1030963115, 122157260, 1848484002, 1387601044)], log_size: 0
            }
        };
        let bounds = array![3];

        let queries = Queries { positions: array![5], log_domain_size: 4 };
        let domain = CircleDomain {
            half_coset: Coset { initial_index: 603979776, step_size: 2147483648, log_size: 0 }
        };
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        domain, array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)]
                    )
                ]
            }
        ];

        (config, proof, bounds, queries, decommitted_values)
    }

    fn proof_with_linear_last_layer() -> ProofValues {
        let config = FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 1, n_queries: 1
        };

        let proof = FriProof {
            inner_layers: array![
                FriLayerProof {
                    evals_subset: array![qm31(1654551922, 1975507039, 724492960, 302041406)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x02894fb64f5b5ad74ad6868ded445416d52840c2c4a36499f0eb37a03841bfc8,
                            0x05d3f79e2cfd15b605e1e8eb759aa79e775e89df7c4ae5966efe3b96d3554003
                        ],
                        column_witness: array![]
                    },
                    commitment: 0x03e5bad5822d062c05ff947d282dc2d56a6a420d14f2f74972bb5b01287731a7
                },
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(1166420758, 1481024254, 705780805, 948549530),
                    qm31(1166420758, 1481024254, 705780805, 948549530)
                ],
                log_size: 1
            }
        };
        let bounds = array![3];

        let queries = Queries { positions: array![5], log_domain_size: 4 };
        let domain = CircleDomain {
            half_coset: Coset { initial_index: 603979776, step_size: 2147483648, log_size: 0 }
        };
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        domain, array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)]
                    )
                ]
            }
        ];

        (config, proof, bounds, queries, decommitted_values)
    }

    fn proof_with_last_layer_of_degree_four() -> ProofValues {
        let config = FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 1
        };
        let proof = FriProof {
            inner_layers: array![
                FriLayerProof {
                    evals_subset: array![qm31(421951112, 668736057, 785571716, 551382471),],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x07434e99a997fed5183f02e248b2d77ce047e45a63418dd8039630b139d72487,
                            0x01e1aafd718c486b5e9b35927b27a6eb71ef97cdc7009c9f246647db63a7960c,
                            0x0718cb047c50ba071b9a4696537695f273f42a7af8bfb0e465190b905548f754,
                            0x040db6d0f16909d1daaf710e3fa9663ef52419ac5ae5433c915ac5939809eb79,
                            0x06eb066c6e21999bc152bbac0a4b93c6c80b702f6ff7860be62cc10b89aa8352,
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc,
                },
                FriLayerProof {
                    evals_subset: array![qm31(1535376180, 1101522806, 788857635, 1882322924),],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x036ecb4e522350744312fa6da1ed85f6b7975885983a1baf9563feae7b7f799a,
                            0x017bdda6c344feddd93884211b626ca806da73bfa55cd7eef54b687dd744651a,
                            0x038d80d42b4192fd30dc894d5a26f3db757da5313c7940685058641091eb6d71,
                            0x0406355da40056abcf1278c92f3ab9aa52ca028fe437e6dbe15cdbcc7b83eed0,
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x046198bc34caa0b046234fa46ef591327a6864cb8a373bc13ce2cc9b3d5f3720,
                },
                FriLayerProof {
                    evals_subset: array![qm31(419894864, 130791138, 1485752547, 11937027),],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x0454d5cffc792c2308fb8dcf992c255f0535048b7bfbe9d08c1c3ae92178cd16,
                            0x071f311ea2e00f2e44066f0a577f27e62648b66152afa3122e0aebe7420fbcd2,
                            0x037c8315cf3525ea7097be7b687f44f9f0cecf1054ec553e183f0a9d2bd0b5d7,
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x0344b5796a1b37e154053d94532921bd1dc9db98b454d0a7537974e2b9fc17b5,
                },
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(1449311934, 1632038525, 278574869, 690369710),
                    qm31(1449311934, 1632038525, 278574869, 690369710),
                    qm31(1449311934, 1632038525, 278574869, 690369710),
                    qm31(1449311934, 1632038525, 278574869, 690369710),
                ],
                log_size: 2,
            },
        };
        let bounds = array![6];
        let queries = Queries { positions: array![5], log_domain_size: 7 };
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 545259520, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(1464849549, 0, 0, 0), qm31(35402781, 0, 0, 0),],
                    ),
                ],
            }
        ];
        (config, proof, bounds, queries, decommitted_values)
    }


    fn proof_with_mixed_degree_1() -> ProofValues {
        let config = FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 2
        };

        let proof = FriProof {
            inner_layers: array![
                FriLayerProof {
                    evals_subset: array![
                        qm31(1332072020, 1609661801, 1897498023, 686558487),
                        qm31(886239056, 1157828441, 2019876782, 1060063104)
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x07434e99a997fed5183f02e248b2d77ce047e45a63418dd8039630b139d72487,
                            0x020bcf949298f97180c360f6d55c2f65c19b9f3811c917d0368fe7203b53abcc,
                            0x01e1aafd718c486b5e9b35927b27a6eb71ef97cdc7009c9f246647db63a7960c,
                            0x062dd5d3993b66c78baf3608a2ed3de1ad865d0b174e006c8047b91fde98e462,
                            0x0718cb047c50ba071b9a4696537695f273f42a7af8bfb0e465190b905548f754,
                            0x055191c91b0668bab9271863162448c3396e8c2fc29f61bb621858210f4d0771,
                            0x040db6d0f16909d1daaf710e3fa9663ef52419ac5ae5433c915ac5939809eb79,
                            0x06ff62ebff373bc63508ad4c9c9997e38aa91331e1159c2809d81fd20b7a07e3
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(1263943078, 172354194, 2127081660, 1999316363),
                        qm31(1311532324, 582549508, 1702052122, 36581530)
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x05b7057376e8da41e7d1da285482571944f47848332279ce0de6b5ceeb21cb22,
                            0x00e3a1b78a35229bb9a60ad0d1bab478f52a087cd15c51dfa83cd47fc6bb7334,
                            0x013b2a8963d1de05e52484bf6e62fe25855780625d8e6f831cbac73801339267,
                            0x0163c94c52552862374f1d7b09036d7cf74b4d59914c4393503cfd9bc49d53d3,
                            0x0668d865abd1cb2b868c20784728cd48a6c4cbd926da318ce8814c5dae779fd0,
                            0x0774e25d9d61fc18f3c2a365213b81fd36cced1626e02afc4dbe4aef52021769,
                        ],
                        column_witness: array![]
                    },
                    commitment: 0x06a3f2b104508429f6b74edcd62044afb4f618302a382f281fee118b12dc9dbd
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(1660083285, 865580381, 2025493291, 1151079474),
                        qm31(24828450, 1304266370, 129024597, 1635057579)
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x03c70415b07af713627bd1405e284be50c30ce6b628fb6a7d2accd3bb631c04c,
                            0x062c36d3bd5fec84f54c5e835935a923db06521e937e3fbdd99cd9cd9701a329,
                            0x0414361ae7771b465d1ed7241c6a9c383e19cee6db3230e16164ded0da216c4d,
                            0x03abab172aeee1c04052395036cc50a51fb2497cfd307c96f32e718c4b3639cc
                        ],
                        column_witness: array![]
                    },
                    commitment: 0x03d2565deb5099be20df825aabfe4678a0922d6b4a988d23a553c9f06b5bf96e
                }
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                    qm31(1365318542, 1863705492, 1698090260, 381798840)
                ],
                log_size: 2
            }
        };
        let bounds = array![6, 5, 4];
        let queries = Queries { positions: array![7, 70], log_domain_size: 7 };
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1619001344, step_size: 2147483648, log_size: 0
                            }
                        },
                        array![qm31(83295654, 0, 0, 0), qm31(666640840, 0, 0, 0)]
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1652555776, step_size: 2147483648, log_size: 0
                            }
                        },
                        array![qm31(1598588979, 0, 0, 0), qm31(1615371031, 0, 0, 0)]
                    )
                ]
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1090519040, step_size: 2147483648, log_size: 0
                            }
                        },
                        array![qm31(985597997, 0, 0, 0), qm31(139496415, 0, 0, 0)]
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1157627904, step_size: 2147483648, log_size: 0
                            }
                        },
                        array![qm31(1718103579, 0, 0, 0), qm31(1537119660, 0, 0, 0)]
                    )
                ]
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 33554432, step_size: 2147483648, log_size: 0
                            }
                        },
                        array![qm31(1645691043, 0, 0, 0), qm31(2009531552, 0, 0, 0)]
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 167772160, step_size: 2147483648, log_size: 0
                            }
                        },
                        array![qm31(354887788, 0, 0, 0), qm31(934393698, 0, 0, 0)]
                    )
                ]
            }
        ];

        (config, proof, bounds, queries, decommitted_values)
    }

    fn proof_with_mixed_degree_2() -> (
        FriConfig, FriProof, Array<u32>, Array<SparseCircleEvaluation>
    ) {
        let config = FriConfig {
            log_blowup_factor: 1, log_last_layer_degree_bound: 2, n_queries: 3,
        };

        let proof = FriProof {
            inner_layers: array![
                FriLayerProof {
                    evals_subset: array![
                        qm31(1398603058, 1957874897, 461138270, 1700080921),
                        qm31(393493522, 576752954, 1963336729, 1268892468),
                        qm31(97718382, 739321442, 646668452, 906233770),
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x0220da6892f2906e76c2713dc027eba3b2df3dfc6c680d354061eb59372822d5,
                            0x020bcf949298f97180c360f6d55c2f65c19b9f3811c917d0368fe7203b53abcc,
                            0x0367082a2edcf72c44ec838abbd372aa27d39ecc3387791bf686a712db309846,
                            0x028514dd0ce02e8e3266b17b788f200d1ae49cc5f007fe3bd98e90529192aac3,
                            0x062dd5d3993b66c78baf3608a2ed3de1ad865d0b174e006c8047b91fde98e462,
                            0x04c76a6b839945fb3cdab23a3c01333a0fa755eaa0631d76fc2d7f77cb9dbeb8,
                            0x03af1609280ef18b58dfe676fa9ac9288ebc4f2a48f511fe714b059c487455da,
                            0x01c0a53fdf814604afe54aebd2a6d2880b072e217367b3adcc8a9bc14269015f,
                            0x06ff62ebff373bc63508ad4c9c9997e38aa91331e1159c2809d81fd20b7a07e3,
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc,
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(1943731343, 338094235, 1579129158, 1325042400),
                        qm31(1311532324, 582549508, 1702052122, 36581530),
                        qm31(1561129265, 1456838851, 1325040656, 1580898325),
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x021bdc711e4823702cb7da701c301ebc832ffc967a21932d66f7998e9efbbf46,
                            0x00e3a1b78a35229bb9a60ad0d1bab478f52a087cd15c51dfa83cd47fc6bb7334,
                            0x0046d76cf189a1c1a9aad123f2c6a447af2c9fa4a7f58e11cd1852c00011a74b,
                            0x03f8cb35e41d5291f1539b1cd73b018d6510aa85ba3bc9720e6014aa95ec4248,
                            0x002d5922250cdbfedf908cabd24a158e9bdbb3de503e7636376c8a74921b8d41,
                            0x0774e25d9d61fc18f3c2a365213b81fd36cced1626e02afc4dbe4aef52021769
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x06a3f2b104508429f6b74edcd62044afb4f618302a382f281fee118b12dc9dbd,
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(1219072197, 1782590850, 228657378, 784891462),
                        qm31(24828450, 1304266370, 129024597, 1635057579),
                        qm31(715531896, 1292811410, 725451910, 1608811481),
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x058978bedb6abe931a3de1cdff9bce0f7e7ac7b14c9c2107ea66679874a67e9a,
                            0x027cf2f25d11835dbf4d3e0b0a3ab32f5b75de4f9904d1873115ecb5f2bd0555,
                            0x03abab172aeee1c04052395036cc50a51fb2497cfd307c96f32e718c4b3639cc
                        ],
                        column_witness: array![],
                    },
                    commitment: 0x03d2565deb5099be20df825aabfe4678a0922d6b4a988d23a553c9f06b5bf96e,
                },
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                    qm31(1365318542, 1863705492, 1698090260, 381798840),
                ],
                log_size: 2,
            },
        };
        let bounds = array![6, 5, 4];
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 209715200, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(1784241578, 0, 0, 0), qm31(714402375, 0, 0, 0),],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 578813952, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(673384396, 0, 0, 0), qm31(475618425, 0, 0, 0),],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 981467136, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(315059915, 0, 0, 0), qm31(558088919, 0, 0, 0),],
                    ),
                ],
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 419430400, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(142767236, 0, 0, 0), qm31(537984732, 0, 0, 0),],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1157627904, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(1718103579, 0, 0, 0), qm31(1537119660, 0, 0, 0),],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1962934272, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(2124636505, 0, 0, 0), qm31(1506525049, 0, 0, 0),],
                    ),
                ],
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 838860800, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(1014591066, 0, 0, 0), qm31(1931899148, 0, 0, 0),],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 167772160, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(354887788, 0, 0, 0), qm31(934393698, 0, 0, 0),],
                    ),
                    CircleEvaluationImpl::new(
                        CircleDomain {
                            half_coset: Coset {
                                initial_index: 1778384896, step_size: 2147483648, log_size: 0,
                            },
                        },
                        array![qm31(509977960, 0, 0, 0), qm31(1887908506, 0, 0, 0),],
                    ),
                ],
            },
        ];
        (config, proof, bounds, decommitted_values)
    }
}

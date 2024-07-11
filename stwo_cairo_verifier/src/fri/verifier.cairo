use core::option::OptionTrait;
use core::result::ResultTrait;
use stwo_cairo_verifier::fri::query::QueriesImplTrait;
use stwo_cairo_verifier::fri::domain::LineDomainTrait;
use stwo_cairo_verifier::vcs::verifier::MerkleVerifierTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::channel::ChannelTrait;
use stwo_cairo_verifier::fields::qm31::{QM31Impl, QM31, qm31};
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier};
use stwo_cairo_verifier::vcs::hasher::PoseidonMerkleHasher;
use stwo_cairo_verifier::channel::Channel;
use super::domain::{Coset, CosetImpl, LineDomain, CircleDomain, LineDomainImpl};
use super::evaluation::{
    LineEvaluation, LineEvaluationImpl, CircleEvaluation, SparseLineEvaluation,
    SparseLineEvaluationImpl, SparseCircleEvaluation, SparseCircleEvaluationImpl,
    project_to_fft_space
};
use super::query::{Queries, QueriesImpl};
use super::polynomial::{LinePoly, LinePolyImpl};
use stwo_cairo_verifier::utils::{bit_reverse_index, pow, pow_qm31, qm31_zero_array};

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

#[derive(Drop)]
struct FriLayerVerifier {
    degree_bound: u32,
    domain: LineDomain,
    folding_alpha: QM31,
    layer_index: usize,
    proof: @FriLayerProof,
}

#[generate_trait]
impl FriLayerVerifierImpl of FriLayerVerifierTrait {
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
        // TODO: check this approach
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

        let mut queries_per_log_size: Felt252Dict<Nullable<Span<usize>>> = Default::default();
        queries_per_log_size
            .insert(
                self.domain.log_size().into(), NullableTrait::new(decommitment_positions.span())
            );

        let decommitment = self.proof.decommitment.clone();
        let result = merkle_verifier
            .verify(queries_per_log_size, actual_decommitment_array, decommitment.clone());

        let evals_at_folded_queries = sparse_evaluation.fold(*self.folding_alpha);
        match result {
            Result::Ok(()) => Result::Ok((folded_queries, evals_at_folded_queries)),
            Result::Err(_) => Result::Err(FriVerificationError::InnerLayerCommitmentInvalid)
        }
    }

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

            if (error) {
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

#[derive(Clone, Copy, Drop)]
pub struct FriConfig {
    log_blowup_factor: u32,
    log_last_layer_degree_bound: usize,
    n_queries: usize,
}

#[derive(Drop, Clone)]
pub struct FriLayerProof {
    pub evals_subset: Array<QM31>,
    pub decommitment: MerkleDecommitment::<PoseidonMerkleHasher>,
    pub decomposition_coeff: QM31,
    pub commitment: felt252,
}

#[derive(Drop)]
pub struct FriProof {
    pub inner_layers: Array<FriLayerProof>,
    pub last_layer_poly: LinePoly,
}

#[derive(Drop)]
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
impl FriVerifierImpl of FriVerifierTrait {
    fn commit(
        mut channel: Channel, config: FriConfig, proof: FriProof, column_bounds: Array<u32>
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
            channel.mix_felts(array![*proof.decomposition_coeff].span());
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

        if proof.last_layer_poly.len() > pow(2, config.log_last_layer_degree_bound) {
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

    fn decommit_on_queries(
        self: @FriVerifier, queries: @Queries, decommitted_values: Array<SparseCircleEvaluation>
    ) -> Result<(), FriVerificationError> {
        let (last_layer_queries, last_layer_query_evals) = self
            .decommit_inner_layers(queries, @decommitted_values)?;
        self.decommit_last_layer(last_layer_queries, last_layer_query_evals)
    }

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

                combined_sparse_evals =
                    project_to_fft_space(
                        @layer_queries,
                        combined_sparse_evals,
                        **current_layer.proof.decomposition_coeff,
                    );

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

    // TODO: Return opening positions
    fn column_query_positions(ref self: FriVerifier, ref channel: Channel) {
        let queries = QueriesImpl::generate(
            ref channel,
            *self.column_bounds[0] + self.config.log_blowup_factor,
            self.config.n_queries
        );
        self.queries = Option::Some(queries);
    }

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
}

#[cfg(test)]
mod tests {
    use stwo_cairo_verifier::fri::verifier::FriVerifierTrait;
    use stwo_cairo_verifier::fri::verifier::FriLayerVerifierTrait;
    use core::array::ArrayTrait;
    use stwo_cairo_verifier::channel::ChannelTrait;
    use stwo_cairo_verifier::fields::qm31::qm31;
    use stwo_cairo_verifier::vcs::verifier::MerkleDecommitment;
    use stwo_cairo_verifier::fri::domain::{Coset, CircleDomain};
    use stwo_cairo_verifier::fri::evaluation::{CircleEvaluation, SparseCircleEvaluation};
    use stwo_cairo_verifier::fri::query::Queries;
    use stwo_cairo_verifier::fri::polynomial::LinePoly;
    use super::{
        FriProof, FriLayerProof, FriConfig, FriVerifier, FriVerifierImpl, FriLayerVerifier,
        FriLayerVerifierImpl, FriVerificationError
    };

    type ProofValues = (FriConfig, FriProof, Array<u32>, Queries, Array<SparseCircleEvaluation>);

    #[test]
    fn valid_proof_with_constant_last_layer_passes_verification() {
        let (config, proof, bounds, queries, decommitted_values) = proof_with_constant_last_layer();

        let channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(channel, config, proof, bounds).unwrap();

        verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
    }

    #[test]
    fn valid_proof_passes_verification() {
        let (config, proof, bounds, queries, decommitted_values) = proof_with_linear_last_layer();

        let channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(channel, config, proof, bounds).unwrap();

        verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
    }

    #[test]
    fn valid_mixed_degree_proof_passes_verification() {
        let (config, proof, bounds, queries, decommitted_values) = proof_with_mixed_degree_1();

        let channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(channel, config, proof, bounds).unwrap();

        verifier.decommit_on_queries(@queries, decommitted_values).unwrap();
    }

    #[test]
    fn valid_mixed_degree_end_to_end_proof_passes_verification() {
        let (config, proof, bounds, decommitted_values) = proof_with_mixed_degree_2();
        let mut channel = ChannelTrait::new(0x00);
        let mut verifier = FriVerifierImpl::commit(channel, config, proof, bounds).unwrap();

        let mut channel = ChannelTrait::new(0x00);
        verifier.column_query_positions(ref channel);

        verifier.decommit(decommitted_values).unwrap();
    }

    #[test]
    fn proof_with_removed_layer_fails_verification() {
        let (config, proof, bounds, _queries, _decommitted_values) = proof_with_mixed_degree_1();

        let mut invalid_config = config;
        invalid_config.log_last_layer_degree_bound -= 1;

        let channel = ChannelTrait::new(0x00);
        let result = FriVerifierImpl::commit(channel, invalid_config, proof, bounds);

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

        let channel = ChannelTrait::new(0x00);
        let result = FriVerifierImpl::commit(channel, invalid_config, proof, bounds);

        match result {
            Result::Ok(_) => { panic!("Verifier should return InvalidNumFriLayers"); },
            Result::Err(error) => { assert!(error == FriVerificationError::InvalidNumFriLayers); }
        }
    }

    #[test]
    fn proof_with_invalid_inner_layer_evaluation_fails_verification() {
        let (config, proof, bounds, queries, decommitted_values) =
            proof_with_last_layer_of_degree_four();
        let inner_layers = @proof.inner_layers;
        // Create an invalid proof by removing an evaluation from the second layer's proof
        let invalid_proof = {
            let mut invalid_inner_layers = array![];
            invalid_inner_layers.append(proof.inner_layers[0].clone());
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
                        decomposition_coeff: *inner_layers[1].decomposition_coeff,
                        commitment: *inner_layers[1].commitment
                    }
                );
            let mut i = 2;
            while i < proof.inner_layers.len() {
                invalid_inner_layers.append(proof.inner_layers[i].clone());
                i += 1;
            };

            FriProof {
                inner_layers: invalid_inner_layers, last_layer_poly: proof.last_layer_poly.clone()
            }
        };

        let channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(channel, config, invalid_proof, bounds.clone())
            .unwrap();
        let verification_result = verifier.decommit_on_queries(@queries, decommitted_values);

        match verification_result {
            Result::Ok(_) => {
                panic!("Verifier should return InnerLayerEvaluationsInvalid");
            },
            Result::Err(error) => {
                assert!(error == FriVerificationError::InnerLayerEvaluationsInvalid);
            }
        }
    }

    #[test]
    fn proof_with_invalid_inner_layer_decommitment_fails_verification() {
        let (config, proof, bounds, queries, decommitted_values) =
            proof_with_last_layer_of_degree_four();
        
        let inner_layers = @proof.inner_layers;
        // Create an invalid proof by modifying the committed values in the second layer.
        let invalid_proof = {
            let mut invalid_inner_layers = array![];
            invalid_inner_layers.append(proof.inner_layers[0].clone());
            let mut invalid_evals_subset = array![*inner_layers[1].evals_subset[0] + qm31(1, 0, 0, 0)];
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
                        decomposition_coeff: *inner_layers[1].decomposition_coeff,
                        commitment: *inner_layers[1].commitment
                    }
                );
            let mut i = 2;
            while i < proof.inner_layers.len() {
                invalid_inner_layers.append(proof.inner_layers[i].clone());
                i += 1;
            };

            FriProof {
                inner_layers: invalid_inner_layers, last_layer_poly: proof.last_layer_poly.clone()
            }
        };
        let channel = ChannelTrait::new(0x00);
        let verifier = FriVerifierImpl::commit(channel, config, invalid_proof, bounds).unwrap();

        let verification_result = verifier.decommit_on_queries(@queries, decommitted_values);

        match verification_result {
            Result::Ok(_) => {
                panic!("Verifier should return InnerLayerCommitmentInvalid");
            },
            Result::Err(error) => {
                assert!(error == FriVerificationError::InnerLayerCommitmentInvalid);
            }
        }
    }

    // Proofs extracted from Stwo's rust implementation

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
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x03e5bad5822d062c05ff947d282dc2d56a6a420d14f2f74972bb5b01287731a7
                },
                FriLayerProof {
                    evals_subset: array![qm31(1948473851, 1004529211, 1304438646, 1985407493)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x031e0d7125162e8c9022c7b9967b792472c77ac6f35b7a13703544e16d715d83
                        ],
                        column_witness: array![]
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x0536ed50a0da2d18e090646e869b52c8b202e6167758cd9ac514cf95179e9641
                }
            ],
            last_layer_poly: LinePoly {
                coeffs: array![qm31(940020369, 1979164784, 955004309, 315468455)], log_size: 0
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
                    CircleEvaluation {
                        domain, values: array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)]
                    }
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
                    evals_subset: array![qm31(1387266930, 149259209, 1148988082, 1930518101)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x043160f2f363f93858aea3f6e621c1afc86d3fb1df6486d287347137cf3dfed1,
                            0x02b0c0da3e416172be84dacba1e7b069e330431a954afd500e314f0298494a8c,
                            0x021cf336150f7443d0860ccba345ab39f50825f1f44926a891ea2e05e258826c,
                            0x01127fb653bf375b6248814ce48aa39235b6194146d54861160139e30076c47b
                        ],
                        column_witness: array![]
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x007a651ff28db891d325f04db37e9db38c77979ca66ee6a244eaa1b2c60aaf15
                },
                FriLayerProof {
                    evals_subset: array![qm31(861269867, 123962490, 964314161, 1930884004)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x03b4b07433212611ac20d82849e4fae015c82fc96c404ee9d647256fcd12a07b,
                            0x04898493682e4be2195e9b129f431be042c08a354e91d3747813f585bcb4aaca,
                            0x017e767a71f3b55f18382b203c6146e695b27c81f96ece5f2ab44cafa743b0e7
                        ],
                        column_witness: array![]
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x06dc699d087a38aaf30c8ad95545c06ab641a9e2f690f790403f53437dc66392
                },
                FriLayerProof {
                    evals_subset: array![qm31(974589897, 1592795796, 649052897, 863407657)],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x079bc5cbec102a3c4a35a415b07ef7779ea5a41069331775c363f7cc7439be5b,
                            0x01f0d2cb65d190a549b197c2b741b74156de322744fce68cb658033a0b390311,
                        ],
                        column_witness: array![]
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x0736746d72f39154aec3ef46d5c0e3855b4801d16c2c4feff404d4f0ccefd050
                }
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(1080276375, 1725024947, 477465525, 102017026),
                    qm31(1080276375, 1725024947, 477465525, 102017026)
                ],
                log_size: 1
            }
        };
        let bounds = array![5];

        let queries = Queries { positions: array![5], log_domain_size: 6 };
        let domain = CircleDomain {
            half_coset: Coset { initial_index: 553648128, step_size: 2147483648, log_size: 0 }
        };
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain, values: array![qm31(807167738, 0, 0, 0), qm31(1359821401, 0, 0, 0)]
                    }
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
                    decomposition_coeff: qm31(0, 0, 0, 0),
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
                    decomposition_coeff: qm31(0, 0, 0, 0),
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
                    decomposition_coeff: qm31(0, 0, 0, 0),
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
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 545259520, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(1464849549, 0, 0, 0), qm31(35402781, 0, 0, 0),],
                    },
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
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(1274461871, 275737803, 1667187951, 1863765347),
                        qm31(1340580983, 256049648, 1818983416, 980463906)
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x060fde801827a1681d30225a4fe690998e31a7bcd6c3c45667a8828c2e27916f,
                            0x0207cd96c394d8bd203468ae483528de8d3b92914b031b5ea405147de9b64e3c,
                            0x0033793e1985da45f1b7fded0a5dc55fe95cd69ddce3935972f7ffc971311d50,
                            0x047d27eaf0d659e9afdf7ea6c102f82ccf1a40d06aa442ddbf294f17a393b057,
                            0x036c9af3bfb0bfe616814083f6faced81dd408a6b97c4503c919a39769b97dcb,
                            0x04ff41ff563354e1ad44fc2c36df75456706351c4e7f95595889466bc37e9594
                        ],
                        column_witness: array![]
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x011501b85ce5c3170d26ab4a39969af378459856c01b2026a107e7cf977d3a40
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(2076487708, 821015293, 1124764643, 1515478998),
                        qm31(1388613128, 1250129043, 2846551, 1151418480)
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x0377991e3583c5c64569c71d3038fdc4ff0d60760121092f21049fd2a47b0b3a,
                            0x06de824fd62cb9081040a3d2f8b46b3b22f66c031d81aa6061c535968df0eafa,
                            0x034ecdc04e481b229d7483ae21e5d73b3815fa61232b86856ce8e32f9d3bea64,
                            0x02194f358c42ceb8027744f606257cf8ea158dcffc72aa62c71870631691f3e6
                        ],
                        column_witness: array![]
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x06db2192b3d4aec51d86779b026ed4c173b2679bd2da24cf6ad8b4ba5ab79143
                }
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(751072370, 712476851, 986633684, 1014125985),
                    qm31(751072370, 712476851, 986633684, 1014125985),
                    qm31(751072370, 712476851, 986633684, 1014125985),
                    qm31(751072370, 712476851, 986633684, 1014125985)
                ],
                log_size: 2
            }
        };
        let bounds = array![6, 5, 4];
        let queries = Queries { positions: array![7, 70], log_domain_size: 7 };
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1619001344, step_size: 2147483648, log_size: 0
                            }
                        },
                        values: array![qm31(83295654, 0, 0, 0), qm31(666640840, 0, 0, 0)]
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1652555776, step_size: 2147483648, log_size: 0
                            }
                        },
                        values: array![qm31(1598588979, 0, 0, 0), qm31(1615371031, 0, 0, 0)]
                    }
                ]
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1090519040, step_size: 2147483648, log_size: 0
                            }
                        },
                        values: array![qm31(985597997, 0, 0, 0), qm31(139496415, 0, 0, 0)]
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1157627904, step_size: 2147483648, log_size: 0
                            }
                        },
                        values: array![qm31(1718103579, 0, 0, 0), qm31(1537119660, 0, 0, 0)]
                    }
                ]
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 33554432, step_size: 2147483648, log_size: 0
                            }
                        },
                        values: array![qm31(1645691043, 0, 0, 0), qm31(2009531552, 0, 0, 0)]
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 167772160, step_size: 2147483648, log_size: 0
                            }
                        },
                        values: array![qm31(354887788, 0, 0, 0), qm31(934393698, 0, 0, 0)]
                    }
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
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x07bc3121028865ac7ce98ec2cdbc6b4716ef91880374f6a8e93661fe51a759dc,
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(587836539, 506913333, 1813710983, 1401047129),
                        qm31(1340580983, 256049648, 1818983416, 980463906),
                        qm31(1604063481, 102792848, 617877666, 280621577),
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x03c9f83549383c1794af686f48b750224daa051420285e027aa31f5e6563ef91,
                            0x0207cd96c394d8bd203468ae483528de8d3b92914b031b5ea405147de9b64e3c,
                            0x0142d76928317b0616d3ce086c07ac6e040e669da0d4249f53617f5e61230fbf,
                            0x02f4363840fc3c457a00744025155415f00bf2318e716f02403e767583974ceb,
                            0x05db032206eda11f83633f76676bc490ceed6aed02c9331abde8657b2eb57d14,
                            0x04ff41ff563354e1ad44fc2c36df75456706351c4e7f95595889466bc37e9594,
                        ],
                        column_witness: array![],
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x011501b85ce5c3170d26ab4a39969af378459856c01b2026a107e7cf977d3a40,
                },
                FriLayerProof {
                    evals_subset: array![
                        qm31(230668059, 937515353, 336211937, 1486617083),
                        qm31(1388613128, 1250129043, 2846551, 1151418480),
                        qm31(1414633709, 1659698730, 45239225, 1630318681),
                    ],
                    decommitment: MerkleDecommitment {
                        hash_witness: array![
                            0x00d2e8ddf56ef114806c00e0ff510c401142ed60433ada9b93ed3c548ae37cc8,
                            0x00e4552111db4ea23c2e693011a692ee6468f3de1d3c99c47f193eadd7b2b655,
                            0x02194f358c42ceb8027744f606257cf8ea158dcffc72aa62c71870631691f3e6,
                        ],
                        column_witness: array![],
                    },
                    decomposition_coeff: qm31(0, 0, 0, 0),
                    commitment: 0x06db2192b3d4aec51d86779b026ed4c173b2679bd2da24cf6ad8b4ba5ab79143,
                },
            ],
            last_layer_poly: LinePoly {
                coeffs: array![
                    qm31(751072370, 712476851, 986633684, 1014125985),
                    qm31(751072370, 712476851, 986633684, 1014125985),
                    qm31(751072370, 712476851, 986633684, 1014125985),
                    qm31(751072370, 712476851, 986633684, 1014125985),
                ],
                log_size: 2,
            },
        };
        let bounds = array![6, 5, 4];
        let decommitted_values = array![
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 209715200, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(1784241578, 0, 0, 0), qm31(714402375, 0, 0, 0),],
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 578813952, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(673384396, 0, 0, 0), qm31(475618425, 0, 0, 0),],
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 981467136, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(315059915, 0, 0, 0), qm31(558088919, 0, 0, 0),],
                    },
                ],
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 419430400, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(142767236, 0, 0, 0), qm31(537984732, 0, 0, 0),],
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1157627904, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(1718103579, 0, 0, 0), qm31(1537119660, 0, 0, 0),],
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1962934272, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(2124636505, 0, 0, 0), qm31(1506525049, 0, 0, 0),],
                    },
                ],
            },
            SparseCircleEvaluation {
                subcircle_evals: array![
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 838860800, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(1014591066, 0, 0, 0), qm31(1931899148, 0, 0, 0),],
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 167772160, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(354887788, 0, 0, 0), qm31(934393698, 0, 0, 0),],
                    },
                    CircleEvaluation {
                        domain: CircleDomain {
                            half_coset: Coset {
                                initial_index: 1778384896, step_size: 2147483648, log_size: 0,
                            },
                        },
                        values: array![qm31(509977960, 0, 0, 0), qm31(1887908506, 0, 0, 0),],
                    },
                ],
            },
        ];
        (config, proof, bounds, decommitted_values)
    }
}

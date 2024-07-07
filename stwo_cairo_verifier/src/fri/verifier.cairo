use core::result::ResultTrait;
use stwo_cairo_verifier::fri::query::QueriesImplTrait;
use stwo_cairo_verifier::fri::domain::LineDomainTrait;
use stwo_cairo_verifier::vcs::verifier::MerkleVerifierTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::channel::ChannelTrait;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier};
use stwo_cairo_verifier::vcs::hasher::PoseidonMerkleHasher;
use stwo_cairo_verifier::channel::Channel;
use super::domain::{Coset, CosetImpl, LineDomain, CircleDomain, LineDomainImpl};
use super::evaluation::{
    LineEvaluation, LineEvaluationImpl, CircleEvaluation, SparseLineEvaluation,
    SparseLineEvaluationImpl, SparseCircleEvaluation, SparseCircleEvaluationImpl, project_to_fft_space
};
use super::query::{Queries, QueriesImpl};
use super::polynomial::{LinePoly, LinePolyImpl};
use super::utils::{bit_reverse_index, pow, pow_qm31, qm31_zero_array};

pub const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;
pub const FOLD_STEP: u32 = 1;

#[derive(Debug, Drop)]
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
        let mut j = 0;
        while i < (sparse_evaluation).subline_evals.len() {
            let subline_eval = sparse_evaluation.subline_evals[i];
            while j < (sparse_evaluation.subline_evals[i]).values.len() {
                let value = subline_eval.values[j];
                column_0.append(*value.a.a);
                column_1.append(*value.a.b);
                column_2.append(*value.b.a);
                column_3.append(*value.b.b);
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

        let merkle_verifier = MerkleVerifier::<
            PoseidonMerkleHasher
        > {
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
            while i < queries.positions.len() && (*queries.positions[i - 1] / pow(2, FOLD_STEP)) == (*queries.positions[i] / pow(2, FOLD_STEP)) {
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
                if  (j < end_subline_index) && (*queries.positions[j] == eval_position) {
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

        if proof_evals_index != (*self.proof.evals_subset).len() {
            // TODO: add the layer to the error?
            return Result::Err(FriVerificationError::InnerLayerEvaluationsInvalid);
        }

        // TODO: return the correct error InnerLayerEvaluationsInvalid
        Result::Ok(SparseLineEvaluation { subline_evals: all_subline_evals })
    }
}

#[derive(Clone, Copy, Drop)]
pub struct FriConfig {
    log_blowup_factor: u32,
    log_last_layer_degree_bound: usize,
    n_queries: usize,
}

#[derive(Drop)]
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
        let mut layer_domain = LineDomain{
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
        self: @FriVerifier, queries: Queries, decommitted_values: Array<SparseCircleEvaluation>
    ) -> Result<(), FriVerificationError> {
        let (last_layer_queries, last_layer_query_evals) = self
            .decommit_inner_layers(queries, @decommitted_values)?;
        self.decommit_last_layer(last_layer_queries, last_layer_query_evals)
    }

    fn decommit_inner_layers(
        self: @FriVerifier, queries: Queries, decommitted_values: @Array<SparseCircleEvaluation>
    ) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        let circle_poly_alpha = self.circle_poly_alpha;
        let circle_poly_alpha_sq = *circle_poly_alpha * *circle_poly_alpha;

        let mut error = false;
        let mut layer_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);
        let mut layer_query_evals = qm31_zero_array(layer_queries.len());

        let mut inner_layers_index = 0;
        let mut column_bound_index = 0;
        while inner_layers_index < self.inner_layers.len() {
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
                error = true;
            } else {
                let (a, b) = result.unwrap();
                layer_queries = a;
                layer_query_evals = b;
            }
            inner_layers_index += 1;
        };

        if (error) {
            // TODO: return error
            return Result::Err(FriVerificationError::InvalidNumFriLayers);
        } else {
            return Result::Ok((layer_queries, layer_query_evals));
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
}

#[test]
fn valid_proof_with_constant_last_layer_passes_verification() {
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
                    hash_witness: array![0x031e0d7125162e8c9022c7b9967b792472c77ac6f35b7a13703544e16d715d83],
                    column_witness: array![] 
                }, 
                decomposition_coeff: qm31(0, 0, 0, 0),
                commitment: 0x0536ed50a0da2d18e090646e869b52c8b202e6167758cd9ac514cf95179e9641
            }
        ],
        last_layer_poly: LinePoly {
            coeffs: array![qm31(940020369, 1979164784, 955004309, 315468455)],
            log_size: 0
        }
    };

    let decommitment_value = array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)];
    let domain = CircleDomain {
        half_coset: Coset {
            initial_index: 603979776,
            step_size: 2147483648,
            log_size: 0
        }
    };

    let queries = Queries { positions: array![5], log_domain_size: 4 };

    let channel = ChannelTrait::new(0x00);
    let config = FriConfig { log_blowup_factor: 1, log_last_layer_degree_bound: 0, n_queries: 1 };
    let bound = array![3];

    let verifier = FriVerifierImpl::commit(channel, config, proof, bound).unwrap();
    let decommitted_values = array![
        SparseCircleEvaluation {
            subcircle_evals: array![CircleEvaluation { domain, values: decommitment_value }]
        }
    ];
    verifier.decommit_on_queries(queries, decommitted_values).unwrap();
}

#[test]
fn valid_proof_passes_verification() {

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

    let decommitment_value = array![qm31(807167738, 0, 0, 0), qm31(1359821401, 0, 0, 0)];
    let domain = CircleDomain {
        half_coset: Coset {
            initial_index: 553648128,
            step_size: 2147483648,
            log_size: 0
        }
    };

    let queries = Queries { positions: array![5], log_domain_size: 6 };

    let channel = ChannelTrait::new(0x00);
    let config = FriConfig { log_blowup_factor: 1, log_last_layer_degree_bound: 1, n_queries: 1 };
    let bound = array![5];

    let verifier = FriVerifierImpl::commit(channel, config, proof, bound).unwrap();
    let decommitted_values = array![
        SparseCircleEvaluation {
            subcircle_evals: array![CircleEvaluation { domain, values: decommitment_value }]
        }
    ];
    verifier.decommit_on_queries(queries, decommitted_values).unwrap();
}
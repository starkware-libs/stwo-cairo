use core::clone::Clone;
use core::result::ResultTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::channel::ChannelTrait;
use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::vcs::verifier::MerkleDecommitment;
use stwo_cairo_verifier::channel::Channel;

const CIRCLE_TO_LINE_FOLD_STEP: u32 = 1;
const FOLD_STEP: u32 = 1;

#[derive(Drop, Clone)]
pub struct SparseCircleEvaluation {
    subcircle_evals: Array<CircleEvaluation>
}

#[derive(Drop, Clone)]
pub struct CircleEvaluation {
    domain: CircleDomain,
    values: Array<QM31>
}

#[generate_trait]
impl SparseCircleEvaluationImpl of SparseCircleEvaluationImplTrait {
    fn accumulate(self: @SparseCircleEvaluation, rhs: @SparseCircleEvaluation, alpha: QM31) -> SparseCircleEvaluation {
        // TODO: implement
        SparseCircleEvaluation {
            subcircle_evals: array![]
        }
    }

    fn fold(self: @SparseCircleEvaluation, alpha: QM31) -> Array<QM31> {
        // TODO: implement and remove clone in Queries
        array![qm31(0,0,0,0), qm31(0,0,0,0)]
    }
}

fn bit_reverse_index(index: usize, bits: u32) -> usize {
    // TODO: implement
    index
}

fn pow(base: u32, exponent: u32) -> u32 {
    // TODO: implement or include from alexandria
    1
}

fn pow_qm31(base: QM31, exponent: u32) -> QM31 {
    // TODO: implement
    qm31(1, 0, 0, 0)
}

fn qm31_zero_array(n: u32) -> Array<QM31> {
    let mut result = array![];
    let mut i = 0;
    while i < n {
        result.append(qm31(0, 0, 0, 0));
        i += 1;
    };
    result
}

fn project_to_fft_space(
    queries: @Queries,
    evals: SparseCircleEvaluation,
    lambda: QM31
) -> SparseCircleEvaluation {
    // TODO: implement
    evals
}

#[derive(Drop)]
pub enum FriVerificationError {
    InvalidNumFriLayers,
    LastLayerDegreeInvalid,
    LastLayerEvaluationsInvalid
}

#[derive(Copy, Drop)]
pub struct LineDomain {
}

#[derive(Copy, Drop)]
pub struct CircleDomain {
}

#[generate_trait]
impl LineDomainImpl of LineDomainTrait {
    fn double(self: LineDomain) -> LineDomain {
        // TODO: implement
        self
    }

    fn at(self: @LineDomain, index: usize) -> M31 {
        // TODO: implement
        m31(1)
    }

    fn log_size(self: @LineDomain) -> usize {
        // TODO: implement
        1
    }
}

#[derive(Drop)]
pub struct LinePoly {
    coeffs: Array<QM31>,
    log_size: u32,
}

#[generate_trait]
impl LinePolyImpl of LinePolyTrait {
    fn len(self: @LinePoly) -> usize {
        // TODO: implement
        1
    }

    fn eval_at_point(self: @LinePoly, x: QM31) -> QM31 {
        // TODO: implement
        x
    }
}


#[derive(Drop, Clone)]
pub struct Queries {
    pub positions: Array<usize>,
    pub log_domain_size: u32,
}

#[generate_trait]
impl QueriesImpl of QueriesImplTrait {
    fn len(self: @Queries) -> usize {
        // TODO: implement
        2
    }

    fn fold(self: @Queries, n_folds: u32) -> Queries {
        // TODO: implement and remove clone in Queries
        self.clone()
    }
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
    fn verify_and_fold(self: @FriLayerVerifier, queries: @Queries, evals_at_queries: @Array<QM31>) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        Result::Ok((queries.clone(), array![qm31(0,0,0,0)]))
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
    pub decommitment: MerkleDecommitment,
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
    fn commit(mut channel: Channel, config: FriConfig, proof: FriProof, column_bounds: Array<u32>) -> Result<FriVerifier, FriVerificationError> {
        let max_column_bound = column_bounds[0];
        let expected_query_log_domain_size = *max_column_bound + config.log_blowup_factor;
        let circle_poly_alpha = channel.draw_felt();

        let mut inner_layers = array![];
        let mut layer_bound = *max_column_bound - CIRCLE_TO_LINE_FOLD_STEP;
        let mut layer_domain = LineDomain{};

        let mut layer_index = 0;
        let mut invalid_fri_layers_number = false;
        while layer_index < proof.inner_layers.len() {
            let proof = proof.inner_layers[layer_index];
            channel.mix_digest(*proof.commitment);
            channel.mix_felts(array![*proof.decomposition_coeff].span());
            let folding_alpha = channel.draw_felt();
            inner_layers.append(FriLayerVerifier {
                degree_bound: layer_bound,
                domain: layer_domain,
                folding_alpha,
                layer_index,
                proof,
            });

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

        let last_layer_domain = LineDomain{};
    
        if proof.last_layer_poly.len() > pow(2, config.log_last_layer_degree_bound) {
            return Result::Err(FriVerificationError::LastLayerDegreeInvalid);
        }

        channel.mix_felts(proof.last_layer_poly.coeffs.span());

        Result::Ok(FriVerifier {
            config,
            circle_poly_alpha,
            column_bounds,
            expected_query_log_domain_size,
            inner_layers,
            last_layer_domain,
            last_layer_poly: proof.last_layer_poly,
            queries: Option::None
        })
    }

    fn decommit_on_queries(
        self: @FriVerifier,
        queries: Queries,
        decommitted_values: Array<SparseCircleEvaluation>
    ) -> Result<(), FriVerificationError> {
        let (last_layer_queries, last_layer_query_evals) = self.decommit_inner_layers(queries, @decommitted_values)?;
        self.decommit_last_layer(last_layer_queries, last_layer_query_evals)
    }

    fn decommit_inner_layers(
        self: @FriVerifier,
        queries: Queries,
        decommitted_values: @Array<SparseCircleEvaluation>
    ) -> Result<(Queries, Array<QM31>), FriVerificationError> {
        // TODO: adapt for multi-fri
        let circle_poly_alpha = self.circle_poly_alpha;
        let circle_poly_alpha_sq = *circle_poly_alpha * *circle_poly_alpha;

        let mut error = false;
        let mut layer_queries = queries.fold(CIRCLE_TO_LINE_FOLD_STEP);
        let mut layer_query_evals = qm31_zero_array(layer_queries.len());

        let mut inner_layers_index = 0;
        let mut column_bound_index = 0;
        while inner_layers_index < self.inner_layers.len() {
            let current_layer = self.inner_layers[inner_layers_index];
            if column_bound_index < self.column_bounds.len() && *self.column_bounds[column_bound_index] - CIRCLE_TO_LINE_FOLD_STEP  == *current_layer.degree_bound {
                let mut n_columns_in_layer = 1;
                // TODO: remove clone?
                let mut combined_sparse_evals = decommitted_values[column_bound_index].clone();

                column_bound_index += 1;

                while column_bound_index < self.column_bounds.len() && *self.column_bounds[column_bound_index] - CIRCLE_TO_LINE_FOLD_STEP  == *current_layer.degree_bound {
                    combined_sparse_evals = combined_sparse_evals.accumulate(decommitted_values[column_bound_index], circle_poly_alpha_sq);
                    column_bound_index += 1;
                    n_columns_in_layer += 1;
                };
                
                combined_sparse_evals = project_to_fft_space(
                    @layer_queries,
                    combined_sparse_evals,
                    **current_layer.proof.decomposition_coeff,
                );

                let folded_evals = combined_sparse_evals.fold(*circle_poly_alpha);
                let prev_layer_combination_factor = pow_qm31(circle_poly_alpha_sq, n_columns_in_layer);

                let mut k = 0;
                let mut new_layer_query_evals: Array<QM31> = array![];
                assert!(folded_evals.len() == layer_queries.len());
                while k <  folded_evals.len() {
                    new_layer_query_evals.append(*layer_query_evals[k] * prev_layer_combination_factor + *folded_evals[k]);
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

        if(error) {
            // TODO: return error
            return Result::Err(FriVerificationError::InvalidNumFriLayers);
        } else {
            return Result::Ok((layer_queries, layer_query_evals));
        }
    }

    fn decommit_last_layer(
        self: @FriVerifier,
        queries: Queries,
        query_evals: Array<QM31>,
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
fn test_fri_verifier() {
    let proof = FriProof {
        inner_layers: array![FriLayerProof {
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
        }],
        last_layer_poly: LinePoly{
            coeffs: array![
                qm31(1700411592, 1718151617, 1386964511, 2008082344), 
                qm31(1700411592, 1718151617, 1386964511, 2008082344)
            ],
            log_size: 1
        }
    };

    let log_degree = 3;
    let log_domain_size = 4;
    let decommitment_value = array![qm31(1990458477, 0, 0, 0), qm31(1966717173, 0, 0, 0)];
    let domain = CircleDomain{};
    let queries = Queries {positions: array![5], log_domain_size};

    let channel = ChannelTrait::new(0x00);
    let config = FriConfig { log_blowup_factor: 1, log_last_layer_degree_bound: 1, n_queries: 1 };
    let bound = array![log_degree];

    let verifier = FriVerifierImpl::commit(channel, config, proof, bound).unwrap();
    let decommitted_values = array![SparseCircleEvaluation{subcircle_evals: array![CircleEvaluation{domain, values: decommitment_value}]}];
    verifier.decommit_on_queries(queries, decommitted_values).unwrap();

    // assert!(verifier.verify(channel, proof));
}

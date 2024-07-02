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

#[derive(Drop)]
pub struct SparseLineEvaluation {
    subline_evals: Array<LineEvaluation>,
}

#[derive(Drop)]
pub struct LineEvaluation {
    values: Array<QM31>,
    domain: LineDomain
}

#[generate_trait]
impl LineEvaluationImpl of LineEvaluationTrait {
    fn new(domain: LineDomain, values: Array<QM31>) -> LineEvaluation {
        // TODO: implement asserts
        LineEvaluation {
            values: values,
            domain: domain
        }
    }
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

fn bit_reverse_index(mut index: usize, mut bits: u32) -> usize {
    assert!(bits < 32);
    let mut result = 0;
    let mut pow_of_two = 1;
    while bits > 0 {
        result *= 2;
        result = result | ((index / pow_of_two) & 1);
        pow_of_two *= 2;
        bits -= 1;
    };
    result
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

fn dummy_line_domain() -> LineDomain {
    LineDomain {
        coset: Coset {
            initial_index: CirclePointIndex { index: 0},
            step_size: CirclePointIndex { index: 0},
            log_size: 1
        }
    }
}

#[derive(Copy, Debug, PartialEq, Eq, Drop)]
pub struct CirclePointIndex {
    index: usize
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Drop)]
pub struct Coset {
    pub initial_index: CirclePointIndex,
    //pub initial: CirclePoint<M31>,
    pub step_size: CirclePointIndex,
    //pub step: CirclePoint<M31>,
    pub log_size: u32,
}

#[generate_trait]
impl CosetImpl of CosetTrait {
    fn index_at(self: @Coset, index: usize) -> CirclePointIndex {
        // TODO: implement
        CirclePointIndex { index: 0 }
    }

    fn new(_initial_index: CirclePointIndex, log_size: u32) -> Coset {
        // TODO: implement
        Coset {
            initial_index: CirclePointIndex {index: 0},
            step_size: CirclePointIndex {index: 0},
            log_size: 1
        }
    }
}

#[derive(Drop)]
pub enum FriVerificationError {
    InvalidNumFriLayers,
    LastLayerDegreeInvalid,
    LastLayerEvaluationsInvalid,
    InnerLayerEvaluationsInvalid
}

#[derive(Copy, Drop)]
pub struct LineDomain {
    coset: Coset,
}

#[derive(Copy, Drop)]
pub struct CircleDomain {
}

#[generate_trait]
impl LineDomainImpl of LineDomainTrait {
    fn new(coset: Coset) -> LineDomain {
        // TODO: Implement, it does some assertions.
        LineDomain {
            coset: coset
        }
    }
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
        self.positions.len()
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
        let decommitment = self.proof.decommitment.clone();
        let commitment = self.proof.commitment;

        let sparse_evaluation = @self.extract_evaluation(queries, evals_at_queries)?;

        let mut actual_decommitment_evals: Array<QM31> = array![];
        let mut i = 0;
        let mut j = 0;
        while i < (sparse_evaluation).subline_evals.len() {
            let subline_eval = sparse_evaluation.subline_evals[i];
            while j < (sparse_evaluation.subline_evals[i]).values.len() {
                actual_decommitment_evals.append(*subline_eval.values[j]);
                j += 1;
            };
            i += 1;
        };

        let folded_queries = queries.fold(FOLD_STEP);

        let mut decommitment_positions = array![];
        let mut i = 0;
        while i < folded_queries.positions.len() {
           let start = *folded_queries.positions[i] * pow(2, FOLD_STEP);
           let end = start + pow(2, FOLD_STEP);
           let mut j = start;
           while j < end {
               decommitment_positions.append(j);
               j += 1;
           };
           i += 1;
        };

        // let merkle_verifier = MerkleVerifier::new(
        //    commitment,
        //    vec![self.domain.log_size(); SECURE_EXTENSION_DEGREE],
        // );

        Result::Ok((queries.clone(), array![qm31(0,0,0,0)]))
    }

    fn extract_evaluation(self: @FriLayerVerifier,
                          queries: @Queries,
                          evals_at_queries: @Array<QM31>) -> Result<SparseLineEvaluation, FriVerificationError> {
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
            while i + 1 < queries.positions.len() && (*queries.positions[i] / pow(2, FOLD_STEP)) == (*queries.positions[i + 1] / pow(2, FOLD_STEP)) {
                i = i + 1;
            };
            let end_subline_index = i;

            // These are the values whose evaluations are required.
            let subline_start = (*queries.positions[start_subline_index] / pow(2, FOLD_STEP)) * pow(2, FOLD_STEP);
            let subline_end = subline_start + pow(2, FOLD_STEP);

            let mut subline_evals: Array<QM31> = array![];

            let mut j = start_subline_index;
            let mut eval_position = subline_start;

            while eval_position < subline_end {
                if *queries.positions[j] == eval_position {
                    subline_evals.append(evals_at_queries[evals_at_queries_index].clone());
                
                    evals_at_queries_index += 1;
                    j += 1;
                } else {
                    if(proof_evals_index < (*self.proof.evals_subset).len()) {
                        subline_evals.append((*self.proof.evals_subset)[proof_evals_index].clone());
                        proof_evals_index += 1;
                    } else {
                        error = true;
                        break;
                    }
                }
                eval_position += 1;
            };

            if(error) {
                break;
            }

            let subline_initial_index = bit_reverse_index(subline_start, self.domain.log_size());
            let subline_initial = self.domain.coset.index_at(subline_initial_index);
            let subline_domain = LineDomainImpl::new(CosetImpl::new(subline_initial, FOLD_STEP));

            all_subline_evals.append(
                LineEvaluationImpl::new(
                    subline_domain,
                    subline_evals
                )
            );
        };

        if proof_evals_index != (*self.proof.evals_subset).len() {
            // TODO: add the layer to the error?
            return Result::Err(FriVerificationError::InnerLayerEvaluationsInvalid);
        }

        // TODO: return the correct error InnerLayerEvaluationsInvalid
        Result::Ok(SparseLineEvaluation {subline_evals: all_subline_evals })
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
        let mut layer_domain = dummy_line_domain(); // TODO: replace

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

        let last_layer_domain = dummy_line_domain(); // TODO: replace
    
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
                assert!(folded_evals.len() == layer_query_evals.len());
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

#[test]
fn test_bit_reverse() {
    // 1 bit
    assert_eq!(0, bit_reverse_index(0, 1));
    assert_eq!(1, bit_reverse_index(1, 1));

    // 2 bits
    assert_eq!(0, bit_reverse_index(0, 2));
    assert_eq!(2, bit_reverse_index(1, 2));
    assert_eq!(1, bit_reverse_index(2, 2));
    assert_eq!(3, bit_reverse_index(3, 2));

    // 3 bits
    assert_eq!(0, bit_reverse_index(0, 3));
    assert_eq!(4, bit_reverse_index(1, 3));
    assert_eq!(2, bit_reverse_index(2, 3));
    assert_eq!(6, bit_reverse_index(3, 3));

    // 16 bits
    assert_eq!(24415, bit_reverse_index(64250, 16));

    // 31 bits
    assert_eq!(16448250, bit_reverse_index(800042880, 31));
}


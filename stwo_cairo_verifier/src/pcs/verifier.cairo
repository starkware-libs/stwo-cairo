use core::dict::Felt252Dict;
use core::iter::{Iterator, IntoIterator};
use stwo_cairo_verifier::channel::{Channel, ChannelTrait};
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::fields::qm31::QM31;
use stwo_cairo_verifier::fri::{FriProof, FriVerifierImpl};
use stwo_cairo_verifier::pcs::quotients::{fri_answers, PointSample};
use stwo_cairo_verifier::queries::SparseSubCircleDomainImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::vcs::hasher::PoseidonMerkleHasher;
use stwo_cairo_verifier::vcs::verifier::{MerkleVerifier, MerkleVerifierTrait, MerkleDecommitment};
use stwo_cairo_verifier::verifier::{VerificationError, FriVerificationErrorIntoVerificationError};
use stwo_cairo_verifier::{TreeArray, ColumnArray};
use super::PcsConfig;


/// The verifier side of a FRI polynomial commitment scheme. See [super].
// TODO(andrew): Make generic on MerkleChannel.
#[derive(Drop)]
pub struct CommitmentSchemeVerifier {
    pub trees: Array<MerkleVerifier<PoseidonMerkleHasher>>,
    pub config: PcsConfig,
}

#[generate_trait]
pub impl CommitmentSchemeVerifierImpl of CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier {
        CommitmentSchemeVerifier { trees: array![], config }
    }

    /// Returns the log sizes of each column in each commitment tree.
    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<@Array<u32>> {
        let mut res = array![];
        for tree in self.trees.span() {
            res.append(tree.column_log_sizes);
        };
        res
    }

    /// Reads a commitment from the prover.
    // TODO(andrew): Make commitment MerkleHasher hash type.
    // TODO(andrew): Make channel MerkleChannel generic channel.
    fn commit(
        ref self: CommitmentSchemeVerifier,
        commitment: felt252,
        log_sizes: @Array<u32>,
        ref channel: Channel,
    ) {
        channel.mix_digest(commitment);
        let mut extended_log_sizes = array![];
        for log_size in log_sizes
            .span() {
                extended_log_sizes.append(*log_size + self.config.fri_config.log_blowup_factor);
            };
        self
            .trees
            .append(MerkleVerifier { root: commitment, column_log_sizes: extended_log_sizes, });
    }

    // TODO(andrew): Introduce ColumnArray and TreeArray to make types less confusing.
    fn verify_values(
        self: @CommitmentSchemeVerifier,
        sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) -> Result<(), VerificationError> {
        let CommitmentSchemeProof { sampled_values,
        decommitments,
        queried_values,
        proof_of_work,
        fri_proof } =
            proof;

        let mut flattened_sampled_values = array![];

        for sampled_values in sampled_values
            .span() {
                for column_sampled_values in sampled_values
                    .span() {
                        for sampled_value in column_sampled_values
                            .span() {
                                flattened_sampled_values.append(*sampled_value);
                            };
                    };
            };

        channel.mix_felts(flattened_sampled_values.span());

        let random_coeff = channel.draw_felt();
        let column_log_sizes = self.column_log_sizes();
        let log_blowup_factor = *self.config.fri_config.log_blowup_factor;
        let column_log_bounds = get_column_log_bounds(@column_log_sizes, log_blowup_factor);

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier =
            match FriVerifierImpl::commit(
                ref channel, *self.config.fri_config, fri_proof, column_log_bounds
            ) {
            Result::Ok(fri_verifier) => fri_verifier,
            Result::Err(err) => { return Result::Err(VerificationError::Fri(err)); },
        };

        // Verify proof of work.
        channel.mix_nonce(proof_of_work);

        if channel.trailing_zeros() < *self.config.pow_bits {
            return Result::Err(VerificationError::ProofOfWork);
        }

        // Get FRI query domains.
        let (mut fri_query_domain_per_log_size, fri_query_domain_log_sizes) = fri_verifier
            .column_query_positions(ref channel);

        let n_trees = self.trees.len();

        // Verify merkle decommitments.
        let mut decommitments = decommitments.into_iter();

        let mut tree_i = 0;
        loop {
            if tree_i == n_trees {
                break Result::Ok(());
            }

            let tree = self.trees[tree_i];
            let decommitment = decommitments.next().unwrap();
            let queried_values = queried_values[tree_i];

            let mut queries_per_log_size = Default::default();

            for log_size in fri_query_domain_log_sizes {
                let log_size_felt252 = (*log_size).into();
                let domain = fri_query_domain_per_log_size.get(log_size_felt252).deref();
                // TODO: Flatten all domains ahead of time outside the loops.
                queries_per_log_size
                    .insert(log_size_felt252, NullableTrait::new(domain.flatten().span()));
            };

            if let Result::Err(err) = tree
                .verify(queries_per_log_size, queried_values, decommitment) {
                break Result::Err(VerificationError::Merkle(err));
            }

            tree_i += 1;
        }?;

        // Check iterators have been fully consumed.
        assert!(decommitments.next().is_none());

        // Answer FRI queries.
        let samples = get_flattened_samples(sampled_points, sampled_values);

        let mut flattened_column_log_sizes = array![];

        for column_log_sizes in column_log_sizes {
            for log_size in column_log_sizes
                .span() {
                    flattened_column_log_sizes.append(*log_size);
                };
        };

        // TODO(andrew): Flattening not nessesary. Check how costly.
        let flattened_query_values = get_flattened_query_values(queried_values);

        let fri_answers = fri_answers(
            @flattened_column_log_sizes,
            @samples,
            random_coeff,
            fri_query_domain_per_log_size,
            @flattened_query_values,
        )?;

        if let Result::Err(err) = fri_verifier.decommit(fri_answers) {
            return Result::Err(VerificationError::Fri(err));
        }

        Result::Ok(())
    }
}

/// Returns all column log bounds deduped and sorted in ascending order.
#[inline]
fn get_column_log_bounds(
    column_log_sizes: @TreeArray<@ColumnArray<u32>>, log_blowup_factor: u32
) -> Array<u32> {
    // The max column degree bound.
    const MAX_LOG_BOUND: u32 = 29;

    let mut bounds_set: Felt252Dict<bool> = Default::default();

    for tree_column_log_sizes in column_log_sizes
        .span() {
            for column_log_size in (*tree_column_log_sizes)
                .span() {
                    let column_log_bound = *column_log_size - log_blowup_factor;
                    assert!(column_log_bound <= MAX_LOG_BOUND);
                    bounds_set.insert(column_log_bound.into(), true);
                };
        };

    let mut bounds = array![];

    let mut i = MAX_LOG_BOUND;
    while i != 0 {
        if bounds_set.get(i.into()) {
            bounds.append(i);
        }
        i -= 1;
    };

    bounds
}

#[inline]
fn get_flattened_samples(
    sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
    sampled_values: TreeArray<ColumnArray<Array<QM31>>>
) -> ColumnArray<Array<PointSample>> {
    let mut res = array![];
    let n_trees = sampled_points.len();
    assert!(sampled_points.len() == sampled_values.len());

    let mut tree_i = 0;
    while tree_i < n_trees {
        let tree_points = sampled_points[tree_i];
        let tree_values = sampled_values[tree_i];
        assert!(tree_points.len() == tree_values.len());
        let n_columns = tree_points.len();

        let mut column_i = 0;
        while column_i < n_columns {
            let column_points = tree_points[column_i];
            let column_values = tree_values[column_i];
            let n_samples = column_points.len();
            let mut column_samples = array![];

            let mut sample_i = 0;
            while sample_i < n_samples {
                let point = *column_points[sample_i];
                let value = *column_values[sample_i];
                column_samples.append(PointSample { point, value });
                sample_i += 1;
            };

            res.append(column_samples);
            column_i += 1;
        };

        tree_i += 1;
    };
    res
}

#[inline]
fn get_flattened_query_values(
    query_values: TreeArray<ColumnArray<Array<M31>>>
) -> ColumnArray<Array<M31>> {
    let mut res = array![];
    for query_values in query_values {
        for column_query_values in query_values {
            res.append(column_query_values);
        };
    };
    res
}

#[derive(Drop)]
pub struct CommitmentSchemeProof {
    pub sampled_values: TreeArray<ColumnArray<Array<QM31>>>,
    pub decommitments: TreeArray<MerkleDecommitment<PoseidonMerkleHasher>>,
    pub queried_values: TreeArray<ColumnArray<Array<M31>>>,
    pub proof_of_work: u64,
    pub fri_proof: FriProof,
}


use core::iter::{IntoIterator, Iterator};
use stwo_cairo_verifier::channel::{Channel, ChannelTrait};
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::fields::qm31::QM31;
use stwo_cairo_verifier::fri::{FriProof, FriVerifierImpl};
use stwo_cairo_verifier::pcs::quotients::{PointSample, fri_answers};
use stwo_cairo_verifier::queries::SparseSubCircleDomainImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::vcs::hasher::PoseidonMerkleHasher;
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use stwo_cairo_verifier::verifier::{FriVerificationErrorIntoVerificationError, VerificationError};
use stwo_cairo_verifier::{ColumnArray, TreeArray};
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
        for log_size in log_sizes.span() {
            extended_log_sizes.append(*log_size + self.config.fri_config.log_blowup_factor);
        };
        self
            .trees
            .append(MerkleVerifier { root: commitment, column_log_sizes: extended_log_sizes });
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

        for sampled_values in sampled_values.span() {
            for column_sampled_values in sampled_values.span() {
                for sampled_value in column_sampled_values.span() {
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
                ref channel, *self.config.fri_config, fri_proof, column_log_bounds,
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
            for log_size in column_log_sizes.span() {
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
    column_log_sizes: @TreeArray<@ColumnArray<u32>>, log_blowup_factor: u32,
) -> Array<u32> {
    let mut column_log_bound_0 = false;
    let mut column_log_bound_1 = false;
    let mut column_log_bound_2 = false;
    let mut column_log_bound_3 = false;
    let mut column_log_bound_4 = false;
    let mut column_log_bound_5 = false;
    let mut column_log_bound_6 = false;
    let mut column_log_bound_7 = false;
    let mut column_log_bound_8 = false;
    let mut column_log_bound_9 = false;
    let mut column_log_bound_10 = false;
    let mut column_log_bound_11 = false;
    let mut column_log_bound_12 = false;
    let mut column_log_bound_13 = false;
    let mut column_log_bound_14 = false;
    let mut column_log_bound_15 = false;
    let mut column_log_bound_16 = false;
    let mut column_log_bound_17 = false;
    let mut column_log_bound_18 = false;
    let mut column_log_bound_19 = false;
    let mut column_log_bound_20 = false;
    let mut column_log_bound_21 = false;
    let mut column_log_bound_22 = false;
    let mut column_log_bound_23 = false;
    let mut column_log_bound_24 = false;
    let mut column_log_bound_25 = false;
    let mut column_log_bound_26 = false;
    let mut column_log_bound_27 = false;
    let mut column_log_bound_28 = false;
    let mut column_log_bound_29 = false;
    let mut column_log_bound_30 = false;
    let mut column_log_bound_31 = false;

    let mut tree_i = 0;
    while tree_i != column_log_sizes.len() {
        let tree_column_log_sizes = *column_log_sizes[tree_i];
        let n_columns = tree_column_log_sizes.len();

        let mut column_i = 0;
        // TODO(andrew): This might be an expensive approach for traces >10,000 columns since
        // function passes >30 variables as arguments.
        while column_i != n_columns {
            let column_log_size = *tree_column_log_sizes[column_i];
            let column_log_bound = column_log_size - log_blowup_factor;
            match column_log_bound {
                0 => column_log_bound_0 = true,
                1 => column_log_bound_1 = true,
                2 => column_log_bound_2 = true,
                3 => column_log_bound_3 = true,
                4 => column_log_bound_4 = true,
                5 => column_log_bound_5 = true,
                6 => column_log_bound_6 = true,
                7 => column_log_bound_7 = true,
                8 => column_log_bound_8 = true,
                9 => column_log_bound_9 = true,
                10 => column_log_bound_10 = true,
                11 => column_log_bound_11 = true,
                12 => column_log_bound_12 = true,
                13 => column_log_bound_13 = true,
                14 => column_log_bound_14 = true,
                15 => column_log_bound_15 = true,
                16 => column_log_bound_16 = true,
                17 => column_log_bound_17 = true,
                18 => column_log_bound_18 = true,
                19 => column_log_bound_19 = true,
                20 => column_log_bound_20 = true,
                21 => column_log_bound_21 = true,
                22 => column_log_bound_22 = true,
                23 => column_log_bound_23 = true,
                24 => column_log_bound_24 = true,
                25 => column_log_bound_25 = true,
                26 => column_log_bound_26 = true,
                27 => column_log_bound_27 = true,
                28 => column_log_bound_28 = true,
                29 => column_log_bound_29 = true,
                _ => panic!("Invalid bound"),
            }
            column_i += 1;
        };

        tree_i += 1;
    };

    let mut bounds = array![];

    if column_log_bound_31 {
        bounds.append(31);
    }
    if column_log_bound_30 {
        bounds.append(30);
    }
    if column_log_bound_29 {
        bounds.append(29);
    }
    if column_log_bound_28 {
        bounds.append(28);
    }
    if column_log_bound_27 {
        bounds.append(27);
    }
    if column_log_bound_26 {
        bounds.append(26);
    }
    if column_log_bound_25 {
        bounds.append(25);
    }
    if column_log_bound_24 {
        bounds.append(24);
    }
    if column_log_bound_23 {
        bounds.append(23);
    }
    if column_log_bound_22 {
        bounds.append(22);
    }
    if column_log_bound_21 {
        bounds.append(21);
    }
    if column_log_bound_20 {
        bounds.append(20);
    }
    if column_log_bound_19 {
        bounds.append(19);
    }
    if column_log_bound_18 {
        bounds.append(18);
    }
    if column_log_bound_17 {
        bounds.append(17);
    }
    if column_log_bound_16 {
        bounds.append(16);
    }
    if column_log_bound_15 {
        bounds.append(15);
    }
    if column_log_bound_14 {
        bounds.append(14);
    }
    if column_log_bound_13 {
        bounds.append(13);
    }
    if column_log_bound_12 {
        bounds.append(12);
    }
    if column_log_bound_11 {
        bounds.append(11);
    }
    if column_log_bound_10 {
        bounds.append(10);
    }
    if column_log_bound_9 {
        bounds.append(9);
    }
    if column_log_bound_8 {
        bounds.append(8);
    }
    if column_log_bound_7 {
        bounds.append(7);
    }
    if column_log_bound_6 {
        bounds.append(6);
    }
    if column_log_bound_5 {
        bounds.append(5);
    }
    if column_log_bound_4 {
        bounds.append(4);
    }
    if column_log_bound_3 {
        bounds.append(3);
    }
    if column_log_bound_2 {
        bounds.append(2);
    }
    if column_log_bound_1 {
        bounds.append(1);
    }
    if column_log_bound_0 {
        bounds.append(0);
    }

    bounds
}

#[inline]
fn get_flattened_samples(
    sampled_points: TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>,
    sampled_values: TreeArray<ColumnArray<Array<QM31>>>,
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
    query_values: TreeArray<ColumnArray<Array<M31>>>,
) -> ColumnArray<Array<M31>> {
    let mut res = array![];
    for query_values in query_values {
        for column_query_values in query_values {
            res.append(column_query_values);
        };
    };
    res
}

#[derive(Drop, Serde)]
pub struct CommitmentSchemeProof {
    pub sampled_values: TreeArray<ColumnArray<Array<QM31>>>,
    pub decommitments: TreeArray<MerkleDecommitment<PoseidonMerkleHasher>>,
    pub queried_values: TreeArray<ColumnArray<Array<M31>>>,
    pub proof_of_work: u64,
    pub fri_proof: FriProof,
}


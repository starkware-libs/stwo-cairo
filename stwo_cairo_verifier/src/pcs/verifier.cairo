use stwo_cairo_verifier::channel::ChannelTrait;
use core::result::ResultTrait;
use core::array::ToSpanTrait;
use core::array::ArrayTrait;
use stwo_cairo_verifier::vcs::hasher::MerkleHasher;
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use stwo_cairo_verifier::vcs::hasher::{PoseidonMerkleHasher};
use stwo_cairo_verifier::fri::{FriConfig, FriProof, FriVerifierImpl};
use stwo_cairo_verifier::channel::{ChannelTime, Channel, ChannelImpl};
use stwo_cairo_verifier::fields::qm31::QM31;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::queries::{
    SparseSubCircleDomain, SparseSubCircleDomainTrait, get_sparse_sub_circle_domain_dict
};
use stwo_cairo_verifier::pcs::quotients::{PointSample, fri_answers};
use stwo_cairo_verifier::poly::circle::SparseCircleEvaluation;
use core::nullable::{NullableTrait, match_nullable, FromNullableResult};
use core::dict::Felt252DictEntryTrait;
use stwo_cairo_verifier::sort::MaximumToMinimumSortedIterator;
use stwo_cairo_verifier::fri::FriVerificationError;
use stwo_cairo_verifier::utils::{flat_3d_array, substract_map_2d_array};


/// The verifier side of a FRI polynomial commitment scheme. See [super].
#[derive(Drop)]
pub struct CommitmentSchemeVerifier {
    pub trees: Array<MerkleVerifier<PoseidonMerkleHasher>>,
    pub config: PcsConfig,
}

pub trait CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier;

    /// An Array<Array<u32>> of the log sizes of each column in each commitment tree.
    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<Array<u32>>;

    /// Reads a commitment from the prover.
    fn commit(
        ref self: CommitmentSchemeVerifier,
        commitment: felt252,
        log_sizes: Array<u32>,
        channel: Channel
    ) -> Channel;

    fn verify_values(
        self: @CommitmentSchemeVerifier,
        sampled_points: Array<Array<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) -> Result<(), VerificationError>;
}

impl CommitmentSchemeVerifierImpl of CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier {
        CommitmentSchemeVerifier { trees: array![], config: config }
    }

    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<Array<u32>> {
        let mut result = array![];

        let mut i = 0;
        while i < self.trees.len() {
            result.append(self.trees.at(i).column_log_sizes.clone());
            i = i + 1;
        };

        result
    }

    fn commit(
        ref self: CommitmentSchemeVerifier,
        commitment: felt252,
        log_sizes: Array<u32>,
        channel: Channel
    ) -> Channel {
        let channel_copy = channel.clone();

        MerkleChannelTraitImpl::mix_root(channel_copy, commitment);
        let log_blowup_factor = self.config.fri_config.log_blowup_factor;
        let mut extended_log_sizes: Array<u32> = ArrayTrait::new();

        let mut i = 0;
        while i < log_sizes.len() {
            extended_log_sizes.append(*log_sizes.at(i) + log_blowup_factor);
            i = i + 1;
        };
        let verifier = MerkleVerifier::<
            PoseidonMerkleHasher
        > { root: commitment, column_log_sizes: extended_log_sizes };

        self.trees.append(verifier);

        channel_copy
    }

    fn verify_values(
        self: @CommitmentSchemeVerifier,
        sampled_points: Array<Array<Array<CirclePoint<QM31>>>>,
        proof: CommitmentSchemeProof,
        ref channel: Channel,
    ) -> Result<(), VerificationError> {
        let flattened = flat_3d_array(proof.sampled_values.span());
        channel.mix_felts(flattened.span());

        let random_coeff = channel.draw_felt();

        let column_log_sizes = self.column_log_sizes();

        let vec_to_sort = substract_map_2d_array(
            column_log_sizes.span(), *self.config.fri_config.log_blowup_factor
        );

        let mut bounds = array![];
        let mut iterator = MaximumToMinimumSortedIterator::iterate(vec_to_sort.span());
        while let Option::Some((_, x)) = iterator.next_deduplicated() {
            bounds.append(x);
        };

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier = FriVerifierImpl::commit(
            ref channel, *self.config.fri_config, proof.fri_proof, bounds
        )
            .unwrap();

        channel.mix_nonce(proof.proof_of_work);

        // Verify merkle decommitments.
        assert_eq!(self.trees.len(), proof.queried_values.len());
        assert_eq!(self.trees.len(), proof.decommitments.len());
        let queried_snap = proof.queried_values.span();
        let (mut fri_query_domains, column_log_sizes) = fri_verifier
            .column_query_positions(ref channel);
        let mut i = 0;
        while i < self.trees.len() {
            //// Get FRI query domains.
            let mut queries: Felt252Dict<Nullable<Array<usize>>> = Default::default();
            let mut j = 0;
            while j < column_log_sizes.len() {
                let log_size = *column_log_sizes.at(j);
                let domain = get_sparse_sub_circle_domain_dict(ref fri_query_domains, log_size);

                queries.insert(log_size.into(), NullableTrait::new(domain.flatten()));
                j = j + 1;
            };
            self
                .trees[i]
                .verify(queries, queried_snap[i].clone(), proof.decommitments[i].clone())
                .unwrap();
            i = i + 1;
        };
        // Answer FRI queries.
        let snap_points = @sampled_points;
        let snap_values = @proof.sampled_values;
        assert_eq!(snap_points.len(), snap_values.len());
        let mut samples: Array<Array<PointSample>> = array![];
        let mut i = 0;
        while i < snap_points.len() {
            let mut j = 0;
            let snap_points_i = snap_points.at(i);
            let snap_values_i = snap_values.at(i);
            assert_eq!(snap_points_i.len(), snap_values_i.len());
            while j < snap_points_i.len() {
                let mut k = 0;
                let snap_points_i_j = snap_points_i.at(j);
                let snap_values_i_j = snap_values_i.at(j);
                let mut col = array![];
                while k < snap_points_i_j.len() {
                    let snap_points_i_j_k = snap_points_i_j.at(k);
                    let snap_values_i_j_k = snap_values_i_j.at(k);
                    col
                        .append(
                            PointSample {
                                point: snap_points_i_j_k.clone(), value: snap_values_i_j_k.clone()
                            }
                        );
                    k = k + 1;
                };
                samples.append(col);
                j = j + 1;
            };
            i = i + 1;
        };

        let mut i = 0;
        let mut flattened_queried_values: Array<Span<M31>> = array![];
        while i < queried_snap.len() {
            let queried_values_i = queried_snap.at(i);
            let mut j = 0;
            while j < queried_values_i.len() {
                flattened_queried_values.append(*queried_values_i.at(j));
                j = j + 1;
            };
            i = i + 1;
        };

        let column_log_sizes = self.column_log_sizes();
        let mut flattened_column_log_sizes = array![];
        let mut i = 0;
        while i < self.column_log_sizes().len() {
            let mut j = 0;
            let column_log_sizes_i = column_log_sizes.at(i);
            while j < column_log_sizes_i.len() {
                flattened_column_log_sizes.append(column_log_sizes_i.at(j).clone());
                j = j + 1;
            };
            i = i + 1;
        };

        let fri_answers: Array<SparseCircleEvaluation> = fri_answers(
            flattened_column_log_sizes,
            samples,
            random_coeff,
            ref fri_query_domains,
            flattened_queried_values
        )
            .unwrap();

        match fri_verifier.decommit(fri_answers) {
            Result::Ok(()) => Result::Ok(()),
            Result::Err(fri_error) => Result::Err(VerificationError::Fri(fri_error))
        }
    }
}

#[derive(Drop)]
pub struct PcsConfig {
    pub pow_bits: u32,
    pub fri_config: FriConfig,
}

#[derive(Drop)]
pub struct CommitmentSchemeProof {
    sampled_values: Array<Array<Array<QM31>>>,
    decommitments: Array<MerkleDecommitment>,
    queried_values: Array<Array<Span<M31>>>,
    proof_of_work: u64,
    fri_proof: FriProof
}

pub trait MerkleChannelTrait {
    fn mix_root(channel: Channel, root: felt252);
}

impl MerkleChannelTraitImpl of MerkleChannelTrait {
    fn mix_root(mut channel: Channel, root: felt252) {
        channel.mix_digest(root)
    }
}

#[derive(Debug, Drop, PartialEq)]
pub enum VerificationError {
    InvalidStructure,
    Error,
    ProofOfWork,
    Fri: FriVerificationError,
}

#[cfg(test)]
mod tests {
    use super::{
        CommitmentSchemeVerifier, PcsConfig, CommitmentSchemeVerifierImpl, CommitmentSchemeProof
    };
    use stwo_cairo_verifier::fri::{FriConfig, FriProof, FriLayerProof};
    use stwo_cairo_verifier::poly::line::LinePoly;
    use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment};
    use stwo_cairo_verifier::channel::{ChannelTime, Channel};
    use stwo_cairo_verifier::circle::CirclePoint;
    use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
    use stwo_cairo_verifier::fields::m31::m31;

    #[test]
    fn test_pcs_verifier() {
        let config = PcsConfig {
            pow_bits: 10,
            fri_config: FriConfig {
                log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
            },
        };

        let channel = Channel {
            digest: 0x00, // Default
            channel_time: ChannelTime { n_challenges: 0, // Default
             n_sent: 0 // Default
             }
        };
        let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

        let commitment_1 = 0x01cafae415ba4e4f6648b9c8d0c44a664060485580ac65ff8c161fb756836bd5;
        let sizes_1 = array![10, 10, 10, 10];

        assert_eq!(commitment_scheme.trees.len(), 0);
        let channel = commitment_scheme.commit(commitment_1, sizes_1, channel);
        assert_eq!(commitment_scheme.trees.len(), 1);
        assert_eq!(
            commitment_scheme.trees[0].root,
            @0x01cafae415ba4e4f6648b9c8d0c44a664060485580ac65ff8c161fb756836bd5
        );
        assert_eq!(commitment_scheme.trees[0].column_log_sizes, @array![14, 14, 14, 14]);

        let commitment_2 = 0x0478dd9207927ad71f7c07e332b745a3d3aa08f593fcb033ef756d7438994595;
        let sizes_2 = array![10, 10, 10, 10, 10, 10, 10, 10];
        assert_eq!(commitment_scheme.trees.len(), 1);
        let channel = commitment_scheme.commit(commitment_2, sizes_2, channel);
        assert_eq!(commitment_scheme.trees.len(), 2);
        assert_eq!(
            commitment_scheme.trees[1].root,
            @0x0478dd9207927ad71f7c07e332b745a3d3aa08f593fcb033ef756d7438994595
        );
        assert_eq!(
            commitment_scheme.trees[1].column_log_sizes, @array![14, 14, 14, 14, 14, 14, 14, 14]
        );

        let commitment_3 = 0x032fb1cb4a18da436f91b455ef3a8153b55eab841ba8b3fee7fa33ec050356bc;
        let sizes_3 = array![10, 10, 10, 10];
        assert_eq!(commitment_scheme.trees.len(), 2);
        commitment_scheme.commit(commitment_3, sizes_3, channel);
        assert_eq!(commitment_scheme.trees.len(), 3);
        assert_eq!(
            commitment_scheme.trees[2].root,
            @0x032fb1cb4a18da436f91b455ef3a8153b55eab841ba8b3fee7fa33ec050356bc
        );
        assert_eq!(commitment_scheme.trees[2].column_log_sizes, @array![14, 14, 14, 14]);
    }

    #[test]
    fn test_pcs_verifier_verify_values() {
        let config = PcsConfig {
            pow_bits: 10,
            fri_config: FriConfig {
                log_last_layer_degree_bound: 0, log_blowup_factor: 1, n_queries: 1
            },
        };

        let channel = Channel {
            digest: 0x00, // Default
            channel_time: ChannelTime { n_challenges: 0, // Default
             n_sent: 0 // Default
             }
        };
        let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

        let commitment_1 = 0x03be559c45bbcfed8f4557f6201ad09cac7bb4d339e70a8aa9886ac3a130ca8e;
        let sizes_1 = array![3];

        assert_eq!(commitment_scheme.trees.len(), 0);
        let mut _channel = commitment_scheme.commit(commitment_1, sizes_1, channel);
        assert_eq!(commitment_scheme.trees.len(), 1);
        assert_eq!(
            commitment_scheme.trees[0].root,
            @0x03be559c45bbcfed8f4557f6201ad09cac7bb4d339e70a8aa9886ac3a130ca8e
        );
        assert_eq!(commitment_scheme.trees[0].column_log_sizes, @array![4]);

        let sample_points = array![
            array![
                array![
                    CirclePoint::<
                        QM31
                    > {
                        x: qm31(1395048677, 640591314, 342871101, 1049385418),
                        y: qm31(474688795, 2119282552, 160740005, 798859953)
                    }
                ]
            ]
        ];
        let proof = CommitmentSchemeProof {
            sampled_values: array![
                array![array![qm31(2082657879, 1175528048, 1000432343, 763013627)]]
            ],
            decommitments: array![
                MerkleDecommitment {
                    hash_witness: array![
                        0x008616ef876c5a76edb3abf09fd8ffd5d80ccadce1ad581844bbaa7235a2da56,
                        0x04d8220ddc27d89ae6846d9191ecf02c1c0bbcb25a6f7ac4685a9bf42f1f010f,
                        0x073f38df4fcaa0170ee42e66ead2de1824dedfa6413be32cc365595d116cc32b
                    ],
                    column_witness: array![]
                }
            ],
            queried_values: array![array![array![m31(1323727772), m31(1323695004)].span()]],
            proof_of_work: 2,
            fri_proof: FriProof {
                inner_layers: array![
                    FriLayerProof {
                        evals_subset: array![qm31(1095793631, 1536834955, 2042516263, 1366783014)],
                        decommitment: MerkleDecommitment {
                            hash_witness: array![
                                0x0574b67cc46ad2d8f1f45ba903b57f7374e0358585e1129276cb0ad055c5ab9e,
                                0x06156efae86345fb4e6c116dcdd41a00c430c91b3304163ba150ad51965f952d
                            ],
                            column_witness: array![]
                        },
                        commitment: 0x0627d9d20f6b14fbd148a257e77e56017fa4984332ceb30d87d71f79564a5541
                    },
                    FriLayerProof {
                        evals_subset: array![qm31(872305103, 1427717794, 368086230, 1461103938)],
                        decommitment: MerkleDecommitment {
                            hash_witness: array![
                                0x0469214fcf5d28d3d24123d4f04b03309dca680896fb50e64cfdabd225050d3b
                            ],
                            column_witness: array![]
                        },
                        commitment: 0x0013431268a11ebb22826f67d87d1625f1064799577cc4709373d93ef05e1971
                    }
                ],
                last_layer_poly: LinePoly {
                    coeffs: array![qm31(42760190, 1889301382, 1748376489, 1325373839)], log_size: 0
                }
            }
        };

        let mut channel = Channel {
            digest: 0x028f415beb869c1e81a7a773bac2943b9a7217814631ce58d1f361d44625aabd,
            channel_time: ChannelTime { n_challenges: 1, // Default
             n_sent: 0 // Default
             }
        };
        assert!(commitment_scheme.verify_values(sample_points, proof, ref channel).is_ok());
    }

    #[test]
    fn test_pcs_verifier_verify_values_with_two_columns() {
        let config = PcsConfig {
            pow_bits: 10,
            fri_config: FriConfig {
                log_last_layer_degree_bound: 0, log_blowup_factor: 2, n_queries: 1
            },
        };

        let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

        let commitment_1 = 0x02be650f83421e25718bc944a3e5ca0b52100be1cb6306cfecc0551b01ee24f1;
        let sizes_1 = array![3, 3];
        let channel = Channel {
            digest: 0x00, // Default
            channel_time: ChannelTime { n_challenges: 0, // Default
             n_sent: 0 // Default
             }
        };

        assert_eq!(commitment_scheme.trees.len(), 0);
        let mut _channel = commitment_scheme.commit(commitment_1, sizes_1, channel);

        let sample_points = array![
            array![
                array![
                    CirclePoint::<
                        QM31
                    > {
                        x: qm31(356457033, 539869277, 1638539218, 1613878625),
                        y: qm31(639930869, 2131654109, 1818864611, 630128131)
                    }
                ],
                array![
                    CirclePoint::<
                        QM31
                    > {
                        x: qm31(356457033, 539869277, 1638539218, 1613878625),
                        y: qm31(639930869, 2131654109, 1818864611, 630128131)
                    },
                ]
            ]
        ];

        let proof = CommitmentSchemeProof {
            sampled_values: array![
                array![
                    array![qm31(559256455, 986819756, 561539378, 1752458196)],
                    array![qm31(559256455, 986819756, 561539378, 1752458196)]
                ]
            ],
            decommitments: array![
                MerkleDecommitment {
                    hash_witness: array![
                        0x0088cdeb082e837ba81e9ce5f293e6d56c58a24988c3acb1482a2ad04d8bb1a8,
                        0x00c9c615e229eb375ef28d7ef29d4506811892b15eac2b450b1d8f13f1cd39d8,
                        0x0715458f765d9d7068ffd3b5600e9c92ed2815bb4eee448b1f99ea961f740d34,
                        0x0165db01d03ac7d360b17d3cfbf1ca0f2260706582e3c1ddefbcc3f2657acd85
                    ],
                    column_witness: array![]
                }
            ],
            queried_values: array![
                array![
                    array![m31(731175456), m31(140407102)].span(),
                    array![m31(731175456), m31(140407102)].span()
                ]
            ],
            proof_of_work: 4,
            fri_proof: FriProof {
                inner_layers: array![
                    FriLayerProof {
                        evals_subset: array![qm31(1004118346, 1000509807, 1653523982, 1428430565)],
                        decommitment: MerkleDecommitment {
                            hash_witness: array![
                                0x002b7d8ecfbd1adc96f20a436d796b48a9c15e27bc5e93d8100a2719b0471268,
                                0x05ab4820bfbadd0a81d6683800674c61377fac610cb7551d369f211831da8b57,
                                0x04f6d5c5405ac31c42399882d8cd66fb5b41a1f28268b6440f772881e1f4bc08
                            ],
                            column_witness: array![]
                        },
                        commitment: 0x0717a053c4ee81421414f5dbd8ccb8791c7a2dc92738882282b49abbb4e36ef8
                    },
                    FriLayerProof {
                        evals_subset: array![qm31(906335630, 1881205341, 387775203, 822761293)],
                        decommitment: MerkleDecommitment {
                            hash_witness: array![
                                0x04bcd7cbdf860b26bf3830bc1bd4529bcadd6717d7441daf9f3138c8dd2fa211,
                                0x04eefde96cc5a781e9e7ae174f286d4fcb834ccdb30f1c152c3f2e6d16ed823c,
                            ],
                            column_witness: array![]
                        },
                        commitment: 0x0043f9df4c8647865883a3db137a891c7745517dcab180a22e88a9f0062bfbc8
                    }
                ],
                last_layer_poly: LinePoly {
                    coeffs: array![qm31(1343873773, 1040148734, 1229933415, 444417470)], log_size: 0
                }
            }
        };

        let mut channel = Channel {
            digest: 0x02c7c944d236c3a1b2d11eb8e32bb747e7eff599dafb90fed0ffb265617a8900,
            channel_time: ChannelTime { n_challenges: 1, // Default
             n_sent: 0 // Default
             }
        };
        assert!(commitment_scheme.verify_values(sample_points, proof, ref channel).is_ok());
    }
}

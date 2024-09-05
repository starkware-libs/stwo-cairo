use core::result::ResultTrait;
use core::array::ToSpanTrait;
use core::array::ArrayTrait;
use core::nullable::{NullableTrait};
use stwo_cairo_verifier::vcs::hasher::MerkleHasher;
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier, MerkleVerifierTrait};
use stwo_cairo_verifier::vcs::hasher::{PoseidonMerkleHasher};
use stwo_cairo_verifier::fri::verifier::{FriConfig, FriProof, FriVerifierImpl};
use stwo_cairo_verifier::channel::{ChannelTime, Channel, ChannelImpl};
use stwo_cairo_verifier::fields::qm31::QM31;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::queries::{SparseSubCircleDomain, SparseSubCircleDomainTrait};
use stwo_cairo_verifier::pcs::quotients::{PointSample, fri_answers};
use stwo_cairo_verifier::fri::evaluation::SparseCircleEvaluation;

#[derive(Drop)]
pub struct CommitmentSchemeVerifier {
    pub trees: Array<MerkleVerifier<PoseidonMerkleHasher>>,
    pub config: PcsConfig,
}

pub trait CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier;

    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<Array<u32>>;

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
        // TODO: code
        CommitmentSchemeVerifier { trees: array![], config: config }
    }

    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<Array<u32>> {
        let mut i = 0;

        let mut result = array![];
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
        // TODO: code
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
        let sampled_values_copy = proof.sampled_values.clone();
        let mut flattened = array![];

        let mut i: u32 = 0;
        while i < sampled_values_copy.len() {
            let mut j: u32 = 0;
            let i_copy = sampled_values_copy.at(i);
            while j < i_copy.len() {
                let mut k: u32 = 0;
                let j_copy = i_copy.at(j);
                while k < j_copy.len() {
                    flattened.append(j_copy.at(k).clone());
                    k = k + 1;
                };
                j = j + 1;
            };
            i = i + 1;
        };
        let bounds = array![]; // TODO

        channel.mix_felts(flattened.span());
        let random_coeff = channel.draw_felt();
        let column_log_sizes = self.column_log_sizes();

        // FRI commitment phase on OODS quotients.
        let mut fri_verifier = FriVerifierImpl::commit(channel, *self.config.fri_config, proof.fri_proof, bounds).unwrap();

        channel.mix_nonce(proof.proof_of_work);

        // Verify merkle decommitments.
        assert_eq!(self.trees.len(), proof.queried_values.len());
        assert_eq!(self.trees.len(), proof.decommitments.len());
        let queried_snap = proof.queried_values.span();

        let mut i = 0;
        while i < self.trees.len() {
            //// Get FRI query domains.
            let fri_query_domains = fri_verifier.column_query_positions(ref channel);
            let mut queries: Felt252Dict<Nullable<Span<usize>>> = Default::default();
            let mut i = 0;
            while i < fri_query_domains.len() {
                let (log_size, domain) = fri_query_domains[i];
                queries.insert(*log_size, NullableTrait::new(domain.flatten()));
                i = i + 1;
            };

            self.trees[i].verify(queries, queried_snap[i].clone(), proof.decommitments[i].clone()).unwrap();
            i = i + 1;
        };

        // Answer FRI queries.
        let snap_points = @sampled_points;
        let snap_values = @proof.sampled_values;
        let mut samples: Array<Array<PointSample>> =  array![];
        let mut i = 0;
        while i < snap_points.len() {
            let mut j = 0;
            let snap_points_i = snap_points.at(i);
            let snap_values_i = snap_values.at(i);
            while j < snap_points_i.len() {
                let mut k = 0;
                let snap_points_i_j = snap_points_i.at(j);
                let snap_values_i_j = snap_values_i.at(j);
                let mut col = array![];
                while k < snap_points_i_j.len() {
                    let snap_points_i_j_k = snap_points_i_j.at(k);
                    let snap_values_i_j_k = snap_values_i_j.at(k);
                    col.append(PointSample {
                            point: snap_points_i_j_k.clone(),
                            value: snap_values_i_j_k.clone()
                        });
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

        // TODO: Make sure this channel is correct (no wrong appends in the middle).
        let fri_query_domains = fri_verifier.column_query_positions(ref channel);
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
            fri_query_domains,
            flattened_queried_values
        ).unwrap();
        fri_verifier.decommit(fri_answers);

        // TODO: code
        Result::Ok(())
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
    Error,
}

#[cfg(test)]
mod tests {
    use super::{
        CommitmentSchemeVerifier, PcsConfig, CommitmentSchemeVerifierImpl, CommitmentSchemeProof
    };
    use stwo_cairo_verifier::fri::verifier::{FriConfig, FriProof, FriLayerProof};
    use stwo_cairo_verifier::fri::polynomial::LinePoly;
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
        let mut channel = commitment_scheme.commit(commitment_1, sizes_1, channel);
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

        assert!(commitment_scheme.verify_values(sample_points, proof, ref channel).is_ok());
    }
}

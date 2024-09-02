use core::array::ArrayTrait;
use stwo_cairo_verifier::vcs::hasher::MerkleHasher;
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier};
use stwo_cairo_verifier::vcs::hasher::{PoseidonMerkleHasher};
use stwo_cairo_verifier::fri::verifier::{FriConfig};
use stwo_cairo_verifier::channel::{ChannelTime, Channel, ChannelImpl};


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
        sampled_points: Array<Array<Array<CirclePointQM31>>>,
        proof: CommitmentSchemeProof,
        channel: Channel,
    ) -> Result<(), VerificationError>;
}

impl CommitmentSchemeVerifierImpl of CommitmentSchemeVerifierTrait {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier {
        // TODO: code
        CommitmentSchemeVerifier {
            trees: array![],
            config: config
        }
    }

    fn column_log_sizes(self: @CommitmentSchemeVerifier) -> Array<Array<u32>> {
        // TODO: code
        array![]
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
        while i < log_sizes.len(){
            extended_log_sizes.append(
                *log_sizes.at(i) + log_blowup_factor
            );
            i = i+1;
        };
        let verifier = MerkleVerifier::<PoseidonMerkleHasher>{root: commitment,
                                      column_log_sizes: extended_log_sizes};
        
        self.trees.append(verifier);

        channel_copy
    }

    fn verify_values(
        self: @CommitmentSchemeVerifier,
        sampled_points: Array<Array<Array<CirclePointQM31>>>,
        proof: CommitmentSchemeProof,
        channel: Channel,
    ) -> Result<(), VerificationError> {
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
pub struct CirclePointQM31 {

}

#[derive(Drop)]
pub struct CommitmentSchemeProof {
    
}

pub trait MerkleChannelTrait {
    fn mix_root(channel: Channel, root: felt252);
}

impl MerkleChannelTraitImpl of MerkleChannelTrait{
    fn mix_root(mut channel: Channel, root: felt252){
        channel.mix_digest(root)
    }
}


#[derive(Debug, Drop, PartialEq)]
pub enum VerificationError {
    Error,
}

#[cfg(test)]
mod tests {
    use super::{CommitmentSchemeVerifier, PcsConfig, CommitmentSchemeVerifierImpl};
    use stwo_cairo_verifier::fri::verifier::FriConfig;
    use stwo_cairo_verifier::channel::{ChannelTime, Channel};

    #[test]
    fn test_pcs_verifier() {
        let config = PcsConfig {
            pow_bits: 10,
            fri_config: FriConfig {
                log_last_layer_degree_bound: 5,
                log_blowup_factor: 4,
                n_queries: 64
            },
        };

        let channel = Channel {
            digest: 0x00, // Default
            channel_time: ChannelTime {
                n_challenges: 0, // Default
                n_sent: 0 // Default
            }
        };
        let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

        let commitment_1 = 0x01cafae415ba4e4f6648b9c8d0c44a664060485580ac65ff8c161fb756836bd5;
        let sizes_1 = array![10, 10, 10, 10];

        assert_eq!(commitment_scheme.trees.len(), 0);
        let channel = commitment_scheme.commit(commitment_1, sizes_1, channel);
        assert_eq!(commitment_scheme.trees.len(), 1);
        assert_eq!(commitment_scheme.trees[0].root, @0x01cafae415ba4e4f6648b9c8d0c44a664060485580ac65ff8c161fb756836bd5);
        assert_eq!(commitment_scheme.trees[0].column_log_sizes, @array![14, 14, 14, 14]);

        let commitment_2 = 0x0478dd9207927ad71f7c07e332b745a3d3aa08f593fcb033ef756d7438994595;
        let sizes_2 = array![10, 10, 10, 10, 10, 10, 10, 10];
        assert_eq!(commitment_scheme.trees.len(), 1);
        let channel = commitment_scheme.commit(commitment_2, sizes_2, channel);
        assert_eq!(commitment_scheme.trees.len(), 2);
        assert_eq!(commitment_scheme.trees[1].root, @0x0478dd9207927ad71f7c07e332b745a3d3aa08f593fcb033ef756d7438994595);
        assert_eq!(commitment_scheme.trees[1].column_log_sizes, @array![14, 14, 14, 14, 14, 14, 14, 14]);

        let commitment_3 = 0x032fb1cb4a18da436f91b455ef3a8153b55eab841ba8b3fee7fa33ec050356bc;
        let sizes_3 = array![10, 10, 10, 10];
        assert_eq!(commitment_scheme.trees.len(), 2);
        commitment_scheme.commit(commitment_3, sizes_3, channel);
        assert_eq!(commitment_scheme.trees.len(), 3);
        assert_eq!(commitment_scheme.trees[2].root, @0x032fb1cb4a18da436f91b455ef3a8153b55eab841ba8b3fee7fa33ec050356bc);
        assert_eq!(commitment_scheme.trees[2].column_log_sizes, @array![14, 14, 14, 14]);
    }
}

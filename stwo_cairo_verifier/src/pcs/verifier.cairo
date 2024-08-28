use stwo_cairo_verifier::vcs::hasher::MerkleHasher;
use stwo_cairo_verifier::vcs::verifier::{MerkleDecommitment, MerkleVerifier};
use stwo_cairo_verifier::fri::verifier::{FriConfig};
use stwo_cairo_verifier::channel::{Channel};

/// The verifier side of a FRI polynomial commitment scheme. See [super].
pub struct CommitmentSchemeVerifier<impl H: MerkleHasher> {
    pub trees: Array<MerkleVerifier<H>>,
    pub config: PcsConfig,
}

pub trait CommitmentSchemeVerifierTrait<impl H: MerkleHasher> {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier<H>;

    fn column_log_sizes(self: @CommitmentSchemeVerifier<H>) -> Array<Array<u32>>;

    fn commit(
        self: @CommitmentSchemeVerifier<H>,
        commitment: H::Hash,
        log_sizes: Array<u32>,
        channel: Channel
    );

    fn verify_values(
        self: @CommitmentSchemeVerifier<H>,
        sampled_points: Array<Array<Array<CirclePointQM31>>>,
        proof: CommitmentSchemeProof,
        channel: Channel,
    ) -> Result<(), VerificationError>;
}

impl CommitmentSchemeVerifierImpl<impl H: MerkleHasher, +Drop<H::Hash>> of CommitmentSchemeVerifierTrait<H> {
    fn new(config: PcsConfig) -> CommitmentSchemeVerifier<H> {
        // TODO: code
        CommitmentSchemeVerifier::<H> {
            trees: array![],
            config: config
        }
    }

    fn column_log_sizes(self: @CommitmentSchemeVerifier<H>) -> Array<Array<u32>> {
        // TODO: code
        array![]
    }

    fn commit(
        self: @CommitmentSchemeVerifier<H>,
        commitment: H::Hash,
        log_sizes: Array<u32>,
        channel: Channel
    ) {
        // TODO: code
    }

    fn verify_values(
        self: @CommitmentSchemeVerifier<H>,
        sampled_points: Array<Array<Array<CirclePointQM31>>>,
        proof: CommitmentSchemeProof,
        channel: Channel,
    ) -> Result<(), VerificationError> {
        // TODO: code
        Result::Ok(())
    }
}

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

#[derive(Debug, Drop, PartialEq)]
pub enum VerificationError {
    Error,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pcs_verifier() {
        //assert_eq(0, 1);
    }
}

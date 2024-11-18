use stwo_cairo_verifier::fri::FriVerificationError;
use stwo_cairo_verifier::vcs::verifier::MerkleVerificationError;

#[derive(Clone, Drop)]
pub enum VerificationError {
    /// Proof has invalid structure.
    InvalidStructure: felt252,
    /// Lookup values do not match.
    InvalidLookup: felt252,
    /// Merkle proof invalid.
    Merkle: MerkleVerificationError,
    /// Proof of work verification failed.
    ProofOfWork,
    /// FRI proof is invalid.
    Fri: FriVerificationError,
    /// Invalid OODS eval.
    OodsNotMatching,
}

pub impl FriVerificationErrorIntoVerificationError of Into<
    FriVerificationError, VerificationError
> {
    fn into(self: FriVerificationError) -> VerificationError {
        VerificationError::Fri(self)
    }
}

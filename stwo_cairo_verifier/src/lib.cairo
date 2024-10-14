mod channel;
mod circle;
mod fields;
mod fri;
mod pcs;
mod poly;
mod queries;
mod utils;
mod vcs;

pub use fields::{BaseField, SecureField};

fn main() {}

#[derive(Clone, Drop)]
pub enum VerificationError {
    /// Proof has invalid structure.
    InvalidStructure: felt252,
    /// Lookup values do not match.
    InvalidLookup: felt252,
    /// Merkle proof invalid.
    Merkle: vcs::verifier::MerkleVerificationError,
    /// Proof of work verification failed.
    ProofOfWork,
    // OodsNotMatching,
// Fri(#[from] FriVerificationError),
}

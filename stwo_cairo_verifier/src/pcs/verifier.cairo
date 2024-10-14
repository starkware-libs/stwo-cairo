use stwo_cairo_verifier::fri::FriVerificationError;

#[derive(Debug, Drop, PartialEq)]
pub enum VerificationError {
    InvalidStructure,
    Error,
    ProofOfWork,
    Fri: FriVerificationError,
}

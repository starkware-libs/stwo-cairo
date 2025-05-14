// TODO(anatg): Change to #[cfg(any(not(feature: "poseidon252_verifier")), test)] when any is
// supported.
#[cfg(not(feature: "poseidon252_verifier"))]
pub mod blake2s_hasher;
#[cfg(test)]
#[cfg(not(feature: "poseidon252_verifier"))]
mod blake2s_hasher_test;

pub mod hasher;
// TODO(anatg): Add #[cfg(any(feature: "poseidon252_verifier"), test)] when any is supported.
mod poseidon_hasher;
#[cfg(test)]
mod poseidon_hasher_test;

pub mod verifier;
#[cfg(not(feature: "poseidon252_verifier"))]
pub use blake2s_hasher::Blake2sMerkleHasher as MerkleHasher;
#[cfg(feature: "poseidon252_verifier")]
pub use poseidon_hasher::PoseidonMerkleHasher as MerkleHasher;

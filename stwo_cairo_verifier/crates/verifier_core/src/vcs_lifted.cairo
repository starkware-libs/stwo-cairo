#[cfg(not(feature: "poseidon252_verifier"))]
pub mod blake2s_hasher;
#[cfg(test)]
#[cfg(not(feature: "poseidon252_verifier"))]
mod blake2s_hasher_test;

pub mod hasher;

#[cfg(feature: "poseidon252_verifier")]
mod poseidon_hasher;
#[cfg(test)]
#[cfg(feature: "poseidon252_verifier")]
mod poseidon_hasher_test;

pub mod verifier;
#[cfg(test)]
mod verifier_test;
#[cfg(not(feature: "poseidon252_verifier"))]
pub use blake2s_hasher::Blake2sMerkleHasher as MerkleHasher;
#[cfg(feature: "poseidon252_verifier")]
pub use poseidon_hasher::PoseidonMerkleHasher as MerkleHasher;


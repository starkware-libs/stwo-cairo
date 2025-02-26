pub mod blake2s_hasher;
pub mod hasher;
mod poseidon_hasher;
pub mod verifier;
#[cfg(feature: "blake2s_verifier")]
pub use blake2s_hasher::Blake2sMerkleHasher as MerkleHasher;
#[cfg(not(feature: "blake2s_verifier"))]
pub use poseidon_hasher::PoseidonMerkleHasher as MerkleHasher;

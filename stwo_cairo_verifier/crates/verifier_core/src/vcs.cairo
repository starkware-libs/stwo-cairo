pub mod blake2s_hasher;
pub mod hasher;
mod poseidon_hasher;
pub mod verifier;
#[cfg(not(feature: "poseidon252_verifier"))]
pub use blake2s_hasher::Blake2sMerkleHasher as MerkleHasher;
#[cfg(feature: "poseidon252_verifier")]
pub use poseidon_hasher::PoseidonMerkleHasher as MerkleHasher;

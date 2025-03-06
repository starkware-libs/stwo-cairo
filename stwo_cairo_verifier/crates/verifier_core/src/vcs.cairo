pub mod blake2s_hasher;
mod poseidon_hasher;
pub mod verifier;
#[cfg(feature: "blake2s_verifier")]
pub use blake2s_hasher::Blake2sMerkleHasherImpl as MerkleHasherImpl;
#[cfg(not(feature: "blake2s_verifier"))]
pub use poseidon_hasher::PoseidonMerkleHasherImpl as MerkleHasherImpl;

/// A Merkle node hash is a hash of:
///   [left_child_hash, right_child_hash], column0_value, column1_value, ...
/// "[]" denotes optional values.
/// The largest Merkle layer has no left and right child hashes. The rest of the layers have
/// children hashes.
/// At each layer, the tree may have multiple columns of the same length as the layer.
/// Each node in that layer contains one value from each column.
pub trait MerkleHasher {
    type Hash;

    /// Hashes a single Merkle node.
    fn hash_node(
        children_hashes: Option<(Self::Hash, Self::Hash)>, column_values: Span<crate::BaseField>,
    ) -> Self::Hash;
}

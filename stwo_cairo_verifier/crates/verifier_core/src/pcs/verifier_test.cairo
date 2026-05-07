#[cfg(not(feature: "poseidon252_verifier"))]
use core::box::BoxImpl;
#[cfg(not(feature: "poseidon252_verifier"))]
use crate::vcs::blake2s_hasher::{Blake2sHash, Blake2sMerkleHasher};
#[cfg(not(feature: "poseidon252_verifier"))]
use crate::vcs::verifier::MerkleVerifier;
#[cfg(not(feature: "poseidon252_verifier"))]
use super::verifier::get_trace_lde_log_size;
use super::verifier::prepare_preprocessed_query_positions;

#[test]
fn test_pp_queries_lift() {
    let positions = array![0, 107, 214, 321, 428, 535, 642, 749, 856, 963].span();
    // Compute `pos >> (5 + 1) << 1 + (pos & 1)`
    assert_eq!(
        prepare_preprocessed_query_positions(positions, 15, 10),
        array![0, 3, 6, 11, 12, 17, 20, 23, 26, 31].span(),
    );
    // Positions are not modified if `lifting_log_size == pp_max_log_size`.
    assert_eq!(prepare_preprocessed_query_positions(positions, 10, 10), positions);
    // Compute `pos >> 1 << (5 + 1) + (pos & 1)`
    assert_eq!(
        prepare_preprocessed_query_positions(positions, 10, 15),
        array![0, 3393, 6848, 10241, 13696, 17089, 20544, 23937, 27392, 30785].span(),
    );
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[test]
fn test_get_trace_lde_log_size_ignores_lifted_tree_height() {
    // Heights are lifted (e.g., as if `lifting_log_size = 9 > trace_lde = 5 + 2 = 7`),
    // but `get_trace_lde_log_size` should ignore `tree_height = lifting_log_size` and derive from
    // the trace tree's column log degree bounds + log_blowup_factor.
    let log_blowup_factor: u32 = 2;
    let preprocessed_max_log_deg_bound: u32 = 3;
    let trace_max_log_deg_bound: u32 = 5;
    let lifted_tree_height: u32 = 9;
    let dummy_root = Blake2sHash { hash: BoxImpl::new([0; 8]) };

    let preprocessed = MerkleVerifier::<
        Blake2sMerkleHasher,
    > {
        root: dummy_root,
        tree_height: lifted_tree_height,
        column_log_deg_bounds: array![preprocessed_max_log_deg_bound].span(),
    };
    let trace = MerkleVerifier::<
        Blake2sMerkleHasher,
    > {
        root: dummy_root,
        tree_height: lifted_tree_height,
        column_log_deg_bounds: array![trace_max_log_deg_bound, trace_max_log_deg_bound].span(),
    };
    let interaction = MerkleVerifier::<
        Blake2sMerkleHasher,
    > {
        root: dummy_root,
        tree_height: lifted_tree_height,
        column_log_deg_bounds: array![trace_max_log_deg_bound].span(),
    };
    let trees = array![preprocessed, trace, interaction];

    assert_eq!(
        get_trace_lde_log_size(@trees, log_blowup_factor),
        trace_max_log_deg_bound + log_blowup_factor,
    );
}

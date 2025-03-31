use cairo_air::preprocessed::PreProcessedTrace;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::pcs::CommitmentTreeProver;
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::vcs::ops::MerkleHasher;

/// Generates the root of the preprocessed trace commitment tree for a given `log_blowup_factor`.
// TODO(Shahars): remove allow.
#[allow(unused)]
pub fn generate_preprocessed_commitment_root<MC: MerkleChannel>(
    log_blowup_factor: u32,
) -> <<MC as MerkleChannel>::H as MerkleHasher>::Hash
where
    SimdBackend: BackendForChannel<MC>,
{
    let preprocessed_trace = PreProcessedTrace::canonical();

    // Precompute twiddles for the commitment scheme.
    let max_log_size = preprocessed_trace.log_sizes().into_iter().max().unwrap();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(max_log_size + log_blowup_factor)
            .circle_domain()
            .half_coset,
    );

    // Generate the commitment tree.
    let polys = SimdBackend::interpolate_columns(preprocessed_trace.gen_trace(), &twiddles);
    let commitment_scheme = CommitmentTreeProver::<SimdBackend, MC>::new(
        polys,
        log_blowup_factor,
        &mut MC::C::default(),
        &twiddles,
    );

    commitment_scheme.commitment.root()
}

#[cfg(feature = "slow-tests")]
#[test]
fn test_canonical_preprocessed_root_regression() {
    use stwo_prover::core::vcs::blake2_hash::Blake2sHash;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    let log_blowup_factor = 1;
    let expected = Blake2sHash::from(
        hex::decode("9568f095c1aa2b59bc46f17e4fa12ac0da651a4cafa2b4d9c2e81f04a05864c4")
            .expect("Invalid hex string"),
    );

    let root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(log_blowup_factor);

    assert_eq!(root, expected);
}

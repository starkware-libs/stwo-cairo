use cairo_air::PreProcessedTraceVariant;
use stwo::core::channel::MerkleChannel;
use stwo::core::poly::circle::CanonicCoset;
use stwo::core::vcs::MerkleHasher;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::BackendForChannel;
use stwo::prover::poly::circle::PolyOps;
use stwo::prover::CommitmentTreeProver;

/// Generates the root of the preprocessed trace commitment tree for a given `log_blowup_factor`.
// TODO(Shahars): remove allow.
#[allow(unused)]
pub fn generate_preprocessed_commitment_root<MC: MerkleChannel>(
    log_blowup_factor: u32,
    preprocessed_trace: PreProcessedTraceVariant,
) -> <<MC as MerkleChannel>::H as MerkleHasher>::Hash
where
    SimdBackend: BackendForChannel<MC>,
{
    let preprocessed_trace = preprocessed_trace.to_preprocessed_trace();

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
        false,
    );

    commitment_scheme.commitment.root()
}

#[cfg(feature = "slow-tests")]
#[test]
fn test_canonical_preprocessed_root_regression() {
    use stwo::core::vcs::blake2_hash::Blake2sHash;
    use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    let log_blowup_factor = 1;
    let expected = Blake2sHash::from(
        hex::decode("57ede4a7b442aedf01cd69a6f8864a09aa9af391e398ad20edbf2ea804c471c7")
            .expect("Invalid hex string"),
    );

    let root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(
        log_blowup_factor,
        PreProcessedTraceVariant::Canonical,
    );

    assert_eq!(root, expected);
}

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
    );

    commitment_scheme.commitment.root()
}

#[cfg(feature = "slow-tests")]
#[test]
fn test_canonical_preprocessed_root_regression() {
    use stwo::core::vcs::blake2_hash::Blake2sHash;
    use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    let log_blowup_factor = 1;
    (1..6).for_each(|i| {
        println!("log_blowup_factor: {}", i);
        let cannonical_root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(
            i,
            PreProcessedTraceVariant::Canonical,
        );
        println!("cannonical_root: {:?}", cannonical_root.0.array_chunks::<4>().map(|&bytes| format!("{:#010x}", u32::from_be_bytes(bytes))).collect::<Vec<_>>().join(", "));
        let cannonical_root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(
            i,
            PreProcessedTraceVariant::CanonicalWithoutPedersen,
        );
        println!("no pedersen: {:x?}", cannonical_root);
    });
    let expected = Blake2sHash::from(
        hex::decode("624694144d437d26d82fd305ad88d3b4e4bb2bd3578504745ebe3372c9abba41")
            .expect("Invalid hex string"),
    );

    let root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(
        log_blowup_factor,
        PreProcessedTraceVariant::Canonical,
    );

    assert_eq!(root, expected);
}

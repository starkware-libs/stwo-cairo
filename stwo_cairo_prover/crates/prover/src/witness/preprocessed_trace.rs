use std::sync::Arc;

use cairo_air::PreProcessedTraceVariant;
use stwo::core::channel::MerkleChannel;
use stwo::core::fields::m31::BaseField;
use stwo::core::poly::circle::CanonicCoset;
use stwo::core::vcs_lifted::MerkleHasherLifted;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::BackendForChannel;
use stwo::prover::poly::circle::{CircleEvaluation, PolyOps};
use stwo::prover::poly::BitReversedOrder;
use stwo::prover::CommitmentTreeProver;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;

/// Generates the root of the preprocessed trace commitment tree for a given `log_blowup_factor`.
// TODO(Shahars): remove allow.
#[allow(unused)]
pub fn generate_preprocessed_commitment_root<MC: MerkleChannel>(
    log_blowup_factor: u32,
    preprocessed_trace: PreProcessedTraceVariant,
) -> <<MC as MerkleChannel>::H as MerkleHasherLifted>::Hash
where
    SimdBackend: BackendForChannel<MC>,
{
    let preprocessed_trace = Arc::new(preprocessed_trace.to_preprocessed_trace());

    // Precompute twiddles for the commitment scheme.
    let max_log_size = preprocessed_trace.log_sizes().into_iter().max().unwrap();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(max_log_size + log_blowup_factor)
            .circle_domain()
            .half_coset,
    );

    // Generate the commitment tree.
    let polys = SimdBackend::interpolate_columns(gen_trace(preprocessed_trace), &twiddles);
    let commitment_scheme = CommitmentTreeProver::<SimdBackend, MC>::new(
        polys,
        log_blowup_factor,
        &mut MC::C::default(),
        &twiddles,
        false,
        None,
    );

    commitment_scheme.commitment.root()
}

pub fn gen_trace(
    preprocessed_trace: Arc<PreProcessedTrace>,
) -> Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>> {
    preprocessed_trace
        .columns
        .iter()
        .map(|c| c.gen_column_simd())
        .collect()
}

#[cfg(feature = "slow-tests")]
#[test]
fn test_canonical_preprocessed_root_regression() {
    use stwo::core::vcs::blake2_hash::Blake2sHash;
    use stwo::core::vcs_lifted::blake2_merkle::Blake2sMerkleChannel;

    let log_blowup_factor = 1;
    let expected = Blake2sHash::from(
        hex::decode("f779c325f72d3a0f58e13859c8cf90ef7097ad2a5c17849730cdfcceff16faf6")
            .expect("Invalid hex string"),
    );

    let root = generate_preprocessed_commitment_root::<Blake2sMerkleChannel>(
        log_blowup_factor,
        PreProcessedTraceVariant::Canonical,
    );

    assert_eq!(root, expected);
}

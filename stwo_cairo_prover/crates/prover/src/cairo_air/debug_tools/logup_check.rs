use stwo_cairo_adapter::ProverInput;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Blake2sChannel;
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use crate::cairo_air::air::{lookup_sum, CairoClaimGenerator, CairoInteractionElements};
use crate::cairo_air::prover::LOG_MAX_ROWS;

/// Computes the logup sum without actually generating the proof.
/// Used for debugging purposes.
pub fn quick_interaction_claim(input: ProverInput) -> QM31 {
    let pcs_config = PcsConfig::default();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + pcs_config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );
    let channel = &mut Blake2sChannel::default();
    let mut commitment_scheme =
        CommitmentSchemeProver::<SimdBackend, Blake2sMerkleChannel>::new(pcs_config, &twiddles);

    // Base trace.
    let cairo_claim_generator = CairoClaimGenerator::new(input);
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);

    // Interaction trace.
    let interaction_elements = CairoInteractionElements::draw(channel);
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim =
        interaction_generator.write_interaction_trace(&mut tree_builder, &interaction_elements);

    lookup_sum(&claim, &interaction_elements, &interaction_claim)
}

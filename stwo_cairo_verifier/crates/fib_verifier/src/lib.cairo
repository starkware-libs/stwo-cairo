use stwo_verifier_core::channel::ChannelImpl;
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31One, QM31Zero};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::verifier::{Air, verify};
use stwo_verifier_core::FibAir;
use stwo_verifier_core::verifier::StarkProof;


#[executable]
fn main(proof: StarkProof) {
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15,
        },
    };

    // Verify.
    let log_size = 20;
    let air = FibAir::<128> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], [].span(), ref channel);
    commitment_scheme
        .commit(
            *proof.commitment_scheme_proof.commitments[1],
            ArrayImpl::new_repeated(128, log_size).span(),
            ref channel,
        );

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}
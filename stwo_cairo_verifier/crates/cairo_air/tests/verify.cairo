use stwo_verifier_core::channel::ChannelImpl;
use stwo_verifier_core::circle::{CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl};
use stwo_verifier_core::fields::qm31::{QM31Impl, QM31One};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::verifier::verify;

mod basic_with_blowup_2_proof;

#[test]
#[available_gas(100000000000)]
#[ignore]
fn test_proof_with_blowup_2() {
    let proof = basic_with_blowup_2_proof::proof();

    if let Result::Err(err) = stwo_cairo_air::verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }
}

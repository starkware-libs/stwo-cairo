use stwo_cairo_verifier::channel::ChannelImpl;
use stwo_cairo_verifier::circle::{CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl};
use stwo_cairo_verifier::fields::qm31::{QM31Impl, QM31One};
use stwo_cairo_verifier::fri::FriConfig;
use stwo_cairo_verifier::pcs::PcsConfig;
use stwo_cairo_verifier::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::verifier::verify;
mod airs;

pub mod constraint_framework;
mod proofs;

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_128_column_with_blowup_16() {
//     let proof = proofs::horizontal_fib_128_column_with_blowup_16::proof();
//     let config = PcsConfig {
//         pow_bits: 0,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15,
//         },
//     };

//     // Verify.
//     let log_size = 20;
//     let air = airs::fib::FibAir::<128> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], [].span(), ref
//     channel);
//     commitment_scheme
//         .commit(
//             *proof.commitment_scheme_proof.commitments[1],
//             ArrayImpl::new_repeated(128, log_size).span(),
//             ref channel,
//         );

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_128_column_with_blowup_2() {
//     let proof = proofs::horizontal_fib_128_column_with_blowup_2::proof();
//     let config = PcsConfig {
//         pow_bits: 0,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60,
//         },
//     };

//     // Verify.
//     let log_size = 20;
//     let air = airs::fib::FibAir::<128> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], [].span(), ref
//     channel);
//     commitment_scheme
//         .commit(
//             *proof.commitment_scheme_proof.commitments[1],
//             ArrayImpl::new_repeated(128, log_size).span(),
//             ref channel,
//         );

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

#[test]
#[available_gas(100000000000)]
fn test_cairo_with_blowup_2() {
    let proof = proofs::cairo_basic_with_blowup_2::proof();

    if let Result::Err(err) = airs::cairo::verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }
}

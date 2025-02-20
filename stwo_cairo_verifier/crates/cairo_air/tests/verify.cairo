mod basic_with_blowup_2_proof;

// TODO(andrew): Add back in with new proof data.
#[test]
#[ignore]
#[available_gas(100000000000)]
#[ignore]
fn test_proof_with_blowup_2() {
    let proof = basic_with_blowup_2_proof::proof();
    if let Err(err) = stwo_cairo_air::verify_cairo(proof) {
        panic!("Verification failed: {:?}", err);
    }
}

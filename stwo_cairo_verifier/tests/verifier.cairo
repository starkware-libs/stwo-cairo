use stwo_cairo_verifier::channel::ChannelImpl;
use stwo_cairo_verifier::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_cairo_verifier::fields::qm31::{QM31, QM31Impl, QM31One, QM31Zero};
use stwo_cairo_verifier::fri::FriConfig;
use stwo_cairo_verifier::pcs::PcsConfig;
use stwo_cairo_verifier::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_cairo_verifier::poly::circle::CanonicCosetImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::verifier::{Air, verify};
use stwo_cairo_verifier::{ColumnArray, TreeArray};

mod proofs;

// #[test]
// fn test_deserialise() {
//     let test_proof = proofs::test_proof::proof();
//     println!("TEST PROOF WORKED");
// }

#[test]
#[available_gas(100000000000)]
fn test_horizontal_fib_128_column_with_blowup_16() {
    let proof = proofs::horizontal_fib_128_column_with_blowup_16::proof();
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15,
        },
    };

    // Verify.
    let log_size = 20;
    let air = HorizontalFibAir::<128> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], @array![], ref channel);
    commitment_scheme
        .commit(
            *proof.commitment_scheme_proof.commitments[1],
            @ArrayImpl::new_repeated(128, log_size),
            ref channel,
        );

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

#[test]
#[available_gas(100000000000)]
fn test_horizontal_fib_128_column_with_blowup_2() {
    let proof = proofs::horizontal_fib_128_column_with_blowup_2::proof();
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60,
        },
    };

    // Verify.
    let log_size = 20;
    let air = HorizontalFibAir::<128> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], @array![], ref channel);
    commitment_scheme
        .commit(
            *proof.commitment_scheme_proof.commitments[1],
            @ArrayImpl::new_repeated(128, log_size),
            ref channel,
        );

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

#[derive(Drop)]
struct HorizontalFibAir<const N_COLUMNS: usize> {
    log_size: u32,
}

impl HorizontalFibAirImpl<const N_COLUMNS: usize> of Air<HorizontalFibAir<N_COLUMNS>> {
    fn composition_log_degree_bound(self: @HorizontalFibAir<N_COLUMNS>) -> u32 {
        *self.log_size + 1
    }

    fn mask_points(
        self: @HorizontalFibAir<N_COLUMNS>, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        array![array![], ArrayImpl::new_repeated(N_COLUMNS, array![point])]
    }

    fn eval_composition_polynomial_at_point(
        self: @HorizontalFibAir<N_COLUMNS>,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let base_trace_tree = mask_values[1].span();
        let mut constraint_acc = QM31Zero::zero();

        for i in 2..N_COLUMNS {
            let a_col: @Array<QM31> = base_trace_tree[i - 2];
            let b_col: @Array<QM31> = base_trace_tree[i - 1];
            let c_col: @Array<QM31> = base_trace_tree[i];
            let a: QM31 = *a_col[0];
            let b: QM31 = *b_col[0];
            let c: QM31 = *c_col[0];
            let constraint: QM31 = c - b * b - a * a;
            constraint_acc = constraint_acc * random_coeff + constraint;
        };

        let trace_domain = CanonicCosetImpl::new(*self.log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraint_acc / vanish_eval
    }
}

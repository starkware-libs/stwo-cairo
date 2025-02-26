use stwo_verifier_core::channel::poseidon252::new_channel;
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
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray, TreeSpan};

mod fib_128_column_with_blowup_16_proof;
mod fib_128_column_with_blowup_2_proof;

// TODO(andrew): Add back in with new proof data.
#[test]
#[available_gas(100000000000)]
#[ignore]
fn test_horizontal_fib_128_column_with_blowup_16() {
    let proof = fib_128_column_with_blowup_16_proof::proof();
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15,
        },
    };

    // Verify.
    let log_size = 20;
    let air = FibAir::<128> { log_size };
    let mut channel = new_channel(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], [].span(), ref channel);
    commitment_scheme
        .commit(
            *proof.commitment_scheme_proof.commitments[1],
            ArrayImpl::new_repeated(n: 128, v: log_size).span(),
            ref channel,
        );

    if let Err(err) = verify(air, ref channel, proof, commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

// TODO(andrew): Add back in with new proof data.
#[test]
#[available_gas(100000000000)]
#[ignore]
fn test_horizontal_fib_128_column_with_blowup_2() {
    let proof = fib_128_column_with_blowup_2_proof::proof();
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60,
        },
    };

    // Verify.
    let log_size = 20;
    let air = FibAir::<128> { log_size };
    let mut channel = new_channel(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme.commit(*proof.commitment_scheme_proof.commitments[0], [].span(), ref channel);
    commitment_scheme
        .commit(
            *proof.commitment_scheme_proof.commitments[1],
            ArrayImpl::new_repeated(n: 128, v: log_size).span(),
            ref channel,
        );

    if let Err(err) = verify(air, ref channel, proof, commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

#[derive(Drop)]
pub struct FibAir<const N_COLUMNS: usize> {
    pub log_size: u32,
}

impl FibAirImpl<const N_COLUMNS: usize> of Air<FibAir<N_COLUMNS>> {
    fn composition_log_degree_bound(self: @FibAir<N_COLUMNS>) -> u32 {
        *self.log_size + 1
    }

    fn mask_points(
        self: @FibAir<N_COLUMNS>, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        array![array![], ArrayImpl::new_repeated(n: N_COLUMNS, v: array![point])]
    }

    fn eval_composition_polynomial_at_point(
        self: @FibAir<N_COLUMNS>,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let base_trace_tree = *mask_values[1];
        let mut constraint_acc = QM31Zero::zero();

        for i in 2..N_COLUMNS {
            let a_col = *base_trace_tree[i - 2];
            let b_col = *base_trace_tree[i - 1];
            let c_col = *base_trace_tree[i];
            let a = *a_col[0];
            let b = *b_col[0];
            let c = *c_col[0];
            let constraint = c - b * b - a * a;
            constraint_acc = constraint_acc * random_coeff + constraint;
        }

        let trace_domain = CanonicCosetImpl::new(*self.log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraint_acc / vanish_eval
    }
}

use stwo_cairo_verifier::channel::ChannelImpl;
use stwo_cairo_verifier::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl
};
use stwo_cairo_verifier::fields::qm31::{QM31, QM31Impl, QM31One, QM31Zero};
use stwo_cairo_verifier::fri::FriConfig;
use stwo_cairo_verifier::pcs::PcsConfig;
use stwo_cairo_verifier::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_cairo_verifier::poly::circle::CanonicCosetImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::verifier::{Air, verify};
use stwo_cairo_verifier::{TreeArray, ColumnArray};
use stwo_cairo_verifier_integrationtest::proofs;

// TODO(andrew): Report bug. Data is invalid i.e. for some reason when using 
// this type it thinks `log_last_layer_degree_bound` is 4.
//
// const PCS_CONFIG: PcsConfig =
//     PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
//         }
//     };

#[test]
#[available_gas(100000000000)]
fn test_fib_1_column() {
    let proof = proofs::fib_1_column::proof();
    let config = PcsConfig {
        pow_bits: 10,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
        }
    };

    // Verify.
    let log_size = 10;
    let air = FibAir::<1> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    // Trace columns.
    commitment_scheme.commit(*proof.commitments[0], @array![log_size], ref channel);
    // Constant columns.
    commitment_scheme.commit(*proof.commitments[1], @array![log_size], ref channel);

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

#[test]
#[available_gas(100000000000)]
fn test_fib_2_column() {
    let proof = proofs::fib_2_column::proof();
    let config = PcsConfig {
        pow_bits: 10,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
        }
    };

    // Verify.
    let log_size = 10;
    let air = FibAir::<2> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    // Trace columns.
    commitment_scheme
        .commit(*proof.commitments[0], @ArrayImpl::new_repeated(2, log_size), ref channel);
    // Constant columns.
    commitment_scheme.commit(*proof.commitments[1], @array![log_size], ref channel);

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

#[test]
#[available_gas(100000000000)]
fn test_fib_10_column() {
    let proof = proofs::fib_10_column::proof();
    let config = PcsConfig {
        pow_bits: 10,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
        }
    };

    // Verify.
    let log_size = 10;
    let air = FibAir::<10> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    // Trace columns.
    commitment_scheme
        .commit(*proof.commitments[0], @ArrayImpl::new_repeated(10, log_size), ref channel);
    // Constant columns.
    commitment_scheme.commit(*proof.commitments[1], @array![log_size], ref channel);

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

#[derive(Drop)]
struct FibAir<const N_COLUMNS: usize> {
    log_size: u32,
}

impl FibAirImpl<const N_COLUMNS: usize> of Air<FibAir<N_COLUMNS>> {
    fn composition_log_degree_bound(self: @FibAir<N_COLUMNS>) -> u32 {
        *self.log_size + 1
    }

    fn mask_points(
        self: @FibAir<N_COLUMNS>, point: CirclePoint<QM31>
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        let trace_gen = CirclePointIndexImpl::subgroup_gen(*self.log_size).to_point().into();
        let trace_gen_neg = -trace_gen;

        let curr = point;
        let prev = curr.add_circle_point_m31(trace_gen_neg);
        let prev_prev = prev.add_circle_point_m31(trace_gen_neg);

        let base_trace_points = ArrayImpl::new_repeated(N_COLUMNS, array![curr, prev, prev_prev]);
        let constant_trace_points = array![array![curr, prev]];

        array![base_trace_points, constant_trace_points,]
    }

    fn eval_composition_polynomial_at_point(
        self: @FibAir<N_COLUMNS>,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31
    ) -> QM31 {
        let base_trace_tree = mask_values[0].span();
        let constant_trace_tree = mask_values[1].span();
        let constant_trace_col0 = constant_trace_tree[0].span();
        let is_first = *constant_trace_col0[0];
        let is_second = *constant_trace_col0[1];

        let is_first_or_second = is_first + is_second;
        let is_not_first_or_second = QM31One::one() - is_first_or_second.clone();

        let mut constraint_acc = QM31Zero::zero();

        for column_i in 0
            ..N_COLUMNS {
                let base_trace_col = base_trace_tree[column_i].span();
                let curr = *base_trace_col[0];
                let prev = *base_trace_col[1];
                let prev_prev = *base_trace_col[2];

                let constraint0 = (curr - prev - prev_prev) * is_not_first_or_second;
                let constraint1 = (curr - QM31One::one()) * is_first_or_second;

                constraint_acc = (constraint_acc * random_coeff + constraint0) * random_coeff
                    + constraint1;
            };

        let trace_domain = CanonicCosetImpl::new(*self.log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraint_acc / vanish_eval
    }
}

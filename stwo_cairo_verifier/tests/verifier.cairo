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

mod proofs;

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

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_5_column() {
//     let proof = proofs::horizontal_fib_5_column::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
//         }
//     };

//     // Verify.
//     let log_size = 10;
//     let air = HorizontalFibAir::<5> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(5, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_8_column() {
//     let proof = proofs::horizontal_fib_8_column::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
//         }
//     };

//     // Verify.
//     let log_size = 10;
//     let air = HorizontalFibAir::<8> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(8, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_10_column() {
//     let proof = proofs::horizontal_fib_10_column::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
//         }
//     };

//     // Verify.
//     let log_size = 10;
//     let air = HorizontalFibAir::<10> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(10, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_15_column() {
//     let proof = proofs::horizontal_fib_15_column::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 5, log_blowup_factor: 4, n_queries: 64
//         }
//     };

//     // Verify.
//     let log_size = 10;
//     let air = HorizontalFibAir::<15> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(15, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_32_column_with_blowup_2() {
//     let proof = proofs::horizontal_fib_32_column_with_blowup_2::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<32> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(32, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

#[test]
#[available_gas(100000000000)]
fn test_horizontal_fib_128_column_with_blowup_2() {
    // let _ = starknet::testing::cheatcode::<'print_program_size'>(array![].span());
    let proof = proofs::horizontal_fib_128_column_with_blowup_2::proof();
    let config = PcsConfig {
        pow_bits: 10,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60
        }
    };

    // Verify.
    let log_size = 20;
    let air = HorizontalFibAir::<128> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme
        .commit(*proof.commitments[0], @ArrayImpl::new_repeated(128, log_size), ref channel);

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_32_column_with_blowup_16() {
//     let proof = proofs::horizontal_fib_32_column_with_blowup_16::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 6, log_blowup_factor: 4, n_queries: 15
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<32> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(32, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

#[test]
#[available_gas(100000000000)]
fn test_horizontal_fib_128_column_with_blowup_16() {
    let proof = proofs::horizontal_fib_128_column_with_blowup_16::proof();
    let config = PcsConfig {
        pow_bits: 10,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15
        }
    };

    // Verify.
    let log_size = 20;
    let air = HorizontalFibAir::<128> { log_size };
    let mut channel = ChannelImpl::new(0);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

    // Decommit.
    commitment_scheme
        .commit(*proof.commitments[0], @ArrayImpl::new_repeated(128, log_size), ref channel);

    if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
        panic!("Verification failed: {:?}", err);
    }
}

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_256_column_with_blowup_16() {
//     let proof = proofs::horizontal_fib_256_column_with_blowup_16::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<256> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(256, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// // fn partition_into_bit_segments<const N: usize>(
// //     mut value: Simd<u32, N_LANES>,
// //     mut n_bits_per_segment: [usize; N],
// // ) -> [Simd<u32, N_LANES>; N] {
// //     let mut segments = [Simd::splat(0); N];
// //     for (segment, segment_n_bits) in zip(&mut segments, n_bits_per_segment) {
// //         let mask = Simd::splat((1 << segment_n_bits) - 1);
// //         *segment = value & mask;
// //         value >>= segment_n_bits;
// //     }
// //     segments
// // }

// //     n_bits_per_secment.reverse();
// //     n_bits_per_segment.map(|segment_n_bits| {
// //         let mask = Simd::splat((1 << segment_n_bits) - 1);
// //         let segment = value & mask;
// //         value >>= segment_n_bits;
// //         segment
// //     });
// // }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_512_column_with_blowup_16() {
//     let proof = proofs::horizontal_fib_512_column_with_blowup_16::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<512> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(512, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }


// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_1024_column_with_blowup_2() {
//     let proof = proofs::horizontal_fib_1024_column_with_blowup_2::proof();
//     let config = PcsConfig {
//         pow_bits: 11,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<1024> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(1024, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }


// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_1024_column_with_blowup_2_dummy() {
//     let proof = proofs::horizontal_fib_1024_column_with_blowup_2_dummy::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60
//         }
//     };

//     // Verify.
//     let log_size = 10;
//     let air = HorizontalFibAir::<1024> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(1024, log_size), ref channel);

//     // println!("channel digest after commit: {}", channel.digest);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_1024_column_with_blowup_16() {
//     let proof = proofs::horizontal_fib_1024_column_with_blowup_16::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 4, log_blowup_factor: 4, n_queries: 15
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<1024> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(1024, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

// #[test]
// #[available_gas(100000000000)]
// fn test_horizontal_fib_256_column_with_blowup_2() {
//     let proof = proofs::horizontal_fib_256_column_with_blowup_2::proof();
//     let config = PcsConfig {
//         pow_bits: 10,
//         fri_config: FriConfig {
//             log_last_layer_degree_bound: 6, log_blowup_factor: 1, n_queries: 60
//         }
//     };

//     // Verify.
//     let log_size = 20;
//     let air = HorizontalFibAir::<256> { log_size };
//     let mut channel = ChannelImpl::new(0);
//     let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(config);

//     // Decommit.
//     commitment_scheme
//         .commit(*proof.commitments[0], @ArrayImpl::new_repeated(256, log_size), ref channel);

//     if let Result::Err(err) = verify(air, ref channel, proof, ref commitment_scheme) {
//         panic!("Verification failed: {:?}", err);
//     }
// }

#[derive(Drop)]
struct VerticalFibAir<const N_COLUMNS: usize> {
    log_size: u32,
}

impl VerticalFibAirImpl<const N_COLUMNS: usize> of Air<VerticalFibAir<N_COLUMNS>> {
    fn composition_log_degree_bound(self: @VerticalFibAir<N_COLUMNS>) -> u32 {
        *self.log_size + 1
    }

    fn mask_points(
        self: @VerticalFibAir<N_COLUMNS>, point: CirclePoint<QM31>
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
        self: @VerticalFibAir<N_COLUMNS>,
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

#[derive(Drop)]
struct HorizontalFibAir<const N_COLUMNS: usize> {
    log_size: u32,
}

impl HorizontalFibAirImpl<const N_COLUMNS: usize> of Air<HorizontalFibAir<N_COLUMNS>> {
    fn composition_log_degree_bound(self: @HorizontalFibAir<N_COLUMNS>) -> u32 {
        *self.log_size + 1
    }

    fn mask_points(
        self: @HorizontalFibAir<N_COLUMNS>, point: CirclePoint<QM31>
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        array![ArrayImpl::new_repeated(N_COLUMNS, array![point])]
    }

    fn eval_composition_polynomial_at_point(
        self: @HorizontalFibAir<N_COLUMNS>,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31
    ) -> QM31 {
        let base_trace_tree = mask_values[0].span();
        let mut constraint_acc = QM31Zero::zero();

        for i in 2
            ..N_COLUMNS {
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

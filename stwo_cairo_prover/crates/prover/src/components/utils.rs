use itertools::Itertools;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::Column;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

pub fn to_evals(
    trace_values: Vec<BaseColumn>,
) -> Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>> {
    let trace = trace_values
        .into_iter()
        .map(|eval| {
            // TODO(Ohad): Support non-power of 2 inputs.
            let domain = CanonicCoset::new(
                eval.len()
                    .checked_ilog2()
                    .expect("Input is not a power of 2!"),
            )
            .circle_domain();
            CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(domain, eval)
        })
        .collect_vec();
    trace
}

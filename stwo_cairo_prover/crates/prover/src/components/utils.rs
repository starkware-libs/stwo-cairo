use std::ops::{Mul, Sub};

use itertools::Itertools;
use num_traits::{One, Zero};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::Column;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

pub fn to_evals(
    trace_values: Vec<BaseColumn>,
) -> Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>> {
    trace_values
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
        .collect_vec()
}

#[derive(Clone)]
pub struct FakeElements;
impl FakeElements {
    pub fn draw(_channel: &mut impl Channel) -> Self {
        Self
    }
    pub fn combine<F: Copy, EF>(&self, _values: &[F]) -> EF
    where
        EF: Copy + Zero + From<F> + From<SecureField> + Mul<F, Output = EF> + Sub<EF, Output = EF>,
    {
        EF::zero()
    }
    pub fn combine_frac<F: Copy + Zero, EF>(&self, _mult: F, _values: &[F]) -> Fraction<F, EF>
    where
        EF: Copy
            + Zero
            + One
            + From<F>
            + From<SecureField>
            + Mul<F, Output = EF>
            + Sub<EF, Output = EF>,
    {
        Fraction {
            numerator: Zero::zero(),
            denominator: One::one(),
        }
    }
    pub fn dummy() -> Self {
        Self
    }
}

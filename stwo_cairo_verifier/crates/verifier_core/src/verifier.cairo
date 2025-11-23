use core::num::traits::Pow;
use crate::channel::{Channel, ChannelTrait};
use crate::circle::{ChannelGetRandomCirclePointImpl, CirclePoint};
// TODO(Ilya): Remove this once we bump the compiler version.
#[allow(unused_imports)]
use crate::fields::qm31::QM31_EXTENSION_DEGREE;
use crate::fields::qm31::{QM31, QM31Trait};
use crate::pcs::PcsConfigTrait;
use crate::pcs::verifier::{
    CommitmentSchemeProof, CommitmentSchemeVerifier, CommitmentSchemeVerifierImpl,
};
use crate::utils::{ArrayImpl, SpanImpl};
use crate::vcs::MerkleHasher;
use crate::{ColumnSpan, Hash, TreeSpan};

// The composition polynomial is split into 2 polynomials, so it will have the same degree bound as
// the largest trace column.
const LOG_COMPOSITION_SPLIT_FACTOR: u32 = 1;
const COMPOSITION_SPLIT_FACTOR: u32 = 2_u32.pow(LOG_COMPOSITION_SPLIT_FACTOR);

/// Arithmetic Intermediate Representation (AIR).
///
/// An Air instance is assumed to already contain all the information needed to evaluate the
/// constraints. For instance, all interaction elements are assumed to be present in it. Therefore,
/// an AIR is generated only after the initial trace commitment phase.
pub trait Air<T> {
    /// The degree of the composition polynomial.
    fn composition_log_degree_bound(self: @T) -> u32;

    /// Evaluates the constraint quotients combination of the AIR at `point`.
    fn eval_composition_polynomial_at_point(
        self: @T,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31;
}

/// Given a commitment to the traces, and an AIR definition, verifies the proof.
pub fn verify<A, +Air<A>, +Drop<A>>(
    air: A,
    ref channel: Channel,
    proof: StarkProof,
    mut commitment_scheme: CommitmentSchemeVerifier,
    min_security_bits: u32,
    composition_commitment: Hash,
) {
    let StarkProof { commitment_scheme_proof } = proof;

    // Check that there are enough security bits.
    assert!(
        commitment_scheme_proof.config.security_bits() >= min_security_bits,
        "{}",
        VerificationError::SecurityBitsTooLow,
    );

    // Draw a random coefficient from the channel to be used in the composition polynomial.
    // The composition polynomial is defined as: Σ_i (composition_random_coeff^i * quotient_i).
    let composition_random_coeff = channel.draw_secure_felt();

    let composition_log_degree_bound = air.composition_log_degree_bound();
    let split_composition_log_degree_bound = composition_log_degree_bound
        - LOG_COMPOSITION_SPLIT_FACTOR;

    // Read composition polynomial commitment, there are 8 columns, 4 columns for left,
    // and 4 columns for right, where composition(z) = left(z) + pi^{log_size-2} * right(z).
    commitment_scheme
        .commit(
            composition_commitment,
            [split_composition_log_degree_bound; COMPOSITION_SPLIT_FACTOR * QM31_EXTENSION_DEGREE]
                .span(),
            ref channel,
            commitment_scheme_proof.config.fri_config.log_blowup_factor,
        );

    // Draw OOD point.
    let ood_point = channel.get_random_point();

    let sampled_oods_values = commitment_scheme_proof.sampled_values;

    let composition_oods_eval = try_extract_composition_eval(
        sampled_oods_values, ood_point, composition_log_degree_bound,
    )
        .unwrap_or_else(
            || panic!("{}", VerificationError::InvalidStructure('Invalid sampled_values')),
        );

    // Evaluate composition polynomial at OOD point and check that it matches the trace OOD values.
    assert!(
        composition_oods_eval == air
            .eval_composition_polynomial_at_point(
                ood_point, sampled_oods_values, composition_random_coeff,
            ),
        "{}",
        VerificationError::OodsNotMatching,
    );

    commitment_scheme.verify_values(ood_point, commitment_scheme_proof, ref channel);
}

fn circle_double_x(x: QM31) -> QM31 {
    let sqx = x * x;
    sqx + sqx - core::num::traits::One::one()
}
fn repeated_circle_double_x(x: QM31, n: u32) -> QM31 {
    let mut res = x;
    for _ in 0..n {
        res = circle_double_x(res);
    }
    res
}

/// Attempts to extract and compute the composition trace evaluation from the mask.
/// Returns `None` if the mask does not match the expected structure.
fn try_extract_composition_eval(
    mask: TreeSpan<ColumnSpan<Span<QM31>>>,
    oods_point: CirclePoint<QM31>,
    composition_log_size: u32,
) -> Option<QM31> {
    let cols = *mask.last()?;
    let [c0, c1, c2, c3, c4, c5, c6, c7] = (*cols.try_into()?).unbox();
    let [v0] = (*c0.try_into()?).unbox();
    let [v1] = (*c1.try_into()?).unbox();
    let [v2] = (*c2.try_into()?).unbox();
    let [v3] = (*c3.try_into()?).unbox();
    let [v4] = (*c4.try_into()?).unbox();
    let [v5] = (*c5.try_into()?).unbox();
    let [v6] = (*c6.try_into()?).unbox();
    let [v7] = (*c7.try_into()?).unbox();

    let [left, right] = [
        QM31Trait::from_partial_evals([v0, v1, v2, v3]),
        QM31Trait::from_partial_evals([v4, v5, v6, v7]),
    ];
    Some(left + repeated_circle_double_x(x: oods_point.x, n: composition_log_size - 2) * right)
}

#[derive(Drop, Serde)]
pub struct StarkProof {
    pub commitment_scheme_proof: CommitmentSchemeProof,
}

#[derive(Drop, Debug)]
pub enum VerificationError {
    /// Proof has invalid structure.
    InvalidStructure: felt252,
    /// Proof of work verification failed.
    QueriesProofOfWork,
    /// Invalid OODS eval.
    OodsNotMatching,
    /// Security bits are too low.
    SecurityBitsTooLow,
}

impl VerificationErrorDisplay of core::fmt::Display<VerificationError> {
    fn fmt(self: @VerificationError, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            VerificationError::InvalidStructure(error) => write!(
                f, "Proof has invalid structure: {}", error,
            ),
            VerificationError::QueriesProofOfWork => write!(f, "Proof of work verification failed"),
            VerificationError::OodsNotMatching => write!(f, "Invalid OODS eval"),
            VerificationError::SecurityBitsTooLow => write!(f, "Security bits are too low"),
        }
    }
}

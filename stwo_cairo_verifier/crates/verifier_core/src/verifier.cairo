use crate::channel::{Channel, ChannelTrait};
use crate::circle::{ChannelGetRandomCirclePointImpl, CirclePoint};
use crate::fields::qm31::{QM31, QM31Trait, QM31_EXTENSION_DEGREE};
use crate::fri::FriVerificationError;
use crate::pcs::PcsConfigTrait;
use crate::pcs::verifier::{
    CommitmentSchemeProof, CommitmentSchemeVerifier, CommitmentSchemeVerifierImpl,
};
use crate::utils::{ArrayImpl, SpanImpl};
use crate::vcs::MerkleHasher;
use crate::vcs::verifier::MerkleVerificationError;
use crate::{ColumnArray, ColumnSpan, TreeArray, TreeSpan};

/// Arithmetic Intermediate Representation (AIR).
///
/// An Air instance is assumed to already contain all the information needed to evaluate the
/// constraints. For instance, all interaction elements are assumed to be present in it. Therefore,
/// an AIR is generated only after the initial trace commitment phase.
pub trait Air<T> {
    /// The degree of the composition polynomial.
    fn composition_log_degree_bound(self: @T) -> u32;

    /// Returns the mask points for each trace column.
    /// The returned `TreeArray` should be of size `n_interaction_phases`.
    fn mask_points(
        self: @T, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>;

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
) -> Result<(), VerificationError> {
    let random_coeff = channel.draw_felt();
    let StarkProof { commitment_scheme_proof } = proof;

    // Check that there are enough security bits.
    if commitment_scheme_proof.config.security_bits() < min_security_bits {
        return Err(VerificationError::SecurityBitsTooLow);
    }

    // Read composition polynomial commitment.
    commitment_scheme
        .commit(
            commitment_scheme_proof.commitments.last().unwrap().clone(),
            ArrayImpl::new_repeated(n: QM31_EXTENSION_DEGREE, v: air.composition_log_degree_bound())
                .span(),
            ref channel,
        );

    // Draw OOD point.
    let ood_point = channel.get_random_point();

    // Get mask sample points relative to OOD point.
    let mut sample_points = air.mask_points(ood_point);
    // Add the composition polynomial mask points.
    sample_points.append(ArrayImpl::new_repeated(n: QM31_EXTENSION_DEGREE, v: array![ood_point]));

    let sampled_oods_values = commitment_scheme_proof.sampled_values;

    let composition_oods_eval = match extract_composition_eval(sampled_oods_values) {
        Ok(composition_oods_eval) => composition_oods_eval,
        Err(_) => { return Err(VerificationError::InvalidStructure('Invalid sampled_values')); },
    };

    // Evaluate composition polynomial at OOD point and check that it matches the trace OOD values.
    if composition_oods_eval != air
        .eval_composition_polynomial_at_point(ood_point, sampled_oods_values, random_coeff) {
        return Err(VerificationError::OodsNotMatching);
    }

    commitment_scheme.verify_values(sample_points, commitment_scheme_proof, ref channel)?;

    Ok(())
}

/// Extracts the composition trace evaluation from the mask.
fn extract_composition_eval(
    mask: TreeSpan<ColumnSpan<Span<QM31>>>,
) -> Result<QM31, InvalidOodsSampleStructure> {
    let cols = *mask.last().ok_or(InvalidOodsSampleStructure {})?;
    let [c0, c1, c2, c3] = (*cols.try_into().ok_or(InvalidOodsSampleStructure {})?).unbox();
    let [v0] = (*c0.try_into().ok_or(InvalidOodsSampleStructure {})?).unbox();
    let [v1] = (*c1.try_into().ok_or(InvalidOodsSampleStructure {})?).unbox();
    let [v2] = (*c2.try_into().ok_or(InvalidOodsSampleStructure {})?).unbox();
    let [v3] = (*c3.try_into().ok_or(InvalidOodsSampleStructure {})?).unbox();
    Ok(QM31Trait::from_partial_evals([v0, v1, v2, v3]))
}

/// Error when the sampled values have an invalid structure.
#[derive(Clone, Copy, Debug, Drop)]
pub struct InvalidOodsSampleStructure {}

#[derive(Drop, Serde)]
pub struct StarkProof {
    pub commitment_scheme_proof: CommitmentSchemeProof,
}

#[derive(Drop, Debug)]
pub enum VerificationError {
    /// Proof has invalid structure.
    InvalidStructure: felt252,
    /// Lookup values do not match.
    InvalidLookup: felt252,
    /// Merkle proof invalid.
    Merkle: MerkleVerificationError,
    /// Proof of work verification failed.
    ProofOfWork,
    /// FRI proof is invalid.
    Fri: FriVerificationError,
    /// Invalid OODS eval.
    OodsNotMatching,
    /// Security bits are too low.
    SecurityBitsTooLow,
}

pub impl FriVerificationErrorIntoVerificationError of Into<
    FriVerificationError, VerificationError,
> {
    fn into(self: FriVerificationError) -> VerificationError {
        VerificationError::Fri(self)
    }
}

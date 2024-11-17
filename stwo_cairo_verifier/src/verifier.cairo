use stwo_cairo_verifier::channel::{Channel, ChannelTrait};
use stwo_cairo_verifier::circle::{ChannelGetRandomCirclePointImpl, CirclePoint};
use stwo_cairo_verifier::fields::qm31::{QM31, QM31Impl, QM31_EXTENSION_DEGREE};
use stwo_cairo_verifier::fri::FriVerificationError;
use stwo_cairo_verifier::pcs::verifier::{
    CommitmentSchemeProof, CommitmentSchemeVerifier, CommitmentSchemeVerifierImpl,
};
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::vcs::hasher::PoseidonMerkleHasher;
use stwo_cairo_verifier::vcs::verifier::MerkleVerificationError;
use stwo_cairo_verifier::{ColumnArray, TreeArray};

pub trait Air<T> {
    fn composition_log_degree_bound(self: @T) -> u32;

    fn mask_points(
        self: @T, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>;

    fn eval_composition_polynomial_at_point(
        self: @T,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31,
    ) -> QM31;
}

// TODO: Deal with preprocessed columns.
pub fn verify<A, +Air<A>, +Drop<A>>(
    air: A,
    ref channel: Channel,
    proof: StarkProof,
    ref commitment_scheme: CommitmentSchemeVerifier,
) -> Result<(), VerificationError> {
    let random_coeff = channel.draw_felt();

    // Read composition polynomial commitment.
    commitment_scheme
        .commit(
            *proof.commitments[proof.commitments.len() - 1],
            @ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, air.composition_log_degree_bound()),
            ref channel,
        );

    // Draw OODS point.
    let oods_point = channel.get_random_point();

    // Get mask sample points relative to oods point.
    let mut sample_points = air.mask_points(oods_point);
    // Add the composition polynomial mask points.
    sample_points.append(ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, array![oods_point]));

    let sampled_oods_values = @proof.commitment_scheme_proof.sampled_values;

    let composition_oods_eval = match extract_composition_eval(sampled_oods_values) {
        Result::Ok(composition_oods_eval) => composition_oods_eval,
        Result::Err(_) => {
            return Result::Err(VerificationError::InvalidStructure('Invalid sampled_values'));
        },
    };

    // Evaluate composition polynomial at OODS point and check that it matches the trace OODS
    // values. This is a sanity check.
    if composition_oods_eval != air
        .eval_composition_polynomial_at_point(oods_point, sampled_oods_values, random_coeff) {
        return Result::Err(VerificationError::OodsNotMatching);
    }

    commitment_scheme.verify_values(sample_points, proof.commitment_scheme_proof, ref channel)
}

/// Extracts the composition trace evaluation from the mask.
fn extract_composition_eval(
    mask: @TreeArray<ColumnArray<Array<QM31>>>,
) -> Result<QM31, InvalidOodsSampleStructure> {
    let composition_cols = mask[mask.len() - 1];

    if composition_cols.len() != 4 {
        return Result::Err(InvalidOodsSampleStructure {});
    }

    let coordinate_evals = [
        extract_composition_coordinate_eval(composition_cols[0])?,
        extract_composition_coordinate_eval(composition_cols[1])?,
        extract_composition_coordinate_eval(composition_cols[2])?,
        extract_composition_coordinate_eval(composition_cols[3])?,
    ];

    Result::Ok(QM31Impl::from_partial_evals(coordinate_evals))
}

fn extract_composition_coordinate_eval(
    composition_coordinate_col: @Array<QM31>,
) -> Result<QM31, InvalidOodsSampleStructure> {
    if composition_coordinate_col.len() != 1 {
        return Result::Err(InvalidOodsSampleStructure {});
    }

    Result::Ok(*composition_coordinate_col[0])
}

/// Error when the sampled values have an invalid structure.
#[derive(Clone, Copy, Debug, Drop)]
pub struct InvalidOodsSampleStructure {}

#[derive(Drop)]
pub struct StarkProof {
    pub commitments: TreeArray<felt252>,
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
}

pub impl FriVerificationErrorIntoVerificationError of Into<
    FriVerificationError, VerificationError,
> {
    fn into(self: FriVerificationError) -> VerificationError {
        VerificationError::Fri(self)
    }
}

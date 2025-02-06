use crate::channel::{Channel, ChannelTrait};
use crate::circle::{ChannelGetRandomCirclePointImpl, CirclePoint};
use crate::fields::qm31::{QM31, QM31Impl, QM31_EXTENSION_DEGREE};
use crate::fri::FriVerificationError;
use crate::pcs::verifier::{
    CommitmentSchemeProof, CommitmentSchemeVerifier, CommitmentSchemeVerifierImpl,
};
use crate::utils::ArrayImpl;
use crate::vcs::hasher::PoseidonMerkleHasher;
use crate::vcs::verifier::MerkleVerificationError;
use crate::{ColumnArray, ColumnSpan, TreeArray, TreeSpan};

pub trait Air<T> {
    fn composition_log_degree_bound(self: @T) -> u32;

    fn mask_points(
        self: @T, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>>;

    fn eval_composition_polynomial_at_point(
        self: @T,
        point: CirclePoint<QM31>,
        mask_values: TreeSpan<ColumnSpan<Span<QM31>>>,
        random_coeff: QM31,
    ) -> QM31;
}

// TODO: Deal with preprocessed columns.
pub fn verify<A, +Air<A>, +Drop<A>>(
    air: A,
    ref channel: Channel,
    proof: StarkProof<felt252>,
    mut commitment_scheme: CommitmentSchemeVerifier,
) -> Result<(), VerificationError> {
    let random_coeff = channel.draw_felt();
    let StarkProof { commitment_scheme_proof } = proof;

    // Read composition polynomial commitment.
    commitment_scheme
        .commit(
            *commitment_scheme_proof.commitments[commitment_scheme_proof.commitments.len() - 1],
            ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, air.composition_log_degree_bound())
                .span(),
            ref channel,
        );

    // Draw OODS point.
    let oods_point = channel.get_random_point();

    // Get mask sample points relative to oods point.
    let mut sample_points = air.mask_points(oods_point);
    // Add the composition polynomial mask points.
    sample_points.append(ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, array![oods_point]));

    let sampled_oods_values = commitment_scheme_proof.sampled_values;

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

    commitment_scheme.verify_values(sample_points, commitment_scheme_proof, ref channel)?;

    Result::Ok(())
}

/// Extracts the composition trace evaluation from the mask.
fn extract_composition_eval(
    mask: TreeSpan<ColumnSpan<Span<QM31>>>,
) -> Result<QM31, InvalidOodsSampleStructure> {
    let composition_cols = *mask[mask.len() - 1];

    if composition_cols.len() != 4 {
        return Result::Err(InvalidOodsSampleStructure {});
    }

    let coordinate_evals = [
        extract_composition_coordinate_eval(*composition_cols[0])?,
        extract_composition_coordinate_eval(*composition_cols[1])?,
        extract_composition_coordinate_eval(*composition_cols[2])?,
        extract_composition_coordinate_eval(*composition_cols[3])?,
    ];

    Result::Ok(QM31Impl::from_partial_evals(coordinate_evals))
}

fn extract_composition_coordinate_eval(
    composition_coordinate_col: Span<QM31>,
) -> Result<QM31, InvalidOodsSampleStructure> {
    if composition_coordinate_col.len() != 1 {
        return Result::Err(InvalidOodsSampleStructure {});
    }

    Result::Ok(*composition_coordinate_col[0])
}

/// Error when the sampled values have an invalid structure.
#[derive(Clone, Copy, Debug, Drop)]
pub struct InvalidOodsSampleStructure {}

// TODO(andrew): Consider removing this type and Serde.
// Instead just read from a proof buffer like the STARK verifier on Ethereum.
#[derive(Drop)]
pub struct StarkProof<HashT> {
    pub commitment_scheme_proof: CommitmentSchemeProof<HashT>,
}

impl StarkProofSerde<
    HashT, +Drop<HashT>, +core::serde::Serde<HashT>,
> of core::serde::Serde<StarkProof<HashT>> {
    fn serialize(self: @StarkProof<HashT>, ref output: Array<felt252>) {
        self.commitment_scheme_proof.serialize(ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<StarkProof<HashT>> {
        Option::Some(
            StarkProof {
                commitment_scheme_proof: Serde::<
                    CommitmentSchemeProof<HashT>,
                >::deserialize(ref serialized)?,
            },
        )
    }
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

//! `stwo_circuit_air`: AIR-specific verifier-side logic written in Cairo for the stwo-circuits
//! circuit.
use circuit_air::CircuitAirNewImpl;
use core::dict::{Felt252DictTrait, SquashedFelt252DictTrait};
use core::num::traits::Zero;
use stwo_constraint_framework::{LookupElementsImpl, override_preprocessed_trace_log_sizes};
pub use stwo_constraint_framework::{RelationUse, RelationUsesDict, accumulate_relation_uses};
use stwo_verifier_core::Hash;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::m31::{M31Trait, P_U32};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use stwo_verifier_core::pcs::PcsConfigTrait;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::verifier::{StarkProof, VerificationError, verify};

pub mod circuit_air;

pub mod claims;
pub mod component_indices;
use claims::{
    CircuitClaim, CircuitClaimImpl, CircuitInteractionClaim, CircuitInteractionClaimImpl,
    accumulate_circuit_relation_uses, column_log_sizes_per_tree, derive_component_log_sizes,
    lookup_sum,
};
use component_indices::N_COMPONENTS;
pub mod components;
pub mod prelude;
pub mod preprocessed_columns;
pub mod relations;

// Security constants.
pub const INTERACTION_POW_BITS: u32 = 20;
const SECURITY_BITS: u32 = 96;

pub const P_U32_CONST: u32 = P_U32;

#[derive(Drop, Serde)]
pub struct CircuitProof {
    pub claim: CircuitClaim,
    pub interaction_pow: u64,
    pub interaction_claim: CircuitInteractionClaim,
    pub stark_proof: StarkProof,
    /// Salt used in the channel initialization.
    pub channel_salt: u32,
}

/// Configuration parameters that describe the circuit topology. They are
/// NOT mixed into the channel.
#[derive(Drop, Serde)]
pub struct CircuitVerifierConfig {
    /// Number of public output values the circuit produces (its `Output` gates).
    pub n_outputs: u32,
    /// Expected preprocessed trace root.
    pub preprocessed_root: Hash,
    /// Per-column log sizes in the circuit's preprocessed trace, in `crate::preprocessed_columns`
    /// order.
    pub preprocessed_column_log_sizes: Array<u32>,
    /// lifting log size = trace_log_size + log_blowup_factor.
    pub lifting_log_size: u32,
}

/// The output of a circuit verification.
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub output_values: Array<QM31>,
}

/// Returns the output of the verifier: the circuit's public output values.
pub fn get_verification_output(proof: @CircuitProof) -> VerificationOutput {
    let mut output_values = array![];
    for v in proof.claim.public_data.output_values.span() {
        output_values.append(*v);
    }
    VerificationOutput { output_values }
}

pub fn verify_circuit(proof: CircuitProof, config: @CircuitVerifierConfig) {
    let CircuitProof {
        claim, interaction_pow, interaction_claim, stark_proof, channel_salt,
    } = proof;

    // The circuit produces a fixed number of public outputs (its topology); the claim must
    // provide exactly that many.
    assert!(claim.public_data.output_values.len() == *config.n_outputs);

    let pcs_config = stark_proof.commitment_scheme_proof.config;
    assert!(pcs_config.lifting_log_size.is_some());
    let lifting_log_size = pcs_config.lifting_log_size.unwrap();
    assert!(lifting_log_size == *config.lifting_log_size);

    // Component log sizes are derived verifier-side from the preprocessed column log sizes
    // (supplied by the config); they are not carried in the claim.
    let component_log_sizes = derive_component_log_sizes(
        config.preprocessed_column_log_sizes.span(),
    );

    verify_claim(component_log_sizes);

    let mut channel: Channel = Default::default();
    // Mix channel salt. Note that we first reduce it modulo `M31::P`, then cast it as QM31.
    let channel_salt_as_felt: QM31 = M31Trait::reduce_u32(channel_salt).into();
    channel.mix_felts([channel_salt_as_felt].span());

    pcs_config.mix_into(ref channel);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new(pcs_config);

    // Unpack commitments.
    let commitments: @Box<[Hash; 4]> = stark_proof
        .commitment_scheme_proof
        .commitments
        .try_into()
        .unwrap();
    let [
        preprocessed_commitment,
        trace_commitment,
        interaction_trace_commitment,
        composition_commitment,
    ] =
        commitments
        .unbox();

    // Override the aggregate preprocessed log sizes with the config-supplied ones
    let log_sizes = override_preprocessed_trace_log_sizes(
        column_log_sizes_per_tree(component_log_sizes), config.preprocessed_column_log_sizes.span(),
    );
    let log_sizes_box: @Box<[Span<u32>; 3]> = log_sizes.span().try_into().unwrap();
    let [preprocessed_log_sizes, trace_log_sizes, interaction_trace_log_sizes] = log_sizes_box
        .unbox();

    let log_blowup_factor = pcs_config.fri_config.log_blowup_factor;

    // Preprocessed trace.
    assert!(preprocessed_commitment == config.preprocessed_root.clone());
    commitment_scheme.commit(preprocessed_commitment, preprocessed_log_sizes, ref channel);

    // Claim and base trace.
    claim.mix_into(ref channel);
    commitment_scheme.commit(trace_commitment, trace_log_sizes, ref channel);

    // Interaction proof of work.
    assert!(
        channel.verify_pow_nonce(INTERACTION_POW_BITS, interaction_pow),
        "{}",
        VerificationError::InteractionProofOfWork,
    );
    channel.mix_u64(interaction_pow);

    // Pick the interaction elements.
    let common_lookup_elements = LookupElementsImpl::draw(ref channel);
    assert!(
        lookup_sum(@claim, @common_lookup_elements, @interaction_claim).is_zero(),
        "{}",
        VerificationError::InvalidLogupSum,
    );

    // Interaction trace.
    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(interaction_trace_commitment, interaction_trace_log_sizes, ref channel);

    // The circuit commits all trees at the lifted height; the composition degree bound is
    // `trace_log_size + 1` for the degree-2 constraints, where `trace_log_size = lifting - blowup`.
    let circuit_air_log_degree_bound = lifting_log_size - log_blowup_factor + 1;
    let circuit_air = CircuitAirNewImpl::new(
        component_log_sizes, @common_lookup_elements, @interaction_claim,
    );

    verify(
        stark_proof,
        circuit_air,
        circuit_air_log_degree_bound,
        composition_commitment,
        commitment_scheme,
        ref channel,
        SECURITY_BITS,
    );
}

/// Verifies the claim of the circuit proof.
///
/// Checks that, for every lookup relation, the total number of uses across all components is
/// less than `P`. This mirrors the soundness requirement enforced by
/// `stwo-circuits/crates/stark_verifier/src/verify.rs::check_relation_uses`; the Rust version
/// has to work in the QM31 circuit domain and therefore uses a shifted sum
/// (`sum(uses_per_row * (floor(num_rows/DIV) + 1)) < floor(P/DIV)`) to avoid overflow.
/// In Cairo we have native `u64` arithmetic, so the accumulator in
/// `stwo_constraint_framework::accumulate_relation_uses` already sums directly — the check
/// here is the plain `< P` bound on that accumulated `u64`.
fn verify_claim(component_log_sizes: [u32; N_COMPONENTS]) {
    let mut relation_uses: RelationUsesDict = Default::default();
    accumulate_circuit_relation_uses(component_log_sizes, ref relation_uses);

    let squashed = relation_uses.squash();
    let entries = squashed.into_entries();
    for entry in entries {
        let (_relation_id, _first_uses, last_uses) = entry;
        assert!(last_uses < P_U32.into(), "A relation has more than P-1 uses");
    }
}

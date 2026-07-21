//! `stwo_circuit_air`: AIR-specific verifier-side logic written in Cairo for the stwo-circuits
//! circuit.
use circuit_air::CircuitAirNewImpl;
use core::dict::{Felt252DictTrait, SquashedFelt252DictTrait};
use core::num::traits::Zero;
use multiverifier_consts::{
    COMPONENT_LOG_SIZES, N_OUTPUTS, PREPROCESSED_COLUMN_LOG_SIZES, circuit_pcs_config,
};
use stwo_constraint_framework::LookupElementsImpl;
pub use stwo_constraint_framework::{RelationUse, RelationUsesDict, accumulate_relation_uses};
use stwo_verifier_core::Hash;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::m31::{M31Trait, P_U32};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::fields::qm31::QM31Trait;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde};
use stwo_verifier_core::pcs::PcsConfigTrait;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::SpanExTrait;
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_core::vcs::blake2s_hasher::Blake2sHash;
use stwo_verifier_core::verifier::{StarkProof, VerificationError, verify};
#[cfg(not(feature: "poseidon252_verifier"))]
use stwo_verifier_utils::blake2s::hash_u32s;

pub mod circuit_air;

pub mod claims;
pub mod per_component;
use claims::{
    CircuitClaim, CircuitClaimImpl, CircuitInteractionClaim, CircuitInteractionClaimImpl,
    accumulate_circuit_relation_uses, column_log_sizes_per_tree, lookup_sum,
};
use per_component::PerComponent;
pub mod circuit_hash;
pub mod components;
pub mod multiverifier_consts;
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

/// The output of a circuit verification: `blake2s(preprocessed_root || output_values)`,
/// where `preprocessed_root` is the proof's preprocessed-trace (tree 0) commitment.
#[derive(Drop, Serde)]
pub struct VerificationOutput {
    pub output_hash: Hash,
}

/// Returns the output of the verifier: `blake2s(preprocessed_root || output_values)`, where
/// `preprocessed_root` is the proof's preprocessed-trace (tree 0) commitment.
#[cfg(not(feature: "poseidon252_verifier"))]
pub fn get_verification_output(
    commitments: @Box<[Hash; 4]>, output_values: Span<QM31>,
) -> VerificationOutput {
    let [preprocessed_commitment, _, _, _] = commitments.unbox();
    let [r0, r1, r2, r3, r4, r5, r6, r7] = preprocessed_commitment.hash.unbox();
    let mut words = array![r0, r1, r2, r3, r4, r5, r6, r7];
    for value in output_values {
        let [c0, c1, c2, c3] = (*value).to_fixed_array();
        words.append(c0.into());
        words.append(c1.into());
        words.append(c2.into());
        words.append(c3.into());
    }
    VerificationOutput { output_hash: Blake2sHash { hash: hash_u32s(words.span()) } }
}

#[cfg(feature: "poseidon252_verifier")]
pub fn get_verification_output(
    _commitments: @Box<[Hash; 4]>, _output_values: Span<QM31>,
) -> VerificationOutput {
    panic!("the privacy recursive circuit verifier only supports the blake2s hasher")
}

pub fn verify_circuit(proof: CircuitProof) {
    let CircuitProof {
        claim, interaction_pow, interaction_claim, stark_proof, channel_salt,
    } = proof;

    let preprocessed_column_log_sizes = PREPROCESSED_COLUMN_LOG_SIZES;

    // The circuit produces a fixed number of public outputs (its topology); the claim must
    // provide exactly that many.
    assert!(claim.public_data.output_values.len() == N_OUTPUTS);

    // Pin the proof's PCS config to the circuit's hardcoded canonical config. This rejects any
    // proof produced with weaker/mismatched FRI parameters.
    let pcs_config = stark_proof.commitment_scheme_proof.config;
    assert!(pcs_config == circuit_pcs_config(), "unexpected proof pcs config");

    let component_log_sizes = COMPONENT_LOG_SIZES;
    verify_claim(component_log_sizes);

    let mut channel: Channel = Default::default();
    // Mix channel salt. Note that we first reduce it modulo `M31::P`, then cast it as QM31.
    let channel_salt_as_felt: QM31 = M31Trait::reduce_u32(channel_salt).into();
    channel.mix_felts([channel_salt_as_felt].span());

    pcs_config.mix_into(ref channel);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new();

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

    let log_sizes = column_log_sizes_per_tree(component_log_sizes);
    let log_sizes_box: @Box<[Span<u32>; 3]> = log_sizes.span().try_into().unwrap();
    let [_, trace_log_sizes, interaction_trace_log_sizes] = log_sizes_box.unbox();

    let log_blowup_factor = pcs_config.fri_config.log_blowup_factor;

    // Preprocessed trace. The preprocessed column log sizes are the hardcoded ones rather than
    // the values derived from `component_log_sizes`. The preprocessed-trace commitment itself is
    // taken from the proof and exposed in the verification output; binding it to the expected
    // circuit topology is the responsibility of whoever consumes that output.
    commitment_scheme
        .commit(
            preprocessed_commitment,
            preprocessed_column_log_sizes.span(),
            ref channel,
            log_blowup_factor,
        );

    // Claim and base trace.
    claim.mix_into(ref channel);
    commitment_scheme.commit(trace_commitment, trace_log_sizes, ref channel, log_blowup_factor);

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
        .commit(
            interaction_trace_commitment,
            interaction_trace_log_sizes,
            ref channel,
            log_blowup_factor,
        );

    // The circuit commits all trees at the lifted height of its largest extended domain (largest
    // preprocessed column log size + log_blowup_factor). `verify` expects the trace's log degree
    // bound (`trace_log_size = lifting - blowup`); the composition polynomial's raw degree bound is
    // one higher (degree-2 constraints) but it is split into 2 polynomials before LDE, bringing its
    // per-column degree bound back down to the trace's.
    let trace_log_degree_bound = *preprocessed_column_log_sizes.span().max().unwrap();
    let circuit_air = CircuitAirNewImpl::new(
        component_log_sizes, @common_lookup_elements, @interaction_claim,
    );

    verify(
        stark_proof,
        circuit_air,
        trace_log_degree_bound,
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
fn verify_claim(component_log_sizes: PerComponent<u32>) {
    let mut relation_uses: RelationUsesDict = Default::default();
    accumulate_circuit_relation_uses(component_log_sizes, ref relation_uses);

    let squashed = relation_uses.squash();
    let entries = squashed.into_entries();
    for entry in entries {
        let (_relation_id, _first_uses, last_uses) = entry;
        assert!(last_uses < P_U32.into(), "A relation has more than P-1 uses");
    }
}

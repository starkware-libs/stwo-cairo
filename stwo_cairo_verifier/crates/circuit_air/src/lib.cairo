use core::dict::{Felt252DictTrait, SquashedFelt252DictTrait};
use core::num::traits::Zero;
use stwo_constraint_framework::LookupElementsImpl;
pub use stwo_constraint_framework::{RelationUse, RelationUsesDict, accumulate_relation_uses};
use stwo_verifier_core::Hash;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::fields::m31::{M31Trait, P_U32};
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31Trait};
use stwo_verifier_core::fri::FriConfig;
use stwo_verifier_core::pcs::PcsConfig;
use stwo_verifier_core::pcs::verifier::{CommitmentSchemeVerifierImpl, get_trace_lde_log_size};
use stwo_verifier_core::verifier::{StarkProof, verify};

pub mod blake2s_consts;
pub mod circuit_air;
use circuit_air::{CircuitAirNewImpl, override_preprocessed_trace_log_sizes};

pub mod circuit_component;
pub mod claim;
pub mod claims;
use claims::{
    CircuitClaim, CircuitClaimImpl, CircuitInteractionClaim, CircuitInteractionClaimImpl,
    lookup_sum,
};
pub mod components;
pub mod prelude;
pub mod preprocessed_columns;
pub mod relations;

// Security constants. Must match stwo-circuits/crates/cairo_air/src/verify.rs.
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

/// Verifier-side configuration parameters that describe the circuit topology. They are
/// NOT mixed into the channel — they must be agreed between prover and verifier out of
/// band (e.g., compiled into the Cairo binary or supplied as an input). Mirrors the
/// fields of `stwo-circuits/crates/circuit_air/src/verify.rs::CircuitConfig` that the
/// verifier consumes.
#[derive(Drop, Serde)]
pub struct CircuitVerifierConfig {
    /// Variable indices of the circuit's `Output` gates. One entry per public output value.
    pub output_addresses: Array<u32>,
    /// Number of Blake gates in the circuit (drives the Blake-IV public logup contribution).
    pub n_blake_gates: u32,
    /// Expected preprocessed trace root. Matches the prover-side commitment that is produced
    /// during preprocessing; the verifier re-checks the commitment read from the proof
    /// against this value.
    pub preprocessed_root: Hash,
    /// Per-column log sizes in the circuit's preprocessed trace, in the canonical
    /// (prover-side) column order. Used to override the per-component aggregate when
    /// computing the PCS tree log sizes.
    pub preprocessed_column_log_sizes: Array<u32>,
    /// `trace_log_size + log_blowup_factor` — the rust circuit prover packs this into the
    /// channel via `PcsConfig::mix_into`, but cairo's `PcsConfig` does not carry the
    /// field, so it is supplied here out-of-band. Analogous to the rust in-circuit
    /// verifier reading it from `ProofConfig.fri.log_trace_size`.
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

    let pcs_config = stark_proof.commitment_scheme_proof.config;

    verify_claim(@claim);

    let mut channel: Channel = Default::default();
    let channel_salt_as_felt: QM31 = M31Trait::reduce_u32(channel_salt).into();
    channel.mix_felts([channel_salt_as_felt].span());

    // The rust circuit prover mixes `lifting_log_size` alongside the rest of the pcs
    // config (see `PcsConfig::mix_into` in stwo). Cairo's `PcsConfig` doesn't carry it,
    // so it's supplied via `CircuitVerifierConfig`.
    mix_pcs_config_with_lifting(ref channel, pcs_config, *config.lifting_log_size);
    let mut commitment_scheme = CommitmentSchemeVerifierImpl::new();

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

    // Override the aggregate preprocessed log sizes with the config-supplied ones: the
    // preprocessed trace is a single flat trace for the whole circuit, not a concatenation
    // of per-component preprocessed columns.
    let log_sizes = override_preprocessed_trace_log_sizes(
        claim.log_sizes(), config.preprocessed_column_log_sizes.span(),
    );
    let log_sizes_box: @Box<[Span<u32>; 3]> = log_sizes.span().try_into().unwrap();
    let [preprocessed_log_sizes, trace_log_sizes, interaction_trace_log_sizes] = log_sizes_box
        .unbox();

    let log_blowup_factor = pcs_config.fri_config.log_blowup_factor;
    // The circuit prover sets `lifting_log_size = params.trace_log_size + log_blowup_factor`
    // on its `PcsConfig`. Cairo's `PcsConfig` doesn't carry it, so it's supplied via
    // `CircuitVerifierConfig` and threaded into every commit call so the verifier-side
    // merkle trees match the prover-lifted heights (matching stwo's `MerkleVerifierLifted::new(.., Some(h))`).
    let lifting_log_size_opt: Option<u32> = Option::Some(*config.lifting_log_size);
    // The preprocessed root must match the prover-side commitment. Supplied by the
    // verifier config (out-of-band), not carried in the proof.
    assert!(preprocessed_commitment == config.preprocessed_root.clone());

    commitment_scheme
        .commit(
            preprocessed_commitment,
            preprocessed_log_sizes,
            ref channel,
            log_blowup_factor,
            lifting_log_size_opt,
        );
    claim.mix_into(ref channel);

    commitment_scheme
        .commit(
            trace_commitment,
            trace_log_sizes,
            ref channel,
            log_blowup_factor,
            lifting_log_size_opt,
        );
    assert!(
        channel.verify_pow_nonce(INTERACTION_POW_BITS, interaction_pow),
        "{}",
        CircuitVerificationError::InteractionProofOfWork,
    );
    channel.mix_u64(interaction_pow);

    let common_lookup_elements = LookupElementsImpl::draw(ref channel);
    assert!(
        lookup_sum(@claim, @common_lookup_elements, @interaction_claim, config).is_zero(),
        "{}",
        CircuitVerificationError::InvalidLogupSum,
    );

    interaction_claim.mix_into(ref channel);
    commitment_scheme
        .commit(
            interaction_trace_commitment,
            interaction_trace_log_sizes,
            ref channel,
            log_blowup_factor,
            lifting_log_size_opt,
        );

    let trace_lde_log_size = get_trace_lde_log_size(@commitment_scheme.trees);
    let trace_log_size = trace_lde_log_size - pcs_config.fri_config.log_blowup_factor;
    // The maximal constraint degree is 2, so the per-component degree bound is
    // `trace_log_size + 1`. When the prover overrides `lifting_log_size` (which the
    // circuit prover does — it sets `lifting_log_size = params.trace_log_size +
    // log_blowup_factor`, where `params.trace_log_size = max(max_pp_log_size,
    // blake_g_log_size)` and may exceed the trace column log sizes), the composition
    // polynomial is committed at `lifting_log_size - log_blowup_factor`, so the OODS
    // denominator must also be evaluated at that domain. Match stwo's
    // `verify::verify_ex` which uses `max(components.composition_log_degree_bound(),
    // lifting_log_size - log_blowup_factor + 1)`.
    let lifting_log_size = *config.lifting_log_size;
    let lifting_based_bound = lifting_log_size - log_blowup_factor + 1;
    let components_bound = trace_log_size + 1;
    let circuit_air_log_degree_bound =
        if lifting_based_bound > components_bound { lifting_based_bound } else { components_bound };
    let circuit_air = CircuitAirNewImpl::new(@claim, @common_lookup_elements, @interaction_claim);

    verify(
        stark_proof,
        circuit_air,
        circuit_air_log_degree_bound,
        composition_commitment,
        commitment_scheme,
        ref channel,
        SECURITY_BITS,
        lifting_log_size_opt,
    );
}

/// Mixes `pcs_config` into `channel` matching the rust prover's
/// `stwo::core::pcs::PcsConfig::mix_into`, including `lifting_log_size` in the second QM31
/// (slot 1, alongside `fold_step` in slot 0). Cairo's `PcsConfig` does not carry the
/// field, so it's supplied here as a separate argument from `CircuitVerifierConfig`.
fn mix_pcs_config_with_lifting(
    ref channel: Channel, pcs_config: PcsConfig, lifting_log_size: u32,
) {
    let PcsConfig { pow_bits, fri_config } = pcs_config;
    let FriConfig { log_blowup_factor, log_last_layer_degree_bound, n_queries, fold_step } =
        fri_config;
    let zero = stwo_verifier_core::fields::m31::M31 { inner: 0 };
    channel
        .mix_felts(
            array![
                QM31Trait::from_fixed_array(
                    [
                        pow_bits.try_into().unwrap(),
                        log_blowup_factor.try_into().unwrap(),
                        n_queries.try_into().unwrap(),
                        log_last_layer_degree_bound.try_into().unwrap(),
                    ],
                ),
                QM31Trait::from_fixed_array(
                    [
                        fold_step.try_into().unwrap(),
                        lifting_log_size.try_into().unwrap(),
                        zero,
                        zero,
                    ],
                ),
            ]
                .span(),
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
fn verify_claim(claim: @CircuitClaim) {
    let mut relation_uses: RelationUsesDict = Default::default();
    claim.accumulate_relation_uses(ref relation_uses);

    let squashed = relation_uses.squash();
    let entries = squashed.into_entries();
    for entry in entries {
        let (_relation_id, _first_uses, last_uses) = entry;
        assert!(last_uses < P_U32.into(), "A relation has more than P-1 uses");
    }
}

#[derive(Drop, Debug)]
pub enum CircuitVerificationError {
    InteractionProofOfWork,
    InvalidLogupSum,
}

impl CircuitVerificationErrorDisplay of core::fmt::Display<CircuitVerificationError> {
    fn fmt(
        self: @CircuitVerificationError, ref f: core::fmt::Formatter,
    ) -> Result<(), core::fmt::Error> {
        match self {
            CircuitVerificationError::InteractionProofOfWork => write!(
                f, "Interaction Proof Of Work",
            ),
            CircuitVerificationError::InvalidLogupSum => write!(f, "Logup sum is not zero"),
        }
    }
}

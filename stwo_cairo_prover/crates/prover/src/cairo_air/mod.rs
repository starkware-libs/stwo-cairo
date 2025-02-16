pub mod air;
pub mod builtins_air;
mod debug_tools;
pub mod opcodes_air;
pub mod poseidon;
pub mod preprocessed;
pub mod preprocessed_utils;
pub mod range_checks_air;

use air::{lookup_sum, CairoClaimGenerator, CairoComponents, CairoInteractionElements, CairoProof};
use debug_tools::track_cairo_relations;
use num_traits::Zero;
use preprocessed::PreProcessedTrace;
use stwo_prover::constraint_framework::relation_tracker::RelationSummary;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fri::FriConfig;
use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, ProvingError, VerificationError};
use thiserror::Error;
use tracing::{span, Level};

use crate::adapter::ProverInput;
use crate::components::memory::LOG_MEMORY_ADDRESS_BOUND;
use crate::components::memory_address_to_id::component::MEMORY_ADDRESS_TO_ID_SPLIT;

// TODO(Ohad): decide dynamically.
const LOG_MAX_ROWS: u32 = 22;

pub fn prove_cairo<MC: MerkleChannel>(
    input: ProverInput,
    ProverConfig {
        track_relations,
        display_components,
    }: ProverConfig,
) -> Result<CairoProof<MC::H>, ProvingError>
where
    SimdBackend: BackendForChannel<MC>,
{
    let _span = span!(Level::INFO, "prove_cairo").entered();
    // TODO(Ohad): Propogate config from CLI args.
    let config = PcsConfig {
        // NOTE: low pow_bits might yield non-deterministic POWs.
        pow_bits: 20,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 2,
            log_blowup_factor: 1,
            n_queries: 15,
        },
    };
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut MC::C::default();
    let mut commitment_scheme = CommitmentSchemeProver::<SimdBackend, MC>::new(config, &twiddles);

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(PreProcessedTrace::new().gen_trace());
    tree_builder.commit(channel);

    // Run Cairo.
    let cairo_claim_generator = CairoClaimGenerator::new(input);
    // Base trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let span = span!(Level::INFO, "Base trace").entered();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    span.exit();
    claim.mix_into(channel);
    tree_builder.commit(channel);

    if track_relations {
        let relation_summary = RelationSummary::summarize_relations(&track_cairo_relations(
            &commitment_scheme,
            &claim,
        ))
        .cleaned();
        println!("Relations summary: {:?}", relation_summary);
    }

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim =
        interaction_generator.write_interaction_trace(&mut tree_builder, &interaction_elements);

    // Validate lookup argument.
    debug_assert_eq!(
        lookup_sum(&claim, &interaction_elements, &interaction_claim),
        SecureField::zero()
    );

    interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_builder.provers();

    // Prove stark.
    let proof = prove::<SimdBackend, _>(&components, channel, commitment_scheme)?;

    if display_components {
        println!("{}", component_builder);
    }

    Ok(CairoProof {
        claim,
        interaction_claim,
        stark_proof: proof,
    })
}

pub fn verify_cairo<MC: MerkleChannel>(
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<MC::H>,
) -> Result<(), CairoVerificationError> {
    // Auxiliary verifications.
    // Assert that ADDRESS->ID component does not overflow.
    assert!(
        (1 << claim.memory_address_to_id.log_size) * MEMORY_ADDRESS_TO_ID_SPLIT
            <= (1 << LOG_MEMORY_ADDRESS_BOUND)
    );

    // Setup STARK protocol.
    // TODO(Ohad): Propagate config from CLI args.
    let config = PcsConfig {
        pow_bits: 0,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 2,
            log_blowup_factor: 1,
            n_queries: 15,
        },
    };
    let channel = &mut MC::C::default();
    let commitment_scheme_verifier = &mut CommitmentSchemeVerifier::<MC>::new(config);

    let log_sizes = claim.log_sizes();

    // Preproccessed trace.
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &log_sizes[0], channel);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &log_sizes[1], channel);
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Verify lookup argument.
    if lookup_sum(&claim, &interaction_elements, &interaction_claim) != SecureField::zero() {
        return Err(CairoVerificationError::InvalidLogupSum);
    }
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[2], &log_sizes[2], channel);

    let component_generator =
        CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_generator.components();

    // Verify stark.
    verify(
        &components,
        channel,
        commitment_scheme_verifier,
        stark_proof,
    )
    .map_err(CairoVerificationError::Stark)
}

#[derive(Default)]
pub struct ProverConfig {
    /// Display components' metadata.
    pub display_components: bool,
    /// Show the relations that do not sum to 0.
    /// `Relation` is a proof related concept, and will be properly documented in the future.
    /// Used for internal debugging of the lookup argument.
    /// NOTE: Negatively affects performance.
    // TODO(Ohad): Remove this flag.
    pub track_relations: bool,
}
impl ProverConfig {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    track_relations: bool,
    display_components: bool,
}
impl ConfigBuilder {
    pub fn track_relations(mut self, value: bool) -> Self {
        self.track_relations = value;
        self
    }

    pub fn display_components(mut self, value: bool) -> Self {
        self.display_components = value;
        self
    }

    pub fn build(self) -> ProverConfig {
        ProverConfig {
            track_relations: self.track_relations,
            display_components: self.display_components,
        }
    }
}

#[derive(Error, Debug)]
pub enum CairoVerificationError {
    #[error("Invalid logup sum")]
    InvalidLogupSum,
    #[error("Stark verification error: {0}")]
    Stark(#[from] VerificationError),
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use cairo_lang_casm::casm;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    use super::ProverConfig;
    use crate::adapter::plain::input_from_plain_casm;
    use crate::adapter::vm_import::adapt_vm_output;
    use crate::cairo_air::{prove_cairo, verify_cairo, ProverInput};

    /// Creates a prover input from `pub.json`, `priv.json`, `mem`, and `trace` files.
    ///
    /// # Expects
    /// - These files must be stored in the `test_data/test_name` directory and contain valid Cairo
    ///   program data.
    /// - They can be downloaded from Google Storage using `./scripts/download_test_data.sh`.   See
    ///   `input/README.md` for details.
    ///
    /// # Panics
    /// - If it fails to convert the files into a prover input.
    pub fn test_input(test_name: &str) -> ProverInput {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/");
        d.push(test_name);

        adapt_vm_output(d.join("pub.json").as_path(), d.join("priv.json").as_path()).expect(
            "
            Failed to read test files. Checkout input/README.md.",
        )
    }

    fn test_basic_cairo_air_input() -> ProverInput {
        let u128_max = u128::MAX;
        let instructions = casm! {
            // TODO(AlonH): Add actual range check segment.
            // Manually writing range check builtin segment of size 40 to memory.
            [ap] = u128_max, ap++;
            [ap + 38] = 1, ap++;
            ap += 38;

            [ap] = 10, ap++;
            call rel 4;
            jmp rel 11;

            jmp rel 4 if [fp-3] != 0;
            jmp rel 6;
            [ap] = [fp-3] + (-1), ap++;
            call rel (-6);
            ret;
        }
        .instructions;

        input_from_plain_casm(instructions)
    }

    pub fn test_cfg() -> ProverConfig {
        ProverConfig {
            // TODO(Gali): Set track relations to true after implementing get_preprocessed_column in
            // relation_tracker in stwo.
            track_relations: false,
            display_components: true,
        }
    }

    #[test]
    fn test_basic_cairo_air() {
        let cairo_proof =
            prove_cairo::<Blake2sMerkleChannel>(test_basic_cairo_air_input(), test_cfg()).unwrap();
        verify_cairo::<Blake2sMerkleChannel>(cairo_proof).unwrap();
    }

    #[cfg(feature = "nightly")]
    mod nightly_tests {
        use std::io::Write;

        use itertools::Itertools;
        use stwo_cairo_serialize::CairoSerialize;
        use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;

        use super::*;

        #[test]
        fn generate_and_serialise_proof() {
            let cairo_proof =
                prove_cairo::<Poseidon252MerkleChannel>(test_basic_cairo_air_input(), test_cfg())
                    .unwrap();
            let mut output = Vec::new();
            CairoSerialize::serialize(&cairo_proof, &mut output);
            let proof_str = output.iter().map(|v| v.to_string()).join(",");
            let mut file = std::fs::File::create("proof.cairo").unwrap();
            file.write_all(proof_str.as_bytes()).unwrap();
            verify_cairo::<Poseidon252MerkleChannel>(cairo_proof).unwrap();
        }
    }

    #[cfg(feature = "slow-tests")]
    pub mod slow_tests {
        use itertools::Itertools;
        use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

        use super::*;

        #[test]
        fn test_full_cairo_air() {
            let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(
                test_input("test_read_from_small_files"),
                test_cfg(),
            )
            .unwrap();
            verify_cairo::<Blake2sMerkleChannel>(cairo_proof).unwrap();
        }

        #[test]
        fn test_prove_verify_all_opcode_components() {
            let input = test_input("test_prove_verify_all_opcode_components");
            for (opcode, n_instances) in input.state_transitions.casm_states_by_opcode.counts() {
                // TODO(Stav): Remove when `Blake` opcode is in the VM.
                if opcode == "blake2s_opcode" {
                    continue;
                }

                // TODO(Ohadn): Remove when `qm31_add_mul_opcode` opcode is in the VM.
                if opcode == "qm31_add_mul_opcode" {
                    continue;
                }

                assert!(
                    n_instances > 0,
                    "{} isn't used in E2E full-Cairo opcode test",
                    opcode
                );
            }
            let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(
                test_input("test_prove_verify_all_opcode_components"),
                test_cfg(),
            )
            .unwrap();
            verify_cairo::<Blake2sMerkleChannel>(cairo_proof).unwrap();
        }

        #[test]
        fn test_proof_stability() {
            let n_proofs_to_compare = 10;

            let proofs = (0..n_proofs_to_compare)
                .map(|_| {
                    serde_json::to_string(
                        &prove_cairo::<Blake2sMerkleChannel>(
                            test_basic_cairo_air_input(),
                            test_cfg(),
                        )
                        .unwrap(),
                    )
                    .unwrap()
                })
                .collect_vec();

            assert!(proofs.iter().all_equal());
        }
    }
}

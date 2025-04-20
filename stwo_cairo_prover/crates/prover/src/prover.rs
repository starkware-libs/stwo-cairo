use cairo_air::air::{lookup_sum, CairoComponents, CairoInteractionElements};
use cairo_air::verifier::INTERACTION_POW_BITS;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::ProverInput;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fri::FriConfig;
use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::proof_of_work::GrindOps;
use stwo_prover::core::prover::{prove, ProvingError};
use tracing::{event, span, Level};

use crate::witness::cairo::CairoClaimGenerator;
use crate::witness::utils::witness_trace_cells;

pub(crate) const LOG_MAX_ROWS: u32 = 26;

// Stack-overflows induced by the partial_ec_mul write trace function.
// Decreasing this value may cause a stack-overflow during witness generation.
const RAYON_THREAD_STACK_SIZE: usize = 4194304;

pub fn prove_cairo<MC: MerkleChannel>(
    input: ProverInput,
    pcs_config: PcsConfig,
    preprocessed_trace: PreProcessedTraceVariant,
) -> Result<CairoProof<MC::H>, ProvingError>
where
    SimdBackend: BackendForChannel<MC>,
{
    // TODO(Ohad): remove when un-inlining the witness generation is done.
    rayon::ThreadPoolBuilder::new()
        .stack_size(RAYON_THREAD_STACK_SIZE)
        .build_global()
        .unwrap();
    let _span = span!(Level::INFO, "prove_cairo").entered();
    // Composition polynomial domain log size is LOG_MAX_ROWS + 1, double it
    // because we compute on a half-coset, and account for blowup factor.
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + pcs_config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut MC::C::default();
    let mut commitment_scheme =
        CommitmentSchemeProver::<SimdBackend, MC>::new(pcs_config, &twiddles);

    // Preprocessed trace.
    let preprocessed_trace = preprocessed_trace.to_preprocessed_trace();
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(preprocessed_trace.gen_trace());
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

    // Draw interaction elements.
    let interaction_pow = SimdBackend::grind(channel, INTERACTION_POW_BITS);
    channel.mix_u64(interaction_pow);
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let span = span!(Level::INFO, "Interaction trace").entered();
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim =
        interaction_generator.write_interaction_trace(&mut tree_builder, &interaction_elements);
    span.exit();

    tracing::info!(
        "Witness trace cells: {:?}",
        witness_trace_cells(&claim, &preprocessed_trace)
    );
    // Validate lookup argument.
    debug_assert_eq!(
        lookup_sum(&claim, &interaction_elements, &interaction_claim),
        SecureField::zero()
    );

    interaction_claim.mix_into(channel);
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponents::new(
        &claim,
        &interaction_elements,
        &interaction_claim,
        &preprocessed_trace.ids(),
    );

    // TODO(Ohad): move to a testing routine.
    #[cfg(feature = "relation-tracker")]
    {
        use crate::debug_tools::relation_tracker::track_and_summarize_cairo_relations;
        let summary = track_and_summarize_cairo_relations(
            &commitment_scheme,
            &component_builder,
            &claim.public_data,
        );
        tracing::info!("Relations summary: {:?}", summary);
    }

    let components = component_builder.provers();

    // Prove stark.
    let span = span!(Level::INFO, "Prove STARKs").entered();
    let proof = prove::<SimdBackend, _>(&components, channel, commitment_scheme)?;
    span.exit();

    event!(name: "component_info", Level::DEBUG, "Components: {}", component_builder);

    Ok(CairoProof {
        claim,
        interaction_pow,
        interaction_claim,
        stark_proof: proof,
    })
}

#[derive(Default)]
pub struct ProverConfig {
    /// Display components' metadata.
    pub display_components: bool,
}

/// Concrete parameters of the proving system.
/// Used both for producing and verifying proofs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProverParameters {
    /// Channel hash function.
    pub channel_hash: ChannelHash,
    /// Parameters of the commitment scheme.
    pub pcs_config: PcsConfig,
    /// Preprocessed trace.
    pub preprocessed_trace: PreProcessedTraceVariant,
}

/// The hash function used for commitments, for the prover-verifier channel,
/// and for PoW grinding.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelHash {
    /// Default variant, the fastest option.
    Blake2s,
    /// A variant for recursive proof verification.
    /// Note that using `Poseidon252` results in a significant decrease in proving speed compared
    /// to `Blake2s` (because of the large field emulation)
    Poseidon252,
}

/// The default prover parameters for prod use (96 bits of security).
/// The formula is `security_bits = pow_bits + log_blowup_factor * n_queries`.
pub fn default_prod_prover_parameters() -> ProverParameters {
    ProverParameters {
        channel_hash: ChannelHash::Blake2s,
        pcs_config: PcsConfig {
            // Stay within 500ms on M3.
            pow_bits: 26,
            fri_config: FriConfig {
                log_last_layer_degree_bound: 0,
                // Blowup factor > 1 significantly degrades proving speed.
                // Can be in range [1, 16].
                log_blowup_factor: 1,
                // The more FRI queries, the larger the proof.
                // Proving time is not affected much by increasing this value.
                n_queries: 70,
            },
        },
        preprocessed_trace: PreProcessedTraceVariant::Canonical,
    }
}

#[cfg(test)]
pub mod tests {
    use cairo_air::preprocessed::testing_preprocessed_tree;

    use crate::debug_tools::assert_constraints::assert_cairo_constraints;
    use crate::test_utils::prover_input_from_compiled_cairo_program;

    #[test]
    fn test_all_cairo_constraints() {
        let input =
            prover_input_from_compiled_cairo_program("test_prove_verify_all_opcode_components");
        let pp_tree = testing_preprocessed_tree(20);
        assert_cairo_constraints(input, pp_tree);
    }

    #[cfg(test)]
    #[cfg(feature = "nightly")]
    mod nightly_tests {
        use std::io::Write;

        use cairo_air::verifier::verify_cairo;
        use cairo_air::PreProcessedTraceVariant;
        use itertools::Itertools;
        use stwo_cairo_serialize::CairoSerialize;
        use stwo_prover::core::pcs::PcsConfig;
        use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
        use test_log::test;

        use super::*;
        use crate::prover::prove_cairo;

        #[test]
        fn generate_and_serialise_proof() {
            let input =
                prover_input_from_compiled_cairo_program("test_prove_verify_all_opcode_components");
            let preprocessed_trace = PreProcessedTraceVariant::Canonical;
            let cairo_proof = prove_cairo::<Poseidon252MerkleChannel>(
                input,
                PcsConfig::default(),
                preprocessed_trace,
            )
            .unwrap();
            let mut output = Vec::new();
            CairoSerialize::serialize(&cairo_proof, &mut output);
            let proof_str = output.iter().map(|v| v.to_string()).join(",");
            let mut file = std::fs::File::create("proof.cairo").unwrap();
            file.write_all(proof_str.as_bytes()).unwrap();
            verify_cairo::<Poseidon252MerkleChannel>(
                cairo_proof,
                PcsConfig::default(),
                preprocessed_trace,
            )
            .unwrap();
        }
    }

    #[cfg(test)]
    #[cfg(feature = "slow-tests")]
    pub mod slow_tests {
        use std::path::PathBuf;

        use cairo_air::preprocessed::PreProcessedTrace;
        use cairo_air::verifier::verify_cairo;
        use itertools::Itertools;
        use stwo_cairo_adapter::plain::prover_input_from_vm_output;
        use stwo_prover::core::pcs::PcsConfig;
        use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;
        use test_log::test;

        use super::*;
        use crate::debug_tools::assert_constraints::assert_cairo_constraints;
        use crate::prover::{prove_cairo, PreProcessedTraceVariant, ProverInput};

        pub fn get_prover_input_info_path(test_name: &str) -> PathBuf {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../test_data/")
                .join(test_name)
                .join("prover_input_info.json")
        }

        // TODO(Ohad): fine-grained constraints tests.
        #[test]
        fn test_cairo_constraints() {
            let input =
                prover_input_from_compiled_cairo_program("test_prove_verify_all_opcode_components");
            assert_cairo_constraints(input, PreProcessedTrace::canonical_without_pedersen());
        }

        #[test]
        fn test_prove_verify_all_opcode_components() {
            let input =
                prover_input_from_compiled_cairo_program("test_prove_verify_all_opcode_components");
            for (opcode, n_instances) in &input.state_transitions.casm_states_by_opcode.counts() {
                assert!(
                    *n_instances > 0,
                    "{} isn't used in E2E full-Cairo opcode test",
                    opcode
                );
            }
            let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
            let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(
                input,
                PcsConfig::default(),
                preprocessed_trace,
            )
            .unwrap();
            verify_cairo::<Blake2sMerkleChannel>(
                cairo_proof,
                PcsConfig::default(),
                preprocessed_trace,
            )
            .unwrap();
        }

        #[test]
        fn test_prove_verify_all_opcode_components_from_file() {
            let prover_input_file_path =
                get_prover_input_info_path("test_prove_verify_all_opcode_components");
            let input = prover_input_from_vm_output(&prover_input_file_path)
                .expect("Failed to create prover input from vm output");
            for (opcode, n_instances) in &input.state_transitions.casm_states_by_opcode.counts() {
                assert!(
                    *n_instances > 0,
                    "{} isn't used in E2E full-Cairo opcode test",
                    opcode
                );
            }
            let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
            let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(
                input,
                PcsConfig::default(),
                preprocessed_trace,
            )
            .unwrap();
            verify_cairo::<Blake2sMerkleChannel>(
                cairo_proof,
                PcsConfig::default(),
                preprocessed_trace,
            )
            .unwrap();
        }

        // TODO(Ohad): remove ignore.
        #[ignore = "POW is not deterministic"]
        #[test]
        fn test_proof_stability() {
            let n_proofs_to_compare = 10;
            let input =
                prover_input_from_compiled_cairo_program("test_prove_verify_all_opcode_components");

            let proofs = (0..n_proofs_to_compare)
                .map(|_| {
                    serde_json::to_string(
                        &prove_cairo::<Blake2sMerkleChannel>(
                            input.clone(),
                            PcsConfig::default(),
                            PreProcessedTraceVariant::Canonical,
                        )
                        .unwrap(),
                    )
                    .unwrap()
                })
                .collect_vec();

            assert!(proofs.iter().all_equal());
        }

        /// These tests' inputs were generated using cairo-vm with 50 instances of each builtin.
        pub mod builtin_tests {
            use test_log::test;

            use super::*;
            use crate::test_utils::prover_input_from_compiled_cairo_program;

            /// Asserts that all supported builtins are present in the input.
            /// Panics if any of the builtins is missing.
            fn assert_all_builtins_in_input(input: &ProverInput) {
                let empty_builtins: Vec<_> = input
                    .builtins_segments
                    .get_counts()
                    .iter()
                    .filter(|&(_, &count)| count == 0)
                    .map(|(name, _)| format!("{:?}", name))
                    .collect();

                if !empty_builtins.is_empty() {
                    panic!("Builtins missing in the input: {:?}", empty_builtins);
                }
            }

            #[test]
            fn test_prove_verify_all_builtins() {
                let input =
                    prover_input_from_compiled_cairo_program("test_prove_verify_all_builtins");
                assert_all_builtins_in_input(&input);
                let preprocessed_trace = PreProcessedTraceVariant::Canonical;
                let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(
                    input,
                    PcsConfig::default(),
                    preprocessed_trace,
                )
                .unwrap();
                verify_cairo::<Blake2sMerkleChannel>(
                    cairo_proof,
                    PcsConfig::default(),
                    preprocessed_trace,
                )
                .unwrap();
            }

            #[test]
            fn test_prove_verify_all_builtins_from_file() {
                let prover_input_file_path =
                    get_prover_input_info_path("test_prove_verify_all_builtins");
                let input = prover_input_from_vm_output(&prover_input_file_path)
                    .expect("Failed to create prover input from vm output");
                assert_all_builtins_in_input(&input);
                let preprocessed_trace = PreProcessedTraceVariant::Canonical;
                let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(
                    input,
                    PcsConfig::default(),
                    preprocessed_trace,
                )
                .unwrap();
                verify_cairo::<Blake2sMerkleChannel>(
                    cairo_proof,
                    PcsConfig::default(),
                    preprocessed_trace,
                )
                .unwrap();
            }

            #[test]
            fn test_add_mod_builtin_constraints() {
                let input =
                    prover_input_from_compiled_cairo_program("test_prove_verify_add_mod_builtin");
                assert_cairo_constraints(input, PreProcessedTrace::canonical_without_pedersen());
            }

            #[test]
            fn test_bitwise_builtin_constraints() {
                let input =
                    prover_input_from_compiled_cairo_program("test_prove_verify_bitwise_builtin");
                assert_cairo_constraints(input, testing_preprocessed_tree(19));
            }

            #[test]
            fn test_mul_mod_builtin_constraints() {
                let input =
                    prover_input_from_compiled_cairo_program("test_prove_verify_mul_mod_builtin");
                assert_cairo_constraints(input, testing_preprocessed_tree(19));
            }

            #[test]
            fn test_pedersen_builtin_constraints() {
                let input =
                    prover_input_from_compiled_cairo_program("test_prove_verify_pedersen_builtin");
                assert_cairo_constraints(input, PreProcessedTrace::canonical());
            }

            #[test]
            fn test_poseidon_builtin_constraints() {
                let input =
                    prover_input_from_compiled_cairo_program("test_prove_verify_poseidon_builtin");
                assert_cairo_constraints(input, testing_preprocessed_tree(19));
            }

            #[test]
            fn test_poseidon_builtin_constraints_from_file() {
                let prover_input_file_path =
                    get_prover_input_info_path("test_prove_verify_poseidon_builtin");
                let input = prover_input_from_vm_output(&prover_input_file_path)
                    .expect("Failed to create prover input from vm output");
                assert_cairo_constraints(input, testing_preprocessed_tree(19));
            }

            #[test]
            fn test_range_check_bits_96_builtin_constraints() {
                let input = prover_input_from_compiled_cairo_program(
                    "test_prove_verify_range_check_bits_96_builtin",
                );
                assert_cairo_constraints(input, testing_preprocessed_tree(19));
            }

            #[test]
            fn test_range_check_bits_128_builtin_constraints() {
                let input = prover_input_from_compiled_cairo_program(
                    "test_prove_verify_range_check_bits_128_builtin",
                );
                assert_cairo_constraints(input, testing_preprocessed_tree(19));
            }
        }
    }
}

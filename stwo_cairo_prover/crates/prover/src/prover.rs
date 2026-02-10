use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use cairo_air::air::CairoComponents;
use cairo_air::claims::lookup_sum;
use cairo_air::relations::CommonLookupElements;
use cairo_air::utils::{serialize_proof_to_file, ProofFormat};
use cairo_air::verifier::{verify_cairo, INTERACTION_POW_BITS};
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo::core::channel::{Channel, MerkleChannel};
use stwo::core::fields::qm31::SecureField;
use stwo::core::fri::FriConfig;
use stwo::core::pcs::PcsConfig;
use stwo::core::poly::circle::CanonicCoset;
use stwo::core::proof_of_work::GrindOps;
use stwo::core::vcs_lifted::blake2_merkle::{Blake2sM31MerkleChannel, Blake2sMerkleChannel};
use stwo::core::vcs_lifted::merkle_hasher::MerkleHasherLifted;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::BackendForChannel;
use stwo::prover::poly::circle::PolyOps;
use stwo::prover::{prove_ex, CommitmentSchemeProver, ProvingError};
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_serialize::CairoSerialize;
use tracing::{event, span, Level};

use crate::utils::cairo_provers;
use crate::witness::cairo::create_cairo_claim_generator;
use crate::witness::preprocessed_trace::gen_trace;
use crate::witness::utils::witness_trace_cells;

mod json {
    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    pub use serde_json::from_str;
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    pub use sonic_rs::from_str;
}

pub(crate) const LOG_MAX_ROWS: u32 = 27;

fn prove_verify_serialize<MC: MerkleChannel>(
    input: ProverInput,
    verify: bool,
    proof_path: &Path,
    proof_format: ProofFormat,
    proof_params: ProverParameters,
) -> Result<()>
where
    SimdBackend: BackendForChannel<MC>,
    MC::H: MerkleHasherLifted + Serialize,
    <MC::H as MerkleHasherLifted>::Hash: CairoSerialize,
{
    let cairo_proof = prove_cairo::<MC>(input, proof_params)?;
    if verify {
        verify_cairo::<MC>(cairo_proof.clone().into())?;
    }
    serialize_proof_to_file(&cairo_proof, proof_path, proof_format)?;
    Ok(())
}

pub fn prove_cairo<MC: MerkleChannel>(
    input: ProverInput,
    prover_params: ProverParameters,
) -> Result<CairoProof<MC::H>, ProvingError>
where
    SimdBackend: BackendForChannel<MC>,
{
    let _span = span!(Level::INFO, "prove_cairo").entered();
    let ProverParameters {
        channel_hash: _,
        channel_salt,
        pcs_config,
        preprocessed_trace,
        store_polynomials_coefficients,
    } = prover_params;

    let cairo_air_log_degree_bound = 1;
    let max_domain_size = LOG_MAX_ROWS
        + std::cmp::max(
            cairo_air_log_degree_bound,
            pcs_config.fri_config.log_blowup_factor,
        );
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(max_domain_size)
            .circle_domain()
            .half_coset,
    );

    // Setup protocol.
    let channel = &mut MC::C::default();

    // Mix channel salt. Note that we first reduce it modulo `M31::P`, then cast it as QM31.
    channel.mix_felts(&[channel_salt.into()]);
    pcs_config.mix_into(channel);
    let mut commitment_scheme =
        CommitmentSchemeProver::<SimdBackend, MC>::new(pcs_config, &twiddles);
    if store_polynomials_coefficients {
        commitment_scheme.set_store_polynomials_coefficients();
    }
    // Preprocessed trace.
    let preprocessed_trace = Arc::new(preprocessed_trace.to_preprocessed_trace());
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(gen_trace(preprocessed_trace.clone()));
    tree_builder.commit(channel);

    // Run Cairo.
    let cairo_claim_generator = create_cairo_claim_generator(input, preprocessed_trace.clone());
    // Base trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    let span = span!(Level::INFO, "Base trace").entered();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    span.exit();

    claim.mix_into(channel);
    tree_builder.commit(channel);

    eprintln!("trace_size: {:?}", commitment_scheme.trees.last().unwrap().commitment.layers.len() - 1);

    // Draw interaction elements.
    let interaction_pow = SimdBackend::grind(channel, INTERACTION_POW_BITS);
    channel.mix_u64(interaction_pow);
    let interaction_elements = CommonLookupElements::draw(channel);

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

    let components = cairo_provers(&component_builder);

    // Prove stark.
    let span = span!(Level::INFO, "Prove STARKs").entered();
    let include_all_preprocessed_columns = true;
    let proof = prove_ex::<SimdBackend, _>(&components, channel, commitment_scheme, include_all_preprocessed_columns)?;
    span.exit();

    event!(name: "component_info", Level::DEBUG, "Components: {}", component_builder);

    Ok(CairoProof {
        claim,
        interaction_pow,
        interaction_claim,
        extended_stark_proof: proof,
        channel_salt,
        preprocessed_trace_variant: prover_params.preprocessed_trace,
    })
}

/// Concrete parameters of the proving system.
/// Used both for producing and verifying proofs.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProverParameters {
    /// Channel hash function.
    pub channel_hash: ChannelHash,
    /// Salt for the channel initialization.
    /// Note that the salt is only used to allow recomputation of the proof with other draws
    /// of the randomness, in case of failure due to unprovable draws (e.g. a zero in the
    /// denominator).
    pub channel_salt: u32,
    /// Parameters of the commitment scheme.
    pub pcs_config: PcsConfig,
    /// Preprocessed trace.
    pub preprocessed_trace: PreProcessedTraceVariant,
    /// Whether or not to store the polynomials coefficients. Affects runtime-memory usage
    /// trade-off. Default is `false`.
    pub store_polynomials_coefficients: bool,
}

/// The hash function used for commitments, for the prover-verifier channel,
/// and for PoW grinding.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelHash {
    /// Default variant, the fastest option.
    Blake2s,
    /// A variant for Blake2s where modulo M31 is applied to every 32bits in the output.
    Blake2sM31,
    /// A variant for recursive proof verification.
    /// Note that using `Poseidon252` results in a significant decrease in proving speed compared
    /// to `Blake2s` (because of the large field emulation)
    Poseidon252,
}

/// Generates proof given the Cairo VM output and prover config/parameters.
/// Serializes the proof as JSON and write to the output path.
/// Verifies the proof in case the respective flag is set.
pub fn create_and_serialize_proof(
    input: ProverInput,
    verify: bool,
    proof_path: PathBuf,
    proof_format: ProofFormat,
    proof_params_json: Option<PathBuf>,
) -> Result<()> {
    let proof_params = if let Some(proof_params_json) = proof_params_json {
        json::from_str(&read_to_string(&proof_params_json)?)?
    } else {
        // The default prover parameters for prod use (96 bits of security).
        // The formula is `security_bits = pow_bits + log_blowup_factor * n_queries`.
        ProverParameters {
            channel_hash: ChannelHash::Blake2s,
            channel_salt: 0,
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
                lifting_log_size: None,
            },
            preprocessed_trace: PreProcessedTraceVariant::Canonical,
            store_polynomials_coefficients: false,
        }
    };

    match proof_params.channel_hash {
        ChannelHash::Blake2s => {
            prove_verify_serialize::<Blake2sMerkleChannel>(
                input,
                verify,
                &proof_path,
                proof_format,
                proof_params,
            )?;
        }
        ChannelHash::Blake2sM31 => {
            prove_verify_serialize::<Blake2sM31MerkleChannel>(
                input,
                verify,
                &proof_path,
                proof_format,
                proof_params,
            )?;
        }
        #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
        ChannelHash::Poseidon252 => {
            unimplemented!("Poseidon252 is not supported for wasm targets");
        }
        #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
        ChannelHash::Poseidon252 => {
            use stwo::core::vcs_lifted::poseidon252_merkle::Poseidon252MerkleChannel;
            prove_verify_serialize::<Poseidon252MerkleChannel>(
                input,
                verify,
                &proof_path,
                proof_format,
                proof_params,
            )?;
        }
    };

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use stwo_cairo_common::preprocessed_columns::preprocessed_trace::{
        testing_preprocessed_tree, PreProcessedTrace,
    };
    use stwo_cairo_dev_utils::utils::get_compiled_cairo_program_path;
    use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};

    use crate::debug_tools::assert_constraints::assert_cairo_constraints;

    #[test]
    fn test_all_cairo_constraints() {
        let compiled_program =
            get_compiled_cairo_program_path("test_prove_verify_all_opcode_components");
        let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
        let pp_tree = Arc::new(testing_preprocessed_tree(24));
        assert_cairo_constraints(input, pp_tree);
    }

    #[test]
    fn test_all_cairo_constraints_small_ppt() {
        let compiled_program =
            get_compiled_cairo_program_path("test_prove_verify_all_opcode_components");
        let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
        let pp_tree = Arc::new(PreProcessedTrace::canonical_small());
        assert_cairo_constraints(input, pp_tree);
    }

    #[cfg(test)]
    #[cfg(feature = "nightly")]
    mod nightly_tests {
        use std::io::Write;
        use std::process::Command;

        use cairo_air::PreProcessedTraceVariant;
        use stwo::core::fri::FriConfig;
        use stwo::core::pcs::PcsConfig;
        use stwo::core::vcs_lifted::poseidon252_merkle::Poseidon252MerkleChannel;
        use stwo_cairo_dev_utils::utils::get_proof_file_path;
        use stwo_cairo_serialize::CairoSerialize;
        use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};
        use tempfile::NamedTempFile;
        use test_log::test;

        use super::*;
        use crate::prover::{prove_cairo, ChannelHash, ProverParameters};

        #[test]
        fn test_poseidon_e2e_prove_cairo_verify_ret_opcode_components() {
            let compiled_program = get_compiled_cairo_program_path("test_prove_verify_ret_opcode");
            let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
            let prover_params = ProverParameters {
                channel_hash: ChannelHash::Poseidon252,
                pcs_config: PcsConfig {
                    pow_bits: 20,
                    fri_config: FriConfig::new(0, 1, 90),
                    lifting_log_size: None,
                },
                preprocessed_trace: PreProcessedTraceVariant::CanonicalWithoutPedersen,
                channel_salt: 42,
                store_polynomials_coefficients: false,
            };
            let cairo_proof =
                prove_cairo::<Poseidon252MerkleChannel>(input, prover_params).unwrap();
            let mut proof_file = NamedTempFile::new().unwrap();
            let mut serialized: Vec<starknet_ff::FieldElement> = Vec::new();
            CairoSerialize::serialize(&cairo_proof, &mut serialized);
            let proof_hex: Vec<String> = serialized
                .into_iter()
                .map(|felt| format!("0x{felt:x}"))
                .collect();
            proof_file
                .write_all(sonic_rs::to_string_pretty(&proof_hex).unwrap().as_bytes())
                .unwrap();
            let expected_proof_file = get_proof_file_path("test_prove_verify_ret_opcode");

            if std::env::var("FIX_PROOF").is_ok() {
                std::fs::copy(proof_file.path(), &expected_proof_file)
                    .expect("Failed to overwrite expected proof file");
            }

            // Compare the contents of proof_file and expected_proof_file
            let proof_file_contents = std::fs::read_to_string(proof_file.path())
                .expect("Failed to read generated proof file");
            let expected_proof_contents = std::fs::read_to_string(&expected_proof_file)
                .expect("Failed to read expected proof file");
            assert!(
                proof_file_contents == expected_proof_contents,
                "Generated proof file does not match the expected proof file"
            );

            let status = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "(cd ../../../stwo_cairo_verifier; \
                    scarb execute --package stwo_cairo_verifier \
                    --arguments-file {} --output standard --target standalone \
                    --features poseidon252_verifier
                    )",
                    proof_file.path().to_str().unwrap()
                ))
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .status()
                .unwrap();

            assert!(status.success());
        }
    }

    #[cfg(test)]
    pub mod slow_tests {
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::process::Command;

        use cairo_air::utils::binary_serialize_to_file;
        use cairo_air::verifier::verify_cairo;
        use cairo_air::CairoProofForRustVerifier;
        use itertools::Itertools;
        use stwo::core::fri::FriConfig;
        use stwo::core::pcs::PcsConfig;
        use stwo::core::vcs_lifted::blake2_merkle::{
            Blake2sM31MerkleChannel, Blake2sMerkleChannel,
        };
        use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
        use stwo_cairo_dev_utils::utils::{get_compiled_cairo_program_path, get_proof_file_path};
        use stwo_cairo_serialize::CairoSerialize;
        use tempfile::NamedTempFile;
        use test_log::test;

        use super::*;
        use crate::debug_tools::assert_constraints::assert_cairo_constraints;
        use crate::prover::{
            prove_cairo, ChannelHash, PreProcessedTraceVariant, ProverInput, ProverParameters,
        };

        // TODO(Ohad): fine-grained constraints tests.
        #[test]
        fn test_cairo_constraints() {
            let compiled_program =
                get_compiled_cairo_program_path("test_prove_verify_all_opcode_components");
            let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
            assert_cairo_constraints(
                input,
                Arc::new(PreProcessedTrace::canonical_without_pedersen()),
            );
        }

        #[test_log::test]
        fn test_prove_verify_all_opcode_components() {
            let compiled_program =
                get_compiled_cairo_program_path("test_prove_verify_all_opcode_components");
            let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
            for (opcode, n_instances) in &input.state_transitions.casm_states_by_opcode.counts() {
                assert!(
                    *n_instances > 0,
                    "{opcode} isn't used in E2E full-Cairo opcode test"
                );
            }
            let prover_params = ProverParameters {
                channel_hash: ChannelHash::Blake2s,
                pcs_config: PcsConfig::default(),
                preprocessed_trace: PreProcessedTraceVariant::CanonicalWithoutPedersen,
                channel_salt: 0,
                store_polynomials_coefficients: true,
            };
            let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(input, prover_params).unwrap();
            verify_cairo::<Blake2sMerkleChannel>(cairo_proof.into()).unwrap();
        }

        #[test]
        fn test_e2e_prove_cairo_verify_all_opcode_components() {
            let compiled_program =
                get_compiled_cairo_program_path("test_prove_verify_all_opcode_components");
            let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
            let prover_params = ProverParameters {
                channel_hash: ChannelHash::Blake2s,
                pcs_config: PcsConfig {
                    pow_bits: 26,
                    fri_config: FriConfig::new(0, 1, 70),
                    lifting_log_size: None,
                },
                preprocessed_trace: PreProcessedTraceVariant::Canonical,
                channel_salt: 0,
                store_polynomials_coefficients: false,
            };
            let cairo_proof = prove_cairo::<Blake2sMerkleChannel>(input, prover_params).unwrap();
            let mut proof_file = NamedTempFile::new().unwrap();
            let mut serialized: Vec<starknet_ff::FieldElement> = Vec::new();
            CairoSerialize::serialize(&cairo_proof, &mut serialized);
            let proof_hex: Vec<String> = serialized
                .into_iter()
                .map(|felt| format!("0x{felt:x}"))
                .collect();
            proof_file
                .write_all(sonic_rs::to_string_pretty(&proof_hex).unwrap().as_bytes())
                .unwrap();

            let expected_proof_file =
                get_proof_file_path("test_prove_verify_all_opcode_components");
            if std::env::var("FIX_PROOF").is_ok() {
                std::fs::copy(proof_file.path(), &expected_proof_file)
                    .expect("Failed to overwrite expected proof file");
            }

            // Compare the contents of proof_file and expected_proof_file
            let proof_file_contents = std::fs::read_to_string(proof_file.path())
                .expect("Failed to read generated proof file");
            let expected_proof_contents = std::fs::read_to_string(&expected_proof_file)
                .expect("Failed to read expected proof file");
            assert!(
                proof_file_contents == expected_proof_contents,
                "Generated proof file does not match the expected proof file"
            );

            let status = Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "(cd ../../../stwo_cairo_verifier; \
                    scarb execute --package stwo_cairo_verifier \
                    --arguments-file {} --output standard --target standalone \
                    --features qm31_opcode
                    )",
                    proof_file.path().to_str().unwrap()
                ))
                .current_dir(env!("CARGO_MANIFEST_DIR"))
                .status()
                .unwrap();

            assert!(status.success());
        }

        #[test]
        fn test_e2e_prove_cairo_verify_all_builtins() {
            let compiled_program =
                get_compiled_cairo_program_path("test_prove_verify_all_builtins");
            let proof_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open("all_builtins.bin")
                .unwrap();
            let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
            let prover_params = ProverParameters {
                channel_hash: ChannelHash::Blake2s,
                pcs_config: PcsConfig {
                    pow_bits: 26,
                    fri_config: FriConfig::new(0, 1, 70),
                    lifting_log_size: Some(26),
                },
                preprocessed_trace: PreProcessedTraceVariant::Canonical,
                channel_salt: 0,
                store_polynomials_coefficients: false,
            };
            let cairo_proof = prove_cairo::<Blake2sM31MerkleChannel>(input, prover_params).unwrap();

            binary_serialize_to_file(&cairo_proof, &proof_file).unwrap();
        }

        fn test_proof_stability(path: &str, n_proofs_to_compare: usize) {
            let compiled_program = get_compiled_cairo_program_path(path);
            let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
            let prover_params = ProverParameters {
                channel_hash: ChannelHash::Blake2s,
                pcs_config: PcsConfig::default(),
                preprocessed_trace: PreProcessedTraceVariant::Canonical,
                channel_salt: 0,
                store_polynomials_coefficients: false,
            };
            let proofs = (0..n_proofs_to_compare)
                .map(|_| {
                    let proof: CairoProofForRustVerifier<_> =
                        prove_cairo::<Blake2sMerkleChannel>(input.clone(), prover_params)
                            .unwrap()
                            .into();
                    sonic_rs::to_string(&proof).unwrap()
                })
                .collect_vec();

            assert!(proofs.iter().all_equal());
        }

        #[test]
        fn test_opcodes_proof_stability() {
            test_proof_stability("test_prove_verify_all_opcode_components", 2);
        }

        #[test]
        fn test_builtins_proof_stability() {
            test_proof_stability("test_prove_verify_all_builtins", 2);
        }

        /// These tests' inputs were generated using cairo-vm with 50 instances of each builtin.
        pub mod builtin_tests {
            use stwo::core::pcs::PcsConfig;
            use stwo_cairo_common::preprocessed_columns::preprocessed_trace::testing_preprocessed_tree;
            use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};
            use test_log::test;

            use super::*;

            /// Asserts that all supported builtins are present in the input.
            /// Panics if any of the builtins is missing.
            fn assert_all_builtins_in_input(input: &ProverInput) {
                let empty_builtins: Vec<_> = input
                    .builtin_segments
                    .get_counts()
                    .into_iter()
                    .filter(|(_, count)| *count == 0)
                    .map(|(name, _)| name)
                    .collect();

                if !empty_builtins.is_empty() {
                    panic!("Builtins missing in the input: {empty_builtins:?}");
                }
            }

            #[test]
            fn test_prove_verify_all_builtins() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_all_builtins");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_all_builtins_in_input(&input);
                let prover_params = ProverParameters {
                    channel_hash: ChannelHash::Blake2s,
                    pcs_config: PcsConfig::default(),
                    preprocessed_trace: PreProcessedTraceVariant::Canonical,
                    channel_salt: 0,
                    store_polynomials_coefficients: false,
                };
                let cairo_proof =
                    prove_cairo::<Blake2sMerkleChannel>(input, prover_params).unwrap();
                verify_cairo::<Blake2sMerkleChannel>(cairo_proof.into()).unwrap();
            }

            #[test]
            fn test_prove_verify_all_builtins_canonical_small() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_all_builtins");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_all_builtins_in_input(&input);
                let prover_params = ProverParameters {
                    channel_hash: ChannelHash::Blake2s,
                    pcs_config: PcsConfig::default(),
                    preprocessed_trace: PreProcessedTraceVariant::CanonicalSmall,
                    channel_salt: 0,
                    store_polynomials_coefficients: false,
                };
                let cairo_proof =
                    prove_cairo::<Blake2sMerkleChannel>(input, prover_params).unwrap();
                verify_cairo::<Blake2sMerkleChannel>(cairo_proof.into()).unwrap();
            }

            #[test]
            fn test_add_mod_builtin_constraints() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_add_mod_builtin");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(
                    input,
                    Arc::new(PreProcessedTrace::canonical_without_pedersen()),
                );
            }

            #[test]
            fn test_bitwise_builtin_constraints() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_bitwise_builtin");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(testing_preprocessed_tree(20)));
            }

            #[test]
            fn test_mul_mod_builtin_constraints() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_mul_mod_builtin");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(testing_preprocessed_tree(20)));
            }

            #[test]
            fn test_pedersen_builtin_constraints() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_pedersen_builtin");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(PreProcessedTrace::canonical()));
            }

            #[test]
            fn test_pedersen_narrow_windows_builtin_constraints() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_pedersen_builtin");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(PreProcessedTrace::canonical_small()));
            }

            #[test]
            fn test_poseidon_builtin_constraints() {
                let compiled_program =
                    get_compiled_cairo_program_path("test_prove_verify_poseidon_builtin");
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(testing_preprocessed_tree(20)));
            }

            #[test]
            fn test_range_check_bits_96_builtin_constraints() {
                let compiled_program = get_compiled_cairo_program_path(
                    "test_prove_verify_range_check_bits_96_builtin",
                );
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(testing_preprocessed_tree(20)));
            }

            #[test]
            fn test_range_check_bits_128_builtin_constraints() {
                let compiled_program = get_compiled_cairo_program_path(
                    "test_prove_verify_range_check_bits_128_builtin",
                );
                let input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
                assert_cairo_constraints(input, Arc::new(testing_preprocessed_tree(20)));
            }

            #[test]
            fn test_poseidon_aggregator() {
                let prover_params = ProverParameters {
                    channel_hash: ChannelHash::Blake2s,
                    pcs_config: PcsConfig::default(),
                    preprocessed_trace: PreProcessedTraceVariant::Canonical,
                    channel_salt: 0,
                    store_polynomials_coefficients: false,
                };

                // Run poseidon builtin with 15 different instances.
                let compiled_program_a =
                    get_compiled_cairo_program_path("test_prove_verify_poseidon_builtin");
                let input_a = run_and_adapt(&compiled_program_a, ProgramType::Json, None).unwrap();
                let proof_a = prove_cairo::<Blake2sMerkleChannel>(input_a, prover_params).unwrap();
                let poseidon_builtin_size_a = 2u32.pow(
                    proof_a
                        .claim
                        .poseidon_builtin
                        .expect("Poseidon builtin is not present in the claim")
                        .log_size,
                );
                assert!(poseidon_builtin_size_a == 16, "Expected program to contain 15 poseidon instances, which then padded to the next power of two");

                let poseidon_aggregator_log_size_a = proof_a
                    .claim
                    .poseidon_aggregator
                    .expect("Poseidon context is not present in the claim")
                    .log_size;

                // Run poseidon builtin with 15 different instances, each one 30 times.
                let compiled_program_b =
                    get_compiled_cairo_program_path("test_poseidon_aggregator");
                let input_b = run_and_adapt(&compiled_program_b, ProgramType::Json, None).unwrap();
                let proof_b = prove_cairo::<Blake2sMerkleChannel>(input_b, prover_params).unwrap();
                let poseidon_builtin_size_b = 2u32.pow(
                    proof_b
                        .claim
                        .poseidon_builtin
                        .expect("Poseidon builtin is not present in the claim")
                        .log_size,
                );
                assert!(poseidon_builtin_size_b == 512, "Expected program to contain 15*30 poseidon instances, which then padded to the next power of two");

                let poseidon_aggregator_log_size_b = proof_b
                    .claim
                    .poseidon_aggregator
                    .expect("Poseidon context is not present in the claim")
                    .log_size;

                assert_eq!(
                    poseidon_aggregator_log_size_a,
                    poseidon_aggregator_log_size_b,
                    "Poseidon aggregator log size should be the same for both proof because it uses multiplicity"
                );
            }

            #[test]
            fn test_pedersen_aggregator() {
                let prover_params = ProverParameters {
                    channel_hash: ChannelHash::Blake2s,
                    pcs_config: PcsConfig::default(),
                    preprocessed_trace: PreProcessedTraceVariant::Canonical,
                    channel_salt: 0,
                    store_polynomials_coefficients: false,
                };

                // Run pedersen builtin with 15 different instances.
                let compiled_program_a =
                    get_compiled_cairo_program_path("test_prove_verify_pedersen_builtin");
                let input_a = run_and_adapt(&compiled_program_a, ProgramType::Json, None).unwrap();
                let proof_a = prove_cairo::<Blake2sMerkleChannel>(input_a, prover_params).unwrap();
                let pedersen_builtin_size_a = 2u32.pow(
                    proof_a
                        .claim
                        .pedersen_builtin
                        .expect("Pedersen builtin is not present in the claim")
                        .log_size,
                );
                assert!(pedersen_builtin_size_a == 16, "Expected program to contain 15 pedersen instances, which then padded to the next power of two");

                let pedersen_aggregator_log_size_a = proof_a
                    .claim
                    .pedersen_aggregator_window_bits_18
                    .expect("Pedersen context is not present in the claim")
                    .log_size;

                // Run pedersen builtin with 15 different instances, each one 30 times.
                let compiled_program_b =
                    get_compiled_cairo_program_path("test_pedersen_aggregator");
                let input_b = run_and_adapt(&compiled_program_b, ProgramType::Json, None).unwrap();
                let proof_b = prove_cairo::<Blake2sMerkleChannel>(input_b, prover_params).unwrap();
                let pedersen_builtin_size_b = 2u32.pow(
                    proof_b
                        .claim
                        .pedersen_builtin
                        .expect("Pedersen builtin is not present in the claim")
                        .log_size,
                );
                assert!(pedersen_builtin_size_b == 512, "Expected program to contain 15*30 pedersen instances, which then padded to the next power of two");

                let pedersen_aggregator_log_size_b = proof_b
                    .claim
                    .pedersen_aggregator_window_bits_18
                    .expect("Pedersen context is not present in the claim")
                    .log_size;

                assert_eq!(
                    pedersen_aggregator_log_size_a,
                    pedersen_aggregator_log_size_b,
                    "Pedersen aggregator log size should be the same for both proof because it uses multiplicity"
                );
            }
        }
    }
}

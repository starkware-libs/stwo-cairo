use num_traits::Zero;
use serde::{Deserialize, Serialize};
use stwo_cairo_adapter::ProverInput;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::BackendForChannel;
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fri::FriConfig;
use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, ProvingError};
use tracing::{span, Level};

use crate::cairo_air::air::{
    lookup_sum, CairoClaimGenerator, CairoComponents, CairoInteractionElements,
};

pub(crate) const LOG_MAX_ROWS: u32 = 22;

use super::CairoProof;
use crate::cairo_air::preprocessed::PreProcessedTrace;

pub struct CairoProver {
    parameters: ProverParameters,
}
impl CairoProver {
    pub fn new(parameters: ProverParameters) -> Self {
        Self { parameters }
    }

    pub fn prove<MC: MerkleChannel>(
        &self,
        input: ProverInput,
    ) -> Result<(CairoProof<MC::H>, String), ProvingError>
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let ProverParameters { pcs_config } = self.parameters;
        let _span = span!(Level::INFO, "test_prove").entered();
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

        #[cfg(feature = "relation-tracker")]
        {
            use stwo_prover::constraint_framework::relation_tracker::RelationSummary;

            use crate::cairo_air::debug_tools::relation_tracker::track_cairo_relations;
            tracing::info!(
                "Relations summary: {:?}",
                RelationSummary::summarize_relations(&track_cairo_relations(
                    &commitment_scheme,
                    &claim,
                ))
                .cleaned()
            );
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
        let component_builder =
            CairoComponents::new(&claim, &interaction_elements, &interaction_claim);

        // TODO(Ohad): decide an API for execution resources.
        let execution_resources = format!("{component_builder}");
        let components = component_builder.provers();

        // Prove stark.
        let proof = prove::<SimdBackend, _>(&components, channel, commitment_scheme)?;

        Ok((
            CairoProof {
                claim,
                interaction_claim,
                stark_proof: proof,
            },
            execution_resources,
        ))
    }
}

/// Concrete parameters of the proving system.
/// Used both for producing and verifying proofs.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProverParameters {
    /// Parameters of the commitment scheme.
    pub pcs_config: PcsConfig,
    // TODO(m-kus): add channel hash type here
}

/// The default prover parameters for prod use (96 bits of security).
/// The formula is `security_bits = pow_bits + log_blowup_factor * n_queries`.
pub fn default_prod_prover_parameters() -> ProverParameters {
    ProverParameters {
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
    }
}

#[cfg(test)]
pub mod tests {
    use cairo_lang_casm::casm;
    use stwo_cairo_adapter::plain::input_from_plain_casm;
    use stwo_cairo_adapter::ProverInput;
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::backend::BackendForChannel;
    use stwo_prover::core::channel::MerkleChannel;
    use stwo_prover::core::prover::ProvingError;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

    use super::{default_prod_prover_parameters, CairoProver};
    use crate::cairo_air::verifier::{verify_cairo, CairoVerificationError};
    use crate::cairo_air::CairoProof;

    fn test_basic_cairo_air_input() -> ProverInput {
        let u128_max = u128::MAX;
        let instructions = casm! {
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

    fn test_prove<MC: MerkleChannel>(input: ProverInput) -> Result<CairoProof<MC::H>, ProvingError>
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let prover = CairoProver::new(default_prod_prover_parameters());

        let (cairo_proof, _component_info) = prover.prove::<MC>(input)?;

        Ok(cairo_proof)
    }

    fn test_verify<MC: MerkleChannel>(
        proof: CairoProof<MC::H>,
    ) -> Result<(), CairoVerificationError>
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let pcs_config = default_prod_prover_parameters().pcs_config;
        verify_cairo::<MC>(proof, pcs_config)
    }

    #[test]
    fn test_basic_cairo_air() {
        let input = test_basic_cairo_air_input();
        let proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
        test_verify::<Blake2sMerkleChannel>(proof).unwrap();
    }

    #[cfg(test)]
    #[cfg(feature = "nightly")]
    mod nightly_tests {
        use std::io::Write;

        use itertools::Itertools;
        use stwo_cairo_serialize::CairoSerialize;
        use stwo_prover::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;

        use super::*;

        #[test]
        fn generate_and_serialise_proof() {
            let input = test_basic_cairo_air_input();
            let cairo_proof = test_prove::<Poseidon252MerkleChannel>(input).unwrap();

            let mut output = Vec::new();
            CairoSerialize::serialize(&cairo_proof, &mut output);
            let proof_str = output.iter().map(|v| v.to_string()).join(",");
            let mut file = std::fs::File::create("proof.cairo").unwrap();
            file.write_all(proof_str.as_bytes()).unwrap();

            test_verify::<Poseidon252MerkleChannel>(cairo_proof).unwrap();
        }
    }

    #[cfg(test)]
    #[cfg(feature = "slow-tests")]
    pub mod slow_tests {
        use itertools::Itertools;
        use stwo_cairo_adapter::vm_import::generate_test_input;
        use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

        use super::*;

        #[test]
        fn test_full_cairo_air() {
            let cairo_proof = test_prove::<Blake2sMerkleChannel>(generate_test_input(
                "test_read_from_small_files",
            ))
            .unwrap();
            test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
        }

        #[test]
        fn test_prove_verify_all_opcode_components() {
            let input = generate_test_input("test_prove_verify_all_opcode_components_v2");
            for (opcode, n_instances) in input.state_transitions.casm_states_by_opcode.counts() {
                // TODO(Stav): Remove when `Blake` opcode is in the VM.
                if opcode == "blake2s_opcode" {
                    continue;
                }

                assert!(
                    n_instances > 0,
                    "{} isn't used in E2E full-Cairo opcode test",
                    opcode
                );
            }
            let cairo_proof = test_prove::<Blake2sMerkleChannel>(generate_test_input(
                "test_prove_verify_all_opcode_components_v2",
            ))
            .unwrap();
            test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
        }

        // TODO(Ohad): remove ignore.
        #[ignore = "POW is not deterministic"]
        #[test]
        fn test_proof_stability() {
            let n_proofs_to_compare = 10;
            let prover = CairoProver::new(default_prod_prover_parameters());

            let proofs = (0..n_proofs_to_compare)
                .map(|_| {
                    let input = test_basic_cairo_air_input();
                    let proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                    serde_json::to_string(&proof).unwrap()
                })
                .collect_vec();

            assert!(proofs.iter().all_equal());
        }

        /// These tests' inputs were generated using cairo-vm with 50 instances of each builtin.
        pub mod builtin_tests {
            use cairo_vm::air_public_input::MemorySegmentAddresses;
            use stwo_cairo_adapter::vm_import::{generate_test_input, MemoryEntryIter};
            use stwo_cairo_utils::file_utils::open_file;

            use super::*;

            /// Asserts that all builtins are present in the input.
            /// Panics if any of the builtins is missing.
            fn assert_all_builtins_in_input(input: &ProverInput) {
                let empty_builtins = input
                    .builtins_segments
                    .get_counts()
                    .iter()
                    .filter(|(_, &count)| count == 0)
                    .map(|(name, _)| format!("{:?}", name))
                    .collect_vec();
                assert!(
                    empty_builtins.is_empty(),
                    "The following builtins are missing: {}",
                    empty_builtins.join(", ")
                );
            }

            #[test]
            fn test_prove_verify_all_builtins() {
                let input = generate_test_input("test_prove_verify_all_builtins");
                assert_all_builtins_in_input(&input);
                let cairo_proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
            }

            #[test]
            fn test_prove_verify_add_mod_builtin() {
                let input = generate_test_input("test_prove_verify_add_mod_builtin");
                let cairo_proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
            }

            /// Asserts that there is an unused `add` value in the first instance in bitwise
            /// builtin segment, inducing a "hole".
            fn assert_bitwise_builtin_has_holes(
                test_name: &str,
                bitwise_segment: &Option<MemorySegmentAddresses>,
            ) {
                let bitwise_segment = bitwise_segment.as_ref().unwrap();
                let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                d.push("../../test_data/");
                d.push(test_name);
                let mut memory_file =
                    std::io::BufReader::new(open_file(d.join("mem").as_path()).unwrap());
                let memory_entries = MemoryEntryIter(&mut memory_file).collect_vec();
                assert!(memory_entries
                    .iter()
                    .all(|entry| entry.address != (bitwise_segment.begin_addr + 2) as u64));
            }

            #[test]
            fn test_prove_verify_bitwise_builtin() {
                let input = generate_test_input("test_prove_verify_bitwise_builtin");
                assert_bitwise_builtin_has_holes(
                    "test_prove_verify_bitwise_builtin",
                    &input.builtins_segments.bitwise,
                );
                let cairo_proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
            }

            #[test]
            fn test_prove_verify_mul_mod_builtin() {
                let input = generate_test_input("test_prove_verify_mul_mod_builtin");
                let cairo_proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
            }

            #[test]
            fn test_prove_verify_range_check_bits_96_builtin() {
                let input = generate_test_input("test_prove_verify_range_check_bits_96_builtin");
                let cairo_proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
            }

            #[test]
            fn test_prove_verify_range_check_bits_128_builtin() {
                let input = generate_test_input("test_prove_verify_range_check_bits_128_builtin");
                let cairo_proof = test_prove::<Blake2sMerkleChannel>(input).unwrap();
                test_verify::<Blake2sMerkleChannel>(cairo_proof).unwrap();
            }
        }
    }
}

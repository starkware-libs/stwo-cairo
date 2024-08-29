mod air;

use air::{
    CairoClaim, CairoComponents, CairoInteractionClaim, CairoInteractionElements, CairoProvers,
};
use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Blake2sChannel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::{
    CommitmentSchemeProver, CommitmentSchemeVerifier, PcsConfig, TreeVec,
};
use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
use stwo_prover::core::prover::{prove, verify, StarkProof, VerificationError};
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use stwo_prover::core::vcs::ops::MerkleHasher;
use stwo_prover::core::InteractionElements;
use thiserror::Error;
use tracing::{span, Level};

use crate::input::CairoInput;

pub struct CairoProof<H: MerkleHasher> {
    pub claim: CairoClaim,
    pub interaction_claim: CairoInteractionClaim,
    pub stark_proof: StarkProof<H>,
}

pub fn lookup_sum_valid(
    claim: &CairoClaim,
    elements: &CairoInteractionElements,
    interaction_claim: &CairoInteractionClaim,
) -> bool {
    let mut sum = interaction_claim.logup_sum();
    // Public memory.
    // TODO(spapini): Optimized inverse.
    // sum += claim
    //     .public_memory
    //     .iter()
    //     .map(|(addr, val)| {
    //         elements
    //             .opcode.memory_elements.addr_to_id
    //             .combine::<M31, QM31>(
    //                 &[
    //                     [M31::from_u32_unchecked(*addr)].as_slice(),
    //                     split_f252(*val).as_slice(),
    //                 ]
    //                 .concat(),
    //             )
    //             .inverse()
    //     })
    //     .sum::<SecureField>();
    // TODO: include initial and final state.

    // Extra transitions.
    // TODO: Check that the extra transitions are valid.
    let extra_transitions = chain![
        claim.opcodes.extra_transitions.iter().copied(),
        [(1, [claim.initial_state, claim.final_state])].into_iter()
    ];
    let frac: Fraction<QM31, QM31> = extra_transitions
        .map(|(n, transition)| {
            elements.opcode.state.combine_frac(
                M31::from(n),
                &[
                    M31::from(transition[0].pc),
                    M31::from(transition[0].ap),
                    M31::from(transition[0].fp),
                ],
            ) + elements.opcode.state.combine_frac(
                -M31::from(n),
                &[
                    M31::from(transition[1].pc),
                    M31::from(transition[1].ap),
                    M31::from(transition[1].fp),
                ],
            )
        })
        .reduce(|a, b| a + b)
        .unwrap();

    sum += frac.numerator / frac.denominator;
    sum == SecureField::zero()
}

pub const LOG_MAX_ROWS: u32 = 20;
pub fn prove_cairo(config: PcsConfig, input: CairoInput) -> CairoProof<Blake2sMerkleHasher> {
    let _span = span!(Level::INFO, "Proof").entered();
    let span = span!(Level::INFO, "Twiddles").entered();
    let twiddles = SimdBackend::precompute_twiddles(
        CanonicCoset::new(LOG_MAX_ROWS + config.fri_config.log_blowup_factor + 2)
            .circle_domain()
            .half_coset,
    );
    span.exit();

    // Setup protocol.
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme = &mut CommitmentSchemeProver::new(config, &twiddles);

    let cairo_provers = CairoProvers::new(input);

    // Base trace.
    let span = span!(Level::INFO, "Trace gen").entered();
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, cairo_provers) = cairo_provers.generate(&mut tree_builder);

    claim.mix_into(channel);
    span.exit();
    tree_builder.commit(channel);

    // Draw interaction elements.
    let interaction_elements = CairoInteractionElements::draw(channel);

    // Interaction trace.
    let span = span!(Level::INFO, "Interaction trace gen").entered();
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim = cairo_provers.generate(&interaction_elements, &mut tree_builder);
    interaction_claim.mix_into(channel);
    span.exit();
    tree_builder.commit(channel);

    // Component provers.
    let component_builder = CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_builder.provers();

    // TODO: Remove. Only for debugging.
    if false {
        let _span = span!(Level::INFO, "DEBUG: Assert constraints").entered();
        assert!(
            lookup_sum_valid(&claim, &interaction_elements, &interaction_claim),
            "Lookups are invalid"
        );

        let mut trace_polys = commitment_scheme
            .trees
            .as_ref()
            .map(|t| t.polynomials.iter().cloned());
        component_builder.assert_constraints(&mut trace_polys);
    }

    // Assert sizes.
    let actual_lens = commitment_scheme.polynomials().map_cols(|x| x.log_size());
    let expected_lens = claim.log_sizes();
    assert_eq!(actual_lens.0, expected_lens.0);

    let component_lens = TreeVec::concat_cols(
        component_builder
            .provers()
            .map(|c| c.trace_log_degree_bounds()),
    );
    assert_eq!(actual_lens.0, component_lens.0);

    // Compute total trace size.
    let total_size: u64 = commitment_scheme
        .polynomials()
        .iter()
        .flat_map(|x| x.iter().map(|y| (1 << y.log_size()) as u64))
        .sum();
    println!("Total trace size: {}", total_size);

    // Prove stark.
    let proof = prove::<SimdBackend, _>(
        &components.collect_vec(),
        channel,
        &InteractionElements::default(),
        commitment_scheme,
    )
    .unwrap();

    CairoProof {
        claim,
        interaction_claim,
        stark_proof: proof,
    }
}

pub fn verify_cairo(
    config: PcsConfig,
    CairoProof {
        claim,
        interaction_claim,
        stark_proof,
    }: CairoProof<Blake2sMerkleHasher>,
) -> Result<(), CairoVerificationError> {
    // Verify.
    let channel = &mut Blake2sChannel::default();
    let commitment_scheme_verifier =
        &mut CommitmentSchemeVerifier::<Blake2sMerkleChannel>::new(config);

    claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[0], &claim.log_sizes()[0], channel);
    let interaction_elements = CairoInteractionElements::draw(channel);
    if !lookup_sum_valid(&claim, &interaction_elements, &interaction_claim) {
        return Err(CairoVerificationError::InvalidLogupSum);
    }
    interaction_claim.mix_into(channel);
    commitment_scheme_verifier.commit(stark_proof.commitments[1], &claim.log_sizes()[1], channel);

    let component_generator =
        CairoComponents::new(&claim, &interaction_elements, &interaction_claim);
    let components = component_generator.components();

    verify(
        &components.collect_vec(),
        channel,
        &InteractionElements::default(),
        commitment_scheme_verifier,
        stark_proof,
    )
    .map_err(CairoVerificationError::Stark)
}

#[derive(Error, Debug)]
pub enum CairoVerificationError {
    #[error("Invalid logup sum")]
    InvalidLogupSum,
    #[error("Stark verification error: {0}")]
    Stark(#[from] VerificationError),
}

#[cfg(test)]
mod tests {
    use cairo_lang_casm::casm;
    use stwo_prover::core::pcs::PcsConfig;

    use crate::cairo_air::{prove_cairo, verify_cairo, CairoInput};
    use crate::input::plain::input_from_plain_casm;

    fn big_input() -> CairoInput {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/sample0");

        crate::input::vm_import::import_from_vm_output(
            d.join("pub.json").as_path(),
            d.join("priv.json").as_path(),
        )
        .expect(
            "
            Failed to read test files. Maybe git-lfs is not installed? Checkout README.md.",
        )
    }
    fn test_input() -> CairoInput {
        let instructions = casm! {
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

    #[test_log::test]
    fn test_cairo_air_simple() {
        let config = PcsConfig::default();
        let cairo_proof = prove_cairo(config, test_input());

        verify_cairo(config, cairo_proof).unwrap();
    }

    #[ignore]
    #[test_log::test]
    fn test_cairo_air_big() {
        let config = PcsConfig::default();
        let cairo_proof = prove_cairo(config, big_input());

        verify_cairo(config, cairo_proof).unwrap();
    }
}

use num_traits::{One, Zero};
use stwo_prover::core::backend::simd::m31::PackedM31;

use super::memory::component::N_M31_IN_FELT252;

pub mod component;
pub mod simd_trace;

pub fn packed_felt_252_one() -> [PackedM31; N_M31_IN_FELT252] {
    let mut value = [PackedM31::zero(); N_M31_IN_FELT252];
    value[0] = PackedM31::one();
    value
}

#[cfg(test)]
pub mod tests {
    use itertools::Itertools;
    use stwo_prover::constraint_framework::constant_columns::gen_is_first;
    use stwo_prover::constraint_framework::logup::LookupElements;
    use stwo_prover::core::air::{Air, AirProver};
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::channel::{Blake2sChannel, Channel};
    use stwo_prover::core::fields::m31::BaseField;
    use stwo_prover::core::fields::IntoSlice;
    use stwo_prover::core::pcs::{CommitmentSchemeProver, CommitmentSchemeVerifier};
    use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};
    use stwo_prover::core::prover::{prove, verify, LOG_BLOWUP_FACTOR};
    use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
    use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;
    use stwo_prover::core::InteractionElements;

    use super::simd_trace::RetOpcodeInteractionGenerator;
    use crate::components::memory;
    use crate::components::ret_opcode::simd_trace::RetOpcodeComponent;
    use crate::components::tests::{
        generate_test_simd_ret, generate_test_simd_ret_memory, CairoTestAir,
    };

    #[test]
    fn test_cairo_air() {
        let ret_trace_generator = generate_test_simd_ret();
        let mut memory_trace_generator = generate_test_simd_ret_memory();
        let log_n_rows = 7;

        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(log_n_rows + LOG_BLOWUP_FACTOR + 2)
                .circle_domain()
                .half_coset,
        );

        // Setup protocol.
        let channel = &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme = &mut CommitmentSchemeProver::new(LOG_BLOWUP_FACTOR, &twiddles);

        // Trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        let ret_lookup_data =
            ret_trace_generator.write_trace(&mut tree_builder, &mut memory_trace_generator);
        let memory_lookup_data = memory_trace_generator.write_trace(&mut tree_builder);
        tree_builder.commit(channel);

        // Draw lookup element.
        let memory_lookup_elements = LookupElements::draw(channel);

        // Interaction trace.
        let (memory_interaction, memory_claimed_sum) =
            memory_lookup_data.write_interaction_trace(&memory_lookup_elements);
        let (ret_interaction, ret_claimed_sum) =
            RetOpcodeInteractionGenerator::write_interaction_trace(
                7,
                &ret_lookup_data,
                &memory_lookup_elements,
            );

        assert_eq!(memory_claimed_sum, ret_claimed_sum);
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(ret_interaction);
        tree_builder.extend_evals(memory_interaction);
        tree_builder.commit(channel);

        // Constant Trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        let is_first = gen_is_first::<SimdBackend>(7);
        let memory_is_first = gen_is_first::<SimdBackend>(7);
        tree_builder.extend_evals(vec![is_first, memory_is_first]);
        tree_builder.commit(channel);

        let ret_component = RetOpcodeComponent {
            log_size: 7,
            claimed_sum: ret_claimed_sum,
            lookup_elements: memory_lookup_elements.clone(),
        };
        let memory_component = memory::component::MemoryComponent {
            log_n_rows: 7,
            claimed_sum: memory_claimed_sum,
            lookup_elements: memory_lookup_elements.clone(),
        };

        let air = CairoTestAir {
            ret_component,
            memory_component,
        };

        let proof = prove::<SimdBackend, _, _>(
            &air.component_provers(),
            channel,
            &InteractionElements::default(),
            commitment_scheme,
        )
        .unwrap();

        // Verify.
        let verifier_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let commitment_scheme_verifier =
            &mut CommitmentSchemeVerifier::<Blake2sMerkleHasher>::new();
        let mut interaction_0_logbounds = Vec::new();
        let mut interaction_1_logbounds = Vec::new();
        let mut interaction_2_logbounds: Vec<u32> = Vec::new();
        air.component_provers()
            .iter()
            .map(|cp| cp.trace_log_degree_bounds().0)
            .map(|log_bounds| {
                assert_eq!(log_bounds.len(), 3);
                interaction_0_logbounds.extend(&log_bounds[0]);
                interaction_1_logbounds.extend(&log_bounds[1]);
                interaction_2_logbounds.extend(&log_bounds[2]);
            })
            .collect_vec();

        commitment_scheme_verifier.commit(
            proof.commitments[0],
            &interaction_0_logbounds,
            verifier_channel,
        );

        let lookup_elements = LookupElements::draw(verifier_channel);
        assert_eq!(lookup_elements, memory_lookup_elements);
        commitment_scheme_verifier.commit(
            proof.commitments[1],
            &interaction_1_logbounds,
            verifier_channel,
        );

        commitment_scheme_verifier.commit(
            proof.commitments[2],
            &interaction_2_logbounds,
            verifier_channel,
        );

        verify(
            &air.components(),
            verifier_channel,
            &InteractionElements::default(),
            commitment_scheme_verifier,
            proof,
        )
        .unwrap();
    }
}

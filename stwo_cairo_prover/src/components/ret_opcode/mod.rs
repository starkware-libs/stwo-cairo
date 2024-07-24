pub mod component;
pub mod cpu_prover;
pub mod test_utils;
pub mod trace;

#[cfg(test)]
pub mod tests {
    use itertools::Itertools;
    use num_traits::{One, Zero};
    use stwo_prover::core::backend::CpuBackend;
    use stwo_prover::core::channel::{Blake2sChannel, Channel};
    use stwo_prover::core::fields::m31::{BaseField, M31};
    use stwo_prover::core::fields::qm31::SecureField;
    use stwo_prover::core::fields::IntoSlice;
    use stwo_prover::core::utils::shifted_secure_combination;
    use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
    use stwo_prover::core::vcs::hasher::Hasher;
    use stwo_prover::trace_generation::{
        commit_and_prove, commit_and_verify, AirTraceGenerator, AirTraceVerifier,
        ComponentTraceGenerator,
    };

    use super::component::RET_COMPONENT_ID;
    use super::test_utils::TestAir;
    use super::trace::RetOpcodeCpuTraceGenerator;
    use crate::components::memory::component::{
        MemoryTraceGenerator, MEMORY_ALPHA, MEMORY_COMPONENT_ID, MEMORY_Z, N_M31_IN_FELT252,
    };
    use crate::components::ret_opcode::test_utils::TestRetAirGenerator;

    #[test]
    fn test_ret_interaction_trace() {
        let mut air_generator = TestRetAirGenerator::new();
        let trace = air_generator.write_trace();
        let prover_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let interaction_elements = air_generator.interaction_elements(prover_channel);
        let alpha = interaction_elements[MEMORY_ALPHA];
        let z = interaction_elements[MEMORY_Z];
        let mut expected_logup_sum = SecureField::zero();
        for i in 0..8 {
            assert_eq!(trace[0].values[i], M31::from_u32_unchecked(i as u32));
            let mut address_and_value = [M31::zero(); N_M31_IN_FELT252 + 1];
            address_and_value[0] = M31::from_u32_unchecked(i as u32);
            address_and_value[1] = M31::one();
            expected_logup_sum +=
                M31::one() / shifted_secure_combination(&address_and_value, alpha, z);
        }

        let interaction_trace = air_generator
            .interact(&trace, &interaction_elements)
            .into_iter()
            .take(4)
            .collect_vec();
        let logup_sum =
            SecureField::from_m31_array(std::array::from_fn(|j| interaction_trace[j][1]));

        assert_eq!(logup_sum, expected_logup_sum);
    }

    #[test]
    fn test_ret_proof() {
        let mut air_generator = TestRetAirGenerator::new();
        let trace = air_generator.write_trace();
        let prover_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let proof = commit_and_prove::<CpuBackend>(&air_generator, prover_channel, trace).unwrap();

        let verifier_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let air = TestAir {
            memory_component: air_generator
                .registry
                .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID)
                .component(),
            ret_component: air_generator
                .registry
                .get_generator::<RetOpcodeCpuTraceGenerator>(RET_COMPONENT_ID)
                .component(),
        };
        commit_and_verify(proof, &air, verifier_channel).unwrap();
    }
}

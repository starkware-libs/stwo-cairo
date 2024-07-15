pub mod component;
pub mod cpu_prover;
pub mod trace;

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::BTreeMap;

    use component::{RetOpcode, RET_COMPONENT_ID};
    use itertools::Itertools;
    use num_traits::{One, Zero};
    use stwo_prover::core::air::{Air, AirProver, Component, ComponentProver};
    use stwo_prover::core::backend::CpuBackend;
    use stwo_prover::core::channel::{Blake2sChannel, Channel};
    use stwo_prover::core::fields::m31::{BaseField, M31};
    use stwo_prover::core::fields::qm31::SecureField;
    use stwo_prover::core::fields::IntoSlice;
    use stwo_prover::core::poly::circle::CircleEvaluation;
    use stwo_prover::core::poly::BitReversedOrder;
    use stwo_prover::core::prover::VerificationError;
    use stwo_prover::core::utils::shifted_secure_combination;
    use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
    use stwo_prover::core::vcs::hasher::Hasher;
    use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
    use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
    use stwo_prover::trace_generation::{
        AirTraceGenerator, AirTraceVerifier, ComponentTraceGenerator,
    };
    use trace::RetOpcodeCpuTraceGenerator;

    use super::*;
    use crate::components::memory::component::{
        MemoryComponent, MemoryTraceGenerator, MEMORY_ALPHA, MEMORY_COMPONENT_ID, MEMORY_Z,
        N_M31_IN_FELT252,
    };

    pub fn register_test_ret_memory(registry: &mut ComponentGenerationRegistry) {
        registry.register(
            MEMORY_COMPONENT_ID,
            MemoryTraceGenerator::new("".to_string()),
        );
        let mut value = [M31::from_u32_unchecked(0); N_M31_IN_FELT252];
        value[0] = M31::from_u32_unchecked(1);

        registry
            .get_generator_mut::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID)
            .values = vec![value; 8];
    }

    pub fn register_test_ret(registry: &mut ComponentGenerationRegistry) {
        registry.register(
            RET_COMPONENT_ID,
            RetOpcodeCpuTraceGenerator { inputs: vec![] },
        );
        let inputs = (0..8)
            .map(|i| {
                [
                    M31::from_u32_unchecked(i),
                    M31::from_u32_unchecked(2),
                    M31::from_u32_unchecked(2),
                ]
            })
            .collect_vec();
        registry
            .get_generator_mut::<RetOpcodeCpuTraceGenerator>(RET_COMPONENT_ID)
            .add_inputs(&inputs);
    }

    struct TestRetAirGenerator {
        pub registry: ComponentGenerationRegistry,
    }

    impl TestRetAirGenerator {
        pub fn new() -> Self {
            let mut registry = ComponentGenerationRegistry::default();
            register_test_ret_memory(&mut registry);
            register_test_ret(&mut registry);
            Self { registry }
        }
    }

    impl AirTraceVerifier for TestRetAirGenerator {
        fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
            let elements = channel.draw_felts(2);
            InteractionElements::new(BTreeMap::from_iter(vec![
                (MEMORY_ALPHA.to_string(), elements[0]),
                (MEMORY_Z.to_string(), elements[1]),
            ]))
        }
    }

    impl AirTraceGenerator<CpuBackend> for TestRetAirGenerator {
        fn write_trace(
            &mut self,
        ) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
            // TODO(Ohad): add memory trace.
            let ret_trace =
                RetOpcodeCpuTraceGenerator::write_trace(RET_COMPONENT_ID, &mut self.registry);
            let memory_trace =
                MemoryTraceGenerator::write_trace(MEMORY_COMPONENT_ID, &mut self.registry);
            ret_trace.into_iter().chain(memory_trace).collect()
        }

        fn interact(
            &self,
            trace: &ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>,
            elements: &InteractionElements,
        ) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
            let ret_trace = trace.iter().take(7).collect_vec();
            let memory_trace = trace.iter().skip(7).collect_vec();
            let ret_intraction_trace = self
                .registry
                .get_generator::<RetOpcodeCpuTraceGenerator>(RET_COMPONENT_ID)
                .write_interaction_trace(&ret_trace, elements);
            let memory_interaction_trace = self
                .registry
                .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID)
                .write_interaction_trace(&memory_trace, elements);

            ret_intraction_trace
                .into_iter()
                .chain(memory_interaction_trace)
                .collect()
        }

        fn to_air_prover(&self) -> impl AirProver<CpuBackend> {
            let ret_component_generator = self
                .registry
                .get_generator::<RetOpcodeCpuTraceGenerator>(RET_COMPONENT_ID);
            let memory_component_generator = self
                .registry
                .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID);
            TestAir {
                ret_component: ret_component_generator.component(),
                memory_component: memory_component_generator.component(),
            }
        }

        fn composition_log_degree_bound(&self) -> u32 {
            let component_generator = self
                .registry
                .get_generator::<RetOpcodeCpuTraceGenerator>(RET_COMPONENT_ID);
            component_generator
                .component()
                .max_constraint_log_degree_bound()
        }
    }

    #[derive(Clone)]
    pub struct TestAir {
        pub ret_component: RetOpcode,
        pub memory_component: MemoryComponent,
    }

    impl Air for TestAir {
        fn components(&self) -> Vec<&dyn Component> {
            vec![&self.ret_component, &self.memory_component]
        }

        fn verify_lookups(&self, _lookup_values: &LookupValues) -> Result<(), VerificationError> {
            Ok(())
        }
    }

    impl AirProver<CpuBackend> for TestAir {
        fn prover_components(&self) -> Vec<&dyn ComponentProver<CpuBackend>> {
            vec![&self.ret_component, &self.memory_component]
        }
    }

    impl AirTraceVerifier for TestAir {
        fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
            let elements = channel.draw_felts(2);
            InteractionElements::new(BTreeMap::from_iter(vec![
                (MEMORY_ALPHA.to_string(), elements[0]),
                (MEMORY_Z.to_string(), elements[1]),
            ]))
        }
    }

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
}

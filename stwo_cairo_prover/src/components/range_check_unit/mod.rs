pub mod component;
pub mod component_prover;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use component::{RangeCheckUnitComponent, RangeCheckUnitTraceGenerator, RC_COMPONENT_ID, RC_Z};
    use stwo_prover::core::air::{Air, AirProver, Component, ComponentProver};
    use stwo_prover::core::backend::CpuBackend;
    use stwo_prover::core::channel::{Blake2sChannel, Channel};
    use stwo_prover::core::fields::m31::BaseField;
    use stwo_prover::core::fields::IntoSlice;
    use stwo_prover::core::poly::circle::CircleEvaluation;
    use stwo_prover::core::poly::BitReversedOrder;
    use stwo_prover::core::prover::VerificationError;
    use stwo_prover::core::vcs::blake2_hash::Blake2sHasher;
    use stwo_prover::core::vcs::hasher::Hasher;
    use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
    use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
    use stwo_prover::trace_generation::{
        commit_and_prove, commit_and_verify, AirTraceGenerator, AirTraceVerifier,
        ComponentTraceGenerator,
    };

    use super::*;

    pub fn register_test_rc(registry: &mut ComponentGenerationRegistry) {
        registry.register(RC_COMPONENT_ID, RangeCheckUnitTraceGenerator::new(8));
        vec![
            vec![BaseField::from_u32_unchecked(0); 3],
            vec![BaseField::from_u32_unchecked(1); 1],
            vec![BaseField::from_u32_unchecked(2); 2],
            vec![BaseField::from_u32_unchecked(3); 5],
            vec![BaseField::from_u32_unchecked(4); 10],
            vec![BaseField::from_u32_unchecked(5); 1],
            vec![BaseField::from_u32_unchecked(6); 0],
            vec![BaseField::from_u32_unchecked(7); 1],
        ]
        .into_iter()
        .flatten()
        .for_each(|input| {
            registry
                .get_generator_mut::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID)
                .add_inputs(&input);
        });
    }

    struct TestAirGenerator {
        pub registry: ComponentGenerationRegistry,
    }

    impl TestAirGenerator {
        pub fn new() -> Self {
            let mut registry = ComponentGenerationRegistry::default();
            register_test_rc(&mut registry);
            Self { registry }
        }
    }

    impl AirTraceVerifier for TestAirGenerator {
        fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
            let element = channel.draw_felts(1)[0];
            InteractionElements::new(BTreeMap::from_iter(vec![(RC_Z.to_string(), element)]))
        }

        fn verify_lookups(&self, _lookup_values: &LookupValues) -> Result<(), VerificationError> {
            Ok(())
        }
    }

    impl AirTraceGenerator<CpuBackend> for TestAirGenerator {
        fn write_trace(
            &mut self,
        ) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
            RangeCheckUnitTraceGenerator::write_trace(RC_COMPONENT_ID, &mut self.registry)
        }

        fn interact(
            &self,
            trace: &ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>,
            elements: &InteractionElements,
        ) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
            let component_generator = self
                .registry
                .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID);
            component_generator.write_interaction_trace(&trace.iter().collect(), elements)
        }

        fn to_air_prover(&self) -> impl AirProver<CpuBackend> {
            let component_generator = self
                .registry
                .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID);
            TestAir {
                component: component_generator.component(),
            }
        }

        fn composition_log_degree_bound(&self) -> u32 {
            let component_generator = self
                .registry
                .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID);
            component_generator
                .component()
                .max_constraint_log_degree_bound()
        }
    }

    #[derive(Clone)]
    pub struct TestAir {
        pub component: RangeCheckUnitComponent,
    }

    impl Air for TestAir {
        fn components(&self) -> Vec<&dyn Component> {
            vec![&self.component]
        }
    }

    impl AirProver<CpuBackend> for TestAir {
        fn prover_components(&self) -> Vec<&dyn ComponentProver<CpuBackend>> {
            vec![&self.component]
        }
    }

    impl AirTraceVerifier for TestAir {
        fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
            let element = channel.draw_felts(1)[0];
            InteractionElements::new(BTreeMap::from_iter(vec![(RC_Z.to_string(), element)]))
        }

        fn verify_lookups(&self, _lookup_values: &LookupValues) -> Result<(), VerificationError> {
            Ok(())
        }
    }

    #[test]
    fn test_rc_constraints() {
        let mut air = TestAirGenerator::new();
        let trace = air.write_trace();
        let prover_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let proof = commit_and_prove::<CpuBackend>(&air, prover_channel, trace).unwrap();

        let verifier_channel =
            &mut Blake2sChannel::new(Blake2sHasher::hash(BaseField::into_slice(&[])));
        let air = TestAir {
            component: air
                .registry
                .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID)
                .component(),
        };
        commit_and_verify(proof, &air, verifier_channel).unwrap();
    }
}

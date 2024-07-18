use std::collections::BTreeMap;

use itertools::Itertools;
use stwo_prover::core::air::{Air, AirProver, Component, ComponentProver};
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::poly::circle::CircleEvaluation;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::VerificationError;
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{AirTraceGenerator, AirTraceVerifier, ComponentTraceGenerator};

use crate::components::memory::component::{
    MemoryComponent, MemoryTraceGenerator, MAX_MEMORY_CELL_VALUE, MEMORY_ALPHA,
    MEMORY_COMPONENT_ID, MEMORY_Z, N_M31_IN_FELT252, N_MEMORY_COLUMNS,
};
use crate::components::range_check_unit::component::{
    RangeCheckUnitTraceGenerator, RC_COMPONENT_ID, RC_Z,
};
use crate::components::ret_opcode::component::{RetOpcode, RET_COMPONENT_ID, RET_N_TRACE_CELLS};
use crate::components::ret_opcode::trace::RetOpcodeCpuTraceGenerator;

pub fn register_test_ret_memory(registry: &mut ComponentGenerationRegistry) {
    registry.register(
        MEMORY_COMPONENT_ID,
        MemoryTraceGenerator::new("".to_string()),
    );
    registry.register(
        RC_COMPONENT_ID,
        RangeCheckUnitTraceGenerator::new(MAX_MEMORY_CELL_VALUE),
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

pub struct TestRetAirGenerator {
    pub registry: ComponentGenerationRegistry,
}

impl TestRetAirGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut registry = ComponentGenerationRegistry::default();
        register_test_ret_memory(&mut registry);
        register_test_ret(&mut registry);
        Self { registry }
    }
}

impl AirTraceVerifier for TestRetAirGenerator {
    fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
        let elements = channel.draw_felts(3);
        InteractionElements::new(BTreeMap::from_iter(vec![
            (MEMORY_ALPHA.to_string(), elements[0]),
            (MEMORY_Z.to_string(), elements[1]),
            (RC_Z.to_string(), elements[2]),
        ]))
    }
}

impl AirTraceGenerator<CpuBackend> for TestRetAirGenerator {
    fn write_trace(&mut self) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
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
        let trace_iter = &mut trace.iter();
        let ret_trace = trace_iter.take(RET_N_TRACE_CELLS).collect_vec();
        let ret_intraction_trace = self
            .registry
            .get_generator::<RetOpcodeCpuTraceGenerator>(RET_COMPONENT_ID)
            .write_interaction_trace(&ret_trace, elements);
        let memory_trace = trace_iter.take(N_MEMORY_COLUMNS).collect_vec();
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
        let elements = channel.draw_felts(3);
        InteractionElements::new(BTreeMap::from_iter(vec![
            (MEMORY_ALPHA.to_string(), elements[0]),
            (MEMORY_Z.to_string(), elements[1]),
            (RC_Z.to_string(), elements[2]),
        ]))
    }
}

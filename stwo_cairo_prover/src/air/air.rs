use std::collections::BTreeMap;

use stwo_prover::core::air::{Air, AirProver, Component, ComponentProver};
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::channel::{Blake2sChannel, Channel};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::CircleEvaluation;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::VerificationError;
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{AirTraceGenerator, AirTraceVerifier, ComponentTraceGenerator};

use crate::components::memory::component::{
    MemoryComponent, MemoryTraceGenerator, MEMORY_ADDRESS_BOUND, MEMORY_ALPHA, MEMORY_COMPONENT_ID,
    MEMORY_Z, N_MEMORY_COLUMNS,
};
use crate::components::range_check_unit::component::{
    RangeCheckUnitComponent, RangeCheckUnitTraceGenerator, N_RC_COLUMNS, RC_COMPONENT_ID, RC_Z,
};

struct CairoAirGenerator {
    pub registry: ComponentGenerationRegistry,
}

impl CairoAirGenerator {
    pub fn new(path: String) -> Self {
        let mut registry = ComponentGenerationRegistry::default();
        registry.register(MEMORY_COMPONENT_ID, MemoryTraceGenerator::new(path));
        registry.register(
            RC_COMPONENT_ID,
            RangeCheckUnitTraceGenerator::new(MEMORY_ADDRESS_BOUND),
        );
        Self { registry }
    }
}

impl AirTraceVerifier for CairoAirGenerator {
    fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
        let elements = channel.draw_felts(3);
        InteractionElements::new(BTreeMap::from_iter(vec![
            (MEMORY_ALPHA.to_string(), elements[0]),
            (MEMORY_Z.to_string(), elements[1]),
            (RC_Z.to_string(), elements[2]),
        ]))
    }
}

impl AirTraceGenerator<CpuBackend> for CairoAirGenerator {
    fn write_trace(&mut self) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let mut trace = Vec::with_capacity(N_MEMORY_COLUMNS + N_RC_COLUMNS);
        trace.extend(MemoryTraceGenerator::write_trace(
            MEMORY_COMPONENT_ID,
            &mut self.registry,
        ));
        trace.extend(RangeCheckUnitTraceGenerator::write_trace(
            RC_COMPONENT_ID,
            &mut self.registry,
        ));
        trace
    }

    fn interact(
        &self,
        trace: &ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>,
        elements: &InteractionElements,
    ) -> Vec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let mut interaction_trace = Vec::new();
        let trace_iter = &mut trace.iter();
        let memory_generator = self
            .registry
            .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID);
        interaction_trace.extend(
            memory_generator
                .write_interaction_trace(&trace_iter.take(N_MEMORY_COLUMNS).collect(), elements),
        );
        let rc_generator = self
            .registry
            .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID);
        interaction_trace.extend(
            rc_generator
                .write_interaction_trace(&trace_iter.take(N_RC_COLUMNS).collect(), elements),
        );
        interaction_trace
    }

    fn to_air_prover(&self) -> impl AirProver<CpuBackend> {
        let memory = self
            .registry
            .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID);
        let range_check_unit = self
            .registry
            .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID);
        CairoAir {
            memory: memory.component(),
            range_check_unit: range_check_unit.component(),
        }
    }

    fn composition_log_degree_bound(&self) -> u32 {
        let component_generator = self
            .registry
            .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID);
        component_generator
            .component()
            .max_constraint_log_degree_bound()
    }
}

#[derive(Clone)]
pub struct CairoAir {
    pub memory: MemoryComponent,
    pub range_check_unit: RangeCheckUnitComponent,
}

impl Air for CairoAir {
    fn components(&self) -> Vec<&dyn Component> {
        vec![&self.memory, &self.range_check_unit]
    }

    fn verify_lookups(&self, _lookup_values: &LookupValues) -> Result<(), VerificationError> {
        Ok(())
    }
}

impl AirProver<CpuBackend> for CairoAir {
    fn prover_components(&self) -> Vec<&dyn ComponentProver<CpuBackend>> {
        vec![&self.memory, &self.range_check_unit]
    }
}

impl AirTraceVerifier for CairoAir {
    fn interaction_elements(&self, channel: &mut Blake2sChannel) -> InteractionElements {
        let elements = channel.draw_felts(3);
        InteractionElements::new(BTreeMap::from_iter(vec![
            (MEMORY_ALPHA.to_string(), elements[0]),
            (MEMORY_Z.to_string(), elements[1]),
            (RC_Z.to_string(), elements[2]),
        ]))
    }
}

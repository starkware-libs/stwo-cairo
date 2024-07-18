use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;

use crate::components::memory::component::{
    MemoryTraceGenerator, MAX_MEMORY_CELL_VALUE, MEMORY_COMPONENT_ID,
};
use crate::components::range_check_unit::component::{
    RangeCheckUnitTraceGenerator, RC_COMPONENT_ID,
};

pub fn register_test_memory(registry: &mut ComponentGenerationRegistry) {
    registry.register(
        MEMORY_COMPONENT_ID,
        MemoryTraceGenerator::new("".to_string()),
    );
    registry.register(
        RC_COMPONENT_ID,
        RangeCheckUnitTraceGenerator::new(MAX_MEMORY_CELL_VALUE),
    );
}

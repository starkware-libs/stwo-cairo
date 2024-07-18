use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::ComponentTraceGenerator;

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
            .get_generator_mut::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID)
            .add_inputs(&input);
    });
}

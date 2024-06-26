use itertools::Itertools;
use num_traits::Zero;

use stwo_prover::core::air::ComponentTraceWriter;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::{ColumnVec, InteractionElements};

use stwo_prover::trace_generation::registry::ComponentRegistry;
use stwo_prover::trace_generation::{ComponentGen, TraceGenerator};

pub const RC_Z: &str = "RangeCheckUnit_Z";
pub const RC_COMPONENT_ID: &str = "RC_UNIT";

pub struct RangeCheckUnitComponent {
    pub max_value: usize,
    pub multiplicities: Vec<u32>,
}

impl RangeCheckUnitComponent {
    pub fn new(max_value: usize) -> Self {
        assert!(max_value.is_power_of_two());
        Self {
            max_value,
            multiplicities: vec![0; max_value],
        }
    }
}

impl ComponentGen for RangeCheckUnitComponent {}

impl TraceGenerator<CpuBackend> for RangeCheckUnitComponent {
    type ComponentInputs = Vec<BaseField>;

    fn add_inputs(&mut self, inputs: &Self::ComponentInputs) {
        for input in inputs {
            let input = input.0 as usize;
            if input < self.max_value {
                self.multiplicities[input] += 1;
            }
        }
    }

    fn write_trace(
        component_id: &str,
        registry: &mut ComponentRegistry,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let rc_unit_component = registry.get_component::<RangeCheckUnitComponent>(component_id);
        let rc_max_value = rc_unit_component.max_value;

        let mut trace = vec![vec![BaseField::zero(); rc_max_value]; 1];
        for (i, multiplicity) in rc_unit_component.multiplicities.iter().enumerate() {
            trace[0][i] = BaseField::from_u32_unchecked(*multiplicity);
        }

        let domain = CanonicCoset::new(rc_max_value.checked_ilog2().unwrap()).circle_domain();
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<CpuBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec()
    }
}

impl ComponentTraceWriter<CpuBackend> for RangeCheckUnitComponent {
    fn write_interaction_trace(
        &self,
        trace: &ColumnVec<&CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>,
        elements: &InteractionElements,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let interaction_trace_domain = trace[0].domain;
        let z = elements[RC_Z];

        let interaction_values =
            trace[0]
                .values
                .iter()
                .enumerate()
                .fold(Vec::new(), |mut acc, (i, multiplicity)| {
                    let value = BaseField::from_u32_unchecked(i as u32);
                    if let Some(&last) = acc.last() {
                        acc.push(last + (value - z).inverse() * *multiplicity);
                    } else {
                        acc.push((value - z).inverse() * *multiplicity);
                    }
                    acc
                });

        vec![CircleEvaluation::new(
            interaction_trace_domain,
            interaction_values,
        )]
    }
}

#[cfg(test)]
mod tests {
    use stwo_prover::core::fields::m31::P;

    use super::*;

    #[test]
    fn test_rc_unit_trace() {
        let mut registry = ComponentRegistry::default();
        let random_seed: usize = 17;
        registry.register_component(RC_COMPONENT_ID, RangeCheckUnitComponent::new(8));
        let inputs = vec![
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
        .collect_vec();
        registry
            .get_component_mut::<RangeCheckUnitComponent>(RC_COMPONENT_ID)
            .add_inputs(&inputs);
        let interaction_elements = InteractionElements::new(
            [(
                RC_Z.to_string(),
                BaseField::from_u32_unchecked(random_seed as u32),
            )]
            .into(),
        );

        let trace = RangeCheckUnitComponent::write_trace(RC_COMPONENT_ID, &mut registry);
        let interaction_trace = registry
            .get_component::<RangeCheckUnitComponent>(RC_COMPONENT_ID)
            .write_interaction_trace(&trace.iter().collect(), &interaction_elements);
        let mut trace_sum = BaseField::zero();
        for i in 0..8 {
            trace_sum +=
                trace[0].values[i] / BaseField::from_u32_unchecked(P - (random_seed - i) as u32);
        }
        let logup_sum = interaction_trace.first().unwrap().values[7];

        assert_eq!(logup_sum, trace_sum);
    }
}

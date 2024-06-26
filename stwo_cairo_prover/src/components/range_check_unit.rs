use itertools::Itertools;
use num_traits::Zero;

use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::{Component, ComponentTraceWriter};
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::circle::CirclePoint;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::{ColumnVec, InteractionElements};

use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{ComponentGen, TraceGenerator};

pub const RC_Z: &str = "RangeCheckUnit_Z";
pub const RC_COMPONENT_ID: &str = "RC_UNIT";

pub struct RangeCheckUnitComponent {
    pub log_n_instances: usize,
}

pub struct RangeCheckUnitTraceGenerator {
    pub max_value: usize,
    pub multiplicities: Vec<u32>,
}

impl Component for RangeCheckUnitComponent {
    fn n_constraints(&self) -> usize {
        unimplemented!()
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        unimplemented!()
    }

    fn n_interaction_phases(&self) -> u32 {
        unimplemented!()
    }

    fn trace_log_degree_bounds(&self) -> TreeVec<ColumnVec<u32>> {
        unimplemented!()
    }

    fn mask_points(
        &self,
        _point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        unimplemented!()
    }

    fn evaluate_constraint_quotients_at_point(
        &self,
        _point: CirclePoint<SecureField>,
        _mask: &ColumnVec<Vec<SecureField>>,
        _evaluation_accumulator: &mut PointEvaluationAccumulator,
        _interaction_elements: &InteractionElements,
    ) {
        unimplemented!()
    }

    fn interaction_element_ids(&self) -> Vec<String> {
        unimplemented!()
    }
}

impl RangeCheckUnitTraceGenerator {
    pub fn new(max_value: usize) -> Self {
        assert!(max_value.is_power_of_two());
        Self {
            max_value,
            multiplicities: vec![0; max_value],
        }
    }
}

impl ComponentGen for RangeCheckUnitTraceGenerator {}

impl TraceGenerator<CpuBackend> for RangeCheckUnitTraceGenerator {
    type Component = RangeCheckUnitComponent;
    type Inputs = Vec<BaseField>;

    fn add_inputs(&mut self, inputs: &Self::Inputs) {
        for input in inputs {
            let input = input.0 as usize;
            debug_assert!(input < self.max_value, "Input out of range");
            self.multiplicities[input] += 1;
        }
    }

    fn write_trace(
        component_id: &str,
        registry: &mut ComponentGenerationRegistry,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let rc_unit_trace_generator =
            registry.get_generator::<RangeCheckUnitTraceGenerator>(component_id);
        let rc_max_value = rc_unit_trace_generator.max_value;

        let mut trace = vec![vec![BaseField::zero(); rc_max_value]; 1];
        for (i, multiplicity) in rc_unit_trace_generator.multiplicities.iter().enumerate() {
            trace[0][i] = BaseField::from_u32_unchecked(*multiplicity);
        }

        let domain = CanonicCoset::new(rc_max_value.checked_ilog2().unwrap()).circle_domain();
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<CpuBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec()
    }

    fn component(&self) -> RangeCheckUnitComponent {
        RangeCheckUnitComponent {
            log_n_instances: self.max_value.checked_ilog2().unwrap() as usize,
        }
    }
}

impl ComponentTraceWriter<CpuBackend> for RangeCheckUnitTraceGenerator {
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
                    match acc.last() {
                        Some(&last) => {
                            acc.push(last + (z - value).inverse() * *multiplicity);
                        }
                        None => {
                            acc.push((z - value).inverse() * *multiplicity);
                        }
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
    use super::*;

    #[test]
    fn test_rc_unit_trace() {
        let mut registry = ComponentGenerationRegistry::default();
        let random_seed: usize = 117;
        registry.register(RC_COMPONENT_ID, RangeCheckUnitTraceGenerator::new(8));
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
            .get_generator_mut::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID)
            .add_inputs(&inputs);

        let trace = RangeCheckUnitTraceGenerator::write_trace(RC_COMPONENT_ID, &mut registry);
        let interaction_elements = InteractionElements::new(
            [(
                RC_Z.to_string(),
                BaseField::from_u32_unchecked(random_seed as u32),
            )]
            .into(),
        );
        let interaction_trace = registry
            .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID)
            .write_interaction_trace(&trace.iter().collect(), &interaction_elements);

        let mut trace_sum = BaseField::zero();
        for i in 0..8 {
            trace_sum += trace.last().unwrap().values[i]
                / BaseField::from_u32_unchecked((random_seed - i) as u32);
        }
        let logup_sum = interaction_trace.first().unwrap().last().unwrap();

        assert_eq!(*logup_sum, trace_sum);
    }
}

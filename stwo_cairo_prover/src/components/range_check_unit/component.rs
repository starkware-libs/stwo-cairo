use itertools::{zip_eq, Itertools};
use num_traits::Zero;
use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::mask::fixed_mask_points;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::circle::{CirclePoint, Coset};
use stwo_prover::core::constraints::point_vanishing;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::{SecureColumn, SECURE_EXTENSION_DEGREE};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::{BASE_TRACE, INTERACTION_TRACE};
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{ComponentGen, ComponentTraceGenerator};

pub const RC_Z: &str = "RangeCheckUnit_Z";
pub const RC_COMPONENT_ID: &str = "RC_UNIT";
pub const RC_LOOKUP_VALUE_0: &str = "RC_UNIT_LOOKUP_0";
pub const RC_LOOKUP_VALUE_1: &str = "RC_UNIT_LOOKUP_1";
pub const RC_LOOKUP_VALUE_2: &str = "RC_UNIT_LOOKUP_2";
pub const RC_LOOKUP_VALUE_3: &str = "RC_UNIT_LOOKUP_3";

#[derive(Clone)]
pub struct RangeCheckUnitComponent {
    pub log_n_instances: u32,
}

pub struct RangeCheckUnitTraceGenerator {
    pub max_value: usize,
    pub multiplicities: Vec<u32>,
}

impl RangeCheckUnitComponent {
    fn evaluate_lookup_boundary_constraints_at_point(
        &self,
        point: CirclePoint<SecureField>,
        mask: &TreeVec<Vec<Vec<SecureField>>>,
        evaluation_accumulator: &mut PointEvaluationAccumulator,
        interaction_elements: &InteractionElements,
        constraint_zero_domain: Coset,
        lookup_values: &LookupValues,
    ) {
        let z = interaction_elements[RC_Z];
        let value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][0]));
        let numerator = value * (z - mask[BASE_TRACE][0][0]) - mask[BASE_TRACE][1][0];
        let denom = point_vanishing(constraint_zero_domain.at(0), point);
        evaluation_accumulator.accumulate(numerator / denom);

        let lookup_value = SecureField::from_m31(
            lookup_values[RC_LOOKUP_VALUE_0],
            lookup_values[RC_LOOKUP_VALUE_1],
            lookup_values[RC_LOOKUP_VALUE_2],
            lookup_values[RC_LOOKUP_VALUE_3],
        );
        let numerator = value - lookup_value;
        let denom = point_vanishing(
            constraint_zero_domain.at(constraint_zero_domain.size() - 1),
            point,
        );
        evaluation_accumulator.accumulate(numerator / denom);
    }
}

impl Component for RangeCheckUnitComponent {
    fn n_constraints(&self) -> usize {
        2
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_instances + 1
    }

    fn trace_log_degree_bounds(&self) -> TreeVec<ColumnVec<u32>> {
        TreeVec::new(vec![
            vec![self.log_n_instances; 2],
            vec![self.log_n_instances; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn mask_points(
        &self,
        point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        TreeVec::new(vec![
            fixed_mask_points(&vec![vec![0_usize]; 2], point),
            vec![vec![point]; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn evaluate_constraint_quotients_at_point(
        &self,
        point: CirclePoint<SecureField>,
        mask: &TreeVec<Vec<Vec<SecureField>>>,
        evaluation_accumulator: &mut PointEvaluationAccumulator,
        interaction_elements: &InteractionElements,
        lookup_values: &LookupValues,
    ) {
        let constraint_zero_domain = CanonicCoset::new(self.log_n_instances).coset;
        self.evaluate_lookup_boundary_constraints_at_point(
            point,
            mask,
            evaluation_accumulator,
            interaction_elements,
            constraint_zero_domain,
            lookup_values,
        );
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

impl ComponentTraceGenerator<CpuBackend> for RangeCheckUnitTraceGenerator {
    type Component = RangeCheckUnitComponent;
    type Inputs = Vec<BaseField>;

    fn add_inputs(&mut self, inputs: &Self::Inputs) {
        for input in inputs {
            let input = input.0 as usize;
            // TODO: replace the debug_assert! with an error return.
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

        let mut trace = vec![vec![BaseField::zero(); rc_max_value]; 2];
        for (i, multiplicity) in rc_unit_trace_generator.multiplicities.iter().enumerate() {
            trace[0][i] = BaseField::from_u32_unchecked(i as u32);
            trace[1][i] = BaseField::from_u32_unchecked(*multiplicity);
        }

        let domain = CanonicCoset::new(rc_max_value.checked_ilog2().unwrap()).circle_domain();
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<CpuBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec()
    }

    fn write_interaction_trace(
        &self,
        trace: &ColumnVec<&CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>,
        elements: &InteractionElements,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let interaction_trace_domain = trace[0].domain;
        let z = elements[RC_Z];

        let mut last = SecureField::zero();
        let denoms = trace[0].values.iter().map(|value| z - *value).collect_vec();
        let mut denom_inverses = vec![SecureField::zero(); denoms.len()];
        SecureField::batch_inverse(&denoms, &mut denom_inverses);
        let logup_values = zip_eq(denom_inverses, &trace[1].values).fold(
            Vec::new(),
            |mut acc, (denom_inverse, multiplicity)| {
                let interaction_value = last + (denom_inverse * *multiplicity);
                acc.push(interaction_value);
                last = interaction_value;
                acc
            },
        );
        let secure_column: SecureColumn<CpuBackend> = logup_values.into_iter().collect();
        secure_column
            .columns
            .into_iter()
            .map(|eval| CircleEvaluation::new(interaction_trace_domain, eval))
            .collect_vec()
    }

    fn component(&self) -> RangeCheckUnitComponent {
        RangeCheckUnitComponent {
            log_n_instances: self.max_value.checked_ilog2().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::range_check_unit::tests::register_test_rc;

    #[test]
    fn test_rc_unit_trace() {
        let mut registry = ComponentGenerationRegistry::default();
        register_test_rc(&mut registry);
        let trace = RangeCheckUnitTraceGenerator::write_trace(RC_COMPONENT_ID, &mut registry);
        let random_value = SecureField::from_u32_unchecked(1, 2, 3, 117);
        let interaction_elements =
            InteractionElements::new([(RC_Z.to_string(), random_value)].into());
        let interaction_trace = registry
            .get_generator::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID)
            .write_interaction_trace(&trace.iter().collect(), &interaction_elements);

        let mut trace_sum = SecureField::zero();
        for i in 0..8 {
            assert_eq!(trace[0].values[i], BaseField::from_u32_unchecked(i as u32));
            trace_sum += trace.last().unwrap().values[i]
                / (random_value - BaseField::from_u32_unchecked(i as u32));
        }
        let logup_sum = SecureField::from_m31_array(std::array::from_fn(|j| {
            *interaction_trace[j].last().unwrap()
        }));

        assert_eq!(logup_sum, trace_sum);
    }
}

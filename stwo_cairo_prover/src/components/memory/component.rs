use itertools::{zip_eq, Itertools};
use num_traits::{One, Zero};
use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::mask::fixed_mask_points;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::circle::CirclePoint;
use stwo_prover::core::constraints::{coset_vanishing, point_excluder, point_vanishing};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::{SecureColumn, SECURE_EXTENSION_DEGREE};
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{
    bit_reverse_index, coset_order_to_circle_domain_order_index, shifted_secure_combination,
};
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{
    ComponentGen, ComponentTraceGenerator, BASE_TRACE, INTERACTION_TRACE,
};

use crate::components::range_check_unit::component::{
    RangeCheckUnitTraceGenerator, RC_COMPONENT_ID, RC_Z,
};

pub const MEMORY_ALPHA: &str = "MEMORY_ALPHA";
pub const MEMORY_Z: &str = "MEMORY_Z";
pub const MEMORY_COMPONENT_ID: &str = "MEMORY";
pub const MEMORY_LOOKUP_VALUE_0: &str = "MEMORY_LOOKUP_0";
pub const MEMORY_LOOKUP_VALUE_1: &str = "MEMORY_LOOKUP_1";
pub const MEMORY_LOOKUP_VALUE_2: &str = "MEMORY_LOOKUP_2";
pub const MEMORY_LOOKUP_VALUE_3: &str = "MEMORY_LOOKUP_3";
pub const MEMORY_RC_LOOKUP_VALUE_0: &str = "MEMORY_RC_LOOKUP_0";
pub const MEMORY_RC_LOOKUP_VALUE_1: &str = "MEMORY_RC_LOOKUP_1";
pub const MEMORY_RC_LOOKUP_VALUE_2: &str = "MEMORY_RC_LOOKUP_2";
pub const MEMORY_RC_LOOKUP_VALUE_3: &str = "MEMORY_RC_LOOKUP_3";

pub const MAX_MEMORY_CELL_VALUE: usize = 1 << 9;
pub const N_M31_IN_FELT252: usize = 28;
pub const MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252 + 1;
// TODO(AlonH): Make memory size configurable.
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 3;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;
// Addresses, M31 values, and multiplicities.
pub const N_MEMORY_COLUMNS: usize = 1 + N_M31_IN_FELT252 + 1;

/// Addresses are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
pub struct MemoryTraceGenerator {
    // TODO(AlonH): Consider to change values to be Felt252.
    pub values: Vec<[BaseField; N_M31_IN_FELT252]>,
    pub multiplicities: Vec<u32>,
}

#[derive(Clone)]
pub struct MemoryComponent {
    pub log_n_rows: u32,
}

impl MemoryComponent {
    pub const fn n_columns(&self) -> usize {
        N_MEMORY_COLUMNS
    }
}

impl MemoryTraceGenerator {
    pub fn new(_path: String) -> Self {
        // TODO(AlonH): change to read from file.
        let values = (0..MEMORY_ADDRESS_BOUND)
            .map(|i| {
                let value = BaseField::from_u32_unchecked(i as u32);
                [value; N_M31_IN_FELT252]
            })
            .collect();
        let multiplicities = vec![0; MEMORY_ADDRESS_BOUND];
        Self {
            values,
            multiplicities,
        }
    }
}

impl ComponentGen for MemoryTraceGenerator {}

impl ComponentTraceGenerator<CpuBackend> for MemoryTraceGenerator {
    type Component = MemoryComponent;
    type Inputs = BaseField;

    fn add_inputs(&mut self, inputs: &Self::Inputs) {
        let input = inputs.0 as usize;
        // TODO: replace the debug_assert! with an error return.
        debug_assert!(input < MEMORY_ADDRESS_BOUND, "Input out of range");
        self.multiplicities[input] += 1;
    }

    fn write_trace(
        component_id: &str,
        registry: &mut ComponentGenerationRegistry,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let memory_trace_generator = registry.get_generator::<MemoryTraceGenerator>(component_id);

        let mut trace = vec![vec![BaseField::zero(); MEMORY_ADDRESS_BOUND]; N_M31_IN_FELT252 + 2];
        for (i, (values, multiplicity)) in zip_eq(
            &memory_trace_generator.values,
            &memory_trace_generator.multiplicities,
        )
        .enumerate()
        {
            // TODO(AlonH): Either create a constant column for the addresses and remove it from
            // here or add constraints to the column here.
            trace[0][i] = BaseField::from_u32_unchecked(i as u32);
            for (j, value) in values.iter().enumerate() {
                trace[j + 1][i] = *value;
            }
            trace[MULTIPLICITY_COLUMN_OFFSET][i] = BaseField::from_u32_unchecked(*multiplicity);
        }

        let rc_generator =
            registry.get_generator_mut::<RangeCheckUnitTraceGenerator>(RC_COMPONENT_ID);
        for column in trace[1..MULTIPLICITY_COLUMN_OFFSET].iter() {
            column
                .iter()
                .for_each(|input| rc_generator.add_inputs(input));
        }

        let domain = CanonicCoset::new(LOG_MEMORY_ADDRESS_BOUND).circle_domain();
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
        let domain_size = interaction_trace_domain.size();
        let (alpha, z, rc_z) = (elements[MEMORY_ALPHA], elements[MEMORY_Z], elements[RC_Z]);

        let addresses_and_values: Vec<[BaseField; N_M31_IN_FELT252 + 1]> = (0
            ..MEMORY_ADDRESS_BOUND)
            .map(|i| std::array::from_fn(|j| trace[j].values[i]))
            .collect_vec();
        let denoms = addresses_and_values
            .iter()
            .map(|address_and_value| shifted_secure_combination(address_and_value, alpha, z))
            .collect_vec();
        let mut denom_inverses = vec![SecureField::zero(); domain_size];
        SecureField::batch_inverse(&denoms, &mut denom_inverses);
        let mut logup_values = vec![SecureField::zero(); domain_size];
        let mut rc_logup_values = vec![vec![SecureField::zero(); domain_size]; N_M31_IN_FELT252];
        let mut last = SecureField::zero();
        let mut column_rc_last = SecureField::zero();
        let log_size = interaction_trace_domain.log_size();
        for i in 0..domain_size {
            let index = bit_reverse_index(
                coset_order_to_circle_domain_order_index(i, log_size),
                log_size,
            );
            let interaction_value =
                last + (denom_inverses[index] * trace[MULTIPLICITY_COLUMN_OFFSET].values[index]);
            logup_values[index] = interaction_value;
            last = interaction_value;

            let mut row_rc_last = SecureField::zero();
            // TODO(AlonH): Batch inverse.
            for j in 0..(N_M31_IN_FELT252 - 1) {
                let rc_interaction_value =
                    row_rc_last + (rc_z - trace[j + 1].values[index]).inverse();
                rc_logup_values[j][index] = rc_interaction_value;
                row_rc_last = rc_interaction_value;
            }
            let rc_interaction_value = column_rc_last
                + row_rc_last
                + (rc_z - trace[N_M31_IN_FELT252].values[index]).inverse();
            rc_logup_values[N_M31_IN_FELT252 - 1][index] = rc_interaction_value;
            column_rc_last = rc_interaction_value;
        }
        let secure_column: SecureColumn<CpuBackend> = logup_values.into_iter().collect();
        let rc_columns: Vec<Vec<BaseField>> = rc_logup_values
            .into_iter()
            .flat_map(|values| {
                values
                    .into_iter()
                    .collect::<SecureColumn<CpuBackend>>()
                    .columns
            })
            .collect_vec();
        secure_column
            .columns
            .into_iter()
            .chain(rc_columns)
            .map(|eval| CircleEvaluation::new(interaction_trace_domain, eval))
            .collect_vec()
    }

    fn component(&self) -> Self::Component {
        MemoryComponent {
            log_n_rows: LOG_MEMORY_ADDRESS_BOUND,
        }
    }
}

impl Component for MemoryComponent {
    fn n_constraints(&self) -> usize {
        4
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        LOG_MEMORY_ADDRESS_BOUND + 1
    }

    fn trace_log_degree_bounds(&self) -> TreeVec<ColumnVec<u32>> {
        TreeVec::new(vec![
            vec![self.log_n_rows; self.n_columns()],
            vec![self.log_n_rows; SECURE_EXTENSION_DEGREE * (1 + N_M31_IN_FELT252)],
        ])
    }

    fn mask_points(
        &self,
        point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        let domain = CanonicCoset::new(self.log_n_rows);
        let mut interaction_points =
            vec![vec![point, point - domain.step().into_ef()]; SECURE_EXTENSION_DEGREE];
        interaction_points.append(&mut vec![
            vec![point];
            SECURE_EXTENSION_DEGREE * (N_M31_IN_FELT252 - 1)
        ]);
        interaction_points.append(&mut vec![
            vec![point, point - domain.step().into_ef()];
            SECURE_EXTENSION_DEGREE
        ]);
        TreeVec::new(vec![
            fixed_mask_points(&vec![vec![0_usize]; self.n_columns()], point),
            interaction_points,
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
        // TODO(AlonH): Add constraints to the range check interaction columns.
        // First lookup point boundary constraint.
        let constraint_zero_domain = CanonicCoset::new(self.log_n_rows).coset;
        let (alpha, z, rc_z) = (
            interaction_elements[MEMORY_ALPHA],
            interaction_elements[MEMORY_Z],
            interaction_elements[RC_Z],
        );
        let value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][0]));
        let address_and_value: [SecureField; N_M31_IN_FELT252 + 1] =
            std::array::from_fn(|i| mask[BASE_TRACE][i][0]);
        let numerator = value * shifted_secure_combination(&address_and_value, alpha, z)
            - mask[BASE_TRACE][MULTIPLICITY_COLUMN_OFFSET][0];
        let denom = point_vanishing(constraint_zero_domain.at(0), point);
        evaluation_accumulator.accumulate(numerator / denom);

        // Last lookup point boundary constraint.
        let lookup_value = SecureField::from_m31(
            lookup_values[MEMORY_LOOKUP_VALUE_0],
            lookup_values[MEMORY_LOOKUP_VALUE_1],
            lookup_values[MEMORY_LOOKUP_VALUE_2],
            lookup_values[MEMORY_LOOKUP_VALUE_3],
        );
        let numerator = value - lookup_value;
        let denom = point_vanishing(
            constraint_zero_domain.at(constraint_zero_domain.size() - 1),
            point,
        );
        evaluation_accumulator.accumulate(numerator / denom);

        // Lookup step constraint.
        let prev_value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][1]));
        let numerator = (value - prev_value)
            * shifted_secure_combination(&address_and_value, alpha, z)
            - mask[BASE_TRACE][MULTIPLICITY_COLUMN_OFFSET][0];
        let denom = coset_vanishing(constraint_zero_domain, point)
            / point_excluder(constraint_zero_domain.at(0), point);
        evaluation_accumulator.accumulate(numerator / denom);

        // First range check lookup column constraint.
        let rc_value = SecureField::from_partial_evals(std::array::from_fn(|i| {
            mask[INTERACTION_TRACE][i + SECURE_EXTENSION_DEGREE][0]
        }));
        let numerator = rc_value * (rc_z - address_and_value[1]) - BaseField::one();
        let denom = coset_vanishing(constraint_zero_domain, point);
        evaluation_accumulator.accumulate(numerator / denom);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::register_test_memory;

    #[test]
    fn test_memory_trace() {
        let mut registry = ComponentGenerationRegistry::default();
        register_test_memory(&mut registry);
        let trace = MemoryTraceGenerator::write_trace(MEMORY_COMPONENT_ID, &mut registry);
        let alpha = SecureField::from_u32_unchecked(1, 2, 3, 117);
        let z = SecureField::from_u32_unchecked(2, 3, 4, 118);
        let rc_z = SecureField::from_u32_unchecked(3, 4, 5, 119);
        let interaction_elements = InteractionElements::new(
            [
                (MEMORY_ALPHA.to_string(), alpha),
                (MEMORY_Z.to_string(), z),
                (RC_Z.to_string(), rc_z),
            ]
            .into(),
        );
        let interaction_trace = registry
            .get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID)
            .write_interaction_trace(&trace.iter().collect(), &interaction_elements);

        let mut expected_logup_sum = SecureField::zero();
        for i in 0..MEMORY_ADDRESS_BOUND {
            assert_eq!(trace[0].values[i], BaseField::from_u32_unchecked(i as u32));
            expected_logup_sum += trace.last().unwrap().values[i]
                / shifted_secure_combination(
                    &[BaseField::from_u32_unchecked(i as u32); N_M31_IN_FELT252 + 1],
                    alpha,
                    z,
                );
        }
        let logup_sum =
            SecureField::from_m31_array(std::array::from_fn(|j| interaction_trace[j][1]));

        assert_eq!(logup_sum, expected_logup_sum);
    }
}

use itertools::{zip_eq, Itertools};
use num_traits::Zero;
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

pub const MEMORY_ALPHA: &str = "MEMORY_ALPHA";
pub const MEMORY_Z: &str = "MEMORY_Z";
pub const MEMORY_COMPONENT_ID: &str = "MEMORY";
pub const MEMORY_LOOKUP_VALUE_0: &str = "MEMORY_LOOKUP_0";
pub const MEMORY_LOOKUP_VALUE_1: &str = "MEMORY_LOOKUP_1";
pub const MEMORY_LOOKUP_VALUE_2: &str = "MEMORY_LOOKUP_2";
pub const MEMORY_LOOKUP_VALUE_3: &str = "MEMORY_LOOKUP_3";

pub const N_M31_IN_FELT252: usize = 21;
pub const MULTIPLICITY_COLUMN: usize = 22;
// TODO(AlonH): Make memory size configurable.
pub const LOG_MEMORY_ADDRESS_BOUND: u32 = 3;
pub const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

/// Addresses are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value contain 12 bits).
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
        N_M31_IN_FELT252 + 2
    }
}

impl MemoryTraceGenerator {
    pub fn new(_path: String) -> Self {
        // TODO(AlonH): change to read from file.
        let values = vec![[BaseField::zero(); N_M31_IN_FELT252]; MEMORY_ADDRESS_BOUND];
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
                trace[j + 1][i] = BaseField::from_u32_unchecked(value.0);
            }
            trace[MULTIPLICITY_COLUMN][i] = BaseField::from_u32_unchecked(*multiplicity);
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
        let (alpha, z) = (elements[MEMORY_ALPHA], elements[MEMORY_Z]);

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
        let mut last = SecureField::zero();
        let log_size = interaction_trace_domain.log_size();
        for i in 0..domain_size {
            let index = bit_reverse_index(
                coset_order_to_circle_domain_order_index(i, log_size),
                log_size,
            );
            let interaction_value =
                last + (denom_inverses[index] * trace[MULTIPLICITY_COLUMN].values[index]);
            logup_values[index] = interaction_value;
            last = interaction_value;
        }
        let secure_column: SecureColumn<CpuBackend> = logup_values.into_iter().collect();
        secure_column
            .columns
            .into_iter()
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
        3
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        LOG_MEMORY_ADDRESS_BOUND + 1
    }

    fn trace_log_degree_bounds(&self) -> TreeVec<ColumnVec<u32>> {
        TreeVec::new(vec![
            vec![self.log_n_rows; self.n_columns()],
            vec![self.log_n_rows; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn mask_points(
        &self,
        point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        let domain = CanonicCoset::new(self.log_n_rows);
        TreeVec::new(vec![
            fixed_mask_points(&vec![vec![0_usize]; self.n_columns()], point),
            vec![vec![point, point - domain.step().into_ef()]; SECURE_EXTENSION_DEGREE],
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
        // First lookup point boundary constraint.
        let constraint_zero_domain = CanonicCoset::new(self.log_n_rows).coset;
        let (alpha, z) = (
            interaction_elements[MEMORY_ALPHA],
            interaction_elements[MEMORY_Z],
        );
        let value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][0]));
        let address_and_value: [SecureField; N_M31_IN_FELT252 + 1] =
            std::array::from_fn(|i| mask[BASE_TRACE][i][0]);
        let numerator = value * shifted_secure_combination(&address_and_value, alpha, z)
            - mask[BASE_TRACE][MULTIPLICITY_COLUMN][0];
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
        let denom = point_vanishing(constraint_zero_domain.at(1), point);
        evaluation_accumulator.accumulate(numerator / denom);

        // Lookup step constraint.
        let prev_value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][1]));
        let numerator = (value - prev_value)
            * shifted_secure_combination(&address_and_value, alpha, z)
            - mask[BASE_TRACE][22][0];
        let denom = coset_vanishing(constraint_zero_domain, point)
            / point_excluder(constraint_zero_domain.at(0), point);
        evaluation_accumulator.accumulate(numerator / denom);
    }
}

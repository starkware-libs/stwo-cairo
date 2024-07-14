use itertools::{zip_eq, Itertools};
use num_traits::Zero;
use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::mask::fixed_mask_points;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::circle::CirclePoint;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{ComponentGen, ComponentTraceGenerator};

const N_M31_IN_FELT252: usize = 21;
const LOG_MEMORY_ADDRESS_BOUND: u32 = 20;
const MEMORY_ADDRESS_BOUND: usize = 1 << LOG_MEMORY_ADDRESS_BOUND;

/// Addresses are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value contain 12 bits).
pub struct MemoryTraceGenerator {
    // TODO(AlonH): Consider to change values to be Felt252.
    pub values: Vec<[M31; N_M31_IN_FELT252]>,
    pub multiplicities: Vec<u32>,
}

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
        let values = vec![[M31::zero(); N_M31_IN_FELT252]; MEMORY_ADDRESS_BOUND];
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
    type Inputs = M31;

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
            trace[22][i] = BaseField::from_u32_unchecked(*multiplicity);
        }

        let domain = CanonicCoset::new(LOG_MEMORY_ADDRESS_BOUND).circle_domain();
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<CpuBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec()
    }

    fn write_interaction_trace(
        &self,
        _trace: &ColumnVec<&CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>,
        _elements: &InteractionElements,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        todo!()
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
        _point: CirclePoint<SecureField>,
        _mask: &TreeVec<Vec<Vec<SecureField>>>,
        _evaluation_accumulator: &mut PointEvaluationAccumulator,
        _interaction_elements: &InteractionElements,
        _lookup_values: &LookupValues,
    ) {
        todo!()
    }
}

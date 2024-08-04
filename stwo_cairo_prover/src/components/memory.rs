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
use stwo_prover::core::{ColumnVec, InteractionElements};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{ComponentGen, TraceGenerator};

const MEMORY_ADDRESS_BOUND: usize = 1 << 24;

/// Addresses are continous and start from 0.
/// Values are Felt252 stored as 21 M31 values (each value contain 12 bits).
pub struct MemoryTraceGenerator {
    // TODO: Consider to change values to be Felt252.
    pub values: Vec<[M31; 21]>,
    pub multiplicities: Vec<u32>,
}

pub struct MemoryComponent {
    pub log_n_instances: usize,
}

impl MemoryComponent {
    pub fn n_columns(&self) -> usize {
        21 + 1
    }
}

impl MemoryTraceGenerator {
    pub fn new(_path: String) -> Self {
        // TODO: change to read from file.
        let values = vec![[M31::zero(); 21]; MEMORY_ADDRESS_BOUND];
        let multiplicities = vec![0; MEMORY_ADDRESS_BOUND];
        Self {
            values,
            multiplicities,
        }
    }
}

impl ComponentGen for MemoryTraceGenerator {}

impl TraceGenerator<CpuBackend> for MemoryTraceGenerator {
    type Component = MemoryComponent;
    type Inputs = M31;

    fn add_inputs(&mut self, inputs: &Self::Inputs) {
        self.multiplicities[inputs.0 as usize] += 1;
    }

    fn write_trace(
        component_id: &str,
        registry: &mut ComponentGenerationRegistry,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>> {
        let memory_trace_generator = registry.get_generator::<MemoryTraceGenerator>(component_id);

        let mut trace = vec![
            vec![BaseField::zero(); MEMORY_ADDRESS_BOUND];
            memory_trace_generator.component().n_columns()
        ];
        for (i, (values, multiplicity)) in zip_eq(
            &memory_trace_generator.values,
            &memory_trace_generator.multiplicities,
        )
        .enumerate()
        {
            for (j, value) in values.iter().enumerate() {
                trace[j][i] = BaseField::from_u32_unchecked(value.0);
            }
            trace[21][i] = BaseField::from_u32_unchecked(*multiplicity);
        }

        let domain =
            CanonicCoset::new(MEMORY_ADDRESS_BOUND.checked_ilog2().unwrap()).circle_domain();
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<CpuBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec()
    }

    fn component(&self) -> Self::Component {
        MemoryComponent {
            log_n_instances: MEMORY_ADDRESS_BOUND.checked_ilog2().unwrap() as usize,
        }
    }
}

impl Component for MemoryComponent {
    fn n_constraints(&self) -> usize {
        3
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        24 + 1
    }

    fn n_interaction_phases(&self) -> u32 {
        2
    }

    fn trace_log_degree_bounds(&self) -> TreeVec<ColumnVec<u32>> {
        TreeVec::new(vec![
            vec![self.log_n_instances as u32; self.n_columns()],
            vec![self.log_n_instances as u32; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn mask_points(
        &self,
        point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        TreeVec::new(vec![
            fixed_mask_points(&vec![vec![0_usize]; self.n_columns()], point),
            vec![vec![point]; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn interaction_element_ids(&self) -> Vec<String> {
        todo!()
    }

    fn evaluate_constraint_quotients_at_point(
        &self,
        _point: CirclePoint<SecureField>,
        _mask: &ColumnVec<Vec<SecureField>>,
        _evaluation_accumulator: &mut PointEvaluationAccumulator,
        _interaction_elements: &InteractionElements,
    ) {
        todo!()
    }
}

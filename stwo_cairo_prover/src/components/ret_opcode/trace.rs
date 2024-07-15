use itertools::Itertools;
use num_traits::{One, Zero};
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SecureColumn;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{
    bit_reverse_index, coset_order_to_circle_domain_order_index, shifted_secure_combination,
};
use stwo_prover::core::{ColumnVec, InteractionElements};
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{ComponentGen, ComponentTraceGenerator};

use super::component::RetOpcode;
use crate::components::memory::component::{
    MemoryTraceGenerator, MEMORY_ALPHA, MEMORY_COMPONENT_ID, MEMORY_Z, N_M31_IN_FELT252,
};

pub struct RetOpcodeCpuTraceGenerator {
    pub inputs: Vec<[M31; 3]>,
}
impl ComponentGen for RetOpcodeCpuTraceGenerator {}

impl ComponentTraceGenerator<CpuBackend> for RetOpcodeCpuTraceGenerator {
    type Component = RetOpcode;
    type Inputs = Vec<[M31; 3]>;

    fn write_trace(
        component_id: &str,
        registry: &mut ComponentGenerationRegistry,
    ) -> Vec<CpuCircleEvaluation<M31, BitReversedOrder>> {
        let generator = registry.get_generator::<Self>(component_id);
        let memory_trace_generator =
            registry.get_generator::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID);
        let (trace, sub_component_inputs) = write_trace_cpu(
            &generator.component(),
            &generator.inputs,
            memory_trace_generator,
        );
        let trace_generator =
            registry.get_generator_mut::<MemoryTraceGenerator>(MEMORY_COMPONENT_ID);
        sub_component_inputs.memory_inputs.iter().for_each(|input| {
            trace_generator.add_inputs(input);
        });
        trace
    }

    fn add_inputs(&mut self, inputs: &Self::Inputs) {
        self.inputs.extend(inputs);
    }

    fn component(&self) -> RetOpcode {
        // TODO(Ohad): Support non-power of 2 inputs.
        RetOpcode {
            log_n_instances: self
                .inputs
                .len()
                .checked_ilog2()
                .expect("Input not a power of 2!"),
        }
    }

    fn write_interaction_trace(
        &self,
        trace: &ColumnVec<&CircleEvaluation<CpuBackend, M31, BitReversedOrder>>,
        elements: &InteractionElements,
    ) -> ColumnVec<CircleEvaluation<CpuBackend, M31, BitReversedOrder>> {
        let interaction_trace_domain = trace[0].domain;
        let domain_size = interaction_trace_domain.size();
        let log_domain_size = interaction_trace_domain.log_size();
        let (memory_alpha, memory_z) = (elements[MEMORY_ALPHA], elements[MEMORY_Z]);

        // PC Column.
        let pc_column = &trace[0].values;
        let denoms = pc_column
            .iter()
            .copied()
            .map(|pc| {
                let mut address_and_value = [M31::zero(); N_M31_IN_FELT252 + 1];
                address_and_value[0] = pc;
                address_and_value[1] = M31::one();
                shifted_secure_combination(&address_and_value, memory_alpha, memory_z)
            })
            .collect_vec();
        let mut denom_inverses = vec![SecureField::zero(); domain_size];
        SecureField::batch_inverse(&denoms, &mut denom_inverses);

        let mut logup_values = vec![SecureField::zero(); domain_size];
        let mut prefix_sum = SecureField::zero();
        for i in 0..domain_size {
            let index = bit_reverse_index(
                coset_order_to_circle_domain_order_index(i, log_domain_size),
                log_domain_size,
            );
            prefix_sum += denom_inverses[index];
            logup_values[index] = prefix_sum;
        }

        let secure_column: SecureColumn<CpuBackend> = logup_values.into_iter().collect();
        secure_column
            .columns
            .into_iter()
            .map(|eval| CircleEvaluation::new(interaction_trace_domain, eval))
            .collect_vec()
    }
}

#[allow(non_snake_case)]
pub struct ReturnedInputs {
    pub memory_inputs: Vec<M31>,
}

impl ReturnedInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self {
            memory_inputs: Vec::with_capacity(capacity * 3),
        }
    }
}

pub fn write_trace_cpu(
    component: &RetOpcode,
    inputs: &[[M31; 3]],
    memory_trace_generator: &MemoryTraceGenerator,
) -> (
    Vec<CpuCircleEvaluation<M31, BitReversedOrder>>,
    ReturnedInputs,
) {
    let n_trace_columns = component.trace_log_degree_bounds()[0].len();
    let mut trace_values = vec![vec![M31::zero(); inputs.len()]; n_trace_columns];
    let mut sub_components_inputs = ReturnedInputs::with_capacity(inputs.len());
    inputs.iter().enumerate().for_each(|(i, secret)| {
        write_trace_row(
            &mut trace_values,
            *secret,
            i,
            &mut sub_components_inputs,
            memory_trace_generator,
        );
    });

    let trace = trace_values
        .into_iter()
        .map(|eval| {
            // TODO(Ohad): Support non-power of 2 inputs.
            let domain = CanonicCoset::new(
                eval.len()
                    .checked_ilog2()
                    .expect("Input is not a power of 2!"),
            )
            .circle_domain();
            CpuCircleEvaluation::<M31, BitReversedOrder>::new(domain, eval)
        })
        .collect_vec();

    (trace, sub_components_inputs)
}

// Ret trace row:
// | pc | ap | fp | [fp-1].0 | [fp-1].1 | [fp-2].0 | [fp-2].1 |
// TODO(Ohad): redo when air team decides how it should look.
fn write_trace_row(
    dst: &mut [Vec<M31>],
    ret_opcode_input: [M31; 3],
    row_index: usize,
    returned_inputs: &mut ReturnedInputs,
    memory_trace_generator: &MemoryTraceGenerator,
) {
    let deduction_tmp_0 = [
        ret_opcode_input[0],
        ret_opcode_input[1],
        ret_opcode_input[2],
    ];
    let col0 = deduction_tmp_0[0];
    dst[0][row_index] = col0;
    let col1 = deduction_tmp_0[1];
    dst[1][row_index] = col1;
    let col2 = deduction_tmp_0[2];
    dst[2][row_index] = col2;
    returned_inputs.memory_inputs.push(col0);
    // TODO(Ohad): implement and uncomment.
    // returned_inputs.memory_inputs.push((col2) - (M31::from(1)));
    let mem_fp_minus_one = memory_trace_generator.deduce_output((col2) - (M31::from(1)));
    let col3 = mem_fp_minus_one[0];
    dst[3][row_index] = col3;
    let col4 = mem_fp_minus_one[1];
    dst[4][row_index] = col4;
    // returned_inputs.memory_inputs.push((col2) - (M31::from(2)));
    let mem_fp_minus_two = memory_trace_generator.deduce_output((col2) - (M31::from(2)));
    let col5 = mem_fp_minus_two[0];
    dst[5][row_index] = col5;
    let col6 = mem_fp_minus_two[1];
    dst[6][row_index] = col6;
}

#![allow(unused_imports)]
use air_infra::core::prover_types::*;
use itertools::Itertools;
use num_traits::Zero;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::cpu::CpuCircleEvaluation;
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::trace_generation::registry::ComponentGenerationRegistry;
use stwo_prover::trace_generation::{ComponentGen, TraceGenerator};

use super::component::RetOpcode;

#[allow(non_camel_case_types)]
#[derive(Default)]
pub struct RetOpcodeCpuTraceGenerator {
    pub inputs: Vec<[M31; 3]>,
}
impl ComponentGen for RetOpcodeCpuTraceGenerator {}

impl TraceGenerator<CpuBackend> for RetOpcodeCpuTraceGenerator {
    type Component = RetOpcode;
    type Inputs = Vec<[M31; 3]>;

    fn write_trace(
        component_id: &str,
        registry: &mut ComponentGenerationRegistry,
    ) -> Vec<CpuCircleEvaluation<M31, BitReversedOrder>> {
        let generator = registry.get_generator::<Self>(component_id);
        #[allow(unused_variables)]
        let (trace, sub_component_inputs) =
            write_trace_cpu(&generator.component(), &generator.inputs);
        let sub_component_i = registry
            .get_generator_mut::<Memory<M31, Felt252>>(
                "Memory__FeltExpr__Felt252Expr",
            );
        sub_component_i.add_inputs(&sub_component_inputs.0);
        let sub_component_i = registry
            .get_generator_mut::<Memory<M31, Felt252>>(
                "Memory__FeltExpr__Felt252Expr",
            );
        sub_component_i.add_inputs(&sub_component_inputs.1);
        let sub_component_i = registry
            .get_generator_mut::<Memory<M31, Felt252>>(
                "Memory__FeltExpr__Felt252Expr",
            );
        sub_component_i.add_inputs(&sub_component_inputs.2);
        trace
    }

    fn add_inputs(&mut self, inputs: &Self::Inputs) {
        self.inputs.extend(inputs);
    }

    fn component(&self) -> RetOpcode {
        RetOpcode {
            log_n_instances: self
                .inputs
                .len()
                .checked_ilog2()
                .expect("Input not a power of 2!"),
        }
    }
}

pub struct ReturnedInputs(pub Vec<M31>, pub Vec<M31>, pub Vec<M31>);

impl ReturnedInputs {
    #[allow(unused_variables)]
    fn with_capacity(capacity: usize) -> Self {
        Self(
            Vec::with_capacity(capacity),
            Vec::with_capacity(capacity),
            Vec::with_capacity(capacity),
        )
    }
}

#[allow(clippy::ptr_arg)]
#[allow(clippy::type_complexity)]
#[allow(clippy::let_unit_value)]
pub fn write_trace_cpu(
    component: &RetOpcode,
    secrets: &Vec<[M31; 3]>,
) -> (
    Vec<CpuCircleEvaluation<M31, BitReversedOrder>>,
    ReturnedInputs,
) {
    let n_trace_columns = component.trace_log_degree_bounds()[0].len();
    let mut trace_values = vec![vec![M31::zero(); secrets.len()]; n_trace_columns];
    let mut sub_components_inputs = ReturnedInputs::with_capacity(secrets.len());
    secrets.iter().enumerate().for_each(|(i, secret)| {
        write_trace_row(&mut trace_values, *secret, i, &mut sub_components_inputs)
    });

    let trace = trace_values
        .into_iter()
        .map(|eval| {
            let domain =
                CanonicCoset::new(eval.len().checked_ilog2().expect("Input not a power of 2!"))
                    .circle_domain();
            CpuCircleEvaluation::<M31, BitReversedOrder>::new(domain, eval)
        })
        .collect_vec();

    (trace, sub_components_inputs)
}

#[allow(non_snake_case)]
#[allow(clippy::useless_conversion)]
#[allow(clippy::type_complexity)]
fn write_trace_row(
    dst: &mut [Vec<M31>],
    RetOpcode_input: [M31; 3],
    row_index: usize,
    #[allow(unused_variables)] returned_inputs: &mut ReturnedInputs,
) {
    let deduction_tmp_0 = [RetOpcode_input[0], RetOpcode_input[1], RetOpcode_input[2]];
    let col0 = deduction_tmp_0[0];
    dst[0][row_index] = col0.into();
    let col1 = deduction_tmp_0[1];
    dst[1][row_index] = col1.into();
    let col2 = deduction_tmp_0[2];
    dst[2][row_index] = col2.into();
    returned_inputs.0.push(col0);
    let deduction_tmp_2 = Memory<M31, Felt252>::deduce_output(col0);
    returned_inputs.1.push((col2) - (M31::from(1)));
    let deduction_tmp_4 =
        Memory<M31, Felt252>::deduce_output((col2) - (M31::from(1)));
    let col3 = deduction_tmp_4.get_felt(usize::from(0));
    dst[3][row_index] = col3.into();
    let col4 = deduction_tmp_4.get_felt(usize::from(1));
    dst[4][row_index] = col4.into();
    returned_inputs.2.push((col2) - (M31::from(2)));
    let deduction_tmp_5 =
        Memory<M31, Felt252>::deduce_output((col2) - (M31::from(2)));
    let col5 = deduction_tmp_5.get_felt(usize::from(0));
    dst[5][row_index] = col5.into();
    let col6 = deduction_tmp_5.get_felt(usize::from(1));
    dst[6][row_index] = col6.into();
}

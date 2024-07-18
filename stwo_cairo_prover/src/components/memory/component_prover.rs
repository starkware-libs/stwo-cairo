use std::collections::BTreeMap;

use num_traits::{One, Zero};
use stwo_prover::core::air::accumulation::DomainEvaluationAccumulator;
use stwo_prover::core::air::{Component, ComponentProver, ComponentTrace};
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::constraints::coset_vanishing;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::utils::{bit_reverse, shifted_secure_combination};
use stwo_prover::core::{InteractionElements, LookupValues};
use stwo_prover::trace_generation::{BASE_TRACE, INTERACTION_TRACE};

use super::component::{
    MemoryComponent, MEMORY_ALPHA, MEMORY_LOOKUP_VALUE_0, MEMORY_LOOKUP_VALUE_1,
    MEMORY_LOOKUP_VALUE_2, MEMORY_LOOKUP_VALUE_3, MEMORY_Z, MULTIPLICITY_COLUMN_OFFSET,
    N_M31_IN_FELT252,
};
use crate::components::range_check_unit::component::RC_Z;

impl ComponentProver<CpuBackend> for MemoryComponent {
    fn evaluate_constraint_quotients_on_domain(
        &self,
        trace: &ComponentTrace<'_, CpuBackend>,
        evaluation_accumulator: &mut DomainEvaluationAccumulator<CpuBackend>,
        interaction_elements: &InteractionElements,
        lookup_values: &LookupValues,
    ) {
        let max_constraint_degree = self.max_constraint_log_degree_bound();
        let trace_eval_domain = CanonicCoset::new(max_constraint_degree).circle_domain();
        let trace_evals = &trace.evals;
        let zero_domain = CanonicCoset::new(self.log_n_rows).coset;
        let [mut accum] =
            evaluation_accumulator.columns([(max_constraint_degree, self.n_constraints())]);

        let mut denoms = vec![];
        for point in trace_eval_domain.iter() {
            denoms.push(coset_vanishing(zero_domain, point));
        }
        bit_reverse(&mut denoms);
        let mut denom_inverses = vec![BaseField::zero(); 1 << (max_constraint_degree)];
        BaseField::batch_inverse(&denoms, &mut denom_inverses);
        let (alpha, z, rc_z) = (
            interaction_elements[MEMORY_ALPHA],
            interaction_elements[MEMORY_Z],
            interaction_elements[RC_Z],
        );

        let _lookup_value = SecureField::from_m31(
            lookup_values[MEMORY_LOOKUP_VALUE_0],
            lookup_values[MEMORY_LOOKUP_VALUE_1],
            lookup_values[MEMORY_LOOKUP_VALUE_2],
            lookup_values[MEMORY_LOOKUP_VALUE_3],
        );
        for (i, denom_inverse) in denom_inverses.iter().enumerate() {
            let value = SecureField::from_m31_array(std::array::from_fn(|j| {
                trace_evals[INTERACTION_TRACE][j][i]
            }));
            let address_and_value: [BaseField; N_M31_IN_FELT252 + 1] =
                std::array::from_fn(|j| trace_evals[BASE_TRACE][j][i]);

            // First interaction column constraint.
            let mut numerator = accum.random_coeff_powers[N_M31_IN_FELT252 - 1]
                * (value * shifted_secure_combination(&address_and_value, alpha, z)
                    - trace_evals[BASE_TRACE][MULTIPLICITY_COLUMN_OFFSET][i]);

            // Middle interaction columns constraints.
            let mut prev_row_value = value;
            #[allow(clippy::needless_range_loop)]
            for j in 1..N_M31_IN_FELT252 {
                let value = SecureField::from_m31_array(std::array::from_fn(|k| {
                    trace_evals[INTERACTION_TRACE][j * SECURE_EXTENSION_DEGREE + k][i]
                }));
                numerator += accum.random_coeff_powers[N_M31_IN_FELT252 - j - 1]
                    * ((value - prev_row_value) * (rc_z - address_and_value[j]) - BaseField::one());
                prev_row_value = value;
            }

            accum.accumulate(i, numerator * *denom_inverse);
        }
    }

    fn lookup_values(&self, trace: &ComponentTrace<'_, CpuBackend>) -> LookupValues {
        let domain = CanonicCoset::new(self.log_n_rows);
        let trace_poly = &trace.polys[INTERACTION_TRACE];
        let values = BTreeMap::from_iter([
            (
                MEMORY_LOOKUP_VALUE_0.to_string(),
                trace_poly[4 * N_M31_IN_FELT252]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                MEMORY_LOOKUP_VALUE_1.to_string(),
                trace_poly[4 * N_M31_IN_FELT252 + 1]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                MEMORY_LOOKUP_VALUE_2.to_string(),
                trace_poly[4 * N_M31_IN_FELT252 + 2]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                MEMORY_LOOKUP_VALUE_3.to_string(),
                trace_poly[4 * N_M31_IN_FELT252 + 3]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
        ]);
        LookupValues::new(values)
    }
}

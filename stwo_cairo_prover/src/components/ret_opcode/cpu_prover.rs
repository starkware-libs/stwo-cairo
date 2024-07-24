#![allow(unused_imports)]
use std::collections::BTreeMap;

use itertools::izip;
use num_traits::identities::Zero;
use num_traits::One;
use stwo_prover::core::air::accumulation::DomainEvaluationAccumulator;
use stwo_prover::core::air::{Component, ComponentProver, ComponentTrace};
use stwo_prover::core::backend::{Column, CpuBackend};
use stwo_prover::core::constraints::{coset_vanishing, point_excluder};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::utils::{
    bit_reverse, point_vanish_denominator_inverses, previous_bit_reversed_circle_domain_index,
    shifted_secure_combination,
};
use stwo_prover::core::{InteractionElements, LookupValues};
use stwo_prover::trace_generation::{BASE_TRACE, INTERACTION_TRACE};

use super::component::{
    RetOpcode, RET_LOOKUP_VALUE_0, RET_LOOKUP_VALUE_1, RET_LOOKUP_VALUE_2, RET_LOOKUP_VALUE_3,
};
use crate::components::memory::component::{MEMORY_ALPHA, MEMORY_Z, N_M31_IN_FELT252};

impl ComponentProver<CpuBackend> for RetOpcode {
    #[allow(unused_parens)]
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
        let zero_domain = CanonicCoset::new(self.log_n_instances).coset;
        let [mut accum] =
            evaluation_accumulator.columns([(max_constraint_degree, self.n_constraints())]);

        // TODO(AlonH): Get all denominators in one loop and don't perform unnecessary inversions.
        let first_point_denom_inverses =
            point_vanish_denominator_inverses(trace_eval_domain, zero_domain.at(0));
        let last_point_denom_inverses = point_vanish_denominator_inverses(
            trace_eval_domain,
            zero_domain.at(zero_domain.size() - 1),
        );
        let mut step_denoms = vec![];
        for point in trace_eval_domain.iter() {
            step_denoms.push(
                coset_vanishing(zero_domain, point) / point_excluder(zero_domain.at(0), point),
            );
        }
        bit_reverse(&mut step_denoms);
        let mut step_denom_inverses = vec![BaseField::zero(); 1 << (max_constraint_degree)];
        BaseField::batch_inverse(&step_denoms, &mut step_denom_inverses);

        let (alpha, z) = (
            interaction_elements[MEMORY_ALPHA],
            interaction_elements[MEMORY_Z],
        );

        let lookup_value = SecureField::from_m31(
            lookup_values[RET_LOOKUP_VALUE_0],
            lookup_values[RET_LOOKUP_VALUE_1],
            lookup_values[RET_LOOKUP_VALUE_2],
            lookup_values[RET_LOOKUP_VALUE_3],
        );

        for (i, (first_point_denom_inverse, last_point_denom_inverse, step_denom_inverse)) in izip!(
            first_point_denom_inverses,
            last_point_denom_inverses,
            step_denom_inverses,
        )
        .enumerate()
        {
            // Value = InteractionPoly(i);
            let value = SecureField::from_m31_array(std::array::from_fn(|j| {
                trace_evals[INTERACTION_TRACE][j][i]
            }));

            // PrevValue = InteractionPoly(i - g);
            let prev_index = previous_bit_reversed_circle_domain_index(
                i,
                zero_domain.log_size,
                trace_eval_domain.log_size(),
            );
            let prev_value = SecureField::from_m31_array(std::array::from_fn(|j| {
                trace_evals[INTERACTION_TRACE][j][prev_index]
            }));

            // PC Column.
            let address = trace_evals[BASE_TRACE][0][i];
            let value_at_pc = BaseField::one();
            let mut address_and_value = [BaseField::zero(); N_M31_IN_FELT252 + 1];
            address_and_value[0] = address;
            address_and_value[1] = value_at_pc;

            // TODO(Ohad): add remaining lookup constraints.

            let first_point_numerator = accum.random_coeff_powers[2]
                * (value * shifted_secure_combination(&address_and_value, alpha, z)
                    - SecureField::one());
            let last_point_numerator = accum.random_coeff_powers[1] * (value - lookup_value);
            let step_numerator = accum.random_coeff_powers[0]
                * ((value - prev_value) * shifted_secure_combination(&address_and_value, alpha, z)
                    - SecureField::from_u32_unchecked(1, 0, 0, 0));
            accum.accumulate(
                i,
                first_point_numerator * first_point_denom_inverse
                    + last_point_numerator * last_point_denom_inverse
                    + step_numerator * step_denom_inverse,
            );
        }
    }

    fn lookup_values(&self, trace: &ComponentTrace<'_, CpuBackend>) -> LookupValues {
        let domain = CanonicCoset::new(self.log_n_instances);
        let trace_poly = &trace.polys[INTERACTION_TRACE];
        let values = BTreeMap::from_iter([
            (
                RET_LOOKUP_VALUE_0.to_string(),
                trace_poly[0]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                RET_LOOKUP_VALUE_1.to_string(),
                trace_poly[1]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                RET_LOOKUP_VALUE_2.to_string(),
                trace_poly[2]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                RET_LOOKUP_VALUE_3.to_string(),
                trace_poly[3]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
        ]);
        LookupValues::new(values)
    }
}

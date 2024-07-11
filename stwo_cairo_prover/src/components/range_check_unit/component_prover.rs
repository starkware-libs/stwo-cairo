use std::collections::BTreeMap;

use itertools::zip_eq;
use stwo_prover::core::air::accumulation::{ColumnAccumulator, DomainEvaluationAccumulator};
use stwo_prover::core::air::{Component, ComponentProver, ComponentTrace};
use stwo_prover::core::backend::CpuBackend;
use stwo_prover::core::circle::Coset;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleDomain, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::prover::{BASE_TRACE, INTERACTION_TRACE};
use stwo_prover::core::utils::point_vanish_denominator_inverses;
use stwo_prover::core::{InteractionElements, LookupValues};

use super::component::{
    RangeCheckUnitComponent, RC_LOOKUP_VALUE_0, RC_LOOKUP_VALUE_1, RC_LOOKUP_VALUE_2,
    RC_LOOKUP_VALUE_3, RC_Z,
};

impl RangeCheckUnitComponent {
    fn evaluate_lookup_boundary_constraints(
        &self,
        trace_evals: &TreeVec<Vec<&CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>>,
        trace_eval_domain: CircleDomain,
        zero_domain: Coset,
        accum: &mut ColumnAccumulator<'_, CpuBackend>,
        interaction_elements: &InteractionElements,
        lookup_values: &LookupValues,
    ) {
        let first_point_denom_inverses =
            point_vanish_denominator_inverses(trace_eval_domain, zero_domain.at(0));
        let last_point_denom_inverses = point_vanish_denominator_inverses(
            trace_eval_domain,
            zero_domain.at(zero_domain.size() - 1),
        );
        let z = interaction_elements[RC_Z];
        let lookup_value = SecureField::from_m31(
            lookup_values[RC_LOOKUP_VALUE_0],
            lookup_values[RC_LOOKUP_VALUE_1],
            lookup_values[RC_LOOKUP_VALUE_2],
            lookup_values[RC_LOOKUP_VALUE_3],
        );
        for (i, (first_point_denom_inverse, last_point_denom_inverse)) in
            zip_eq(first_point_denom_inverses, last_point_denom_inverses).enumerate()
        {
            let value = SecureField::from_m31_array(std::array::from_fn(|j| {
                trace_evals[INTERACTION_TRACE][j][i]
            }));
            let first_point_numerator = accum.random_coeff_powers[1]
                * (value * (z - trace_evals[BASE_TRACE][0][i]) - trace_evals[BASE_TRACE][1][i]);
            let last_point_numerator = accum.random_coeff_powers[0] * (value - lookup_value);
            accum.accumulate(
                i,
                first_point_numerator * first_point_denom_inverse
                    + last_point_numerator * last_point_denom_inverse,
            );
        }
    }
}

impl ComponentProver<CpuBackend> for RangeCheckUnitComponent {
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

        self.evaluate_lookup_boundary_constraints(
            trace_evals,
            trace_eval_domain,
            zero_domain,
            &mut accum,
            interaction_elements,
            lookup_values,
        );
    }

    fn lookup_values(&self, trace: &ComponentTrace<'_, CpuBackend>) -> LookupValues {
        let domain = CanonicCoset::new(self.log_n_instances);
        let trace_poly = &trace.polys[INTERACTION_TRACE];
        let values = BTreeMap::from_iter([
            (
                RC_LOOKUP_VALUE_0.to_string(),
                trace_poly[0]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                RC_LOOKUP_VALUE_1.to_string(),
                trace_poly[1]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                RC_LOOKUP_VALUE_2.to_string(),
                trace_poly[2]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
            (
                RC_LOOKUP_VALUE_3.to_string(),
                trace_poly[3]
                    .eval_at_point(domain.at(domain.size() - 1).into_ef())
                    .try_into()
                    .unwrap(),
            ),
        ]);
        LookupValues::new(values)
    }
}

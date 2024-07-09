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

use super::component::{RangeCheckUnitComponent, RC_Z};

impl RangeCheckUnitComponent {
    fn evaluate_lookup_boundary_constraints(
        &self,
        trace_evals: &TreeVec<Vec<&CircleEvaluation<CpuBackend, BaseField, BitReversedOrder>>>,
        trace_eval_domain: CircleDomain,
        zero_domain: Coset,
        accum: &mut ColumnAccumulator<'_, CpuBackend>,
        interaction_elements: &InteractionElements,
        _lookup_values: &LookupValues,
    ) {
        let denom_inverses =
            point_vanish_denominator_inverses(trace_eval_domain, zero_domain.at(0));
        let z = interaction_elements[RC_Z];
        for (i, denom_inverse) in denom_inverses.iter().enumerate() {
            let value = SecureField::from_m31_array(std::array::from_fn(|j| {
                trace_evals[INTERACTION_TRACE][j][i]
            }));
            let numerator = accum.random_coeff_powers[0]
                * (value * (z - trace_evals[BASE_TRACE][0][i]) - trace_evals[BASE_TRACE][1][i]);
            accum.accumulate(i, numerator * *denom_inverse);
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

    fn lookup_values(&self, _trace: &ComponentTrace<'_, CpuBackend>) -> LookupValues {
        LookupValues::default()
    }
}

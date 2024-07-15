#![allow(unused_imports)]
use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::mask::fixed_mask_points;
use stwo_prover::core::air::Component;
use stwo_prover::core::circle::CirclePoint;
use stwo_prover::core::constraints::coset_vanishing;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::{ColumnVec, InteractionElements};

#[allow(non_camel_case_types)]
pub struct RetOpcode {
    pub log_n_instances: u32,
}

impl Component for RetOpcode {
    fn n_constraints(&self) -> usize {
        todo!()
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_n_instances + 1
    }

    fn trace_log_degree_bounds(&self) -> TreeVec<Vec<u32>> {
        TreeVec(vec![vec![self.log_n_instances; 7], vec![]])
    }

    fn mask_points(
        &self,
        point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        TreeVec(vec![
            fixed_mask_points(&vec![vec![0_usize]; 7], point),
            vec![],
        ])
    }
    
    fn evaluate_constraint_quotients_at_point(
        &self,
        point: CirclePoint<SecureField>,
        mask: &TreeVec<ColumnVec<Vec<SecureField>>>,
        evaluation_accumulator: &mut PointEvaluationAccumulator,
        interaction_elements: &InteractionElements,
        lookup_values: &stwo_prover::core::LookupValues,
    ) {
        todo!()
    }

}

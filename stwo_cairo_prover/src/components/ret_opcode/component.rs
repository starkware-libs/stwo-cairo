use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::mask::fixed_mask_points;
use stwo_prover::core::air::Component;
use stwo_prover::core::circle::CirclePoint;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};

pub const RET_N_TRACE_CELLS: usize = 7;

pub const RET_COMPONENT_ID: &str = "RET";
pub const RET_LOOKUP_VALUE_0: &str = "RET_LOOKUP_0";
pub const RET_LOOKUP_VALUE_1: &str = "RET_LOOKUP_1";
pub const RET_LOOKUP_VALUE_2: &str = "RET_LOOKUP_2";
pub const RET_LOOKUP_VALUE_3: &str = "RET_LOOKUP_3";

#[derive(Clone)]
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
        TreeVec(vec![
            vec![self.log_n_instances; RET_N_TRACE_CELLS],
            vec![self.log_n_instances; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn mask_points(
        &self,
        point: CirclePoint<SecureField>,
    ) -> TreeVec<ColumnVec<Vec<CirclePoint<SecureField>>>> {
        let domain = CanonicCoset::new(self.log_n_instances);
        TreeVec(vec![
            fixed_mask_points(&vec![vec![0_usize]; RET_N_TRACE_CELLS], point),
            vec![vec![point, point - domain.step().into_ef()]; SECURE_EXTENSION_DEGREE],
        ])
    }

    fn evaluate_constraint_quotients_at_point(
        &self,
        _point: CirclePoint<SecureField>,
        _mask: &TreeVec<ColumnVec<Vec<SecureField>>>,
        _evaluation_accumulator: &mut PointEvaluationAccumulator,
        _interaction_elements: &InteractionElements,
        _lookup_values: &LookupValues,
    ) {
        todo!()
    }
}

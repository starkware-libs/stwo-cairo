use num_traits::{One, Zero};
use stwo_prover::core::air::accumulation::PointEvaluationAccumulator;
use stwo_prover::core::air::mask::fixed_mask_points;
use stwo_prover::core::air::Component;
use stwo_prover::core::circle::CirclePoint;
use stwo_prover::core::constraints::{coset_vanishing, point_excluder, point_vanishing};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::core::poly::circle::CanonicCoset;
use stwo_prover::core::utils::shifted_secure_combination;
use stwo_prover::core::{ColumnVec, InteractionElements, LookupValues};
use stwo_prover::trace_generation::{BASE_TRACE, INTERACTION_TRACE};

use crate::components::memory::component::{MEMORY_ALPHA, MEMORY_Z, N_M31_IN_FELT252};

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
        3
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
        point: CirclePoint<SecureField>,
        mask: &TreeVec<ColumnVec<Vec<SecureField>>>,
        evaluation_accumulator: &mut PointEvaluationAccumulator,
        interaction_elements: &InteractionElements,
        lookup_values: &LookupValues,
    ) {
        // First lookup point boundary constraint.
        let constraint_zero_domain = CanonicCoset::new(self.log_n_instances).coset;
        let (alpha, z) = (
            interaction_elements[MEMORY_ALPHA],
            interaction_elements[MEMORY_Z],
        );
        let value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][0]));
        let address = mask[BASE_TRACE][0][0];
        let value_at_pc = SecureField::one();
        let mut address_and_value = [SecureField::zero(); N_M31_IN_FELT252 + 1];
        address_and_value[0] = address;
        address_and_value[1] = value_at_pc;

        let numerator =
            value * shifted_secure_combination(&address_and_value, alpha, z) - SecureField::one();
        let denom = point_vanishing(constraint_zero_domain.at(0), point);
        evaluation_accumulator.accumulate(numerator / denom);

        // Last lookup point boundary constraint.
        let lookup_value = SecureField::from_m31(
            lookup_values[RET_LOOKUP_VALUE_0],
            lookup_values[RET_LOOKUP_VALUE_1],
            lookup_values[RET_LOOKUP_VALUE_2],
            lookup_values[RET_LOOKUP_VALUE_3],
        );
        let numerator = value - lookup_value;
        let denom = point_vanishing(
            constraint_zero_domain.at(constraint_zero_domain.size() - 1),
            point,
        );
        evaluation_accumulator.accumulate(numerator / denom);

        // Lookup step constraint.
        let prev_value =
            SecureField::from_partial_evals(std::array::from_fn(|i| mask[INTERACTION_TRACE][i][1]));
        let numerator = (value - prev_value)
            * shifted_secure_combination(&address_and_value, alpha, z)
            - SecureField::one();
        let denom = coset_vanishing(constraint_zero_domain, point)
            / point_excluder(constraint_zero_domain.at(0), point);
        evaluation_accumulator.accumulate(numerator / denom);
    }
}

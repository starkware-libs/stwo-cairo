use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::qm31::{QM31, QM31Zero};
use stwo_cairo_verifier::poly::circle::CanonicCosetImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::verifier::Air;
use stwo_cairo_verifier::{ColumnArray, TreeArray};

#[derive(Drop)]
pub struct FibAir<const N_COLUMNS: usize> {
    pub log_size: u32,
}

impl FibAirImpl<const N_COLUMNS: usize> of Air<FibAir<N_COLUMNS>> {
    fn composition_log_degree_bound(self: @FibAir<N_COLUMNS>) -> u32 {
        *self.log_size + 1
    }

    fn mask_points(
        self: @FibAir<N_COLUMNS>, point: CirclePoint<QM31>,
    ) -> TreeArray<ColumnArray<Array<CirclePoint<QM31>>>> {
        array![array![], ArrayImpl::new_repeated(N_COLUMNS, array![point])]
    }

    fn eval_composition_polynomial_at_point(
        self: @FibAir<N_COLUMNS>,
        point: CirclePoint<QM31>,
        mask_values: @TreeArray<ColumnArray<Array<QM31>>>,
        random_coeff: QM31,
    ) -> QM31 {
        let base_trace_tree = mask_values[1].span();
        let mut constraint_acc = QM31Zero::zero();

        for i in 2..N_COLUMNS {
            let a_col: @Array<QM31> = base_trace_tree[i - 2];
            let b_col: @Array<QM31> = base_trace_tree[i - 1];
            let c_col: @Array<QM31> = base_trace_tree[i];
            let a: QM31 = *a_col[0];
            let b: QM31 = *b_col[0];
            let c: QM31 = *c_col[0];
            let constraint: QM31 = c - b * b - a * a;
            constraint_acc = constraint_acc * random_coeff + constraint;
        };

        let trace_domain = CanonicCosetImpl::new(*self.log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraint_acc / vanish_eval
    }
}

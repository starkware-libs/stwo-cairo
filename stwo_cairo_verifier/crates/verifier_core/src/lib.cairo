pub mod channel;
pub mod circle;
mod circle_mul_table;
pub mod fields;
pub mod fri;
pub mod pcs;
pub mod poly;
mod queries;
pub mod utils;
pub mod vcs;
pub mod verifier;

pub use fields::{BaseField, SecureField};

/// An array in which each element relates (by index) to a column in the trace.
pub type ColumnArray<T> = Array<T>;

/// A span in which each element relates (by index) to a column in the trace.
pub type ColumnSpan<T> = Span<T>;

/// An array in which each element relates (by index) to a commitment tree.
pub type TreeArray<T> = Array<T>;

/// A span in which each element relates (by index) to a commitment tree.
pub type TreeSpan<T> = Span<T>;

use stwo_verifier_core::channel::ChannelImpl;
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl,
};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31One, QM31Zero};
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::verifier::Air;



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
            let a_col = base_trace_tree[i - 2];
            let b_col = base_trace_tree[i - 1];
            let c_col = base_trace_tree[i];
            let a = *a_col[0];
            let b = *b_col[0];
            let c = *c_col[0];
            let constraint = c - b * b - a * a;
            constraint_acc = constraint_acc * random_coeff + constraint;
        };

        let trace_domain = CanonicCosetImpl::new(*self.log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraint_acc / vanish_eval
    }
}

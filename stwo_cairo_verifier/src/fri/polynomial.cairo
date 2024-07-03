use stwo_cairo_verifier::fields::qm31::{QM31, qm31};

#[derive(Drop)]
pub struct LinePoly {
    pub coeffs: Array<QM31>,
    pub log_size: u32,
}

#[generate_trait]
pub impl LinePolyImpl of LinePolyTrait {
    fn len(self: @LinePoly) -> usize {
        // TODO: implement
        1
    }

    fn eval_at_point(self: @LinePoly, x: QM31) -> QM31 {
        // TODO: implement
        x
    }
}

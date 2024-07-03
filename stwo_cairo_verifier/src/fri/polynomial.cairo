use stwo_cairo_verifier::fields::qm31::{QM31, qm31};

#[derive(Drop)]
pub struct LinePoly {
    pub coeffs: Array<QM31>,
    pub log_size: u32,
}

#[generate_trait]
pub impl LinePolyImpl of LinePolyTrait {
    fn len(self: @LinePoly) -> usize {
        self.coeffs.len()
    }

    fn eval_at_point(self: @LinePoly, x: QM31) -> QM31 {
        if self.len() == 1 {
            *self.coeffs[0]
        } else if self.len() == 2 {
            *self.coeffs[0] + x * *self.coeffs[1]
        } else {
            // TODO: implement for non-linear polynomials
            x
        }
    }
}

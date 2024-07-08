use stwo_cairo_verifier::fields::qm31::{QM31, qm31};
use stwo_cairo_verifier::fields::m31::{m31};

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
        } else if self.len() == 4 {
            let t0 = x;
            let t1 = m31(2).into() * x * x - m31(1).into();
            *self.coeffs[0] + t1 * *self.coeffs[1] + t0 * (*self.coeffs[2] + t1 * *self.coeffs[3])
        } else {
            // TODO: generalize
            x
        }
    }
}

#[test]
fn test_line_poly_1() {
    let line_poly = LinePoly {
        coeffs: array![
            qm31(1080276375, 1725024947, 477465525, 102017026),
            qm31(1080276375, 1725024947, 477465525, 102017026)
        ],
        log_size: 1
    };
    let x = m31(590768354);
    let result = line_poly.eval_at_point(x.into());
    let expected_result = qm31(515899232, 1030391528, 1006544539, 11142505);
    assert_eq!(expected_result, result);
}

#[test]
fn test_line_poly_2() {
    let line_poly = LinePoly { coeffs: array![qm31(1, 2, 3, 4), qm31(5, 6, 7, 8)], log_size: 1 };
    let x = m31(10);
    let result = line_poly.eval_at_point(x.into());
    let expected_result = qm31(51, 62, 73, 84);
    assert_eq!(expected_result, result);
}

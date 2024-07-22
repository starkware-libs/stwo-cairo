use stwo_cairo_verifier::fields::SecureField;
use stwo_cairo_verifier::fields::m31::m31;

/// A univariate polynomial represented by its coefficients in the line part of the FFT-basis {bᵢ}
/// in bit reversed order. See Definition 4 of the Circle STARKs paper
/// https://eprint.iacr.org/2024/278. The line part of the FFT-basis is formed by the elements bᵢ
/// with i even.
///
/// Example: The line part of the FFT of order 4 in natural order is {b₀, b₂, b₄, b₆}, where
/// b₀ = 1, b₂ = x, b₄ = 2x² − 1, and b₆ = x(2x² − 1). The same basis but in bit
/// reversed order is {b₀, b₄, b₂, b₆}.
/// Then, the polynomial p represented by coeffs = [a₀, a₁, a₂, a₃] and log_size = 2 is
/// p = a₀ * b₀ + a₁ * b₄ + a₂ * b₂ + a₃ * b₆.
#[derive(Drop, Clone)]
pub struct LinePoly {
    pub coeffs: Array<SecureField>,
    pub log_size: u32,
}

#[generate_trait]
pub impl LinePolyImpl of LinePolyTrait {
    fn len(self: @LinePoly) -> usize {
        self.coeffs.len()
    }

    // TODO: Consider replacing with FFT.
    fn eval_at_point(self: @LinePoly, mut x: SecureField) -> SecureField {
        let mut mappings = array![];
        let mut i = 0;
        while i < *self.log_size {
            mappings.append(x);
            let x_square = x * x;
            x = x_square + x_square - m31(1).into();
            i += 1;
        };

        let mut level = self.coeffs.clone();

        let mappings = @mappings;
        let mut i = mappings.len();
        while i > 0 {
            i -= 1;
            let mut new_level = array![];
            let mut j = 0;
            while j < level.len() {
                new_level.append(*level[j] + *mappings[i] * *level[j + 1]);
                j += 2;
            };
            level = new_level;
        };

        assert_eq!(level.len(), 1);
        *level[0]
    }
}

#[cfg(test)]
mod tests {
    use super::{LinePoly, LinePolyTrait};
    use stwo_cairo_verifier::fields::qm31::qm31;
    use stwo_cairo_verifier::fields::m31::m31;

    #[test]
    fn test_eval_at_point_1() {
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
    fn test_eval_at_point_2() {
        let line_poly = LinePoly {
            coeffs: array![qm31(1, 2, 3, 4), qm31(5, 6, 7, 8)], log_size: 1
        };
        let x = m31(10);
        let result = line_poly.eval_at_point(x.into());
        let expected_result = qm31(51, 62, 73, 84);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_eval_at_point_3() {
        let poly = LinePoly {
            coeffs: array![
                qm31(1, 2, 3, 4),
                qm31(5, 6, 7, 8),
                qm31(9, 10, 11, 12),
                qm31(13, 14, 15, 16),
                qm31(17, 18, 19, 20),
                qm31(21, 22, 23, 24),
                qm31(25, 26, 27, 28),
                qm31(29, 30, 31, 32),
            ],
            log_size: 3
        };
        let x = qm31(2, 5, 7, 11);

        let result = poly.eval_at_point(x);

        let expected_result = qm31(1857853974, 839310133, 939318020, 651207981);
        assert_eq!(expected_result, result);
    }
}

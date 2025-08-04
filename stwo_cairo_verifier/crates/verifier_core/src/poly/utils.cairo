use crate::fields::m31::{M31, MulByM31Trait};
use crate::fields::qm31::{QM31, QM31Trait};
use crate::fields::{BaseField, SecureField};

/// Folds values recursively in `O(n)` by a hierarchical application of folding factors.
///
/// i.e. folding `n = 8` values with `folding_factors = [x, y, z]`:
///
/// ```text
///               n2=n1+x*n2
///           /               \
///     n1=n3+y*n4          n2=n5+y*n6
///      /      \            /      \
/// n3=a+z*b  n4=c+z*d  n5=e+z*f  n6=g+z*h
///   /  \      /  \      /  \      /  \
///  a    b    c    d    e    f    g    h
/// ```
///
/// # Panics
///
/// Panics if the number of values is not a power of two or if an incorrect number of of folding
/// factors is provided.
pub fn fold(
    values: @Array<SecureField>,
    folding_factors: @Array<BaseField>,
    index: usize,
    level: usize,
    n: usize,
) -> SecureField {
    if n == 1 {
        return *values[index];
    }

    let lhs_val = fold(values, folding_factors, index, level + 1, n / 2);
    let rhs_val = fold(values, folding_factors, index + n / 2, level + 1, n / 2);
    return lhs_val + rhs_val.mul_m31(*folding_factors[level]);
}

#[inline]
pub fn butterfly(v0: QM31, v1: QM31, twid: M31) -> (QM31, QM31) {
    let tmp = v1.mul_m31(twid);
    (v0 + tmp, v0 - tmp)
}

#[inline]
pub fn ibutterfly(v0: QM31, v1: QM31, itwid: M31) -> (QM31, QM31) {
    (v0 + v1, (v0 - v1).mul_m31(itwid))
}

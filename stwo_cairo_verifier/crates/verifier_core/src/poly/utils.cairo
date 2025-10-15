use crate::fields::m31::{M31, MulByM31Trait};
use crate::fields::qm31::{PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31};

#[inline]
pub fn butterfly(v0: QM31, v1: QM31, twid: M31) -> (QM31, QM31) {
    let tmp = v1.mul_m31(twid);
    (v0 + tmp, v0 - tmp)
}

/// Performs an ibutterfly on the two values of the function, then folds using `fold_alpha`.
#[inline]
pub fn fri_fold(v0: QM31, v1: QM31, itwid: M31, fold_alpha: QM31) -> QM31 {
    let [v0, v1]: [PackedUnreducedQM31; 2] = [v0.into(), v1.into()];
    let (f0, f1) = (v0 + v1, (v0 - v1).mul_m31(itwid));
    PackedUnreducedQM31Trait::packed_fused_mul_add(f1, fold_alpha, f0)
}

use crate::fields::m31::{M31, MulByM31Trait};
use crate::fields::qm31::{PackedUnreducedQM31, QM31};

#[inline]
pub fn butterfly(v0: QM31, v1: QM31, twid: M31) -> (QM31, QM31) {
    let tmp = v1.mul_m31(twid);
    (v0 + tmp, v0 - tmp)
}

#[inline]
pub fn ibutterfly(v0: QM31, v1: QM31, itwid: M31) -> (PackedUnreducedQM31, PackedUnreducedQM31) {
    let [v0, v1]: [PackedUnreducedQM31; 2] = [v0.into(), v1.into()];
    (v0 + v1, (v0 - v1).mul_m31(itwid))
}

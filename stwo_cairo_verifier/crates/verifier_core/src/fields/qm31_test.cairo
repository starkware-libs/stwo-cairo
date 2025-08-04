use crate::fields::Invertible;
use crate::fields::m31::{MulByM31Trait, m31};
use crate::fields::qm31::{
    PackedUnreducedQM31, PackedUnreducedQM31Trait, QM31, QM31Impl, QM31Trait, qm31_const,
};

#[test]
fn test_QM31() {
    let qm0 = qm31_const::<1, 2, 3, 4>();
    let qm1 = qm31_const::<4, 5, 6, 7>();
    let m = m31(8);
    let qm = Into::<_, QM31>::into(m);
    let qm0_x_qm1 = qm31_const::<2147483576, 93, 2147483631, 50>();

    assert_eq!(qm0 + qm1, qm31_const::<5, 7, 9, 11>());
    assert_eq!(qm1 + m.into(), qm1 + qm);
    assert_eq!(qm0 * qm1, qm0_x_qm1);
    assert_eq!(qm1 * m.into(), qm1 * qm);
    assert_eq!(-qm0, qm31_const::<2147483646, 2147483645, 2147483644, 2147483643>());
    assert_eq!(qm0 - qm1, qm31_const::<2147483644, 2147483644, 2147483644, 2147483644>());
    assert_eq!(qm1 - m.into(), qm1 - qm);
    assert_eq!(qm0_x_qm1 * qm1.inverse(), qm31_const::<1, 2, 3, 4>());
    assert_eq!(qm1 * m.inverse().into(), qm1 * qm.inverse());
    assert_eq!(qm1.mul_m31(m), qm1 * m.into());
}

#[test]
fn test_fused_mul_add() {
    let a = qm31_const::<2147483643, 2147483557, 958, 2147483646>();
    let b = qm31_const::<2147483464, 75, 2147482726, 2147477523>();
    let c = qm31_const::<2, 2147483646, 2147483646, 2147483639>();

    let res = QM31Trait::fused_mul_add(a, b, c);

    assert_eq!(res, a * b + c);
}

#[test]
fn test_fused_mul_sub() {
    let a = qm31_const::<2147483643, 2147483557, 958, 2147483646>();
    let b = qm31_const::<2147483464, 75, 2147482726, 2147477523>();
    let c = qm31_const::<2, 2147483646, 2147483646, 2147483639>();

    let res = QM31Trait::fused_mul_sub(a, b, c);

    assert_eq!(res, a * b - c);
}

#[test]
fn test_packed_unreduced_qm31() {
    let a = qm31_const::<2147483643, 2147483557, 958, 2147483646>();

    let packed_a: PackedUnreducedQM31 = a.into();
    let b = m31(2147483464);

    let res_unreduced = packed_a.mul_m31(b.into());
    let res = res_unreduced.reduce();

    assert_eq!(res, a.mul_m31(b));
}

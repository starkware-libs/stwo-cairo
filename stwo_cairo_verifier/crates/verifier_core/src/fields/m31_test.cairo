use crate::fields::Invertible;
use crate::fields::m31::{P_U32 as P, m31};

const POW2_15: u32 = 0b1000000000000000;
const POW2_16: u32 = 0b10000000000000000;

#[test]
fn test_m31() {
    assert_eq!(m31(1) + m31(2), m31(3));
    assert_eq!(m31(3) - m31(2), m31(1));
    assert_eq!(m31(P - 1) + m31(1), m31(0));
    assert_eq!(m31(0) - m31(1), m31(P - 1));
    assert_eq!(m31(0) - m31(P - 1), m31(1));
}

#[test]
#[should_panic]
fn test_m31_fail() {
    m31(P);
}

#[test]
fn test_m31_inv() {
    assert_eq!(m31(POW2_15).inverse(), m31(POW2_16));
}

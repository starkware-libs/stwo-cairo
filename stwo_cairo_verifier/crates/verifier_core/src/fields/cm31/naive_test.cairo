use crate::fields::cm31::cm31_const;
use crate::fields::m31::m31;

#[test]
fn test_cm31() {
    let cm0 = cm31_const::<1, 2>();
    let cm1 = cm31_const::<4, 5>();
    let m = m31(8);
    let cm = cm31_const::<8, 0>();
    let cm0_x_cm1 = cm31_const::<0x7ffffff9, 13>();

    assert_eq!(cm0 + cm1, cm31_const::<5, 7>());
    assert_eq!(cm1 + m.into(), cm1 + cm);
    assert_eq!(cm0 * cm1, cm0_x_cm1);
    assert_eq!(cm1 * m.into(), cm1 * cm);
    assert_eq!(-cm0, cm31_const::<0x7ffffffe, 0x7ffffffd>());
    assert_eq!(cm0 - cm1, cm31_const::<0x7ffffffc, 0x7ffffffc>());
    assert_eq!(cm1 - m.into(), cm1 - cm);
}

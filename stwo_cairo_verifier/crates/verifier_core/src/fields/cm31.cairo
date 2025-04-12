use super::m31::M31;

mod soft;
use soft::*;

pub trait CM31Trait {
    // TODO(andrew): When associated types are supported, support `Mul<CM31, M31>`.
    fn mul_m31(self: CM31, rhs: M31) -> CM31;

    // TODO(andrew): When associated types are supported, support `Sub<CM31, M31>`.
    fn sub_m31(self: CM31, rhs: M31) -> CM31;

    fn unpack(self: CM31) -> (M31, M31);

    fn pack(a: M31, b: M31) -> CM31;
}

impl DisplayCM31 of core::fmt::Display<CM31> {
    fn fmt(self: @CM31, ref f: core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let (a, b) = (*self).unpack();
        let a: u32 = a.into();
        let b: u32 = b.into();
        write!(f, "{} + {}i", @a, @b)
    }
}

#[cfg(test)]
mod tests {
    use super::cm31_const;
    use super::super::m31::m31;

    #[test]
    fn test_cm31() {
        let cm0 = cm31_const::<1, 2>();
        let cm1 = cm31_const::<4, 5>();
        let m = m31(8);
        let cm = cm31_const::<8, 0>();
        let cm0_x_cm1 = cm31_const::<2147483641, 13>();

        assert_eq!(cm0 + cm1, cm31_const::<5, 7>());
        assert_eq!(cm1 + m.into(), cm1 + cm);
        assert_eq!(cm0 * cm1, cm0_x_cm1);
        assert_eq!(cm1 * m.into(), cm1 * cm);
        assert_eq!(-cm0, cm31_const::<2147483646, 2147483645>());
        assert_eq!(cm0 - cm1, cm31_const::<2147483644, 2147483644>());
        assert_eq!(cm1 - m.into(), cm1 - cm);
    }
}

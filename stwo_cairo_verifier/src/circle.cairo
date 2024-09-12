use core::num::traits::{WrappingAdd, WrappingMul, WrappingSub};
use stwo_cairo_verifier::fields::cm31::{CM31};
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::fields::qm31::{QM31, QM31Impl};
use super::utils::pow;

pub const M31_CIRCLE_GEN: CirclePointM31 =
    CirclePointM31 { x: M31 { inner: 2 }, y: M31 { inner: 1268011823 }, };

/// A generator for the circle group over [`SecureField`].
pub const QM31_CIRCLE_GEN: CirclePointQM31 =
    CirclePointQM31 {
        x: QM31 {
            a: CM31 { a: M31 { inner: 1 }, b: M31 { inner: 0 }, },
            b: CM31 { a: M31 { inner: 478637715 }, b: M31 { inner: 513582971 } }
        },
        y: QM31 {
            a: CM31 { a: M31 { inner: 992285211 }, b: M31 { inner: 649143431 } },
            b: CM31 { a: M31 { inner: 740191619 }, b: M31 { inner: 1186584352 } }
        },
    };

pub const M31_CIRCLE_LOG_ORDER: u32 = 31;

/// Equals `2^31`.
pub const M31_CIRCLE_ORDER: u32 = 2147483648;

/// Equals `2^31 - 1`.
pub const M31_CIRCLE_ORDER_BIT_MASK: u32 = 0x7fffffff;

/// Equals `2^32 - 1`.
pub const U32_BIT_MASK: u64 = 0xffffffff;

#[derive(Drop, Copy, Debug, PartialEq)]
pub struct CirclePointM31 {
    pub x: M31,
    pub y: M31,
}

#[generate_trait]
pub impl CirclePointM31Impl of CirclePointM31Trait {
    /// Returns the neutral element of the circle.
    fn zero() -> CirclePointM31 {
        CirclePointM31 { x: m31(1), y: m31(0) }
    }

    fn mul(self: @CirclePointM31, mut scalar: u32) -> CirclePointM31 {
        let mut result = Self::zero();
        let mut cur = *self;
        while scalar > 0 {
            if scalar & 1 == 1 {
                result = result + cur;
            }
            cur = cur + cur;
            scalar = scalar / 2;
        };
        result
    }
}

impl CirclePointM31Add of Add<CirclePointM31> {
    // The operation of the circle as a group with additive notation.
    fn add(lhs: CirclePointM31, rhs: CirclePointM31) -> CirclePointM31 {
        CirclePointM31 { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

impl CirclePointM31IntoCirclePointQM31 of Into<CirclePointM31, CirclePointQM31> {
    #[inline]
    fn into(self: CirclePointM31) -> CirclePointQM31 {
        CirclePointQM31 { x: self.x.into(), y: self.y.into() }
    }
}

#[derive(Drop, Copy, Debug, PartialEq)]
pub struct CirclePointQM31 {
    pub x: QM31,
    pub y: QM31,
}

#[generate_trait]
pub impl CirclePointQM31Impl of CirclePointQM31Trait {
    /// Returns the neutral element of the circle.
    fn zero() -> CirclePointQM31 {
        CirclePointM31Impl::zero().into()
    }
    fn complex_conjugate(self: CirclePointQM31) -> CirclePointQM31 {
        CirclePointQM31 { x: self.x.complex_conjugate(), y: self.y.complex_conjugate(), }
    }

    #[cfg(test)]
    fn mul(self: @CirclePointQM31, mut scalar: u128) -> CirclePointQM31 {
        let mut result = Self::zero();
        let mut cur = *self;
        while scalar > 0 {
            if scalar & 1 == 1 {
                result = result + cur;
            }
            cur = cur + cur;
            scalar = scalar / 2;
        };
        result
    }
}

impl CirclePointQM31Add of Add<CirclePointQM31> {
    // The operation of the circle as a group with additive notation.
    fn add(lhs: CirclePointQM31, rhs: CirclePointQM31) -> CirclePointQM31 {
        CirclePointQM31 { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

/// Integer i that represent the circle point `i * M31_CIRCLE_GEN`.
///
/// Treated as an additive ring modulo `1 << M31_CIRCLE_LOG_ORDER`.
#[derive(Copy, Debug, Drop)]
pub struct CirclePointIndex {
    /// The index, stored as an unreduced `u32` for performance reasons.
    index: u32,
}

#[generate_trait]
pub impl CirclePointIndexImpl of CirclePointIndexTrait {
    fn new(index: u32) -> CirclePointIndex {
        CirclePointIndex { index }
    }

    fn zero() -> CirclePointIndex {
        CirclePointIndex { index: 0 }
    }

    fn generator() -> CirclePointIndex {
        CirclePointIndex { index: 1 }
    }

    fn reduce(self: @CirclePointIndex) -> CirclePointIndex {
        CirclePointIndex { index: *self.index & (pow(2, M31_CIRCLE_LOG_ORDER) - 1) }
    }

    fn subgroup_gen(log_size: u32) -> CirclePointIndex {
        assert!(log_size <= M31_CIRCLE_LOG_ORDER);
        CirclePointIndex { index: pow(2, M31_CIRCLE_LOG_ORDER - log_size) }
    }

    // TODO(andrew): When associated types are supported, support the Mul<Self, u32>.
    fn mul(self: @CirclePointIndex, scalar: u32) -> CirclePointIndex {
        CirclePointIndex { index: (*self.index).wrapping_mul(scalar) }
    }

    fn index(self: @CirclePointIndex) -> u32 {
        self.reduce().index
    }

    fn to_point(self: @CirclePointIndex) -> CirclePointM31 {
        // No need to call `reduce()`.
        M31_CIRCLE_GEN.mul(*self.index)
    }
}

impl CirclePointIndexAdd of Add<CirclePointIndex> {
    #[inline]
    fn add(lhs: CirclePointIndex, rhs: CirclePointIndex) -> CirclePointIndex {
        CirclePointIndex { index: lhs.index.wrapping_add(rhs.index) }
    }
}

impl CirclePointIndexNeg of Neg<CirclePointIndex> {
    #[inline]
    fn neg(a: CirclePointIndex) -> CirclePointIndex {
        CirclePointIndex { index: M31_CIRCLE_ORDER.wrapping_sub(a.index) }
    }
}

impl CirclePointIndexPartialEx of PartialEq<CirclePointIndex> {
    fn eq(lhs: @CirclePointIndex, rhs: @CirclePointIndex) -> bool {
        lhs.index() == rhs.index()
    }

    fn ne(lhs: @CirclePointIndex, rhs: @CirclePointIndex) -> bool {
        lhs.index() != rhs.index()
    }
}


#[derive(Copy, Debug, PartialEq, Drop)]
pub struct Coset {
    pub initial_index: CirclePointIndex,
    pub step_size: CirclePointIndex,
    pub log_size: u32,
}

#[generate_trait]
pub impl CosetImpl of CosetTrait {
    fn new(initial_index: CirclePointIndex, log_size: u32) -> Coset {
        let step_size = CirclePointIndexImpl::subgroup_gen(log_size);
        Coset { initial_index, step_size, log_size }
    }

    /// Returns the [`CirclePointIndex`] of the `i`th coset element.
    fn index_at(self: @Coset, i: u32) -> CirclePointIndex {
        *self.initial_index + self.step_size.mul(i)
    }

    fn double(self: @Coset) -> Coset {
        assert!(*self.log_size > 0);
        Coset {
            initial_index: *self.initial_index + *self.initial_index,
            step_size: *self.step_size + *self.step_size,
            log_size: *self.log_size - 1
        }
    }

    fn size(self: @Coset) -> usize {
        pow(2, *self.log_size)
    }

    #[inline]
    fn at(self: @Coset, i: usize) -> CirclePointM31 {
        self.index_at(i).to_point()
    }

    /// Creates a coset of the form `G_{2n} + <G_n>`.
    ///
    /// For example, for `n = 8`, we get the point indices `[1, 3, 5, 7, 9, 11, 13, 15]`.
    fn odds(log_size: u32) -> Coset {
        Self::new(CirclePointIndexImpl::subgroup_gen(log_size + 1), log_size)
    }
}

/// A coset of the form `G_{2n} + <G_n>`, where `G_n` is the generator of the subgroup of order `n`.
///
/// The ordering on this coset is `G_{2n} + i * G_n`. These cosets can be used as a
/// [`CircleDomain`], and be interpolated on. Note that this changes the ordering on the coset to be
/// like [`CircleDomain`], which is `G_{2n} + i * G_{n/2}` and then `-G_{2n} -i * G_{n/2}`. For
/// example, the `X`s below are a canonic coset with `n = 8`.
///
/// ```text
///    X O X
///  O       O
/// X         X
/// O         O
/// X         X
///  O       O
///    X O X
/// ```
#[derive(Copy, Debug, PartialEq, Drop)]
pub struct CanonicCoset {
    pub coset: Coset,
}

#[generate_trait]
pub impl CanonicCosetImpl of CanonicCosetTrait {
    fn new(log_size: u32) -> CanonicCoset {
        assert!(log_size > 0);
        CanonicCoset { coset: CosetImpl::odds(log_size), }
    }

    /// Gets the full coset represented `G_{2n} + <G_n>`.
    fn coset(self: @CanonicCoset) -> @Coset {
        self.coset
    }

    /// Gets half of the coset (its conjugate complements to the whole coset), `G_{2n} + <G_{n/2}>`.
    fn half_coset(self: @CanonicCoset) -> Coset {
        assert!(*self.coset.log_size > 0);
        Coset {
            initial_index: *self.coset.initial_index,
            step_size: *self.coset.step_size + *self.coset.step_size,
            log_size: *self.coset.log_size - 1,
        }
    }

    /// Gets the [CircleDomain] representing the same point set (in another order).
    fn circle_domain(self: @CanonicCoset) -> CircleDomain {
        CircleDomainImpl::new(self.half_coset())
    }
}

/// A valid domain for circle polynomial interpolation and evaluation.
///
/// Valid domains are a disjoint union of two conjugate cosets: `+-C + <G_n>`.
/// The ordering defined on this domain is `C + iG_n`, and then `-C - iG_n`.
#[derive(Copy, Debug, PartialEq, Drop)]
pub struct CircleDomain {
    pub half_coset: Coset,
}

#[generate_trait]
pub impl CircleDomainImpl of CircleDomainTrait {
    /// Given a coset `C + <G_n>`, constructs the circle domain `+-C + <G_n>` (i.e.,
    /// this coset and its conjugate).
    fn new(half_coset: Coset) -> CircleDomain {
        CircleDomain { half_coset }
    }

    #[inline]
    fn log_size(self: @CircleDomain) -> u32 {
        *self.half_coset.log_size + 1
    }

    fn size(self: @CircleDomain) -> u32 {
        pow(2, self.log_size())
    }

    /// Returns the [`CirclePointIndex`] of the `i`th domain element.
    fn index_at(self: @CircleDomain, i: u32) -> CirclePointIndex {
        if i < self.half_coset.size() {
            self.half_coset.index_at(i)
        } else {
            -self.half_coset.index_at(i - self.half_coset.size())
        }
    }

    #[inline]
    fn at(self: @CircleDomain, i: usize) -> CirclePointM31 {
        self.index_at(i).to_point()
    }
}

#[cfg(test)]
mod tests {
    use stwo_cairo_verifier::fields::m31::m31;
    use super::{
        M31_CIRCLE_GEN, M31_CIRCLE_ORDER, CirclePointM31, CirclePointM31Impl, Coset, CosetImpl,
        CirclePointIndexImpl
    };

    #[test]
    fn test_add_1() {
        let i = CirclePointM31 { x: m31(0), y: m31(1) };
        let result = i + i;
        let expected_result = CirclePointM31 { x: -m31(1), y: m31(0) };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_add_2() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePointM31 { x: m31(1737427771), y: m31(309481134) };
        let result = point_1 + point_2;
        let expected_result = CirclePointM31 { x: m31(1476625263), y: m31(1040927458) };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_zero_1() {
        let result = CirclePointM31Impl::zero();
        let expected_result = CirclePointM31 { x: m31(1), y: m31(0) };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_zero_2() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePointM31Impl::zero();
        let expected_result = point_1.clone();
        let result = point_1 + point_2;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mul_1() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(5);
        let expected_result = point_1 + point_1 + point_1 + point_1 + point_1;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mul_2() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(8);
        let mut expected_result = point_1 + point_1;
        expected_result = expected_result + expected_result;
        expected_result = expected_result + expected_result;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mul_3() {
        let point_1 = CirclePointM31 { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(418776494);
        let expected_result = CirclePointM31 { x: m31(1987283985), y: m31(1500510905) };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_generator_order() {
        let half_order = M31_CIRCLE_ORDER / 2;
        let mut result = M31_CIRCLE_GEN.mul(half_order);
        let expected_result = CirclePointM31 { x: -m31(1), y: m31(0) };

        // Assert `M31_CIRCLE_GEN^{2^30}` equals `-1`.
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_coset_index_at() {
        let log_size = 5;
        let initial_index = CirclePointIndexImpl::new(16777216);
        let coset = CosetImpl::new(initial_index, log_size);

        let result = coset.index_at(8);

        assert_eq!(result, CirclePointIndexImpl::new(553648128));
    }

    #[test]
    fn test_coset_double() {
        let log_size = 5;
        let initial_index = CirclePointIndexImpl::new(16777216);
        let coset = CosetImpl::new(initial_index, log_size);

        let result = coset.double();

        let expected_result = Coset {
            initial_index: CirclePointIndexImpl::new(33554432),
            step_size: CirclePointIndexImpl::new(134217728),
            log_size: 4
        };
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_coset_at() {
        let initial_index = CirclePointIndexImpl::new(16777216);
        let log_size = 5;
        let coset = CosetImpl::new(initial_index, log_size);

        let result = coset.at(17);

        assert_eq!(result, CirclePointM31 { x: m31(7144319), y: m31(1742797653) });
    }

    #[test]
    fn test_coset_size() {
        let initial_index = CirclePointIndexImpl::new(16777216);
        let log_size = 5;
        let coset = CosetImpl::new(initial_index, log_size);

        let result = coset.size();

        assert_eq!(result, 32);
    }
}


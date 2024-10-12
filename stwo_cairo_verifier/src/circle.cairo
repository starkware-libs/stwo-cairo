use core::num::traits::one::One;
use core::num::traits::zero::Zero;
use core::num::traits::{WrappingAdd, WrappingSub, WrappingMul};
use stwo_cairo_verifier::fields::cm31::CM31;
use stwo_cairo_verifier::fields::m31::{M31, M31Impl};
use stwo_cairo_verifier::fields::qm31::{QM31Impl, QM31, QM31Trait};
use super::utils::pow;

/// A generator for the circle group over [`M31`].
pub const M31_CIRCLE_GEN: CirclePoint<M31> =
    CirclePoint { x: M31 { inner: 2 }, y: M31 { inner: 1268011823 }, };

pub const M31_CIRCLE_LOG_ORDER: u32 = 31;

/// Equals `2^31`.
pub const M31_CIRCLE_ORDER: u32 = 2147483648;

/// Equals `2^31 - 1`.
pub const M31_CIRCLE_ORDER_BIT_MASK: u32 = 0x7fffffff;

/// A generator for the circle group over [`QM31`].
pub const QM31_CIRCLE_GEN: CirclePoint<QM31> =
    CirclePoint {
        x: QM31 {
            a: CM31 { a: M31 { inner: 1 }, b: M31 { inner: 0 }, },
            b: CM31 { a: M31 { inner: 478637715 }, b: M31 { inner: 513582971 } }
        },
        y: QM31 {
            a: CM31 { a: M31 { inner: 992285211 }, b: M31 { inner: 649143431 } },
            b: CM31 { a: M31 { inner: 740191619 }, b: M31 { inner: 1186584352 } }
        },
    };

/// Order of [`QM31_CIRCLE_GEN`].
pub const QM31_CIRCLE_ORDER: u128 = 21267647892944572736998860269687930880;

/// A point on the complex circle. Treated as an additive group.
#[derive(Drop, Copy, Debug, PartialEq)]
pub struct CirclePoint<F> {
    pub x: F,
    pub y: F
}

pub trait CirclePointTrait<
    F, +Add<F>, +Sub<F>, +Mul<F>, +Drop<F>, +Copy<F>, +Zero<F>, +One<F>, +PartialEq<F>
> {
    // Returns the neutral element of the circle.
    fn zero() -> CirclePoint<F> {
        CirclePoint { x: One::one(), y: Zero::zero() }
    }

    /// Applies the circle's x-coordinate doubling map.
    fn double_x(x: F) -> F {
        let sqx = x * x;
        sqx + sqx - One::one()
    }

    /// Returns the log order of a point.
    ///
    /// All points have an order of the form `2^k`.
    fn log_order(
        self: @CirclePoint<F>
    ) -> u32 {
        // we only need the x-coordinate to check order since the only point
        // with x=1 is the circle's identity
        let mut res = 0;
        let mut cur = self.x.clone();
        while cur != One::one() {
            cur = Self::double_x(cur);
            res += 1;
        };
        res
    }

    fn mul(
        self: @CirclePoint<F>, scalar: u128
    ) -> CirclePoint<
        F
    > {
        // TODO: `mut scalar: u128` doesn't work in trait.
        let mut scalar = scalar;
        let mut result = Self::zero();
        let mut cur = *self;
        while scalar != 0 {
            if scalar & 1 == 1 {
                result = result + cur;
            }
            cur = cur + cur;
            scalar /= 2;
        };
        result
    }
}

impl CirclePointAdd<F, +Add<F>, +Sub<F>, +Mul<F>, +Drop<F>, +Copy<F>> of Add<CirclePoint<F>> {
    /// Performs the operation of the circle as a group with additive notation.
    fn add(lhs: CirclePoint<F>, rhs: CirclePoint<F>) -> CirclePoint<F> {
        CirclePoint { x: lhs.x * rhs.x - lhs.y * rhs.y, y: lhs.x * rhs.y + lhs.y * rhs.x }
    }
}

pub impl CirclePointM31Impl of CirclePointTrait<M31> {}

pub impl CirclePointQM31Impl of CirclePointTrait<QM31> {}

#[generate_trait]
pub impl ComplexConjugateImpl of ComplexConjugateTrait {
    fn complex_conjugate(self: CirclePoint<QM31>) -> CirclePoint<QM31> {
        CirclePoint { x: self.x.complex_conjugate(), y: self.y.complex_conjugate() }
    }
}

/// Represents the coset `initial + <step>`.
#[derive(Copy, Clone, Debug, PartialEq, Drop)]
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

    fn index_at(self: @Coset, index: usize) -> CirclePointIndex {
        *self.initial_index + self.step_size.mul(index)
    }

    fn double(self: @Coset) -> Coset {
        assert!(*self.log_size > 0);
        Coset {
            initial_index: *self.initial_index + *self.initial_index,
            step_size: *self.step_size + *self.step_size,
            log_size: *self.log_size - 1
        }
    }

    #[inline]
    fn at(self: @Coset, index: usize) -> CirclePoint<M31> {
        self.index_at(index).to_point()
    }

    /// Returns the size of the coset.
    fn size(self: @Coset) -> usize {
        pow(2, *self.log_size)
    }

    /// Creates a coset of the form `G_2n + <G_n>`.
    ///
    /// For example, for `n=8`, we get the point indices `[1,3,5,7,9,11,13,15]`.
    fn odds(log_size: u32) -> Coset {
        Self::new(CirclePointIndexImpl::subgroup_gen(log_size + 1), log_size)
    }

    /// Creates a coset of the form `G_4n + <G_n>`.
    /// 
    /// For example, for `n=8`, we get the point indices `[1,5,9,13,17,21,25,29]`.
    /// Its conjugate will be `[3,7,11,15,19,23,27,31]`.
    fn half_odds(log_size: u32) -> Coset {
        Self::new(CirclePointIndexImpl::subgroup_gen(log_size + 2), log_size)
    }
}

/// Integer `i` that represent the circle point `i * M31_CIRCLE_GEN`.
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
        CirclePointIndex { index: *self.index & M31_CIRCLE_ORDER_BIT_MASK }
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

    fn to_point(self: @CirclePointIndex) -> CirclePoint<M31> {
        // No need to call `reduce()`.
        M31_CIRCLE_GEN.mul((*self.index).into())
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

#[cfg(test)]
mod tests {
    use stwo_cairo_verifier::fields::m31::m31;
    use stwo_cairo_verifier::fields::qm31::{QM31One, qm31};
    use stwo_cairo_verifier::utils::pow;
    use super::{M31_CIRCLE_GEN, CirclePointQM31Impl, QM31_CIRCLE_GEN, M31_CIRCLE_ORDER, CirclePoint, CirclePointM31Impl, CirclePointIndexImpl, Coset, CosetImpl, QM31_CIRCLE_ORDER};

    #[test]
    fn test_add_1() {
        let i = CirclePoint { x: m31(0), y: m31(1) };
        let result = i + i;

        assert_eq!(result, CirclePoint { x: -m31(1), y: m31(0) });
    }

    #[test]
    fn test_add_2() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePoint { x: m31(1737427771), y: m31(309481134) };
        let result = point_1 + point_2;

        assert_eq!(result, CirclePoint { x: m31(1476625263), y: m31(1040927458) });
    }

    #[test]
    fn test_zero_1() {
        let result = CirclePointM31Impl::zero();

        assert_eq!(result, CirclePoint { x: m31(1), y: m31(0) });
    }

    #[test]
    fn test_zero_2() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let point_2 = CirclePointM31Impl::zero();
        let result = point_1 + point_2;

        assert_eq!(result, point_1.clone());
    }

    #[test]
    fn test_mul_1() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(5);

        assert_eq!(result, point_1 + point_1 + point_1 + point_1 + point_1);
    }

    #[test]
    fn test_mul_2() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(8);

        assert_eq!(
            result, point_1 + point_1 + point_1 + point_1 + point_1 + point_1 + point_1 + point_1
        );
    }

    #[test]
    fn test_mul_3() {
        let point_1 = CirclePoint { x: m31(750649172), y: m31(1991648574) };
        let result = point_1.mul(418776494);

        assert_eq!(result, CirclePoint { x: m31(1987283985), y: m31(1500510905) });
    }

    #[test]
    fn test_generator_order() {
        let half_order = M31_CIRCLE_ORDER / 2;
        let mut result = M31_CIRCLE_GEN.mul(half_order.into());

        // Assert `M31_CIRCLE_GEN^{2^30}` equals `-1`.
        assert_eq!(result, CirclePoint { x: -m31(1), y: m31(0) });
    }

    #[test]
    fn test_generator() {
        let mut result = M31_CIRCLE_GEN.mul(pow(2, 30).into());

        assert_eq!(result, CirclePoint { x: -m31(1), y: m31(0) });
    }

    #[test]
    fn test_coset_index_at() {
        let coset = Coset { initial_index: CirclePointIndexImpl::new(16777216), log_size: 5, step_size: CirclePointIndexImpl::new(67108864) };
        let result = coset.index_at(8);

        assert_eq!(result, CirclePointIndexImpl::new(553648128));
    }

    #[test]
    fn test_coset_constructor() {
        let result = CosetImpl::new(CirclePointIndexImpl::new(16777216), 5);

        assert_eq!(result, Coset { initial_index: CirclePointIndexImpl::new(16777216), log_size: 5, step_size: CirclePointIndexImpl::new(67108864) });
    }

    #[test]
    fn test_coset_double() {
        let coset = Coset { initial_index: CirclePointIndexImpl::new(16777216), step_size: CirclePointIndexImpl::new(67108864), log_size: 5 };
        let result = coset.double();

        assert_eq!(result, Coset { initial_index: CirclePointIndexImpl::new(33554432), step_size: CirclePointIndexImpl::new(134217728), log_size: 4 });
    }

    #[test]
    fn test_coset_at() {
        let coset = Coset { initial_index: CirclePointIndexImpl::new(16777216), step_size: CirclePointIndexImpl::new(67108864), log_size: 5 };
        let result = coset.at(17);

        assert_eq!(result, CirclePoint { x: m31(7144319), y: m31(1742797653) });
    }

    #[test]
    fn test_coset_size() {
        let coset = Coset { initial_index: CirclePointIndexImpl::new(16777216), step_size: CirclePointIndexImpl::new(67108864), log_size: 5 };
        let result = coset.size();

        assert_eq!(result, 32);
    }

    #[test]
    fn test_qm31_circle_gen() {
        assert_eq!(QM31_CIRCLE_GEN.mul(QM31_CIRCLE_ORDER / 2), CirclePoint { x: -qm31(1, 0, 0, 0), y: qm31(0, 0, 0, 0) });
    }
}


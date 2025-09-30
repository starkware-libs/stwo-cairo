use core::num::traits::Zero;
use crate::circle::{
    CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointM31Impl, CirclePointTrait,
    Coset, CosetImpl,
};
use crate::fields::m31::M31;
use crate::fields::qm31::QM31;
use crate::utils::pow2;

/// A valid domain for circle polynomial interpolation and evaluation.
///
/// Valid domains are a disjoint union of two conjugate cosets: `+-C + <G_n>`.
/// The ordering defined on this domain is `C + iG_n`, and then `-C - iG_n`.
#[derive(Debug, Copy, Drop, PartialEq)]
pub struct CircleDomain {
    pub half_coset: Coset,
}

#[generate_trait]
pub impl CircleDomainImpl of CircleDomainTrait {
    /// Given a coset `C + <G_n>`, constructs the circle domain `+-C + <G_n>` (i.e.
    /// this coset and its conjugate).
    fn new(half_coset: Coset) -> CircleDomain {
        CircleDomain { half_coset }
    }

    /// Returns the size of the domain.
    #[inline]
    fn size(self: @CircleDomain) -> usize {
        pow2(self.log_size())
    }

    /// Returns the log size of the domain.
    #[inline]
    fn log_size(self: @CircleDomain) -> usize {
        *self.half_coset.log_size + 1
    }

    /// Returns the [`CirclePointIndex`] of the `i`th domain element.
    fn index_at(self: @CircleDomain, index: usize) -> CirclePointIndex {
        if index < self.half_coset.size() {
            self.half_coset.index_at(index)
        } else {
            -self.half_coset.index_at(index - self.half_coset.size())
        }
    }

    #[inline]
    fn at(self: @CircleDomain, index: usize) -> CirclePoint<M31> {
        self.index_at(index).to_point()
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
        assert!(log_size.is_non_zero());
        CanonicCoset { coset: CosetImpl::odds(log_size) }
    }

    /// Gets the full coset represented `G_{2n} + <G_n>`.
    fn coset(self: @CanonicCoset) -> @Coset {
        self.coset
    }

    /// Gets half of the coset (its conjugate complements to the whole coset), `G_{2n} + <G_{n/2}>`.
    fn half_coset(self: @CanonicCoset) -> Coset {
        assert!(self.coset.log_size.is_non_zero());
        Coset {
            initial_index: *self.coset.initial_index,
            step: *self.coset.step + *self.coset.step,
            log_size: *self.coset.log_size - 1,
        }
    }

    /// Gets the [`CircleDomain`] representing the same point set (in another order).
    fn circle_domain(self: @CanonicCoset) -> CircleDomain {
        CircleDomainImpl::new(self.half_coset())
    }

    /// Evaluates the coset's vanishing polynomial at point `p`.
    fn eval_vanishing(self: @CanonicCoset, p: CirclePoint<QM31>) -> QM31 {
        let mut x = p.x;

        // The formula for the x coordinate of the double of a point.
        for _ in 1..*self.coset.log_size {
            x = CirclePointTrait::double_x(x);
        }

        x
    }
}

/// An evaluation defined on a [`CircleDomain`].
///
/// The values are ordered according to the [`CircleDomain`] ordering.
#[derive(Debug, Drop, Clone, PartialEq)]
pub struct CircleEvaluation {
    pub bit_reversed_values: Array<QM31>,
    pub domain: CircleDomain,
}

#[generate_trait]
pub impl CircleEvaluationImpl of CircleEvaluationTrait {
    fn new(domain: CircleDomain, bit_reversed_values: Array<QM31>) -> CircleEvaluation {
        CircleEvaluation { bit_reversed_values, domain }
    }
}

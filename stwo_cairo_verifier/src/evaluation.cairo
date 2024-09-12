use stwo_cairo_verifier::circle::CircleDomain;
use stwo_cairo_verifier::fields::m31::M31;
use stwo_cairo_verifier::fields::qm31::QM31;

/// An evaluation defined on a [`CircleDomain`].
///
/// The values are ordered according to the [`CircleDomain`] ordering.
#[derive(Debug, Clone, Drop)]
pub struct CircleEvaluationM31<EvalOrder> {
    pub domain: CircleDomain,
    pub values: Array<M31>,
}

#[generate_trait]
pub impl CircleEvaluationM31Impl<EvalOrder> of CircleEvaluationM31Trait<EvalOrder> {
    #[inline]
    fn new(domain: CircleDomain, values: Array<M31>) -> CircleEvaluationM31<EvalOrder> {
        CircleEvaluationM31 { domain, values }
    }
}

#[derive(Debug, Drop, PartialEq)]
pub struct CircleEvaluationQM31<EvalOrder> {
    pub domain: CircleDomain,
    pub values: Array<QM31>,
}

#[generate_trait]
pub impl CircleEvaluationQM31Impl<EvalOrder> of CircleEvaluationQM31Trait<EvalOrder> {
    #[inline]
    fn new(domain: CircleDomain, values: Array<QM31>) -> CircleEvaluationQM31<EvalOrder> {
        CircleEvaluationQM31 { domain, values }
    }
}

/// Marker type for bit-reversed evaluation ordering.
#[derive(Debug, Copy, Drop, PartialEq)]
pub struct BitReversedOrder {}

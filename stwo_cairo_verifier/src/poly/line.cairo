use stwo_cairo_verifier::circle::CirclePointM31Trait;
use core::option::OptionTrait;
use core::clone::Clone;
use core::result::ResultTrait;
use stwo_cairo_verifier::fields::m31::{M31, m31};
use stwo_cairo_verifier::utils::pow;
use stwo_cairo_verifier::circle::{
    Coset, CosetImpl, CirclePointM31, CirclePointM31Impl, M31_CIRCLE_GEN, CIRCLE_ORDER
};

#[derive(Copy, Drop, Debug)]
pub struct LineDomain {
    pub coset: Coset,
}


#[generate_trait]
pub impl LineDomainImpl of LineDomainTrait {
    fn new(coset: Coset) -> LineDomain {
        // TODO: Implement assertions.
        LineDomain { coset: coset }
    }

    fn double(self: @LineDomain) -> LineDomain {
        LineDomain { coset: self.coset.double() }
    }

    fn at(self: @LineDomain, index: usize) -> M31 {
        self.coset.at(index).x
    }

    fn log_size(self: @LineDomain) -> usize {
        *self.coset.log_size
    }

    fn size(self: @LineDomain) -> usize {
        self.coset.size()
    }
}

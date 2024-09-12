use stwo_cairo_verifier::circle::{CircleDomain, CircleDomainImpl, CosetImpl};
use stwo_cairo_verifier::utils::{pow, bit_reverse_index};

#[derive(Debug, PartialEq, Drop)]
pub struct SparseSubCircleDomain {
    pub domains: Array<SubCircleDomain>,
    pub large_domain_log_size: u32,
}

#[generate_trait]
pub impl SparseSubCircleDomainImpl of SparseSubCircleDomainTrait {
    fn size(self: @SparseSubCircleDomain) -> usize {
        let mut size = 0;
        for domain in self.domains.span() {
            size += domain.size();
        };
        size
    }
}

/// Represents a circle domain relative to a larger circle domain.
#[derive(Debug, Copy, PartialEq, Drop)]
pub struct SubCircleDomain {
    /// Specifies which subdomain within the larger domain.
    pub sub_domain_index: usize,
    pub log_size: u32,
}

#[generate_trait]
pub impl SubCircleDomainImpl of SubCircleDomainTrait {
    fn size(self: @SubCircleDomain) -> usize {
        pow(2, *self.log_size).into()
    }

    fn to_circle_domain(self: @SubCircleDomain, query_domain: @CircleDomain) -> CircleDomain {
        let domain_index = bit_reverse_index(
            *self.sub_domain_index * pow(2, *self.log_size), query_domain.log_size()
        );
        let initial_index = query_domain.index_at(domain_index);
        let half_coset = CosetImpl::new(initial_index, *self.log_size - 1);
        CircleDomainImpl::new(half_coset)
    }
}
// impl SubCircleDomain {
//     /// Returns the decommitment positions relative to a commitment on a larger domain.
//     pub fn to_decommitment_positions(&self) -> Vec<usize> {
//         (self.sub_domain_index << self.log_size..(self.sub_domain_index + 1) <<
//         self.log_size).collect()
//     }
// }



use super::utils::pow;


/// Represents a circle domain relative to a larger circle domain. The `initial_index` is the bit
/// reversed query index in the larger domain.
#[derive(Drop)]
pub struct SubCircleDomain {
    pub coset_index: usize,
    pub log_size: u32,
}


#[generate_trait]
pub impl SubCircleDomainImpl of SubCircleDomainTrait {
    fn to_decommitment_positions(self: @SubCircleDomain) -> Array<usize> {
        let mut res = array![];
        let start = *self.coset_index * pow(2, *self.log_size); 
        let end = (*self.coset_index + 1) * pow(2, *self.log_size);
        let i = start;
        while i < end {
            res.append(i);
        };
        res
    }
}

#[derive(Drop)]
pub struct SparseSubCircleDomain {
    pub domains: Array<SubCircleDomain>,
    pub large_domain_log_size: u32,
}

#[generate_trait]
pub impl SparseSubCircleDomainImpl of SparseSubCircleDomainTrait {
    fn flatten(self: @SparseSubCircleDomain) -> Span<usize> {
        let mut res = array![];
        let mut i = 0;
        while i < self.domains.len() {
            let positions = self.domains[i].to_decommitment_positions();
            let mut j = 0;
            while j < positions.len() {
                res.append(*positions[j]);
                j = j + 1;
            };
            i = i + 1;
        };
        res.span()
    }
}

use super::utils::pow;
use core::traits::Copy;
use core::nullable::{NullableTrait, match_nullable, FromNullableResult};
use core::dict::Felt252DictEntryTrait;


/// Represents a circle domain relative to a larger circle domain. The `initial_index` is the bit
/// reversed query index in the larger domain.
#[derive(Drop, Debug, Copy)]
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
        let mut i = start;
        while i < end {
            res.append(i);
            i = i + 1;
        };
        res
    }
}

#[derive(Drop, Debug)]
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


pub fn get_sparse_sub_circle_domain_dict(ref dictionary: Felt252Dict<Nullable<SparseSubCircleDomain>>, key: u32)
 -> SparseSubCircleDomain {
    let (entry, nullable_value) = dictionary.entry(key.into());
    match match_nullable(nullable_value) {
        FromNullableResult::Null => panic!("No value found"),
        FromNullableResult::NotNull(value) => {
            let previous_value = value.unbox();
            let copy = SparseSubCircleDomain { 
                domains: previous_value.domains.clone(),
                large_domain_log_size: previous_value.large_domain_log_size
            };
            dictionary = entry.finalize(NullableTrait::new(previous_value));

            copy
        }
    }
}
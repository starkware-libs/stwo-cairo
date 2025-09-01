use core::dict::{Felt252Dict, SquashedFelt252DictTrait};
use crate::channel::{Channel, ChannelTrait};
use crate::circle::CosetImpl;
use super::utils::{ArrayImpl, pow2};

/// An ordered set of query positions.
#[derive(Drop, Copy, Debug, PartialEq)]
pub struct Queries {
    /// Query positions sorted in ascending order.
    pub positions: Span<usize>,
    /// Size of the domain from which the queries were sampled.
    pub log_domain_size: u32,
}

#[generate_trait]
pub impl QueriesImpl of QueriesImplTrait {
    /// Returns an ascending list of query indices uniformly sampled over the range
    /// [0, 2^`log_query_size`).
    ///
    /// Panics if `log_domain_size` is >=32.
    fn generate(ref channel: Channel, log_domain_size: u32, n_queries: usize) -> Queries {
        let mut positions_dict: Felt252Dict<felt252> = Default::default();
        let mut n_dict_entries = 0;
        let domain_size: NonZero<u32> = pow2(log_domain_size).try_into().unwrap();
        while n_dict_entries != n_queries {
            let mut random_words = channel.draw_u32s();
            for word in random_words {
                let (_, position) = DivRem::div_rem(*word, domain_size);
                positions_dict.insert(position.into(), 0);
                n_dict_entries += 1;
                if n_dict_entries == n_queries {
                    break;
                }
            }
        }

        // A squashed dict's entries are sorted by key in ascending order.
        let dict_entries = positions_dict.squash().into_entries();
        let mut sorted_positions: Array<u32> = array![];

        for (position, _, _) in dict_entries {
            sorted_positions.append(position.try_into().unwrap());
        }

        Queries { positions: sorted_positions.span(), log_domain_size }
    }

    fn len(self: @Queries) -> usize {
        (*self.positions).len()
    }

    /// Calculates the matching query indices in a folded domain (i.e each domain point is doubled)
    /// given `self` (the queries of the original domain) and the number of folds between domains.
    fn fold(self: Queries, n_folds: u32) -> Queries {
        Queries {
            positions: get_folded_query_positions(self.positions, n_folds),
            log_domain_size: self.log_domain_size - n_folds,
        }
    }
}

/// Returns a deduped list of folded query positions.
///
/// # Panics
///
/// Panics if query positions is empty.
pub fn get_folded_query_positions(mut query_positions: Span<usize>, n_folds: u32) -> Span<usize> {
    let folding_factor = pow2(n_folds);
    let mut prev_folded_position = *query_positions.pop_front().unwrap() / folding_factor;
    let mut folded_positions = array![prev_folded_position];

    for position in query_positions {
        let folded_position = *position / folding_factor;

        if folded_position != prev_folded_position {
            folded_positions.append(folded_position);
            prev_folded_position = folded_position;
        }
    }

    folded_positions.span()
}

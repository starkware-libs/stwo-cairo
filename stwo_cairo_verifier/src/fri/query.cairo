use super::utils::pow;

#[derive(Drop)]
pub struct Queries {
    pub positions: Array<usize>,
    pub log_domain_size: u32,
}

#[generate_trait]
pub impl QueriesImpl of QueriesImplTrait {
    fn len(self: @Queries) -> usize {
        self.positions.len()
    }

    fn get_position(self: @Queries, i: u32) -> usize {
        *self.positions[i]
    }

    fn fold(self: @Queries, n_folds: u32) -> Queries {
        // TODO: implement
        // TODO: remove clone in Queries
        assert!(n_folds <= *self.log_domain_size);
        assert!(self.positions.len() > 0);
        let mut folding_factor = pow(2, n_folds);
        let mut new_last_position = *self.positions[0] / folding_factor;
        let mut new_positions = array![new_last_position];
        let mut i = 1;
        while i < self.positions.len() {
            let new_position = *self.positions[i] / folding_factor;
            if new_position != new_last_position {
                new_last_position = new_position;
                new_positions.append(new_last_position);
            }
            i += 1;
        };
        Queries {
            positions: new_positions,
            log_domain_size: *self.log_domain_size - n_folds
        }
    }
}

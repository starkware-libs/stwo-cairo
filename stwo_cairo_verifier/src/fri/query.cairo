#[derive(Drop, Clone)]
pub struct Queries {
    pub positions: Array<usize>,
    pub log_domain_size: u32,
}

#[generate_trait]
pub impl QueriesImpl of QueriesImplTrait {
    fn len(self: @Queries) -> usize {
        self.positions.len()
    }

    fn fold(self: @Queries, n_folds: u32) -> Queries {
        // TODO: implement and remove clone in Queries
        self.clone()
    }
}

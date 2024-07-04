use super::utils::pow;

#[derive(Drop, Debug, PartialEq, Eq)]
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
        assert!(n_folds <= *self.log_domain_size);
        assert!(self.positions.len() > 0);
        let folding_factor = pow(2, n_folds);
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
        Queries { positions: new_positions, log_domain_size: *self.log_domain_size - n_folds }
    }
}

#[test]
fn test_fold_1() {
    let queries = Queries { positions: array![5], log_domain_size: 6 };
    let result = queries.fold(1);
    let expected_result = Queries { positions: array![2], log_domain_size: 5 };
    assert_eq!(expected_result, result);
}

#[test]
fn test_fold_2() {
    let queries = Queries { positions: array![17, 27], log_domain_size: 6 };
    let result = queries.fold(1);
    let expected_result = Queries { positions: array![8, 13], log_domain_size: 5 };
    assert_eq!(expected_result, result);
}

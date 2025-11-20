use core::array::ArrayImpl;
use core::dict::Felt252Dict;
use core::nullable::NullableTrait;
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl};
use crate::fields::m31::m31;
use crate::fields::qm31::{QM31, qm31_const};
use crate::pcs::quotients::{
    ColumnSampleBatch, QuotientConstantsImpl, accumulate_row_quotients, fri_answers,
    fri_answers_for_log_size,
};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{
    DictImpl, group_columns_by_degree_bound, pad_and_transpose_columns_by_log_deg_bound_per_tree,
};

#[test]
fn test_fri_answers_for_log_size() {
    let log_size = 5;
    let p0 = qm31_circle_gen();
    let p1 = p0 + qm31_circle_gen();
    let p2 = p1 + qm31_circle_gen();

    let sample0 = qm31_const::<0, 1, 2, 3>();
    let sample1 = qm31_const::<1, 2, 3, 4>();
    let sample2 = qm31_const::<2, 3, 4, 5>();

    let sample_batches_by_point: Array<ColumnSampleBatch> = array![
        ColumnSampleBatch {
            point: p0, columns_and_values: array![(0, sample0), (1, sample0), (2, sample0)],
        },
        ColumnSampleBatch { point: p1, columns_and_values: array![(0, sample1)] },
        ColumnSampleBatch { point: p2, columns_and_values: array![(0, sample2), (2, sample2)] },
    ];

    let random_coeff = qm31_const::<9, 8, 7, 6>();
    let query_positions = array![4, 5, 6, 7].span();
    let col0_query_values = array![m31(1), m31(2), m31(3), m31(4)].span();
    let col1_query_values = array![m31(1), m31(1), m31(2), m31(3)].span();
    let col2_query_values = array![m31(1), m31(1), m31(1), m31(2)].span();
    let mut query_evals = array![col0_query_values, col1_query_values, col2_query_values].span();
    let n_columns = array![1, 1, 1];

    let res = fri_answers_for_log_size(
        log_size,
        sample_batches_by_point,
        random_coeff,
        query_positions,
        ref query_evals,
        n_columns,
    );

    assert!(
        res == array![
            qm31_const::<1751113829, 332344125, 1622434138, 1967872823>(),
            qm31_const::<37174731, 1816229254, 1292576490, 1183150403>(),
            qm31_const::<1998145805, 851520628, 952663142, 1250966585>(),
            qm31_const::<1450133329, 759548396, 1834647419, 2039849642>(),
        ]
            .span(),
    );
}

#[test]
fn test_fri_answers() {
    let log_blowup_factor = 2;
    let col0_degree_bound = 3;
    let col1_degree_bound = 5;
    let tree2_deg_bounds_by_column = array![col0_degree_bound, col1_degree_bound];
    let empty_span = array![].span();
    let columns_by_degree_bound_per_tree = array![
        empty_span, empty_span, group_columns_by_degree_bound(tree2_deg_bounds_by_column.span()),
    ]
        .span();
    let oods_point = qm31_circle_gen();

    let sample0 = qm31_const::<0, 1, 2, 3>();
    let sample1 = qm31_const::<1, 2, 3, 4>();
    let col0_samples = array![sample0, sample1].span();
    let col1_samples = array![sample0].span();
    let empty_span = array![].span();
    let samples_per_column_per_tree = array![
        empty_span, empty_span, array![col0_samples, col1_samples].span(),
    ]
        .span();

    let random_coeff = qm31_const::<9, 8, 7, 6>();
    let col0_query_positions = array![4, 5].span();
    let col1_query_positions = array![6, 7].span();
    let mut query_domain_per_log_size: Felt252Dict = Default::default();
    query_domain_per_log_size.insert(5, NullableTrait::new(col0_query_positions));
    query_domain_per_log_size.replace(7, NullableTrait::new(col1_query_positions));
    let empty_span = array![].span();
    let query_evals = array![empty_span, empty_span, array![m31(3), m31(7), m31(9), m31(2)].span()];

    let column_indices_per_tree_by_degree_bound =
        pad_and_transpose_columns_by_log_deg_bound_per_tree(
        columns_by_degree_bound_per_tree,
    );

    let res = fri_answers(
        column_indices_per_tree_by_degree_bound,
        log_blowup_factor,
        oods_point,
        samples_per_column_per_tree,
        random_coeff,
        query_domain_per_log_size,
        query_evals,
    );

    assert!(res.len() == 2);
    assert!(
        res[0] == @array![
            qm31_const::<0, 0, 656830455, 182280913>(), qm31_const::<0, 0, 874296291, 243916229>(),
        ]
            .span(),
    );
    assert!(
        res[1] == @array![
            qm31_const::<1773248131, 739472855, 1101315608, 1566614904>(),
            qm31_const::<1860417161, 220971565, 1674891473, 1353808926>(),
        ]
            .span(),
    );
}

#[test]
fn test_accumulate_row_quotients() {
    let alpha = qm31_const::<4, 3, 2, 1>();
    let domain = CircleDomainImpl::new(CosetImpl::new(CirclePointIndexImpl::new(1), 0));
    let queried_values_at_row = array![m31(5), m31(1)].span();
    let p0 = qm31_circle_gen();
    let p1 = qm31_circle_gen() + qm31_circle_gen();
    let sample_batches = array![
        ColumnSampleBatch {
            point: p0, columns_and_values: array![(0, qm31_const::<0, 1, 2, 3>())],
        },
        ColumnSampleBatch {
            point: p1, columns_and_values: array![(1, qm31_const::<1, 2, 3, 4>())],
        },
    ];
    let quotient_constants = QuotientConstantsImpl::gen(@sample_batches, alpha);

    let res = accumulate_row_quotients(
        @sample_batches, queried_values_at_row, @quotient_constants, domain.at(0),
    );

    assert_eq!(res, qm31_const::<62788819, 954251344, 1656594753, 2134380397>());
}

// Test used to benchmark step counts.
#[test]
fn test_fri_answers_with_1000_columns() {
    // NOTE: Forge fails if these are declared `const ...`.
    let log_size: u32 = 16;
    let n_queries: usize = 20;
    let n_columns: usize = 1000;
    let random_coeff = qm31_const::<9, 8, 7, 6>();
    assert!(n_columns >= 3, "First three columns are manually created");
    let mut query_positions = array![];
    for query_position in 0..n_queries {
        query_positions.append(query_position);
    }
    let p0 = qm31_circle_gen();
    let p1 = p0 + qm31_circle_gen();
    let p2 = p1 + qm31_circle_gen();
    let sample0 = qm31_const::<0, 1, 2, 3>();
    let sample1 = qm31_const::<1, 2, 3, 4>();
    let sample2 = qm31_const::<2, 3, 4, 5>();

    let mut query_values = array![];
    for i in 0..n_queries {
        for _ in 0..n_columns {
            query_values.append(m31(i));
        }
    }

    let mut p0_column_and_values = array![];
    for column_index in 0..n_columns {
        p0_column_and_values.append((column_index, sample0));
    }

    let sample_batches_by_point: Array<ColumnSampleBatch> = array![
        ColumnSampleBatch { point: p0, columns_and_values: p0_column_and_values },
        ColumnSampleBatch { point: p1, columns_and_values: array![(0, sample1)] },
        ColumnSampleBatch { point: p2, columns_and_values: array![(0, sample2), (2, sample2)] },
    ];

    let n_columns = array![0, n_columns, 0];
    let mut query_evals = array![array![].span(), query_values.span(), array![].span()].span();

    let _res = fri_answers_for_log_size(
        log_size,
        sample_batches_by_point,
        random_coeff,
        query_positions.span(),
        ref query_evals,
        n_columns,
    );
}

/// Returns a generator for the circle group over [`QM31`].
fn qm31_circle_gen() -> CirclePoint<QM31> {
    let x = qm31_const::<0x1, 0x0, 0x1C876E93, 0x1E9CA77B>();
    let y = qm31_const::<0x3B25121B, 0x26B12487, 0x2C1E6D83, 0x46B9D720>();
    CirclePoint { x, y }
}

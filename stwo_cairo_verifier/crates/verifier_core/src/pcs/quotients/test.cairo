use core::array::ArrayImpl;
use core::dict::Felt252Dict;
use core::nullable::NullableTrait;
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl};
use crate::fields::m31::m31;
use crate::fields::qm31::{PackedUnreducedQM31Trait, QM31, qm31_const};
use crate::pcs::quotients::{
    ColumnSampleBatch, ColumnSampleBatchImpl, ComplexConjugateLineCoeffsImpl, PointSample,
    QuotientConstantsImpl, accumulate_row_quotients, fri_answers, fri_answers_for_log_size,
};
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{DictImpl, group_columns_by_log_size};

#[test]
fn test_fri_answers_for_log_size() {
    let log_size = 5;
    let p0 = qm31_circle_gen();
    let p1 = p0 + qm31_circle_gen();
    let p2 = p1 + qm31_circle_gen();
    let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
    let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
    let sample2 = PointSample { point: p2, value: qm31_const::<2, 3, 4, 5>() };
    let col0_samples = array![sample0, sample1, sample2];
    let col1_samples = array![sample0];
    let col2_samples = array![sample0, sample2];
    let samples_by_column = array![@col0_samples, @col1_samples, @col2_samples];
    let random_coeff = qm31_const::<9, 8, 7, 6>();
    let query_positions = array![4, 5, 6, 7].span();
    let col0_query_values = array![m31(1), m31(2), m31(3), m31(4)].span();
    let col1_query_values = array![m31(1), m31(1), m31(2), m31(3)].span();
    let col2_query_values = array![m31(1), m31(1), m31(1), m31(2)].span();
    let mut query_evals = array![col0_query_values, col1_query_values, col2_query_values];
    let n_columns = array![1, 1, 1];

    let res = fri_answers_for_log_size(
        log_size, samples_by_column, random_coeff, query_positions, ref query_evals, n_columns,
    );

    assert!(
        res == array![
            qm31_const::<1791980583, 1709376644, 1911116353, 1204412580>(),
            qm31_const::<1417689272, 1640898968, 1760036812, 1705156550>(),
            qm31_const::<503725777, 621939055, 1324380556, 1450763049>(),
            qm31_const::<1895961752, 170000503, 1562444038, 1465755799>(),
        ]
            .span(),
    );
}

#[test]
fn test_fri_answers() {
    let col0_log_size = 5;
    let col1_log_size = 7;
    let tree2_log_sizes = array![col0_log_size, col1_log_size];
    let empty_span = array![].span();
    let columns_by_log_sizes_per_tree = array![
        empty_span, empty_span, group_columns_by_log_size(tree2_log_sizes.span()),
    ]
        .span();
    let p0 = qm31_circle_gen();
    let p1 = qm31_circle_gen() + qm31_circle_gen();
    let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
    let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
    let col0_samples = array![sample0, sample1];
    let col1_samples = array![sample0];
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

    let res = fri_answers(
        columns_by_log_sizes_per_tree,
        samples_per_column_per_tree,
        random_coeff,
        query_domain_per_log_size,
        query_evals,
    );

    assert!(
        res == array![
            array![
                qm31_const::<1791306293, 1053124067, 158259497, 452720916>(),
                qm31_const::<212478330, 1383090185, 1622369493, 599681801>(),
            ]
                .span(),
            array![
                qm31_const::<834593128, 54438530, 120431711, 2027138945>(),
                qm31_const::<1820575540, 1615656673, 695030281, 674192396>(),
            ]
                .span(),
        ],
    );
}

#[test]
fn test_complex_conjugate_line_coeffs_impl() {
    let point = qm31_circle_gen();
    let value = qm31_const::<9, 8, 7, 6>();
    let alpha = qm31_const::<2, 3, 4, 5>();

    let res = ComplexConjugateLineCoeffsImpl::new(@point, value, alpha);

    assert!(res.alpha_mul_a.reduce() == qm31_const::<126, 2147483415, 8, 2147483581>());
    assert!(res.alpha_mul_b.reduce() == qm31_const::<20238140, 1378415613, 17263450, 142791233>());
    assert!(
        res.alpha_mul_c.reduce() == qm31_const::<865924731, 72415967, 2011255989, 1549931113>(),
    );
}

#[test]
fn test_column_sample_batch_group_by_point() {
    let p0 = qm31_circle_gen();
    let p1 = p0 + qm31_circle_gen();
    let p2 = p1 + qm31_circle_gen();
    let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
    let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
    let sample2 = PointSample { point: p2, value: qm31_const::<2, 3, 4, 5>() };
    let col0_samples = array![sample0, sample1, sample2];
    let col1_samples = array![sample0];
    let col2_samples = array![sample0, sample2];
    let samples_per_column = array![@col0_samples, @col1_samples, @col2_samples];

    let grouped_samples = ColumnSampleBatchImpl::group_by_point(samples_per_column);

    assert!(
        grouped_samples == array![
            ColumnSampleBatch {
                point: sample0.point,
                columns_and_values: array![
                    (0, @sample0.value), (1, @sample0.value), (2, @sample0.value),
                ],
            },
            ColumnSampleBatch {
                point: sample1.point, columns_and_values: array![(0, @sample1.value)],
            },
            ColumnSampleBatch {
                point: sample2.point,
                columns_and_values: array![(0, @sample2.value), (2, @sample2.value)],
            },
        ],
    )
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
            point: p0, columns_and_values: array![(0, @qm31_const::<0, 1, 2, 3>())],
        },
        ColumnSampleBatch {
            point: p1, columns_and_values: array![(1, @qm31_const::<1, 2, 3, 4>())],
        },
    ];
    let quotient_constants = QuotientConstantsImpl::gen(@sample_batches, alpha);

    let res = accumulate_row_quotients(
        @sample_batches, queried_values_at_row, @quotient_constants, domain.at(0),
    );

    assert_eq!(res, qm31_const::<545815778, 838613809, 1761463254, 2019099482>());
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
    let sample0 = PointSample { point: p0, value: qm31_const::<0, 1, 2, 3>() };
    let sample1 = PointSample { point: p1, value: qm31_const::<1, 2, 3, 4>() };
    let sample2 = PointSample { point: p2, value: qm31_const::<2, 3, 4, 5>() };
    let col0_samples = array![sample0, sample1, sample2];
    let col1_samples = array![sample0];
    let col2_samples = array![sample0, sample2];
    let mut samples_per_column = array![@col0_samples, @col1_samples, @col2_samples];
    let mut query_values = array![];

    for i in 0..n_queries {
        for _ in 0..n_columns {
            query_values.append(m31(i));
        }
    }
    for _ in samples_per_column.len()..n_columns {
        samples_per_column.append(@col1_samples);
    }

    let n_columns = array![0, n_columns, 0];
    let mut query_evals = array![array![].span(), query_values.span(), array![].span()];

    let _res = fri_answers_for_log_size(
        log_size,
        samples_per_column,
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

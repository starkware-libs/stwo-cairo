use core::array;
use core::array::ArrayImpl;
use core::box::BoxImpl;
use core::num::traits::One;
use crate::circle::{CirclePoint, CirclePointIndexImpl, CosetImpl};
use crate::fields::m31::m31;
use crate::fields::qm31::{QM31, qm31_const};
use crate::pcs::quotients::{ColumnSampleBatch, QuotientConstantsImpl, fri_answers};
use crate::pcs::verifier::CommitmentSchemeVerifierImpl;
use crate::poly::circle::{CanonicCosetImpl, CircleDomainImpl, CircleEvaluationImpl};
use crate::utils::{
    DictImpl, group_columns_by_degree_bound, pad_and_transpose_columns_by_log_deg_bound_per_tree,
};
use crate::vcs::blake2s_hasher::Blake2sMerkleHasher;
use super::{CanonicCosetTrait, accumulate_row_quotients};

/// Expected values were generated in the stwo repo at stwo/src/core/pcs/quotients, commit
/// 0eb4a52af8a3317e040aef3438251f85e4da1693.
#[test]
fn test_fri_answers() {
    let log_blowup_factor = 2;
    let col0_degree_bound = 3;
    let col1_degree_bound = 5;
    let col2_degree_bound = col1_degree_bound;
    let oods_point = qm31_circle_gen();

    let sample0 = qm31_const::<0, 1, 2, 3>();
    let sample1 = qm31_const::<1, 2, 3, 4>();
    let col0_samples = array![sample1, sample0].span();
    let col1_samples = array![sample0].span();
    let col2_samples = array![sample1, sample0].span();
    let empty_span = array![].span();
    let samples_per_column_per_tree = array![
        empty_span, empty_span, array![col0_samples, col1_samples, col2_samples].span(),
    ]
        .span();
    let columns_by_degree_bound_per_tree = array![
        array![].span(), array![].span(),
        group_columns_by_degree_bound(
            array![col0_degree_bound, col1_degree_bound, col2_degree_bound].span(),
        ),
    ]
        .span();

    let columns_per_tree_by_degree_bound = pad_and_transpose_columns_by_log_deg_bound_per_tree(
        columns_by_degree_bound_per_tree,
    );

    let random_coeff = qm31_const::<9, 8, 7, 6>();
    let query_positions = array![4, 5].span();
    let empty_span = array![].span();
    let query_evals = array![
        empty_span, empty_span, array![m31(3), m31(7), m31(9), m31(2), m31(4), m31(10)].span(),
    ];

    let res = fri_answers(
        columns_per_tree_by_degree_bound,
        log_blowup_factor,
        oods_point,
        samples_per_column_per_tree,
        random_coeff,
        query_positions,
        query_evals,
        col1_degree_bound,
    );
    assert!(res.len() == 2);
    assert!(*res[0] == qm31_const::<131767160, 1083041990, 899288613, 1431425829>());
    assert!(*res[1] == qm31_const::<113115484, 414784867, 764329603, 1012622739>());
}

#[test]
fn test_accumulate_row_quotients() {
    let random_coeff = qm31_const::<4, 3, 2, 1>();
    let log_size = 5;
    let domain = CanonicCosetImpl::new(log_size);
    let queried_values_at_row = array![m31(5), m31(1)].span();
    let p0 = qm31_circle_gen();
    let p1 = qm31_circle_gen() + qm31_circle_gen();
    let sample_batches = array![
        ColumnSampleBatch {
            point: p0, cols_vals_and_pows: array![(0, qm31_const::<0, 1, 2, 3>(), One::one())],
        },
        ColumnSampleBatch {
            point: p1, cols_vals_and_pows: array![(1, qm31_const::<1, 2, 3, 4>(), random_coeff)],
        },
    ]
        .span();
    let quotient_constants = QuotientConstantsImpl::gen(sample_batches);

    let res = accumulate_row_quotients(
        sample_batches, queried_values_at_row, @quotient_constants, domain.circle_domain().at(0),
    );
    assert_eq!(res, qm31_const::<1090243653, 141518822, 29401430, 491190325>());
}

// Test used to benchmark step counts.
#[test]
fn test_fri_answers_with_1000_columns() {
    // NOTE: Forge fails if these are declared `const ...`.
    let log_degree_bound: u32 = 15;
    let n_queries: usize = 20;
    let n_columns: usize = 1000;
    let random_coeff = qm31_const::<9, 8, 7, 6>();
    assert!(n_columns >= 3, "First three columns are manually created");
    let mut query_positions = array![];
    for query_position in 0..n_queries {
        query_positions.append(query_position);
    }

    let sample0 = qm31_const::<0, 1, 2, 3>();
    let sample1 = qm31_const::<1, 2, 3, 4>();
    let sample2 = qm31_const::<2, 3, 4, 5>();

    let mut query_values = array![];
    for i in 0..n_queries {
        for _ in 0..n_columns {
            query_values.append(m31(i));
        }
    }

    let mut samples = array![];
    // Manually add samples for the first three columns.
    samples.append(array![sample0].span());
    samples.append(array![sample1].span());
    samples.append(array![sample0, sample2].span());
    for _ in 3..n_columns {
        samples.append(array![sample0].span())
    }

    let mut size_vector = array![];
    for _ in 0..n_columns {
        size_vector.append(log_degree_bound)
    }
    let columns_by_degree_bound_per_tree = array![
        array![].span(), group_columns_by_degree_bound(size_vector.span()), array![].span(),
    ]
        .span();
    let columns_per_tree_by_degree_bound = pad_and_transpose_columns_by_log_deg_bound_per_tree(
        columns_by_degree_bound_per_tree,
    );
    let mut queried_values_per_tree = array![array![].span(), query_values.span(), array![].span()];
    let oods_point = qm31_circle_gen();
    let log_blowup_factor = 1;
    let _res = fri_answers(
        columns_per_tree_by_degree_bound,
        log_blowup_factor,
        oods_point,
        array![array![].span(), samples.span(), array![].span()].span(),
        random_coeff,
        query_positions.span(),
        queried_values_per_tree,
        log_degree_bound,
    );
}

/// Returns a generator for the circle group over [`QM31`].
fn qm31_circle_gen() -> CirclePoint<QM31> {
    let x = qm31_const::<0x1, 0x0, 0x1C876E93, 0x1E9CA77B>();
    let y = qm31_const::<0x3B25121B, 0x26B12487, 0x2C1E6D83, 0x46B9D720>();
    CirclePoint { x, y }
}

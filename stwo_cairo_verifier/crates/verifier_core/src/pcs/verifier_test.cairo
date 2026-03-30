use super::verifier::prepare_preprocessed_query_positions;

#[test]
fn test_pp_queries_lift() {
    let positions = array![0, 107, 214, 321, 428, 535, 642, 749, 856, 963].span();
    // Compute `pos >> (5 + 1) << 1 + (pos & 1)`
    assert_eq!(
        prepare_preprocessed_query_positions(positions, 15, 10),
        array![0, 3, 6, 11, 12, 17, 20, 23, 26, 31].span(),
    );
    // Positions are not modified if `lifting_log_size == pp_max_log_size`.
    assert_eq!(prepare_preprocessed_query_positions(positions, 10, 10), positions);
    // Compute `pos >> 1 << (5 + 1) + (pos & 1)`
    assert_eq!(
        prepare_preprocessed_query_positions(positions, 10, 15),
        array![0, 3393, 6848, 10241, 13696, 17089, 20544, 23937, 27392, 30785].span(),
    );
}

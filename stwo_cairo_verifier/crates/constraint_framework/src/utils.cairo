use stwo_verifier_core::TreeArray;

pub fn tree_array_concat_cols(tree_array: Array<TreeArray<Span<u32>>>) -> TreeArray<Span<u32>> {
    let mut tree0 = array![];
    let mut tree1 = array![];
    let mut tree2 = array![];

    for curr_tree in tree_array.span() {
        // TODO: Instead of changing this to make it generic just refactor so the
        // whole function can be removed.
        assert!(curr_tree.len() <= 3);

        if curr_tree.len() > 0 {
            tree0.append_span(*curr_tree[0]);
        }
        if curr_tree.len() > 1 {
            tree1.append_span(*curr_tree[1]);
        }
        if curr_tree.len() > 2 {
            tree2.append_span(*curr_tree[2]);
        }
    }

    array![tree0.span(), tree1.span(), tree2.span()]
}

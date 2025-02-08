use path_oram::tree::Tree;
const Z: usize = 4; // Bucket size
const N: usize = 16; // Block data size
const L: usize = 3; // Example height

#[test]
fn test_tree_initialization() {
    let tree = Tree::<L, N, Z>::new();
    let expected_size = (1 << (L + 1)) - 1; // 2^(L+1) - 1

    assert_eq!(tree.tree.len(), expected_size, "Tree should have the correct number of nodes");
    
    // Ensure all buckets are empty
    for bucket in tree.tree.iter() {
        assert!(bucket.storage.iter().all(|block| block.dummy), "All blocks should be dummies in a new tree");
    }
}

#[test]
fn test_calc_path() {
    let tree = Tree::<L, N, Z>::new();
    
    let leaf_index = 5; // Example index
    let path = tree.calc_path(leaf_index);

    assert!(path.len() > 0, "Path should not be empty");
    assert_eq!(path[path.len() - 1], tree.tree[0], "Last element in path should be root");
}
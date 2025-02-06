use path_oram::bucket::{Bucket, Block};  // Use crate to access the bucket module

#[test]
fn test_create_block() {
    let block = Block::<10>::new(1, [1; 10], false);
    assert_eq!(block.id, 1);
    assert_eq!(block.data, [1; 10]);
    assert_eq!(block.dummy, false);
}


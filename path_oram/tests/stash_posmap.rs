use path_oram::stash::Stash;
use path_oram::posmap::PosMap;
use path_oram::bucket::Block;

#[test]
fn test_stash_insert_and_get() {
    let mut stash = Stash::new();
    let block = Block::new(1, [1; 8], false); // Assuming Block has an ID and data
    
    stash.insert(block.clone());
    
    // Ensure the block exists
    let retrieved = stash.get(1);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, 1);
}

#[test]
fn test_stash_remove() {
    let mut stash = Stash::new();
    let block = Block::new(2,[99; 8], false);

    stash.insert(block.clone());
    let removed = stash.evict(2);
    
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().id, 2);
    
    // Ensure it no longer exists
    assert!(stash.get(2).is_none());
}

#[test]
fn test_posmap_insert_and_lookup() {
    let mut pos_map = PosMap::new();
    
    pos_map.set(1, 42);
    assert_eq!(pos_map.get(1), Some(42));
}

#[test]
fn test_posmap_remove() {
    let mut pos_map = PosMap::new();
    
    pos_map.set(3, 100);
    assert_eq!(pos_map.get(3), Some(100));
    
    pos_map.remove(3);
    assert_eq!(pos_map.get(3), None);
}

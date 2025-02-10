use std::collections::HashMap;
use crate::bucket::Block;

pub struct Stash<const N: usize> {
    storage: HashMap<u64, Block<N>>,
}

impl<const N: usize> Stash<N> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn insert(&mut self, block: Block<N>) {
        self.storage.insert(block.id, block);
    }

    pub fn evict(&mut self, id: u64) -> Option<Block<N>> {
        self.storage.remove(&id)
    }

    pub fn get(&self, id: u64) -> Option<&Block<N>> {
        self.storage.get(&id)
    }
}
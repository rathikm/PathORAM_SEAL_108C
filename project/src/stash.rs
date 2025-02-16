use std::collections::HashMap;
use crate::bucket::Block;

pub struct Stash<const N: usize> {
    pub storage: Vec<Block<N>>,
}

impl<const N: usize> Stash<N> {
    pub fn new() -> Self {
        Self {
            storage: vec![],
        }
    }

    pub fn insert(&mut self, block: Block<N>) {
        self.storage.push(block);
    }

    pub fn evict(&mut self, address: u64) {
        self.storage.retain(|block| block.address != address);
    }

    pub fn get(&self, address: u64) -> Option<&Block<N>> {
        self.storage.iter().find(|block| block.address == address)
    }

    pub fn get_mut(&mut self, address: u64) -> Option<&mut Block<N>> {
        self.storage.iter_mut().find(|block| block.address == address)
    }
    

    pub fn cut(&mut self, n: usize) {
        self.storage.truncate(n);
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}
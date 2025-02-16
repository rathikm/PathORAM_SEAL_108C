use std::collections::HashMap;
use rand::Rng;

pub struct PosMap {
    pub map: HashMap<u64, u64>,
}

impl PosMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, address: u64) -> Option<u64> {
        self.map.get(&address).copied()
    }

    pub fn set(&mut self, address: u64, leaf: u64) -> Option<u64>{
        self.map.insert(address, leaf)
    }

    pub fn remove(&mut self, address: u64) {
        self.map.remove(&address);
    }

    pub fn leaf_rand_assign(&mut self, address: u64, n_leaves: u64) {
        let rand_leaf = rand::thread_rng().gen_range(0..n_leaves);
        self.set(address, rand_leaf);
    }
}
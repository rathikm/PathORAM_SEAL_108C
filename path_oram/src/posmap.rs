use std::collections::HashMap;
use rand::Rng;

pub struct PosMap {
    map: HashMap<u64, u64>,
}

impl PosMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, id: u64) -> Option<u64> {
        self.map.get(&id).copied()
    }

    pub fn set(&mut self, id: u64, leaf: u64) -> Option<u64>{
        self.map.insert(id, leaf)
    }

    pub fn remove(&mut self, id: u64) {
        self.map.remove(&id);
    }

    pub fn leaf_rand_assign(&mut self, id: u64, n_leaves: u64) {
        let rand_leaf = rand::thread_rng().gen_range(0..n_leaves);
        self.set(id, rand_leaf);
    }
}
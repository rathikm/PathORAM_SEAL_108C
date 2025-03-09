use rand::Rng;
use std::collections::HashMap;
use std::cmp::min;
use crate::bucket::{Bucket, Block};
use crate::posmap::PosMap;
use crate::stash::Stash;
use crate::tree::Tree; 

pub const Z: usize = 4; //bucket size (Z blocks per bucket)
pub const L: usize = 3; //tree height
pub const N: usize = 8; //block size (N u8s per block)

pub struct ORAM {
    pub tree: Tree<L, N, Z>, 
    pub stash: Stash<N>,
    pub position: PosMap,
    pub key_leaf: HashMap<u64, Vec<u64>>,
    pub padding_param: u64,
}


impl ORAM {
    // Create a new ORAM initalizing an empty stash and position map.
    // Tree is initalized to contain L buckets with dumy data in them 
    pub fn new() -> Self {
        ORAM {
            tree: Tree::new(),
            stash: Stash::new(),
            position: PosMap::new(),
            key_leaf: HashMap::new(),
            padding_param: 2,
        }
    }

    pub fn init(&mut self) {
        //tree height is L
        //complete binary tree -> 2^(L+1)-1 buckets
        //multiply by Z to get # of blocks = B
        //0..B-1 is the address space
        let mut rng = rand::thread_rng();
        let B = self.tree.tree.len() * Z;
        for i in 0..B {
            let rand_leaf = rng.gen_range(0..self.tree.num_leaves());
            self.position.set(i as u64, rand_leaf as u64, 0 as u64);
        }

        for (&address, &(leaf, value)) in &self.position.map {
            let path_indices = self.tree.calc_path_indices(leaf as usize);
            let mut inserted = false;
            
            for bucket_ind in path_indices.iter().rev() {
                let real_block: Block<N> = Block::new(address, [0; N], false);
                if self.tree.tree[*bucket_ind].insert(real_block) {
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                let real_block: Block<N> = Block::new(address, [0; N], false);
                self.stash.insert(real_block);
            }
        }
    }

    pub fn addr_space_len(&self) -> u64 {
        (self.tree.tree.len() * Z) as u64
    }

    pub fn write_record(&mut self, key: u64, value: &str, address: u64) {
        self.key_leaf.entry(key).or_insert_with(Vec::new).push(address);
        let bytes = value.as_bytes();
        let mut arr = [0u8; N];
        // Copy the bytes into the array; the rest remains as zeros.
        arr[..bytes.len()].copy_from_slice(bytes);
        self.access("write".to_string(), address, arr);
    }

    pub fn read_records(&mut self, key: u64) -> Vec<String> {
        // If the key doesn't exist, default to an empty vector.
        let addresses: Vec<u64> = self.key_leaf.get(&key).cloned().unwrap_or_default();
        let mut res: Vec<[u8; N]> = Vec::new();
    
        for a in addresses {
            // Assuming self.access returns a [u8; N] array for the given address.
            res.push(self.access("read".to_string(), a, [0u8; N]));
        }

        //calculate padding for dummy records
        let original_len = res.len() as f64;
        let base: f64 = self.padding_param as f64;
        let padded_exp: u64 = original_len.log(base).ceil() as u64;
        let padded: u64 = 1 << padded_exp;

        let dummies = padded - (original_len as u64);

        for i in 0..dummies {
            res.push([0u8; N]);
        }
        
        res.iter()
            .map(|arr| {
                // Remove trailing zeros.
                let trimmed = arr.iter()
                    .cloned()
                    .take_while(|&b| b != 0)
                    .collect::<Vec<u8>>();
                // Convert the trimmed bytes to a String (assuming valid UTF-8).
                std::str::from_utf8(&trimmed)
                    .expect("Invalid UTF-8")
                    .to_string()
            })
            .collect()
    }

    fn read_bucket(&self, bucket: &Bucket<Z, N>) -> [Block<N>; Z]{
        bucket.storage.clone()
    }

    fn write_bucket(&self, bucket: &mut Bucket<Z, N>, blocks: Vec<Block<N>>) {
        let data_len = blocks.len();
        for i in 0..blocks.len() {
            bucket.write(blocks[i], i);
        }       
        let dummies = Z - data_len;
        for i in 0..dummies {
            bucket.write(Block::empty(), data_len + i);
        }        
    }

    pub fn access_key_val(&mut self, op: String, address: u64, key: u64, val: &str) -> [u8; N] {
        let bytes = val.as_bytes();
        if bytes.len() > N {
            return [0; N];
        }
        let mut arr = [0u8; N];
        // Copy the bytes into the array; the rest remains as zeros.
        arr[..bytes.len()].copy_from_slice(bytes);
        if op == "write" {
            if let Some(&(leaf, old_key)) = self.position.map.get(&address) {
                self.position.map.insert(address, (leaf, key));
            }
        }
        
        self.access(op, address, arr)
    }

    pub fn access(&mut self, op: String, address: u64, data_new: [u8; N]) -> [u8; N]{
        // Check if the address a is already in the position map. If it is, get the position
        // Otherwise add it to the position map and get that value?
        let x: (u64, u64) = self.position.get(address).unwrap_or((0, 0));
        self.position.leaf_rand_assign(address, (1 << L) - 1, x.1);

        //contains the buckets in the path from leaf x to root,
        //leaf first
        let path_x: Vec<Bucket<Z, N>> = self.tree.calc_path(x.0 as usize); 
        for bucket in path_x.iter().rev() {
            let blocks = self.read_bucket(bucket);
            for block in blocks {
                self.stash.insert(block);
            }
        }
        //stash should now contain blocks from each bucket in path
        let data: [u8; N] = self.stash.get(address).unwrap().data.clone();
        if op == "write" {
            //dont need to evict just edit data (resolved)
            self.stash.get_mut(address).unwrap().set_data(data_new);
        }

        //eviction procedure
        let mut alt_stash = Stash::new();
        for i in L..0 {
            let mut buck: Bucket<Z, N> = path_x[i];
            for block in self.stash.storage.iter() {
                print!("{}\n",block.address);
                let other_path = self.tree.calc_path(self.position
                                                    .get(block.address).
                                                    unwrap_or((0, 0)).0 as usize);
                let other_buck = other_path[i];
                if buck == other_buck {
                    alt_stash.insert(block.clone());
                }
            }
            let alt_len = alt_stash.len();
            let mini = min(alt_len, Z);
            alt_stash.cut(mini);
            self.stash.storage.retain(|block| !alt_stash.storage.contains(block));
            self.write_bucket(&mut buck, alt_stash.storage.clone());
        }
        data
      
    }

}


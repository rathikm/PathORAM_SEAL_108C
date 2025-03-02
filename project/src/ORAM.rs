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
}


impl ORAM {
    // Create a new ORAM initalizing an empty stash and position map.
    // Tree is initalized to contain L buckets with dumy data in them 
    pub fn new() -> Self {
        ORAM {
            tree: Tree::new(),
            stash: Stash::new(),
            position: PosMap::new()
        }
    }

    pub fn init(&mut self) {
        //tree height is L
        //complete binary tree -> 2^(L+1)-1 buckets
        //multiply by Z to get # of blocks = B
        //0..B-1 is the address space
        let mut rng = rand::thread_rng();
        let B = self.tree.tree.len();
        for i in 0..B {
            let rand_leaf = rng.gen_range(0..self.tree.num_leaves());
            self.position.set(i as u64, rand_leaf as u64);
        }

        for (&address, &leaf) in &self.position.map {
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

    pub fn access(&mut self, op: String, address: u64, data_new: [u8; N]) -> [u8; N]{
        // Check if the address a is already in the position map. If it is, get the position
        // Otherwise add it to the position map and get that value?
        let x = self.position.get(address).unwrap_or(0);
        self.position.leaf_rand_assign(address, (1 << L) - 1);

        //contains the buckets in the path from leaf x to root,
        //leaf first
        let path_x: Vec<Bucket<Z, N>> = self.tree.calc_path(x as usize); 
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
                                                    unwrap_or(0) as usize);
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


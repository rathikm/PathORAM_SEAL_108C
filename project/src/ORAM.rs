use rand::Rng;
use std::collections::HashMap;
use crate::bucket::Bucket;
use crate::bucket::Block;
use crate::stash::Stash;
use crate::posmap::PosMap;
use crate::tree::Tree;


const Z: usize = 4; // Change based on desired capacity
const L: usize = 10; // Change based on desired capacity of tree 
const N: usize = 4;


pub struct Oram {
    tree: Tree<L,N,Z>,
    stash: Stash<N>,
    position_map: PosMap,
}


impl Oram {

    // Create a new ORAM initalizing an empty stash and position map.
    // Tree is initalized to contain L buckets with dumy data in them 
    fn new() -> Self {
        Oram {
            tree: Tree::new(),
            stash: Stash::new(),
            position_map: PosMap::new(),
        }
    }
    fn access(&mut self, op: String, address: u64, data: Vec<u8>) -> i8 {

        //My attempt at making access in a readable manner. Not sure if this will work but I think readibility is important

        // Line 1-3
        let x = self.position_map.get(address);
        let xStar = rand::thread_rng().gen_range(0..(1 << L));
        self.position_map.set(address, xStar);

        // Line 3-5
        // for l in 0..=L {
        //     let bucket_index = self.get_bucket_index(x, l);
        //     for block in self.tree[bucket_index].blocks {
        //         if block.valid {
        //             self.stash.push(block.clone());
        //         }
        //     }
        // }

        // let x = if let Some(&pos) = self.position_map.get(&address) {
        //     pos
        // } else {
        //     let new_pos = rand::thread_rng().gen_range(0..(1 << L));
        //     self.position_map.insert(address, new_pos);
        //     new_pos
        // };
        // // Iterate over the tree and add all of the blocks in bucket to stash
        // for l in 0..=L {
        //     let bucket_index = self.get_bucket_index(x, l);
        //     for block in &self.tree[bucket_index].blocks {
        //         if block.valid {
        //             self.stash.push(block.clone());
        //         }
        //     }
        // }
        // // I don't understand 
        // let result = self.stash
        //     .iter()
        //     .find(|block| block.address == address)
        //     .map(|block| block.data.clone())
        //     .unwrap_or_else(|| vec![]);


        // if op == "write" {
        //     self.stash.retain(|block| block.address != address);
        //     self.stash.push(Block { valid: true, address, data });
        // }
        0
    }
    fn get_bucket_index(&self, x: usize, l: usize) -> usize {
        // TODO: Implement the logic to get the bucket index based on x and l
        // How does this work again?
        0
    }

}


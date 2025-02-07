use rand::Rng;
use std::collections::HashMap;

const Z: usize = 4; // Change based on desired capacity
const L: usize = 10; // Change based on desired capacity of tree 


struct Block {
    valid: bool,
    address: u32,
    data: Vec<u8>,
}

struct Bucket {
    blocks: Vec<Block>,
}


struct ORAM {
    tree: Vec<Bucket>,
    stash: Vec<Block>,
    position_map: HashMap<u32, usize>,
}


impl ORAM {

    // Create a new ORAM initalizing an empty stash and position map.
    // Tree is initalized to contain L buckets with dumy data in them 
    fn new() -> Self {
        let mut tree = Vec::new();
        for _ in 0..(1 << L) {
            tree.push(Bucket { blocks: vec![Block { valid: false, address: 0, data: vec![] }; Z] });
        }
        ORAM {
            tree,
            stash: Vec::new(),
            position_map: HashMap::new(),
        }
    }
    fn access(&mut self, op: String, address: u32, data: Vec<u8>) -> Vec<u8> {
        // Check if the address a is already in the position map. If it is, get the position
        // Otherwise add it to the position map and get that value?
        let x = if let Some(&pos) = self.position_map.get(&address) {
            pos
        } else {
            let new_pos = rand::thread_rng().gen_range(0..(1 << L));
            self.position_map.insert(address, new_pos);
            new_pos
        };
        // Iterate over the tree and add all of the blocks in bucket to stash
        for l in 0..=L {
            let bucket_index = self.get_bucket_index(x, l);
            for block in &self.tree[bucket_index].blocks {
                if block.valid {
                    self.stash.push(block.clone());
                }
            }
        }
        // I don't understand 
        let result = self.stash
            .iter()
            .find(|block| block.address == address)
            .map(|block| block.data.clone())
            .unwrap_or_else(|| vec![]);


        if op == "write" {
            self.stash.retain(|block| block.address != address);
            self.stash.push(Block { valid: true, address, data });
        }

        
    }
    fn get_bucket_index(&self, x: usize, l: usize) -> usize {
        // TODO: Implement the logic to get the bucket index based on x and l
        // How does this work again?
        0
    }

}


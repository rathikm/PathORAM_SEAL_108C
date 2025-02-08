#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Block<const N: usize> {
    pub id: u64,
    pub data: [u8; N],
    pub dummy: bool,
}

impl <const N: usize> Block<N> {
    pub fn new(id: u64, data: [u8; N], dummy: bool) -> Self {
        Self {id: id, data: data, dummy: dummy}
    }

    pub fn empty() -> Self {
        Self {id: 0, data: [0; N], dummy: true}
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Bucket<const Z: usize, const N: usize> {
    pub storage: [Block<N>; Z],
}

impl <const Z: usize, const N: usize> Bucket<Z, N> {
    pub fn new() -> Self {
        Self {storage: [Block::empty(); Z]}
    }

    pub fn insert(&mut self, block: Block<N>) -> bool{
        if let Some(pos) = self.storage.iter_mut().position(|b| b.dummy) {
            self.storage[pos] = block;
            true
        }
        else {
            false
        }
    }

    pub fn return_real_blocks(&self) -> Vec<Block<N>> {
        let mut real_blocks: Vec<Block<N>> = Vec::new();
        for block in self.storage.iter() {
            if !block.dummy {
                real_blocks.push(block.clone());
            }
        }
        real_blocks
    }
}
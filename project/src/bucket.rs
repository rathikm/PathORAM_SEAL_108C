#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Block<const N: usize> {
    pub address: u64,
    pub data: [u8; N],
    pub dummy: bool,
}

impl <const N: usize> Block<N> {
    pub fn new(address: u64, data: [u8; N], dummy: bool) -> Self {
        Self {address: address, data: data, dummy: dummy}
    }

    pub fn empty() -> Self {
        Self {address: 0, data: [0; N], dummy: true}
    }

    pub fn set_address(&mut self, address: u64) {
        self.address = address;
    }

    pub fn set_data(&mut self, data: [u8; N]) {
        self.data = data;
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

    pub fn write(&mut self, block: Block<N>, ind: usize) {
        self.storage[ind] = block;
    }
}
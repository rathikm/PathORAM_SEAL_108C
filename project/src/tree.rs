use crate::bucket::Bucket;
use crate::bucket::Block;

#[derive(Clone, Debug)]
pub struct Tree<const L: usize, const N: usize, const Z: usize> {
    pub tree: Vec<Bucket<Z, N>>,
}

impl <const L: usize, const N: usize, const Z: usize> Tree <L, N, Z> {
    pub fn new() -> Self {
        let size: usize = (1 << (L+1)) - 1;
        Self {tree: vec![Bucket::new(); size]}
    }

    pub fn calc_path(&self, mut ind: usize) -> Vec<Bucket<Z, N>> {
        let mut path: Vec<Bucket<Z, N>> = vec![];
        while ind > 0 {
            path.push(self.tree[ind]);
            ind = (ind - 1) / 2;
        }
        path.push(self.tree[0]);
        path
    }

    pub fn calc_path_indices(&self, mut ind: usize) -> Vec<usize> {
        let mut path: Vec<usize> = vec![];
        while ind > 0 {
            path.push(ind);
            ind = (ind - 1) / 2;
        }
        path.push(0);
        path
    }
    pub fn num_leaves(&self) -> usize {
        1 << L
    }
}
use crate::ORAM::ORAM;
use std::{collections::{HashMap, HashSet}, hash::Hash};
use rand::seq::SliceRandom;

pub fn build_histogram(oram: &mut ORAM, keys: HashSet<u64>) -> HashMap<u64, Vec<u64>> {
    let mut hist = HashMap::new();
    for k in keys {
        let res_len = oram.read_records(k).len();
        hist.entry(res_len as u64).or_insert_with(Vec::new).push(k);
    }
    hist
}

pub fn query_recovery_attack(keys: HashSet<u64>, hist: HashMap<u64, Vec<u64>>, oram: &mut ORAM) -> f64 {
    let mut correct = 0.0;
    for k in &keys {
        let res_len = oram.read_records(*k).len();
        if let Some(keys_hist) = hist.get(&(res_len as u64)) {
            // Create a random number generator.
            let mut rng = rand::thread_rng();
            if let Some(pred_key) = keys_hist.choose(&mut rng) {
                if *pred_key == *k {
                    correct = correct + 1.0;
                }
            }
        }
    }

    correct / (keys.len() as f64)

}
use crate::ORAM::ORAM;
use std::collections::HashMap;

pub fn build_histogram(oram: &mut ORAM, keys: Vec<u64>) -> HashMap<u64, Vec<u64>> {
    let mut hist = HashMap::new();
    for k in keys {
        let res_len = oram.read_records(k).len();
        hist.entry(res_len as u64).or_insert_with(Vec::new).push(k);
    }
    hist
}

pub fn query_recovery_attack() {
    
}
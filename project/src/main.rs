#![allow(warnings)]

use Project::ORAM::ORAM;
use Project::SEAL::SEAL;
// use rand_core::{SeedableRng, RngCore};
// use rand_pcg::Pcg64Mcg;

use permutation_iterator::Permutor;
use std::collections::HashSet;

fn main() {
    let mut oram = ORAM::new();
    oram.init();
    print!("Hello World\n");

    let max: u64 = 10;
    let mut permutor = Permutor::new_with_u64_key(max,32 as u64);
    for (index,value) in &mut permutor.enumerate() {
    
        println!("{} -> {}",index, value);
    }
    permutor = Permutor::new_with_u64_key(max, 32 as u64);
    let stuff = permutor.nth(3);
    println!("{:?}",stuff);
    // Example data
    let address = 10;
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    // Write data to ORAM
    oram.access("write".to_string(), address, data.clone());
    // Read data from ORAM
    let read_data = oram.access("read".to_string(), address, [9,10,11,12,13,14,15,16]);
    println!("Read data: {:?}", read_data);

    let mut seal = SEAL::new(2);
    let data_vec: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let data_store = [0,0,0,0,0,0,0,0];
    let mut sealArray = seal.ADJOramInit(data_vec,2);
    seal.ADJOramAccess("write".to_string(), 5,data_store, &mut sealArray);
    let dataout = seal.ADJOramAccess("read".to_string(), 5,data_store, &mut sealArray);
    match dataout {
        Ok(data) => println!("Data: {:?}", data),
        Err(e) => println!("Error: {}", e),
    }
    


}

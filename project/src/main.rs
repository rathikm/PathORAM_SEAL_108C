#![allow(warnings)]

use Project::attacks::{build_histogram, query_recovery_attack};
use Project::ORAM::ORAM;
use Project::preprocess::read_csv;
use Project::SEAL::SEAL;
use std::hash::Hash;
// use rand_core::{SeedableRng, RngCore};
// use rand_pcg::Pcg64Mcg;
use std::time::{Duration, Instant};
use permutation_iterator::Permutor;
use std::collections::HashSet;

fn main() {
    let mut oram = ORAM::new();
    oram.init();

    let file_path = "test-oram-data.csv";
    let records = read_csv(file_path, &mut oram);
    let mut keys = HashSet::new();
    for record in &records {
        keys.insert(record.0);
    }
    let names = oram.read_records(15);
    // for name in names {
    //     println!("{}", name);
    // }

    //let keys = vec![10, 15, 20, 25];
    let hist = build_histogram(&mut oram, keys.clone());
    // println!("{:#?}", hist);

    let accuracy = query_recovery_attack(keys, hist, &mut oram);

    println!("{}", accuracy);
  
    // print!("Hello World\n");

    let max: u64 = 10;
    let mut permutor = Permutor::new_with_u64_key(max,32 as u64);
    for (index,value) in &mut permutor.enumerate() {
        println!("{} -> {}",index, value);
    }
    permutor = Permutor::new_with_u64_key(max, 32 as u64);
    let stuff = permutor.nth(3);
    println!("{:?}",stuff);
    // Example data
    // let address = 10;
    // let data = [1, 2, 3, 4, 5, 6, 7, 8];
    // // Write data to ORAM
    // oram.access("write".to_string(), address, data.clone());
    // // Read data from ORAM
    // let read_data = oram.access("read".to_string(), address, [9,10,11,12,13,14,15,16]);
    // println!("Read data: {:?}", read_data);

    let mut seal = SEAL::new(2);
    // let data_vec: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let data_store = [0,0,0,0,0,0,0,0];

    let mut sealArray = seal.ADJOramInit(&records,2);
    let now = Instant::now();
    seal.ADJOramAccess("write".to_string(), 5,data_store, &mut sealArray);
    println!("Write-Time: {:?}", now.elapsed());
    let now2 = Instant::now();
    let dataout = seal.ADJOramAccess("read".to_string(), 5,data_store, &mut sealArray);
    println!("Read Time: {:?}", now2.elapsed());
    match dataout {
        Ok(data) => println!("Data: {:?}", data),
        Err(e) => println!("Error: {}", e),
  }
    // Example data
    // let address = 10;
    // let data = [1, 2, 3, 4, 5, 6, 7, 8];

    // // Write data to ORAM
    // //oram.access("write".to_string(), address, data.clone());
    // let mut read_data = oram.access_key_val("write".to_string(), 10, address, "Mike");
    // read_data = oram.access_key_val("read".to_string(), 10, address, "dummy");
    // Read data from ORAM
    //let read_data = oram.access("read".to_string(), address, [9,10,11,12,13,14,15,16]);
    //println!("Read data: {:?}", read_data);
}


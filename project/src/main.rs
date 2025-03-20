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
use std::fs::File;
use std::io::Write;
const alpha: u64 = 0;

fn main() {
    let mut oram = ORAM::new();
    oram.init();
    //before doing key-value store, testing oram read/write directly
    let mut total_oram_read = Duration::new(0, 0);
    let mut total_oram_write = Duration::new(0, 0);
    for i in 0..100 {
        let mut start = Instant::now();
        let _ = oram.access("write".to_string(), 1, [0; 8]);
        let duration_oram_write = start.elapsed();
        total_oram_write += duration_oram_write;
        start = Instant::now();
        _ = oram.access("read".to_string(), 1, [0; 8]);
        let duration_oram_read = start.elapsed();
        total_oram_read += duration_oram_read;
    }


    let avg_oram_read = total_oram_read / 100;
    let avg_oram_write = total_oram_write / 100;

    println!(
        "Average ORAM Read: {:?}, Average ORAM Write: {:?}",
        avg_oram_read, avg_oram_write
    );

   
    let file_path = "generated.csv";
    let records = read_csv(file_path, &mut oram);
    let mut keys = HashSet::new();
    for record in &records {
        keys.insert(record.0);
    }
    println!("{:?}",keys);
    let names = oram.read_records(15);
    // for name in names {
    //     println!("{}", name);
    // }

    //let keys = vec![10, 15, 20, 25];
    // let hist = build_histogram(&mut oram, keys.clone());
    // // println!("{:#?}", hist);
    // let mut accuracy: f64 = 0.0;
    // for i in 0..100 {
    //     accuracy = accuracy + query_recovery_attack(keys.clone(), hist.clone(), &mut oram);
    // }
    // accuracy = accuracy / 100.0;

    

    // println!("{}", accuracy);
  
    // print!("Hello World\n");

    // let max: u64 = 10;
    // let mut permutor = Permutor::new_with_u64_key(max,32 as u64);
    // for (index,value) in &mut permutor.enumerate() {
    //     println!("{} -> {}",index, value);
    // }
    // permutor = Permutor::new_with_u64_key(max, 32 as u64);
    // let stuff = permutor.nth(3);
    // println!("{:?}",stuff);
    // Example data
    // let address = 10;
    // let data = [1, 2, 3, 4, 5, 6, 7, 8];
    // // Write data to ORAM
    // oram.access("write".to_string(), address, data.clone());
    // // Read data from ORAM
    // let read_data = oram.access("read".to_string(), address, [9,10,11,12,13,14,15,16]);
    // println!("Read data: {:?}", read_data);

    let mut seal = SEAL::new(alpha);
    // let data_vec: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let data_store = [0,0,0,0,0,0,0,0];
    // Testing setup: oram.write_records is currently not working need to fix.
    let mut total_oram_read = Duration::new(0, 0);
    let mut total_seal_read = Duration::new(0, 0);
    let mut total_oram_write = Duration::new(0,0);
    let mut total_seal_write = Duration::new(0, 0);
    let mut count = 0;
    let stringthing = "stetrtrt";
    let mut csv_data = vec![];

    let mut sealArray = seal.ADJOramInit(&records,alpha);
    for key in &keys {
        let data_store = [0u8; 8];
        // println!("{}",key);
        // Timing ORAM read
        let start = Instant::now();
        let _ = oram.read_record(0,*key);
        let duration_oram_read = start.elapsed();
        total_oram_read += duration_oram_read;
        // println!("Alive");
        // Timing SEAL read
        let start = Instant::now();
        let _ = seal.ADJOramAccess("read".to_string(), *key, data_store, &mut sealArray);
        let duration_seal_read = start.elapsed();
        total_seal_read += duration_seal_read;
        // println!("Alive2");
        let start = Instant::now();
        oram.write_record(*key, &stringthing, *key);
        let duration_oram_write = start.elapsed();
        total_oram_write+=duration_oram_write;
        // Timing SEAL write
        let start = Instant::now();
        let _ = seal.ADJOramAccess("write".to_string(), *key, data_store, &mut sealArray);
        let duration_seal_write = start.elapsed();
        total_seal_write += duration_seal_write;
        // println!("Alive3");
        csv_data.push((
            key,
            duration_oram_read.as_micros(),
            duration_seal_read.as_micros(),
            duration_oram_write.as_micros(),
            duration_seal_write.as_micros(),
        ));
        println!(
            "Key: {}, ORAM Read: {:?}, SEAL Read: {:?}, ORAM Write: {:?}, SEAL Write: {:?}",
            key, duration_oram_read, duration_seal_read, duration_oram_write, duration_seal_write
        );

        count += 1;
    }
    let avg_oram_read = total_oram_read / count;
    let avg_seal_read = total_seal_read / count;
    let avg_oram_write =total_oram_write / count;
    let avg_seal_write = total_seal_write / count;
    println!(
        "Average ORAM Read: {:?}, Average SEAL Read: {:?}, Average ORAM Write: {:?}, Average SEAL Write: {:?}",
        avg_oram_read, avg_seal_read, avg_oram_write, avg_seal_write
    );
    let mut wtr = csv::Writer::from_path("timing_data.csv").expect("Failed to create CSV writer");
    wtr.write_record(&["Key", "ORAM Read (μs)", "SEAL Read (μs)", "ORAM Write (μs)", "SEAL Write (μs)"])
        .expect("Failed to write CSV header");

    for (key, oram_read, seal_read, oram_write, seal_write) in csv_data {
        wtr.write_record(&[
            key.to_string(),
            oram_read.to_string(),
            seal_read.to_string(),
            oram_write.to_string(),
            seal_write.to_string(),
        ])
        .expect("Failed to write CSV record");
    }

    wtr.write_record(&[
        "Average".to_string(),
        avg_oram_read.as_micros().to_string(),
        avg_seal_read.as_micros().to_string(),
        avg_oram_write.as_micros().to_string(),
        avg_seal_write.as_micros().to_string(),
    ])
    .expect("Failed to write CSV average record");

    wtr.flush().expect("Failed to flush CSV writer");
//     match dataout {
//         Ok(data) => println!("Data: {:?}", data),
//         Err(e) => println!("Error: {}", e),
//   }
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


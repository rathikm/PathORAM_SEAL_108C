#![allow(warnings)]

use Project::attacks::{build_histogram, query_recovery_attack};
use Project::ORAM::ORAM;
use Project::preprocess::read_csv;

fn main() {
    let mut oram = ORAM::new();
    oram.init();

    let file_path = "test-oram-data.csv";
    let keys = read_csv(file_path, &mut oram);
    let names = oram.read_records(15);
    // for name in names {
    //     println!("{}", name);
    // }

    //let keys = vec![10, 15, 20, 25];
    let hist = build_histogram(&mut oram, keys.clone());
    println!("{:#?}", hist);

    let accuracy = query_recovery_attack(keys, hist, &mut oram);

    println!("{}", accuracy);
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


#![allow(warnings)]

use Project::ORAM::ORAM;

fn main() {
    let mut oram = ORAM::new();
    oram.init();

    // Example data
    let address = 10;
    let data = [1, 2, 3, 4, 5, 6, 7, 8];

    // Write data to ORAM
    //oram.access("write".to_string(), address, data.clone());
    let mut read_data = oram.access_key_val("write".to_string(), 10, address, "Mike");
    read_data = oram.access_key_val("read".to_string(), 10, address, "dummy");
    // Read data from ORAM
    //let read_data = oram.access("read".to_string(), address, [9,10,11,12,13,14,15,16]);
    println!("Read data: {:?}", read_data);
}


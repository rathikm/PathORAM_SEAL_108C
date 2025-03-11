#![allow(warnings)]

// use Project::ORAM::ORAM;
use Project::SEAL::SEAL;


fn main() {
    // let mut oram = ORAM::new();
    // oram.init();
    // print!("Hello World\n");

    // // Example data
    // let address = 10;
    // let data = [1, 2, 3, 4, 5, 6, 7, 8];

    // // Write data to ORAM
    // oram.access("write".to_string(), address, data.clone());

    // // Read data from ORAM
    // let read_data = oram.access("read".to_string(), address, [9,10,11,12,13,14,15,16]);
    // println!("Read data: {:?}", read_data);

    let mut seal = SEAL::new(2);
    


}


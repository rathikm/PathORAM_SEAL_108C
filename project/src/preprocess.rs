use std::error::Error;
use std::fs;
use serde::Deserialize;
use crate::ORAM::ORAM;

#[derive(Debug, Deserialize)]
pub struct Record {
    age: u64,
    name: String,
}

pub fn read_csv(file_path: &str, oram: &mut ORAM) -> Result<(), Box<dyn Error>> {
    // let content = fs::read_to_string(file_path)?;
    // println!("File content:\n{}", content);

    // Create a CSV reader from the file path.
    let mut rdr = csv::Reader::from_path(file_path)?;
    let addr_space = oram.addr_space_len();
    let mut counter = 0;

    // Iterate over each deserialized record.
    for result in rdr.deserialize::<Record>() {
        // Each iteration returns a Result<Record, Error>
        match result {
            Ok(record) => {
                //dbg!(&record); // Prints record with debug info.
                oram.write_record(record.age, &record.name, counter);
                counter += 1;
            },
            Err(e) => {
                println!("Error deserializing record: {}", e);
            }
        }
    }
    Ok(())
}


use std::collections::HashSet;
use std::error::Error;
use std::fs;
use serde::Deserialize;
use crate::ORAM::ORAM;

#[derive(Debug, Deserialize)]
pub struct Record {
    age: u64,
    name: String,
}

pub fn read_csv(file_path: &str, oram: &mut ORAM) -> Vec<(u64, String)> {
    // Properly handle the result of creating the CSV reader.
    let mut rdr = csv::Reader::from_path(file_path)
        .expect("Failed to open CSV file");
    
    let mut counter = 0;
    let mut records = Vec::new();
    
    // Iterate over each deserialized record.
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => {
                // Write the record to ORAM and insert the age into the set.
                oram.write_record(record.age, &record.name, counter);
                records.push((record.age, record.name));
                counter += 1;
            },
            Err(e) => {
                println!("Error deserializing record: {}", e);
            }
        }
    }
    
    records
}



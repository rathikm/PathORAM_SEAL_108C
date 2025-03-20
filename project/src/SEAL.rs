use crate::ORAM::ORAM;
use aes_gcm::{
    aead::{Aead, AeadCore},
    Aes256Gcm, Nonce, Key, KeyInit
};
use rand::rngs::OsRng;
use crate::ORAM::{Z, L, N};
use aes_gcm::AeadInPlace;
use std::fmt;
use rand::Rng;
use permutation_iterator::Permutor;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use hmac::{Hmac, Mac};
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;

pub struct SEAL {
    pub k: u64,
    pub alpha: u64,
    pub maxV: u64,
    pub key: [u8; 32]
}

impl SEAL {

    // Instantiate a new SEAL construction. Takes in the security parameter alpha. 
    pub fn new(alpha: u64) -> Self {
        let mut rng = rand::thread_rng();
        let mut key = [0u8; 32]; // 256-bit key
        rng.fill(&mut key);
        SEAL {
            k: rng.gen(),
            alpha,
            maxV: 0,
            key: key,
        }
    }
    //Initalizes an AdjOram as described in the pseudocode of the SEAL paper. Returns the construction as a Vec of ORAMs.
    // Data is taken in as a vec of u64s, and stored in each ORAM as a padded array of N values(see Path-ORAM implementation)
    // Length of Data, n, should be greater than 2 to the power of alpha, otherwise the function will panic.
    // pub fn ADJOramInit(&mut self, array: Vec<u64>,alpha: u64)-> Vec<ORAM> {
    //     self.alpha = alpha;
    //     self.maxV = array.len() as u64;
    //     let mew = 2_i32.pow(self.alpha as u32) as usize;
    //     // Intitalizing the arrays based on alpha
    //     println!("Array Len: {}", array.len());
    //     let s_size =  array.len() / mew.clone();
    //     println!("S Dimensions: {} {}", mew, s_size);
    //     let mut S = vec![vec![0u8; s_size]; mew];
    //     // 5-7
    //     for i in 0..array.len() {
    //         let (l, phi) = self.compute_phi_and_l(i, self.alpha);
    //         println!("L and Phi: {} {}",l,phi);
    //         S[l][phi] = array[i] as u8;
    //     }

    //     // Loop intalizes S. We now want to store each array in S in it's own ORAM
    //     // To do this, initalize EM 
    //     let oram_array = self.initialize_oram_array(mew,S);
    //     oram_array
    // }

    // In SEAL.rs, change ADJOramInit’s signature:
    pub fn ADJOramInit(&mut self, records: &[(u64, String)], alpha: u64) -> Vec<ORAM> {
        self.alpha = alpha;
        self.maxV = records.len() as u64;
        let mew = 2_u32.pow(self.alpha as u32) as usize;
        // Create one ORAM instance per “row” in the SEAL construction.
        let mut oram_array = Vec::with_capacity(mew);
        for _ in 0..mew {
            let mut oram = ORAM::new();
            oram.init();
            oram_array.push(oram);
        }
        
        // Iterate over records by reference so they aren’t moved.
        for (i, (key, value)) in records.iter().enumerate() {
            let (l, phi) = self.compute_phi_and_l(i, self.alpha);
            // Use phi as the address (or modify as needed)
            oram_array[l].write_record(*key, value, phi as u64);
        }
        
        oram_array
    }
    // Function as described in the paper. Result on success is an array of length N. So if value nitially stored was 5, will return [5,0,0,0,0,0,0,0]
    // pub fn ADJOramAccess(&mut self, op: String, i: u64, vSubi: [u8; N],  EM: &mut Vec<ORAM>) -> Result<[u8; N], SEALError> {
    //     if let maxV = self.maxV {
    //         if i > maxV {
    //             return Err(SEALError::IndexOutOfBounds);
    //         }
    //     } else {
    //         return Err(SEALError::IndexOutOfBounds);
    //     }

    //     let mew = 2_i32.pow(self.alpha as u32) as usize;
    //     let (l, phi) = self.compute_phi_and_l(i as usize, self.alpha);
    //     let result = EM[l].access(op,(phi).try_into().unwrap(),vSubi);
    //     Ok(result)
    // }

    pub fn ADJOramAccess(&mut self, op: String, i: u64, vSubi: [u8; N], EM: &mut Vec<ORAM>) -> Result<[u8; N], SEALError> {
        if i >= self.maxV {
            return Err(SEALError::IndexOutOfBounds);
        }
        // Compute destination based on record index
        let alpha = self.alpha;
        let (l, phi) = self.compute_phi_and_l(i as usize, alpha);
        
        if op == "write" {
            // Convert the [u8; N] input into a string (assumes valid UTF‑8)
            let value_str = std::str::from_utf8(&vSubi).unwrap_or("").to_string();
            // For updating, you might need to know the record’s original key.
            // If you don’t maintain a separate mapping, you can use phi as a temporary key.
            EM[l].write_record(phi as u64, &value_str, phi as u64);
            Ok(vSubi)
        } else if op == "read" {
            // Read all records under key φ (or, if available, use the stored original key)
            let records: [u8; N] = EM[l].access("read".to_string(), phi as u64, [0u8; N]);
            // println!("{:?}", records);
            Ok(records)
            // Here we simply take the first record.
            // println!("{:?}",record_str);

        } else {
            Err(SEALError::InvalidOperation)
        }
    }
    

    // Given S, the array as described in the paper pseudocode, and mew, loop over S and add data to the corresponding ORAM.
    // This is done by creating an ORAM(outer loop) and then in the inner loop iterating over the corresponding row of S and storing each value there in that oram using oram.access
    // This is done iterativley because the permutation of the data(using compute_phi_and_l) has already been done and is reflected in the datas placement in S
    // Returns oram_array, which is returned by the outer function ADJOramInit
    fn initialize_oram_array(&mut self,mew: usize,S: Vec<Vec<u8>>) -> Vec<ORAM> {
        let mut oram_array = Vec::with_capacity(mew);
        for oram_num in 0..mew {
            let mut oram = ORAM::new();
            oram.init();
            for i in 0..(S[oram_num].len()) {
                // println!("I: {}", i);
                // let (l, phi) = self.compute_phi_and_l(i, self.alpha);
                // println!("L and Phi: {} {}",l,phi);
                let padded_data = Self::pad_to_length(vec![S[oram_num][i]], N);
                oram.access("write".to_string(), (i).try_into().unwrap(), padded_data);
            }
            // TODO: Not sure if this is the correct way we should be initalizing each ORAM. Might need another helper for multiple access.
            oram_array.push(oram);
        }
        oram_array
    }
    // Does the calculation of l and phi given alpha and index of value in the array.
    // Uses Permutor based on lenth of data
    fn compute_phi_and_l(&mut self, i: usize, alpha: u64) -> (usize, usize) {
        // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        // println!("{}", i);
        // let mut block = (i as u64).to_le_bytes().to_vec();
        // println!("{:?}", block);
        // cipher.encrypt_in_place(&nonce, b"", &mut block).expect("encryption failure!");
        // let array: [u8; 8] = truncate_to_8_elements(block);


        // let num_bits =  64 - (self.maxV-1).leading_zeros() as u64;
        // // println!("{}", num_bits);
        // let mask = (1 << num_bits) -1;
        // // println!("{}", mask);
        // let inst = Instant::now();
        // let prp_output = self.p.nth(i).expect("Failed to Permute") & mask;
        // let result = inst.elapsed();
        // println!("PRP: {:?}", result);
        // // println!("PRP Output: {:?}", prp_output);

        // // let num_bits = 64 - prp_output.leading_zeros() as u64;

        // let l = (prp_output >> (num_bits - alpha)) as usize;
        // let phi = (prp_output & ((1 << (num_bits- alpha)) - 1)) as usize;
        // (l, phi)
    let mut mac = <HmacSha256 as Mac>::new_from_slice(&self.key).expect("HMAC can take a key of any size");
    mac.update(&(i as u64).to_le_bytes());

    // Generate the PRF output
    let prf_output = mac.finalize().into_bytes();

    // Convert the first 8 bytes of the PRF output into a u64
    let prp_output = u64::from_le_bytes(prf_output[..8].try_into().unwrap());

    // Calculate the number of bits required to represent `maxV`
    let num_bits = 64 - (self.maxV - 1).leading_zeros() as u64;

    // Mask the PRF output to fit within the range of `maxV`
    let mask = (1 << num_bits) - 1;
    let prp_output = prp_output & mask;

    // Compute `l` and `phi` from the PRF output
    let l = (prp_output >> (num_bits - alpha)) as usize;
    let phi = (prp_output & ((1 << (num_bits - alpha)) - 1)) as usize;

    (l, phi)
    }
    // Helper function to make data work with Path-ORAM
    fn pad_to_length(mut data: Vec<u8>, length: usize) -> [u8; N] {
        data.resize(length, 0); 
        let mut array = [0u8; N];
        array.copy_from_slice(&data[..N]); 
        array
    }
}

#[derive(Debug)]
pub enum SEALError {
    IndexOutOfBounds,
    InvalidOperation,  // Added for unsupported op values
}


impl fmt::Display for SEALError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SEALError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            SEALError::InvalidOperation => write!(f, "Invalid op")
        }
    }
}

impl std::error::Error for SEALError {}
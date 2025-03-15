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



pub struct SEAL {
    pub k: u64,
    pub alpha: u64,
    pub maxV: u64,
}

impl SEAL {
    pub fn new(alpha: u64) -> Self {
        let mut rng = rand::thread_rng();
        SEAL {
            k: rng.gen(),
            alpha,
            maxV: 0,
        }
    }

    pub fn ADJOramInit(&mut self, array: Vec<u64>,alpha: u64)-> Vec<ORAM> {
        self.alpha = alpha;
        self.maxV = array.len() as u64;
        let mew = 2_i32.pow(self.alpha as u32) as usize;
        // Intitalizing the arrays based on alpha
        println!("Array Len: {}", array.len());
        let s_size =  array.len() / mew.clone();
        println!("S Dimensions: {} {}", mew, s_size);
        let mut S = vec![vec![0u8; s_size]; mew];
        // 5-7, not sure if there is a better way of doing this.
        //Basically using a block cipher as a PRP which should work?
        for i in 0..array.len() {
            let (l, phi) = self.compute_phi_and_l(i, self.alpha);
            println!("L and Phi: {} {}",l,phi);
            S[l][phi] = array[i] as u8;
        }

        // Loop intalizes S. We now want to store each array in S in it's own ORAM
        // To do this, initalize EM 
        let oram_array = self.initialize_oram_array(mew,S);
        oram_array
    }
    // &mut self, op: String, address: u64, data_new: [u8; N]) -> [u8; N]
    pub fn ADJOramAccess(&mut self, op: String, i: u64, vSubi: [u8; N],  EM: &mut Vec<ORAM>) -> Result<[u8; N], SEALError> {
        if let maxV = self.maxV {
            if i > maxV {
                return Err(SEALError::IndexOutOfBounds);
            }
        } else {
            return Err(SEALError::IndexOutOfBounds);
        }

        let mew = 2_i32.pow(self.alpha as u32) as usize;
        let (l, phi) = self.compute_phi_and_l(i as usize, self.alpha);
        let result = EM[l].access(op,(phi).try_into().unwrap(),vSubi);
        Ok(result)
    }
    fn initialize_oram_array(&mut self,mew: usize,S: Vec<Vec<u8>>) -> Vec<ORAM> {
        let mut oram_array = Vec::with_capacity(mew);
        for oram_num in 0..mew {
            let mut oram = ORAM::new();
            oram.init();
            // TODO: Not sure about the indexing here. Might need assistance troubleshooting this.
            for i in 0..(S[oram_num].len()) {
                println!("I: {}", i);
                let (l, phi) = self.compute_phi_and_l(i, self.alpha);
                println!("L and Phi: {} {}",l,phi);
                let padded_data = Self::pad_to_length(vec![S[oram_num][i]], N);
                oram.access("write".to_string(), (i).try_into().unwrap(), padded_data);
            }
            // TODO: Not sure if this is the correct way we should be initalizing each ORAM. Might need another helper for multiple access.
            oram_array.push(oram);
        }
        oram_array
    }
    fn compute_phi_and_l(&self, i: usize, alpha: u64) -> (usize, usize) {
        // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        // println!("{}", i);
        // let mut block = (i as u64).to_le_bytes().to_vec();
        // println!("{:?}", block);
        // cipher.encrypt_in_place(&nonce, b"", &mut block).expect("encryption failure!");
        // let array: [u8; 8] = truncate_to_8_elements(block);
        let mut p = Permutor::new_with_u64_key(self.maxV, self.k);
        let prp_output = p.nth(i).expect("Failed to Permute") & 0b111;
        println!("PRP Output: {:?}", prp_output);

        // let num_bits = 64 - prp_output.leading_zeros() as u64;

        let l = (prp_output >> (3 - alpha)) as usize;
        let phi = (prp_output & ((1 << (3 - alpha)) - 1)) as usize;
        (l, phi)
    }
    fn pad_to_length(mut data: Vec<u8>, length: usize) -> [u8; N] {
        data.resize(length, 0); 
        let mut array = [0u8; N];
        array.copy_from_slice(&data[..N]); 
        array
    }
}

fn truncate_to_8_elements(data: Vec<u8>) -> [u8; 8] {
    let mut array = [0u8; 8];
    let slice = &data[..8]; 
    array.copy_from_slice(slice);
    array
}

#[derive(Debug)]
pub enum SEALError {
    IndexOutOfBounds,
}

impl fmt::Display for SEALError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SEALError::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl std::error::Error for SEALError {}
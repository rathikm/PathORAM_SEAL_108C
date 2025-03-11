use crate::ORAM::ORAM;
use aes_gcm::{
    aead::{Aead, AeadCore},
    Aes256Gcm, Nonce, Key, KeyInit
};
use rand::rngs::OsRng;
use crate::ORAM::{Z, L, N};
use aes_gcm::AeadInPlace;

pub struct SEAL {
    pub k: Key<Aes256Gcm>,
    pub alpha: u64,
}

impl SEAL {
    pub fn new(alpha: u64) -> Self {
        SEAL {
            k: Aes256Gcm::generate_key(OsRng),
            alpha,
        }
    }

    pub fn ADJOramInit(&mut self, securityParam: u64,array: Vec<u64>,alpha: u64)-> Vec<ORAM> {
        self.alpha = alpha;
        let mew = 2_i32.pow(self.alpha as u32) as usize;
        // Intitalizing the arrays based on alpha
        let s_size = mew.clone() / array.len();
        let mut S = vec![vec![0u8; s_size]; mew];
        let cipher = Aes256Gcm::new(&self.k);
        // 5-7, not sure if there is a better way of doing this.
        //Basically using a block cipher as a PRP which should work?
        for i in 0..array.len() {
            let (l, phi) = Self::compute_phi_and_l(i, self.alpha, &cipher);
            S[l+1][phi+1] = array[i] as u8;
        }

        // Loop intalizes S. We now want to store each array in S in it's own ORAM
        // To do this, initalize EM 
        let oram_array = self.initialize_oram_array(mew,S);
        oram_array
    }
    // &mut self, op: String, address: u64, data_new: [u8; N]) -> [u8; N]
    pub fn ADJOramAccess(&mut self, op: String, i: u64, vSubi: [u8; N], mut EM: Vec<ORAM>) -> [u8; N] {
        let mew = 2_i32.pow(self.alpha as u32) as usize;
        let cipher = Aes256Gcm::new(&self.k);
        let (l, phi) = Self::compute_phi_and_l(i as usize, self.alpha, &cipher);
        let result = EM[l+1].access(op,(phi+1).try_into().unwrap(),vSubi);
        result 
    }
    fn initialize_oram_array(&mut self,mew: usize,S: Vec<Vec<u8>>) -> Vec<ORAM> {
        let mut oram_array = Vec::with_capacity(mew);
        let cipher = Aes256Gcm::new(&self.k);
        for oram_num in 0..mew {
            let mut oram = ORAM::new();
            oram.init();
            // TODO: Not sure about the indexing here. Might need assistance troubleshooting this.
            for i in 0..(S[oram_num].len()) {
                let (l, phi) = Self::compute_phi_and_l(i, self.alpha, &cipher);
                let padded_data = Self::pad_to_length(vec![S[oram_num][l+1]], N);
                oram.access("write".to_string(), (phi+1).try_into().unwrap(), padded_data);
            }
            // TODO: Not sure if this is the correct way we should be initalizing each ORAM. Might need another helper for multiple access.
            oram_array.push(oram);
        }
        oram_array
    }
    fn compute_phi_and_l(i: usize, alpha: u64, cipher: &Aes256Gcm) -> (usize, usize) {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let mut block = i.to_le_bytes().to_vec();
        cipher.encrypt_in_place(&nonce, b"", &mut block).expect("encryption failure!");
        let array: [u8; 8] = block.try_into().expect("Not 8 element vec");
        let prp_output = u64::from_le_bytes(array);
        let l = (prp_output >> (64 - alpha)) as usize;
        let phi = (prp_output & ((1 << (64 - alpha)) - 1)) as usize;
        (l, phi)
    }
    fn pad_to_length(mut data: Vec<u8>, length: usize) -> [u8; N] {
        data.resize(length, 0); 
        let mut array = [0u8; N];
        array.copy_from_slice(&data[..N]); 
        array
    }
}
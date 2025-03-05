use crate::ORAM::ORAM;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

pub struct SEAL {
    pub k,
    pub alpha: u64,
}

impl SEAL {
    pub fn new() -> Self {
        SEAL {
            k: 0,
            alpha, 0
            encrypted
        }
    }

    pub fn ADJOramInit(&mut self, securityParam: u64,array: Vec<u64>,alpha: u64) {
        self.alpha = alpha
        let mew = 2.powf(self.alpha);
        self.k = Aes256Gcm::generate_key(OsRng);


        // Intitalizing the arrays based on alpha
        let s_size = mew.clone() / len(array);
        let S = [[0u8; s_size]; mew];
        let cipher = Aes256Gcm::new(&self.k);
        // 5-7, not sure if there is a better way of doing this.
        //Basically using a block cipher as a PRP which should work?
        for i in 0..len(array) {
            let (l, phi) = compute_phi_and_l(i, self.alpha, &cipher);
            S[l+1][phi+1] = array[i] as u8;
        }

        // Loop intalizes S. We now want to store each array in S in it's own ORAM
        // To do this, initalize EM 
        let oram_array = initalize_oram_array(mew);
        oram_array
    }
    fn initialize_oram_array(&mut self,mew: usize,S) -> Vec<ORAM> {
        let mut oram_array = Vec::with_capacity(mew);
        let cipher = Aes256Gcm::new(&self.k);
        for oram_num in 0..mew {
            let mut oram = ORAM::new();
            oram.init();
            // TODO: Not sure about the indexing here. Might need assistance troubleshooting this.
            for i in 0..(len(S[oram_num])):
                let (l, phi) = compute_phi_and_l(i, self.alpha, &cipher);
                oram.access("write".to_string(), phi+1, S[oram_num][l+1])
            // TODO: Not sure if this is the correct way we should be initalizing each ORAM. Might need another helper for multiple access.
            oram_array.push(oram);
        }
        oram_array
    }
    fn compute_phi_and_l(i: u64, alpha: u32, cipher: &Aes256Gcm) -> (usize, usize) {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let mut block = i.to_le_bytes();
        cipher.encrypt_in_place(&nonce, b"", &mut block).expect("encryption failure!");
        let prp_output = u64::from_le_bytes(block);
        let l = (prp_output >> (64 - alpha)) as usize;
        let phi = (prp_output & ((1 << (64 - alpha)) - 1)) as usize;
        (l, phi)
    }
    
}
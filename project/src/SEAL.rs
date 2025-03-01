use Project::ORAM::ORAM;
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

        // 5-7, not sure if there is a better way of doing this.
        //Basically using a block cipher as a PRP which should work?
        for i in 1..len(array) {
            let cipher = Aes256Gcm::new(&self.k);
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng)); // 96-bits; unique per message
            let mut block = array[i].to_le_bytes();
            cipher.encrypt_in_place(&nonce, b"", &mut block).expect("encryption failure!");
            let prp_output = u64::from_le_bytes(block);
            let l = prp_output >> (64 - self.alpha);
            let phi = prp_output & ((1 << (64 - self.alpha)) - 1);
            S[l as usize][phi as usize] = array[i] as u8;
        }

        // Loop intalizes S. We now want to store each array in S in it's own ORAM
        // To do this, initalize EM 
        for i in 1..mew {
        }



    }
}

use std::simd;

#[allow(unused)]
use rand::{Rng};

use super::primitives::*;
use crate::imp::{Flags, Rounds};
use std::simd::Simd;

pub struct Tekton256 {
    keys: [Simd<u8, 16>; 8],
    flags: Flags
}

impl Tekton256 {

    pub fn new(key: [u8; 32], flags: Flags) -> Tekton256{

        let mut keys: [Simd<u8, 16>; 8] = [simd::u8x16::splat(0); 8];

        for i in 0..8 {
            let bytes = key.map(|x| (x << i).wrapping_mul(113));
            let mut hi: [u8;16] = [0;16];
            hi.copy_from_slice(&bytes[0..16]);
            let mut lo: [u8; 16] = [0;16];
            lo.copy_from_slice(&bytes[16..32]);
            for i in 0..16 {
                lo[i] ^= hi[i];
            }
            keys[i] = simd::u8x16::from_array(lo);
        }

        return Tekton256 {
            keys,
            flags
        }
    }

    #[inline]
    pub fn encrypt(&self, payload: &mut [u8; 16]){

    
        let mut state = simd::u8x16::from_array(*payload);
        
        if self.flags.rounds == Rounds::SAFER {
            state = encrypt_round(state, self.keys[0]);
            state = encrypt_round(state, self.keys[1]);
        }
        state = encrypt_round(state, self.keys[2]);
        state = encrypt_round(state, self.keys[3]);
        state = encrypt_round(state, self.keys[4]);
        state = encrypt_round(state, self.keys[5]);
        state = encrypt_round(state, self.keys[6]);
        state = encrypt_round(state, self.keys[7]);
        
        *payload = *state.as_array();
    }

    #[inline]
    pub fn decrypt(&self, cipher: &mut [u8; 16]){

        let mut state = simd::u8x16::from_array(*cipher);
        
        state = decrypt_round(state, self.keys[7]);
        state = decrypt_round(state, self.keys[6]);
        state = decrypt_round(state, self.keys[5]);
        state = decrypt_round(state, self.keys[4]);
        state = decrypt_round(state, self.keys[3]);   
        state = decrypt_round(state, self.keys[2]);

        if self.flags.rounds == Rounds::SAFER {
            state = decrypt_round(state, self.keys[1]);
            state = decrypt_round(state, self.keys[0]);
        }
        
        *cipher = *state.as_array();
    }
}


#[test]
fn test_encrypt_decrypt(){
    use crate::imp::{Flags, Rounds};
    let lo_key: u128 = rand::thread_rng().gen();
    let hi_key: u128 = rand::thread_rng().gen();

    let mut key: [u8; 32] = [0; 32];
    key[..16].copy_from_slice(&lo_key.to_be_bytes());
    key[16..32].copy_from_slice(&hi_key.to_be_bytes());


    let test_in_loop = |tekton: Tekton256| {
        for _ in 0..1000 {
            let p: u128 = rand::thread_rng().gen();
            let pb: [u8; 16] = p.to_be_bytes();
    
            let mut enc = pb.clone();
            tekton.encrypt(&mut enc);
    
            let mut dec = enc.clone();
            tekton.decrypt(&mut dec);
    
            assert_eq!(dec, pb);
            assert_ne!(enc, pb);
        }
    };

    test_in_loop(
        Tekton256::new(key, 
        Flags {
            rounds: Rounds::FASTER}));

    test_in_loop(
        Tekton256::new(key, 
        Flags {
            rounds: Rounds::SAFER}));

    
}
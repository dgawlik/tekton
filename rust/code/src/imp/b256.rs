
use std::simd;

#[allow(unused)]
use rand::{Rng};

use super::primitives::*;
use crate::imp::{Flags, Mode};


pub struct Tekton256 {
    keys: [[u8; 16]; 8],
    flags: Flags
}

impl Tekton256 {

    pub fn new(key: [u8; 32], flags: Flags) -> Tekton256{

        let mut keys: [[u8; 16]; 8] = [[0; 16]; 8];

        for i in 0..8 {
            let bytes = key.map(|x| (x << i).wrapping_mul(113));
            let mut hi: [u8;16] = [0;16];
            hi.copy_from_slice(&bytes[0..16]);
            let mut lo: [u8; 16] = [0;16];
            lo.copy_from_slice(&bytes[16..32]);
            for i in 0..16 {
                lo[i] ^= hi[i];
            }
            keys[i] = lo;
        }

        return Tekton256 {
            keys,
            flags
        }
    }

    #[inline]
    pub fn encrypt(&self, payload: &mut [u8; 16]){

        match self.flags.mode {

            Mode::BYTE => {
                let mut state = simd::u8x16::from_array(*payload);
                state = encrypt_round_b(state, self.keys[0], &self.flags);
                state = encrypt_round_b(state, self.keys[1], &self.flags);
                state = encrypt_round_b(state, self.keys[2], &self.flags);
                state = encrypt_round_b(state, self.keys[3], &self.flags);
                state = encrypt_round_b(state, self.keys[4], &self.flags);
                state = encrypt_round_b(state, self.keys[5], &self.flags);
                state = encrypt_round_b(state, self.keys[6], &self.flags);
                state = encrypt_round_b(state, self.keys[7], &self.flags);
                *payload = *state.as_array();
            },

            Mode::INT => {
                let payload_i = unsafe {
                    std::mem::transmute::<[u8; 16], [u32; 4]>(*payload)
                };
             
                let mut state = simd::u32x4::from_array(payload_i);
                state = encrypt_round_i(state, self.keys[0], &self.flags);
                state = encrypt_round_i(state, self.keys[1], &self.flags);
                state = encrypt_round_i(state, self.keys[2], &self.flags);
                state = encrypt_round_i(state, self.keys[3], &self.flags);
                state = encrypt_round_i(state, self.keys[4], &self.flags);
                state = encrypt_round_i(state, self.keys[5], &self.flags);
                state = encrypt_round_i(state, self.keys[6], &self.flags);
                state = encrypt_round_i(state, self.keys[7], &self.flags);
    
                *payload = unsafe {
                    std::mem::transmute::<[u32; 4], [u8; 16]>(*state.as_array())
                };
            }

        }
    }

    #[inline]
    pub fn decrypt(&self, cipher: &mut [u8; 16]){
        match self.flags.mode {

            Mode::BYTE => {
                let mut state = simd::u8x16::from_array(*cipher);
                state = decrypt_round_b(state, self.keys[7], &self.flags);
                state = decrypt_round_b(state, self.keys[6], &self.flags);
                state = decrypt_round_b(state, self.keys[5], &self.flags);
                state = decrypt_round_b(state, self.keys[4], &self.flags);
                state = decrypt_round_b(state, self.keys[3], &self.flags);
                state = decrypt_round_b(state, self.keys[2], &self.flags);
                state = decrypt_round_b(state, self.keys[1], &self.flags);
                state = decrypt_round_b(state, self.keys[0], &self.flags);
                *cipher = *state.as_array();
            },

            Mode::INT => {
                let payload_i = unsafe {
                    std::mem::transmute::<[u8; 16], [u32; 4]>(*cipher)
                };
             
                let mut state = simd::u32x4::from_array(payload_i);
                state = decrypt_round_i(state, self.keys[7], &self.flags);
                state = decrypt_round_i(state, self.keys[6], &self.flags);
                state = decrypt_round_i(state, self.keys[5], &self.flags);
                state = decrypt_round_i(state, self.keys[4], &self.flags);
                state = decrypt_round_i(state, self.keys[3], &self.flags);
                state = decrypt_round_i(state, self.keys[2], &self.flags);
                state = decrypt_round_i(state, self.keys[1], &self.flags);
                state = decrypt_round_i(state, self.keys[0], &self.flags);
    
                *cipher = unsafe {
                    std::mem::transmute::<[u32; 4], [u8; 16]>(*state.as_array())
                };
            }
        };

    }
}


#[test]
fn test_encrypt_decrypt(){
    use crate::imp::{Flags, Mode, Permute};
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
            permute: Permute::PERMUTE, 
            mode: Mode::BYTE}));

    test_in_loop(
        Tekton256::new(key, 
        Flags {
            permute: Permute::ROTATE, 
            mode: Mode::BYTE}));

    test_in_loop(
        Tekton256::new(key, 
        Flags {
            permute: Permute::PERMUTE, 
            mode: Mode::INT}));

    test_in_loop(
        Tekton256::new(key, 
        Flags {
            permute: Permute::ROTATE, 
            mode: Mode::INT}));

    
}
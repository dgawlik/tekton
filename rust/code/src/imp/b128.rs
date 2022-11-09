use std::simd;

#[allow(unused)]
use rand::{Rng};

use crate::imp::{Flags, Mode, Permute};

use super::primitives::*;


pub struct Tekton128 {
    keys: [[u8; 16]; 5],
    flags: Flags
}

impl Tekton128 {

    pub fn new(key: [u8; 16], flags: Flags) -> Tekton128{

        let mut keys: [[u8; 16]; 5] = [[0; 16]; 5];

        for i in 0..5 {
            let bytes = key.map(|x| (x << i).wrapping_mul(113));
            let ki = bytes;
            keys[i] = ki;
        }

        return Tekton128 {
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
    let key: u128 = rand::thread_rng().gen();


    let test_in_loop = |tekton: Tekton128| {
        for _ in 0..1000 {
            let p: u128 = rand::thread_rng().gen();
            let pb = p.to_be_bytes();
    
            let mut enc = pb.clone();
            tekton.encrypt(&mut enc);
    
            let mut dec = enc.clone();
            tekton.decrypt(&mut dec);
    
            assert_eq!(dec, pb);
            assert_ne!(enc, pb);
        }
    };

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            permute: Permute::PERMUTE, 
            mode: Mode::BYTE}));

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            permute: Permute::ROTATE, 
            mode: Mode::BYTE}));

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            permute: Permute::PERMUTE, 
            mode: Mode::INT}));

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            permute: Permute::ROTATE, 
            mode: Mode::INT}));

    
}
use std::simd;

#[allow(unused)]
use rand::{Rng};

use crate::imp::{Flags, Mode};

use super::{primitives::*, Rounds};

use simd::Simd;


pub struct Tekton128 {
    keys: [Simd<u8, 16>; 5],
    flags: Flags
}

impl Tekton128 {

    pub fn new(key: [u8; 16], flags: Flags) -> Tekton128{

        let mut keys: [Simd<u8, 16>; 5] = [simd::u8x16::splat(0); 5];

        for i in 0..5 {
            let bytes = key.map(|x| (x << i).wrapping_mul(113));
            let ki = simd::u8x16::from_array(bytes);
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
               
                if self.flags.rounds == Rounds::SAFER {
                    state = encrypt_round_b(state, self.keys[0]);
                    state = encrypt_round_b(state, self.keys[1]); 
                }
                
                state = encrypt_round_b(state, self.keys[2]);
                state = encrypt_round_b(state, self.keys[3]);
                state = encrypt_round_b(state, self.keys[4]);
                
                
                *payload = *state.as_array();
            },

            Mode::INT => {
                let payload_i = unsafe {
                    std::mem::transmute::<[u8; 16], [u32; 4]>(*payload)
                };
             
                let mut state = simd::u32x4::from_array(payload_i);
               
                
                if self.flags.rounds == Rounds::SAFER {
                    state = encrypt_round_i(state, self.keys[0]);
                }
                
                state = encrypt_round_i(state, self.keys[1]);   
                state = encrypt_round_i(state, self.keys[2]);
                state = encrypt_round_i(state, self.keys[3]);
                state = encrypt_round_i(state, self.keys[4]);
                
    
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
               
                state = decrypt_round_b(state, self.keys[4]);
                state = decrypt_round_b(state, self.keys[3]);
                state = decrypt_round_b(state, self.keys[2]);
                
                if self.flags.rounds == Rounds::SAFER {
                    state = decrypt_round_b(state, self.keys[1]);
                    state = decrypt_round_b(state, self.keys[0]);
                }

                *cipher = *state.as_array();
            },

            Mode::INT => {
                let payload_i = unsafe {
                    std::mem::transmute::<[u8; 16], [u32; 4]>(*cipher)
                };
             
                let mut state = simd::u32x4::from_array(payload_i);
                
                state = decrypt_round_i(state, self.keys[4]);
                state = decrypt_round_i(state, self.keys[3]);
                state = decrypt_round_i(state, self.keys[2]);
                state = decrypt_round_i(state, self.keys[1]);
                if self.flags.rounds == Rounds::SAFER {
                    state = decrypt_round_i(state, self.keys[0]);
                }
    
                *cipher = unsafe {
                    std::mem::transmute::<[u32; 4], [u8; 16]>(*state.as_array())
                };
            }
        };
    }
}


#[test]
fn test_encrypt_decrypt(){
    use crate::imp::{Flags, Mode, Rounds};
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
            rounds: Rounds::FASTER, 
            mode: Mode::BYTE}));

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            rounds: Rounds::SAFER, 
            mode: Mode::BYTE}));

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            rounds: Rounds::FASTER, 
            mode: Mode::INT}));

    test_in_loop(
        Tekton128::new(key.to_be_bytes(), 
        Flags {
            rounds: Rounds::SAFER, 
            mode: Mode::INT}));

    
}
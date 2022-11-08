use std::simd::Simd;
use std::simd;

#[allow(unused)]
use rand::{Rng};

use crate::imp::{Flags, Mode, Permute};

use super::primitives;


// i32 prime: 112_012_097 <-> 1_347_249_345
// i16 prime: 30_011 <-> 39_923



const P: [usize; 16] = [3, 7, 13, 0, 11, 1, 15, 2, 4, 12, 5, 9, 6, 8, 14, 10];
const INV_P: [usize; 16] = [3, 5, 7, 0, 8, 10, 12, 1, 13, 11, 15, 4, 9, 2, 14, 6];
const PI: [usize; 4] = [1,3,0,2];
const INV_PI: [usize; 4] = [2,0,3,1];

const S: Simd<u8, 16> = simd::u8x16::from_array([113; 16]);
const INV_S: Simd<u8, 16> = simd::u8x16::from_array([145; 16]);

const SI: Simd<u32, 4> = simd::u32x4::from_array([1_347_249_345; 4]);
const INV_SI: Simd<u32, 4> = simd::u32x4::from_array([112_012_097; 4]);

const M1: Simd<u8, 16> = simd::u8x16::from_array([0b01010101; 16]);
const M2: Simd<u8, 16> = simd::u8x16::from_array([0b00110011; 16]);
const M3: Simd<u8, 16> = simd::u8x16::from_array([0b00001111; 16]);

const M1I: Simd<u32, 4> = simd::u32x4::from_array([0b01010101_01010101_01010101_01010101; 4]);
const M2I: Simd<u32, 4> = simd::u32x4::from_array([0b00110011_00110011_00110011_00110011; 4]);
const M3I: Simd<u32, 4> = simd::u32x4::from_array([0b00001111_00001111_00001111_00001111; 4]);
const M4I: Simd<u32, 4> = simd::u32x4::from_array([0b00000000_11111111_00000000_11111111; 4]);
const M5I: Simd<u32, 4> = simd::u32x4::from_array([0b00000000_00000000_11111111_11111111; 4]);

const SH1: Simd<u8, 16> = simd::u8x16::from_array([1; 16]);
const SH2: Simd<u8, 16> = simd::u8x16::from_array([2; 16]);
const SH3: Simd<u8, 16> = simd::u8x16::from_array([4; 16]);

const SH1I: Simd<u32, 4> = simd::u32x4::from_array([1; 4]);
const SH2I: Simd<u32, 4> = simd::u32x4::from_array([2; 4]);
const SH3I: Simd<u32, 4> = simd::u32x4::from_array([4; 4]);
const SH4I: Simd<u32, 4> = simd::u32x4::from_array([8; 4]);
const SH5I: Simd<u32, 4> = simd::u32x4::from_array([16; 4]);

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
    pub fn encrypt_round_b(&self, state: Simd<u8, 16>, i: usize, mode: &Permute) -> Simd<u8, 16>{
        let mut s = state;
        s ^= simd::u8x16::from_array(self.keys[i]);
        s = primitives::diffusion_byte(s, M1, M2, M3, SH1, SH2, SH3);
        s = primitives::substitute!(s, S);

        s =  match *mode {
            Permute::PERMUTE => primitives::permute!(s, P),
            Permute::ROTATE =>  primitives::rotate_byte(s)
        };

        s
    }

    #[inline]
    pub fn decrypt_round_b(&self, state: Simd<u8, 16>, i: usize, mode: &Permute) -> Simd<u8, 16>{
        let mut s = state;

        s = match *mode {
            Permute::PERMUTE => primitives::permute!(s, INV_P),
            Permute::ROTATE => primitives::inverse_rotate_byte(s)
        };
        
        s = primitives::substitute!(s, INV_S);
        s = primitives::diffusion_byte(s, M1, M2, M3, SH1, SH2, SH3);
        s ^= simd::u8x16::from_array(self.keys[i]);
        s
    }

    #[inline]
    pub fn encrypt_round_i(&self, state: Simd<u32, 4>, i: usize, mode: &Permute) -> Simd<u32, 4>{
        let mut s = state;
        let key = unsafe {
            std::mem::transmute::<[u8; 16], [u32; 4]>(self.keys[i])
        };

        s ^= simd::u32x4::from_array(key);
        s = primitives::diffusion_int(s, M1I, M2I, M3I, M4I, M5I, SH1I, SH2I, SH3I, SH4I, SH5I);
        s = primitives::substitute!(s, SI);

        s = match *mode {
            Permute::PERMUTE => primitives::permute!(s, PI),
            Permute::ROTATE => primitives::rotate_int(s)
        };
        
        s
    }

    #[inline]
    pub fn decrypt_round_i(&self, state: Simd<u32, 4>, i: usize, mode: &Permute) -> Simd<u32, 4>{
        let mut s = state;
        let key = unsafe {
            std::mem::transmute::<[u8; 16], [u32; 4]>(self.keys[i])
        };

        s = match *mode {
            Permute::PERMUTE => primitives::permute!(s, INV_PI),
            Permute::ROTATE => primitives::inverse_rotate_int(s)
        };
       
        s = primitives::substitute!(s, INV_SI);
        s = primitives::diffusion_int(s, M1I, M2I, M3I, M4I, M5I, SH1I, SH2I, SH3I, SH4I, SH5I);
        s ^= simd::u32x4::from_array(key);
        s
    }

    #[inline]
    pub fn encrypt(&self, payload: &mut [u8; 16]){

        match self.flags.mode {

            Mode::BYTE => {
                let mut state = simd::u8x16::from_array(*payload);
                state = self.encrypt_round_b(state, 0, &self.flags.permute);
                state = self.encrypt_round_b(state, 1, &self.flags.permute);
                state = self.encrypt_round_b(state, 2, &self.flags.permute);
                state = self.encrypt_round_b(state, 3, &self.flags.permute);
                state = self.encrypt_round_b(state, 4, &self.flags.permute);
                *payload = *state.as_array();
            },

            Mode::INT => {
                let payload_i = unsafe {
                    std::mem::transmute::<[u8; 16], [u32; 4]>(*payload)
                };
             
                let mut state = simd::u32x4::from_array(payload_i);
                state = self.encrypt_round_i(state, 0, &self.flags.permute);
                state = self.encrypt_round_i(state, 1, &self.flags.permute);
                state = self.encrypt_round_i(state, 2, &self.flags.permute);
                state = self.encrypt_round_i(state, 3, &self.flags.permute);
                state = self.encrypt_round_i(state, 4, &self.flags.permute);
    
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
                state = self.decrypt_round_b(state, 4, &self.flags.permute);
                state = self.decrypt_round_b(state, 3, &self.flags.permute);
                state = self.decrypt_round_b(state, 2, &self.flags.permute);
                state = self.decrypt_round_b(state, 1, &self.flags.permute);
                state = self.decrypt_round_b(state, 0, &self.flags.permute);
                *cipher = *state.as_array();
            },

            Mode::INT => {
                let payload_i = unsafe {
                    std::mem::transmute::<[u8; 16], [u32; 4]>(*cipher)
                };
             
                let mut state = simd::u32x4::from_array(payload_i);
                state = self.decrypt_round_i(state, 4, &self.flags.permute);
                state = self.decrypt_round_i(state, 3, &self.flags.permute);
                state = self.decrypt_round_i(state, 2, &self.flags.permute);
                state = self.decrypt_round_i(state, 1, &self.flags.permute);
                state = self.decrypt_round_i(state, 0, &self.flags.permute);
    
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
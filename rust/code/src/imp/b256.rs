use std::simd::Simd;
use std::simd;

#[allow(unused)]
use rand::{Rng};

use super::primitives::{self, Permute};


// i32 prime: 112_012_097 <-> 1_347_249_345
// i16 prime: 30_011 <-> 39_923



const P: [usize; 32] = [18, 20, 4, 22, 21, 1, 11, 16, 5, 9, 19, 10, 15, 24, 6, 23, 28, 31, 7, 27, 12, 13, 17, 26, 30, 3, 25, 2, 8, 14, 0, 29];
const INV_P: [usize; 32] = [30, 5, 27, 25, 2, 8, 14, 18, 28, 9, 11, 6, 20, 21, 29, 12, 7, 22, 0, 10, 1, 4, 3, 15, 13, 26, 23, 19, 16, 31, 24, 17];
const PI: [usize; 8] = [6,3,1,4,0,7,5,2];
const INV_PI: [usize; 8] = [4,2,7,1,3,6,0,5];

const S: Simd<u8, 32> = simd::u8x32::from_array([113; 32]);
const INV_S: Simd<u8, 32> = simd::u8x32::from_array([145; 32]);

const SI: Simd<u32, 8> = simd::u32x8::from_array([1_347_249_345; 8]);
const INV_SI: Simd<u32, 8> = simd::u32x8::from_array([112_012_097; 8]);

const M1: Simd<u8, 32> = simd::u8x32::from_array([0b01010101; 32]);
const M2: Simd<u8, 32> = simd::u8x32::from_array([0b00110011; 32]);
const M3: Simd<u8, 32> = simd::u8x32::from_array([0b00001111; 32]);

const M1I: Simd<u32, 8> = simd::u32x8::from_array([0b01010101_01010101_01010101_01010101; 8]);
const M2I: Simd<u32, 8> = simd::u32x8::from_array([0b00110011_00110011_00110011_00110011; 8]);
const M3I: Simd<u32, 8> = simd::u32x8::from_array([0b00001111_00001111_00001111_00001111; 8]);
const M4I: Simd<u32, 8> = simd::u32x8::from_array([0b00000000_11111111_00000000_11111111; 8]);
const M5I: Simd<u32, 8> = simd::u32x8::from_array([0b00000000_00000000_11111111_11111111; 8]);

const SH1: Simd<u8, 32> = simd::u8x32::from_array([1; 32]);
const SH2: Simd<u8, 32> = simd::u8x32::from_array([2; 32]);
const SH3: Simd<u8, 32> = simd::u8x32::from_array([4; 32]);

const SH1I: Simd<u32, 8> = simd::u32x8::from_array([1; 8]);
const SH2I: Simd<u32, 8> = simd::u32x8::from_array([2; 8]);
const SH3I: Simd<u32, 8> = simd::u32x8::from_array([4; 8]);
const SH4I: Simd<u32, 8> = simd::u32x8::from_array([8; 8]);
const SH5I: Simd<u32, 8> = simd::u32x8::from_array([16; 8]);

pub struct Tekton256 {
    keys: [[u8; 32]; 8],
    flags: primitives::Flags
}

impl Tekton256 {

    pub fn new(key: [u8; 32], flags: primitives::Flags) -> Tekton256{

        let mut keys: [[u8; 32]; 8] = [[0; 32]; 8];

        for i in 0..8 {
            let bytes = key.map(|x| (x << i).wrapping_mul(113));
            let ki = bytes;
            keys[i] = ki;
        }

        return Tekton256 {
            keys,
            flags
        }
    }

    #[inline]
    pub fn encrypt_round_b(&self, state: Simd<u8, 32>, i: usize, mode: &Permute) -> Simd<u8, 32>{
        let mut s = state;
        s ^= simd::u8x32::from_array(self.keys[i]);
        s = primitives::diffusion_byte(s, M1, M2, M3, SH1, SH2, SH3);
        s = primitives::substitute!(s, S);
        s = if *mode == Permute::PERMUTE {
            primitives::permute!(s, P)
        }
        else {
            primitives::rotate_byte(s)
        };
        s
    }

    #[inline]
    pub fn decrypt_round_b(&self, state: Simd<u8, 32>, i: usize, mode: &Permute) -> Simd<u8, 32>{
        let mut s = state;
        s = if *mode == Permute::PERMUTE {
            primitives::permute!(s, INV_P)
        }
        else {
            primitives::inverse_rotate_byte(s)
        };
        s = primitives::substitute!(s, INV_S);
        s = primitives::diffusion_byte(s, M1, M2, M3, SH1, SH2, SH3);
        s ^= simd::u8x32::from_array(self.keys[i]);
        s
    }

    #[inline]
    pub fn encrypt_round_i(&self, state: Simd<u32, 8>, i: usize, mode: &Permute) -> Simd<u32, 8>{
        let mut s = state;
        let key = unsafe {
            std::mem::transmute::<[u8; 32], [u32; 8]>(self.keys[i])
        };

        s ^= simd::u32x8::from_array(key);
        s = primitives::diffusion_int(s, M1I, M2I, M3I, M4I, M5I, SH1I, SH2I, SH3I, SH4I, SH5I);
        s = primitives::substitute!(s, SI);
        s = if *mode == Permute::PERMUTE {
            primitives::permute!(s, PI)
        }
        else {
            primitives::rotate_int(s)
        };
        s
    }

    #[inline]
    pub fn decrypt_round_i(&self, state: Simd<u32, 8>, i: usize, mode: &Permute) -> Simd<u32, 8>{
        let mut s = state;
        let key = unsafe {
            std::mem::transmute::<[u8; 32], [u32; 8]>(self.keys[i])
        };

        s = if *mode == Permute::PERMUTE {
            primitives::permute!(s, INV_PI)
        }
        else {
            primitives::inverse_rotate_int(s)
        };
        s = primitives::substitute!(s, INV_SI);
        s = primitives::diffusion_int(s, M1I, M2I, M3I, M4I, M5I, SH1I, SH2I, SH3I, SH4I, SH5I);
        s ^= simd::u32x8::from_array(key);
        s
    }

    #[inline]
    pub fn encrypt(&self, payload: &mut [u8; 32]){

        if self.flags.mode == primitives::Mode::BYTE {

            let mut state = simd::u8x32::from_array(*payload);
            state = self.encrypt_round_b(state, 0, &self.flags.permute);
            state = self.encrypt_round_b(state, 1, &self.flags.permute);
            state = self.encrypt_round_b(state, 2, &self.flags.permute);
            state = self.encrypt_round_b(state, 3, &self.flags.permute);
            state = self.encrypt_round_b(state, 4, &self.flags.permute);
            *payload = *state.as_array();
        }
        else {
            let payload_i = unsafe {
                std::mem::transmute::<[u8; 32], [u32; 8]>(*payload)
            };
         
            let mut state = simd::u32x8::from_array(payload_i);
            state = self.encrypt_round_i(state, 0, &self.flags.permute);
            state = self.encrypt_round_i(state, 1, &self.flags.permute);
            state = self.encrypt_round_i(state, 2, &self.flags.permute);
            state = self.encrypt_round_i(state, 3, &self.flags.permute);
            state = self.encrypt_round_i(state, 4, &self.flags.permute);

            *payload = unsafe {
                std::mem::transmute::<[u32; 8], [u8; 32]>(*state.as_array())
            };
        }
    }

    #[inline]
    pub fn decrypt(&self, cipher: &mut [u8; 32]){
        if self.flags.mode == primitives::Mode::BYTE {

            let mut state = simd::u8x32::from_array(*cipher);
            state = self.decrypt_round_b(state, 4, &self.flags.permute);
            state = self.decrypt_round_b(state, 3, &self.flags.permute);
            state = self.decrypt_round_b(state, 2, &self.flags.permute);
            state = self.decrypt_round_b(state, 1, &self.flags.permute);
            state = self.decrypt_round_b(state, 0, &self.flags.permute);
            *cipher = *state.as_array();
        }
        else {
            let payload_i = unsafe {
                std::mem::transmute::<[u8; 32], [u32; 8]>(*cipher)
            };
         
            let mut state = simd::u32x8::from_array(payload_i);
            state = self.decrypt_round_i(state, 4, &self.flags.permute);
            state = self.decrypt_round_i(state, 3, &self.flags.permute);
            state = self.decrypt_round_i(state, 2, &self.flags.permute);
            state = self.decrypt_round_i(state, 1, &self.flags.permute);
            state = self.decrypt_round_i(state, 0, &self.flags.permute);

            *cipher = unsafe {
                std::mem::transmute::<[u32; 8], [u8; 32]>(*state.as_array())
            };
        }
    }
}


#[test]
fn test_encrypt_decrypt(){
    let lo_key: u128 = rand::thread_rng().gen();
    let hi_key: u128 = rand::thread_rng().gen();

    let mut key: [u8; 32] = [0; 32];
    key[..16].copy_from_slice(&lo_key.to_be_bytes());
    key[16..32].copy_from_slice(&hi_key.to_be_bytes());


    let test_in_loop = |tekton: Tekton256| {
        for _ in 0..1000 {
            let lo_p: u128 = rand::thread_rng().gen();
            let hi_p: u128 = rand::thread_rng().gen();

            let mut pb: [u8; 32] = [0; 32];
            pb[..16].copy_from_slice(&lo_p.to_be_bytes());
            pb[16..32].copy_from_slice(&hi_p.to_be_bytes());
    
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
        primitives::Flags {
            permute: primitives::Permute::PERMUTE, 
            mode: primitives::Mode::BYTE}));

    test_in_loop(
        Tekton256::new(key, 
        primitives::Flags {
            permute: primitives::Permute::ROTATE, 
            mode: primitives::Mode::BYTE}));

    test_in_loop(
        Tekton256::new(key, 
        primitives::Flags {
            permute: primitives::Permute::PERMUTE, 
            mode: primitives::Mode::INT}));

    test_in_loop(
        Tekton256::new(key, 
        primitives::Flags {
            permute: primitives::Permute::ROTATE, 
            mode: primitives::Mode::INT}));

    
}
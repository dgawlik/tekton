// use std::simd::Simd;
// use std::simd;

// #[allow(unused)]
// use rand::{Rng};

// use super::primitives::{self, permute};



// const P: [usize; 32] = [18, 20, 4, 22, 21, 1, 11, 16, 5, 9, 19, 10, 15, 24, 6, 23, 28, 31, 7, 27, 12, 13, 17, 26, 30, 3, 25, 2, 8, 14, 0, 29];
// const INV_P: [usize; 32] = [30, 5, 27, 25, 2, 8, 14, 18, 28, 9, 11, 6, 20, 21, 29, 12, 7, 22, 0, 10, 1, 4, 3, 15, 13, 26, 23, 19, 16, 31, 24, 17];

// const S: Simd<u8, 32> = simd::u8x32::from_array([113; 32]);
// const INV_S: Simd<u8, 32> = simd::u8x32::from_array([145; 32]);

// const M1: Simd<u8, 32> = simd::u8x32::from_array([0b01010101; 32]);
// const M2: Simd<u8, 32> = simd::u8x32::from_array([0b00110011; 32]);
// const M3: Simd<u8, 32> = simd::u8x32::from_array([0b00001111; 32]);

// const SH1: Simd<u8, 32> = simd::u8x32::from_array([1; 32]);
// const SH2: Simd<u8, 32> = simd::u8x32::from_array([2; 32]);
// const SH3: Simd<u8, 32> = simd::u8x32::from_array([4; 32]);



// pub struct Tekton256 {
//     keys: [[u8; 32]; 8],
// }

// impl Tekton256 {

//     pub fn new(key: [u8; 32]) -> Tekton256{

//         let mut keys: [[u8; 32]; 8] = [[0; 32]; 8];

//         for i in 0..8 {
//             let bytes = key.map(|x| (x << i).wrapping_mul(113));
//             let ki = bytes;
//             keys[i] = ki;
//         }

//         return Tekton256 {
//             keys
//         }
//     }

//     #[inline]
//     pub fn encrypt(&self, payload: &mut [u8; 32]){

//         let mut state = simd::u8x32::from_array(*payload);
//         state = permute!(state, P);
//         state ^= simd::u8x32::from_array(self.keys[0]);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state = primitives::substitute!(state, S);
//         state = primitives::rotate(state);
       
//         state ^= simd::u8x32::from_array(self.keys[1]);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state = primitives::substitute!(state, S);
//         state = primitives::rotate(state);

//         state ^= simd::u8x32::from_array(self.keys[2]);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state = primitives::substitute!(state, S);
//         state = primitives::rotate(state);

//         state ^= simd::u8x32::from_array(self.keys[3]);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state = primitives::substitute!(state, S);
//         state = primitives::rotate(state);

//         state ^= simd::u8x32::from_array(self.keys[4]);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state = primitives::substitute!(state, S);
//         state = primitives::rotate(state);

//         *payload = *state.as_array();
//     }

//     #[inline]
//     pub fn decrypt(&self, cipher: &mut [u8; 32]){
//         let mut state = simd::u8x32::from_array(*cipher);
//         state = primitives::inverse_rotate(state);
//         state = primitives::substitute!(state, INV_S);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state ^= simd::u8x32::from_array(self.keys[4]);
      

//         state = primitives::inverse_rotate(state);
//         state = primitives::substitute!(state, INV_S);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state ^= simd::u8x32::from_array(self.keys[3]);

//         state = primitives::inverse_rotate(state);
//         state = primitives::substitute!(state, INV_S);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state ^= simd::u8x32::from_array(self.keys[2]);

//         state = primitives::inverse_rotate(state);
//         state = primitives::substitute!(state, INV_S);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state ^= simd::u8x32::from_array(self.keys[1]);

//         state = primitives::inverse_rotate(state);
//         state = primitives::substitute!(state, INV_S);
//         state = primitives::diffusion(state, M1, M2, M3, SH1, SH2, SH3);
//         state ^= simd::u8x32::from_array(self.keys[0]);
//         state = permute!(state, INV_P);

//         *cipher = *state.as_array()
//     }
// }


// #[test]
// fn test_encrypt_decrypt(){
//     let lo_key: u128 = rand::thread_rng().gen();
//     let hi_key: u128 = rand::thread_rng().gen();

//     let mut key: [u8; 32] = [0; 32];
//     key[..16].copy_from_slice(&lo_key.to_be_bytes());
//     key[16..32].copy_from_slice(&hi_key.to_be_bytes());

//     let tekton = Tekton256::new(key);

//     for _ in 0..1000 {
//         let lo_p: u128 = rand::thread_rng().gen();
//         let hi_p: u128 = rand::thread_rng().gen();

//         let mut p: [u8; 32] = [0; 32];
//         p[..16].copy_from_slice(&lo_p.to_be_bytes());
//         p[16..32].copy_from_slice(&hi_p.to_be_bytes());

//         let mut enc = p.clone();
//         tekton.encrypt(&mut enc);

//         let mut dec = enc.clone();
//         tekton.decrypt(&mut dec);

//         assert_eq!(dec, p);
//         assert_ne!(enc, p);
//     }
// }
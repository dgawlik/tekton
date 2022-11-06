use std::simd::{self};

#[allow(unused)]
use rand::{Rng};

use std::simd::Simd;


#[inline]
fn diffusion(a: Simd<u8, 16>) -> Simd<u8, 16>{

    let a2: Simd<u8, 16> = a.rotate_lanes_left::<1>();

    let p1 = (a2 & M1) << SH1;
    let p2 = (a2 & M2) << SH2;
    let p3 = (a2 & M3) << SH3;

    return a ^ p1 ^ p2 ^ p3;
}

const P: [usize; 16] = [3, 7, 13, 0, 11, 1, 15, 2, 4, 12, 5, 9, 6, 8, 14, 10];
const INV_P: [usize; 16] = [3, 5, 7, 0, 8, 10, 12, 1, 13, 11, 15, 4, 9, 2, 14, 6];

const S: Simd<u8, 16> = simd::u8x16::from_array([113; 16]);
const INV_S: Simd<u8, 16> = simd::u8x16::from_array([145; 16]);

const M1: Simd<u8, 16> = simd::u8x16::from_array([0b01010101; 16]);
const M2: Simd<u8, 16> = simd::u8x16::from_array([0b00110011; 16]);
const M3: Simd<u8, 16> = simd::u8x16::from_array([0b00001111; 16]);

const SH1: Simd<u8, 16> = simd::u8x16::from_array([1; 16]);
const SH2: Simd<u8, 16> = simd::u8x16::from_array([2; 16]);
const SH3: Simd<u8, 16> = simd::u8x16::from_array([4; 16]);


#[inline]
fn permute(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return simd::simd_swizzle!(a, P);
}

#[inline]
fn rotate(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_left::<5>();
}

#[inline]
fn inverse_rotate(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_right::<5>();
}

#[inline]
fn inverse_permute(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return simd::simd_swizzle!(a, INV_P);
}

#[inline]
fn substitute(a: Simd<u8, 16>) -> Simd<u8, 16> {
    return a * S;
}

#[inline]
fn inverse_substitute(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a * INV_S;
}

#[inline]
fn xor_with(a: Simd<u8, 16>, k: Simd<u8, 16>) -> Simd<u8, 16> {
   return a ^ k;
}


pub struct Tekton128 {
    keys: [[u8; 16]; 5],
}

impl Tekton128 {

    pub fn new(key: [u8; 16]) -> Tekton128{

        let mut keys: [[u8; 16]; 5] = [[0; 16]; 5];

        for i in 0..5 {
            let bytes = key.map(|x| (x << i).wrapping_mul(113));
            let ki = bytes;
            keys[i] = ki;
        }

        return Tekton128 {
            keys
        }
    }

    #[inline]
    pub fn encrypt(&self, payload: &mut [u8; 16]){

        let mut state = simd::u8x16::from_array(*payload);
        state = xor_with(state, simd::u8x16::from_array(self.keys[0]));
        state = diffusion(state);
        state = substitute(state);
        state = permute(state);
       
        state = xor_with(state, simd::u8x16::from_array(self.keys[1]));
        state = diffusion(state);
        state = substitute(state);
        state = rotate(state);

        state = xor_with(state, simd::u8x16::from_array(self.keys[2]));
        state = diffusion(state);
        state = substitute(state);
        state = rotate(state);

        state = xor_with(state, simd::u8x16::from_array(self.keys[3]));
        state = diffusion(state);
        state = substitute(state);
        state = rotate(state);

        state = xor_with(state, simd::u8x16::from_array(self.keys[4]));
        state = diffusion(state);
        state = substitute(state);
        state = rotate(state);

        *payload = *state.as_array();
    }

    #[inline]
    pub fn decrypt(&self, cipher: &mut [u8; 16]){
        let mut state = simd::u8x16::from_array(*cipher);
        state = inverse_rotate(state);
        state = inverse_substitute(state);
        state = diffusion(state);
        state = xor_with(state, simd::u8x16::from_array(self.keys[4]));
      

        state = inverse_rotate(state);
        state = inverse_substitute(state);
        state = diffusion(state);
        state = xor_with(state, simd::u8x16::from_array(self.keys[3]));

        state = inverse_rotate(state);
        state = inverse_substitute(state);
        state = diffusion(state);
        state = xor_with(state, simd::u8x16::from_array(self.keys[2]));

        state = inverse_rotate(state);
        state = inverse_substitute(state);
        state = diffusion(state);
        state = xor_with(state, simd::u8x16::from_array(self.keys[1]));

        state = inverse_permute(state);
        state = inverse_substitute(state);
        state = diffusion(state);
        state = xor_with(state, simd::u8x16::from_array(self.keys[0]));

        *cipher = *state.as_array()
    }
}


#[test]
fn test_encrypt_decrypt(){
    let key: u128 = rand::thread_rng().gen();

    let tekton = Tekton128::new(key.to_be_bytes());

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
}
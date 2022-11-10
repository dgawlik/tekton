use std::simd::{Simd};
use crate::imp::{Flags, Permute};
use std::simd;


#[inline]
pub fn rotate_b(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_left::<5>();
}

#[inline]
pub fn inverse_rotate_b(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_right::<5>();
}

#[inline]
pub fn rotate_i(a: Simd<u32, 4>) -> Simd<u32, 4>{
    let b = a.rotate_lanes_left::<1>();
    return (b >> simd::u32x4::splat(8))|(b << (simd::u32x4::splat(24)));
}

#[inline]
pub fn inverse_rotate_i(a: Simd<u32, 4>) -> Simd<u32, 4>{
    let b = a.rotate_lanes_right::<1>();
    return (b << simd::u32x4::splat(8))|(b >> (simd::u32x4::splat(24)));
}


#[inline]
pub fn diffusion_b(a: Simd<u8, 16>, 
    m1: Simd<u8, 16>, m2:Simd<u8, 16>, m3:Simd<u8,16>,
    sh1: Simd<u8, 16>, sh2:Simd<u8, 16>, sh3:Simd<u8,16>,) -> Simd<u8, 16>{

    let a2: Simd<u8, 16> = a.rotate_lanes_left::<1>();
    // let a3: Simd<u8, 16> = a.rotate_lanes_left::<2>();

    let p1 = (a2 & m1) << sh1;
    let p2 = (a2 & m2) << sh2;
    let p3 = (a2 & m3) << sh3;

    // let p4 = (a3 & m1) << sh1;
    // let p5 = (a3 & m2) << sh2;
    // let p6 = (a3 & m3) << sh3;

    return a ^ p1 ^ p2 ^ p3;
}

#[inline]
pub fn diffusion_i(a: Simd<u32, 4>, 
    m1: Simd<u32, 4>, m2:Simd<u32, 4>, m3:Simd<u32,4>, m4: Simd<u32, 4>, m5: Simd<u32, 4>,
    sh1: Simd<u32, 4>, sh2:Simd<u32, 4>, sh3:Simd<u32,4>, sh4:Simd<u32, 4>, sh5: Simd<u32, 4>) -> Simd<u32, 4>{

    let a2: Simd<u32, 4> = a.rotate_lanes_left::<1>();

    let p1 = (a2 & m1) << sh1;
    let p2 = (a2 & m2) << sh2;
    let p3 = (a2 & m3) << sh3;
    let p4 = (a2 & m4) << sh4;
    let p5 = (a2 & m5) << sh5;

    return a ^ p1 ^ p2 ^ p3 ^ p4 ^ p5;
}



macro_rules! permute {
   
    ($a:expr, $lit:expr) => {
        simd::simd_swizzle!($a, $lit)
    };
}

pub(crate) use permute;

macro_rules! substitute {
   
    ($a:expr, $s:expr) => {
        $a * $s
    };
}

pub(crate) use substitute;


const P: [usize; 16] = [15, 12, 13, 14, 3,0, 1, 2,7,4, 5, 6,11,8,9,10];
const INV_P: [usize; 16] = [5,6,7,4,9,10,11,8,13,14,15,12,1,2,3, 0];
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


#[inline]
pub fn encrypt_round_b(state: Simd<u8, 16>, key: [u8; 16], mode: &Flags) -> Simd<u8, 16>{
    let mut s = state;
    s ^= simd::u8x16::from_array(key);
    s = diffusion_b(s, M1, M2, M3, SH1, SH2, SH3);
    s = substitute!(s, S);

    s =  match mode.permute {
        Permute::PERMUTE => permute!(s, P),
        Permute::ROTATE =>  rotate_b(s)
    };

    s
}

#[inline]
pub fn decrypt_round_b(state: Simd<u8, 16>, key: [u8; 16], mode: &Flags) -> Simd<u8, 16>{
    let mut s = state;

    s = match mode.permute {
        Permute::PERMUTE => permute!(s, INV_P),
        Permute::ROTATE => inverse_rotate_b(s)
    };
    
    s = substitute!(s, INV_S);
    s = diffusion_b(s, M1, M2, M3, SH1, SH2, SH3);
    s ^= simd::u8x16::from_array(key);
    s
}

#[inline]
pub fn encrypt_round_i(state: Simd<u32, 4>, key: [u8; 16], mode: &Flags) -> Simd<u32, 4>{
    let mut s = state;
    let key = unsafe {
        std::mem::transmute::<[u8; 16], [u32; 4]>(key)
    };

    s ^= simd::u32x4::from_array(key);
    s = diffusion_i(s, M1I, M2I, M3I, M4I, M5I, SH1I, SH2I, SH3I, SH4I, SH5I);
    s = substitute!(s, SI);

    s = match mode.permute {
        Permute::PERMUTE => permute!(s, PI),
        Permute::ROTATE => rotate_i(s)
    };
    
    s
}

#[inline]
pub fn decrypt_round_i(state: Simd<u32, 4>, key: [u8; 16], mode: &Flags) -> Simd<u32, 4>{
    let mut s = state;
    let key = unsafe {
        std::mem::transmute::<[u8; 16], [u32; 4]>(key)
    };

    s = match mode.permute {
        Permute::PERMUTE => permute!(s, INV_PI),
        Permute::ROTATE => inverse_rotate_i(s)
    };
    
    s = substitute!(s, INV_SI);
    s = diffusion_i(s, M1I, M2I, M3I, M4I, M5I, SH1I, SH2I, SH3I, SH4I, SH5I);
    s ^= simd::u32x4::from_array(key);
    s
}
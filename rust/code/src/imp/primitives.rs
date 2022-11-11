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
pub fn diffusion_b(a: Simd<u8, 16>, inverse: bool) -> Simd<u8, 16>{

    let A = a;

    let mut p1 = (A & M1b) << simd::u8x16::splat(1);
    let mut p2 = (A & M2b) << simd::u8x16::splat(2);
    let mut p3 = (A & M3b) << simd::u8x16::splat(4);
    let mut p4 = (A & M4b).rotate_lanes_left::<1>();
    let mut p5 = (A & M5b).rotate_lanes_left::<2>();
    let mut p6 = (A & M6b).rotate_lanes_left::<4>();
    let mut p7 = (A & M7b).rotate_lanes_left::<8>();
    if inverse {
        p1 = (A & !M1b) >> simd::u8x16::splat(1);
        p2 = (A & !M2b) >> simd::u8x16::splat(2);
        p3 = (A & !M3b) >> simd::u8x16::splat(4);
        p4 = (A & !M4b).rotate_lanes_right::<1>();
        p5 = (A & !M5b).rotate_lanes_right::<2>();
        p6 = (A & !M6b).rotate_lanes_right::<4>();
        p7 = (A & !M7b).rotate_lanes_right::<8>();
    }

    return A ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6 ^ p7;
}

#[inline]
pub fn diffusion_i(a: Simd<u32, 4>, inverse: bool) -> Simd<u32, 4>{
    let A = a;

    let mut p1 = (A & M1i) << simd::u32x4::splat(1);
    let mut p2 = (A & M2i) << simd::u32x4::splat(2);
    let mut p3 = (A & M3i) << simd::u32x4::splat(4);
    let mut p4 = (A & M4i) << simd::u32x4::splat(8);
    let mut p5 = (A & M5i) << simd::u32x4::splat(16);
    let mut p6 = (A & M6i).rotate_lanes_left::<1>();
    let mut p7 = (A & M7i).rotate_lanes_left::<2>();

    if inverse {
        p1 = (A & !M1i) >> simd::u32x4::splat(1);
        p2 = (A & !M2i) >> simd::u32x4::splat(2);
        p3 = (A & !M3i) >> simd::u32x4::splat(4);
        p4 = (A & !M4i) >> simd::u32x4::splat(8);
        p5 = (A & !M5i) >> simd::u32x4::splat(16);
        p6 = (A & !M6i).rotate_lanes_right::<1>();
        p7 = (A & !M7i).rotate_lanes_right::<2>();
    }

    return A ^ p1 ^ p2 ^ p3 ^ p4 ^ p5 ^ p6 ^ p7;
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


const P: [usize; 16] = [7, 4, 5, 6,  11, 8, 9, 10, 15, 12, 13, 14,  3, 0, 1, 2,];
const INV_P: [usize; 16] = [13, 14, 15, 12, 1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8];
const PI: [usize; 4] = [1,3,0,2];
const INV_PI: [usize; 4] = [2,0,3,1];

const S: Simd<u8, 16> = simd::u8x16::from_array([191; 16]);
const INV_S: Simd<u8, 16> = simd::u8x16::from_array([63; 16]);

const SI: Simd<u32, 4> = simd::u32x4::from_array([1_347_249_345; 4]);
const INV_SI: Simd<u32, 4> = simd::u32x4::from_array([112_012_097; 4]);


// const M1: u128 = 0b01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101_01010101;
// const M2: u128 = 0b00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011_00110011;
// const M3: u128 = 0b00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111_00001111;
// const M4: u128 = 0b00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111_00000000_11111111;
// const M5: u128 = 0b00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111_00000000_00000000_11111111_11111111;
// const M6: u128 = 0b00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111;
// const M7: u128 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;


const M1b: Simd<u8, 16> = simd::u8x16::from_array([0b01010101; 16]);
const M2b: Simd<u8, 16> = simd::u8x16::from_array([0b00110011; 16]);
const M3b: Simd<u8, 16> = simd::u8x16::from_array([0b00001111; 16]);
const M4b: Simd<u8, 16> = simd::u8x16::from_array([0b00000000 ,0b11111111 ,0b00000000 ,0b11111111 ,0b00000000 ,0b11111111 ,0b00000000 ,0b11111111 ,0b00000000 ,0b11111111 ,0b00000000 ,0b11111111 ,0b00000000 ,0b11111111 ,0b00000000 ,0b11111111]);
const M5b: Simd<u8, 16> = simd::u8x16::from_array([0b00000000 ,0b00000000 ,0b11111111 ,0b11111111 ,0b00000000 ,0b00000000 ,0b11111111 ,0b11111111 ,0b00000000 ,0b00000000 ,0b11111111 ,0b11111111 ,0b00000000 ,0b00000000 ,0b11111111 ,0b11111111]);
const M6b: Simd<u8, 16> = simd::u8x16::from_array([0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111 ,0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111]);
const M7b: Simd<u8, 16> = simd::u8x16::from_array([0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b00000000 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111 ,0b11111111]);


const M1i: Simd<u32, 4> = simd::u32x4::from_array([0b01010101_01010101_01010101_01010101; 4]);
const M2i: Simd<u32, 4> = simd::u32x4::from_array([0b00110011_00110011_00110011_00110011; 4]);
const M3i: Simd<u32, 4> = simd::u32x4::from_array([0b00001111_00001111_00001111_00001111; 4]);
const M4i: Simd<u32, 4> = simd::u32x4::from_array([0b00000000_11111111_00000000_11111111, 0b00000000_11111111_00000000_11111111, 0b00000000_11111111_00000000_11111111, 0b00000000_11111111_00000000_11111111]);
const M5i: Simd<u32, 4> = simd::u32x4::from_array([0b00000000_00000000_11111111_11111111, 0b00000000_00000000_11111111_11111111, 0b00000000_00000000_11111111_11111111, 0b00000000_00000000_11111111_11111111]);
const M6i: Simd<u32, 4> = simd::u32x4::from_array([0b00000000_00000000_00000000_00000000, 0b11111111_11111111_11111111_11111111, 0b00000000_00000000_00000000_00000000, 0b11111111_11111111_11111111_11111111]);
const M7i: Simd<u32, 4> = simd::u32x4::from_array([0b00000000_00000000_00000000_00000000, 0b00000000_00000000_00000000_00000000, 0b11111111_11111111_11111111_11111111, 0b11111111_11111111_11111111_11111111]);



#[inline]
pub fn encrypt_round_b(state: Simd<u8, 16>, key: [u8; 16], mode: &Flags) -> Simd<u8, 16>{
    let mut s = state;
    s ^= simd::u8x16::from_array(key);
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
    s ^= simd::u32x4::from_array(key);
    s
}
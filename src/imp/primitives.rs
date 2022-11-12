use std::simd::{Simd};
use std::simd;


#[inline]
pub fn rotate_b(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_left::<7>();
}

#[inline]
pub fn inverse_rotate_b(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_right::<7>();
}

#[inline]
pub fn rotate_i(a: Simd<u16, 8>) -> Simd<u16, 8>{
    let b = a.rotate_lanes_left::<2>();
    return (b >> simd::u16x8::splat(8))|(b << (simd::u16x8::splat(8)));
}

#[inline]
pub fn inverse_rotate_i(a: Simd<u16, 8>) -> Simd<u16, 8>{
    let b = a.rotate_lanes_right::<2>();
    return (b << simd::u16x8::splat(8))|(b >> (simd::u16x8::splat(8)));
}

#[inline]
pub fn expansion_b(a: Simd<u8, 16>) -> Simd<u8, 16> {
    let b: Simd<u64, 2> = unsafe {
        std::mem::transmute::<Simd<u8, 16>, Simd<u64, 2>>(a)
    };

    let b = b * E;
    

    return unsafe {
        std::mem::transmute::<Simd<u64, 2>, Simd<u8, 16>>(b)
    };
}



#[inline]
pub fn inv_expansion_b(a: Simd<u8, 16>) -> Simd<u8, 16> {
    let b: Simd<u64, 2> = unsafe {
        std::mem::transmute::<Simd<u8, 16>, Simd<u64, 2>>(a)
    };

    let b = b * INV_E;

    return unsafe {
        std::mem::transmute::<Simd<u64, 2>, Simd<u8, 16>>(b)
    };
}

#[inline]
pub fn expansion_i(a: Simd<u16, 8>) -> Simd<u16, 8> {
    let b: Simd<u64, 2> = unsafe {
        std::mem::transmute::<Simd<u16, 8>, Simd<u64, 2>>(a)
    };

    let b = b * E;
    

    return unsafe {
        std::mem::transmute::<Simd<u64, 2>, Simd<u16, 8>>(b)
    };
}



#[inline]
pub fn inv_expansion_i(a: Simd<u16, 8>) -> Simd<u16, 8> {
    let b: Simd<u64, 2> = unsafe {
        std::mem::transmute::<Simd<u16, 8>, Simd<u64, 2>>(a)
    };

    let b = b * INV_E;

    return unsafe {
        std::mem::transmute::<Simd<u64, 2>, Simd<u16, 8>>(b)
    };
}






macro_rules! substitute {
   
    ($a:expr, $s:expr) => {
        $a * $s
    };
}

pub(crate) use substitute;


const S: Simd<u8, 16> = simd::u8x16::from_array([191; 16]);
const INV_S: Simd<u8, 16> = simd::u8x16::from_array([63; 16]);

const SS: Simd<u16, 8> = simd::u16x8::from_array([38805; 8]);
const INV_SS: Simd<u16, 8> = simd::u16x8::from_array([64445; 8]);

const E: Simd<u64, 2> = simd::u64x2::from_array([0b01001111_01001111_01001111_01001111_01001111_01001111_01001111_01001111; 2]);
const INV_E: Simd<u64, 2> = simd::u64x2::from_array([1167515447703136175; 2]);




#[inline]
pub fn encrypt_round_b(state: Simd<u8, 16>, key: Simd<u8, 16>) -> Simd<u8, 16>{
    let mut s = state;
    s ^= key;
    s = expansion_b(s);
    s = substitute!(s, S);
    s =  rotate_b(s);

    s
}

#[inline]
pub fn decrypt_round_b(state: Simd<u8, 16>, key: Simd<u8, 16>) -> Simd<u8, 16>{
    let mut s = state;

    s = inverse_rotate_b(s);
    s = substitute!(s, INV_S);
    s = inv_expansion_b(s);
    s ^= key;
    s
}

#[inline]
pub fn encrypt_round_i(state: Simd<u16, 8>, key: Simd<u8, 16>) -> Simd<u16, 8>{
    let mut s = state;
    let key = unsafe {
        std::mem::transmute::<Simd<u8, 16>, Simd<u16, 8>>(key)
    };

    s ^= key;
    s = expansion_i(s);
    s = substitute!(s, SS);
    s = rotate_i(s);
    
    s
}

#[inline]
pub fn decrypt_round_i(state: Simd<u16, 8>, key: Simd<u8, 16>) -> Simd<u16, 8>{
    let mut s = state;
    let key = unsafe {
        std::mem::transmute::<Simd<u8, 16>, Simd<u16, 8>>(key)
    };
    s= inverse_rotate_i(s);
    
    s = substitute!(s, INV_SS);
    s = inv_expansion_i(s);
    s ^= key;
    s
}
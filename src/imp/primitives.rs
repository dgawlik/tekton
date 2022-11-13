use std::simd::{Simd};
use std::simd;


#[inline]
pub fn rotate(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_left::<7>();
}

#[inline]
pub fn inverse_rotate(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_right::<7>();
}


#[inline]
pub fn expansion(a: Simd<u8, 16>) -> Simd<u8, 16> {
    let b: Simd<u64, 2> = unsafe {
        std::mem::transmute::<Simd<u8, 16>, Simd<u64, 2>>(a)
    };

    let b = b * E;
    

    return unsafe {
        std::mem::transmute::<Simd<u64, 2>, Simd<u8, 16>>(b)
    };
}



#[inline]
pub fn inv_expansion(a: Simd<u8, 16>) -> Simd<u8, 16> {
    let b: Simd<u64, 2> = unsafe {
        std::mem::transmute::<Simd<u8, 16>, Simd<u64, 2>>(a)
    };

    let b = b * INV_E;

    return unsafe {
        std::mem::transmute::<Simd<u64, 2>, Simd<u8, 16>>(b)
    };
}






const S: Simd<u8, 16> = simd::u8x16::from_array([191; 16]);
const INV_S: Simd<u8, 16> = simd::u8x16::from_array([63; 16]);

const E: Simd<u64, 2> = simd::u64x2::from_array([0b01001111_01001111_01001111_01001111_01001111_01001111_01001111_01001111; 2]);
const INV_E: Simd<u64, 2> = simd::u64x2::from_array([1167515447703136175; 2]);




#[inline]
pub fn encrypt_round(state: Simd<u8, 16>, key: Simd<u8, 16>) -> Simd<u8, 16>{
    let mut s = state;
    s ^= key;
    s = expansion(s);
    s = s * S;
    s =  rotate(s);

    s
}

#[inline]
pub fn decrypt_round(state: Simd<u8, 16>, key: Simd<u8, 16>) -> Simd<u8, 16>{
    let mut s = state;

    s = inverse_rotate(s);
    s = s * INV_S;
    s = inv_expansion(s);
    s ^= key;
    s
}
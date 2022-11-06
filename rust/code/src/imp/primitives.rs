use std::simd::Simd;


#[inline]
pub fn rotate(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_left::<5>();
}

#[inline]
pub fn inverse_rotate(a: Simd<u8, 16>) -> Simd<u8, 16>{
    return a.rotate_lanes_right::<5>();
}

#[inline]
pub fn diffusion(a: Simd<u8, 16>, 
    m1: Simd<u8, 16>, m2:Simd<u8, 16>, m3:Simd<u8,16>,
    sh1: Simd<u8, 16>, sh2:Simd<u8, 16>, sh3:Simd<u8,16>,) -> Simd<u8, 16>{

    let a2: Simd<u8, 16> = a.rotate_lanes_left::<1>();

    let p1 = (a2 & m1) << sh1;
    let p2 = (a2 & m2) << sh2;
    let p3 = (a2 & m3) << sh3;

    return a ^ p1 ^ p2 ^ p3;
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
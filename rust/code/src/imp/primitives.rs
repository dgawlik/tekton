use std::simd::{Simd, LaneCount, SupportedLaneCount};


#[inline]
pub fn rotate<const L: usize>(a: Simd<u8, L>) -> Simd<u8, L>
where LaneCount<L>: SupportedLaneCount{
    return a.rotate_lanes_left::<5>();
}

#[inline]
pub fn inverse_rotate<const L: usize>(a: Simd<u8, L>) -> Simd<u8, L>
where LaneCount<L>: SupportedLaneCount{
    return a.rotate_lanes_right::<5>();
}


#[inline]
pub fn diffusion<const L: usize>(a: Simd<u8, L>, 
    m1: Simd<u8, L>, m2:Simd<u8, L>, m3:Simd<u8,L>,
    sh1: Simd<u8, L>, sh2:Simd<u8, L>, sh3:Simd<u8,L>,) -> Simd<u8, L>
    where LaneCount<L>: SupportedLaneCount{

    let a2: Simd<u8, L> = a.rotate_lanes_left::<1>();

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
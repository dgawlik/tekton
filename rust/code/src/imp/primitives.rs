use std::simd::{Simd, LaneCount, SupportedLaneCount};


#[inline]
pub fn rotate_byte<const L: usize>(a: Simd<u8, L>) -> Simd<u8, L>
where LaneCount<L>: SupportedLaneCount{
    return a.rotate_lanes_left::<5>();
}

#[inline]
pub fn rotate_int<const L: usize>(a: Simd<u32, L>) -> Simd<u32, L>
where LaneCount<L>: SupportedLaneCount{
    return a.rotate_lanes_left::<1>();
}

#[inline]
pub fn inverse_rotate_byte<const L: usize>(a: Simd<u8, L>) -> Simd<u8, L>
where LaneCount<L>: SupportedLaneCount{
    return a.rotate_lanes_right::<5>();
}


#[inline]
pub fn inverse_rotate_int<const L: usize>(a: Simd<u32, L>) -> Simd<u32, L>
where LaneCount<L>: SupportedLaneCount{
    return a.rotate_lanes_right::<1>();
}


#[inline]
pub fn diffusion_byte<const L: usize>(a: Simd<u8, L>, 
    m1: Simd<u8, L>, m2:Simd<u8, L>, m3:Simd<u8,L>,
    sh1: Simd<u8, L>, sh2:Simd<u8, L>, sh3:Simd<u8,L>,) -> Simd<u8, L>
    where LaneCount<L>: SupportedLaneCount{

    let a2: Simd<u8, L> = a.rotate_lanes_left::<1>();

    let p1 = (a2 & m1) << sh1;
    let p2 = (a2 & m2) << sh2;
    let p3 = (a2 & m3) << sh3;

    return a ^ p1 ^ p2 ^ p3;
}

#[inline]
pub fn diffusion_int<const L: usize>(a: Simd<u32, L>, 
    m1: Simd<u32, L>, m2:Simd<u32, L>, m3:Simd<u32,L>, m4: Simd<u32, L>, m5: Simd<u32, L>,
    sh1: Simd<u32, L>, sh2:Simd<u32, L>, sh3:Simd<u32,L>, sh4:Simd<u32, L>, sh5: Simd<u32, L>) -> Simd<u32, L>
    where LaneCount<L>: SupportedLaneCount{

    let a2: Simd<u32, L> = a.rotate_lanes_left::<1>();

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


#[derive(PartialEq)]
pub enum Permute {
    PERMUTE,
    ROTATE
}


#[derive(PartialEq)]
pub enum Mode {
    BYTE,
    INT
}


pub struct Flags {
    pub permute: Permute,
    pub mode: Mode
}
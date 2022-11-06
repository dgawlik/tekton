#![feature(portable_simd)]



pub mod b128 {
    use std::simd::{self};
    use rand::{Rng};
    use std::simd::Simd;

    // fn gcd_extended(a: u16, b:u16, x: &mut u16, y: &mut u16) -> u16{
    //     if a == 0 {
    //         *x = 0_u16;
    //         *y = 1_u16;
    //         return b;
    //     }
    //     else {
    //         let mut x1: u16 = 0_u16;
    //         let mut y1: u16 = 0_u16;

    //         let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

    //         *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
    //         *y = x1;

    //         return gcd;
    //     }
    // }

    // fn mod_inverse(a: u8) -> u8 {
    //     let M = 256_u16;

    //     let mut x: u16 = 0;
    //     let mut y: u16 = 0;

    //     gcd_extended(a.into(), M, &mut x, &mut y);
    //     return (((x % M) + M) % M).to_be_bytes()[1];
    // }

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


    // #[test]
    // fn test_mod_inverse(){
    // let a: u8 = 113;
    // let inv_a = mod_inverse(a);

    // assert_eq!(1, a.wrapping_mul(inv_a));
    // }

    // #[test]
    // fn test_diffusion(){
    //     let n: u128 = rand::thread_rng().gen();
    //     let diff_n = diffusion(n);
    //     let diff2_n = diffusion(diff_n);

    //     assert_eq!(diff2_n, n);
    //     assert_ne!(n, diff_n);
    // }

    // #[test]
    // fn test_print_inverse_permutation(){
    //     let mut n: [usize; 16] = [0; 16];
    //     for i in 0..16 {
    //         n[P[i]] = i;
    //     }
    //     return println!("{:?}", n);
    // }

    // #[test]
    // fn test_print_inverse_substitution(){
    //     return println!("{:?}", mod_inverse(113));
    // }

    // #[test]
    // fn test_permute(){
    //     let n: u128 = rand::thread_rng().gen();
    //     let nb = n.to_be_bytes();
        
    //     let mut nbp = nb.clone();
    //     permute(&mut nbp);

    //     let mut n2b = nbp.clone();
    //     inverse_permute(&mut n2b);

    //     assert_eq!(n2b, nb);
    //     assert_ne!(nbp ,nb);
    // }

    // #[test]
    // fn test_substitute(){
    //     let n: u128 = rand::thread_rng().gen();
    //     let nb = n.to_be_bytes();
        
    //     let mut nsb = nb.clone();
    //     substitute(&mut nsb);

    //     let mut n2b = nsb.clone();
    //     inverse_substitute(&mut n2b);

    //     assert_eq!(n2b, nb);
    //     assert_ne!(nsb ,nb);
    // }

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
}

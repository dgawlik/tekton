

#[allow(dead_code)]
fn gcd_extended(a: u16, b:u16, x: &mut u16, y: &mut u16) -> u16{
    if a == 0 {
        *x = 0_u16;
        *y = 1_u16;
        return b;
    }
    else {
        let mut x1: u16 = 0_u16;
        let mut y1: u16 = 0_u16;

        let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

        *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
        *y = x1;

        return gcd;
    }
}

#[allow(dead_code)]
fn mod_inverse(a: u8) -> u8 {
    let m = 256_u16;

    let mut x: u16 = 0;
    let mut y: u16 = 0;

    gcd_extended(a.into(), m, &mut x, &mut y);
    return (((x % m) + m) % m) as u8;
}

pub struct Histogram<const F:usize>{
    bins: [u32; F],
    milestones: [u128; F]
}

impl<const F:usize> Histogram<F> {

    pub fn new() -> Histogram<F> {
        let mut milestones = [0; F];
        for i in 0..F {
            milestones[i] = Histogram::<F>::mark(i);
        }
        let bins = [0; F];

        return Histogram {
            bins, milestones
        }
    }

    fn mark(i: usize) -> u128 {
        return u128::from((i+1) as u32)*(u128::MAX/u128::from(F as u32));
    }

    pub fn update(&mut self, payload: [u8; 16]){
        let num = u128::from_be_bytes(payload);

        for i in 0..F {
            if num < Histogram::<F>::mark(i) {
                self.bins[i] += 1;
                break;
            }
        }
    }

    pub fn density(&self) -> [f64; F] {
        let mut count: f64 = 0.0;

        for i in 0..F {
            count += self.bins[i] as f64;
        }

        return self.bins.map(|x| (x as f64)/count);
    }

    pub fn uniformness(&self) -> f64 {
        let density = self.density();

        let mean: f64 = density.into_iter().sum::<f64>()/(density.len() as f64);

        let devs = density.map(|x| (x - mean).abs());

        return devs.into_iter().sum();
    }
}

#[test]
pub fn calc_inverse(){
    // print!("{}", mod_inverse(191));
    print!("{}", (63 as u8).wrapping_mul(191));
}

use bitreader::BitReader;

#[test]
pub fn find_best_substitution(){
    let mut primes: [u8; 256] = [0; 256];

    for i in 0..256 {
        primes[i] = i as u8; 
    }

    for i in 2..16 {
        let mut it = 2;
        while it*i < 256 {
            primes[it*i] = 0;
            it += 1;
        }
    }

    for p in primes.into_iter().filter(|x| *x != 1 && *x != 0){
        
        let mut hamming_dists: [f64; 256] = [0.0; 256];
        for i in 0..256 {
            let ri = (i as u8).wrapping_mul(p);

            let _i = (i as u8).to_be_bytes();
            let _ri = ri.to_be_bytes();

            let mut bi = BitReader::new(&_i);
            let mut bri = BitReader::new(&_ri);

            let mut count = 0;
            for j in 0..8 {
                if bi.read_bool() != bri.read_bool() {
                    count += 1;
                }
            }

            hamming_dists[i] = count as f64;
        }

        let mut avg: f64 = hamming_dists.into_iter().sum();
        avg /= 256.0;
        println!("{} avg hamming distance: {}", p, avg);
    }
}


#[test]
pub fn calculate_inverse_permutation(){
    let x: [u8; 16] = [7, 4, 5, 6,  11, 8, 9, 10, 15, 12, 13, 14,  3, 0, 1, 2,];
    let mut inv_x: [u8; 16] = [0; 16];

    for i in 0..16 {
        inv_x[x[i] as usize] = i as u8;
    }

    println!("{:?}", inv_x);
}


#[allow(dead_code)]
fn gcd_extended(a: u64, b:u64, x: &mut u64, y: &mut u64) -> u64{
    if a == 0 {
        *x = 0_u64;
        *y = 1_u64;
        return b;
    }
    else {
        let mut x1: u64 = 0_u64;
        let mut y1: u64 = 0_u64;

        let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

        *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
        *y = x1;

        return gcd;
    }
}

#[allow(dead_code)]
fn mod_inverse(a: u32) -> u32 {
    let m = 4294967296_u64;

    let mut x: u64 = 0;
    let mut y: u64 = 0;

    gcd_extended(a.into(), m, &mut x, &mut y);
    return (((x % m) + m) % m) as u32;
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
    // print!("{}", mod_inverse(112_012_097));
    print!("{}", (1347249345 as u32).wrapping_mul(112_012_097));
}
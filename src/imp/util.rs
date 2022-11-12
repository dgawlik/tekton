use num::{self, integer::Roots};
use bitreader::BitReader;
use is_prime;



pub trait NumUtil<T> {

    fn hamming_distance(&self, other: &T) -> i32;

    fn sbox_sanity(&self);

    fn print_binary(&self);

    fn is_prime(&self) -> bool;

    fn inverse(&self) -> T;
}

impl NumUtil<u8> for u8 {
    
    fn hamming_distance(&self, other: &u8) -> i32 {
        let s = self.to_be_bytes();
        let o = other.to_be_bytes();

        let mut sr = BitReader::new(&s);
        let mut or = BitReader::new(&o);

        let mut count = 0;
        for _ in 0..8 {
            if sr.read_bool() != or.read_bool() {
                count += 1;
            }
        }
        return count;
    }

    fn sbox_sanity(&self) {
        let p = *self as u16;

        for i in 0..256_u16 {
            let r = (i * p) % 256;
            if r == i {
                println!("identity on {}", i)
            }
        }
    }

    fn print_binary(&self) {
        println!("{:b}", self);
    }

    fn is_prime(&self) -> bool {
        return is_prime::is_prime(&self.to_string());
    }
    
    fn inverse(&self) -> u8 {
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
        
        fn mod_inverse(a: u8) -> u8 {
            let m = 256_u16;
        
            let mut x: u16 = 0;
            let mut y: u16 = 0;
        
            gcd_extended(a.into(), m, &mut x, &mut y);
            return (((x % m) + m) % m) as u8;
        }

        return mod_inverse(*self);
    }

}

impl NumUtil<u16> for u16 {
    
    fn hamming_distance(&self, other: &u16) -> i32 {
        let s = self.to_be_bytes();
        let o = other.to_be_bytes();

        let mut sr = BitReader::new(&s);
        let mut or = BitReader::new(&o);

        let mut count = 0;
        for _ in 0..16 {
            if sr.read_bool() != or.read_bool() {
                count += 1;
            }
        }
        return count;
    }

    fn sbox_sanity(&self) {
        let p = *self as u32;

        for i in 0..65536_u32 {
            let r = (i * p) % 65536;
            if r == i {
                println!("identity on {}", i)
            }
        }
    }

    fn print_binary(&self) {
        println!("{:b}", self);
    }

    fn is_prime(&self) -> bool {
        return is_prime::is_prime(&self.to_string());
    }
    
    fn inverse(&self) -> u16 {
        fn gcd_extended(a: u32, b:u32, x: &mut u32, y: &mut u32) -> u32{
            if a == 0 {
                *x = 0_u32;
                *y = 1_u32;
                return b;
            }
            else {
                let mut x1: u32 = 0_u32;
                let mut y1: u32 = 0_u32;
        
                let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);
        
                *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
                *y = x1;
        
                return gcd;
            }
        }
        
        fn mod_inverse(a: u16) -> u16 {
            let m = 65536_u32;
        
            let mut x: u32 = 0;
            let mut y: u32 = 0;
        
            gcd_extended(a.into(), m, &mut x, &mut y);
            return (((x % m) + m) % m) as u16;
        }

        return mod_inverse(*self);
    }

}



pub struct Histogram<const F:usize>{
    bins: [u32; F],
    #[allow(dead_code)]
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


pub fn primes<const L: usize>() -> Vec<i32>{
    let mut primes: [i32; L] = [0; L];

    for i in 0..L {
        primes[i] = i as i32; 
    }

    for i in 2..L.sqrt() {
        let mut it = 2;
        while it*i < 256 {
            primes[it*i] = 0;
            it += 1;
        }
    }

    return primes.into_iter().filter(|x| *x != 1 && *x != 0).collect()
}



#[test]
pub fn find_best_substitution(){

    for p in primes::<256>(){
        
        let mut hamming_dists: [f64; 256] = [0.0; 256];
        for i in 0..256 {
            hamming_dists[i] = (i as u8).hamming_distance(&(p as u8)) as f64;
        }

        let mut avg: f64 = hamming_dists.into_iter().sum();
        avg /= 256.0;
        println!("{} avg hamming distance: {}", p, avg);
    }
}

#[test]
pub fn find_best_substitution_u16(){
    use rand::{Rng};

    for i in 0..2_000 {
        let p: u16 = rand::thread_rng().gen();
        
        let mut hamming_dists: [f64; 1000] = [0.0; 1000];
        for i in 0..1000 {
            hamming_dists[i] = (i as u16).hamming_distance(&(p as u16)) as f64;
        }

        let mut avg: f64 = hamming_dists.into_iter().sum();
        avg /= 1000.0;
        println!("{} avg hamming distance: {}", p, avg);
    }
}

#[test]
pub fn print_inverse(){
    println!("{}", 38805_u16.inverse());
    println!("{}", 38805_u16.inverse().wrapping_mul(38805_u16)); //64445
}



pub fn fitness(p: u64) -> f64{
    use bitreader::BitReader;
    use rand::{Rng};

    let mut hamming_dists: [f64; 1_000] = [0.0; 1_000];
    for i in 0..1_000 {
        let num: u64 = rand::thread_rng().gen();
        let ri = num.wrapping_mul(p);

        let _i = num.to_be_bytes();
        let _ri = ri.to_be_bytes();

        let mut bi = BitReader::new(&_i);
        let mut bri = BitReader::new(&_ri);

        let mut count = 0;
        let mut ones = 0;
        for _ in 0..64 {
            let bbi = bi.read_bool().unwrap();
            let bbri = bri.read_bool().unwrap();
            // if bbi != bbri {
            //     count += 1;
            // }
            if bbri {
                ones += 1;
            }
        }

        count += 32-(ones - 32_i32).abs();

        hamming_dists[i] = count as f64;
    }

    let avg: f64 = hamming_dists.into_iter().sum();
    avg / 1_000.0
}



// 0b0111111111111111111111111111111111111111111111111111111111111111
// #[test]
// pub fn find_best_substitution_int(){
//     use genetic_algorithm::strategy::evolve::prelude::*;
//     use rand::{Rng};
//     extern crate is_prime;
//     use bitreader::BitReader;
//     use is_prime::*;
    
//     let genotype = BinaryGenotype::builder() 
//         .with_genes_size(64)            
//         .build()
//         .unwrap();


//     #[derive(Clone, Debug)]
//     pub struct CountTrue;
//     impl Fitness for CountTrue {
//         type Genotype = BinaryGenotype;
//         fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
            
//             let mut p: u64 = 0;
//             p |= 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
//             for i in 1..64 {
//                 p <<= 1;
//                 if chromosome.genes[i] {
//                     p |= 0b_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
//                 }  
//             }

//             // if !is_prime(&p.to_string()){
//             //     return Some(0 as FitnessValue)
//             // }

//             let avg = fitness(p);
//             println!("fitness: {}", avg);
//             Some(avg as FitnessValue)
//         }
//     }

//     let mut rng = rand::thread_rng();    
//     let evolve = Evolve::builder()
//         .with_genotype(genotype)
//         .with_population_size(100)        // evolve with 100 chromosomes
//         .with_target_fitness_score(30)   // goal is 100 times true in the best chromosome
//         .with_fitness(CountTrue)          // count the number of true values in the chromosomes
//         .with_crossover(CrossoverUniform(true)) // crossover all individual genes between 2 chromosomes for offspring
//         .with_mutate(MutateOnce(0.2))     // mutate a single gene with a 20% probability per chromosome
//         .with_compete(CompeteElite)       // sort the chromosomes by fitness to determine crossover order
//         .call(&mut rng)
//         .unwrap();

//     println!("{}", evolve);

// }

#[test]
pub fn calculate_inverse_permutation(){
    let x: [u8; 16] = [  5, 6,7,4,  13, 14,15,12,  9, 10,11, 8, 1, 2,3,0];
    let mut inv_x: [u8; 16] = [0; 16];

    for i in 0..16 {
        inv_x[x[i] as usize] = i as u8;
    }

    println!("{:?}", inv_x);
}
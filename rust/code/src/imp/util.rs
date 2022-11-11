

#[allow(dead_code)]
fn gcd_extended(a: u128, b:u128, x: &mut u128, y: &mut u128) -> u128{
    if a == 0 {
        *x = 0_u128;
        *y = 1_u128;
        return b;
    }
    else {
        let mut x1: u128 = 0_u128;
        let mut y1: u128 = 0_u128;

        let gcd = gcd_extended(b % a, a, &mut x1, &mut y1);

        *x = y1.wrapping_sub((b/a).wrapping_mul(x1));
        *y = x1;

        return gcd;
    }
}

#[allow(dead_code)]
fn mod_inverse(a: u64) -> u64 {
    let m = 9_223_372_036_854_775_808_u128;

    let mut x: u128 = 0;
    let mut y: u128 = 0;

    gcd_extended(a.into(), m, &mut x, &mut y);
    return (((x % m) + m) % m) as u64;
}

#[test]
pub fn calc_inverse(){
    println!("{}", mod_inverse(0b0111111111111111111111111111111111111111111111111111111111110101));
    println!("{}", (5869418568907584605 as u64).wrapping_mul(0b0111111111111111111111111111111111111111111111111111111111110101));
}

#[test]
pub fn print_binary(){
    println!("{:b}", 127);
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


// 0b1111111111111111111111111111111111111111111111111111011101110010
// 0b0111111111111111111111111111111111111111111111111111111111110101
// #[test]
// pub fn find_best_substitution_int(){
//     use genetic_algorithm::strategy::evolve::prelude::*;
//     use rand::{Rng};
//     extern crate is_prime;
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
//             p |= 0b_10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
//             for i in 1..64 {
//                 p >>= 1;
//                 if chromosome.genes[i] {
//                     p |= 0b_10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
//                 }  
//             }

//             // if !is_prime(&p.to_string()){
//             //     return Some(0 as FitnessValue)
//             // }

//             let mut hamming_dists: [f64; 1_000] = [0.0; 1_000];
//             for i in 0..1_000 {
//                 let num: u64 = rand::thread_rng().gen();
//                 let ri = num.wrapping_mul(p);

//                 let _i = num.to_be_bytes();
//                 let _ri = ri.to_be_bytes();

//                 let mut bi = BitReader::new(&_i);
//                 let mut bri = BitReader::new(&_ri);

//                 let mut count = 0;
//                 for j in 0..64 {
//                     if bi.read_bool() != bri.read_bool() {
//                         count += 1;
//                     }
//                 }

//                 hamming_dists[i] = count as f64;
//             }

//             let mut avg: f64 = hamming_dists.into_iter().sum();
//             avg /= 1_000.0;
//             println!("fitness: {}", avg);
//             Some(avg as FitnessValue)
//         }
//     }

//     let mut rng = rand::thread_rng();    
//     let evolve = Evolve::builder()
//         .with_genotype(genotype)
//         .with_population_size(100)        // evolve with 100 chromosomes
//         .with_target_fitness_score(60)   // goal is 100 times true in the best chromosome
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
    let x: [u8; 16] = [  5, 6,7,4,   9, 10,11, 8,   13, 14,15,12,    1, 2,3,0];
    let mut inv_x: [u8; 16] = [0; 16];

    for i in 0..16 {
        inv_x[x[i] as usize] = i as u8;
    }

    println!("{:?}", inv_x);
}
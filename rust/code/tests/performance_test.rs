// #![feature(portable_simd)]

// use tekton::imp::b128::Tekton128;
// use tekton::imp::b256::Tekton256;

// use std::time::{Instant};
// use rand::{Rng};

// use aes::{Aes128, Aes256};
// use aes::cipher::{
//     BlockEncrypt, BlockDecrypt, KeyInit,
//     generic_array::GenericArray,
// };



// #[test]
// fn test_tekton128_performance(){
//     let key: u128 = rand::thread_rng().gen();

//     let tekton = Tekton128::new(key.to_be_bytes());

//     let mut payload: [[u8; 16]; 1_000_000] = [[0; 16]; 1_000_000];

//     for i in 0..1_000_000 {
//         payload[i] = rand::thread_rng().gen::<u128>().to_be_bytes();
//     }

//     let mut enc: [[u8; 16]; 1_000_000] = [[0; 16]; 1_000_000];
//     let mut dec: [[u8; 16]; 1_000_000] = [[0; 16]; 1_000_000];


//     let start = Instant::now();
//     for i in 0..1_000_000 {
//         enc[i] = payload[i];
//         tekton.encrypt(&mut enc[i]);
//         dec[i] = enc[i];
//         tekton.decrypt(&mut dec[i]);
//     }
//     let duration = start.elapsed();

//     let a = dec[999999][0];
//     println!("{0}", a);
//     println!("Tekton 128bit: 1M nonces: {0:?}", duration);

// }

// #[test]
// fn test_aes128_performance(){
//     let key: u128 = rand::thread_rng().gen();

//     let kb = GenericArray::from(key.to_be_bytes());


//     let p: u128 = rand::thread_rng().gen();

//     let mut block = GenericArray::from(p.to_be_bytes());

//     let cipher = Aes128::new(&kb);


//     let start = Instant::now();
//     for _ in 0..1_000_000 {
//         cipher.encrypt_block(&mut block);
//         cipher.decrypt_block(&mut block);
//     }
//     let duration = start.elapsed();

//     println!("AES 128bit: 1M nonces: {0:?}", duration);

// }

// fn rand_u256() -> [u8; 32]{
//     let lo_a: u128 = rand::thread_rng().gen();
//     let hi_a: u128 = rand::thread_rng().gen();

//     let mut a: [u8; 32] = [0; 32];
//     a[..16].copy_from_slice(&lo_a.to_be_bytes());
//     a[16..32].copy_from_slice(&hi_a.to_be_bytes());

//     return a;
// }

// #[test]
// fn test_tekton256_performance(){
//     let key = rand_u256();

//     let tekton = Tekton256::new(key);

//     let mut payload: [[u8; 32]; 1_000_000] = [[0; 32]; 1_000_000];

//     for i in 0..1_000_000 {
//         payload[i] = rand_u256();
//     }

//     let mut enc: [[u8; 32]; 1_000_000] = [[0; 32]; 1_000_000];
//     let mut dec: [[u8; 32]; 1_000_000] = [[0; 32]; 1_000_000];


//     let start = Instant::now();
//     for i in 0..1_000_000 {
//         enc[i] = payload[i];
//         tekton.encrypt(&mut enc[i]);
//         dec[i] = enc[i];
//         tekton.decrypt(&mut dec[i]);
//     }
//     let duration = start.elapsed();

//     let a = dec[999999][0];
//     println!("{0}", a);
//     println!("Tekton 256bit: 1M nonces: {0:?}", duration);

// }

// #[test]
// fn test_aes256_performance(){
//     let key = rand_u256();

//     let kb = GenericArray::from(key);


//     let p: u128 = rand::thread_rng().gen();

//     let mut block = GenericArray::from(p.to_be_bytes());

//     let cipher = Aes256::new(&kb);


//     let start = Instant::now();
//     for _ in 0..1_000_000 {
//         cipher.encrypt_block(&mut block);
//         cipher.decrypt_block(&mut block);
//     }
//     let duration = start.elapsed();

//     println!("AES 256bit: 1M nonces: {0:?}", duration);

// }

// // #[test]
// // fn generate_permutation(){
// //     let mut a: [u8; 32] = [0; 32];
// //     let mut invA: [u8; 32] = [0; 32];

// //     for i in 0..32 {
// //         a[i] = i as u8;
// //     }

// //     for i in (0..32).rev() {
// //         let idx: usize = rand::thread_rng().gen::<usize>() % (i+1);
// //         (a[i], a[idx]) = (a[idx], a[i]);
// //     }

// //     print!("{:?}", a);

// //     for i in 0..32 {
// //         invA[a[i] as usize] = i as u8;
// //     }

// //     print!("{:?}", invA);
// // }
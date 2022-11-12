#![feature(portable_simd)]

use tekton::imp::b128::Tekton128;
use tekton::imp::b256::Tekton256;

use std::time::{Instant};
use rand::{Rng};


use aes::{Aes128};
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

use tekton::imp::{Flags, Rounds};

fn rand_u256() -> [u8; 32]{
    let lo_a: u128 = rand::thread_rng().gen();
    let hi_a: u128 = rand::thread_rng().gen();

    let mut a: [u8; 32] = [0; 32];
    a[..16].copy_from_slice(&lo_a.to_be_bytes());
    a[16..32].copy_from_slice(&hi_a.to_be_bytes());

    return a;
}


#[test]
fn test_compare_perfomances_128(){
    let key: u128 = rand::thread_rng().gen();

    let mut payload: [u8; 16] = [0; 16];

    payload = rand::thread_rng().gen::<u128>().to_be_bytes();
    

    let mut enc: [u8; 16] = [0; 16];

    let kb = GenericArray::from(key.to_be_bytes());


    let p: u128 = rand::thread_rng().gen();

    let mut block = GenericArray::from(p.to_be_bytes());

    let cipher = Aes128::new(&kb);

    let mut work_t = std::hint::black_box(|tekton: Tekton128| {
        for _ in 0..1_000_000 {
            enc = payload;
            tekton.encrypt(&mut enc);
            tekton.decrypt(&mut enc);
        }
    });

    let mut work_a = || {
        for _ in 0..1_000_000 {
            cipher.encrypt_block(&mut block);
            cipher.decrypt_block(&mut block);
        }
    };

    let tekton_bp = Tekton128::new(key.to_be_bytes(),
        Flags { rounds: Rounds::FASTER });

    let start = Instant::now();
    work_t(tekton_bp);
    let duration = start.elapsed();

    println!("Tekton (128bit)(3x): 1M nonces: {0:?}", duration);

    let tekton_br = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER });

    let start = Instant::now();
    work_t(tekton_br);
    let duration = start.elapsed();

    println!("Tekton (128bit)(5x): 1M nonces: {0:?}", duration);

    let start = Instant::now();
    work_a();
    let duration = start.elapsed();

    println!("AES (128bit): 1M nonces: {0:?}", duration);



    let key = rand_u256();

    let mut payload: [u8; 16] = [0; 16];

    payload = rand::thread_rng().gen::<u128>().to_be_bytes();
    

    let mut enc: [u8; 16] = [0; 16];

    let mut work_t = std::hint::black_box(|tekton: Tekton256| {
        for _ in 0..1_000_000 {
            enc = payload;
            tekton.encrypt(&mut enc);
            tekton.decrypt(&mut enc);
        }
    });

    let tekton_bp = Tekton256::new(key,
        Flags { rounds: Rounds::FASTER });

    let start = Instant::now();
    work_t(tekton_bp);
    let duration = start.elapsed();

    println!("Tekton (256bit)(5x): 1M nonces: {0:?}", duration);

    let tekton_br = Tekton256::new(key,
    Flags { rounds: Rounds::SAFER});

    let start = Instant::now();
    work_t(tekton_br);
    let duration = start.elapsed();

    println!("Tekton (256bit)(8x): 1M nonces: {0:?}", duration);
}
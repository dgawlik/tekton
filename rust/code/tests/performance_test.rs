#![feature(portable_simd)]

use tekton::imp::b128::Tekton128;

use std::time::{Instant};
use rand::{Rng};

use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};



#[test]
fn test_tekton128_performance(){
    let key: u128 = rand::thread_rng().gen();

    let tekton = Tekton128::new(key.to_be_bytes());

    let mut payload: [[u8; 16]; 1_000_000] = [[0; 16]; 1_000_000];

    for i in 0..1_000_000 {
        payload[i] = rand::thread_rng().gen::<u128>().to_be_bytes();
    }

    let mut enc: [[u8; 16]; 1_000_000] = [[0; 16]; 1_000_000];
    let mut dec: [[u8; 16]; 1_000_000] = [[0; 16]; 1_000_000];


    let start = Instant::now();
    for i in 0..1_000_000 {
        enc[i] = payload[i];
        tekton.encrypt(&mut enc[i]);
        dec[i] = enc[i];
        tekton.decrypt(&mut dec[i]);
    }
    let duration = start.elapsed();

    let a = dec[999999][0];
    println!("{0}", a);
    println!("Tekton 128bit: 1M nonces: {0:?}", duration);

}

#[test]
fn test_aes128_performance(){
    let key: u128 = rand::thread_rng().gen();

    let kb = GenericArray::from(key.to_be_bytes());


    let p: u128 = rand::thread_rng().gen();

    let mut block = GenericArray::from(p.to_be_bytes());

    let cipher = Aes128::new(&kb);


    let start = Instant::now();
    for _ in 0..1_000_000 {
        cipher.encrypt_block(&mut block);
        cipher.decrypt_block(&mut block);
    }
    let duration = start.elapsed();

    println!("AES 128bit: 1M nonces: {0:?}", duration);

}
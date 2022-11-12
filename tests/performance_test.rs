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

use tekton::imp::{Flags, Mode, Rounds};

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

    let mut payload: [[u8; 16]; 100_000] = [[0; 16]; 100_000];

    for i in 0..100_000 {
        payload[i] = rand::thread_rng().gen::<u128>().to_be_bytes();
    }

    let mut enc: [[u8; 16]; 100_000] = [[0; 16]; 100_000];
    let mut dec: [[u8; 16]; 100_000] = [[0; 16]; 100_000];


    let kb = GenericArray::from(key.to_be_bytes());


    let p: u128 = rand::thread_rng().gen();

    let mut block = GenericArray::from(p.to_be_bytes());

    let cipher = Aes128::new(&kb);

    let mut work_t = |tekton: Tekton128| {
        for i in 0..100_000 {
            enc[i] = payload[i];
            tekton.encrypt(&mut enc[i]);
            dec[i] = enc[i];
            tekton.decrypt(&mut dec[i]);
        }
    };

    let mut work_a = || {
        for _ in 0..100_000 {
            cipher.encrypt_block(&mut block);
            cipher.decrypt_block(&mut block);
        }
    };

    let tekton_bp = Tekton128::new(key.to_be_bytes(),
        Flags { rounds: Rounds::FASTER, mode: Mode::BYTE });

    let start = Instant::now();
    work_t(tekton_bp);
    let duration = start.elapsed();

    println!("Tekton (128bit)(faster, byte): 100K nonces: {0:?}", duration);

    let tekton_br = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER, mode: Mode::BYTE });

    let start = Instant::now();
    work_t(tekton_br);
    let duration = start.elapsed();

    println!("Tekton (128bit)(safer, byte): 100K nonces: {0:?}", duration);

    let tekton_ip = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::FASTER, mode: Mode::INT });

    let start = Instant::now();
    work_t(tekton_ip);
    let duration = start.elapsed();

    println!("Tekton (128bit)(faster, int): 100K nonces: {0:?}", duration);

    let tekton_ir = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER, mode: Mode::INT });

    let start = Instant::now();
    work_t(tekton_ir);
    let duration = start.elapsed();

    println!("Tekton (128bit)(safer, int): 100K nonces: {0:?}", duration);


    let start = Instant::now();
    work_a();
    let duration = start.elapsed();

    println!("AES (128bit): 100K nonces: {0:?}", duration);

    let a = dec[99999][0];
    println!("{0}", a);


    let key = rand_u256();

    let mut payload: [[u8; 16]; 100_000] = [[0; 16]; 100_000];

    for i in 0..100_000 {
        payload[i].copy_from_slice(&rand_u256()[0..16]);
    }

    let mut enc: [[u8; 16]; 100_000] = [[0; 16]; 100_000];
    let mut dec: [[u8; 16]; 100_000] = [[0; 16]; 100_000];

    let mut work_t = |tekton: Tekton256| {
        for i in 0..100_000 {
            enc[i] = payload[i];
            tekton.encrypt(&mut enc[i]);
            dec[i] = enc[i];
            tekton.decrypt(&mut dec[i]);
        }
    };

    let tekton_bp = Tekton256::new(key,
        Flags { rounds: Rounds::FASTER, mode: Mode::BYTE });

    let start = Instant::now();
    work_t(tekton_bp);
    let duration = start.elapsed();

    println!("Tekton (256bit)(faster, byte): 100K nonces: {0:?}", duration);

    let tekton_br = Tekton256::new(key,
    Flags { rounds: Rounds::SAFER, mode: Mode::BYTE });

    let start = Instant::now();
    work_t(tekton_br);
    let duration = start.elapsed();

    println!("Tekton (256bit)(safer, byte): 100K nonces: {0:?}", duration);

    let tekton_ip = Tekton256::new(key,
    Flags { rounds: Rounds::FASTER, mode: Mode::INT });

    let start = Instant::now();
    work_t(tekton_ip);
    let duration = start.elapsed();

    println!("Tekton (256bit)(faster, int): 100K nonces: {0:?}", duration);

    let tekton_ir = Tekton256::new(key,
    Flags { rounds: Rounds::SAFER, mode: Mode::INT });

    let start = Instant::now();
    work_t(tekton_ir);
    let duration = start.elapsed();

    println!("Tekton (256bit)(safer, int): 100K nonces: {0:?}", duration);


    let a = dec[99999][0];
    println!("{0}", a);
    
}
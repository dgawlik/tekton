use tekton::imp::b128::Tekton128;
use tekton::imp::b256::Tekton256;

use std::time::{Instant};
use rand::{Rng};
use rand::distributions::{Distribution,  Standard};
use tekton::imp::util::{Histogram};

use aes::{Aes128};
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

use tekton::imp::{Flags, Mode, Permute};



fn rand_u256() -> [u8; 32]{
    let mut rng = rand::thread_rng();
    let lo_a: f64 = Standard.sample(&mut rng);
    let hi_a: f64 = Standard.sample(&mut rng);

    let mut a: [u8; 32] = [0; 32];
    a[..16].copy_from_slice(&(lo_a as u128 * u128::MAX).to_be_bytes());
    a[16..32].copy_from_slice(&(hi_a as u128 * u128::MAX).to_be_bytes());

    return a;
}



#[test]
fn test_compare_statistics_128(){
    let key: u128 = rand::thread_rng().gen();

    let mut rng = rand::thread_rng();

    let mut payload: [[u8; 16]; 100_000] = [[0; 16]; 100_000];

    for i in 0..100_000 {
        let v: f64 = Standard.sample(&mut rng);
        payload[i] = ((v as u128) * u128::MAX).to_le_bytes();
    }

    let mut enc: [[u8; 16]; 100_000] = [[0; 16]; 100_000];


    let kb = GenericArray::from(key.to_be_bytes());


    let v: f64 = Standard.sample(&mut rng);
    let p = ((v as u128) * u128::MAX).to_le_bytes();

    let mut block = GenericArray::from(p);

    let cipher = Aes128::new(&kb);

    let mut work_t = |tekton: Tekton128| {
        let mut hist = Histogram::<1000>::new();

        for i in 0..100_000 {
            enc[i] = payload[i];
            tekton.encrypt(&mut enc[i]);
            hist.update(enc[i]);
        }

        hist.uniformness()
    };

    let mut work_a = || {
        let mut hist = Histogram::<1000>::new();
        for _ in 0..100_000 {
            cipher.encrypt_block(&mut block);
            hist.update(block.into())
        }

        hist.uniformness()
    };

    let tekton_bp = Tekton128::new(key.to_be_bytes(),
        Flags { permute: Permute::PERMUTE, mode: Mode::BYTE });

    let u = work_t(tekton_bp);

    println!("Tekton (128bit)(perm, byte) uniformness: {0:?}", u);

    let tekton_br = Tekton128::new(key.to_be_bytes(),
    Flags { permute: Permute::ROTATE, mode: Mode::BYTE });

    let u = work_t(tekton_br);

    println!("Tekton (128bit)(rot, byte) uniformness: {0:?}", u);

    let tekton_ip = Tekton128::new(key.to_be_bytes(),
    Flags { permute: Permute::PERMUTE, mode: Mode::INT });

    let u = work_t(tekton_ip);

    println!("Tekton (128bit)(perm, int) uniformness: {0:?}", u);

    let tekton_ir = Tekton128::new(key.to_be_bytes(),
    Flags { permute: Permute::ROTATE, mode: Mode::INT });

    let u = work_t(tekton_ir);

    println!("Tekton (128bit)(rot, int) uniformness: {0:?}", u);

    let  u = work_a();

    println!("AES (128bit) uniformness: {0:?}", u);


    let key = rand_u256();


    let mut enc: [[u8; 16]; 100_000] = [[0; 16]; 100_000];

    let mut work_t = |tekton: Tekton256| {
        let mut hist = Histogram::<1000>::new();

        for i in 0..100_000 {
            enc[i] = payload[i];
            tekton.encrypt(&mut enc[i]);
            hist.update(enc[i]);
        }

        hist.uniformness()
    };

    let tekton_bp = Tekton256::new(key,
        Flags { permute: Permute::PERMUTE, mode: Mode::BYTE });

    let u = work_t(tekton_bp);

    println!("Tekton (256bit)(perm, byte) uniformness: {0:?}", u);

    let tekton_br = Tekton256::new(key,
    Flags { permute: Permute::ROTATE, mode: Mode::BYTE });

    let u = work_t(tekton_br);

    println!("Tekton (256bit)(rot, byte) uniformness: {0:?}", u);

    let tekton_ip = Tekton256::new(key,
    Flags { permute: Permute::PERMUTE, mode: Mode::INT });

    let u = work_t(tekton_ip);

    println!("Tekton (256bit)(perm, int) unifomness: {0:?}", u);

    let tekton_ir = Tekton256::new(key,
    Flags { permute: Permute::ROTATE, mode: Mode::INT });

    let u = work_t(tekton_ir);

    println!("Tekton (256bit)(rot, int) uniformness: {0:?}", u);
    
}
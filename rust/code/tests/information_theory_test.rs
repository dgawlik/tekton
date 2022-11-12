use tekton::imp::b128::Tekton128;
use bitreader::BitReader;

use rand::{Rng};
use tekton::imp::util::{Histogram};
use rand_distr::{Normal, Distribution};

use aes::{Aes128};
use aes::cipher::{
    BlockEncrypt, KeyInit,
    generic_array::GenericArray,
};

use tekton::imp::{Flags, Mode, Rounds};





#[test]
fn test_compare_statistics_128(){
    let key: u128 = rand::thread_rng().gen();

    let mut rng = rand::thread_rng();
    let normal = Normal::new(u128::MAX as f64, (u128::MAX as f64)/100 as f64).unwrap();

    let mut payload: [[u8; 16]; 100_000] = [[0; 16]; 100_000];

    for i in 0..100_000 {
        let v: f64 = normal.sample(&mut rng);
        payload[i] = (v as u128).to_le_bytes();
    }

    let mut enc: [[u8; 16]; 100_000] = [[0; 16]; 100_000];


    let kb = GenericArray::from(key.to_be_bytes());


   

    let cipher = Aes128::new(&kb);

    let mut uniformness_t = |tekton: Tekton128| {
        let mut hist = Histogram::<1000>::new();

        for i in 0..100_000 {
            enc[i] = payload[i];
            tekton.encrypt(&mut enc[i]);
            hist.update(enc[i]);
        }

        hist.uniformness()
    };

    let confusion_t = |tekton: Tekton128| {
        

        let mut c: [f64; 100] = [0.0; 100];

        for j in 0..100 {
            let mut rng = rand::thread_rng();
            let _p: u128 = rng.gen();
            let p = _p.to_be_bytes();

            let mut conf: [f64; 128] = [0.0; 128];
            for i in 0..128 {
                let mut p0 = p.clone();
                p0[i/8] = p0[i/8] & !(1 << (i%8));

                let mut enc_p0 = p0.clone();
                tekton.encrypt(&mut enc_p0);

                let mut p1 = p.clone();
                p1[i/8] = p1[i/8] | (1 << (i%8));

                let mut enc_p1 = p1.clone();
                tekton.encrypt(&mut enc_p1);

                let mut rdr0 = BitReader::new(&enc_p0);
                let mut rdr1 = BitReader::new(&enc_p1);

                let mut different = 0;
                for _ in 0..128 {
                    let b0 = rdr0.read_bool().unwrap();
                    let b1 = rdr1.read_bool().unwrap();

                    if b0 != b1 {
                        different += 1;
                    }
                }
                conf[i] = different as f64;
            }

            let conf: f64 = conf.into_iter().sum();
            c[j] = conf/128.0;
        }

        let avg: f64 = c.into_iter().sum();
        return avg/100.0;
    };

    let mut uniformness_a = || {
        let mut hist = Histogram::<1000>::new();
        for _ in 0..100_000 {
            let v: f64 = normal.sample(&mut rng);
            let p = (v as u128).to_le_bytes();
        
            let mut block = GenericArray::from(p);
            
            cipher.encrypt_block(&mut block);
            hist.update(block.into())
        }

        hist.uniformness()
    };

    let confusion_a = || {
        let mut rng = rand::thread_rng();
        let _p: u128 = rng.gen();
        let p = _p.to_be_bytes();

        let mut conf: [f64; 128] = [0.0; 128];
        for i in 0..128 {
            let mut p0 = p.clone();
            p0[i/8] = p0[i/8] & !(1 << (i%8));

            let mut enc_p0 = GenericArray::from(p0);
            cipher.encrypt_block(&mut enc_p0);

            let mut p1 = p.clone();
            p1[i/8] = p1[i/8] | (1 << (i%8));

            let mut enc_p1 =  GenericArray::from(p1);
            cipher.encrypt_block(&mut enc_p1);

            let mut rdr0 = BitReader::new(&enc_p0);
            let mut rdr1 = BitReader::new(&enc_p1);

            let mut different = 0;
            for _ in 0..128 {
                let b0 = rdr0.read_bool();
                let b1 = rdr1.read_bool();

                if b0 != b1 {
                    different += 1;
                }
            }
            conf[i] = different as f64;
        }

        let conf: f64 = conf.into_iter().sum();
        return conf/128.0;
    };

    let tekton_bp = Tekton128::new(key.to_be_bytes(),
        Flags { rounds: Rounds::FASTER, mode: Mode::BYTE });

    let u = uniformness_t(tekton_bp);

    println!("Tekton (128bit)(faster, byte) uniformness: {0:?}", u);

    let tekton_br = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER, mode: Mode::BYTE });

    let u = uniformness_t(tekton_br);

    println!("Tekton (128bit)(safer, byte) uniformness: {0:?}", u);

    let tekton_ip = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::FASTER, mode: Mode::INT });

    let u = uniformness_t(tekton_ip);

    println!("Tekton (128bit)(faster, int) uniformness: {0:?}", u);

    let tekton_ir = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER, mode: Mode::INT });

    let u = uniformness_t(tekton_ir);

    println!("Tekton (128bit)(safer, int) uniformness: {0:?}", u);

    let  u = uniformness_a();

    println!("AES (128bit) uniformness: {0:?}", u);


    println!("--------------");

    let tekton_bp = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::FASTER, mode: Mode::BYTE });

    let u = confusion_t(tekton_bp);

    println!("Tekton (128bit)(faster, byte) confusion: {0:?}", u);

    let tekton_br = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER, mode: Mode::BYTE });

    let u = confusion_t(tekton_br);

    println!("Tekton (128bit)(safer, byte) confusion: {0:?}", u);

    let tekton_ip = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::FASTER, mode: Mode::INT });

    let u = confusion_t(tekton_ip);

    println!("Tekton (128bit)(faster, int) confusion: {0:?}", u);

    let tekton_ir = Tekton128::new(key.to_be_bytes(),
    Flags { rounds: Rounds::SAFER, mode: Mode::INT });

    let u = confusion_t(tekton_ir);

    println!("Tekton (128bit)(safer, int) confusion: {0:?}", u);

    let  u = confusion_a();

    println!("AES (128bit) confusion: {0:?}", u);
    
}
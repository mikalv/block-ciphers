#![no_std]
#![cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#![feature(test)]
extern crate aesni;
extern crate test;

use aesni::{Aes128, BlockCipher};

#[bench]
pub fn aes128_encrypt(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.encrypt_block(&mut input);
        test::black_box(&input);
    });
    bh.bytes = input.len() as u64;
}

#[bench]
pub fn aes128_decrypt(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.decrypt_block(&mut input);
        test::black_box(&input);
    });
    bh.bytes = input.len() as u64;
}

#[bench]
pub fn aes128_encrypt8(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.encrypt_blocks(&mut input);
        test::black_box(&input);
    });
    bh.bytes = (input[0].len()*input.len()) as u64;
}

#[bench]
pub fn aes128_decrypt8(bh: &mut test::Bencher) {
    let cipher = Aes128::new(&Default::default());
    let mut input = Default::default();

    bh.iter(|| {
        cipher.decrypt_blocks(&mut input);
        test::black_box(&input);
    });
    bh.bytes = (input[0].len()*input.len()) as u64;
}

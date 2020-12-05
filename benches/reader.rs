#![feature(test)]

extern crate test;

use dmxparser::{from_reader, from_slice};
use test::Bencher;

static DATA: &'static [u8] = include_bytes!("../tests/fixtures/test.vmap");

#[bench]
fn bench_slice(b: &mut Bencher) {
    b.iter(|| from_slice(DATA));
}

#[bench]
fn bench_reader(b: &mut Bencher) {
    b.iter(|| from_reader(DATA));
}

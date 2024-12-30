#![feature(test)]

extern crate test;

use logcosh::{clogcosh, ctrivial_logcosh, logcosh, trivial_logcosh};
use num::complex::Complex;
use test::{black_box, Bencher};

#[bench]
fn bench_logcosh_low(b: &mut Bencher) {
    let x = 1e-5;
    b.iter(|| black_box(logcosh(x)));
}

#[bench]
fn bench_logcosh_mid(b: &mut Bencher) {
    let x = 1.0;
    b.iter(|| black_box(logcosh(x)));
}

#[bench]
fn bench_logcosh_high(b: &mut Bencher) {
    let x = 1e3;
    b.iter(|| black_box(logcosh(x)));
}

#[bench]
fn bench_logcosh_trivial_low(b: &mut Bencher) {
    let x = 1e-5;
    b.iter(|| black_box(trivial_logcosh(x)));
}

#[bench]
fn bench_logcosh_trivial_mid(b: &mut Bencher) {
    let x = 1.0;
    b.iter(|| black_box(trivial_logcosh(x)));
}

#[bench]
fn bench_logcosh_trivial_high(b: &mut Bencher) {
    let x = 1e3;
    b.iter(|| black_box(trivial_logcosh(x)));
}

#[bench]
fn bench_clogcosh_low(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e-5, 2e-6);
    b.iter(|| black_box(clogcosh(x)));
}

#[bench]
fn bench_clogcosh_mid(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1.0, -1.0);
    b.iter(|| black_box(clogcosh(x)));
}

#[bench]
fn bench_clogcosh_high(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e3, 1e2);
    b.iter(|| black_box(clogcosh(x)));
}
#[bench]
fn bench_clogcosh_trivial_low(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e-5, 2e-6);
    b.iter(|| black_box(ctrivial_logcosh(x)));
}

#[bench]
fn bench_clogcosh_trivial_mid(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1.0, -1.0);
    b.iter(|| black_box(ctrivial_logcosh(x)));
}

#[bench]
fn bench_clogcosh_trivial_high(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e3, 1e2);
    b.iter(|| black_box(ctrivial_logcosh(x)));
}

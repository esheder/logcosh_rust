#![feature(test)]

extern crate test;

use logcosh::{
    approx_clogcosh, approx_logcosh, approx_logcosh_c32, clogcosh, logcosh, logcosh_c32,
};
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
fn bench_logcosh_approx_low(b: &mut Bencher) {
    let x = 1e-5;
    b.iter(|| black_box(approx_logcosh(x)));
}

#[bench]
fn bench_logcosh_approx_mid(b: &mut Bencher) {
    let x = 1.0;
    b.iter(|| black_box(approx_logcosh(x)));
}

#[bench]
fn bench_logcosh_approx_high(b: &mut Bencher) {
    let x = 1e3;
    b.iter(|| black_box(approx_logcosh(x)));
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
fn bench_clogcosh_approx_low(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e-5, 2e-6);
    b.iter(|| black_box(approx_clogcosh(x)));
}

#[bench]
fn bench_clogcosh_approx_mid(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1.0, -1.0);
    b.iter(|| black_box(approx_clogcosh(x)));
}

#[bench]
fn bench_clogcosh_approx_high(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e3, 1e2);
    b.iter(|| black_box(approx_clogcosh(x)));
}

#[bench]
fn bench_logcosh_c32_low(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e-5, 2e-6);
    b.iter(|| black_box(logcosh_c32(x)));
}

#[bench]
fn bench_logcosh_c32_mid(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1.0, -1.0);
    b.iter(|| black_box(logcosh_c32(x)));
}

#[bench]
fn bench_logcosh_c32_high(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e3, 1e2);
    b.iter(|| black_box(logcosh_c32(x)));
}

#[bench]
fn bench_approx_logcosh_c32_low(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e-5, 2e-6);
    b.iter(|| black_box(approx_logcosh_c32(x)));
}

#[bench]
fn bench_approx_logcosh_c32_mid(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1.0, -1.0);
    b.iter(|| black_box(approx_logcosh_c32(x)));
}

#[bench]
fn bench_approx_logcosh_c32_high(b: &mut Bencher) {
    let x: Complex<f32> = Complex::new(1e3, 1e2);
    b.iter(|| black_box(approx_logcosh_c32(x)));
}

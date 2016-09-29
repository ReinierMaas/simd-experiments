#![feature(test)]

extern crate simd_library;
extern crate test;

const LENGTH: usize = 2048;
const A: &'static [u32] = &[5; LENGTH];
const B: &'static [u32] = &[6; LENGTH];

#[bench]
fn bench_normal(b: &mut test::Bencher) {
    let mut c: &mut [u32] = &mut [0; LENGTH];
    b.iter(|| {
        let mut c = test::black_box(&mut c);
        simd_library::prod_normal(A, B, LENGTH, &mut c);
        test::black_box(&c);
    });
}

#[bench]
fn bench_sse(b: &mut test::Bencher) {
    let mut c: &mut [u32] = &mut [0; LENGTH];
    b.iter(|| {
        let mut c = test::black_box(&mut c);
        simd_library::prod_sse(A, B, LENGTH, &mut c);
        test::black_box(&c);
    });
}

#[bench]
fn bench_avx(b: &mut test::Bencher) {
    let mut c: &mut [u32] = &mut [0; LENGTH];
    b.iter(|| {
        let mut c = test::black_box(&mut c);
        simd_library::prod_avx(A, B, LENGTH, &mut c);
        test::black_box(&c);
    });
}
#![feature(test)]

extern crate simd;
extern crate test;

use simd::u32x4;

const LENGTH: usize = 2048;
const A: &'static [u32] = &[5; LENGTH];
const B: &'static [u32] = &[6; LENGTH];

fn main() {
    let mut c = [0; LENGTH];

    let mut f = move || prod_sse(A, B, &mut c);
    f();

    let c2: &[u32] = &c;
    println!("{:?}", c2);
}

#[inline(never)]
fn prod_sse(a: &[u32], b: &[u32], out: &mut [u32]) {
    for i in 0..LENGTH / 4 {
        let a4 = u32x4::load(a, i * 4);
        let b4 = u32x4::load(b, i * 4);
        let c4 = a4 * b4;
        c4.store(out, i * 4);
    }
}

#[inline(never)]
fn prod_normal(a: &[u32], b: &[u32], out: &mut [u32]) {
    for i in 0..LENGTH {
        out[i] = a[i] * b[i];
    }
}



#[inline(never)]
fn prod_avx(a: &[u32], b: &[u32], out: &mut [u32]) {

}

#[bench]
fn bench_normal(b: &mut test::Bencher) {
    let mut c = [0; LENGTH];
    b.iter(|| {
        prod_normal(A, B, &mut c);
        test::black_box(&c);
    });
}

#[bench]
fn bench_sse(b: &mut test::Bencher) {
    let mut c = [0; LENGTH];
    b.iter(|| {
        prod_sse(A, B, &mut c);
        test::black_box(&c);
    });
}

//#[bench]
fn bench_avx(b: &mut test::Bencher) {
    let c = &mut [0; LENGTH];
    b.iter(|| {
        prod_avx(A, B, c);
        test::black_box(&c);
    });
}

extern crate simd;

use simd::u32x4;
use simd::x86::avx::u32x8;

pub fn prod_normal(a: &[u32], b: &[u32], len: usize, out: &mut [u32]) {
    for i in 0..len {
        out[i] = a[i] * b[i];
    }
}

pub fn prod_sse(a: &[u32], b: &[u32], len: usize, out: &mut [u32]) {
    for i in 0..len / 4 {
        let a4 = u32x4::load(a, i * 4);
        let b4 = u32x4::load(b, i * 4);
        let c4 = a4 * b4;
        c4.store(out, i * 4);
    }
}

pub fn prod_avx(a: &[u32], b: &[u32], len: usize, out: &mut [u32]) {
    for i in 0..len / 8 {
        let a8 = u32x8::load(a, i * 8);
        let b8 = u32x8::load(b, i * 8);
        let c8 = a8 * b8;
        c8.store(out, i * 8);
    }
}

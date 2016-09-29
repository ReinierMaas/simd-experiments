extern crate simd_library;

const LENGTH: usize = 2048;
const A: &'static [u32] = &[5; LENGTH];
const B: &'static [u32] = &[6; LENGTH];

fn main() {
    let mut c = [0; LENGTH];

    simd_library::prod_normal(A, B, LENGTH, &mut c);
    simd_library::prod_sse(A, B, LENGTH, &mut c);
    simd_library::prod_avx(A, B, LENGTH, &mut c);

    println!("{:?}", c.len());
}

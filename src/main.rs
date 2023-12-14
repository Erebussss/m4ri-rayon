#![feature(portable_simd)]
use m4ri_rayon::binary_matrix::BinaryMatrix;
use std::simd::{num::SimdUint, *};
fn main() {
    let si: u8x16 = u8x16::splat(1);
    println!("{}");
}

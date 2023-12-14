#![feature(portable_simd)]
use m4ri_rayon::binary_matrix::BinaryMatrix;
use std::simd::*;
fn main() {
    let test = BinaryMatrix::<u8x32>::new(2, 36);
    println!("{test:#?}");
}

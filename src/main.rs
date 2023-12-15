use rand::Rng;
// #![feature(portable_simd)]
// use m4ri_rayon::binary_matrix::BinaryMatrix;
// use std::simd::*;
fn main() {
    let mut rng = rand::thread_rng();
    let mut vec1: Vec<u32> = (0..100000).map(|_| rng.gen()).collect();
    let vec2: Vec<u32> = (0..100000).map(|_| rng.gen()).collect();

    for i in 0..vec1.len() {
        vec1[i] ^= vec2[i];
    }
    let mut res: u32 = 0;
    for i in vec1 {
        res ^= i;
    }
    println!("{}", res);
}

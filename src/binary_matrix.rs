use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::{
    simd::num::SimdUint,
    simd::SimdElement,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct BinaryMatrix<T>
where
    T: SimdElement + SimdUint + Default,
{
    pub nrows: usize,
    pub ncols: usize,
    pub tbyte: usize,
    pub width: usize,
    pub data: Vec<Arc<RwLock<Vec<T>>>>,
}
impl<T> BinaryMatrix<T>
where
    T: SimdElement + SimdUint + Default,
{
    pub fn new(nrows: usize, ncols: usize) -> BinaryMatrix<T> {
        let tbyte = std::mem::size_of::<T>();
        let width = (ncols + tbyte * 8 - 1) / (tbyte * 8);
        let data = (0..nrows)
            .map(|_| Arc::new(RwLock::new(vec![T::default(); width])))
            .collect();
        BinaryMatrix {
            nrows,
            ncols,
            tbyte,
            width,
            data,
        }
    }
}

impl<T> BinaryMatrix<T>
where
    T: SimdElement + SimdUint + Default,
{
    pub fn rand(&mut self) {
        let mut rng = rand::thread_rng();
        for row in &mut self.data {
            let mut data = row.write().unwrap();
            for elem in &mut *data {
                for i in 0..T::lanes() {
                    elem[i] = rng.gen();
                }
            }
        }
    }
}

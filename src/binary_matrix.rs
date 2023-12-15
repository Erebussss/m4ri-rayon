use rand::Rng;
use rayon::prelude::*;
use std::{
    fmt::{self, Display},
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct BinaryMatrix {
    pub nrows: usize,
    pub ncols: usize,
    pub width: usize,
    pub data: Vec<Arc<RwLock<Vec<u64>>>>,
}

impl BinaryMatrix {
    pub fn new(nrows: usize, ncols: usize) -> BinaryMatrix {
        let width = (ncols + 63) / 64;
        let data = (0..nrows)
            .map(|_| Arc::new(RwLock::new(vec![0u64; width])))
            .collect();
        BinaryMatrix {
            nrows,
            ncols,
            width,
            data,
        }
    }

    pub fn rand(&mut self) {
        self.data.par_iter_mut().for_each(|row| {
            let mut data = row.write().unwrap();
            let mut rng = rand::thread_rng();
            for elem in data.iter_mut() {
                *elem = rng.gen();
            }
        });
    }
}

impl Display for BinaryMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            let data = row.read().unwrap();
            for elem in data.iter().take(data.len() - 1) {
                write!(f, "{:016x}", elem)?;
            }
            writeln!(
                f,
                "{:016x}",
                data.last().unwrap() & (!0u64 << (64 - ((self.ncols - 1) % 64 + 1)))
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rand() {
        let mut m = BinaryMatrix::new(10, 100);
        m.rand();
        println!("{}", m);
    }
}

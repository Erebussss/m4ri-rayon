use rand::Rng;
use rayon::prelude::*;
use std::{
    fmt::{self, Display},
    sync::{Arc, RwLock},
};

use super::BLOCK_SIZE;
use super::TABLE_NUM;

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
    fn make_table(&self, indices: &[usize; TABLE_NUM + 1]) -> Vec<BinaryMatrix> {
        (0..(indices.len() - 1))
            .map(|i| {
                let table_size = indices[i + 1] - indices[i];
                let row_base = indices[i];
                let tmp: BinaryMatrix = BinaryMatrix::new(1 << table_size, self.ncols);
                (0..(1 << table_size))
                    .into_par_iter()
                    .for_each(|table_row| {
                        let mut tmp_row = tmp.data[table_row].write().unwrap();
                        (0..table_size)
                            .filter(|&j| (table_row & (1 << (table_size - 1 - j))) != 0)
                            .for_each(|j| {
                                let src = self.data[row_base + j].read().unwrap();
                                for k in 0..self.width {
                                    tmp_row[k] ^= src[k];
                                }
                            });
                    });
                tmp
            })
            .collect()
    }
    pub fn binary_matrix_m4rm(&self, b: &BinaryMatrix) -> BinaryMatrix {
        assert_eq!(self.ncols, b.nrows);
        let result = BinaryMatrix::new(self.nrows, b.ncols);
        (0..b.nrows).step_by(BLOCK_SIZE).for_each(|block_row| {
            let row_count = BLOCK_SIZE.min(b.nrows - block_row);
            let mut indices = [row_count / TABLE_NUM; TABLE_NUM + 1];
            indices[0] = 0;
            (1..indices.len()).for_each(|i| {
                indices[i] += indices[i - 1];
                if i <= row_count % TABLE_NUM {
                    indices[i] += 1;
                }
            });
            assert_eq!(indices[TABLE_NUM], row_count);

            let table = b.make_table(&indices);
            let block = block_row / BLOCK_SIZE;

            (0..self.nrows).into_par_iter().for_each(|r| {
                let mut dst_row = result.data[r].write().unwrap();
                let idx = self.data[r].read().unwrap()[block] as usize;
                (0..(indices.len() - 1)).for_each(|i| {
                    let src_row =
                        (idx << indices[i]) >> (BLOCK_SIZE - (indices[i + 1] - indices[i]));
                    let src_row = table[i].data[src_row].read().unwrap();
                    (0..result.width).for_each(|j| {
                        dst_row[j] ^= src_row[j];
                    });
                });
            });
        });
        result
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
    impl BinaryMatrix {
        fn make_table_single(&self, indices: &[usize; TABLE_NUM + 1]) -> Vec<BinaryMatrix> {
            (0..(indices.len() - 1))
                .map(|i| {
                    let table_size = indices[i + 1] - indices[i];
                    let row_base = indices[i];
                    let tmp: BinaryMatrix = BinaryMatrix::new(1 << table_size, self.ncols);
                    (0..(1 << table_size)).into_iter().for_each(|table_row| {
                        let mut tmp_row = tmp.data[table_row].write().unwrap();
                        (0..table_size)
                            .filter(|&j| (table_row & (1 << (table_size - 1 - j))) != 0)
                            .for_each(|j| {
                                let src = self.data[row_base + j].read().unwrap();
                                for k in 0..self.width {
                                    tmp_row[k] ^= src[k];
                                }
                            });
                    });
                    tmp
                })
                .collect()
        }
        pub fn binary_matrix_m4rm_single(&self, b: &BinaryMatrix) -> BinaryMatrix {
            assert_eq!(self.ncols, b.nrows);
            let result = BinaryMatrix::new(self.nrows, b.ncols);
            (0..b.nrows).step_by(BLOCK_SIZE).for_each(|block_row| {
                let row_count = BLOCK_SIZE.min(b.nrows - block_row);
                let mut indices = [row_count / TABLE_NUM; TABLE_NUM + 1];
                indices[0] = 0;
                (1..indices.len()).for_each(|i| {
                    indices[i] += indices[i - 1];
                    if i <= row_count % TABLE_NUM {
                        indices[i] += 1;
                    }
                });
                assert_eq!(indices[TABLE_NUM], row_count);

                let table = b.make_table_single(&indices);
                let block = block_row / BLOCK_SIZE;

                (0..self.nrows).into_iter().for_each(|r| {
                    let mut dst_row = result.data[r].write().unwrap();
                    let idx = self.data[r].read().unwrap()[block] as usize;
                    (0..(indices.len() - 1)).for_each(|i| {
                        let src_row =
                            (idx << indices[i]) >> (BLOCK_SIZE - (indices[i + 1] - indices[i]));
                        let src_row = table[i].data[src_row].read().unwrap();
                        (0..result.width).for_each(|j| {
                            dst_row[j] ^= src_row[j];
                        });
                    });
                });
            });
            result
        }
    }
    #[test]
    fn test_rand() {
        let mut m = BinaryMatrix::new(10, 100);
        m.rand();
        println!("{}", m);
    }
    #[test]
    fn test_make_table() {
        let mut binary_matrix = BinaryMatrix::new(8, 8);
        binary_matrix.rand();
        println!("{}", binary_matrix);
        let indices = [0, 1, 3, 6, 8, 8, 8, 8, 8];
        let result = binary_matrix.make_table(&indices);
        println!("tablesize:{}", result.len());
        for i in result {
            println!("{}", i);
        }
    }
    #[test]
    fn test_m4rm() {
        let size = 128;
        let mut binary_matrix1 = BinaryMatrix::new(size, size);
        binary_matrix1.rand();
        println!("{}", binary_matrix1);
        let mut binary_matrix2 = BinaryMatrix::new(size, size);
        binary_matrix2.rand();
        println!("{}", binary_matrix2);
        let result = binary_matrix1.binary_matrix_m4rm(&binary_matrix2);
        println!("{}", result);
    }
}

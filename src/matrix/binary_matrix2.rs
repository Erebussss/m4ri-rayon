use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    fs::File,
    io::prelude::*,
};

use super::BLOCK_SIZE;
use super::TABLE_NUM;
#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryMatrix2 {
    pub nrows: usize,
    pub ncols: usize,
    pub width: usize,
    pub data: Vec<u64>,
}

impl BinaryMatrix2 {
    pub fn new(nrows: usize, ncols: usize) -> BinaryMatrix2 {
        let width = (ncols + 63) / 64;
        let data = vec![0u64; nrows * width];
        BinaryMatrix2 {
            nrows,
            ncols,
            width,
            data,
        }
    }
    pub fn from(nrows: usize, ncols: usize, data: Vec<u64>) -> BinaryMatrix2 {
        let width = (ncols + 63) / 64;
        assert_eq!(data.len(), nrows * width);
        BinaryMatrix2 {
            nrows,
            ncols,
            width,
            data,
        }
    }
    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }
    pub fn read_from_file(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let matrix: BinaryMatrix2 = bincode::deserialize(&buffer).unwrap();
        Ok(matrix)
    }
    pub fn rand(&mut self) {
        self.data.par_chunks_exact_mut(self.width).for_each(|row| {
            let mut rng = rand::thread_rng();
            for elem in row.iter_mut() {
                *elem = rng.gen();
            }
        });
    }
    fn make_table(&self, indices: &[usize; TABLE_NUM + 1]) -> Vec<BinaryMatrix2> {
        (0..(indices.len() - 1))
            .map(|i| {
                let table_size = indices[i + 1] - indices[i];
                let row_base = indices[i];
                let mut tmp: BinaryMatrix2 = BinaryMatrix2::new(1 << table_size, self.ncols);
                tmp.data
                    .par_chunks_exact_mut(tmp.width)
                    .enumerate()
                    .for_each(|(table_row, tmp_row)| {
                        (0..table_size)
                            .filter(|&j| (table_row & (1 << (table_size - 1 - j))) != 0)
                            .for_each(|j| {
                                let src = &self.data
                                    [(row_base + j) * self.width..(row_base + j + 1) * self.width];
                                for k in 0..self.width {
                                    tmp_row[k] ^= src[k];
                                }
                            });
                    });
                tmp
            })
            .collect()
    }
    pub fn m4rm(&self, b: &BinaryMatrix2) -> BinaryMatrix2 {
        assert_eq!(self.ncols, b.nrows);
        let mut result = BinaryMatrix2::new(self.nrows, b.ncols);
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

            result
                .data
                .par_chunks_exact_mut(result.width)
                .enumerate()
                .for_each(|(r, dst_row)| {
                    let idx = &self.data[r * self.width + block];
                    (0..(indices.len() - 1)).for_each(|i| {
                        let src_row = ((idx << indices[i])
                            >> (BLOCK_SIZE - (indices[i + 1] - indices[i])))
                            as usize;
                        let src_row = &table[i].data
                            [src_row * table[i].width..(src_row + 1) * table[i].width];
                        (0..result.width).for_each(|j| {
                            dst_row[j] ^= src_row[j];
                        });
                    });
                });
        });
        result
    }
}

impl PartialEq for BinaryMatrix2 {
    fn eq(&self, other: &Self) -> bool {
        if self.nrows != other.nrows || self.ncols != other.ncols {
            return false;
        }
        self.data == other.data
    }
}

impl Display for BinaryMatrix2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.data.chunks(self.width) {
            for elem in row.iter().take(row.len() - 1) {
                write!(f, "{:016x}", elem)?;
            }
            writeln!(
                f,
                "{:016x}",
                row.last().unwrap() & (!0u64 << (64 - ((self.ncols - 1) % 64 + 1)))
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
        let mut m = BinaryMatrix2::new(10, 100);
        m.rand();
        println!("{}", m);
    }
    #[test]
    fn test_make_table() {
        let mut binary_matrix = BinaryMatrix2::new(8, 8);
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
        let mut binary_matrix1 = BinaryMatrix2::new(size, size);
        binary_matrix1.rand();
        println!("{}", binary_matrix1);
        let mut binary_matrix2 = BinaryMatrix2::new(size, size);
        binary_matrix2.rand();
        println!("{}", binary_matrix2);
        let result = binary_matrix1.m4rm(&binary_matrix2);
        println!("{}", result);
    }
    #[test]
    fn test_file() {
        let mut matrix = BinaryMatrix2::new(100, 100);
        matrix.rand();
        matrix.write_to_file("test.bin").unwrap();
        let matrix2 = BinaryMatrix2::read_from_file("test.bin").unwrap();
        assert_eq!(matrix, matrix2);
        std::fs::remove_file("test.bin").unwrap();
    }
}

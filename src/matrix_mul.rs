use rayon::iter::IndexedParallelIterator;
use rayon::prelude::*;

use crate::binary_matrix::BinaryMatrix;
use crate::binary_matrix::BinaryMatrix2;

const BLOCK_SIZE: usize = 64;
const TABLE_NUM: usize = 8;

// trait MatrixOperations {
//     fn make_table(&self, indices: &[usize; TABLE_NUM + 1]) -> Vec<Box<Self>>;
//     fn binary_matrix_m4rm(&self, b: &Self) -> Box<Self>;
// }

fn make_table(indices: &[usize; TABLE_NUM + 1], matrix: &BinaryMatrix) -> Vec<BinaryMatrix> {
    (0..(indices.len() - 1))
        .map(|i| {
            let table_size = indices[i + 1] - indices[i];
            let row_base = indices[i];
            let tmp: BinaryMatrix = BinaryMatrix::new(1 << table_size, matrix.ncols);
            (0..(1 << table_size))
                .into_par_iter()
                .for_each(|table_row| {
                    let mut tmp_row = tmp.data[table_row].write().unwrap();
                    (0..table_size)
                        .filter(|&j| (table_row & (1 << (table_size - 1 - j))) != 0)
                        .for_each(|j| {
                            let src = matrix.data[row_base + j].read().unwrap();
                            for k in 0..matrix.width {
                                tmp_row[k] ^= src[k];
                            }
                        });
                });
            tmp
        })
        .collect()
}
fn make_table_single(indices: &[usize; TABLE_NUM + 1], matrix: &BinaryMatrix) -> Vec<BinaryMatrix> {
    (0..(indices.len() - 1))
        .map(|i| {
            let table_size = indices[i + 1] - indices[i];
            let row_base = indices[i];
            let tmp: BinaryMatrix = BinaryMatrix::new(1 << table_size, matrix.ncols);
            (0..(1 << table_size)).into_iter().for_each(|table_row| {
                let mut tmp_row = tmp.data[table_row].write().unwrap();
                (0..table_size)
                    .filter(|&j| (table_row & (1 << (table_size - 1 - j))) != 0)
                    .for_each(|j| {
                        let src = matrix.data[row_base + j].read().unwrap();
                        for k in 0..matrix.width {
                            tmp_row[k] ^= src[k];
                        }
                    });
            });
            tmp
        })
        .collect()
}

fn make_table2(indices: &[usize; TABLE_NUM + 1], matrix: &BinaryMatrix2) -> Vec<BinaryMatrix2> {
    (0..(indices.len() - 1))
        .map(|i| {
            let table_size = indices[i + 1] - indices[i];
            let row_base = indices[i];
            let mut tmp: BinaryMatrix2 = BinaryMatrix2::new(1 << table_size, matrix.ncols);
            tmp.data
                .par_chunks_exact_mut(tmp.width)
                .enumerate()
                .for_each(|(table_row, tmp_row)| {
                    (0..table_size)
                        .filter(|&j| (table_row & (1 << (table_size - 1 - j))) != 0)
                        .for_each(|j| {
                            let src = &matrix.data
                                [(row_base + j) * matrix.width..(row_base + j + 1) * matrix.width];
                            for k in 0..matrix.width {
                                tmp_row[k] ^= src[k];
                            }
                        });
                });
            tmp
        })
        .collect()
}

pub fn binary_matrix_m4rm(a: &BinaryMatrix, b: &BinaryMatrix) -> BinaryMatrix {
    assert_eq!(a.ncols, b.nrows);
    let result = BinaryMatrix::new(a.nrows, b.ncols);
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

        let table = make_table(&indices, b);
        let block = block_row / BLOCK_SIZE;

        (0..a.nrows).into_par_iter().for_each(|r| {
            let mut dst_row = result.data[r].write().unwrap();
            let idx = a.data[r].read().unwrap()[block] as usize;
            (0..(indices.len() - 1)).for_each(|i| {
                let src_row = (idx << indices[i]) >> (BLOCK_SIZE - (indices[i + 1] - indices[i]));
                let src_row = table[i].data[src_row].read().unwrap();
                (0..result.width).for_each(|j| {
                    dst_row[j] ^= src_row[j];
                });
            });
        });
    });
    result
}
pub fn binary_matrix_m4rm_single(a: &BinaryMatrix, b: &BinaryMatrix) -> BinaryMatrix {
    assert_eq!(a.ncols, b.nrows);
    let result = BinaryMatrix::new(a.nrows, b.ncols);
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

        let table = make_table_single(&indices, b);
        let block = block_row / BLOCK_SIZE;

        (0..a.nrows).into_iter().for_each(|r| {
            let mut dst_row = result.data[r].write().unwrap();
            let idx = a.data[r].read().unwrap()[block] as usize;
            (0..(indices.len() - 1)).for_each(|i| {
                let src_row = (idx << indices[i]) >> (BLOCK_SIZE - (indices[i + 1] - indices[i]));
                let src_row = table[i].data[src_row].read().unwrap();
                (0..result.width).for_each(|j| {
                    dst_row[j] ^= src_row[j];
                });
            });
        });
    });
    result
}
pub fn binary_matrix_m4rm2(a: &BinaryMatrix2, b: &BinaryMatrix2) -> BinaryMatrix2 {
    assert_eq!(a.ncols, b.nrows);
    let mut result = BinaryMatrix2::new(a.nrows, b.ncols);
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

        let table = make_table2(&indices, b);
        let block = block_row / BLOCK_SIZE;

        result
            .data
            .par_chunks_exact_mut(result.width)
            .enumerate()
            .for_each(|(r, dst_row)| {
                let idx = &a.data[r * a.width + block];
                (0..(indices.len() - 1)).for_each(|i| {
                    let src_row = ((idx << indices[i])
                        >> (BLOCK_SIZE - (indices[i + 1] - indices[i])))
                        as usize;
                    let src_row =
                        &table[i].data[src_row * table[i].width..(src_row + 1) * table[i].width];
                    (0..result.width).for_each(|j| {
                        dst_row[j] ^= src_row[j];
                    });
                });
            });
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_table() {
        let mut binary_matrix = BinaryMatrix::new(8, 8);
        binary_matrix.rand();
        println!("{}", binary_matrix);
        let indices = [0, 1, 3, 6, 8, 8, 8, 8, 8];
        let result = make_table(&indices, &binary_matrix);
        println!("tablesize:{}", result.len());
        for i in result {
            println!("{}", i);
        }
    }
    #[test]
    fn test_make_table2() {
        let mut binary_matrix = BinaryMatrix2::new(8, 8);
        binary_matrix.rand();
        println!("{}", binary_matrix);
        let indices = [0, 1, 3, 6, 8, 8, 8, 8, 8];
        let result = make_table2(&indices, &binary_matrix);
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
        let result = binary_matrix_m4rm(&binary_matrix1, &binary_matrix2);
        println!("{}", result);
    }
    #[test]
    fn test_m4rm2() {
        let size = 128;
        let mut binary_matrix1 = BinaryMatrix2::new(size, size);
        binary_matrix1.rand();
        println!("{}", binary_matrix1);
        let mut binary_matrix2 = BinaryMatrix2::new(size, size);
        binary_matrix2.rand();
        println!("{}", binary_matrix2);
        let result = binary_matrix_m4rm2(&binary_matrix1, &binary_matrix2);
        println!("{}", result);
    }
}

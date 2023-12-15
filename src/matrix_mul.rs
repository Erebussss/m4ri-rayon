use rayon::prelude::*;

use crate::binary_matrix::BinaryMatrix;

const TABLE_SIZE: usize = 8;
const BLOCK_SIZE: usize = 64;
const TABLE_NUM: usize = 8;

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

pub fn binary_matrix_m4rm(a: &BinaryMatrix, b: &BinaryMatrix) -> BinaryMatrix {
    assert!(a.ncols == b.nrows);
    let mut c = BinaryMatrix::new(a.nrows, b.ncols);
    for now_block_row in (0..b.nrows).step_by(BLOCK_SIZE) {
        let now_row = BLOCK_SIZE.min(b.nrows - now_block_row);
        let mut indices = [now_row / 8; 9];
        indices[0] = 0;
        for i in 1..8 {
            indices[i] += indices[i - 1];
            if i <= now_row % 8 {
                indices[i] += 1;
            }
        }
        assert!(indices[8] == now_row);

        let table = make_table(&indices, b);

        (0..a.nrows).into_par_iter().for_each(|r| {
            let mut tmp_row = c.data[r].write().unwrap();
            let now_block = now_block_row / BLOCK_SIZE;
            let idx = a.data[r].read().unwrap();
        })
    }
    c
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
}

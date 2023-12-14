use rayon::prelude::*;

use crate::binary_matrix::BinaryMatrix;

const BLOCK_SIZE: usize = 64;
const TABLE_SIZE: usize = 8;
const TABLE_NUM: usize = BLOCK_SIZE / TABLE_SIZE;

pub fn binary_matrix_m4rm(a: &BinaryMatrix, b: &BinaryMatrix) -> BinaryMatrix {
    assert!(a.ncols == b.nrows);
    let mut c = BinaryMatrix::new(a.nrows, b.ncols);
    let mut tb = vec![vec![0u64; (1 << TABLE_SIZE) * b.width]; TABLE_NUM];
    for now_block_row in (0..b.nrows).step_by(BLOCK_SIZE) {
        for now_table_row in
            (now_block_row..b.nrows.min(now_block_row + BLOCK_SIZE)).step_by(TABLE_SIZE)
        {
            let now_table_idx = (now_table_row - now_block_row) / TABLE_SIZE;
            let now_table_size = b.nrows.min(now_table_row + TABLE_SIZE) - now_table_row;
            (0..(1 << now_table_size))
                .into_par_iter()
                .for_each(|i: usize| {
                    for j in 0..now_table_size {
                        if i & (1 << (TABLE_SIZE - 1 - j)) != 0 {}
                    }
                })
        }
    }
    c
}

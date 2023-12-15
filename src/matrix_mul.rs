use rayon::prelude::*;

use crate::binary_matrix::BinaryMatrix;

// const TABLE_SIZE: usize = 8;
// const BLOCK_SIZE: usize = 64;
// const TABLE_NUM: usize = 8;

fn make_table(idx: Vec<usize>, b: &BinaryMatrix) -> Vec<BinaryMatrix> {
    let mut tb: Vec<BinaryMatrix> = Vec::new();
    idx.iter().zip(idx.iter().next()).for_each(|(rb, re)| {
        let tbn = re - rb;
        let tmp: BinaryMatrix = BinaryMatrix::new(1 << tbn, b.ncols);
        (0..(1 << tbn)).into_par_iter().for_each(|tbr| {
            let mut tmp = tmp.data[tbr].write().unwrap();
            for i in 0..tbn {
                if tbr & (1 << (tbn - 1 - i)) == 1 {
                    let src = b.data[rb + i].read().unwrap();
                    for j in 0..b.width {
                        tmp[j] ^= src[j];
                    }
                }
            }
        });
        tb.push(tmp);
    });
    tb
}

// pub fn binary_matrix_m4rm(a: &BinaryMatrix<T>, b: &BinaryMatrix<T>) -> BinaryMatrix<T> {
//     assert!(a.ncols == b.nrows);
//     let mut c = BinaryMatrix::new(a.nrows, b.ncols);
//     let mut tb = vec![vec![0u64; (1 << TABLE_SIZE) * b.width]; TABLE_NUM];
//     for now_block_row in (0..b.nrows).step_by(BLOCK_SIZE) {
//         for now_table_row in
//             (now_block_row..b.nrows.min(now_block_row + BLOCK_SIZE)).step_by(TABLE_SIZE)
//         {
//             let now_table_idx = (now_table_row - now_block_row) / TABLE_SIZE;
//             let now_table_size = b.nrows.min(now_table_row + TABLE_SIZE) - now_table_row;
//             (0..(1 << now_table_size))
//                 .into_par_iter()
//                 .for_each(|i: usize| {
//                     for j in 0..now_table_size {
//                         if i & (1 << (TABLE_SIZE - 1 - j)) != 0 {}
//                     }
//                 })
//         }
//     }
//     c
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_table() {
        let mut binary_matrix = BinaryMatrix::new(8, 8);
        binary_matrix.rand();
        println!("{}", binary_matrix);
        let idx = vec![0, 8];
        let result = make_table(idx, &binary_matrix);
        println!("tablesize:{}", result.len());
        println!("{}", result[0]);
    }
}

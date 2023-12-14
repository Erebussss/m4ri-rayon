use std::{
    ops::BitXorAssign,
    simd::{num::SimdUint, SimdElement},
};

use rayon::prelude::*;

use crate::binary_matrix::BinaryMatrix;

const TABLE_SIZE: usize = 8;
fn make_table<T>(idx: Vec<usize>, b: &BinaryMatrix<T>) -> Vec<BinaryMatrix<T>>
where
    T: SimdElement + SimdUint + BitXorAssign + Send + Sync + Default,
{
    let mut tb: Vec<BinaryMatrix<T>> = Vec::new();
    idx.iter().zip(idx.iter().next()).for_each(|(rb, re)| {
        let tbn = re - rb;
        let tmp: BinaryMatrix<T> = BinaryMatrix::new(1 << tbn, b.ncols);
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
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_make_table() {
//         // Create a sample BinaryMatrix
//         let mut binary_matrix = BinaryMatrix::new(4, 4);
//         binary_matrix.data[0] = Arc::new(RwLock::new([1, 0, 1, 0]));
//         binary_matrix.data[1] = Arc::new(RwLock::new([0, 1, 0, 1]));
//         binary_matrix.data[2] = Arc::new(RwLock::new([1, 1, 0, 0]));
//         binary_matrix.data[3] = Arc::new(RwLock::new([0, 0, 1, 1]));

//         // Define the index vector
//         let idx = vec![0, 2];

//         // Call the make_table function
//         let result = make_table(idx, &binary_matrix);

//         // Assert the result
//         assert_eq!(result.len(), 2);
//         assert_eq!(result[0].data[0].read().unwrap(), [1, 0, 1, 0]);
//         assert_eq!(result[0].data[1].read().unwrap(), [0, 1, 0, 1]);
//         assert_eq!(result[1].data[0].read().unwrap(), [1, 1, 0, 0]);
//         assert_eq!(result[1].data[1].read().unwrap(), [0, 0, 1, 1]);
//     }
// }

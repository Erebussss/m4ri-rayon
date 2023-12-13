use crate::binary_matrix::BinaryMatrix;

const TABLE_SIZE: usize = 1 << 8;

pub fn binary_matrix_m4rm(a: &BinaryMatrix, b: &BinaryMatrix) -> BinaryMatrix {
    assert!(a.ncols == b.nrows);
    let mut c = BinaryMatrix::new(a.nrows, b.ncols);
    let mut tb = vec![0; TABLE_SIZE * b.width];

    c
}

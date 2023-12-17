#[deprecated]
pub mod binary_matrix1;
pub mod binary_matrix2;

const BLOCK_SIZE: usize = 64;
const TABLE_NUM: usize = 8;

pub use binary_matrix2::BinaryMatrix2 as BM;

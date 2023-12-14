#[derive(Debug)]
pub struct BinaryMatrix<T>
where
    T: std::ops::BitXor<Output = T> + Copy + Default,
{
    pub nrows: usize,
    pub ncols: usize,
    pub tbyte: usize,
    pub width: usize,
    pub data: Vec<T>,
}
impl<T> BinaryMatrix<T>
where
    T: std::ops::BitXor<Output = T> + Copy + Default,
{
    pub fn new(nrows: usize, ncols: usize) -> BinaryMatrix<T> {
        let tbyte = std::mem::size_of::<T>();
        let width = (ncols + tbyte * 8 - 1) / (tbyte * 8);
        let data = vec![T::default(); nrows * width];
        BinaryMatrix {
            nrows,
            ncols,
            tbyte,
            width,
            data,
        }
    }
}

pub struct BinaryMatrix {
    pub nrows: usize,
    pub ncols: usize,
    pub width: usize,
    pub data: Vec<u64>,
}
impl BinaryMatrix {
    pub fn new(nrows: usize, ncols: usize) -> BinaryMatrix {
        let width = (ncols + 63) / 64;
        let data = vec![0; nrows * width];
        BinaryMatrix {
            nrows,
            ncols,
            width,
            data,
        }
    }
}

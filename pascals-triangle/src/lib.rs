mod binom;
use binom::binom::pascal_row;

pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let result = (0..row_count).into_iter().map(|x| pascal_row(x)).collect();
        PascalsTriangle(result)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let result = self.0.clone();
        result
    }
}

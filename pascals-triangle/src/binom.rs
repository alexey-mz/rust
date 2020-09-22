pub mod binom {
    use factorial::Factorial;

    fn elem (row: u32, elem: u32) -> u32 {
        row.factorial() / (elem.factorial() * (row - elem).factorial())
    }

    pub fn pascal_row (row: u32) -> Vec<u32> {
        (0..=row).into_iter().map(|x| elem(row, x)).collect::<Vec<u32>>()
    }
}
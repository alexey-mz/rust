use factorial::Factorial;

fn elem (row: u32, elem: u32) -> u32 {
    row.factorial() / (elem.factorial() * (row - elem).factorial())
}

fn pascal_row (row: u32) -> Vec<u32> {
    (0..=row).into_iter().map(|x| elem(row, x)).collect::<Vec<u32>>()
}



fn main() {
    let n = elem(0, 0);
    let n_row = pascal_row(3);
    let result: Vec<_> = (0..3u32).into_iter().map(|x| pascal_row(x)).collect();
    println!("{:?}", result);
}

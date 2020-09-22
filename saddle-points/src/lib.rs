pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input.iter().enumerate().fold(vec![], |mut acc, (row_index, row)| {
        row.iter().enumerate().fold(acc, |mut acc, (col_index, &element)| {
            let column = input.iter().map(|x| x[col_index]).collect::<Vec<_>>();
            if element == *row.iter().max().unwrap() && element == *column.iter().min().unwrap() {
                acc.push((row_index, col_index));
            }
            acc
        })
    })
}

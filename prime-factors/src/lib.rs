fn find_first_div (n: u64) -> u64 {
    (2..=n).find(|x| n % x == 0).unwrap()
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut test = n;
    let mut result = vec![];
    while test != 1 {
        result.push(find_first_div(test));
        test = test / find_first_div(test);
    }
    result
}

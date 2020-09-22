fn check_prime(n: usize) -> bool {
    match n {
        0 | 1 => return false,
        _ => {
            for i in 2..n-1 {
                if n % i == 0 {
                    return false
                }
            }
            return true
        }
    }
}

pub fn nth(n: usize) -> usize {
    let mut result: Vec<usize> = Vec::new();
    let mut index: usize = 0;
    while result.len() <= n {
        if check_prime(index) {
            result.push(index);
            index += 1;
        }
        else {
            index += 1;
            continue;
        }
    }
    *result.iter().last().unwrap()
}

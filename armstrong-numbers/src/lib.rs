use std::convert::TryInto;

fn int_to_vec(n: u32) -> Vec<u32> {
    let mut len: u32 = n;
    let mut temp: u32 = 0;
    let mut result = Vec::new();
    while len != 0 {
        temp = len % 10;
        len = (len - temp) / 10;
        result.push(temp);
    }
    result
}

pub fn is_armstrong_number(num: u32) -> bool {
    if num == int_to_vec(num).iter().fold(0, |acc, x| {
        acc + x.pow(int_to_vec(num).len().try_into().unwrap()) as u32
    }) {
        true
    }
    else {
        false
    }
}

pub fn check(candidate: &str) -> bool {
    let len = candidate.replace(&['-', ' '][..], "").len();
    let mut temp: Vec<char> = candidate.replace(&['-', ' '][..], "").to_lowercase().chars().collect();
    temp.sort();
    temp.dedup();
    return temp.len() == len
}

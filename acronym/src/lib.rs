pub fn abbreviate(phrase: &str) -> String {
    let t = phrase.split(|ch: char| !ch.is_alphabetic()).filter(|&x| !x.is_empty()).filter(|&x| x.len() > 1 || x.chars().next().unwrap().is_uppercase()).collect::<Vec<&str>>();
    t.iter().fold(String::new(), |mut acc, &x| {
        if x.chars().all(|x| x.is_uppercase()) {
            acc.push(x.chars().next().unwrap());
        }
        else if (x.chars().filter(|ch| ch.is_uppercase()).collect::<String>()).len() > 1 {
            acc.push_str(x.chars().filter(|x| x.is_uppercase()).collect::<String>().as_str())
        }
        else {
            acc.push(x.chars().next().unwrap().to_ascii_uppercase());
        }
        acc
    })
}

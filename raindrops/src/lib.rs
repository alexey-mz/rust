pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)

    // let mut result = "".to_string();
    let divs = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    // for (i, str) in &divs {
    //     match n % i {
    //         0 => result.push_str(str),
    //         _ => (),
    //     }
    // }
    // if result.is_empty() {
    //     n.to_string()
    // }
    // else {
    //     result
    // }
    let res = divs.iter().map(|(i, str)| {
        match n % i {
            0 => str,
            _ => ""
        }
    }).collect::<Vec<&str>>().join("");
    match res.is_empty() {
        true => n.to_string(),
        false => res
    }
}

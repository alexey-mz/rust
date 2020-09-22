pub fn build_proverb(list: &[&str]) -> String {
    let mut result = Vec::new();
    list.iter().for_each(|x| {
        let index = list.iter().position(|y| y == x).unwrap();
        let msg = match  list.get(index + 1) {
            Some(_) => format!("For want of a {} the {} was lost.", list[index], list[index + 1]),
            None => format!("And all for the want of a {}.", list[0])
        };
        result.push(msg);
    });
    result.join("\n")
}
